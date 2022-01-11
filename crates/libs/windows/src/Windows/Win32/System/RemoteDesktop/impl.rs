#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
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
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IADsTSUserExVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IADsTSUserExImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IADsTSUserExVtbl {
        unsafe extern "system" fn TerminalServicesProfilePath<Impl: IADsTSUserExImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetTerminalServicesProfilePath<Impl: IADsTSUserExImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pnewval: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn TerminalServicesHomeDirectory<Impl: IADsTSUserExImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetTerminalServicesHomeDirectory<Impl: IADsTSUserExImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pnewval: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn TerminalServicesHomeDrive<Impl: IADsTSUserExImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetTerminalServicesHomeDrive<Impl: IADsTSUserExImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pnewval: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn AllowLogon<Impl: IADsTSUserExImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetAllowLogon<Impl: IADsTSUserExImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newval: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn EnableRemoteControl<Impl: IADsTSUserExImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetEnableRemoteControl<Impl: IADsTSUserExImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newval: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn MaxDisconnectionTime<Impl: IADsTSUserExImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetMaxDisconnectionTime<Impl: IADsTSUserExImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newval: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn MaxConnectionTime<Impl: IADsTSUserExImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetMaxConnectionTime<Impl: IADsTSUserExImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newval: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn MaxIdleTime<Impl: IADsTSUserExImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetMaxIdleTime<Impl: IADsTSUserExImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newval: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ReconnectionAction<Impl: IADsTSUserExImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pnewval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetReconnectionAction<Impl: IADsTSUserExImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newval: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn BrokenConnectionAction<Impl: IADsTSUserExImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pnewval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetBrokenConnectionAction<Impl: IADsTSUserExImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newval: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ConnectClientDrivesAtLogon<Impl: IADsTSUserExImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pnewval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetConnectClientDrivesAtLogon<Impl: IADsTSUserExImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newval: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ConnectClientPrintersAtLogon<Impl: IADsTSUserExImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetConnectClientPrintersAtLogon<Impl: IADsTSUserExImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newval: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn DefaultToMainPrinter<Impl: IADsTSUserExImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetDefaultToMainPrinter<Impl: IADsTSUserExImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newval: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn TerminalServicesWorkDirectory<Impl: IADsTSUserExImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetTerminalServicesWorkDirectory<Impl: IADsTSUserExImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pnewval: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn TerminalServicesInitialProgram<Impl: IADsTSUserExImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetTerminalServicesInitialProgram<Impl: IADsTSUserExImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pnewval: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            GetTypeInfoCount::<Impl, IMPL_OFFSET>,
            GetTypeInfo::<Impl, IMPL_OFFSET>,
            GetIDsOfNames::<Impl, IMPL_OFFSET>,
            Invoke::<Impl, IMPL_OFFSET>,
            TerminalServicesProfilePath::<Impl, IMPL_OFFSET>,
            SetTerminalServicesProfilePath::<Impl, IMPL_OFFSET>,
            TerminalServicesHomeDirectory::<Impl, IMPL_OFFSET>,
            SetTerminalServicesHomeDirectory::<Impl, IMPL_OFFSET>,
            TerminalServicesHomeDrive::<Impl, IMPL_OFFSET>,
            SetTerminalServicesHomeDrive::<Impl, IMPL_OFFSET>,
            AllowLogon::<Impl, IMPL_OFFSET>,
            SetAllowLogon::<Impl, IMPL_OFFSET>,
            EnableRemoteControl::<Impl, IMPL_OFFSET>,
            SetEnableRemoteControl::<Impl, IMPL_OFFSET>,
            MaxDisconnectionTime::<Impl, IMPL_OFFSET>,
            SetMaxDisconnectionTime::<Impl, IMPL_OFFSET>,
            MaxConnectionTime::<Impl, IMPL_OFFSET>,
            SetMaxConnectionTime::<Impl, IMPL_OFFSET>,
            MaxIdleTime::<Impl, IMPL_OFFSET>,
            SetMaxIdleTime::<Impl, IMPL_OFFSET>,
            ReconnectionAction::<Impl, IMPL_OFFSET>,
            SetReconnectionAction::<Impl, IMPL_OFFSET>,
            BrokenConnectionAction::<Impl, IMPL_OFFSET>,
            SetBrokenConnectionAction::<Impl, IMPL_OFFSET>,
            ConnectClientDrivesAtLogon::<Impl, IMPL_OFFSET>,
            SetConnectClientDrivesAtLogon::<Impl, IMPL_OFFSET>,
            ConnectClientPrintersAtLogon::<Impl, IMPL_OFFSET>,
            SetConnectClientPrintersAtLogon::<Impl, IMPL_OFFSET>,
            DefaultToMainPrinter::<Impl, IMPL_OFFSET>,
            SetDefaultToMainPrinter::<Impl, IMPL_OFFSET>,
            TerminalServicesWorkDirectory::<Impl, IMPL_OFFSET>,
            SetTerminalServicesWorkDirectory::<Impl, IMPL_OFFSET>,
            TerminalServicesInitialProgram::<Impl, IMPL_OFFSET>,
            SetTerminalServicesInitialProgram::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IADsTSUserEx as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IAudioDeviceEndpointImpl: Sized {
    fn SetBuffer();
    fn GetRTCaps();
    fn GetEventDrivenCapable();
    fn WriteExclusiveModeParametersToSharedMemory();
}
#[cfg(feature = "Win32_Foundation")]
impl IAudioDeviceEndpointVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAudioDeviceEndpointImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAudioDeviceEndpointVtbl {
        unsafe extern "system" fn SetBuffer<Impl: IAudioDeviceEndpointImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, maxperiod: i64, u32latencycoefficient: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetRTCaps<Impl: IAudioDeviceEndpointImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbisrtcapable: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetEventDrivenCapable<Impl: IAudioDeviceEndpointImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbiseventcapable: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn WriteExclusiveModeParametersToSharedMemory<Impl: IAudioDeviceEndpointImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, htargetprocess: usize, hnsperiod: i64, hnsbufferduration: i64, u32latencycoefficient: u32, pu32sharedmemorysize: *mut u32, phsharedmemory: *mut usize) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, SetBuffer::<Impl, IMPL_OFFSET>, GetRTCaps::<Impl, IMPL_OFFSET>, GetEventDrivenCapable::<Impl, IMPL_OFFSET>, WriteExclusiveModeParametersToSharedMemory::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAudioDeviceEndpoint as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Media_Audio"))]
pub trait IAudioEndpointImpl: Sized {
    fn GetFrameFormat();
    fn GetFramesPerPacket();
    fn GetLatency();
    fn SetStreamFlags();
    fn SetEventHandle();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Media_Audio"))]
impl IAudioEndpointVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAudioEndpointImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAudioEndpointVtbl {
        unsafe extern "system" fn GetFrameFormat<Impl: IAudioEndpointImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppformat: *mut *mut super::super::Media::Audio::WAVEFORMATEX) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetFramesPerPacket<Impl: IAudioEndpointImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pframesperpacket: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetLatency<Impl: IAudioEndpointImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, platency: *mut i64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetStreamFlags<Impl: IAudioEndpointImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, streamflags: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetEventHandle<Impl: IAudioEndpointImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, eventhandle: super::super::Foundation::HANDLE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetFrameFormat::<Impl, IMPL_OFFSET>, GetFramesPerPacket::<Impl, IMPL_OFFSET>, GetLatency::<Impl, IMPL_OFFSET>, SetStreamFlags::<Impl, IMPL_OFFSET>, SetEventHandle::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAudioEndpoint as ::windows::core::Interface>::IID
    }
}
pub trait IAudioEndpointControlImpl: Sized {
    fn Start();
    fn Reset();
    fn Stop();
}
impl IAudioEndpointControlVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAudioEndpointControlImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAudioEndpointControlVtbl {
        unsafe extern "system" fn Start<Impl: IAudioEndpointControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Reset<Impl: IAudioEndpointControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Stop<Impl: IAudioEndpointControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, Start::<Impl, IMPL_OFFSET>, Reset::<Impl, IMPL_OFFSET>, Stop::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAudioEndpointControl as ::windows::core::Interface>::IID
    }
}
pub trait IAudioEndpointRTImpl: Sized {
    fn GetCurrentPadding();
    fn ProcessingComplete();
    fn SetPinInactive();
    fn SetPinActive();
}
impl IAudioEndpointRTVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAudioEndpointRTImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAudioEndpointRTVtbl {
        unsafe extern "system" fn GetCurrentPadding<Impl: IAudioEndpointRTImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppadding: *mut i64, paecurrentposition: *mut AE_CURRENT_POSITION) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ProcessingComplete<Impl: IAudioEndpointRTImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetPinInactive<Impl: IAudioEndpointRTImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetPinActive<Impl: IAudioEndpointRTImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetCurrentPadding::<Impl, IMPL_OFFSET>, ProcessingComplete::<Impl, IMPL_OFFSET>, SetPinInactive::<Impl, IMPL_OFFSET>, SetPinActive::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAudioEndpointRT as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Media_Audio_Apo")]
pub trait IAudioInputEndpointRTImpl: Sized {
    fn GetInputDataPointer();
    fn ReleaseInputDataPointer();
    fn PulseEndpoint();
}
#[cfg(feature = "Win32_Media_Audio_Apo")]
impl IAudioInputEndpointRTVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAudioInputEndpointRTImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAudioInputEndpointRTVtbl {
        unsafe extern "system" fn GetInputDataPointer<Impl: IAudioInputEndpointRTImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pconnectionproperty: *mut super::super::Media::Audio::Apo::APO_CONNECTION_PROPERTY, paetimestamp: *mut AE_CURRENT_POSITION) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ReleaseInputDataPointer<Impl: IAudioInputEndpointRTImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, u32framecount: u32, pdatapointer: usize) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn PulseEndpoint<Impl: IAudioInputEndpointRTImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetInputDataPointer::<Impl, IMPL_OFFSET>, ReleaseInputDataPointer::<Impl, IMPL_OFFSET>, PulseEndpoint::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAudioInputEndpointRT as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Media_Audio_Apo")]
