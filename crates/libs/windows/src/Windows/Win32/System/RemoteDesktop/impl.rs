#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IADsTSUserEx_Impl: Sized + super::Com::IDispatch_Impl {
    fn TerminalServicesProfilePath(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetTerminalServicesProfilePath(&mut self, pnewval: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn TerminalServicesHomeDirectory(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetTerminalServicesHomeDirectory(&mut self, pnewval: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn TerminalServicesHomeDrive(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetTerminalServicesHomeDrive(&mut self, pnewval: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn AllowLogon(&mut self) -> ::windows::core::Result<i32>;
    fn SetAllowLogon(&mut self, newval: i32) -> ::windows::core::Result<()>;
    fn EnableRemoteControl(&mut self) -> ::windows::core::Result<i32>;
    fn SetEnableRemoteControl(&mut self, newval: i32) -> ::windows::core::Result<()>;
    fn MaxDisconnectionTime(&mut self) -> ::windows::core::Result<i32>;
    fn SetMaxDisconnectionTime(&mut self, newval: i32) -> ::windows::core::Result<()>;
    fn MaxConnectionTime(&mut self) -> ::windows::core::Result<i32>;
    fn SetMaxConnectionTime(&mut self, newval: i32) -> ::windows::core::Result<()>;
    fn MaxIdleTime(&mut self) -> ::windows::core::Result<i32>;
    fn SetMaxIdleTime(&mut self, newval: i32) -> ::windows::core::Result<()>;
    fn ReconnectionAction(&mut self) -> ::windows::core::Result<i32>;
    fn SetReconnectionAction(&mut self, newval: i32) -> ::windows::core::Result<()>;
    fn BrokenConnectionAction(&mut self) -> ::windows::core::Result<i32>;
    fn SetBrokenConnectionAction(&mut self, newval: i32) -> ::windows::core::Result<()>;
    fn ConnectClientDrivesAtLogon(&mut self) -> ::windows::core::Result<i32>;
    fn SetConnectClientDrivesAtLogon(&mut self, newval: i32) -> ::windows::core::Result<()>;
    fn ConnectClientPrintersAtLogon(&mut self) -> ::windows::core::Result<i32>;
    fn SetConnectClientPrintersAtLogon(&mut self, newval: i32) -> ::windows::core::Result<()>;
    fn DefaultToMainPrinter(&mut self) -> ::windows::core::Result<i32>;
    fn SetDefaultToMainPrinter(&mut self, newval: i32) -> ::windows::core::Result<()>;
    fn TerminalServicesWorkDirectory(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetTerminalServicesWorkDirectory(&mut self, pnewval: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn TerminalServicesInitialProgram(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetTerminalServicesInitialProgram(&mut self, pnewval: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IADsTSUserEx_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IADsTSUserEx_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IADsTSUserEx_Vtbl {
        unsafe extern "system" fn TerminalServicesProfilePath<Impl: IADsTSUserEx_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TerminalServicesProfilePath() {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetTerminalServicesProfilePath<Impl: IADsTSUserEx_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pnewval: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetTerminalServicesProfilePath(::core::mem::transmute_copy(&pnewval)).into()
        }
        unsafe extern "system" fn TerminalServicesHomeDirectory<Impl: IADsTSUserEx_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TerminalServicesHomeDirectory() {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetTerminalServicesHomeDirectory<Impl: IADsTSUserEx_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pnewval: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetTerminalServicesHomeDirectory(::core::mem::transmute_copy(&pnewval)).into()
        }
        unsafe extern "system" fn TerminalServicesHomeDrive<Impl: IADsTSUserEx_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TerminalServicesHomeDrive() {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetTerminalServicesHomeDrive<Impl: IADsTSUserEx_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pnewval: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetTerminalServicesHomeDrive(::core::mem::transmute_copy(&pnewval)).into()
        }
        unsafe extern "system" fn AllowLogon<Impl: IADsTSUserEx_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AllowLogon() {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAllowLogon<Impl: IADsTSUserEx_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newval: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetAllowLogon(::core::mem::transmute_copy(&newval)).into()
        }
        unsafe extern "system" fn EnableRemoteControl<Impl: IADsTSUserEx_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EnableRemoteControl() {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetEnableRemoteControl<Impl: IADsTSUserEx_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newval: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetEnableRemoteControl(::core::mem::transmute_copy(&newval)).into()
        }
        unsafe extern "system" fn MaxDisconnectionTime<Impl: IADsTSUserEx_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MaxDisconnectionTime() {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMaxDisconnectionTime<Impl: IADsTSUserEx_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newval: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetMaxDisconnectionTime(::core::mem::transmute_copy(&newval)).into()
        }
        unsafe extern "system" fn MaxConnectionTime<Impl: IADsTSUserEx_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MaxConnectionTime() {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMaxConnectionTime<Impl: IADsTSUserEx_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newval: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetMaxConnectionTime(::core::mem::transmute_copy(&newval)).into()
        }
        unsafe extern "system" fn MaxIdleTime<Impl: IADsTSUserEx_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MaxIdleTime() {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMaxIdleTime<Impl: IADsTSUserEx_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newval: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetMaxIdleTime(::core::mem::transmute_copy(&newval)).into()
        }
        unsafe extern "system" fn ReconnectionAction<Impl: IADsTSUserEx_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pnewval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ReconnectionAction() {
                ::core::result::Result::Ok(ok__) => {
                    *pnewval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetReconnectionAction<Impl: IADsTSUserEx_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newval: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetReconnectionAction(::core::mem::transmute_copy(&newval)).into()
        }
        unsafe extern "system" fn BrokenConnectionAction<Impl: IADsTSUserEx_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pnewval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).BrokenConnectionAction() {
                ::core::result::Result::Ok(ok__) => {
                    *pnewval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetBrokenConnectionAction<Impl: IADsTSUserEx_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newval: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetBrokenConnectionAction(::core::mem::transmute_copy(&newval)).into()
        }
        unsafe extern "system" fn ConnectClientDrivesAtLogon<Impl: IADsTSUserEx_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pnewval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ConnectClientDrivesAtLogon() {
                ::core::result::Result::Ok(ok__) => {
                    *pnewval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetConnectClientDrivesAtLogon<Impl: IADsTSUserEx_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newval: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetConnectClientDrivesAtLogon(::core::mem::transmute_copy(&newval)).into()
        }
        unsafe extern "system" fn ConnectClientPrintersAtLogon<Impl: IADsTSUserEx_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ConnectClientPrintersAtLogon() {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetConnectClientPrintersAtLogon<Impl: IADsTSUserEx_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newval: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetConnectClientPrintersAtLogon(::core::mem::transmute_copy(&newval)).into()
        }
        unsafe extern "system" fn DefaultToMainPrinter<Impl: IADsTSUserEx_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DefaultToMainPrinter() {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDefaultToMainPrinter<Impl: IADsTSUserEx_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newval: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetDefaultToMainPrinter(::core::mem::transmute_copy(&newval)).into()
        }
        unsafe extern "system" fn TerminalServicesWorkDirectory<Impl: IADsTSUserEx_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TerminalServicesWorkDirectory() {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetTerminalServicesWorkDirectory<Impl: IADsTSUserEx_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pnewval: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetTerminalServicesWorkDirectory(::core::mem::transmute_copy(&pnewval)).into()
        }
        unsafe extern "system" fn TerminalServicesInitialProgram<Impl: IADsTSUserEx_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TerminalServicesInitialProgram() {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetTerminalServicesInitialProgram<Impl: IADsTSUserEx_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pnewval: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetTerminalServicesInitialProgram(::core::mem::transmute_copy(&pnewval)).into()
        }
        Self {
            base: super::Com::IDispatch_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            TerminalServicesProfilePath: TerminalServicesProfilePath::<Impl, IMPL_OFFSET>,
            SetTerminalServicesProfilePath: SetTerminalServicesProfilePath::<Impl, IMPL_OFFSET>,
            TerminalServicesHomeDirectory: TerminalServicesHomeDirectory::<Impl, IMPL_OFFSET>,
            SetTerminalServicesHomeDirectory: SetTerminalServicesHomeDirectory::<Impl, IMPL_OFFSET>,
            TerminalServicesHomeDrive: TerminalServicesHomeDrive::<Impl, IMPL_OFFSET>,
            SetTerminalServicesHomeDrive: SetTerminalServicesHomeDrive::<Impl, IMPL_OFFSET>,
            AllowLogon: AllowLogon::<Impl, IMPL_OFFSET>,
            SetAllowLogon: SetAllowLogon::<Impl, IMPL_OFFSET>,
            EnableRemoteControl: EnableRemoteControl::<Impl, IMPL_OFFSET>,
            SetEnableRemoteControl: SetEnableRemoteControl::<Impl, IMPL_OFFSET>,
            MaxDisconnectionTime: MaxDisconnectionTime::<Impl, IMPL_OFFSET>,
            SetMaxDisconnectionTime: SetMaxDisconnectionTime::<Impl, IMPL_OFFSET>,
            MaxConnectionTime: MaxConnectionTime::<Impl, IMPL_OFFSET>,
            SetMaxConnectionTime: SetMaxConnectionTime::<Impl, IMPL_OFFSET>,
            MaxIdleTime: MaxIdleTime::<Impl, IMPL_OFFSET>,
            SetMaxIdleTime: SetMaxIdleTime::<Impl, IMPL_OFFSET>,
            ReconnectionAction: ReconnectionAction::<Impl, IMPL_OFFSET>,
            SetReconnectionAction: SetReconnectionAction::<Impl, IMPL_OFFSET>,
            BrokenConnectionAction: BrokenConnectionAction::<Impl, IMPL_OFFSET>,
            SetBrokenConnectionAction: SetBrokenConnectionAction::<Impl, IMPL_OFFSET>,
            ConnectClientDrivesAtLogon: ConnectClientDrivesAtLogon::<Impl, IMPL_OFFSET>,
            SetConnectClientDrivesAtLogon: SetConnectClientDrivesAtLogon::<Impl, IMPL_OFFSET>,
            ConnectClientPrintersAtLogon: ConnectClientPrintersAtLogon::<Impl, IMPL_OFFSET>,
            SetConnectClientPrintersAtLogon: SetConnectClientPrintersAtLogon::<Impl, IMPL_OFFSET>,
            DefaultToMainPrinter: DefaultToMainPrinter::<Impl, IMPL_OFFSET>,
            SetDefaultToMainPrinter: SetDefaultToMainPrinter::<Impl, IMPL_OFFSET>,
            TerminalServicesWorkDirectory: TerminalServicesWorkDirectory::<Impl, IMPL_OFFSET>,
            SetTerminalServicesWorkDirectory: SetTerminalServicesWorkDirectory::<Impl, IMPL_OFFSET>,
            TerminalServicesInitialProgram: TerminalServicesInitialProgram::<Impl, IMPL_OFFSET>,
            SetTerminalServicesInitialProgram: SetTerminalServicesInitialProgram::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IADsTSUserEx as ::windows::core::Interface>::IID || iid == &<super::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IAudioDeviceEndpoint_Impl: Sized {
    fn SetBuffer(&mut self, maxperiod: i64, u32latencycoefficient: u32) -> ::windows::core::Result<()>;
    fn GetRTCaps(&mut self) -> ::windows::core::Result<super::super::Foundation::BOOL>;
    fn GetEventDrivenCapable(&mut self) -> ::windows::core::Result<super::super::Foundation::BOOL>;
    fn WriteExclusiveModeParametersToSharedMemory(&mut self, htargetprocess: usize, hnsperiod: i64, hnsbufferduration: i64, u32latencycoefficient: u32, pu32sharedmemorysize: *mut u32, phsharedmemory: *mut usize) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IAudioDeviceEndpoint_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAudioDeviceEndpoint_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAudioDeviceEndpoint_Vtbl {
        unsafe extern "system" fn SetBuffer<Impl: IAudioDeviceEndpoint_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, maxperiod: i64, u32latencycoefficient: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetBuffer(::core::mem::transmute_copy(&maxperiod), ::core::mem::transmute_copy(&u32latencycoefficient)).into()
        }
        unsafe extern "system" fn GetRTCaps<Impl: IAudioDeviceEndpoint_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbisrtcapable: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetRTCaps() {
                ::core::result::Result::Ok(ok__) => {
                    *pbisrtcapable = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetEventDrivenCapable<Impl: IAudioDeviceEndpoint_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbiseventcapable: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetEventDrivenCapable() {
                ::core::result::Result::Ok(ok__) => {
                    *pbiseventcapable = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn WriteExclusiveModeParametersToSharedMemory<Impl: IAudioDeviceEndpoint_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, htargetprocess: usize, hnsperiod: i64, hnsbufferduration: i64, u32latencycoefficient: u32, pu32sharedmemorysize: *mut u32, phsharedmemory: *mut usize) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).WriteExclusiveModeParametersToSharedMemory(::core::mem::transmute_copy(&htargetprocess), ::core::mem::transmute_copy(&hnsperiod), ::core::mem::transmute_copy(&hnsbufferduration), ::core::mem::transmute_copy(&u32latencycoefficient), ::core::mem::transmute_copy(&pu32sharedmemorysize), ::core::mem::transmute_copy(&phsharedmemory)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            SetBuffer: SetBuffer::<Impl, IMPL_OFFSET>,
            GetRTCaps: GetRTCaps::<Impl, IMPL_OFFSET>,
            GetEventDrivenCapable: GetEventDrivenCapable::<Impl, IMPL_OFFSET>,
            WriteExclusiveModeParametersToSharedMemory: WriteExclusiveModeParametersToSharedMemory::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAudioDeviceEndpoint as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Media_Audio"))]
pub trait IAudioEndpoint_Impl: Sized {
    fn GetFrameFormat(&mut self) -> ::windows::core::Result<*mut super::super::Media::Audio::WAVEFORMATEX>;
    fn GetFramesPerPacket(&mut self) -> ::windows::core::Result<u32>;
    fn GetLatency(&mut self) -> ::windows::core::Result<i64>;
    fn SetStreamFlags(&mut self, streamflags: u32) -> ::windows::core::Result<()>;
    fn SetEventHandle(&mut self, eventhandle: super::super::Foundation::HANDLE) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Media_Audio"))]
impl IAudioEndpoint_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAudioEndpoint_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAudioEndpoint_Vtbl {
        unsafe extern "system" fn GetFrameFormat<Impl: IAudioEndpoint_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppformat: *mut *mut super::super::Media::Audio::WAVEFORMATEX) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetFrameFormat() {
                ::core::result::Result::Ok(ok__) => {
                    *ppformat = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFramesPerPacket<Impl: IAudioEndpoint_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pframesperpacket: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetFramesPerPacket() {
                ::core::result::Result::Ok(ok__) => {
                    *pframesperpacket = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetLatency<Impl: IAudioEndpoint_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, platency: *mut i64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetLatency() {
                ::core::result::Result::Ok(ok__) => {
                    *platency = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetStreamFlags<Impl: IAudioEndpoint_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, streamflags: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetStreamFlags(::core::mem::transmute_copy(&streamflags)).into()
        }
        unsafe extern "system" fn SetEventHandle<Impl: IAudioEndpoint_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, eventhandle: super::super::Foundation::HANDLE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetEventHandle(::core::mem::transmute_copy(&eventhandle)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            GetFrameFormat: GetFrameFormat::<Impl, IMPL_OFFSET>,
            GetFramesPerPacket: GetFramesPerPacket::<Impl, IMPL_OFFSET>,
            GetLatency: GetLatency::<Impl, IMPL_OFFSET>,
            SetStreamFlags: SetStreamFlags::<Impl, IMPL_OFFSET>,
            SetEventHandle: SetEventHandle::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAudioEndpoint as ::windows::core::Interface>::IID
    }
}
pub trait IAudioEndpointControl_Impl: Sized {
    fn Start(&mut self) -> ::windows::core::Result<()>;
    fn Reset(&mut self) -> ::windows::core::Result<()>;
    fn Stop(&mut self) -> ::windows::core::Result<()>;
}
impl IAudioEndpointControl_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAudioEndpointControl_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAudioEndpointControl_Vtbl {
        unsafe extern "system" fn Start<Impl: IAudioEndpointControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Start().into()
        }
        unsafe extern "system" fn Reset<Impl: IAudioEndpointControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Reset().into()
        }
        unsafe extern "system" fn Stop<Impl: IAudioEndpointControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Stop().into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            Start: Start::<Impl, IMPL_OFFSET>,
            Reset: Reset::<Impl, IMPL_OFFSET>,
            Stop: Stop::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAudioEndpointControl as ::windows::core::Interface>::IID
    }
}
pub trait IAudioEndpointRT_Impl: Sized {
    fn GetCurrentPadding(&mut self, ppadding: *mut i64, paecurrentposition: *mut AE_CURRENT_POSITION);
    fn ProcessingComplete(&mut self);
    fn SetPinInactive(&mut self) -> ::windows::core::Result<()>;
    fn SetPinActive(&mut self) -> ::windows::core::Result<()>;
}
impl IAudioEndpointRT_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAudioEndpointRT_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAudioEndpointRT_Vtbl {
        unsafe extern "system" fn GetCurrentPadding<Impl: IAudioEndpointRT_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppadding: *mut i64, paecurrentposition: *mut AE_CURRENT_POSITION) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetCurrentPadding(::core::mem::transmute_copy(&ppadding), ::core::mem::transmute_copy(&paecurrentposition))
        }
        unsafe extern "system" fn ProcessingComplete<Impl: IAudioEndpointRT_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ProcessingComplete()
        }
        unsafe extern "system" fn SetPinInactive<Impl: IAudioEndpointRT_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetPinInactive().into()
        }
        unsafe extern "system" fn SetPinActive<Impl: IAudioEndpointRT_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetPinActive().into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            GetCurrentPadding: GetCurrentPadding::<Impl, IMPL_OFFSET>,
            ProcessingComplete: ProcessingComplete::<Impl, IMPL_OFFSET>,
            SetPinInactive: SetPinInactive::<Impl, IMPL_OFFSET>,
            SetPinActive: SetPinActive::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAudioEndpointRT as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Media_Audio_Apo")]
pub trait IAudioInputEndpointRT_Impl: Sized {
    fn GetInputDataPointer(&mut self, pconnectionproperty: *mut super::super::Media::Audio::Apo::APO_CONNECTION_PROPERTY, paetimestamp: *mut AE_CURRENT_POSITION);
    fn ReleaseInputDataPointer(&mut self, u32framecount: u32, pdatapointer: usize);
    fn PulseEndpoint(&mut self);
}
#[cfg(feature = "Win32_Media_Audio_Apo")]
impl IAudioInputEndpointRT_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAudioInputEndpointRT_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAudioInputEndpointRT_Vtbl {
        unsafe extern "system" fn GetInputDataPointer<Impl: IAudioInputEndpointRT_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pconnectionproperty: *mut super::super::Media::Audio::Apo::APO_CONNECTION_PROPERTY, paetimestamp: *mut AE_CURRENT_POSITION) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetInputDataPointer(::core::mem::transmute_copy(&pconnectionproperty), ::core::mem::transmute_copy(&paetimestamp))
        }
        unsafe extern "system" fn ReleaseInputDataPointer<Impl: IAudioInputEndpointRT_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, u32framecount: u32, pdatapointer: usize) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ReleaseInputDataPointer(::core::mem::transmute_copy(&u32framecount), ::core::mem::transmute_copy(&pdatapointer))
        }
        unsafe extern "system" fn PulseEndpoint<Impl: IAudioInputEndpointRT_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).PulseEndpoint()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            GetInputDataPointer: GetInputDataPointer::<Impl, IMPL_OFFSET>,
            ReleaseInputDataPointer: ReleaseInputDataPointer::<Impl, IMPL_OFFSET>,
            PulseEndpoint: PulseEndpoint::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAudioInputEndpointRT as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Media_Audio_Apo")]
pub trait IAudioOutputEndpointRT_Impl: Sized {
    fn GetOutputDataPointer(&mut self, u32framecount: u32, paetimestamp: *const AE_CURRENT_POSITION) -> usize;
    fn ReleaseOutputDataPointer(&mut self, pconnectionproperty: *const super::super::Media::Audio::Apo::APO_CONNECTION_PROPERTY);
    fn PulseEndpoint(&mut self);
}
#[cfg(feature = "Win32_Media_Audio_Apo")]
impl IAudioOutputEndpointRT_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAudioOutputEndpointRT_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAudioOutputEndpointRT_Vtbl {
        unsafe extern "system" fn GetOutputDataPointer<Impl: IAudioOutputEndpointRT_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, u32framecount: u32, paetimestamp: *const AE_CURRENT_POSITION) -> usize {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetOutputDataPointer(::core::mem::transmute_copy(&u32framecount), ::core::mem::transmute_copy(&paetimestamp))
        }
        unsafe extern "system" fn ReleaseOutputDataPointer<Impl: IAudioOutputEndpointRT_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pconnectionproperty: *const super::super::Media::Audio::Apo::APO_CONNECTION_PROPERTY) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ReleaseOutputDataPointer(::core::mem::transmute_copy(&pconnectionproperty))
        }
        unsafe extern "system" fn PulseEndpoint<Impl: IAudioOutputEndpointRT_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).PulseEndpoint()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            GetOutputDataPointer: GetOutputDataPointer::<Impl, IMPL_OFFSET>,
            ReleaseOutputDataPointer: ReleaseOutputDataPointer::<Impl, IMPL_OFFSET>,
            PulseEndpoint: PulseEndpoint::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAudioOutputEndpointRT as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IRemoteDesktopClient_Impl: Sized + super::Com::IDispatch_Impl {
    fn Connect(&mut self) -> ::windows::core::Result<()>;
    fn Disconnect(&mut self) -> ::windows::core::Result<()>;
    fn Reconnect(&mut self, width: u32, height: u32) -> ::windows::core::Result<()>;
    fn Settings(&mut self) -> ::windows::core::Result<IRemoteDesktopClientSettings>;
    fn Actions(&mut self) -> ::windows::core::Result<IRemoteDesktopClientActions>;
    fn TouchPointer(&mut self) -> ::windows::core::Result<IRemoteDesktopClientTouchPointer>;
    fn DeleteSavedCredentials(&mut self, servername: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn UpdateSessionDisplaySettings(&mut self, width: u32, height: u32) -> ::windows::core::Result<()>;
    fn attachEvent(&mut self, eventname: &super::super::Foundation::BSTR, callback: &::core::option::Option<super::Com::IDispatch>) -> ::windows::core::Result<()>;
    fn detachEvent(&mut self, eventname: &super::super::Foundation::BSTR, callback: &::core::option::Option<super::Com::IDispatch>) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IRemoteDesktopClient_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRemoteDesktopClient_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IRemoteDesktopClient_Vtbl {
        unsafe extern "system" fn Connect<Impl: IRemoteDesktopClient_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Connect().into()
        }
        unsafe extern "system" fn Disconnect<Impl: IRemoteDesktopClient_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Disconnect().into()
        }
        unsafe extern "system" fn Reconnect<Impl: IRemoteDesktopClient_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, width: u32, height: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Reconnect(::core::mem::transmute_copy(&width), ::core::mem::transmute_copy(&height)).into()
        }
        unsafe extern "system" fn Settings<Impl: IRemoteDesktopClient_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, settings: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Settings() {
                ::core::result::Result::Ok(ok__) => {
                    *settings = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Actions<Impl: IRemoteDesktopClient_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, actions: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Actions() {
                ::core::result::Result::Ok(ok__) => {
                    *actions = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TouchPointer<Impl: IRemoteDesktopClient_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, touchpointer: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TouchPointer() {
                ::core::result::Result::Ok(ok__) => {
                    *touchpointer = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DeleteSavedCredentials<Impl: IRemoteDesktopClient_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, servername: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).DeleteSavedCredentials(::core::mem::transmute_copy(&servername)).into()
        }
        unsafe extern "system" fn UpdateSessionDisplaySettings<Impl: IRemoteDesktopClient_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, width: u32, height: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).UpdateSessionDisplaySettings(::core::mem::transmute_copy(&width), ::core::mem::transmute_copy(&height)).into()
        }
        unsafe extern "system" fn attachEvent<Impl: IRemoteDesktopClient_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, eventname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, callback: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).attachEvent(::core::mem::transmute_copy(&eventname), ::core::mem::transmute(&callback)).into()
        }
        unsafe extern "system" fn detachEvent<Impl: IRemoteDesktopClient_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, eventname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, callback: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).detachEvent(::core::mem::transmute_copy(&eventname), ::core::mem::transmute(&callback)).into()
        }
        Self {
            base: super::Com::IDispatch_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            Connect: Connect::<Impl, IMPL_OFFSET>,
            Disconnect: Disconnect::<Impl, IMPL_OFFSET>,
            Reconnect: Reconnect::<Impl, IMPL_OFFSET>,
            Settings: Settings::<Impl, IMPL_OFFSET>,
            Actions: Actions::<Impl, IMPL_OFFSET>,
            TouchPointer: TouchPointer::<Impl, IMPL_OFFSET>,
            DeleteSavedCredentials: DeleteSavedCredentials::<Impl, IMPL_OFFSET>,
            UpdateSessionDisplaySettings: UpdateSessionDisplaySettings::<Impl, IMPL_OFFSET>,
            attachEvent: attachEvent::<Impl, IMPL_OFFSET>,
            detachEvent: detachEvent::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRemoteDesktopClient as ::windows::core::Interface>::IID || iid == &<super::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IRemoteDesktopClientActions_Impl: Sized + super::Com::IDispatch_Impl {
    fn SuspendScreenUpdates(&mut self) -> ::windows::core::Result<()>;
    fn ResumeScreenUpdates(&mut self) -> ::windows::core::Result<()>;
    fn ExecuteRemoteAction(&mut self, remoteaction: RemoteActionType) -> ::windows::core::Result<()>;
    fn GetSnapshot(&mut self, snapshotencoding: SnapshotEncodingType, snapshotformat: SnapshotFormatType, snapshotwidth: u32, snapshotheight: u32) -> ::windows::core::Result<super::super::Foundation::BSTR>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IRemoteDesktopClientActions_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRemoteDesktopClientActions_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IRemoteDesktopClientActions_Vtbl {
        unsafe extern "system" fn SuspendScreenUpdates<Impl: IRemoteDesktopClientActions_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SuspendScreenUpdates().into()
        }
        unsafe extern "system" fn ResumeScreenUpdates<Impl: IRemoteDesktopClientActions_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ResumeScreenUpdates().into()
        }
        unsafe extern "system" fn ExecuteRemoteAction<Impl: IRemoteDesktopClientActions_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, remoteaction: RemoteActionType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ExecuteRemoteAction(::core::mem::transmute_copy(&remoteaction)).into()
        }
        unsafe extern "system" fn GetSnapshot<Impl: IRemoteDesktopClientActions_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, snapshotencoding: SnapshotEncodingType, snapshotformat: SnapshotFormatType, snapshotwidth: u32, snapshotheight: u32, snapshotdata: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetSnapshot(::core::mem::transmute_copy(&snapshotencoding), ::core::mem::transmute_copy(&snapshotformat), ::core::mem::transmute_copy(&snapshotwidth), ::core::mem::transmute_copy(&snapshotheight)) {
                ::core::result::Result::Ok(ok__) => {
                    *snapshotdata = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: super::Com::IDispatch_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            SuspendScreenUpdates: SuspendScreenUpdates::<Impl, IMPL_OFFSET>,
            ResumeScreenUpdates: ResumeScreenUpdates::<Impl, IMPL_OFFSET>,
            ExecuteRemoteAction: ExecuteRemoteAction::<Impl, IMPL_OFFSET>,
            GetSnapshot: GetSnapshot::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRemoteDesktopClientActions as ::windows::core::Interface>::IID || iid == &<super::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IRemoteDesktopClientSettings_Impl: Sized + super::Com::IDispatch_Impl {
    fn ApplySettings(&mut self, rdpfilecontents: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn RetrieveSettings(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn GetRdpProperty(&mut self, propertyname: &super::super::Foundation::BSTR) -> ::windows::core::Result<super::Com::VARIANT>;
    fn SetRdpProperty(&mut self, propertyname: &super::super::Foundation::BSTR, value: &super::Com::VARIANT) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IRemoteDesktopClientSettings_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRemoteDesktopClientSettings_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IRemoteDesktopClientSettings_Vtbl {
        unsafe extern "system" fn ApplySettings<Impl: IRemoteDesktopClientSettings_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rdpfilecontents: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ApplySettings(::core::mem::transmute_copy(&rdpfilecontents)).into()
        }
        unsafe extern "system" fn RetrieveSettings<Impl: IRemoteDesktopClientSettings_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rdpfilecontents: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RetrieveSettings() {
                ::core::result::Result::Ok(ok__) => {
                    *rdpfilecontents = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetRdpProperty<Impl: IRemoteDesktopClientSettings_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, propertyname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, value: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetRdpProperty(::core::mem::transmute_copy(&propertyname)) {
                ::core::result::Result::Ok(ok__) => {
                    *value = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetRdpProperty<Impl: IRemoteDesktopClientSettings_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, propertyname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, value: ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetRdpProperty(::core::mem::transmute_copy(&propertyname), ::core::mem::transmute_copy(&value)).into()
        }
        Self {
            base: super::Com::IDispatch_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            ApplySettings: ApplySettings::<Impl, IMPL_OFFSET>,
            RetrieveSettings: RetrieveSettings::<Impl, IMPL_OFFSET>,
            GetRdpProperty: GetRdpProperty::<Impl, IMPL_OFFSET>,
            SetRdpProperty: SetRdpProperty::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRemoteDesktopClientSettings as ::windows::core::Interface>::IID || iid == &<super::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IRemoteDesktopClientTouchPointer_Impl: Sized + super::Com::IDispatch_Impl {
    fn SetEnabled(&mut self, enabled: i16) -> ::windows::core::Result<()>;
    fn Enabled(&mut self) -> ::windows::core::Result<i16>;
    fn SetEventsEnabled(&mut self, eventsenabled: i16) -> ::windows::core::Result<()>;
    fn EventsEnabled(&mut self) -> ::windows::core::Result<i16>;
    fn SetPointerSpeed(&mut self, pointerspeed: u32) -> ::windows::core::Result<()>;
    fn PointerSpeed(&mut self) -> ::windows::core::Result<u32>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IRemoteDesktopClientTouchPointer_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRemoteDesktopClientTouchPointer_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IRemoteDesktopClientTouchPointer_Vtbl {
        unsafe extern "system" fn SetEnabled<Impl: IRemoteDesktopClientTouchPointer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, enabled: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetEnabled(::core::mem::transmute_copy(&enabled)).into()
        }
        unsafe extern "system" fn Enabled<Impl: IRemoteDesktopClientTouchPointer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, enabled: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Enabled() {
                ::core::result::Result::Ok(ok__) => {
                    *enabled = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetEventsEnabled<Impl: IRemoteDesktopClientTouchPointer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, eventsenabled: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetEventsEnabled(::core::mem::transmute_copy(&eventsenabled)).into()
        }
        unsafe extern "system" fn EventsEnabled<Impl: IRemoteDesktopClientTouchPointer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, eventsenabled: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EventsEnabled() {
                ::core::result::Result::Ok(ok__) => {
                    *eventsenabled = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPointerSpeed<Impl: IRemoteDesktopClientTouchPointer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pointerspeed: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetPointerSpeed(::core::mem::transmute_copy(&pointerspeed)).into()
        }
        unsafe extern "system" fn PointerSpeed<Impl: IRemoteDesktopClientTouchPointer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pointerspeed: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PointerSpeed() {
                ::core::result::Result::Ok(ok__) => {
                    *pointerspeed = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: super::Com::IDispatch_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            SetEnabled: SetEnabled::<Impl, IMPL_OFFSET>,
            Enabled: Enabled::<Impl, IMPL_OFFSET>,
            SetEventsEnabled: SetEventsEnabled::<Impl, IMPL_OFFSET>,
            EventsEnabled: EventsEnabled::<Impl, IMPL_OFFSET>,
            SetPointerSpeed: SetPointerSpeed::<Impl, IMPL_OFFSET>,
            PointerSpeed: PointerSpeed::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRemoteDesktopClientTouchPointer as ::windows::core::Interface>::IID || iid == &<super::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
pub trait IRemoteSystemAdditionalInfoProvider_Impl: Sized {
    fn GetAdditionalInfo(&mut self, deduplicationid: *mut ::windows::core::HSTRING, riid: *const ::windows::core::GUID, mapview: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()>;
}
impl IRemoteSystemAdditionalInfoProvider_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRemoteSystemAdditionalInfoProvider_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IRemoteSystemAdditionalInfoProvider_Vtbl {
        unsafe extern "system" fn GetAdditionalInfo<Impl: IRemoteSystemAdditionalInfoProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, deduplicationid: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>, riid: *const ::windows::core::GUID, mapview: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetAdditionalInfo(::core::mem::transmute_copy(&deduplicationid), ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&mapview)).into()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), GetAdditionalInfo: GetAdditionalInfo::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRemoteSystemAdditionalInfoProvider as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait ITSGAccountingEngine_Impl: Sized {
    fn DoAccounting(&mut self, accountingdatatype: AAAccountingDataType, accountingdata: &AAAccountingData) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ITSGAccountingEngine_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITSGAccountingEngine_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITSGAccountingEngine_Vtbl {
        unsafe extern "system" fn DoAccounting<Impl: ITSGAccountingEngine_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, accountingdatatype: AAAccountingDataType, accountingdata: ::core::mem::ManuallyDrop<AAAccountingData>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).DoAccounting(::core::mem::transmute_copy(&accountingdatatype), ::core::mem::transmute_copy(&accountingdata)).into()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), DoAccounting: DoAccounting::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITSGAccountingEngine as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait ITSGAuthenticateUserSink_Impl: Sized {
    fn OnUserAuthenticated(&mut self, username: &super::super::Foundation::BSTR, userdomain: &super::super::Foundation::BSTR, context: usize, usertoken: super::super::Foundation::HANDLE_PTR) -> ::windows::core::Result<()>;
    fn OnUserAuthenticationFailed(&mut self, context: usize, genericerrorcode: ::windows::core::HRESULT, specificerrorcode: ::windows::core::HRESULT) -> ::windows::core::Result<()>;
    fn ReauthenticateUser(&mut self, context: usize) -> ::windows::core::Result<()>;
    fn DisconnectUser(&mut self, context: usize) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ITSGAuthenticateUserSink_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITSGAuthenticateUserSink_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITSGAuthenticateUserSink_Vtbl {
        unsafe extern "system" fn OnUserAuthenticated<Impl: ITSGAuthenticateUserSink_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, username: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, userdomain: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, context: usize, usertoken: super::super::Foundation::HANDLE_PTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnUserAuthenticated(::core::mem::transmute_copy(&username), ::core::mem::transmute_copy(&userdomain), ::core::mem::transmute_copy(&context), ::core::mem::transmute_copy(&usertoken)).into()
        }
        unsafe extern "system" fn OnUserAuthenticationFailed<Impl: ITSGAuthenticateUserSink_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, context: usize, genericerrorcode: ::windows::core::HRESULT, specificerrorcode: ::windows::core::HRESULT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnUserAuthenticationFailed(::core::mem::transmute_copy(&context), ::core::mem::transmute_copy(&genericerrorcode), ::core::mem::transmute_copy(&specificerrorcode)).into()
        }
        unsafe extern "system" fn ReauthenticateUser<Impl: ITSGAuthenticateUserSink_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, context: usize) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ReauthenticateUser(::core::mem::transmute_copy(&context)).into()
        }
        unsafe extern "system" fn DisconnectUser<Impl: ITSGAuthenticateUserSink_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, context: usize) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).DisconnectUser(::core::mem::transmute_copy(&context)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            OnUserAuthenticated: OnUserAuthenticated::<Impl, IMPL_OFFSET>,
            OnUserAuthenticationFailed: OnUserAuthenticationFailed::<Impl, IMPL_OFFSET>,
            ReauthenticateUser: ReauthenticateUser::<Impl, IMPL_OFFSET>,
            DisconnectUser: DisconnectUser::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITSGAuthenticateUserSink as ::windows::core::Interface>::IID
    }
}
pub trait ITSGAuthenticationEngine_Impl: Sized {
    fn AuthenticateUser(&mut self, mainsessionid: &::windows::core::GUID, cookiedata: *const u8, numcookiebytes: u32, context: usize, psink: &::core::option::Option<ITSGAuthenticateUserSink>) -> ::windows::core::Result<()>;
    fn CancelAuthentication(&mut self, mainsessionid: &::windows::core::GUID, context: usize) -> ::windows::core::Result<()>;
}
impl ITSGAuthenticationEngine_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITSGAuthenticationEngine_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITSGAuthenticationEngine_Vtbl {
        unsafe extern "system" fn AuthenticateUser<Impl: ITSGAuthenticationEngine_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, mainsessionid: ::windows::core::GUID, cookiedata: *const u8, numcookiebytes: u32, context: usize, psink: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AuthenticateUser(::core::mem::transmute_copy(&mainsessionid), ::core::mem::transmute_copy(&cookiedata), ::core::mem::transmute_copy(&numcookiebytes), ::core::mem::transmute_copy(&context), ::core::mem::transmute(&psink)).into()
        }
        unsafe extern "system" fn CancelAuthentication<Impl: ITSGAuthenticationEngine_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, mainsessionid: ::windows::core::GUID, context: usize) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).CancelAuthentication(::core::mem::transmute_copy(&mainsessionid), ::core::mem::transmute_copy(&context)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            AuthenticateUser: AuthenticateUser::<Impl, IMPL_OFFSET>,
            CancelAuthentication: CancelAuthentication::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITSGAuthenticationEngine as ::windows::core::Interface>::IID
    }
}
pub trait ITSGAuthorizeConnectionSink_Impl: Sized {
    fn OnConnectionAuthorized(&mut self, hrin: ::windows::core::HRESULT, mainsessionid: &::windows::core::GUID, cbsohresponse: u32, pbsohresponse: *const u8, idletimeout: u32, sessiontimeout: u32, sessiontimeoutaction: SESSION_TIMEOUT_ACTION_TYPE, trustclass: AATrustClassID, policyattributes: *const u32) -> ::windows::core::Result<()>;
}
impl ITSGAuthorizeConnectionSink_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITSGAuthorizeConnectionSink_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITSGAuthorizeConnectionSink_Vtbl {
        unsafe extern "system" fn OnConnectionAuthorized<Impl: ITSGAuthorizeConnectionSink_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hrin: ::windows::core::HRESULT, mainsessionid: ::windows::core::GUID, cbsohresponse: u32, pbsohresponse: *const u8, idletimeout: u32, sessiontimeout: u32, sessiontimeoutaction: SESSION_TIMEOUT_ACTION_TYPE, trustclass: AATrustClassID, policyattributes: *const u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnConnectionAuthorized(::core::mem::transmute_copy(&hrin), ::core::mem::transmute_copy(&mainsessionid), ::core::mem::transmute_copy(&cbsohresponse), ::core::mem::transmute_copy(&pbsohresponse), ::core::mem::transmute_copy(&idletimeout), ::core::mem::transmute_copy(&sessiontimeout), ::core::mem::transmute_copy(&sessiontimeoutaction), ::core::mem::transmute_copy(&trustclass), ::core::mem::transmute_copy(&policyattributes)).into()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), OnConnectionAuthorized: OnConnectionAuthorized::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITSGAuthorizeConnectionSink as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait ITSGAuthorizeResourceSink_Impl: Sized {
    fn OnChannelAuthorized(&mut self, hrin: ::windows::core::HRESULT, mainsessionid: &::windows::core::GUID, subsessionid: i32, allowedresourcenames: *const super::super::Foundation::BSTR, numallowedresourcenames: u32, failedresourcenames: *const super::super::Foundation::BSTR, numfailedresourcenames: u32) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ITSGAuthorizeResourceSink_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITSGAuthorizeResourceSink_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITSGAuthorizeResourceSink_Vtbl {
        unsafe extern "system" fn OnChannelAuthorized<Impl: ITSGAuthorizeResourceSink_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hrin: ::windows::core::HRESULT, mainsessionid: ::windows::core::GUID, subsessionid: i32, allowedresourcenames: *const super::super::Foundation::BSTR, numallowedresourcenames: u32, failedresourcenames: *const super::super::Foundation::BSTR, numfailedresourcenames: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnChannelAuthorized(::core::mem::transmute_copy(&hrin), ::core::mem::transmute_copy(&mainsessionid), ::core::mem::transmute_copy(&subsessionid), ::core::mem::transmute_copy(&allowedresourcenames), ::core::mem::transmute_copy(&numallowedresourcenames), ::core::mem::transmute_copy(&failedresourcenames), ::core::mem::transmute_copy(&numfailedresourcenames)).into()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), OnChannelAuthorized: OnChannelAuthorized::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITSGAuthorizeResourceSink as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait ITSGPolicyEngine_Impl: Sized {
    fn AuthorizeConnection(&mut self, mainsessionid: &::windows::core::GUID, username: &super::super::Foundation::BSTR, authtype: AAAuthSchemes, clientmachineip: &super::super::Foundation::BSTR, clientmachinename: &super::super::Foundation::BSTR, sohdata: *const u8, numsohbytes: u32, cookiedata: *const u8, numcookiebytes: u32, usertoken: super::super::Foundation::HANDLE_PTR, psink: &::core::option::Option<ITSGAuthorizeConnectionSink>) -> ::windows::core::Result<()>;
    fn AuthorizeResource(&mut self, mainsessionid: &::windows::core::GUID, subsessionid: i32, username: &super::super::Foundation::BSTR, resourcenames: *const super::super::Foundation::BSTR, numresources: u32, alternateresourcenames: *const super::super::Foundation::BSTR, numalternateresourcename: u32, portnumber: u32, operation: &super::super::Foundation::BSTR, cookie: *const u8, numbytesincookie: u32, psink: &::core::option::Option<ITSGAuthorizeResourceSink>) -> ::windows::core::Result<()>;
    fn Refresh(&mut self) -> ::windows::core::Result<()>;
    fn IsQuarantineEnabled(&mut self) -> ::windows::core::Result<super::super::Foundation::BOOL>;
}
#[cfg(feature = "Win32_Foundation")]
impl ITSGPolicyEngine_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITSGPolicyEngine_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITSGPolicyEngine_Vtbl {
        unsafe extern "system" fn AuthorizeConnection<Impl: ITSGPolicyEngine_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, mainsessionid: ::windows::core::GUID, username: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, authtype: AAAuthSchemes, clientmachineip: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, clientmachinename: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, sohdata: *const u8, numsohbytes: u32, cookiedata: *const u8, numcookiebytes: u32, usertoken: super::super::Foundation::HANDLE_PTR, psink: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this)
                .AuthorizeConnection(::core::mem::transmute_copy(&mainsessionid), ::core::mem::transmute_copy(&username), ::core::mem::transmute_copy(&authtype), ::core::mem::transmute_copy(&clientmachineip), ::core::mem::transmute_copy(&clientmachinename), ::core::mem::transmute_copy(&sohdata), ::core::mem::transmute_copy(&numsohbytes), ::core::mem::transmute_copy(&cookiedata), ::core::mem::transmute_copy(&numcookiebytes), ::core::mem::transmute_copy(&usertoken), ::core::mem::transmute(&psink))
                .into()
        }
        unsafe extern "system" fn AuthorizeResource<Impl: ITSGPolicyEngine_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, mainsessionid: ::windows::core::GUID, subsessionid: i32, username: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, resourcenames: *const super::super::Foundation::BSTR, numresources: u32, alternateresourcenames: *const super::super::Foundation::BSTR, numalternateresourcename: u32, portnumber: u32, operation: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, cookie: *const u8, numbytesincookie: u32, psink: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this)
                .AuthorizeResource(
                    ::core::mem::transmute_copy(&mainsessionid),
                    ::core::mem::transmute_copy(&subsessionid),
                    ::core::mem::transmute_copy(&username),
                    ::core::mem::transmute_copy(&resourcenames),
                    ::core::mem::transmute_copy(&numresources),
                    ::core::mem::transmute_copy(&alternateresourcenames),
                    ::core::mem::transmute_copy(&numalternateresourcename),
                    ::core::mem::transmute_copy(&portnumber),
                    ::core::mem::transmute_copy(&operation),
                    ::core::mem::transmute_copy(&cookie),
                    ::core::mem::transmute_copy(&numbytesincookie),
                    ::core::mem::transmute(&psink),
                )
                .into()
        }
        unsafe extern "system" fn Refresh<Impl: ITSGPolicyEngine_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Refresh().into()
        }
        unsafe extern "system" fn IsQuarantineEnabled<Impl: ITSGPolicyEngine_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, quarantineenabled: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsQuarantineEnabled() {
                ::core::result::Result::Ok(ok__) => {
                    *quarantineenabled = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            AuthorizeConnection: AuthorizeConnection::<Impl, IMPL_OFFSET>,
            AuthorizeResource: AuthorizeResource::<Impl, IMPL_OFFSET>,
            Refresh: Refresh::<Impl, IMPL_OFFSET>,
            IsQuarantineEnabled: IsQuarantineEnabled::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITSGPolicyEngine as ::windows::core::Interface>::IID
    }
}
pub trait ITsSbBaseNotifySink_Impl: Sized {
    fn OnError(&mut self, hrerror: ::windows::core::HRESULT) -> ::windows::core::Result<()>;
    fn OnReportStatus(&mut self, messagetype: CLIENT_MESSAGE_TYPE, messageid: u32) -> ::windows::core::Result<()>;
}
impl ITsSbBaseNotifySink_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITsSbBaseNotifySink_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITsSbBaseNotifySink_Vtbl {
        unsafe extern "system" fn OnError<Impl: ITsSbBaseNotifySink_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hrerror: ::windows::core::HRESULT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnError(::core::mem::transmute_copy(&hrerror)).into()
        }
        unsafe extern "system" fn OnReportStatus<Impl: ITsSbBaseNotifySink_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, messagetype: CLIENT_MESSAGE_TYPE, messageid: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnReportStatus(::core::mem::transmute_copy(&messagetype), ::core::mem::transmute_copy(&messageid)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            OnError: OnError::<Impl, IMPL_OFFSET>,
            OnReportStatus: OnReportStatus::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITsSbBaseNotifySink as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Ole"))]
pub trait ITsSbClientConnection_Impl: Sized {
    fn UserName(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn Domain(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn InitialProgram(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn LoadBalanceResult(&mut self) -> ::windows::core::Result<ITsSbLoadBalanceResult>;
    fn FarmName(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn PutContext(&mut self, contextid: &super::super::Foundation::BSTR, context: &super::Com::VARIANT) -> ::windows::core::Result<super::Com::VARIANT>;
    fn GetContext(&mut self, contextid: &super::super::Foundation::BSTR) -> ::windows::core::Result<super::Com::VARIANT>;
    fn Environment(&mut self) -> ::windows::core::Result<ITsSbEnvironment>;
    fn ConnectionError(&mut self) -> ::windows::core::Result<()>;
    fn SamUserAccount(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn ClientConnectionPropertySet(&mut self) -> ::windows::core::Result<ITsSbClientConnectionPropertySet>;
    fn IsFirstAssignment(&mut self) -> ::windows::core::Result<super::super::Foundation::BOOL>;
    fn RdFarmType(&mut self) -> ::windows::core::Result<RD_FARM_TYPE>;
    fn UserSidString(&mut self) -> ::windows::core::Result<*mut i8>;
    fn GetDisconnectedSession(&mut self) -> ::windows::core::Result<ITsSbSession>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Ole"))]
impl ITsSbClientConnection_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITsSbClientConnection_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITsSbClientConnection_Vtbl {
        unsafe extern "system" fn UserName<Impl: ITsSbClientConnection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).UserName() {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Domain<Impl: ITsSbClientConnection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Domain() {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn InitialProgram<Impl: ITsSbClientConnection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).InitialProgram() {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LoadBalanceResult<Impl: ITsSbClientConnection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LoadBalanceResult() {
                ::core::result::Result::Ok(ok__) => {
                    *ppval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FarmName<Impl: ITsSbClientConnection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FarmName() {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PutContext<Impl: ITsSbClientConnection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, contextid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, context: ::core::mem::ManuallyDrop<super::Com::VARIANT>, existingcontext: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PutContext(::core::mem::transmute_copy(&contextid), ::core::mem::transmute_copy(&context)) {
                ::core::result::Result::Ok(ok__) => {
                    *existingcontext = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetContext<Impl: ITsSbClientConnection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, contextid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, context: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetContext(::core::mem::transmute_copy(&contextid)) {
                ::core::result::Result::Ok(ok__) => {
                    *context = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Environment<Impl: ITsSbClientConnection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenvironment: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Environment() {
                ::core::result::Result::Ok(ok__) => {
                    *ppenvironment = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ConnectionError<Impl: ITsSbClientConnection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ConnectionError().into()
        }
        unsafe extern "system" fn SamUserAccount<Impl: ITsSbClientConnection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SamUserAccount() {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ClientConnectionPropertySet<Impl: ITsSbClientConnection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pppropertyset: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ClientConnectionPropertySet() {
                ::core::result::Result::Ok(ok__) => {
                    *pppropertyset = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsFirstAssignment<Impl: ITsSbClientConnection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppval: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsFirstAssignment() {
                ::core::result::Result::Ok(ok__) => {
                    *ppval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RdFarmType<Impl: ITsSbClientConnection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, prdfarmtype: *mut RD_FARM_TYPE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RdFarmType() {
                ::core::result::Result::Ok(ok__) => {
                    *prdfarmtype = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UserSidString<Impl: ITsSbClientConnection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszusersidstring: *mut *mut i8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).UserSidString() {
                ::core::result::Result::Ok(ok__) => {
                    *pszusersidstring = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDisconnectedSession<Impl: ITsSbClientConnection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppsession: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDisconnectedSession() {
                ::core::result::Result::Ok(ok__) => {
                    *ppsession = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            UserName: UserName::<Impl, IMPL_OFFSET>,
            Domain: Domain::<Impl, IMPL_OFFSET>,
            InitialProgram: InitialProgram::<Impl, IMPL_OFFSET>,
            LoadBalanceResult: LoadBalanceResult::<Impl, IMPL_OFFSET>,
            FarmName: FarmName::<Impl, IMPL_OFFSET>,
            PutContext: PutContext::<Impl, IMPL_OFFSET>,
            GetContext: GetContext::<Impl, IMPL_OFFSET>,
            Environment: Environment::<Impl, IMPL_OFFSET>,
            ConnectionError: ConnectionError::<Impl, IMPL_OFFSET>,
            SamUserAccount: SamUserAccount::<Impl, IMPL_OFFSET>,
            ClientConnectionPropertySet: ClientConnectionPropertySet::<Impl, IMPL_OFFSET>,
            IsFirstAssignment: IsFirstAssignment::<Impl, IMPL_OFFSET>,
            RdFarmType: RdFarmType::<Impl, IMPL_OFFSET>,
            UserSidString: UserSidString::<Impl, IMPL_OFFSET>,
            GetDisconnectedSession: GetDisconnectedSession::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITsSbClientConnection as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Ole"))]
pub trait ITsSbClientConnectionPropertySet_Impl: Sized + super::Com::StructuredStorage::IPropertyBag_Impl + ITsSbPropertySet_Impl {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Ole"))]
impl ITsSbClientConnectionPropertySet_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITsSbClientConnectionPropertySet_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITsSbClientConnectionPropertySet_Vtbl {
        Self { base: ITsSbPropertySet_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITsSbClientConnectionPropertySet as ::windows::core::Interface>::IID || iid == &<super::Com::StructuredStorage::IPropertyBag as ::windows::core::Interface>::IID || iid == &<ITsSbPropertySet as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
pub trait ITsSbEnvironment_Impl: Sized {
    fn Name(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn ServerWeight(&mut self) -> ::windows::core::Result<u32>;
    fn EnvironmentPropertySet(&mut self) -> ::windows::core::Result<ITsSbEnvironmentPropertySet>;
    fn SetEnvironmentPropertySet(&mut self, pval: &::core::option::Option<ITsSbEnvironmentPropertySet>) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
impl ITsSbEnvironment_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITsSbEnvironment_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITsSbEnvironment_Vtbl {
        unsafe extern "system" fn Name<Impl: ITsSbEnvironment_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Name() {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ServerWeight<Impl: ITsSbEnvironment_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ServerWeight() {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnvironmentPropertySet<Impl: ITsSbEnvironment_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pppropertyset: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EnvironmentPropertySet() {
                ::core::result::Result::Ok(ok__) => {
                    *pppropertyset = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetEnvironmentPropertySet<Impl: ITsSbEnvironment_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetEnvironmentPropertySet(::core::mem::transmute(&pval)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            Name: Name::<Impl, IMPL_OFFSET>,
            ServerWeight: ServerWeight::<Impl, IMPL_OFFSET>,
            EnvironmentPropertySet: EnvironmentPropertySet::<Impl, IMPL_OFFSET>,
            SetEnvironmentPropertySet: SetEnvironmentPropertySet::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITsSbEnvironment as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Ole"))]
pub trait ITsSbEnvironmentPropertySet_Impl: Sized + super::Com::StructuredStorage::IPropertyBag_Impl + ITsSbPropertySet_Impl {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Ole"))]
impl ITsSbEnvironmentPropertySet_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITsSbEnvironmentPropertySet_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITsSbEnvironmentPropertySet_Vtbl {
        Self { base: ITsSbPropertySet_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITsSbEnvironmentPropertySet as ::windows::core::Interface>::IID || iid == &<super::Com::StructuredStorage::IPropertyBag as ::windows::core::Interface>::IID || iid == &<ITsSbPropertySet as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
pub trait ITsSbFilterPluginStore_Impl: Sized {
    fn SaveProperties(&mut self, ppropertyset: &::core::option::Option<ITsSbPropertySet>) -> ::windows::core::Result<()>;
    fn EnumerateProperties(&mut self) -> ::windows::core::Result<ITsSbPropertySet>;
    fn DeleteProperties(&mut self, propertyname: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
impl ITsSbFilterPluginStore_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITsSbFilterPluginStore_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITsSbFilterPluginStore_Vtbl {
        unsafe extern "system" fn SaveProperties<Impl: ITsSbFilterPluginStore_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppropertyset: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SaveProperties(::core::mem::transmute(&ppropertyset)).into()
        }
        unsafe extern "system" fn EnumerateProperties<Impl: ITsSbFilterPluginStore_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pppropertyset: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EnumerateProperties() {
                ::core::result::Result::Ok(ok__) => {
                    *pppropertyset = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DeleteProperties<Impl: ITsSbFilterPluginStore_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, propertyname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).DeleteProperties(::core::mem::transmute_copy(&propertyname)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            SaveProperties: SaveProperties::<Impl, IMPL_OFFSET>,
            EnumerateProperties: EnumerateProperties::<Impl, IMPL_OFFSET>,
            DeleteProperties: DeleteProperties::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITsSbFilterPluginStore as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait ITsSbGenericNotifySink_Impl: Sized {
    fn OnCompleted(&mut self, status: ::windows::core::HRESULT) -> ::windows::core::Result<()>;
    fn GetWaitTimeout(&mut self) -> ::windows::core::Result<super::super::Foundation::FILETIME>;
}
#[cfg(feature = "Win32_Foundation")]
impl ITsSbGenericNotifySink_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITsSbGenericNotifySink_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITsSbGenericNotifySink_Vtbl {
        unsafe extern "system" fn OnCompleted<Impl: ITsSbGenericNotifySink_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, status: ::windows::core::HRESULT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnCompleted(::core::mem::transmute_copy(&status)).into()
        }
        unsafe extern "system" fn GetWaitTimeout<Impl: ITsSbGenericNotifySink_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfttimeout: *mut super::super::Foundation::FILETIME) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetWaitTimeout() {
                ::core::result::Result::Ok(ok__) => {
                    *pfttimeout = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            OnCompleted: OnCompleted::<Impl, IMPL_OFFSET>,
            GetWaitTimeout: GetWaitTimeout::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITsSbGenericNotifySink as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ITsSbGlobalStore_Impl: Sized {
    fn QueryTarget(&mut self, providername: &super::super::Foundation::BSTR, targetname: &super::super::Foundation::BSTR, farmname: &super::super::Foundation::BSTR) -> ::windows::core::Result<ITsSbTarget>;
    fn QuerySessionBySessionId(&mut self, providername: &super::super::Foundation::BSTR, dwsessionid: u32, targetname: &super::super::Foundation::BSTR) -> ::windows::core::Result<ITsSbSession>;
    fn EnumerateFarms(&mut self, providername: &super::super::Foundation::BSTR, pdwcount: *mut u32, pval: *mut *mut super::Com::SAFEARRAY) -> ::windows::core::Result<()>;
    fn EnumerateTargets(&mut self, providername: &super::super::Foundation::BSTR, farmname: &super::super::Foundation::BSTR, envname: &super::super::Foundation::BSTR, pdwcount: *mut u32, pval: *mut *mut ::core::option::Option<ITsSbTarget>) -> ::windows::core::Result<()>;
    fn EnumerateEnvironmentsByProvider(&mut self, providername: &super::super::Foundation::BSTR, pdwcount: *mut u32, ppval: *mut *mut ::core::option::Option<ITsSbEnvironment>) -> ::windows::core::Result<()>;
    fn EnumerateSessions(&mut self, providername: &super::super::Foundation::BSTR, targetname: &super::super::Foundation::BSTR, username: &super::super::Foundation::BSTR, userdomain: &super::super::Foundation::BSTR, poolname: &super::super::Foundation::BSTR, initialprogram: &super::super::Foundation::BSTR, psessionstate: *const TSSESSION_STATE, pdwcount: *mut u32, ppval: *mut *mut ::core::option::Option<ITsSbSession>) -> ::windows::core::Result<()>;
    fn GetFarmProperty(&mut self, farmname: &super::super::Foundation::BSTR, propertyname: &super::super::Foundation::BSTR, pvarvalue: *const super::Com::VARIANT) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ITsSbGlobalStore_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITsSbGlobalStore_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITsSbGlobalStore_Vtbl {
        unsafe extern "system" fn QueryTarget<Impl: ITsSbGlobalStore_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, providername: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, targetname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, farmname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pptarget: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).QueryTarget(::core::mem::transmute_copy(&providername), ::core::mem::transmute_copy(&targetname), ::core::mem::transmute_copy(&farmname)) {
                ::core::result::Result::Ok(ok__) => {
                    *pptarget = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn QuerySessionBySessionId<Impl: ITsSbGlobalStore_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, providername: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, dwsessionid: u32, targetname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ppsession: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).QuerySessionBySessionId(::core::mem::transmute_copy(&providername), ::core::mem::transmute_copy(&dwsessionid), ::core::mem::transmute_copy(&targetname)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppsession = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnumerateFarms<Impl: ITsSbGlobalStore_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, providername: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pdwcount: *mut u32, pval: *mut *mut super::Com::SAFEARRAY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).EnumerateFarms(::core::mem::transmute_copy(&providername), ::core::mem::transmute_copy(&pdwcount), ::core::mem::transmute_copy(&pval)).into()
        }
        unsafe extern "system" fn EnumerateTargets<Impl: ITsSbGlobalStore_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, providername: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, farmname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, envname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pdwcount: *mut u32, pval: *mut *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).EnumerateTargets(::core::mem::transmute_copy(&providername), ::core::mem::transmute_copy(&farmname), ::core::mem::transmute_copy(&envname), ::core::mem::transmute_copy(&pdwcount), ::core::mem::transmute_copy(&pval)).into()
        }
        unsafe extern "system" fn EnumerateEnvironmentsByProvider<Impl: ITsSbGlobalStore_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, providername: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pdwcount: *mut u32, ppval: *mut *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).EnumerateEnvironmentsByProvider(::core::mem::transmute_copy(&providername), ::core::mem::transmute_copy(&pdwcount), ::core::mem::transmute_copy(&ppval)).into()
        }
        unsafe extern "system" fn EnumerateSessions<Impl: ITsSbGlobalStore_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, providername: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, targetname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, username: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, userdomain: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, poolname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, initialprogram: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, psessionstate: *const TSSESSION_STATE, pdwcount: *mut u32, ppval: *mut *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).EnumerateSessions(::core::mem::transmute_copy(&providername), ::core::mem::transmute_copy(&targetname), ::core::mem::transmute_copy(&username), ::core::mem::transmute_copy(&userdomain), ::core::mem::transmute_copy(&poolname), ::core::mem::transmute_copy(&initialprogram), ::core::mem::transmute_copy(&psessionstate), ::core::mem::transmute_copy(&pdwcount), ::core::mem::transmute_copy(&ppval)).into()
        }
        unsafe extern "system" fn GetFarmProperty<Impl: ITsSbGlobalStore_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, farmname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, propertyname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pvarvalue: *const super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetFarmProperty(::core::mem::transmute_copy(&farmname), ::core::mem::transmute_copy(&propertyname), ::core::mem::transmute_copy(&pvarvalue)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            QueryTarget: QueryTarget::<Impl, IMPL_OFFSET>,
            QuerySessionBySessionId: QuerySessionBySessionId::<Impl, IMPL_OFFSET>,
            EnumerateFarms: EnumerateFarms::<Impl, IMPL_OFFSET>,
            EnumerateTargets: EnumerateTargets::<Impl, IMPL_OFFSET>,
            EnumerateEnvironmentsByProvider: EnumerateEnvironmentsByProvider::<Impl, IMPL_OFFSET>,
            EnumerateSessions: EnumerateSessions::<Impl, IMPL_OFFSET>,
            GetFarmProperty: GetFarmProperty::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITsSbGlobalStore as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait ITsSbLoadBalanceResult_Impl: Sized {
    fn TargetName(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
}
#[cfg(feature = "Win32_Foundation")]
impl ITsSbLoadBalanceResult_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITsSbLoadBalanceResult_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITsSbLoadBalanceResult_Vtbl {
        unsafe extern "system" fn TargetName<Impl: ITsSbLoadBalanceResult_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TargetName() {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), TargetName: TargetName::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITsSbLoadBalanceResult as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com_StructuredStorage")]
pub trait ITsSbLoadBalancing_Impl: Sized + ITsSbPlugin_Impl {
    fn GetMostSuitableTarget(&mut self, pconnection: &::core::option::Option<ITsSbClientConnection>, plbsink: &::core::option::Option<ITsSbLoadBalancingNotifySink>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_System_Com_StructuredStorage")]
impl ITsSbLoadBalancing_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITsSbLoadBalancing_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITsSbLoadBalancing_Vtbl {
        unsafe extern "system" fn GetMostSuitableTarget<Impl: ITsSbLoadBalancing_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pconnection: ::windows::core::RawPtr, plbsink: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetMostSuitableTarget(::core::mem::transmute(&pconnection), ::core::mem::transmute(&plbsink)).into()
        }
        Self { base: ITsSbPlugin_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(), GetMostSuitableTarget: GetMostSuitableTarget::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITsSbLoadBalancing as ::windows::core::Interface>::IID || iid == &<ITsSbPlugin as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait ITsSbLoadBalancingNotifySink_Impl: Sized + ITsSbBaseNotifySink_Impl {
    fn OnGetMostSuitableTarget(&mut self, plbresult: &::core::option::Option<ITsSbLoadBalanceResult>, fisnewconnection: super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ITsSbLoadBalancingNotifySink_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITsSbLoadBalancingNotifySink_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITsSbLoadBalancingNotifySink_Vtbl {
        unsafe extern "system" fn OnGetMostSuitableTarget<Impl: ITsSbLoadBalancingNotifySink_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plbresult: ::windows::core::RawPtr, fisnewconnection: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnGetMostSuitableTarget(::core::mem::transmute(&plbresult), ::core::mem::transmute_copy(&fisnewconnection)).into()
        }
        Self {
            base: ITsSbBaseNotifySink_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            OnGetMostSuitableTarget: OnGetMostSuitableTarget::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITsSbLoadBalancingNotifySink as ::windows::core::Interface>::IID || iid == &<ITsSbBaseNotifySink as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com_StructuredStorage")]
pub trait ITsSbOrchestration_Impl: Sized + ITsSbPlugin_Impl {
    fn PrepareTargetForConnect(&mut self, pconnection: &::core::option::Option<ITsSbClientConnection>, porchestrationnotifysink: &::core::option::Option<ITsSbOrchestrationNotifySink>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_System_Com_StructuredStorage")]
impl ITsSbOrchestration_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITsSbOrchestration_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITsSbOrchestration_Vtbl {
        unsafe extern "system" fn PrepareTargetForConnect<Impl: ITsSbOrchestration_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pconnection: ::windows::core::RawPtr, porchestrationnotifysink: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).PrepareTargetForConnect(::core::mem::transmute(&pconnection), ::core::mem::transmute(&porchestrationnotifysink)).into()
        }
        Self { base: ITsSbPlugin_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(), PrepareTargetForConnect: PrepareTargetForConnect::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITsSbOrchestration as ::windows::core::Interface>::IID || iid == &<ITsSbPlugin as ::windows::core::Interface>::IID
    }
}
pub trait ITsSbOrchestrationNotifySink_Impl: Sized + ITsSbBaseNotifySink_Impl {
    fn OnReadyToConnect(&mut self, ptarget: &::core::option::Option<ITsSbTarget>) -> ::windows::core::Result<()>;
}
impl ITsSbOrchestrationNotifySink_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITsSbOrchestrationNotifySink_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITsSbOrchestrationNotifySink_Vtbl {
        unsafe extern "system" fn OnReadyToConnect<Impl: ITsSbOrchestrationNotifySink_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ptarget: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnReadyToConnect(::core::mem::transmute(&ptarget)).into()
        }
        Self { base: ITsSbBaseNotifySink_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(), OnReadyToConnect: OnReadyToConnect::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITsSbOrchestrationNotifySink as ::windows::core::Interface>::IID || iid == &<ITsSbBaseNotifySink as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com_StructuredStorage")]
pub trait ITsSbPlacement_Impl: Sized + ITsSbPlugin_Impl {
    fn QueryEnvironmentForTarget(&mut self, pconnection: &::core::option::Option<ITsSbClientConnection>, pplacementsink: &::core::option::Option<ITsSbPlacementNotifySink>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_System_Com_StructuredStorage")]
impl ITsSbPlacement_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITsSbPlacement_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITsSbPlacement_Vtbl {
        unsafe extern "system" fn QueryEnvironmentForTarget<Impl: ITsSbPlacement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pconnection: ::windows::core::RawPtr, pplacementsink: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).QueryEnvironmentForTarget(::core::mem::transmute(&pconnection), ::core::mem::transmute(&pplacementsink)).into()
        }
        Self {
            base: ITsSbPlugin_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            QueryEnvironmentForTarget: QueryEnvironmentForTarget::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITsSbPlacement as ::windows::core::Interface>::IID || iid == &<ITsSbPlugin as ::windows::core::Interface>::IID
    }
}
pub trait ITsSbPlacementNotifySink_Impl: Sized + ITsSbBaseNotifySink_Impl {
    fn OnQueryEnvironmentCompleted(&mut self, penvironment: &::core::option::Option<ITsSbEnvironment>) -> ::windows::core::Result<()>;
}
impl ITsSbPlacementNotifySink_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITsSbPlacementNotifySink_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITsSbPlacementNotifySink_Vtbl {
        unsafe extern "system" fn OnQueryEnvironmentCompleted<Impl: ITsSbPlacementNotifySink_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, penvironment: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnQueryEnvironmentCompleted(::core::mem::transmute(&penvironment)).into()
        }
        Self {
            base: ITsSbBaseNotifySink_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            OnQueryEnvironmentCompleted: OnQueryEnvironmentCompleted::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITsSbPlacementNotifySink as ::windows::core::Interface>::IID || iid == &<ITsSbBaseNotifySink as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com_StructuredStorage")]
pub trait ITsSbPlugin_Impl: Sized {
    fn Initialize(&mut self, pprovider: &::core::option::Option<ITsSbProvider>, pnotifysink: &::core::option::Option<ITsSbPluginNotifySink>, ppropertyset: &::core::option::Option<ITsSbPluginPropertySet>) -> ::windows::core::Result<()>;
    fn Terminate(&mut self, hr: ::windows::core::HRESULT) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_System_Com_StructuredStorage")]
impl ITsSbPlugin_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITsSbPlugin_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITsSbPlugin_Vtbl {
        unsafe extern "system" fn Initialize<Impl: ITsSbPlugin_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pprovider: ::windows::core::RawPtr, pnotifysink: ::windows::core::RawPtr, ppropertyset: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Initialize(::core::mem::transmute(&pprovider), ::core::mem::transmute(&pnotifysink), ::core::mem::transmute(&ppropertyset)).into()
        }
        unsafe extern "system" fn Terminate<Impl: ITsSbPlugin_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hr: ::windows::core::HRESULT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Terminate(::core::mem::transmute_copy(&hr)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            Initialize: Initialize::<Impl, IMPL_OFFSET>,
            Terminate: Terminate::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITsSbPlugin as ::windows::core::Interface>::IID
    }
}
pub trait ITsSbPluginNotifySink_Impl: Sized + ITsSbBaseNotifySink_Impl {
    fn OnInitialized(&mut self, hr: ::windows::core::HRESULT) -> ::windows::core::Result<()>;
    fn OnTerminated(&mut self) -> ::windows::core::Result<()>;
}
impl ITsSbPluginNotifySink_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITsSbPluginNotifySink_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITsSbPluginNotifySink_Vtbl {
        unsafe extern "system" fn OnInitialized<Impl: ITsSbPluginNotifySink_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hr: ::windows::core::HRESULT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnInitialized(::core::mem::transmute_copy(&hr)).into()
        }
        unsafe extern "system" fn OnTerminated<Impl: ITsSbPluginNotifySink_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnTerminated().into()
        }
        Self {
            base: ITsSbBaseNotifySink_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            OnInitialized: OnInitialized::<Impl, IMPL_OFFSET>,
            OnTerminated: OnTerminated::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITsSbPluginNotifySink as ::windows::core::Interface>::IID || iid == &<ITsSbBaseNotifySink as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Ole"))]
pub trait ITsSbPluginPropertySet_Impl: Sized + super::Com::StructuredStorage::IPropertyBag_Impl + ITsSbPropertySet_Impl {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Ole"))]
impl ITsSbPluginPropertySet_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITsSbPluginPropertySet_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITsSbPluginPropertySet_Vtbl {
        Self { base: ITsSbPropertySet_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITsSbPluginPropertySet as ::windows::core::Interface>::IID || iid == &<super::Com::StructuredStorage::IPropertyBag as ::windows::core::Interface>::IID || iid == &<ITsSbPropertySet as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Ole"))]
pub trait ITsSbPropertySet_Impl: Sized + super::Com::StructuredStorage::IPropertyBag_Impl {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Ole"))]
impl ITsSbPropertySet_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITsSbPropertySet_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITsSbPropertySet_Vtbl {
        Self { base: super::Com::StructuredStorage::IPropertyBag_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITsSbPropertySet as ::windows::core::Interface>::IID || iid == &<super::Com::StructuredStorage::IPropertyBag as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
pub trait ITsSbProvider_Impl: Sized {
    fn CreateTargetObject(&mut self, targetname: &super::super::Foundation::BSTR, environmentname: &super::super::Foundation::BSTR) -> ::windows::core::Result<ITsSbTarget>;
    fn CreateLoadBalanceResultObject(&mut self, targetname: &super::super::Foundation::BSTR) -> ::windows::core::Result<ITsSbLoadBalanceResult>;
    fn CreateSessionObject(&mut self, targetname: &super::super::Foundation::BSTR, username: &super::super::Foundation::BSTR, domain: &super::super::Foundation::BSTR, sessionid: u32) -> ::windows::core::Result<ITsSbSession>;
    fn CreatePluginPropertySet(&mut self) -> ::windows::core::Result<ITsSbPluginPropertySet>;
    fn CreateTargetPropertySetObject(&mut self) -> ::windows::core::Result<ITsSbTargetPropertySet>;
    fn CreateEnvironmentObject(&mut self, name: &super::super::Foundation::BSTR, serverweight: u32) -> ::windows::core::Result<ITsSbEnvironment>;
    fn GetResourcePluginStore(&mut self) -> ::windows::core::Result<ITsSbResourcePluginStore>;
    fn GetFilterPluginStore(&mut self) -> ::windows::core::Result<ITsSbFilterPluginStore>;
    fn RegisterForNotification(&mut self, notificationtype: u32, resourcetomonitor: &super::super::Foundation::BSTR, ppluginnotification: &::core::option::Option<ITsSbResourceNotification>) -> ::windows::core::Result<()>;
    fn UnRegisterForNotification(&mut self, notificationtype: u32, resourcetomonitor: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn GetInstanceOfGlobalStore(&mut self) -> ::windows::core::Result<ITsSbGlobalStore>;
    fn CreateEnvironmentPropertySetObject(&mut self) -> ::windows::core::Result<ITsSbEnvironmentPropertySet>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
impl ITsSbProvider_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITsSbProvider_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITsSbProvider_Vtbl {
        unsafe extern "system" fn CreateTargetObject<Impl: ITsSbProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, targetname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, environmentname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pptarget: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateTargetObject(::core::mem::transmute_copy(&targetname), ::core::mem::transmute_copy(&environmentname)) {
                ::core::result::Result::Ok(ok__) => {
                    *pptarget = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateLoadBalanceResultObject<Impl: ITsSbProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, targetname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pplbresult: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateLoadBalanceResultObject(::core::mem::transmute_copy(&targetname)) {
                ::core::result::Result::Ok(ok__) => {
                    *pplbresult = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateSessionObject<Impl: ITsSbProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, targetname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, username: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, domain: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, sessionid: u32, ppsession: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateSessionObject(::core::mem::transmute_copy(&targetname), ::core::mem::transmute_copy(&username), ::core::mem::transmute_copy(&domain), ::core::mem::transmute_copy(&sessionid)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppsession = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreatePluginPropertySet<Impl: ITsSbProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pppropertyset: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreatePluginPropertySet() {
                ::core::result::Result::Ok(ok__) => {
                    *pppropertyset = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateTargetPropertySetObject<Impl: ITsSbProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pppropertyset: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateTargetPropertySetObject() {
                ::core::result::Result::Ok(ok__) => {
                    *pppropertyset = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateEnvironmentObject<Impl: ITsSbProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, serverweight: u32, ppenvironment: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateEnvironmentObject(::core::mem::transmute_copy(&name), ::core::mem::transmute_copy(&serverweight)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppenvironment = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetResourcePluginStore<Impl: ITsSbProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppstore: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetResourcePluginStore() {
                ::core::result::Result::Ok(ok__) => {
                    *ppstore = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFilterPluginStore<Impl: ITsSbProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppstore: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetFilterPluginStore() {
                ::core::result::Result::Ok(ok__) => {
                    *ppstore = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RegisterForNotification<Impl: ITsSbProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, notificationtype: u32, resourcetomonitor: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ppluginnotification: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RegisterForNotification(::core::mem::transmute_copy(&notificationtype), ::core::mem::transmute_copy(&resourcetomonitor), ::core::mem::transmute(&ppluginnotification)).into()
        }
        unsafe extern "system" fn UnRegisterForNotification<Impl: ITsSbProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, notificationtype: u32, resourcetomonitor: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).UnRegisterForNotification(::core::mem::transmute_copy(&notificationtype), ::core::mem::transmute_copy(&resourcetomonitor)).into()
        }
        unsafe extern "system" fn GetInstanceOfGlobalStore<Impl: ITsSbProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppglobalstore: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetInstanceOfGlobalStore() {
                ::core::result::Result::Ok(ok__) => {
                    *ppglobalstore = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateEnvironmentPropertySetObject<Impl: ITsSbProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pppropertyset: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateEnvironmentPropertySetObject() {
                ::core::result::Result::Ok(ok__) => {
                    *pppropertyset = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            CreateTargetObject: CreateTargetObject::<Impl, IMPL_OFFSET>,
            CreateLoadBalanceResultObject: CreateLoadBalanceResultObject::<Impl, IMPL_OFFSET>,
            CreateSessionObject: CreateSessionObject::<Impl, IMPL_OFFSET>,
            CreatePluginPropertySet: CreatePluginPropertySet::<Impl, IMPL_OFFSET>,
            CreateTargetPropertySetObject: CreateTargetPropertySetObject::<Impl, IMPL_OFFSET>,
            CreateEnvironmentObject: CreateEnvironmentObject::<Impl, IMPL_OFFSET>,
            GetResourcePluginStore: GetResourcePluginStore::<Impl, IMPL_OFFSET>,
            GetFilterPluginStore: GetFilterPluginStore::<Impl, IMPL_OFFSET>,
            RegisterForNotification: RegisterForNotification::<Impl, IMPL_OFFSET>,
            UnRegisterForNotification: UnRegisterForNotification::<Impl, IMPL_OFFSET>,
            GetInstanceOfGlobalStore: GetInstanceOfGlobalStore::<Impl, IMPL_OFFSET>,
            CreateEnvironmentPropertySetObject: CreateEnvironmentPropertySetObject::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITsSbProvider as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
pub trait ITsSbProvisioning_Impl: Sized + ITsSbPlugin_Impl {
    fn CreateVirtualMachines(&mut self, jobxmlstring: &super::super::Foundation::BSTR, jobguid: &super::super::Foundation::BSTR, psink: &::core::option::Option<ITsSbProvisioningPluginNotifySink>) -> ::windows::core::Result<()>;
    fn PatchVirtualMachines(&mut self, jobxmlstring: &super::super::Foundation::BSTR, jobguid: &super::super::Foundation::BSTR, psink: &::core::option::Option<ITsSbProvisioningPluginNotifySink>, pvmpatchinfo: *const VM_PATCH_INFO) -> ::windows::core::Result<()>;
    fn DeleteVirtualMachines(&mut self, jobxmlstring: &super::super::Foundation::BSTR, jobguid: &super::super::Foundation::BSTR, psink: &::core::option::Option<ITsSbProvisioningPluginNotifySink>) -> ::windows::core::Result<()>;
    fn CancelJob(&mut self, jobguid: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
impl ITsSbProvisioning_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITsSbProvisioning_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITsSbProvisioning_Vtbl {
        unsafe extern "system" fn CreateVirtualMachines<Impl: ITsSbProvisioning_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, jobxmlstring: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, jobguid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, psink: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).CreateVirtualMachines(::core::mem::transmute_copy(&jobxmlstring), ::core::mem::transmute_copy(&jobguid), ::core::mem::transmute(&psink)).into()
        }
        unsafe extern "system" fn PatchVirtualMachines<Impl: ITsSbProvisioning_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, jobxmlstring: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, jobguid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, psink: ::windows::core::RawPtr, pvmpatchinfo: *const VM_PATCH_INFO) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).PatchVirtualMachines(::core::mem::transmute_copy(&jobxmlstring), ::core::mem::transmute_copy(&jobguid), ::core::mem::transmute(&psink), ::core::mem::transmute_copy(&pvmpatchinfo)).into()
        }
        unsafe extern "system" fn DeleteVirtualMachines<Impl: ITsSbProvisioning_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, jobxmlstring: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, jobguid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, psink: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).DeleteVirtualMachines(::core::mem::transmute_copy(&jobxmlstring), ::core::mem::transmute_copy(&jobguid), ::core::mem::transmute(&psink)).into()
        }
        unsafe extern "system" fn CancelJob<Impl: ITsSbProvisioning_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, jobguid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).CancelJob(::core::mem::transmute_copy(&jobguid)).into()
        }
        Self {
            base: ITsSbPlugin_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            CreateVirtualMachines: CreateVirtualMachines::<Impl, IMPL_OFFSET>,
            PatchVirtualMachines: PatchVirtualMachines::<Impl, IMPL_OFFSET>,
            DeleteVirtualMachines: DeleteVirtualMachines::<Impl, IMPL_OFFSET>,
            CancelJob: CancelJob::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITsSbProvisioning as ::windows::core::Interface>::IID || iid == &<ITsSbPlugin as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait ITsSbProvisioningPluginNotifySink_Impl: Sized {
    fn OnJobCreated(&mut self, pvmnotifyinfo: *const VM_NOTIFY_INFO) -> ::windows::core::Result<()>;
    fn OnVirtualMachineStatusChanged(&mut self, pvmnotifyentry: *const VM_NOTIFY_ENTRY, vmnotifystatus: VM_NOTIFY_STATUS, errorcode: ::windows::core::HRESULT, errordescr: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn OnJobCompleted(&mut self, resultcode: ::windows::core::HRESULT, resultdescription: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn OnJobCancelled(&mut self) -> ::windows::core::Result<()>;
    fn LockVirtualMachine(&mut self, pvmnotifyentry: *const VM_NOTIFY_ENTRY) -> ::windows::core::Result<()>;
    fn OnVirtualMachineHostStatusChanged(&mut self, vmhost: &super::super::Foundation::BSTR, vmhostnotifystatus: VM_HOST_NOTIFY_STATUS, errorcode: ::windows::core::HRESULT, errordescr: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ITsSbProvisioningPluginNotifySink_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITsSbProvisioningPluginNotifySink_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITsSbProvisioningPluginNotifySink_Vtbl {
        unsafe extern "system" fn OnJobCreated<Impl: ITsSbProvisioningPluginNotifySink_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvmnotifyinfo: *const VM_NOTIFY_INFO) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnJobCreated(::core::mem::transmute_copy(&pvmnotifyinfo)).into()
        }
        unsafe extern "system" fn OnVirtualMachineStatusChanged<Impl: ITsSbProvisioningPluginNotifySink_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvmnotifyentry: *const VM_NOTIFY_ENTRY, vmnotifystatus: VM_NOTIFY_STATUS, errorcode: ::windows::core::HRESULT, errordescr: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnVirtualMachineStatusChanged(::core::mem::transmute_copy(&pvmnotifyentry), ::core::mem::transmute_copy(&vmnotifystatus), ::core::mem::transmute_copy(&errorcode), ::core::mem::transmute_copy(&errordescr)).into()
        }
        unsafe extern "system" fn OnJobCompleted<Impl: ITsSbProvisioningPluginNotifySink_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, resultcode: ::windows::core::HRESULT, resultdescription: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnJobCompleted(::core::mem::transmute_copy(&resultcode), ::core::mem::transmute_copy(&resultdescription)).into()
        }
        unsafe extern "system" fn OnJobCancelled<Impl: ITsSbProvisioningPluginNotifySink_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnJobCancelled().into()
        }
        unsafe extern "system" fn LockVirtualMachine<Impl: ITsSbProvisioningPluginNotifySink_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvmnotifyentry: *const VM_NOTIFY_ENTRY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).LockVirtualMachine(::core::mem::transmute_copy(&pvmnotifyentry)).into()
        }
        unsafe extern "system" fn OnVirtualMachineHostStatusChanged<Impl: ITsSbProvisioningPluginNotifySink_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, vmhost: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, vmhostnotifystatus: VM_HOST_NOTIFY_STATUS, errorcode: ::windows::core::HRESULT, errordescr: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnVirtualMachineHostStatusChanged(::core::mem::transmute_copy(&vmhost), ::core::mem::transmute_copy(&vmhostnotifystatus), ::core::mem::transmute_copy(&errorcode), ::core::mem::transmute_copy(&errordescr)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            OnJobCreated: OnJobCreated::<Impl, IMPL_OFFSET>,
            OnVirtualMachineStatusChanged: OnVirtualMachineStatusChanged::<Impl, IMPL_OFFSET>,
            OnJobCompleted: OnJobCompleted::<Impl, IMPL_OFFSET>,
            OnJobCancelled: OnJobCancelled::<Impl, IMPL_OFFSET>,
            LockVirtualMachine: LockVirtualMachine::<Impl, IMPL_OFFSET>,
            OnVirtualMachineHostStatusChanged: OnVirtualMachineHostStatusChanged::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITsSbProvisioningPluginNotifySink as ::windows::core::Interface>::IID
    }
}
pub trait ITsSbResourceNotification_Impl: Sized {
    fn NotifySessionChange(&mut self, changetype: TSSESSION_STATE, psession: &::core::option::Option<ITsSbSession>) -> ::windows::core::Result<()>;
    fn NotifyTargetChange(&mut self, targetchangetype: u32, ptarget: &::core::option::Option<ITsSbTarget>) -> ::windows::core::Result<()>;
    fn NotifyClientConnectionStateChange(&mut self, changetype: CONNECTION_CHANGE_NOTIFICATION, pconnection: &::core::option::Option<ITsSbClientConnection>) -> ::windows::core::Result<()>;
}
impl ITsSbResourceNotification_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITsSbResourceNotification_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITsSbResourceNotification_Vtbl {
        unsafe extern "system" fn NotifySessionChange<Impl: ITsSbResourceNotification_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, changetype: TSSESSION_STATE, psession: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).NotifySessionChange(::core::mem::transmute_copy(&changetype), ::core::mem::transmute(&psession)).into()
        }
        unsafe extern "system" fn NotifyTargetChange<Impl: ITsSbResourceNotification_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, targetchangetype: u32, ptarget: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).NotifyTargetChange(::core::mem::transmute_copy(&targetchangetype), ::core::mem::transmute(&ptarget)).into()
        }
        unsafe extern "system" fn NotifyClientConnectionStateChange<Impl: ITsSbResourceNotification_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, changetype: CONNECTION_CHANGE_NOTIFICATION, pconnection: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).NotifyClientConnectionStateChange(::core::mem::transmute_copy(&changetype), ::core::mem::transmute(&pconnection)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            NotifySessionChange: NotifySessionChange::<Impl, IMPL_OFFSET>,
            NotifyTargetChange: NotifyTargetChange::<Impl, IMPL_OFFSET>,
            NotifyClientConnectionStateChange: NotifyClientConnectionStateChange::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITsSbResourceNotification as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait ITsSbResourceNotificationEx_Impl: Sized {
    fn NotifySessionChangeEx(&mut self, targetname: &super::super::Foundation::BSTR, username: &super::super::Foundation::BSTR, domain: &super::super::Foundation::BSTR, sessionid: u32, sessionstate: TSSESSION_STATE) -> ::windows::core::Result<()>;
    fn NotifyTargetChangeEx(&mut self, targetname: &super::super::Foundation::BSTR, targetchangetype: u32) -> ::windows::core::Result<()>;
    fn NotifyClientConnectionStateChangeEx(&mut self, username: &super::super::Foundation::BSTR, domain: &super::super::Foundation::BSTR, initialprogram: &super::super::Foundation::BSTR, poolname: &super::super::Foundation::BSTR, targetname: &super::super::Foundation::BSTR, connectionchangetype: CONNECTION_CHANGE_NOTIFICATION) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ITsSbResourceNotificationEx_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITsSbResourceNotificationEx_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITsSbResourceNotificationEx_Vtbl {
        unsafe extern "system" fn NotifySessionChangeEx<Impl: ITsSbResourceNotificationEx_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, targetname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, username: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, domain: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, sessionid: u32, sessionstate: TSSESSION_STATE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).NotifySessionChangeEx(::core::mem::transmute_copy(&targetname), ::core::mem::transmute_copy(&username), ::core::mem::transmute_copy(&domain), ::core::mem::transmute_copy(&sessionid), ::core::mem::transmute_copy(&sessionstate)).into()
        }
        unsafe extern "system" fn NotifyTargetChangeEx<Impl: ITsSbResourceNotificationEx_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, targetname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, targetchangetype: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).NotifyTargetChangeEx(::core::mem::transmute_copy(&targetname), ::core::mem::transmute_copy(&targetchangetype)).into()
        }
        unsafe extern "system" fn NotifyClientConnectionStateChangeEx<Impl: ITsSbResourceNotificationEx_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, username: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, domain: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, initialprogram: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, poolname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, targetname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, connectionchangetype: CONNECTION_CHANGE_NOTIFICATION) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).NotifyClientConnectionStateChangeEx(::core::mem::transmute_copy(&username), ::core::mem::transmute_copy(&domain), ::core::mem::transmute_copy(&initialprogram), ::core::mem::transmute_copy(&poolname), ::core::mem::transmute_copy(&targetname), ::core::mem::transmute_copy(&connectionchangetype)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            NotifySessionChangeEx: NotifySessionChangeEx::<Impl, IMPL_OFFSET>,
            NotifyTargetChangeEx: NotifyTargetChangeEx::<Impl, IMPL_OFFSET>,
            NotifyClientConnectionStateChangeEx: NotifyClientConnectionStateChangeEx::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITsSbResourceNotificationEx as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com_StructuredStorage")]
pub trait ITsSbResourcePlugin_Impl: Sized + ITsSbPlugin_Impl {}
#[cfg(feature = "Win32_System_Com_StructuredStorage")]
impl ITsSbResourcePlugin_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITsSbResourcePlugin_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITsSbResourcePlugin_Vtbl {
        Self { base: ITsSbPlugin_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITsSbResourcePlugin as ::windows::core::Interface>::IID || iid == &<ITsSbPlugin as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ITsSbResourcePluginStore_Impl: Sized {
    fn QueryTarget(&mut self, targetname: &super::super::Foundation::BSTR, farmname: &super::super::Foundation::BSTR) -> ::windows::core::Result<ITsSbTarget>;
    fn QuerySessionBySessionId(&mut self, dwsessionid: u32, targetname: &super::super::Foundation::BSTR) -> ::windows::core::Result<ITsSbSession>;
    fn AddTargetToStore(&mut self, ptarget: &::core::option::Option<ITsSbTarget>) -> ::windows::core::Result<()>;
    fn AddSessionToStore(&mut self, psession: &::core::option::Option<ITsSbSession>) -> ::windows::core::Result<()>;
    fn AddEnvironmentToStore(&mut self, penvironment: &::core::option::Option<ITsSbEnvironment>) -> ::windows::core::Result<()>;
    fn RemoveEnvironmentFromStore(&mut self, environmentname: &super::super::Foundation::BSTR, bignoreowner: super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn EnumerateFarms(&mut self, pdwcount: *mut u32, pval: *mut *mut super::Com::SAFEARRAY) -> ::windows::core::Result<()>;
    fn QueryEnvironment(&mut self, environmentname: &super::super::Foundation::BSTR) -> ::windows::core::Result<ITsSbEnvironment>;
    fn EnumerateEnvironments(&mut self, pdwcount: *mut u32, pval: *mut *mut ::core::option::Option<ITsSbEnvironment>) -> ::windows::core::Result<()>;
    fn SaveTarget(&mut self, ptarget: &::core::option::Option<ITsSbTarget>, bforcewrite: super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn SaveEnvironment(&mut self, penvironment: &::core::option::Option<ITsSbEnvironment>, bforcewrite: super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn SaveSession(&mut self, psession: &::core::option::Option<ITsSbSession>) -> ::windows::core::Result<()>;
    fn SetTargetProperty(&mut self, targetname: &super::super::Foundation::BSTR, propertyname: &super::super::Foundation::BSTR, pproperty: *const super::Com::VARIANT) -> ::windows::core::Result<()>;
    fn SetEnvironmentProperty(&mut self, environmentname: &super::super::Foundation::BSTR, propertyname: &super::super::Foundation::BSTR, pproperty: *const super::Com::VARIANT) -> ::windows::core::Result<()>;
    fn SetTargetState(&mut self, targetname: &super::super::Foundation::BSTR, newstate: TARGET_STATE) -> ::windows::core::Result<TARGET_STATE>;
    fn SetSessionState(&mut self, sbsession: &::core::option::Option<ITsSbSession>) -> ::windows::core::Result<()>;
    fn EnumerateTargets(&mut self, farmname: &super::super::Foundation::BSTR, envname: &super::super::Foundation::BSTR, sortbyfieldid: TS_SB_SORT_BY, sortybypropname: &super::super::Foundation::BSTR, pdwcount: *mut u32, pval: *mut *mut ::core::option::Option<ITsSbTarget>) -> ::windows::core::Result<()>;
    fn EnumerateSessions(&mut self, targetname: &super::super::Foundation::BSTR, username: &super::super::Foundation::BSTR, userdomain: &super::super::Foundation::BSTR, poolname: &super::super::Foundation::BSTR, initialprogram: &super::super::Foundation::BSTR, psessionstate: *const TSSESSION_STATE, pdwcount: *mut u32, ppval: *mut *mut ::core::option::Option<ITsSbSession>) -> ::windows::core::Result<()>;
    fn GetFarmProperty(&mut self, farmname: &super::super::Foundation::BSTR, propertyname: &super::super::Foundation::BSTR, pvarvalue: *const super::Com::VARIANT) -> ::windows::core::Result<()>;
    fn DeleteTarget(&mut self, targetname: &super::super::Foundation::BSTR, hostname: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn SetTargetPropertyWithVersionCheck(&mut self, ptarget: &::core::option::Option<ITsSbTarget>, propertyname: &super::super::Foundation::BSTR, pproperty: *const super::Com::VARIANT) -> ::windows::core::Result<()>;
    fn SetEnvironmentPropertyWithVersionCheck(&mut self, penvironment: &::core::option::Option<ITsSbEnvironment>, propertyname: &super::super::Foundation::BSTR, pproperty: *const super::Com::VARIANT) -> ::windows::core::Result<()>;
    fn AcquireTargetLock(&mut self, targetname: &super::super::Foundation::BSTR, dwtimeout: u32) -> ::windows::core::Result<::windows::core::IUnknown>;
    fn ReleaseTargetLock(&mut self, pcontext: &::core::option::Option<::windows::core::IUnknown>) -> ::windows::core::Result<()>;
    fn TestAndSetServerState(&mut self, poolname: &super::super::Foundation::BSTR, serverfqdn: &super::super::Foundation::BSTR, newstate: TARGET_STATE, teststate: TARGET_STATE) -> ::windows::core::Result<TARGET_STATE>;
    fn SetServerWaitingToStart(&mut self, poolname: &super::super::Foundation::BSTR, servername: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn GetServerState(&mut self, poolname: &super::super::Foundation::BSTR, serverfqdn: &super::super::Foundation::BSTR) -> ::windows::core::Result<TARGET_STATE>;
    fn SetServerDrainMode(&mut self, serverfqdn: &super::super::Foundation::BSTR, drainmode: u32) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ITsSbResourcePluginStore_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITsSbResourcePluginStore_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITsSbResourcePluginStore_Vtbl {
        unsafe extern "system" fn QueryTarget<Impl: ITsSbResourcePluginStore_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, targetname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, farmname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pptarget: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).QueryTarget(::core::mem::transmute_copy(&targetname), ::core::mem::transmute_copy(&farmname)) {
                ::core::result::Result::Ok(ok__) => {
                    *pptarget = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn QuerySessionBySessionId<Impl: ITsSbResourcePluginStore_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwsessionid: u32, targetname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ppsession: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).QuerySessionBySessionId(::core::mem::transmute_copy(&dwsessionid), ::core::mem::transmute_copy(&targetname)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppsession = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AddTargetToStore<Impl: ITsSbResourcePluginStore_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ptarget: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AddTargetToStore(::core::mem::transmute(&ptarget)).into()
        }
        unsafe extern "system" fn AddSessionToStore<Impl: ITsSbResourcePluginStore_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psession: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AddSessionToStore(::core::mem::transmute(&psession)).into()
        }
        unsafe extern "system" fn AddEnvironmentToStore<Impl: ITsSbResourcePluginStore_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, penvironment: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AddEnvironmentToStore(::core::mem::transmute(&penvironment)).into()
        }
        unsafe extern "system" fn RemoveEnvironmentFromStore<Impl: ITsSbResourcePluginStore_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, environmentname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bignoreowner: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveEnvironmentFromStore(::core::mem::transmute_copy(&environmentname), ::core::mem::transmute_copy(&bignoreowner)).into()
        }
        unsafe extern "system" fn EnumerateFarms<Impl: ITsSbResourcePluginStore_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwcount: *mut u32, pval: *mut *mut super::Com::SAFEARRAY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).EnumerateFarms(::core::mem::transmute_copy(&pdwcount), ::core::mem::transmute_copy(&pval)).into()
        }
        unsafe extern "system" fn QueryEnvironment<Impl: ITsSbResourcePluginStore_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, environmentname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ppenvironment: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).QueryEnvironment(::core::mem::transmute_copy(&environmentname)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppenvironment = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnumerateEnvironments<Impl: ITsSbResourcePluginStore_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwcount: *mut u32, pval: *mut *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).EnumerateEnvironments(::core::mem::transmute_copy(&pdwcount), ::core::mem::transmute_copy(&pval)).into()
        }
        unsafe extern "system" fn SaveTarget<Impl: ITsSbResourcePluginStore_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ptarget: ::windows::core::RawPtr, bforcewrite: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SaveTarget(::core::mem::transmute(&ptarget), ::core::mem::transmute_copy(&bforcewrite)).into()
        }
        unsafe extern "system" fn SaveEnvironment<Impl: ITsSbResourcePluginStore_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, penvironment: ::windows::core::RawPtr, bforcewrite: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SaveEnvironment(::core::mem::transmute(&penvironment), ::core::mem::transmute_copy(&bforcewrite)).into()
        }
        unsafe extern "system" fn SaveSession<Impl: ITsSbResourcePluginStore_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psession: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SaveSession(::core::mem::transmute(&psession)).into()
        }
        unsafe extern "system" fn SetTargetProperty<Impl: ITsSbResourcePluginStore_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, targetname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, propertyname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pproperty: *const super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetTargetProperty(::core::mem::transmute_copy(&targetname), ::core::mem::transmute_copy(&propertyname), ::core::mem::transmute_copy(&pproperty)).into()
        }
        unsafe extern "system" fn SetEnvironmentProperty<Impl: ITsSbResourcePluginStore_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, environmentname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, propertyname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pproperty: *const super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetEnvironmentProperty(::core::mem::transmute_copy(&environmentname), ::core::mem::transmute_copy(&propertyname), ::core::mem::transmute_copy(&pproperty)).into()
        }
        unsafe extern "system" fn SetTargetState<Impl: ITsSbResourcePluginStore_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, targetname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, newstate: TARGET_STATE, poldstate: *mut TARGET_STATE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetTargetState(::core::mem::transmute_copy(&targetname), ::core::mem::transmute_copy(&newstate)) {
                ::core::result::Result::Ok(ok__) => {
                    *poldstate = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSessionState<Impl: ITsSbResourcePluginStore_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, sbsession: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetSessionState(::core::mem::transmute(&sbsession)).into()
        }
        unsafe extern "system" fn EnumerateTargets<Impl: ITsSbResourcePluginStore_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, farmname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, envname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, sortbyfieldid: TS_SB_SORT_BY, sortybypropname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pdwcount: *mut u32, pval: *mut *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).EnumerateTargets(::core::mem::transmute_copy(&farmname), ::core::mem::transmute_copy(&envname), ::core::mem::transmute_copy(&sortbyfieldid), ::core::mem::transmute_copy(&sortybypropname), ::core::mem::transmute_copy(&pdwcount), ::core::mem::transmute_copy(&pval)).into()
        }
        unsafe extern "system" fn EnumerateSessions<Impl: ITsSbResourcePluginStore_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, targetname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, username: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, userdomain: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, poolname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, initialprogram: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, psessionstate: *const TSSESSION_STATE, pdwcount: *mut u32, ppval: *mut *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).EnumerateSessions(::core::mem::transmute_copy(&targetname), ::core::mem::transmute_copy(&username), ::core::mem::transmute_copy(&userdomain), ::core::mem::transmute_copy(&poolname), ::core::mem::transmute_copy(&initialprogram), ::core::mem::transmute_copy(&psessionstate), ::core::mem::transmute_copy(&pdwcount), ::core::mem::transmute_copy(&ppval)).into()
        }
        unsafe extern "system" fn GetFarmProperty<Impl: ITsSbResourcePluginStore_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, farmname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, propertyname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pvarvalue: *const super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetFarmProperty(::core::mem::transmute_copy(&farmname), ::core::mem::transmute_copy(&propertyname), ::core::mem::transmute_copy(&pvarvalue)).into()
        }
        unsafe extern "system" fn DeleteTarget<Impl: ITsSbResourcePluginStore_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, targetname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, hostname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).DeleteTarget(::core::mem::transmute_copy(&targetname), ::core::mem::transmute_copy(&hostname)).into()
        }
        unsafe extern "system" fn SetTargetPropertyWithVersionCheck<Impl: ITsSbResourcePluginStore_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ptarget: ::windows::core::RawPtr, propertyname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pproperty: *const super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetTargetPropertyWithVersionCheck(::core::mem::transmute(&ptarget), ::core::mem::transmute_copy(&propertyname), ::core::mem::transmute_copy(&pproperty)).into()
        }
        unsafe extern "system" fn SetEnvironmentPropertyWithVersionCheck<Impl: ITsSbResourcePluginStore_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, penvironment: ::windows::core::RawPtr, propertyname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pproperty: *const super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetEnvironmentPropertyWithVersionCheck(::core::mem::transmute(&penvironment), ::core::mem::transmute_copy(&propertyname), ::core::mem::transmute_copy(&pproperty)).into()
        }
        unsafe extern "system" fn AcquireTargetLock<Impl: ITsSbResourcePluginStore_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, targetname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, dwtimeout: u32, ppcontext: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AcquireTargetLock(::core::mem::transmute_copy(&targetname), ::core::mem::transmute_copy(&dwtimeout)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppcontext = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ReleaseTargetLock<Impl: ITsSbResourcePluginStore_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcontext: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ReleaseTargetLock(::core::mem::transmute(&pcontext)).into()
        }
        unsafe extern "system" fn TestAndSetServerState<Impl: ITsSbResourcePluginStore_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, poolname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, serverfqdn: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, newstate: TARGET_STATE, teststate: TARGET_STATE, pinitstate: *mut TARGET_STATE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TestAndSetServerState(::core::mem::transmute_copy(&poolname), ::core::mem::transmute_copy(&serverfqdn), ::core::mem::transmute_copy(&newstate), ::core::mem::transmute_copy(&teststate)) {
                ::core::result::Result::Ok(ok__) => {
                    *pinitstate = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetServerWaitingToStart<Impl: ITsSbResourcePluginStore_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, poolname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, servername: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetServerWaitingToStart(::core::mem::transmute_copy(&poolname), ::core::mem::transmute_copy(&servername)).into()
        }
        unsafe extern "system" fn GetServerState<Impl: ITsSbResourcePluginStore_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, poolname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, serverfqdn: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pstate: *mut TARGET_STATE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetServerState(::core::mem::transmute_copy(&poolname), ::core::mem::transmute_copy(&serverfqdn)) {
                ::core::result::Result::Ok(ok__) => {
                    *pstate = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetServerDrainMode<Impl: ITsSbResourcePluginStore_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, serverfqdn: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, drainmode: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetServerDrainMode(::core::mem::transmute_copy(&serverfqdn), ::core::mem::transmute_copy(&drainmode)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            QueryTarget: QueryTarget::<Impl, IMPL_OFFSET>,
            QuerySessionBySessionId: QuerySessionBySessionId::<Impl, IMPL_OFFSET>,
            AddTargetToStore: AddTargetToStore::<Impl, IMPL_OFFSET>,
            AddSessionToStore: AddSessionToStore::<Impl, IMPL_OFFSET>,
            AddEnvironmentToStore: AddEnvironmentToStore::<Impl, IMPL_OFFSET>,
            RemoveEnvironmentFromStore: RemoveEnvironmentFromStore::<Impl, IMPL_OFFSET>,
            EnumerateFarms: EnumerateFarms::<Impl, IMPL_OFFSET>,
            QueryEnvironment: QueryEnvironment::<Impl, IMPL_OFFSET>,
            EnumerateEnvironments: EnumerateEnvironments::<Impl, IMPL_OFFSET>,
            SaveTarget: SaveTarget::<Impl, IMPL_OFFSET>,
            SaveEnvironment: SaveEnvironment::<Impl, IMPL_OFFSET>,
            SaveSession: SaveSession::<Impl, IMPL_OFFSET>,
            SetTargetProperty: SetTargetProperty::<Impl, IMPL_OFFSET>,
            SetEnvironmentProperty: SetEnvironmentProperty::<Impl, IMPL_OFFSET>,
            SetTargetState: SetTargetState::<Impl, IMPL_OFFSET>,
            SetSessionState: SetSessionState::<Impl, IMPL_OFFSET>,
            EnumerateTargets: EnumerateTargets::<Impl, IMPL_OFFSET>,
            EnumerateSessions: EnumerateSessions::<Impl, IMPL_OFFSET>,
            GetFarmProperty: GetFarmProperty::<Impl, IMPL_OFFSET>,
            DeleteTarget: DeleteTarget::<Impl, IMPL_OFFSET>,
            SetTargetPropertyWithVersionCheck: SetTargetPropertyWithVersionCheck::<Impl, IMPL_OFFSET>,
            SetEnvironmentPropertyWithVersionCheck: SetEnvironmentPropertyWithVersionCheck::<Impl, IMPL_OFFSET>,
            AcquireTargetLock: AcquireTargetLock::<Impl, IMPL_OFFSET>,
            ReleaseTargetLock: ReleaseTargetLock::<Impl, IMPL_OFFSET>,
            TestAndSetServerState: TestAndSetServerState::<Impl, IMPL_OFFSET>,
            SetServerWaitingToStart: SetServerWaitingToStart::<Impl, IMPL_OFFSET>,
            GetServerState: GetServerState::<Impl, IMPL_OFFSET>,
            SetServerDrainMode: SetServerDrainMode::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITsSbResourcePluginStore as ::windows::core::Interface>::IID
    }
}
pub trait ITsSbServiceNotification_Impl: Sized {
    fn NotifyServiceFailure(&mut self) -> ::windows::core::Result<()>;
    fn NotifyServiceSuccess(&mut self) -> ::windows::core::Result<()>;
}
impl ITsSbServiceNotification_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITsSbServiceNotification_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITsSbServiceNotification_Vtbl {
        unsafe extern "system" fn NotifyServiceFailure<Impl: ITsSbServiceNotification_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).NotifyServiceFailure().into()
        }
        unsafe extern "system" fn NotifyServiceSuccess<Impl: ITsSbServiceNotification_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).NotifyServiceSuccess().into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            NotifyServiceFailure: NotifyServiceFailure::<Impl, IMPL_OFFSET>,
            NotifyServiceSuccess: NotifyServiceSuccess::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITsSbServiceNotification as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait ITsSbSession_Impl: Sized {
    fn SessionId(&mut self) -> ::windows::core::Result<u32>;
    fn TargetName(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetTargetName(&mut self, targetname: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn Username(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn Domain(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn State(&mut self) -> ::windows::core::Result<TSSESSION_STATE>;
    fn SetState(&mut self, state: TSSESSION_STATE) -> ::windows::core::Result<()>;
    fn CreateTime(&mut self) -> ::windows::core::Result<super::super::Foundation::FILETIME>;
    fn SetCreateTime(&mut self, time: &super::super::Foundation::FILETIME) -> ::windows::core::Result<()>;
    fn DisconnectTime(&mut self) -> ::windows::core::Result<super::super::Foundation::FILETIME>;
    fn SetDisconnectTime(&mut self, time: &super::super::Foundation::FILETIME) -> ::windows::core::Result<()>;
    fn InitialProgram(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetInitialProgram(&mut self, application: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn ClientDisplay(&mut self) -> ::windows::core::Result<CLIENT_DISPLAY>;
    fn SetClientDisplay(&mut self, pclientdisplay: &CLIENT_DISPLAY) -> ::windows::core::Result<()>;
    fn ProtocolType(&mut self) -> ::windows::core::Result<u32>;
    fn SetProtocolType(&mut self, val: u32) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ITsSbSession_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITsSbSession_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITsSbSession_Vtbl {
        unsafe extern "system" fn SessionId<Impl: ITsSbSession_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SessionId() {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TargetName<Impl: ITsSbSession_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, targetname: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TargetName() {
                ::core::result::Result::Ok(ok__) => {
                    *targetname = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetTargetName<Impl: ITsSbSession_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, targetname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetTargetName(::core::mem::transmute_copy(&targetname)).into()
        }
        unsafe extern "system" fn Username<Impl: ITsSbSession_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, username: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Username() {
                ::core::result::Result::Ok(ok__) => {
                    *username = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Domain<Impl: ITsSbSession_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, domain: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Domain() {
                ::core::result::Result::Ok(ok__) => {
                    *domain = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn State<Impl: ITsSbSession_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pstate: *mut TSSESSION_STATE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).State() {
                ::core::result::Result::Ok(ok__) => {
                    *pstate = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetState<Impl: ITsSbSession_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, state: TSSESSION_STATE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetState(::core::mem::transmute_copy(&state)).into()
        }
        unsafe extern "system" fn CreateTime<Impl: ITsSbSession_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ptime: *mut super::super::Foundation::FILETIME) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateTime() {
                ::core::result::Result::Ok(ok__) => {
                    *ptime = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCreateTime<Impl: ITsSbSession_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, time: super::super::Foundation::FILETIME) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetCreateTime(::core::mem::transmute_copy(&time)).into()
        }
        unsafe extern "system" fn DisconnectTime<Impl: ITsSbSession_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ptime: *mut super::super::Foundation::FILETIME) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DisconnectTime() {
                ::core::result::Result::Ok(ok__) => {
                    *ptime = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDisconnectTime<Impl: ITsSbSession_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, time: super::super::Foundation::FILETIME) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetDisconnectTime(::core::mem::transmute_copy(&time)).into()
        }
        unsafe extern "system" fn InitialProgram<Impl: ITsSbSession_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, app: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).InitialProgram() {
                ::core::result::Result::Ok(ok__) => {
                    *app = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetInitialProgram<Impl: ITsSbSession_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, application: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetInitialProgram(::core::mem::transmute_copy(&application)).into()
        }
        unsafe extern "system" fn ClientDisplay<Impl: ITsSbSession_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pclientdisplay: *mut CLIENT_DISPLAY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ClientDisplay() {
                ::core::result::Result::Ok(ok__) => {
                    *pclientdisplay = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetClientDisplay<Impl: ITsSbSession_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pclientdisplay: CLIENT_DISPLAY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetClientDisplay(::core::mem::transmute_copy(&pclientdisplay)).into()
        }
        unsafe extern "system" fn ProtocolType<Impl: ITsSbSession_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ProtocolType() {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetProtocolType<Impl: ITsSbSession_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, val: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetProtocolType(::core::mem::transmute_copy(&val)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            SessionId: SessionId::<Impl, IMPL_OFFSET>,
            TargetName: TargetName::<Impl, IMPL_OFFSET>,
            SetTargetName: SetTargetName::<Impl, IMPL_OFFSET>,
            Username: Username::<Impl, IMPL_OFFSET>,
            Domain: Domain::<Impl, IMPL_OFFSET>,
            State: State::<Impl, IMPL_OFFSET>,
            SetState: SetState::<Impl, IMPL_OFFSET>,
            CreateTime: CreateTime::<Impl, IMPL_OFFSET>,
            SetCreateTime: SetCreateTime::<Impl, IMPL_OFFSET>,
            DisconnectTime: DisconnectTime::<Impl, IMPL_OFFSET>,
            SetDisconnectTime: SetDisconnectTime::<Impl, IMPL_OFFSET>,
            InitialProgram: InitialProgram::<Impl, IMPL_OFFSET>,
            SetInitialProgram: SetInitialProgram::<Impl, IMPL_OFFSET>,
            ClientDisplay: ClientDisplay::<Impl, IMPL_OFFSET>,
            SetClientDisplay: SetClientDisplay::<Impl, IMPL_OFFSET>,
            ProtocolType: ProtocolType::<Impl, IMPL_OFFSET>,
            SetProtocolType: SetProtocolType::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITsSbSession as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
pub trait ITsSbTarget_Impl: Sized {
    fn TargetName(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetTargetName(&mut self, val: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn FarmName(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetFarmName(&mut self, val: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn TargetFQDN(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetTargetFQDN(&mut self, val: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn TargetNetbios(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetTargetNetbios(&mut self, val: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn IpAddresses(&mut self, sockaddr: *mut TSSD_ConnectionPoint, numaddresses: *mut u32) -> ::windows::core::Result<()>;
    fn SetIpAddresses(&mut self, sockaddr: *const TSSD_ConnectionPoint, numaddresses: u32) -> ::windows::core::Result<()>;
    fn TargetState(&mut self) -> ::windows::core::Result<TARGET_STATE>;
    fn SetTargetState(&mut self, state: TARGET_STATE) -> ::windows::core::Result<()>;
    fn TargetPropertySet(&mut self) -> ::windows::core::Result<ITsSbTargetPropertySet>;
    fn SetTargetPropertySet(&mut self, pval: &::core::option::Option<ITsSbTargetPropertySet>) -> ::windows::core::Result<()>;
    fn EnvironmentName(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetEnvironmentName(&mut self, val: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn NumSessions(&mut self) -> ::windows::core::Result<u32>;
    fn NumPendingConnections(&mut self) -> ::windows::core::Result<u32>;
    fn TargetLoad(&mut self) -> ::windows::core::Result<u32>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
impl ITsSbTarget_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITsSbTarget_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITsSbTarget_Vtbl {
        unsafe extern "system" fn TargetName<Impl: ITsSbTarget_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TargetName() {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetTargetName<Impl: ITsSbTarget_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, val: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetTargetName(::core::mem::transmute_copy(&val)).into()
        }
        unsafe extern "system" fn FarmName<Impl: ITsSbTarget_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FarmName() {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetFarmName<Impl: ITsSbTarget_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, val: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetFarmName(::core::mem::transmute_copy(&val)).into()
        }
        unsafe extern "system" fn TargetFQDN<Impl: ITsSbTarget_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, targetfqdnname: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TargetFQDN() {
                ::core::result::Result::Ok(ok__) => {
                    *targetfqdnname = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetTargetFQDN<Impl: ITsSbTarget_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, val: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetTargetFQDN(::core::mem::transmute_copy(&val)).into()
        }
        unsafe extern "system" fn TargetNetbios<Impl: ITsSbTarget_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, targetnetbiosname: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TargetNetbios() {
                ::core::result::Result::Ok(ok__) => {
                    *targetnetbiosname = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetTargetNetbios<Impl: ITsSbTarget_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, val: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetTargetNetbios(::core::mem::transmute_copy(&val)).into()
        }
        unsafe extern "system" fn IpAddresses<Impl: ITsSbTarget_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, sockaddr: *mut TSSD_ConnectionPoint, numaddresses: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).IpAddresses(::core::mem::transmute_copy(&sockaddr), ::core::mem::transmute_copy(&numaddresses)).into()
        }
        unsafe extern "system" fn SetIpAddresses<Impl: ITsSbTarget_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, sockaddr: *const TSSD_ConnectionPoint, numaddresses: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetIpAddresses(::core::mem::transmute_copy(&sockaddr), ::core::mem::transmute_copy(&numaddresses)).into()
        }
        unsafe extern "system" fn TargetState<Impl: ITsSbTarget_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pstate: *mut TARGET_STATE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TargetState() {
                ::core::result::Result::Ok(ok__) => {
                    *pstate = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetTargetState<Impl: ITsSbTarget_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, state: TARGET_STATE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetTargetState(::core::mem::transmute_copy(&state)).into()
        }
        unsafe extern "system" fn TargetPropertySet<Impl: ITsSbTarget_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pppropertyset: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TargetPropertySet() {
                ::core::result::Result::Ok(ok__) => {
                    *pppropertyset = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetTargetPropertySet<Impl: ITsSbTarget_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetTargetPropertySet(::core::mem::transmute(&pval)).into()
        }
        unsafe extern "system" fn EnvironmentName<Impl: ITsSbTarget_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EnvironmentName() {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetEnvironmentName<Impl: ITsSbTarget_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, val: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetEnvironmentName(::core::mem::transmute_copy(&val)).into()
        }
        unsafe extern "system" fn NumSessions<Impl: ITsSbTarget_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pnumsessions: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).NumSessions() {
                ::core::result::Result::Ok(ok__) => {
                    *pnumsessions = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn NumPendingConnections<Impl: ITsSbTarget_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pnumpendingconnections: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).NumPendingConnections() {
                ::core::result::Result::Ok(ok__) => {
                    *pnumpendingconnections = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TargetLoad<Impl: ITsSbTarget_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ptargetload: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TargetLoad() {
                ::core::result::Result::Ok(ok__) => {
                    *ptargetload = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            TargetName: TargetName::<Impl, IMPL_OFFSET>,
            SetTargetName: SetTargetName::<Impl, IMPL_OFFSET>,
            FarmName: FarmName::<Impl, IMPL_OFFSET>,
            SetFarmName: SetFarmName::<Impl, IMPL_OFFSET>,
            TargetFQDN: TargetFQDN::<Impl, IMPL_OFFSET>,
            SetTargetFQDN: SetTargetFQDN::<Impl, IMPL_OFFSET>,
            TargetNetbios: TargetNetbios::<Impl, IMPL_OFFSET>,
            SetTargetNetbios: SetTargetNetbios::<Impl, IMPL_OFFSET>,
            IpAddresses: IpAddresses::<Impl, IMPL_OFFSET>,
            SetIpAddresses: SetIpAddresses::<Impl, IMPL_OFFSET>,
            TargetState: TargetState::<Impl, IMPL_OFFSET>,
            SetTargetState: SetTargetState::<Impl, IMPL_OFFSET>,
            TargetPropertySet: TargetPropertySet::<Impl, IMPL_OFFSET>,
            SetTargetPropertySet: SetTargetPropertySet::<Impl, IMPL_OFFSET>,
            EnvironmentName: EnvironmentName::<Impl, IMPL_OFFSET>,
            SetEnvironmentName: SetEnvironmentName::<Impl, IMPL_OFFSET>,
            NumSessions: NumSessions::<Impl, IMPL_OFFSET>,
            NumPendingConnections: NumPendingConnections::<Impl, IMPL_OFFSET>,
            TargetLoad: TargetLoad::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITsSbTarget as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Ole"))]
pub trait ITsSbTargetPropertySet_Impl: Sized + super::Com::StructuredStorage::IPropertyBag_Impl + ITsSbPropertySet_Impl {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Ole"))]
impl ITsSbTargetPropertySet_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITsSbTargetPropertySet_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITsSbTargetPropertySet_Vtbl {
        Self { base: ITsSbPropertySet_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITsSbTargetPropertySet as ::windows::core::Interface>::IID || iid == &<super::Com::StructuredStorage::IPropertyBag as ::windows::core::Interface>::IID || iid == &<ITsSbPropertySet as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub trait ITsSbTaskInfo_Impl: Sized {
    fn TargetId(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn StartTime(&mut self) -> ::windows::core::Result<super::super::Foundation::FILETIME>;
    fn EndTime(&mut self) -> ::windows::core::Result<super::super::Foundation::FILETIME>;
    fn Deadline(&mut self) -> ::windows::core::Result<super::super::Foundation::FILETIME>;
    fn Identifier(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn Label(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn Context(&mut self) -> ::windows::core::Result<*mut super::Com::SAFEARRAY>;
    fn Plugin(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn Status(&mut self) -> ::windows::core::Result<RDV_TASK_STATUS>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ITsSbTaskInfo_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITsSbTaskInfo_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITsSbTaskInfo_Vtbl {
        unsafe extern "system" fn TargetId<Impl: ITsSbTaskInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pname: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TargetId() {
                ::core::result::Result::Ok(ok__) => {
                    *pname = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn StartTime<Impl: ITsSbTaskInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pstarttime: *mut super::super::Foundation::FILETIME) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).StartTime() {
                ::core::result::Result::Ok(ok__) => {
                    *pstarttime = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EndTime<Impl: ITsSbTaskInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pendtime: *mut super::super::Foundation::FILETIME) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EndTime() {
                ::core::result::Result::Ok(ok__) => {
                    *pendtime = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Deadline<Impl: ITsSbTaskInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdeadline: *mut super::super::Foundation::FILETIME) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Deadline() {
                ::core::result::Result::Ok(ok__) => {
                    *pdeadline = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Identifier<Impl: ITsSbTaskInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pidentifier: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Identifier() {
                ::core::result::Result::Ok(ok__) => {
                    *pidentifier = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Label<Impl: ITsSbTaskInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plabel: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Label() {
                ::core::result::Result::Ok(ok__) => {
                    *plabel = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Context<Impl: ITsSbTaskInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcontext: *mut *mut super::Com::SAFEARRAY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Context() {
                ::core::result::Result::Ok(ok__) => {
                    *pcontext = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Plugin<Impl: ITsSbTaskInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pplugin: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Plugin() {
                ::core::result::Result::Ok(ok__) => {
                    *pplugin = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Status<Impl: ITsSbTaskInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pstatus: *mut RDV_TASK_STATUS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Status() {
                ::core::result::Result::Ok(ok__) => {
                    *pstatus = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            TargetId: TargetId::<Impl, IMPL_OFFSET>,
            StartTime: StartTime::<Impl, IMPL_OFFSET>,
            EndTime: EndTime::<Impl, IMPL_OFFSET>,
            Deadline: Deadline::<Impl, IMPL_OFFSET>,
            Identifier: Identifier::<Impl, IMPL_OFFSET>,
            Label: Label::<Impl, IMPL_OFFSET>,
            Context: Context::<Impl, IMPL_OFFSET>,
            Plugin: Plugin::<Impl, IMPL_OFFSET>,
            Status: Status::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITsSbTaskInfo as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
pub trait ITsSbTaskPlugin_Impl: Sized + ITsSbPlugin_Impl {
    fn InitializeTaskPlugin(&mut self, pitssbtaskpluginnotifysink: &::core::option::Option<ITsSbTaskPluginNotifySink>) -> ::windows::core::Result<()>;
    fn SetTaskQueue(&mut self, pszhostname: &super::super::Foundation::BSTR, sbtaskinfosize: u32, pitssbtaskinfo: *const ::core::option::Option<ITsSbTaskInfo>) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
impl ITsSbTaskPlugin_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITsSbTaskPlugin_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITsSbTaskPlugin_Vtbl {
        unsafe extern "system" fn InitializeTaskPlugin<Impl: ITsSbTaskPlugin_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pitssbtaskpluginnotifysink: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).InitializeTaskPlugin(::core::mem::transmute(&pitssbtaskpluginnotifysink)).into()
        }
        unsafe extern "system" fn SetTaskQueue<Impl: ITsSbTaskPlugin_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszhostname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, sbtaskinfosize: u32, pitssbtaskinfo: *const ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetTaskQueue(::core::mem::transmute_copy(&pszhostname), ::core::mem::transmute_copy(&sbtaskinfosize), ::core::mem::transmute_copy(&pitssbtaskinfo)).into()
        }
        Self {
            base: ITsSbPlugin_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            InitializeTaskPlugin: InitializeTaskPlugin::<Impl, IMPL_OFFSET>,
            SetTaskQueue: SetTaskQueue::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITsSbTaskPlugin as ::windows::core::Interface>::IID || iid == &<ITsSbPlugin as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub trait ITsSbTaskPluginNotifySink_Impl: Sized + ITsSbBaseNotifySink_Impl {
    fn OnSetTaskTime(&mut self, sztargetname: &super::super::Foundation::BSTR, taskstarttime: &super::super::Foundation::FILETIME, taskendtime: &super::super::Foundation::FILETIME, taskdeadline: &super::super::Foundation::FILETIME, sztasklabel: &super::super::Foundation::BSTR, sztaskidentifier: &super::super::Foundation::BSTR, sztaskplugin: &super::super::Foundation::BSTR, dwtaskstatus: u32, sacontext: *const super::Com::SAFEARRAY) -> ::windows::core::Result<()>;
    fn OnDeleteTaskTime(&mut self, sztargetname: &super::super::Foundation::BSTR, sztaskidentifier: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn OnUpdateTaskStatus(&mut self, sztargetname: &super::super::Foundation::BSTR, taskidentifier: &super::super::Foundation::BSTR, taskstatus: RDV_TASK_STATUS) -> ::windows::core::Result<()>;
    fn OnReportTasks(&mut self, szhostname: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ITsSbTaskPluginNotifySink_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITsSbTaskPluginNotifySink_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITsSbTaskPluginNotifySink_Vtbl {
        unsafe extern "system" fn OnSetTaskTime<Impl: ITsSbTaskPluginNotifySink_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, sztargetname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, taskstarttime: super::super::Foundation::FILETIME, taskendtime: super::super::Foundation::FILETIME, taskdeadline: super::super::Foundation::FILETIME, sztasklabel: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, sztaskidentifier: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, sztaskplugin: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, dwtaskstatus: u32, sacontext: *const super::Com::SAFEARRAY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnSetTaskTime(::core::mem::transmute_copy(&sztargetname), ::core::mem::transmute_copy(&taskstarttime), ::core::mem::transmute_copy(&taskendtime), ::core::mem::transmute_copy(&taskdeadline), ::core::mem::transmute_copy(&sztasklabel), ::core::mem::transmute_copy(&sztaskidentifier), ::core::mem::transmute_copy(&sztaskplugin), ::core::mem::transmute_copy(&dwtaskstatus), ::core::mem::transmute_copy(&sacontext)).into()
        }
        unsafe extern "system" fn OnDeleteTaskTime<Impl: ITsSbTaskPluginNotifySink_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, sztargetname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, sztaskidentifier: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnDeleteTaskTime(::core::mem::transmute_copy(&sztargetname), ::core::mem::transmute_copy(&sztaskidentifier)).into()
        }
        unsafe extern "system" fn OnUpdateTaskStatus<Impl: ITsSbTaskPluginNotifySink_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, sztargetname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, taskidentifier: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, taskstatus: RDV_TASK_STATUS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnUpdateTaskStatus(::core::mem::transmute_copy(&sztargetname), ::core::mem::transmute_copy(&taskidentifier), ::core::mem::transmute_copy(&taskstatus)).into()
        }
        unsafe extern "system" fn OnReportTasks<Impl: ITsSbTaskPluginNotifySink_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, szhostname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnReportTasks(::core::mem::transmute_copy(&szhostname)).into()
        }
        Self {
            base: ITsSbBaseNotifySink_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            OnSetTaskTime: OnSetTaskTime::<Impl, IMPL_OFFSET>,
            OnDeleteTaskTime: OnDeleteTaskTime::<Impl, IMPL_OFFSET>,
            OnUpdateTaskStatus: OnUpdateTaskStatus::<Impl, IMPL_OFFSET>,
            OnReportTasks: OnReportTasks::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITsSbTaskPluginNotifySink as ::windows::core::Interface>::IID || iid == &<ITsSbBaseNotifySink as ::windows::core::Interface>::IID
    }
}
pub trait IWRdsEnhancedFastReconnectArbitrator_Impl: Sized {
    fn GetSessionForEnhancedFastReconnect(&mut self, psessionidarray: *const i32, dwsessioncount: u32) -> ::windows::core::Result<i32>;
}
impl IWRdsEnhancedFastReconnectArbitrator_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWRdsEnhancedFastReconnectArbitrator_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWRdsEnhancedFastReconnectArbitrator_Vtbl {
        unsafe extern "system" fn GetSessionForEnhancedFastReconnect<Impl: IWRdsEnhancedFastReconnectArbitrator_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psessionidarray: *const i32, dwsessioncount: u32, presultsessionid: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetSessionForEnhancedFastReconnect(::core::mem::transmute_copy(&psessionidarray), ::core::mem::transmute_copy(&dwsessioncount)) {
                ::core::result::Result::Ok(ok__) => {
                    *presultsessionid = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            GetSessionForEnhancedFastReconnect: GetSessionForEnhancedFastReconnect::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWRdsEnhancedFastReconnectArbitrator as ::windows::core::Interface>::IID
    }
}
pub trait IWRdsGraphicsChannel_Impl: Sized {
    fn Write(&mut self, cbsize: u32, pbuffer: *const u8, pcontext: &::core::option::Option<::windows::core::IUnknown>) -> ::windows::core::Result<()>;
    fn Close(&mut self) -> ::windows::core::Result<()>;
    fn Open(&mut self, pchannelevents: &::core::option::Option<IWRdsGraphicsChannelEvents>, popencontext: &::core::option::Option<::windows::core::IUnknown>) -> ::windows::core::Result<()>;
}
impl IWRdsGraphicsChannel_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWRdsGraphicsChannel_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWRdsGraphicsChannel_Vtbl {
        unsafe extern "system" fn Write<Impl: IWRdsGraphicsChannel_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cbsize: u32, pbuffer: *const u8, pcontext: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Write(::core::mem::transmute_copy(&cbsize), ::core::mem::transmute_copy(&pbuffer), ::core::mem::transmute(&pcontext)).into()
        }
        unsafe extern "system" fn Close<Impl: IWRdsGraphicsChannel_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Close().into()
        }
        unsafe extern "system" fn Open<Impl: IWRdsGraphicsChannel_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pchannelevents: ::windows::core::RawPtr, popencontext: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Open(::core::mem::transmute(&pchannelevents), ::core::mem::transmute(&popencontext)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            Write: Write::<Impl, IMPL_OFFSET>,
            Close: Close::<Impl, IMPL_OFFSET>,
            Open: Open::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWRdsGraphicsChannel as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IWRdsGraphicsChannelEvents_Impl: Sized {
    fn OnDataReceived(&mut self, cbsize: u32, pbuffer: *const u8) -> ::windows::core::Result<()>;
    fn OnClose(&mut self) -> ::windows::core::Result<()>;
    fn OnChannelOpened(&mut self, openresult: ::windows::core::HRESULT, popencontext: &::core::option::Option<::windows::core::IUnknown>) -> ::windows::core::Result<()>;
    fn OnDataSent(&mut self, pwritecontext: &::core::option::Option<::windows::core::IUnknown>, bcancelled: super::super::Foundation::BOOL, pbuffer: *const u8, cbbuffer: u32) -> ::windows::core::Result<()>;
    fn OnMetricsUpdate(&mut self, bandwidth: u32, rtt: u32, lastsentbyteindex: u64) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IWRdsGraphicsChannelEvents_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWRdsGraphicsChannelEvents_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWRdsGraphicsChannelEvents_Vtbl {
        unsafe extern "system" fn OnDataReceived<Impl: IWRdsGraphicsChannelEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cbsize: u32, pbuffer: *const u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnDataReceived(::core::mem::transmute_copy(&cbsize), ::core::mem::transmute_copy(&pbuffer)).into()
        }
        unsafe extern "system" fn OnClose<Impl: IWRdsGraphicsChannelEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnClose().into()
        }
        unsafe extern "system" fn OnChannelOpened<Impl: IWRdsGraphicsChannelEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, openresult: ::windows::core::HRESULT, popencontext: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnChannelOpened(::core::mem::transmute_copy(&openresult), ::core::mem::transmute(&popencontext)).into()
        }
        unsafe extern "system" fn OnDataSent<Impl: IWRdsGraphicsChannelEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwritecontext: *mut ::core::ffi::c_void, bcancelled: super::super::Foundation::BOOL, pbuffer: *const u8, cbbuffer: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnDataSent(::core::mem::transmute(&pwritecontext), ::core::mem::transmute_copy(&bcancelled), ::core::mem::transmute_copy(&pbuffer), ::core::mem::transmute_copy(&cbbuffer)).into()
        }
        unsafe extern "system" fn OnMetricsUpdate<Impl: IWRdsGraphicsChannelEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bandwidth: u32, rtt: u32, lastsentbyteindex: u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnMetricsUpdate(::core::mem::transmute_copy(&bandwidth), ::core::mem::transmute_copy(&rtt), ::core::mem::transmute_copy(&lastsentbyteindex)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            OnDataReceived: OnDataReceived::<Impl, IMPL_OFFSET>,
            OnClose: OnClose::<Impl, IMPL_OFFSET>,
            OnChannelOpened: OnChannelOpened::<Impl, IMPL_OFFSET>,
            OnDataSent: OnDataSent::<Impl, IMPL_OFFSET>,
            OnMetricsUpdate: OnMetricsUpdate::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWRdsGraphicsChannelEvents as ::windows::core::Interface>::IID
    }
}
pub trait IWRdsGraphicsChannelManager_Impl: Sized {
    fn CreateChannel(&mut self, pszchannelname: *const u8, channeltype: WRdsGraphicsChannelType) -> ::windows::core::Result<IWRdsGraphicsChannel>;
}
impl IWRdsGraphicsChannelManager_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWRdsGraphicsChannelManager_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWRdsGraphicsChannelManager_Vtbl {
        unsafe extern "system" fn CreateChannel<Impl: IWRdsGraphicsChannelManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszchannelname: *const u8, channeltype: WRdsGraphicsChannelType, ppvirtualchannel: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateChannel(::core::mem::transmute_copy(&pszchannelname), ::core::mem::transmute_copy(&channeltype)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppvirtualchannel = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), CreateChannel: CreateChannel::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWRdsGraphicsChannelManager as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IWRdsProtocolConnection_Impl: Sized {
    fn GetLogonErrorRedirector(&mut self) -> ::windows::core::Result<IWRdsProtocolLogonErrorRedirector>;
    fn AcceptConnection(&mut self) -> ::windows::core::Result<()>;
    fn GetClientData(&mut self) -> ::windows::core::Result<WTS_CLIENT_DATA>;
    fn GetClientMonitorData(&mut self, pnummonitors: *mut u32, pprimarymonitor: *mut u32) -> ::windows::core::Result<()>;
    fn GetUserCredentials(&mut self) -> ::windows::core::Result<WTS_USER_CREDENTIAL>;
    fn GetLicenseConnection(&mut self) -> ::windows::core::Result<IWRdsProtocolLicenseConnection>;
    fn AuthenticateClientToSession(&mut self) -> ::windows::core::Result<WTS_SESSION_ID>;
    fn NotifySessionId(&mut self, sessionid: *const WTS_SESSION_ID, sessionhandle: super::super::Foundation::HANDLE_PTR) -> ::windows::core::Result<()>;
    fn GetInputHandles(&mut self, pkeyboardhandle: *mut super::super::Foundation::HANDLE_PTR, pmousehandle: *mut super::super::Foundation::HANDLE_PTR, pbeephandle: *mut super::super::Foundation::HANDLE_PTR) -> ::windows::core::Result<()>;
    fn GetVideoHandle(&mut self) -> ::windows::core::Result<super::super::Foundation::HANDLE_PTR>;
    fn ConnectNotify(&mut self, sessionid: u32) -> ::windows::core::Result<()>;
    fn IsUserAllowedToLogon(&mut self, sessionid: u32, usertoken: super::super::Foundation::HANDLE_PTR, pdomainname: super::super::Foundation::PWSTR, pusername: super::super::Foundation::PWSTR) -> ::windows::core::Result<()>;
    fn SessionArbitrationEnumeration(&mut self, husertoken: super::super::Foundation::HANDLE_PTR, bsinglesessionperuserenabled: super::super::Foundation::BOOL, psessionidarray: *mut u32, pdwsessionidentifiercount: *mut u32) -> ::windows::core::Result<()>;
    fn LogonNotify(&mut self, hclienttoken: super::super::Foundation::HANDLE_PTR, wszusername: super::super::Foundation::PWSTR, wszdomainname: super::super::Foundation::PWSTR, sessionid: *const WTS_SESSION_ID, pwrdsconnectionsettings: *mut WRDS_CONNECTION_SETTINGS) -> ::windows::core::Result<()>;
    fn PreDisconnect(&mut self, disconnectreason: u32) -> ::windows::core::Result<()>;
    fn DisconnectNotify(&mut self) -> ::windows::core::Result<()>;
    fn Close(&mut self) -> ::windows::core::Result<()>;
    fn GetProtocolStatus(&mut self) -> ::windows::core::Result<WTS_PROTOCOL_STATUS>;
    fn GetLastInputTime(&mut self) -> ::windows::core::Result<u64>;
    fn SetErrorInfo(&mut self, ulerror: u32) -> ::windows::core::Result<()>;
    fn CreateVirtualChannel(&mut self, szendpointname: super::super::Foundation::PSTR, bstatic: super::super::Foundation::BOOL, requestedpriority: u32) -> ::windows::core::Result<usize>;
    fn QueryProperty(&mut self, querytype: &::windows::core::GUID, ulnumentriesin: u32, ulnumentriesout: u32, ppropertyentriesin: *const WTS_PROPERTY_VALUE, ppropertyentriesout: *mut WTS_PROPERTY_VALUE) -> ::windows::core::Result<()>;
    fn GetShadowConnection(&mut self) -> ::windows::core::Result<IWRdsProtocolShadowConnection>;
    fn NotifyCommandProcessCreated(&mut self, sessionid: u32) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IWRdsProtocolConnection_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWRdsProtocolConnection_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWRdsProtocolConnection_Vtbl {
        unsafe extern "system" fn GetLogonErrorRedirector<Impl: IWRdsProtocolConnection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pplogonerrorredir: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetLogonErrorRedirector() {
                ::core::result::Result::Ok(ok__) => {
                    *pplogonerrorredir = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AcceptConnection<Impl: IWRdsProtocolConnection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AcceptConnection().into()
        }
        unsafe extern "system" fn GetClientData<Impl: IWRdsProtocolConnection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pclientdata: *mut WTS_CLIENT_DATA) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetClientData() {
                ::core::result::Result::Ok(ok__) => {
                    *pclientdata = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetClientMonitorData<Impl: IWRdsProtocolConnection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pnummonitors: *mut u32, pprimarymonitor: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetClientMonitorData(::core::mem::transmute_copy(&pnummonitors), ::core::mem::transmute_copy(&pprimarymonitor)).into()
        }
        unsafe extern "system" fn GetUserCredentials<Impl: IWRdsProtocolConnection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pusercreds: *mut WTS_USER_CREDENTIAL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetUserCredentials() {
                ::core::result::Result::Ok(ok__) => {
                    *pusercreds = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetLicenseConnection<Impl: IWRdsProtocolConnection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pplicenseconnection: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetLicenseConnection() {
                ::core::result::Result::Ok(ok__) => {
                    *pplicenseconnection = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AuthenticateClientToSession<Impl: IWRdsProtocolConnection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, sessionid: *mut WTS_SESSION_ID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AuthenticateClientToSession() {
                ::core::result::Result::Ok(ok__) => {
                    *sessionid = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn NotifySessionId<Impl: IWRdsProtocolConnection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, sessionid: *const WTS_SESSION_ID, sessionhandle: super::super::Foundation::HANDLE_PTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).NotifySessionId(::core::mem::transmute_copy(&sessionid), ::core::mem::transmute_copy(&sessionhandle)).into()
        }
        unsafe extern "system" fn GetInputHandles<Impl: IWRdsProtocolConnection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pkeyboardhandle: *mut super::super::Foundation::HANDLE_PTR, pmousehandle: *mut super::super::Foundation::HANDLE_PTR, pbeephandle: *mut super::super::Foundation::HANDLE_PTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetInputHandles(::core::mem::transmute_copy(&pkeyboardhandle), ::core::mem::transmute_copy(&pmousehandle), ::core::mem::transmute_copy(&pbeephandle)).into()
        }
        unsafe extern "system" fn GetVideoHandle<Impl: IWRdsProtocolConnection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvideohandle: *mut super::super::Foundation::HANDLE_PTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetVideoHandle() {
                ::core::result::Result::Ok(ok__) => {
                    *pvideohandle = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ConnectNotify<Impl: IWRdsProtocolConnection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, sessionid: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ConnectNotify(::core::mem::transmute_copy(&sessionid)).into()
        }
        unsafe extern "system" fn IsUserAllowedToLogon<Impl: IWRdsProtocolConnection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, sessionid: u32, usertoken: super::super::Foundation::HANDLE_PTR, pdomainname: super::super::Foundation::PWSTR, pusername: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).IsUserAllowedToLogon(::core::mem::transmute_copy(&sessionid), ::core::mem::transmute_copy(&usertoken), ::core::mem::transmute_copy(&pdomainname), ::core::mem::transmute_copy(&pusername)).into()
        }
        unsafe extern "system" fn SessionArbitrationEnumeration<Impl: IWRdsProtocolConnection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, husertoken: super::super::Foundation::HANDLE_PTR, bsinglesessionperuserenabled: super::super::Foundation::BOOL, psessionidarray: *mut u32, pdwsessionidentifiercount: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SessionArbitrationEnumeration(::core::mem::transmute_copy(&husertoken), ::core::mem::transmute_copy(&bsinglesessionperuserenabled), ::core::mem::transmute_copy(&psessionidarray), ::core::mem::transmute_copy(&pdwsessionidentifiercount)).into()
        }
        unsafe extern "system" fn LogonNotify<Impl: IWRdsProtocolConnection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hclienttoken: super::super::Foundation::HANDLE_PTR, wszusername: super::super::Foundation::PWSTR, wszdomainname: super::super::Foundation::PWSTR, sessionid: *const WTS_SESSION_ID, pwrdsconnectionsettings: *mut WRDS_CONNECTION_SETTINGS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).LogonNotify(::core::mem::transmute_copy(&hclienttoken), ::core::mem::transmute_copy(&wszusername), ::core::mem::transmute_copy(&wszdomainname), ::core::mem::transmute_copy(&sessionid), ::core::mem::transmute_copy(&pwrdsconnectionsettings)).into()
        }
        unsafe extern "system" fn PreDisconnect<Impl: IWRdsProtocolConnection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, disconnectreason: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).PreDisconnect(::core::mem::transmute_copy(&disconnectreason)).into()
        }
        unsafe extern "system" fn DisconnectNotify<Impl: IWRdsProtocolConnection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).DisconnectNotify().into()
        }
        unsafe extern "system" fn Close<Impl: IWRdsProtocolConnection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Close().into()
        }
        unsafe extern "system" fn GetProtocolStatus<Impl: IWRdsProtocolConnection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pprotocolstatus: *mut WTS_PROTOCOL_STATUS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetProtocolStatus() {
                ::core::result::Result::Ok(ok__) => {
                    *pprotocolstatus = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetLastInputTime<Impl: IWRdsProtocolConnection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plastinputtime: *mut u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetLastInputTime() {
                ::core::result::Result::Ok(ok__) => {
                    *plastinputtime = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetErrorInfo<Impl: IWRdsProtocolConnection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ulerror: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetErrorInfo(::core::mem::transmute_copy(&ulerror)).into()
        }
        unsafe extern "system" fn CreateVirtualChannel<Impl: IWRdsProtocolConnection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, szendpointname: super::super::Foundation::PSTR, bstatic: super::super::Foundation::BOOL, requestedpriority: u32, phchannel: *mut usize) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateVirtualChannel(::core::mem::transmute_copy(&szendpointname), ::core::mem::transmute_copy(&bstatic), ::core::mem::transmute_copy(&requestedpriority)) {
                ::core::result::Result::Ok(ok__) => {
                    *phchannel = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn QueryProperty<Impl: IWRdsProtocolConnection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, querytype: ::windows::core::GUID, ulnumentriesin: u32, ulnumentriesout: u32, ppropertyentriesin: *const WTS_PROPERTY_VALUE, ppropertyentriesout: *mut WTS_PROPERTY_VALUE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).QueryProperty(::core::mem::transmute_copy(&querytype), ::core::mem::transmute_copy(&ulnumentriesin), ::core::mem::transmute_copy(&ulnumentriesout), ::core::mem::transmute_copy(&ppropertyentriesin), ::core::mem::transmute_copy(&ppropertyentriesout)).into()
        }
        unsafe extern "system" fn GetShadowConnection<Impl: IWRdsProtocolConnection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppshadowconnection: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetShadowConnection() {
                ::core::result::Result::Ok(ok__) => {
                    *ppshadowconnection = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn NotifyCommandProcessCreated<Impl: IWRdsProtocolConnection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, sessionid: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).NotifyCommandProcessCreated(::core::mem::transmute_copy(&sessionid)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            GetLogonErrorRedirector: GetLogonErrorRedirector::<Impl, IMPL_OFFSET>,
            AcceptConnection: AcceptConnection::<Impl, IMPL_OFFSET>,
            GetClientData: GetClientData::<Impl, IMPL_OFFSET>,
            GetClientMonitorData: GetClientMonitorData::<Impl, IMPL_OFFSET>,
            GetUserCredentials: GetUserCredentials::<Impl, IMPL_OFFSET>,
            GetLicenseConnection: GetLicenseConnection::<Impl, IMPL_OFFSET>,
            AuthenticateClientToSession: AuthenticateClientToSession::<Impl, IMPL_OFFSET>,
            NotifySessionId: NotifySessionId::<Impl, IMPL_OFFSET>,
            GetInputHandles: GetInputHandles::<Impl, IMPL_OFFSET>,
            GetVideoHandle: GetVideoHandle::<Impl, IMPL_OFFSET>,
            ConnectNotify: ConnectNotify::<Impl, IMPL_OFFSET>,
            IsUserAllowedToLogon: IsUserAllowedToLogon::<Impl, IMPL_OFFSET>,
            SessionArbitrationEnumeration: SessionArbitrationEnumeration::<Impl, IMPL_OFFSET>,
            LogonNotify: LogonNotify::<Impl, IMPL_OFFSET>,
            PreDisconnect: PreDisconnect::<Impl, IMPL_OFFSET>,
            DisconnectNotify: DisconnectNotify::<Impl, IMPL_OFFSET>,
            Close: Close::<Impl, IMPL_OFFSET>,
            GetProtocolStatus: GetProtocolStatus::<Impl, IMPL_OFFSET>,
            GetLastInputTime: GetLastInputTime::<Impl, IMPL_OFFSET>,
            SetErrorInfo: SetErrorInfo::<Impl, IMPL_OFFSET>,
            CreateVirtualChannel: CreateVirtualChannel::<Impl, IMPL_OFFSET>,
            QueryProperty: QueryProperty::<Impl, IMPL_OFFSET>,
            GetShadowConnection: GetShadowConnection::<Impl, IMPL_OFFSET>,
            NotifyCommandProcessCreated: NotifyCommandProcessCreated::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWRdsProtocolConnection as ::windows::core::Interface>::IID
    }
}
pub trait IWRdsProtocolConnectionCallback_Impl: Sized {
    fn OnReady(&mut self) -> ::windows::core::Result<()>;
    fn BrokenConnection(&mut self, reason: u32, source: u32) -> ::windows::core::Result<()>;
    fn StopScreenUpdates(&mut self) -> ::windows::core::Result<()>;
    fn RedrawWindow(&mut self, rect: *const WTS_SMALL_RECT) -> ::windows::core::Result<()>;
    fn GetConnectionId(&mut self) -> ::windows::core::Result<u32>;
}
impl IWRdsProtocolConnectionCallback_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWRdsProtocolConnectionCallback_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWRdsProtocolConnectionCallback_Vtbl {
        unsafe extern "system" fn OnReady<Impl: IWRdsProtocolConnectionCallback_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnReady().into()
        }
        unsafe extern "system" fn BrokenConnection<Impl: IWRdsProtocolConnectionCallback_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, reason: u32, source: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).BrokenConnection(::core::mem::transmute_copy(&reason), ::core::mem::transmute_copy(&source)).into()
        }
        unsafe extern "system" fn StopScreenUpdates<Impl: IWRdsProtocolConnectionCallback_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).StopScreenUpdates().into()
        }
        unsafe extern "system" fn RedrawWindow<Impl: IWRdsProtocolConnectionCallback_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rect: *const WTS_SMALL_RECT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RedrawWindow(::core::mem::transmute_copy(&rect)).into()
        }
        unsafe extern "system" fn GetConnectionId<Impl: IWRdsProtocolConnectionCallback_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pconnectionid: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetConnectionId() {
                ::core::result::Result::Ok(ok__) => {
                    *pconnectionid = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            OnReady: OnReady::<Impl, IMPL_OFFSET>,
            BrokenConnection: BrokenConnection::<Impl, IMPL_OFFSET>,
            StopScreenUpdates: StopScreenUpdates::<Impl, IMPL_OFFSET>,
            RedrawWindow: RedrawWindow::<Impl, IMPL_OFFSET>,
            GetConnectionId: GetConnectionId::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWRdsProtocolConnectionCallback as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IWRdsProtocolConnectionSettings_Impl: Sized {
    fn SetConnectionSetting(&mut self, propertyid: &::windows::core::GUID, ppropertyentriesin: *const WTS_PROPERTY_VALUE) -> ::windows::core::Result<()>;
    fn GetConnectionSetting(&mut self, propertyid: &::windows::core::GUID) -> ::windows::core::Result<WTS_PROPERTY_VALUE>;
}
#[cfg(feature = "Win32_Foundation")]
impl IWRdsProtocolConnectionSettings_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWRdsProtocolConnectionSettings_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWRdsProtocolConnectionSettings_Vtbl {
        unsafe extern "system" fn SetConnectionSetting<Impl: IWRdsProtocolConnectionSettings_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, propertyid: ::windows::core::GUID, ppropertyentriesin: *const WTS_PROPERTY_VALUE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetConnectionSetting(::core::mem::transmute_copy(&propertyid), ::core::mem::transmute_copy(&ppropertyentriesin)).into()
        }
        unsafe extern "system" fn GetConnectionSetting<Impl: IWRdsProtocolConnectionSettings_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, propertyid: ::windows::core::GUID, ppropertyentriesout: *mut WTS_PROPERTY_VALUE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetConnectionSetting(::core::mem::transmute_copy(&propertyid)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppropertyentriesout = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            SetConnectionSetting: SetConnectionSetting::<Impl, IMPL_OFFSET>,
            GetConnectionSetting: GetConnectionSetting::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWRdsProtocolConnectionSettings as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IWRdsProtocolLicenseConnection_Impl: Sized {
    fn RequestLicensingCapabilities(&mut self, pplicensecapabilities: *mut WTS_LICENSE_CAPABILITIES, pcblicensecapabilities: *mut u32) -> ::windows::core::Result<()>;
    fn SendClientLicense(&mut self, pclientlicense: *const u8, cbclientlicense: u32) -> ::windows::core::Result<()>;
    fn RequestClientLicense(&mut self, reserve1: *const u8, reserve2: u32, ppclientlicense: *mut u8, pcbclientlicense: *mut u32) -> ::windows::core::Result<()>;
    fn ProtocolComplete(&mut self, ulcomplete: u32) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IWRdsProtocolLicenseConnection_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWRdsProtocolLicenseConnection_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWRdsProtocolLicenseConnection_Vtbl {
        unsafe extern "system" fn RequestLicensingCapabilities<Impl: IWRdsProtocolLicenseConnection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pplicensecapabilities: *mut WTS_LICENSE_CAPABILITIES, pcblicensecapabilities: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RequestLicensingCapabilities(::core::mem::transmute_copy(&pplicensecapabilities), ::core::mem::transmute_copy(&pcblicensecapabilities)).into()
        }
        unsafe extern "system" fn SendClientLicense<Impl: IWRdsProtocolLicenseConnection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pclientlicense: *const u8, cbclientlicense: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SendClientLicense(::core::mem::transmute_copy(&pclientlicense), ::core::mem::transmute_copy(&cbclientlicense)).into()
        }
        unsafe extern "system" fn RequestClientLicense<Impl: IWRdsProtocolLicenseConnection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, reserve1: *const u8, reserve2: u32, ppclientlicense: *mut u8, pcbclientlicense: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RequestClientLicense(::core::mem::transmute_copy(&reserve1), ::core::mem::transmute_copy(&reserve2), ::core::mem::transmute_copy(&ppclientlicense), ::core::mem::transmute_copy(&pcbclientlicense)).into()
        }
        unsafe extern "system" fn ProtocolComplete<Impl: IWRdsProtocolLicenseConnection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ulcomplete: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ProtocolComplete(::core::mem::transmute_copy(&ulcomplete)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            RequestLicensingCapabilities: RequestLicensingCapabilities::<Impl, IMPL_OFFSET>,
            SendClientLicense: SendClientLicense::<Impl, IMPL_OFFSET>,
            RequestClientLicense: RequestClientLicense::<Impl, IMPL_OFFSET>,
            ProtocolComplete: ProtocolComplete::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWRdsProtocolLicenseConnection as ::windows::core::Interface>::IID
    }
}
pub trait IWRdsProtocolListener_Impl: Sized {
    fn GetSettings(&mut self, wrdslistenersettinglevel: WRDS_LISTENER_SETTING_LEVEL) -> ::windows::core::Result<WRDS_LISTENER_SETTINGS>;
    fn StartListen(&mut self, pcallback: &::core::option::Option<IWRdsProtocolListenerCallback>) -> ::windows::core::Result<()>;
    fn StopListen(&mut self) -> ::windows::core::Result<()>;
}
impl IWRdsProtocolListener_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWRdsProtocolListener_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWRdsProtocolListener_Vtbl {
        unsafe extern "system" fn GetSettings<Impl: IWRdsProtocolListener_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wrdslistenersettinglevel: WRDS_LISTENER_SETTING_LEVEL, pwrdslistenersettings: *mut WRDS_LISTENER_SETTINGS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetSettings(::core::mem::transmute_copy(&wrdslistenersettinglevel)) {
                ::core::result::Result::Ok(ok__) => {
                    *pwrdslistenersettings = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn StartListen<Impl: IWRdsProtocolListener_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcallback: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).StartListen(::core::mem::transmute(&pcallback)).into()
        }
        unsafe extern "system" fn StopListen<Impl: IWRdsProtocolListener_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).StopListen().into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            GetSettings: GetSettings::<Impl, IMPL_OFFSET>,
            StartListen: StartListen::<Impl, IMPL_OFFSET>,
            StopListen: StopListen::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWRdsProtocolListener as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IWRdsProtocolListenerCallback_Impl: Sized {
    fn OnConnected(&mut self, pconnection: &::core::option::Option<IWRdsProtocolConnection>, pwrdsconnectionsettings: *const WRDS_CONNECTION_SETTINGS) -> ::windows::core::Result<IWRdsProtocolConnectionCallback>;
}
#[cfg(feature = "Win32_Foundation")]
impl IWRdsProtocolListenerCallback_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWRdsProtocolListenerCallback_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWRdsProtocolListenerCallback_Vtbl {
        unsafe extern "system" fn OnConnected<Impl: IWRdsProtocolListenerCallback_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pconnection: ::windows::core::RawPtr, pwrdsconnectionsettings: *const WRDS_CONNECTION_SETTINGS, pcallback: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OnConnected(::core::mem::transmute(&pconnection), ::core::mem::transmute_copy(&pwrdsconnectionsettings)) {
                ::core::result::Result::Ok(ok__) => {
                    *pcallback = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), OnConnected: OnConnected::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWRdsProtocolListenerCallback as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IWRdsProtocolLogonErrorRedirector_Impl: Sized {
    fn OnBeginPainting(&mut self) -> ::windows::core::Result<()>;
    fn RedirectStatus(&mut self, pszmessage: super::super::Foundation::PWSTR) -> ::windows::core::Result<WTS_LOGON_ERROR_REDIRECTOR_RESPONSE>;
    fn RedirectMessage(&mut self, pszcaption: super::super::Foundation::PWSTR, pszmessage: super::super::Foundation::PWSTR, utype: u32) -> ::windows::core::Result<WTS_LOGON_ERROR_REDIRECTOR_RESPONSE>;
    fn RedirectLogonError(&mut self, ntsstatus: i32, ntssubstatus: i32, pszcaption: super::super::Foundation::PWSTR, pszmessage: super::super::Foundation::PWSTR, utype: u32) -> ::windows::core::Result<WTS_LOGON_ERROR_REDIRECTOR_RESPONSE>;
}
#[cfg(feature = "Win32_Foundation")]
impl IWRdsProtocolLogonErrorRedirector_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWRdsProtocolLogonErrorRedirector_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWRdsProtocolLogonErrorRedirector_Vtbl {
        unsafe extern "system" fn OnBeginPainting<Impl: IWRdsProtocolLogonErrorRedirector_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnBeginPainting().into()
        }
        unsafe extern "system" fn RedirectStatus<Impl: IWRdsProtocolLogonErrorRedirector_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszmessage: super::super::Foundation::PWSTR, presponse: *mut WTS_LOGON_ERROR_REDIRECTOR_RESPONSE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RedirectStatus(::core::mem::transmute_copy(&pszmessage)) {
                ::core::result::Result::Ok(ok__) => {
                    *presponse = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RedirectMessage<Impl: IWRdsProtocolLogonErrorRedirector_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszcaption: super::super::Foundation::PWSTR, pszmessage: super::super::Foundation::PWSTR, utype: u32, presponse: *mut WTS_LOGON_ERROR_REDIRECTOR_RESPONSE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RedirectMessage(::core::mem::transmute_copy(&pszcaption), ::core::mem::transmute_copy(&pszmessage), ::core::mem::transmute_copy(&utype)) {
                ::core::result::Result::Ok(ok__) => {
                    *presponse = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RedirectLogonError<Impl: IWRdsProtocolLogonErrorRedirector_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ntsstatus: i32, ntssubstatus: i32, pszcaption: super::super::Foundation::PWSTR, pszmessage: super::super::Foundation::PWSTR, utype: u32, presponse: *mut WTS_LOGON_ERROR_REDIRECTOR_RESPONSE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RedirectLogonError(::core::mem::transmute_copy(&ntsstatus), ::core::mem::transmute_copy(&ntssubstatus), ::core::mem::transmute_copy(&pszcaption), ::core::mem::transmute_copy(&pszmessage), ::core::mem::transmute_copy(&utype)) {
                ::core::result::Result::Ok(ok__) => {
                    *presponse = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            OnBeginPainting: OnBeginPainting::<Impl, IMPL_OFFSET>,
            RedirectStatus: RedirectStatus::<Impl, IMPL_OFFSET>,
            RedirectMessage: RedirectMessage::<Impl, IMPL_OFFSET>,
            RedirectLogonError: RedirectLogonError::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWRdsProtocolLogonErrorRedirector as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IWRdsProtocolManager_Impl: Sized {
    fn Initialize(&mut self, piwrdssettings: &::core::option::Option<IWRdsProtocolSettings>, pwrdssettings: *const WRDS_SETTINGS) -> ::windows::core::Result<()>;
    fn CreateListener(&mut self, wszlistenername: super::super::Foundation::PWSTR) -> ::windows::core::Result<IWRdsProtocolListener>;
    fn NotifyServiceStateChange(&mut self, ptsservicestatechange: *const WTS_SERVICE_STATE) -> ::windows::core::Result<()>;
    fn NotifySessionOfServiceStart(&mut self, sessionid: *const WTS_SESSION_ID) -> ::windows::core::Result<()>;
    fn NotifySessionOfServiceStop(&mut self, sessionid: *const WTS_SESSION_ID) -> ::windows::core::Result<()>;
    fn NotifySessionStateChange(&mut self, sessionid: *const WTS_SESSION_ID, eventid: u32) -> ::windows::core::Result<()>;
    fn NotifySettingsChange(&mut self, pwrdssettings: *const WRDS_SETTINGS) -> ::windows::core::Result<()>;
    fn Uninitialize(&mut self) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IWRdsProtocolManager_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWRdsProtocolManager_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWRdsProtocolManager_Vtbl {
        unsafe extern "system" fn Initialize<Impl: IWRdsProtocolManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, piwrdssettings: ::windows::core::RawPtr, pwrdssettings: *const WRDS_SETTINGS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Initialize(::core::mem::transmute(&piwrdssettings), ::core::mem::transmute_copy(&pwrdssettings)).into()
        }
        unsafe extern "system" fn CreateListener<Impl: IWRdsProtocolManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wszlistenername: super::super::Foundation::PWSTR, pprotocollistener: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateListener(::core::mem::transmute_copy(&wszlistenername)) {
                ::core::result::Result::Ok(ok__) => {
                    *pprotocollistener = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn NotifyServiceStateChange<Impl: IWRdsProtocolManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ptsservicestatechange: *const WTS_SERVICE_STATE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).NotifyServiceStateChange(::core::mem::transmute_copy(&ptsservicestatechange)).into()
        }
        unsafe extern "system" fn NotifySessionOfServiceStart<Impl: IWRdsProtocolManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, sessionid: *const WTS_SESSION_ID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).NotifySessionOfServiceStart(::core::mem::transmute_copy(&sessionid)).into()
        }
        unsafe extern "system" fn NotifySessionOfServiceStop<Impl: IWRdsProtocolManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, sessionid: *const WTS_SESSION_ID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).NotifySessionOfServiceStop(::core::mem::transmute_copy(&sessionid)).into()
        }
        unsafe extern "system" fn NotifySessionStateChange<Impl: IWRdsProtocolManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, sessionid: *const WTS_SESSION_ID, eventid: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).NotifySessionStateChange(::core::mem::transmute_copy(&sessionid), ::core::mem::transmute_copy(&eventid)).into()
        }
        unsafe extern "system" fn NotifySettingsChange<Impl: IWRdsProtocolManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwrdssettings: *const WRDS_SETTINGS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).NotifySettingsChange(::core::mem::transmute_copy(&pwrdssettings)).into()
        }
        unsafe extern "system" fn Uninitialize<Impl: IWRdsProtocolManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Uninitialize().into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            Initialize: Initialize::<Impl, IMPL_OFFSET>,
            CreateListener: CreateListener::<Impl, IMPL_OFFSET>,
            NotifyServiceStateChange: NotifyServiceStateChange::<Impl, IMPL_OFFSET>,
            NotifySessionOfServiceStart: NotifySessionOfServiceStart::<Impl, IMPL_OFFSET>,
            NotifySessionOfServiceStop: NotifySessionOfServiceStop::<Impl, IMPL_OFFSET>,
            NotifySessionStateChange: NotifySessionStateChange::<Impl, IMPL_OFFSET>,
            NotifySettingsChange: NotifySettingsChange::<Impl, IMPL_OFFSET>,
            Uninitialize: Uninitialize::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWRdsProtocolManager as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IWRdsProtocolSettings_Impl: Sized {
    fn GetSettings(&mut self, wrdssettingtype: WRDS_SETTING_TYPE, wrdssettinglevel: WRDS_SETTING_LEVEL) -> ::windows::core::Result<WRDS_SETTINGS>;
    fn MergeSettings(&mut self, pwrdssettings: *const WRDS_SETTINGS, wrdsconnectionsettinglevel: WRDS_CONNECTION_SETTING_LEVEL, pwrdsconnectionsettings: *mut WRDS_CONNECTION_SETTINGS) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IWRdsProtocolSettings_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWRdsProtocolSettings_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWRdsProtocolSettings_Vtbl {
        unsafe extern "system" fn GetSettings<Impl: IWRdsProtocolSettings_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wrdssettingtype: WRDS_SETTING_TYPE, wrdssettinglevel: WRDS_SETTING_LEVEL, pwrdssettings: *mut WRDS_SETTINGS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetSettings(::core::mem::transmute_copy(&wrdssettingtype), ::core::mem::transmute_copy(&wrdssettinglevel)) {
                ::core::result::Result::Ok(ok__) => {
                    *pwrdssettings = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MergeSettings<Impl: IWRdsProtocolSettings_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwrdssettings: *const WRDS_SETTINGS, wrdsconnectionsettinglevel: WRDS_CONNECTION_SETTING_LEVEL, pwrdsconnectionsettings: *mut WRDS_CONNECTION_SETTINGS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).MergeSettings(::core::mem::transmute_copy(&pwrdssettings), ::core::mem::transmute_copy(&wrdsconnectionsettinglevel), ::core::mem::transmute_copy(&pwrdsconnectionsettings)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            GetSettings: GetSettings::<Impl, IMPL_OFFSET>,
            MergeSettings: MergeSettings::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWRdsProtocolSettings as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IWRdsProtocolShadowCallback_Impl: Sized {
    fn StopShadow(&mut self) -> ::windows::core::Result<()>;
    fn InvokeTargetShadow(&mut self, ptargetservername: super::super::Foundation::PWSTR, targetsessionid: u32, pparam1: *const u8, param1size: u32, pparam2: *const u8, param2size: u32, pparam3: *const u8, param3size: u32, pparam4: *const u8, param4size: u32, pclientname: super::super::Foundation::PWSTR) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IWRdsProtocolShadowCallback_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWRdsProtocolShadowCallback_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWRdsProtocolShadowCallback_Vtbl {
        unsafe extern "system" fn StopShadow<Impl: IWRdsProtocolShadowCallback_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).StopShadow().into()
        }
        unsafe extern "system" fn InvokeTargetShadow<Impl: IWRdsProtocolShadowCallback_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ptargetservername: super::super::Foundation::PWSTR, targetsessionid: u32, pparam1: *const u8, param1size: u32, pparam2: *const u8, param2size: u32, pparam3: *const u8, param3size: u32, pparam4: *const u8, param4size: u32, pclientname: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this)
                .InvokeTargetShadow(::core::mem::transmute_copy(&ptargetservername), ::core::mem::transmute_copy(&targetsessionid), ::core::mem::transmute_copy(&pparam1), ::core::mem::transmute_copy(&param1size), ::core::mem::transmute_copy(&pparam2), ::core::mem::transmute_copy(&param2size), ::core::mem::transmute_copy(&pparam3), ::core::mem::transmute_copy(&param3size), ::core::mem::transmute_copy(&pparam4), ::core::mem::transmute_copy(&param4size), ::core::mem::transmute_copy(&pclientname))
                .into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            StopShadow: StopShadow::<Impl, IMPL_OFFSET>,
            InvokeTargetShadow: InvokeTargetShadow::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWRdsProtocolShadowCallback as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IWRdsProtocolShadowConnection_Impl: Sized {
    fn Start(&mut self, ptargetservername: super::super::Foundation::PWSTR, targetsessionid: u32, hotkeyvk: u8, hotkeymodifiers: u16, pshadowcallback: &::core::option::Option<IWRdsProtocolShadowCallback>) -> ::windows::core::Result<()>;
    fn Stop(&mut self) -> ::windows::core::Result<()>;
    fn DoTarget(&mut self, pparam1: *const u8, param1size: u32, pparam2: *const u8, param2size: u32, pparam3: *const u8, param3size: u32, pparam4: *const u8, param4size: u32, pclientname: super::super::Foundation::PWSTR) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IWRdsProtocolShadowConnection_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWRdsProtocolShadowConnection_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWRdsProtocolShadowConnection_Vtbl {
        unsafe extern "system" fn Start<Impl: IWRdsProtocolShadowConnection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ptargetservername: super::super::Foundation::PWSTR, targetsessionid: u32, hotkeyvk: u8, hotkeymodifiers: u16, pshadowcallback: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Start(::core::mem::transmute_copy(&ptargetservername), ::core::mem::transmute_copy(&targetsessionid), ::core::mem::transmute_copy(&hotkeyvk), ::core::mem::transmute_copy(&hotkeymodifiers), ::core::mem::transmute(&pshadowcallback)).into()
        }
        unsafe extern "system" fn Stop<Impl: IWRdsProtocolShadowConnection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Stop().into()
        }
        unsafe extern "system" fn DoTarget<Impl: IWRdsProtocolShadowConnection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pparam1: *const u8, param1size: u32, pparam2: *const u8, param2size: u32, pparam3: *const u8, param3size: u32, pparam4: *const u8, param4size: u32, pclientname: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).DoTarget(::core::mem::transmute_copy(&pparam1), ::core::mem::transmute_copy(&param1size), ::core::mem::transmute_copy(&pparam2), ::core::mem::transmute_copy(&param2size), ::core::mem::transmute_copy(&pparam3), ::core::mem::transmute_copy(&param3size), ::core::mem::transmute_copy(&pparam4), ::core::mem::transmute_copy(&param4size), ::core::mem::transmute_copy(&pclientname)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            Start: Start::<Impl, IMPL_OFFSET>,
            Stop: Stop::<Impl, IMPL_OFFSET>,
            DoTarget: DoTarget::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWRdsProtocolShadowConnection as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IWRdsWddmIddProps_Impl: Sized {
    fn GetHardwareId(&mut self, pdisplaydriverhardwareid: super::super::Foundation::PWSTR, count: u32) -> ::windows::core::Result<()>;
    fn OnDriverLoad(&mut self, sessionid: u32, driverhandle: super::super::Foundation::HANDLE_PTR) -> ::windows::core::Result<()>;
    fn OnDriverUnload(&mut self, sessionid: u32) -> ::windows::core::Result<()>;
    fn EnableWddmIdd(&mut self, enabled: super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IWRdsWddmIddProps_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWRdsWddmIddProps_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWRdsWddmIddProps_Vtbl {
        unsafe extern "system" fn GetHardwareId<Impl: IWRdsWddmIddProps_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdisplaydriverhardwareid: super::super::Foundation::PWSTR, count: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetHardwareId(::core::mem::transmute_copy(&pdisplaydriverhardwareid), ::core::mem::transmute_copy(&count)).into()
        }
        unsafe extern "system" fn OnDriverLoad<Impl: IWRdsWddmIddProps_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, sessionid: u32, driverhandle: super::super::Foundation::HANDLE_PTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnDriverLoad(::core::mem::transmute_copy(&sessionid), ::core::mem::transmute_copy(&driverhandle)).into()
        }
        unsafe extern "system" fn OnDriverUnload<Impl: IWRdsWddmIddProps_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, sessionid: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnDriverUnload(::core::mem::transmute_copy(&sessionid)).into()
        }
        unsafe extern "system" fn EnableWddmIdd<Impl: IWRdsWddmIddProps_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, enabled: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).EnableWddmIdd(::core::mem::transmute_copy(&enabled)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            GetHardwareId: GetHardwareId::<Impl, IMPL_OFFSET>,
            OnDriverLoad: OnDriverLoad::<Impl, IMPL_OFFSET>,
            OnDriverUnload: OnDriverUnload::<Impl, IMPL_OFFSET>,
            EnableWddmIdd: EnableWddmIdd::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWRdsWddmIddProps as ::windows::core::Interface>::IID
    }
}
pub trait IWTSBitmapRenderService_Impl: Sized {
    fn GetMappedRenderer(&mut self, mappingid: u64, pmappedrenderercallback: &::core::option::Option<IWTSBitmapRendererCallback>) -> ::windows::core::Result<IWTSBitmapRenderer>;
}
impl IWTSBitmapRenderService_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWTSBitmapRenderService_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWTSBitmapRenderService_Vtbl {
        unsafe extern "system" fn GetMappedRenderer<Impl: IWTSBitmapRenderService_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, mappingid: u64, pmappedrenderercallback: ::windows::core::RawPtr, ppmappedrenderer: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetMappedRenderer(::core::mem::transmute_copy(&mappingid), ::core::mem::transmute(&pmappedrenderercallback)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppmappedrenderer = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), GetMappedRenderer: GetMappedRenderer::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWTSBitmapRenderService as ::windows::core::Interface>::IID
    }
}
pub trait IWTSBitmapRenderer_Impl: Sized {
    fn Render(&mut self, imageformat: &::windows::core::GUID, dwwidth: u32, dwheight: u32, cbstride: i32, cbimagebuffer: u32, pimagebuffer: *const u8) -> ::windows::core::Result<()>;
    fn GetRendererStatistics(&mut self) -> ::windows::core::Result<BITMAP_RENDERER_STATISTICS>;
    fn RemoveMapping(&mut self) -> ::windows::core::Result<()>;
}
impl IWTSBitmapRenderer_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWTSBitmapRenderer_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWTSBitmapRenderer_Vtbl {
        unsafe extern "system" fn Render<Impl: IWTSBitmapRenderer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, imageformat: ::windows::core::GUID, dwwidth: u32, dwheight: u32, cbstride: i32, cbimagebuffer: u32, pimagebuffer: *const u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Render(::core::mem::transmute_copy(&imageformat), ::core::mem::transmute_copy(&dwwidth), ::core::mem::transmute_copy(&dwheight), ::core::mem::transmute_copy(&cbstride), ::core::mem::transmute_copy(&cbimagebuffer), ::core::mem::transmute_copy(&pimagebuffer)).into()
        }
        unsafe extern "system" fn GetRendererStatistics<Impl: IWTSBitmapRenderer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pstatistics: *mut BITMAP_RENDERER_STATISTICS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetRendererStatistics() {
                ::core::result::Result::Ok(ok__) => {
                    *pstatistics = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveMapping<Impl: IWTSBitmapRenderer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveMapping().into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            Render: Render::<Impl, IMPL_OFFSET>,
            GetRendererStatistics: GetRendererStatistics::<Impl, IMPL_OFFSET>,
            RemoveMapping: RemoveMapping::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWTSBitmapRenderer as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IWTSBitmapRendererCallback_Impl: Sized {
    fn OnTargetSizeChanged(&mut self, rcnewsize: &super::super::Foundation::RECT) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IWTSBitmapRendererCallback_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWTSBitmapRendererCallback_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWTSBitmapRendererCallback_Vtbl {
        unsafe extern "system" fn OnTargetSizeChanged<Impl: IWTSBitmapRendererCallback_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rcnewsize: super::super::Foundation::RECT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnTargetSizeChanged(::core::mem::transmute_copy(&rcnewsize)).into()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), OnTargetSizeChanged: OnTargetSizeChanged::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWTSBitmapRendererCallback as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com_StructuredStorage")]
pub trait IWTSListener_Impl: Sized {
    fn GetConfiguration(&mut self) -> ::windows::core::Result<super::Com::StructuredStorage::IPropertyBag>;
}
#[cfg(feature = "Win32_System_Com_StructuredStorage")]
impl IWTSListener_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWTSListener_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWTSListener_Vtbl {
        unsafe extern "system" fn GetConfiguration<Impl: IWTSListener_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pppropertybag: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetConfiguration() {
                ::core::result::Result::Ok(ok__) => {
                    *pppropertybag = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), GetConfiguration: GetConfiguration::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWTSListener as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IWTSListenerCallback_Impl: Sized {
    fn OnNewChannelConnection(&mut self, pchannel: &::core::option::Option<IWTSVirtualChannel>, data: &super::super::Foundation::BSTR, pbaccept: *mut super::super::Foundation::BOOL, ppcallback: *mut ::core::option::Option<IWTSVirtualChannelCallback>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IWTSListenerCallback_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWTSListenerCallback_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWTSListenerCallback_Vtbl {
        unsafe extern "system" fn OnNewChannelConnection<Impl: IWTSListenerCallback_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pchannel: ::windows::core::RawPtr, data: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pbaccept: *mut super::super::Foundation::BOOL, ppcallback: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnNewChannelConnection(::core::mem::transmute(&pchannel), ::core::mem::transmute_copy(&data), ::core::mem::transmute_copy(&pbaccept), ::core::mem::transmute_copy(&ppcallback)).into()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), OnNewChannelConnection: OnNewChannelConnection::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWTSListenerCallback as ::windows::core::Interface>::IID
    }
}
pub trait IWTSPlugin_Impl: Sized {
    fn Initialize(&mut self, pchannelmgr: &::core::option::Option<IWTSVirtualChannelManager>) -> ::windows::core::Result<()>;
    fn Connected(&mut self) -> ::windows::core::Result<()>;
    fn Disconnected(&mut self, dwdisconnectcode: u32) -> ::windows::core::Result<()>;
    fn Terminated(&mut self) -> ::windows::core::Result<()>;
}
impl IWTSPlugin_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWTSPlugin_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWTSPlugin_Vtbl {
        unsafe extern "system" fn Initialize<Impl: IWTSPlugin_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pchannelmgr: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Initialize(::core::mem::transmute(&pchannelmgr)).into()
        }
        unsafe extern "system" fn Connected<Impl: IWTSPlugin_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Connected().into()
        }
        unsafe extern "system" fn Disconnected<Impl: IWTSPlugin_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwdisconnectcode: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Disconnected(::core::mem::transmute_copy(&dwdisconnectcode)).into()
        }
        unsafe extern "system" fn Terminated<Impl: IWTSPlugin_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Terminated().into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            Initialize: Initialize::<Impl, IMPL_OFFSET>,
            Connected: Connected::<Impl, IMPL_OFFSET>,
            Disconnected: Disconnected::<Impl, IMPL_OFFSET>,
            Terminated: Terminated::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWTSPlugin as ::windows::core::Interface>::IID
    }
}
pub trait IWTSPluginServiceProvider_Impl: Sized {
    fn GetService(&mut self, serviceid: &::windows::core::GUID) -> ::windows::core::Result<::windows::core::IUnknown>;
}
impl IWTSPluginServiceProvider_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWTSPluginServiceProvider_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWTSPluginServiceProvider_Vtbl {
        unsafe extern "system" fn GetService<Impl: IWTSPluginServiceProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, serviceid: ::windows::core::GUID, ppunkobject: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetService(::core::mem::transmute_copy(&serviceid)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppunkobject = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), GetService: GetService::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWTSPluginServiceProvider as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IWTSProtocolConnection_Impl: Sized {
    fn GetLogonErrorRedirector(&mut self) -> ::windows::core::Result<IWTSProtocolLogonErrorRedirector>;
    fn SendPolicyData(&mut self, ppolicydata: *const WTS_POLICY_DATA) -> ::windows::core::Result<()>;
    fn AcceptConnection(&mut self) -> ::windows::core::Result<()>;
    fn GetClientData(&mut self) -> ::windows::core::Result<WTS_CLIENT_DATA>;
    fn GetUserCredentials(&mut self) -> ::windows::core::Result<WTS_USER_CREDENTIAL>;
    fn GetLicenseConnection(&mut self) -> ::windows::core::Result<IWTSProtocolLicenseConnection>;
    fn AuthenticateClientToSession(&mut self) -> ::windows::core::Result<WTS_SESSION_ID>;
    fn NotifySessionId(&mut self, sessionid: *const WTS_SESSION_ID) -> ::windows::core::Result<()>;
    fn GetProtocolHandles(&mut self, pkeyboardhandle: *mut super::super::Foundation::HANDLE_PTR, pmousehandle: *mut super::super::Foundation::HANDLE_PTR, pbeephandle: *mut super::super::Foundation::HANDLE_PTR, pvideohandle: *mut super::super::Foundation::HANDLE_PTR) -> ::windows::core::Result<()>;
    fn ConnectNotify(&mut self, sessionid: u32) -> ::windows::core::Result<()>;
    fn IsUserAllowedToLogon(&mut self, sessionid: u32, usertoken: super::super::Foundation::HANDLE_PTR, pdomainname: super::super::Foundation::PWSTR, pusername: super::super::Foundation::PWSTR) -> ::windows::core::Result<()>;
    fn SessionArbitrationEnumeration(&mut self, husertoken: super::super::Foundation::HANDLE_PTR, bsinglesessionperuserenabled: super::super::Foundation::BOOL, psessionidarray: *mut u32, pdwsessionidentifiercount: *mut u32) -> ::windows::core::Result<()>;
    fn LogonNotify(&mut self, hclienttoken: super::super::Foundation::HANDLE_PTR, wszusername: super::super::Foundation::PWSTR, wszdomainname: super::super::Foundation::PWSTR, sessionid: *const WTS_SESSION_ID) -> ::windows::core::Result<()>;
    fn GetUserData(&mut self, ppolicydata: *const WTS_POLICY_DATA, pclientdata: *mut WTS_USER_DATA) -> ::windows::core::Result<()>;
    fn DisconnectNotify(&mut self) -> ::windows::core::Result<()>;
    fn Close(&mut self) -> ::windows::core::Result<()>;
    fn GetProtocolStatus(&mut self) -> ::windows::core::Result<WTS_PROTOCOL_STATUS>;
    fn GetLastInputTime(&mut self) -> ::windows::core::Result<u64>;
    fn SetErrorInfo(&mut self, ulerror: u32) -> ::windows::core::Result<()>;
    fn SendBeep(&mut self, frequency: u32, duration: u32) -> ::windows::core::Result<()>;
    fn CreateVirtualChannel(&mut self, szendpointname: super::super::Foundation::PSTR, bstatic: super::super::Foundation::BOOL, requestedpriority: u32) -> ::windows::core::Result<usize>;
    fn QueryProperty(&mut self, querytype: &::windows::core::GUID, ulnumentriesin: u32, ulnumentriesout: u32, ppropertyentriesin: *const WTS_PROPERTY_VALUE, ppropertyentriesout: *mut WTS_PROPERTY_VALUE) -> ::windows::core::Result<()>;
    fn GetShadowConnection(&mut self) -> ::windows::core::Result<IWTSProtocolShadowConnection>;
}
#[cfg(feature = "Win32_Foundation")]
impl IWTSProtocolConnection_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWTSProtocolConnection_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWTSProtocolConnection_Vtbl {
        unsafe extern "system" fn GetLogonErrorRedirector<Impl: IWTSProtocolConnection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pplogonerrorredir: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetLogonErrorRedirector() {
                ::core::result::Result::Ok(ok__) => {
                    *pplogonerrorredir = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SendPolicyData<Impl: IWTSProtocolConnection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppolicydata: *const WTS_POLICY_DATA) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SendPolicyData(::core::mem::transmute_copy(&ppolicydata)).into()
        }
        unsafe extern "system" fn AcceptConnection<Impl: IWTSProtocolConnection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AcceptConnection().into()
        }
        unsafe extern "system" fn GetClientData<Impl: IWTSProtocolConnection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pclientdata: *mut WTS_CLIENT_DATA) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetClientData() {
                ::core::result::Result::Ok(ok__) => {
                    *pclientdata = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetUserCredentials<Impl: IWTSProtocolConnection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pusercreds: *mut WTS_USER_CREDENTIAL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetUserCredentials() {
                ::core::result::Result::Ok(ok__) => {
                    *pusercreds = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetLicenseConnection<Impl: IWTSProtocolConnection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pplicenseconnection: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetLicenseConnection() {
                ::core::result::Result::Ok(ok__) => {
                    *pplicenseconnection = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AuthenticateClientToSession<Impl: IWTSProtocolConnection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, sessionid: *mut WTS_SESSION_ID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AuthenticateClientToSession() {
                ::core::result::Result::Ok(ok__) => {
                    *sessionid = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn NotifySessionId<Impl: IWTSProtocolConnection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, sessionid: *const WTS_SESSION_ID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).NotifySessionId(::core::mem::transmute_copy(&sessionid)).into()
        }
        unsafe extern "system" fn GetProtocolHandles<Impl: IWTSProtocolConnection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pkeyboardhandle: *mut super::super::Foundation::HANDLE_PTR, pmousehandle: *mut super::super::Foundation::HANDLE_PTR, pbeephandle: *mut super::super::Foundation::HANDLE_PTR, pvideohandle: *mut super::super::Foundation::HANDLE_PTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetProtocolHandles(::core::mem::transmute_copy(&pkeyboardhandle), ::core::mem::transmute_copy(&pmousehandle), ::core::mem::transmute_copy(&pbeephandle), ::core::mem::transmute_copy(&pvideohandle)).into()
        }
        unsafe extern "system" fn ConnectNotify<Impl: IWTSProtocolConnection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, sessionid: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ConnectNotify(::core::mem::transmute_copy(&sessionid)).into()
        }
        unsafe extern "system" fn IsUserAllowedToLogon<Impl: IWTSProtocolConnection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, sessionid: u32, usertoken: super::super::Foundation::HANDLE_PTR, pdomainname: super::super::Foundation::PWSTR, pusername: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).IsUserAllowedToLogon(::core::mem::transmute_copy(&sessionid), ::core::mem::transmute_copy(&usertoken), ::core::mem::transmute_copy(&pdomainname), ::core::mem::transmute_copy(&pusername)).into()
        }
        unsafe extern "system" fn SessionArbitrationEnumeration<Impl: IWTSProtocolConnection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, husertoken: super::super::Foundation::HANDLE_PTR, bsinglesessionperuserenabled: super::super::Foundation::BOOL, psessionidarray: *mut u32, pdwsessionidentifiercount: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SessionArbitrationEnumeration(::core::mem::transmute_copy(&husertoken), ::core::mem::transmute_copy(&bsinglesessionperuserenabled), ::core::mem::transmute_copy(&psessionidarray), ::core::mem::transmute_copy(&pdwsessionidentifiercount)).into()
        }
        unsafe extern "system" fn LogonNotify<Impl: IWTSProtocolConnection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hclienttoken: super::super::Foundation::HANDLE_PTR, wszusername: super::super::Foundation::PWSTR, wszdomainname: super::super::Foundation::PWSTR, sessionid: *const WTS_SESSION_ID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).LogonNotify(::core::mem::transmute_copy(&hclienttoken), ::core::mem::transmute_copy(&wszusername), ::core::mem::transmute_copy(&wszdomainname), ::core::mem::transmute_copy(&sessionid)).into()
        }
        unsafe extern "system" fn GetUserData<Impl: IWTSProtocolConnection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppolicydata: *const WTS_POLICY_DATA, pclientdata: *mut WTS_USER_DATA) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetUserData(::core::mem::transmute_copy(&ppolicydata), ::core::mem::transmute_copy(&pclientdata)).into()
        }
        unsafe extern "system" fn DisconnectNotify<Impl: IWTSProtocolConnection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).DisconnectNotify().into()
        }
        unsafe extern "system" fn Close<Impl: IWTSProtocolConnection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Close().into()
        }
        unsafe extern "system" fn GetProtocolStatus<Impl: IWTSProtocolConnection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pprotocolstatus: *mut WTS_PROTOCOL_STATUS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetProtocolStatus() {
                ::core::result::Result::Ok(ok__) => {
                    *pprotocolstatus = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetLastInputTime<Impl: IWTSProtocolConnection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plastinputtime: *mut u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetLastInputTime() {
                ::core::result::Result::Ok(ok__) => {
                    *plastinputtime = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetErrorInfo<Impl: IWTSProtocolConnection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ulerror: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetErrorInfo(::core::mem::transmute_copy(&ulerror)).into()
        }
        unsafe extern "system" fn SendBeep<Impl: IWTSProtocolConnection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, frequency: u32, duration: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SendBeep(::core::mem::transmute_copy(&frequency), ::core::mem::transmute_copy(&duration)).into()
        }
        unsafe extern "system" fn CreateVirtualChannel<Impl: IWTSProtocolConnection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, szendpointname: super::super::Foundation::PSTR, bstatic: super::super::Foundation::BOOL, requestedpriority: u32, phchannel: *mut usize) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateVirtualChannel(::core::mem::transmute_copy(&szendpointname), ::core::mem::transmute_copy(&bstatic), ::core::mem::transmute_copy(&requestedpriority)) {
                ::core::result::Result::Ok(ok__) => {
                    *phchannel = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn QueryProperty<Impl: IWTSProtocolConnection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, querytype: ::windows::core::GUID, ulnumentriesin: u32, ulnumentriesout: u32, ppropertyentriesin: *const WTS_PROPERTY_VALUE, ppropertyentriesout: *mut WTS_PROPERTY_VALUE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).QueryProperty(::core::mem::transmute_copy(&querytype), ::core::mem::transmute_copy(&ulnumentriesin), ::core::mem::transmute_copy(&ulnumentriesout), ::core::mem::transmute_copy(&ppropertyentriesin), ::core::mem::transmute_copy(&ppropertyentriesout)).into()
        }
        unsafe extern "system" fn GetShadowConnection<Impl: IWTSProtocolConnection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppshadowconnection: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetShadowConnection() {
                ::core::result::Result::Ok(ok__) => {
                    *ppshadowconnection = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            GetLogonErrorRedirector: GetLogonErrorRedirector::<Impl, IMPL_OFFSET>,
            SendPolicyData: SendPolicyData::<Impl, IMPL_OFFSET>,
            AcceptConnection: AcceptConnection::<Impl, IMPL_OFFSET>,
            GetClientData: GetClientData::<Impl, IMPL_OFFSET>,
            GetUserCredentials: GetUserCredentials::<Impl, IMPL_OFFSET>,
            GetLicenseConnection: GetLicenseConnection::<Impl, IMPL_OFFSET>,
            AuthenticateClientToSession: AuthenticateClientToSession::<Impl, IMPL_OFFSET>,
            NotifySessionId: NotifySessionId::<Impl, IMPL_OFFSET>,
            GetProtocolHandles: GetProtocolHandles::<Impl, IMPL_OFFSET>,
            ConnectNotify: ConnectNotify::<Impl, IMPL_OFFSET>,
            IsUserAllowedToLogon: IsUserAllowedToLogon::<Impl, IMPL_OFFSET>,
            SessionArbitrationEnumeration: SessionArbitrationEnumeration::<Impl, IMPL_OFFSET>,
            LogonNotify: LogonNotify::<Impl, IMPL_OFFSET>,
            GetUserData: GetUserData::<Impl, IMPL_OFFSET>,
            DisconnectNotify: DisconnectNotify::<Impl, IMPL_OFFSET>,
            Close: Close::<Impl, IMPL_OFFSET>,
            GetProtocolStatus: GetProtocolStatus::<Impl, IMPL_OFFSET>,
            GetLastInputTime: GetLastInputTime::<Impl, IMPL_OFFSET>,
            SetErrorInfo: SetErrorInfo::<Impl, IMPL_OFFSET>,
            SendBeep: SendBeep::<Impl, IMPL_OFFSET>,
            CreateVirtualChannel: CreateVirtualChannel::<Impl, IMPL_OFFSET>,
            QueryProperty: QueryProperty::<Impl, IMPL_OFFSET>,
            GetShadowConnection: GetShadowConnection::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWTSProtocolConnection as ::windows::core::Interface>::IID
    }
}
pub trait IWTSProtocolConnectionCallback_Impl: Sized {
    fn OnReady(&mut self) -> ::windows::core::Result<()>;
    fn BrokenConnection(&mut self, reason: u32, source: u32) -> ::windows::core::Result<()>;
    fn StopScreenUpdates(&mut self) -> ::windows::core::Result<()>;
    fn RedrawWindow(&mut self, rect: *const WTS_SMALL_RECT) -> ::windows::core::Result<()>;
    fn DisplayIOCtl(&mut self, displayioctl: *const WTS_DISPLAY_IOCTL) -> ::windows::core::Result<()>;
}
impl IWTSProtocolConnectionCallback_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWTSProtocolConnectionCallback_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWTSProtocolConnectionCallback_Vtbl {
        unsafe extern "system" fn OnReady<Impl: IWTSProtocolConnectionCallback_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnReady().into()
        }
        unsafe extern "system" fn BrokenConnection<Impl: IWTSProtocolConnectionCallback_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, reason: u32, source: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).BrokenConnection(::core::mem::transmute_copy(&reason), ::core::mem::transmute_copy(&source)).into()
        }
        unsafe extern "system" fn StopScreenUpdates<Impl: IWTSProtocolConnectionCallback_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).StopScreenUpdates().into()
        }
        unsafe extern "system" fn RedrawWindow<Impl: IWTSProtocolConnectionCallback_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rect: *const WTS_SMALL_RECT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RedrawWindow(::core::mem::transmute_copy(&rect)).into()
        }
        unsafe extern "system" fn DisplayIOCtl<Impl: IWTSProtocolConnectionCallback_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, displayioctl: *const WTS_DISPLAY_IOCTL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).DisplayIOCtl(::core::mem::transmute_copy(&displayioctl)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            OnReady: OnReady::<Impl, IMPL_OFFSET>,
            BrokenConnection: BrokenConnection::<Impl, IMPL_OFFSET>,
            StopScreenUpdates: StopScreenUpdates::<Impl, IMPL_OFFSET>,
            RedrawWindow: RedrawWindow::<Impl, IMPL_OFFSET>,
            DisplayIOCtl: DisplayIOCtl::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWTSProtocolConnectionCallback as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IWTSProtocolLicenseConnection_Impl: Sized {
    fn RequestLicensingCapabilities(&mut self, pplicensecapabilities: *mut WTS_LICENSE_CAPABILITIES, pcblicensecapabilities: *mut u32) -> ::windows::core::Result<()>;
    fn SendClientLicense(&mut self, pclientlicense: *const u8, cbclientlicense: u32) -> ::windows::core::Result<()>;
    fn RequestClientLicense(&mut self, reserve1: *const u8, reserve2: u32, ppclientlicense: *mut u8, pcbclientlicense: *mut u32) -> ::windows::core::Result<()>;
    fn ProtocolComplete(&mut self, ulcomplete: u32) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IWTSProtocolLicenseConnection_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWTSProtocolLicenseConnection_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWTSProtocolLicenseConnection_Vtbl {
        unsafe extern "system" fn RequestLicensingCapabilities<Impl: IWTSProtocolLicenseConnection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pplicensecapabilities: *mut WTS_LICENSE_CAPABILITIES, pcblicensecapabilities: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RequestLicensingCapabilities(::core::mem::transmute_copy(&pplicensecapabilities), ::core::mem::transmute_copy(&pcblicensecapabilities)).into()
        }
        unsafe extern "system" fn SendClientLicense<Impl: IWTSProtocolLicenseConnection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pclientlicense: *const u8, cbclientlicense: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SendClientLicense(::core::mem::transmute_copy(&pclientlicense), ::core::mem::transmute_copy(&cbclientlicense)).into()
        }
        unsafe extern "system" fn RequestClientLicense<Impl: IWTSProtocolLicenseConnection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, reserve1: *const u8, reserve2: u32, ppclientlicense: *mut u8, pcbclientlicense: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RequestClientLicense(::core::mem::transmute_copy(&reserve1), ::core::mem::transmute_copy(&reserve2), ::core::mem::transmute_copy(&ppclientlicense), ::core::mem::transmute_copy(&pcbclientlicense)).into()
        }
        unsafe extern "system" fn ProtocolComplete<Impl: IWTSProtocolLicenseConnection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ulcomplete: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ProtocolComplete(::core::mem::transmute_copy(&ulcomplete)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            RequestLicensingCapabilities: RequestLicensingCapabilities::<Impl, IMPL_OFFSET>,
            SendClientLicense: SendClientLicense::<Impl, IMPL_OFFSET>,
            RequestClientLicense: RequestClientLicense::<Impl, IMPL_OFFSET>,
            ProtocolComplete: ProtocolComplete::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWTSProtocolLicenseConnection as ::windows::core::Interface>::IID
    }
}
pub trait IWTSProtocolListener_Impl: Sized {
    fn StartListen(&mut self, pcallback: &::core::option::Option<IWTSProtocolListenerCallback>) -> ::windows::core::Result<()>;
    fn StopListen(&mut self) -> ::windows::core::Result<()>;
}
impl IWTSProtocolListener_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWTSProtocolListener_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWTSProtocolListener_Vtbl {
        unsafe extern "system" fn StartListen<Impl: IWTSProtocolListener_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcallback: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).StartListen(::core::mem::transmute(&pcallback)).into()
        }
        unsafe extern "system" fn StopListen<Impl: IWTSProtocolListener_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).StopListen().into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            StartListen: StartListen::<Impl, IMPL_OFFSET>,
            StopListen: StopListen::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWTSProtocolListener as ::windows::core::Interface>::IID
    }
}
pub trait IWTSProtocolListenerCallback_Impl: Sized {
    fn OnConnected(&mut self, pconnection: &::core::option::Option<IWTSProtocolConnection>) -> ::windows::core::Result<IWTSProtocolConnectionCallback>;
}
impl IWTSProtocolListenerCallback_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWTSProtocolListenerCallback_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWTSProtocolListenerCallback_Vtbl {
        unsafe extern "system" fn OnConnected<Impl: IWTSProtocolListenerCallback_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pconnection: ::windows::core::RawPtr, pcallback: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OnConnected(::core::mem::transmute(&pconnection)) {
                ::core::result::Result::Ok(ok__) => {
                    *pcallback = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), OnConnected: OnConnected::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWTSProtocolListenerCallback as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IWTSProtocolLogonErrorRedirector_Impl: Sized {
    fn OnBeginPainting(&mut self) -> ::windows::core::Result<()>;
    fn RedirectStatus(&mut self, pszmessage: super::super::Foundation::PWSTR) -> ::windows::core::Result<WTS_LOGON_ERROR_REDIRECTOR_RESPONSE>;
    fn RedirectMessage(&mut self, pszcaption: super::super::Foundation::PWSTR, pszmessage: super::super::Foundation::PWSTR, utype: u32) -> ::windows::core::Result<WTS_LOGON_ERROR_REDIRECTOR_RESPONSE>;
    fn RedirectLogonError(&mut self, ntsstatus: i32, ntssubstatus: i32, pszcaption: super::super::Foundation::PWSTR, pszmessage: super::super::Foundation::PWSTR, utype: u32) -> ::windows::core::Result<WTS_LOGON_ERROR_REDIRECTOR_RESPONSE>;
}
#[cfg(feature = "Win32_Foundation")]
impl IWTSProtocolLogonErrorRedirector_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWTSProtocolLogonErrorRedirector_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWTSProtocolLogonErrorRedirector_Vtbl {
        unsafe extern "system" fn OnBeginPainting<Impl: IWTSProtocolLogonErrorRedirector_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnBeginPainting().into()
        }
        unsafe extern "system" fn RedirectStatus<Impl: IWTSProtocolLogonErrorRedirector_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszmessage: super::super::Foundation::PWSTR, presponse: *mut WTS_LOGON_ERROR_REDIRECTOR_RESPONSE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RedirectStatus(::core::mem::transmute_copy(&pszmessage)) {
                ::core::result::Result::Ok(ok__) => {
                    *presponse = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RedirectMessage<Impl: IWTSProtocolLogonErrorRedirector_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszcaption: super::super::Foundation::PWSTR, pszmessage: super::super::Foundation::PWSTR, utype: u32, presponse: *mut WTS_LOGON_ERROR_REDIRECTOR_RESPONSE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RedirectMessage(::core::mem::transmute_copy(&pszcaption), ::core::mem::transmute_copy(&pszmessage), ::core::mem::transmute_copy(&utype)) {
                ::core::result::Result::Ok(ok__) => {
                    *presponse = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RedirectLogonError<Impl: IWTSProtocolLogonErrorRedirector_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ntsstatus: i32, ntssubstatus: i32, pszcaption: super::super::Foundation::PWSTR, pszmessage: super::super::Foundation::PWSTR, utype: u32, presponse: *mut WTS_LOGON_ERROR_REDIRECTOR_RESPONSE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RedirectLogonError(::core::mem::transmute_copy(&ntsstatus), ::core::mem::transmute_copy(&ntssubstatus), ::core::mem::transmute_copy(&pszcaption), ::core::mem::transmute_copy(&pszmessage), ::core::mem::transmute_copy(&utype)) {
                ::core::result::Result::Ok(ok__) => {
                    *presponse = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            OnBeginPainting: OnBeginPainting::<Impl, IMPL_OFFSET>,
            RedirectStatus: RedirectStatus::<Impl, IMPL_OFFSET>,
            RedirectMessage: RedirectMessage::<Impl, IMPL_OFFSET>,
            RedirectLogonError: RedirectLogonError::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWTSProtocolLogonErrorRedirector as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IWTSProtocolManager_Impl: Sized {
    fn CreateListener(&mut self, wszlistenername: super::super::Foundation::PWSTR) -> ::windows::core::Result<IWTSProtocolListener>;
    fn NotifyServiceStateChange(&mut self, ptsservicestatechange: *const WTS_SERVICE_STATE) -> ::windows::core::Result<()>;
    fn NotifySessionOfServiceStart(&mut self, sessionid: *const WTS_SESSION_ID) -> ::windows::core::Result<()>;
    fn NotifySessionOfServiceStop(&mut self, sessionid: *const WTS_SESSION_ID) -> ::windows::core::Result<()>;
    fn NotifySessionStateChange(&mut self, sessionid: *const WTS_SESSION_ID, eventid: u32) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IWTSProtocolManager_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWTSProtocolManager_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWTSProtocolManager_Vtbl {
        unsafe extern "system" fn CreateListener<Impl: IWTSProtocolManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wszlistenername: super::super::Foundation::PWSTR, pprotocollistener: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateListener(::core::mem::transmute_copy(&wszlistenername)) {
                ::core::result::Result::Ok(ok__) => {
                    *pprotocollistener = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn NotifyServiceStateChange<Impl: IWTSProtocolManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ptsservicestatechange: *const WTS_SERVICE_STATE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).NotifyServiceStateChange(::core::mem::transmute_copy(&ptsservicestatechange)).into()
        }
        unsafe extern "system" fn NotifySessionOfServiceStart<Impl: IWTSProtocolManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, sessionid: *const WTS_SESSION_ID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).NotifySessionOfServiceStart(::core::mem::transmute_copy(&sessionid)).into()
        }
        unsafe extern "system" fn NotifySessionOfServiceStop<Impl: IWTSProtocolManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, sessionid: *const WTS_SESSION_ID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).NotifySessionOfServiceStop(::core::mem::transmute_copy(&sessionid)).into()
        }
        unsafe extern "system" fn NotifySessionStateChange<Impl: IWTSProtocolManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, sessionid: *const WTS_SESSION_ID, eventid: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).NotifySessionStateChange(::core::mem::transmute_copy(&sessionid), ::core::mem::transmute_copy(&eventid)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            CreateListener: CreateListener::<Impl, IMPL_OFFSET>,
            NotifyServiceStateChange: NotifyServiceStateChange::<Impl, IMPL_OFFSET>,
            NotifySessionOfServiceStart: NotifySessionOfServiceStart::<Impl, IMPL_OFFSET>,
            NotifySessionOfServiceStop: NotifySessionOfServiceStop::<Impl, IMPL_OFFSET>,
            NotifySessionStateChange: NotifySessionStateChange::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWTSProtocolManager as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IWTSProtocolShadowCallback_Impl: Sized {
    fn StopShadow(&mut self) -> ::windows::core::Result<()>;
    fn InvokeTargetShadow(&mut self, ptargetservername: super::super::Foundation::PWSTR, targetsessionid: u32, pparam1: *const u8, param1size: u32, pparam2: *const u8, param2size: u32, pparam3: *const u8, param3size: u32, pparam4: *const u8, param4size: u32, pclientname: super::super::Foundation::PWSTR) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IWTSProtocolShadowCallback_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWTSProtocolShadowCallback_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWTSProtocolShadowCallback_Vtbl {
        unsafe extern "system" fn StopShadow<Impl: IWTSProtocolShadowCallback_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).StopShadow().into()
        }
        unsafe extern "system" fn InvokeTargetShadow<Impl: IWTSProtocolShadowCallback_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ptargetservername: super::super::Foundation::PWSTR, targetsessionid: u32, pparam1: *const u8, param1size: u32, pparam2: *const u8, param2size: u32, pparam3: *const u8, param3size: u32, pparam4: *const u8, param4size: u32, pclientname: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this)
                .InvokeTargetShadow(::core::mem::transmute_copy(&ptargetservername), ::core::mem::transmute_copy(&targetsessionid), ::core::mem::transmute_copy(&pparam1), ::core::mem::transmute_copy(&param1size), ::core::mem::transmute_copy(&pparam2), ::core::mem::transmute_copy(&param2size), ::core::mem::transmute_copy(&pparam3), ::core::mem::transmute_copy(&param3size), ::core::mem::transmute_copy(&pparam4), ::core::mem::transmute_copy(&param4size), ::core::mem::transmute_copy(&pclientname))
                .into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            StopShadow: StopShadow::<Impl, IMPL_OFFSET>,
            InvokeTargetShadow: InvokeTargetShadow::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWTSProtocolShadowCallback as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IWTSProtocolShadowConnection_Impl: Sized {
    fn Start(&mut self, ptargetservername: super::super::Foundation::PWSTR, targetsessionid: u32, hotkeyvk: u8, hotkeymodifiers: u16, pshadowcallback: &::core::option::Option<IWTSProtocolShadowCallback>) -> ::windows::core::Result<()>;
    fn Stop(&mut self) -> ::windows::core::Result<()>;
    fn DoTarget(&mut self, pparam1: *const u8, param1size: u32, pparam2: *const u8, param2size: u32, pparam3: *const u8, param3size: u32, pparam4: *const u8, param4size: u32, pclientname: super::super::Foundation::PWSTR) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IWTSProtocolShadowConnection_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWTSProtocolShadowConnection_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWTSProtocolShadowConnection_Vtbl {
        unsafe extern "system" fn Start<Impl: IWTSProtocolShadowConnection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ptargetservername: super::super::Foundation::PWSTR, targetsessionid: u32, hotkeyvk: u8, hotkeymodifiers: u16, pshadowcallback: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Start(::core::mem::transmute_copy(&ptargetservername), ::core::mem::transmute_copy(&targetsessionid), ::core::mem::transmute_copy(&hotkeyvk), ::core::mem::transmute_copy(&hotkeymodifiers), ::core::mem::transmute(&pshadowcallback)).into()
        }
        unsafe extern "system" fn Stop<Impl: IWTSProtocolShadowConnection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Stop().into()
        }
        unsafe extern "system" fn DoTarget<Impl: IWTSProtocolShadowConnection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pparam1: *const u8, param1size: u32, pparam2: *const u8, param2size: u32, pparam3: *const u8, param3size: u32, pparam4: *const u8, param4size: u32, pclientname: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).DoTarget(::core::mem::transmute_copy(&pparam1), ::core::mem::transmute_copy(&param1size), ::core::mem::transmute_copy(&pparam2), ::core::mem::transmute_copy(&param2size), ::core::mem::transmute_copy(&pparam3), ::core::mem::transmute_copy(&param3size), ::core::mem::transmute_copy(&pparam4), ::core::mem::transmute_copy(&param4size), ::core::mem::transmute_copy(&pclientname)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            Start: Start::<Impl, IMPL_OFFSET>,
            Stop: Stop::<Impl, IMPL_OFFSET>,
            DoTarget: DoTarget::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWTSProtocolShadowConnection as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IWTSSBPlugin_Impl: Sized {
    fn Initialize(&mut self) -> ::windows::core::Result<u32>;
    fn WTSSBX_MachineChangeNotification(&mut self, notificationtype: WTSSBX_NOTIFICATION_TYPE, machineid: i32, pmachineinfo: *const WTSSBX_MACHINE_INFO) -> ::windows::core::Result<()>;
    fn WTSSBX_SessionChangeNotification(&mut self, notificationtype: WTSSBX_NOTIFICATION_TYPE, machineid: i32, numofsessions: u32, sessioninfo: *const WTSSBX_SESSION_INFO) -> ::windows::core::Result<()>;
    fn WTSSBX_GetMostSuitableServer(&mut self, username: super::super::Foundation::PWSTR, domainname: super::super::Foundation::PWSTR, applicationtype: super::super::Foundation::PWSTR, farmname: super::super::Foundation::PWSTR, pmachineid: *mut i32) -> ::windows::core::Result<()>;
    fn Terminated(&mut self) -> ::windows::core::Result<()>;
    fn WTSSBX_GetUserExternalSession(&mut self, username: super::super::Foundation::PWSTR, domainname: super::super::Foundation::PWSTR, applicationtype: super::super::Foundation::PWSTR, redirectorinternalip: *const WTSSBX_IP_ADDRESS, psessionid: *mut u32, pmachineconnectinfo: *mut WTSSBX_MACHINE_CONNECT_INFO) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IWTSSBPlugin_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWTSSBPlugin_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWTSSBPlugin_Vtbl {
        unsafe extern "system" fn Initialize<Impl: IWTSSBPlugin_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plugincapabilities: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Initialize() {
                ::core::result::Result::Ok(ok__) => {
                    *plugincapabilities = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn WTSSBX_MachineChangeNotification<Impl: IWTSSBPlugin_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, notificationtype: WTSSBX_NOTIFICATION_TYPE, machineid: i32, pmachineinfo: *const WTSSBX_MACHINE_INFO) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).WTSSBX_MachineChangeNotification(::core::mem::transmute_copy(&notificationtype), ::core::mem::transmute_copy(&machineid), ::core::mem::transmute_copy(&pmachineinfo)).into()
        }
        unsafe extern "system" fn WTSSBX_SessionChangeNotification<Impl: IWTSSBPlugin_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, notificationtype: WTSSBX_NOTIFICATION_TYPE, machineid: i32, numofsessions: u32, sessioninfo: *const WTSSBX_SESSION_INFO) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).WTSSBX_SessionChangeNotification(::core::mem::transmute_copy(&notificationtype), ::core::mem::transmute_copy(&machineid), ::core::mem::transmute_copy(&numofsessions), ::core::mem::transmute_copy(&sessioninfo)).into()
        }
        unsafe extern "system" fn WTSSBX_GetMostSuitableServer<Impl: IWTSSBPlugin_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, username: super::super::Foundation::PWSTR, domainname: super::super::Foundation::PWSTR, applicationtype: super::super::Foundation::PWSTR, farmname: super::super::Foundation::PWSTR, pmachineid: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).WTSSBX_GetMostSuitableServer(::core::mem::transmute_copy(&username), ::core::mem::transmute_copy(&domainname), ::core::mem::transmute_copy(&applicationtype), ::core::mem::transmute_copy(&farmname), ::core::mem::transmute_copy(&pmachineid)).into()
        }
        unsafe extern "system" fn Terminated<Impl: IWTSSBPlugin_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Terminated().into()
        }
        unsafe extern "system" fn WTSSBX_GetUserExternalSession<Impl: IWTSSBPlugin_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, username: super::super::Foundation::PWSTR, domainname: super::super::Foundation::PWSTR, applicationtype: super::super::Foundation::PWSTR, redirectorinternalip: *const WTSSBX_IP_ADDRESS, psessionid: *mut u32, pmachineconnectinfo: *mut WTSSBX_MACHINE_CONNECT_INFO) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).WTSSBX_GetUserExternalSession(::core::mem::transmute_copy(&username), ::core::mem::transmute_copy(&domainname), ::core::mem::transmute_copy(&applicationtype), ::core::mem::transmute_copy(&redirectorinternalip), ::core::mem::transmute_copy(&psessionid), ::core::mem::transmute_copy(&pmachineconnectinfo)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            Initialize: Initialize::<Impl, IMPL_OFFSET>,
            WTSSBX_MachineChangeNotification: WTSSBX_MachineChangeNotification::<Impl, IMPL_OFFSET>,
            WTSSBX_SessionChangeNotification: WTSSBX_SessionChangeNotification::<Impl, IMPL_OFFSET>,
            WTSSBX_GetMostSuitableServer: WTSSBX_GetMostSuitableServer::<Impl, IMPL_OFFSET>,
            Terminated: Terminated::<Impl, IMPL_OFFSET>,
            WTSSBX_GetUserExternalSession: WTSSBX_GetUserExternalSession::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWTSSBPlugin as ::windows::core::Interface>::IID
    }
}
pub trait IWTSVirtualChannel_Impl: Sized {
    fn Write(&mut self, cbsize: u32, pbuffer: *const u8, preserved: &::core::option::Option<::windows::core::IUnknown>) -> ::windows::core::Result<()>;
    fn Close(&mut self) -> ::windows::core::Result<()>;
}
impl IWTSVirtualChannel_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWTSVirtualChannel_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWTSVirtualChannel_Vtbl {
        unsafe extern "system" fn Write<Impl: IWTSVirtualChannel_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cbsize: u32, pbuffer: *const u8, preserved: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Write(::core::mem::transmute_copy(&cbsize), ::core::mem::transmute_copy(&pbuffer), ::core::mem::transmute(&preserved)).into()
        }
        unsafe extern "system" fn Close<Impl: IWTSVirtualChannel_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Close().into()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), Write: Write::<Impl, IMPL_OFFSET>, Close: Close::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWTSVirtualChannel as ::windows::core::Interface>::IID
    }
}
pub trait IWTSVirtualChannelCallback_Impl: Sized {
    fn OnDataReceived(&mut self, cbsize: u32, pbuffer: *const u8) -> ::windows::core::Result<()>;
    fn OnClose(&mut self) -> ::windows::core::Result<()>;
}
impl IWTSVirtualChannelCallback_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWTSVirtualChannelCallback_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWTSVirtualChannelCallback_Vtbl {
        unsafe extern "system" fn OnDataReceived<Impl: IWTSVirtualChannelCallback_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cbsize: u32, pbuffer: *const u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnDataReceived(::core::mem::transmute_copy(&cbsize), ::core::mem::transmute_copy(&pbuffer)).into()
        }
        unsafe extern "system" fn OnClose<Impl: IWTSVirtualChannelCallback_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnClose().into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            OnDataReceived: OnDataReceived::<Impl, IMPL_OFFSET>,
            OnClose: OnClose::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWTSVirtualChannelCallback as ::windows::core::Interface>::IID
    }
}
pub trait IWTSVirtualChannelManager_Impl: Sized {
    fn CreateListener(&mut self, pszchannelname: *const u8, uflags: u32, plistenercallback: &::core::option::Option<IWTSListenerCallback>) -> ::windows::core::Result<IWTSListener>;
}
impl IWTSVirtualChannelManager_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWTSVirtualChannelManager_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWTSVirtualChannelManager_Vtbl {
        unsafe extern "system" fn CreateListener<Impl: IWTSVirtualChannelManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszchannelname: *const u8, uflags: u32, plistenercallback: ::windows::core::RawPtr, pplistener: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateListener(::core::mem::transmute_copy(&pszchannelname), ::core::mem::transmute_copy(&uflags), ::core::mem::transmute(&plistenercallback)) {
                ::core::result::Result::Ok(ok__) => {
                    *pplistener = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), CreateListener: CreateListener::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWTSVirtualChannelManager as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub trait IWorkspace_Impl: Sized {
    fn GetWorkspaceNames(&mut self) -> ::windows::core::Result<*mut super::Com::SAFEARRAY>;
    fn StartRemoteApplication(&mut self, bstrworkspaceid: &super::super::Foundation::BSTR, psaparams: *const super::Com::SAFEARRAY) -> ::windows::core::Result<()>;
    fn GetProcessId(&mut self) -> ::windows::core::Result<u32>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl IWorkspace_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWorkspace_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWorkspace_Vtbl {
        unsafe extern "system" fn GetWorkspaceNames<Impl: IWorkspace_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psawkspnames: *mut *mut super::Com::SAFEARRAY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetWorkspaceNames() {
                ::core::result::Result::Ok(ok__) => {
                    *psawkspnames = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn StartRemoteApplication<Impl: IWorkspace_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrworkspaceid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, psaparams: *const super::Com::SAFEARRAY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).StartRemoteApplication(::core::mem::transmute_copy(&bstrworkspaceid), ::core::mem::transmute_copy(&psaparams)).into()
        }
        unsafe extern "system" fn GetProcessId<Impl: IWorkspace_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pulprocessid: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetProcessId() {
                ::core::result::Result::Ok(ok__) => {
                    *pulprocessid = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            GetWorkspaceNames: GetWorkspaceNames::<Impl, IMPL_OFFSET>,
            StartRemoteApplication: StartRemoteApplication::<Impl, IMPL_OFFSET>,
            GetProcessId: GetProcessId::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWorkspace as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub trait IWorkspace2_Impl: Sized + IWorkspace_Impl {
    fn StartRemoteApplicationEx(&mut self, bstrworkspaceid: &super::super::Foundation::BSTR, bstrrequestingappid: &super::super::Foundation::BSTR, bstrrequestingappfamilyname: &super::super::Foundation::BSTR, blaunchintoimmersiveclient: i16, bstrimmersiveclientactivationcontext: &super::super::Foundation::BSTR, psaparams: *const super::Com::SAFEARRAY) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl IWorkspace2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWorkspace2_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWorkspace2_Vtbl {
        unsafe extern "system" fn StartRemoteApplicationEx<Impl: IWorkspace2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrworkspaceid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrrequestingappid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrrequestingappfamilyname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, blaunchintoimmersiveclient: i16, bstrimmersiveclientactivationcontext: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, psaparams: *const super::Com::SAFEARRAY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).StartRemoteApplicationEx(::core::mem::transmute_copy(&bstrworkspaceid), ::core::mem::transmute_copy(&bstrrequestingappid), ::core::mem::transmute_copy(&bstrrequestingappfamilyname), ::core::mem::transmute_copy(&blaunchintoimmersiveclient), ::core::mem::transmute_copy(&bstrimmersiveclientactivationcontext), ::core::mem::transmute_copy(&psaparams)).into()
        }
        Self {
            base: IWorkspace_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            StartRemoteApplicationEx: StartRemoteApplicationEx::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWorkspace2 as ::windows::core::Interface>::IID || iid == &<IWorkspace as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub trait IWorkspace3_Impl: Sized + IWorkspace_Impl + IWorkspace2_Impl {
    fn GetClaimsToken2(&mut self, bstrclaimshint: &super::super::Foundation::BSTR, bstruserhint: &super::super::Foundation::BSTR, claimcookie: u32, hwndcreduiparent: u32, rectcreduiparent: &super::super::Foundation::RECT) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetClaimsToken(&mut self, bstraccesstoken: &super::super::Foundation::BSTR, ullaccesstokenexpiration: u64, bstrrefreshtoken: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl IWorkspace3_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWorkspace3_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWorkspace3_Vtbl {
        unsafe extern "system" fn GetClaimsToken2<Impl: IWorkspace3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrclaimshint: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstruserhint: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, claimcookie: u32, hwndcreduiparent: u32, rectcreduiparent: super::super::Foundation::RECT, pbstraccesstoken: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetClaimsToken2(::core::mem::transmute_copy(&bstrclaimshint), ::core::mem::transmute_copy(&bstruserhint), ::core::mem::transmute_copy(&claimcookie), ::core::mem::transmute_copy(&hwndcreduiparent), ::core::mem::transmute_copy(&rectcreduiparent)) {
                ::core::result::Result::Ok(ok__) => {
                    *pbstraccesstoken = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetClaimsToken<Impl: IWorkspace3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstraccesstoken: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ullaccesstokenexpiration: u64, bstrrefreshtoken: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetClaimsToken(::core::mem::transmute_copy(&bstraccesstoken), ::core::mem::transmute_copy(&ullaccesstokenexpiration), ::core::mem::transmute_copy(&bstrrefreshtoken)).into()
        }
        Self {
            base: IWorkspace2_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            GetClaimsToken2: GetClaimsToken2::<Impl, IMPL_OFFSET>,
            SetClaimsToken: SetClaimsToken::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWorkspace3 as ::windows::core::Interface>::IID || iid == &<IWorkspace as ::windows::core::Interface>::IID || iid == &<IWorkspace2 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IWorkspaceClientExt_Impl: Sized {
    fn GetResourceId(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn GetResourceDisplayName(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn IssueDisconnect(&mut self) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IWorkspaceClientExt_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWorkspaceClientExt_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWorkspaceClientExt_Vtbl {
        unsafe extern "system" fn GetResourceId<Impl: IWorkspaceClientExt_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrworkspaceid: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetResourceId() {
                ::core::result::Result::Ok(ok__) => {
                    *bstrworkspaceid = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetResourceDisplayName<Impl: IWorkspaceClientExt_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrworkspacedisplayname: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetResourceDisplayName() {
                ::core::result::Result::Ok(ok__) => {
                    *bstrworkspacedisplayname = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IssueDisconnect<Impl: IWorkspaceClientExt_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).IssueDisconnect().into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            GetResourceId: GetResourceId::<Impl, IMPL_OFFSET>,
            GetResourceDisplayName: GetResourceDisplayName::<Impl, IMPL_OFFSET>,
            IssueDisconnect: IssueDisconnect::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWorkspaceClientExt as ::windows::core::Interface>::IID
    }
}
pub trait IWorkspaceRegistration_Impl: Sized {
    fn AddResource(&mut self, punk: &::core::option::Option<IWorkspaceClientExt>) -> ::windows::core::Result<u32>;
    fn RemoveResource(&mut self, dwcookieconnection: u32) -> ::windows::core::Result<()>;
}
impl IWorkspaceRegistration_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWorkspaceRegistration_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWorkspaceRegistration_Vtbl {
        unsafe extern "system" fn AddResource<Impl: IWorkspaceRegistration_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, punk: ::windows::core::RawPtr, pdwcookie: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AddResource(::core::mem::transmute(&punk)) {
                ::core::result::Result::Ok(ok__) => {
                    *pdwcookie = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveResource<Impl: IWorkspaceRegistration_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwcookieconnection: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveResource(::core::mem::transmute_copy(&dwcookieconnection)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            AddResource: AddResource::<Impl, IMPL_OFFSET>,
            RemoveResource: RemoveResource::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWorkspaceRegistration as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IWorkspaceRegistration2_Impl: Sized + IWorkspaceRegistration_Impl {
    fn AddResourceEx(&mut self, punk: &::core::option::Option<IWorkspaceClientExt>, bstreventloguploadaddress: &super::super::Foundation::BSTR, pdwcookie: *mut u32, correlationid: &::windows::core::GUID) -> ::windows::core::Result<()>;
    fn RemoveResourceEx(&mut self, dwcookieconnection: u32, correlationid: &::windows::core::GUID) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IWorkspaceRegistration2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWorkspaceRegistration2_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWorkspaceRegistration2_Vtbl {
        unsafe extern "system" fn AddResourceEx<Impl: IWorkspaceRegistration2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, punk: ::windows::core::RawPtr, bstreventloguploadaddress: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pdwcookie: *mut u32, correlationid: ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AddResourceEx(::core::mem::transmute(&punk), ::core::mem::transmute_copy(&bstreventloguploadaddress), ::core::mem::transmute_copy(&pdwcookie), ::core::mem::transmute_copy(&correlationid)).into()
        }
        unsafe extern "system" fn RemoveResourceEx<Impl: IWorkspaceRegistration2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwcookieconnection: u32, correlationid: ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveResourceEx(::core::mem::transmute_copy(&dwcookieconnection), ::core::mem::transmute_copy(&correlationid)).into()
        }
        Self {
            base: IWorkspaceRegistration_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            AddResourceEx: AddResourceEx::<Impl, IMPL_OFFSET>,
            RemoveResourceEx: RemoveResourceEx::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWorkspaceRegistration2 as ::windows::core::Interface>::IID || iid == &<IWorkspaceRegistration as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IWorkspaceReportMessage_Impl: Sized {
    fn RegisterErrorLogMessage(&mut self, bstrmessage: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn IsErrorMessageRegistered(&mut self, bstrwkspid: &super::super::Foundation::BSTR, dwerrortype: u32, bstrerrormessagetype: &super::super::Foundation::BSTR, dwerrorcode: u32) -> ::windows::core::Result<i16>;
    fn RegisterErrorEvent(&mut self, bstrwkspid: &super::super::Foundation::BSTR, dwerrortype: u32, bstrerrormessagetype: &super::super::Foundation::BSTR, dwerrorcode: u32) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IWorkspaceReportMessage_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWorkspaceReportMessage_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWorkspaceReportMessage_Vtbl {
        unsafe extern "system" fn RegisterErrorLogMessage<Impl: IWorkspaceReportMessage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrmessage: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RegisterErrorLogMessage(::core::mem::transmute_copy(&bstrmessage)).into()
        }
        unsafe extern "system" fn IsErrorMessageRegistered<Impl: IWorkspaceReportMessage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrwkspid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, dwerrortype: u32, bstrerrormessagetype: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, dwerrorcode: u32, pferrorexist: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsErrorMessageRegistered(::core::mem::transmute_copy(&bstrwkspid), ::core::mem::transmute_copy(&dwerrortype), ::core::mem::transmute_copy(&bstrerrormessagetype), ::core::mem::transmute_copy(&dwerrorcode)) {
                ::core::result::Result::Ok(ok__) => {
                    *pferrorexist = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RegisterErrorEvent<Impl: IWorkspaceReportMessage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrwkspid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, dwerrortype: u32, bstrerrormessagetype: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, dwerrorcode: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RegisterErrorEvent(::core::mem::transmute_copy(&bstrwkspid), ::core::mem::transmute_copy(&dwerrortype), ::core::mem::transmute_copy(&bstrerrormessagetype), ::core::mem::transmute_copy(&dwerrorcode)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            RegisterErrorLogMessage: RegisterErrorLogMessage::<Impl, IMPL_OFFSET>,
            IsErrorMessageRegistered: IsErrorMessageRegistered::<Impl, IMPL_OFFSET>,
            RegisterErrorEvent: RegisterErrorEvent::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWorkspaceReportMessage as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IWorkspaceResTypeRegistry_Impl: Sized + super::Com::IDispatch_Impl {
    fn AddResourceType(&mut self, fmachinewide: i16, bstrfileextension: &super::super::Foundation::BSTR, bstrlauncher: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn DeleteResourceType(&mut self, fmachinewide: i16, bstrfileextension: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn GetRegisteredFileExtensions(&mut self, fmachinewide: i16) -> ::windows::core::Result<*mut super::Com::SAFEARRAY>;
    fn GetResourceTypeInfo(&mut self, fmachinewide: i16, bstrfileextension: &super::super::Foundation::BSTR) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn ModifyResourceType(&mut self, fmachinewide: i16, bstrfileextension: &super::super::Foundation::BSTR, bstrlauncher: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IWorkspaceResTypeRegistry_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWorkspaceResTypeRegistry_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWorkspaceResTypeRegistry_Vtbl {
        unsafe extern "system" fn AddResourceType<Impl: IWorkspaceResTypeRegistry_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fmachinewide: i16, bstrfileextension: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrlauncher: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AddResourceType(::core::mem::transmute_copy(&fmachinewide), ::core::mem::transmute_copy(&bstrfileextension), ::core::mem::transmute_copy(&bstrlauncher)).into()
        }
        unsafe extern "system" fn DeleteResourceType<Impl: IWorkspaceResTypeRegistry_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fmachinewide: i16, bstrfileextension: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).DeleteResourceType(::core::mem::transmute_copy(&fmachinewide), ::core::mem::transmute_copy(&bstrfileextension)).into()
        }
        unsafe extern "system" fn GetRegisteredFileExtensions<Impl: IWorkspaceResTypeRegistry_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fmachinewide: i16, psafileextensions: *mut *mut super::Com::SAFEARRAY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetRegisteredFileExtensions(::core::mem::transmute_copy(&fmachinewide)) {
                ::core::result::Result::Ok(ok__) => {
                    *psafileextensions = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetResourceTypeInfo<Impl: IWorkspaceResTypeRegistry_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fmachinewide: i16, bstrfileextension: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pbstrlauncher: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetResourceTypeInfo(::core::mem::transmute_copy(&fmachinewide), ::core::mem::transmute_copy(&bstrfileextension)) {
                ::core::result::Result::Ok(ok__) => {
                    *pbstrlauncher = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ModifyResourceType<Impl: IWorkspaceResTypeRegistry_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fmachinewide: i16, bstrfileextension: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrlauncher: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ModifyResourceType(::core::mem::transmute_copy(&fmachinewide), ::core::mem::transmute_copy(&bstrfileextension), ::core::mem::transmute_copy(&bstrlauncher)).into()
        }
        Self {
            base: super::Com::IDispatch_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            AddResourceType: AddResourceType::<Impl, IMPL_OFFSET>,
            DeleteResourceType: DeleteResourceType::<Impl, IMPL_OFFSET>,
            GetRegisteredFileExtensions: GetRegisteredFileExtensions::<Impl, IMPL_OFFSET>,
            GetResourceTypeInfo: GetResourceTypeInfo::<Impl, IMPL_OFFSET>,
            ModifyResourceType: ModifyResourceType::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWorkspaceResTypeRegistry as ::windows::core::Interface>::IID || iid == &<super::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IWorkspaceScriptable_Impl: Sized + super::Com::IDispatch_Impl {
    fn DisconnectWorkspace(&mut self, bstrworkspaceid: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn StartWorkspace(&mut self, bstrworkspaceid: &super::super::Foundation::BSTR, bstrusername: &super::super::Foundation::BSTR, bstrpassword: &super::super::Foundation::BSTR, bstrworkspaceparams: &super::super::Foundation::BSTR, ltimeout: i32, lflags: i32) -> ::windows::core::Result<()>;
    fn IsWorkspaceCredentialSpecified(&mut self, bstrworkspaceid: &super::super::Foundation::BSTR, bcountunauthenticatedcredentials: i16) -> ::windows::core::Result<i16>;
    fn IsWorkspaceSSOEnabled(&mut self) -> ::windows::core::Result<i16>;
    fn ClearWorkspaceCredential(&mut self, bstrworkspaceid: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn OnAuthenticated(&mut self, bstrworkspaceid: &super::super::Foundation::BSTR, bstrusername: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn DisconnectWorkspaceByFriendlyName(&mut self, bstrworkspacefriendlyname: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IWorkspaceScriptable_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWorkspaceScriptable_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWorkspaceScriptable_Vtbl {
        unsafe extern "system" fn DisconnectWorkspace<Impl: IWorkspaceScriptable_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrworkspaceid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).DisconnectWorkspace(::core::mem::transmute_copy(&bstrworkspaceid)).into()
        }
        unsafe extern "system" fn StartWorkspace<Impl: IWorkspaceScriptable_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrworkspaceid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrusername: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrpassword: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrworkspaceparams: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ltimeout: i32, lflags: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).StartWorkspace(::core::mem::transmute_copy(&bstrworkspaceid), ::core::mem::transmute_copy(&bstrusername), ::core::mem::transmute_copy(&bstrpassword), ::core::mem::transmute_copy(&bstrworkspaceparams), ::core::mem::transmute_copy(&ltimeout), ::core::mem::transmute_copy(&lflags)).into()
        }
        unsafe extern "system" fn IsWorkspaceCredentialSpecified<Impl: IWorkspaceScriptable_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrworkspaceid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bcountunauthenticatedcredentials: i16, pbcredexist: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsWorkspaceCredentialSpecified(::core::mem::transmute_copy(&bstrworkspaceid), ::core::mem::transmute_copy(&bcountunauthenticatedcredentials)) {
                ::core::result::Result::Ok(ok__) => {
                    *pbcredexist = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsWorkspaceSSOEnabled<Impl: IWorkspaceScriptable_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbssoenabled: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsWorkspaceSSOEnabled() {
                ::core::result::Result::Ok(ok__) => {
                    *pbssoenabled = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ClearWorkspaceCredential<Impl: IWorkspaceScriptable_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrworkspaceid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ClearWorkspaceCredential(::core::mem::transmute_copy(&bstrworkspaceid)).into()
        }
        unsafe extern "system" fn OnAuthenticated<Impl: IWorkspaceScriptable_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrworkspaceid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrusername: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnAuthenticated(::core::mem::transmute_copy(&bstrworkspaceid), ::core::mem::transmute_copy(&bstrusername)).into()
        }
        unsafe extern "system" fn DisconnectWorkspaceByFriendlyName<Impl: IWorkspaceScriptable_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrworkspacefriendlyname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).DisconnectWorkspaceByFriendlyName(::core::mem::transmute_copy(&bstrworkspacefriendlyname)).into()
        }
        Self {
            base: super::Com::IDispatch_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            DisconnectWorkspace: DisconnectWorkspace::<Impl, IMPL_OFFSET>,
            StartWorkspace: StartWorkspace::<Impl, IMPL_OFFSET>,
            IsWorkspaceCredentialSpecified: IsWorkspaceCredentialSpecified::<Impl, IMPL_OFFSET>,
            IsWorkspaceSSOEnabled: IsWorkspaceSSOEnabled::<Impl, IMPL_OFFSET>,
            ClearWorkspaceCredential: ClearWorkspaceCredential::<Impl, IMPL_OFFSET>,
            OnAuthenticated: OnAuthenticated::<Impl, IMPL_OFFSET>,
            DisconnectWorkspaceByFriendlyName: DisconnectWorkspaceByFriendlyName::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWorkspaceScriptable as ::windows::core::Interface>::IID || iid == &<super::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IWorkspaceScriptable2_Impl: Sized + super::Com::IDispatch_Impl + IWorkspaceScriptable_Impl {
    fn StartWorkspaceEx(&mut self, bstrworkspaceid: &super::super::Foundation::BSTR, bstrworkspacefriendlyname: &super::super::Foundation::BSTR, bstrredirectorname: &super::super::Foundation::BSTR, bstrusername: &super::super::Foundation::BSTR, bstrpassword: &super::super::Foundation::BSTR, bstrappcontainer: &super::super::Foundation::BSTR, bstrworkspaceparams: &super::super::Foundation::BSTR, ltimeout: i32, lflags: i32) -> ::windows::core::Result<()>;
    fn ResourceDismissed(&mut self, bstrworkspaceid: &super::super::Foundation::BSTR, bstrworkspacefriendlyname: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IWorkspaceScriptable2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWorkspaceScriptable2_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWorkspaceScriptable2_Vtbl {
        unsafe extern "system" fn StartWorkspaceEx<Impl: IWorkspaceScriptable2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrworkspaceid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrworkspacefriendlyname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrredirectorname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrusername: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrpassword: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrappcontainer: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrworkspaceparams: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ltimeout: i32, lflags: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).StartWorkspaceEx(::core::mem::transmute_copy(&bstrworkspaceid), ::core::mem::transmute_copy(&bstrworkspacefriendlyname), ::core::mem::transmute_copy(&bstrredirectorname), ::core::mem::transmute_copy(&bstrusername), ::core::mem::transmute_copy(&bstrpassword), ::core::mem::transmute_copy(&bstrappcontainer), ::core::mem::transmute_copy(&bstrworkspaceparams), ::core::mem::transmute_copy(&ltimeout), ::core::mem::transmute_copy(&lflags)).into()
        }
        unsafe extern "system" fn ResourceDismissed<Impl: IWorkspaceScriptable2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrworkspaceid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrworkspacefriendlyname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ResourceDismissed(::core::mem::transmute_copy(&bstrworkspaceid), ::core::mem::transmute_copy(&bstrworkspacefriendlyname)).into()
        }
        Self {
            base: IWorkspaceScriptable_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            StartWorkspaceEx: StartWorkspaceEx::<Impl, IMPL_OFFSET>,
            ResourceDismissed: ResourceDismissed::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWorkspaceScriptable2 as ::windows::core::Interface>::IID || iid == &<super::Com::IDispatch as ::windows::core::Interface>::IID || iid == &<IWorkspaceScriptable as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IWorkspaceScriptable3_Impl: Sized + super::Com::IDispatch_Impl + IWorkspaceScriptable_Impl + IWorkspaceScriptable2_Impl {
    fn StartWorkspaceEx2(&mut self, bstrworkspaceid: &super::super::Foundation::BSTR, bstrworkspacefriendlyname: &super::super::Foundation::BSTR, bstrredirectorname: &super::super::Foundation::BSTR, bstrusername: &super::super::Foundation::BSTR, bstrpassword: &super::super::Foundation::BSTR, bstrappcontainer: &super::super::Foundation::BSTR, bstrworkspaceparams: &super::super::Foundation::BSTR, ltimeout: i32, lflags: i32, bstreventloguploadaddress: &super::super::Foundation::BSTR, correlationid: &::windows::core::GUID) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IWorkspaceScriptable3_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWorkspaceScriptable3_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWorkspaceScriptable3_Vtbl {
        unsafe extern "system" fn StartWorkspaceEx2<Impl: IWorkspaceScriptable3_Impl, const OFFSET: isize>(
            this: *mut ::core::ffi::c_void,
            bstrworkspaceid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>,
            bstrworkspacefriendlyname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>,
            bstrredirectorname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>,
            bstrusername: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>,
            bstrpassword: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>,
            bstrappcontainer: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>,
            bstrworkspaceparams: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>,
            ltimeout: i32,
            lflags: i32,
            bstreventloguploadaddress: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>,
            correlationid: ::windows::core::GUID,
        ) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this)
                .StartWorkspaceEx2(
                    ::core::mem::transmute_copy(&bstrworkspaceid),
                    ::core::mem::transmute_copy(&bstrworkspacefriendlyname),
                    ::core::mem::transmute_copy(&bstrredirectorname),
                    ::core::mem::transmute_copy(&bstrusername),
                    ::core::mem::transmute_copy(&bstrpassword),
                    ::core::mem::transmute_copy(&bstrappcontainer),
                    ::core::mem::transmute_copy(&bstrworkspaceparams),
                    ::core::mem::transmute_copy(&ltimeout),
                    ::core::mem::transmute_copy(&lflags),
                    ::core::mem::transmute_copy(&bstreventloguploadaddress),
                    ::core::mem::transmute_copy(&correlationid),
                )
                .into()
        }
        Self { base: IWorkspaceScriptable2_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(), StartWorkspaceEx2: StartWorkspaceEx2::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWorkspaceScriptable3 as ::windows::core::Interface>::IID || iid == &<super::Com::IDispatch as ::windows::core::Interface>::IID || iid == &<IWorkspaceScriptable as ::windows::core::Interface>::IID || iid == &<IWorkspaceScriptable2 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait ItsPubPlugin_Impl: Sized {
    fn GetResourceList(&mut self, userid: super::super::Foundation::PWSTR, pceapplistsize: *mut i32, resourcelist: *mut *mut pluginResource) -> ::windows::core::Result<()>;
    fn GetResource(&mut self, alias: super::super::Foundation::PWSTR, flags: i32) -> ::windows::core::Result<pluginResource>;
    fn GetCacheLastUpdateTime(&mut self) -> ::windows::core::Result<u64>;
    fn pluginName(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn pluginVersion(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn ResolveResource(&mut self, resourcetype: *mut u32, resourcelocation: super::super::Foundation::PWSTR, endpointname: super::super::Foundation::PWSTR, userid: super::super::Foundation::PWSTR, alias: super::super::Foundation::PWSTR) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ItsPubPlugin_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ItsPubPlugin_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ItsPubPlugin_Vtbl {
        unsafe extern "system" fn GetResourceList<Impl: ItsPubPlugin_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, userid: super::super::Foundation::PWSTR, pceapplistsize: *mut i32, resourcelist: *mut *mut pluginResource) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetResourceList(::core::mem::transmute_copy(&userid), ::core::mem::transmute_copy(&pceapplistsize), ::core::mem::transmute_copy(&resourcelist)).into()
        }
        unsafe extern "system" fn GetResource<Impl: ItsPubPlugin_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, alias: super::super::Foundation::PWSTR, flags: i32, resource: *mut pluginResource) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetResource(::core::mem::transmute_copy(&alias), ::core::mem::transmute_copy(&flags)) {
                ::core::result::Result::Ok(ok__) => {
                    *resource = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCacheLastUpdateTime<Impl: ItsPubPlugin_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lastupdatetime: *mut u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetCacheLastUpdateTime() {
                ::core::result::Result::Ok(ok__) => {
                    *lastupdatetime = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn pluginName<Impl: ItsPubPlugin_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).pluginName() {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn pluginVersion<Impl: ItsPubPlugin_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).pluginVersion() {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ResolveResource<Impl: ItsPubPlugin_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, resourcetype: *mut u32, resourcelocation: super::super::Foundation::PWSTR, endpointname: super::super::Foundation::PWSTR, userid: super::super::Foundation::PWSTR, alias: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ResolveResource(::core::mem::transmute_copy(&resourcetype), ::core::mem::transmute_copy(&resourcelocation), ::core::mem::transmute_copy(&endpointname), ::core::mem::transmute_copy(&userid), ::core::mem::transmute_copy(&alias)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            GetResourceList: GetResourceList::<Impl, IMPL_OFFSET>,
            GetResource: GetResource::<Impl, IMPL_OFFSET>,
            GetCacheLastUpdateTime: GetCacheLastUpdateTime::<Impl, IMPL_OFFSET>,
            pluginName: pluginName::<Impl, IMPL_OFFSET>,
            pluginVersion: pluginVersion::<Impl, IMPL_OFFSET>,
            ResolveResource: ResolveResource::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ItsPubPlugin as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait ItsPubPlugin2_Impl: Sized + ItsPubPlugin_Impl {
    fn GetResource2List(&mut self, userid: super::super::Foundation::PWSTR, pceapplistsize: *mut i32, resourcelist: *mut *mut pluginResource2) -> ::windows::core::Result<()>;
    fn GetResource2(&mut self, alias: super::super::Foundation::PWSTR, flags: i32) -> ::windows::core::Result<pluginResource2>;
    fn ResolvePersonalDesktop(&mut self, userid: super::super::Foundation::PWSTR, poolid: super::super::Foundation::PWSTR, epdresolutiontype: TSPUB_PLUGIN_PD_RESOLUTION_TYPE, ppdassignmenttype: *mut TSPUB_PLUGIN_PD_ASSIGNMENT_TYPE, endpointname: super::super::Foundation::PWSTR) -> ::windows::core::Result<()>;
    fn DeletePersonalDesktopAssignment(&mut self, userid: super::super::Foundation::PWSTR, poolid: super::super::Foundation::PWSTR, endpointname: super::super::Foundation::PWSTR) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ItsPubPlugin2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ItsPubPlugin2_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ItsPubPlugin2_Vtbl {
        unsafe extern "system" fn GetResource2List<Impl: ItsPubPlugin2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, userid: super::super::Foundation::PWSTR, pceapplistsize: *mut i32, resourcelist: *mut *mut pluginResource2) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetResource2List(::core::mem::transmute_copy(&userid), ::core::mem::transmute_copy(&pceapplistsize), ::core::mem::transmute_copy(&resourcelist)).into()
        }
        unsafe extern "system" fn GetResource2<Impl: ItsPubPlugin2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, alias: super::super::Foundation::PWSTR, flags: i32, resource: *mut pluginResource2) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetResource2(::core::mem::transmute_copy(&alias), ::core::mem::transmute_copy(&flags)) {
                ::core::result::Result::Ok(ok__) => {
                    *resource = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ResolvePersonalDesktop<Impl: ItsPubPlugin2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, userid: super::super::Foundation::PWSTR, poolid: super::super::Foundation::PWSTR, epdresolutiontype: TSPUB_PLUGIN_PD_RESOLUTION_TYPE, ppdassignmenttype: *mut TSPUB_PLUGIN_PD_ASSIGNMENT_TYPE, endpointname: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ResolvePersonalDesktop(::core::mem::transmute_copy(&userid), ::core::mem::transmute_copy(&poolid), ::core::mem::transmute_copy(&epdresolutiontype), ::core::mem::transmute_copy(&ppdassignmenttype), ::core::mem::transmute_copy(&endpointname)).into()
        }
        unsafe extern "system" fn DeletePersonalDesktopAssignment<Impl: ItsPubPlugin2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, userid: super::super::Foundation::PWSTR, poolid: super::super::Foundation::PWSTR, endpointname: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).DeletePersonalDesktopAssignment(::core::mem::transmute_copy(&userid), ::core::mem::transmute_copy(&poolid), ::core::mem::transmute_copy(&endpointname)).into()
        }
        Self {
            base: ItsPubPlugin_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            GetResource2List: GetResource2List::<Impl, IMPL_OFFSET>,
            GetResource2: GetResource2::<Impl, IMPL_OFFSET>,
            ResolvePersonalDesktop: ResolvePersonalDesktop::<Impl, IMPL_OFFSET>,
            DeletePersonalDesktopAssignment: DeletePersonalDesktopAssignment::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ItsPubPlugin2 as ::windows::core::Interface>::IID || iid == &<ItsPubPlugin as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait _ITSWkspEvents_Impl: Sized + super::Com::IDispatch_Impl {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl _ITSWkspEvents_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: _ITSWkspEvents_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> _ITSWkspEvents_Vtbl {
        Self { base: super::Com::IDispatch_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<_ITSWkspEvents as ::windows::core::Interface>::IID || iid == &<super::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
