#[cfg(feature = "Win32_System_Com")]
pub trait IEnumNetworkConnectionsImpl: Sized + IDispatchImpl {
    fn _NewEnum();
    fn Next();
    fn Skip();
    fn Reset();
    fn Clone();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for IEnumNetworkConnections {
    const NAME: &'static str = "Windows.Win32.Networking.NetworkListManager.IEnumNetworkConnections";
}
#[cfg(feature = "Win32_System_Com")]
impl IEnumNetworkConnectionsVtbl {
    pub const fn new<Impl: IEnumNetworkConnectionsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IEnumNetworkConnectionsVtbl {
        unsafe extern "system" fn _NewEnum<Impl: IEnumNetworkConnectionsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppenumvar: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this)._NewEnum(::core::mem::transmute_copy(&ppenumvar)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Next<Impl: IEnumNetworkConnectionsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, celt: u32, rgelt: *mut ::windows::core::RawPtr, pceltfetched: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Next(celt, ::core::mem::transmute_copy(&rgelt), pceltfetched) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Skip<Impl: IEnumNetworkConnectionsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, celt: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Skip(celt) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Reset<Impl: IEnumNetworkConnectionsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Reset() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Clone<Impl: IEnumNetworkConnectionsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppenumnetwork: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Clone(::core::mem::transmute_copy(&ppenumnetwork)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IEnumNetworkConnections>, base.5, _NewEnum::<Impl, OFFSET>, Next::<Impl, OFFSET>, Skip::<Impl, OFFSET>, Reset::<Impl, OFFSET>, Clone::<Impl, OFFSET>)
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IEnumNetworksImpl: Sized + IDispatchImpl {
    fn _NewEnum();
    fn Next();
    fn Skip();
    fn Reset();
    fn Clone();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for IEnumNetworks {
    const NAME: &'static str = "Windows.Win32.Networking.NetworkListManager.IEnumNetworks";
}
#[cfg(feature = "Win32_System_Com")]
impl IEnumNetworksVtbl {
    pub const fn new<Impl: IEnumNetworksImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IEnumNetworksVtbl {
        unsafe extern "system" fn _NewEnum<Impl: IEnumNetworksImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppenumvar: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this)._NewEnum(::core::mem::transmute_copy(&ppenumvar)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Next<Impl: IEnumNetworksImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, celt: u32, rgelt: *mut ::windows::core::RawPtr, pceltfetched: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Next(celt, ::core::mem::transmute_copy(&rgelt), pceltfetched) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Skip<Impl: IEnumNetworksImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, celt: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Skip(celt) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Reset<Impl: IEnumNetworksImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Reset() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Clone<Impl: IEnumNetworksImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppenumnetwork: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Clone(::core::mem::transmute_copy(&ppenumnetwork)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IEnumNetworks>, base.5, _NewEnum::<Impl, OFFSET>, Next::<Impl, OFFSET>, Skip::<Impl, OFFSET>, Reset::<Impl, OFFSET>, Clone::<Impl, OFFSET>)
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait INetworkImpl: Sized + IDispatchImpl {
    fn GetName();
    fn SetName();
    fn GetDescription();
    fn SetDescription();
    fn GetNetworkId();
    fn GetDomainType();
    fn GetNetworkConnections();
    fn GetTimeCreatedAndConnected();
    fn IsConnectedToInternet();
    fn IsConnected();
    fn GetConnectivity();
    fn GetCategory();
    fn SetCategory();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for INetwork {
    const NAME: &'static str = "Windows.Win32.Networking.NetworkListManager.INetwork";
}
#[cfg(feature = "Win32_System_Com")]
impl INetworkVtbl {
    pub const fn new<Impl: INetworkImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> INetworkVtbl {
        unsafe extern "system" fn GetName<Impl: INetworkImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, psznetworkname: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetName(::core::mem::transmute_copy(&psznetworkname)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetName<Impl: INetworkImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, sznetworknewname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetName(&*(&sznetworknewname as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDescription<Impl: INetworkImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pszdescription: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetDescription(::core::mem::transmute_copy(&pszdescription)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDescription<Impl: INetworkImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, szdescription: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetDescription(&*(&szdescription as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetNetworkId<Impl: INetworkImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pgdguidnetworkid: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetNetworkId(::core::mem::transmute_copy(&pgdguidnetworkid)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDomainType<Impl: INetworkImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pnetworktype: *mut NLM_DOMAIN_TYPE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetDomainType(::core::mem::transmute_copy(&pnetworktype)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetNetworkConnections<Impl: INetworkImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppenumnetworkconnection: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetNetworkConnections(::core::mem::transmute_copy(&ppenumnetworkconnection)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetTimeCreatedAndConnected<Impl: INetworkImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdwlowdatetimecreated: *mut u32, pdwhighdatetimecreated: *mut u32, pdwlowdatetimeconnected: *mut u32, pdwhighdatetimeconnected: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetTimeCreatedAndConnected(::core::mem::transmute_copy(&pdwlowdatetimecreated), ::core::mem::transmute_copy(&pdwhighdatetimecreated), ::core::mem::transmute_copy(&pdwlowdatetimeconnected), ::core::mem::transmute_copy(&pdwhighdatetimeconnected)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsConnectedToInternet<Impl: INetworkImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbisconnected: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).IsConnectedToInternet(::core::mem::transmute_copy(&pbisconnected)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsConnected<Impl: INetworkImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbisconnected: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).IsConnected(::core::mem::transmute_copy(&pbisconnected)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetConnectivity<Impl: INetworkImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pconnectivity: *mut NLM_CONNECTIVITY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetConnectivity(::core::mem::transmute_copy(&pconnectivity)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCategory<Impl: INetworkImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pcategory: *mut NLM_NETWORK_CATEGORY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetCategory(::core::mem::transmute_copy(&pcategory)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCategory<Impl: INetworkImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, newcategory: NLM_NETWORK_CATEGORY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetCategory(newcategory) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(
            base.0,
            base.1,
            base.2,
            base.3,
            ::windows::core::GetRuntimeClassName::<INetwork>,
            base.5,
            GetName::<Impl, OFFSET>,
            SetName::<Impl, OFFSET>,
            GetDescription::<Impl, OFFSET>,
            SetDescription::<Impl, OFFSET>,
            GetNetworkId::<Impl, OFFSET>,
            GetDomainType::<Impl, OFFSET>,
            GetNetworkConnections::<Impl, OFFSET>,
            GetTimeCreatedAndConnected::<Impl, OFFSET>,
            IsConnectedToInternet::<Impl, OFFSET>,
            IsConnected::<Impl, OFFSET>,
            GetConnectivity::<Impl, OFFSET>,
            GetCategory::<Impl, OFFSET>,
            SetCategory::<Impl, OFFSET>,
        )
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait INetworkConnectionImpl: Sized + IDispatchImpl {
    fn GetNetwork();
    fn IsConnectedToInternet();
    fn IsConnected();
    fn GetConnectivity();
    fn GetConnectionId();
    fn GetAdapterId();
    fn GetDomainType();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for INetworkConnection {
    const NAME: &'static str = "Windows.Win32.Networking.NetworkListManager.INetworkConnection";
}
#[cfg(feature = "Win32_System_Com")]
impl INetworkConnectionVtbl {
    pub const fn new<Impl: INetworkConnectionImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> INetworkConnectionVtbl {
        unsafe extern "system" fn GetNetwork<Impl: INetworkConnectionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppnetwork: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetNetwork(::core::mem::transmute_copy(&ppnetwork)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsConnectedToInternet<Impl: INetworkConnectionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbisconnected: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).IsConnectedToInternet(::core::mem::transmute_copy(&pbisconnected)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsConnected<Impl: INetworkConnectionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbisconnected: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).IsConnected(::core::mem::transmute_copy(&pbisconnected)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetConnectivity<Impl: INetworkConnectionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pconnectivity: *mut NLM_CONNECTIVITY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetConnectivity(::core::mem::transmute_copy(&pconnectivity)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetConnectionId<Impl: INetworkConnectionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pgdconnectionid: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetConnectionId(::core::mem::transmute_copy(&pgdconnectionid)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetAdapterId<Impl: INetworkConnectionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pgdadapterid: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetAdapterId(::core::mem::transmute_copy(&pgdadapterid)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDomainType<Impl: INetworkConnectionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdomaintype: *mut NLM_DOMAIN_TYPE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetDomainType(::core::mem::transmute_copy(&pdomaintype)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<INetworkConnection>, base.5, GetNetwork::<Impl, OFFSET>, IsConnectedToInternet::<Impl, OFFSET>, IsConnected::<Impl, OFFSET>, GetConnectivity::<Impl, OFFSET>, GetConnectionId::<Impl, OFFSET>, GetAdapterId::<Impl, OFFSET>, GetDomainType::<Impl, OFFSET>)
    }
}
pub trait INetworkConnectionCostImpl: Sized {
    fn GetCost();
    fn GetDataPlanStatus();
}
impl ::windows::core::RuntimeName for INetworkConnectionCost {
    const NAME: &'static str = "Windows.Win32.Networking.NetworkListManager.INetworkConnectionCost";
}
impl INetworkConnectionCostVtbl {
    pub const fn new<Impl: INetworkConnectionCostImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> INetworkConnectionCostVtbl {
        unsafe extern "system" fn GetCost<Impl: INetworkConnectionCostImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pcost: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetCost(::core::mem::transmute_copy(&pcost)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDataPlanStatus<Impl: INetworkConnectionCostImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdataplanstatus: *mut NLM_DATAPLAN_STATUS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetDataPlanStatus(::core::mem::transmute_copy(&pdataplanstatus)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<INetworkConnectionCost>, base.5, GetCost::<Impl, OFFSET>, GetDataPlanStatus::<Impl, OFFSET>)
    }
}
pub trait INetworkConnectionCostEventsImpl: Sized {
    fn ConnectionCostChanged();
    fn ConnectionDataPlanStatusChanged();
}
impl ::windows::core::RuntimeName for INetworkConnectionCostEvents {
    const NAME: &'static str = "Windows.Win32.Networking.NetworkListManager.INetworkConnectionCostEvents";
}
impl INetworkConnectionCostEventsVtbl {
    pub const fn new<Impl: INetworkConnectionCostEventsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> INetworkConnectionCostEventsVtbl {
        unsafe extern "system" fn ConnectionCostChanged<Impl: INetworkConnectionCostEventsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, connectionid: ::windows::core::GUID, newcost: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ConnectionCostChanged(&*(&connectionid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), newcost) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ConnectionDataPlanStatusChanged<Impl: INetworkConnectionCostEventsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, connectionid: ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ConnectionDataPlanStatusChanged(&*(&connectionid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<INetworkConnectionCostEvents>, base.5, ConnectionCostChanged::<Impl, OFFSET>, ConnectionDataPlanStatusChanged::<Impl, OFFSET>)
    }
}
pub trait INetworkConnectionEventsImpl: Sized {
    fn NetworkConnectionConnectivityChanged();
    fn NetworkConnectionPropertyChanged();
}
impl ::windows::core::RuntimeName for INetworkConnectionEvents {
    const NAME: &'static str = "Windows.Win32.Networking.NetworkListManager.INetworkConnectionEvents";
}
impl INetworkConnectionEventsVtbl {
    pub const fn new<Impl: INetworkConnectionEventsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> INetworkConnectionEventsVtbl {
        unsafe extern "system" fn NetworkConnectionConnectivityChanged<Impl: INetworkConnectionEventsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, connectionid: ::windows::core::GUID, newconnectivity: NLM_CONNECTIVITY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).NetworkConnectionConnectivityChanged(&*(&connectionid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), newconnectivity) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn NetworkConnectionPropertyChanged<Impl: INetworkConnectionEventsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, connectionid: ::windows::core::GUID, flags: NLM_CONNECTION_PROPERTY_CHANGE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).NetworkConnectionPropertyChanged(&*(&connectionid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), flags) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<INetworkConnectionEvents>, base.5, NetworkConnectionConnectivityChanged::<Impl, OFFSET>, NetworkConnectionPropertyChanged::<Impl, OFFSET>)
    }
}
pub trait INetworkCostManagerImpl: Sized {
    fn GetCost();
    fn GetDataPlanStatus();
    fn SetDestinationAddresses();
}
impl ::windows::core::RuntimeName for INetworkCostManager {
    const NAME: &'static str = "Windows.Win32.Networking.NetworkListManager.INetworkCostManager";
}
impl INetworkCostManagerVtbl {
    pub const fn new<Impl: INetworkCostManagerImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> INetworkCostManagerVtbl {
        unsafe extern "system" fn GetCost<Impl: INetworkCostManagerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pcost: *mut u32, pdestipaddr: *const NLM_SOCKADDR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetCost(::core::mem::transmute_copy(&pcost), &*(&pdestipaddr as *const <NLM_SOCKADDR as ::windows::core::Abi>::Abi as *const <NLM_SOCKADDR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDataPlanStatus<Impl: INetworkCostManagerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdataplanstatus: *mut NLM_DATAPLAN_STATUS, pdestipaddr: *const NLM_SOCKADDR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetDataPlanStatus(::core::mem::transmute_copy(&pdataplanstatus), &*(&pdestipaddr as *const <NLM_SOCKADDR as ::windows::core::Abi>::Abi as *const <NLM_SOCKADDR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDestinationAddresses<Impl: INetworkCostManagerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, length: u32, pdestipaddrlist: *const NLM_SOCKADDR, bappend: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetDestinationAddresses(length, &*(&pdestipaddrlist as *const <NLM_SOCKADDR as ::windows::core::Abi>::Abi as *const <NLM_SOCKADDR as ::windows::core::DefaultType>::DefaultType), bappend) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<INetworkCostManager>, base.5, GetCost::<Impl, OFFSET>, GetDataPlanStatus::<Impl, OFFSET>, SetDestinationAddresses::<Impl, OFFSET>)
    }
}
pub trait INetworkCostManagerEventsImpl: Sized {
    fn CostChanged();
    fn DataPlanStatusChanged();
}
impl ::windows::core::RuntimeName for INetworkCostManagerEvents {
    const NAME: &'static str = "Windows.Win32.Networking.NetworkListManager.INetworkCostManagerEvents";
}
impl INetworkCostManagerEventsVtbl {
    pub const fn new<Impl: INetworkCostManagerEventsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> INetworkCostManagerEventsVtbl {
        unsafe extern "system" fn CostChanged<Impl: INetworkCostManagerEventsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, newcost: u32, pdestaddr: *const NLM_SOCKADDR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CostChanged(newcost, &*(&pdestaddr as *const <NLM_SOCKADDR as ::windows::core::Abi>::Abi as *const <NLM_SOCKADDR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DataPlanStatusChanged<Impl: INetworkCostManagerEventsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdestaddr: *const NLM_SOCKADDR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).DataPlanStatusChanged(&*(&pdestaddr as *const <NLM_SOCKADDR as ::windows::core::Abi>::Abi as *const <NLM_SOCKADDR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<INetworkCostManagerEvents>, base.5, CostChanged::<Impl, OFFSET>, DataPlanStatusChanged::<Impl, OFFSET>)
    }
}
pub trait INetworkEventsImpl: Sized {
    fn NetworkAdded();
    fn NetworkDeleted();
    fn NetworkConnectivityChanged();
    fn NetworkPropertyChanged();
}
impl ::windows::core::RuntimeName for INetworkEvents {
    const NAME: &'static str = "Windows.Win32.Networking.NetworkListManager.INetworkEvents";
}
impl INetworkEventsVtbl {
    pub const fn new<Impl: INetworkEventsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> INetworkEventsVtbl {
        unsafe extern "system" fn NetworkAdded<Impl: INetworkEventsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, networkid: ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).NetworkAdded(&*(&networkid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn NetworkDeleted<Impl: INetworkEventsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, networkid: ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).NetworkDeleted(&*(&networkid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn NetworkConnectivityChanged<Impl: INetworkEventsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, networkid: ::windows::core::GUID, newconnectivity: NLM_CONNECTIVITY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).NetworkConnectivityChanged(&*(&networkid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), newconnectivity) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn NetworkPropertyChanged<Impl: INetworkEventsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, networkid: ::windows::core::GUID, flags: NLM_NETWORK_PROPERTY_CHANGE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).NetworkPropertyChanged(&*(&networkid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), flags) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<INetworkEvents>, base.5, NetworkAdded::<Impl, OFFSET>, NetworkDeleted::<Impl, OFFSET>, NetworkConnectivityChanged::<Impl, OFFSET>, NetworkPropertyChanged::<Impl, OFFSET>)
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait INetworkListManagerImpl: Sized + IDispatchImpl {
    fn GetNetworks();
    fn GetNetwork();
    fn GetNetworkConnections();
    fn GetNetworkConnection();
    fn IsConnectedToInternet();
    fn IsConnected();
    fn GetConnectivity();
    fn SetSimulatedProfileInfo();
    fn ClearSimulatedProfileInfo();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for INetworkListManager {
    const NAME: &'static str = "Windows.Win32.Networking.NetworkListManager.INetworkListManager";
}
#[cfg(feature = "Win32_System_Com")]
impl INetworkListManagerVtbl {
    pub const fn new<Impl: INetworkListManagerImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> INetworkListManagerVtbl {
        unsafe extern "system" fn GetNetworks<Impl: INetworkListManagerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, flags: NLM_ENUM_NETWORK, ppenumnetwork: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetNetworks(flags, ::core::mem::transmute_copy(&ppenumnetwork)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetNetwork<Impl: INetworkListManagerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, gdnetworkid: ::windows::core::GUID, ppnetwork: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetNetwork(&*(&gdnetworkid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&ppnetwork)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetNetworkConnections<Impl: INetworkListManagerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetNetworkConnections(::core::mem::transmute_copy(&ppenum)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetNetworkConnection<Impl: INetworkListManagerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, gdnetworkconnectionid: ::windows::core::GUID, ppnetworkconnection: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetNetworkConnection(&*(&gdnetworkconnectionid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&ppnetworkconnection)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsConnectedToInternet<Impl: INetworkListManagerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbisconnected: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).IsConnectedToInternet(::core::mem::transmute_copy(&pbisconnected)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsConnected<Impl: INetworkListManagerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbisconnected: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).IsConnected(::core::mem::transmute_copy(&pbisconnected)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetConnectivity<Impl: INetworkListManagerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pconnectivity: *mut NLM_CONNECTIVITY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetConnectivity(::core::mem::transmute_copy(&pconnectivity)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSimulatedProfileInfo<Impl: INetworkListManagerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, psimulatedinfo: *const NLM_SIMULATED_PROFILE_INFO) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetSimulatedProfileInfo(&*(&psimulatedinfo as *const <NLM_SIMULATED_PROFILE_INFO as ::windows::core::Abi>::Abi as *const <NLM_SIMULATED_PROFILE_INFO as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ClearSimulatedProfileInfo<Impl: INetworkListManagerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ClearSimulatedProfileInfo() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<INetworkListManager>, base.5, GetNetworks::<Impl, OFFSET>, GetNetwork::<Impl, OFFSET>, GetNetworkConnections::<Impl, OFFSET>, GetNetworkConnection::<Impl, OFFSET>, IsConnectedToInternet::<Impl, OFFSET>, IsConnected::<Impl, OFFSET>, GetConnectivity::<Impl, OFFSET>, SetSimulatedProfileInfo::<Impl, OFFSET>, ClearSimulatedProfileInfo::<Impl, OFFSET>)
    }
}
pub trait INetworkListManagerEventsImpl: Sized {
    fn ConnectivityChanged();
}
impl ::windows::core::RuntimeName for INetworkListManagerEvents {
    const NAME: &'static str = "Windows.Win32.Networking.NetworkListManager.INetworkListManagerEvents";
}
impl INetworkListManagerEventsVtbl {
    pub const fn new<Impl: INetworkListManagerEventsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> INetworkListManagerEventsVtbl {
        unsafe extern "system" fn ConnectivityChanged<Impl: INetworkListManagerEventsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, newconnectivity: NLM_CONNECTIVITY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ConnectivityChanged(newconnectivity) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<INetworkListManagerEvents>, base.5, ConnectivityChanged::<Impl, OFFSET>)
    }
}
