#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IEnumNetworkConnectionsImpl: Sized + IDispatchImpl {
    fn _NewEnum();
    fn Next();
    fn Skip();
    fn Reset();
    fn Clone();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IEnumNetworkConnectionsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IEnumNetworkConnectionsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IEnumNetworkConnectionsVtbl {
        unsafe extern "system" fn _NewEnum<Impl: IEnumNetworkConnectionsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenumvar: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Next<Impl: IEnumNetworkConnectionsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32, rgelt: *mut ::windows::core::RawPtr, pceltfetched: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Skip<Impl: IEnumNetworkConnectionsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Reset<Impl: IEnumNetworkConnectionsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Clone<Impl: IEnumNetworkConnectionsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenumnetwork: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
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
    fn _NewEnum();
    fn Next();
    fn Skip();
    fn Reset();
    fn Clone();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IEnumNetworksVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IEnumNetworksImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IEnumNetworksVtbl {
        unsafe extern "system" fn _NewEnum<Impl: IEnumNetworksImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenumvar: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Next<Impl: IEnumNetworksImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32, rgelt: *mut ::windows::core::RawPtr, pceltfetched: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Skip<Impl: IEnumNetworksImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Reset<Impl: IEnumNetworksImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Clone<Impl: IEnumNetworksImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenumnetwork: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
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
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl INetworkVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: INetworkImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> INetworkVtbl {
        unsafe extern "system" fn GetName<Impl: INetworkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psznetworkname: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetName<Impl: INetworkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, sznetworknewname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetDescription<Impl: INetworkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszdescription: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetDescription<Impl: INetworkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, szdescription: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetNetworkId<Impl: INetworkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pgdguidnetworkid: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetDomainType<Impl: INetworkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pnetworktype: *mut NLM_DOMAIN_TYPE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetNetworkConnections<Impl: INetworkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenumnetworkconnection: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetTimeCreatedAndConnected<Impl: INetworkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwlowdatetimecreated: *mut u32, pdwhighdatetimecreated: *mut u32, pdwlowdatetimeconnected: *mut u32, pdwhighdatetimeconnected: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn IsConnectedToInternet<Impl: INetworkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbisconnected: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn IsConnected<Impl: INetworkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbisconnected: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetConnectivity<Impl: INetworkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pconnectivity: *mut NLM_CONNECTIVITY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetCategory<Impl: INetworkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcategory: *mut NLM_NETWORK_CATEGORY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetCategory<Impl: INetworkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newcategory: NLM_NETWORK_CATEGORY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
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
    fn GetNetwork();
    fn IsConnectedToInternet();
    fn IsConnected();
    fn GetConnectivity();
    fn GetConnectionId();
    fn GetAdapterId();
    fn GetDomainType();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl INetworkConnectionVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: INetworkConnectionImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> INetworkConnectionVtbl {
        unsafe extern "system" fn GetNetwork<Impl: INetworkConnectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppnetwork: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn IsConnectedToInternet<Impl: INetworkConnectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbisconnected: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn IsConnected<Impl: INetworkConnectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbisconnected: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetConnectivity<Impl: INetworkConnectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pconnectivity: *mut NLM_CONNECTIVITY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetConnectionId<Impl: INetworkConnectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pgdconnectionid: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetAdapterId<Impl: INetworkConnectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pgdadapterid: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetDomainType<Impl: INetworkConnectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdomaintype: *mut NLM_DOMAIN_TYPE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
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
    fn GetCost();
    fn GetDataPlanStatus();
}
#[cfg(feature = "Win32_Foundation")]
impl INetworkConnectionCostVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: INetworkConnectionCostImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> INetworkConnectionCostVtbl {
        unsafe extern "system" fn GetCost<Impl: INetworkConnectionCostImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcost: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetDataPlanStatus<Impl: INetworkConnectionCostImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdataplanstatus: *mut NLM_DATAPLAN_STATUS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
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
    fn ConnectionCostChanged();
    fn ConnectionDataPlanStatusChanged();
}
impl INetworkConnectionCostEventsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: INetworkConnectionCostEventsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> INetworkConnectionCostEventsVtbl {
        unsafe extern "system" fn ConnectionCostChanged<Impl: INetworkConnectionCostEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, connectionid: ::windows::core::GUID, newcost: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ConnectionDataPlanStatusChanged<Impl: INetworkConnectionCostEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, connectionid: ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
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
    fn NetworkConnectionConnectivityChanged();
    fn NetworkConnectionPropertyChanged();
}
impl INetworkConnectionEventsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: INetworkConnectionEventsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> INetworkConnectionEventsVtbl {
        unsafe extern "system" fn NetworkConnectionConnectivityChanged<Impl: INetworkConnectionEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, connectionid: ::windows::core::GUID, newconnectivity: NLM_CONNECTIVITY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn NetworkConnectionPropertyChanged<Impl: INetworkConnectionEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, connectionid: ::windows::core::GUID, flags: NLM_CONNECTION_PROPERTY_CHANGE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
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
    fn GetCost();
    fn GetDataPlanStatus();
    fn SetDestinationAddresses();
}
#[cfg(feature = "Win32_Foundation")]
impl INetworkCostManagerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: INetworkCostManagerImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> INetworkCostManagerVtbl {
        unsafe extern "system" fn GetCost<Impl: INetworkCostManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcost: *mut u32, pdestipaddr: *const NLM_SOCKADDR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetDataPlanStatus<Impl: INetworkCostManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdataplanstatus: *mut NLM_DATAPLAN_STATUS, pdestipaddr: *const NLM_SOCKADDR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetDestinationAddresses<Impl: INetworkCostManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, length: u32, pdestipaddrlist: *const NLM_SOCKADDR, bappend: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
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
    fn CostChanged();
    fn DataPlanStatusChanged();
}
impl INetworkCostManagerEventsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: INetworkCostManagerEventsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> INetworkCostManagerEventsVtbl {
        unsafe extern "system" fn CostChanged<Impl: INetworkCostManagerEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newcost: u32, pdestaddr: *const NLM_SOCKADDR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn DataPlanStatusChanged<Impl: INetworkCostManagerEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdestaddr: *const NLM_SOCKADDR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
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
    fn NetworkAdded();
    fn NetworkDeleted();
    fn NetworkConnectivityChanged();
    fn NetworkPropertyChanged();
}
impl INetworkEventsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: INetworkEventsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> INetworkEventsVtbl {
        unsafe extern "system" fn NetworkAdded<Impl: INetworkEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, networkid: ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn NetworkDeleted<Impl: INetworkEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, networkid: ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn NetworkConnectivityChanged<Impl: INetworkEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, networkid: ::windows::core::GUID, newconnectivity: NLM_CONNECTIVITY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn NetworkPropertyChanged<Impl: INetworkEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, networkid: ::windows::core::GUID, flags: NLM_NETWORK_PROPERTY_CHANGE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
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
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl INetworkListManagerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: INetworkListManagerImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> INetworkListManagerVtbl {
        unsafe extern "system" fn GetNetworks<Impl: INetworkListManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, flags: NLM_ENUM_NETWORK, ppenumnetwork: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetNetwork<Impl: INetworkListManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, gdnetworkid: ::windows::core::GUID, ppnetwork: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetNetworkConnections<Impl: INetworkListManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetNetworkConnection<Impl: INetworkListManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, gdnetworkconnectionid: ::windows::core::GUID, ppnetworkconnection: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn IsConnectedToInternet<Impl: INetworkListManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbisconnected: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn IsConnected<Impl: INetworkListManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbisconnected: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetConnectivity<Impl: INetworkListManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pconnectivity: *mut NLM_CONNECTIVITY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetSimulatedProfileInfo<Impl: INetworkListManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psimulatedinfo: *const NLM_SIMULATED_PROFILE_INFO) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ClearSimulatedProfileInfo<Impl: INetworkListManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
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
    fn ConnectivityChanged();
}
impl INetworkListManagerEventsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: INetworkListManagerEventsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> INetworkListManagerEventsVtbl {
        unsafe extern "system" fn ConnectivityChanged<Impl: INetworkListManagerEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newconnectivity: NLM_CONNECTIVITY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), ConnectivityChanged: ConnectivityChanged::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<INetworkListManagerEvents as ::windows::core::Interface>::IID
    }
}
