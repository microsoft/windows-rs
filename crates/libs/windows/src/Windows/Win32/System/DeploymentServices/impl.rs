#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IWdsTransportCacheable_Impl: Sized + super::Com::IDispatch_Impl {
    fn Dirty(&self) -> ::windows::core::Result<i16>;
    fn Discard(&self) -> ::windows::core::Result<()>;
    fn Refresh(&self) -> ::windows::core::Result<()>;
    fn Commit(&self) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IWdsTransportCacheable_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWdsTransportCacheable_Impl, const OFFSET: isize>() -> IWdsTransportCacheable_Vtbl {
        unsafe extern "system" fn Dirty<Identity: ::windows::core::IUnknownImpl, Impl: IWdsTransportCacheable_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbdirty: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Dirty() {
                ::core::result::Result::Ok(ok__) => {
                    *pbdirty = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Discard<Identity: ::windows::core::IUnknownImpl, Impl: IWdsTransportCacheable_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Discard().into()
        }
        unsafe extern "system" fn Refresh<Identity: ::windows::core::IUnknownImpl, Impl: IWdsTransportCacheable_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Refresh().into()
        }
        unsafe extern "system" fn Commit<Identity: ::windows::core::IUnknownImpl, Impl: IWdsTransportCacheable_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Commit().into()
        }
        Self {
            base: super::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            Dirty: Dirty::<Identity, Impl, OFFSET>,
            Discard: Discard::<Identity, Impl, OFFSET>,
            Refresh: Refresh::<Identity, Impl, OFFSET>,
            Commit: Commit::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWdsTransportCacheable as ::windows::core::Interface>::IID || iid == &<super::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IWdsTransportClient_Impl: Sized + super::Com::IDispatch_Impl {
    fn Session(&self) -> ::windows::core::Result<IWdsTransportSession>;
    fn Id(&self) -> ::windows::core::Result<u32>;
    fn Name(&self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn MacAddress(&self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn IpAddress(&self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn PercentCompletion(&self) -> ::windows::core::Result<u32>;
    fn JoinDuration(&self) -> ::windows::core::Result<u32>;
    fn CpuUtilization(&self) -> ::windows::core::Result<u32>;
    fn MemoryUtilization(&self) -> ::windows::core::Result<u32>;
    fn NetworkUtilization(&self) -> ::windows::core::Result<u32>;
    fn UserIdentity(&self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn Disconnect(&self, disconnectiontype: WDSTRANSPORT_DISCONNECT_TYPE) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IWdsTransportClient_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWdsTransportClient_Impl, const OFFSET: isize>() -> IWdsTransportClient_Vtbl {
        unsafe extern "system" fn Session<Identity: ::windows::core::IUnknownImpl, Impl: IWdsTransportClient_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppwdstransportsession: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Session() {
                ::core::result::Result::Ok(ok__) => {
                    *ppwdstransportsession = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Id<Identity: ::windows::core::IUnknownImpl, Impl: IWdsTransportClient_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pulid: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Id() {
                ::core::result::Result::Ok(ok__) => {
                    *pulid = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Name<Identity: ::windows::core::IUnknownImpl, Impl: IWdsTransportClient_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbszname: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Name() {
                ::core::result::Result::Ok(ok__) => {
                    *pbszname = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MacAddress<Identity: ::windows::core::IUnknownImpl, Impl: IWdsTransportClient_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbszmacaddress: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).MacAddress() {
                ::core::result::Result::Ok(ok__) => {
                    *pbszmacaddress = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IpAddress<Identity: ::windows::core::IUnknownImpl, Impl: IWdsTransportClient_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbszipaddress: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).IpAddress() {
                ::core::result::Result::Ok(ok__) => {
                    *pbszipaddress = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PercentCompletion<Identity: ::windows::core::IUnknownImpl, Impl: IWdsTransportClient_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pulpercentcompletion: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).PercentCompletion() {
                ::core::result::Result::Ok(ok__) => {
                    *pulpercentcompletion = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn JoinDuration<Identity: ::windows::core::IUnknownImpl, Impl: IWdsTransportClient_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, puljoinduration: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).JoinDuration() {
                ::core::result::Result::Ok(ok__) => {
                    *puljoinduration = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CpuUtilization<Identity: ::windows::core::IUnknownImpl, Impl: IWdsTransportClient_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pulcpuutilization: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CpuUtilization() {
                ::core::result::Result::Ok(ok__) => {
                    *pulcpuutilization = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MemoryUtilization<Identity: ::windows::core::IUnknownImpl, Impl: IWdsTransportClient_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pulmemoryutilization: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).MemoryUtilization() {
                ::core::result::Result::Ok(ok__) => {
                    *pulmemoryutilization = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn NetworkUtilization<Identity: ::windows::core::IUnknownImpl, Impl: IWdsTransportClient_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pulnetworkutilization: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).NetworkUtilization() {
                ::core::result::Result::Ok(ok__) => {
                    *pulnetworkutilization = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UserIdentity<Identity: ::windows::core::IUnknownImpl, Impl: IWdsTransportClient_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbszuseridentity: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).UserIdentity() {
                ::core::result::Result::Ok(ok__) => {
                    *pbszuseridentity = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Disconnect<Identity: ::windows::core::IUnknownImpl, Impl: IWdsTransportClient_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, disconnectiontype: WDSTRANSPORT_DISCONNECT_TYPE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Disconnect(::core::mem::transmute_copy(&disconnectiontype)).into()
        }
        Self {
            base: super::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            Session: Session::<Identity, Impl, OFFSET>,
            Id: Id::<Identity, Impl, OFFSET>,
            Name: Name::<Identity, Impl, OFFSET>,
            MacAddress: MacAddress::<Identity, Impl, OFFSET>,
            IpAddress: IpAddress::<Identity, Impl, OFFSET>,
            PercentCompletion: PercentCompletion::<Identity, Impl, OFFSET>,
            JoinDuration: JoinDuration::<Identity, Impl, OFFSET>,
            CpuUtilization: CpuUtilization::<Identity, Impl, OFFSET>,
            MemoryUtilization: MemoryUtilization::<Identity, Impl, OFFSET>,
            NetworkUtilization: NetworkUtilization::<Identity, Impl, OFFSET>,
            UserIdentity: UserIdentity::<Identity, Impl, OFFSET>,
            Disconnect: Disconnect::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWdsTransportClient as ::windows::core::Interface>::IID || iid == &<super::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IWdsTransportCollection_Impl: Sized + super::Com::IDispatch_Impl {
    fn Count(&self) -> ::windows::core::Result<u32>;
    fn Item(&self, ulindex: u32) -> ::windows::core::Result<super::Com::IDispatch>;
    fn _NewEnum(&self) -> ::windows::core::Result<::windows::core::IUnknown>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IWdsTransportCollection_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWdsTransportCollection_Impl, const OFFSET: isize>() -> IWdsTransportCollection_Vtbl {
        unsafe extern "system" fn Count<Identity: ::windows::core::IUnknownImpl, Impl: IWdsTransportCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pulcount: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Count() {
                ::core::result::Result::Ok(ok__) => {
                    *pulcount = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Item<Identity: ::windows::core::IUnknownImpl, Impl: IWdsTransportCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ulindex: u32, ppval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Item(::core::mem::transmute_copy(&ulindex)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn _NewEnum<Identity: ::windows::core::IUnknownImpl, Impl: IWdsTransportCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppval: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this)._NewEnum() {
                ::core::result::Result::Ok(ok__) => {
                    *ppval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: super::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            Count: Count::<Identity, Impl, OFFSET>,
            Item: Item::<Identity, Impl, OFFSET>,
            _NewEnum: _NewEnum::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWdsTransportCollection as ::windows::core::Interface>::IID || iid == &<super::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IWdsTransportConfigurationManager_Impl: Sized + super::Com::IDispatch_Impl {
    fn ServicePolicy(&self) -> ::windows::core::Result<IWdsTransportServicePolicy>;
    fn DiagnosticsPolicy(&self) -> ::windows::core::Result<IWdsTransportDiagnosticsPolicy>;
    fn WdsTransportServicesRunning(&self, brealtimestatus: i16) -> ::windows::core::Result<i16>;
    fn EnableWdsTransportServices(&self) -> ::windows::core::Result<()>;
    fn DisableWdsTransportServices(&self) -> ::windows::core::Result<()>;
    fn StartWdsTransportServices(&self) -> ::windows::core::Result<()>;
    fn StopWdsTransportServices(&self) -> ::windows::core::Result<()>;
    fn RestartWdsTransportServices(&self) -> ::windows::core::Result<()>;
    fn NotifyWdsTransportServices(&self, servicenotification: WDSTRANSPORT_SERVICE_NOTIFICATION) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IWdsTransportConfigurationManager_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWdsTransportConfigurationManager_Impl, const OFFSET: isize>() -> IWdsTransportConfigurationManager_Vtbl {
        unsafe extern "system" fn ServicePolicy<Identity: ::windows::core::IUnknownImpl, Impl: IWdsTransportConfigurationManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppwdstransportservicepolicy: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).ServicePolicy() {
                ::core::result::Result::Ok(ok__) => {
                    *ppwdstransportservicepolicy = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DiagnosticsPolicy<Identity: ::windows::core::IUnknownImpl, Impl: IWdsTransportConfigurationManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppwdstransportdiagnosticspolicy: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).DiagnosticsPolicy() {
                ::core::result::Result::Ok(ok__) => {
                    *ppwdstransportdiagnosticspolicy = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn WdsTransportServicesRunning<Identity: ::windows::core::IUnknownImpl, Impl: IWdsTransportConfigurationManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, brealtimestatus: i16, pbservicesrunning: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).WdsTransportServicesRunning(::core::mem::transmute_copy(&brealtimestatus)) {
                ::core::result::Result::Ok(ok__) => {
                    *pbservicesrunning = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnableWdsTransportServices<Identity: ::windows::core::IUnknownImpl, Impl: IWdsTransportConfigurationManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).EnableWdsTransportServices().into()
        }
        unsafe extern "system" fn DisableWdsTransportServices<Identity: ::windows::core::IUnknownImpl, Impl: IWdsTransportConfigurationManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).DisableWdsTransportServices().into()
        }
        unsafe extern "system" fn StartWdsTransportServices<Identity: ::windows::core::IUnknownImpl, Impl: IWdsTransportConfigurationManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).StartWdsTransportServices().into()
        }
        unsafe extern "system" fn StopWdsTransportServices<Identity: ::windows::core::IUnknownImpl, Impl: IWdsTransportConfigurationManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).StopWdsTransportServices().into()
        }
        unsafe extern "system" fn RestartWdsTransportServices<Identity: ::windows::core::IUnknownImpl, Impl: IWdsTransportConfigurationManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).RestartWdsTransportServices().into()
        }
        unsafe extern "system" fn NotifyWdsTransportServices<Identity: ::windows::core::IUnknownImpl, Impl: IWdsTransportConfigurationManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, servicenotification: WDSTRANSPORT_SERVICE_NOTIFICATION) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).NotifyWdsTransportServices(::core::mem::transmute_copy(&servicenotification)).into()
        }
        Self {
            base: super::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            ServicePolicy: ServicePolicy::<Identity, Impl, OFFSET>,
            DiagnosticsPolicy: DiagnosticsPolicy::<Identity, Impl, OFFSET>,
            WdsTransportServicesRunning: WdsTransportServicesRunning::<Identity, Impl, OFFSET>,
            EnableWdsTransportServices: EnableWdsTransportServices::<Identity, Impl, OFFSET>,
            DisableWdsTransportServices: DisableWdsTransportServices::<Identity, Impl, OFFSET>,
            StartWdsTransportServices: StartWdsTransportServices::<Identity, Impl, OFFSET>,
            StopWdsTransportServices: StopWdsTransportServices::<Identity, Impl, OFFSET>,
            RestartWdsTransportServices: RestartWdsTransportServices::<Identity, Impl, OFFSET>,
            NotifyWdsTransportServices: NotifyWdsTransportServices::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWdsTransportConfigurationManager as ::windows::core::Interface>::IID || iid == &<super::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IWdsTransportConfigurationManager2_Impl: Sized + super::Com::IDispatch_Impl + IWdsTransportConfigurationManager_Impl {
    fn MulticastSessionPolicy(&self) -> ::windows::core::Result<IWdsTransportMulticastSessionPolicy>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IWdsTransportConfigurationManager2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWdsTransportConfigurationManager2_Impl, const OFFSET: isize>() -> IWdsTransportConfigurationManager2_Vtbl {
        unsafe extern "system" fn MulticastSessionPolicy<Identity: ::windows::core::IUnknownImpl, Impl: IWdsTransportConfigurationManager2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppwdstransportmulticastsessionpolicy: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).MulticastSessionPolicy() {
                ::core::result::Result::Ok(ok__) => {
                    *ppwdstransportmulticastsessionpolicy = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: IWdsTransportConfigurationManager_Vtbl::new::<Identity, Impl, OFFSET>(),
            MulticastSessionPolicy: MulticastSessionPolicy::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWdsTransportConfigurationManager2 as ::windows::core::Interface>::IID || iid == &<super::Com::IDispatch as ::windows::core::Interface>::IID || iid == &<IWdsTransportConfigurationManager as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IWdsTransportContent_Impl: Sized + super::Com::IDispatch_Impl {
    fn Namespace(&self) -> ::windows::core::Result<IWdsTransportNamespace>;
    fn Id(&self) -> ::windows::core::Result<u32>;
    fn Name(&self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn RetrieveSessions(&self) -> ::windows::core::Result<IWdsTransportCollection>;
    fn Terminate(&self) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IWdsTransportContent_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWdsTransportContent_Impl, const OFFSET: isize>() -> IWdsTransportContent_Vtbl {
        unsafe extern "system" fn Namespace<Identity: ::windows::core::IUnknownImpl, Impl: IWdsTransportContent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppwdstransportnamespace: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Namespace() {
                ::core::result::Result::Ok(ok__) => {
                    *ppwdstransportnamespace = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Id<Identity: ::windows::core::IUnknownImpl, Impl: IWdsTransportContent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pulid: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Id() {
                ::core::result::Result::Ok(ok__) => {
                    *pulid = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Name<Identity: ::windows::core::IUnknownImpl, Impl: IWdsTransportContent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbszname: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Name() {
                ::core::result::Result::Ok(ok__) => {
                    *pbszname = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RetrieveSessions<Identity: ::windows::core::IUnknownImpl, Impl: IWdsTransportContent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppwdstransportsessions: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).RetrieveSessions() {
                ::core::result::Result::Ok(ok__) => {
                    *ppwdstransportsessions = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Terminate<Identity: ::windows::core::IUnknownImpl, Impl: IWdsTransportContent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Terminate().into()
        }
        Self {
            base: super::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            Namespace: Namespace::<Identity, Impl, OFFSET>,
            Id: Id::<Identity, Impl, OFFSET>,
            Name: Name::<Identity, Impl, OFFSET>,
            RetrieveSessions: RetrieveSessions::<Identity, Impl, OFFSET>,
            Terminate: Terminate::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWdsTransportContent as ::windows::core::Interface>::IID || iid == &<super::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IWdsTransportContentProvider_Impl: Sized + super::Com::IDispatch_Impl {
    fn Name(&self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn Description(&self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn FilePath(&self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn InitializationRoutine(&self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IWdsTransportContentProvider_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWdsTransportContentProvider_Impl, const OFFSET: isize>() -> IWdsTransportContentProvider_Vtbl {
        unsafe extern "system" fn Name<Identity: ::windows::core::IUnknownImpl, Impl: IWdsTransportContentProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbszname: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Name() {
                ::core::result::Result::Ok(ok__) => {
                    *pbszname = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Description<Identity: ::windows::core::IUnknownImpl, Impl: IWdsTransportContentProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbszdescription: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Description() {
                ::core::result::Result::Ok(ok__) => {
                    *pbszdescription = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FilePath<Identity: ::windows::core::IUnknownImpl, Impl: IWdsTransportContentProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbszfilepath: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).FilePath() {
                ::core::result::Result::Ok(ok__) => {
                    *pbszfilepath = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn InitializationRoutine<Identity: ::windows::core::IUnknownImpl, Impl: IWdsTransportContentProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbszinitializationroutine: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).InitializationRoutine() {
                ::core::result::Result::Ok(ok__) => {
                    *pbszinitializationroutine = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: super::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            Name: Name::<Identity, Impl, OFFSET>,
            Description: Description::<Identity, Impl, OFFSET>,
            FilePath: FilePath::<Identity, Impl, OFFSET>,
            InitializationRoutine: InitializationRoutine::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWdsTransportContentProvider as ::windows::core::Interface>::IID || iid == &<super::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IWdsTransportDiagnosticsPolicy_Impl: Sized + super::Com::IDispatch_Impl + IWdsTransportCacheable_Impl {
    fn Enabled(&self) -> ::windows::core::Result<i16>;
    fn SetEnabled(&self, benabled: i16) -> ::windows::core::Result<()>;
    fn Components(&self) -> ::windows::core::Result<u32>;
    fn SetComponents(&self, ulcomponents: u32) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IWdsTransportDiagnosticsPolicy_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWdsTransportDiagnosticsPolicy_Impl, const OFFSET: isize>() -> IWdsTransportDiagnosticsPolicy_Vtbl {
        unsafe extern "system" fn Enabled<Identity: ::windows::core::IUnknownImpl, Impl: IWdsTransportDiagnosticsPolicy_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbenabled: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Enabled() {
                ::core::result::Result::Ok(ok__) => {
                    *pbenabled = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetEnabled<Identity: ::windows::core::IUnknownImpl, Impl: IWdsTransportDiagnosticsPolicy_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, benabled: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetEnabled(::core::mem::transmute_copy(&benabled)).into()
        }
        unsafe extern "system" fn Components<Identity: ::windows::core::IUnknownImpl, Impl: IWdsTransportDiagnosticsPolicy_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pulcomponents: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Components() {
                ::core::result::Result::Ok(ok__) => {
                    *pulcomponents = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetComponents<Identity: ::windows::core::IUnknownImpl, Impl: IWdsTransportDiagnosticsPolicy_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ulcomponents: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetComponents(::core::mem::transmute_copy(&ulcomponents)).into()
        }
        Self {
            base: IWdsTransportCacheable_Vtbl::new::<Identity, Impl, OFFSET>(),
            Enabled: Enabled::<Identity, Impl, OFFSET>,
            SetEnabled: SetEnabled::<Identity, Impl, OFFSET>,
            Components: Components::<Identity, Impl, OFFSET>,
            SetComponents: SetComponents::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWdsTransportDiagnosticsPolicy as ::windows::core::Interface>::IID || iid == &<super::Com::IDispatch as ::windows::core::Interface>::IID || iid == &<IWdsTransportCacheable as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IWdsTransportManager_Impl: Sized + super::Com::IDispatch_Impl {
    fn GetWdsTransportServer(&self, bszservername: &super::super::Foundation::BSTR) -> ::windows::core::Result<IWdsTransportServer>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IWdsTransportManager_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWdsTransportManager_Impl, const OFFSET: isize>() -> IWdsTransportManager_Vtbl {
        unsafe extern "system" fn GetWdsTransportServer<Identity: ::windows::core::IUnknownImpl, Impl: IWdsTransportManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bszservername: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ppwdstransportserver: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetWdsTransportServer(::core::mem::transmute(&bszservername)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppwdstransportserver = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: super::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(), GetWdsTransportServer: GetWdsTransportServer::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWdsTransportManager as ::windows::core::Interface>::IID || iid == &<super::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IWdsTransportMulticastSessionPolicy_Impl: Sized + super::Com::IDispatch_Impl + IWdsTransportCacheable_Impl {
    fn SlowClientHandling(&self) -> ::windows::core::Result<WDSTRANSPORT_SLOW_CLIENT_HANDLING_TYPE>;
    fn SetSlowClientHandling(&self, slowclienthandling: WDSTRANSPORT_SLOW_CLIENT_HANDLING_TYPE) -> ::windows::core::Result<()>;
    fn AutoDisconnectThreshold(&self) -> ::windows::core::Result<u32>;
    fn SetAutoDisconnectThreshold(&self, ulthreshold: u32) -> ::windows::core::Result<()>;
    fn MultistreamStreamCount(&self) -> ::windows::core::Result<u32>;
    fn SetMultistreamStreamCount(&self, ulstreamcount: u32) -> ::windows::core::Result<()>;
    fn SlowClientFallback(&self) -> ::windows::core::Result<i16>;
    fn SetSlowClientFallback(&self, bclientfallback: i16) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IWdsTransportMulticastSessionPolicy_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWdsTransportMulticastSessionPolicy_Impl, const OFFSET: isize>() -> IWdsTransportMulticastSessionPolicy_Vtbl {
        unsafe extern "system" fn SlowClientHandling<Identity: ::windows::core::IUnknownImpl, Impl: IWdsTransportMulticastSessionPolicy_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pslowclienthandling: *mut WDSTRANSPORT_SLOW_CLIENT_HANDLING_TYPE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).SlowClientHandling() {
                ::core::result::Result::Ok(ok__) => {
                    *pslowclienthandling = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSlowClientHandling<Identity: ::windows::core::IUnknownImpl, Impl: IWdsTransportMulticastSessionPolicy_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, slowclienthandling: WDSTRANSPORT_SLOW_CLIENT_HANDLING_TYPE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetSlowClientHandling(::core::mem::transmute_copy(&slowclienthandling)).into()
        }
        unsafe extern "system" fn AutoDisconnectThreshold<Identity: ::windows::core::IUnknownImpl, Impl: IWdsTransportMulticastSessionPolicy_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pulthreshold: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).AutoDisconnectThreshold() {
                ::core::result::Result::Ok(ok__) => {
                    *pulthreshold = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAutoDisconnectThreshold<Identity: ::windows::core::IUnknownImpl, Impl: IWdsTransportMulticastSessionPolicy_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ulthreshold: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetAutoDisconnectThreshold(::core::mem::transmute_copy(&ulthreshold)).into()
        }
        unsafe extern "system" fn MultistreamStreamCount<Identity: ::windows::core::IUnknownImpl, Impl: IWdsTransportMulticastSessionPolicy_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pulstreamcount: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).MultistreamStreamCount() {
                ::core::result::Result::Ok(ok__) => {
                    *pulstreamcount = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMultistreamStreamCount<Identity: ::windows::core::IUnknownImpl, Impl: IWdsTransportMulticastSessionPolicy_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ulstreamcount: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetMultistreamStreamCount(::core::mem::transmute_copy(&ulstreamcount)).into()
        }
        unsafe extern "system" fn SlowClientFallback<Identity: ::windows::core::IUnknownImpl, Impl: IWdsTransportMulticastSessionPolicy_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbclientfallback: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).SlowClientFallback() {
                ::core::result::Result::Ok(ok__) => {
                    *pbclientfallback = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSlowClientFallback<Identity: ::windows::core::IUnknownImpl, Impl: IWdsTransportMulticastSessionPolicy_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bclientfallback: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetSlowClientFallback(::core::mem::transmute_copy(&bclientfallback)).into()
        }
        Self {
            base: IWdsTransportCacheable_Vtbl::new::<Identity, Impl, OFFSET>(),
            SlowClientHandling: SlowClientHandling::<Identity, Impl, OFFSET>,
            SetSlowClientHandling: SetSlowClientHandling::<Identity, Impl, OFFSET>,
            AutoDisconnectThreshold: AutoDisconnectThreshold::<Identity, Impl, OFFSET>,
            SetAutoDisconnectThreshold: SetAutoDisconnectThreshold::<Identity, Impl, OFFSET>,
            MultistreamStreamCount: MultistreamStreamCount::<Identity, Impl, OFFSET>,
            SetMultistreamStreamCount: SetMultistreamStreamCount::<Identity, Impl, OFFSET>,
            SlowClientFallback: SlowClientFallback::<Identity, Impl, OFFSET>,
            SetSlowClientFallback: SetSlowClientFallback::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWdsTransportMulticastSessionPolicy as ::windows::core::Interface>::IID || iid == &<super::Com::IDispatch as ::windows::core::Interface>::IID || iid == &<IWdsTransportCacheable as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IWdsTransportNamespace_Impl: Sized + super::Com::IDispatch_Impl {
    fn Type(&self) -> ::windows::core::Result<WDSTRANSPORT_NAMESPACE_TYPE>;
    fn Id(&self) -> ::windows::core::Result<u32>;
    fn Name(&self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetName(&self, bszname: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn FriendlyName(&self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetFriendlyName(&self, bszfriendlyname: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn Description(&self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetDescription(&self, bszdescription: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn ContentProvider(&self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetContentProvider(&self, bszcontentprovider: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn Configuration(&self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetConfiguration(&self, bszconfiguration: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn Registered(&self) -> ::windows::core::Result<i16>;
    fn Tombstoned(&self) -> ::windows::core::Result<i16>;
    fn TombstoneTime(&self) -> ::windows::core::Result<f64>;
    fn TransmissionStarted(&self) -> ::windows::core::Result<i16>;
    fn Register(&self) -> ::windows::core::Result<()>;
    fn Deregister(&self, bterminatesessions: i16) -> ::windows::core::Result<()>;
    fn Clone(&self) -> ::windows::core::Result<IWdsTransportNamespace>;
    fn Refresh(&self) -> ::windows::core::Result<()>;
    fn RetrieveContents(&self) -> ::windows::core::Result<IWdsTransportCollection>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IWdsTransportNamespace_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWdsTransportNamespace_Impl, const OFFSET: isize>() -> IWdsTransportNamespace_Vtbl {
        unsafe extern "system" fn Type<Identity: ::windows::core::IUnknownImpl, Impl: IWdsTransportNamespace_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ptype: *mut WDSTRANSPORT_NAMESPACE_TYPE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Type() {
                ::core::result::Result::Ok(ok__) => {
                    *ptype = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Id<Identity: ::windows::core::IUnknownImpl, Impl: IWdsTransportNamespace_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pulid: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Id() {
                ::core::result::Result::Ok(ok__) => {
                    *pulid = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Name<Identity: ::windows::core::IUnknownImpl, Impl: IWdsTransportNamespace_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbszname: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Name() {
                ::core::result::Result::Ok(ok__) => {
                    *pbszname = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetName<Identity: ::windows::core::IUnknownImpl, Impl: IWdsTransportNamespace_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bszname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetName(::core::mem::transmute(&bszname)).into()
        }
        unsafe extern "system" fn FriendlyName<Identity: ::windows::core::IUnknownImpl, Impl: IWdsTransportNamespace_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbszfriendlyname: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).FriendlyName() {
                ::core::result::Result::Ok(ok__) => {
                    *pbszfriendlyname = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetFriendlyName<Identity: ::windows::core::IUnknownImpl, Impl: IWdsTransportNamespace_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bszfriendlyname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetFriendlyName(::core::mem::transmute(&bszfriendlyname)).into()
        }
        unsafe extern "system" fn Description<Identity: ::windows::core::IUnknownImpl, Impl: IWdsTransportNamespace_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbszdescription: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Description() {
                ::core::result::Result::Ok(ok__) => {
                    *pbszdescription = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDescription<Identity: ::windows::core::IUnknownImpl, Impl: IWdsTransportNamespace_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bszdescription: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetDescription(::core::mem::transmute(&bszdescription)).into()
        }
        unsafe extern "system" fn ContentProvider<Identity: ::windows::core::IUnknownImpl, Impl: IWdsTransportNamespace_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbszcontentprovider: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).ContentProvider() {
                ::core::result::Result::Ok(ok__) => {
                    *pbszcontentprovider = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetContentProvider<Identity: ::windows::core::IUnknownImpl, Impl: IWdsTransportNamespace_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bszcontentprovider: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetContentProvider(::core::mem::transmute(&bszcontentprovider)).into()
        }
        unsafe extern "system" fn Configuration<Identity: ::windows::core::IUnknownImpl, Impl: IWdsTransportNamespace_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbszconfiguration: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Configuration() {
                ::core::result::Result::Ok(ok__) => {
                    *pbszconfiguration = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetConfiguration<Identity: ::windows::core::IUnknownImpl, Impl: IWdsTransportNamespace_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bszconfiguration: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetConfiguration(::core::mem::transmute(&bszconfiguration)).into()
        }
        unsafe extern "system" fn Registered<Identity: ::windows::core::IUnknownImpl, Impl: IWdsTransportNamespace_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbregistered: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Registered() {
                ::core::result::Result::Ok(ok__) => {
                    *pbregistered = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Tombstoned<Identity: ::windows::core::IUnknownImpl, Impl: IWdsTransportNamespace_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbtombstoned: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Tombstoned() {
                ::core::result::Result::Ok(ok__) => {
                    *pbtombstoned = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TombstoneTime<Identity: ::windows::core::IUnknownImpl, Impl: IWdsTransportNamespace_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ptombstonetime: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).TombstoneTime() {
                ::core::result::Result::Ok(ok__) => {
                    *ptombstonetime = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TransmissionStarted<Identity: ::windows::core::IUnknownImpl, Impl: IWdsTransportNamespace_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbtransmissionstarted: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).TransmissionStarted() {
                ::core::result::Result::Ok(ok__) => {
                    *pbtransmissionstarted = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Register<Identity: ::windows::core::IUnknownImpl, Impl: IWdsTransportNamespace_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Register().into()
        }
        unsafe extern "system" fn Deregister<Identity: ::windows::core::IUnknownImpl, Impl: IWdsTransportNamespace_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bterminatesessions: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Deregister(::core::mem::transmute_copy(&bterminatesessions)).into()
        }
        unsafe extern "system" fn Clone<Identity: ::windows::core::IUnknownImpl, Impl: IWdsTransportNamespace_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppwdstransportnamespaceclone: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Clone() {
                ::core::result::Result::Ok(ok__) => {
                    *ppwdstransportnamespaceclone = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Refresh<Identity: ::windows::core::IUnknownImpl, Impl: IWdsTransportNamespace_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Refresh().into()
        }
        unsafe extern "system" fn RetrieveContents<Identity: ::windows::core::IUnknownImpl, Impl: IWdsTransportNamespace_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppwdstransportcontents: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).RetrieveContents() {
                ::core::result::Result::Ok(ok__) => {
                    *ppwdstransportcontents = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: super::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            Type: Type::<Identity, Impl, OFFSET>,
            Id: Id::<Identity, Impl, OFFSET>,
            Name: Name::<Identity, Impl, OFFSET>,
            SetName: SetName::<Identity, Impl, OFFSET>,
            FriendlyName: FriendlyName::<Identity, Impl, OFFSET>,
            SetFriendlyName: SetFriendlyName::<Identity, Impl, OFFSET>,
            Description: Description::<Identity, Impl, OFFSET>,
            SetDescription: SetDescription::<Identity, Impl, OFFSET>,
            ContentProvider: ContentProvider::<Identity, Impl, OFFSET>,
            SetContentProvider: SetContentProvider::<Identity, Impl, OFFSET>,
            Configuration: Configuration::<Identity, Impl, OFFSET>,
            SetConfiguration: SetConfiguration::<Identity, Impl, OFFSET>,
            Registered: Registered::<Identity, Impl, OFFSET>,
            Tombstoned: Tombstoned::<Identity, Impl, OFFSET>,
            TombstoneTime: TombstoneTime::<Identity, Impl, OFFSET>,
            TransmissionStarted: TransmissionStarted::<Identity, Impl, OFFSET>,
            Register: Register::<Identity, Impl, OFFSET>,
            Deregister: Deregister::<Identity, Impl, OFFSET>,
            Clone: Clone::<Identity, Impl, OFFSET>,
            Refresh: Refresh::<Identity, Impl, OFFSET>,
            RetrieveContents: RetrieveContents::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWdsTransportNamespace as ::windows::core::Interface>::IID || iid == &<super::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IWdsTransportNamespaceAutoCast_Impl: Sized + super::Com::IDispatch_Impl + IWdsTransportNamespace_Impl {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IWdsTransportNamespaceAutoCast_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWdsTransportNamespaceAutoCast_Impl, const OFFSET: isize>() -> IWdsTransportNamespaceAutoCast_Vtbl {
        Self { base: IWdsTransportNamespace_Vtbl::new::<Identity, Impl, OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWdsTransportNamespaceAutoCast as ::windows::core::Interface>::IID || iid == &<super::Com::IDispatch as ::windows::core::Interface>::IID || iid == &<IWdsTransportNamespace as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IWdsTransportNamespaceManager_Impl: Sized + super::Com::IDispatch_Impl {
    fn CreateNamespace(&self, namespacetype: WDSTRANSPORT_NAMESPACE_TYPE, bsznamespacename: &super::super::Foundation::BSTR, bszcontentprovider: &super::super::Foundation::BSTR, bszconfiguration: &super::super::Foundation::BSTR) -> ::windows::core::Result<IWdsTransportNamespace>;
    fn RetrieveNamespace(&self, bsznamespacename: &super::super::Foundation::BSTR) -> ::windows::core::Result<IWdsTransportNamespace>;
    fn RetrieveNamespaces(&self, bszcontentprovider: &super::super::Foundation::BSTR, bsznamespacename: &super::super::Foundation::BSTR, bincludetombstones: i16) -> ::windows::core::Result<IWdsTransportCollection>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IWdsTransportNamespaceManager_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWdsTransportNamespaceManager_Impl, const OFFSET: isize>() -> IWdsTransportNamespaceManager_Vtbl {
        unsafe extern "system" fn CreateNamespace<Identity: ::windows::core::IUnknownImpl, Impl: IWdsTransportNamespaceManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, namespacetype: WDSTRANSPORT_NAMESPACE_TYPE, bsznamespacename: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bszcontentprovider: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bszconfiguration: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ppwdstransportnamespace: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CreateNamespace(::core::mem::transmute_copy(&namespacetype), ::core::mem::transmute(&bsznamespacename), ::core::mem::transmute(&bszcontentprovider), ::core::mem::transmute(&bszconfiguration)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppwdstransportnamespace = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RetrieveNamespace<Identity: ::windows::core::IUnknownImpl, Impl: IWdsTransportNamespaceManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bsznamespacename: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ppwdstransportnamespace: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).RetrieveNamespace(::core::mem::transmute(&bsznamespacename)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppwdstransportnamespace = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RetrieveNamespaces<Identity: ::windows::core::IUnknownImpl, Impl: IWdsTransportNamespaceManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bszcontentprovider: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bsznamespacename: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bincludetombstones: i16, ppwdstransportnamespaces: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).RetrieveNamespaces(::core::mem::transmute(&bszcontentprovider), ::core::mem::transmute(&bsznamespacename), ::core::mem::transmute_copy(&bincludetombstones)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppwdstransportnamespaces = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: super::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            CreateNamespace: CreateNamespace::<Identity, Impl, OFFSET>,
            RetrieveNamespace: RetrieveNamespace::<Identity, Impl, OFFSET>,
            RetrieveNamespaces: RetrieveNamespaces::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWdsTransportNamespaceManager as ::windows::core::Interface>::IID || iid == &<super::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IWdsTransportNamespaceScheduledCast_Impl: Sized + super::Com::IDispatch_Impl + IWdsTransportNamespace_Impl {
    fn StartTransmission(&self) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IWdsTransportNamespaceScheduledCast_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWdsTransportNamespaceScheduledCast_Impl, const OFFSET: isize>() -> IWdsTransportNamespaceScheduledCast_Vtbl {
        unsafe extern "system" fn StartTransmission<Identity: ::windows::core::IUnknownImpl, Impl: IWdsTransportNamespaceScheduledCast_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).StartTransmission().into()
        }
        Self { base: IWdsTransportNamespace_Vtbl::new::<Identity, Impl, OFFSET>(), StartTransmission: StartTransmission::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWdsTransportNamespaceScheduledCast as ::windows::core::Interface>::IID || iid == &<super::Com::IDispatch as ::windows::core::Interface>::IID || iid == &<IWdsTransportNamespace as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IWdsTransportNamespaceScheduledCastAutoStart_Impl: Sized + super::Com::IDispatch_Impl + IWdsTransportNamespace_Impl + IWdsTransportNamespaceScheduledCast_Impl {
    fn MinimumClients(&self) -> ::windows::core::Result<u32>;
    fn SetMinimumClients(&self, ulminimumclients: u32) -> ::windows::core::Result<()>;
    fn StartTime(&self) -> ::windows::core::Result<f64>;
    fn SetStartTime(&self, starttime: f64) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IWdsTransportNamespaceScheduledCastAutoStart_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWdsTransportNamespaceScheduledCastAutoStart_Impl, const OFFSET: isize>() -> IWdsTransportNamespaceScheduledCastAutoStart_Vtbl {
        unsafe extern "system" fn MinimumClients<Identity: ::windows::core::IUnknownImpl, Impl: IWdsTransportNamespaceScheduledCastAutoStart_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pulminimumclients: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).MinimumClients() {
                ::core::result::Result::Ok(ok__) => {
                    *pulminimumclients = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMinimumClients<Identity: ::windows::core::IUnknownImpl, Impl: IWdsTransportNamespaceScheduledCastAutoStart_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ulminimumclients: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetMinimumClients(::core::mem::transmute_copy(&ulminimumclients)).into()
        }
        unsafe extern "system" fn StartTime<Identity: ::windows::core::IUnknownImpl, Impl: IWdsTransportNamespaceScheduledCastAutoStart_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pstarttime: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).StartTime() {
                ::core::result::Result::Ok(ok__) => {
                    *pstarttime = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetStartTime<Identity: ::windows::core::IUnknownImpl, Impl: IWdsTransportNamespaceScheduledCastAutoStart_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, starttime: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetStartTime(::core::mem::transmute_copy(&starttime)).into()
        }
        Self {
            base: IWdsTransportNamespaceScheduledCast_Vtbl::new::<Identity, Impl, OFFSET>(),
            MinimumClients: MinimumClients::<Identity, Impl, OFFSET>,
            SetMinimumClients: SetMinimumClients::<Identity, Impl, OFFSET>,
            StartTime: StartTime::<Identity, Impl, OFFSET>,
            SetStartTime: SetStartTime::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWdsTransportNamespaceScheduledCastAutoStart as ::windows::core::Interface>::IID || iid == &<super::Com::IDispatch as ::windows::core::Interface>::IID || iid == &<IWdsTransportNamespace as ::windows::core::Interface>::IID || iid == &<IWdsTransportNamespaceScheduledCast as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IWdsTransportNamespaceScheduledCastManualStart_Impl: Sized + super::Com::IDispatch_Impl + IWdsTransportNamespace_Impl + IWdsTransportNamespaceScheduledCast_Impl {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IWdsTransportNamespaceScheduledCastManualStart_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWdsTransportNamespaceScheduledCastManualStart_Impl, const OFFSET: isize>() -> IWdsTransportNamespaceScheduledCastManualStart_Vtbl {
        Self { base: IWdsTransportNamespaceScheduledCast_Vtbl::new::<Identity, Impl, OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWdsTransportNamespaceScheduledCastManualStart as ::windows::core::Interface>::IID || iid == &<super::Com::IDispatch as ::windows::core::Interface>::IID || iid == &<IWdsTransportNamespace as ::windows::core::Interface>::IID || iid == &<IWdsTransportNamespaceScheduledCast as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IWdsTransportServer_Impl: Sized + super::Com::IDispatch_Impl {
    fn Name(&self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetupManager(&self) -> ::windows::core::Result<IWdsTransportSetupManager>;
    fn ConfigurationManager(&self) -> ::windows::core::Result<IWdsTransportConfigurationManager>;
    fn NamespaceManager(&self) -> ::windows::core::Result<IWdsTransportNamespaceManager>;
    fn DisconnectClient(&self, ulclientid: u32, disconnectiontype: WDSTRANSPORT_DISCONNECT_TYPE) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IWdsTransportServer_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWdsTransportServer_Impl, const OFFSET: isize>() -> IWdsTransportServer_Vtbl {
        unsafe extern "system" fn Name<Identity: ::windows::core::IUnknownImpl, Impl: IWdsTransportServer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbszname: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Name() {
                ::core::result::Result::Ok(ok__) => {
                    *pbszname = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetupManager<Identity: ::windows::core::IUnknownImpl, Impl: IWdsTransportServer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppwdstransportsetupmanager: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).SetupManager() {
                ::core::result::Result::Ok(ok__) => {
                    *ppwdstransportsetupmanager = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ConfigurationManager<Identity: ::windows::core::IUnknownImpl, Impl: IWdsTransportServer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppwdstransportconfigurationmanager: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).ConfigurationManager() {
                ::core::result::Result::Ok(ok__) => {
                    *ppwdstransportconfigurationmanager = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn NamespaceManager<Identity: ::windows::core::IUnknownImpl, Impl: IWdsTransportServer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppwdstransportnamespacemanager: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).NamespaceManager() {
                ::core::result::Result::Ok(ok__) => {
                    *ppwdstransportnamespacemanager = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DisconnectClient<Identity: ::windows::core::IUnknownImpl, Impl: IWdsTransportServer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ulclientid: u32, disconnectiontype: WDSTRANSPORT_DISCONNECT_TYPE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).DisconnectClient(::core::mem::transmute_copy(&ulclientid), ::core::mem::transmute_copy(&disconnectiontype)).into()
        }
        Self {
            base: super::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            Name: Name::<Identity, Impl, OFFSET>,
            SetupManager: SetupManager::<Identity, Impl, OFFSET>,
            ConfigurationManager: ConfigurationManager::<Identity, Impl, OFFSET>,
            NamespaceManager: NamespaceManager::<Identity, Impl, OFFSET>,
            DisconnectClient: DisconnectClient::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWdsTransportServer as ::windows::core::Interface>::IID || iid == &<super::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IWdsTransportServer2_Impl: Sized + super::Com::IDispatch_Impl + IWdsTransportServer_Impl {
    fn TftpManager(&self) -> ::windows::core::Result<IWdsTransportTftpManager>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IWdsTransportServer2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWdsTransportServer2_Impl, const OFFSET: isize>() -> IWdsTransportServer2_Vtbl {
        unsafe extern "system" fn TftpManager<Identity: ::windows::core::IUnknownImpl, Impl: IWdsTransportServer2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppwdstransporttftpmanager: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).TftpManager() {
                ::core::result::Result::Ok(ok__) => {
                    *ppwdstransporttftpmanager = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: IWdsTransportServer_Vtbl::new::<Identity, Impl, OFFSET>(), TftpManager: TftpManager::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWdsTransportServer2 as ::windows::core::Interface>::IID || iid == &<super::Com::IDispatch as ::windows::core::Interface>::IID || iid == &<IWdsTransportServer as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IWdsTransportServicePolicy_Impl: Sized + super::Com::IDispatch_Impl + IWdsTransportCacheable_Impl {
    fn IpAddressSource(&self, addresstype: WDSTRANSPORT_IP_ADDRESS_TYPE) -> ::windows::core::Result<WDSTRANSPORT_IP_ADDRESS_SOURCE_TYPE>;
    fn SetIpAddressSource(&self, addresstype: WDSTRANSPORT_IP_ADDRESS_TYPE, sourcetype: WDSTRANSPORT_IP_ADDRESS_SOURCE_TYPE) -> ::windows::core::Result<()>;
    fn StartIpAddress(&self, addresstype: WDSTRANSPORT_IP_ADDRESS_TYPE) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetStartIpAddress(&self, addresstype: WDSTRANSPORT_IP_ADDRESS_TYPE, bszstartipaddress: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn EndIpAddress(&self, addresstype: WDSTRANSPORT_IP_ADDRESS_TYPE) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetEndIpAddress(&self, addresstype: WDSTRANSPORT_IP_ADDRESS_TYPE, bszendipaddress: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn StartPort(&self) -> ::windows::core::Result<u32>;
    fn SetStartPort(&self, ulstartport: u32) -> ::windows::core::Result<()>;
    fn EndPort(&self) -> ::windows::core::Result<u32>;
    fn SetEndPort(&self, ulendport: u32) -> ::windows::core::Result<()>;
    fn NetworkProfile(&self) -> ::windows::core::Result<WDSTRANSPORT_NETWORK_PROFILE_TYPE>;
    fn SetNetworkProfile(&self, profiletype: WDSTRANSPORT_NETWORK_PROFILE_TYPE) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IWdsTransportServicePolicy_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWdsTransportServicePolicy_Impl, const OFFSET: isize>() -> IWdsTransportServicePolicy_Vtbl {
        unsafe extern "system" fn IpAddressSource<Identity: ::windows::core::IUnknownImpl, Impl: IWdsTransportServicePolicy_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, addresstype: WDSTRANSPORT_IP_ADDRESS_TYPE, psourcetype: *mut WDSTRANSPORT_IP_ADDRESS_SOURCE_TYPE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).IpAddressSource(::core::mem::transmute_copy(&addresstype)) {
                ::core::result::Result::Ok(ok__) => {
                    *psourcetype = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIpAddressSource<Identity: ::windows::core::IUnknownImpl, Impl: IWdsTransportServicePolicy_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, addresstype: WDSTRANSPORT_IP_ADDRESS_TYPE, sourcetype: WDSTRANSPORT_IP_ADDRESS_SOURCE_TYPE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetIpAddressSource(::core::mem::transmute_copy(&addresstype), ::core::mem::transmute_copy(&sourcetype)).into()
        }
        unsafe extern "system" fn StartIpAddress<Identity: ::windows::core::IUnknownImpl, Impl: IWdsTransportServicePolicy_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, addresstype: WDSTRANSPORT_IP_ADDRESS_TYPE, pbszstartipaddress: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).StartIpAddress(::core::mem::transmute_copy(&addresstype)) {
                ::core::result::Result::Ok(ok__) => {
                    *pbszstartipaddress = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetStartIpAddress<Identity: ::windows::core::IUnknownImpl, Impl: IWdsTransportServicePolicy_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, addresstype: WDSTRANSPORT_IP_ADDRESS_TYPE, bszstartipaddress: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetStartIpAddress(::core::mem::transmute_copy(&addresstype), ::core::mem::transmute(&bszstartipaddress)).into()
        }
        unsafe extern "system" fn EndIpAddress<Identity: ::windows::core::IUnknownImpl, Impl: IWdsTransportServicePolicy_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, addresstype: WDSTRANSPORT_IP_ADDRESS_TYPE, pbszendipaddress: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).EndIpAddress(::core::mem::transmute_copy(&addresstype)) {
                ::core::result::Result::Ok(ok__) => {
                    *pbszendipaddress = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetEndIpAddress<Identity: ::windows::core::IUnknownImpl, Impl: IWdsTransportServicePolicy_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, addresstype: WDSTRANSPORT_IP_ADDRESS_TYPE, bszendipaddress: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetEndIpAddress(::core::mem::transmute_copy(&addresstype), ::core::mem::transmute(&bszendipaddress)).into()
        }
        unsafe extern "system" fn StartPort<Identity: ::windows::core::IUnknownImpl, Impl: IWdsTransportServicePolicy_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pulstartport: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).StartPort() {
                ::core::result::Result::Ok(ok__) => {
                    *pulstartport = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetStartPort<Identity: ::windows::core::IUnknownImpl, Impl: IWdsTransportServicePolicy_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ulstartport: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetStartPort(::core::mem::transmute_copy(&ulstartport)).into()
        }
        unsafe extern "system" fn EndPort<Identity: ::windows::core::IUnknownImpl, Impl: IWdsTransportServicePolicy_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pulendport: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).EndPort() {
                ::core::result::Result::Ok(ok__) => {
                    *pulendport = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetEndPort<Identity: ::windows::core::IUnknownImpl, Impl: IWdsTransportServicePolicy_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ulendport: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetEndPort(::core::mem::transmute_copy(&ulendport)).into()
        }
        unsafe extern "system" fn NetworkProfile<Identity: ::windows::core::IUnknownImpl, Impl: IWdsTransportServicePolicy_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pprofiletype: *mut WDSTRANSPORT_NETWORK_PROFILE_TYPE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).NetworkProfile() {
                ::core::result::Result::Ok(ok__) => {
                    *pprofiletype = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetNetworkProfile<Identity: ::windows::core::IUnknownImpl, Impl: IWdsTransportServicePolicy_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, profiletype: WDSTRANSPORT_NETWORK_PROFILE_TYPE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetNetworkProfile(::core::mem::transmute_copy(&profiletype)).into()
        }
        Self {
            base: IWdsTransportCacheable_Vtbl::new::<Identity, Impl, OFFSET>(),
            IpAddressSource: IpAddressSource::<Identity, Impl, OFFSET>,
            SetIpAddressSource: SetIpAddressSource::<Identity, Impl, OFFSET>,
            StartIpAddress: StartIpAddress::<Identity, Impl, OFFSET>,
            SetStartIpAddress: SetStartIpAddress::<Identity, Impl, OFFSET>,
            EndIpAddress: EndIpAddress::<Identity, Impl, OFFSET>,
            SetEndIpAddress: SetEndIpAddress::<Identity, Impl, OFFSET>,
            StartPort: StartPort::<Identity, Impl, OFFSET>,
            SetStartPort: SetStartPort::<Identity, Impl, OFFSET>,
            EndPort: EndPort::<Identity, Impl, OFFSET>,
            SetEndPort: SetEndPort::<Identity, Impl, OFFSET>,
            NetworkProfile: NetworkProfile::<Identity, Impl, OFFSET>,
            SetNetworkProfile: SetNetworkProfile::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWdsTransportServicePolicy as ::windows::core::Interface>::IID || iid == &<super::Com::IDispatch as ::windows::core::Interface>::IID || iid == &<IWdsTransportCacheable as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IWdsTransportServicePolicy2_Impl: Sized + super::Com::IDispatch_Impl + IWdsTransportCacheable_Impl + IWdsTransportServicePolicy_Impl {
    fn UdpPortPolicy(&self) -> ::windows::core::Result<WDSTRANSPORT_UDP_PORT_POLICY>;
    fn SetUdpPortPolicy(&self, udpportpolicy: WDSTRANSPORT_UDP_PORT_POLICY) -> ::windows::core::Result<()>;
    fn TftpMaximumBlockSize(&self) -> ::windows::core::Result<u32>;
    fn SetTftpMaximumBlockSize(&self, ultftpmaximumblocksize: u32) -> ::windows::core::Result<()>;
    fn EnableTftpVariableWindowExtension(&self) -> ::windows::core::Result<i16>;
    fn SetEnableTftpVariableWindowExtension(&self, benabletftpvariablewindowextension: i16) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IWdsTransportServicePolicy2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWdsTransportServicePolicy2_Impl, const OFFSET: isize>() -> IWdsTransportServicePolicy2_Vtbl {
        unsafe extern "system" fn UdpPortPolicy<Identity: ::windows::core::IUnknownImpl, Impl: IWdsTransportServicePolicy2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pudpportpolicy: *mut WDSTRANSPORT_UDP_PORT_POLICY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).UdpPortPolicy() {
                ::core::result::Result::Ok(ok__) => {
                    *pudpportpolicy = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetUdpPortPolicy<Identity: ::windows::core::IUnknownImpl, Impl: IWdsTransportServicePolicy2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, udpportpolicy: WDSTRANSPORT_UDP_PORT_POLICY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetUdpPortPolicy(::core::mem::transmute_copy(&udpportpolicy)).into()
        }
        unsafe extern "system" fn TftpMaximumBlockSize<Identity: ::windows::core::IUnknownImpl, Impl: IWdsTransportServicePolicy2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pultftpmaximumblocksize: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).TftpMaximumBlockSize() {
                ::core::result::Result::Ok(ok__) => {
                    *pultftpmaximumblocksize = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetTftpMaximumBlockSize<Identity: ::windows::core::IUnknownImpl, Impl: IWdsTransportServicePolicy2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ultftpmaximumblocksize: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetTftpMaximumBlockSize(::core::mem::transmute_copy(&ultftpmaximumblocksize)).into()
        }
        unsafe extern "system" fn EnableTftpVariableWindowExtension<Identity: ::windows::core::IUnknownImpl, Impl: IWdsTransportServicePolicy2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbenabletftpvariablewindowextension: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).EnableTftpVariableWindowExtension() {
                ::core::result::Result::Ok(ok__) => {
                    *pbenabletftpvariablewindowextension = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetEnableTftpVariableWindowExtension<Identity: ::windows::core::IUnknownImpl, Impl: IWdsTransportServicePolicy2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, benabletftpvariablewindowextension: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetEnableTftpVariableWindowExtension(::core::mem::transmute_copy(&benabletftpvariablewindowextension)).into()
        }
        Self {
            base: IWdsTransportServicePolicy_Vtbl::new::<Identity, Impl, OFFSET>(),
            UdpPortPolicy: UdpPortPolicy::<Identity, Impl, OFFSET>,
            SetUdpPortPolicy: SetUdpPortPolicy::<Identity, Impl, OFFSET>,
            TftpMaximumBlockSize: TftpMaximumBlockSize::<Identity, Impl, OFFSET>,
            SetTftpMaximumBlockSize: SetTftpMaximumBlockSize::<Identity, Impl, OFFSET>,
            EnableTftpVariableWindowExtension: EnableTftpVariableWindowExtension::<Identity, Impl, OFFSET>,
            SetEnableTftpVariableWindowExtension: SetEnableTftpVariableWindowExtension::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWdsTransportServicePolicy2 as ::windows::core::Interface>::IID || iid == &<super::Com::IDispatch as ::windows::core::Interface>::IID || iid == &<IWdsTransportCacheable as ::windows::core::Interface>::IID || iid == &<IWdsTransportServicePolicy as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IWdsTransportSession_Impl: Sized + super::Com::IDispatch_Impl {
    fn Content(&self) -> ::windows::core::Result<IWdsTransportContent>;
    fn Id(&self) -> ::windows::core::Result<u32>;
    fn NetworkInterfaceName(&self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn NetworkInterfaceAddress(&self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn TransferRate(&self) -> ::windows::core::Result<u32>;
    fn MasterClientId(&self) -> ::windows::core::Result<u32>;
    fn RetrieveClients(&self) -> ::windows::core::Result<IWdsTransportCollection>;
    fn Terminate(&self) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IWdsTransportSession_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWdsTransportSession_Impl, const OFFSET: isize>() -> IWdsTransportSession_Vtbl {
        unsafe extern "system" fn Content<Identity: ::windows::core::IUnknownImpl, Impl: IWdsTransportSession_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppwdstransportcontent: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Content() {
                ::core::result::Result::Ok(ok__) => {
                    *ppwdstransportcontent = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Id<Identity: ::windows::core::IUnknownImpl, Impl: IWdsTransportSession_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pulid: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Id() {
                ::core::result::Result::Ok(ok__) => {
                    *pulid = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn NetworkInterfaceName<Identity: ::windows::core::IUnknownImpl, Impl: IWdsTransportSession_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbsznetworkinterfacename: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).NetworkInterfaceName() {
                ::core::result::Result::Ok(ok__) => {
                    *pbsznetworkinterfacename = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn NetworkInterfaceAddress<Identity: ::windows::core::IUnknownImpl, Impl: IWdsTransportSession_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbsznetworkinterfaceaddress: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).NetworkInterfaceAddress() {
                ::core::result::Result::Ok(ok__) => {
                    *pbsznetworkinterfaceaddress = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TransferRate<Identity: ::windows::core::IUnknownImpl, Impl: IWdsTransportSession_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pultransferrate: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).TransferRate() {
                ::core::result::Result::Ok(ok__) => {
                    *pultransferrate = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MasterClientId<Identity: ::windows::core::IUnknownImpl, Impl: IWdsTransportSession_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pulmasterclientid: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).MasterClientId() {
                ::core::result::Result::Ok(ok__) => {
                    *pulmasterclientid = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RetrieveClients<Identity: ::windows::core::IUnknownImpl, Impl: IWdsTransportSession_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppwdstransportclients: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).RetrieveClients() {
                ::core::result::Result::Ok(ok__) => {
                    *ppwdstransportclients = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Terminate<Identity: ::windows::core::IUnknownImpl, Impl: IWdsTransportSession_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Terminate().into()
        }
        Self {
            base: super::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            Content: Content::<Identity, Impl, OFFSET>,
            Id: Id::<Identity, Impl, OFFSET>,
            NetworkInterfaceName: NetworkInterfaceName::<Identity, Impl, OFFSET>,
            NetworkInterfaceAddress: NetworkInterfaceAddress::<Identity, Impl, OFFSET>,
            TransferRate: TransferRate::<Identity, Impl, OFFSET>,
            MasterClientId: MasterClientId::<Identity, Impl, OFFSET>,
            RetrieveClients: RetrieveClients::<Identity, Impl, OFFSET>,
            Terminate: Terminate::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWdsTransportSession as ::windows::core::Interface>::IID || iid == &<super::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IWdsTransportSetupManager_Impl: Sized + super::Com::IDispatch_Impl {
    fn Version(&self) -> ::windows::core::Result<u64>;
    fn InstalledFeatures(&self) -> ::windows::core::Result<u32>;
    fn Protocols(&self) -> ::windows::core::Result<u32>;
    fn RegisterContentProvider(&self, bszname: &super::super::Foundation::BSTR, bszdescription: &super::super::Foundation::BSTR, bszfilepath: &super::super::Foundation::BSTR, bszinitializationroutine: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn DeregisterContentProvider(&self, bszname: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IWdsTransportSetupManager_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWdsTransportSetupManager_Impl, const OFFSET: isize>() -> IWdsTransportSetupManager_Vtbl {
        unsafe extern "system" fn Version<Identity: ::windows::core::IUnknownImpl, Impl: IWdsTransportSetupManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pullversion: *mut u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Version() {
                ::core::result::Result::Ok(ok__) => {
                    *pullversion = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn InstalledFeatures<Identity: ::windows::core::IUnknownImpl, Impl: IWdsTransportSetupManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pulinstalledfeatures: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).InstalledFeatures() {
                ::core::result::Result::Ok(ok__) => {
                    *pulinstalledfeatures = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Protocols<Identity: ::windows::core::IUnknownImpl, Impl: IWdsTransportSetupManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pulprotocols: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Protocols() {
                ::core::result::Result::Ok(ok__) => {
                    *pulprotocols = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RegisterContentProvider<Identity: ::windows::core::IUnknownImpl, Impl: IWdsTransportSetupManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bszname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bszdescription: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bszfilepath: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bszinitializationroutine: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).RegisterContentProvider(::core::mem::transmute(&bszname), ::core::mem::transmute(&bszdescription), ::core::mem::transmute(&bszfilepath), ::core::mem::transmute(&bszinitializationroutine)).into()
        }
        unsafe extern "system" fn DeregisterContentProvider<Identity: ::windows::core::IUnknownImpl, Impl: IWdsTransportSetupManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bszname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).DeregisterContentProvider(::core::mem::transmute(&bszname)).into()
        }
        Self {
            base: super::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            Version: Version::<Identity, Impl, OFFSET>,
            InstalledFeatures: InstalledFeatures::<Identity, Impl, OFFSET>,
            Protocols: Protocols::<Identity, Impl, OFFSET>,
            RegisterContentProvider: RegisterContentProvider::<Identity, Impl, OFFSET>,
            DeregisterContentProvider: DeregisterContentProvider::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWdsTransportSetupManager as ::windows::core::Interface>::IID || iid == &<super::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IWdsTransportSetupManager2_Impl: Sized + super::Com::IDispatch_Impl + IWdsTransportSetupManager_Impl {
    fn TftpCapabilities(&self) -> ::windows::core::Result<u32>;
    fn ContentProviders(&self) -> ::windows::core::Result<IWdsTransportCollection>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IWdsTransportSetupManager2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWdsTransportSetupManager2_Impl, const OFFSET: isize>() -> IWdsTransportSetupManager2_Vtbl {
        unsafe extern "system" fn TftpCapabilities<Identity: ::windows::core::IUnknownImpl, Impl: IWdsTransportSetupManager2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pultftpcapabilities: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).TftpCapabilities() {
                ::core::result::Result::Ok(ok__) => {
                    *pultftpcapabilities = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ContentProviders<Identity: ::windows::core::IUnknownImpl, Impl: IWdsTransportSetupManager2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppprovidercollection: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).ContentProviders() {
                ::core::result::Result::Ok(ok__) => {
                    *ppprovidercollection = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: IWdsTransportSetupManager_Vtbl::new::<Identity, Impl, OFFSET>(),
            TftpCapabilities: TftpCapabilities::<Identity, Impl, OFFSET>,
            ContentProviders: ContentProviders::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWdsTransportSetupManager2 as ::windows::core::Interface>::IID || iid == &<super::Com::IDispatch as ::windows::core::Interface>::IID || iid == &<IWdsTransportSetupManager as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IWdsTransportTftpClient_Impl: Sized + super::Com::IDispatch_Impl {
    fn FileName(&self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn IpAddress(&self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn Timeout(&self) -> ::windows::core::Result<u32>;
    fn CurrentFileOffset(&self) -> ::windows::core::Result<u64>;
    fn FileSize(&self) -> ::windows::core::Result<u64>;
    fn BlockSize(&self) -> ::windows::core::Result<u32>;
    fn WindowSize(&self) -> ::windows::core::Result<u32>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IWdsTransportTftpClient_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWdsTransportTftpClient_Impl, const OFFSET: isize>() -> IWdsTransportTftpClient_Vtbl {
        unsafe extern "system" fn FileName<Identity: ::windows::core::IUnknownImpl, Impl: IWdsTransportTftpClient_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbszfilename: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).FileName() {
                ::core::result::Result::Ok(ok__) => {
                    *pbszfilename = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IpAddress<Identity: ::windows::core::IUnknownImpl, Impl: IWdsTransportTftpClient_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbszipaddress: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).IpAddress() {
                ::core::result::Result::Ok(ok__) => {
                    *pbszipaddress = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Timeout<Identity: ::windows::core::IUnknownImpl, Impl: IWdsTransportTftpClient_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pultimeout: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Timeout() {
                ::core::result::Result::Ok(ok__) => {
                    *pultimeout = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CurrentFileOffset<Identity: ::windows::core::IUnknownImpl, Impl: IWdsTransportTftpClient_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pul64currentoffset: *mut u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CurrentFileOffset() {
                ::core::result::Result::Ok(ok__) => {
                    *pul64currentoffset = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FileSize<Identity: ::windows::core::IUnknownImpl, Impl: IWdsTransportTftpClient_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pul64filesize: *mut u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).FileSize() {
                ::core::result::Result::Ok(ok__) => {
                    *pul64filesize = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BlockSize<Identity: ::windows::core::IUnknownImpl, Impl: IWdsTransportTftpClient_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pulblocksize: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).BlockSize() {
                ::core::result::Result::Ok(ok__) => {
                    *pulblocksize = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn WindowSize<Identity: ::windows::core::IUnknownImpl, Impl: IWdsTransportTftpClient_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pulwindowsize: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).WindowSize() {
                ::core::result::Result::Ok(ok__) => {
                    *pulwindowsize = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: super::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            FileName: FileName::<Identity, Impl, OFFSET>,
            IpAddress: IpAddress::<Identity, Impl, OFFSET>,
            Timeout: Timeout::<Identity, Impl, OFFSET>,
            CurrentFileOffset: CurrentFileOffset::<Identity, Impl, OFFSET>,
            FileSize: FileSize::<Identity, Impl, OFFSET>,
            BlockSize: BlockSize::<Identity, Impl, OFFSET>,
            WindowSize: WindowSize::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWdsTransportTftpClient as ::windows::core::Interface>::IID || iid == &<super::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IWdsTransportTftpManager_Impl: Sized + super::Com::IDispatch_Impl {
    fn RetrieveTftpClients(&self) -> ::windows::core::Result<IWdsTransportCollection>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IWdsTransportTftpManager_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWdsTransportTftpManager_Impl, const OFFSET: isize>() -> IWdsTransportTftpManager_Vtbl {
        unsafe extern "system" fn RetrieveTftpClients<Identity: ::windows::core::IUnknownImpl, Impl: IWdsTransportTftpManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppwdstransporttftpclients: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).RetrieveTftpClients() {
                ::core::result::Result::Ok(ok__) => {
                    *ppwdstransporttftpclients = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: super::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(), RetrieveTftpClients: RetrieveTftpClients::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWdsTransportTftpManager as ::windows::core::Interface>::IID || iid == &<super::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
