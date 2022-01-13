#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IEnumNetworkConnectionsImpl: Sized + IDispatchImpl {
    fn _NewEnum(&mut self) -> ::windows::core::Result<super::super::System::Ole::IEnumVARIANT>;
    fn Next(&mut self, celt: u32, rgelt: *mut ::core::option::Option<INetworkConnection>, pceltfetched: *mut u32) -> ::windows::core::Result<()>;
    fn Skip(&mut self, celt: u32) -> ::windows::core::Result<()>;
    fn Reset(&mut self) -> ::windows::core::Result<()>;
    fn Clone(&mut self) -> ::windows::core::Result<IEnumNetworkConnections>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IEnumNetworkConnectionsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IEnumNetworkConnectionsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IEnumNetworkConnectionsVtbl {
        unsafe extern "system" fn _NewEnum<Impl: IEnumNetworkConnectionsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenumvar: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this)._NewEnum() {
                ::core::result::Result::Ok(ok__) => {
                    *ppenumvar = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Next<Impl: IEnumNetworkConnectionsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32, rgelt: *mut ::windows::core::RawPtr, pceltfetched: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Next(::core::mem::transmute_copy(&celt), ::core::mem::transmute_copy(&rgelt), ::core::mem::transmute_copy(&pceltfetched)).into()
        }
        unsafe extern "system" fn Skip<Impl: IEnumNetworkConnectionsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Skip(::core::mem::transmute_copy(&celt)).into()
        }
        unsafe extern "system" fn Reset<Impl: IEnumNetworkConnectionsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Reset().into()
        }
        unsafe extern "system" fn Clone<Impl: IEnumNetworkConnectionsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenumnetwork: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Clone() {
                ::core::result::Result::Ok(ok__) => {
                    *ppenumnetwork = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: IDispatchVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            _NewEnum: _NewEnum::<Impl, IMPL_OFFSET>,
            Next: Next::<Impl, IMPL_OFFSET>,
            Skip: Skip::<Impl, IMPL_OFFSET>,
            Reset: Reset::<Impl, IMPL_OFFSET>,
            Clone: Clone::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IEnumNetworkConnections as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IEnumNetworksImpl: Sized + IDispatchImpl {
    fn _NewEnum(&mut self) -> ::windows::core::Result<super::super::System::Ole::IEnumVARIANT>;
    fn Next(&mut self, celt: u32, rgelt: *mut ::core::option::Option<INetwork>, pceltfetched: *mut u32) -> ::windows::core::Result<()>;
    fn Skip(&mut self, celt: u32) -> ::windows::core::Result<()>;
    fn Reset(&mut self) -> ::windows::core::Result<()>;
    fn Clone(&mut self) -> ::windows::core::Result<IEnumNetworks>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IEnumNetworksVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IEnumNetworksImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IEnumNetworksVtbl {
        unsafe extern "system" fn _NewEnum<Impl: IEnumNetworksImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenumvar: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this)._NewEnum() {
                ::core::result::Result::Ok(ok__) => {
                    *ppenumvar = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Next<Impl: IEnumNetworksImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32, rgelt: *mut ::windows::core::RawPtr, pceltfetched: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Next(::core::mem::transmute_copy(&celt), ::core::mem::transmute_copy(&rgelt), ::core::mem::transmute_copy(&pceltfetched)).into()
        }
        unsafe extern "system" fn Skip<Impl: IEnumNetworksImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Skip(::core::mem::transmute_copy(&celt)).into()
        }
        unsafe extern "system" fn Reset<Impl: IEnumNetworksImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Reset().into()
        }
        unsafe extern "system" fn Clone<Impl: IEnumNetworksImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenumnetwork: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Clone() {
                ::core::result::Result::Ok(ok__) => {
                    *ppenumnetwork = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: IDispatchVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            _NewEnum: _NewEnum::<Impl, IMPL_OFFSET>,
            Next: Next::<Impl, IMPL_OFFSET>,
            Skip: Skip::<Impl, IMPL_OFFSET>,
            Reset: Reset::<Impl, IMPL_OFFSET>,
            Clone: Clone::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IEnumNetworks as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait INetworkImpl: Sized + IDispatchImpl {
    fn GetName(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetName(&mut self, sznetworknewname: super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn GetDescription(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetDescription(&mut self, szdescription: super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn GetNetworkId(&mut self) -> ::windows::core::Result<::windows::core::GUID>;
    fn GetDomainType(&mut self) -> ::windows::core::Result<NLM_DOMAIN_TYPE>;
    fn GetNetworkConnections(&mut self) -> ::windows::core::Result<IEnumNetworkConnections>;
    fn GetTimeCreatedAndConnected(&mut self, pdwlowdatetimecreated: *mut u32, pdwhighdatetimecreated: *mut u32, pdwlowdatetimeconnected: *mut u32, pdwhighdatetimeconnected: *mut u32) -> ::windows::core::Result<()>;
    fn IsConnectedToInternet(&mut self) -> ::windows::core::Result<i16>;
    fn IsConnected(&mut self) -> ::windows::core::Result<i16>;
    fn GetConnectivity(&mut self) -> ::windows::core::Result<NLM_CONNECTIVITY>;
    fn GetCategory(&mut self) -> ::windows::core::Result<NLM_NETWORK_CATEGORY>;
    fn SetCategory(&mut self, newcategory: NLM_NETWORK_CATEGORY) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl INetworkVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: INetworkImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> INetworkVtbl {
        unsafe extern "system" fn GetName<Impl: INetworkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psznetworkname: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetName() {
                ::core::result::Result::Ok(ok__) => {
                    *psznetworkname = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetName<Impl: INetworkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, sznetworknewname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetName(::core::mem::transmute_copy(&sznetworknewname)).into()
        }
        unsafe extern "system" fn GetDescription<Impl: INetworkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszdescription: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDescription() {
                ::core::result::Result::Ok(ok__) => {
                    *pszdescription = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDescription<Impl: INetworkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, szdescription: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetDescription(::core::mem::transmute_copy(&szdescription)).into()
        }
        unsafe extern "system" fn GetNetworkId<Impl: INetworkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pgdguidnetworkid: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetNetworkId() {
                ::core::result::Result::Ok(ok__) => {
                    *pgdguidnetworkid = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDomainType<Impl: INetworkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pnetworktype: *mut NLM_DOMAIN_TYPE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDomainType() {
                ::core::result::Result::Ok(ok__) => {
                    *pnetworktype = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetNetworkConnections<Impl: INetworkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenumnetworkconnection: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetNetworkConnections() {
                ::core::result::Result::Ok(ok__) => {
                    *ppenumnetworkconnection = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetTimeCreatedAndConnected<Impl: INetworkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwlowdatetimecreated: *mut u32, pdwhighdatetimecreated: *mut u32, pdwlowdatetimeconnected: *mut u32, pdwhighdatetimeconnected: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetTimeCreatedAndConnected(::core::mem::transmute_copy(&pdwlowdatetimecreated), ::core::mem::transmute_copy(&pdwhighdatetimecreated), ::core::mem::transmute_copy(&pdwlowdatetimeconnected), ::core::mem::transmute_copy(&pdwhighdatetimeconnected)).into()
        }
        unsafe extern "system" fn IsConnectedToInternet<Impl: INetworkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbisconnected: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsConnectedToInternet() {
                ::core::result::Result::Ok(ok__) => {
                    *pbisconnected = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsConnected<Impl: INetworkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbisconnected: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsConnected() {
                ::core::result::Result::Ok(ok__) => {
                    *pbisconnected = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetConnectivity<Impl: INetworkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pconnectivity: *mut NLM_CONNECTIVITY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetConnectivity() {
                ::core::result::Result::Ok(ok__) => {
                    *pconnectivity = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCategory<Impl: INetworkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcategory: *mut NLM_NETWORK_CATEGORY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetCategory() {
                ::core::result::Result::Ok(ok__) => {
                    *pcategory = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCategory<Impl: INetworkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newcategory: NLM_NETWORK_CATEGORY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetCategory(::core::mem::transmute_copy(&newcategory)).into()
        }
        Self {
            base: IDispatchVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            GetName: GetName::<Impl, IMPL_OFFSET>,
            SetName: SetName::<Impl, IMPL_OFFSET>,
            GetDescription: GetDescription::<Impl, IMPL_OFFSET>,
            SetDescription: SetDescription::<Impl, IMPL_OFFSET>,
            GetNetworkId: GetNetworkId::<Impl, IMPL_OFFSET>,
            GetDomainType: GetDomainType::<Impl, IMPL_OFFSET>,
            GetNetworkConnections: GetNetworkConnections::<Impl, IMPL_OFFSET>,
            GetTimeCreatedAndConnected: GetTimeCreatedAndConnected::<Impl, IMPL_OFFSET>,
            IsConnectedToInternet: IsConnectedToInternet::<Impl, IMPL_OFFSET>,
            IsConnected: IsConnected::<Impl, IMPL_OFFSET>,
            GetConnectivity: GetConnectivity::<Impl, IMPL_OFFSET>,
            GetCategory: GetCategory::<Impl, IMPL_OFFSET>,
            SetCategory: SetCategory::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<INetwork as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait INetworkConnectionImpl: Sized + IDispatchImpl {
    fn GetNetwork(&mut self) -> ::windows::core::Result<INetwork>;
    fn IsConnectedToInternet(&mut self) -> ::windows::core::Result<i16>;
    fn IsConnected(&mut self) -> ::windows::core::Result<i16>;
    fn GetConnectivity(&mut self) -> ::windows::core::Result<NLM_CONNECTIVITY>;
    fn GetConnectionId(&mut self) -> ::windows::core::Result<::windows::core::GUID>;
    fn GetAdapterId(&mut self) -> ::windows::core::Result<::windows::core::GUID>;
    fn GetDomainType(&mut self) -> ::windows::core::Result<NLM_DOMAIN_TYPE>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl INetworkConnectionVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: INetworkConnectionImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> INetworkConnectionVtbl {
        unsafe extern "system" fn GetNetwork<Impl: INetworkConnectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppnetwork: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetNetwork() {
                ::core::result::Result::Ok(ok__) => {
                    *ppnetwork = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsConnectedToInternet<Impl: INetworkConnectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbisconnected: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsConnectedToInternet() {
                ::core::result::Result::Ok(ok__) => {
                    *pbisconnected = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsConnected<Impl: INetworkConnectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbisconnected: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsConnected() {
                ::core::result::Result::Ok(ok__) => {
                    *pbisconnected = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetConnectivity<Impl: INetworkConnectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pconnectivity: *mut NLM_CONNECTIVITY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetConnectivity() {
                ::core::result::Result::Ok(ok__) => {
                    *pconnectivity = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetConnectionId<Impl: INetworkConnectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pgdconnectionid: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetConnectionId() {
                ::core::result::Result::Ok(ok__) => {
                    *pgdconnectionid = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetAdapterId<Impl: INetworkConnectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pgdadapterid: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetAdapterId() {
                ::core::result::Result::Ok(ok__) => {
                    *pgdadapterid = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDomainType<Impl: INetworkConnectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdomaintype: *mut NLM_DOMAIN_TYPE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDomainType() {
                ::core::result::Result::Ok(ok__) => {
                    *pdomaintype = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: IDispatchVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            GetNetwork: GetNetwork::<Impl, IMPL_OFFSET>,
            IsConnectedToInternet: IsConnectedToInternet::<Impl, IMPL_OFFSET>,
            IsConnected: IsConnected::<Impl, IMPL_OFFSET>,
            GetConnectivity: GetConnectivity::<Impl, IMPL_OFFSET>,
            GetConnectionId: GetConnectionId::<Impl, IMPL_OFFSET>,
            GetAdapterId: GetAdapterId::<Impl, IMPL_OFFSET>,
            GetDomainType: GetDomainType::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<INetworkConnection as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait INetworkConnectionCostImpl: Sized {
    fn GetCost(&mut self) -> ::windows::core::Result<u32>;
    fn GetDataPlanStatus(&mut self) -> ::windows::core::Result<NLM_DATAPLAN_STATUS>;
}
#[cfg(feature = "Win32_Foundation")]
impl INetworkConnectionCostVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: INetworkConnectionCostImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> INetworkConnectionCostVtbl {
        unsafe extern "system" fn GetCost<Impl: INetworkConnectionCostImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcost: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetCost() {
                ::core::result::Result::Ok(ok__) => {
                    *pcost = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDataPlanStatus<Impl: INetworkConnectionCostImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdataplanstatus: *mut NLM_DATAPLAN_STATUS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDataPlanStatus() {
                ::core::result::Result::Ok(ok__) => {
                    *pdataplanstatus = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            GetCost: GetCost::<Impl, IMPL_OFFSET>,
            GetDataPlanStatus: GetDataPlanStatus::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<INetworkConnectionCost as ::windows::core::Interface>::IID
    }
}
pub trait INetworkConnectionCostEventsImpl: Sized {
    fn ConnectionCostChanged(&mut self, connectionid: ::windows::core::GUID, newcost: u32) -> ::windows::core::Result<()>;
    fn ConnectionDataPlanStatusChanged(&mut self, connectionid: ::windows::core::GUID) -> ::windows::core::Result<()>;
}
impl INetworkConnectionCostEventsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: INetworkConnectionCostEventsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> INetworkConnectionCostEventsVtbl {
        unsafe extern "system" fn ConnectionCostChanged<Impl: INetworkConnectionCostEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, connectionid: ::windows::core::GUID, newcost: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ConnectionCostChanged(::core::mem::transmute_copy(&connectionid), ::core::mem::transmute_copy(&newcost)).into()
        }
        unsafe extern "system" fn ConnectionDataPlanStatusChanged<Impl: INetworkConnectionCostEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, connectionid: ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ConnectionDataPlanStatusChanged(::core::mem::transmute_copy(&connectionid)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            ConnectionCostChanged: ConnectionCostChanged::<Impl, IMPL_OFFSET>,
            ConnectionDataPlanStatusChanged: ConnectionDataPlanStatusChanged::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<INetworkConnectionCostEvents as ::windows::core::Interface>::IID
    }
}
pub trait INetworkConnectionEventsImpl: Sized {
    fn NetworkConnectionConnectivityChanged(&mut self, connectionid: ::windows::core::GUID, newconnectivity: NLM_CONNECTIVITY) -> ::windows::core::Result<()>;
    fn NetworkConnectionPropertyChanged(&mut self, connectionid: ::windows::core::GUID, flags: NLM_CONNECTION_PROPERTY_CHANGE) -> ::windows::core::Result<()>;
}
impl INetworkConnectionEventsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: INetworkConnectionEventsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> INetworkConnectionEventsVtbl {
        unsafe extern "system" fn NetworkConnectionConnectivityChanged<Impl: INetworkConnectionEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, connectionid: ::windows::core::GUID, newconnectivity: NLM_CONNECTIVITY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).NetworkConnectionConnectivityChanged(::core::mem::transmute_copy(&connectionid), ::core::mem::transmute_copy(&newconnectivity)).into()
        }
        unsafe extern "system" fn NetworkConnectionPropertyChanged<Impl: INetworkConnectionEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, connectionid: ::windows::core::GUID, flags: NLM_CONNECTION_PROPERTY_CHANGE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).NetworkConnectionPropertyChanged(::core::mem::transmute_copy(&connectionid), ::core::mem::transmute_copy(&flags)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            NetworkConnectionConnectivityChanged: NetworkConnectionConnectivityChanged::<Impl, IMPL_OFFSET>,
            NetworkConnectionPropertyChanged: NetworkConnectionPropertyChanged::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<INetworkConnectionEvents as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait INetworkCostManagerImpl: Sized {
    fn GetCost(&mut self, pcost: *mut u32, pdestipaddr: *const NLM_SOCKADDR) -> ::windows::core::Result<()>;
    fn GetDataPlanStatus(&mut self, pdataplanstatus: *mut NLM_DATAPLAN_STATUS, pdestipaddr: *const NLM_SOCKADDR) -> ::windows::core::Result<()>;
    fn SetDestinationAddresses(&mut self, length: u32, pdestipaddrlist: *const NLM_SOCKADDR, bappend: i16) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl INetworkCostManagerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: INetworkCostManagerImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> INetworkCostManagerVtbl {
        unsafe extern "system" fn GetCost<Impl: INetworkCostManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcost: *mut u32, pdestipaddr: *const NLM_SOCKADDR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetCost(::core::mem::transmute_copy(&pcost), ::core::mem::transmute_copy(&pdestipaddr)).into()
        }
        unsafe extern "system" fn GetDataPlanStatus<Impl: INetworkCostManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdataplanstatus: *mut NLM_DATAPLAN_STATUS, pdestipaddr: *const NLM_SOCKADDR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetDataPlanStatus(::core::mem::transmute_copy(&pdataplanstatus), ::core::mem::transmute_copy(&pdestipaddr)).into()
        }
        unsafe extern "system" fn SetDestinationAddresses<Impl: INetworkCostManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, length: u32, pdestipaddrlist: *const NLM_SOCKADDR, bappend: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetDestinationAddresses(::core::mem::transmute_copy(&length), ::core::mem::transmute_copy(&pdestipaddrlist), ::core::mem::transmute_copy(&bappend)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            GetCost: GetCost::<Impl, IMPL_OFFSET>,
            GetDataPlanStatus: GetDataPlanStatus::<Impl, IMPL_OFFSET>,
            SetDestinationAddresses: SetDestinationAddresses::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<INetworkCostManager as ::windows::core::Interface>::IID
    }
}
pub trait INetworkCostManagerEventsImpl: Sized {
    fn CostChanged(&mut self, newcost: u32, pdestaddr: *const NLM_SOCKADDR) -> ::windows::core::Result<()>;
    fn DataPlanStatusChanged(&mut self, pdestaddr: *const NLM_SOCKADDR) -> ::windows::core::Result<()>;
}
impl INetworkCostManagerEventsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: INetworkCostManagerEventsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> INetworkCostManagerEventsVtbl {
        unsafe extern "system" fn CostChanged<Impl: INetworkCostManagerEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newcost: u32, pdestaddr: *const NLM_SOCKADDR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).CostChanged(::core::mem::transmute_copy(&newcost), ::core::mem::transmute_copy(&pdestaddr)).into()
        }
        unsafe extern "system" fn DataPlanStatusChanged<Impl: INetworkCostManagerEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdestaddr: *const NLM_SOCKADDR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).DataPlanStatusChanged(::core::mem::transmute_copy(&pdestaddr)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            CostChanged: CostChanged::<Impl, IMPL_OFFSET>,
            DataPlanStatusChanged: DataPlanStatusChanged::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<INetworkCostManagerEvents as ::windows::core::Interface>::IID
    }
}
pub trait INetworkEventsImpl: Sized {
    fn NetworkAdded(&mut self, networkid: ::windows::core::GUID) -> ::windows::core::Result<()>;
    fn NetworkDeleted(&mut self, networkid: ::windows::core::GUID) -> ::windows::core::Result<()>;
    fn NetworkConnectivityChanged(&mut self, networkid: ::windows::core::GUID, newconnectivity: NLM_CONNECTIVITY) -> ::windows::core::Result<()>;
    fn NetworkPropertyChanged(&mut self, networkid: ::windows::core::GUID, flags: NLM_NETWORK_PROPERTY_CHANGE) -> ::windows::core::Result<()>;
}
impl INetworkEventsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: INetworkEventsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> INetworkEventsVtbl {
        unsafe extern "system" fn NetworkAdded<Impl: INetworkEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, networkid: ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).NetworkAdded(::core::mem::transmute_copy(&networkid)).into()
        }
        unsafe extern "system" fn NetworkDeleted<Impl: INetworkEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, networkid: ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).NetworkDeleted(::core::mem::transmute_copy(&networkid)).into()
        }
        unsafe extern "system" fn NetworkConnectivityChanged<Impl: INetworkEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, networkid: ::windows::core::GUID, newconnectivity: NLM_CONNECTIVITY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).NetworkConnectivityChanged(::core::mem::transmute_copy(&networkid), ::core::mem::transmute_copy(&newconnectivity)).into()
        }
        unsafe extern "system" fn NetworkPropertyChanged<Impl: INetworkEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, networkid: ::windows::core::GUID, flags: NLM_NETWORK_PROPERTY_CHANGE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).NetworkPropertyChanged(::core::mem::transmute_copy(&networkid), ::core::mem::transmute_copy(&flags)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            NetworkAdded: NetworkAdded::<Impl, IMPL_OFFSET>,
            NetworkDeleted: NetworkDeleted::<Impl, IMPL_OFFSET>,
            NetworkConnectivityChanged: NetworkConnectivityChanged::<Impl, IMPL_OFFSET>,
            NetworkPropertyChanged: NetworkPropertyChanged::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<INetworkEvents as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait INetworkListManagerImpl: Sized + IDispatchImpl {
    fn GetNetworks(&mut self, flags: NLM_ENUM_NETWORK) -> ::windows::core::Result<IEnumNetworks>;
    fn GetNetwork(&mut self, gdnetworkid: ::windows::core::GUID) -> ::windows::core::Result<INetwork>;
    fn GetNetworkConnections(&mut self) -> ::windows::core::Result<IEnumNetworkConnections>;
    fn GetNetworkConnection(&mut self, gdnetworkconnectionid: ::windows::core::GUID) -> ::windows::core::Result<INetworkConnection>;
    fn IsConnectedToInternet(&mut self) -> ::windows::core::Result<i16>;
    fn IsConnected(&mut self) -> ::windows::core::Result<i16>;
    fn GetConnectivity(&mut self) -> ::windows::core::Result<NLM_CONNECTIVITY>;
    fn SetSimulatedProfileInfo(&mut self, psimulatedinfo: *const NLM_SIMULATED_PROFILE_INFO) -> ::windows::core::Result<()>;
    fn ClearSimulatedProfileInfo(&mut self) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl INetworkListManagerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: INetworkListManagerImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> INetworkListManagerVtbl {
        unsafe extern "system" fn GetNetworks<Impl: INetworkListManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, flags: NLM_ENUM_NETWORK, ppenumnetwork: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetNetworks(::core::mem::transmute_copy(&flags)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppenumnetwork = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetNetwork<Impl: INetworkListManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, gdnetworkid: ::windows::core::GUID, ppnetwork: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetNetwork(::core::mem::transmute_copy(&gdnetworkid)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppnetwork = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetNetworkConnections<Impl: INetworkListManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetNetworkConnections() {
                ::core::result::Result::Ok(ok__) => {
                    *ppenum = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetNetworkConnection<Impl: INetworkListManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, gdnetworkconnectionid: ::windows::core::GUID, ppnetworkconnection: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetNetworkConnection(::core::mem::transmute_copy(&gdnetworkconnectionid)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppnetworkconnection = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsConnectedToInternet<Impl: INetworkListManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbisconnected: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsConnectedToInternet() {
                ::core::result::Result::Ok(ok__) => {
                    *pbisconnected = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsConnected<Impl: INetworkListManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbisconnected: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsConnected() {
                ::core::result::Result::Ok(ok__) => {
                    *pbisconnected = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetConnectivity<Impl: INetworkListManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pconnectivity: *mut NLM_CONNECTIVITY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetConnectivity() {
                ::core::result::Result::Ok(ok__) => {
                    *pconnectivity = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSimulatedProfileInfo<Impl: INetworkListManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psimulatedinfo: *const NLM_SIMULATED_PROFILE_INFO) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetSimulatedProfileInfo(::core::mem::transmute_copy(&psimulatedinfo)).into()
        }
        unsafe extern "system" fn ClearSimulatedProfileInfo<Impl: INetworkListManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ClearSimulatedProfileInfo().into()
        }
        Self {
            base: IDispatchVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            GetNetworks: GetNetworks::<Impl, IMPL_OFFSET>,
            GetNetwork: GetNetwork::<Impl, IMPL_OFFSET>,
            GetNetworkConnections: GetNetworkConnections::<Impl, IMPL_OFFSET>,
            GetNetworkConnection: GetNetworkConnection::<Impl, IMPL_OFFSET>,
            IsConnectedToInternet: IsConnectedToInternet::<Impl, IMPL_OFFSET>,
            IsConnected: IsConnected::<Impl, IMPL_OFFSET>,
            GetConnectivity: GetConnectivity::<Impl, IMPL_OFFSET>,
            SetSimulatedProfileInfo: SetSimulatedProfileInfo::<Impl, IMPL_OFFSET>,
            ClearSimulatedProfileInfo: ClearSimulatedProfileInfo::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<INetworkListManager as ::windows::core::Interface>::IID
    }
}
pub trait INetworkListManagerEventsImpl: Sized {
    fn ConnectivityChanged(&mut self, newconnectivity: NLM_CONNECTIVITY) -> ::windows::core::Result<()>;
}
impl INetworkListManagerEventsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: INetworkListManagerEventsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> INetworkListManagerEventsVtbl {
        unsafe extern "system" fn ConnectivityChanged<Impl: INetworkListManagerEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newconnectivity: NLM_CONNECTIVITY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ConnectivityChanged(::core::mem::transmute_copy(&newconnectivity)).into()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), ConnectivityChanged: ConnectivityChanged::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<INetworkListManagerEvents as ::windows::core::Interface>::IID
    }
}
