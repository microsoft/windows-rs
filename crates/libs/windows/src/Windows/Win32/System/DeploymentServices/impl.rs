#[cfg(feature = "Win32_System_Com")]
pub trait IWdsTransportCacheableImpl: Sized + IDispatchImpl {
    fn Dirty();
    fn Discard();
    fn Refresh();
    fn Commit();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for IWdsTransportCacheable {
    const NAME: &'static str = "Windows.Win32.System.DeploymentServices.IWdsTransportCacheable";
}
#[cfg(feature = "Win32_System_Com")]
impl IWdsTransportCacheableVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWdsTransportCacheableImpl, const OFFSET: isize>() -> IWdsTransportCacheableVtbl {
        unsafe extern "system" fn Dirty<Impl: IWdsTransportCacheableImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbdirty: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Dirty(::core::mem::transmute_copy(&pbdirty)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Discard<Impl: IWdsTransportCacheableImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Discard() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Refresh<Impl: IWdsTransportCacheableImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Refresh() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Commit<Impl: IWdsTransportCacheableImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Commit() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IWdsTransportCacheable>, ::windows::core::GetTrustLevel, Dirty::<Impl, OFFSET>, Discard::<Impl, OFFSET>, Refresh::<Impl, OFFSET>, Commit::<Impl, OFFSET>)
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IWdsTransportClientImpl: Sized + IDispatchImpl {
    fn Session();
    fn Id();
    fn Name();
    fn MacAddress();
    fn IpAddress();
    fn PercentCompletion();
    fn JoinDuration();
    fn CpuUtilization();
    fn MemoryUtilization();
    fn NetworkUtilization();
    fn UserIdentity();
    fn Disconnect();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for IWdsTransportClient {
    const NAME: &'static str = "Windows.Win32.System.DeploymentServices.IWdsTransportClient";
}
#[cfg(feature = "Win32_System_Com")]
impl IWdsTransportClientVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWdsTransportClientImpl, const OFFSET: isize>() -> IWdsTransportClientVtbl {
        unsafe extern "system" fn Session<Impl: IWdsTransportClientImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppwdstransportsession: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Session(::core::mem::transmute_copy(&ppwdstransportsession)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Id<Impl: IWdsTransportClientImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pulid: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Id(::core::mem::transmute_copy(&pulid)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Name<Impl: IWdsTransportClientImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbszname: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Name(::core::mem::transmute_copy(&pbszname)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MacAddress<Impl: IWdsTransportClientImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbszmacaddress: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MacAddress(::core::mem::transmute_copy(&pbszmacaddress)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IpAddress<Impl: IWdsTransportClientImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbszipaddress: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IpAddress(::core::mem::transmute_copy(&pbszipaddress)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PercentCompletion<Impl: IWdsTransportClientImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pulpercentcompletion: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PercentCompletion(::core::mem::transmute_copy(&pulpercentcompletion)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn JoinDuration<Impl: IWdsTransportClientImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, puljoinduration: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).JoinDuration(::core::mem::transmute_copy(&puljoinduration)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CpuUtilization<Impl: IWdsTransportClientImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pulcpuutilization: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CpuUtilization(::core::mem::transmute_copy(&pulcpuutilization)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MemoryUtilization<Impl: IWdsTransportClientImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pulmemoryutilization: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MemoryUtilization(::core::mem::transmute_copy(&pulmemoryutilization)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn NetworkUtilization<Impl: IWdsTransportClientImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pulnetworkutilization: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).NetworkUtilization(::core::mem::transmute_copy(&pulnetworkutilization)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UserIdentity<Impl: IWdsTransportClientImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbszuseridentity: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).UserIdentity(::core::mem::transmute_copy(&pbszuseridentity)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Disconnect<Impl: IWdsTransportClientImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, disconnectiontype: WDSTRANSPORT_DISCONNECT_TYPE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Disconnect(disconnectiontype) {
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
            ::windows::core::GetRuntimeClassName::<IWdsTransportClient>,
            ::windows::core::GetTrustLevel,
            Session::<Impl, OFFSET>,
            Id::<Impl, OFFSET>,
            Name::<Impl, OFFSET>,
            MacAddress::<Impl, OFFSET>,
            IpAddress::<Impl, OFFSET>,
            PercentCompletion::<Impl, OFFSET>,
            JoinDuration::<Impl, OFFSET>,
            CpuUtilization::<Impl, OFFSET>,
            MemoryUtilization::<Impl, OFFSET>,
            NetworkUtilization::<Impl, OFFSET>,
            UserIdentity::<Impl, OFFSET>,
            Disconnect::<Impl, OFFSET>,
        )
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IWdsTransportCollectionImpl: Sized + IDispatchImpl {
    fn Count();
    fn Item();
    fn _NewEnum();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for IWdsTransportCollection {
    const NAME: &'static str = "Windows.Win32.System.DeploymentServices.IWdsTransportCollection";
}
#[cfg(feature = "Win32_System_Com")]
impl IWdsTransportCollectionVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWdsTransportCollectionImpl, const OFFSET: isize>() -> IWdsTransportCollectionVtbl {
        unsafe extern "system" fn Count<Impl: IWdsTransportCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pulcount: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Count(::core::mem::transmute_copy(&pulcount)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Item<Impl: IWdsTransportCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ulindex: u32, ppval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Item(ulindex, ::core::mem::transmute_copy(&ppval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn _NewEnum<Impl: IWdsTransportCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppval: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this)._NewEnum(::core::mem::transmute_copy(&ppval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IWdsTransportCollection>, ::windows::core::GetTrustLevel, Count::<Impl, OFFSET>, Item::<Impl, OFFSET>, _NewEnum::<Impl, OFFSET>)
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IWdsTransportConfigurationManagerImpl: Sized + IDispatchImpl {
    fn ServicePolicy();
    fn DiagnosticsPolicy();
    fn WdsTransportServicesRunning();
    fn EnableWdsTransportServices();
    fn DisableWdsTransportServices();
    fn StartWdsTransportServices();
    fn StopWdsTransportServices();
    fn RestartWdsTransportServices();
    fn NotifyWdsTransportServices();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for IWdsTransportConfigurationManager {
    const NAME: &'static str = "Windows.Win32.System.DeploymentServices.IWdsTransportConfigurationManager";
}
#[cfg(feature = "Win32_System_Com")]
impl IWdsTransportConfigurationManagerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWdsTransportConfigurationManagerImpl, const OFFSET: isize>() -> IWdsTransportConfigurationManagerVtbl {
        unsafe extern "system" fn ServicePolicy<Impl: IWdsTransportConfigurationManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppwdstransportservicepolicy: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ServicePolicy(::core::mem::transmute_copy(&ppwdstransportservicepolicy)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DiagnosticsPolicy<Impl: IWdsTransportConfigurationManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppwdstransportdiagnosticspolicy: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DiagnosticsPolicy(::core::mem::transmute_copy(&ppwdstransportdiagnosticspolicy)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn WdsTransportServicesRunning<Impl: IWdsTransportConfigurationManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, brealtimestatus: i16, pbservicesrunning: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).WdsTransportServicesRunning(brealtimestatus, ::core::mem::transmute_copy(&pbservicesrunning)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnableWdsTransportServices<Impl: IWdsTransportConfigurationManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EnableWdsTransportServices() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DisableWdsTransportServices<Impl: IWdsTransportConfigurationManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DisableWdsTransportServices() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn StartWdsTransportServices<Impl: IWdsTransportConfigurationManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).StartWdsTransportServices() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn StopWdsTransportServices<Impl: IWdsTransportConfigurationManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).StopWdsTransportServices() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RestartWdsTransportServices<Impl: IWdsTransportConfigurationManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RestartWdsTransportServices() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn NotifyWdsTransportServices<Impl: IWdsTransportConfigurationManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, servicenotification: WDSTRANSPORT_SERVICE_NOTIFICATION) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).NotifyWdsTransportServices(servicenotification) {
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
            ::windows::core::GetRuntimeClassName::<IWdsTransportConfigurationManager>,
            ::windows::core::GetTrustLevel,
            ServicePolicy::<Impl, OFFSET>,
            DiagnosticsPolicy::<Impl, OFFSET>,
            WdsTransportServicesRunning::<Impl, OFFSET>,
            EnableWdsTransportServices::<Impl, OFFSET>,
            DisableWdsTransportServices::<Impl, OFFSET>,
            StartWdsTransportServices::<Impl, OFFSET>,
            StopWdsTransportServices::<Impl, OFFSET>,
            RestartWdsTransportServices::<Impl, OFFSET>,
            NotifyWdsTransportServices::<Impl, OFFSET>,
        )
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IWdsTransportConfigurationManager2Impl: Sized + IWdsTransportConfigurationManagerImpl + IDispatchImpl {
    fn MulticastSessionPolicy();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for IWdsTransportConfigurationManager2 {
    const NAME: &'static str = "Windows.Win32.System.DeploymentServices.IWdsTransportConfigurationManager2";
}
#[cfg(feature = "Win32_System_Com")]
impl IWdsTransportConfigurationManager2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWdsTransportConfigurationManager2Impl, const OFFSET: isize>() -> IWdsTransportConfigurationManager2Vtbl {
        unsafe extern "system" fn MulticastSessionPolicy<Impl: IWdsTransportConfigurationManager2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppwdstransportmulticastsessionpolicy: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MulticastSessionPolicy(::core::mem::transmute_copy(&ppwdstransportmulticastsessionpolicy)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IWdsTransportConfigurationManager2>, ::windows::core::GetTrustLevel, MulticastSessionPolicy::<Impl, OFFSET>)
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IWdsTransportContentImpl: Sized + IDispatchImpl {
    fn Namespace();
    fn Id();
    fn Name();
    fn RetrieveSessions();
    fn Terminate();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for IWdsTransportContent {
    const NAME: &'static str = "Windows.Win32.System.DeploymentServices.IWdsTransportContent";
}
#[cfg(feature = "Win32_System_Com")]
impl IWdsTransportContentVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWdsTransportContentImpl, const OFFSET: isize>() -> IWdsTransportContentVtbl {
        unsafe extern "system" fn Namespace<Impl: IWdsTransportContentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppwdstransportnamespace: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Namespace(::core::mem::transmute_copy(&ppwdstransportnamespace)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Id<Impl: IWdsTransportContentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pulid: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Id(::core::mem::transmute_copy(&pulid)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Name<Impl: IWdsTransportContentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbszname: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Name(::core::mem::transmute_copy(&pbszname)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RetrieveSessions<Impl: IWdsTransportContentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppwdstransportsessions: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RetrieveSessions(::core::mem::transmute_copy(&ppwdstransportsessions)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Terminate<Impl: IWdsTransportContentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Terminate() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IWdsTransportContent>, ::windows::core::GetTrustLevel, Namespace::<Impl, OFFSET>, Id::<Impl, OFFSET>, Name::<Impl, OFFSET>, RetrieveSessions::<Impl, OFFSET>, Terminate::<Impl, OFFSET>)
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IWdsTransportContentProviderImpl: Sized + IDispatchImpl {
    fn Name();
    fn Description();
    fn FilePath();
    fn InitializationRoutine();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for IWdsTransportContentProvider {
    const NAME: &'static str = "Windows.Win32.System.DeploymentServices.IWdsTransportContentProvider";
}
#[cfg(feature = "Win32_System_Com")]
impl IWdsTransportContentProviderVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWdsTransportContentProviderImpl, const OFFSET: isize>() -> IWdsTransportContentProviderVtbl {
        unsafe extern "system" fn Name<Impl: IWdsTransportContentProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbszname: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Name(::core::mem::transmute_copy(&pbszname)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Description<Impl: IWdsTransportContentProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbszdescription: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Description(::core::mem::transmute_copy(&pbszdescription)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FilePath<Impl: IWdsTransportContentProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbszfilepath: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FilePath(::core::mem::transmute_copy(&pbszfilepath)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn InitializationRoutine<Impl: IWdsTransportContentProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbszinitializationroutine: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).InitializationRoutine(::core::mem::transmute_copy(&pbszinitializationroutine)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IWdsTransportContentProvider>, ::windows::core::GetTrustLevel, Name::<Impl, OFFSET>, Description::<Impl, OFFSET>, FilePath::<Impl, OFFSET>, InitializationRoutine::<Impl, OFFSET>)
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IWdsTransportDiagnosticsPolicyImpl: Sized + IWdsTransportCacheableImpl + IDispatchImpl {
    fn Enabled();
    fn SetEnabled();
    fn Components();
    fn SetComponents();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for IWdsTransportDiagnosticsPolicy {
    const NAME: &'static str = "Windows.Win32.System.DeploymentServices.IWdsTransportDiagnosticsPolicy";
}
#[cfg(feature = "Win32_System_Com")]
impl IWdsTransportDiagnosticsPolicyVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWdsTransportDiagnosticsPolicyImpl, const OFFSET: isize>() -> IWdsTransportDiagnosticsPolicyVtbl {
        unsafe extern "system" fn Enabled<Impl: IWdsTransportDiagnosticsPolicyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbenabled: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Enabled(::core::mem::transmute_copy(&pbenabled)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetEnabled<Impl: IWdsTransportDiagnosticsPolicyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, benabled: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetEnabled(benabled) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Components<Impl: IWdsTransportDiagnosticsPolicyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pulcomponents: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Components(::core::mem::transmute_copy(&pulcomponents)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetComponents<Impl: IWdsTransportDiagnosticsPolicyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ulcomponents: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetComponents(ulcomponents) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IWdsTransportDiagnosticsPolicy>, ::windows::core::GetTrustLevel, Enabled::<Impl, OFFSET>, SetEnabled::<Impl, OFFSET>, Components::<Impl, OFFSET>, SetComponents::<Impl, OFFSET>)
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IWdsTransportManagerImpl: Sized + IDispatchImpl {
    fn GetWdsTransportServer();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for IWdsTransportManager {
    const NAME: &'static str = "Windows.Win32.System.DeploymentServices.IWdsTransportManager";
}
#[cfg(feature = "Win32_System_Com")]
impl IWdsTransportManagerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWdsTransportManagerImpl, const OFFSET: isize>() -> IWdsTransportManagerVtbl {
        unsafe extern "system" fn GetWdsTransportServer<Impl: IWdsTransportManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bszservername: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ppwdstransportserver: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetWdsTransportServer(&*(&bszservername as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&ppwdstransportserver)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IWdsTransportManager>, ::windows::core::GetTrustLevel, GetWdsTransportServer::<Impl, OFFSET>)
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IWdsTransportMulticastSessionPolicyImpl: Sized + IWdsTransportCacheableImpl + IDispatchImpl {
    fn SlowClientHandling();
    fn SetSlowClientHandling();
    fn AutoDisconnectThreshold();
    fn SetAutoDisconnectThreshold();
    fn MultistreamStreamCount();
    fn SetMultistreamStreamCount();
    fn SlowClientFallback();
    fn SetSlowClientFallback();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for IWdsTransportMulticastSessionPolicy {
    const NAME: &'static str = "Windows.Win32.System.DeploymentServices.IWdsTransportMulticastSessionPolicy";
}
#[cfg(feature = "Win32_System_Com")]
impl IWdsTransportMulticastSessionPolicyVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWdsTransportMulticastSessionPolicyImpl, const OFFSET: isize>() -> IWdsTransportMulticastSessionPolicyVtbl {
        unsafe extern "system" fn SlowClientHandling<Impl: IWdsTransportMulticastSessionPolicyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pslowclienthandling: *mut WDSTRANSPORT_SLOW_CLIENT_HANDLING_TYPE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SlowClientHandling(::core::mem::transmute_copy(&pslowclienthandling)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSlowClientHandling<Impl: IWdsTransportMulticastSessionPolicyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, slowclienthandling: WDSTRANSPORT_SLOW_CLIENT_HANDLING_TYPE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetSlowClientHandling(slowclienthandling) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AutoDisconnectThreshold<Impl: IWdsTransportMulticastSessionPolicyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pulthreshold: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AutoDisconnectThreshold(::core::mem::transmute_copy(&pulthreshold)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAutoDisconnectThreshold<Impl: IWdsTransportMulticastSessionPolicyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ulthreshold: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetAutoDisconnectThreshold(ulthreshold) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MultistreamStreamCount<Impl: IWdsTransportMulticastSessionPolicyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pulstreamcount: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MultistreamStreamCount(::core::mem::transmute_copy(&pulstreamcount)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMultistreamStreamCount<Impl: IWdsTransportMulticastSessionPolicyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ulstreamcount: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetMultistreamStreamCount(ulstreamcount) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SlowClientFallback<Impl: IWdsTransportMulticastSessionPolicyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbclientfallback: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SlowClientFallback(::core::mem::transmute_copy(&pbclientfallback)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSlowClientFallback<Impl: IWdsTransportMulticastSessionPolicyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bclientfallback: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetSlowClientFallback(bclientfallback) {
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
            ::windows::core::GetRuntimeClassName::<IWdsTransportMulticastSessionPolicy>,
            ::windows::core::GetTrustLevel,
            SlowClientHandling::<Impl, OFFSET>,
            SetSlowClientHandling::<Impl, OFFSET>,
            AutoDisconnectThreshold::<Impl, OFFSET>,
            SetAutoDisconnectThreshold::<Impl, OFFSET>,
            MultistreamStreamCount::<Impl, OFFSET>,
            SetMultistreamStreamCount::<Impl, OFFSET>,
            SlowClientFallback::<Impl, OFFSET>,
            SetSlowClientFallback::<Impl, OFFSET>,
        )
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IWdsTransportNamespaceImpl: Sized + IDispatchImpl {
    fn Type();
    fn Id();
    fn Name();
    fn SetName();
    fn FriendlyName();
    fn SetFriendlyName();
    fn Description();
    fn SetDescription();
    fn ContentProvider();
    fn SetContentProvider();
    fn Configuration();
    fn SetConfiguration();
    fn Registered();
    fn Tombstoned();
    fn TombstoneTime();
    fn TransmissionStarted();
    fn Register();
    fn Deregister();
    fn Clone();
    fn Refresh();
    fn RetrieveContents();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for IWdsTransportNamespace {
    const NAME: &'static str = "Windows.Win32.System.DeploymentServices.IWdsTransportNamespace";
}
#[cfg(feature = "Win32_System_Com")]
impl IWdsTransportNamespaceVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWdsTransportNamespaceImpl, const OFFSET: isize>() -> IWdsTransportNamespaceVtbl {
        unsafe extern "system" fn Type<Impl: IWdsTransportNamespaceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ptype: *mut WDSTRANSPORT_NAMESPACE_TYPE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Type(::core::mem::transmute_copy(&ptype)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Id<Impl: IWdsTransportNamespaceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pulid: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Id(::core::mem::transmute_copy(&pulid)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Name<Impl: IWdsTransportNamespaceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbszname: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Name(::core::mem::transmute_copy(&pbszname)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetName<Impl: IWdsTransportNamespaceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bszname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetName(&*(&bszname as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FriendlyName<Impl: IWdsTransportNamespaceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbszfriendlyname: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FriendlyName(::core::mem::transmute_copy(&pbszfriendlyname)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetFriendlyName<Impl: IWdsTransportNamespaceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bszfriendlyname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetFriendlyName(&*(&bszfriendlyname as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Description<Impl: IWdsTransportNamespaceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbszdescription: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Description(::core::mem::transmute_copy(&pbszdescription)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDescription<Impl: IWdsTransportNamespaceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bszdescription: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetDescription(&*(&bszdescription as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ContentProvider<Impl: IWdsTransportNamespaceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbszcontentprovider: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ContentProvider(::core::mem::transmute_copy(&pbszcontentprovider)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetContentProvider<Impl: IWdsTransportNamespaceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bszcontentprovider: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetContentProvider(&*(&bszcontentprovider as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Configuration<Impl: IWdsTransportNamespaceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbszconfiguration: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Configuration(::core::mem::transmute_copy(&pbszconfiguration)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetConfiguration<Impl: IWdsTransportNamespaceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bszconfiguration: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetConfiguration(&*(&bszconfiguration as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Registered<Impl: IWdsTransportNamespaceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbregistered: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Registered(::core::mem::transmute_copy(&pbregistered)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Tombstoned<Impl: IWdsTransportNamespaceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbtombstoned: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Tombstoned(::core::mem::transmute_copy(&pbtombstoned)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TombstoneTime<Impl: IWdsTransportNamespaceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ptombstonetime: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TombstoneTime(::core::mem::transmute_copy(&ptombstonetime)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TransmissionStarted<Impl: IWdsTransportNamespaceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbtransmissionstarted: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TransmissionStarted(::core::mem::transmute_copy(&pbtransmissionstarted)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Register<Impl: IWdsTransportNamespaceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Register() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Deregister<Impl: IWdsTransportNamespaceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bterminatesessions: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Deregister(bterminatesessions) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Clone<Impl: IWdsTransportNamespaceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppwdstransportnamespaceclone: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Clone(::core::mem::transmute_copy(&ppwdstransportnamespaceclone)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Refresh<Impl: IWdsTransportNamespaceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Refresh() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RetrieveContents<Impl: IWdsTransportNamespaceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppwdstransportcontents: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RetrieveContents(::core::mem::transmute_copy(&ppwdstransportcontents)) {
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
            ::windows::core::GetRuntimeClassName::<IWdsTransportNamespace>,
            ::windows::core::GetTrustLevel,
            Type::<Impl, OFFSET>,
            Id::<Impl, OFFSET>,
            Name::<Impl, OFFSET>,
            SetName::<Impl, OFFSET>,
            FriendlyName::<Impl, OFFSET>,
            SetFriendlyName::<Impl, OFFSET>,
            Description::<Impl, OFFSET>,
            SetDescription::<Impl, OFFSET>,
            ContentProvider::<Impl, OFFSET>,
            SetContentProvider::<Impl, OFFSET>,
            Configuration::<Impl, OFFSET>,
            SetConfiguration::<Impl, OFFSET>,
            Registered::<Impl, OFFSET>,
            Tombstoned::<Impl, OFFSET>,
            TombstoneTime::<Impl, OFFSET>,
            TransmissionStarted::<Impl, OFFSET>,
            Register::<Impl, OFFSET>,
            Deregister::<Impl, OFFSET>,
            Clone::<Impl, OFFSET>,
            Refresh::<Impl, OFFSET>,
            RetrieveContents::<Impl, OFFSET>,
        )
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IWdsTransportNamespaceAutoCastImpl: Sized + IWdsTransportNamespaceImpl + IDispatchImpl {}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for IWdsTransportNamespaceAutoCast {
    const NAME: &'static str = "Windows.Win32.System.DeploymentServices.IWdsTransportNamespaceAutoCast";
}
#[cfg(feature = "Win32_System_Com")]
impl IWdsTransportNamespaceAutoCastVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWdsTransportNamespaceAutoCastImpl, const OFFSET: isize>() -> IWdsTransportNamespaceAutoCastVtbl {
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IWdsTransportNamespaceAutoCast>, ::windows::core::GetTrustLevel)
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IWdsTransportNamespaceManagerImpl: Sized + IDispatchImpl {
    fn CreateNamespace();
    fn RetrieveNamespace();
    fn RetrieveNamespaces();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for IWdsTransportNamespaceManager {
    const NAME: &'static str = "Windows.Win32.System.DeploymentServices.IWdsTransportNamespaceManager";
}
#[cfg(feature = "Win32_System_Com")]
impl IWdsTransportNamespaceManagerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWdsTransportNamespaceManagerImpl, const OFFSET: isize>() -> IWdsTransportNamespaceManagerVtbl {
        unsafe extern "system" fn CreateNamespace<Impl: IWdsTransportNamespaceManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, namespacetype: WDSTRANSPORT_NAMESPACE_TYPE, bsznamespacename: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bszcontentprovider: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bszconfiguration: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ppwdstransportnamespace: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateNamespace(
                namespacetype,
                &*(&bsznamespacename as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&bszcontentprovider as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&bszconfiguration as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType),
                ::core::mem::transmute_copy(&ppwdstransportnamespace),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RetrieveNamespace<Impl: IWdsTransportNamespaceManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bsznamespacename: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ppwdstransportnamespace: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RetrieveNamespace(&*(&bsznamespacename as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&ppwdstransportnamespace)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RetrieveNamespaces<Impl: IWdsTransportNamespaceManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bszcontentprovider: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bsznamespacename: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bincludetombstones: i16, ppwdstransportnamespaces: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RetrieveNamespaces(&*(&bszcontentprovider as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), &*(&bsznamespacename as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), bincludetombstones, ::core::mem::transmute_copy(&ppwdstransportnamespaces)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IWdsTransportNamespaceManager>, ::windows::core::GetTrustLevel, CreateNamespace::<Impl, OFFSET>, RetrieveNamespace::<Impl, OFFSET>, RetrieveNamespaces::<Impl, OFFSET>)
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IWdsTransportNamespaceScheduledCastImpl: Sized + IWdsTransportNamespaceImpl + IDispatchImpl {
    fn StartTransmission();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for IWdsTransportNamespaceScheduledCast {
    const NAME: &'static str = "Windows.Win32.System.DeploymentServices.IWdsTransportNamespaceScheduledCast";
}
#[cfg(feature = "Win32_System_Com")]
impl IWdsTransportNamespaceScheduledCastVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWdsTransportNamespaceScheduledCastImpl, const OFFSET: isize>() -> IWdsTransportNamespaceScheduledCastVtbl {
        unsafe extern "system" fn StartTransmission<Impl: IWdsTransportNamespaceScheduledCastImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).StartTransmission() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IWdsTransportNamespaceScheduledCast>, ::windows::core::GetTrustLevel, StartTransmission::<Impl, OFFSET>)
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IWdsTransportNamespaceScheduledCastAutoStartImpl: Sized + IWdsTransportNamespaceScheduledCastImpl + IWdsTransportNamespaceImpl + IDispatchImpl {
    fn MinimumClients();
    fn SetMinimumClients();
    fn StartTime();
    fn SetStartTime();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for IWdsTransportNamespaceScheduledCastAutoStart {
    const NAME: &'static str = "Windows.Win32.System.DeploymentServices.IWdsTransportNamespaceScheduledCastAutoStart";
}
#[cfg(feature = "Win32_System_Com")]
impl IWdsTransportNamespaceScheduledCastAutoStartVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWdsTransportNamespaceScheduledCastAutoStartImpl, const OFFSET: isize>() -> IWdsTransportNamespaceScheduledCastAutoStartVtbl {
        unsafe extern "system" fn MinimumClients<Impl: IWdsTransportNamespaceScheduledCastAutoStartImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pulminimumclients: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MinimumClients(::core::mem::transmute_copy(&pulminimumclients)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMinimumClients<Impl: IWdsTransportNamespaceScheduledCastAutoStartImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ulminimumclients: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetMinimumClients(ulminimumclients) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn StartTime<Impl: IWdsTransportNamespaceScheduledCastAutoStartImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pstarttime: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).StartTime(::core::mem::transmute_copy(&pstarttime)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetStartTime<Impl: IWdsTransportNamespaceScheduledCastAutoStartImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, starttime: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetStartTime(starttime) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IWdsTransportNamespaceScheduledCastAutoStart>, ::windows::core::GetTrustLevel, MinimumClients::<Impl, OFFSET>, SetMinimumClients::<Impl, OFFSET>, StartTime::<Impl, OFFSET>, SetStartTime::<Impl, OFFSET>)
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IWdsTransportNamespaceScheduledCastManualStartImpl: Sized + IWdsTransportNamespaceScheduledCastImpl + IWdsTransportNamespaceImpl + IDispatchImpl {}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for IWdsTransportNamespaceScheduledCastManualStart {
    const NAME: &'static str = "Windows.Win32.System.DeploymentServices.IWdsTransportNamespaceScheduledCastManualStart";
}
#[cfg(feature = "Win32_System_Com")]
impl IWdsTransportNamespaceScheduledCastManualStartVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWdsTransportNamespaceScheduledCastManualStartImpl, const OFFSET: isize>() -> IWdsTransportNamespaceScheduledCastManualStartVtbl {
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IWdsTransportNamespaceScheduledCastManualStart>, ::windows::core::GetTrustLevel)
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IWdsTransportServerImpl: Sized + IDispatchImpl {
    fn Name();
    fn SetupManager();
    fn ConfigurationManager();
    fn NamespaceManager();
    fn DisconnectClient();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for IWdsTransportServer {
    const NAME: &'static str = "Windows.Win32.System.DeploymentServices.IWdsTransportServer";
}
#[cfg(feature = "Win32_System_Com")]
impl IWdsTransportServerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWdsTransportServerImpl, const OFFSET: isize>() -> IWdsTransportServerVtbl {
        unsafe extern "system" fn Name<Impl: IWdsTransportServerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbszname: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Name(::core::mem::transmute_copy(&pbszname)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetupManager<Impl: IWdsTransportServerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppwdstransportsetupmanager: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetupManager(::core::mem::transmute_copy(&ppwdstransportsetupmanager)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ConfigurationManager<Impl: IWdsTransportServerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppwdstransportconfigurationmanager: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ConfigurationManager(::core::mem::transmute_copy(&ppwdstransportconfigurationmanager)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn NamespaceManager<Impl: IWdsTransportServerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppwdstransportnamespacemanager: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).NamespaceManager(::core::mem::transmute_copy(&ppwdstransportnamespacemanager)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DisconnectClient<Impl: IWdsTransportServerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ulclientid: u32, disconnectiontype: WDSTRANSPORT_DISCONNECT_TYPE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DisconnectClient(ulclientid, disconnectiontype) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IWdsTransportServer>, ::windows::core::GetTrustLevel, Name::<Impl, OFFSET>, SetupManager::<Impl, OFFSET>, ConfigurationManager::<Impl, OFFSET>, NamespaceManager::<Impl, OFFSET>, DisconnectClient::<Impl, OFFSET>)
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IWdsTransportServer2Impl: Sized + IWdsTransportServerImpl + IDispatchImpl {
    fn TftpManager();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for IWdsTransportServer2 {
    const NAME: &'static str = "Windows.Win32.System.DeploymentServices.IWdsTransportServer2";
}
#[cfg(feature = "Win32_System_Com")]
impl IWdsTransportServer2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWdsTransportServer2Impl, const OFFSET: isize>() -> IWdsTransportServer2Vtbl {
        unsafe extern "system" fn TftpManager<Impl: IWdsTransportServer2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppwdstransporttftpmanager: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TftpManager(::core::mem::transmute_copy(&ppwdstransporttftpmanager)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IWdsTransportServer2>, ::windows::core::GetTrustLevel, TftpManager::<Impl, OFFSET>)
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IWdsTransportServicePolicyImpl: Sized + IWdsTransportCacheableImpl + IDispatchImpl {
    fn IpAddressSource();
    fn SetIpAddressSource();
    fn StartIpAddress();
    fn SetStartIpAddress();
    fn EndIpAddress();
    fn SetEndIpAddress();
    fn StartPort();
    fn SetStartPort();
    fn EndPort();
    fn SetEndPort();
    fn NetworkProfile();
    fn SetNetworkProfile();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for IWdsTransportServicePolicy {
    const NAME: &'static str = "Windows.Win32.System.DeploymentServices.IWdsTransportServicePolicy";
}
#[cfg(feature = "Win32_System_Com")]
impl IWdsTransportServicePolicyVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWdsTransportServicePolicyImpl, const OFFSET: isize>() -> IWdsTransportServicePolicyVtbl {
        unsafe extern "system" fn IpAddressSource<Impl: IWdsTransportServicePolicyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, addresstype: WDSTRANSPORT_IP_ADDRESS_TYPE, psourcetype: *mut WDSTRANSPORT_IP_ADDRESS_SOURCE_TYPE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IpAddressSource(addresstype, ::core::mem::transmute_copy(&psourcetype)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIpAddressSource<Impl: IWdsTransportServicePolicyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, addresstype: WDSTRANSPORT_IP_ADDRESS_TYPE, sourcetype: WDSTRANSPORT_IP_ADDRESS_SOURCE_TYPE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetIpAddressSource(addresstype, sourcetype) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn StartIpAddress<Impl: IWdsTransportServicePolicyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, addresstype: WDSTRANSPORT_IP_ADDRESS_TYPE, pbszstartipaddress: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).StartIpAddress(addresstype, ::core::mem::transmute_copy(&pbszstartipaddress)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetStartIpAddress<Impl: IWdsTransportServicePolicyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, addresstype: WDSTRANSPORT_IP_ADDRESS_TYPE, bszstartipaddress: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetStartIpAddress(addresstype, &*(&bszstartipaddress as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EndIpAddress<Impl: IWdsTransportServicePolicyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, addresstype: WDSTRANSPORT_IP_ADDRESS_TYPE, pbszendipaddress: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EndIpAddress(addresstype, ::core::mem::transmute_copy(&pbszendipaddress)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetEndIpAddress<Impl: IWdsTransportServicePolicyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, addresstype: WDSTRANSPORT_IP_ADDRESS_TYPE, bszendipaddress: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetEndIpAddress(addresstype, &*(&bszendipaddress as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn StartPort<Impl: IWdsTransportServicePolicyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pulstartport: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).StartPort(::core::mem::transmute_copy(&pulstartport)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetStartPort<Impl: IWdsTransportServicePolicyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ulstartport: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetStartPort(ulstartport) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EndPort<Impl: IWdsTransportServicePolicyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pulendport: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EndPort(::core::mem::transmute_copy(&pulendport)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetEndPort<Impl: IWdsTransportServicePolicyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ulendport: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetEndPort(ulendport) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn NetworkProfile<Impl: IWdsTransportServicePolicyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pprofiletype: *mut WDSTRANSPORT_NETWORK_PROFILE_TYPE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).NetworkProfile(::core::mem::transmute_copy(&pprofiletype)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetNetworkProfile<Impl: IWdsTransportServicePolicyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, profiletype: WDSTRANSPORT_NETWORK_PROFILE_TYPE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetNetworkProfile(profiletype) {
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
            ::windows::core::GetRuntimeClassName::<IWdsTransportServicePolicy>,
            ::windows::core::GetTrustLevel,
            IpAddressSource::<Impl, OFFSET>,
            SetIpAddressSource::<Impl, OFFSET>,
            StartIpAddress::<Impl, OFFSET>,
            SetStartIpAddress::<Impl, OFFSET>,
            EndIpAddress::<Impl, OFFSET>,
            SetEndIpAddress::<Impl, OFFSET>,
            StartPort::<Impl, OFFSET>,
            SetStartPort::<Impl, OFFSET>,
            EndPort::<Impl, OFFSET>,
            SetEndPort::<Impl, OFFSET>,
            NetworkProfile::<Impl, OFFSET>,
            SetNetworkProfile::<Impl, OFFSET>,
        )
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IWdsTransportServicePolicy2Impl: Sized + IWdsTransportServicePolicyImpl + IWdsTransportCacheableImpl + IDispatchImpl {
    fn UdpPortPolicy();
    fn SetUdpPortPolicy();
    fn TftpMaximumBlockSize();
    fn SetTftpMaximumBlockSize();
    fn EnableTftpVariableWindowExtension();
    fn SetEnableTftpVariableWindowExtension();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for IWdsTransportServicePolicy2 {
    const NAME: &'static str = "Windows.Win32.System.DeploymentServices.IWdsTransportServicePolicy2";
}
#[cfg(feature = "Win32_System_Com")]
impl IWdsTransportServicePolicy2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWdsTransportServicePolicy2Impl, const OFFSET: isize>() -> IWdsTransportServicePolicy2Vtbl {
        unsafe extern "system" fn UdpPortPolicy<Impl: IWdsTransportServicePolicy2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pudpportpolicy: *mut WDSTRANSPORT_UDP_PORT_POLICY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).UdpPortPolicy(::core::mem::transmute_copy(&pudpportpolicy)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetUdpPortPolicy<Impl: IWdsTransportServicePolicy2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, udpportpolicy: WDSTRANSPORT_UDP_PORT_POLICY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetUdpPortPolicy(udpportpolicy) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TftpMaximumBlockSize<Impl: IWdsTransportServicePolicy2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pultftpmaximumblocksize: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TftpMaximumBlockSize(::core::mem::transmute_copy(&pultftpmaximumblocksize)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetTftpMaximumBlockSize<Impl: IWdsTransportServicePolicy2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ultftpmaximumblocksize: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetTftpMaximumBlockSize(ultftpmaximumblocksize) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnableTftpVariableWindowExtension<Impl: IWdsTransportServicePolicy2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbenabletftpvariablewindowextension: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EnableTftpVariableWindowExtension(::core::mem::transmute_copy(&pbenabletftpvariablewindowextension)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetEnableTftpVariableWindowExtension<Impl: IWdsTransportServicePolicy2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, benabletftpvariablewindowextension: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetEnableTftpVariableWindowExtension(benabletftpvariablewindowextension) {
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
            ::windows::core::GetRuntimeClassName::<IWdsTransportServicePolicy2>,
            ::windows::core::GetTrustLevel,
            UdpPortPolicy::<Impl, OFFSET>,
            SetUdpPortPolicy::<Impl, OFFSET>,
            TftpMaximumBlockSize::<Impl, OFFSET>,
            SetTftpMaximumBlockSize::<Impl, OFFSET>,
            EnableTftpVariableWindowExtension::<Impl, OFFSET>,
            SetEnableTftpVariableWindowExtension::<Impl, OFFSET>,
        )
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IWdsTransportSessionImpl: Sized + IDispatchImpl {
    fn Content();
    fn Id();
    fn NetworkInterfaceName();
    fn NetworkInterfaceAddress();
    fn TransferRate();
    fn MasterClientId();
    fn RetrieveClients();
    fn Terminate();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for IWdsTransportSession {
    const NAME: &'static str = "Windows.Win32.System.DeploymentServices.IWdsTransportSession";
}
#[cfg(feature = "Win32_System_Com")]
impl IWdsTransportSessionVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWdsTransportSessionImpl, const OFFSET: isize>() -> IWdsTransportSessionVtbl {
        unsafe extern "system" fn Content<Impl: IWdsTransportSessionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppwdstransportcontent: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Content(::core::mem::transmute_copy(&ppwdstransportcontent)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Id<Impl: IWdsTransportSessionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pulid: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Id(::core::mem::transmute_copy(&pulid)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn NetworkInterfaceName<Impl: IWdsTransportSessionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbsznetworkinterfacename: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).NetworkInterfaceName(::core::mem::transmute_copy(&pbsznetworkinterfacename)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn NetworkInterfaceAddress<Impl: IWdsTransportSessionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbsznetworkinterfaceaddress: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).NetworkInterfaceAddress(::core::mem::transmute_copy(&pbsznetworkinterfaceaddress)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TransferRate<Impl: IWdsTransportSessionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pultransferrate: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TransferRate(::core::mem::transmute_copy(&pultransferrate)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MasterClientId<Impl: IWdsTransportSessionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pulmasterclientid: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MasterClientId(::core::mem::transmute_copy(&pulmasterclientid)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RetrieveClients<Impl: IWdsTransportSessionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppwdstransportclients: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RetrieveClients(::core::mem::transmute_copy(&ppwdstransportclients)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Terminate<Impl: IWdsTransportSessionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Terminate() {
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
            ::windows::core::GetRuntimeClassName::<IWdsTransportSession>,
            ::windows::core::GetTrustLevel,
            Content::<Impl, OFFSET>,
            Id::<Impl, OFFSET>,
            NetworkInterfaceName::<Impl, OFFSET>,
            NetworkInterfaceAddress::<Impl, OFFSET>,
            TransferRate::<Impl, OFFSET>,
            MasterClientId::<Impl, OFFSET>,
            RetrieveClients::<Impl, OFFSET>,
            Terminate::<Impl, OFFSET>,
        )
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IWdsTransportSetupManagerImpl: Sized + IDispatchImpl {
    fn Version();
    fn InstalledFeatures();
    fn Protocols();
    fn RegisterContentProvider();
    fn DeregisterContentProvider();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for IWdsTransportSetupManager {
    const NAME: &'static str = "Windows.Win32.System.DeploymentServices.IWdsTransportSetupManager";
}
#[cfg(feature = "Win32_System_Com")]
impl IWdsTransportSetupManagerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWdsTransportSetupManagerImpl, const OFFSET: isize>() -> IWdsTransportSetupManagerVtbl {
        unsafe extern "system" fn Version<Impl: IWdsTransportSetupManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pullversion: *mut u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Version(::core::mem::transmute_copy(&pullversion)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn InstalledFeatures<Impl: IWdsTransportSetupManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pulinstalledfeatures: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).InstalledFeatures(::core::mem::transmute_copy(&pulinstalledfeatures)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Protocols<Impl: IWdsTransportSetupManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pulprotocols: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Protocols(::core::mem::transmute_copy(&pulprotocols)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RegisterContentProvider<Impl: IWdsTransportSetupManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bszname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bszdescription: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bszfilepath: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bszinitializationroutine: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RegisterContentProvider(
                &*(&bszname as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&bszdescription as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&bszfilepath as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&bszinitializationroutine as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DeregisterContentProvider<Impl: IWdsTransportSetupManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bszname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DeregisterContentProvider(&*(&bszname as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IWdsTransportSetupManager>, ::windows::core::GetTrustLevel, Version::<Impl, OFFSET>, InstalledFeatures::<Impl, OFFSET>, Protocols::<Impl, OFFSET>, RegisterContentProvider::<Impl, OFFSET>, DeregisterContentProvider::<Impl, OFFSET>)
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IWdsTransportSetupManager2Impl: Sized + IWdsTransportSetupManagerImpl + IDispatchImpl {
    fn TftpCapabilities();
    fn ContentProviders();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for IWdsTransportSetupManager2 {
    const NAME: &'static str = "Windows.Win32.System.DeploymentServices.IWdsTransportSetupManager2";
}
#[cfg(feature = "Win32_System_Com")]
impl IWdsTransportSetupManager2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWdsTransportSetupManager2Impl, const OFFSET: isize>() -> IWdsTransportSetupManager2Vtbl {
        unsafe extern "system" fn TftpCapabilities<Impl: IWdsTransportSetupManager2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pultftpcapabilities: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TftpCapabilities(::core::mem::transmute_copy(&pultftpcapabilities)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ContentProviders<Impl: IWdsTransportSetupManager2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppprovidercollection: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ContentProviders(::core::mem::transmute_copy(&ppprovidercollection)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IWdsTransportSetupManager2>, ::windows::core::GetTrustLevel, TftpCapabilities::<Impl, OFFSET>, ContentProviders::<Impl, OFFSET>)
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IWdsTransportTftpClientImpl: Sized + IDispatchImpl {
    fn FileName();
    fn IpAddress();
    fn Timeout();
    fn CurrentFileOffset();
    fn FileSize();
    fn BlockSize();
    fn WindowSize();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for IWdsTransportTftpClient {
    const NAME: &'static str = "Windows.Win32.System.DeploymentServices.IWdsTransportTftpClient";
}
#[cfg(feature = "Win32_System_Com")]
impl IWdsTransportTftpClientVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWdsTransportTftpClientImpl, const OFFSET: isize>() -> IWdsTransportTftpClientVtbl {
        unsafe extern "system" fn FileName<Impl: IWdsTransportTftpClientImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbszfilename: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FileName(::core::mem::transmute_copy(&pbszfilename)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IpAddress<Impl: IWdsTransportTftpClientImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbszipaddress: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IpAddress(::core::mem::transmute_copy(&pbszipaddress)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Timeout<Impl: IWdsTransportTftpClientImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pultimeout: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Timeout(::core::mem::transmute_copy(&pultimeout)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CurrentFileOffset<Impl: IWdsTransportTftpClientImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pul64currentoffset: *mut u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CurrentFileOffset(::core::mem::transmute_copy(&pul64currentoffset)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FileSize<Impl: IWdsTransportTftpClientImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pul64filesize: *mut u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FileSize(::core::mem::transmute_copy(&pul64filesize)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BlockSize<Impl: IWdsTransportTftpClientImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pulblocksize: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).BlockSize(::core::mem::transmute_copy(&pulblocksize)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn WindowSize<Impl: IWdsTransportTftpClientImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pulwindowsize: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).WindowSize(::core::mem::transmute_copy(&pulwindowsize)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IWdsTransportTftpClient>, ::windows::core::GetTrustLevel, FileName::<Impl, OFFSET>, IpAddress::<Impl, OFFSET>, Timeout::<Impl, OFFSET>, CurrentFileOffset::<Impl, OFFSET>, FileSize::<Impl, OFFSET>, BlockSize::<Impl, OFFSET>, WindowSize::<Impl, OFFSET>)
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IWdsTransportTftpManagerImpl: Sized + IDispatchImpl {
    fn RetrieveTftpClients();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for IWdsTransportTftpManager {
    const NAME: &'static str = "Windows.Win32.System.DeploymentServices.IWdsTransportTftpManager";
}
#[cfg(feature = "Win32_System_Com")]
impl IWdsTransportTftpManagerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWdsTransportTftpManagerImpl, const OFFSET: isize>() -> IWdsTransportTftpManagerVtbl {
        unsafe extern "system" fn RetrieveTftpClients<Impl: IWdsTransportTftpManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppwdstransporttftpclients: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RetrieveTftpClients(::core::mem::transmute_copy(&ppwdstransporttftpclients)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IWdsTransportTftpManager>, ::windows::core::GetTrustLevel, RetrieveTftpClients::<Impl, OFFSET>)
    }
}
