#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IWdsTransportCacheable_Impl: Sized + super::Com::IDispatch_Impl {
    fn Dirty(&self) -> ::windows::core::Result<i16>;
    fn Discard(&self) -> ::windows::core::Result<()>;
    fn Refresh(&self) -> ::windows::core::Result<()>;
    fn Commit(&self) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for IWdsTransportCacheable {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IWdsTransportCacheable_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWdsTransportCacheable_Impl, const OFFSET: isize>() -> IWdsTransportCacheable_Vtbl {
        unsafe extern "system" fn Dirty<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWdsTransportCacheable_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbdirty: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Dirty() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbdirty, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Discard<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWdsTransportCacheable_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Discard().into()
        }
        unsafe extern "system" fn Refresh<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWdsTransportCacheable_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Refresh().into()
        }
        unsafe extern "system" fn Commit<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWdsTransportCacheable_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Commit().into()
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
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
impl ::windows::core::RuntimeName for IWdsTransportClient {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IWdsTransportClient_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWdsTransportClient_Impl, const OFFSET: isize>() -> IWdsTransportClient_Vtbl {
        unsafe extern "system" fn Session<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWdsTransportClient_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppwdstransportsession: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Session() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppwdstransportsession, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Id<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWdsTransportClient_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pulid: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Id() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pulid, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Name<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWdsTransportClient_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbszname: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Name() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbszname, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MacAddress<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWdsTransportClient_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbszmacaddress: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.MacAddress() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbszmacaddress, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IpAddress<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWdsTransportClient_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbszipaddress: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.IpAddress() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbszipaddress, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PercentCompletion<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWdsTransportClient_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pulpercentcompletion: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.PercentCompletion() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pulpercentcompletion, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn JoinDuration<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWdsTransportClient_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, puljoinduration: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.JoinDuration() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(puljoinduration, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CpuUtilization<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWdsTransportClient_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pulcpuutilization: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CpuUtilization() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pulcpuutilization, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MemoryUtilization<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWdsTransportClient_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pulmemoryutilization: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.MemoryUtilization() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pulmemoryutilization, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn NetworkUtilization<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWdsTransportClient_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pulnetworkutilization: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.NetworkUtilization() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pulnetworkutilization, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UserIdentity<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWdsTransportClient_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbszuseridentity: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.UserIdentity() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbszuseridentity, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Disconnect<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWdsTransportClient_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, disconnectiontype: WDSTRANSPORT_DISCONNECT_TYPE) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Disconnect(::core::mem::transmute_copy(&disconnectiontype)).into()
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
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
    fn get_Item(&self, ulindex: u32) -> ::windows::core::Result<super::Com::IDispatch>;
    fn _NewEnum(&self) -> ::windows::core::Result<::windows::core::IUnknown>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for IWdsTransportCollection {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IWdsTransportCollection_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWdsTransportCollection_Impl, const OFFSET: isize>() -> IWdsTransportCollection_Vtbl {
        unsafe extern "system" fn Count<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWdsTransportCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pulcount: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Count() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pulcount, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn get_Item<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWdsTransportCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ulindex: u32, ppval: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.get_Item(::core::mem::transmute_copy(&ulindex)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppval, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn _NewEnum<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWdsTransportCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppval: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this._NewEnum() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppval, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            Count: Count::<Identity, Impl, OFFSET>,
            get_Item: get_Item::<Identity, Impl, OFFSET>,
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
    fn get_WdsTransportServicesRunning(&self, brealtimestatus: i16) -> ::windows::core::Result<i16>;
    fn EnableWdsTransportServices(&self) -> ::windows::core::Result<()>;
    fn DisableWdsTransportServices(&self) -> ::windows::core::Result<()>;
    fn StartWdsTransportServices(&self) -> ::windows::core::Result<()>;
    fn StopWdsTransportServices(&self) -> ::windows::core::Result<()>;
    fn RestartWdsTransportServices(&self) -> ::windows::core::Result<()>;
    fn NotifyWdsTransportServices(&self, servicenotification: WDSTRANSPORT_SERVICE_NOTIFICATION) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for IWdsTransportConfigurationManager {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IWdsTransportConfigurationManager_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWdsTransportConfigurationManager_Impl, const OFFSET: isize>() -> IWdsTransportConfigurationManager_Vtbl {
        unsafe extern "system" fn ServicePolicy<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWdsTransportConfigurationManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppwdstransportservicepolicy: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.ServicePolicy() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppwdstransportservicepolicy, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DiagnosticsPolicy<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWdsTransportConfigurationManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppwdstransportdiagnosticspolicy: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.DiagnosticsPolicy() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppwdstransportdiagnosticspolicy, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn get_WdsTransportServicesRunning<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWdsTransportConfigurationManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, brealtimestatus: i16, pbservicesrunning: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.get_WdsTransportServicesRunning(::core::mem::transmute_copy(&brealtimestatus)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbservicesrunning, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnableWdsTransportServices<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWdsTransportConfigurationManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.EnableWdsTransportServices().into()
        }
        unsafe extern "system" fn DisableWdsTransportServices<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWdsTransportConfigurationManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.DisableWdsTransportServices().into()
        }
        unsafe extern "system" fn StartWdsTransportServices<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWdsTransportConfigurationManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.StartWdsTransportServices().into()
        }
        unsafe extern "system" fn StopWdsTransportServices<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWdsTransportConfigurationManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.StopWdsTransportServices().into()
        }
        unsafe extern "system" fn RestartWdsTransportServices<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWdsTransportConfigurationManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.RestartWdsTransportServices().into()
        }
        unsafe extern "system" fn NotifyWdsTransportServices<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWdsTransportConfigurationManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, servicenotification: WDSTRANSPORT_SERVICE_NOTIFICATION) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.NotifyWdsTransportServices(::core::mem::transmute_copy(&servicenotification)).into()
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            ServicePolicy: ServicePolicy::<Identity, Impl, OFFSET>,
            DiagnosticsPolicy: DiagnosticsPolicy::<Identity, Impl, OFFSET>,
            get_WdsTransportServicesRunning: get_WdsTransportServicesRunning::<Identity, Impl, OFFSET>,
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
impl ::windows::core::RuntimeName for IWdsTransportConfigurationManager2 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IWdsTransportConfigurationManager2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWdsTransportConfigurationManager2_Impl, const OFFSET: isize>() -> IWdsTransportConfigurationManager2_Vtbl {
        unsafe extern "system" fn MulticastSessionPolicy<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWdsTransportConfigurationManager2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppwdstransportmulticastsessionpolicy: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.MulticastSessionPolicy() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppwdstransportmulticastsessionpolicy, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: IWdsTransportConfigurationManager_Vtbl::new::<Identity, Impl, OFFSET>(),
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
impl ::windows::core::RuntimeName for IWdsTransportContent {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IWdsTransportContent_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWdsTransportContent_Impl, const OFFSET: isize>() -> IWdsTransportContent_Vtbl {
        unsafe extern "system" fn Namespace<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWdsTransportContent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppwdstransportnamespace: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Namespace() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppwdstransportnamespace, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Id<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWdsTransportContent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pulid: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Id() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pulid, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Name<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWdsTransportContent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbszname: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Name() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbszname, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RetrieveSessions<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWdsTransportContent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppwdstransportsessions: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.RetrieveSessions() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppwdstransportsessions, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Terminate<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWdsTransportContent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Terminate().into()
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
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
impl ::windows::core::RuntimeName for IWdsTransportContentProvider {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IWdsTransportContentProvider_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWdsTransportContentProvider_Impl, const OFFSET: isize>() -> IWdsTransportContentProvider_Vtbl {
        unsafe extern "system" fn Name<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWdsTransportContentProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbszname: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Name() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbszname, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Description<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWdsTransportContentProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbszdescription: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Description() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbszdescription, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FilePath<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWdsTransportContentProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbszfilepath: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.FilePath() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbszfilepath, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn InitializationRoutine<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWdsTransportContentProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbszinitializationroutine: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.InitializationRoutine() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbszinitializationroutine, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
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
impl ::windows::core::RuntimeName for IWdsTransportDiagnosticsPolicy {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IWdsTransportDiagnosticsPolicy_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWdsTransportDiagnosticsPolicy_Impl, const OFFSET: isize>() -> IWdsTransportDiagnosticsPolicy_Vtbl {
        unsafe extern "system" fn Enabled<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWdsTransportDiagnosticsPolicy_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbenabled: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Enabled() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbenabled, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetEnabled<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWdsTransportDiagnosticsPolicy_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, benabled: i16) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetEnabled(::core::mem::transmute_copy(&benabled)).into()
        }
        unsafe extern "system" fn Components<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWdsTransportDiagnosticsPolicy_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pulcomponents: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Components() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pulcomponents, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetComponents<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWdsTransportDiagnosticsPolicy_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ulcomponents: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetComponents(::core::mem::transmute_copy(&ulcomponents)).into()
        }
        Self {
            base__: IWdsTransportCacheable_Vtbl::new::<Identity, Impl, OFFSET>(),
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
impl ::windows::core::RuntimeName for IWdsTransportManager {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IWdsTransportManager_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWdsTransportManager_Impl, const OFFSET: isize>() -> IWdsTransportManager_Vtbl {
        unsafe extern "system" fn GetWdsTransportServer<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWdsTransportManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bszservername: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ppwdstransportserver: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetWdsTransportServer(::core::mem::transmute(&bszservername)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppwdstransportserver, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base__: super::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(), GetWdsTransportServer: GetWdsTransportServer::<Identity, Impl, OFFSET> }
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
impl ::windows::core::RuntimeName for IWdsTransportMulticastSessionPolicy {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IWdsTransportMulticastSessionPolicy_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWdsTransportMulticastSessionPolicy_Impl, const OFFSET: isize>() -> IWdsTransportMulticastSessionPolicy_Vtbl {
        unsafe extern "system" fn SlowClientHandling<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWdsTransportMulticastSessionPolicy_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pslowclienthandling: *mut WDSTRANSPORT_SLOW_CLIENT_HANDLING_TYPE) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.SlowClientHandling() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pslowclienthandling, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSlowClientHandling<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWdsTransportMulticastSessionPolicy_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, slowclienthandling: WDSTRANSPORT_SLOW_CLIENT_HANDLING_TYPE) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetSlowClientHandling(::core::mem::transmute_copy(&slowclienthandling)).into()
        }
        unsafe extern "system" fn AutoDisconnectThreshold<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWdsTransportMulticastSessionPolicy_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pulthreshold: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.AutoDisconnectThreshold() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pulthreshold, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAutoDisconnectThreshold<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWdsTransportMulticastSessionPolicy_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ulthreshold: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetAutoDisconnectThreshold(::core::mem::transmute_copy(&ulthreshold)).into()
        }
        unsafe extern "system" fn MultistreamStreamCount<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWdsTransportMulticastSessionPolicy_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pulstreamcount: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.MultistreamStreamCount() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pulstreamcount, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMultistreamStreamCount<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWdsTransportMulticastSessionPolicy_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ulstreamcount: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetMultistreamStreamCount(::core::mem::transmute_copy(&ulstreamcount)).into()
        }
        unsafe extern "system" fn SlowClientFallback<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWdsTransportMulticastSessionPolicy_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbclientfallback: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.SlowClientFallback() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbclientfallback, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSlowClientFallback<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWdsTransportMulticastSessionPolicy_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bclientfallback: i16) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetSlowClientFallback(::core::mem::transmute_copy(&bclientfallback)).into()
        }
        Self {
            base__: IWdsTransportCacheable_Vtbl::new::<Identity, Impl, OFFSET>(),
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
impl ::windows::core::RuntimeName for IWdsTransportNamespace {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IWdsTransportNamespace_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWdsTransportNamespace_Impl, const OFFSET: isize>() -> IWdsTransportNamespace_Vtbl {
        unsafe extern "system" fn Type<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWdsTransportNamespace_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ptype: *mut WDSTRANSPORT_NAMESPACE_TYPE) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Type() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ptype, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Id<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWdsTransportNamespace_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pulid: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Id() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pulid, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Name<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWdsTransportNamespace_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbszname: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Name() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbszname, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetName<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWdsTransportNamespace_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bszname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetName(::core::mem::transmute(&bszname)).into()
        }
        unsafe extern "system" fn FriendlyName<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWdsTransportNamespace_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbszfriendlyname: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.FriendlyName() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbszfriendlyname, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetFriendlyName<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWdsTransportNamespace_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bszfriendlyname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetFriendlyName(::core::mem::transmute(&bszfriendlyname)).into()
        }
        unsafe extern "system" fn Description<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWdsTransportNamespace_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbszdescription: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Description() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbszdescription, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDescription<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWdsTransportNamespace_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bszdescription: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetDescription(::core::mem::transmute(&bszdescription)).into()
        }
        unsafe extern "system" fn ContentProvider<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWdsTransportNamespace_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbszcontentprovider: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.ContentProvider() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbszcontentprovider, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetContentProvider<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWdsTransportNamespace_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bszcontentprovider: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetContentProvider(::core::mem::transmute(&bszcontentprovider)).into()
        }
        unsafe extern "system" fn Configuration<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWdsTransportNamespace_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbszconfiguration: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Configuration() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbszconfiguration, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetConfiguration<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWdsTransportNamespace_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bszconfiguration: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetConfiguration(::core::mem::transmute(&bszconfiguration)).into()
        }
        unsafe extern "system" fn Registered<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWdsTransportNamespace_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbregistered: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Registered() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbregistered, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Tombstoned<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWdsTransportNamespace_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbtombstoned: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Tombstoned() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbtombstoned, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TombstoneTime<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWdsTransportNamespace_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ptombstonetime: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.TombstoneTime() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ptombstonetime, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TransmissionStarted<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWdsTransportNamespace_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbtransmissionstarted: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.TransmissionStarted() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbtransmissionstarted, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Register<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWdsTransportNamespace_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Register().into()
        }
        unsafe extern "system" fn Deregister<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWdsTransportNamespace_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bterminatesessions: i16) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Deregister(::core::mem::transmute_copy(&bterminatesessions)).into()
        }
        unsafe extern "system" fn Clone<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWdsTransportNamespace_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppwdstransportnamespaceclone: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Clone() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppwdstransportnamespaceclone, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Refresh<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWdsTransportNamespace_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Refresh().into()
        }
        unsafe extern "system" fn RetrieveContents<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWdsTransportNamespace_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppwdstransportcontents: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.RetrieveContents() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppwdstransportcontents, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
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
impl ::windows::core::RuntimeName for IWdsTransportNamespaceAutoCast {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IWdsTransportNamespaceAutoCast_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWdsTransportNamespaceAutoCast_Impl, const OFFSET: isize>() -> IWdsTransportNamespaceAutoCast_Vtbl {
        Self { base__: IWdsTransportNamespace_Vtbl::new::<Identity, Impl, OFFSET>() }
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
impl ::windows::core::RuntimeName for IWdsTransportNamespaceManager {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IWdsTransportNamespaceManager_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWdsTransportNamespaceManager_Impl, const OFFSET: isize>() -> IWdsTransportNamespaceManager_Vtbl {
        unsafe extern "system" fn CreateNamespace<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWdsTransportNamespaceManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, namespacetype: WDSTRANSPORT_NAMESPACE_TYPE, bsznamespacename: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bszcontentprovider: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bszconfiguration: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ppwdstransportnamespace: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CreateNamespace(::core::mem::transmute_copy(&namespacetype), ::core::mem::transmute(&bsznamespacename), ::core::mem::transmute(&bszcontentprovider), ::core::mem::transmute(&bszconfiguration)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppwdstransportnamespace, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RetrieveNamespace<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWdsTransportNamespaceManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bsznamespacename: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ppwdstransportnamespace: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.RetrieveNamespace(::core::mem::transmute(&bsznamespacename)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppwdstransportnamespace, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RetrieveNamespaces<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWdsTransportNamespaceManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bszcontentprovider: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bsznamespacename: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bincludetombstones: i16, ppwdstransportnamespaces: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.RetrieveNamespaces(::core::mem::transmute(&bszcontentprovider), ::core::mem::transmute(&bsznamespacename), ::core::mem::transmute_copy(&bincludetombstones)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppwdstransportnamespaces, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
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
impl ::windows::core::RuntimeName for IWdsTransportNamespaceScheduledCast {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IWdsTransportNamespaceScheduledCast_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWdsTransportNamespaceScheduledCast_Impl, const OFFSET: isize>() -> IWdsTransportNamespaceScheduledCast_Vtbl {
        unsafe extern "system" fn StartTransmission<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWdsTransportNamespaceScheduledCast_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.StartTransmission().into()
        }
        Self { base__: IWdsTransportNamespace_Vtbl::new::<Identity, Impl, OFFSET>(), StartTransmission: StartTransmission::<Identity, Impl, OFFSET> }
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
impl ::windows::core::RuntimeName for IWdsTransportNamespaceScheduledCastAutoStart {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IWdsTransportNamespaceScheduledCastAutoStart_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWdsTransportNamespaceScheduledCastAutoStart_Impl, const OFFSET: isize>() -> IWdsTransportNamespaceScheduledCastAutoStart_Vtbl {
        unsafe extern "system" fn MinimumClients<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWdsTransportNamespaceScheduledCastAutoStart_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pulminimumclients: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.MinimumClients() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pulminimumclients, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMinimumClients<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWdsTransportNamespaceScheduledCastAutoStart_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ulminimumclients: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetMinimumClients(::core::mem::transmute_copy(&ulminimumclients)).into()
        }
        unsafe extern "system" fn StartTime<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWdsTransportNamespaceScheduledCastAutoStart_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pstarttime: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.StartTime() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pstarttime, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetStartTime<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWdsTransportNamespaceScheduledCastAutoStart_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, starttime: f64) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetStartTime(::core::mem::transmute_copy(&starttime)).into()
        }
        Self {
            base__: IWdsTransportNamespaceScheduledCast_Vtbl::new::<Identity, Impl, OFFSET>(),
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
impl ::windows::core::RuntimeName for IWdsTransportNamespaceScheduledCastManualStart {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IWdsTransportNamespaceScheduledCastManualStart_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWdsTransportNamespaceScheduledCastManualStart_Impl, const OFFSET: isize>() -> IWdsTransportNamespaceScheduledCastManualStart_Vtbl {
        Self { base__: IWdsTransportNamespaceScheduledCast_Vtbl::new::<Identity, Impl, OFFSET>() }
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
impl ::windows::core::RuntimeName for IWdsTransportServer {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IWdsTransportServer_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWdsTransportServer_Impl, const OFFSET: isize>() -> IWdsTransportServer_Vtbl {
        unsafe extern "system" fn Name<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWdsTransportServer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbszname: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Name() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbszname, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetupManager<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWdsTransportServer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppwdstransportsetupmanager: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.SetupManager() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppwdstransportsetupmanager, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ConfigurationManager<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWdsTransportServer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppwdstransportconfigurationmanager: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.ConfigurationManager() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppwdstransportconfigurationmanager, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn NamespaceManager<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWdsTransportServer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppwdstransportnamespacemanager: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.NamespaceManager() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppwdstransportnamespacemanager, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DisconnectClient<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWdsTransportServer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ulclientid: u32, disconnectiontype: WDSTRANSPORT_DISCONNECT_TYPE) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.DisconnectClient(::core::mem::transmute_copy(&ulclientid), ::core::mem::transmute_copy(&disconnectiontype)).into()
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
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
impl ::windows::core::RuntimeName for IWdsTransportServer2 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IWdsTransportServer2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWdsTransportServer2_Impl, const OFFSET: isize>() -> IWdsTransportServer2_Vtbl {
        unsafe extern "system" fn TftpManager<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWdsTransportServer2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppwdstransporttftpmanager: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.TftpManager() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppwdstransporttftpmanager, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base__: IWdsTransportServer_Vtbl::new::<Identity, Impl, OFFSET>(), TftpManager: TftpManager::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWdsTransportServer2 as ::windows::core::Interface>::IID || iid == &<super::Com::IDispatch as ::windows::core::Interface>::IID || iid == &<IWdsTransportServer as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IWdsTransportServicePolicy_Impl: Sized + super::Com::IDispatch_Impl + IWdsTransportCacheable_Impl {
    fn get_IpAddressSource(&self, addresstype: WDSTRANSPORT_IP_ADDRESS_TYPE) -> ::windows::core::Result<WDSTRANSPORT_IP_ADDRESS_SOURCE_TYPE>;
    fn put_IpAddressSource(&self, addresstype: WDSTRANSPORT_IP_ADDRESS_TYPE, sourcetype: WDSTRANSPORT_IP_ADDRESS_SOURCE_TYPE) -> ::windows::core::Result<()>;
    fn get_StartIpAddress(&self, addresstype: WDSTRANSPORT_IP_ADDRESS_TYPE) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn put_StartIpAddress(&self, addresstype: WDSTRANSPORT_IP_ADDRESS_TYPE, bszstartipaddress: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn get_EndIpAddress(&self, addresstype: WDSTRANSPORT_IP_ADDRESS_TYPE) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn put_EndIpAddress(&self, addresstype: WDSTRANSPORT_IP_ADDRESS_TYPE, bszendipaddress: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn StartPort(&self) -> ::windows::core::Result<u32>;
    fn SetStartPort(&self, ulstartport: u32) -> ::windows::core::Result<()>;
    fn EndPort(&self) -> ::windows::core::Result<u32>;
    fn SetEndPort(&self, ulendport: u32) -> ::windows::core::Result<()>;
    fn NetworkProfile(&self) -> ::windows::core::Result<WDSTRANSPORT_NETWORK_PROFILE_TYPE>;
    fn SetNetworkProfile(&self, profiletype: WDSTRANSPORT_NETWORK_PROFILE_TYPE) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for IWdsTransportServicePolicy {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IWdsTransportServicePolicy_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWdsTransportServicePolicy_Impl, const OFFSET: isize>() -> IWdsTransportServicePolicy_Vtbl {
        unsafe extern "system" fn get_IpAddressSource<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWdsTransportServicePolicy_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, addresstype: WDSTRANSPORT_IP_ADDRESS_TYPE, psourcetype: *mut WDSTRANSPORT_IP_ADDRESS_SOURCE_TYPE) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.get_IpAddressSource(::core::mem::transmute_copy(&addresstype)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(psourcetype, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn put_IpAddressSource<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWdsTransportServicePolicy_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, addresstype: WDSTRANSPORT_IP_ADDRESS_TYPE, sourcetype: WDSTRANSPORT_IP_ADDRESS_SOURCE_TYPE) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.put_IpAddressSource(::core::mem::transmute_copy(&addresstype), ::core::mem::transmute_copy(&sourcetype)).into()
        }
        unsafe extern "system" fn get_StartIpAddress<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWdsTransportServicePolicy_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, addresstype: WDSTRANSPORT_IP_ADDRESS_TYPE, pbszstartipaddress: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.get_StartIpAddress(::core::mem::transmute_copy(&addresstype)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbszstartipaddress, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn put_StartIpAddress<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWdsTransportServicePolicy_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, addresstype: WDSTRANSPORT_IP_ADDRESS_TYPE, bszstartipaddress: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.put_StartIpAddress(::core::mem::transmute_copy(&addresstype), ::core::mem::transmute(&bszstartipaddress)).into()
        }
        unsafe extern "system" fn get_EndIpAddress<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWdsTransportServicePolicy_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, addresstype: WDSTRANSPORT_IP_ADDRESS_TYPE, pbszendipaddress: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.get_EndIpAddress(::core::mem::transmute_copy(&addresstype)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbszendipaddress, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn put_EndIpAddress<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWdsTransportServicePolicy_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, addresstype: WDSTRANSPORT_IP_ADDRESS_TYPE, bszendipaddress: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.put_EndIpAddress(::core::mem::transmute_copy(&addresstype), ::core::mem::transmute(&bszendipaddress)).into()
        }
        unsafe extern "system" fn StartPort<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWdsTransportServicePolicy_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pulstartport: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.StartPort() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pulstartport, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetStartPort<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWdsTransportServicePolicy_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ulstartport: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetStartPort(::core::mem::transmute_copy(&ulstartport)).into()
        }
        unsafe extern "system" fn EndPort<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWdsTransportServicePolicy_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pulendport: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.EndPort() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pulendport, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetEndPort<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWdsTransportServicePolicy_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ulendport: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetEndPort(::core::mem::transmute_copy(&ulendport)).into()
        }
        unsafe extern "system" fn NetworkProfile<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWdsTransportServicePolicy_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pprofiletype: *mut WDSTRANSPORT_NETWORK_PROFILE_TYPE) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.NetworkProfile() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pprofiletype, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetNetworkProfile<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWdsTransportServicePolicy_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, profiletype: WDSTRANSPORT_NETWORK_PROFILE_TYPE) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetNetworkProfile(::core::mem::transmute_copy(&profiletype)).into()
        }
        Self {
            base__: IWdsTransportCacheable_Vtbl::new::<Identity, Impl, OFFSET>(),
            get_IpAddressSource: get_IpAddressSource::<Identity, Impl, OFFSET>,
            put_IpAddressSource: put_IpAddressSource::<Identity, Impl, OFFSET>,
            get_StartIpAddress: get_StartIpAddress::<Identity, Impl, OFFSET>,
            put_StartIpAddress: put_StartIpAddress::<Identity, Impl, OFFSET>,
            get_EndIpAddress: get_EndIpAddress::<Identity, Impl, OFFSET>,
            put_EndIpAddress: put_EndIpAddress::<Identity, Impl, OFFSET>,
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
impl ::windows::core::RuntimeName for IWdsTransportServicePolicy2 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IWdsTransportServicePolicy2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWdsTransportServicePolicy2_Impl, const OFFSET: isize>() -> IWdsTransportServicePolicy2_Vtbl {
        unsafe extern "system" fn UdpPortPolicy<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWdsTransportServicePolicy2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pudpportpolicy: *mut WDSTRANSPORT_UDP_PORT_POLICY) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.UdpPortPolicy() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pudpportpolicy, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetUdpPortPolicy<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWdsTransportServicePolicy2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, udpportpolicy: WDSTRANSPORT_UDP_PORT_POLICY) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetUdpPortPolicy(::core::mem::transmute_copy(&udpportpolicy)).into()
        }
        unsafe extern "system" fn TftpMaximumBlockSize<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWdsTransportServicePolicy2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pultftpmaximumblocksize: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.TftpMaximumBlockSize() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pultftpmaximumblocksize, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetTftpMaximumBlockSize<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWdsTransportServicePolicy2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ultftpmaximumblocksize: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetTftpMaximumBlockSize(::core::mem::transmute_copy(&ultftpmaximumblocksize)).into()
        }
        unsafe extern "system" fn EnableTftpVariableWindowExtension<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWdsTransportServicePolicy2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbenabletftpvariablewindowextension: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.EnableTftpVariableWindowExtension() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbenabletftpvariablewindowextension, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetEnableTftpVariableWindowExtension<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWdsTransportServicePolicy2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, benabletftpvariablewindowextension: i16) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetEnableTftpVariableWindowExtension(::core::mem::transmute_copy(&benabletftpvariablewindowextension)).into()
        }
        Self {
            base__: IWdsTransportServicePolicy_Vtbl::new::<Identity, Impl, OFFSET>(),
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
impl ::windows::core::RuntimeName for IWdsTransportSession {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IWdsTransportSession_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWdsTransportSession_Impl, const OFFSET: isize>() -> IWdsTransportSession_Vtbl {
        unsafe extern "system" fn Content<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWdsTransportSession_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppwdstransportcontent: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Content() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppwdstransportcontent, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Id<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWdsTransportSession_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pulid: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Id() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pulid, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn NetworkInterfaceName<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWdsTransportSession_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbsznetworkinterfacename: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.NetworkInterfaceName() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbsznetworkinterfacename, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn NetworkInterfaceAddress<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWdsTransportSession_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbsznetworkinterfaceaddress: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.NetworkInterfaceAddress() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbsznetworkinterfaceaddress, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TransferRate<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWdsTransportSession_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pultransferrate: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.TransferRate() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pultransferrate, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MasterClientId<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWdsTransportSession_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pulmasterclientid: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.MasterClientId() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pulmasterclientid, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RetrieveClients<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWdsTransportSession_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppwdstransportclients: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.RetrieveClients() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppwdstransportclients, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Terminate<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWdsTransportSession_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Terminate().into()
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
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
impl ::windows::core::RuntimeName for IWdsTransportSetupManager {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IWdsTransportSetupManager_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWdsTransportSetupManager_Impl, const OFFSET: isize>() -> IWdsTransportSetupManager_Vtbl {
        unsafe extern "system" fn Version<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWdsTransportSetupManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pullversion: *mut u64) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Version() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pullversion, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn InstalledFeatures<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWdsTransportSetupManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pulinstalledfeatures: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.InstalledFeatures() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pulinstalledfeatures, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Protocols<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWdsTransportSetupManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pulprotocols: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Protocols() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pulprotocols, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RegisterContentProvider<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWdsTransportSetupManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bszname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bszdescription: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bszfilepath: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bszinitializationroutine: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.RegisterContentProvider(::core::mem::transmute(&bszname), ::core::mem::transmute(&bszdescription), ::core::mem::transmute(&bszfilepath), ::core::mem::transmute(&bszinitializationroutine)).into()
        }
        unsafe extern "system" fn DeregisterContentProvider<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWdsTransportSetupManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bszname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.DeregisterContentProvider(::core::mem::transmute(&bszname)).into()
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
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
impl ::windows::core::RuntimeName for IWdsTransportSetupManager2 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IWdsTransportSetupManager2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWdsTransportSetupManager2_Impl, const OFFSET: isize>() -> IWdsTransportSetupManager2_Vtbl {
        unsafe extern "system" fn TftpCapabilities<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWdsTransportSetupManager2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pultftpcapabilities: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.TftpCapabilities() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pultftpcapabilities, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ContentProviders<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWdsTransportSetupManager2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppprovidercollection: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.ContentProviders() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppprovidercollection, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: IWdsTransportSetupManager_Vtbl::new::<Identity, Impl, OFFSET>(),
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
impl ::windows::core::RuntimeName for IWdsTransportTftpClient {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IWdsTransportTftpClient_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWdsTransportTftpClient_Impl, const OFFSET: isize>() -> IWdsTransportTftpClient_Vtbl {
        unsafe extern "system" fn FileName<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWdsTransportTftpClient_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbszfilename: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.FileName() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbszfilename, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IpAddress<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWdsTransportTftpClient_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbszipaddress: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.IpAddress() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbszipaddress, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Timeout<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWdsTransportTftpClient_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pultimeout: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Timeout() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pultimeout, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CurrentFileOffset<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWdsTransportTftpClient_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pul64currentoffset: *mut u64) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CurrentFileOffset() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pul64currentoffset, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FileSize<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWdsTransportTftpClient_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pul64filesize: *mut u64) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.FileSize() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pul64filesize, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BlockSize<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWdsTransportTftpClient_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pulblocksize: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.BlockSize() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pulblocksize, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn WindowSize<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWdsTransportTftpClient_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pulwindowsize: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.WindowSize() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pulwindowsize, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
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
impl ::windows::core::RuntimeName for IWdsTransportTftpManager {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IWdsTransportTftpManager_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWdsTransportTftpManager_Impl, const OFFSET: isize>() -> IWdsTransportTftpManager_Vtbl {
        unsafe extern "system" fn RetrieveTftpClients<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWdsTransportTftpManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppwdstransporttftpclients: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.RetrieveTftpClients() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppwdstransporttftpclients, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base__: super::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(), RetrieveTftpClients: RetrieveTftpClients::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWdsTransportTftpManager as ::windows::core::Interface>::IID || iid == &<super::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
