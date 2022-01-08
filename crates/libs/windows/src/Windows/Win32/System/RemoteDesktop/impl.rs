#[cfg(feature = "Win32_System_Com")]
pub trait IADsTSUserExImpl: Sized + IDispatchImpl {
    fn TerminalServicesProfilePath();
    fn SetTerminalServicesProfilePath();
    fn TerminalServicesHomeDirectory();
    fn SetTerminalServicesHomeDirectory();
    fn TerminalServicesHomeDrive();
    fn SetTerminalServicesHomeDrive();
    fn AllowLogon();
    fn SetAllowLogon();
    fn EnableRemoteControl();
    fn SetEnableRemoteControl();
    fn MaxDisconnectionTime();
    fn SetMaxDisconnectionTime();
    fn MaxConnectionTime();
    fn SetMaxConnectionTime();
    fn MaxIdleTime();
    fn SetMaxIdleTime();
    fn ReconnectionAction();
    fn SetReconnectionAction();
    fn BrokenConnectionAction();
    fn SetBrokenConnectionAction();
    fn ConnectClientDrivesAtLogon();
    fn SetConnectClientDrivesAtLogon();
    fn ConnectClientPrintersAtLogon();
    fn SetConnectClientPrintersAtLogon();
    fn DefaultToMainPrinter();
    fn SetDefaultToMainPrinter();
    fn TerminalServicesWorkDirectory();
    fn SetTerminalServicesWorkDirectory();
    fn TerminalServicesInitialProgram();
    fn SetTerminalServicesInitialProgram();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for IADsTSUserEx {
    const NAME: &'static str = "Windows.Win32.System.RemoteDesktop.IADsTSUserEx";
}
#[cfg(feature = "Win32_System_Com")]
impl IADsTSUserExVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IADsTSUserExImpl, const OFFSET: isize>() -> IADsTSUserExVtbl {
        unsafe extern "system" fn TerminalServicesProfilePath<Impl: IADsTSUserExImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TerminalServicesProfilePath(::core::mem::transmute_copy(&pval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetTerminalServicesProfilePath<Impl: IADsTSUserExImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pnewval: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetTerminalServicesProfilePath(&*(&pnewval as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TerminalServicesHomeDirectory<Impl: IADsTSUserExImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TerminalServicesHomeDirectory(::core::mem::transmute_copy(&pval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetTerminalServicesHomeDirectory<Impl: IADsTSUserExImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pnewval: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetTerminalServicesHomeDirectory(&*(&pnewval as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TerminalServicesHomeDrive<Impl: IADsTSUserExImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TerminalServicesHomeDrive(::core::mem::transmute_copy(&pval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetTerminalServicesHomeDrive<Impl: IADsTSUserExImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pnewval: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetTerminalServicesHomeDrive(&*(&pnewval as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AllowLogon<Impl: IADsTSUserExImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AllowLogon(::core::mem::transmute_copy(&pval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAllowLogon<Impl: IADsTSUserExImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newval: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetAllowLogon(newval) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnableRemoteControl<Impl: IADsTSUserExImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EnableRemoteControl(::core::mem::transmute_copy(&pval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetEnableRemoteControl<Impl: IADsTSUserExImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newval: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetEnableRemoteControl(newval) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MaxDisconnectionTime<Impl: IADsTSUserExImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MaxDisconnectionTime(::core::mem::transmute_copy(&pval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMaxDisconnectionTime<Impl: IADsTSUserExImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newval: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetMaxDisconnectionTime(newval) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MaxConnectionTime<Impl: IADsTSUserExImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MaxConnectionTime(::core::mem::transmute_copy(&pval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMaxConnectionTime<Impl: IADsTSUserExImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newval: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetMaxConnectionTime(newval) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MaxIdleTime<Impl: IADsTSUserExImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MaxIdleTime(::core::mem::transmute_copy(&pval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMaxIdleTime<Impl: IADsTSUserExImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newval: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetMaxIdleTime(newval) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ReconnectionAction<Impl: IADsTSUserExImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pnewval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ReconnectionAction(::core::mem::transmute_copy(&pnewval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetReconnectionAction<Impl: IADsTSUserExImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newval: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetReconnectionAction(newval) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BrokenConnectionAction<Impl: IADsTSUserExImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pnewval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).BrokenConnectionAction(::core::mem::transmute_copy(&pnewval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetBrokenConnectionAction<Impl: IADsTSUserExImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newval: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetBrokenConnectionAction(newval) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ConnectClientDrivesAtLogon<Impl: IADsTSUserExImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pnewval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ConnectClientDrivesAtLogon(::core::mem::transmute_copy(&pnewval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetConnectClientDrivesAtLogon<Impl: IADsTSUserExImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newval: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetConnectClientDrivesAtLogon(newval) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ConnectClientPrintersAtLogon<Impl: IADsTSUserExImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ConnectClientPrintersAtLogon(::core::mem::transmute_copy(&pval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetConnectClientPrintersAtLogon<Impl: IADsTSUserExImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newval: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetConnectClientPrintersAtLogon(newval) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DefaultToMainPrinter<Impl: IADsTSUserExImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DefaultToMainPrinter(::core::mem::transmute_copy(&pval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDefaultToMainPrinter<Impl: IADsTSUserExImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newval: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetDefaultToMainPrinter(newval) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TerminalServicesWorkDirectory<Impl: IADsTSUserExImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TerminalServicesWorkDirectory(::core::mem::transmute_copy(&pval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetTerminalServicesWorkDirectory<Impl: IADsTSUserExImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pnewval: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetTerminalServicesWorkDirectory(&*(&pnewval as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TerminalServicesInitialProgram<Impl: IADsTSUserExImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TerminalServicesInitialProgram(::core::mem::transmute_copy(&pval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetTerminalServicesInitialProgram<Impl: IADsTSUserExImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pnewval: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetTerminalServicesInitialProgram(&*(&pnewval as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
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
            ::windows::core::GetRuntimeClassName::<IADsTSUserEx>,
            ::windows::core::GetTrustLevel,
            TerminalServicesProfilePath::<Impl, OFFSET>,
            SetTerminalServicesProfilePath::<Impl, OFFSET>,
            TerminalServicesHomeDirectory::<Impl, OFFSET>,
            SetTerminalServicesHomeDirectory::<Impl, OFFSET>,
            TerminalServicesHomeDrive::<Impl, OFFSET>,
            SetTerminalServicesHomeDrive::<Impl, OFFSET>,
            AllowLogon::<Impl, OFFSET>,
            SetAllowLogon::<Impl, OFFSET>,
            EnableRemoteControl::<Impl, OFFSET>,
            SetEnableRemoteControl::<Impl, OFFSET>,
            MaxDisconnectionTime::<Impl, OFFSET>,
            SetMaxDisconnectionTime::<Impl, OFFSET>,
            MaxConnectionTime::<Impl, OFFSET>,
            SetMaxConnectionTime::<Impl, OFFSET>,
            MaxIdleTime::<Impl, OFFSET>,
            SetMaxIdleTime::<Impl, OFFSET>,
            ReconnectionAction::<Impl, OFFSET>,
            SetReconnectionAction::<Impl, OFFSET>,
            BrokenConnectionAction::<Impl, OFFSET>,
            SetBrokenConnectionAction::<Impl, OFFSET>,
            ConnectClientDrivesAtLogon::<Impl, OFFSET>,
            SetConnectClientDrivesAtLogon::<Impl, OFFSET>,
            ConnectClientPrintersAtLogon::<Impl, OFFSET>,
            SetConnectClientPrintersAtLogon::<Impl, OFFSET>,
            DefaultToMainPrinter::<Impl, OFFSET>,
            SetDefaultToMainPrinter::<Impl, OFFSET>,
            TerminalServicesWorkDirectory::<Impl, OFFSET>,
            SetTerminalServicesWorkDirectory::<Impl, OFFSET>,
            TerminalServicesInitialProgram::<Impl, OFFSET>,
            SetTerminalServicesInitialProgram::<Impl, OFFSET>,
        )
    }
}
pub trait IAudioDeviceEndpointImpl: Sized {
    fn SetBuffer();
    fn GetRTCaps();
    fn GetEventDrivenCapable();
    fn WriteExclusiveModeParametersToSharedMemory();
}
impl ::windows::core::RuntimeName for IAudioDeviceEndpoint {
    const NAME: &'static str = "Windows.Win32.System.RemoteDesktop.IAudioDeviceEndpoint";
}
impl IAudioDeviceEndpointVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAudioDeviceEndpointImpl, const OFFSET: isize>() -> IAudioDeviceEndpointVtbl {
        unsafe extern "system" fn SetBuffer<Impl: IAudioDeviceEndpointImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, maxperiod: i64, u32latencycoefficient: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetBuffer(maxperiod, u32latencycoefficient) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetRTCaps<Impl: IAudioDeviceEndpointImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbisrtcapable: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetRTCaps(::core::mem::transmute_copy(&pbisrtcapable)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetEventDrivenCapable<Impl: IAudioDeviceEndpointImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbiseventcapable: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetEventDrivenCapable(::core::mem::transmute_copy(&pbiseventcapable)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn WriteExclusiveModeParametersToSharedMemory<Impl: IAudioDeviceEndpointImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, htargetprocess: usize, hnsperiod: i64, hnsbufferduration: i64, u32latencycoefficient: u32, pu32sharedmemorysize: *mut u32, phsharedmemory: *mut usize) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).WriteExclusiveModeParametersToSharedMemory(htargetprocess, hnsperiod, hnsbufferduration, u32latencycoefficient, ::core::mem::transmute_copy(&pu32sharedmemorysize), ::core::mem::transmute_copy(&phsharedmemory)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IAudioDeviceEndpoint>, ::windows::core::GetTrustLevel, SetBuffer::<Impl, OFFSET>, GetRTCaps::<Impl, OFFSET>, GetEventDrivenCapable::<Impl, OFFSET>, WriteExclusiveModeParametersToSharedMemory::<Impl, OFFSET>)
    }
}
pub trait IAudioEndpointImpl: Sized {
    fn GetFrameFormat();
    fn GetFramesPerPacket();
    fn GetLatency();
    fn SetStreamFlags();
    fn SetEventHandle();
}
impl ::windows::core::RuntimeName for IAudioEndpoint {
    const NAME: &'static str = "Windows.Win32.System.RemoteDesktop.IAudioEndpoint";
}
impl IAudioEndpointVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAudioEndpointImpl, const OFFSET: isize>() -> IAudioEndpointVtbl {
        unsafe extern "system" fn GetFrameFormat<Impl: IAudioEndpointImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppformat: *mut *mut super::super::Media::Audio::WAVEFORMATEX) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetFrameFormat(::core::mem::transmute_copy(&ppformat)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFramesPerPacket<Impl: IAudioEndpointImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pframesperpacket: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetFramesPerPacket(::core::mem::transmute_copy(&pframesperpacket)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetLatency<Impl: IAudioEndpointImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, platency: *mut i64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetLatency(::core::mem::transmute_copy(&platency)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetStreamFlags<Impl: IAudioEndpointImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, streamflags: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetStreamFlags(streamflags) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetEventHandle<Impl: IAudioEndpointImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, eventhandle: super::super::Foundation::HANDLE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetEventHandle(&*(&eventhandle as *const <super::super::Foundation::HANDLE as ::windows::core::Abi>::Abi as *const <super::super::Foundation::HANDLE as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IAudioEndpoint>, ::windows::core::GetTrustLevel, GetFrameFormat::<Impl, OFFSET>, GetFramesPerPacket::<Impl, OFFSET>, GetLatency::<Impl, OFFSET>, SetStreamFlags::<Impl, OFFSET>, SetEventHandle::<Impl, OFFSET>)
    }
}
pub trait IAudioEndpointControlImpl: Sized {
    fn Start();
    fn Reset();
    fn Stop();
}
impl ::windows::core::RuntimeName for IAudioEndpointControl {
    const NAME: &'static str = "Windows.Win32.System.RemoteDesktop.IAudioEndpointControl";
}
impl IAudioEndpointControlVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAudioEndpointControlImpl, const OFFSET: isize>() -> IAudioEndpointControlVtbl {
        unsafe extern "system" fn Start<Impl: IAudioEndpointControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Start() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Reset<Impl: IAudioEndpointControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Reset() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Stop<Impl: IAudioEndpointControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Stop() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IAudioEndpointControl>, ::windows::core::GetTrustLevel, Start::<Impl, OFFSET>, Reset::<Impl, OFFSET>, Stop::<Impl, OFFSET>)
    }
}
pub trait IAudioEndpointRTImpl: Sized {
    fn GetCurrentPadding();
    fn ProcessingComplete();
    fn SetPinInactive();
    fn SetPinActive();
}
impl ::windows::core::RuntimeName for IAudioEndpointRT {
    const NAME: &'static str = "Windows.Win32.System.RemoteDesktop.IAudioEndpointRT";
}
impl IAudioEndpointRTVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAudioEndpointRTImpl, const OFFSET: isize>() -> IAudioEndpointRTVtbl {
        unsafe extern "system" fn GetCurrentPadding<Impl: IAudioEndpointRTImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppadding: *mut i64, paecurrentposition: *mut AE_CURRENT_POSITION) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetCurrentPadding(::core::mem::transmute_copy(&ppadding), ::core::mem::transmute_copy(&paecurrentposition)).into()
        }
        unsafe extern "system" fn ProcessingComplete<Impl: IAudioEndpointRTImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ProcessingComplete().into()
        }
        unsafe extern "system" fn SetPinInactive<Impl: IAudioEndpointRTImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetPinInactive() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPinActive<Impl: IAudioEndpointRTImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetPinActive() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IAudioEndpointRT>, ::windows::core::GetTrustLevel, GetCurrentPadding::<Impl, OFFSET>, ProcessingComplete::<Impl, OFFSET>, SetPinInactive::<Impl, OFFSET>, SetPinActive::<Impl, OFFSET>)
    }
}
pub trait IAudioInputEndpointRTImpl: Sized {
    fn GetInputDataPointer();
    fn ReleaseInputDataPointer();
    fn PulseEndpoint();
}
impl ::windows::core::RuntimeName for IAudioInputEndpointRT {
    const NAME: &'static str = "Windows.Win32.System.RemoteDesktop.IAudioInputEndpointRT";
}
impl IAudioInputEndpointRTVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAudioInputEndpointRTImpl, const OFFSET: isize>() -> IAudioInputEndpointRTVtbl {
        unsafe extern "system" fn GetInputDataPointer<Impl: IAudioInputEndpointRTImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pconnectionproperty: *mut super::super::Media::Audio::Apo::APO_CONNECTION_PROPERTY, paetimestamp: *mut AE_CURRENT_POSITION) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetInputDataPointer(&*(&pconnectionproperty as *const <super::super::Media::Audio::Apo::APO_CONNECTION_PROPERTY as ::windows::core::Abi>::Abi as *const <super::super::Media::Audio::Apo::APO_CONNECTION_PROPERTY as ::windows::core::DefaultType>::DefaultType), &*(&paetimestamp as *const <AE_CURRENT_POSITION as ::windows::core::Abi>::Abi as *const <AE_CURRENT_POSITION as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn ReleaseInputDataPointer<Impl: IAudioInputEndpointRTImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, u32framecount: u32, pdatapointer: usize) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ReleaseInputDataPointer(u32framecount, pdatapointer).into()
        }
        unsafe extern "system" fn PulseEndpoint<Impl: IAudioInputEndpointRTImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).PulseEndpoint().into()
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IAudioInputEndpointRT>, ::windows::core::GetTrustLevel, GetInputDataPointer::<Impl, OFFSET>, ReleaseInputDataPointer::<Impl, OFFSET>, PulseEndpoint::<Impl, OFFSET>)
    }
}
pub trait IAudioOutputEndpointRTImpl: Sized {
    fn GetOutputDataPointer();
    fn ReleaseOutputDataPointer();
    fn PulseEndpoint();
}
impl ::windows::core::RuntimeName for IAudioOutputEndpointRT {
    const NAME: &'static str = "Windows.Win32.System.RemoteDesktop.IAudioOutputEndpointRT";
}
impl IAudioOutputEndpointRTVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAudioOutputEndpointRTImpl, const OFFSET: isize>() -> IAudioOutputEndpointRTVtbl {
        unsafe extern "system" fn GetOutputDataPointer<Impl: IAudioOutputEndpointRTImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, u32framecount: u32, paetimestamp: *const AE_CURRENT_POSITION) -> usize {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetOutputDataPointer(u32framecount, &*(&paetimestamp as *const <AE_CURRENT_POSITION as ::windows::core::Abi>::Abi as *const <AE_CURRENT_POSITION as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ReleaseOutputDataPointer<Impl: IAudioOutputEndpointRTImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pconnectionproperty: *const super::super::Media::Audio::Apo::APO_CONNECTION_PROPERTY) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ReleaseOutputDataPointer(&*(&pconnectionproperty as *const <super::super::Media::Audio::Apo::APO_CONNECTION_PROPERTY as ::windows::core::Abi>::Abi as *const <super::super::Media::Audio::Apo::APO_CONNECTION_PROPERTY as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn PulseEndpoint<Impl: IAudioOutputEndpointRTImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).PulseEndpoint().into()
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IAudioOutputEndpointRT>, ::windows::core::GetTrustLevel, GetOutputDataPointer::<Impl, OFFSET>, ReleaseOutputDataPointer::<Impl, OFFSET>, PulseEndpoint::<Impl, OFFSET>)
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IRemoteDesktopClientImpl: Sized + IDispatchImpl {
    fn Connect();
    fn Disconnect();
    fn Reconnect();
    fn Settings();
    fn Actions();
    fn TouchPointer();
    fn DeleteSavedCredentials();
    fn UpdateSessionDisplaySettings();
    fn attachEvent();
    fn detachEvent();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for IRemoteDesktopClient {
    const NAME: &'static str = "Windows.Win32.System.RemoteDesktop.IRemoteDesktopClient";
}
#[cfg(feature = "Win32_System_Com")]
impl IRemoteDesktopClientVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRemoteDesktopClientImpl, const OFFSET: isize>() -> IRemoteDesktopClientVtbl {
        unsafe extern "system" fn Connect<Impl: IRemoteDesktopClientImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Connect() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Disconnect<Impl: IRemoteDesktopClientImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Disconnect() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Reconnect<Impl: IRemoteDesktopClientImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, width: u32, height: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Reconnect(width, height) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Settings<Impl: IRemoteDesktopClientImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, settings: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Settings(::core::mem::transmute_copy(&settings)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Actions<Impl: IRemoteDesktopClientImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, actions: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Actions(::core::mem::transmute_copy(&actions)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TouchPointer<Impl: IRemoteDesktopClientImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, touchpointer: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TouchPointer(::core::mem::transmute_copy(&touchpointer)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DeleteSavedCredentials<Impl: IRemoteDesktopClientImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, servername: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DeleteSavedCredentials(&*(&servername as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UpdateSessionDisplaySettings<Impl: IRemoteDesktopClientImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, width: u32, height: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).UpdateSessionDisplaySettings(width, height) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn attachEvent<Impl: IRemoteDesktopClientImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, eventname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, callback: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).attachEvent(&*(&eventname as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), &*(&callback as *const <super::Com::IDispatch as ::windows::core::Abi>::Abi as *const <super::Com::IDispatch as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn detachEvent<Impl: IRemoteDesktopClientImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, eventname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, callback: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).detachEvent(&*(&eventname as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), &*(&callback as *const <super::Com::IDispatch as ::windows::core::Abi>::Abi as *const <super::Com::IDispatch as ::windows::core::DefaultType>::DefaultType)) {
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
            ::windows::core::GetRuntimeClassName::<IRemoteDesktopClient>,
            ::windows::core::GetTrustLevel,
            Connect::<Impl, OFFSET>,
            Disconnect::<Impl, OFFSET>,
            Reconnect::<Impl, OFFSET>,
            Settings::<Impl, OFFSET>,
            Actions::<Impl, OFFSET>,
            TouchPointer::<Impl, OFFSET>,
            DeleteSavedCredentials::<Impl, OFFSET>,
            UpdateSessionDisplaySettings::<Impl, OFFSET>,
            attachEvent::<Impl, OFFSET>,
            detachEvent::<Impl, OFFSET>,
        )
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IRemoteDesktopClientActionsImpl: Sized + IDispatchImpl {
    fn SuspendScreenUpdates();
    fn ResumeScreenUpdates();
    fn ExecuteRemoteAction();
    fn GetSnapshot();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for IRemoteDesktopClientActions {
    const NAME: &'static str = "Windows.Win32.System.RemoteDesktop.IRemoteDesktopClientActions";
}
#[cfg(feature = "Win32_System_Com")]
impl IRemoteDesktopClientActionsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRemoteDesktopClientActionsImpl, const OFFSET: isize>() -> IRemoteDesktopClientActionsVtbl {
        unsafe extern "system" fn SuspendScreenUpdates<Impl: IRemoteDesktopClientActionsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SuspendScreenUpdates() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ResumeScreenUpdates<Impl: IRemoteDesktopClientActionsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ResumeScreenUpdates() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ExecuteRemoteAction<Impl: IRemoteDesktopClientActionsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, remoteaction: RemoteActionType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ExecuteRemoteAction(remoteaction) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSnapshot<Impl: IRemoteDesktopClientActionsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, snapshotencoding: SnapshotEncodingType, snapshotformat: SnapshotFormatType, snapshotwidth: u32, snapshotheight: u32, snapshotdata: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetSnapshot(snapshotencoding, snapshotformat, snapshotwidth, snapshotheight, ::core::mem::transmute_copy(&snapshotdata)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IRemoteDesktopClientActions>, ::windows::core::GetTrustLevel, SuspendScreenUpdates::<Impl, OFFSET>, ResumeScreenUpdates::<Impl, OFFSET>, ExecuteRemoteAction::<Impl, OFFSET>, GetSnapshot::<Impl, OFFSET>)
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IRemoteDesktopClientSettingsImpl: Sized + IDispatchImpl {
    fn ApplySettings();
    fn RetrieveSettings();
    fn GetRdpProperty();
    fn SetRdpProperty();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for IRemoteDesktopClientSettings {
    const NAME: &'static str = "Windows.Win32.System.RemoteDesktop.IRemoteDesktopClientSettings";
}
#[cfg(feature = "Win32_System_Com")]
impl IRemoteDesktopClientSettingsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRemoteDesktopClientSettingsImpl, const OFFSET: isize>() -> IRemoteDesktopClientSettingsVtbl {
        unsafe extern "system" fn ApplySettings<Impl: IRemoteDesktopClientSettingsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rdpfilecontents: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ApplySettings(&*(&rdpfilecontents as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RetrieveSettings<Impl: IRemoteDesktopClientSettingsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rdpfilecontents: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RetrieveSettings(::core::mem::transmute_copy(&rdpfilecontents)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetRdpProperty<Impl: IRemoteDesktopClientSettingsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, propertyname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, value: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetRdpProperty(&*(&propertyname as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&value)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetRdpProperty<Impl: IRemoteDesktopClientSettingsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, propertyname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, value: ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetRdpProperty(&*(&propertyname as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), &*(&value as *const <super::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::Com::VARIANT as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IRemoteDesktopClientSettings>, ::windows::core::GetTrustLevel, ApplySettings::<Impl, OFFSET>, RetrieveSettings::<Impl, OFFSET>, GetRdpProperty::<Impl, OFFSET>, SetRdpProperty::<Impl, OFFSET>)
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IRemoteDesktopClientTouchPointerImpl: Sized + IDispatchImpl {
    fn SetEnabled();
    fn Enabled();
    fn SetEventsEnabled();
    fn EventsEnabled();
    fn SetPointerSpeed();
    fn PointerSpeed();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for IRemoteDesktopClientTouchPointer {
    const NAME: &'static str = "Windows.Win32.System.RemoteDesktop.IRemoteDesktopClientTouchPointer";
}
#[cfg(feature = "Win32_System_Com")]
impl IRemoteDesktopClientTouchPointerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRemoteDesktopClientTouchPointerImpl, const OFFSET: isize>() -> IRemoteDesktopClientTouchPointerVtbl {
        unsafe extern "system" fn SetEnabled<Impl: IRemoteDesktopClientTouchPointerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, enabled: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetEnabled(enabled) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Enabled<Impl: IRemoteDesktopClientTouchPointerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, enabled: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Enabled(::core::mem::transmute_copy(&enabled)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetEventsEnabled<Impl: IRemoteDesktopClientTouchPointerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, eventsenabled: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetEventsEnabled(eventsenabled) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EventsEnabled<Impl: IRemoteDesktopClientTouchPointerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, eventsenabled: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EventsEnabled(::core::mem::transmute_copy(&eventsenabled)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPointerSpeed<Impl: IRemoteDesktopClientTouchPointerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pointerspeed: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetPointerSpeed(pointerspeed) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PointerSpeed<Impl: IRemoteDesktopClientTouchPointerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pointerspeed: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PointerSpeed(::core::mem::transmute_copy(&pointerspeed)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IRemoteDesktopClientTouchPointer>, ::windows::core::GetTrustLevel, SetEnabled::<Impl, OFFSET>, Enabled::<Impl, OFFSET>, SetEventsEnabled::<Impl, OFFSET>, EventsEnabled::<Impl, OFFSET>, SetPointerSpeed::<Impl, OFFSET>, PointerSpeed::<Impl, OFFSET>)
    }
}
pub trait IRemoteSystemAdditionalInfoProviderImpl: Sized {
    fn GetAdditionalInfo();
}
impl ::windows::core::RuntimeName for IRemoteSystemAdditionalInfoProvider {
    const NAME: &'static str = "Windows.Win32.System.RemoteDesktop.IRemoteSystemAdditionalInfoProvider";
}
impl IRemoteSystemAdditionalInfoProviderVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRemoteSystemAdditionalInfoProviderImpl, const OFFSET: isize>() -> IRemoteSystemAdditionalInfoProviderVtbl {
        unsafe extern "system" fn GetAdditionalInfo<Impl: IRemoteSystemAdditionalInfoProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, deduplicationid: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>, riid: &::windows::core::GUID, mapview: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetAdditionalInfo(::core::mem::transmute_copy(&deduplicationid), &*(&riid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&mapview)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IRemoteSystemAdditionalInfoProvider>, ::windows::core::GetTrustLevel, GetAdditionalInfo::<Impl, OFFSET>)
    }
}
pub trait ITSGAccountingEngineImpl: Sized {
    fn DoAccounting();
}
impl ::windows::core::RuntimeName for ITSGAccountingEngine {
    const NAME: &'static str = "Windows.Win32.System.RemoteDesktop.ITSGAccountingEngine";
}
impl ITSGAccountingEngineVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITSGAccountingEngineImpl, const OFFSET: isize>() -> ITSGAccountingEngineVtbl {
        unsafe extern "system" fn DoAccounting<Impl: ITSGAccountingEngineImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, accountingdatatype: AAAccountingDataType, accountingdata: ::core::mem::ManuallyDrop<AAAccountingData>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DoAccounting(accountingdatatype, &*(&accountingdata as *const <AAAccountingData as ::windows::core::Abi>::Abi as *const <AAAccountingData as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ITSGAccountingEngine>, ::windows::core::GetTrustLevel, DoAccounting::<Impl, OFFSET>)
    }
}
pub trait ITSGAuthenticateUserSinkImpl: Sized {
    fn OnUserAuthenticated();
    fn OnUserAuthenticationFailed();
    fn ReauthenticateUser();
    fn DisconnectUser();
}
impl ::windows::core::RuntimeName for ITSGAuthenticateUserSink {
    const NAME: &'static str = "Windows.Win32.System.RemoteDesktop.ITSGAuthenticateUserSink";
}
impl ITSGAuthenticateUserSinkVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITSGAuthenticateUserSinkImpl, const OFFSET: isize>() -> ITSGAuthenticateUserSinkVtbl {
        unsafe extern "system" fn OnUserAuthenticated<Impl: ITSGAuthenticateUserSinkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, username: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, userdomain: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, context: usize, usertoken: super::super::Foundation::HANDLE_PTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OnUserAuthenticated(
                &*(&username as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&userdomain as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType),
                context,
                &*(&usertoken as *const <super::super::Foundation::HANDLE_PTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::HANDLE_PTR as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OnUserAuthenticationFailed<Impl: ITSGAuthenticateUserSinkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, context: usize, genericerrorcode: ::windows::core::HRESULT, specificerrorcode: ::windows::core::HRESULT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OnUserAuthenticationFailed(context, genericerrorcode, specificerrorcode) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ReauthenticateUser<Impl: ITSGAuthenticateUserSinkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, context: usize) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ReauthenticateUser(context) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DisconnectUser<Impl: ITSGAuthenticateUserSinkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, context: usize) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DisconnectUser(context) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ITSGAuthenticateUserSink>, ::windows::core::GetTrustLevel, OnUserAuthenticated::<Impl, OFFSET>, OnUserAuthenticationFailed::<Impl, OFFSET>, ReauthenticateUser::<Impl, OFFSET>, DisconnectUser::<Impl, OFFSET>)
    }
}
pub trait ITSGAuthenticationEngineImpl: Sized {
    fn AuthenticateUser();
    fn CancelAuthentication();
}
impl ::windows::core::RuntimeName for ITSGAuthenticationEngine {
    const NAME: &'static str = "Windows.Win32.System.RemoteDesktop.ITSGAuthenticationEngine";
}
impl ITSGAuthenticationEngineVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITSGAuthenticationEngineImpl, const OFFSET: isize>() -> ITSGAuthenticationEngineVtbl {
        unsafe extern "system" fn AuthenticateUser<Impl: ITSGAuthenticationEngineImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, mainsessionid: ::windows::core::GUID, cookiedata: *const u8, numcookiebytes: u32, context: usize, psink: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AuthenticateUser(&*(&mainsessionid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), cookiedata, numcookiebytes, context, &*(&psink as *const <ITSGAuthenticateUserSink as ::windows::core::Abi>::Abi as *const <ITSGAuthenticateUserSink as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CancelAuthentication<Impl: ITSGAuthenticationEngineImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, mainsessionid: ::windows::core::GUID, context: usize) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CancelAuthentication(&*(&mainsessionid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), context) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ITSGAuthenticationEngine>, ::windows::core::GetTrustLevel, AuthenticateUser::<Impl, OFFSET>, CancelAuthentication::<Impl, OFFSET>)
    }
}
pub trait ITSGAuthorizeConnectionSinkImpl: Sized {
    fn OnConnectionAuthorized();
}
impl ::windows::core::RuntimeName for ITSGAuthorizeConnectionSink {
    const NAME: &'static str = "Windows.Win32.System.RemoteDesktop.ITSGAuthorizeConnectionSink";
}
impl ITSGAuthorizeConnectionSinkVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITSGAuthorizeConnectionSinkImpl, const OFFSET: isize>() -> ITSGAuthorizeConnectionSinkVtbl {
        unsafe extern "system" fn OnConnectionAuthorized<Impl: ITSGAuthorizeConnectionSinkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hrin: ::windows::core::HRESULT, mainsessionid: ::windows::core::GUID, cbsohresponse: u32, pbsohresponse: *const u8, idletimeout: u32, sessiontimeout: u32, sessiontimeoutaction: SESSION_TIMEOUT_ACTION_TYPE, trustclass: AATrustClassID, policyattributes: *const u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OnConnectionAuthorized(hrin, &*(&mainsessionid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), cbsohresponse, pbsohresponse, idletimeout, sessiontimeout, sessiontimeoutaction, trustclass, policyattributes) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ITSGAuthorizeConnectionSink>, ::windows::core::GetTrustLevel, OnConnectionAuthorized::<Impl, OFFSET>)
    }
}
pub trait ITSGAuthorizeResourceSinkImpl: Sized {
    fn OnChannelAuthorized();
}
impl ::windows::core::RuntimeName for ITSGAuthorizeResourceSink {
    const NAME: &'static str = "Windows.Win32.System.RemoteDesktop.ITSGAuthorizeResourceSink";
}
impl ITSGAuthorizeResourceSinkVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITSGAuthorizeResourceSinkImpl, const OFFSET: isize>() -> ITSGAuthorizeResourceSinkVtbl {
        unsafe extern "system" fn OnChannelAuthorized<Impl: ITSGAuthorizeResourceSinkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hrin: ::windows::core::HRESULT, mainsessionid: ::windows::core::GUID, subsessionid: i32, allowedresourcenames: *const super::super::Foundation::BSTR, numallowedresourcenames: u32, failedresourcenames: *const super::super::Foundation::BSTR, numfailedresourcenames: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OnChannelAuthorized(
                hrin,
                &*(&mainsessionid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType),
                subsessionid,
                &*(&allowedresourcenames as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType),
                numallowedresourcenames,
                &*(&failedresourcenames as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType),
                numfailedresourcenames,
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ITSGAuthorizeResourceSink>, ::windows::core::GetTrustLevel, OnChannelAuthorized::<Impl, OFFSET>)
    }
}
pub trait ITSGPolicyEngineImpl: Sized {
    fn AuthorizeConnection();
    fn AuthorizeResource();
    fn Refresh();
    fn IsQuarantineEnabled();
}
impl ::windows::core::RuntimeName for ITSGPolicyEngine {
    const NAME: &'static str = "Windows.Win32.System.RemoteDesktop.ITSGPolicyEngine";
}
impl ITSGPolicyEngineVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITSGPolicyEngineImpl, const OFFSET: isize>() -> ITSGPolicyEngineVtbl {
        unsafe extern "system" fn AuthorizeConnection<Impl: ITSGPolicyEngineImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, mainsessionid: ::windows::core::GUID, username: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, authtype: AAAuthSchemes, clientmachineip: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, clientmachinename: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, sohdata: *const u8, numsohbytes: u32, cookiedata: *const u8, numcookiebytes: u32, usertoken: super::super::Foundation::HANDLE_PTR, psink: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AuthorizeConnection(
                &*(&mainsessionid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType),
                &*(&username as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType),
                authtype,
                &*(&clientmachineip as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&clientmachinename as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType),
                sohdata,
                numsohbytes,
                cookiedata,
                numcookiebytes,
                &*(&usertoken as *const <super::super::Foundation::HANDLE_PTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::HANDLE_PTR as ::windows::core::DefaultType>::DefaultType),
                &*(&psink as *const <ITSGAuthorizeConnectionSink as ::windows::core::Abi>::Abi as *const <ITSGAuthorizeConnectionSink as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AuthorizeResource<Impl: ITSGPolicyEngineImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, mainsessionid: ::windows::core::GUID, subsessionid: i32, username: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, resourcenames: *const super::super::Foundation::BSTR, numresources: u32, alternateresourcenames: *const super::super::Foundation::BSTR, numalternateresourcename: u32, portnumber: u32, operation: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, cookie: *const u8, numbytesincookie: u32, psink: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AuthorizeResource(
                &*(&mainsessionid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType),
                subsessionid,
                &*(&username as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&resourcenames as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType),
                numresources,
                &*(&alternateresourcenames as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType),
                numalternateresourcename,
                portnumber,
                &*(&operation as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType),
                cookie,
                numbytesincookie,
                &*(&psink as *const <ITSGAuthorizeResourceSink as ::windows::core::Abi>::Abi as *const <ITSGAuthorizeResourceSink as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Refresh<Impl: ITSGPolicyEngineImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn IsQuarantineEnabled<Impl: ITSGPolicyEngineImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, quarantineenabled: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsQuarantineEnabled(::core::mem::transmute_copy(&quarantineenabled)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ITSGPolicyEngine>, ::windows::core::GetTrustLevel, AuthorizeConnection::<Impl, OFFSET>, AuthorizeResource::<Impl, OFFSET>, Refresh::<Impl, OFFSET>, IsQuarantineEnabled::<Impl, OFFSET>)
    }
}
pub trait ITsSbBaseNotifySinkImpl: Sized {
    fn OnError();
    fn OnReportStatus();
}
impl ::windows::core::RuntimeName for ITsSbBaseNotifySink {
    const NAME: &'static str = "Windows.Win32.System.RemoteDesktop.ITsSbBaseNotifySink";
}
impl ITsSbBaseNotifySinkVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITsSbBaseNotifySinkImpl, const OFFSET: isize>() -> ITsSbBaseNotifySinkVtbl {
        unsafe extern "system" fn OnError<Impl: ITsSbBaseNotifySinkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hrerror: ::windows::core::HRESULT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OnError(hrerror) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OnReportStatus<Impl: ITsSbBaseNotifySinkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, messagetype: CLIENT_MESSAGE_TYPE, messageid: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OnReportStatus(messagetype, messageid) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ITsSbBaseNotifySink>, ::windows::core::GetTrustLevel, OnError::<Impl, OFFSET>, OnReportStatus::<Impl, OFFSET>)
    }
}
pub trait ITsSbClientConnectionImpl: Sized {
    fn UserName();
    fn Domain();
    fn InitialProgram();
    fn LoadBalanceResult();
    fn FarmName();
    fn PutContext();
    fn GetContext();
    fn Environment();
    fn ConnectionError();
    fn SamUserAccount();
    fn ClientConnectionPropertySet();
    fn IsFirstAssignment();
    fn RdFarmType();
    fn UserSidString();
    fn GetDisconnectedSession();
}
impl ::windows::core::RuntimeName for ITsSbClientConnection {
    const NAME: &'static str = "Windows.Win32.System.RemoteDesktop.ITsSbClientConnection";
}
impl ITsSbClientConnectionVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITsSbClientConnectionImpl, const OFFSET: isize>() -> ITsSbClientConnectionVtbl {
        unsafe extern "system" fn UserName<Impl: ITsSbClientConnectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).UserName(::core::mem::transmute_copy(&pval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Domain<Impl: ITsSbClientConnectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Domain(::core::mem::transmute_copy(&pval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn InitialProgram<Impl: ITsSbClientConnectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).InitialProgram(::core::mem::transmute_copy(&pval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LoadBalanceResult<Impl: ITsSbClientConnectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LoadBalanceResult(::core::mem::transmute_copy(&ppval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FarmName<Impl: ITsSbClientConnectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FarmName(::core::mem::transmute_copy(&pval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PutContext<Impl: ITsSbClientConnectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, contextid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, context: ::core::mem::ManuallyDrop<super::Com::VARIANT>, existingcontext: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PutContext(&*(&contextid as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), &*(&context as *const <super::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::Com::VARIANT as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&existingcontext)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetContext<Impl: ITsSbClientConnectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, contextid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, context: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetContext(&*(&contextid as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&context)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Environment<Impl: ITsSbClientConnectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenvironment: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Environment(::core::mem::transmute_copy(&ppenvironment)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ConnectionError<Impl: ITsSbClientConnectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ConnectionError() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SamUserAccount<Impl: ITsSbClientConnectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SamUserAccount(::core::mem::transmute_copy(&pval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ClientConnectionPropertySet<Impl: ITsSbClientConnectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pppropertyset: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ClientConnectionPropertySet(::core::mem::transmute_copy(&pppropertyset)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsFirstAssignment<Impl: ITsSbClientConnectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppval: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsFirstAssignment(::core::mem::transmute_copy(&ppval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RdFarmType<Impl: ITsSbClientConnectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, prdfarmtype: *mut RD_FARM_TYPE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RdFarmType(::core::mem::transmute_copy(&prdfarmtype)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UserSidString<Impl: ITsSbClientConnectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszusersidstring: *mut *mut i8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).UserSidString(::core::mem::transmute_copy(&pszusersidstring)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDisconnectedSession<Impl: ITsSbClientConnectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppsession: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDisconnectedSession(::core::mem::transmute_copy(&ppsession)) {
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
            ::windows::core::GetRuntimeClassName::<ITsSbClientConnection>,
            ::windows::core::GetTrustLevel,
            UserName::<Impl, OFFSET>,
            Domain::<Impl, OFFSET>,
            InitialProgram::<Impl, OFFSET>,
            LoadBalanceResult::<Impl, OFFSET>,
            FarmName::<Impl, OFFSET>,
            PutContext::<Impl, OFFSET>,
            GetContext::<Impl, OFFSET>,
            Environment::<Impl, OFFSET>,
            ConnectionError::<Impl, OFFSET>,
            SamUserAccount::<Impl, OFFSET>,
            ClientConnectionPropertySet::<Impl, OFFSET>,
            IsFirstAssignment::<Impl, OFFSET>,
            RdFarmType::<Impl, OFFSET>,
            UserSidString::<Impl, OFFSET>,
            GetDisconnectedSession::<Impl, OFFSET>,
        )
    }
}
#[cfg(feature = "Win32_System_Com_StructuredStorage")]
pub trait ITsSbClientConnectionPropertySetImpl: Sized + ITsSbPropertySetImpl + IPropertyBagImpl {}
#[cfg(feature = "Win32_System_Com_StructuredStorage")]
impl ::windows::core::RuntimeName for ITsSbClientConnectionPropertySet {
    const NAME: &'static str = "Windows.Win32.System.RemoteDesktop.ITsSbClientConnectionPropertySet";
}
#[cfg(feature = "Win32_System_Com_StructuredStorage")]
impl ITsSbClientConnectionPropertySetVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITsSbClientConnectionPropertySetImpl, const OFFSET: isize>() -> ITsSbClientConnectionPropertySetVtbl {
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ITsSbClientConnectionPropertySet>, ::windows::core::GetTrustLevel)
    }
}
pub trait ITsSbEnvironmentImpl: Sized {
    fn Name();
    fn ServerWeight();
    fn EnvironmentPropertySet();
    fn SetEnvironmentPropertySet();
}
impl ::windows::core::RuntimeName for ITsSbEnvironment {
    const NAME: &'static str = "Windows.Win32.System.RemoteDesktop.ITsSbEnvironment";
}
impl ITsSbEnvironmentVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITsSbEnvironmentImpl, const OFFSET: isize>() -> ITsSbEnvironmentVtbl {
        unsafe extern "system" fn Name<Impl: ITsSbEnvironmentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Name(::core::mem::transmute_copy(&pval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ServerWeight<Impl: ITsSbEnvironmentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ServerWeight(::core::mem::transmute_copy(&pval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnvironmentPropertySet<Impl: ITsSbEnvironmentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pppropertyset: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EnvironmentPropertySet(::core::mem::transmute_copy(&pppropertyset)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetEnvironmentPropertySet<Impl: ITsSbEnvironmentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetEnvironmentPropertySet(&*(&pval as *const <ITsSbEnvironmentPropertySet as ::windows::core::Abi>::Abi as *const <ITsSbEnvironmentPropertySet as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ITsSbEnvironment>, ::windows::core::GetTrustLevel, Name::<Impl, OFFSET>, ServerWeight::<Impl, OFFSET>, EnvironmentPropertySet::<Impl, OFFSET>, SetEnvironmentPropertySet::<Impl, OFFSET>)
    }
}
#[cfg(feature = "Win32_System_Com_StructuredStorage")]
pub trait ITsSbEnvironmentPropertySetImpl: Sized + ITsSbPropertySetImpl + IPropertyBagImpl {}
#[cfg(feature = "Win32_System_Com_StructuredStorage")]
impl ::windows::core::RuntimeName for ITsSbEnvironmentPropertySet {
    const NAME: &'static str = "Windows.Win32.System.RemoteDesktop.ITsSbEnvironmentPropertySet";
}
#[cfg(feature = "Win32_System_Com_StructuredStorage")]
impl ITsSbEnvironmentPropertySetVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITsSbEnvironmentPropertySetImpl, const OFFSET: isize>() -> ITsSbEnvironmentPropertySetVtbl {
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ITsSbEnvironmentPropertySet>, ::windows::core::GetTrustLevel)
    }
}
pub trait ITsSbFilterPluginStoreImpl: Sized {
    fn SaveProperties();
    fn EnumerateProperties();
    fn DeleteProperties();
}
impl ::windows::core::RuntimeName for ITsSbFilterPluginStore {
    const NAME: &'static str = "Windows.Win32.System.RemoteDesktop.ITsSbFilterPluginStore";
}
impl ITsSbFilterPluginStoreVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITsSbFilterPluginStoreImpl, const OFFSET: isize>() -> ITsSbFilterPluginStoreVtbl {
        unsafe extern "system" fn SaveProperties<Impl: ITsSbFilterPluginStoreImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppropertyset: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SaveProperties(&*(&ppropertyset as *const <ITsSbPropertySet as ::windows::core::Abi>::Abi as *const <ITsSbPropertySet as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnumerateProperties<Impl: ITsSbFilterPluginStoreImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pppropertyset: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EnumerateProperties(::core::mem::transmute_copy(&pppropertyset)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DeleteProperties<Impl: ITsSbFilterPluginStoreImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, propertyname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DeleteProperties(&*(&propertyname as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ITsSbFilterPluginStore>, ::windows::core::GetTrustLevel, SaveProperties::<Impl, OFFSET>, EnumerateProperties::<Impl, OFFSET>, DeleteProperties::<Impl, OFFSET>)
    }
}
pub trait ITsSbGenericNotifySinkImpl: Sized {
    fn OnCompleted();
    fn GetWaitTimeout();
}
impl ::windows::core::RuntimeName for ITsSbGenericNotifySink {
    const NAME: &'static str = "Windows.Win32.System.RemoteDesktop.ITsSbGenericNotifySink";
}
impl ITsSbGenericNotifySinkVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITsSbGenericNotifySinkImpl, const OFFSET: isize>() -> ITsSbGenericNotifySinkVtbl {
        unsafe extern "system" fn OnCompleted<Impl: ITsSbGenericNotifySinkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, status: ::windows::core::HRESULT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OnCompleted(status) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetWaitTimeout<Impl: ITsSbGenericNotifySinkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfttimeout: *mut super::super::Foundation::FILETIME) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetWaitTimeout(::core::mem::transmute_copy(&pfttimeout)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ITsSbGenericNotifySink>, ::windows::core::GetTrustLevel, OnCompleted::<Impl, OFFSET>, GetWaitTimeout::<Impl, OFFSET>)
    }
}
pub trait ITsSbGlobalStoreImpl: Sized {
    fn QueryTarget();
    fn QuerySessionBySessionId();
    fn EnumerateFarms();
    fn EnumerateTargets();
    fn EnumerateEnvironmentsByProvider();
    fn EnumerateSessions();
    fn GetFarmProperty();
}
impl ::windows::core::RuntimeName for ITsSbGlobalStore {
    const NAME: &'static str = "Windows.Win32.System.RemoteDesktop.ITsSbGlobalStore";
}
impl ITsSbGlobalStoreVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITsSbGlobalStoreImpl, const OFFSET: isize>() -> ITsSbGlobalStoreVtbl {
        unsafe extern "system" fn QueryTarget<Impl: ITsSbGlobalStoreImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, providername: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, targetname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, farmname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pptarget: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).QueryTarget(
                &*(&providername as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&targetname as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&farmname as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType),
                ::core::mem::transmute_copy(&pptarget),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn QuerySessionBySessionId<Impl: ITsSbGlobalStoreImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, providername: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, dwsessionid: u32, targetname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ppsession: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).QuerySessionBySessionId(&*(&providername as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), dwsessionid, &*(&targetname as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&ppsession)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnumerateFarms<Impl: ITsSbGlobalStoreImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, providername: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pdwcount: *mut u32, pval: *mut *mut super::Com::SAFEARRAY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EnumerateFarms(&*(&providername as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&pdwcount), ::core::mem::transmute_copy(&pval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnumerateTargets<Impl: ITsSbGlobalStoreImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, providername: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, farmname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, envname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pdwcount: *mut u32, pval: *mut *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EnumerateTargets(
                &*(&providername as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&farmname as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&envname as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType),
                pdwcount,
                ::core::mem::transmute_copy(&pval),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnumerateEnvironmentsByProvider<Impl: ITsSbGlobalStoreImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, providername: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pdwcount: *mut u32, ppval: *mut *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EnumerateEnvironmentsByProvider(&*(&providername as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), pdwcount, ::core::mem::transmute_copy(&ppval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnumerateSessions<Impl: ITsSbGlobalStoreImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, providername: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, targetname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, username: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, userdomain: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, poolname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, initialprogram: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, psessionstate: *const TSSESSION_STATE, pdwcount: *mut u32, ppval: *mut *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EnumerateSessions(
                &*(&providername as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&targetname as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&username as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&userdomain as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&poolname as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&initialprogram as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType),
                psessionstate,
                pdwcount,
                ::core::mem::transmute_copy(&ppval),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFarmProperty<Impl: ITsSbGlobalStoreImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, farmname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, propertyname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pvarvalue: *const super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetFarmProperty(
                &*(&farmname as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&propertyname as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&pvarvalue as *const <super::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::Com::VARIANT as ::windows::core::DefaultType>::DefaultType),
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
            ::windows::core::GetRuntimeClassName::<ITsSbGlobalStore>,
            ::windows::core::GetTrustLevel,
            QueryTarget::<Impl, OFFSET>,
            QuerySessionBySessionId::<Impl, OFFSET>,
            EnumerateFarms::<Impl, OFFSET>,
            EnumerateTargets::<Impl, OFFSET>,
            EnumerateEnvironmentsByProvider::<Impl, OFFSET>,
            EnumerateSessions::<Impl, OFFSET>,
            GetFarmProperty::<Impl, OFFSET>,
        )
    }
}
pub trait ITsSbLoadBalanceResultImpl: Sized {
    fn TargetName();
}
impl ::windows::core::RuntimeName for ITsSbLoadBalanceResult {
    const NAME: &'static str = "Windows.Win32.System.RemoteDesktop.ITsSbLoadBalanceResult";
}
impl ITsSbLoadBalanceResultVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITsSbLoadBalanceResultImpl, const OFFSET: isize>() -> ITsSbLoadBalanceResultVtbl {
        unsafe extern "system" fn TargetName<Impl: ITsSbLoadBalanceResultImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TargetName(::core::mem::transmute_copy(&pval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ITsSbLoadBalanceResult>, ::windows::core::GetTrustLevel, TargetName::<Impl, OFFSET>)
    }
}
pub trait ITsSbLoadBalancingImpl: Sized + ITsSbPluginImpl {
    fn GetMostSuitableTarget();
}
impl ::windows::core::RuntimeName for ITsSbLoadBalancing {
    const NAME: &'static str = "Windows.Win32.System.RemoteDesktop.ITsSbLoadBalancing";
}
impl ITsSbLoadBalancingVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITsSbLoadBalancingImpl, const OFFSET: isize>() -> ITsSbLoadBalancingVtbl {
        unsafe extern "system" fn GetMostSuitableTarget<Impl: ITsSbLoadBalancingImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pconnection: ::windows::core::RawPtr, plbsink: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetMostSuitableTarget(&*(&pconnection as *const <ITsSbClientConnection as ::windows::core::Abi>::Abi as *const <ITsSbClientConnection as ::windows::core::DefaultType>::DefaultType), &*(&plbsink as *const <ITsSbLoadBalancingNotifySink as ::windows::core::Abi>::Abi as *const <ITsSbLoadBalancingNotifySink as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ITsSbLoadBalancing>, ::windows::core::GetTrustLevel, GetMostSuitableTarget::<Impl, OFFSET>)
    }
}
pub trait ITsSbLoadBalancingNotifySinkImpl: Sized + ITsSbBaseNotifySinkImpl {
    fn OnGetMostSuitableTarget();
}
impl ::windows::core::RuntimeName for ITsSbLoadBalancingNotifySink {
    const NAME: &'static str = "Windows.Win32.System.RemoteDesktop.ITsSbLoadBalancingNotifySink";
}
impl ITsSbLoadBalancingNotifySinkVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITsSbLoadBalancingNotifySinkImpl, const OFFSET: isize>() -> ITsSbLoadBalancingNotifySinkVtbl {
        unsafe extern "system" fn OnGetMostSuitableTarget<Impl: ITsSbLoadBalancingNotifySinkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plbresult: ::windows::core::RawPtr, fisnewconnection: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OnGetMostSuitableTarget(&*(&plbresult as *const <ITsSbLoadBalanceResult as ::windows::core::Abi>::Abi as *const <ITsSbLoadBalanceResult as ::windows::core::DefaultType>::DefaultType), &*(&fisnewconnection as *const <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ITsSbLoadBalancingNotifySink>, ::windows::core::GetTrustLevel, OnGetMostSuitableTarget::<Impl, OFFSET>)
    }
}
pub trait ITsSbOrchestrationImpl: Sized + ITsSbPluginImpl {
    fn PrepareTargetForConnect();
}
impl ::windows::core::RuntimeName for ITsSbOrchestration {
    const NAME: &'static str = "Windows.Win32.System.RemoteDesktop.ITsSbOrchestration";
}
impl ITsSbOrchestrationVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITsSbOrchestrationImpl, const OFFSET: isize>() -> ITsSbOrchestrationVtbl {
        unsafe extern "system" fn PrepareTargetForConnect<Impl: ITsSbOrchestrationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pconnection: ::windows::core::RawPtr, porchestrationnotifysink: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PrepareTargetForConnect(&*(&pconnection as *const <ITsSbClientConnection as ::windows::core::Abi>::Abi as *const <ITsSbClientConnection as ::windows::core::DefaultType>::DefaultType), &*(&porchestrationnotifysink as *const <ITsSbOrchestrationNotifySink as ::windows::core::Abi>::Abi as *const <ITsSbOrchestrationNotifySink as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ITsSbOrchestration>, ::windows::core::GetTrustLevel, PrepareTargetForConnect::<Impl, OFFSET>)
    }
}
pub trait ITsSbOrchestrationNotifySinkImpl: Sized + ITsSbBaseNotifySinkImpl {
    fn OnReadyToConnect();
}
impl ::windows::core::RuntimeName for ITsSbOrchestrationNotifySink {
    const NAME: &'static str = "Windows.Win32.System.RemoteDesktop.ITsSbOrchestrationNotifySink";
}
impl ITsSbOrchestrationNotifySinkVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITsSbOrchestrationNotifySinkImpl, const OFFSET: isize>() -> ITsSbOrchestrationNotifySinkVtbl {
        unsafe extern "system" fn OnReadyToConnect<Impl: ITsSbOrchestrationNotifySinkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ptarget: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OnReadyToConnect(&*(&ptarget as *const <ITsSbTarget as ::windows::core::Abi>::Abi as *const <ITsSbTarget as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ITsSbOrchestrationNotifySink>, ::windows::core::GetTrustLevel, OnReadyToConnect::<Impl, OFFSET>)
    }
}
pub trait ITsSbPlacementImpl: Sized + ITsSbPluginImpl {
    fn QueryEnvironmentForTarget();
}
impl ::windows::core::RuntimeName for ITsSbPlacement {
    const NAME: &'static str = "Windows.Win32.System.RemoteDesktop.ITsSbPlacement";
}
impl ITsSbPlacementVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITsSbPlacementImpl, const OFFSET: isize>() -> ITsSbPlacementVtbl {
        unsafe extern "system" fn QueryEnvironmentForTarget<Impl: ITsSbPlacementImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pconnection: ::windows::core::RawPtr, pplacementsink: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).QueryEnvironmentForTarget(&*(&pconnection as *const <ITsSbClientConnection as ::windows::core::Abi>::Abi as *const <ITsSbClientConnection as ::windows::core::DefaultType>::DefaultType), &*(&pplacementsink as *const <ITsSbPlacementNotifySink as ::windows::core::Abi>::Abi as *const <ITsSbPlacementNotifySink as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ITsSbPlacement>, ::windows::core::GetTrustLevel, QueryEnvironmentForTarget::<Impl, OFFSET>)
    }
}
pub trait ITsSbPlacementNotifySinkImpl: Sized + ITsSbBaseNotifySinkImpl {
    fn OnQueryEnvironmentCompleted();
}
impl ::windows::core::RuntimeName for ITsSbPlacementNotifySink {
    const NAME: &'static str = "Windows.Win32.System.RemoteDesktop.ITsSbPlacementNotifySink";
}
impl ITsSbPlacementNotifySinkVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITsSbPlacementNotifySinkImpl, const OFFSET: isize>() -> ITsSbPlacementNotifySinkVtbl {
        unsafe extern "system" fn OnQueryEnvironmentCompleted<Impl: ITsSbPlacementNotifySinkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, penvironment: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OnQueryEnvironmentCompleted(&*(&penvironment as *const <ITsSbEnvironment as ::windows::core::Abi>::Abi as *const <ITsSbEnvironment as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ITsSbPlacementNotifySink>, ::windows::core::GetTrustLevel, OnQueryEnvironmentCompleted::<Impl, OFFSET>)
    }
}
pub trait ITsSbPluginImpl: Sized {
    fn Initialize();
    fn Terminate();
}
impl ::windows::core::RuntimeName for ITsSbPlugin {
    const NAME: &'static str = "Windows.Win32.System.RemoteDesktop.ITsSbPlugin";
}
impl ITsSbPluginVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITsSbPluginImpl, const OFFSET: isize>() -> ITsSbPluginVtbl {
        unsafe extern "system" fn Initialize<Impl: ITsSbPluginImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pprovider: ::windows::core::RawPtr, pnotifysink: ::windows::core::RawPtr, ppropertyset: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Initialize(&*(&pprovider as *const <ITsSbProvider as ::windows::core::Abi>::Abi as *const <ITsSbProvider as ::windows::core::DefaultType>::DefaultType), &*(&pnotifysink as *const <ITsSbPluginNotifySink as ::windows::core::Abi>::Abi as *const <ITsSbPluginNotifySink as ::windows::core::DefaultType>::DefaultType), &*(&ppropertyset as *const <ITsSbPluginPropertySet as ::windows::core::Abi>::Abi as *const <ITsSbPluginPropertySet as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Terminate<Impl: ITsSbPluginImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hr: ::windows::core::HRESULT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Terminate(hr) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ITsSbPlugin>, ::windows::core::GetTrustLevel, Initialize::<Impl, OFFSET>, Terminate::<Impl, OFFSET>)
    }
}
pub trait ITsSbPluginNotifySinkImpl: Sized + ITsSbBaseNotifySinkImpl {
    fn OnInitialized();
    fn OnTerminated();
}
impl ::windows::core::RuntimeName for ITsSbPluginNotifySink {
    const NAME: &'static str = "Windows.Win32.System.RemoteDesktop.ITsSbPluginNotifySink";
}
impl ITsSbPluginNotifySinkVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITsSbPluginNotifySinkImpl, const OFFSET: isize>() -> ITsSbPluginNotifySinkVtbl {
        unsafe extern "system" fn OnInitialized<Impl: ITsSbPluginNotifySinkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hr: ::windows::core::HRESULT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OnInitialized(hr) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OnTerminated<Impl: ITsSbPluginNotifySinkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OnTerminated() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ITsSbPluginNotifySink>, ::windows::core::GetTrustLevel, OnInitialized::<Impl, OFFSET>, OnTerminated::<Impl, OFFSET>)
    }
}
#[cfg(feature = "Win32_System_Com_StructuredStorage")]
pub trait ITsSbPluginPropertySetImpl: Sized + ITsSbPropertySetImpl + IPropertyBagImpl {}
#[cfg(feature = "Win32_System_Com_StructuredStorage")]
impl ::windows::core::RuntimeName for ITsSbPluginPropertySet {
    const NAME: &'static str = "Windows.Win32.System.RemoteDesktop.ITsSbPluginPropertySet";
}
#[cfg(feature = "Win32_System_Com_StructuredStorage")]
impl ITsSbPluginPropertySetVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITsSbPluginPropertySetImpl, const OFFSET: isize>() -> ITsSbPluginPropertySetVtbl {
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ITsSbPluginPropertySet>, ::windows::core::GetTrustLevel)
    }
}
#[cfg(feature = "Win32_System_Com_StructuredStorage")]
pub trait ITsSbPropertySetImpl: Sized + IPropertyBagImpl {}
#[cfg(feature = "Win32_System_Com_StructuredStorage")]
impl ::windows::core::RuntimeName for ITsSbPropertySet {
    const NAME: &'static str = "Windows.Win32.System.RemoteDesktop.ITsSbPropertySet";
}
#[cfg(feature = "Win32_System_Com_StructuredStorage")]
impl ITsSbPropertySetVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITsSbPropertySetImpl, const OFFSET: isize>() -> ITsSbPropertySetVtbl {
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ITsSbPropertySet>, ::windows::core::GetTrustLevel)
    }
}
pub trait ITsSbProviderImpl: Sized {
    fn CreateTargetObject();
    fn CreateLoadBalanceResultObject();
    fn CreateSessionObject();
    fn CreatePluginPropertySet();
    fn CreateTargetPropertySetObject();
    fn CreateEnvironmentObject();
    fn GetResourcePluginStore();
    fn GetFilterPluginStore();
    fn RegisterForNotification();
    fn UnRegisterForNotification();
    fn GetInstanceOfGlobalStore();
    fn CreateEnvironmentPropertySetObject();
}
impl ::windows::core::RuntimeName for ITsSbProvider {
    const NAME: &'static str = "Windows.Win32.System.RemoteDesktop.ITsSbProvider";
}
impl ITsSbProviderVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITsSbProviderImpl, const OFFSET: isize>() -> ITsSbProviderVtbl {
        unsafe extern "system" fn CreateTargetObject<Impl: ITsSbProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, targetname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, environmentname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pptarget: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateTargetObject(&*(&targetname as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), &*(&environmentname as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&pptarget)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateLoadBalanceResultObject<Impl: ITsSbProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, targetname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pplbresult: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateLoadBalanceResultObject(&*(&targetname as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&pplbresult)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateSessionObject<Impl: ITsSbProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, targetname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, username: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, domain: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, sessionid: u32, ppsession: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateSessionObject(
                &*(&targetname as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&username as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&domain as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType),
                sessionid,
                ::core::mem::transmute_copy(&ppsession),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreatePluginPropertySet<Impl: ITsSbProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pppropertyset: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreatePluginPropertySet(::core::mem::transmute_copy(&pppropertyset)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateTargetPropertySetObject<Impl: ITsSbProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pppropertyset: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateTargetPropertySetObject(::core::mem::transmute_copy(&pppropertyset)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateEnvironmentObject<Impl: ITsSbProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, serverweight: u32, ppenvironment: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateEnvironmentObject(&*(&name as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), serverweight, ::core::mem::transmute_copy(&ppenvironment)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetResourcePluginStore<Impl: ITsSbProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppstore: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetResourcePluginStore(::core::mem::transmute_copy(&ppstore)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFilterPluginStore<Impl: ITsSbProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppstore: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetFilterPluginStore(::core::mem::transmute_copy(&ppstore)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RegisterForNotification<Impl: ITsSbProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, notificationtype: u32, resourcetomonitor: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ppluginnotification: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RegisterForNotification(notificationtype, &*(&resourcetomonitor as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), &*(&ppluginnotification as *const <ITsSbResourceNotification as ::windows::core::Abi>::Abi as *const <ITsSbResourceNotification as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UnRegisterForNotification<Impl: ITsSbProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, notificationtype: u32, resourcetomonitor: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).UnRegisterForNotification(notificationtype, &*(&resourcetomonitor as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetInstanceOfGlobalStore<Impl: ITsSbProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppglobalstore: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetInstanceOfGlobalStore(::core::mem::transmute_copy(&ppglobalstore)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateEnvironmentPropertySetObject<Impl: ITsSbProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pppropertyset: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateEnvironmentPropertySetObject(::core::mem::transmute_copy(&pppropertyset)) {
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
            ::windows::core::GetRuntimeClassName::<ITsSbProvider>,
            ::windows::core::GetTrustLevel,
            CreateTargetObject::<Impl, OFFSET>,
            CreateLoadBalanceResultObject::<Impl, OFFSET>,
            CreateSessionObject::<Impl, OFFSET>,
            CreatePluginPropertySet::<Impl, OFFSET>,
            CreateTargetPropertySetObject::<Impl, OFFSET>,
            CreateEnvironmentObject::<Impl, OFFSET>,
            GetResourcePluginStore::<Impl, OFFSET>,
            GetFilterPluginStore::<Impl, OFFSET>,
            RegisterForNotification::<Impl, OFFSET>,
            UnRegisterForNotification::<Impl, OFFSET>,
            GetInstanceOfGlobalStore::<Impl, OFFSET>,
            CreateEnvironmentPropertySetObject::<Impl, OFFSET>,
        )
    }
}
pub trait ITsSbProvisioningImpl: Sized + ITsSbPluginImpl {
    fn CreateVirtualMachines();
    fn PatchVirtualMachines();
    fn DeleteVirtualMachines();
    fn CancelJob();
}
impl ::windows::core::RuntimeName for ITsSbProvisioning {
    const NAME: &'static str = "Windows.Win32.System.RemoteDesktop.ITsSbProvisioning";
}
impl ITsSbProvisioningVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITsSbProvisioningImpl, const OFFSET: isize>() -> ITsSbProvisioningVtbl {
        unsafe extern "system" fn CreateVirtualMachines<Impl: ITsSbProvisioningImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, jobxmlstring: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, jobguid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, psink: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateVirtualMachines(
                &*(&jobxmlstring as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&jobguid as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&psink as *const <ITsSbProvisioningPluginNotifySink as ::windows::core::Abi>::Abi as *const <ITsSbProvisioningPluginNotifySink as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PatchVirtualMachines<Impl: ITsSbProvisioningImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, jobxmlstring: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, jobguid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, psink: ::windows::core::RawPtr, pvmpatchinfo: *const VM_PATCH_INFO) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PatchVirtualMachines(
                &*(&jobxmlstring as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&jobguid as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&psink as *const <ITsSbProvisioningPluginNotifySink as ::windows::core::Abi>::Abi as *const <ITsSbProvisioningPluginNotifySink as ::windows::core::DefaultType>::DefaultType),
                &*(&pvmpatchinfo as *const <VM_PATCH_INFO as ::windows::core::Abi>::Abi as *const <VM_PATCH_INFO as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DeleteVirtualMachines<Impl: ITsSbProvisioningImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, jobxmlstring: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, jobguid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, psink: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DeleteVirtualMachines(
                &*(&jobxmlstring as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&jobguid as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&psink as *const <ITsSbProvisioningPluginNotifySink as ::windows::core::Abi>::Abi as *const <ITsSbProvisioningPluginNotifySink as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CancelJob<Impl: ITsSbProvisioningImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, jobguid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CancelJob(&*(&jobguid as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ITsSbProvisioning>, ::windows::core::GetTrustLevel, CreateVirtualMachines::<Impl, OFFSET>, PatchVirtualMachines::<Impl, OFFSET>, DeleteVirtualMachines::<Impl, OFFSET>, CancelJob::<Impl, OFFSET>)
    }
}
pub trait ITsSbProvisioningPluginNotifySinkImpl: Sized {
    fn OnJobCreated();
    fn OnVirtualMachineStatusChanged();
    fn OnJobCompleted();
    fn OnJobCancelled();
    fn LockVirtualMachine();
    fn OnVirtualMachineHostStatusChanged();
}
impl ::windows::core::RuntimeName for ITsSbProvisioningPluginNotifySink {
    const NAME: &'static str = "Windows.Win32.System.RemoteDesktop.ITsSbProvisioningPluginNotifySink";
}
impl ITsSbProvisioningPluginNotifySinkVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITsSbProvisioningPluginNotifySinkImpl, const OFFSET: isize>() -> ITsSbProvisioningPluginNotifySinkVtbl {
        unsafe extern "system" fn OnJobCreated<Impl: ITsSbProvisioningPluginNotifySinkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvmnotifyinfo: *const VM_NOTIFY_INFO) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OnJobCreated(&*(&pvmnotifyinfo as *const <VM_NOTIFY_INFO as ::windows::core::Abi>::Abi as *const <VM_NOTIFY_INFO as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OnVirtualMachineStatusChanged<Impl: ITsSbProvisioningPluginNotifySinkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvmnotifyentry: *const VM_NOTIFY_ENTRY, vmnotifystatus: VM_NOTIFY_STATUS, errorcode: ::windows::core::HRESULT, errordescr: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OnVirtualMachineStatusChanged(&*(&pvmnotifyentry as *const <VM_NOTIFY_ENTRY as ::windows::core::Abi>::Abi as *const <VM_NOTIFY_ENTRY as ::windows::core::DefaultType>::DefaultType), vmnotifystatus, errorcode, &*(&errordescr as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OnJobCompleted<Impl: ITsSbProvisioningPluginNotifySinkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, resultcode: ::windows::core::HRESULT, resultdescription: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OnJobCompleted(resultcode, &*(&resultdescription as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OnJobCancelled<Impl: ITsSbProvisioningPluginNotifySinkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OnJobCancelled() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LockVirtualMachine<Impl: ITsSbProvisioningPluginNotifySinkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvmnotifyentry: *const VM_NOTIFY_ENTRY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LockVirtualMachine(&*(&pvmnotifyentry as *const <VM_NOTIFY_ENTRY as ::windows::core::Abi>::Abi as *const <VM_NOTIFY_ENTRY as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OnVirtualMachineHostStatusChanged<Impl: ITsSbProvisioningPluginNotifySinkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, vmhost: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, vmhostnotifystatus: VM_HOST_NOTIFY_STATUS, errorcode: ::windows::core::HRESULT, errordescr: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OnVirtualMachineHostStatusChanged(&*(&vmhost as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), vmhostnotifystatus, errorcode, &*(&errordescr as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
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
            ::windows::core::GetRuntimeClassName::<ITsSbProvisioningPluginNotifySink>,
            ::windows::core::GetTrustLevel,
            OnJobCreated::<Impl, OFFSET>,
            OnVirtualMachineStatusChanged::<Impl, OFFSET>,
            OnJobCompleted::<Impl, OFFSET>,
            OnJobCancelled::<Impl, OFFSET>,
            LockVirtualMachine::<Impl, OFFSET>,
            OnVirtualMachineHostStatusChanged::<Impl, OFFSET>,
        )
    }
}
pub trait ITsSbResourceNotificationImpl: Sized {
    fn NotifySessionChange();
    fn NotifyTargetChange();
    fn NotifyClientConnectionStateChange();
}
impl ::windows::core::RuntimeName for ITsSbResourceNotification {
    const NAME: &'static str = "Windows.Win32.System.RemoteDesktop.ITsSbResourceNotification";
}
impl ITsSbResourceNotificationVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITsSbResourceNotificationImpl, const OFFSET: isize>() -> ITsSbResourceNotificationVtbl {
        unsafe extern "system" fn NotifySessionChange<Impl: ITsSbResourceNotificationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, changetype: TSSESSION_STATE, psession: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).NotifySessionChange(changetype, &*(&psession as *const <ITsSbSession as ::windows::core::Abi>::Abi as *const <ITsSbSession as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn NotifyTargetChange<Impl: ITsSbResourceNotificationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, targetchangetype: u32, ptarget: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).NotifyTargetChange(targetchangetype, &*(&ptarget as *const <ITsSbTarget as ::windows::core::Abi>::Abi as *const <ITsSbTarget as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn NotifyClientConnectionStateChange<Impl: ITsSbResourceNotificationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, changetype: CONNECTION_CHANGE_NOTIFICATION, pconnection: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).NotifyClientConnectionStateChange(changetype, &*(&pconnection as *const <ITsSbClientConnection as ::windows::core::Abi>::Abi as *const <ITsSbClientConnection as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ITsSbResourceNotification>, ::windows::core::GetTrustLevel, NotifySessionChange::<Impl, OFFSET>, NotifyTargetChange::<Impl, OFFSET>, NotifyClientConnectionStateChange::<Impl, OFFSET>)
    }
}
pub trait ITsSbResourceNotificationExImpl: Sized {
    fn NotifySessionChangeEx();
    fn NotifyTargetChangeEx();
    fn NotifyClientConnectionStateChangeEx();
}
impl ::windows::core::RuntimeName for ITsSbResourceNotificationEx {
    const NAME: &'static str = "Windows.Win32.System.RemoteDesktop.ITsSbResourceNotificationEx";
}
impl ITsSbResourceNotificationExVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITsSbResourceNotificationExImpl, const OFFSET: isize>() -> ITsSbResourceNotificationExVtbl {
        unsafe extern "system" fn NotifySessionChangeEx<Impl: ITsSbResourceNotificationExImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, targetname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, username: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, domain: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, sessionid: u32, sessionstate: TSSESSION_STATE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).NotifySessionChangeEx(
                &*(&targetname as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&username as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&domain as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType),
                sessionid,
                sessionstate,
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn NotifyTargetChangeEx<Impl: ITsSbResourceNotificationExImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, targetname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, targetchangetype: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).NotifyTargetChangeEx(&*(&targetname as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), targetchangetype) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn NotifyClientConnectionStateChangeEx<Impl: ITsSbResourceNotificationExImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, username: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, domain: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, initialprogram: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, poolname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, targetname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, connectionchangetype: CONNECTION_CHANGE_NOTIFICATION) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).NotifyClientConnectionStateChangeEx(
                &*(&username as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&domain as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&initialprogram as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&poolname as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&targetname as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType),
                connectionchangetype,
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ITsSbResourceNotificationEx>, ::windows::core::GetTrustLevel, NotifySessionChangeEx::<Impl, OFFSET>, NotifyTargetChangeEx::<Impl, OFFSET>, NotifyClientConnectionStateChangeEx::<Impl, OFFSET>)
    }
}
pub trait ITsSbResourcePluginImpl: Sized + ITsSbPluginImpl {}
impl ::windows::core::RuntimeName for ITsSbResourcePlugin {
    const NAME: &'static str = "Windows.Win32.System.RemoteDesktop.ITsSbResourcePlugin";
}
impl ITsSbResourcePluginVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITsSbResourcePluginImpl, const OFFSET: isize>() -> ITsSbResourcePluginVtbl {
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ITsSbResourcePlugin>, ::windows::core::GetTrustLevel)
    }
}
pub trait ITsSbResourcePluginStoreImpl: Sized {
    fn QueryTarget();
    fn QuerySessionBySessionId();
    fn AddTargetToStore();
    fn AddSessionToStore();
    fn AddEnvironmentToStore();
    fn RemoveEnvironmentFromStore();
    fn EnumerateFarms();
    fn QueryEnvironment();
    fn EnumerateEnvironments();
    fn SaveTarget();
    fn SaveEnvironment();
    fn SaveSession();
    fn SetTargetProperty();
    fn SetEnvironmentProperty();
    fn SetTargetState();
    fn SetSessionState();
    fn EnumerateTargets();
    fn EnumerateSessions();
    fn GetFarmProperty();
    fn DeleteTarget();
    fn SetTargetPropertyWithVersionCheck();
    fn SetEnvironmentPropertyWithVersionCheck();
    fn AcquireTargetLock();
    fn ReleaseTargetLock();
    fn TestAndSetServerState();
    fn SetServerWaitingToStart();
    fn GetServerState();
    fn SetServerDrainMode();
}
impl ::windows::core::RuntimeName for ITsSbResourcePluginStore {
    const NAME: &'static str = "Windows.Win32.System.RemoteDesktop.ITsSbResourcePluginStore";
}
impl ITsSbResourcePluginStoreVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITsSbResourcePluginStoreImpl, const OFFSET: isize>() -> ITsSbResourcePluginStoreVtbl {
        unsafe extern "system" fn QueryTarget<Impl: ITsSbResourcePluginStoreImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, targetname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, farmname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pptarget: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).QueryTarget(&*(&targetname as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), &*(&farmname as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&pptarget)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn QuerySessionBySessionId<Impl: ITsSbResourcePluginStoreImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwsessionid: u32, targetname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ppsession: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).QuerySessionBySessionId(dwsessionid, &*(&targetname as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&ppsession)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AddTargetToStore<Impl: ITsSbResourcePluginStoreImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ptarget: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AddTargetToStore(&*(&ptarget as *const <ITsSbTarget as ::windows::core::Abi>::Abi as *const <ITsSbTarget as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AddSessionToStore<Impl: ITsSbResourcePluginStoreImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psession: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AddSessionToStore(&*(&psession as *const <ITsSbSession as ::windows::core::Abi>::Abi as *const <ITsSbSession as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AddEnvironmentToStore<Impl: ITsSbResourcePluginStoreImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, penvironment: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AddEnvironmentToStore(&*(&penvironment as *const <ITsSbEnvironment as ::windows::core::Abi>::Abi as *const <ITsSbEnvironment as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveEnvironmentFromStore<Impl: ITsSbResourcePluginStoreImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, environmentname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bignoreowner: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RemoveEnvironmentFromStore(&*(&environmentname as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), &*(&bignoreowner as *const <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnumerateFarms<Impl: ITsSbResourcePluginStoreImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwcount: *mut u32, pval: *mut *mut super::Com::SAFEARRAY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EnumerateFarms(::core::mem::transmute_copy(&pdwcount), ::core::mem::transmute_copy(&pval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn QueryEnvironment<Impl: ITsSbResourcePluginStoreImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, environmentname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ppenvironment: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).QueryEnvironment(&*(&environmentname as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&ppenvironment)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnumerateEnvironments<Impl: ITsSbResourcePluginStoreImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwcount: *mut u32, pval: *mut *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EnumerateEnvironments(pdwcount, ::core::mem::transmute_copy(&pval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SaveTarget<Impl: ITsSbResourcePluginStoreImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ptarget: ::windows::core::RawPtr, bforcewrite: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SaveTarget(&*(&ptarget as *const <ITsSbTarget as ::windows::core::Abi>::Abi as *const <ITsSbTarget as ::windows::core::DefaultType>::DefaultType), &*(&bforcewrite as *const <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SaveEnvironment<Impl: ITsSbResourcePluginStoreImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, penvironment: ::windows::core::RawPtr, bforcewrite: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SaveEnvironment(&*(&penvironment as *const <ITsSbEnvironment as ::windows::core::Abi>::Abi as *const <ITsSbEnvironment as ::windows::core::DefaultType>::DefaultType), &*(&bforcewrite as *const <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SaveSession<Impl: ITsSbResourcePluginStoreImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psession: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SaveSession(&*(&psession as *const <ITsSbSession as ::windows::core::Abi>::Abi as *const <ITsSbSession as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetTargetProperty<Impl: ITsSbResourcePluginStoreImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, targetname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, propertyname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pproperty: *const super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetTargetProperty(
                &*(&targetname as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&propertyname as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&pproperty as *const <super::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::Com::VARIANT as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetEnvironmentProperty<Impl: ITsSbResourcePluginStoreImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, environmentname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, propertyname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pproperty: *const super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetEnvironmentProperty(
                &*(&environmentname as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&propertyname as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&pproperty as *const <super::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::Com::VARIANT as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetTargetState<Impl: ITsSbResourcePluginStoreImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, targetname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, newstate: TARGET_STATE, poldstate: *mut TARGET_STATE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetTargetState(&*(&targetname as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), newstate, ::core::mem::transmute_copy(&poldstate)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSessionState<Impl: ITsSbResourcePluginStoreImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, sbsession: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetSessionState(&*(&sbsession as *const <ITsSbSession as ::windows::core::Abi>::Abi as *const <ITsSbSession as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnumerateTargets<Impl: ITsSbResourcePluginStoreImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, farmname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, envname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, sortbyfieldid: TS_SB_SORT_BY, sortybypropname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pdwcount: *mut u32, pval: *mut *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EnumerateTargets(
                &*(&farmname as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&envname as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType),
                sortbyfieldid,
                &*(&sortybypropname as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType),
                pdwcount,
                ::core::mem::transmute_copy(&pval),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnumerateSessions<Impl: ITsSbResourcePluginStoreImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, targetname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, username: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, userdomain: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, poolname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, initialprogram: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, psessionstate: *const TSSESSION_STATE, pdwcount: *mut u32, ppval: *mut *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EnumerateSessions(
                &*(&targetname as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&username as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&userdomain as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&poolname as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&initialprogram as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType),
                psessionstate,
                pdwcount,
                ::core::mem::transmute_copy(&ppval),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFarmProperty<Impl: ITsSbResourcePluginStoreImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, farmname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, propertyname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pvarvalue: *const super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetFarmProperty(
                &*(&farmname as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&propertyname as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&pvarvalue as *const <super::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::Com::VARIANT as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DeleteTarget<Impl: ITsSbResourcePluginStoreImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, targetname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, hostname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DeleteTarget(&*(&targetname as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), &*(&hostname as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetTargetPropertyWithVersionCheck<Impl: ITsSbResourcePluginStoreImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ptarget: ::windows::core::RawPtr, propertyname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pproperty: *const super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetTargetPropertyWithVersionCheck(&*(&ptarget as *const <ITsSbTarget as ::windows::core::Abi>::Abi as *const <ITsSbTarget as ::windows::core::DefaultType>::DefaultType), &*(&propertyname as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), &*(&pproperty as *const <super::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::Com::VARIANT as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetEnvironmentPropertyWithVersionCheck<Impl: ITsSbResourcePluginStoreImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, penvironment: ::windows::core::RawPtr, propertyname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pproperty: *const super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetEnvironmentPropertyWithVersionCheck(
                &*(&penvironment as *const <ITsSbEnvironment as ::windows::core::Abi>::Abi as *const <ITsSbEnvironment as ::windows::core::DefaultType>::DefaultType),
                &*(&propertyname as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&pproperty as *const <super::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::Com::VARIANT as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AcquireTargetLock<Impl: ITsSbResourcePluginStoreImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, targetname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, dwtimeout: u32, ppcontext: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AcquireTargetLock(&*(&targetname as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), dwtimeout, ::core::mem::transmute_copy(&ppcontext)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ReleaseTargetLock<Impl: ITsSbResourcePluginStoreImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcontext: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ReleaseTargetLock(&*(&pcontext as *const <::windows::core::IUnknown as ::windows::core::Abi>::Abi as *const <::windows::core::IUnknown as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TestAndSetServerState<Impl: ITsSbResourcePluginStoreImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, poolname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, serverfqdn: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, newstate: TARGET_STATE, teststate: TARGET_STATE, pinitstate: *mut TARGET_STATE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TestAndSetServerState(&*(&poolname as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), &*(&serverfqdn as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), newstate, teststate, ::core::mem::transmute_copy(&pinitstate)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetServerWaitingToStart<Impl: ITsSbResourcePluginStoreImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, poolname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, servername: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetServerWaitingToStart(&*(&poolname as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), &*(&servername as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetServerState<Impl: ITsSbResourcePluginStoreImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, poolname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, serverfqdn: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pstate: *mut TARGET_STATE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetServerState(&*(&poolname as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), &*(&serverfqdn as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&pstate)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetServerDrainMode<Impl: ITsSbResourcePluginStoreImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, serverfqdn: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, drainmode: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetServerDrainMode(&*(&serverfqdn as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), drainmode) {
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
            ::windows::core::GetRuntimeClassName::<ITsSbResourcePluginStore>,
            ::windows::core::GetTrustLevel,
            QueryTarget::<Impl, OFFSET>,
            QuerySessionBySessionId::<Impl, OFFSET>,
            AddTargetToStore::<Impl, OFFSET>,
            AddSessionToStore::<Impl, OFFSET>,
            AddEnvironmentToStore::<Impl, OFFSET>,
            RemoveEnvironmentFromStore::<Impl, OFFSET>,
            EnumerateFarms::<Impl, OFFSET>,
            QueryEnvironment::<Impl, OFFSET>,
            EnumerateEnvironments::<Impl, OFFSET>,
            SaveTarget::<Impl, OFFSET>,
            SaveEnvironment::<Impl, OFFSET>,
            SaveSession::<Impl, OFFSET>,
            SetTargetProperty::<Impl, OFFSET>,
            SetEnvironmentProperty::<Impl, OFFSET>,
            SetTargetState::<Impl, OFFSET>,
            SetSessionState::<Impl, OFFSET>,
            EnumerateTargets::<Impl, OFFSET>,
            EnumerateSessions::<Impl, OFFSET>,
            GetFarmProperty::<Impl, OFFSET>,
            DeleteTarget::<Impl, OFFSET>,
            SetTargetPropertyWithVersionCheck::<Impl, OFFSET>,
            SetEnvironmentPropertyWithVersionCheck::<Impl, OFFSET>,
            AcquireTargetLock::<Impl, OFFSET>,
            ReleaseTargetLock::<Impl, OFFSET>,
            TestAndSetServerState::<Impl, OFFSET>,
            SetServerWaitingToStart::<Impl, OFFSET>,
            GetServerState::<Impl, OFFSET>,
            SetServerDrainMode::<Impl, OFFSET>,
        )
    }
}
pub trait ITsSbServiceNotificationImpl: Sized {
    fn NotifyServiceFailure();
    fn NotifyServiceSuccess();
}
impl ::windows::core::RuntimeName for ITsSbServiceNotification {
    const NAME: &'static str = "Windows.Win32.System.RemoteDesktop.ITsSbServiceNotification";
}
impl ITsSbServiceNotificationVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITsSbServiceNotificationImpl, const OFFSET: isize>() -> ITsSbServiceNotificationVtbl {
        unsafe extern "system" fn NotifyServiceFailure<Impl: ITsSbServiceNotificationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).NotifyServiceFailure() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn NotifyServiceSuccess<Impl: ITsSbServiceNotificationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).NotifyServiceSuccess() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ITsSbServiceNotification>, ::windows::core::GetTrustLevel, NotifyServiceFailure::<Impl, OFFSET>, NotifyServiceSuccess::<Impl, OFFSET>)
    }
}
pub trait ITsSbSessionImpl: Sized {
    fn SessionId();
    fn TargetName();
    fn SetTargetName();
    fn Username();
    fn Domain();
    fn State();
    fn SetState();
    fn CreateTime();
    fn SetCreateTime();
    fn DisconnectTime();
    fn SetDisconnectTime();
    fn InitialProgram();
    fn SetInitialProgram();
    fn ClientDisplay();
    fn SetClientDisplay();
    fn ProtocolType();
    fn SetProtocolType();
}
impl ::windows::core::RuntimeName for ITsSbSession {
    const NAME: &'static str = "Windows.Win32.System.RemoteDesktop.ITsSbSession";
}
impl ITsSbSessionVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITsSbSessionImpl, const OFFSET: isize>() -> ITsSbSessionVtbl {
        unsafe extern "system" fn SessionId<Impl: ITsSbSessionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SessionId(::core::mem::transmute_copy(&pval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TargetName<Impl: ITsSbSessionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, targetname: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TargetName(::core::mem::transmute_copy(&targetname)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetTargetName<Impl: ITsSbSessionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, targetname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetTargetName(&*(&targetname as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Username<Impl: ITsSbSessionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, username: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Username(::core::mem::transmute_copy(&username)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Domain<Impl: ITsSbSessionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, domain: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Domain(::core::mem::transmute_copy(&domain)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn State<Impl: ITsSbSessionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pstate: *mut TSSESSION_STATE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).State(::core::mem::transmute_copy(&pstate)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetState<Impl: ITsSbSessionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, state: TSSESSION_STATE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetState(state) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateTime<Impl: ITsSbSessionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ptime: *mut super::super::Foundation::FILETIME) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateTime(::core::mem::transmute_copy(&ptime)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCreateTime<Impl: ITsSbSessionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, time: super::super::Foundation::FILETIME) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetCreateTime(&*(&time as *const <super::super::Foundation::FILETIME as ::windows::core::Abi>::Abi as *const <super::super::Foundation::FILETIME as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DisconnectTime<Impl: ITsSbSessionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ptime: *mut super::super::Foundation::FILETIME) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DisconnectTime(::core::mem::transmute_copy(&ptime)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDisconnectTime<Impl: ITsSbSessionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, time: super::super::Foundation::FILETIME) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetDisconnectTime(&*(&time as *const <super::super::Foundation::FILETIME as ::windows::core::Abi>::Abi as *const <super::super::Foundation::FILETIME as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn InitialProgram<Impl: ITsSbSessionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, app: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).InitialProgram(::core::mem::transmute_copy(&app)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetInitialProgram<Impl: ITsSbSessionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, application: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetInitialProgram(&*(&application as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ClientDisplay<Impl: ITsSbSessionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pclientdisplay: *mut CLIENT_DISPLAY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ClientDisplay(::core::mem::transmute_copy(&pclientdisplay)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetClientDisplay<Impl: ITsSbSessionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pclientdisplay: CLIENT_DISPLAY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetClientDisplay(&*(&pclientdisplay as *const <CLIENT_DISPLAY as ::windows::core::Abi>::Abi as *const <CLIENT_DISPLAY as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ProtocolType<Impl: ITsSbSessionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ProtocolType(::core::mem::transmute_copy(&pval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetProtocolType<Impl: ITsSbSessionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, val: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetProtocolType(val) {
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
            ::windows::core::GetRuntimeClassName::<ITsSbSession>,
            ::windows::core::GetTrustLevel,
            SessionId::<Impl, OFFSET>,
            TargetName::<Impl, OFFSET>,
            SetTargetName::<Impl, OFFSET>,
            Username::<Impl, OFFSET>,
            Domain::<Impl, OFFSET>,
            State::<Impl, OFFSET>,
            SetState::<Impl, OFFSET>,
            CreateTime::<Impl, OFFSET>,
            SetCreateTime::<Impl, OFFSET>,
            DisconnectTime::<Impl, OFFSET>,
            SetDisconnectTime::<Impl, OFFSET>,
            InitialProgram::<Impl, OFFSET>,
            SetInitialProgram::<Impl, OFFSET>,
            ClientDisplay::<Impl, OFFSET>,
            SetClientDisplay::<Impl, OFFSET>,
            ProtocolType::<Impl, OFFSET>,
            SetProtocolType::<Impl, OFFSET>,
        )
    }
}
pub trait ITsSbTargetImpl: Sized {
    fn TargetName();
    fn SetTargetName();
    fn FarmName();
    fn SetFarmName();
    fn TargetFQDN();
    fn SetTargetFQDN();
    fn TargetNetbios();
    fn SetTargetNetbios();
    fn IpAddresses();
    fn SetIpAddresses();
    fn TargetState();
    fn SetTargetState();
    fn TargetPropertySet();
    fn SetTargetPropertySet();
    fn EnvironmentName();
    fn SetEnvironmentName();
    fn NumSessions();
    fn NumPendingConnections();
    fn TargetLoad();
}
impl ::windows::core::RuntimeName for ITsSbTarget {
    const NAME: &'static str = "Windows.Win32.System.RemoteDesktop.ITsSbTarget";
}
impl ITsSbTargetVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITsSbTargetImpl, const OFFSET: isize>() -> ITsSbTargetVtbl {
        unsafe extern "system" fn TargetName<Impl: ITsSbTargetImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TargetName(::core::mem::transmute_copy(&pval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetTargetName<Impl: ITsSbTargetImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, val: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetTargetName(&*(&val as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FarmName<Impl: ITsSbTargetImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FarmName(::core::mem::transmute_copy(&pval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetFarmName<Impl: ITsSbTargetImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, val: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetFarmName(&*(&val as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TargetFQDN<Impl: ITsSbTargetImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, targetfqdnname: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TargetFQDN(::core::mem::transmute_copy(&targetfqdnname)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetTargetFQDN<Impl: ITsSbTargetImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, val: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetTargetFQDN(&*(&val as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TargetNetbios<Impl: ITsSbTargetImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, targetnetbiosname: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TargetNetbios(::core::mem::transmute_copy(&targetnetbiosname)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetTargetNetbios<Impl: ITsSbTargetImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, val: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetTargetNetbios(&*(&val as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IpAddresses<Impl: ITsSbTargetImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, sockaddr: *mut TSSD_ConnectionPoint, numaddresses: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IpAddresses(::core::mem::transmute_copy(&sockaddr), numaddresses) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIpAddresses<Impl: ITsSbTargetImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, sockaddr: *const TSSD_ConnectionPoint, numaddresses: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetIpAddresses(&*(&sockaddr as *const <TSSD_ConnectionPoint as ::windows::core::Abi>::Abi as *const <TSSD_ConnectionPoint as ::windows::core::DefaultType>::DefaultType), numaddresses) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TargetState<Impl: ITsSbTargetImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pstate: *mut TARGET_STATE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TargetState(::core::mem::transmute_copy(&pstate)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetTargetState<Impl: ITsSbTargetImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, state: TARGET_STATE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetTargetState(state) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TargetPropertySet<Impl: ITsSbTargetImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pppropertyset: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TargetPropertySet(::core::mem::transmute_copy(&pppropertyset)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetTargetPropertySet<Impl: ITsSbTargetImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetTargetPropertySet(&*(&pval as *const <ITsSbTargetPropertySet as ::windows::core::Abi>::Abi as *const <ITsSbTargetPropertySet as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnvironmentName<Impl: ITsSbTargetImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EnvironmentName(::core::mem::transmute_copy(&pval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetEnvironmentName<Impl: ITsSbTargetImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, val: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetEnvironmentName(&*(&val as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn NumSessions<Impl: ITsSbTargetImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pnumsessions: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).NumSessions(::core::mem::transmute_copy(&pnumsessions)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn NumPendingConnections<Impl: ITsSbTargetImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pnumpendingconnections: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).NumPendingConnections(::core::mem::transmute_copy(&pnumpendingconnections)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TargetLoad<Impl: ITsSbTargetImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ptargetload: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TargetLoad(::core::mem::transmute_copy(&ptargetload)) {
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
            ::windows::core::GetRuntimeClassName::<ITsSbTarget>,
            ::windows::core::GetTrustLevel,
            TargetName::<Impl, OFFSET>,
            SetTargetName::<Impl, OFFSET>,
            FarmName::<Impl, OFFSET>,
            SetFarmName::<Impl, OFFSET>,
            TargetFQDN::<Impl, OFFSET>,
            SetTargetFQDN::<Impl, OFFSET>,
            TargetNetbios::<Impl, OFFSET>,
            SetTargetNetbios::<Impl, OFFSET>,
            IpAddresses::<Impl, OFFSET>,
            SetIpAddresses::<Impl, OFFSET>,
            TargetState::<Impl, OFFSET>,
            SetTargetState::<Impl, OFFSET>,
            TargetPropertySet::<Impl, OFFSET>,
            SetTargetPropertySet::<Impl, OFFSET>,
            EnvironmentName::<Impl, OFFSET>,
            SetEnvironmentName::<Impl, OFFSET>,
            NumSessions::<Impl, OFFSET>,
            NumPendingConnections::<Impl, OFFSET>,
            TargetLoad::<Impl, OFFSET>,
        )
    }
}
#[cfg(feature = "Win32_System_Com_StructuredStorage")]
pub trait ITsSbTargetPropertySetImpl: Sized + ITsSbPropertySetImpl + IPropertyBagImpl {}
#[cfg(feature = "Win32_System_Com_StructuredStorage")]
impl ::windows::core::RuntimeName for ITsSbTargetPropertySet {
    const NAME: &'static str = "Windows.Win32.System.RemoteDesktop.ITsSbTargetPropertySet";
}
#[cfg(feature = "Win32_System_Com_StructuredStorage")]
impl ITsSbTargetPropertySetVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITsSbTargetPropertySetImpl, const OFFSET: isize>() -> ITsSbTargetPropertySetVtbl {
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ITsSbTargetPropertySet>, ::windows::core::GetTrustLevel)
    }
}
pub trait ITsSbTaskInfoImpl: Sized {
    fn TargetId();
    fn StartTime();
    fn EndTime();
    fn Deadline();
    fn Identifier();
    fn Label();
    fn Context();
    fn Plugin();
    fn Status();
}
impl ::windows::core::RuntimeName for ITsSbTaskInfo {
    const NAME: &'static str = "Windows.Win32.System.RemoteDesktop.ITsSbTaskInfo";
}
impl ITsSbTaskInfoVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITsSbTaskInfoImpl, const OFFSET: isize>() -> ITsSbTaskInfoVtbl {
        unsafe extern "system" fn TargetId<Impl: ITsSbTaskInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pname: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TargetId(::core::mem::transmute_copy(&pname)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn StartTime<Impl: ITsSbTaskInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pstarttime: *mut super::super::Foundation::FILETIME) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn EndTime<Impl: ITsSbTaskInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pendtime: *mut super::super::Foundation::FILETIME) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EndTime(::core::mem::transmute_copy(&pendtime)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Deadline<Impl: ITsSbTaskInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdeadline: *mut super::super::Foundation::FILETIME) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Deadline(::core::mem::transmute_copy(&pdeadline)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Identifier<Impl: ITsSbTaskInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pidentifier: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Identifier(::core::mem::transmute_copy(&pidentifier)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Label<Impl: ITsSbTaskInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plabel: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Label(::core::mem::transmute_copy(&plabel)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Context<Impl: ITsSbTaskInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcontext: *mut *mut super::Com::SAFEARRAY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Context(::core::mem::transmute_copy(&pcontext)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Plugin<Impl: ITsSbTaskInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pplugin: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Plugin(::core::mem::transmute_copy(&pplugin)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Status<Impl: ITsSbTaskInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pstatus: *mut RDV_TASK_STATUS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Status(::core::mem::transmute_copy(&pstatus)) {
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
            ::windows::core::GetRuntimeClassName::<ITsSbTaskInfo>,
            ::windows::core::GetTrustLevel,
            TargetId::<Impl, OFFSET>,
            StartTime::<Impl, OFFSET>,
            EndTime::<Impl, OFFSET>,
            Deadline::<Impl, OFFSET>,
            Identifier::<Impl, OFFSET>,
            Label::<Impl, OFFSET>,
            Context::<Impl, OFFSET>,
            Plugin::<Impl, OFFSET>,
            Status::<Impl, OFFSET>,
        )
    }
}
pub trait ITsSbTaskPluginImpl: Sized + ITsSbPluginImpl {
    fn InitializeTaskPlugin();
    fn SetTaskQueue();
}
impl ::windows::core::RuntimeName for ITsSbTaskPlugin {
    const NAME: &'static str = "Windows.Win32.System.RemoteDesktop.ITsSbTaskPlugin";
}
impl ITsSbTaskPluginVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITsSbTaskPluginImpl, const OFFSET: isize>() -> ITsSbTaskPluginVtbl {
        unsafe extern "system" fn InitializeTaskPlugin<Impl: ITsSbTaskPluginImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pitssbtaskpluginnotifysink: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).InitializeTaskPlugin(&*(&pitssbtaskpluginnotifysink as *const <ITsSbTaskPluginNotifySink as ::windows::core::Abi>::Abi as *const <ITsSbTaskPluginNotifySink as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetTaskQueue<Impl: ITsSbTaskPluginImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszhostname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, sbtaskinfosize: u32, pitssbtaskinfo: *const ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetTaskQueue(&*(&pszhostname as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), sbtaskinfosize, &*(&pitssbtaskinfo as *const <ITsSbTaskInfo as ::windows::core::Abi>::Abi as *const <ITsSbTaskInfo as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ITsSbTaskPlugin>, ::windows::core::GetTrustLevel, InitializeTaskPlugin::<Impl, OFFSET>, SetTaskQueue::<Impl, OFFSET>)
    }
}
pub trait ITsSbTaskPluginNotifySinkImpl: Sized + ITsSbBaseNotifySinkImpl {
    fn OnSetTaskTime();
    fn OnDeleteTaskTime();
    fn OnUpdateTaskStatus();
    fn OnReportTasks();
}
impl ::windows::core::RuntimeName for ITsSbTaskPluginNotifySink {
    const NAME: &'static str = "Windows.Win32.System.RemoteDesktop.ITsSbTaskPluginNotifySink";
}
impl ITsSbTaskPluginNotifySinkVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITsSbTaskPluginNotifySinkImpl, const OFFSET: isize>() -> ITsSbTaskPluginNotifySinkVtbl {
        unsafe extern "system" fn OnSetTaskTime<Impl: ITsSbTaskPluginNotifySinkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, sztargetname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, taskstarttime: super::super::Foundation::FILETIME, taskendtime: super::super::Foundation::FILETIME, taskdeadline: super::super::Foundation::FILETIME, sztasklabel: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, sztaskidentifier: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, sztaskplugin: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, dwtaskstatus: u32, sacontext: *const super::Com::SAFEARRAY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OnSetTaskTime(
                &*(&sztargetname as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&taskstarttime as *const <super::super::Foundation::FILETIME as ::windows::core::Abi>::Abi as *const <super::super::Foundation::FILETIME as ::windows::core::DefaultType>::DefaultType),
                &*(&taskendtime as *const <super::super::Foundation::FILETIME as ::windows::core::Abi>::Abi as *const <super::super::Foundation::FILETIME as ::windows::core::DefaultType>::DefaultType),
                &*(&taskdeadline as *const <super::super::Foundation::FILETIME as ::windows::core::Abi>::Abi as *const <super::super::Foundation::FILETIME as ::windows::core::DefaultType>::DefaultType),
                &*(&sztasklabel as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&sztaskidentifier as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&sztaskplugin as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType),
                dwtaskstatus,
                &*(&sacontext as *const <super::Com::SAFEARRAY as ::windows::core::Abi>::Abi as *const <super::Com::SAFEARRAY as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OnDeleteTaskTime<Impl: ITsSbTaskPluginNotifySinkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, sztargetname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, sztaskidentifier: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OnDeleteTaskTime(&*(&sztargetname as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), &*(&sztaskidentifier as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OnUpdateTaskStatus<Impl: ITsSbTaskPluginNotifySinkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, sztargetname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, taskidentifier: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, taskstatus: RDV_TASK_STATUS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OnUpdateTaskStatus(&*(&sztargetname as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), &*(&taskidentifier as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), taskstatus) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OnReportTasks<Impl: ITsSbTaskPluginNotifySinkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, szhostname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OnReportTasks(&*(&szhostname as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ITsSbTaskPluginNotifySink>, ::windows::core::GetTrustLevel, OnSetTaskTime::<Impl, OFFSET>, OnDeleteTaskTime::<Impl, OFFSET>, OnUpdateTaskStatus::<Impl, OFFSET>, OnReportTasks::<Impl, OFFSET>)
    }
}
pub trait IWRdsEnhancedFastReconnectArbitratorImpl: Sized {
    fn GetSessionForEnhancedFastReconnect();
}
impl ::windows::core::RuntimeName for IWRdsEnhancedFastReconnectArbitrator {
    const NAME: &'static str = "Windows.Win32.System.RemoteDesktop.IWRdsEnhancedFastReconnectArbitrator";
}
impl IWRdsEnhancedFastReconnectArbitratorVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWRdsEnhancedFastReconnectArbitratorImpl, const OFFSET: isize>() -> IWRdsEnhancedFastReconnectArbitratorVtbl {
        unsafe extern "system" fn GetSessionForEnhancedFastReconnect<Impl: IWRdsEnhancedFastReconnectArbitratorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psessionidarray: *const i32, dwsessioncount: u32, presultsessionid: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetSessionForEnhancedFastReconnect(psessionidarray, dwsessioncount, ::core::mem::transmute_copy(&presultsessionid)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IWRdsEnhancedFastReconnectArbitrator>, ::windows::core::GetTrustLevel, GetSessionForEnhancedFastReconnect::<Impl, OFFSET>)
    }
}
pub trait IWRdsGraphicsChannelImpl: Sized {
    fn Write();
    fn Close();
    fn Open();
}
impl ::windows::core::RuntimeName for IWRdsGraphicsChannel {
    const NAME: &'static str = "Windows.Win32.System.RemoteDesktop.IWRdsGraphicsChannel";
}
impl IWRdsGraphicsChannelVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWRdsGraphicsChannelImpl, const OFFSET: isize>() -> IWRdsGraphicsChannelVtbl {
        unsafe extern "system" fn Write<Impl: IWRdsGraphicsChannelImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cbsize: u32, pbuffer: *const u8, pcontext: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Write(cbsize, pbuffer, &*(&pcontext as *const <::windows::core::IUnknown as ::windows::core::Abi>::Abi as *const <::windows::core::IUnknown as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Close<Impl: IWRdsGraphicsChannelImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Close() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Open<Impl: IWRdsGraphicsChannelImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pchannelevents: ::windows::core::RawPtr, popencontext: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Open(&*(&pchannelevents as *const <IWRdsGraphicsChannelEvents as ::windows::core::Abi>::Abi as *const <IWRdsGraphicsChannelEvents as ::windows::core::DefaultType>::DefaultType), &*(&popencontext as *const <::windows::core::IUnknown as ::windows::core::Abi>::Abi as *const <::windows::core::IUnknown as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IWRdsGraphicsChannel>, ::windows::core::GetTrustLevel, Write::<Impl, OFFSET>, Close::<Impl, OFFSET>, Open::<Impl, OFFSET>)
    }
}
pub trait IWRdsGraphicsChannelEventsImpl: Sized {
    fn OnDataReceived();
    fn OnClose();
    fn OnChannelOpened();
    fn OnDataSent();
    fn OnMetricsUpdate();
}
impl ::windows::core::RuntimeName for IWRdsGraphicsChannelEvents {
    const NAME: &'static str = "Windows.Win32.System.RemoteDesktop.IWRdsGraphicsChannelEvents";
}
impl IWRdsGraphicsChannelEventsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWRdsGraphicsChannelEventsImpl, const OFFSET: isize>() -> IWRdsGraphicsChannelEventsVtbl {
        unsafe extern "system" fn OnDataReceived<Impl: IWRdsGraphicsChannelEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cbsize: u32, pbuffer: *const u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OnDataReceived(cbsize, pbuffer) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OnClose<Impl: IWRdsGraphicsChannelEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OnClose() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OnChannelOpened<Impl: IWRdsGraphicsChannelEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, openresult: ::windows::core::HRESULT, popencontext: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OnChannelOpened(openresult, &*(&popencontext as *const <::windows::core::IUnknown as ::windows::core::Abi>::Abi as *const <::windows::core::IUnknown as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OnDataSent<Impl: IWRdsGraphicsChannelEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwritecontext: *mut ::core::ffi::c_void, bcancelled: super::super::Foundation::BOOL, pbuffer: *const u8, cbbuffer: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OnDataSent(&*(&pwritecontext as *const <::windows::core::IUnknown as ::windows::core::Abi>::Abi as *const <::windows::core::IUnknown as ::windows::core::DefaultType>::DefaultType), &*(&bcancelled as *const <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType), pbuffer, cbbuffer) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OnMetricsUpdate<Impl: IWRdsGraphicsChannelEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bandwidth: u32, rtt: u32, lastsentbyteindex: u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OnMetricsUpdate(bandwidth, rtt, lastsentbyteindex) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IWRdsGraphicsChannelEvents>, ::windows::core::GetTrustLevel, OnDataReceived::<Impl, OFFSET>, OnClose::<Impl, OFFSET>, OnChannelOpened::<Impl, OFFSET>, OnDataSent::<Impl, OFFSET>, OnMetricsUpdate::<Impl, OFFSET>)
    }
}
pub trait IWRdsGraphicsChannelManagerImpl: Sized {
    fn CreateChannel();
}
impl ::windows::core::RuntimeName for IWRdsGraphicsChannelManager {
    const NAME: &'static str = "Windows.Win32.System.RemoteDesktop.IWRdsGraphicsChannelManager";
}
impl IWRdsGraphicsChannelManagerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWRdsGraphicsChannelManagerImpl, const OFFSET: isize>() -> IWRdsGraphicsChannelManagerVtbl {
        unsafe extern "system" fn CreateChannel<Impl: IWRdsGraphicsChannelManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszchannelname: *const u8, channeltype: WRdsGraphicsChannelType, ppvirtualchannel: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateChannel(pszchannelname, channeltype, ::core::mem::transmute_copy(&ppvirtualchannel)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IWRdsGraphicsChannelManager>, ::windows::core::GetTrustLevel, CreateChannel::<Impl, OFFSET>)
    }
}
pub trait IWRdsProtocolConnectionImpl: Sized {
    fn GetLogonErrorRedirector();
    fn AcceptConnection();
    fn GetClientData();
    fn GetClientMonitorData();
    fn GetUserCredentials();
    fn GetLicenseConnection();
    fn AuthenticateClientToSession();
    fn NotifySessionId();
    fn GetInputHandles();
    fn GetVideoHandle();
    fn ConnectNotify();
    fn IsUserAllowedToLogon();
    fn SessionArbitrationEnumeration();
    fn LogonNotify();
    fn PreDisconnect();
    fn DisconnectNotify();
    fn Close();
    fn GetProtocolStatus();
    fn GetLastInputTime();
    fn SetErrorInfo();
    fn CreateVirtualChannel();
    fn QueryProperty();
    fn GetShadowConnection();
    fn NotifyCommandProcessCreated();
}
impl ::windows::core::RuntimeName for IWRdsProtocolConnection {
    const NAME: &'static str = "Windows.Win32.System.RemoteDesktop.IWRdsProtocolConnection";
}
impl IWRdsProtocolConnectionVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWRdsProtocolConnectionImpl, const OFFSET: isize>() -> IWRdsProtocolConnectionVtbl {
        unsafe extern "system" fn GetLogonErrorRedirector<Impl: IWRdsProtocolConnectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pplogonerrorredir: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetLogonErrorRedirector(::core::mem::transmute_copy(&pplogonerrorredir)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AcceptConnection<Impl: IWRdsProtocolConnectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AcceptConnection() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetClientData<Impl: IWRdsProtocolConnectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pclientdata: *mut WTS_CLIENT_DATA) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetClientData(::core::mem::transmute_copy(&pclientdata)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetClientMonitorData<Impl: IWRdsProtocolConnectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pnummonitors: *mut u32, pprimarymonitor: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetClientMonitorData(::core::mem::transmute_copy(&pnummonitors), ::core::mem::transmute_copy(&pprimarymonitor)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetUserCredentials<Impl: IWRdsProtocolConnectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pusercreds: *mut WTS_USER_CREDENTIAL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetUserCredentials(::core::mem::transmute_copy(&pusercreds)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetLicenseConnection<Impl: IWRdsProtocolConnectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pplicenseconnection: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetLicenseConnection(::core::mem::transmute_copy(&pplicenseconnection)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AuthenticateClientToSession<Impl: IWRdsProtocolConnectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, sessionid: *mut WTS_SESSION_ID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AuthenticateClientToSession(::core::mem::transmute_copy(&sessionid)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn NotifySessionId<Impl: IWRdsProtocolConnectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, sessionid: *const WTS_SESSION_ID, sessionhandle: super::super::Foundation::HANDLE_PTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).NotifySessionId(&*(&sessionid as *const <WTS_SESSION_ID as ::windows::core::Abi>::Abi as *const <WTS_SESSION_ID as ::windows::core::DefaultType>::DefaultType), &*(&sessionhandle as *const <super::super::Foundation::HANDLE_PTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::HANDLE_PTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetInputHandles<Impl: IWRdsProtocolConnectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pkeyboardhandle: *mut super::super::Foundation::HANDLE_PTR, pmousehandle: *mut super::super::Foundation::HANDLE_PTR, pbeephandle: *mut super::super::Foundation::HANDLE_PTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetInputHandles(::core::mem::transmute_copy(&pkeyboardhandle), ::core::mem::transmute_copy(&pmousehandle), ::core::mem::transmute_copy(&pbeephandle)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetVideoHandle<Impl: IWRdsProtocolConnectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvideohandle: *mut super::super::Foundation::HANDLE_PTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetVideoHandle(::core::mem::transmute_copy(&pvideohandle)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ConnectNotify<Impl: IWRdsProtocolConnectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, sessionid: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ConnectNotify(sessionid) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsUserAllowedToLogon<Impl: IWRdsProtocolConnectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, sessionid: u32, usertoken: super::super::Foundation::HANDLE_PTR, pdomainname: super::super::Foundation::PWSTR, pusername: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsUserAllowedToLogon(
                sessionid,
                &*(&usertoken as *const <super::super::Foundation::HANDLE_PTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::HANDLE_PTR as ::windows::core::DefaultType>::DefaultType),
                &*(&pdomainname as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&pusername as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SessionArbitrationEnumeration<Impl: IWRdsProtocolConnectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, husertoken: super::super::Foundation::HANDLE_PTR, bsinglesessionperuserenabled: super::super::Foundation::BOOL, psessionidarray: *mut u32, pdwsessionidentifiercount: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SessionArbitrationEnumeration(&*(&husertoken as *const <super::super::Foundation::HANDLE_PTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::HANDLE_PTR as ::windows::core::DefaultType>::DefaultType), &*(&bsinglesessionperuserenabled as *const <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&psessionidarray), pdwsessionidentifiercount) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LogonNotify<Impl: IWRdsProtocolConnectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hclienttoken: super::super::Foundation::HANDLE_PTR, wszusername: super::super::Foundation::PWSTR, wszdomainname: super::super::Foundation::PWSTR, sessionid: *const WTS_SESSION_ID, pwrdsconnectionsettings: *mut WRDS_CONNECTION_SETTINGS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LogonNotify(
                &*(&hclienttoken as *const <super::super::Foundation::HANDLE_PTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::HANDLE_PTR as ::windows::core::DefaultType>::DefaultType),
                &*(&wszusername as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&wszdomainname as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&sessionid as *const <WTS_SESSION_ID as ::windows::core::Abi>::Abi as *const <WTS_SESSION_ID as ::windows::core::DefaultType>::DefaultType),
                &*(&pwrdsconnectionsettings as *const <WRDS_CONNECTION_SETTINGS as ::windows::core::Abi>::Abi as *const <WRDS_CONNECTION_SETTINGS as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PreDisconnect<Impl: IWRdsProtocolConnectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, disconnectreason: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PreDisconnect(disconnectreason) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DisconnectNotify<Impl: IWRdsProtocolConnectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DisconnectNotify() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Close<Impl: IWRdsProtocolConnectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Close() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetProtocolStatus<Impl: IWRdsProtocolConnectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pprotocolstatus: *mut WTS_PROTOCOL_STATUS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetProtocolStatus(::core::mem::transmute_copy(&pprotocolstatus)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetLastInputTime<Impl: IWRdsProtocolConnectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plastinputtime: *mut u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetLastInputTime(::core::mem::transmute_copy(&plastinputtime)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetErrorInfo<Impl: IWRdsProtocolConnectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ulerror: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetErrorInfo(ulerror) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateVirtualChannel<Impl: IWRdsProtocolConnectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, szendpointname: super::super::Foundation::PSTR, bstatic: super::super::Foundation::BOOL, requestedpriority: u32, phchannel: *mut usize) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateVirtualChannel(&*(&szendpointname as *const <super::super::Foundation::PSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PSTR as ::windows::core::DefaultType>::DefaultType), &*(&bstatic as *const <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType), requestedpriority, ::core::mem::transmute_copy(&phchannel)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn QueryProperty<Impl: IWRdsProtocolConnectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, querytype: ::windows::core::GUID, ulnumentriesin: u32, ulnumentriesout: u32, ppropertyentriesin: *const WTS_PROPERTY_VALUE, ppropertyentriesout: *mut WTS_PROPERTY_VALUE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).QueryProperty(&*(&querytype as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), ulnumentriesin, ulnumentriesout, &*(&ppropertyentriesin as *const <WTS_PROPERTY_VALUE as ::windows::core::Abi>::Abi as *const <WTS_PROPERTY_VALUE as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&ppropertyentriesout)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetShadowConnection<Impl: IWRdsProtocolConnectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppshadowconnection: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetShadowConnection(::core::mem::transmute_copy(&ppshadowconnection)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn NotifyCommandProcessCreated<Impl: IWRdsProtocolConnectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, sessionid: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).NotifyCommandProcessCreated(sessionid) {
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
            ::windows::core::GetRuntimeClassName::<IWRdsProtocolConnection>,
            ::windows::core::GetTrustLevel,
            GetLogonErrorRedirector::<Impl, OFFSET>,
            AcceptConnection::<Impl, OFFSET>,
            GetClientData::<Impl, OFFSET>,
            GetClientMonitorData::<Impl, OFFSET>,
            GetUserCredentials::<Impl, OFFSET>,
            GetLicenseConnection::<Impl, OFFSET>,
            AuthenticateClientToSession::<Impl, OFFSET>,
            NotifySessionId::<Impl, OFFSET>,
            GetInputHandles::<Impl, OFFSET>,
            GetVideoHandle::<Impl, OFFSET>,
            ConnectNotify::<Impl, OFFSET>,
            IsUserAllowedToLogon::<Impl, OFFSET>,
            SessionArbitrationEnumeration::<Impl, OFFSET>,
            LogonNotify::<Impl, OFFSET>,
            PreDisconnect::<Impl, OFFSET>,
            DisconnectNotify::<Impl, OFFSET>,
            Close::<Impl, OFFSET>,
            GetProtocolStatus::<Impl, OFFSET>,
            GetLastInputTime::<Impl, OFFSET>,
            SetErrorInfo::<Impl, OFFSET>,
            CreateVirtualChannel::<Impl, OFFSET>,
            QueryProperty::<Impl, OFFSET>,
            GetShadowConnection::<Impl, OFFSET>,
            NotifyCommandProcessCreated::<Impl, OFFSET>,
        )
    }
}
pub trait IWRdsProtocolConnectionCallbackImpl: Sized {
    fn OnReady();
    fn BrokenConnection();
    fn StopScreenUpdates();
    fn RedrawWindow();
    fn GetConnectionId();
}
impl ::windows::core::RuntimeName for IWRdsProtocolConnectionCallback {
    const NAME: &'static str = "Windows.Win32.System.RemoteDesktop.IWRdsProtocolConnectionCallback";
}
impl IWRdsProtocolConnectionCallbackVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWRdsProtocolConnectionCallbackImpl, const OFFSET: isize>() -> IWRdsProtocolConnectionCallbackVtbl {
        unsafe extern "system" fn OnReady<Impl: IWRdsProtocolConnectionCallbackImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OnReady() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BrokenConnection<Impl: IWRdsProtocolConnectionCallbackImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, reason: u32, source: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).BrokenConnection(reason, source) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn StopScreenUpdates<Impl: IWRdsProtocolConnectionCallbackImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).StopScreenUpdates() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RedrawWindow<Impl: IWRdsProtocolConnectionCallbackImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rect: *const WTS_SMALL_RECT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RedrawWindow(&*(&rect as *const <WTS_SMALL_RECT as ::windows::core::Abi>::Abi as *const <WTS_SMALL_RECT as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetConnectionId<Impl: IWRdsProtocolConnectionCallbackImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pconnectionid: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetConnectionId(::core::mem::transmute_copy(&pconnectionid)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IWRdsProtocolConnectionCallback>, ::windows::core::GetTrustLevel, OnReady::<Impl, OFFSET>, BrokenConnection::<Impl, OFFSET>, StopScreenUpdates::<Impl, OFFSET>, RedrawWindow::<Impl, OFFSET>, GetConnectionId::<Impl, OFFSET>)
    }
}
pub trait IWRdsProtocolConnectionSettingsImpl: Sized {
    fn SetConnectionSetting();
    fn GetConnectionSetting();
}
impl ::windows::core::RuntimeName for IWRdsProtocolConnectionSettings {
    const NAME: &'static str = "Windows.Win32.System.RemoteDesktop.IWRdsProtocolConnectionSettings";
}
impl IWRdsProtocolConnectionSettingsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWRdsProtocolConnectionSettingsImpl, const OFFSET: isize>() -> IWRdsProtocolConnectionSettingsVtbl {
        unsafe extern "system" fn SetConnectionSetting<Impl: IWRdsProtocolConnectionSettingsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, propertyid: ::windows::core::GUID, ppropertyentriesin: *const WTS_PROPERTY_VALUE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetConnectionSetting(&*(&propertyid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), &*(&ppropertyentriesin as *const <WTS_PROPERTY_VALUE as ::windows::core::Abi>::Abi as *const <WTS_PROPERTY_VALUE as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetConnectionSetting<Impl: IWRdsProtocolConnectionSettingsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, propertyid: ::windows::core::GUID, ppropertyentriesout: *mut WTS_PROPERTY_VALUE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetConnectionSetting(&*(&propertyid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&ppropertyentriesout)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IWRdsProtocolConnectionSettings>, ::windows::core::GetTrustLevel, SetConnectionSetting::<Impl, OFFSET>, GetConnectionSetting::<Impl, OFFSET>)
    }
}
pub trait IWRdsProtocolLicenseConnectionImpl: Sized {
    fn RequestLicensingCapabilities();
    fn SendClientLicense();
    fn RequestClientLicense();
    fn ProtocolComplete();
}
impl ::windows::core::RuntimeName for IWRdsProtocolLicenseConnection {
    const NAME: &'static str = "Windows.Win32.System.RemoteDesktop.IWRdsProtocolLicenseConnection";
}
impl IWRdsProtocolLicenseConnectionVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWRdsProtocolLicenseConnectionImpl, const OFFSET: isize>() -> IWRdsProtocolLicenseConnectionVtbl {
        unsafe extern "system" fn RequestLicensingCapabilities<Impl: IWRdsProtocolLicenseConnectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pplicensecapabilities: *mut WTS_LICENSE_CAPABILITIES, pcblicensecapabilities: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RequestLicensingCapabilities(::core::mem::transmute_copy(&pplicensecapabilities), pcblicensecapabilities) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SendClientLicense<Impl: IWRdsProtocolLicenseConnectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pclientlicense: *const u8, cbclientlicense: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SendClientLicense(pclientlicense, cbclientlicense) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RequestClientLicense<Impl: IWRdsProtocolLicenseConnectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, reserve1: *const u8, reserve2: u32, ppclientlicense: *mut u8, pcbclientlicense: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RequestClientLicense(reserve1, reserve2, ::core::mem::transmute_copy(&ppclientlicense), pcbclientlicense) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ProtocolComplete<Impl: IWRdsProtocolLicenseConnectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ulcomplete: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ProtocolComplete(ulcomplete) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IWRdsProtocolLicenseConnection>, ::windows::core::GetTrustLevel, RequestLicensingCapabilities::<Impl, OFFSET>, SendClientLicense::<Impl, OFFSET>, RequestClientLicense::<Impl, OFFSET>, ProtocolComplete::<Impl, OFFSET>)
    }
}
pub trait IWRdsProtocolListenerImpl: Sized {
    fn GetSettings();
    fn StartListen();
    fn StopListen();
}
impl ::windows::core::RuntimeName for IWRdsProtocolListener {
    const NAME: &'static str = "Windows.Win32.System.RemoteDesktop.IWRdsProtocolListener";
}
impl IWRdsProtocolListenerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWRdsProtocolListenerImpl, const OFFSET: isize>() -> IWRdsProtocolListenerVtbl {
        unsafe extern "system" fn GetSettings<Impl: IWRdsProtocolListenerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wrdslistenersettinglevel: WRDS_LISTENER_SETTING_LEVEL, pwrdslistenersettings: *mut WRDS_LISTENER_SETTINGS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetSettings(wrdslistenersettinglevel, ::core::mem::transmute_copy(&pwrdslistenersettings)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn StartListen<Impl: IWRdsProtocolListenerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcallback: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).StartListen(&*(&pcallback as *const <IWRdsProtocolListenerCallback as ::windows::core::Abi>::Abi as *const <IWRdsProtocolListenerCallback as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn StopListen<Impl: IWRdsProtocolListenerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).StopListen() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IWRdsProtocolListener>, ::windows::core::GetTrustLevel, GetSettings::<Impl, OFFSET>, StartListen::<Impl, OFFSET>, StopListen::<Impl, OFFSET>)
    }
}
pub trait IWRdsProtocolListenerCallbackImpl: Sized {
    fn OnConnected();
}
impl ::windows::core::RuntimeName for IWRdsProtocolListenerCallback {
    const NAME: &'static str = "Windows.Win32.System.RemoteDesktop.IWRdsProtocolListenerCallback";
}
impl IWRdsProtocolListenerCallbackVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWRdsProtocolListenerCallbackImpl, const OFFSET: isize>() -> IWRdsProtocolListenerCallbackVtbl {
        unsafe extern "system" fn OnConnected<Impl: IWRdsProtocolListenerCallbackImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pconnection: ::windows::core::RawPtr, pwrdsconnectionsettings: *const WRDS_CONNECTION_SETTINGS, pcallback: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OnConnected(&*(&pconnection as *const <IWRdsProtocolConnection as ::windows::core::Abi>::Abi as *const <IWRdsProtocolConnection as ::windows::core::DefaultType>::DefaultType), &*(&pwrdsconnectionsettings as *const <WRDS_CONNECTION_SETTINGS as ::windows::core::Abi>::Abi as *const <WRDS_CONNECTION_SETTINGS as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&pcallback)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IWRdsProtocolListenerCallback>, ::windows::core::GetTrustLevel, OnConnected::<Impl, OFFSET>)
    }
}
pub trait IWRdsProtocolLogonErrorRedirectorImpl: Sized {
    fn OnBeginPainting();
    fn RedirectStatus();
    fn RedirectMessage();
    fn RedirectLogonError();
}
impl ::windows::core::RuntimeName for IWRdsProtocolLogonErrorRedirector {
    const NAME: &'static str = "Windows.Win32.System.RemoteDesktop.IWRdsProtocolLogonErrorRedirector";
}
impl IWRdsProtocolLogonErrorRedirectorVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWRdsProtocolLogonErrorRedirectorImpl, const OFFSET: isize>() -> IWRdsProtocolLogonErrorRedirectorVtbl {
        unsafe extern "system" fn OnBeginPainting<Impl: IWRdsProtocolLogonErrorRedirectorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OnBeginPainting() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RedirectStatus<Impl: IWRdsProtocolLogonErrorRedirectorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszmessage: super::super::Foundation::PWSTR, presponse: *mut WTS_LOGON_ERROR_REDIRECTOR_RESPONSE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RedirectStatus(&*(&pszmessage as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&presponse)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RedirectMessage<Impl: IWRdsProtocolLogonErrorRedirectorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszcaption: super::super::Foundation::PWSTR, pszmessage: super::super::Foundation::PWSTR, utype: u32, presponse: *mut WTS_LOGON_ERROR_REDIRECTOR_RESPONSE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RedirectMessage(&*(&pszcaption as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), &*(&pszmessage as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), utype, ::core::mem::transmute_copy(&presponse)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RedirectLogonError<Impl: IWRdsProtocolLogonErrorRedirectorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ntsstatus: i32, ntssubstatus: i32, pszcaption: super::super::Foundation::PWSTR, pszmessage: super::super::Foundation::PWSTR, utype: u32, presponse: *mut WTS_LOGON_ERROR_REDIRECTOR_RESPONSE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RedirectLogonError(ntsstatus, ntssubstatus, &*(&pszcaption as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), &*(&pszmessage as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), utype, ::core::mem::transmute_copy(&presponse)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IWRdsProtocolLogonErrorRedirector>, ::windows::core::GetTrustLevel, OnBeginPainting::<Impl, OFFSET>, RedirectStatus::<Impl, OFFSET>, RedirectMessage::<Impl, OFFSET>, RedirectLogonError::<Impl, OFFSET>)
    }
}
pub trait IWRdsProtocolManagerImpl: Sized {
    fn Initialize();
    fn CreateListener();
    fn NotifyServiceStateChange();
    fn NotifySessionOfServiceStart();
    fn NotifySessionOfServiceStop();
    fn NotifySessionStateChange();
    fn NotifySettingsChange();
    fn Uninitialize();
}
impl ::windows::core::RuntimeName for IWRdsProtocolManager {
    const NAME: &'static str = "Windows.Win32.System.RemoteDesktop.IWRdsProtocolManager";
}
impl IWRdsProtocolManagerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWRdsProtocolManagerImpl, const OFFSET: isize>() -> IWRdsProtocolManagerVtbl {
        unsafe extern "system" fn Initialize<Impl: IWRdsProtocolManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, piwrdssettings: ::windows::core::RawPtr, pwrdssettings: *const WRDS_SETTINGS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Initialize(&*(&piwrdssettings as *const <IWRdsProtocolSettings as ::windows::core::Abi>::Abi as *const <IWRdsProtocolSettings as ::windows::core::DefaultType>::DefaultType), &*(&pwrdssettings as *const <WRDS_SETTINGS as ::windows::core::Abi>::Abi as *const <WRDS_SETTINGS as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateListener<Impl: IWRdsProtocolManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wszlistenername: super::super::Foundation::PWSTR, pprotocollistener: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateListener(&*(&wszlistenername as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&pprotocollistener)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn NotifyServiceStateChange<Impl: IWRdsProtocolManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ptsservicestatechange: *const WTS_SERVICE_STATE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).NotifyServiceStateChange(&*(&ptsservicestatechange as *const <WTS_SERVICE_STATE as ::windows::core::Abi>::Abi as *const <WTS_SERVICE_STATE as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn NotifySessionOfServiceStart<Impl: IWRdsProtocolManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, sessionid: *const WTS_SESSION_ID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).NotifySessionOfServiceStart(&*(&sessionid as *const <WTS_SESSION_ID as ::windows::core::Abi>::Abi as *const <WTS_SESSION_ID as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn NotifySessionOfServiceStop<Impl: IWRdsProtocolManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, sessionid: *const WTS_SESSION_ID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).NotifySessionOfServiceStop(&*(&sessionid as *const <WTS_SESSION_ID as ::windows::core::Abi>::Abi as *const <WTS_SESSION_ID as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn NotifySessionStateChange<Impl: IWRdsProtocolManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, sessionid: *const WTS_SESSION_ID, eventid: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).NotifySessionStateChange(&*(&sessionid as *const <WTS_SESSION_ID as ::windows::core::Abi>::Abi as *const <WTS_SESSION_ID as ::windows::core::DefaultType>::DefaultType), eventid) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn NotifySettingsChange<Impl: IWRdsProtocolManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwrdssettings: *const WRDS_SETTINGS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).NotifySettingsChange(&*(&pwrdssettings as *const <WRDS_SETTINGS as ::windows::core::Abi>::Abi as *const <WRDS_SETTINGS as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Uninitialize<Impl: IWRdsProtocolManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Uninitialize() {
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
            ::windows::core::GetRuntimeClassName::<IWRdsProtocolManager>,
            ::windows::core::GetTrustLevel,
            Initialize::<Impl, OFFSET>,
            CreateListener::<Impl, OFFSET>,
            NotifyServiceStateChange::<Impl, OFFSET>,
            NotifySessionOfServiceStart::<Impl, OFFSET>,
            NotifySessionOfServiceStop::<Impl, OFFSET>,
            NotifySessionStateChange::<Impl, OFFSET>,
            NotifySettingsChange::<Impl, OFFSET>,
            Uninitialize::<Impl, OFFSET>,
        )
    }
}
pub trait IWRdsProtocolSettingsImpl: Sized {
    fn GetSettings();
    fn MergeSettings();
}
impl ::windows::core::RuntimeName for IWRdsProtocolSettings {
    const NAME: &'static str = "Windows.Win32.System.RemoteDesktop.IWRdsProtocolSettings";
}
impl IWRdsProtocolSettingsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWRdsProtocolSettingsImpl, const OFFSET: isize>() -> IWRdsProtocolSettingsVtbl {
        unsafe extern "system" fn GetSettings<Impl: IWRdsProtocolSettingsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wrdssettingtype: WRDS_SETTING_TYPE, wrdssettinglevel: WRDS_SETTING_LEVEL, pwrdssettings: *mut WRDS_SETTINGS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetSettings(wrdssettingtype, wrdssettinglevel, ::core::mem::transmute_copy(&pwrdssettings)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MergeSettings<Impl: IWRdsProtocolSettingsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwrdssettings: *const WRDS_SETTINGS, wrdsconnectionsettinglevel: WRDS_CONNECTION_SETTING_LEVEL, pwrdsconnectionsettings: *mut WRDS_CONNECTION_SETTINGS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MergeSettings(&*(&pwrdssettings as *const <WRDS_SETTINGS as ::windows::core::Abi>::Abi as *const <WRDS_SETTINGS as ::windows::core::DefaultType>::DefaultType), wrdsconnectionsettinglevel, &*(&pwrdsconnectionsettings as *const <WRDS_CONNECTION_SETTINGS as ::windows::core::Abi>::Abi as *const <WRDS_CONNECTION_SETTINGS as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IWRdsProtocolSettings>, ::windows::core::GetTrustLevel, GetSettings::<Impl, OFFSET>, MergeSettings::<Impl, OFFSET>)
    }
}
pub trait IWRdsProtocolShadowCallbackImpl: Sized {
    fn StopShadow();
    fn InvokeTargetShadow();
}
impl ::windows::core::RuntimeName for IWRdsProtocolShadowCallback {
    const NAME: &'static str = "Windows.Win32.System.RemoteDesktop.IWRdsProtocolShadowCallback";
}
impl IWRdsProtocolShadowCallbackVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWRdsProtocolShadowCallbackImpl, const OFFSET: isize>() -> IWRdsProtocolShadowCallbackVtbl {
        unsafe extern "system" fn StopShadow<Impl: IWRdsProtocolShadowCallbackImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).StopShadow() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn InvokeTargetShadow<Impl: IWRdsProtocolShadowCallbackImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ptargetservername: super::super::Foundation::PWSTR, targetsessionid: u32, pparam1: *const u8, param1size: u32, pparam2: *const u8, param2size: u32, pparam3: *const u8, param3size: u32, pparam4: *const u8, param4size: u32, pclientname: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).InvokeTargetShadow(&*(&ptargetservername as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), targetsessionid, pparam1, param1size, pparam2, param2size, pparam3, param3size, pparam4, param4size, &*(&pclientname as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IWRdsProtocolShadowCallback>, ::windows::core::GetTrustLevel, StopShadow::<Impl, OFFSET>, InvokeTargetShadow::<Impl, OFFSET>)
    }
}
pub trait IWRdsProtocolShadowConnectionImpl: Sized {
    fn Start();
    fn Stop();
    fn DoTarget();
}
impl ::windows::core::RuntimeName for IWRdsProtocolShadowConnection {
    const NAME: &'static str = "Windows.Win32.System.RemoteDesktop.IWRdsProtocolShadowConnection";
}
impl IWRdsProtocolShadowConnectionVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWRdsProtocolShadowConnectionImpl, const OFFSET: isize>() -> IWRdsProtocolShadowConnectionVtbl {
        unsafe extern "system" fn Start<Impl: IWRdsProtocolShadowConnectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ptargetservername: super::super::Foundation::PWSTR, targetsessionid: u32, hotkeyvk: u8, hotkeymodifiers: u16, pshadowcallback: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Start(&*(&ptargetservername as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), targetsessionid, hotkeyvk, hotkeymodifiers, &*(&pshadowcallback as *const <IWRdsProtocolShadowCallback as ::windows::core::Abi>::Abi as *const <IWRdsProtocolShadowCallback as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Stop<Impl: IWRdsProtocolShadowConnectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Stop() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DoTarget<Impl: IWRdsProtocolShadowConnectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pparam1: *const u8, param1size: u32, pparam2: *const u8, param2size: u32, pparam3: *const u8, param3size: u32, pparam4: *const u8, param4size: u32, pclientname: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DoTarget(pparam1, param1size, pparam2, param2size, pparam3, param3size, pparam4, param4size, &*(&pclientname as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IWRdsProtocolShadowConnection>, ::windows::core::GetTrustLevel, Start::<Impl, OFFSET>, Stop::<Impl, OFFSET>, DoTarget::<Impl, OFFSET>)
    }
}
pub trait IWRdsWddmIddPropsImpl: Sized {
    fn GetHardwareId();
    fn OnDriverLoad();
    fn OnDriverUnload();
    fn EnableWddmIdd();
}
impl ::windows::core::RuntimeName for IWRdsWddmIddProps {
    const NAME: &'static str = "Windows.Win32.System.RemoteDesktop.IWRdsWddmIddProps";
}
impl IWRdsWddmIddPropsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWRdsWddmIddPropsImpl, const OFFSET: isize>() -> IWRdsWddmIddPropsVtbl {
        unsafe extern "system" fn GetHardwareId<Impl: IWRdsWddmIddPropsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdisplaydriverhardwareid: super::super::Foundation::PWSTR, count: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetHardwareId(&*(&pdisplaydriverhardwareid as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), count) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OnDriverLoad<Impl: IWRdsWddmIddPropsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, sessionid: u32, driverhandle: super::super::Foundation::HANDLE_PTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OnDriverLoad(sessionid, &*(&driverhandle as *const <super::super::Foundation::HANDLE_PTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::HANDLE_PTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OnDriverUnload<Impl: IWRdsWddmIddPropsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, sessionid: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OnDriverUnload(sessionid) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnableWddmIdd<Impl: IWRdsWddmIddPropsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, enabled: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EnableWddmIdd(&*(&enabled as *const <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IWRdsWddmIddProps>, ::windows::core::GetTrustLevel, GetHardwareId::<Impl, OFFSET>, OnDriverLoad::<Impl, OFFSET>, OnDriverUnload::<Impl, OFFSET>, EnableWddmIdd::<Impl, OFFSET>)
    }
}
pub trait IWTSBitmapRenderServiceImpl: Sized {
    fn GetMappedRenderer();
}
impl ::windows::core::RuntimeName for IWTSBitmapRenderService {
    const NAME: &'static str = "Windows.Win32.System.RemoteDesktop.IWTSBitmapRenderService";
}
impl IWTSBitmapRenderServiceVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWTSBitmapRenderServiceImpl, const OFFSET: isize>() -> IWTSBitmapRenderServiceVtbl {
        unsafe extern "system" fn GetMappedRenderer<Impl: IWTSBitmapRenderServiceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, mappingid: u64, pmappedrenderercallback: ::windows::core::RawPtr, ppmappedrenderer: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetMappedRenderer(mappingid, &*(&pmappedrenderercallback as *const <IWTSBitmapRendererCallback as ::windows::core::Abi>::Abi as *const <IWTSBitmapRendererCallback as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&ppmappedrenderer)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IWTSBitmapRenderService>, ::windows::core::GetTrustLevel, GetMappedRenderer::<Impl, OFFSET>)
    }
}
pub trait IWTSBitmapRendererImpl: Sized {
    fn Render();
    fn GetRendererStatistics();
    fn RemoveMapping();
}
impl ::windows::core::RuntimeName for IWTSBitmapRenderer {
    const NAME: &'static str = "Windows.Win32.System.RemoteDesktop.IWTSBitmapRenderer";
}
impl IWTSBitmapRendererVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWTSBitmapRendererImpl, const OFFSET: isize>() -> IWTSBitmapRendererVtbl {
        unsafe extern "system" fn Render<Impl: IWTSBitmapRendererImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, imageformat: ::windows::core::GUID, dwwidth: u32, dwheight: u32, cbstride: i32, cbimagebuffer: u32, pimagebuffer: *const u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Render(&*(&imageformat as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), dwwidth, dwheight, cbstride, cbimagebuffer, pimagebuffer) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetRendererStatistics<Impl: IWTSBitmapRendererImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pstatistics: *mut BITMAP_RENDERER_STATISTICS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetRendererStatistics(::core::mem::transmute_copy(&pstatistics)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveMapping<Impl: IWTSBitmapRendererImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RemoveMapping() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IWTSBitmapRenderer>, ::windows::core::GetTrustLevel, Render::<Impl, OFFSET>, GetRendererStatistics::<Impl, OFFSET>, RemoveMapping::<Impl, OFFSET>)
    }
}
pub trait IWTSBitmapRendererCallbackImpl: Sized {
    fn OnTargetSizeChanged();
}
impl ::windows::core::RuntimeName for IWTSBitmapRendererCallback {
    const NAME: &'static str = "Windows.Win32.System.RemoteDesktop.IWTSBitmapRendererCallback";
}
impl IWTSBitmapRendererCallbackVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWTSBitmapRendererCallbackImpl, const OFFSET: isize>() -> IWTSBitmapRendererCallbackVtbl {
        unsafe extern "system" fn OnTargetSizeChanged<Impl: IWTSBitmapRendererCallbackImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rcnewsize: super::super::Foundation::RECT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OnTargetSizeChanged(&*(&rcnewsize as *const <super::super::Foundation::RECT as ::windows::core::Abi>::Abi as *const <super::super::Foundation::RECT as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IWTSBitmapRendererCallback>, ::windows::core::GetTrustLevel, OnTargetSizeChanged::<Impl, OFFSET>)
    }
}
pub trait IWTSListenerImpl: Sized {
    fn GetConfiguration();
}
impl ::windows::core::RuntimeName for IWTSListener {
    const NAME: &'static str = "Windows.Win32.System.RemoteDesktop.IWTSListener";
}
impl IWTSListenerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWTSListenerImpl, const OFFSET: isize>() -> IWTSListenerVtbl {
        unsafe extern "system" fn GetConfiguration<Impl: IWTSListenerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pppropertybag: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetConfiguration(::core::mem::transmute_copy(&pppropertybag)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IWTSListener>, ::windows::core::GetTrustLevel, GetConfiguration::<Impl, OFFSET>)
    }
}
pub trait IWTSListenerCallbackImpl: Sized {
    fn OnNewChannelConnection();
}
impl ::windows::core::RuntimeName for IWTSListenerCallback {
    const NAME: &'static str = "Windows.Win32.System.RemoteDesktop.IWTSListenerCallback";
}
impl IWTSListenerCallbackVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWTSListenerCallbackImpl, const OFFSET: isize>() -> IWTSListenerCallbackVtbl {
        unsafe extern "system" fn OnNewChannelConnection<Impl: IWTSListenerCallbackImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pchannel: ::windows::core::RawPtr, data: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pbaccept: *mut super::super::Foundation::BOOL, ppcallback: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OnNewChannelConnection(&*(&pchannel as *const <IWTSVirtualChannel as ::windows::core::Abi>::Abi as *const <IWTSVirtualChannel as ::windows::core::DefaultType>::DefaultType), &*(&data as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&pbaccept), ::core::mem::transmute_copy(&ppcallback)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IWTSListenerCallback>, ::windows::core::GetTrustLevel, OnNewChannelConnection::<Impl, OFFSET>)
    }
}
pub trait IWTSPluginImpl: Sized {
    fn Initialize();
    fn Connected();
    fn Disconnected();
    fn Terminated();
}
impl ::windows::core::RuntimeName for IWTSPlugin {
    const NAME: &'static str = "Windows.Win32.System.RemoteDesktop.IWTSPlugin";
}
impl IWTSPluginVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWTSPluginImpl, const OFFSET: isize>() -> IWTSPluginVtbl {
        unsafe extern "system" fn Initialize<Impl: IWTSPluginImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pchannelmgr: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Initialize(&*(&pchannelmgr as *const <IWTSVirtualChannelManager as ::windows::core::Abi>::Abi as *const <IWTSVirtualChannelManager as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Connected<Impl: IWTSPluginImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Connected() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Disconnected<Impl: IWTSPluginImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwdisconnectcode: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Disconnected(dwdisconnectcode) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Terminated<Impl: IWTSPluginImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Terminated() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IWTSPlugin>, ::windows::core::GetTrustLevel, Initialize::<Impl, OFFSET>, Connected::<Impl, OFFSET>, Disconnected::<Impl, OFFSET>, Terminated::<Impl, OFFSET>)
    }
}
pub trait IWTSPluginServiceProviderImpl: Sized {
    fn GetService();
}
impl ::windows::core::RuntimeName for IWTSPluginServiceProvider {
    const NAME: &'static str = "Windows.Win32.System.RemoteDesktop.IWTSPluginServiceProvider";
}
impl IWTSPluginServiceProviderVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWTSPluginServiceProviderImpl, const OFFSET: isize>() -> IWTSPluginServiceProviderVtbl {
        unsafe extern "system" fn GetService<Impl: IWTSPluginServiceProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, serviceid: ::windows::core::GUID, ppunkobject: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetService(&*(&serviceid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&ppunkobject)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IWTSPluginServiceProvider>, ::windows::core::GetTrustLevel, GetService::<Impl, OFFSET>)
    }
}
pub trait IWTSProtocolConnectionImpl: Sized {
    fn GetLogonErrorRedirector();
    fn SendPolicyData();
    fn AcceptConnection();
    fn GetClientData();
    fn GetUserCredentials();
    fn GetLicenseConnection();
    fn AuthenticateClientToSession();
    fn NotifySessionId();
    fn GetProtocolHandles();
    fn ConnectNotify();
    fn IsUserAllowedToLogon();
    fn SessionArbitrationEnumeration();
    fn LogonNotify();
    fn GetUserData();
    fn DisconnectNotify();
    fn Close();
    fn GetProtocolStatus();
    fn GetLastInputTime();
    fn SetErrorInfo();
    fn SendBeep();
    fn CreateVirtualChannel();
    fn QueryProperty();
    fn GetShadowConnection();
}
impl ::windows::core::RuntimeName for IWTSProtocolConnection {
    const NAME: &'static str = "Windows.Win32.System.RemoteDesktop.IWTSProtocolConnection";
}
impl IWTSProtocolConnectionVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWTSProtocolConnectionImpl, const OFFSET: isize>() -> IWTSProtocolConnectionVtbl {
        unsafe extern "system" fn GetLogonErrorRedirector<Impl: IWTSProtocolConnectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pplogonerrorredir: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetLogonErrorRedirector(::core::mem::transmute_copy(&pplogonerrorredir)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SendPolicyData<Impl: IWTSProtocolConnectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppolicydata: *const WTS_POLICY_DATA) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SendPolicyData(&*(&ppolicydata as *const <WTS_POLICY_DATA as ::windows::core::Abi>::Abi as *const <WTS_POLICY_DATA as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AcceptConnection<Impl: IWTSProtocolConnectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AcceptConnection() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetClientData<Impl: IWTSProtocolConnectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pclientdata: *mut WTS_CLIENT_DATA) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetClientData(::core::mem::transmute_copy(&pclientdata)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetUserCredentials<Impl: IWTSProtocolConnectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pusercreds: *mut WTS_USER_CREDENTIAL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetUserCredentials(::core::mem::transmute_copy(&pusercreds)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetLicenseConnection<Impl: IWTSProtocolConnectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pplicenseconnection: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetLicenseConnection(::core::mem::transmute_copy(&pplicenseconnection)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AuthenticateClientToSession<Impl: IWTSProtocolConnectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, sessionid: *mut WTS_SESSION_ID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AuthenticateClientToSession(::core::mem::transmute_copy(&sessionid)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn NotifySessionId<Impl: IWTSProtocolConnectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, sessionid: *const WTS_SESSION_ID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).NotifySessionId(&*(&sessionid as *const <WTS_SESSION_ID as ::windows::core::Abi>::Abi as *const <WTS_SESSION_ID as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetProtocolHandles<Impl: IWTSProtocolConnectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pkeyboardhandle: *mut super::super::Foundation::HANDLE_PTR, pmousehandle: *mut super::super::Foundation::HANDLE_PTR, pbeephandle: *mut super::super::Foundation::HANDLE_PTR, pvideohandle: *mut super::super::Foundation::HANDLE_PTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetProtocolHandles(::core::mem::transmute_copy(&pkeyboardhandle), ::core::mem::transmute_copy(&pmousehandle), ::core::mem::transmute_copy(&pbeephandle), ::core::mem::transmute_copy(&pvideohandle)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ConnectNotify<Impl: IWTSProtocolConnectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, sessionid: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ConnectNotify(sessionid) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsUserAllowedToLogon<Impl: IWTSProtocolConnectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, sessionid: u32, usertoken: super::super::Foundation::HANDLE_PTR, pdomainname: super::super::Foundation::PWSTR, pusername: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsUserAllowedToLogon(
                sessionid,
                &*(&usertoken as *const <super::super::Foundation::HANDLE_PTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::HANDLE_PTR as ::windows::core::DefaultType>::DefaultType),
                &*(&pdomainname as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&pusername as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SessionArbitrationEnumeration<Impl: IWTSProtocolConnectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, husertoken: super::super::Foundation::HANDLE_PTR, bsinglesessionperuserenabled: super::super::Foundation::BOOL, psessionidarray: *mut u32, pdwsessionidentifiercount: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SessionArbitrationEnumeration(&*(&husertoken as *const <super::super::Foundation::HANDLE_PTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::HANDLE_PTR as ::windows::core::DefaultType>::DefaultType), &*(&bsinglesessionperuserenabled as *const <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&psessionidarray), pdwsessionidentifiercount) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LogonNotify<Impl: IWTSProtocolConnectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hclienttoken: super::super::Foundation::HANDLE_PTR, wszusername: super::super::Foundation::PWSTR, wszdomainname: super::super::Foundation::PWSTR, sessionid: *const WTS_SESSION_ID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LogonNotify(
                &*(&hclienttoken as *const <super::super::Foundation::HANDLE_PTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::HANDLE_PTR as ::windows::core::DefaultType>::DefaultType),
                &*(&wszusername as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&wszdomainname as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&sessionid as *const <WTS_SESSION_ID as ::windows::core::Abi>::Abi as *const <WTS_SESSION_ID as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetUserData<Impl: IWTSProtocolConnectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppolicydata: *const WTS_POLICY_DATA, pclientdata: *mut WTS_USER_DATA) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetUserData(&*(&ppolicydata as *const <WTS_POLICY_DATA as ::windows::core::Abi>::Abi as *const <WTS_POLICY_DATA as ::windows::core::DefaultType>::DefaultType), &*(&pclientdata as *const <WTS_USER_DATA as ::windows::core::Abi>::Abi as *const <WTS_USER_DATA as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DisconnectNotify<Impl: IWTSProtocolConnectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DisconnectNotify() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Close<Impl: IWTSProtocolConnectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Close() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetProtocolStatus<Impl: IWTSProtocolConnectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pprotocolstatus: *mut WTS_PROTOCOL_STATUS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetProtocolStatus(::core::mem::transmute_copy(&pprotocolstatus)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetLastInputTime<Impl: IWTSProtocolConnectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plastinputtime: *mut u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetLastInputTime(::core::mem::transmute_copy(&plastinputtime)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetErrorInfo<Impl: IWTSProtocolConnectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ulerror: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetErrorInfo(ulerror) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SendBeep<Impl: IWTSProtocolConnectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, frequency: u32, duration: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SendBeep(frequency, duration) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateVirtualChannel<Impl: IWTSProtocolConnectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, szendpointname: super::super::Foundation::PSTR, bstatic: super::super::Foundation::BOOL, requestedpriority: u32, phchannel: *mut usize) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateVirtualChannel(&*(&szendpointname as *const <super::super::Foundation::PSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PSTR as ::windows::core::DefaultType>::DefaultType), &*(&bstatic as *const <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType), requestedpriority, ::core::mem::transmute_copy(&phchannel)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn QueryProperty<Impl: IWTSProtocolConnectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, querytype: ::windows::core::GUID, ulnumentriesin: u32, ulnumentriesout: u32, ppropertyentriesin: *const WTS_PROPERTY_VALUE, ppropertyentriesout: *mut WTS_PROPERTY_VALUE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).QueryProperty(&*(&querytype as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), ulnumentriesin, ulnumentriesout, &*(&ppropertyentriesin as *const <WTS_PROPERTY_VALUE as ::windows::core::Abi>::Abi as *const <WTS_PROPERTY_VALUE as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&ppropertyentriesout)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetShadowConnection<Impl: IWTSProtocolConnectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppshadowconnection: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetShadowConnection(::core::mem::transmute_copy(&ppshadowconnection)) {
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
            ::windows::core::GetRuntimeClassName::<IWTSProtocolConnection>,
            ::windows::core::GetTrustLevel,
            GetLogonErrorRedirector::<Impl, OFFSET>,
            SendPolicyData::<Impl, OFFSET>,
            AcceptConnection::<Impl, OFFSET>,
            GetClientData::<Impl, OFFSET>,
            GetUserCredentials::<Impl, OFFSET>,
            GetLicenseConnection::<Impl, OFFSET>,
            AuthenticateClientToSession::<Impl, OFFSET>,
            NotifySessionId::<Impl, OFFSET>,
            GetProtocolHandles::<Impl, OFFSET>,
            ConnectNotify::<Impl, OFFSET>,
            IsUserAllowedToLogon::<Impl, OFFSET>,
            SessionArbitrationEnumeration::<Impl, OFFSET>,
            LogonNotify::<Impl, OFFSET>,
            GetUserData::<Impl, OFFSET>,
            DisconnectNotify::<Impl, OFFSET>,
            Close::<Impl, OFFSET>,
            GetProtocolStatus::<Impl, OFFSET>,
            GetLastInputTime::<Impl, OFFSET>,
            SetErrorInfo::<Impl, OFFSET>,
            SendBeep::<Impl, OFFSET>,
            CreateVirtualChannel::<Impl, OFFSET>,
            QueryProperty::<Impl, OFFSET>,
            GetShadowConnection::<Impl, OFFSET>,
        )
    }
}
pub trait IWTSProtocolConnectionCallbackImpl: Sized {
    fn OnReady();
    fn BrokenConnection();
    fn StopScreenUpdates();
    fn RedrawWindow();
    fn DisplayIOCtl();
}
impl ::windows::core::RuntimeName for IWTSProtocolConnectionCallback {
    const NAME: &'static str = "Windows.Win32.System.RemoteDesktop.IWTSProtocolConnectionCallback";
}
impl IWTSProtocolConnectionCallbackVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWTSProtocolConnectionCallbackImpl, const OFFSET: isize>() -> IWTSProtocolConnectionCallbackVtbl {
        unsafe extern "system" fn OnReady<Impl: IWTSProtocolConnectionCallbackImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OnReady() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BrokenConnection<Impl: IWTSProtocolConnectionCallbackImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, reason: u32, source: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).BrokenConnection(reason, source) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn StopScreenUpdates<Impl: IWTSProtocolConnectionCallbackImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).StopScreenUpdates() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RedrawWindow<Impl: IWTSProtocolConnectionCallbackImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rect: *const WTS_SMALL_RECT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RedrawWindow(&*(&rect as *const <WTS_SMALL_RECT as ::windows::core::Abi>::Abi as *const <WTS_SMALL_RECT as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DisplayIOCtl<Impl: IWTSProtocolConnectionCallbackImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, displayioctl: *const WTS_DISPLAY_IOCTL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DisplayIOCtl(&*(&displayioctl as *const <WTS_DISPLAY_IOCTL as ::windows::core::Abi>::Abi as *const <WTS_DISPLAY_IOCTL as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IWTSProtocolConnectionCallback>, ::windows::core::GetTrustLevel, OnReady::<Impl, OFFSET>, BrokenConnection::<Impl, OFFSET>, StopScreenUpdates::<Impl, OFFSET>, RedrawWindow::<Impl, OFFSET>, DisplayIOCtl::<Impl, OFFSET>)
    }
}
pub trait IWTSProtocolLicenseConnectionImpl: Sized {
    fn RequestLicensingCapabilities();
    fn SendClientLicense();
    fn RequestClientLicense();
    fn ProtocolComplete();
}
impl ::windows::core::RuntimeName for IWTSProtocolLicenseConnection {
    const NAME: &'static str = "Windows.Win32.System.RemoteDesktop.IWTSProtocolLicenseConnection";
}
impl IWTSProtocolLicenseConnectionVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWTSProtocolLicenseConnectionImpl, const OFFSET: isize>() -> IWTSProtocolLicenseConnectionVtbl {
        unsafe extern "system" fn RequestLicensingCapabilities<Impl: IWTSProtocolLicenseConnectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pplicensecapabilities: *mut WTS_LICENSE_CAPABILITIES, pcblicensecapabilities: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RequestLicensingCapabilities(::core::mem::transmute_copy(&pplicensecapabilities), pcblicensecapabilities) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SendClientLicense<Impl: IWTSProtocolLicenseConnectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pclientlicense: *const u8, cbclientlicense: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SendClientLicense(pclientlicense, cbclientlicense) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RequestClientLicense<Impl: IWTSProtocolLicenseConnectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, reserve1: *const u8, reserve2: u32, ppclientlicense: *mut u8, pcbclientlicense: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RequestClientLicense(reserve1, reserve2, ::core::mem::transmute_copy(&ppclientlicense), pcbclientlicense) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ProtocolComplete<Impl: IWTSProtocolLicenseConnectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ulcomplete: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ProtocolComplete(ulcomplete) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IWTSProtocolLicenseConnection>, ::windows::core::GetTrustLevel, RequestLicensingCapabilities::<Impl, OFFSET>, SendClientLicense::<Impl, OFFSET>, RequestClientLicense::<Impl, OFFSET>, ProtocolComplete::<Impl, OFFSET>)
    }
}
pub trait IWTSProtocolListenerImpl: Sized {
    fn StartListen();
    fn StopListen();
}
impl ::windows::core::RuntimeName for IWTSProtocolListener {
    const NAME: &'static str = "Windows.Win32.System.RemoteDesktop.IWTSProtocolListener";
}
impl IWTSProtocolListenerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWTSProtocolListenerImpl, const OFFSET: isize>() -> IWTSProtocolListenerVtbl {
        unsafe extern "system" fn StartListen<Impl: IWTSProtocolListenerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcallback: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).StartListen(&*(&pcallback as *const <IWTSProtocolListenerCallback as ::windows::core::Abi>::Abi as *const <IWTSProtocolListenerCallback as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn StopListen<Impl: IWTSProtocolListenerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).StopListen() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IWTSProtocolListener>, ::windows::core::GetTrustLevel, StartListen::<Impl, OFFSET>, StopListen::<Impl, OFFSET>)
    }
}
pub trait IWTSProtocolListenerCallbackImpl: Sized {
    fn OnConnected();
}
impl ::windows::core::RuntimeName for IWTSProtocolListenerCallback {
    const NAME: &'static str = "Windows.Win32.System.RemoteDesktop.IWTSProtocolListenerCallback";
}
impl IWTSProtocolListenerCallbackVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWTSProtocolListenerCallbackImpl, const OFFSET: isize>() -> IWTSProtocolListenerCallbackVtbl {
        unsafe extern "system" fn OnConnected<Impl: IWTSProtocolListenerCallbackImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pconnection: ::windows::core::RawPtr, pcallback: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OnConnected(&*(&pconnection as *const <IWTSProtocolConnection as ::windows::core::Abi>::Abi as *const <IWTSProtocolConnection as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&pcallback)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IWTSProtocolListenerCallback>, ::windows::core::GetTrustLevel, OnConnected::<Impl, OFFSET>)
    }
}
pub trait IWTSProtocolLogonErrorRedirectorImpl: Sized {
    fn OnBeginPainting();
    fn RedirectStatus();
    fn RedirectMessage();
    fn RedirectLogonError();
}
impl ::windows::core::RuntimeName for IWTSProtocolLogonErrorRedirector {
    const NAME: &'static str = "Windows.Win32.System.RemoteDesktop.IWTSProtocolLogonErrorRedirector";
}
impl IWTSProtocolLogonErrorRedirectorVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWTSProtocolLogonErrorRedirectorImpl, const OFFSET: isize>() -> IWTSProtocolLogonErrorRedirectorVtbl {
        unsafe extern "system" fn OnBeginPainting<Impl: IWTSProtocolLogonErrorRedirectorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OnBeginPainting() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RedirectStatus<Impl: IWTSProtocolLogonErrorRedirectorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszmessage: super::super::Foundation::PWSTR, presponse: *mut WTS_LOGON_ERROR_REDIRECTOR_RESPONSE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RedirectStatus(&*(&pszmessage as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&presponse)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RedirectMessage<Impl: IWTSProtocolLogonErrorRedirectorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszcaption: super::super::Foundation::PWSTR, pszmessage: super::super::Foundation::PWSTR, utype: u32, presponse: *mut WTS_LOGON_ERROR_REDIRECTOR_RESPONSE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RedirectMessage(&*(&pszcaption as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), &*(&pszmessage as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), utype, ::core::mem::transmute_copy(&presponse)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RedirectLogonError<Impl: IWTSProtocolLogonErrorRedirectorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ntsstatus: i32, ntssubstatus: i32, pszcaption: super::super::Foundation::PWSTR, pszmessage: super::super::Foundation::PWSTR, utype: u32, presponse: *mut WTS_LOGON_ERROR_REDIRECTOR_RESPONSE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RedirectLogonError(ntsstatus, ntssubstatus, &*(&pszcaption as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), &*(&pszmessage as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), utype, ::core::mem::transmute_copy(&presponse)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IWTSProtocolLogonErrorRedirector>, ::windows::core::GetTrustLevel, OnBeginPainting::<Impl, OFFSET>, RedirectStatus::<Impl, OFFSET>, RedirectMessage::<Impl, OFFSET>, RedirectLogonError::<Impl, OFFSET>)
    }
}
pub trait IWTSProtocolManagerImpl: Sized {
    fn CreateListener();
    fn NotifyServiceStateChange();
    fn NotifySessionOfServiceStart();
    fn NotifySessionOfServiceStop();
    fn NotifySessionStateChange();
}
impl ::windows::core::RuntimeName for IWTSProtocolManager {
    const NAME: &'static str = "Windows.Win32.System.RemoteDesktop.IWTSProtocolManager";
}
impl IWTSProtocolManagerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWTSProtocolManagerImpl, const OFFSET: isize>() -> IWTSProtocolManagerVtbl {
        unsafe extern "system" fn CreateListener<Impl: IWTSProtocolManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wszlistenername: super::super::Foundation::PWSTR, pprotocollistener: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateListener(&*(&wszlistenername as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&pprotocollistener)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn NotifyServiceStateChange<Impl: IWTSProtocolManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ptsservicestatechange: *const WTS_SERVICE_STATE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).NotifyServiceStateChange(&*(&ptsservicestatechange as *const <WTS_SERVICE_STATE as ::windows::core::Abi>::Abi as *const <WTS_SERVICE_STATE as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn NotifySessionOfServiceStart<Impl: IWTSProtocolManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, sessionid: *const WTS_SESSION_ID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).NotifySessionOfServiceStart(&*(&sessionid as *const <WTS_SESSION_ID as ::windows::core::Abi>::Abi as *const <WTS_SESSION_ID as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn NotifySessionOfServiceStop<Impl: IWTSProtocolManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, sessionid: *const WTS_SESSION_ID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).NotifySessionOfServiceStop(&*(&sessionid as *const <WTS_SESSION_ID as ::windows::core::Abi>::Abi as *const <WTS_SESSION_ID as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn NotifySessionStateChange<Impl: IWTSProtocolManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, sessionid: *const WTS_SESSION_ID, eventid: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).NotifySessionStateChange(&*(&sessionid as *const <WTS_SESSION_ID as ::windows::core::Abi>::Abi as *const <WTS_SESSION_ID as ::windows::core::DefaultType>::DefaultType), eventid) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IWTSProtocolManager>, ::windows::core::GetTrustLevel, CreateListener::<Impl, OFFSET>, NotifyServiceStateChange::<Impl, OFFSET>, NotifySessionOfServiceStart::<Impl, OFFSET>, NotifySessionOfServiceStop::<Impl, OFFSET>, NotifySessionStateChange::<Impl, OFFSET>)
    }
}
pub trait IWTSProtocolShadowCallbackImpl: Sized {
    fn StopShadow();
    fn InvokeTargetShadow();
}
impl ::windows::core::RuntimeName for IWTSProtocolShadowCallback {
    const NAME: &'static str = "Windows.Win32.System.RemoteDesktop.IWTSProtocolShadowCallback";
}
impl IWTSProtocolShadowCallbackVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWTSProtocolShadowCallbackImpl, const OFFSET: isize>() -> IWTSProtocolShadowCallbackVtbl {
        unsafe extern "system" fn StopShadow<Impl: IWTSProtocolShadowCallbackImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).StopShadow() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn InvokeTargetShadow<Impl: IWTSProtocolShadowCallbackImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ptargetservername: super::super::Foundation::PWSTR, targetsessionid: u32, pparam1: *const u8, param1size: u32, pparam2: *const u8, param2size: u32, pparam3: *const u8, param3size: u32, pparam4: *const u8, param4size: u32, pclientname: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).InvokeTargetShadow(&*(&ptargetservername as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), targetsessionid, pparam1, param1size, pparam2, param2size, pparam3, param3size, pparam4, param4size, &*(&pclientname as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IWTSProtocolShadowCallback>, ::windows::core::GetTrustLevel, StopShadow::<Impl, OFFSET>, InvokeTargetShadow::<Impl, OFFSET>)
    }
}
pub trait IWTSProtocolShadowConnectionImpl: Sized {
    fn Start();
    fn Stop();
    fn DoTarget();
}
impl ::windows::core::RuntimeName for IWTSProtocolShadowConnection {
    const NAME: &'static str = "Windows.Win32.System.RemoteDesktop.IWTSProtocolShadowConnection";
}
impl IWTSProtocolShadowConnectionVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWTSProtocolShadowConnectionImpl, const OFFSET: isize>() -> IWTSProtocolShadowConnectionVtbl {
        unsafe extern "system" fn Start<Impl: IWTSProtocolShadowConnectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ptargetservername: super::super::Foundation::PWSTR, targetsessionid: u32, hotkeyvk: u8, hotkeymodifiers: u16, pshadowcallback: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Start(&*(&ptargetservername as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), targetsessionid, hotkeyvk, hotkeymodifiers, &*(&pshadowcallback as *const <IWTSProtocolShadowCallback as ::windows::core::Abi>::Abi as *const <IWTSProtocolShadowCallback as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Stop<Impl: IWTSProtocolShadowConnectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Stop() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DoTarget<Impl: IWTSProtocolShadowConnectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pparam1: *const u8, param1size: u32, pparam2: *const u8, param2size: u32, pparam3: *const u8, param3size: u32, pparam4: *const u8, param4size: u32, pclientname: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DoTarget(pparam1, param1size, pparam2, param2size, pparam3, param3size, pparam4, param4size, &*(&pclientname as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IWTSProtocolShadowConnection>, ::windows::core::GetTrustLevel, Start::<Impl, OFFSET>, Stop::<Impl, OFFSET>, DoTarget::<Impl, OFFSET>)
    }
}
pub trait IWTSSBPluginImpl: Sized {
    fn Initialize();
    fn WTSSBX_MachineChangeNotification();
    fn WTSSBX_SessionChangeNotification();
    fn WTSSBX_GetMostSuitableServer();
    fn Terminated();
    fn WTSSBX_GetUserExternalSession();
}
impl ::windows::core::RuntimeName for IWTSSBPlugin {
    const NAME: &'static str = "Windows.Win32.System.RemoteDesktop.IWTSSBPlugin";
}
impl IWTSSBPluginVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWTSSBPluginImpl, const OFFSET: isize>() -> IWTSSBPluginVtbl {
        unsafe extern "system" fn Initialize<Impl: IWTSSBPluginImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plugincapabilities: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Initialize(::core::mem::transmute_copy(&plugincapabilities)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn WTSSBX_MachineChangeNotification<Impl: IWTSSBPluginImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, notificationtype: WTSSBX_NOTIFICATION_TYPE, machineid: i32, pmachineinfo: *const WTSSBX_MACHINE_INFO) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).WTSSBX_MachineChangeNotification(notificationtype, machineid, &*(&pmachineinfo as *const <WTSSBX_MACHINE_INFO as ::windows::core::Abi>::Abi as *const <WTSSBX_MACHINE_INFO as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn WTSSBX_SessionChangeNotification<Impl: IWTSSBPluginImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, notificationtype: WTSSBX_NOTIFICATION_TYPE, machineid: i32, numofsessions: u32, sessioninfo: *const WTSSBX_SESSION_INFO) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).WTSSBX_SessionChangeNotification(notificationtype, machineid, numofsessions, &*(&sessioninfo as *const <WTSSBX_SESSION_INFO as ::windows::core::Abi>::Abi as *const <WTSSBX_SESSION_INFO as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn WTSSBX_GetMostSuitableServer<Impl: IWTSSBPluginImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, username: super::super::Foundation::PWSTR, domainname: super::super::Foundation::PWSTR, applicationtype: super::super::Foundation::PWSTR, farmname: super::super::Foundation::PWSTR, pmachineid: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).WTSSBX_GetMostSuitableServer(
                &*(&username as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&domainname as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&applicationtype as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&farmname as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType),
                pmachineid,
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Terminated<Impl: IWTSSBPluginImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Terminated() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn WTSSBX_GetUserExternalSession<Impl: IWTSSBPluginImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, username: super::super::Foundation::PWSTR, domainname: super::super::Foundation::PWSTR, applicationtype: super::super::Foundation::PWSTR, redirectorinternalip: *const WTSSBX_IP_ADDRESS, psessionid: *mut u32, pmachineconnectinfo: *mut WTSSBX_MACHINE_CONNECT_INFO) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).WTSSBX_GetUserExternalSession(
                &*(&username as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&domainname as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&applicationtype as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&redirectorinternalip as *const <WTSSBX_IP_ADDRESS as ::windows::core::Abi>::Abi as *const <WTSSBX_IP_ADDRESS as ::windows::core::DefaultType>::DefaultType),
                ::core::mem::transmute_copy(&psessionid),
                ::core::mem::transmute_copy(&pmachineconnectinfo),
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
            ::windows::core::GetRuntimeClassName::<IWTSSBPlugin>,
            ::windows::core::GetTrustLevel,
            Initialize::<Impl, OFFSET>,
            WTSSBX_MachineChangeNotification::<Impl, OFFSET>,
            WTSSBX_SessionChangeNotification::<Impl, OFFSET>,
            WTSSBX_GetMostSuitableServer::<Impl, OFFSET>,
            Terminated::<Impl, OFFSET>,
            WTSSBX_GetUserExternalSession::<Impl, OFFSET>,
        )
    }
}
pub trait IWTSVirtualChannelImpl: Sized {
    fn Write();
    fn Close();
}
impl ::windows::core::RuntimeName for IWTSVirtualChannel {
    const NAME: &'static str = "Windows.Win32.System.RemoteDesktop.IWTSVirtualChannel";
}
impl IWTSVirtualChannelVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWTSVirtualChannelImpl, const OFFSET: isize>() -> IWTSVirtualChannelVtbl {
        unsafe extern "system" fn Write<Impl: IWTSVirtualChannelImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cbsize: u32, pbuffer: *const u8, preserved: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Write(cbsize, pbuffer, &*(&preserved as *const <::windows::core::IUnknown as ::windows::core::Abi>::Abi as *const <::windows::core::IUnknown as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Close<Impl: IWTSVirtualChannelImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Close() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IWTSVirtualChannel>, ::windows::core::GetTrustLevel, Write::<Impl, OFFSET>, Close::<Impl, OFFSET>)
    }
}
pub trait IWTSVirtualChannelCallbackImpl: Sized {
    fn OnDataReceived();
    fn OnClose();
}
impl ::windows::core::RuntimeName for IWTSVirtualChannelCallback {
    const NAME: &'static str = "Windows.Win32.System.RemoteDesktop.IWTSVirtualChannelCallback";
}
impl IWTSVirtualChannelCallbackVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWTSVirtualChannelCallbackImpl, const OFFSET: isize>() -> IWTSVirtualChannelCallbackVtbl {
        unsafe extern "system" fn OnDataReceived<Impl: IWTSVirtualChannelCallbackImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cbsize: u32, pbuffer: *const u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OnDataReceived(cbsize, pbuffer) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OnClose<Impl: IWTSVirtualChannelCallbackImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OnClose() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IWTSVirtualChannelCallback>, ::windows::core::GetTrustLevel, OnDataReceived::<Impl, OFFSET>, OnClose::<Impl, OFFSET>)
    }
}
pub trait IWTSVirtualChannelManagerImpl: Sized {
    fn CreateListener();
}
impl ::windows::core::RuntimeName for IWTSVirtualChannelManager {
    const NAME: &'static str = "Windows.Win32.System.RemoteDesktop.IWTSVirtualChannelManager";
}
impl IWTSVirtualChannelManagerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWTSVirtualChannelManagerImpl, const OFFSET: isize>() -> IWTSVirtualChannelManagerVtbl {
        unsafe extern "system" fn CreateListener<Impl: IWTSVirtualChannelManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszchannelname: *const u8, uflags: u32, plistenercallback: ::windows::core::RawPtr, pplistener: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateListener(pszchannelname, uflags, &*(&plistenercallback as *const <IWTSListenerCallback as ::windows::core::Abi>::Abi as *const <IWTSListenerCallback as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&pplistener)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IWTSVirtualChannelManager>, ::windows::core::GetTrustLevel, CreateListener::<Impl, OFFSET>)
    }
}
pub trait IWorkspaceImpl: Sized {
    fn GetWorkspaceNames();
    fn StartRemoteApplication();
    fn GetProcessId();
}
impl ::windows::core::RuntimeName for IWorkspace {
    const NAME: &'static str = "Windows.Win32.System.RemoteDesktop.IWorkspace";
}
impl IWorkspaceVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWorkspaceImpl, const OFFSET: isize>() -> IWorkspaceVtbl {
        unsafe extern "system" fn GetWorkspaceNames<Impl: IWorkspaceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psawkspnames: *mut *mut super::Com::SAFEARRAY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetWorkspaceNames(::core::mem::transmute_copy(&psawkspnames)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn StartRemoteApplication<Impl: IWorkspaceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrworkspaceid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, psaparams: *const super::Com::SAFEARRAY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).StartRemoteApplication(&*(&bstrworkspaceid as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), &*(&psaparams as *const <super::Com::SAFEARRAY as ::windows::core::Abi>::Abi as *const <super::Com::SAFEARRAY as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetProcessId<Impl: IWorkspaceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pulprocessid: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetProcessId(::core::mem::transmute_copy(&pulprocessid)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IWorkspace>, ::windows::core::GetTrustLevel, GetWorkspaceNames::<Impl, OFFSET>, StartRemoteApplication::<Impl, OFFSET>, GetProcessId::<Impl, OFFSET>)
    }
}
pub trait IWorkspace2Impl: Sized + IWorkspaceImpl {
    fn StartRemoteApplicationEx();
}
impl ::windows::core::RuntimeName for IWorkspace2 {
    const NAME: &'static str = "Windows.Win32.System.RemoteDesktop.IWorkspace2";
}
impl IWorkspace2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWorkspace2Impl, const OFFSET: isize>() -> IWorkspace2Vtbl {
        unsafe extern "system" fn StartRemoteApplicationEx<Impl: IWorkspace2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrworkspaceid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrrequestingappid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrrequestingappfamilyname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, blaunchintoimmersiveclient: i16, bstrimmersiveclientactivationcontext: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, psaparams: *const super::Com::SAFEARRAY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).StartRemoteApplicationEx(
                &*(&bstrworkspaceid as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&bstrrequestingappid as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&bstrrequestingappfamilyname as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType),
                blaunchintoimmersiveclient,
                &*(&bstrimmersiveclientactivationcontext as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&psaparams as *const <super::Com::SAFEARRAY as ::windows::core::Abi>::Abi as *const <super::Com::SAFEARRAY as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IWorkspace2>, ::windows::core::GetTrustLevel, StartRemoteApplicationEx::<Impl, OFFSET>)
    }
}
pub trait IWorkspace3Impl: Sized + IWorkspace2Impl + IWorkspaceImpl {
    fn GetClaimsToken2();
    fn SetClaimsToken();
}
impl ::windows::core::RuntimeName for IWorkspace3 {
    const NAME: &'static str = "Windows.Win32.System.RemoteDesktop.IWorkspace3";
}
impl IWorkspace3Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWorkspace3Impl, const OFFSET: isize>() -> IWorkspace3Vtbl {
        unsafe extern "system" fn GetClaimsToken2<Impl: IWorkspace3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrclaimshint: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstruserhint: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, claimcookie: u32, hwndcreduiparent: u32, rectcreduiparent: super::super::Foundation::RECT, pbstraccesstoken: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetClaimsToken2(
                &*(&bstrclaimshint as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&bstruserhint as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType),
                claimcookie,
                hwndcreduiparent,
                &*(&rectcreduiparent as *const <super::super::Foundation::RECT as ::windows::core::Abi>::Abi as *const <super::super::Foundation::RECT as ::windows::core::DefaultType>::DefaultType),
                ::core::mem::transmute_copy(&pbstraccesstoken),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetClaimsToken<Impl: IWorkspace3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstraccesstoken: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ullaccesstokenexpiration: u64, bstrrefreshtoken: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetClaimsToken(&*(&bstraccesstoken as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), ullaccesstokenexpiration, &*(&bstrrefreshtoken as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IWorkspace3>, ::windows::core::GetTrustLevel, GetClaimsToken2::<Impl, OFFSET>, SetClaimsToken::<Impl, OFFSET>)
    }
}
pub trait IWorkspaceClientExtImpl: Sized {
    fn GetResourceId();
    fn GetResourceDisplayName();
    fn IssueDisconnect();
}
impl ::windows::core::RuntimeName for IWorkspaceClientExt {
    const NAME: &'static str = "Windows.Win32.System.RemoteDesktop.IWorkspaceClientExt";
}
impl IWorkspaceClientExtVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWorkspaceClientExtImpl, const OFFSET: isize>() -> IWorkspaceClientExtVtbl {
        unsafe extern "system" fn GetResourceId<Impl: IWorkspaceClientExtImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrworkspaceid: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetResourceId(::core::mem::transmute_copy(&bstrworkspaceid)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetResourceDisplayName<Impl: IWorkspaceClientExtImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrworkspacedisplayname: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetResourceDisplayName(::core::mem::transmute_copy(&bstrworkspacedisplayname)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IssueDisconnect<Impl: IWorkspaceClientExtImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IssueDisconnect() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IWorkspaceClientExt>, ::windows::core::GetTrustLevel, GetResourceId::<Impl, OFFSET>, GetResourceDisplayName::<Impl, OFFSET>, IssueDisconnect::<Impl, OFFSET>)
    }
}
pub trait IWorkspaceRegistrationImpl: Sized {
    fn AddResource();
    fn RemoveResource();
}
impl ::windows::core::RuntimeName for IWorkspaceRegistration {
    const NAME: &'static str = "Windows.Win32.System.RemoteDesktop.IWorkspaceRegistration";
}
impl IWorkspaceRegistrationVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWorkspaceRegistrationImpl, const OFFSET: isize>() -> IWorkspaceRegistrationVtbl {
        unsafe extern "system" fn AddResource<Impl: IWorkspaceRegistrationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, punk: ::windows::core::RawPtr, pdwcookie: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AddResource(&*(&punk as *const <IWorkspaceClientExt as ::windows::core::Abi>::Abi as *const <IWorkspaceClientExt as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&pdwcookie)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveResource<Impl: IWorkspaceRegistrationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwcookieconnection: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RemoveResource(dwcookieconnection) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IWorkspaceRegistration>, ::windows::core::GetTrustLevel, AddResource::<Impl, OFFSET>, RemoveResource::<Impl, OFFSET>)
    }
}
pub trait IWorkspaceRegistration2Impl: Sized + IWorkspaceRegistrationImpl {
    fn AddResourceEx();
    fn RemoveResourceEx();
}
impl ::windows::core::RuntimeName for IWorkspaceRegistration2 {
    const NAME: &'static str = "Windows.Win32.System.RemoteDesktop.IWorkspaceRegistration2";
}
impl IWorkspaceRegistration2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWorkspaceRegistration2Impl, const OFFSET: isize>() -> IWorkspaceRegistration2Vtbl {
        unsafe extern "system" fn AddResourceEx<Impl: IWorkspaceRegistration2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, punk: ::windows::core::RawPtr, bstreventloguploadaddress: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pdwcookie: *mut u32, correlationid: ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AddResourceEx(
                &*(&punk as *const <IWorkspaceClientExt as ::windows::core::Abi>::Abi as *const <IWorkspaceClientExt as ::windows::core::DefaultType>::DefaultType),
                &*(&bstreventloguploadaddress as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType),
                ::core::mem::transmute_copy(&pdwcookie),
                &*(&correlationid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveResourceEx<Impl: IWorkspaceRegistration2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwcookieconnection: u32, correlationid: ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RemoveResourceEx(dwcookieconnection, &*(&correlationid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IWorkspaceRegistration2>, ::windows::core::GetTrustLevel, AddResourceEx::<Impl, OFFSET>, RemoveResourceEx::<Impl, OFFSET>)
    }
}
pub trait IWorkspaceReportMessageImpl: Sized {
    fn RegisterErrorLogMessage();
    fn IsErrorMessageRegistered();
    fn RegisterErrorEvent();
}
impl ::windows::core::RuntimeName for IWorkspaceReportMessage {
    const NAME: &'static str = "Windows.Win32.System.RemoteDesktop.IWorkspaceReportMessage";
}
impl IWorkspaceReportMessageVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWorkspaceReportMessageImpl, const OFFSET: isize>() -> IWorkspaceReportMessageVtbl {
        unsafe extern "system" fn RegisterErrorLogMessage<Impl: IWorkspaceReportMessageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrmessage: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RegisterErrorLogMessage(&*(&bstrmessage as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsErrorMessageRegistered<Impl: IWorkspaceReportMessageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrwkspid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, dwerrortype: u32, bstrerrormessagetype: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, dwerrorcode: u32, pferrorexist: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsErrorMessageRegistered(&*(&bstrwkspid as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), dwerrortype, &*(&bstrerrormessagetype as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), dwerrorcode, ::core::mem::transmute_copy(&pferrorexist)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RegisterErrorEvent<Impl: IWorkspaceReportMessageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrwkspid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, dwerrortype: u32, bstrerrormessagetype: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, dwerrorcode: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RegisterErrorEvent(&*(&bstrwkspid as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), dwerrortype, &*(&bstrerrormessagetype as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), dwerrorcode) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IWorkspaceReportMessage>, ::windows::core::GetTrustLevel, RegisterErrorLogMessage::<Impl, OFFSET>, IsErrorMessageRegistered::<Impl, OFFSET>, RegisterErrorEvent::<Impl, OFFSET>)
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IWorkspaceResTypeRegistryImpl: Sized + IDispatchImpl {
    fn AddResourceType();
    fn DeleteResourceType();
    fn GetRegisteredFileExtensions();
    fn GetResourceTypeInfo();
    fn ModifyResourceType();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for IWorkspaceResTypeRegistry {
    const NAME: &'static str = "Windows.Win32.System.RemoteDesktop.IWorkspaceResTypeRegistry";
}
#[cfg(feature = "Win32_System_Com")]
impl IWorkspaceResTypeRegistryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWorkspaceResTypeRegistryImpl, const OFFSET: isize>() -> IWorkspaceResTypeRegistryVtbl {
        unsafe extern "system" fn AddResourceType<Impl: IWorkspaceResTypeRegistryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fmachinewide: i16, bstrfileextension: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrlauncher: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AddResourceType(fmachinewide, &*(&bstrfileextension as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), &*(&bstrlauncher as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DeleteResourceType<Impl: IWorkspaceResTypeRegistryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fmachinewide: i16, bstrfileextension: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DeleteResourceType(fmachinewide, &*(&bstrfileextension as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetRegisteredFileExtensions<Impl: IWorkspaceResTypeRegistryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fmachinewide: i16, psafileextensions: *mut *mut super::Com::SAFEARRAY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetRegisteredFileExtensions(fmachinewide, ::core::mem::transmute_copy(&psafileextensions)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetResourceTypeInfo<Impl: IWorkspaceResTypeRegistryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fmachinewide: i16, bstrfileextension: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pbstrlauncher: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetResourceTypeInfo(fmachinewide, &*(&bstrfileextension as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&pbstrlauncher)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ModifyResourceType<Impl: IWorkspaceResTypeRegistryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fmachinewide: i16, bstrfileextension: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrlauncher: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ModifyResourceType(fmachinewide, &*(&bstrfileextension as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), &*(&bstrlauncher as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IWorkspaceResTypeRegistry>, ::windows::core::GetTrustLevel, AddResourceType::<Impl, OFFSET>, DeleteResourceType::<Impl, OFFSET>, GetRegisteredFileExtensions::<Impl, OFFSET>, GetResourceTypeInfo::<Impl, OFFSET>, ModifyResourceType::<Impl, OFFSET>)
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IWorkspaceScriptableImpl: Sized + IDispatchImpl {
    fn DisconnectWorkspace();
    fn StartWorkspace();
    fn IsWorkspaceCredentialSpecified();
    fn IsWorkspaceSSOEnabled();
    fn ClearWorkspaceCredential();
    fn OnAuthenticated();
    fn DisconnectWorkspaceByFriendlyName();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for IWorkspaceScriptable {
    const NAME: &'static str = "Windows.Win32.System.RemoteDesktop.IWorkspaceScriptable";
}
#[cfg(feature = "Win32_System_Com")]
impl IWorkspaceScriptableVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWorkspaceScriptableImpl, const OFFSET: isize>() -> IWorkspaceScriptableVtbl {
        unsafe extern "system" fn DisconnectWorkspace<Impl: IWorkspaceScriptableImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrworkspaceid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DisconnectWorkspace(&*(&bstrworkspaceid as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn StartWorkspace<Impl: IWorkspaceScriptableImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrworkspaceid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrusername: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrpassword: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrworkspaceparams: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ltimeout: i32, lflags: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).StartWorkspace(
                &*(&bstrworkspaceid as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&bstrusername as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&bstrpassword as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&bstrworkspaceparams as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType),
                ltimeout,
                lflags,
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsWorkspaceCredentialSpecified<Impl: IWorkspaceScriptableImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrworkspaceid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bcountunauthenticatedcredentials: i16, pbcredexist: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsWorkspaceCredentialSpecified(&*(&bstrworkspaceid as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), bcountunauthenticatedcredentials, ::core::mem::transmute_copy(&pbcredexist)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsWorkspaceSSOEnabled<Impl: IWorkspaceScriptableImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbssoenabled: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsWorkspaceSSOEnabled(::core::mem::transmute_copy(&pbssoenabled)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ClearWorkspaceCredential<Impl: IWorkspaceScriptableImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrworkspaceid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ClearWorkspaceCredential(&*(&bstrworkspaceid as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OnAuthenticated<Impl: IWorkspaceScriptableImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrworkspaceid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrusername: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OnAuthenticated(&*(&bstrworkspaceid as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), &*(&bstrusername as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DisconnectWorkspaceByFriendlyName<Impl: IWorkspaceScriptableImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrworkspacefriendlyname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DisconnectWorkspaceByFriendlyName(&*(&bstrworkspacefriendlyname as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
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
            ::windows::core::GetRuntimeClassName::<IWorkspaceScriptable>,
            ::windows::core::GetTrustLevel,
            DisconnectWorkspace::<Impl, OFFSET>,
            StartWorkspace::<Impl, OFFSET>,
            IsWorkspaceCredentialSpecified::<Impl, OFFSET>,
            IsWorkspaceSSOEnabled::<Impl, OFFSET>,
            ClearWorkspaceCredential::<Impl, OFFSET>,
            OnAuthenticated::<Impl, OFFSET>,
            DisconnectWorkspaceByFriendlyName::<Impl, OFFSET>,
        )
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IWorkspaceScriptable2Impl: Sized + IWorkspaceScriptableImpl + IDispatchImpl {
    fn StartWorkspaceEx();
    fn ResourceDismissed();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for IWorkspaceScriptable2 {
    const NAME: &'static str = "Windows.Win32.System.RemoteDesktop.IWorkspaceScriptable2";
}
#[cfg(feature = "Win32_System_Com")]
impl IWorkspaceScriptable2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWorkspaceScriptable2Impl, const OFFSET: isize>() -> IWorkspaceScriptable2Vtbl {
        unsafe extern "system" fn StartWorkspaceEx<Impl: IWorkspaceScriptable2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrworkspaceid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrworkspacefriendlyname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrredirectorname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrusername: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrpassword: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrappcontainer: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrworkspaceparams: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ltimeout: i32, lflags: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).StartWorkspaceEx(
                &*(&bstrworkspaceid as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&bstrworkspacefriendlyname as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&bstrredirectorname as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&bstrusername as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&bstrpassword as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&bstrappcontainer as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&bstrworkspaceparams as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType),
                ltimeout,
                lflags,
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ResourceDismissed<Impl: IWorkspaceScriptable2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrworkspaceid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrworkspacefriendlyname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ResourceDismissed(&*(&bstrworkspaceid as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), &*(&bstrworkspacefriendlyname as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IWorkspaceScriptable2>, ::windows::core::GetTrustLevel, StartWorkspaceEx::<Impl, OFFSET>, ResourceDismissed::<Impl, OFFSET>)
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IWorkspaceScriptable3Impl: Sized + IWorkspaceScriptable2Impl + IWorkspaceScriptableImpl + IDispatchImpl {
    fn StartWorkspaceEx2();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for IWorkspaceScriptable3 {
    const NAME: &'static str = "Windows.Win32.System.RemoteDesktop.IWorkspaceScriptable3";
}
#[cfg(feature = "Win32_System_Com")]
impl IWorkspaceScriptable3Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWorkspaceScriptable3Impl, const OFFSET: isize>() -> IWorkspaceScriptable3Vtbl {
        unsafe extern "system" fn StartWorkspaceEx2<Impl: IWorkspaceScriptable3Impl, const OFFSET: isize>(
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
            match (*this).StartWorkspaceEx2(
                &*(&bstrworkspaceid as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&bstrworkspacefriendlyname as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&bstrredirectorname as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&bstrusername as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&bstrpassword as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&bstrappcontainer as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&bstrworkspaceparams as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType),
                ltimeout,
                lflags,
                &*(&bstreventloguploadaddress as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&correlationid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IWorkspaceScriptable3>, ::windows::core::GetTrustLevel, StartWorkspaceEx2::<Impl, OFFSET>)
    }
}
pub trait ItsPubPluginImpl: Sized {
    fn GetResourceList();
    fn GetResource();
    fn GetCacheLastUpdateTime();
    fn pluginName();
    fn pluginVersion();
    fn ResolveResource();
}
impl ::windows::core::RuntimeName for ItsPubPlugin {
    const NAME: &'static str = "Windows.Win32.System.RemoteDesktop.ItsPubPlugin";
}
impl ItsPubPluginVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ItsPubPluginImpl, const OFFSET: isize>() -> ItsPubPluginVtbl {
        unsafe extern "system" fn GetResourceList<Impl: ItsPubPluginImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, userid: super::super::Foundation::PWSTR, pceapplistsize: *mut i32, resourcelist: *mut *mut pluginResource) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetResourceList(&*(&userid as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&pceapplistsize), ::core::mem::transmute_copy(&resourcelist)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetResource<Impl: ItsPubPluginImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, alias: super::super::Foundation::PWSTR, flags: i32, resource: *mut pluginResource) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetResource(&*(&alias as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), flags, ::core::mem::transmute_copy(&resource)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCacheLastUpdateTime<Impl: ItsPubPluginImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lastupdatetime: *mut u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetCacheLastUpdateTime(::core::mem::transmute_copy(&lastupdatetime)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn pluginName<Impl: ItsPubPluginImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).pluginName(::core::mem::transmute_copy(&pval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn pluginVersion<Impl: ItsPubPluginImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).pluginVersion(::core::mem::transmute_copy(&pval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ResolveResource<Impl: ItsPubPluginImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, resourcetype: *mut u32, resourcelocation: super::super::Foundation::PWSTR, endpointname: super::super::Foundation::PWSTR, userid: super::super::Foundation::PWSTR, alias: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ResolveResource(
                ::core::mem::transmute_copy(&resourcetype),
                ::core::mem::transmute_copy(&resourcelocation),
                ::core::mem::transmute_copy(&endpointname),
                &*(&userid as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&alias as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ItsPubPlugin>, ::windows::core::GetTrustLevel, GetResourceList::<Impl, OFFSET>, GetResource::<Impl, OFFSET>, GetCacheLastUpdateTime::<Impl, OFFSET>, pluginName::<Impl, OFFSET>, pluginVersion::<Impl, OFFSET>, ResolveResource::<Impl, OFFSET>)
    }
}
pub trait ItsPubPlugin2Impl: Sized + ItsPubPluginImpl {
    fn GetResource2List();
    fn GetResource2();
    fn ResolvePersonalDesktop();
    fn DeletePersonalDesktopAssignment();
}
impl ::windows::core::RuntimeName for ItsPubPlugin2 {
    const NAME: &'static str = "Windows.Win32.System.RemoteDesktop.ItsPubPlugin2";
}
impl ItsPubPlugin2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ItsPubPlugin2Impl, const OFFSET: isize>() -> ItsPubPlugin2Vtbl {
        unsafe extern "system" fn GetResource2List<Impl: ItsPubPlugin2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, userid: super::super::Foundation::PWSTR, pceapplistsize: *mut i32, resourcelist: *mut *mut pluginResource2) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetResource2List(&*(&userid as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&pceapplistsize), ::core::mem::transmute_copy(&resourcelist)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetResource2<Impl: ItsPubPlugin2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, alias: super::super::Foundation::PWSTR, flags: i32, resource: *mut pluginResource2) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetResource2(&*(&alias as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), flags, ::core::mem::transmute_copy(&resource)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ResolvePersonalDesktop<Impl: ItsPubPlugin2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, userid: super::super::Foundation::PWSTR, poolid: super::super::Foundation::PWSTR, epdresolutiontype: TSPUB_PLUGIN_PD_RESOLUTION_TYPE, ppdassignmenttype: *mut TSPUB_PLUGIN_PD_ASSIGNMENT_TYPE, endpointname: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ResolvePersonalDesktop(&*(&userid as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), &*(&poolid as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), epdresolutiontype, ::core::mem::transmute_copy(&ppdassignmenttype), ::core::mem::transmute_copy(&endpointname)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DeletePersonalDesktopAssignment<Impl: ItsPubPlugin2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, userid: super::super::Foundation::PWSTR, poolid: super::super::Foundation::PWSTR, endpointname: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DeletePersonalDesktopAssignment(
                &*(&userid as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&poolid as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&endpointname as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ItsPubPlugin2>, ::windows::core::GetTrustLevel, GetResource2List::<Impl, OFFSET>, GetResource2::<Impl, OFFSET>, ResolvePersonalDesktop::<Impl, OFFSET>, DeletePersonalDesktopAssignment::<Impl, OFFSET>)
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait _ITSWkspEventsImpl: Sized + IDispatchImpl {}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for _ITSWkspEvents {
    const NAME: &'static str = "Windows.Win32.System.RemoteDesktop._ITSWkspEvents";
}
#[cfg(feature = "Win32_System_Com")]
impl _ITSWkspEventsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: _ITSWkspEventsImpl, const OFFSET: isize>() -> _ITSWkspEventsVtbl {
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<_ITSWkspEvents>, ::windows::core::GetTrustLevel)
    }
}
