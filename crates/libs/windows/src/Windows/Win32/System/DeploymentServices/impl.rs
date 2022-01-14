#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IWdsTransportCacheable_Impl: Sized + super::Com::IDispatch_Impl {
    fn Dirty(&mut self) -> ::windows::core::Result<i16>;
    fn Discard(&mut self) -> ::windows::core::Result<()>;
    fn Refresh(&mut self) -> ::windows::core::Result<()>;
    fn Commit(&mut self) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IWdsTransportCacheable_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWdsTransportCacheable_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWdsTransportCacheable_Vtbl {
        unsafe extern "system" fn Dirty<Impl: IWdsTransportCacheable_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbdirty: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Dirty() {
                ::core::result::Result::Ok(ok__) => {
                    *pbdirty = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Discard<Impl: IWdsTransportCacheable_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Discard().into()
        }
        unsafe extern "system" fn Refresh<Impl: IWdsTransportCacheable_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Refresh().into()
        }
        unsafe extern "system" fn Commit<Impl: IWdsTransportCacheable_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Commit().into()
        }
        Self {
            base: super::Com::IDispatch_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            Dirty: Dirty::<Impl, IMPL_OFFSET>,
            Discard: Discard::<Impl, IMPL_OFFSET>,
            Refresh: Refresh::<Impl, IMPL_OFFSET>,
            Commit: Commit::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWdsTransportCacheable as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IWdsTransportClient_Impl: Sized + super::Com::IDispatch_Impl {
    fn Session(&mut self) -> ::windows::core::Result<IWdsTransportSession>;
    fn Id(&mut self) -> ::windows::core::Result<u32>;
    fn Name(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn MacAddress(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn IpAddress(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn PercentCompletion(&mut self) -> ::windows::core::Result<u32>;
    fn JoinDuration(&mut self) -> ::windows::core::Result<u32>;
    fn CpuUtilization(&mut self) -> ::windows::core::Result<u32>;
    fn MemoryUtilization(&mut self) -> ::windows::core::Result<u32>;
    fn NetworkUtilization(&mut self) -> ::windows::core::Result<u32>;
    fn UserIdentity(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn Disconnect(&mut self, disconnectiontype: WDSTRANSPORT_DISCONNECT_TYPE) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IWdsTransportClient_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWdsTransportClient_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWdsTransportClient_Vtbl {
        unsafe extern "system" fn Session<Impl: IWdsTransportClient_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppwdstransportsession: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Session() {
                ::core::result::Result::Ok(ok__) => {
                    *ppwdstransportsession = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Id<Impl: IWdsTransportClient_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pulid: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Id() {
                ::core::result::Result::Ok(ok__) => {
                    *pulid = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Name<Impl: IWdsTransportClient_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbszname: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Name() {
                ::core::result::Result::Ok(ok__) => {
                    *pbszname = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MacAddress<Impl: IWdsTransportClient_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbszmacaddress: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MacAddress() {
                ::core::result::Result::Ok(ok__) => {
                    *pbszmacaddress = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IpAddress<Impl: IWdsTransportClient_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbszipaddress: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IpAddress() {
                ::core::result::Result::Ok(ok__) => {
                    *pbszipaddress = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PercentCompletion<Impl: IWdsTransportClient_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pulpercentcompletion: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PercentCompletion() {
                ::core::result::Result::Ok(ok__) => {
                    *pulpercentcompletion = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn JoinDuration<Impl: IWdsTransportClient_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, puljoinduration: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).JoinDuration() {
                ::core::result::Result::Ok(ok__) => {
                    *puljoinduration = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CpuUtilization<Impl: IWdsTransportClient_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pulcpuutilization: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CpuUtilization() {
                ::core::result::Result::Ok(ok__) => {
                    *pulcpuutilization = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MemoryUtilization<Impl: IWdsTransportClient_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pulmemoryutilization: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MemoryUtilization() {
                ::core::result::Result::Ok(ok__) => {
                    *pulmemoryutilization = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn NetworkUtilization<Impl: IWdsTransportClient_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pulnetworkutilization: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).NetworkUtilization() {
                ::core::result::Result::Ok(ok__) => {
                    *pulnetworkutilization = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UserIdentity<Impl: IWdsTransportClient_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbszuseridentity: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).UserIdentity() {
                ::core::result::Result::Ok(ok__) => {
                    *pbszuseridentity = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Disconnect<Impl: IWdsTransportClient_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, disconnectiontype: WDSTRANSPORT_DISCONNECT_TYPE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Disconnect(::core::mem::transmute_copy(&disconnectiontype)).into()
        }
        Self {
            base: super::Com::IDispatch_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            Session: Session::<Impl, IMPL_OFFSET>,
            Id: Id::<Impl, IMPL_OFFSET>,
            Name: Name::<Impl, IMPL_OFFSET>,
            MacAddress: MacAddress::<Impl, IMPL_OFFSET>,
            IpAddress: IpAddress::<Impl, IMPL_OFFSET>,
            PercentCompletion: PercentCompletion::<Impl, IMPL_OFFSET>,
            JoinDuration: JoinDuration::<Impl, IMPL_OFFSET>,
            CpuUtilization: CpuUtilization::<Impl, IMPL_OFFSET>,
            MemoryUtilization: MemoryUtilization::<Impl, IMPL_OFFSET>,
            NetworkUtilization: NetworkUtilization::<Impl, IMPL_OFFSET>,
            UserIdentity: UserIdentity::<Impl, IMPL_OFFSET>,
            Disconnect: Disconnect::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWdsTransportClient as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IWdsTransportCollection_Impl: Sized + super::Com::IDispatch_Impl {
    fn Count(&mut self) -> ::windows::core::Result<u32>;
    fn Item(&mut self, ulindex: u32) -> ::windows::core::Result<super::Com::IDispatch>;
    fn _NewEnum(&mut self) -> ::windows::core::Result<::windows::core::IUnknown>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IWdsTransportCollection_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWdsTransportCollection_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWdsTransportCollection_Vtbl {
        unsafe extern "system" fn Count<Impl: IWdsTransportCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pulcount: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Count() {
                ::core::result::Result::Ok(ok__) => {
                    *pulcount = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Item<Impl: IWdsTransportCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ulindex: u32, ppval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Item(::core::mem::transmute_copy(&ulindex)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn _NewEnum<Impl: IWdsTransportCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppval: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this)._NewEnum() {
                ::core::result::Result::Ok(ok__) => {
                    *ppval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: super::Com::IDispatch_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            Count: Count::<Impl, IMPL_OFFSET>,
            Item: Item::<Impl, IMPL_OFFSET>,
            _NewEnum: _NewEnum::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWdsTransportCollection as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IWdsTransportConfigurationManager_Impl: Sized + super::Com::IDispatch_Impl {
    fn ServicePolicy(&mut self) -> ::windows::core::Result<IWdsTransportServicePolicy>;
    fn DiagnosticsPolicy(&mut self) -> ::windows::core::Result<IWdsTransportDiagnosticsPolicy>;
    fn WdsTransportServicesRunning(&mut self, brealtimestatus: i16) -> ::windows::core::Result<i16>;
    fn EnableWdsTransportServices(&mut self) -> ::windows::core::Result<()>;
    fn DisableWdsTransportServices(&mut self) -> ::windows::core::Result<()>;
    fn StartWdsTransportServices(&mut self) -> ::windows::core::Result<()>;
    fn StopWdsTransportServices(&mut self) -> ::windows::core::Result<()>;
    fn RestartWdsTransportServices(&mut self) -> ::windows::core::Result<()>;
    fn NotifyWdsTransportServices(&mut self, servicenotification: WDSTRANSPORT_SERVICE_NOTIFICATION) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IWdsTransportConfigurationManager_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWdsTransportConfigurationManager_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWdsTransportConfigurationManager_Vtbl {
        unsafe extern "system" fn ServicePolicy<Impl: IWdsTransportConfigurationManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppwdstransportservicepolicy: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ServicePolicy() {
                ::core::result::Result::Ok(ok__) => {
                    *ppwdstransportservicepolicy = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DiagnosticsPolicy<Impl: IWdsTransportConfigurationManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppwdstransportdiagnosticspolicy: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DiagnosticsPolicy() {
                ::core::result::Result::Ok(ok__) => {
                    *ppwdstransportdiagnosticspolicy = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn WdsTransportServicesRunning<Impl: IWdsTransportConfigurationManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, brealtimestatus: i16, pbservicesrunning: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).WdsTransportServicesRunning(::core::mem::transmute_copy(&brealtimestatus)) {
                ::core::result::Result::Ok(ok__) => {
                    *pbservicesrunning = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnableWdsTransportServices<Impl: IWdsTransportConfigurationManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).EnableWdsTransportServices().into()
        }
        unsafe extern "system" fn DisableWdsTransportServices<Impl: IWdsTransportConfigurationManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).DisableWdsTransportServices().into()
        }
        unsafe extern "system" fn StartWdsTransportServices<Impl: IWdsTransportConfigurationManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).StartWdsTransportServices().into()
        }
        unsafe extern "system" fn StopWdsTransportServices<Impl: IWdsTransportConfigurationManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).StopWdsTransportServices().into()
        }
        unsafe extern "system" fn RestartWdsTransportServices<Impl: IWdsTransportConfigurationManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RestartWdsTransportServices().into()
        }
        unsafe extern "system" fn NotifyWdsTransportServices<Impl: IWdsTransportConfigurationManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, servicenotification: WDSTRANSPORT_SERVICE_NOTIFICATION) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).NotifyWdsTransportServices(::core::mem::transmute_copy(&servicenotification)).into()
        }
        Self {
            base: super::Com::IDispatch_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            ServicePolicy: ServicePolicy::<Impl, IMPL_OFFSET>,
            DiagnosticsPolicy: DiagnosticsPolicy::<Impl, IMPL_OFFSET>,
            WdsTransportServicesRunning: WdsTransportServicesRunning::<Impl, IMPL_OFFSET>,
            EnableWdsTransportServices: EnableWdsTransportServices::<Impl, IMPL_OFFSET>,
            DisableWdsTransportServices: DisableWdsTransportServices::<Impl, IMPL_OFFSET>,
            StartWdsTransportServices: StartWdsTransportServices::<Impl, IMPL_OFFSET>,
            StopWdsTransportServices: StopWdsTransportServices::<Impl, IMPL_OFFSET>,
            RestartWdsTransportServices: RestartWdsTransportServices::<Impl, IMPL_OFFSET>,
            NotifyWdsTransportServices: NotifyWdsTransportServices::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWdsTransportConfigurationManager as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IWdsTransportConfigurationManager2_Impl: Sized + super::Com::IDispatch_Impl + IWdsTransportConfigurationManager_Impl {
    fn MulticastSessionPolicy(&mut self) -> ::windows::core::Result<IWdsTransportMulticastSessionPolicy>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IWdsTransportConfigurationManager2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWdsTransportConfigurationManager2_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWdsTransportConfigurationManager2_Vtbl {
        unsafe extern "system" fn MulticastSessionPolicy<Impl: IWdsTransportConfigurationManager2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppwdstransportmulticastsessionpolicy: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MulticastSessionPolicy() {
                ::core::result::Result::Ok(ok__) => {
                    *ppwdstransportmulticastsessionpolicy = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: IWdsTransportConfigurationManager_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            MulticastSessionPolicy: MulticastSessionPolicy::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWdsTransportConfigurationManager2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IWdsTransportContent_Impl: Sized + super::Com::IDispatch_Impl {
    fn Namespace(&mut self) -> ::windows::core::Result<IWdsTransportNamespace>;
    fn Id(&mut self) -> ::windows::core::Result<u32>;
    fn Name(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn RetrieveSessions(&mut self) -> ::windows::core::Result<IWdsTransportCollection>;
    fn Terminate(&mut self) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IWdsTransportContent_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWdsTransportContent_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWdsTransportContent_Vtbl {
        unsafe extern "system" fn Namespace<Impl: IWdsTransportContent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppwdstransportnamespace: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Namespace() {
                ::core::result::Result::Ok(ok__) => {
                    *ppwdstransportnamespace = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Id<Impl: IWdsTransportContent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pulid: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Id() {
                ::core::result::Result::Ok(ok__) => {
                    *pulid = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Name<Impl: IWdsTransportContent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbszname: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Name() {
                ::core::result::Result::Ok(ok__) => {
                    *pbszname = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RetrieveSessions<Impl: IWdsTransportContent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppwdstransportsessions: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RetrieveSessions() {
                ::core::result::Result::Ok(ok__) => {
                    *ppwdstransportsessions = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Terminate<Impl: IWdsTransportContent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Terminate().into()
        }
        Self {
            base: super::Com::IDispatch_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            Namespace: Namespace::<Impl, IMPL_OFFSET>,
            Id: Id::<Impl, IMPL_OFFSET>,
            Name: Name::<Impl, IMPL_OFFSET>,
            RetrieveSessions: RetrieveSessions::<Impl, IMPL_OFFSET>,
            Terminate: Terminate::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWdsTransportContent as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IWdsTransportContentProvider_Impl: Sized + super::Com::IDispatch_Impl {
    fn Name(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn Description(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn FilePath(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn InitializationRoutine(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IWdsTransportContentProvider_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWdsTransportContentProvider_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWdsTransportContentProvider_Vtbl {
        unsafe extern "system" fn Name<Impl: IWdsTransportContentProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbszname: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Name() {
                ::core::result::Result::Ok(ok__) => {
                    *pbszname = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Description<Impl: IWdsTransportContentProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbszdescription: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Description() {
                ::core::result::Result::Ok(ok__) => {
                    *pbszdescription = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FilePath<Impl: IWdsTransportContentProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbszfilepath: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FilePath() {
                ::core::result::Result::Ok(ok__) => {
                    *pbszfilepath = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn InitializationRoutine<Impl: IWdsTransportContentProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbszinitializationroutine: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).InitializationRoutine() {
                ::core::result::Result::Ok(ok__) => {
                    *pbszinitializationroutine = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: super::Com::IDispatch_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            Name: Name::<Impl, IMPL_OFFSET>,
            Description: Description::<Impl, IMPL_OFFSET>,
            FilePath: FilePath::<Impl, IMPL_OFFSET>,
            InitializationRoutine: InitializationRoutine::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWdsTransportContentProvider as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IWdsTransportDiagnosticsPolicy_Impl: Sized + super::Com::IDispatch_Impl + IWdsTransportCacheable_Impl {
    fn Enabled(&mut self) -> ::windows::core::Result<i16>;
    fn SetEnabled(&mut self, benabled: i16) -> ::windows::core::Result<()>;
    fn Components(&mut self) -> ::windows::core::Result<u32>;
    fn SetComponents(&mut self, ulcomponents: u32) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IWdsTransportDiagnosticsPolicy_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWdsTransportDiagnosticsPolicy_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWdsTransportDiagnosticsPolicy_Vtbl {
        unsafe extern "system" fn Enabled<Impl: IWdsTransportDiagnosticsPolicy_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbenabled: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Enabled() {
                ::core::result::Result::Ok(ok__) => {
                    *pbenabled = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetEnabled<Impl: IWdsTransportDiagnosticsPolicy_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, benabled: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetEnabled(::core::mem::transmute_copy(&benabled)).into()
        }
        unsafe extern "system" fn Components<Impl: IWdsTransportDiagnosticsPolicy_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pulcomponents: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Components() {
                ::core::result::Result::Ok(ok__) => {
                    *pulcomponents = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetComponents<Impl: IWdsTransportDiagnosticsPolicy_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ulcomponents: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetComponents(::core::mem::transmute_copy(&ulcomponents)).into()
        }
        Self {
            base: IWdsTransportCacheable_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            Enabled: Enabled::<Impl, IMPL_OFFSET>,
            SetEnabled: SetEnabled::<Impl, IMPL_OFFSET>,
            Components: Components::<Impl, IMPL_OFFSET>,
            SetComponents: SetComponents::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWdsTransportDiagnosticsPolicy as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IWdsTransportManager_Impl: Sized + super::Com::IDispatch_Impl {
    fn GetWdsTransportServer(&mut self, bszservername: super::super::Foundation::BSTR) -> ::windows::core::Result<IWdsTransportServer>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IWdsTransportManager_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWdsTransportManager_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWdsTransportManager_Vtbl {
        unsafe extern "system" fn GetWdsTransportServer<Impl: IWdsTransportManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bszservername: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ppwdstransportserver: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetWdsTransportServer(::core::mem::transmute_copy(&bszservername)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppwdstransportserver = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: super::Com::IDispatch_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            GetWdsTransportServer: GetWdsTransportServer::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWdsTransportManager as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IWdsTransportMulticastSessionPolicy_Impl: Sized + super::Com::IDispatch_Impl + IWdsTransportCacheable_Impl {
    fn SlowClientHandling(&mut self) -> ::windows::core::Result<WDSTRANSPORT_SLOW_CLIENT_HANDLING_TYPE>;
    fn SetSlowClientHandling(&mut self, slowclienthandling: WDSTRANSPORT_SLOW_CLIENT_HANDLING_TYPE) -> ::windows::core::Result<()>;
    fn AutoDisconnectThreshold(&mut self) -> ::windows::core::Result<u32>;
    fn SetAutoDisconnectThreshold(&mut self, ulthreshold: u32) -> ::windows::core::Result<()>;
    fn MultistreamStreamCount(&mut self) -> ::windows::core::Result<u32>;
    fn SetMultistreamStreamCount(&mut self, ulstreamcount: u32) -> ::windows::core::Result<()>;
    fn SlowClientFallback(&mut self) -> ::windows::core::Result<i16>;
    fn SetSlowClientFallback(&mut self, bclientfallback: i16) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IWdsTransportMulticastSessionPolicy_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWdsTransportMulticastSessionPolicy_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWdsTransportMulticastSessionPolicy_Vtbl {
        unsafe extern "system" fn SlowClientHandling<Impl: IWdsTransportMulticastSessionPolicy_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pslowclienthandling: *mut WDSTRANSPORT_SLOW_CLIENT_HANDLING_TYPE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SlowClientHandling() {
                ::core::result::Result::Ok(ok__) => {
                    *pslowclienthandling = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSlowClientHandling<Impl: IWdsTransportMulticastSessionPolicy_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, slowclienthandling: WDSTRANSPORT_SLOW_CLIENT_HANDLING_TYPE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetSlowClientHandling(::core::mem::transmute_copy(&slowclienthandling)).into()
        }
        unsafe extern "system" fn AutoDisconnectThreshold<Impl: IWdsTransportMulticastSessionPolicy_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pulthreshold: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AutoDisconnectThreshold() {
                ::core::result::Result::Ok(ok__) => {
                    *pulthreshold = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAutoDisconnectThreshold<Impl: IWdsTransportMulticastSessionPolicy_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ulthreshold: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetAutoDisconnectThreshold(::core::mem::transmute_copy(&ulthreshold)).into()
        }
        unsafe extern "system" fn MultistreamStreamCount<Impl: IWdsTransportMulticastSessionPolicy_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pulstreamcount: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MultistreamStreamCount() {
                ::core::result::Result::Ok(ok__) => {
                    *pulstreamcount = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMultistreamStreamCount<Impl: IWdsTransportMulticastSessionPolicy_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ulstreamcount: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetMultistreamStreamCount(::core::mem::transmute_copy(&ulstreamcount)).into()
        }
        unsafe extern "system" fn SlowClientFallback<Impl: IWdsTransportMulticastSessionPolicy_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbclientfallback: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SlowClientFallback() {
                ::core::result::Result::Ok(ok__) => {
                    *pbclientfallback = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSlowClientFallback<Impl: IWdsTransportMulticastSessionPolicy_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bclientfallback: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetSlowClientFallback(::core::mem::transmute_copy(&bclientfallback)).into()
        }
        Self {
            base: IWdsTransportCacheable_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            SlowClientHandling: SlowClientHandling::<Impl, IMPL_OFFSET>,
            SetSlowClientHandling: SetSlowClientHandling::<Impl, IMPL_OFFSET>,
            AutoDisconnectThreshold: AutoDisconnectThreshold::<Impl, IMPL_OFFSET>,
            SetAutoDisconnectThreshold: SetAutoDisconnectThreshold::<Impl, IMPL_OFFSET>,
            MultistreamStreamCount: MultistreamStreamCount::<Impl, IMPL_OFFSET>,
            SetMultistreamStreamCount: SetMultistreamStreamCount::<Impl, IMPL_OFFSET>,
            SlowClientFallback: SlowClientFallback::<Impl, IMPL_OFFSET>,
            SetSlowClientFallback: SetSlowClientFallback::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWdsTransportMulticastSessionPolicy as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IWdsTransportNamespace_Impl: Sized + super::Com::IDispatch_Impl {
    fn Type(&mut self) -> ::windows::core::Result<WDSTRANSPORT_NAMESPACE_TYPE>;
    fn Id(&mut self) -> ::windows::core::Result<u32>;
    fn Name(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetName(&mut self, bszname: super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn FriendlyName(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetFriendlyName(&mut self, bszfriendlyname: super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn Description(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetDescription(&mut self, bszdescription: super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn ContentProvider(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetContentProvider(&mut self, bszcontentprovider: super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn Configuration(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetConfiguration(&mut self, bszconfiguration: super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn Registered(&mut self) -> ::windows::core::Result<i16>;
    fn Tombstoned(&mut self) -> ::windows::core::Result<i16>;
    fn TombstoneTime(&mut self) -> ::windows::core::Result<f64>;
    fn TransmissionStarted(&mut self) -> ::windows::core::Result<i16>;
    fn Register(&mut self) -> ::windows::core::Result<()>;
    fn Deregister(&mut self, bterminatesessions: i16) -> ::windows::core::Result<()>;
    fn Clone(&mut self) -> ::windows::core::Result<IWdsTransportNamespace>;
    fn Refresh(&mut self) -> ::windows::core::Result<()>;
    fn RetrieveContents(&mut self) -> ::windows::core::Result<IWdsTransportCollection>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IWdsTransportNamespace_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWdsTransportNamespace_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWdsTransportNamespace_Vtbl {
        unsafe extern "system" fn Type<Impl: IWdsTransportNamespace_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ptype: *mut WDSTRANSPORT_NAMESPACE_TYPE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Type() {
                ::core::result::Result::Ok(ok__) => {
                    *ptype = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Id<Impl: IWdsTransportNamespace_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pulid: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Id() {
                ::core::result::Result::Ok(ok__) => {
                    *pulid = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Name<Impl: IWdsTransportNamespace_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbszname: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Name() {
                ::core::result::Result::Ok(ok__) => {
                    *pbszname = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetName<Impl: IWdsTransportNamespace_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bszname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetName(::core::mem::transmute_copy(&bszname)).into()
        }
        unsafe extern "system" fn FriendlyName<Impl: IWdsTransportNamespace_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbszfriendlyname: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FriendlyName() {
                ::core::result::Result::Ok(ok__) => {
                    *pbszfriendlyname = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetFriendlyName<Impl: IWdsTransportNamespace_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bszfriendlyname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetFriendlyName(::core::mem::transmute_copy(&bszfriendlyname)).into()
        }
        unsafe extern "system" fn Description<Impl: IWdsTransportNamespace_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbszdescription: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Description() {
                ::core::result::Result::Ok(ok__) => {
                    *pbszdescription = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDescription<Impl: IWdsTransportNamespace_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bszdescription: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetDescription(::core::mem::transmute_copy(&bszdescription)).into()
        }
        unsafe extern "system" fn ContentProvider<Impl: IWdsTransportNamespace_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbszcontentprovider: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ContentProvider() {
                ::core::result::Result::Ok(ok__) => {
                    *pbszcontentprovider = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetContentProvider<Impl: IWdsTransportNamespace_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bszcontentprovider: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetContentProvider(::core::mem::transmute_copy(&bszcontentprovider)).into()
        }
        unsafe extern "system" fn Configuration<Impl: IWdsTransportNamespace_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbszconfiguration: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Configuration() {
                ::core::result::Result::Ok(ok__) => {
                    *pbszconfiguration = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetConfiguration<Impl: IWdsTransportNamespace_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bszconfiguration: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetConfiguration(::core::mem::transmute_copy(&bszconfiguration)).into()
        }
        unsafe extern "system" fn Registered<Impl: IWdsTransportNamespace_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbregistered: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Registered() {
                ::core::result::Result::Ok(ok__) => {
                    *pbregistered = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Tombstoned<Impl: IWdsTransportNamespace_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbtombstoned: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Tombstoned() {
                ::core::result::Result::Ok(ok__) => {
                    *pbtombstoned = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TombstoneTime<Impl: IWdsTransportNamespace_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ptombstonetime: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TombstoneTime() {
                ::core::result::Result::Ok(ok__) => {
                    *ptombstonetime = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TransmissionStarted<Impl: IWdsTransportNamespace_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbtransmissionstarted: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TransmissionStarted() {
                ::core::result::Result::Ok(ok__) => {
                    *pbtransmissionstarted = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Register<Impl: IWdsTransportNamespace_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Register().into()
        }
        unsafe extern "system" fn Deregister<Impl: IWdsTransportNamespace_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bterminatesessions: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Deregister(::core::mem::transmute_copy(&bterminatesessions)).into()
        }
        unsafe extern "system" fn Clone<Impl: IWdsTransportNamespace_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppwdstransportnamespaceclone: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Clone() {
                ::core::result::Result::Ok(ok__) => {
                    *ppwdstransportnamespaceclone = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Refresh<Impl: IWdsTransportNamespace_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Refresh().into()
        }
        unsafe extern "system" fn RetrieveContents<Impl: IWdsTransportNamespace_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppwdstransportcontents: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RetrieveContents() {
                ::core::result::Result::Ok(ok__) => {
                    *ppwdstransportcontents = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: super::Com::IDispatch_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            Type: Type::<Impl, IMPL_OFFSET>,
            Id: Id::<Impl, IMPL_OFFSET>,
            Name: Name::<Impl, IMPL_OFFSET>,
            SetName: SetName::<Impl, IMPL_OFFSET>,
            FriendlyName: FriendlyName::<Impl, IMPL_OFFSET>,
            SetFriendlyName: SetFriendlyName::<Impl, IMPL_OFFSET>,
            Description: Description::<Impl, IMPL_OFFSET>,
            SetDescription: SetDescription::<Impl, IMPL_OFFSET>,
            ContentProvider: ContentProvider::<Impl, IMPL_OFFSET>,
            SetContentProvider: SetContentProvider::<Impl, IMPL_OFFSET>,
            Configuration: Configuration::<Impl, IMPL_OFFSET>,
            SetConfiguration: SetConfiguration::<Impl, IMPL_OFFSET>,
            Registered: Registered::<Impl, IMPL_OFFSET>,
            Tombstoned: Tombstoned::<Impl, IMPL_OFFSET>,
            TombstoneTime: TombstoneTime::<Impl, IMPL_OFFSET>,
            TransmissionStarted: TransmissionStarted::<Impl, IMPL_OFFSET>,
            Register: Register::<Impl, IMPL_OFFSET>,
            Deregister: Deregister::<Impl, IMPL_OFFSET>,
            Clone: Clone::<Impl, IMPL_OFFSET>,
            Refresh: Refresh::<Impl, IMPL_OFFSET>,
            RetrieveContents: RetrieveContents::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWdsTransportNamespace as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IWdsTransportNamespaceAutoCast_Impl: Sized + super::Com::IDispatch_Impl + IWdsTransportNamespace_Impl {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IWdsTransportNamespaceAutoCast_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWdsTransportNamespaceAutoCast_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWdsTransportNamespaceAutoCast_Vtbl {
        Self { base: IWdsTransportNamespace_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWdsTransportNamespaceAutoCast as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IWdsTransportNamespaceManager_Impl: Sized + super::Com::IDispatch_Impl {
    fn CreateNamespace(&mut self, namespacetype: WDSTRANSPORT_NAMESPACE_TYPE, bsznamespacename: super::super::Foundation::BSTR, bszcontentprovider: super::super::Foundation::BSTR, bszconfiguration: super::super::Foundation::BSTR) -> ::windows::core::Result<IWdsTransportNamespace>;
    fn RetrieveNamespace(&mut self, bsznamespacename: super::super::Foundation::BSTR) -> ::windows::core::Result<IWdsTransportNamespace>;
    fn RetrieveNamespaces(&mut self, bszcontentprovider: super::super::Foundation::BSTR, bsznamespacename: super::super::Foundation::BSTR, bincludetombstones: i16) -> ::windows::core::Result<IWdsTransportCollection>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IWdsTransportNamespaceManager_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWdsTransportNamespaceManager_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWdsTransportNamespaceManager_Vtbl {
        unsafe extern "system" fn CreateNamespace<Impl: IWdsTransportNamespaceManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, namespacetype: WDSTRANSPORT_NAMESPACE_TYPE, bsznamespacename: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bszcontentprovider: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bszconfiguration: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ppwdstransportnamespace: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateNamespace(::core::mem::transmute_copy(&namespacetype), ::core::mem::transmute_copy(&bsznamespacename), ::core::mem::transmute_copy(&bszcontentprovider), ::core::mem::transmute_copy(&bszconfiguration)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppwdstransportnamespace = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RetrieveNamespace<Impl: IWdsTransportNamespaceManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bsznamespacename: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ppwdstransportnamespace: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RetrieveNamespace(::core::mem::transmute_copy(&bsznamespacename)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppwdstransportnamespace = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RetrieveNamespaces<Impl: IWdsTransportNamespaceManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bszcontentprovider: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bsznamespacename: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bincludetombstones: i16, ppwdstransportnamespaces: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RetrieveNamespaces(::core::mem::transmute_copy(&bszcontentprovider), ::core::mem::transmute_copy(&bsznamespacename), ::core::mem::transmute_copy(&bincludetombstones)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppwdstransportnamespaces = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: super::Com::IDispatch_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            CreateNamespace: CreateNamespace::<Impl, IMPL_OFFSET>,
            RetrieveNamespace: RetrieveNamespace::<Impl, IMPL_OFFSET>,
            RetrieveNamespaces: RetrieveNamespaces::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWdsTransportNamespaceManager as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IWdsTransportNamespaceScheduledCast_Impl: Sized + super::Com::IDispatch_Impl + IWdsTransportNamespace_Impl {
    fn StartTransmission(&mut self) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IWdsTransportNamespaceScheduledCast_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWdsTransportNamespaceScheduledCast_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWdsTransportNamespaceScheduledCast_Vtbl {
        unsafe extern "system" fn StartTransmission<Impl: IWdsTransportNamespaceScheduledCast_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).StartTransmission().into()
        }
        Self { base: IWdsTransportNamespace_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(), StartTransmission: StartTransmission::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWdsTransportNamespaceScheduledCast as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IWdsTransportNamespaceScheduledCastAutoStart_Impl: Sized + super::Com::IDispatch_Impl + IWdsTransportNamespace_Impl + IWdsTransportNamespaceScheduledCast_Impl {
    fn MinimumClients(&mut self) -> ::windows::core::Result<u32>;
    fn SetMinimumClients(&mut self, ulminimumclients: u32) -> ::windows::core::Result<()>;
    fn StartTime(&mut self) -> ::windows::core::Result<f64>;
    fn SetStartTime(&mut self, starttime: f64) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IWdsTransportNamespaceScheduledCastAutoStart_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWdsTransportNamespaceScheduledCastAutoStart_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWdsTransportNamespaceScheduledCastAutoStart_Vtbl {
        unsafe extern "system" fn MinimumClients<Impl: IWdsTransportNamespaceScheduledCastAutoStart_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pulminimumclients: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MinimumClients() {
                ::core::result::Result::Ok(ok__) => {
                    *pulminimumclients = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMinimumClients<Impl: IWdsTransportNamespaceScheduledCastAutoStart_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ulminimumclients: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetMinimumClients(::core::mem::transmute_copy(&ulminimumclients)).into()
        }
        unsafe extern "system" fn StartTime<Impl: IWdsTransportNamespaceScheduledCastAutoStart_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pstarttime: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).StartTime() {
                ::core::result::Result::Ok(ok__) => {
                    *pstarttime = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetStartTime<Impl: IWdsTransportNamespaceScheduledCastAutoStart_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, starttime: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetStartTime(::core::mem::transmute_copy(&starttime)).into()
        }
        Self {
            base: IWdsTransportNamespaceScheduledCast_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            MinimumClients: MinimumClients::<Impl, IMPL_OFFSET>,
            SetMinimumClients: SetMinimumClients::<Impl, IMPL_OFFSET>,
            StartTime: StartTime::<Impl, IMPL_OFFSET>,
            SetStartTime: SetStartTime::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWdsTransportNamespaceScheduledCastAutoStart as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IWdsTransportNamespaceScheduledCastManualStart_Impl: Sized + super::Com::IDispatch_Impl + IWdsTransportNamespace_Impl + IWdsTransportNamespaceScheduledCast_Impl {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IWdsTransportNamespaceScheduledCastManualStart_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWdsTransportNamespaceScheduledCastManualStart_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWdsTransportNamespaceScheduledCastManualStart_Vtbl {
        Self { base: IWdsTransportNamespaceScheduledCast_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWdsTransportNamespaceScheduledCastManualStart as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IWdsTransportServer_Impl: Sized + super::Com::IDispatch_Impl {
    fn Name(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetupManager(&mut self) -> ::windows::core::Result<IWdsTransportSetupManager>;
    fn ConfigurationManager(&mut self) -> ::windows::core::Result<IWdsTransportConfigurationManager>;
    fn NamespaceManager(&mut self) -> ::windows::core::Result<IWdsTransportNamespaceManager>;
    fn DisconnectClient(&mut self, ulclientid: u32, disconnectiontype: WDSTRANSPORT_DISCONNECT_TYPE) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IWdsTransportServer_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWdsTransportServer_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWdsTransportServer_Vtbl {
        unsafe extern "system" fn Name<Impl: IWdsTransportServer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbszname: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Name() {
                ::core::result::Result::Ok(ok__) => {
                    *pbszname = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetupManager<Impl: IWdsTransportServer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppwdstransportsetupmanager: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetupManager() {
                ::core::result::Result::Ok(ok__) => {
                    *ppwdstransportsetupmanager = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ConfigurationManager<Impl: IWdsTransportServer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppwdstransportconfigurationmanager: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ConfigurationManager() {
                ::core::result::Result::Ok(ok__) => {
                    *ppwdstransportconfigurationmanager = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn NamespaceManager<Impl: IWdsTransportServer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppwdstransportnamespacemanager: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).NamespaceManager() {
                ::core::result::Result::Ok(ok__) => {
                    *ppwdstransportnamespacemanager = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DisconnectClient<Impl: IWdsTransportServer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ulclientid: u32, disconnectiontype: WDSTRANSPORT_DISCONNECT_TYPE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).DisconnectClient(::core::mem::transmute_copy(&ulclientid), ::core::mem::transmute_copy(&disconnectiontype)).into()
        }
        Self {
            base: super::Com::IDispatch_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            Name: Name::<Impl, IMPL_OFFSET>,
            SetupManager: SetupManager::<Impl, IMPL_OFFSET>,
            ConfigurationManager: ConfigurationManager::<Impl, IMPL_OFFSET>,
            NamespaceManager: NamespaceManager::<Impl, IMPL_OFFSET>,
            DisconnectClient: DisconnectClient::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWdsTransportServer as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IWdsTransportServer2_Impl: Sized + super::Com::IDispatch_Impl + IWdsTransportServer_Impl {
    fn TftpManager(&mut self) -> ::windows::core::Result<IWdsTransportTftpManager>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IWdsTransportServer2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWdsTransportServer2_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWdsTransportServer2_Vtbl {
        unsafe extern "system" fn TftpManager<Impl: IWdsTransportServer2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppwdstransporttftpmanager: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TftpManager() {
                ::core::result::Result::Ok(ok__) => {
                    *ppwdstransporttftpmanager = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: IWdsTransportServer_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(), TftpManager: TftpManager::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWdsTransportServer2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IWdsTransportServicePolicy_Impl: Sized + super::Com::IDispatch_Impl + IWdsTransportCacheable_Impl {
    fn IpAddressSource(&mut self, addresstype: WDSTRANSPORT_IP_ADDRESS_TYPE) -> ::windows::core::Result<WDSTRANSPORT_IP_ADDRESS_SOURCE_TYPE>;
    fn SetIpAddressSource(&mut self, addresstype: WDSTRANSPORT_IP_ADDRESS_TYPE, sourcetype: WDSTRANSPORT_IP_ADDRESS_SOURCE_TYPE) -> ::windows::core::Result<()>;
    fn StartIpAddress(&mut self, addresstype: WDSTRANSPORT_IP_ADDRESS_TYPE) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetStartIpAddress(&mut self, addresstype: WDSTRANSPORT_IP_ADDRESS_TYPE, bszstartipaddress: super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn EndIpAddress(&mut self, addresstype: WDSTRANSPORT_IP_ADDRESS_TYPE) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetEndIpAddress(&mut self, addresstype: WDSTRANSPORT_IP_ADDRESS_TYPE, bszendipaddress: super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn StartPort(&mut self) -> ::windows::core::Result<u32>;
    fn SetStartPort(&mut self, ulstartport: u32) -> ::windows::core::Result<()>;
    fn EndPort(&mut self) -> ::windows::core::Result<u32>;
    fn SetEndPort(&mut self, ulendport: u32) -> ::windows::core::Result<()>;
    fn NetworkProfile(&mut self) -> ::windows::core::Result<WDSTRANSPORT_NETWORK_PROFILE_TYPE>;
    fn SetNetworkProfile(&mut self, profiletype: WDSTRANSPORT_NETWORK_PROFILE_TYPE) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IWdsTransportServicePolicy_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWdsTransportServicePolicy_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWdsTransportServicePolicy_Vtbl {
        unsafe extern "system" fn IpAddressSource<Impl: IWdsTransportServicePolicy_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, addresstype: WDSTRANSPORT_IP_ADDRESS_TYPE, psourcetype: *mut WDSTRANSPORT_IP_ADDRESS_SOURCE_TYPE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IpAddressSource(::core::mem::transmute_copy(&addresstype)) {
                ::core::result::Result::Ok(ok__) => {
                    *psourcetype = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIpAddressSource<Impl: IWdsTransportServicePolicy_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, addresstype: WDSTRANSPORT_IP_ADDRESS_TYPE, sourcetype: WDSTRANSPORT_IP_ADDRESS_SOURCE_TYPE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetIpAddressSource(::core::mem::transmute_copy(&addresstype), ::core::mem::transmute_copy(&sourcetype)).into()
        }
        unsafe extern "system" fn StartIpAddress<Impl: IWdsTransportServicePolicy_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, addresstype: WDSTRANSPORT_IP_ADDRESS_TYPE, pbszstartipaddress: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).StartIpAddress(::core::mem::transmute_copy(&addresstype)) {
                ::core::result::Result::Ok(ok__) => {
                    *pbszstartipaddress = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetStartIpAddress<Impl: IWdsTransportServicePolicy_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, addresstype: WDSTRANSPORT_IP_ADDRESS_TYPE, bszstartipaddress: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetStartIpAddress(::core::mem::transmute_copy(&addresstype), ::core::mem::transmute_copy(&bszstartipaddress)).into()
        }
        unsafe extern "system" fn EndIpAddress<Impl: IWdsTransportServicePolicy_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, addresstype: WDSTRANSPORT_IP_ADDRESS_TYPE, pbszendipaddress: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EndIpAddress(::core::mem::transmute_copy(&addresstype)) {
                ::core::result::Result::Ok(ok__) => {
                    *pbszendipaddress = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetEndIpAddress<Impl: IWdsTransportServicePolicy_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, addresstype: WDSTRANSPORT_IP_ADDRESS_TYPE, bszendipaddress: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetEndIpAddress(::core::mem::transmute_copy(&addresstype), ::core::mem::transmute_copy(&bszendipaddress)).into()
        }
        unsafe extern "system" fn StartPort<Impl: IWdsTransportServicePolicy_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pulstartport: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).StartPort() {
                ::core::result::Result::Ok(ok__) => {
                    *pulstartport = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetStartPort<Impl: IWdsTransportServicePolicy_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ulstartport: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetStartPort(::core::mem::transmute_copy(&ulstartport)).into()
        }
        unsafe extern "system" fn EndPort<Impl: IWdsTransportServicePolicy_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pulendport: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EndPort() {
                ::core::result::Result::Ok(ok__) => {
                    *pulendport = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetEndPort<Impl: IWdsTransportServicePolicy_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ulendport: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetEndPort(::core::mem::transmute_copy(&ulendport)).into()
        }
        unsafe extern "system" fn NetworkProfile<Impl: IWdsTransportServicePolicy_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pprofiletype: *mut WDSTRANSPORT_NETWORK_PROFILE_TYPE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).NetworkProfile() {
                ::core::result::Result::Ok(ok__) => {
                    *pprofiletype = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetNetworkProfile<Impl: IWdsTransportServicePolicy_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, profiletype: WDSTRANSPORT_NETWORK_PROFILE_TYPE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetNetworkProfile(::core::mem::transmute_copy(&profiletype)).into()
        }
        Self {
            base: IWdsTransportCacheable_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            IpAddressSource: IpAddressSource::<Impl, IMPL_OFFSET>,
            SetIpAddressSource: SetIpAddressSource::<Impl, IMPL_OFFSET>,
            StartIpAddress: StartIpAddress::<Impl, IMPL_OFFSET>,
            SetStartIpAddress: SetStartIpAddress::<Impl, IMPL_OFFSET>,
            EndIpAddress: EndIpAddress::<Impl, IMPL_OFFSET>,
            SetEndIpAddress: SetEndIpAddress::<Impl, IMPL_OFFSET>,
            StartPort: StartPort::<Impl, IMPL_OFFSET>,
            SetStartPort: SetStartPort::<Impl, IMPL_OFFSET>,
            EndPort: EndPort::<Impl, IMPL_OFFSET>,
            SetEndPort: SetEndPort::<Impl, IMPL_OFFSET>,
            NetworkProfile: NetworkProfile::<Impl, IMPL_OFFSET>,
            SetNetworkProfile: SetNetworkProfile::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWdsTransportServicePolicy as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IWdsTransportServicePolicy2_Impl: Sized + super::Com::IDispatch_Impl + IWdsTransportCacheable_Impl + IWdsTransportServicePolicy_Impl {
    fn UdpPortPolicy(&mut self) -> ::windows::core::Result<WDSTRANSPORT_UDP_PORT_POLICY>;
    fn SetUdpPortPolicy(&mut self, udpportpolicy: WDSTRANSPORT_UDP_PORT_POLICY) -> ::windows::core::Result<()>;
    fn TftpMaximumBlockSize(&mut self) -> ::windows::core::Result<u32>;
    fn SetTftpMaximumBlockSize(&mut self, ultftpmaximumblocksize: u32) -> ::windows::core::Result<()>;
    fn EnableTftpVariableWindowExtension(&mut self) -> ::windows::core::Result<i16>;
    fn SetEnableTftpVariableWindowExtension(&mut self, benabletftpvariablewindowextension: i16) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IWdsTransportServicePolicy2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWdsTransportServicePolicy2_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWdsTransportServicePolicy2_Vtbl {
        unsafe extern "system" fn UdpPortPolicy<Impl: IWdsTransportServicePolicy2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pudpportpolicy: *mut WDSTRANSPORT_UDP_PORT_POLICY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).UdpPortPolicy() {
                ::core::result::Result::Ok(ok__) => {
                    *pudpportpolicy = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetUdpPortPolicy<Impl: IWdsTransportServicePolicy2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, udpportpolicy: WDSTRANSPORT_UDP_PORT_POLICY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetUdpPortPolicy(::core::mem::transmute_copy(&udpportpolicy)).into()
        }
        unsafe extern "system" fn TftpMaximumBlockSize<Impl: IWdsTransportServicePolicy2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pultftpmaximumblocksize: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TftpMaximumBlockSize() {
                ::core::result::Result::Ok(ok__) => {
                    *pultftpmaximumblocksize = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetTftpMaximumBlockSize<Impl: IWdsTransportServicePolicy2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ultftpmaximumblocksize: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetTftpMaximumBlockSize(::core::mem::transmute_copy(&ultftpmaximumblocksize)).into()
        }
        unsafe extern "system" fn EnableTftpVariableWindowExtension<Impl: IWdsTransportServicePolicy2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbenabletftpvariablewindowextension: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EnableTftpVariableWindowExtension() {
                ::core::result::Result::Ok(ok__) => {
                    *pbenabletftpvariablewindowextension = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetEnableTftpVariableWindowExtension<Impl: IWdsTransportServicePolicy2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, benabletftpvariablewindowextension: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetEnableTftpVariableWindowExtension(::core::mem::transmute_copy(&benabletftpvariablewindowextension)).into()
        }
        Self {
            base: IWdsTransportServicePolicy_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            UdpPortPolicy: UdpPortPolicy::<Impl, IMPL_OFFSET>,
            SetUdpPortPolicy: SetUdpPortPolicy::<Impl, IMPL_OFFSET>,
            TftpMaximumBlockSize: TftpMaximumBlockSize::<Impl, IMPL_OFFSET>,
            SetTftpMaximumBlockSize: SetTftpMaximumBlockSize::<Impl, IMPL_OFFSET>,
            EnableTftpVariableWindowExtension: EnableTftpVariableWindowExtension::<Impl, IMPL_OFFSET>,
            SetEnableTftpVariableWindowExtension: SetEnableTftpVariableWindowExtension::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWdsTransportServicePolicy2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IWdsTransportSession_Impl: Sized + super::Com::IDispatch_Impl {
    fn Content(&mut self) -> ::windows::core::Result<IWdsTransportContent>;
    fn Id(&mut self) -> ::windows::core::Result<u32>;
    fn NetworkInterfaceName(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn NetworkInterfaceAddress(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn TransferRate(&mut self) -> ::windows::core::Result<u32>;
    fn MasterClientId(&mut self) -> ::windows::core::Result<u32>;
    fn RetrieveClients(&mut self) -> ::windows::core::Result<IWdsTransportCollection>;
    fn Terminate(&mut self) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IWdsTransportSession_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWdsTransportSession_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWdsTransportSession_Vtbl {
        unsafe extern "system" fn Content<Impl: IWdsTransportSession_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppwdstransportcontent: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Content() {
                ::core::result::Result::Ok(ok__) => {
                    *ppwdstransportcontent = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Id<Impl: IWdsTransportSession_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pulid: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Id() {
                ::core::result::Result::Ok(ok__) => {
                    *pulid = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn NetworkInterfaceName<Impl: IWdsTransportSession_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbsznetworkinterfacename: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).NetworkInterfaceName() {
                ::core::result::Result::Ok(ok__) => {
                    *pbsznetworkinterfacename = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn NetworkInterfaceAddress<Impl: IWdsTransportSession_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbsznetworkinterfaceaddress: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).NetworkInterfaceAddress() {
                ::core::result::Result::Ok(ok__) => {
                    *pbsznetworkinterfaceaddress = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TransferRate<Impl: IWdsTransportSession_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pultransferrate: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TransferRate() {
                ::core::result::Result::Ok(ok__) => {
                    *pultransferrate = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MasterClientId<Impl: IWdsTransportSession_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pulmasterclientid: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MasterClientId() {
                ::core::result::Result::Ok(ok__) => {
                    *pulmasterclientid = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RetrieveClients<Impl: IWdsTransportSession_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppwdstransportclients: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RetrieveClients() {
                ::core::result::Result::Ok(ok__) => {
                    *ppwdstransportclients = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Terminate<Impl: IWdsTransportSession_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Terminate().into()
        }
        Self {
            base: super::Com::IDispatch_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            Content: Content::<Impl, IMPL_OFFSET>,
            Id: Id::<Impl, IMPL_OFFSET>,
            NetworkInterfaceName: NetworkInterfaceName::<Impl, IMPL_OFFSET>,
            NetworkInterfaceAddress: NetworkInterfaceAddress::<Impl, IMPL_OFFSET>,
            TransferRate: TransferRate::<Impl, IMPL_OFFSET>,
            MasterClientId: MasterClientId::<Impl, IMPL_OFFSET>,
            RetrieveClients: RetrieveClients::<Impl, IMPL_OFFSET>,
            Terminate: Terminate::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWdsTransportSession as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IWdsTransportSetupManager_Impl: Sized + super::Com::IDispatch_Impl {
    fn Version(&mut self) -> ::windows::core::Result<u64>;
    fn InstalledFeatures(&mut self) -> ::windows::core::Result<u32>;
    fn Protocols(&mut self) -> ::windows::core::Result<u32>;
    fn RegisterContentProvider(&mut self, bszname: super::super::Foundation::BSTR, bszdescription: super::super::Foundation::BSTR, bszfilepath: super::super::Foundation::BSTR, bszinitializationroutine: super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn DeregisterContentProvider(&mut self, bszname: super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IWdsTransportSetupManager_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWdsTransportSetupManager_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWdsTransportSetupManager_Vtbl {
        unsafe extern "system" fn Version<Impl: IWdsTransportSetupManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pullversion: *mut u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Version() {
                ::core::result::Result::Ok(ok__) => {
                    *pullversion = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn InstalledFeatures<Impl: IWdsTransportSetupManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pulinstalledfeatures: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).InstalledFeatures() {
                ::core::result::Result::Ok(ok__) => {
                    *pulinstalledfeatures = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Protocols<Impl: IWdsTransportSetupManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pulprotocols: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Protocols() {
                ::core::result::Result::Ok(ok__) => {
                    *pulprotocols = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RegisterContentProvider<Impl: IWdsTransportSetupManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bszname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bszdescription: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bszfilepath: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bszinitializationroutine: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RegisterContentProvider(::core::mem::transmute_copy(&bszname), ::core::mem::transmute_copy(&bszdescription), ::core::mem::transmute_copy(&bszfilepath), ::core::mem::transmute_copy(&bszinitializationroutine)).into()
        }
        unsafe extern "system" fn DeregisterContentProvider<Impl: IWdsTransportSetupManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bszname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).DeregisterContentProvider(::core::mem::transmute_copy(&bszname)).into()
        }
        Self {
            base: super::Com::IDispatch_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            Version: Version::<Impl, IMPL_OFFSET>,
            InstalledFeatures: InstalledFeatures::<Impl, IMPL_OFFSET>,
            Protocols: Protocols::<Impl, IMPL_OFFSET>,
            RegisterContentProvider: RegisterContentProvider::<Impl, IMPL_OFFSET>,
            DeregisterContentProvider: DeregisterContentProvider::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWdsTransportSetupManager as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IWdsTransportSetupManager2_Impl: Sized + super::Com::IDispatch_Impl + IWdsTransportSetupManager_Impl {
    fn TftpCapabilities(&mut self) -> ::windows::core::Result<u32>;
    fn ContentProviders(&mut self) -> ::windows::core::Result<IWdsTransportCollection>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IWdsTransportSetupManager2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWdsTransportSetupManager2_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWdsTransportSetupManager2_Vtbl {
        unsafe extern "system" fn TftpCapabilities<Impl: IWdsTransportSetupManager2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pultftpcapabilities: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TftpCapabilities() {
                ::core::result::Result::Ok(ok__) => {
                    *pultftpcapabilities = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ContentProviders<Impl: IWdsTransportSetupManager2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppprovidercollection: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ContentProviders() {
                ::core::result::Result::Ok(ok__) => {
                    *ppprovidercollection = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: IWdsTransportSetupManager_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            TftpCapabilities: TftpCapabilities::<Impl, IMPL_OFFSET>,
            ContentProviders: ContentProviders::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWdsTransportSetupManager2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IWdsTransportTftpClient_Impl: Sized + super::Com::IDispatch_Impl {
    fn FileName(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn IpAddress(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn Timeout(&mut self) -> ::windows::core::Result<u32>;
    fn CurrentFileOffset(&mut self) -> ::windows::core::Result<u64>;
    fn FileSize(&mut self) -> ::windows::core::Result<u64>;
    fn BlockSize(&mut self) -> ::windows::core::Result<u32>;
    fn WindowSize(&mut self) -> ::windows::core::Result<u32>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IWdsTransportTftpClient_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWdsTransportTftpClient_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWdsTransportTftpClient_Vtbl {
        unsafe extern "system" fn FileName<Impl: IWdsTransportTftpClient_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbszfilename: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FileName() {
                ::core::result::Result::Ok(ok__) => {
                    *pbszfilename = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IpAddress<Impl: IWdsTransportTftpClient_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbszipaddress: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IpAddress() {
                ::core::result::Result::Ok(ok__) => {
                    *pbszipaddress = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Timeout<Impl: IWdsTransportTftpClient_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pultimeout: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Timeout() {
                ::core::result::Result::Ok(ok__) => {
                    *pultimeout = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CurrentFileOffset<Impl: IWdsTransportTftpClient_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pul64currentoffset: *mut u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CurrentFileOffset() {
                ::core::result::Result::Ok(ok__) => {
                    *pul64currentoffset = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FileSize<Impl: IWdsTransportTftpClient_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pul64filesize: *mut u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FileSize() {
                ::core::result::Result::Ok(ok__) => {
                    *pul64filesize = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BlockSize<Impl: IWdsTransportTftpClient_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pulblocksize: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).BlockSize() {
                ::core::result::Result::Ok(ok__) => {
                    *pulblocksize = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn WindowSize<Impl: IWdsTransportTftpClient_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pulwindowsize: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).WindowSize() {
                ::core::result::Result::Ok(ok__) => {
                    *pulwindowsize = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: super::Com::IDispatch_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            FileName: FileName::<Impl, IMPL_OFFSET>,
            IpAddress: IpAddress::<Impl, IMPL_OFFSET>,
            Timeout: Timeout::<Impl, IMPL_OFFSET>,
            CurrentFileOffset: CurrentFileOffset::<Impl, IMPL_OFFSET>,
            FileSize: FileSize::<Impl, IMPL_OFFSET>,
            BlockSize: BlockSize::<Impl, IMPL_OFFSET>,
            WindowSize: WindowSize::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWdsTransportTftpClient as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IWdsTransportTftpManager_Impl: Sized + super::Com::IDispatch_Impl {
    fn RetrieveTftpClients(&mut self) -> ::windows::core::Result<IWdsTransportCollection>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IWdsTransportTftpManager_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWdsTransportTftpManager_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWdsTransportTftpManager_Vtbl {
        unsafe extern "system" fn RetrieveTftpClients<Impl: IWdsTransportTftpManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppwdstransporttftpclients: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RetrieveTftpClients() {
                ::core::result::Result::Ok(ok__) => {
                    *ppwdstransporttftpclients = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: super::Com::IDispatch_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            RetrieveTftpClients: RetrieveTftpClients::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWdsTransportTftpManager as ::windows::core::Interface>::IID
    }
}