pub trait IAudioOutputEndpointRTImpl: Sized {
    fn GetOutputDataPointer();
    fn ReleaseOutputDataPointer();
    fn PulseEndpoint();
}
#[cfg(feature = "Win32_Media_Audio_Apo")]
impl IAudioOutputEndpointRTVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAudioOutputEndpointRTImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAudioOutputEndpointRTVtbl {
        unsafe extern "system" fn GetOutputDataPointer<Impl: IAudioOutputEndpointRTImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, u32framecount: u32, paetimestamp: *const AE_CURRENT_POSITION) -> usize {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ReleaseOutputDataPointer<Impl: IAudioOutputEndpointRTImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pconnectionproperty: *const super::super::Media::Audio::Apo::APO_CONNECTION_PROPERTY) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn PulseEndpoint<Impl: IAudioOutputEndpointRTImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetOutputDataPointer::<Impl, IMPL_OFFSET>, ReleaseOutputDataPointer::<Impl, IMPL_OFFSET>, PulseEndpoint::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAudioOutputEndpointRT as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
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
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IRemoteDesktopClientVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRemoteDesktopClientImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IRemoteDesktopClientVtbl {
        unsafe extern "system" fn Connect<Impl: IRemoteDesktopClientImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Disconnect<Impl: IRemoteDesktopClientImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Reconnect<Impl: IRemoteDesktopClientImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, width: u32, height: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Settings<Impl: IRemoteDesktopClientImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, settings: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Actions<Impl: IRemoteDesktopClientImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, actions: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn TouchPointer<Impl: IRemoteDesktopClientImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, touchpointer: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn DeleteSavedCredentials<Impl: IRemoteDesktopClientImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, servername: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn UpdateSessionDisplaySettings<Impl: IRemoteDesktopClientImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, width: u32, height: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn attachEvent<Impl: IRemoteDesktopClientImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, eventname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, callback: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn detachEvent<Impl: IRemoteDesktopClientImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, eventname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, callback: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            GetTypeInfoCount::<Impl, IMPL_OFFSET>,
            GetTypeInfo::<Impl, IMPL_OFFSET>,
            GetIDsOfNames::<Impl, IMPL_OFFSET>,
            Invoke::<Impl, IMPL_OFFSET>,
            Connect::<Impl, IMPL_OFFSET>,
            Disconnect::<Impl, IMPL_OFFSET>,
            Reconnect::<Impl, IMPL_OFFSET>,
            Settings::<Impl, IMPL_OFFSET>,
            Actions::<Impl, IMPL_OFFSET>,
            TouchPointer::<Impl, IMPL_OFFSET>,
            DeleteSavedCredentials::<Impl, IMPL_OFFSET>,
            UpdateSessionDisplaySettings::<Impl, IMPL_OFFSET>,
            attachEvent::<Impl, IMPL_OFFSET>,
            detachEvent::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRemoteDesktopClient as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IRemoteDesktopClientActionsImpl: Sized + IDispatchImpl {
    fn SuspendScreenUpdates();
    fn ResumeScreenUpdates();
    fn ExecuteRemoteAction();
    fn GetSnapshot();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IRemoteDesktopClientActionsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRemoteDesktopClientActionsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IRemoteDesktopClientActionsVtbl {
        unsafe extern "system" fn SuspendScreenUpdates<Impl: IRemoteDesktopClientActionsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ResumeScreenUpdates<Impl: IRemoteDesktopClientActionsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ExecuteRemoteAction<Impl: IRemoteDesktopClientActionsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, remoteaction: RemoteActionType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetSnapshot<Impl: IRemoteDesktopClientActionsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, snapshotencoding: SnapshotEncodingType, snapshotformat: SnapshotFormatType, snapshotwidth: u32, snapshotheight: u32, snapshotdata: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetTypeInfoCount::<Impl, IMPL_OFFSET>, GetTypeInfo::<Impl, IMPL_OFFSET>, GetIDsOfNames::<Impl, IMPL_OFFSET>, Invoke::<Impl, IMPL_OFFSET>, SuspendScreenUpdates::<Impl, IMPL_OFFSET>, ResumeScreenUpdates::<Impl, IMPL_OFFSET>, ExecuteRemoteAction::<Impl, IMPL_OFFSET>, GetSnapshot::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRemoteDesktopClientActions as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IRemoteDesktopClientSettingsImpl: Sized + IDispatchImpl {
    fn ApplySettings();
    fn RetrieveSettings();
    fn GetRdpProperty();
    fn SetRdpProperty();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IRemoteDesktopClientSettingsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRemoteDesktopClientSettingsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IRemoteDesktopClientSettingsVtbl {
        unsafe extern "system" fn ApplySettings<Impl: IRemoteDesktopClientSettingsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rdpfilecontents: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn RetrieveSettings<Impl: IRemoteDesktopClientSettingsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rdpfilecontents: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetRdpProperty<Impl: IRemoteDesktopClientSettingsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, propertyname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, value: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetRdpProperty<Impl: IRemoteDesktopClientSettingsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, propertyname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, value: ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetTypeInfoCount::<Impl, IMPL_OFFSET>, GetTypeInfo::<Impl, IMPL_OFFSET>, GetIDsOfNames::<Impl, IMPL_OFFSET>, Invoke::<Impl, IMPL_OFFSET>, ApplySettings::<Impl, IMPL_OFFSET>, RetrieveSettings::<Impl, IMPL_OFFSET>, GetRdpProperty::<Impl, IMPL_OFFSET>, SetRdpProperty::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRemoteDesktopClientSettings as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IRemoteDesktopClientTouchPointerImpl: Sized + IDispatchImpl {
    fn SetEnabled();
    fn Enabled();
    fn SetEventsEnabled();
    fn EventsEnabled();
    fn SetPointerSpeed();
    fn PointerSpeed();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IRemoteDesktopClientTouchPointerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRemoteDesktopClientTouchPointerImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IRemoteDesktopClientTouchPointerVtbl {
        unsafe extern "system" fn SetEnabled<Impl: IRemoteDesktopClientTouchPointerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, enabled: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Enabled<Impl: IRemoteDesktopClientTouchPointerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, enabled: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetEventsEnabled<Impl: IRemoteDesktopClientTouchPointerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, eventsenabled: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn EventsEnabled<Impl: IRemoteDesktopClientTouchPointerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, eventsenabled: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetPointerSpeed<Impl: IRemoteDesktopClientTouchPointerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pointerspeed: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn PointerSpeed<Impl: IRemoteDesktopClientTouchPointerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pointerspeed: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            GetTypeInfoCount::<Impl, IMPL_OFFSET>,
            GetTypeInfo::<Impl, IMPL_OFFSET>,
            GetIDsOfNames::<Impl, IMPL_OFFSET>,
            Invoke::<Impl, IMPL_OFFSET>,
            SetEnabled::<Impl, IMPL_OFFSET>,
            Enabled::<Impl, IMPL_OFFSET>,
            SetEventsEnabled::<Impl, IMPL_OFFSET>,
            EventsEnabled::<Impl, IMPL_OFFSET>,
            SetPointerSpeed::<Impl, IMPL_OFFSET>,
            PointerSpeed::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRemoteDesktopClientTouchPointer as ::windows::core::Interface>::IID
    }
}
pub trait IRemoteSystemAdditionalInfoProviderImpl: Sized {
    fn GetAdditionalInfo();
}
impl IRemoteSystemAdditionalInfoProviderVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRemoteSystemAdditionalInfoProviderImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IRemoteSystemAdditionalInfoProviderVtbl {
        unsafe extern "system" fn GetAdditionalInfo<Impl: IRemoteSystemAdditionalInfoProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, deduplicationid: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>, riid: *const ::windows::core::GUID, mapview: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetAdditionalInfo::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRemoteSystemAdditionalInfoProvider as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait ITSGAccountingEngineImpl: Sized {
    fn DoAccounting();
}
#[cfg(feature = "Win32_Foundation")]
impl ITSGAccountingEngineVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITSGAccountingEngineImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITSGAccountingEngineVtbl {
        unsafe extern "system" fn DoAccounting<Impl: ITSGAccountingEngineImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, accountingdatatype: AAAccountingDataType, accountingdata: ::core::mem::ManuallyDrop<AAAccountingData>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, DoAccounting::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITSGAccountingEngine as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait ITSGAuthenticateUserSinkImpl: Sized {
    fn OnUserAuthenticated();
    fn OnUserAuthenticationFailed();
    fn ReauthenticateUser();
    fn DisconnectUser();
}
#[cfg(feature = "Win32_Foundation")]
impl ITSGAuthenticateUserSinkVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITSGAuthenticateUserSinkImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITSGAuthenticateUserSinkVtbl {
        unsafe extern "system" fn OnUserAuthenticated<Impl: ITSGAuthenticateUserSinkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, username: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, userdomain: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, context: usize, usertoken: super::super::Foundation::HANDLE_PTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn OnUserAuthenticationFailed<Impl: ITSGAuthenticateUserSinkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, context: usize, genericerrorcode: ::windows::core::HRESULT, specificerrorcode: ::windows::core::HRESULT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ReauthenticateUser<Impl: ITSGAuthenticateUserSinkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, context: usize) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn DisconnectUser<Impl: ITSGAuthenticateUserSinkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, context: usize) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, OnUserAuthenticated::<Impl, IMPL_OFFSET>, OnUserAuthenticationFailed::<Impl, IMPL_OFFSET>, ReauthenticateUser::<Impl, IMPL_OFFSET>, DisconnectUser::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITSGAuthenticateUserSink as ::windows::core::Interface>::IID
    }
}
pub trait ITSGAuthenticationEngineImpl: Sized {
    fn AuthenticateUser();
    fn CancelAuthentication();
}
impl ITSGAuthenticationEngineVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITSGAuthenticationEngineImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITSGAuthenticationEngineVtbl {
        unsafe extern "system" fn AuthenticateUser<Impl: ITSGAuthenticationEngineImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, mainsessionid: ::windows::core::GUID, cookiedata: *const u8, numcookiebytes: u32, context: usize, psink: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CancelAuthentication<Impl: ITSGAuthenticationEngineImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, mainsessionid: ::windows::core::GUID, context: usize) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, AuthenticateUser::<Impl, IMPL_OFFSET>, CancelAuthentication::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITSGAuthenticationEngine as ::windows::core::Interface>::IID
    }
}
pub trait ITSGAuthorizeConnectionSinkImpl: Sized {
    fn OnConnectionAuthorized();
}
impl ITSGAuthorizeConnectionSinkVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITSGAuthorizeConnectionSinkImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITSGAuthorizeConnectionSinkVtbl {
        unsafe extern "system" fn OnConnectionAuthorized<Impl: ITSGAuthorizeConnectionSinkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hrin: ::windows::core::HRESULT, mainsessionid: ::windows::core::GUID, cbsohresponse: u32, pbsohresponse: *const u8, idletimeout: u32, sessiontimeout: u32, sessiontimeoutaction: SESSION_TIMEOUT_ACTION_TYPE, trustclass: AATrustClassID, policyattributes: *const u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, OnConnectionAuthorized::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITSGAuthorizeConnectionSink as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait ITSGAuthorizeResourceSinkImpl: Sized {
    fn OnChannelAuthorized();
}
#[cfg(feature = "Win32_Foundation")]
impl ITSGAuthorizeResourceSinkVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITSGAuthorizeResourceSinkImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITSGAuthorizeResourceSinkVtbl {
        unsafe extern "system" fn OnChannelAuthorized<Impl: ITSGAuthorizeResourceSinkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hrin: ::windows::core::HRESULT, mainsessionid: ::windows::core::GUID, subsessionid: i32, allowedresourcenames: *const super::super::Foundation::BSTR, numallowedresourcenames: u32, failedresourcenames: *const super::super::Foundation::BSTR, numfailedresourcenames: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, OnChannelAuthorized::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITSGAuthorizeResourceSink as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait ITSGPolicyEngineImpl: Sized {
    fn AuthorizeConnection();
    fn AuthorizeResource();
    fn Refresh();
    fn IsQuarantineEnabled();
}
#[cfg(feature = "Win32_Foundation")]
impl ITSGPolicyEngineVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITSGPolicyEngineImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITSGPolicyEngineVtbl {
        unsafe extern "system" fn AuthorizeConnection<Impl: ITSGPolicyEngineImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, mainsessionid: ::windows::core::GUID, username: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, authtype: AAAuthSchemes, clientmachineip: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, clientmachinename: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, sohdata: *const u8, numsohbytes: u32, cookiedata: *const u8, numcookiebytes: u32, usertoken: super::super::Foundation::HANDLE_PTR, psink: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn AuthorizeResource<Impl: ITSGPolicyEngineImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, mainsessionid: ::windows::core::GUID, subsessionid: i32, username: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, resourcenames: *const super::super::Foundation::BSTR, numresources: u32, alternateresourcenames: *const super::super::Foundation::BSTR, numalternateresourcename: u32, portnumber: u32, operation: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, cookie: *const u8, numbytesincookie: u32, psink: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Refresh<Impl: ITSGPolicyEngineImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn IsQuarantineEnabled<Impl: ITSGPolicyEngineImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, quarantineenabled: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, AuthorizeConnection::<Impl, IMPL_OFFSET>, AuthorizeResource::<Impl, IMPL_OFFSET>, Refresh::<Impl, IMPL_OFFSET>, IsQuarantineEnabled::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITSGPolicyEngine as ::windows::core::Interface>::IID
    }
}
pub trait ITsSbBaseNotifySinkImpl: Sized {
    fn OnError();
    fn OnReportStatus();
}
impl ITsSbBaseNotifySinkVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITsSbBaseNotifySinkImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITsSbBaseNotifySinkVtbl {
        unsafe extern "system" fn OnError<Impl: ITsSbBaseNotifySinkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hrerror: ::windows::core::HRESULT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn OnReportStatus<Impl: ITsSbBaseNotifySinkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, messagetype: CLIENT_MESSAGE_TYPE, messageid: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, OnError::<Impl, IMPL_OFFSET>, OnReportStatus::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITsSbBaseNotifySink as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
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
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ITsSbClientConnectionVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITsSbClientConnectionImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITsSbClientConnectionVtbl {
        unsafe extern "system" fn UserName<Impl: ITsSbClientConnectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Domain<Impl: ITsSbClientConnectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn InitialProgram<Impl: ITsSbClientConnectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn LoadBalanceResult<Impl: ITsSbClientConnectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn FarmName<Impl: ITsSbClientConnectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn PutContext<Impl: ITsSbClientConnectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, contextid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, context: ::core::mem::ManuallyDrop<super::Com::VARIANT>, existingcontext: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetContext<Impl: ITsSbClientConnectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, contextid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, context: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Environment<Impl: ITsSbClientConnectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenvironment: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ConnectionError<Impl: ITsSbClientConnectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SamUserAccount<Impl: ITsSbClientConnectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ClientConnectionPropertySet<Impl: ITsSbClientConnectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pppropertyset: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn IsFirstAssignment<Impl: ITsSbClientConnectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppval: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn RdFarmType<Impl: ITsSbClientConnectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, prdfarmtype: *mut RD_FARM_TYPE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn UserSidString<Impl: ITsSbClientConnectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszusersidstring: *mut *mut i8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetDisconnectedSession<Impl: ITsSbClientConnectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppsession: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            UserName::<Impl, IMPL_OFFSET>,
            Domain::<Impl, IMPL_OFFSET>,
            InitialProgram::<Impl, IMPL_OFFSET>,
            LoadBalanceResult::<Impl, IMPL_OFFSET>,
            FarmName::<Impl, IMPL_OFFSET>,
            PutContext::<Impl, IMPL_OFFSET>,
            GetContext::<Impl, IMPL_OFFSET>,
            Environment::<Impl, IMPL_OFFSET>,
            ConnectionError::<Impl, IMPL_OFFSET>,
            SamUserAccount::<Impl, IMPL_OFFSET>,
            ClientConnectionPropertySet::<Impl, IMPL_OFFSET>,
            IsFirstAssignment::<Impl, IMPL_OFFSET>,
            RdFarmType::<Impl, IMPL_OFFSET>,
            UserSidString::<Impl, IMPL_OFFSET>,
            GetDisconnectedSession::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITsSbClientConnection as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Ole"))]
pub trait ITsSbClientConnectionPropertySetImpl: Sized + ITsSbPropertySetImpl + IPropertyBagImpl {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Ole"))]
impl ITsSbClientConnectionPropertySetVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITsSbClientConnectionPropertySetImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITsSbClientConnectionPropertySetVtbl {
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, Read::<Impl, IMPL_OFFSET>, Write::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITsSbClientConnectionPropertySet as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait ITsSbEnvironmentImpl: Sized {
    fn Name();
    fn ServerWeight();
    fn EnvironmentPropertySet();
    fn SetEnvironmentPropertySet();
}
#[cfg(feature = "Win32_Foundation")]
impl ITsSbEnvironmentVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITsSbEnvironmentImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITsSbEnvironmentVtbl {
        unsafe extern "system" fn Name<Impl: ITsSbEnvironmentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ServerWeight<Impl: ITsSbEnvironmentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn EnvironmentPropertySet<Impl: ITsSbEnvironmentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pppropertyset: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetEnvironmentPropertySet<Impl: ITsSbEnvironmentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, Name::<Impl, IMPL_OFFSET>, ServerWeight::<Impl, IMPL_OFFSET>, EnvironmentPropertySet::<Impl, IMPL_OFFSET>, SetEnvironmentPropertySet::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITsSbEnvironment as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Ole"))]
pub trait ITsSbEnvironmentPropertySetImpl: Sized + ITsSbPropertySetImpl + IPropertyBagImpl {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Ole"))]
impl ITsSbEnvironmentPropertySetVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITsSbEnvironmentPropertySetImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITsSbEnvironmentPropertySetVtbl {
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, Read::<Impl, IMPL_OFFSET>, Write::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITsSbEnvironmentPropertySet as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait ITsSbFilterPluginStoreImpl: Sized {
    fn SaveProperties();
    fn EnumerateProperties();
    fn DeleteProperties();
}
#[cfg(feature = "Win32_Foundation")]
impl ITsSbFilterPluginStoreVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITsSbFilterPluginStoreImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITsSbFilterPluginStoreVtbl {
        unsafe extern "system" fn SaveProperties<Impl: ITsSbFilterPluginStoreImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppropertyset: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn EnumerateProperties<Impl: ITsSbFilterPluginStoreImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pppropertyset: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn DeleteProperties<Impl: ITsSbFilterPluginStoreImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, propertyname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, SaveProperties::<Impl, IMPL_OFFSET>, EnumerateProperties::<Impl, IMPL_OFFSET>, DeleteProperties::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITsSbFilterPluginStore as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait ITsSbGenericNotifySinkImpl: Sized {
    fn OnCompleted();
    fn GetWaitTimeout();
}
#[cfg(feature = "Win32_Foundation")]
impl ITsSbGenericNotifySinkVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITsSbGenericNotifySinkImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITsSbGenericNotifySinkVtbl {
        unsafe extern "system" fn OnCompleted<Impl: ITsSbGenericNotifySinkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, status: ::windows::core::HRESULT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetWaitTimeout<Impl: ITsSbGenericNotifySinkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfttimeout: *mut super::super::Foundation::FILETIME) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, OnCompleted::<Impl, IMPL_OFFSET>, GetWaitTimeout::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITsSbGenericNotifySink as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ITsSbGlobalStoreImpl: Sized {
    fn QueryTarget();
    fn QuerySessionBySessionId();
    fn EnumerateFarms();
    fn EnumerateTargets();
    fn EnumerateEnvironmentsByProvider();
    fn EnumerateSessions();
    fn GetFarmProperty();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ITsSbGlobalStoreVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITsSbGlobalStoreImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITsSbGlobalStoreVtbl {
        unsafe extern "system" fn QueryTarget<Impl: ITsSbGlobalStoreImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, providername: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, targetname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, farmname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pptarget: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn QuerySessionBySessionId<Impl: ITsSbGlobalStoreImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, providername: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, dwsessionid: u32, targetname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ppsession: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn EnumerateFarms<Impl: ITsSbGlobalStoreImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, providername: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pdwcount: *mut u32, pval: *mut *mut super::Com::SAFEARRAY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn EnumerateTargets<Impl: ITsSbGlobalStoreImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, providername: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, farmname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, envname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pdwcount: *mut u32, pval: *mut *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn EnumerateEnvironmentsByProvider<Impl: ITsSbGlobalStoreImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, providername: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pdwcount: *mut u32, ppval: *mut *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn EnumerateSessions<Impl: ITsSbGlobalStoreImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, providername: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, targetname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, username: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, userdomain: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, poolname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, initialprogram: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, psessionstate: *const TSSESSION_STATE, pdwcount: *mut u32, ppval: *mut *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetFarmProperty<Impl: ITsSbGlobalStoreImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, farmname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, propertyname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pvarvalue: *const super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, QueryTarget::<Impl, IMPL_OFFSET>, QuerySessionBySessionId::<Impl, IMPL_OFFSET>, EnumerateFarms::<Impl, IMPL_OFFSET>, EnumerateTargets::<Impl, IMPL_OFFSET>, EnumerateEnvironmentsByProvider::<Impl, IMPL_OFFSET>, EnumerateSessions::<Impl, IMPL_OFFSET>, GetFarmProperty::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITsSbGlobalStore as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait ITsSbLoadBalanceResultImpl: Sized {
    fn TargetName();
}
#[cfg(feature = "Win32_Foundation")]
impl ITsSbLoadBalanceResultVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITsSbLoadBalanceResultImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITsSbLoadBalanceResultVtbl {
        unsafe extern "system" fn TargetName<Impl: ITsSbLoadBalanceResultImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, TargetName::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITsSbLoadBalanceResult as ::windows::core::Interface>::IID
    }
}
pub trait ITsSbLoadBalancingImpl: Sized + ITsSbPluginImpl {
    fn GetMostSuitableTarget();
}
impl ITsSbLoadBalancingVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITsSbLoadBalancingImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITsSbLoadBalancingVtbl {
        unsafe extern "system" fn GetMostSuitableTarget<Impl: ITsSbLoadBalancingImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pconnection: ::windows::core::RawPtr, plbsink: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, Initialize::<Impl, IMPL_OFFSET>, Terminate::<Impl, IMPL_OFFSET>, GetMostSuitableTarget::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITsSbLoadBalancing as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait ITsSbLoadBalancingNotifySinkImpl: Sized + ITsSbBaseNotifySinkImpl {
    fn OnGetMostSuitableTarget();
}
#[cfg(feature = "Win32_Foundation")]
impl ITsSbLoadBalancingNotifySinkVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITsSbLoadBalancingNotifySinkImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITsSbLoadBalancingNotifySinkVtbl {
        unsafe extern "system" fn OnGetMostSuitableTarget<Impl: ITsSbLoadBalancingNotifySinkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plbresult: ::windows::core::RawPtr, fisnewconnection: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, OnError::<Impl, IMPL_OFFSET>, OnReportStatus::<Impl, IMPL_OFFSET>, OnGetMostSuitableTarget::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITsSbLoadBalancingNotifySink as ::windows::core::Interface>::IID
    }
}
pub trait ITsSbOrchestrationImpl: Sized + ITsSbPluginImpl {
    fn PrepareTargetForConnect();
}
impl ITsSbOrchestrationVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITsSbOrchestrationImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITsSbOrchestrationVtbl {
        unsafe extern "system" fn PrepareTargetForConnect<Impl: ITsSbOrchestrationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pconnection: ::windows::core::RawPtr, porchestrationnotifysink: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, Initialize::<Impl, IMPL_OFFSET>, Terminate::<Impl, IMPL_OFFSET>, PrepareTargetForConnect::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITsSbOrchestration as ::windows::core::Interface>::IID
    }
}
pub trait ITsSbOrchestrationNotifySinkImpl: Sized + ITsSbBaseNotifySinkImpl {
    fn OnReadyToConnect();
}
impl ITsSbOrchestrationNotifySinkVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITsSbOrchestrationNotifySinkImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITsSbOrchestrationNotifySinkVtbl {
        unsafe extern "system" fn OnReadyToConnect<Impl: ITsSbOrchestrationNotifySinkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ptarget: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, OnError::<Impl, IMPL_OFFSET>, OnReportStatus::<Impl, IMPL_OFFSET>, OnReadyToConnect::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITsSbOrchestrationNotifySink as ::windows::core::Interface>::IID
    }
}
pub trait ITsSbPlacementImpl: Sized + ITsSbPluginImpl {
    fn QueryEnvironmentForTarget();
}
impl ITsSbPlacementVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITsSbPlacementImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITsSbPlacementVtbl {
        unsafe extern "system" fn QueryEnvironmentForTarget<Impl: ITsSbPlacementImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pconnection: ::windows::core::RawPtr, pplacementsink: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, Initialize::<Impl, IMPL_OFFSET>, Terminate::<Impl, IMPL_OFFSET>, QueryEnvironmentForTarget::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITsSbPlacement as ::windows::core::Interface>::IID
    }
}
pub trait ITsSbPlacementNotifySinkImpl: Sized + ITsSbBaseNotifySinkImpl {
    fn OnQueryEnvironmentCompleted();
}
impl ITsSbPlacementNotifySinkVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITsSbPlacementNotifySinkImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITsSbPlacementNotifySinkVtbl {
        unsafe extern "system" fn OnQueryEnvironmentCompleted<Impl: ITsSbPlacementNotifySinkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, penvironment: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, OnError::<Impl, IMPL_OFFSET>, OnReportStatus::<Impl, IMPL_OFFSET>, OnQueryEnvironmentCompleted::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITsSbPlacementNotifySink as ::windows::core::Interface>::IID
    }
}
pub trait ITsSbPluginImpl: Sized {
    fn Initialize();
    fn Terminate();
}
impl ITsSbPluginVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITsSbPluginImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITsSbPluginVtbl {
        unsafe extern "system" fn Initialize<Impl: ITsSbPluginImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pprovider: ::windows::core::RawPtr, pnotifysink: ::windows::core::RawPtr, ppropertyset: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Terminate<Impl: ITsSbPluginImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hr: ::windows::core::HRESULT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, Initialize::<Impl, IMPL_OFFSET>, Terminate::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITsSbPlugin as ::windows::core::Interface>::IID
    }
}
pub trait ITsSbPluginNotifySinkImpl: Sized + ITsSbBaseNotifySinkImpl {
    fn OnInitialized();
    fn OnTerminated();
}
impl ITsSbPluginNotifySinkVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITsSbPluginNotifySinkImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITsSbPluginNotifySinkVtbl {
        unsafe extern "system" fn OnInitialized<Impl: ITsSbPluginNotifySinkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hr: ::windows::core::HRESULT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn OnTerminated<Impl: ITsSbPluginNotifySinkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, OnError::<Impl, IMPL_OFFSET>, OnReportStatus::<Impl, IMPL_OFFSET>, OnInitialized::<Impl, IMPL_OFFSET>, OnTerminated::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITsSbPluginNotifySink as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Ole"))]
pub trait ITsSbPluginPropertySetImpl: Sized + ITsSbPropertySetImpl + IPropertyBagImpl {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Ole"))]
impl ITsSbPluginPropertySetVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITsSbPluginPropertySetImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITsSbPluginPropertySetVtbl {
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, Read::<Impl, IMPL_OFFSET>, Write::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITsSbPluginPropertySet as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Ole"))]
pub trait ITsSbPropertySetImpl: Sized + IPropertyBagImpl {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Ole"))]
impl ITsSbPropertySetVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITsSbPropertySetImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITsSbPropertySetVtbl {
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, Read::<Impl, IMPL_OFFSET>, Write::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITsSbPropertySet as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
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
#[cfg(feature = "Win32_Foundation")]
impl ITsSbProviderVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITsSbProviderImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITsSbProviderVtbl {
        unsafe extern "system" fn CreateTargetObject<Impl: ITsSbProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, targetname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, environmentname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pptarget: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CreateLoadBalanceResultObject<Impl: ITsSbProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, targetname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pplbresult: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CreateSessionObject<Impl: ITsSbProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, targetname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, username: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, domain: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, sessionid: u32, ppsession: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CreatePluginPropertySet<Impl: ITsSbProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pppropertyset: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CreateTargetPropertySetObject<Impl: ITsSbProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pppropertyset: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CreateEnvironmentObject<Impl: ITsSbProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, serverweight: u32, ppenvironment: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetResourcePluginStore<Impl: ITsSbProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppstore: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetFilterPluginStore<Impl: ITsSbProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppstore: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn RegisterForNotification<Impl: ITsSbProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, notificationtype: u32, resourcetomonitor: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ppluginnotification: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn UnRegisterForNotification<Impl: ITsSbProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, notificationtype: u32, resourcetomonitor: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetInstanceOfGlobalStore<Impl: ITsSbProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppglobalstore: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CreateEnvironmentPropertySetObject<Impl: ITsSbProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pppropertyset: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            CreateTargetObject::<Impl, IMPL_OFFSET>,
            CreateLoadBalanceResultObject::<Impl, IMPL_OFFSET>,
            CreateSessionObject::<Impl, IMPL_OFFSET>,
            CreatePluginPropertySet::<Impl, IMPL_OFFSET>,
            CreateTargetPropertySetObject::<Impl, IMPL_OFFSET>,
            CreateEnvironmentObject::<Impl, IMPL_OFFSET>,
            GetResourcePluginStore::<Impl, IMPL_OFFSET>,
            GetFilterPluginStore::<Impl, IMPL_OFFSET>,
            RegisterForNotification::<Impl, IMPL_OFFSET>,
            UnRegisterForNotification::<Impl, IMPL_OFFSET>,
            GetInstanceOfGlobalStore::<Impl, IMPL_OFFSET>,
            CreateEnvironmentPropertySetObject::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITsSbProvider as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait ITsSbProvisioningImpl: Sized + ITsSbPluginImpl {
    fn CreateVirtualMachines();
    fn PatchVirtualMachines();
    fn DeleteVirtualMachines();
    fn CancelJob();
}
#[cfg(feature = "Win32_Foundation")]
impl ITsSbProvisioningVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITsSbProvisioningImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITsSbProvisioningVtbl {
        unsafe extern "system" fn CreateVirtualMachines<Impl: ITsSbProvisioningImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, jobxmlstring: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, jobguid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, psink: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn PatchVirtualMachines<Impl: ITsSbProvisioningImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, jobxmlstring: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, jobguid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, psink: ::windows::core::RawPtr, pvmpatchinfo: *const VM_PATCH_INFO) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn DeleteVirtualMachines<Impl: ITsSbProvisioningImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, jobxmlstring: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, jobguid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, psink: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CancelJob<Impl: ITsSbProvisioningImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, jobguid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, Initialize::<Impl, IMPL_OFFSET>, Terminate::<Impl, IMPL_OFFSET>, CreateVirtualMachines::<Impl, IMPL_OFFSET>, PatchVirtualMachines::<Impl, IMPL_OFFSET>, DeleteVirtualMachines::<Impl, IMPL_OFFSET>, CancelJob::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITsSbProvisioning as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait ITsSbProvisioningPluginNotifySinkImpl: Sized {
    fn OnJobCreated();
    fn OnVirtualMachineStatusChanged();
    fn OnJobCompleted();
    fn OnJobCancelled();
    fn LockVirtualMachine();
    fn OnVirtualMachineHostStatusChanged();
}
#[cfg(feature = "Win32_Foundation")]
impl ITsSbProvisioningPluginNotifySinkVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITsSbProvisioningPluginNotifySinkImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITsSbProvisioningPluginNotifySinkVtbl {
        unsafe extern "system" fn OnJobCreated<Impl: ITsSbProvisioningPluginNotifySinkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvmnotifyinfo: *const VM_NOTIFY_INFO) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn OnVirtualMachineStatusChanged<Impl: ITsSbProvisioningPluginNotifySinkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvmnotifyentry: *const VM_NOTIFY_ENTRY, vmnotifystatus: VM_NOTIFY_STATUS, errorcode: ::windows::core::HRESULT, errordescr: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn OnJobCompleted<Impl: ITsSbProvisioningPluginNotifySinkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, resultcode: ::windows::core::HRESULT, resultdescription: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn OnJobCancelled<Impl: ITsSbProvisioningPluginNotifySinkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn LockVirtualMachine<Impl: ITsSbProvisioningPluginNotifySinkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvmnotifyentry: *const VM_NOTIFY_ENTRY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn OnVirtualMachineHostStatusChanged<Impl: ITsSbProvisioningPluginNotifySinkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, vmhost: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, vmhostnotifystatus: VM_HOST_NOTIFY_STATUS, errorcode: ::windows::core::HRESULT, errordescr: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, OnJobCreated::<Impl, IMPL_OFFSET>, OnVirtualMachineStatusChanged::<Impl, IMPL_OFFSET>, OnJobCompleted::<Impl, IMPL_OFFSET>, OnJobCancelled::<Impl, IMPL_OFFSET>, LockVirtualMachine::<Impl, IMPL_OFFSET>, OnVirtualMachineHostStatusChanged::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITsSbProvisioningPluginNotifySink as ::windows::core::Interface>::IID
    }
}
pub trait ITsSbResourceNotificationImpl: Sized {
    fn NotifySessionChange();
    fn NotifyTargetChange();
    fn NotifyClientConnectionStateChange();
}
impl ITsSbResourceNotificationVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITsSbResourceNotificationImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITsSbResourceNotificationVtbl {
        unsafe extern "system" fn NotifySessionChange<Impl: ITsSbResourceNotificationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, changetype: TSSESSION_STATE, psession: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn NotifyTargetChange<Impl: ITsSbResourceNotificationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, targetchangetype: u32, ptarget: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn NotifyClientConnectionStateChange<Impl: ITsSbResourceNotificationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, changetype: CONNECTION_CHANGE_NOTIFICATION, pconnection: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, NotifySessionChange::<Impl, IMPL_OFFSET>, NotifyTargetChange::<Impl, IMPL_OFFSET>, NotifyClientConnectionStateChange::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITsSbResourceNotification as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait ITsSbResourceNotificationExImpl: Sized {
    fn NotifySessionChangeEx();
    fn NotifyTargetChangeEx();
    fn NotifyClientConnectionStateChangeEx();
}
#[cfg(feature = "Win32_Foundation")]
impl ITsSbResourceNotificationExVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITsSbResourceNotificationExImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITsSbResourceNotificationExVtbl {
        unsafe extern "system" fn NotifySessionChangeEx<Impl: ITsSbResourceNotificationExImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, targetname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, username: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, domain: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, sessionid: u32, sessionstate: TSSESSION_STATE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn NotifyTargetChangeEx<Impl: ITsSbResourceNotificationExImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, targetname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, targetchangetype: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn NotifyClientConnectionStateChangeEx<Impl: ITsSbResourceNotificationExImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, username: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, domain: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, initialprogram: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, poolname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, targetname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, connectionchangetype: CONNECTION_CHANGE_NOTIFICATION) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, NotifySessionChangeEx::<Impl, IMPL_OFFSET>, NotifyTargetChangeEx::<Impl, IMPL_OFFSET>, NotifyClientConnectionStateChangeEx::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITsSbResourceNotificationEx as ::windows::core::Interface>::IID
    }
}
pub trait ITsSbResourcePluginImpl: Sized + ITsSbPluginImpl {}
impl ITsSbResourcePluginVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITsSbResourcePluginImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITsSbResourcePluginVtbl {
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, Initialize::<Impl, IMPL_OFFSET>, Terminate::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITsSbResourcePlugin as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
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
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ITsSbResourcePluginStoreVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITsSbResourcePluginStoreImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITsSbResourcePluginStoreVtbl {
        unsafe extern "system" fn QueryTarget<Impl: ITsSbResourcePluginStoreImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, targetname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, farmname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pptarget: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn QuerySessionBySessionId<Impl: ITsSbResourcePluginStoreImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwsessionid: u32, targetname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ppsession: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn AddTargetToStore<Impl: ITsSbResourcePluginStoreImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ptarget: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn AddSessionToStore<Impl: ITsSbResourcePluginStoreImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psession: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn AddEnvironmentToStore<Impl: ITsSbResourcePluginStoreImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, penvironment: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn RemoveEnvironmentFromStore<Impl: ITsSbResourcePluginStoreImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, environmentname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bignoreowner: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn EnumerateFarms<Impl: ITsSbResourcePluginStoreImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwcount: *mut u32, pval: *mut *mut super::Com::SAFEARRAY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn QueryEnvironment<Impl: ITsSbResourcePluginStoreImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, environmentname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ppenvironment: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn EnumerateEnvironments<Impl: ITsSbResourcePluginStoreImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwcount: *mut u32, pval: *mut *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SaveTarget<Impl: ITsSbResourcePluginStoreImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ptarget: ::windows::core::RawPtr, bforcewrite: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SaveEnvironment<Impl: ITsSbResourcePluginStoreImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, penvironment: ::windows::core::RawPtr, bforcewrite: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SaveSession<Impl: ITsSbResourcePluginStoreImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psession: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetTargetProperty<Impl: ITsSbResourcePluginStoreImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, targetname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, propertyname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pproperty: *const super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetEnvironmentProperty<Impl: ITsSbResourcePluginStoreImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, environmentname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, propertyname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pproperty: *const super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetTargetState<Impl: ITsSbResourcePluginStoreImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, targetname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, newstate: TARGET_STATE, poldstate: *mut TARGET_STATE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetSessionState<Impl: ITsSbResourcePluginStoreImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, sbsession: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn EnumerateTargets<Impl: ITsSbResourcePluginStoreImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, farmname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, envname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, sortbyfieldid: TS_SB_SORT_BY, sortybypropname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pdwcount: *mut u32, pval: *mut *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn EnumerateSessions<Impl: ITsSbResourcePluginStoreImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, targetname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, username: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, userdomain: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, poolname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, initialprogram: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, psessionstate: *const TSSESSION_STATE, pdwcount: *mut u32, ppval: *mut *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetFarmProperty<Impl: ITsSbResourcePluginStoreImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, farmname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, propertyname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pvarvalue: *const super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn DeleteTarget<Impl: ITsSbResourcePluginStoreImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, targetname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, hostname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetTargetPropertyWithVersionCheck<Impl: ITsSbResourcePluginStoreImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ptarget: ::windows::core::RawPtr, propertyname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pproperty: *const super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetEnvironmentPropertyWithVersionCheck<Impl: ITsSbResourcePluginStoreImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, penvironment: ::windows::core::RawPtr, propertyname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pproperty: *const super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn AcquireTargetLock<Impl: ITsSbResourcePluginStoreImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, targetname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, dwtimeout: u32, ppcontext: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ReleaseTargetLock<Impl: ITsSbResourcePluginStoreImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcontext: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn TestAndSetServerState<Impl: ITsSbResourcePluginStoreImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, poolname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, serverfqdn: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, newstate: TARGET_STATE, teststate: TARGET_STATE, pinitstate: *mut TARGET_STATE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetServerWaitingToStart<Impl: ITsSbResourcePluginStoreImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, poolname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, servername: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetServerState<Impl: ITsSbResourcePluginStoreImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, poolname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, serverfqdn: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pstate: *mut TARGET_STATE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetServerDrainMode<Impl: ITsSbResourcePluginStoreImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, serverfqdn: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, drainmode: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            QueryTarget::<Impl, IMPL_OFFSET>,
            QuerySessionBySessionId::<Impl, IMPL_OFFSET>,
            AddTargetToStore::<Impl, IMPL_OFFSET>,
            AddSessionToStore::<Impl, IMPL_OFFSET>,
            AddEnvironmentToStore::<Impl, IMPL_OFFSET>,
            RemoveEnvironmentFromStore::<Impl, IMPL_OFFSET>,
            EnumerateFarms::<Impl, IMPL_OFFSET>,
            QueryEnvironment::<Impl, IMPL_OFFSET>,
            EnumerateEnvironments::<Impl, IMPL_OFFSET>,
            SaveTarget::<Impl, IMPL_OFFSET>,
            SaveEnvironment::<Impl, IMPL_OFFSET>,
            SaveSession::<Impl, IMPL_OFFSET>,
            SetTargetProperty::<Impl, IMPL_OFFSET>,
            SetEnvironmentProperty::<Impl, IMPL_OFFSET>,
            SetTargetState::<Impl, IMPL_OFFSET>,
            SetSessionState::<Impl, IMPL_OFFSET>,
            EnumerateTargets::<Impl, IMPL_OFFSET>,
            EnumerateSessions::<Impl, IMPL_OFFSET>,
            GetFarmProperty::<Impl, IMPL_OFFSET>,
            DeleteTarget::<Impl, IMPL_OFFSET>,
            SetTargetPropertyWithVersionCheck::<Impl, IMPL_OFFSET>,
            SetEnvironmentPropertyWithVersionCheck::<Impl, IMPL_OFFSET>,
            AcquireTargetLock::<Impl, IMPL_OFFSET>,
            ReleaseTargetLock::<Impl, IMPL_OFFSET>,
            TestAndSetServerState::<Impl, IMPL_OFFSET>,
            SetServerWaitingToStart::<Impl, IMPL_OFFSET>,
            GetServerState::<Impl, IMPL_OFFSET>,
            SetServerDrainMode::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITsSbResourcePluginStore as ::windows::core::Interface>::IID
    }
}
pub trait ITsSbServiceNotificationImpl: Sized {
    fn NotifyServiceFailure();
    fn NotifyServiceSuccess();
}
impl ITsSbServiceNotificationVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITsSbServiceNotificationImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITsSbServiceNotificationVtbl {
        unsafe extern "system" fn NotifyServiceFailure<Impl: ITsSbServiceNotificationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn NotifyServiceSuccess<Impl: ITsSbServiceNotificationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, NotifyServiceFailure::<Impl, IMPL_OFFSET>, NotifyServiceSuccess::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITsSbServiceNotification as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
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
#[cfg(feature = "Win32_Foundation")]
impl ITsSbSessionVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITsSbSessionImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITsSbSessionVtbl {
        unsafe extern "system" fn SessionId<Impl: ITsSbSessionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn TargetName<Impl: ITsSbSessionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, targetname: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetTargetName<Impl: ITsSbSessionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, targetname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Username<Impl: ITsSbSessionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, username: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Domain<Impl: ITsSbSessionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, domain: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn State<Impl: ITsSbSessionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pstate: *mut TSSESSION_STATE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetState<Impl: ITsSbSessionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, state: TSSESSION_STATE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CreateTime<Impl: ITsSbSessionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ptime: *mut super::super::Foundation::FILETIME) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetCreateTime<Impl: ITsSbSessionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, time: super::super::Foundation::FILETIME) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn DisconnectTime<Impl: ITsSbSessionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ptime: *mut super::super::Foundation::FILETIME) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetDisconnectTime<Impl: ITsSbSessionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, time: super::super::Foundation::FILETIME) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn InitialProgram<Impl: ITsSbSessionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, app: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetInitialProgram<Impl: ITsSbSessionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, application: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ClientDisplay<Impl: ITsSbSessionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pclientdisplay: *mut CLIENT_DISPLAY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetClientDisplay<Impl: ITsSbSessionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pclientdisplay: CLIENT_DISPLAY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ProtocolType<Impl: ITsSbSessionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetProtocolType<Impl: ITsSbSessionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, val: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            SessionId::<Impl, IMPL_OFFSET>,
            TargetName::<Impl, IMPL_OFFSET>,
            SetTargetName::<Impl, IMPL_OFFSET>,
            Username::<Impl, IMPL_OFFSET>,
            Domain::<Impl, IMPL_OFFSET>,
            State::<Impl, IMPL_OFFSET>,
            SetState::<Impl, IMPL_OFFSET>,
            CreateTime::<Impl, IMPL_OFFSET>,
            SetCreateTime::<Impl, IMPL_OFFSET>,
            DisconnectTime::<Impl, IMPL_OFFSET>,
            SetDisconnectTime::<Impl, IMPL_OFFSET>,
            InitialProgram::<Impl, IMPL_OFFSET>,
            SetInitialProgram::<Impl, IMPL_OFFSET>,
            ClientDisplay::<Impl, IMPL_OFFSET>,
            SetClientDisplay::<Impl, IMPL_OFFSET>,
            ProtocolType::<Impl, IMPL_OFFSET>,
            SetProtocolType::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITsSbSession as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
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
#[cfg(feature = "Win32_Foundation")]
impl ITsSbTargetVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITsSbTargetImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITsSbTargetVtbl {
        unsafe extern "system" fn TargetName<Impl: ITsSbTargetImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetTargetName<Impl: ITsSbTargetImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, val: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn FarmName<Impl: ITsSbTargetImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetFarmName<Impl: ITsSbTargetImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, val: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn TargetFQDN<Impl: ITsSbTargetImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, targetfqdnname: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetTargetFQDN<Impl: ITsSbTargetImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, val: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn TargetNetbios<Impl: ITsSbTargetImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, targetnetbiosname: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetTargetNetbios<Impl: ITsSbTargetImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, val: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn IpAddresses<Impl: ITsSbTargetImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, sockaddr: *mut TSSD_ConnectionPoint, numaddresses: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetIpAddresses<Impl: ITsSbTargetImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, sockaddr: *const TSSD_ConnectionPoint, numaddresses: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn TargetState<Impl: ITsSbTargetImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pstate: *mut TARGET_STATE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetTargetState<Impl: ITsSbTargetImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, state: TARGET_STATE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn TargetPropertySet<Impl: ITsSbTargetImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pppropertyset: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetTargetPropertySet<Impl: ITsSbTargetImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn EnvironmentName<Impl: ITsSbTargetImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetEnvironmentName<Impl: ITsSbTargetImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, val: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn NumSessions<Impl: ITsSbTargetImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pnumsessions: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn NumPendingConnections<Impl: ITsSbTargetImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pnumpendingconnections: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn TargetLoad<Impl: ITsSbTargetImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ptargetload: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            TargetName::<Impl, IMPL_OFFSET>,
            SetTargetName::<Impl, IMPL_OFFSET>,
            FarmName::<Impl, IMPL_OFFSET>,
            SetFarmName::<Impl, IMPL_OFFSET>,
            TargetFQDN::<Impl, IMPL_OFFSET>,
            SetTargetFQDN::<Impl, IMPL_OFFSET>,
            TargetNetbios::<Impl, IMPL_OFFSET>,
            SetTargetNetbios::<Impl, IMPL_OFFSET>,
            IpAddresses::<Impl, IMPL_OFFSET>,
            SetIpAddresses::<Impl, IMPL_OFFSET>,
            TargetState::<Impl, IMPL_OFFSET>,
            SetTargetState::<Impl, IMPL_OFFSET>,
            TargetPropertySet::<Impl, IMPL_OFFSET>,
            SetTargetPropertySet::<Impl, IMPL_OFFSET>,
            EnvironmentName::<Impl, IMPL_OFFSET>,
            SetEnvironmentName::<Impl, IMPL_OFFSET>,
            NumSessions::<Impl, IMPL_OFFSET>,
            NumPendingConnections::<Impl, IMPL_OFFSET>,
            TargetLoad::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITsSbTarget as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Ole"))]
pub trait ITsSbTargetPropertySetImpl: Sized + ITsSbPropertySetImpl + IPropertyBagImpl {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Ole"))]
impl ITsSbTargetPropertySetVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITsSbTargetPropertySetImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITsSbTargetPropertySetVtbl {
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, Read::<Impl, IMPL_OFFSET>, Write::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITsSbTargetPropertySet as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
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
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ITsSbTaskInfoVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITsSbTaskInfoImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITsSbTaskInfoVtbl {
        unsafe extern "system" fn TargetId<Impl: ITsSbTaskInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pname: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn StartTime<Impl: ITsSbTaskInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pstarttime: *mut super::super::Foundation::FILETIME) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn EndTime<Impl: ITsSbTaskInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pendtime: *mut super::super::Foundation::FILETIME) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Deadline<Impl: ITsSbTaskInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdeadline: *mut super::super::Foundation::FILETIME) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Identifier<Impl: ITsSbTaskInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pidentifier: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Label<Impl: ITsSbTaskInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plabel: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Context<Impl: ITsSbTaskInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcontext: *mut *mut super::Com::SAFEARRAY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Plugin<Impl: ITsSbTaskInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pplugin: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Status<Impl: ITsSbTaskInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pstatus: *mut RDV_TASK_STATUS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, TargetId::<Impl, IMPL_OFFSET>, StartTime::<Impl, IMPL_OFFSET>, EndTime::<Impl, IMPL_OFFSET>, Deadline::<Impl, IMPL_OFFSET>, Identifier::<Impl, IMPL_OFFSET>, Label::<Impl, IMPL_OFFSET>, Context::<Impl, IMPL_OFFSET>, Plugin::<Impl, IMPL_OFFSET>, Status::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITsSbTaskInfo as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait ITsSbTaskPluginImpl: Sized + ITsSbPluginImpl {
    fn InitializeTaskPlugin();
    fn SetTaskQueue();
}
#[cfg(feature = "Win32_Foundation")]
impl ITsSbTaskPluginVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITsSbTaskPluginImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITsSbTaskPluginVtbl {
        unsafe extern "system" fn InitializeTaskPlugin<Impl: ITsSbTaskPluginImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pitssbtaskpluginnotifysink: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetTaskQueue<Impl: ITsSbTaskPluginImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszhostname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, sbtaskinfosize: u32, pitssbtaskinfo: *const ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, Initialize::<Impl, IMPL_OFFSET>, Terminate::<Impl, IMPL_OFFSET>, InitializeTaskPlugin::<Impl, IMPL_OFFSET>, SetTaskQueue::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITsSbTaskPlugin as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub trait ITsSbTaskPluginNotifySinkImpl: Sized + ITsSbBaseNotifySinkImpl {
    fn OnSetTaskTime();
    fn OnDeleteTaskTime();
    fn OnUpdateTaskStatus();
    fn OnReportTasks();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ITsSbTaskPluginNotifySinkVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITsSbTaskPluginNotifySinkImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITsSbTaskPluginNotifySinkVtbl {
        unsafe extern "system" fn OnSetTaskTime<Impl: ITsSbTaskPluginNotifySinkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, sztargetname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, taskstarttime: super::super::Foundation::FILETIME, taskendtime: super::super::Foundation::FILETIME, taskdeadline: super::super::Foundation::FILETIME, sztasklabel: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, sztaskidentifier: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, sztaskplugin: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, dwtaskstatus: u32, sacontext: *const super::Com::SAFEARRAY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn OnDeleteTaskTime<Impl: ITsSbTaskPluginNotifySinkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, sztargetname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, sztaskidentifier: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn OnUpdateTaskStatus<Impl: ITsSbTaskPluginNotifySinkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, sztargetname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, taskidentifier: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, taskstatus: RDV_TASK_STATUS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn OnReportTasks<Impl: ITsSbTaskPluginNotifySinkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, szhostname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, OnError::<Impl, IMPL_OFFSET>, OnReportStatus::<Impl, IMPL_OFFSET>, OnSetTaskTime::<Impl, IMPL_OFFSET>, OnDeleteTaskTime::<Impl, IMPL_OFFSET>, OnUpdateTaskStatus::<Impl, IMPL_OFFSET>, OnReportTasks::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITsSbTaskPluginNotifySink as ::windows::core::Interface>::IID
    }
}
pub trait IWRdsEnhancedFastReconnectArbitratorImpl: Sized {
    fn GetSessionForEnhancedFastReconnect();
}
impl IWRdsEnhancedFastReconnectArbitratorVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWRdsEnhancedFastReconnectArbitratorImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWRdsEnhancedFastReconnectArbitratorVtbl {
        unsafe extern "system" fn GetSessionForEnhancedFastReconnect<Impl: IWRdsEnhancedFastReconnectArbitratorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psessionidarray: *const i32, dwsessioncount: u32, presultsessionid: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetSessionForEnhancedFastReconnect::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWRdsEnhancedFastReconnectArbitrator as ::windows::core::Interface>::IID
    }
}
pub trait IWRdsGraphicsChannelImpl: Sized {
    fn Write();
    fn Close();
    fn Open();
}
impl IWRdsGraphicsChannelVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWRdsGraphicsChannelImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWRdsGraphicsChannelVtbl {
        unsafe extern "system" fn Write<Impl: IWRdsGraphicsChannelImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cbsize: u32, pbuffer: *const u8, pcontext: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Close<Impl: IWRdsGraphicsChannelImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Open<Impl: IWRdsGraphicsChannelImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pchannelevents: ::windows::core::RawPtr, popencontext: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, Write::<Impl, IMPL_OFFSET>, Close::<Impl, IMPL_OFFSET>, Open::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWRdsGraphicsChannel as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IWRdsGraphicsChannelEventsImpl: Sized {
    fn OnDataReceived();
    fn OnClose();
    fn OnChannelOpened();
    fn OnDataSent();
    fn OnMetricsUpdate();
}
#[cfg(feature = "Win32_Foundation")]
impl IWRdsGraphicsChannelEventsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWRdsGraphicsChannelEventsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWRdsGraphicsChannelEventsVtbl {
        unsafe extern "system" fn OnDataReceived<Impl: IWRdsGraphicsChannelEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cbsize: u32, pbuffer: *const u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn OnClose<Impl: IWRdsGraphicsChannelEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn OnChannelOpened<Impl: IWRdsGraphicsChannelEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, openresult: ::windows::core::HRESULT, popencontext: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn OnDataSent<Impl: IWRdsGraphicsChannelEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwritecontext: *mut ::core::ffi::c_void, bcancelled: super::super::Foundation::BOOL, pbuffer: *const u8, cbbuffer: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn OnMetricsUpdate<Impl: IWRdsGraphicsChannelEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bandwidth: u32, rtt: u32, lastsentbyteindex: u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, OnDataReceived::<Impl, IMPL_OFFSET>, OnClose::<Impl, IMPL_OFFSET>, OnChannelOpened::<Impl, IMPL_OFFSET>, OnDataSent::<Impl, IMPL_OFFSET>, OnMetricsUpdate::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWRdsGraphicsChannelEvents as ::windows::core::Interface>::IID
    }
}
pub trait IWRdsGraphicsChannelManagerImpl: Sized {
    fn CreateChannel();
}
impl IWRdsGraphicsChannelManagerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWRdsGraphicsChannelManagerImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWRdsGraphicsChannelManagerVtbl {
        unsafe extern "system" fn CreateChannel<Impl: IWRdsGraphicsChannelManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszchannelname: *const u8, channeltype: WRdsGraphicsChannelType, ppvirtualchannel: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, CreateChannel::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWRdsGraphicsChannelManager as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
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
#[cfg(feature = "Win32_Foundation")]
impl IWRdsProtocolConnectionVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWRdsProtocolConnectionImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWRdsProtocolConnectionVtbl {
        unsafe extern "system" fn GetLogonErrorRedirector<Impl: IWRdsProtocolConnectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pplogonerrorredir: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn AcceptConnection<Impl: IWRdsProtocolConnectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetClientData<Impl: IWRdsProtocolConnectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pclientdata: *mut WTS_CLIENT_DATA) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetClientMonitorData<Impl: IWRdsProtocolConnectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pnummonitors: *mut u32, pprimarymonitor: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetUserCredentials<Impl: IWRdsProtocolConnectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pusercreds: *mut WTS_USER_CREDENTIAL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetLicenseConnection<Impl: IWRdsProtocolConnectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pplicenseconnection: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn AuthenticateClientToSession<Impl: IWRdsProtocolConnectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, sessionid: *mut WTS_SESSION_ID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn NotifySessionId<Impl: IWRdsProtocolConnectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, sessionid: *const WTS_SESSION_ID, sessionhandle: super::super::Foundation::HANDLE_PTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetInputHandles<Impl: IWRdsProtocolConnectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pkeyboardhandle: *mut super::super::Foundation::HANDLE_PTR, pmousehandle: *mut super::super::Foundation::HANDLE_PTR, pbeephandle: *mut super::super::Foundation::HANDLE_PTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetVideoHandle<Impl: IWRdsProtocolConnectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvideohandle: *mut super::super::Foundation::HANDLE_PTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ConnectNotify<Impl: IWRdsProtocolConnectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, sessionid: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn IsUserAllowedToLogon<Impl: IWRdsProtocolConnectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, sessionid: u32, usertoken: super::super::Foundation::HANDLE_PTR, pdomainname: super::super::Foundation::PWSTR, pusername: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SessionArbitrationEnumeration<Impl: IWRdsProtocolConnectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, husertoken: super::super::Foundation::HANDLE_PTR, bsinglesessionperuserenabled: super::super::Foundation::BOOL, psessionidarray: *mut u32, pdwsessionidentifiercount: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn LogonNotify<Impl: IWRdsProtocolConnectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hclienttoken: super::super::Foundation::HANDLE_PTR, wszusername: super::super::Foundation::PWSTR, wszdomainname: super::super::Foundation::PWSTR, sessionid: *const WTS_SESSION_ID, pwrdsconnectionsettings: *mut WRDS_CONNECTION_SETTINGS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn PreDisconnect<Impl: IWRdsProtocolConnectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, disconnectreason: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn DisconnectNotify<Impl: IWRdsProtocolConnectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Close<Impl: IWRdsProtocolConnectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetProtocolStatus<Impl: IWRdsProtocolConnectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pprotocolstatus: *mut WTS_PROTOCOL_STATUS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetLastInputTime<Impl: IWRdsProtocolConnectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plastinputtime: *mut u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetErrorInfo<Impl: IWRdsProtocolConnectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ulerror: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CreateVirtualChannel<Impl: IWRdsProtocolConnectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, szendpointname: super::super::Foundation::PSTR, bstatic: super::super::Foundation::BOOL, requestedpriority: u32, phchannel: *mut usize) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn QueryProperty<Impl: IWRdsProtocolConnectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, querytype: ::windows::core::GUID, ulnumentriesin: u32, ulnumentriesout: u32, ppropertyentriesin: *const WTS_PROPERTY_VALUE, ppropertyentriesout: *mut WTS_PROPERTY_VALUE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetShadowConnection<Impl: IWRdsProtocolConnectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppshadowconnection: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn NotifyCommandProcessCreated<Impl: IWRdsProtocolConnectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, sessionid: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            GetLogonErrorRedirector::<Impl, IMPL_OFFSET>,
            AcceptConnection::<Impl, IMPL_OFFSET>,
            GetClientData::<Impl, IMPL_OFFSET>,
            GetClientMonitorData::<Impl, IMPL_OFFSET>,
            GetUserCredentials::<Impl, IMPL_OFFSET>,
            GetLicenseConnection::<Impl, IMPL_OFFSET>,
            AuthenticateClientToSession::<Impl, IMPL_OFFSET>,
            NotifySessionId::<Impl, IMPL_OFFSET>,
            GetInputHandles::<Impl, IMPL_OFFSET>,
            GetVideoHandle::<Impl, IMPL_OFFSET>,
            ConnectNotify::<Impl, IMPL_OFFSET>,
            IsUserAllowedToLogon::<Impl, IMPL_OFFSET>,
            SessionArbitrationEnumeration::<Impl, IMPL_OFFSET>,
            LogonNotify::<Impl, IMPL_OFFSET>,
            PreDisconnect::<Impl, IMPL_OFFSET>,
            DisconnectNotify::<Impl, IMPL_OFFSET>,
            Close::<Impl, IMPL_OFFSET>,
            GetProtocolStatus::<Impl, IMPL_OFFSET>,
            GetLastInputTime::<Impl, IMPL_OFFSET>,
            SetErrorInfo::<Impl, IMPL_OFFSET>,
            CreateVirtualChannel::<Impl, IMPL_OFFSET>,
            QueryProperty::<Impl, IMPL_OFFSET>,
            GetShadowConnection::<Impl, IMPL_OFFSET>,
            NotifyCommandProcessCreated::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWRdsProtocolConnection as ::windows::core::Interface>::IID
    }
}
pub trait IWRdsProtocolConnectionCallbackImpl: Sized {
    fn OnReady();
    fn BrokenConnection();
    fn StopScreenUpdates();
    fn RedrawWindow();
    fn GetConnectionId();
}
impl IWRdsProtocolConnectionCallbackVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWRdsProtocolConnectionCallbackImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWRdsProtocolConnectionCallbackVtbl {
        unsafe extern "system" fn OnReady<Impl: IWRdsProtocolConnectionCallbackImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn BrokenConnection<Impl: IWRdsProtocolConnectionCallbackImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, reason: u32, source: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn StopScreenUpdates<Impl: IWRdsProtocolConnectionCallbackImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn RedrawWindow<Impl: IWRdsProtocolConnectionCallbackImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rect: *const WTS_SMALL_RECT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetConnectionId<Impl: IWRdsProtocolConnectionCallbackImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pconnectionid: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, OnReady::<Impl, IMPL_OFFSET>, BrokenConnection::<Impl, IMPL_OFFSET>, StopScreenUpdates::<Impl, IMPL_OFFSET>, RedrawWindow::<Impl, IMPL_OFFSET>, GetConnectionId::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWRdsProtocolConnectionCallback as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IWRdsProtocolConnectionSettingsImpl: Sized {
    fn SetConnectionSetting();
    fn GetConnectionSetting();
}
#[cfg(feature = "Win32_Foundation")]
impl IWRdsProtocolConnectionSettingsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWRdsProtocolConnectionSettingsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWRdsProtocolConnectionSettingsVtbl {
        unsafe extern "system" fn SetConnectionSetting<Impl: IWRdsProtocolConnectionSettingsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, propertyid: ::windows::core::GUID, ppropertyentriesin: *const WTS_PROPERTY_VALUE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetConnectionSetting<Impl: IWRdsProtocolConnectionSettingsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, propertyid: ::windows::core::GUID, ppropertyentriesout: *mut WTS_PROPERTY_VALUE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, SetConnectionSetting::<Impl, IMPL_OFFSET>, GetConnectionSetting::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWRdsProtocolConnectionSettings as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IWRdsProtocolLicenseConnectionImpl: Sized {
    fn RequestLicensingCapabilities();
    fn SendClientLicense();
    fn RequestClientLicense();
    fn ProtocolComplete();
}
#[cfg(feature = "Win32_Foundation")]
impl IWRdsProtocolLicenseConnectionVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWRdsProtocolLicenseConnectionImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWRdsProtocolLicenseConnectionVtbl {
        unsafe extern "system" fn RequestLicensingCapabilities<Impl: IWRdsProtocolLicenseConnectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pplicensecapabilities: *mut WTS_LICENSE_CAPABILITIES, pcblicensecapabilities: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SendClientLicense<Impl: IWRdsProtocolLicenseConnectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pclientlicense: *const u8, cbclientlicense: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn RequestClientLicense<Impl: IWRdsProtocolLicenseConnectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, reserve1: *const u8, reserve2: u32, ppclientlicense: *mut u8, pcbclientlicense: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ProtocolComplete<Impl: IWRdsProtocolLicenseConnectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ulcomplete: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, RequestLicensingCapabilities::<Impl, IMPL_OFFSET>, SendClientLicense::<Impl, IMPL_OFFSET>, RequestClientLicense::<Impl, IMPL_OFFSET>, ProtocolComplete::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWRdsProtocolLicenseConnection as ::windows::core::Interface>::IID
    }
}
pub trait IWRdsProtocolListenerImpl: Sized {
    fn GetSettings();
    fn StartListen();
    fn StopListen();
}
impl IWRdsProtocolListenerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWRdsProtocolListenerImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWRdsProtocolListenerVtbl {
        unsafe extern "system" fn GetSettings<Impl: IWRdsProtocolListenerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wrdslistenersettinglevel: WRDS_LISTENER_SETTING_LEVEL, pwrdslistenersettings: *mut WRDS_LISTENER_SETTINGS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn StartListen<Impl: IWRdsProtocolListenerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcallback: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn StopListen<Impl: IWRdsProtocolListenerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetSettings::<Impl, IMPL_OFFSET>, StartListen::<Impl, IMPL_OFFSET>, StopListen::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWRdsProtocolListener as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IWRdsProtocolListenerCallbackImpl: Sized {
    fn OnConnected();
}
#[cfg(feature = "Win32_Foundation")]
impl IWRdsProtocolListenerCallbackVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWRdsProtocolListenerCallbackImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWRdsProtocolListenerCallbackVtbl {
        unsafe extern "system" fn OnConnected<Impl: IWRdsProtocolListenerCallbackImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pconnection: ::windows::core::RawPtr, pwrdsconnectionsettings: *const WRDS_CONNECTION_SETTINGS, pcallback: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, OnConnected::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWRdsProtocolListenerCallback as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IWRdsProtocolLogonErrorRedirectorImpl: Sized {
    fn OnBeginPainting();
    fn RedirectStatus();
    fn RedirectMessage();
    fn RedirectLogonError();
}
#[cfg(feature = "Win32_Foundation")]
impl IWRdsProtocolLogonErrorRedirectorVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWRdsProtocolLogonErrorRedirectorImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWRdsProtocolLogonErrorRedirectorVtbl {
        unsafe extern "system" fn OnBeginPainting<Impl: IWRdsProtocolLogonErrorRedirectorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn RedirectStatus<Impl: IWRdsProtocolLogonErrorRedirectorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszmessage: super::super::Foundation::PWSTR, presponse: *mut WTS_LOGON_ERROR_REDIRECTOR_RESPONSE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn RedirectMessage<Impl: IWRdsProtocolLogonErrorRedirectorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszcaption: super::super::Foundation::PWSTR, pszmessage: super::super::Foundation::PWSTR, utype: u32, presponse: *mut WTS_LOGON_ERROR_REDIRECTOR_RESPONSE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn RedirectLogonError<Impl: IWRdsProtocolLogonErrorRedirectorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ntsstatus: i32, ntssubstatus: i32, pszcaption: super::super::Foundation::PWSTR, pszmessage: super::super::Foundation::PWSTR, utype: u32, presponse: *mut WTS_LOGON_ERROR_REDIRECTOR_RESPONSE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, OnBeginPainting::<Impl, IMPL_OFFSET>, RedirectStatus::<Impl, IMPL_OFFSET>, RedirectMessage::<Impl, IMPL_OFFSET>, RedirectLogonError::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWRdsProtocolLogonErrorRedirector as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
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
#[cfg(feature = "Win32_Foundation")]
impl IWRdsProtocolManagerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWRdsProtocolManagerImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWRdsProtocolManagerVtbl {
        unsafe extern "system" fn Initialize<Impl: IWRdsProtocolManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, piwrdssettings: ::windows::core::RawPtr, pwrdssettings: *const WRDS_SETTINGS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CreateListener<Impl: IWRdsProtocolManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wszlistenername: super::super::Foundation::PWSTR, pprotocollistener: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn NotifyServiceStateChange<Impl: IWRdsProtocolManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ptsservicestatechange: *const WTS_SERVICE_STATE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn NotifySessionOfServiceStart<Impl: IWRdsProtocolManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, sessionid: *const WTS_SESSION_ID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn NotifySessionOfServiceStop<Impl: IWRdsProtocolManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, sessionid: *const WTS_SESSION_ID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn NotifySessionStateChange<Impl: IWRdsProtocolManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, sessionid: *const WTS_SESSION_ID, eventid: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn NotifySettingsChange<Impl: IWRdsProtocolManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwrdssettings: *const WRDS_SETTINGS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Uninitialize<Impl: IWRdsProtocolManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            Initialize::<Impl, IMPL_OFFSET>,
            CreateListener::<Impl, IMPL_OFFSET>,
            NotifyServiceStateChange::<Impl, IMPL_OFFSET>,
            NotifySessionOfServiceStart::<Impl, IMPL_OFFSET>,
            NotifySessionOfServiceStop::<Impl, IMPL_OFFSET>,
            NotifySessionStateChange::<Impl, IMPL_OFFSET>,
            NotifySettingsChange::<Impl, IMPL_OFFSET>,
            Uninitialize::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWRdsProtocolManager as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IWRdsProtocolSettingsImpl: Sized {
    fn GetSettings();
    fn MergeSettings();
}
#[cfg(feature = "Win32_Foundation")]
impl IWRdsProtocolSettingsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWRdsProtocolSettingsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWRdsProtocolSettingsVtbl {
        unsafe extern "system" fn GetSettings<Impl: IWRdsProtocolSettingsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wrdssettingtype: WRDS_SETTING_TYPE, wrdssettinglevel: WRDS_SETTING_LEVEL, pwrdssettings: *mut WRDS_SETTINGS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn MergeSettings<Impl: IWRdsProtocolSettingsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwrdssettings: *const WRDS_SETTINGS, wrdsconnectionsettinglevel: WRDS_CONNECTION_SETTING_LEVEL, pwrdsconnectionsettings: *mut WRDS_CONNECTION_SETTINGS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetSettings::<Impl, IMPL_OFFSET>, MergeSettings::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWRdsProtocolSettings as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IWRdsProtocolShadowCallbackImpl: Sized {
    fn StopShadow();
    fn InvokeTargetShadow();
}
#[cfg(feature = "Win32_Foundation")]
impl IWRdsProtocolShadowCallbackVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWRdsProtocolShadowCallbackImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWRdsProtocolShadowCallbackVtbl {
        unsafe extern "system" fn StopShadow<Impl: IWRdsProtocolShadowCallbackImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn InvokeTargetShadow<Impl: IWRdsProtocolShadowCallbackImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ptargetservername: super::super::Foundation::PWSTR, targetsessionid: u32, pparam1: *const u8, param1size: u32, pparam2: *const u8, param2size: u32, pparam3: *const u8, param3size: u32, pparam4: *const u8, param4size: u32, pclientname: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, StopShadow::<Impl, IMPL_OFFSET>, InvokeTargetShadow::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWRdsProtocolShadowCallback as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IWRdsProtocolShadowConnectionImpl: Sized {
    fn Start();
    fn Stop();
    fn DoTarget();
}
#[cfg(feature = "Win32_Foundation")]
impl IWRdsProtocolShadowConnectionVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWRdsProtocolShadowConnectionImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWRdsProtocolShadowConnectionVtbl {
        unsafe extern "system" fn Start<Impl: IWRdsProtocolShadowConnectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ptargetservername: super::super::Foundation::PWSTR, targetsessionid: u32, hotkeyvk: u8, hotkeymodifiers: u16, pshadowcallback: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Stop<Impl: IWRdsProtocolShadowConnectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn DoTarget<Impl: IWRdsProtocolShadowConnectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pparam1: *const u8, param1size: u32, pparam2: *const u8, param2size: u32, pparam3: *const u8, param3size: u32, pparam4: *const u8, param4size: u32, pclientname: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, Start::<Impl, IMPL_OFFSET>, Stop::<Impl, IMPL_OFFSET>, DoTarget::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWRdsProtocolShadowConnection as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IWRdsWddmIddPropsImpl: Sized {
    fn GetHardwareId();
    fn OnDriverLoad();
    fn OnDriverUnload();
    fn EnableWddmIdd();
}
#[cfg(feature = "Win32_Foundation")]
impl IWRdsWddmIddPropsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWRdsWddmIddPropsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWRdsWddmIddPropsVtbl {
        unsafe extern "system" fn GetHardwareId<Impl: IWRdsWddmIddPropsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdisplaydriverhardwareid: super::super::Foundation::PWSTR, count: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn OnDriverLoad<Impl: IWRdsWddmIddPropsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, sessionid: u32, driverhandle: super::super::Foundation::HANDLE_PTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn OnDriverUnload<Impl: IWRdsWddmIddPropsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, sessionid: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn EnableWddmIdd<Impl: IWRdsWddmIddPropsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, enabled: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetHardwareId::<Impl, IMPL_OFFSET>, OnDriverLoad::<Impl, IMPL_OFFSET>, OnDriverUnload::<Impl, IMPL_OFFSET>, EnableWddmIdd::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWRdsWddmIddProps as ::windows::core::Interface>::IID
    }
}
pub trait IWTSBitmapRenderServiceImpl: Sized {
    fn GetMappedRenderer();
}
impl IWTSBitmapRenderServiceVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWTSBitmapRenderServiceImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWTSBitmapRenderServiceVtbl {
        unsafe extern "system" fn GetMappedRenderer<Impl: IWTSBitmapRenderServiceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, mappingid: u64, pmappedrenderercallback: ::windows::core::RawPtr, ppmappedrenderer: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetMappedRenderer::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWTSBitmapRenderService as ::windows::core::Interface>::IID
    }
}
pub trait IWTSBitmapRendererImpl: Sized {
    fn Render();
    fn GetRendererStatistics();
    fn RemoveMapping();
}
impl IWTSBitmapRendererVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWTSBitmapRendererImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWTSBitmapRendererVtbl {
        unsafe extern "system" fn Render<Impl: IWTSBitmapRendererImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, imageformat: ::windows::core::GUID, dwwidth: u32, dwheight: u32, cbstride: i32, cbimagebuffer: u32, pimagebuffer: *const u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetRendererStatistics<Impl: IWTSBitmapRendererImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pstatistics: *mut BITMAP_RENDERER_STATISTICS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn RemoveMapping<Impl: IWTSBitmapRendererImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, Render::<Impl, IMPL_OFFSET>, GetRendererStatistics::<Impl, IMPL_OFFSET>, RemoveMapping::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWTSBitmapRenderer as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IWTSBitmapRendererCallbackImpl: Sized {
    fn OnTargetSizeChanged();
}
#[cfg(feature = "Win32_Foundation")]
impl IWTSBitmapRendererCallbackVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWTSBitmapRendererCallbackImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWTSBitmapRendererCallbackVtbl {
        unsafe extern "system" fn OnTargetSizeChanged<Impl: IWTSBitmapRendererCallbackImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rcnewsize: super::super::Foundation::RECT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, OnTargetSizeChanged::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWTSBitmapRendererCallback as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com_StructuredStorage")]
pub trait IWTSListenerImpl: Sized {
    fn GetConfiguration();
}
#[cfg(feature = "Win32_System_Com_StructuredStorage")]
impl IWTSListenerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWTSListenerImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWTSListenerVtbl {
        unsafe extern "system" fn GetConfiguration<Impl: IWTSListenerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pppropertybag: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetConfiguration::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWTSListener as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IWTSListenerCallbackImpl: Sized {
    fn OnNewChannelConnection();
}
#[cfg(feature = "Win32_Foundation")]
impl IWTSListenerCallbackVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWTSListenerCallbackImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWTSListenerCallbackVtbl {
        unsafe extern "system" fn OnNewChannelConnection<Impl: IWTSListenerCallbackImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pchannel: ::windows::core::RawPtr, data: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pbaccept: *mut super::super::Foundation::BOOL, ppcallback: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, OnNewChannelConnection::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWTSListenerCallback as ::windows::core::Interface>::IID
    }
}
pub trait IWTSPluginImpl: Sized {
    fn Initialize();
    fn Connected();
    fn Disconnected();
    fn Terminated();
}
impl IWTSPluginVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWTSPluginImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWTSPluginVtbl {
        unsafe extern "system" fn Initialize<Impl: IWTSPluginImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pchannelmgr: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Connected<Impl: IWTSPluginImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Disconnected<Impl: IWTSPluginImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwdisconnectcode: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Terminated<Impl: IWTSPluginImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, Initialize::<Impl, IMPL_OFFSET>, Connected::<Impl, IMPL_OFFSET>, Disconnected::<Impl, IMPL_OFFSET>, Terminated::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWTSPlugin as ::windows::core::Interface>::IID
    }
}
pub trait IWTSPluginServiceProviderImpl: Sized {
    fn GetService();
}
impl IWTSPluginServiceProviderVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWTSPluginServiceProviderImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWTSPluginServiceProviderVtbl {
        unsafe extern "system" fn GetService<Impl: IWTSPluginServiceProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, serviceid: ::windows::core::GUID, ppunkobject: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetService::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWTSPluginServiceProvider as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
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
#[cfg(feature = "Win32_Foundation")]
impl IWTSProtocolConnectionVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWTSProtocolConnectionImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWTSProtocolConnectionVtbl {
        unsafe extern "system" fn GetLogonErrorRedirector<Impl: IWTSProtocolConnectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pplogonerrorredir: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SendPolicyData<Impl: IWTSProtocolConnectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppolicydata: *const WTS_POLICY_DATA) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn AcceptConnection<Impl: IWTSProtocolConnectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetClientData<Impl: IWTSProtocolConnectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pclientdata: *mut WTS_CLIENT_DATA) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetUserCredentials<Impl: IWTSProtocolConnectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pusercreds: *mut WTS_USER_CREDENTIAL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetLicenseConnection<Impl: IWTSProtocolConnectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pplicenseconnection: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn AuthenticateClientToSession<Impl: IWTSProtocolConnectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, sessionid: *mut WTS_SESSION_ID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn NotifySessionId<Impl: IWTSProtocolConnectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, sessionid: *const WTS_SESSION_ID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetProtocolHandles<Impl: IWTSProtocolConnectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pkeyboardhandle: *mut super::super::Foundation::HANDLE_PTR, pmousehandle: *mut super::super::Foundation::HANDLE_PTR, pbeephandle: *mut super::super::Foundation::HANDLE_PTR, pvideohandle: *mut super::super::Foundation::HANDLE_PTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ConnectNotify<Impl: IWTSProtocolConnectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, sessionid: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn IsUserAllowedToLogon<Impl: IWTSProtocolConnectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, sessionid: u32, usertoken: super::super::Foundation::HANDLE_PTR, pdomainname: super::super::Foundation::PWSTR, pusername: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SessionArbitrationEnumeration<Impl: IWTSProtocolConnectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, husertoken: super::super::Foundation::HANDLE_PTR, bsinglesessionperuserenabled: super::super::Foundation::BOOL, psessionidarray: *mut u32, pdwsessionidentifiercount: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn LogonNotify<Impl: IWTSProtocolConnectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hclienttoken: super::super::Foundation::HANDLE_PTR, wszusername: super::super::Foundation::PWSTR, wszdomainname: super::super::Foundation::PWSTR, sessionid: *const WTS_SESSION_ID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetUserData<Impl: IWTSProtocolConnectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppolicydata: *const WTS_POLICY_DATA, pclientdata: *mut WTS_USER_DATA) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn DisconnectNotify<Impl: IWTSProtocolConnectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Close<Impl: IWTSProtocolConnectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetProtocolStatus<Impl: IWTSProtocolConnectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pprotocolstatus: *mut WTS_PROTOCOL_STATUS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetLastInputTime<Impl: IWTSProtocolConnectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plastinputtime: *mut u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetErrorInfo<Impl: IWTSProtocolConnectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ulerror: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SendBeep<Impl: IWTSProtocolConnectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, frequency: u32, duration: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CreateVirtualChannel<Impl: IWTSProtocolConnectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, szendpointname: super::super::Foundation::PSTR, bstatic: super::super::Foundation::BOOL, requestedpriority: u32, phchannel: *mut usize) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn QueryProperty<Impl: IWTSProtocolConnectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, querytype: ::windows::core::GUID, ulnumentriesin: u32, ulnumentriesout: u32, ppropertyentriesin: *const WTS_PROPERTY_VALUE, ppropertyentriesout: *mut WTS_PROPERTY_VALUE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetShadowConnection<Impl: IWTSProtocolConnectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppshadowconnection: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            GetLogonErrorRedirector::<Impl, IMPL_OFFSET>,
            SendPolicyData::<Impl, IMPL_OFFSET>,
            AcceptConnection::<Impl, IMPL_OFFSET>,
            GetClientData::<Impl, IMPL_OFFSET>,
            GetUserCredentials::<Impl, IMPL_OFFSET>,
            GetLicenseConnection::<Impl, IMPL_OFFSET>,
            AuthenticateClientToSession::<Impl, IMPL_OFFSET>,
            NotifySessionId::<Impl, IMPL_OFFSET>,
            GetProtocolHandles::<Impl, IMPL_OFFSET>,
            ConnectNotify::<Impl, IMPL_OFFSET>,
            IsUserAllowedToLogon::<Impl, IMPL_OFFSET>,
            SessionArbitrationEnumeration::<Impl, IMPL_OFFSET>,
            LogonNotify::<Impl, IMPL_OFFSET>,
            GetUserData::<Impl, IMPL_OFFSET>,
            DisconnectNotify::<Impl, IMPL_OFFSET>,
            Close::<Impl, IMPL_OFFSET>,
            GetProtocolStatus::<Impl, IMPL_OFFSET>,
            GetLastInputTime::<Impl, IMPL_OFFSET>,
            SetErrorInfo::<Impl, IMPL_OFFSET>,
            SendBeep::<Impl, IMPL_OFFSET>,
            CreateVirtualChannel::<Impl, IMPL_OFFSET>,
            QueryProperty::<Impl, IMPL_OFFSET>,
            GetShadowConnection::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWTSProtocolConnection as ::windows::core::Interface>::IID
    }
}
pub trait IWTSProtocolConnectionCallbackImpl: Sized {
    fn OnReady();
    fn BrokenConnection();
    fn StopScreenUpdates();
    fn RedrawWindow();
    fn DisplayIOCtl();
}
impl IWTSProtocolConnectionCallbackVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWTSProtocolConnectionCallbackImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWTSProtocolConnectionCallbackVtbl {
        unsafe extern "system" fn OnReady<Impl: IWTSProtocolConnectionCallbackImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn BrokenConnection<Impl: IWTSProtocolConnectionCallbackImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, reason: u32, source: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn StopScreenUpdates<Impl: IWTSProtocolConnectionCallbackImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn RedrawWindow<Impl: IWTSProtocolConnectionCallbackImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rect: *const WTS_SMALL_RECT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn DisplayIOCtl<Impl: IWTSProtocolConnectionCallbackImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, displayioctl: *const WTS_DISPLAY_IOCTL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, OnReady::<Impl, IMPL_OFFSET>, BrokenConnection::<Impl, IMPL_OFFSET>, StopScreenUpdates::<Impl, IMPL_OFFSET>, RedrawWindow::<Impl, IMPL_OFFSET>, DisplayIOCtl::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWTSProtocolConnectionCallback as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IWTSProtocolLicenseConnectionImpl: Sized {
    fn RequestLicensingCapabilities();
    fn SendClientLicense();
    fn RequestClientLicense();
    fn ProtocolComplete();
}
#[cfg(feature = "Win32_Foundation")]
impl IWTSProtocolLicenseConnectionVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWTSProtocolLicenseConnectionImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWTSProtocolLicenseConnectionVtbl {
        unsafe extern "system" fn RequestLicensingCapabilities<Impl: IWTSProtocolLicenseConnectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pplicensecapabilities: *mut WTS_LICENSE_CAPABILITIES, pcblicensecapabilities: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SendClientLicense<Impl: IWTSProtocolLicenseConnectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pclientlicense: *const u8, cbclientlicense: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn RequestClientLicense<Impl: IWTSProtocolLicenseConnectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, reserve1: *const u8, reserve2: u32, ppclientlicense: *mut u8, pcbclientlicense: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ProtocolComplete<Impl: IWTSProtocolLicenseConnectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ulcomplete: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, RequestLicensingCapabilities::<Impl, IMPL_OFFSET>, SendClientLicense::<Impl, IMPL_OFFSET>, RequestClientLicense::<Impl, IMPL_OFFSET>, ProtocolComplete::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWTSProtocolLicenseConnection as ::windows::core::Interface>::IID
    }
}
pub trait IWTSProtocolListenerImpl: Sized {
    fn StartListen();
    fn StopListen();
}
impl IWTSProtocolListenerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWTSProtocolListenerImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWTSProtocolListenerVtbl {
        unsafe extern "system" fn StartListen<Impl: IWTSProtocolListenerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcallback: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn StopListen<Impl: IWTSProtocolListenerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, StartListen::<Impl, IMPL_OFFSET>, StopListen::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWTSProtocolListener as ::windows::core::Interface>::IID
    }
}
pub trait IWTSProtocolListenerCallbackImpl: Sized {
    fn OnConnected();
}
impl IWTSProtocolListenerCallbackVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWTSProtocolListenerCallbackImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWTSProtocolListenerCallbackVtbl {
        unsafe extern "system" fn OnConnected<Impl: IWTSProtocolListenerCallbackImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pconnection: ::windows::core::RawPtr, pcallback: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, OnConnected::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWTSProtocolListenerCallback as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IWTSProtocolLogonErrorRedirectorImpl: Sized {
    fn OnBeginPainting();
    fn RedirectStatus();
    fn RedirectMessage();
    fn RedirectLogonError();
}
#[cfg(feature = "Win32_Foundation")]
impl IWTSProtocolLogonErrorRedirectorVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWTSProtocolLogonErrorRedirectorImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWTSProtocolLogonErrorRedirectorVtbl {
        unsafe extern "system" fn OnBeginPainting<Impl: IWTSProtocolLogonErrorRedirectorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn RedirectStatus<Impl: IWTSProtocolLogonErrorRedirectorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszmessage: super::super::Foundation::PWSTR, presponse: *mut WTS_LOGON_ERROR_REDIRECTOR_RESPONSE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn RedirectMessage<Impl: IWTSProtocolLogonErrorRedirectorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszcaption: super::super::Foundation::PWSTR, pszmessage: super::super::Foundation::PWSTR, utype: u32, presponse: *mut WTS_LOGON_ERROR_REDIRECTOR_RESPONSE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn RedirectLogonError<Impl: IWTSProtocolLogonErrorRedirectorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ntsstatus: i32, ntssubstatus: i32, pszcaption: super::super::Foundation::PWSTR, pszmessage: super::super::Foundation::PWSTR, utype: u32, presponse: *mut WTS_LOGON_ERROR_REDIRECTOR_RESPONSE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, OnBeginPainting::<Impl, IMPL_OFFSET>, RedirectStatus::<Impl, IMPL_OFFSET>, RedirectMessage::<Impl, IMPL_OFFSET>, RedirectLogonError::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWTSProtocolLogonErrorRedirector as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IWTSProtocolManagerImpl: Sized {
    fn CreateListener();
    fn NotifyServiceStateChange();
    fn NotifySessionOfServiceStart();
    fn NotifySessionOfServiceStop();
    fn NotifySessionStateChange();
}
#[cfg(feature = "Win32_Foundation")]
impl IWTSProtocolManagerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWTSProtocolManagerImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWTSProtocolManagerVtbl {
        unsafe extern "system" fn CreateListener<Impl: IWTSProtocolManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wszlistenername: super::super::Foundation::PWSTR, pprotocollistener: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn NotifyServiceStateChange<Impl: IWTSProtocolManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ptsservicestatechange: *const WTS_SERVICE_STATE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn NotifySessionOfServiceStart<Impl: IWTSProtocolManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, sessionid: *const WTS_SESSION_ID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn NotifySessionOfServiceStop<Impl: IWTSProtocolManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, sessionid: *const WTS_SESSION_ID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn NotifySessionStateChange<Impl: IWTSProtocolManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, sessionid: *const WTS_SESSION_ID, eventid: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, CreateListener::<Impl, IMPL_OFFSET>, NotifyServiceStateChange::<Impl, IMPL_OFFSET>, NotifySessionOfServiceStart::<Impl, IMPL_OFFSET>, NotifySessionOfServiceStop::<Impl, IMPL_OFFSET>, NotifySessionStateChange::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWTSProtocolManager as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IWTSProtocolShadowCallbackImpl: Sized {
    fn StopShadow();
    fn InvokeTargetShadow();
}
#[cfg(feature = "Win32_Foundation")]
impl IWTSProtocolShadowCallbackVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWTSProtocolShadowCallbackImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWTSProtocolShadowCallbackVtbl {
        unsafe extern "system" fn StopShadow<Impl: IWTSProtocolShadowCallbackImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn InvokeTargetShadow<Impl: IWTSProtocolShadowCallbackImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ptargetservername: super::super::Foundation::PWSTR, targetsessionid: u32, pparam1: *const u8, param1size: u32, pparam2: *const u8, param2size: u32, pparam3: *const u8, param3size: u32, pparam4: *const u8, param4size: u32, pclientname: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, StopShadow::<Impl, IMPL_OFFSET>, InvokeTargetShadow::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWTSProtocolShadowCallback as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IWTSProtocolShadowConnectionImpl: Sized {
    fn Start();
    fn Stop();
    fn DoTarget();
}
#[cfg(feature = "Win32_Foundation")]
impl IWTSProtocolShadowConnectionVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWTSProtocolShadowConnectionImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWTSProtocolShadowConnectionVtbl {
        unsafe extern "system" fn Start<Impl: IWTSProtocolShadowConnectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ptargetservername: super::super::Foundation::PWSTR, targetsessionid: u32, hotkeyvk: u8, hotkeymodifiers: u16, pshadowcallback: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Stop<Impl: IWTSProtocolShadowConnectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn DoTarget<Impl: IWTSProtocolShadowConnectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pparam1: *const u8, param1size: u32, pparam2: *const u8, param2size: u32, pparam3: *const u8, param3size: u32, pparam4: *const u8, param4size: u32, pclientname: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, Start::<Impl, IMPL_OFFSET>, Stop::<Impl, IMPL_OFFSET>, DoTarget::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWTSProtocolShadowConnection as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IWTSSBPluginImpl: Sized {
    fn Initialize();
    fn WTSSBX_MachineChangeNotification();
    fn WTSSBX_SessionChangeNotification();
    fn WTSSBX_GetMostSuitableServer();
    fn Terminated();
    fn WTSSBX_GetUserExternalSession();
}
#[cfg(feature = "Win32_Foundation")]
impl IWTSSBPluginVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWTSSBPluginImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWTSSBPluginVtbl {
        unsafe extern "system" fn Initialize<Impl: IWTSSBPluginImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plugincapabilities: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn WTSSBX_MachineChangeNotification<Impl: IWTSSBPluginImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, notificationtype: WTSSBX_NOTIFICATION_TYPE, machineid: i32, pmachineinfo: *const WTSSBX_MACHINE_INFO) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn WTSSBX_SessionChangeNotification<Impl: IWTSSBPluginImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, notificationtype: WTSSBX_NOTIFICATION_TYPE, machineid: i32, numofsessions: u32, sessioninfo: *const WTSSBX_SESSION_INFO) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn WTSSBX_GetMostSuitableServer<Impl: IWTSSBPluginImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, username: super::super::Foundation::PWSTR, domainname: super::super::Foundation::PWSTR, applicationtype: super::super::Foundation::PWSTR, farmname: super::super::Foundation::PWSTR, pmachineid: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Terminated<Impl: IWTSSBPluginImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn WTSSBX_GetUserExternalSession<Impl: IWTSSBPluginImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, username: super::super::Foundation::PWSTR, domainname: super::super::Foundation::PWSTR, applicationtype: super::super::Foundation::PWSTR, redirectorinternalip: *const WTSSBX_IP_ADDRESS, psessionid: *mut u32, pmachineconnectinfo: *mut WTSSBX_MACHINE_CONNECT_INFO) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, Initialize::<Impl, IMPL_OFFSET>, WTSSBX_MachineChangeNotification::<Impl, IMPL_OFFSET>, WTSSBX_SessionChangeNotification::<Impl, IMPL_OFFSET>, WTSSBX_GetMostSuitableServer::<Impl, IMPL_OFFSET>, Terminated::<Impl, IMPL_OFFSET>, WTSSBX_GetUserExternalSession::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWTSSBPlugin as ::windows::core::Interface>::IID
    }
}
pub trait IWTSVirtualChannelImpl: Sized {
    fn Write();
    fn Close();
}
impl IWTSVirtualChannelVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWTSVirtualChannelImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWTSVirtualChannelVtbl {
        unsafe extern "system" fn Write<Impl: IWTSVirtualChannelImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cbsize: u32, pbuffer: *const u8, preserved: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Close<Impl: IWTSVirtualChannelImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, Write::<Impl, IMPL_OFFSET>, Close::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWTSVirtualChannel as ::windows::core::Interface>::IID
    }
}
pub trait IWTSVirtualChannelCallbackImpl: Sized {
    fn OnDataReceived();
    fn OnClose();
}
impl IWTSVirtualChannelCallbackVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWTSVirtualChannelCallbackImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWTSVirtualChannelCallbackVtbl {
        unsafe extern "system" fn OnDataReceived<Impl: IWTSVirtualChannelCallbackImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cbsize: u32, pbuffer: *const u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn OnClose<Impl: IWTSVirtualChannelCallbackImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, OnDataReceived::<Impl, IMPL_OFFSET>, OnClose::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWTSVirtualChannelCallback as ::windows::core::Interface>::IID
    }
}
pub trait IWTSVirtualChannelManagerImpl: Sized {
    fn CreateListener();
}
impl IWTSVirtualChannelManagerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWTSVirtualChannelManagerImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWTSVirtualChannelManagerVtbl {
        unsafe extern "system" fn CreateListener<Impl: IWTSVirtualChannelManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszchannelname: *const u8, uflags: u32, plistenercallback: ::windows::core::RawPtr, pplistener: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, CreateListener::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWTSVirtualChannelManager as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub trait IWorkspaceImpl: Sized {
    fn GetWorkspaceNames();
    fn StartRemoteApplication();
    fn GetProcessId();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl IWorkspaceVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWorkspaceImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWorkspaceVtbl {
        unsafe extern "system" fn GetWorkspaceNames<Impl: IWorkspaceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psawkspnames: *mut *mut super::Com::SAFEARRAY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn StartRemoteApplication<Impl: IWorkspaceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrworkspaceid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, psaparams: *const super::Com::SAFEARRAY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetProcessId<Impl: IWorkspaceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pulprocessid: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetWorkspaceNames::<Impl, IMPL_OFFSET>, StartRemoteApplication::<Impl, IMPL_OFFSET>, GetProcessId::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWorkspace as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub trait IWorkspace2Impl: Sized + IWorkspaceImpl {
    fn StartRemoteApplicationEx();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl IWorkspace2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWorkspace2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWorkspace2Vtbl {
        unsafe extern "system" fn StartRemoteApplicationEx<Impl: IWorkspace2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrworkspaceid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrrequestingappid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrrequestingappfamilyname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, blaunchintoimmersiveclient: i16, bstrimmersiveclientactivationcontext: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, psaparams: *const super::Com::SAFEARRAY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetWorkspaceNames::<Impl, IMPL_OFFSET>, StartRemoteApplication::<Impl, IMPL_OFFSET>, GetProcessId::<Impl, IMPL_OFFSET>, StartRemoteApplicationEx::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWorkspace2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub trait IWorkspace3Impl: Sized + IWorkspace2Impl + IWorkspaceImpl {
    fn GetClaimsToken2();
    fn SetClaimsToken();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl IWorkspace3Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWorkspace3Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWorkspace3Vtbl {
        unsafe extern "system" fn GetClaimsToken2<Impl: IWorkspace3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrclaimshint: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstruserhint: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, claimcookie: u32, hwndcreduiparent: u32, rectcreduiparent: super::super::Foundation::RECT, pbstraccesstoken: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetClaimsToken<Impl: IWorkspace3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstraccesstoken: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ullaccesstokenexpiration: u64, bstrrefreshtoken: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetWorkspaceNames::<Impl, IMPL_OFFSET>, StartRemoteApplication::<Impl, IMPL_OFFSET>, GetProcessId::<Impl, IMPL_OFFSET>, StartRemoteApplicationEx::<Impl, IMPL_OFFSET>, GetClaimsToken2::<Impl, IMPL_OFFSET>, SetClaimsToken::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWorkspace3 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IWorkspaceClientExtImpl: Sized {
    fn GetResourceId();
    fn GetResourceDisplayName();
    fn IssueDisconnect();
}
#[cfg(feature = "Win32_Foundation")]
impl IWorkspaceClientExtVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWorkspaceClientExtImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWorkspaceClientExtVtbl {
        unsafe extern "system" fn GetResourceId<Impl: IWorkspaceClientExtImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrworkspaceid: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetResourceDisplayName<Impl: IWorkspaceClientExtImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrworkspacedisplayname: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn IssueDisconnect<Impl: IWorkspaceClientExtImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetResourceId::<Impl, IMPL_OFFSET>, GetResourceDisplayName::<Impl, IMPL_OFFSET>, IssueDisconnect::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWorkspaceClientExt as ::windows::core::Interface>::IID
    }
}
pub trait IWorkspaceRegistrationImpl: Sized {
    fn AddResource();
    fn RemoveResource();
}
impl IWorkspaceRegistrationVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWorkspaceRegistrationImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWorkspaceRegistrationVtbl {
        unsafe extern "system" fn AddResource<Impl: IWorkspaceRegistrationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, punk: ::windows::core::RawPtr, pdwcookie: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn RemoveResource<Impl: IWorkspaceRegistrationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwcookieconnection: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, AddResource::<Impl, IMPL_OFFSET>, RemoveResource::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWorkspaceRegistration as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IWorkspaceRegistration2Impl: Sized + IWorkspaceRegistrationImpl {
    fn AddResourceEx();
    fn RemoveResourceEx();
}
#[cfg(feature = "Win32_Foundation")]
impl IWorkspaceRegistration2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWorkspaceRegistration2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWorkspaceRegistration2Vtbl {
        unsafe extern "system" fn AddResourceEx<Impl: IWorkspaceRegistration2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, punk: ::windows::core::RawPtr, bstreventloguploadaddress: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pdwcookie: *mut u32, correlationid: ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn RemoveResourceEx<Impl: IWorkspaceRegistration2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwcookieconnection: u32, correlationid: ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, AddResource::<Impl, IMPL_OFFSET>, RemoveResource::<Impl, IMPL_OFFSET>, AddResourceEx::<Impl, IMPL_OFFSET>, RemoveResourceEx::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWorkspaceRegistration2 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IWorkspaceReportMessageImpl: Sized {
    fn RegisterErrorLogMessage();
    fn IsErrorMessageRegistered();
    fn RegisterErrorEvent();
}
#[cfg(feature = "Win32_Foundation")]
impl IWorkspaceReportMessageVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWorkspaceReportMessageImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWorkspaceReportMessageVtbl {
        unsafe extern "system" fn RegisterErrorLogMessage<Impl: IWorkspaceReportMessageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrmessage: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn IsErrorMessageRegistered<Impl: IWorkspaceReportMessageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrwkspid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, dwerrortype: u32, bstrerrormessagetype: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, dwerrorcode: u32, pferrorexist: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn RegisterErrorEvent<Impl: IWorkspaceReportMessageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrwkspid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, dwerrortype: u32, bstrerrormessagetype: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, dwerrorcode: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, RegisterErrorLogMessage::<Impl, IMPL_OFFSET>, IsErrorMessageRegistered::<Impl, IMPL_OFFSET>, RegisterErrorEvent::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWorkspaceReportMessage as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IWorkspaceResTypeRegistryImpl: Sized + IDispatchImpl {
    fn AddResourceType();
    fn DeleteResourceType();
    fn GetRegisteredFileExtensions();
    fn GetResourceTypeInfo();
    fn ModifyResourceType();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IWorkspaceResTypeRegistryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWorkspaceResTypeRegistryImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWorkspaceResTypeRegistryVtbl {
        unsafe extern "system" fn AddResourceType<Impl: IWorkspaceResTypeRegistryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fmachinewide: i16, bstrfileextension: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrlauncher: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn DeleteResourceType<Impl: IWorkspaceResTypeRegistryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fmachinewide: i16, bstrfileextension: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetRegisteredFileExtensions<Impl: IWorkspaceResTypeRegistryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fmachinewide: i16, psafileextensions: *mut *mut super::Com::SAFEARRAY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetResourceTypeInfo<Impl: IWorkspaceResTypeRegistryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fmachinewide: i16, bstrfileextension: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pbstrlauncher: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ModifyResourceType<Impl: IWorkspaceResTypeRegistryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fmachinewide: i16, bstrfileextension: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrlauncher: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            GetTypeInfoCount::<Impl, IMPL_OFFSET>,
            GetTypeInfo::<Impl, IMPL_OFFSET>,
            GetIDsOfNames::<Impl, IMPL_OFFSET>,
            Invoke::<Impl, IMPL_OFFSET>,
            AddResourceType::<Impl, IMPL_OFFSET>,
            DeleteResourceType::<Impl, IMPL_OFFSET>,
            GetRegisteredFileExtensions::<Impl, IMPL_OFFSET>,
            GetResourceTypeInfo::<Impl, IMPL_OFFSET>,
            ModifyResourceType::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWorkspaceResTypeRegistry as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IWorkspaceScriptableImpl: Sized + IDispatchImpl {
    fn DisconnectWorkspace();
    fn StartWorkspace();
    fn IsWorkspaceCredentialSpecified();
    fn IsWorkspaceSSOEnabled();
    fn ClearWorkspaceCredential();
    fn OnAuthenticated();
    fn DisconnectWorkspaceByFriendlyName();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IWorkspaceScriptableVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWorkspaceScriptableImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWorkspaceScriptableVtbl {
        unsafe extern "system" fn DisconnectWorkspace<Impl: IWorkspaceScriptableImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrworkspaceid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn StartWorkspace<Impl: IWorkspaceScriptableImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrworkspaceid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrusername: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrpassword: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrworkspaceparams: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ltimeout: i32, lflags: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn IsWorkspaceCredentialSpecified<Impl: IWorkspaceScriptableImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrworkspaceid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bcountunauthenticatedcredentials: i16, pbcredexist: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn IsWorkspaceSSOEnabled<Impl: IWorkspaceScriptableImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbssoenabled: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ClearWorkspaceCredential<Impl: IWorkspaceScriptableImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrworkspaceid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn OnAuthenticated<Impl: IWorkspaceScriptableImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrworkspaceid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrusername: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn DisconnectWorkspaceByFriendlyName<Impl: IWorkspaceScriptableImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrworkspacefriendlyname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            GetTypeInfoCount::<Impl, IMPL_OFFSET>,
            GetTypeInfo::<Impl, IMPL_OFFSET>,
            GetIDsOfNames::<Impl, IMPL_OFFSET>,
            Invoke::<Impl, IMPL_OFFSET>,
            DisconnectWorkspace::<Impl, IMPL_OFFSET>,
            StartWorkspace::<Impl, IMPL_OFFSET>,
            IsWorkspaceCredentialSpecified::<Impl, IMPL_OFFSET>,
            IsWorkspaceSSOEnabled::<Impl, IMPL_OFFSET>,
            ClearWorkspaceCredential::<Impl, IMPL_OFFSET>,
            OnAuthenticated::<Impl, IMPL_OFFSET>,
            DisconnectWorkspaceByFriendlyName::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWorkspaceScriptable as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IWorkspaceScriptable2Impl: Sized + IWorkspaceScriptableImpl + IDispatchImpl {
    fn StartWorkspaceEx();
    fn ResourceDismissed();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IWorkspaceScriptable2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWorkspaceScriptable2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWorkspaceScriptable2Vtbl {
        unsafe extern "system" fn StartWorkspaceEx<Impl: IWorkspaceScriptable2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrworkspaceid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrworkspacefriendlyname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrredirectorname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrusername: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrpassword: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrappcontainer: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrworkspaceparams: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ltimeout: i32, lflags: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ResourceDismissed<Impl: IWorkspaceScriptable2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrworkspaceid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrworkspacefriendlyname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            GetTypeInfoCount::<Impl, IMPL_OFFSET>,
            GetTypeInfo::<Impl, IMPL_OFFSET>,
            GetIDsOfNames::<Impl, IMPL_OFFSET>,
            Invoke::<Impl, IMPL_OFFSET>,
            DisconnectWorkspace::<Impl, IMPL_OFFSET>,
            StartWorkspace::<Impl, IMPL_OFFSET>,
            IsWorkspaceCredentialSpecified::<Impl, IMPL_OFFSET>,
            IsWorkspaceSSOEnabled::<Impl, IMPL_OFFSET>,
            ClearWorkspaceCredential::<Impl, IMPL_OFFSET>,
            OnAuthenticated::<Impl, IMPL_OFFSET>,
            DisconnectWorkspaceByFriendlyName::<Impl, IMPL_OFFSET>,
            StartWorkspaceEx::<Impl, IMPL_OFFSET>,
            ResourceDismissed::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWorkspaceScriptable2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IWorkspaceScriptable3Impl: Sized + IWorkspaceScriptable2Impl + IWorkspaceScriptableImpl + IDispatchImpl {
    fn StartWorkspaceEx2();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IWorkspaceScriptable3Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWorkspaceScriptable3Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWorkspaceScriptable3Vtbl {
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
            panic!()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            GetTypeInfoCount::<Impl, IMPL_OFFSET>,
            GetTypeInfo::<Impl, IMPL_OFFSET>,
            GetIDsOfNames::<Impl, IMPL_OFFSET>,
            Invoke::<Impl, IMPL_OFFSET>,
            DisconnectWorkspace::<Impl, IMPL_OFFSET>,
            StartWorkspace::<Impl, IMPL_OFFSET>,
            IsWorkspaceCredentialSpecified::<Impl, IMPL_OFFSET>,
            IsWorkspaceSSOEnabled::<Impl, IMPL_OFFSET>,
            ClearWorkspaceCredential::<Impl, IMPL_OFFSET>,
            OnAuthenticated::<Impl, IMPL_OFFSET>,
            DisconnectWorkspaceByFriendlyName::<Impl, IMPL_OFFSET>,
            StartWorkspaceEx::<Impl, IMPL_OFFSET>,
            ResourceDismissed::<Impl, IMPL_OFFSET>,
            StartWorkspaceEx2::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWorkspaceScriptable3 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait ItsPubPluginImpl: Sized {
    fn GetResourceList();
    fn GetResource();
    fn GetCacheLastUpdateTime();
    fn pluginName();
    fn pluginVersion();
    fn ResolveResource();
}
#[cfg(feature = "Win32_Foundation")]
impl ItsPubPluginVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ItsPubPluginImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ItsPubPluginVtbl {
        unsafe extern "system" fn GetResourceList<Impl: ItsPubPluginImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, userid: super::super::Foundation::PWSTR, pceapplistsize: *mut i32, resourcelist: *mut *mut pluginResource) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetResource<Impl: ItsPubPluginImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, alias: super::super::Foundation::PWSTR, flags: i32, resource: *mut pluginResource) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetCacheLastUpdateTime<Impl: ItsPubPluginImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lastupdatetime: *mut u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn pluginName<Impl: ItsPubPluginImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn pluginVersion<Impl: ItsPubPluginImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ResolveResource<Impl: ItsPubPluginImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, resourcetype: *mut u32, resourcelocation: super::super::Foundation::PWSTR, endpointname: super::super::Foundation::PWSTR, userid: super::super::Foundation::PWSTR, alias: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetResourceList::<Impl, IMPL_OFFSET>, GetResource::<Impl, IMPL_OFFSET>, GetCacheLastUpdateTime::<Impl, IMPL_OFFSET>, pluginName::<Impl, IMPL_OFFSET>, pluginVersion::<Impl, IMPL_OFFSET>, ResolveResource::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ItsPubPlugin as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait ItsPubPlugin2Impl: Sized + ItsPubPluginImpl {
    fn GetResource2List();
    fn GetResource2();
    fn ResolvePersonalDesktop();
    fn DeletePersonalDesktopAssignment();
}
#[cfg(feature = "Win32_Foundation")]
impl ItsPubPlugin2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ItsPubPlugin2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ItsPubPlugin2Vtbl {
        unsafe extern "system" fn GetResource2List<Impl: ItsPubPlugin2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, userid: super::super::Foundation::PWSTR, pceapplistsize: *mut i32, resourcelist: *mut *mut pluginResource2) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetResource2<Impl: ItsPubPlugin2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, alias: super::super::Foundation::PWSTR, flags: i32, resource: *mut pluginResource2) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ResolvePersonalDesktop<Impl: ItsPubPlugin2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, userid: super::super::Foundation::PWSTR, poolid: super::super::Foundation::PWSTR, epdresolutiontype: TSPUB_PLUGIN_PD_RESOLUTION_TYPE, ppdassignmenttype: *mut TSPUB_PLUGIN_PD_ASSIGNMENT_TYPE, endpointname: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn DeletePersonalDesktopAssignment<Impl: ItsPubPlugin2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, userid: super::super::Foundation::PWSTR, poolid: super::super::Foundation::PWSTR, endpointname: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            GetResourceList::<Impl, IMPL_OFFSET>,
            GetResource::<Impl, IMPL_OFFSET>,
            GetCacheLastUpdateTime::<Impl, IMPL_OFFSET>,
            pluginName::<Impl, IMPL_OFFSET>,
            pluginVersion::<Impl, IMPL_OFFSET>,
            ResolveResource::<Impl, IMPL_OFFSET>,
            GetResource2List::<Impl, IMPL_OFFSET>,
            GetResource2::<Impl, IMPL_OFFSET>,
            ResolvePersonalDesktop::<Impl, IMPL_OFFSET>,
            DeletePersonalDesktopAssignment::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ItsPubPlugin2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait _ITSWkspEventsImpl: Sized + IDispatchImpl {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl _ITSWkspEventsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: _ITSWkspEventsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> _ITSWkspEventsVtbl {
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetTypeInfoCount::<Impl, IMPL_OFFSET>, GetTypeInfo::<Impl, IMPL_OFFSET>, GetIDsOfNames::<Impl, IMPL_OFFSET>, Invoke::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<_ITSWkspEvents as ::windows::core::Interface>::IID
    }
}
