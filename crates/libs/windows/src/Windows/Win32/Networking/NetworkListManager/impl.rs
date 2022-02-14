#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IEnumNetworkConnections_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn _NewEnum(&self) -> ::windows::core::Result<super::super::System::Ole::IEnumVARIANT>;
    fn Next(&self, celt: u32, rgelt: *mut ::core::option::Option<INetworkConnection>, pceltfetched: *mut u32) -> ::windows::core::Result<()>;
    fn Skip(&self, celt: u32) -> ::windows::core::Result<()>;
    fn Reset(&self) -> ::windows::core::Result<()>;
    fn Clone(&self) -> ::windows::core::Result<IEnumNetworkConnections>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IEnumNetworkConnections_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IEnumNetworkConnections_Impl, const OFFSET: isize>() -> IEnumNetworkConnections_Vtbl {
        unsafe extern "system" fn _NewEnum<Identity: ::windows::core::IUnknownImpl, Impl: IEnumNetworkConnections_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenumvar: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this)._NewEnum() {
                ::core::result::Result::Ok(ok__) => {
                    *ppenumvar = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Next<Identity: ::windows::core::IUnknownImpl, Impl: IEnumNetworkConnections_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32, rgelt: *mut ::windows::core::RawPtr, pceltfetched: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Next(::core::mem::transmute_copy(&celt), ::core::mem::transmute_copy(&rgelt), ::core::mem::transmute_copy(&pceltfetched)).into()
        }
        unsafe extern "system" fn Skip<Identity: ::windows::core::IUnknownImpl, Impl: IEnumNetworkConnections_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Skip(::core::mem::transmute_copy(&celt)).into()
        }
        unsafe extern "system" fn Reset<Identity: ::windows::core::IUnknownImpl, Impl: IEnumNetworkConnections_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Reset().into()
        }
        unsafe extern "system" fn Clone<Identity: ::windows::core::IUnknownImpl, Impl: IEnumNetworkConnections_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenumnetwork: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Clone() {
                ::core::result::Result::Ok(ok__) => {
                    *ppenumnetwork = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            _NewEnum: _NewEnum::<Identity, Impl, OFFSET>,
            Next: Next::<Identity, Impl, OFFSET>,
            Skip: Skip::<Identity, Impl, OFFSET>,
            Reset: Reset::<Identity, Impl, OFFSET>,
            Clone: Clone::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IEnumNetworkConnections as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IEnumNetworks_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn _NewEnum(&self) -> ::windows::core::Result<super::super::System::Ole::IEnumVARIANT>;
    fn Next(&self, celt: u32, rgelt: *mut ::core::option::Option<INetwork>, pceltfetched: *mut u32) -> ::windows::core::Result<()>;
    fn Skip(&self, celt: u32) -> ::windows::core::Result<()>;
    fn Reset(&self) -> ::windows::core::Result<()>;
    fn Clone(&self) -> ::windows::core::Result<IEnumNetworks>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IEnumNetworks_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IEnumNetworks_Impl, const OFFSET: isize>() -> IEnumNetworks_Vtbl {
        unsafe extern "system" fn _NewEnum<Identity: ::windows::core::IUnknownImpl, Impl: IEnumNetworks_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenumvar: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this)._NewEnum() {
                ::core::result::Result::Ok(ok__) => {
                    *ppenumvar = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Next<Identity: ::windows::core::IUnknownImpl, Impl: IEnumNetworks_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32, rgelt: *mut ::windows::core::RawPtr, pceltfetched: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Next(::core::mem::transmute_copy(&celt), ::core::mem::transmute_copy(&rgelt), ::core::mem::transmute_copy(&pceltfetched)).into()
        }
        unsafe extern "system" fn Skip<Identity: ::windows::core::IUnknownImpl, Impl: IEnumNetworks_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Skip(::core::mem::transmute_copy(&celt)).into()
        }
        unsafe extern "system" fn Reset<Identity: ::windows::core::IUnknownImpl, Impl: IEnumNetworks_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Reset().into()
        }
        unsafe extern "system" fn Clone<Identity: ::windows::core::IUnknownImpl, Impl: IEnumNetworks_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenumnetwork: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Clone() {
                ::core::result::Result::Ok(ok__) => {
                    *ppenumnetwork = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            _NewEnum: _NewEnum::<Identity, Impl, OFFSET>,
            Next: Next::<Identity, Impl, OFFSET>,
            Skip: Skip::<Identity, Impl, OFFSET>,
            Reset: Reset::<Identity, Impl, OFFSET>,
            Clone: Clone::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IEnumNetworks as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait INetwork_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn GetName(&self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetName(&self, sznetworknewname: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn GetDescription(&self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetDescription(&self, szdescription: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn GetNetworkId(&self) -> ::windows::core::Result<::windows::core::GUID>;
    fn GetDomainType(&self) -> ::windows::core::Result<NLM_DOMAIN_TYPE>;
    fn GetNetworkConnections(&self) -> ::windows::core::Result<IEnumNetworkConnections>;
    fn GetTimeCreatedAndConnected(&self, pdwlowdatetimecreated: *mut u32, pdwhighdatetimecreated: *mut u32, pdwlowdatetimeconnected: *mut u32, pdwhighdatetimeconnected: *mut u32) -> ::windows::core::Result<()>;
    fn IsConnectedToInternet(&self) -> ::windows::core::Result<i16>;
    fn IsConnected(&self) -> ::windows::core::Result<i16>;
    fn GetConnectivity(&self) -> ::windows::core::Result<NLM_CONNECTIVITY>;
    fn GetCategory(&self) -> ::windows::core::Result<NLM_NETWORK_CATEGORY>;
    fn SetCategory(&self, newcategory: NLM_NETWORK_CATEGORY) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl INetwork_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: INetwork_Impl, const OFFSET: isize>() -> INetwork_Vtbl {
        unsafe extern "system" fn GetName<Identity: ::windows::core::IUnknownImpl, Impl: INetwork_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psznetworkname: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetName() {
                ::core::result::Result::Ok(ok__) => {
                    *psznetworkname = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetName<Identity: ::windows::core::IUnknownImpl, Impl: INetwork_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, sznetworknewname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetName(::core::mem::transmute(&sznetworknewname)).into()
        }
        unsafe extern "system" fn GetDescription<Identity: ::windows::core::IUnknownImpl, Impl: INetwork_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszdescription: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetDescription() {
                ::core::result::Result::Ok(ok__) => {
                    *pszdescription = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDescription<Identity: ::windows::core::IUnknownImpl, Impl: INetwork_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, szdescription: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetDescription(::core::mem::transmute(&szdescription)).into()
        }
        unsafe extern "system" fn GetNetworkId<Identity: ::windows::core::IUnknownImpl, Impl: INetwork_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pgdguidnetworkid: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetNetworkId() {
                ::core::result::Result::Ok(ok__) => {
                    *pgdguidnetworkid = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDomainType<Identity: ::windows::core::IUnknownImpl, Impl: INetwork_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pnetworktype: *mut NLM_DOMAIN_TYPE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetDomainType() {
                ::core::result::Result::Ok(ok__) => {
                    *pnetworktype = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetNetworkConnections<Identity: ::windows::core::IUnknownImpl, Impl: INetwork_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenumnetworkconnection: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetNetworkConnections() {
                ::core::result::Result::Ok(ok__) => {
                    *ppenumnetworkconnection = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetTimeCreatedAndConnected<Identity: ::windows::core::IUnknownImpl, Impl: INetwork_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwlowdatetimecreated: *mut u32, pdwhighdatetimecreated: *mut u32, pdwlowdatetimeconnected: *mut u32, pdwhighdatetimeconnected: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetTimeCreatedAndConnected(::core::mem::transmute_copy(&pdwlowdatetimecreated), ::core::mem::transmute_copy(&pdwhighdatetimecreated), ::core::mem::transmute_copy(&pdwlowdatetimeconnected), ::core::mem::transmute_copy(&pdwhighdatetimeconnected)).into()
        }
        unsafe extern "system" fn IsConnectedToInternet<Identity: ::windows::core::IUnknownImpl, Impl: INetwork_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbisconnected: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).IsConnectedToInternet() {
                ::core::result::Result::Ok(ok__) => {
                    *pbisconnected = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsConnected<Identity: ::windows::core::IUnknownImpl, Impl: INetwork_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbisconnected: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).IsConnected() {
                ::core::result::Result::Ok(ok__) => {
                    *pbisconnected = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetConnectivity<Identity: ::windows::core::IUnknownImpl, Impl: INetwork_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pconnectivity: *mut NLM_CONNECTIVITY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetConnectivity() {
                ::core::result::Result::Ok(ok__) => {
                    *pconnectivity = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCategory<Identity: ::windows::core::IUnknownImpl, Impl: INetwork_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcategory: *mut NLM_NETWORK_CATEGORY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetCategory() {
                ::core::result::Result::Ok(ok__) => {
                    *pcategory = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCategory<Identity: ::windows::core::IUnknownImpl, Impl: INetwork_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newcategory: NLM_NETWORK_CATEGORY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetCategory(::core::mem::transmute_copy(&newcategory)).into()
        }
        Self {
            base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            GetName: GetName::<Identity, Impl, OFFSET>,
            SetName: SetName::<Identity, Impl, OFFSET>,
            GetDescription: GetDescription::<Identity, Impl, OFFSET>,
            SetDescription: SetDescription::<Identity, Impl, OFFSET>,
            GetNetworkId: GetNetworkId::<Identity, Impl, OFFSET>,
            GetDomainType: GetDomainType::<Identity, Impl, OFFSET>,
            GetNetworkConnections: GetNetworkConnections::<Identity, Impl, OFFSET>,
            GetTimeCreatedAndConnected: GetTimeCreatedAndConnected::<Identity, Impl, OFFSET>,
            IsConnectedToInternet: IsConnectedToInternet::<Identity, Impl, OFFSET>,
            IsConnected: IsConnected::<Identity, Impl, OFFSET>,
            GetConnectivity: GetConnectivity::<Identity, Impl, OFFSET>,
            GetCategory: GetCategory::<Identity, Impl, OFFSET>,
            SetCategory: SetCategory::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<INetwork as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait INetworkConnection_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn GetNetwork(&self) -> ::windows::core::Result<INetwork>;
    fn IsConnectedToInternet(&self) -> ::windows::core::Result<i16>;
    fn IsConnected(&self) -> ::windows::core::Result<i16>;
    fn GetConnectivity(&self) -> ::windows::core::Result<NLM_CONNECTIVITY>;
    fn GetConnectionId(&self) -> ::windows::core::Result<::windows::core::GUID>;
    fn GetAdapterId(&self) -> ::windows::core::Result<::windows::core::GUID>;
    fn GetDomainType(&self) -> ::windows::core::Result<NLM_DOMAIN_TYPE>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl INetworkConnection_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: INetworkConnection_Impl, const OFFSET: isize>() -> INetworkConnection_Vtbl {
        unsafe extern "system" fn GetNetwork<Identity: ::windows::core::IUnknownImpl, Impl: INetworkConnection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppnetwork: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetNetwork() {
                ::core::result::Result::Ok(ok__) => {
                    *ppnetwork = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsConnectedToInternet<Identity: ::windows::core::IUnknownImpl, Impl: INetworkConnection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbisconnected: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).IsConnectedToInternet() {
                ::core::result::Result::Ok(ok__) => {
                    *pbisconnected = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsConnected<Identity: ::windows::core::IUnknownImpl, Impl: INetworkConnection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbisconnected: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).IsConnected() {
                ::core::result::Result::Ok(ok__) => {
                    *pbisconnected = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetConnectivity<Identity: ::windows::core::IUnknownImpl, Impl: INetworkConnection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pconnectivity: *mut NLM_CONNECTIVITY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetConnectivity() {
                ::core::result::Result::Ok(ok__) => {
                    *pconnectivity = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetConnectionId<Identity: ::windows::core::IUnknownImpl, Impl: INetworkConnection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pgdconnectionid: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetConnectionId() {
                ::core::result::Result::Ok(ok__) => {
                    *pgdconnectionid = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetAdapterId<Identity: ::windows::core::IUnknownImpl, Impl: INetworkConnection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pgdadapterid: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetAdapterId() {
                ::core::result::Result::Ok(ok__) => {
                    *pgdadapterid = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDomainType<Identity: ::windows::core::IUnknownImpl, Impl: INetworkConnection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdomaintype: *mut NLM_DOMAIN_TYPE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetDomainType() {
                ::core::result::Result::Ok(ok__) => {
                    *pdomaintype = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            GetNetwork: GetNetwork::<Identity, Impl, OFFSET>,
            IsConnectedToInternet: IsConnectedToInternet::<Identity, Impl, OFFSET>,
            IsConnected: IsConnected::<Identity, Impl, OFFSET>,
            GetConnectivity: GetConnectivity::<Identity, Impl, OFFSET>,
            GetConnectionId: GetConnectionId::<Identity, Impl, OFFSET>,
            GetAdapterId: GetAdapterId::<Identity, Impl, OFFSET>,
            GetDomainType: GetDomainType::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<INetworkConnection as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait INetworkConnectionCost_Impl: Sized {
    fn GetCost(&self) -> ::windows::core::Result<u32>;
    fn GetDataPlanStatus(&self) -> ::windows::core::Result<NLM_DATAPLAN_STATUS>;
}
#[cfg(feature = "Win32_Foundation")]
impl INetworkConnectionCost_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: INetworkConnectionCost_Impl, const OFFSET: isize>() -> INetworkConnectionCost_Vtbl {
        unsafe extern "system" fn GetCost<Identity: ::windows::core::IUnknownImpl, Impl: INetworkConnectionCost_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcost: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetCost() {
                ::core::result::Result::Ok(ok__) => {
                    *pcost = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDataPlanStatus<Identity: ::windows::core::IUnknownImpl, Impl: INetworkConnectionCost_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdataplanstatus: *mut NLM_DATAPLAN_STATUS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetDataPlanStatus() {
                ::core::result::Result::Ok(ok__) => {
                    *pdataplanstatus = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            GetCost: GetCost::<Identity, Impl, OFFSET>,
            GetDataPlanStatus: GetDataPlanStatus::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<INetworkConnectionCost as ::windows::core::Interface>::IID
    }
}
pub trait INetworkConnectionCostEvents_Impl: Sized {
    fn ConnectionCostChanged(&self, connectionid: &::windows::core::GUID, newcost: u32) -> ::windows::core::Result<()>;
    fn ConnectionDataPlanStatusChanged(&self, connectionid: &::windows::core::GUID) -> ::windows::core::Result<()>;
}
impl INetworkConnectionCostEvents_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: INetworkConnectionCostEvents_Impl, const OFFSET: isize>() -> INetworkConnectionCostEvents_Vtbl {
        unsafe extern "system" fn ConnectionCostChanged<Identity: ::windows::core::IUnknownImpl, Impl: INetworkConnectionCostEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, connectionid: ::windows::core::GUID, newcost: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).ConnectionCostChanged(::core::mem::transmute(&connectionid), ::core::mem::transmute_copy(&newcost)).into()
        }
        unsafe extern "system" fn ConnectionDataPlanStatusChanged<Identity: ::windows::core::IUnknownImpl, Impl: INetworkConnectionCostEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, connectionid: ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).ConnectionDataPlanStatusChanged(::core::mem::transmute(&connectionid)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            ConnectionCostChanged: ConnectionCostChanged::<Identity, Impl, OFFSET>,
            ConnectionDataPlanStatusChanged: ConnectionDataPlanStatusChanged::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<INetworkConnectionCostEvents as ::windows::core::Interface>::IID
    }
}
pub trait INetworkConnectionEvents_Impl: Sized {
    fn NetworkConnectionConnectivityChanged(&self, connectionid: &::windows::core::GUID, newconnectivity: NLM_CONNECTIVITY) -> ::windows::core::Result<()>;
    fn NetworkConnectionPropertyChanged(&self, connectionid: &::windows::core::GUID, flags: NLM_CONNECTION_PROPERTY_CHANGE) -> ::windows::core::Result<()>;
}
impl INetworkConnectionEvents_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: INetworkConnectionEvents_Impl, const OFFSET: isize>() -> INetworkConnectionEvents_Vtbl {
        unsafe extern "system" fn NetworkConnectionConnectivityChanged<Identity: ::windows::core::IUnknownImpl, Impl: INetworkConnectionEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, connectionid: ::windows::core::GUID, newconnectivity: NLM_CONNECTIVITY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).NetworkConnectionConnectivityChanged(::core::mem::transmute(&connectionid), ::core::mem::transmute_copy(&newconnectivity)).into()
        }
        unsafe extern "system" fn NetworkConnectionPropertyChanged<Identity: ::windows::core::IUnknownImpl, Impl: INetworkConnectionEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, connectionid: ::windows::core::GUID, flags: NLM_CONNECTION_PROPERTY_CHANGE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).NetworkConnectionPropertyChanged(::core::mem::transmute(&connectionid), ::core::mem::transmute_copy(&flags)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            NetworkConnectionConnectivityChanged: NetworkConnectionConnectivityChanged::<Identity, Impl, OFFSET>,
            NetworkConnectionPropertyChanged: NetworkConnectionPropertyChanged::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<INetworkConnectionEvents as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait INetworkCostManager_Impl: Sized {
    fn GetCost(&self, pcost: *mut u32, pdestipaddr: *const NLM_SOCKADDR) -> ::windows::core::Result<()>;
    fn GetDataPlanStatus(&self, pdataplanstatus: *mut NLM_DATAPLAN_STATUS, pdestipaddr: *const NLM_SOCKADDR) -> ::windows::core::Result<()>;
    fn SetDestinationAddresses(&self, length: u32, pdestipaddrlist: *const NLM_SOCKADDR, bappend: i16) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl INetworkCostManager_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: INetworkCostManager_Impl, const OFFSET: isize>() -> INetworkCostManager_Vtbl {
        unsafe extern "system" fn GetCost<Identity: ::windows::core::IUnknownImpl, Impl: INetworkCostManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcost: *mut u32, pdestipaddr: *const NLM_SOCKADDR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetCost(::core::mem::transmute_copy(&pcost), ::core::mem::transmute_copy(&pdestipaddr)).into()
        }
        unsafe extern "system" fn GetDataPlanStatus<Identity: ::windows::core::IUnknownImpl, Impl: INetworkCostManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdataplanstatus: *mut NLM_DATAPLAN_STATUS, pdestipaddr: *const NLM_SOCKADDR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetDataPlanStatus(::core::mem::transmute_copy(&pdataplanstatus), ::core::mem::transmute_copy(&pdestipaddr)).into()
        }
        unsafe extern "system" fn SetDestinationAddresses<Identity: ::windows::core::IUnknownImpl, Impl: INetworkCostManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, length: u32, pdestipaddrlist: *const NLM_SOCKADDR, bappend: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetDestinationAddresses(::core::mem::transmute_copy(&length), ::core::mem::transmute_copy(&pdestipaddrlist), ::core::mem::transmute_copy(&bappend)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            GetCost: GetCost::<Identity, Impl, OFFSET>,
            GetDataPlanStatus: GetDataPlanStatus::<Identity, Impl, OFFSET>,
            SetDestinationAddresses: SetDestinationAddresses::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<INetworkCostManager as ::windows::core::Interface>::IID
    }
}
pub trait INetworkCostManagerEvents_Impl: Sized {
    fn CostChanged(&self, newcost: u32, pdestaddr: *const NLM_SOCKADDR) -> ::windows::core::Result<()>;
    fn DataPlanStatusChanged(&self, pdestaddr: *const NLM_SOCKADDR) -> ::windows::core::Result<()>;
}
impl INetworkCostManagerEvents_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: INetworkCostManagerEvents_Impl, const OFFSET: isize>() -> INetworkCostManagerEvents_Vtbl {
        unsafe extern "system" fn CostChanged<Identity: ::windows::core::IUnknownImpl, Impl: INetworkCostManagerEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newcost: u32, pdestaddr: *const NLM_SOCKADDR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).CostChanged(::core::mem::transmute_copy(&newcost), ::core::mem::transmute_copy(&pdestaddr)).into()
        }
        unsafe extern "system" fn DataPlanStatusChanged<Identity: ::windows::core::IUnknownImpl, Impl: INetworkCostManagerEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdestaddr: *const NLM_SOCKADDR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).DataPlanStatusChanged(::core::mem::transmute_copy(&pdestaddr)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            CostChanged: CostChanged::<Identity, Impl, OFFSET>,
            DataPlanStatusChanged: DataPlanStatusChanged::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<INetworkCostManagerEvents as ::windows::core::Interface>::IID
    }
}
pub trait INetworkEvents_Impl: Sized {
    fn NetworkAdded(&self, networkid: &::windows::core::GUID) -> ::windows::core::Result<()>;
    fn NetworkDeleted(&self, networkid: &::windows::core::GUID) -> ::windows::core::Result<()>;
    fn NetworkConnectivityChanged(&self, networkid: &::windows::core::GUID, newconnectivity: NLM_CONNECTIVITY) -> ::windows::core::Result<()>;
    fn NetworkPropertyChanged(&self, networkid: &::windows::core::GUID, flags: NLM_NETWORK_PROPERTY_CHANGE) -> ::windows::core::Result<()>;
}
impl INetworkEvents_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: INetworkEvents_Impl, const OFFSET: isize>() -> INetworkEvents_Vtbl {
        unsafe extern "system" fn NetworkAdded<Identity: ::windows::core::IUnknownImpl, Impl: INetworkEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, networkid: ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).NetworkAdded(::core::mem::transmute(&networkid)).into()
        }
        unsafe extern "system" fn NetworkDeleted<Identity: ::windows::core::IUnknownImpl, Impl: INetworkEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, networkid: ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).NetworkDeleted(::core::mem::transmute(&networkid)).into()
        }
        unsafe extern "system" fn NetworkConnectivityChanged<Identity: ::windows::core::IUnknownImpl, Impl: INetworkEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, networkid: ::windows::core::GUID, newconnectivity: NLM_CONNECTIVITY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).NetworkConnectivityChanged(::core::mem::transmute(&networkid), ::core::mem::transmute_copy(&newconnectivity)).into()
        }
        unsafe extern "system" fn NetworkPropertyChanged<Identity: ::windows::core::IUnknownImpl, Impl: INetworkEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, networkid: ::windows::core::GUID, flags: NLM_NETWORK_PROPERTY_CHANGE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).NetworkPropertyChanged(::core::mem::transmute(&networkid), ::core::mem::transmute_copy(&flags)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            NetworkAdded: NetworkAdded::<Identity, Impl, OFFSET>,
            NetworkDeleted: NetworkDeleted::<Identity, Impl, OFFSET>,
            NetworkConnectivityChanged: NetworkConnectivityChanged::<Identity, Impl, OFFSET>,
            NetworkPropertyChanged: NetworkPropertyChanged::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<INetworkEvents as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait INetworkListManager_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn GetNetworks(&self, flags: NLM_ENUM_NETWORK) -> ::windows::core::Result<IEnumNetworks>;
    fn GetNetwork(&self, gdnetworkid: &::windows::core::GUID) -> ::windows::core::Result<INetwork>;
    fn GetNetworkConnections(&self) -> ::windows::core::Result<IEnumNetworkConnections>;
    fn GetNetworkConnection(&self, gdnetworkconnectionid: &::windows::core::GUID) -> ::windows::core::Result<INetworkConnection>;
    fn IsConnectedToInternet(&self) -> ::windows::core::Result<i16>;
    fn IsConnected(&self) -> ::windows::core::Result<i16>;
    fn GetConnectivity(&self) -> ::windows::core::Result<NLM_CONNECTIVITY>;
    fn SetSimulatedProfileInfo(&self, psimulatedinfo: *const NLM_SIMULATED_PROFILE_INFO) -> ::windows::core::Result<()>;
    fn ClearSimulatedProfileInfo(&self) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl INetworkListManager_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: INetworkListManager_Impl, const OFFSET: isize>() -> INetworkListManager_Vtbl {
        unsafe extern "system" fn GetNetworks<Identity: ::windows::core::IUnknownImpl, Impl: INetworkListManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, flags: NLM_ENUM_NETWORK, ppenumnetwork: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetNetworks(::core::mem::transmute_copy(&flags)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppenumnetwork = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetNetwork<Identity: ::windows::core::IUnknownImpl, Impl: INetworkListManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, gdnetworkid: ::windows::core::GUID, ppnetwork: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetNetwork(::core::mem::transmute(&gdnetworkid)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppnetwork = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetNetworkConnections<Identity: ::windows::core::IUnknownImpl, Impl: INetworkListManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetNetworkConnections() {
                ::core::result::Result::Ok(ok__) => {
                    *ppenum = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetNetworkConnection<Identity: ::windows::core::IUnknownImpl, Impl: INetworkListManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, gdnetworkconnectionid: ::windows::core::GUID, ppnetworkconnection: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetNetworkConnection(::core::mem::transmute(&gdnetworkconnectionid)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppnetworkconnection = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsConnectedToInternet<Identity: ::windows::core::IUnknownImpl, Impl: INetworkListManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbisconnected: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).IsConnectedToInternet() {
                ::core::result::Result::Ok(ok__) => {
                    *pbisconnected = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsConnected<Identity: ::windows::core::IUnknownImpl, Impl: INetworkListManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbisconnected: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).IsConnected() {
                ::core::result::Result::Ok(ok__) => {
                    *pbisconnected = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetConnectivity<Identity: ::windows::core::IUnknownImpl, Impl: INetworkListManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pconnectivity: *mut NLM_CONNECTIVITY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetConnectivity() {
                ::core::result::Result::Ok(ok__) => {
                    *pconnectivity = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSimulatedProfileInfo<Identity: ::windows::core::IUnknownImpl, Impl: INetworkListManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psimulatedinfo: *const NLM_SIMULATED_PROFILE_INFO) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetSimulatedProfileInfo(::core::mem::transmute_copy(&psimulatedinfo)).into()
        }
        unsafe extern "system" fn ClearSimulatedProfileInfo<Identity: ::windows::core::IUnknownImpl, Impl: INetworkListManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).ClearSimulatedProfileInfo().into()
        }
        Self {
            base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            GetNetworks: GetNetworks::<Identity, Impl, OFFSET>,
            GetNetwork: GetNetwork::<Identity, Impl, OFFSET>,
            GetNetworkConnections: GetNetworkConnections::<Identity, Impl, OFFSET>,
            GetNetworkConnection: GetNetworkConnection::<Identity, Impl, OFFSET>,
            IsConnectedToInternet: IsConnectedToInternet::<Identity, Impl, OFFSET>,
            IsConnected: IsConnected::<Identity, Impl, OFFSET>,
            GetConnectivity: GetConnectivity::<Identity, Impl, OFFSET>,
            SetSimulatedProfileInfo: SetSimulatedProfileInfo::<Identity, Impl, OFFSET>,
            ClearSimulatedProfileInfo: ClearSimulatedProfileInfo::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<INetworkListManager as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
pub trait INetworkListManagerEvents_Impl: Sized {
    fn ConnectivityChanged(&self, newconnectivity: NLM_CONNECTIVITY) -> ::windows::core::Result<()>;
}
impl INetworkListManagerEvents_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: INetworkListManagerEvents_Impl, const OFFSET: isize>() -> INetworkListManagerEvents_Vtbl {
        unsafe extern "system" fn ConnectivityChanged<Identity: ::windows::core::IUnknownImpl, Impl: INetworkListManagerEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newconnectivity: NLM_CONNECTIVITY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).ConnectivityChanged(::core::mem::transmute_copy(&newconnectivity)).into()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(), ConnectivityChanged: ConnectivityChanged::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<INetworkListManagerEvents as ::windows::core::Interface>::IID
    }
}
