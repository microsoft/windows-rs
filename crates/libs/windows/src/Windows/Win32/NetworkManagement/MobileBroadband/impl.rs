#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IDummyMBNUCMExtImpl: Sized + IDispatchImpl {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IDummyMBNUCMExtVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDummyMBNUCMExtImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDummyMBNUCMExtVtbl {
        Self { base: IDispatchVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDummyMBNUCMExt as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IMbnConnectionImpl: Sized {
    fn ConnectionID();
    fn InterfaceID();
    fn Connect();
    fn Disconnect();
    fn GetConnectionState();
    fn GetVoiceCallState();
    fn GetActivationNetworkError();
}
#[cfg(feature = "Win32_Foundation")]
impl IMbnConnectionVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMbnConnectionImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMbnConnectionVtbl {
        unsafe extern "system" fn ConnectionID<Impl: IMbnConnectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, connectionid: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn InterfaceID<Impl: IMbnConnectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, interfaceid: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Connect<Impl: IMbnConnectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, connectionmode: MBN_CONNECTION_MODE, strprofile: super::super::Foundation::PWSTR, requestid: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Disconnect<Impl: IMbnConnectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, requestid: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetConnectionState<Impl: IMbnConnectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, connectionstate: *mut MBN_ACTIVATION_STATE, profilename: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetVoiceCallState<Impl: IMbnConnectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, voicecallstate: *mut MBN_VOICE_CALL_STATE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetActivationNetworkError<Impl: IMbnConnectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, networkerror: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            ConnectionID: ConnectionID::<Impl, IMPL_OFFSET>,
            InterfaceID: InterfaceID::<Impl, IMPL_OFFSET>,
            Connect: Connect::<Impl, IMPL_OFFSET>,
            Disconnect: Disconnect::<Impl, IMPL_OFFSET>,
            GetConnectionState: GetConnectionState::<Impl, IMPL_OFFSET>,
            GetVoiceCallState: GetVoiceCallState::<Impl, IMPL_OFFSET>,
            GetActivationNetworkError: GetActivationNetworkError::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMbnConnection as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub trait IMbnConnectionContextImpl: Sized {
    fn GetProvisionedContexts();
    fn SetProvisionedContext();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl IMbnConnectionContextVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMbnConnectionContextImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMbnConnectionContextVtbl {
        unsafe extern "system" fn GetProvisionedContexts<Impl: IMbnConnectionContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, provisionedcontexts: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetProvisionedContext<Impl: IMbnConnectionContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, provisionedcontexts: ::core::mem::ManuallyDrop<MBN_CONTEXT>, providerid: super::super::Foundation::PWSTR, requestid: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            GetProvisionedContexts: GetProvisionedContexts::<Impl, IMPL_OFFSET>,
            SetProvisionedContext: SetProvisionedContext::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMbnConnectionContext as ::windows::core::Interface>::IID
    }
}
pub trait IMbnConnectionContextEventsImpl: Sized {
    fn OnProvisionedContextListChange();
    fn OnSetProvisionedContextComplete();
}
impl IMbnConnectionContextEventsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMbnConnectionContextEventsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMbnConnectionContextEventsVtbl {
        unsafe extern "system" fn OnProvisionedContextListChange<Impl: IMbnConnectionContextEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newinterface: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn OnSetProvisionedContextComplete<Impl: IMbnConnectionContextEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newinterface: ::windows::core::RawPtr, requestid: u32, status: ::windows::core::HRESULT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            OnProvisionedContextListChange: OnProvisionedContextListChange::<Impl, IMPL_OFFSET>,
            OnSetProvisionedContextComplete: OnSetProvisionedContextComplete::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMbnConnectionContextEvents as ::windows::core::Interface>::IID
    }
}
pub trait IMbnConnectionEventsImpl: Sized {
    fn OnConnectComplete();
    fn OnDisconnectComplete();
    fn OnConnectStateChange();
    fn OnVoiceCallStateChange();
}
impl IMbnConnectionEventsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMbnConnectionEventsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMbnConnectionEventsVtbl {
        unsafe extern "system" fn OnConnectComplete<Impl: IMbnConnectionEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newconnection: ::windows::core::RawPtr, requestid: u32, status: ::windows::core::HRESULT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn OnDisconnectComplete<Impl: IMbnConnectionEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newconnection: ::windows::core::RawPtr, requestid: u32, status: ::windows::core::HRESULT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn OnConnectStateChange<Impl: IMbnConnectionEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newconnection: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn OnVoiceCallStateChange<Impl: IMbnConnectionEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newconnection: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            OnConnectComplete: OnConnectComplete::<Impl, IMPL_OFFSET>,
            OnDisconnectComplete: OnDisconnectComplete::<Impl, IMPL_OFFSET>,
            OnConnectStateChange: OnConnectStateChange::<Impl, IMPL_OFFSET>,
            OnVoiceCallStateChange: OnVoiceCallStateChange::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMbnConnectionEvents as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub trait IMbnConnectionManagerImpl: Sized {
    fn GetConnection();
    fn GetConnections();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl IMbnConnectionManagerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMbnConnectionManagerImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMbnConnectionManagerVtbl {
        unsafe extern "system" fn GetConnection<Impl: IMbnConnectionManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, connectionid: super::super::Foundation::PWSTR, mbnconnection: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetConnections<Impl: IMbnConnectionManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, mbnconnections: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            GetConnection: GetConnection::<Impl, IMPL_OFFSET>,
            GetConnections: GetConnections::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMbnConnectionManager as ::windows::core::Interface>::IID
    }
}
pub trait IMbnConnectionManagerEventsImpl: Sized {
    fn OnConnectionArrival();
    fn OnConnectionRemoval();
}
impl IMbnConnectionManagerEventsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMbnConnectionManagerEventsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMbnConnectionManagerEventsVtbl {
        unsafe extern "system" fn OnConnectionArrival<Impl: IMbnConnectionManagerEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newconnection: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn OnConnectionRemoval<Impl: IMbnConnectionManagerEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, oldconnection: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            OnConnectionArrival: OnConnectionArrival::<Impl, IMPL_OFFSET>,
            OnConnectionRemoval: OnConnectionRemoval::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMbnConnectionManagerEvents as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IMbnConnectionProfileImpl: Sized {
    fn GetProfileXmlData();
    fn UpdateProfile();
    fn Delete();
}
#[cfg(feature = "Win32_Foundation")]
impl IMbnConnectionProfileVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMbnConnectionProfileImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMbnConnectionProfileVtbl {
        unsafe extern "system" fn GetProfileXmlData<Impl: IMbnConnectionProfileImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, profiledata: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn UpdateProfile<Impl: IMbnConnectionProfileImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strprofile: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Delete<Impl: IMbnConnectionProfileImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            GetProfileXmlData: GetProfileXmlData::<Impl, IMPL_OFFSET>,
            UpdateProfile: UpdateProfile::<Impl, IMPL_OFFSET>,
            Delete: Delete::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMbnConnectionProfile as ::windows::core::Interface>::IID
    }
}
pub trait IMbnConnectionProfileEventsImpl: Sized {
    fn OnProfileUpdate();
}
impl IMbnConnectionProfileEventsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMbnConnectionProfileEventsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMbnConnectionProfileEventsVtbl {
        unsafe extern "system" fn OnProfileUpdate<Impl: IMbnConnectionProfileEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newprofile: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), OnProfileUpdate: OnProfileUpdate::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMbnConnectionProfileEvents as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub trait IMbnConnectionProfileManagerImpl: Sized {
    fn GetConnectionProfiles();
    fn GetConnectionProfile();
    fn CreateConnectionProfile();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl IMbnConnectionProfileManagerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMbnConnectionProfileManagerImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMbnConnectionProfileManagerVtbl {
        unsafe extern "system" fn GetConnectionProfiles<Impl: IMbnConnectionProfileManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, mbninterface: ::windows::core::RawPtr, connectionprofiles: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetConnectionProfile<Impl: IMbnConnectionProfileManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, mbninterface: ::windows::core::RawPtr, profilename: super::super::Foundation::PWSTR, connectionprofile: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CreateConnectionProfile<Impl: IMbnConnectionProfileManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, xmlprofile: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            GetConnectionProfiles: GetConnectionProfiles::<Impl, IMPL_OFFSET>,
            GetConnectionProfile: GetConnectionProfile::<Impl, IMPL_OFFSET>,
            CreateConnectionProfile: CreateConnectionProfile::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMbnConnectionProfileManager as ::windows::core::Interface>::IID
    }
}
pub trait IMbnConnectionProfileManagerEventsImpl: Sized {
    fn OnConnectionProfileArrival();
    fn OnConnectionProfileRemoval();
}
impl IMbnConnectionProfileManagerEventsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMbnConnectionProfileManagerEventsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMbnConnectionProfileManagerEventsVtbl {
        unsafe extern "system" fn OnConnectionProfileArrival<Impl: IMbnConnectionProfileManagerEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newconnectionprofile: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn OnConnectionProfileRemoval<Impl: IMbnConnectionProfileManagerEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, oldconnectionprofile: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            OnConnectionProfileArrival: OnConnectionProfileArrival::<Impl, IMPL_OFFSET>,
            OnConnectionProfileRemoval: OnConnectionProfileRemoval::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMbnConnectionProfileManagerEvents as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub trait IMbnDeviceServiceImpl: Sized {
    fn QuerySupportedCommands();
    fn OpenCommandSession();
    fn CloseCommandSession();
    fn SetCommand();
    fn QueryCommand();
    fn OpenDataSession();
    fn CloseDataSession();
    fn WriteData();
    fn InterfaceID();
    fn DeviceServiceID();
    fn IsCommandSessionOpen();
    fn IsDataSessionOpen();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl IMbnDeviceServiceVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMbnDeviceServiceImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMbnDeviceServiceVtbl {
        unsafe extern "system" fn QuerySupportedCommands<Impl: IMbnDeviceServiceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, requestid: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn OpenCommandSession<Impl: IMbnDeviceServiceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, requestid: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CloseCommandSession<Impl: IMbnDeviceServiceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, requestid: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetCommand<Impl: IMbnDeviceServiceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, commandid: u32, deviceservicedata: *const super::super::System::Com::SAFEARRAY, requestid: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn QueryCommand<Impl: IMbnDeviceServiceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, commandid: u32, deviceservicedata: *const super::super::System::Com::SAFEARRAY, requestid: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn OpenDataSession<Impl: IMbnDeviceServiceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, requestid: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CloseDataSession<Impl: IMbnDeviceServiceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, requestid: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn WriteData<Impl: IMbnDeviceServiceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, deviceservicedata: *const super::super::System::Com::SAFEARRAY, requestid: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn InterfaceID<Impl: IMbnDeviceServiceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, interfaceid: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn DeviceServiceID<Impl: IMbnDeviceServiceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, deviceserviceid: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn IsCommandSessionOpen<Impl: IMbnDeviceServiceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn IsDataSessionOpen<Impl: IMbnDeviceServiceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            QuerySupportedCommands: QuerySupportedCommands::<Impl, IMPL_OFFSET>,
            OpenCommandSession: OpenCommandSession::<Impl, IMPL_OFFSET>,
            CloseCommandSession: CloseCommandSession::<Impl, IMPL_OFFSET>,
            SetCommand: SetCommand::<Impl, IMPL_OFFSET>,
            QueryCommand: QueryCommand::<Impl, IMPL_OFFSET>,
            OpenDataSession: OpenDataSession::<Impl, IMPL_OFFSET>,
            CloseDataSession: CloseDataSession::<Impl, IMPL_OFFSET>,
            WriteData: WriteData::<Impl, IMPL_OFFSET>,
            InterfaceID: InterfaceID::<Impl, IMPL_OFFSET>,
            DeviceServiceID: DeviceServiceID::<Impl, IMPL_OFFSET>,
            IsCommandSessionOpen: IsCommandSessionOpen::<Impl, IMPL_OFFSET>,
            IsDataSessionOpen: IsDataSessionOpen::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMbnDeviceService as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IMbnDeviceServiceStateEventsImpl: Sized {
    fn OnSessionsStateChange();
}
#[cfg(feature = "Win32_Foundation")]
impl IMbnDeviceServiceStateEventsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMbnDeviceServiceStateEventsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMbnDeviceServiceStateEventsVtbl {
        unsafe extern "system" fn OnSessionsStateChange<Impl: IMbnDeviceServiceStateEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, interfaceid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, statechange: MBN_DEVICE_SERVICE_SESSIONS_STATE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), OnSessionsStateChange: OnSessionsStateChange::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMbnDeviceServiceStateEvents as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub trait IMbnDeviceServicesContextImpl: Sized {
    fn EnumerateDeviceServices();
    fn GetDeviceService();
    fn MaxCommandSize();
    fn MaxDataSize();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl IMbnDeviceServicesContextVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMbnDeviceServicesContextImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMbnDeviceServicesContextVtbl {
        unsafe extern "system" fn EnumerateDeviceServices<Impl: IMbnDeviceServicesContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, deviceservices: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetDeviceService<Impl: IMbnDeviceServicesContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, deviceserviceid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, mbndeviceservice: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn MaxCommandSize<Impl: IMbnDeviceServicesContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, maxcommandsize: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn MaxDataSize<Impl: IMbnDeviceServicesContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, maxdatasize: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            EnumerateDeviceServices: EnumerateDeviceServices::<Impl, IMPL_OFFSET>,
            GetDeviceService: GetDeviceService::<Impl, IMPL_OFFSET>,
            MaxCommandSize: MaxCommandSize::<Impl, IMPL_OFFSET>,
            MaxDataSize: MaxDataSize::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMbnDeviceServicesContext as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub trait IMbnDeviceServicesEventsImpl: Sized {
    fn OnQuerySupportedCommandsComplete();
    fn OnOpenCommandSessionComplete();
    fn OnCloseCommandSessionComplete();
    fn OnSetCommandComplete();
    fn OnQueryCommandComplete();
    fn OnEventNotification();
    fn OnOpenDataSessionComplete();
    fn OnCloseDataSessionComplete();
    fn OnWriteDataComplete();
    fn OnReadData();
    fn OnInterfaceStateChange();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl IMbnDeviceServicesEventsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMbnDeviceServicesEventsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMbnDeviceServicesEventsVtbl {
        unsafe extern "system" fn OnQuerySupportedCommandsComplete<Impl: IMbnDeviceServicesEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, deviceservice: ::windows::core::RawPtr, commandidlist: *const super::super::System::Com::SAFEARRAY, status: ::windows::core::HRESULT, requestid: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn OnOpenCommandSessionComplete<Impl: IMbnDeviceServicesEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, deviceservice: ::windows::core::RawPtr, status: ::windows::core::HRESULT, requestid: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn OnCloseCommandSessionComplete<Impl: IMbnDeviceServicesEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, deviceservice: ::windows::core::RawPtr, status: ::windows::core::HRESULT, requestid: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn OnSetCommandComplete<Impl: IMbnDeviceServicesEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, deviceservice: ::windows::core::RawPtr, responseid: u32, deviceservicedata: *const super::super::System::Com::SAFEARRAY, status: ::windows::core::HRESULT, requestid: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn OnQueryCommandComplete<Impl: IMbnDeviceServicesEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, deviceservice: ::windows::core::RawPtr, responseid: u32, deviceservicedata: *const super::super::System::Com::SAFEARRAY, status: ::windows::core::HRESULT, requestid: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn OnEventNotification<Impl: IMbnDeviceServicesEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, deviceservice: ::windows::core::RawPtr, eventid: u32, deviceservicedata: *const super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn OnOpenDataSessionComplete<Impl: IMbnDeviceServicesEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, deviceservice: ::windows::core::RawPtr, status: ::windows::core::HRESULT, requestid: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn OnCloseDataSessionComplete<Impl: IMbnDeviceServicesEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, deviceservice: ::windows::core::RawPtr, status: ::windows::core::HRESULT, requestid: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn OnWriteDataComplete<Impl: IMbnDeviceServicesEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, deviceservice: ::windows::core::RawPtr, status: ::windows::core::HRESULT, requestid: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn OnReadData<Impl: IMbnDeviceServicesEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, deviceservice: ::windows::core::RawPtr, deviceservicedata: *const super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn OnInterfaceStateChange<Impl: IMbnDeviceServicesEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, interfaceid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, statechange: MBN_DEVICE_SERVICES_INTERFACE_STATE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            OnQuerySupportedCommandsComplete: OnQuerySupportedCommandsComplete::<Impl, IMPL_OFFSET>,
            OnOpenCommandSessionComplete: OnOpenCommandSessionComplete::<Impl, IMPL_OFFSET>,
            OnCloseCommandSessionComplete: OnCloseCommandSessionComplete::<Impl, IMPL_OFFSET>,
            OnSetCommandComplete: OnSetCommandComplete::<Impl, IMPL_OFFSET>,
            OnQueryCommandComplete: OnQueryCommandComplete::<Impl, IMPL_OFFSET>,
            OnEventNotification: OnEventNotification::<Impl, IMPL_OFFSET>,
            OnOpenDataSessionComplete: OnOpenDataSessionComplete::<Impl, IMPL_OFFSET>,
            OnCloseDataSessionComplete: OnCloseDataSessionComplete::<Impl, IMPL_OFFSET>,
            OnWriteDataComplete: OnWriteDataComplete::<Impl, IMPL_OFFSET>,
            OnReadData: OnReadData::<Impl, IMPL_OFFSET>,
            OnInterfaceStateChange: OnInterfaceStateChange::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMbnDeviceServicesEvents as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IMbnDeviceServicesManagerImpl: Sized {
    fn GetDeviceServicesContext();
}
#[cfg(feature = "Win32_Foundation")]
impl IMbnDeviceServicesManagerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMbnDeviceServicesManagerImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMbnDeviceServicesManagerVtbl {
        unsafe extern "system" fn GetDeviceServicesContext<Impl: IMbnDeviceServicesManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, networkinterfaceid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, mbndevicescontext: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), GetDeviceServicesContext: GetDeviceServicesContext::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMbnDeviceServicesManager as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub trait IMbnInterfaceImpl: Sized {
    fn InterfaceID();
    fn GetInterfaceCapability();
    fn GetSubscriberInformation();
    fn GetReadyState();
    fn InEmergencyMode();
    fn GetHomeProvider();
    fn GetPreferredProviders();
    fn SetPreferredProviders();
    fn GetVisibleProviders();
    fn ScanNetwork();
    fn GetConnection();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl IMbnInterfaceVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMbnInterfaceImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMbnInterfaceVtbl {
        unsafe extern "system" fn InterfaceID<Impl: IMbnInterfaceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, interfaceid: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetInterfaceCapability<Impl: IMbnInterfaceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, interfacecaps: *mut MBN_INTERFACE_CAPS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetSubscriberInformation<Impl: IMbnInterfaceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, subscriberinformation: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetReadyState<Impl: IMbnInterfaceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, readystate: *mut MBN_READY_STATE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn InEmergencyMode<Impl: IMbnInterfaceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, emergencymode: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetHomeProvider<Impl: IMbnInterfaceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, homeprovider: *mut MBN_PROVIDER) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetPreferredProviders<Impl: IMbnInterfaceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, preferredproviders: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetPreferredProviders<Impl: IMbnInterfaceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, preferredproviders: *const super::super::System::Com::SAFEARRAY, requestid: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetVisibleProviders<Impl: IMbnInterfaceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, age: *mut u32, visibleproviders: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ScanNetwork<Impl: IMbnInterfaceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, requestid: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetConnection<Impl: IMbnInterfaceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, mbnconnection: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            InterfaceID: InterfaceID::<Impl, IMPL_OFFSET>,
            GetInterfaceCapability: GetInterfaceCapability::<Impl, IMPL_OFFSET>,
            GetSubscriberInformation: GetSubscriberInformation::<Impl, IMPL_OFFSET>,
            GetReadyState: GetReadyState::<Impl, IMPL_OFFSET>,
            InEmergencyMode: InEmergencyMode::<Impl, IMPL_OFFSET>,
            GetHomeProvider: GetHomeProvider::<Impl, IMPL_OFFSET>,
            GetPreferredProviders: GetPreferredProviders::<Impl, IMPL_OFFSET>,
            SetPreferredProviders: SetPreferredProviders::<Impl, IMPL_OFFSET>,
            GetVisibleProviders: GetVisibleProviders::<Impl, IMPL_OFFSET>,
            ScanNetwork: ScanNetwork::<Impl, IMPL_OFFSET>,
            GetConnection: GetConnection::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMbnInterface as ::windows::core::Interface>::IID
    }
}
pub trait IMbnInterfaceEventsImpl: Sized {
    fn OnInterfaceCapabilityAvailable();
    fn OnSubscriberInformationChange();
    fn OnReadyStateChange();
    fn OnEmergencyModeChange();
    fn OnHomeProviderAvailable();
    fn OnPreferredProvidersChange();
    fn OnSetPreferredProvidersComplete();
    fn OnScanNetworkComplete();
}
impl IMbnInterfaceEventsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMbnInterfaceEventsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMbnInterfaceEventsVtbl {
        unsafe extern "system" fn OnInterfaceCapabilityAvailable<Impl: IMbnInterfaceEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newinterface: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn OnSubscriberInformationChange<Impl: IMbnInterfaceEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newinterface: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn OnReadyStateChange<Impl: IMbnInterfaceEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newinterface: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn OnEmergencyModeChange<Impl: IMbnInterfaceEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newinterface: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn OnHomeProviderAvailable<Impl: IMbnInterfaceEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newinterface: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn OnPreferredProvidersChange<Impl: IMbnInterfaceEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newinterface: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn OnSetPreferredProvidersComplete<Impl: IMbnInterfaceEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newinterface: ::windows::core::RawPtr, requestid: u32, status: ::windows::core::HRESULT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn OnScanNetworkComplete<Impl: IMbnInterfaceEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newinterface: ::windows::core::RawPtr, requestid: u32, status: ::windows::core::HRESULT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            OnInterfaceCapabilityAvailable: OnInterfaceCapabilityAvailable::<Impl, IMPL_OFFSET>,
            OnSubscriberInformationChange: OnSubscriberInformationChange::<Impl, IMPL_OFFSET>,
            OnReadyStateChange: OnReadyStateChange::<Impl, IMPL_OFFSET>,
            OnEmergencyModeChange: OnEmergencyModeChange::<Impl, IMPL_OFFSET>,
            OnHomeProviderAvailable: OnHomeProviderAvailable::<Impl, IMPL_OFFSET>,
            OnPreferredProvidersChange: OnPreferredProvidersChange::<Impl, IMPL_OFFSET>,
            OnSetPreferredProvidersComplete: OnSetPreferredProvidersComplete::<Impl, IMPL_OFFSET>,
            OnScanNetworkComplete: OnScanNetworkComplete::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMbnInterfaceEvents as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub trait IMbnInterfaceManagerImpl: Sized {
    fn GetInterface();
    fn GetInterfaces();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl IMbnInterfaceManagerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMbnInterfaceManagerImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMbnInterfaceManagerVtbl {
        unsafe extern "system" fn GetInterface<Impl: IMbnInterfaceManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, interfaceid: super::super::Foundation::PWSTR, mbninterface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetInterfaces<Impl: IMbnInterfaceManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, mbninterfaces: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            GetInterface: GetInterface::<Impl, IMPL_OFFSET>,
            GetInterfaces: GetInterfaces::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMbnInterfaceManager as ::windows::core::Interface>::IID
    }
}
pub trait IMbnInterfaceManagerEventsImpl: Sized {
    fn OnInterfaceArrival();
    fn OnInterfaceRemoval();
}
impl IMbnInterfaceManagerEventsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMbnInterfaceManagerEventsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMbnInterfaceManagerEventsVtbl {
        unsafe extern "system" fn OnInterfaceArrival<Impl: IMbnInterfaceManagerEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newinterface: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn OnInterfaceRemoval<Impl: IMbnInterfaceManagerEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, oldinterface: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            OnInterfaceArrival: OnInterfaceArrival::<Impl, IMPL_OFFSET>,
            OnInterfaceRemoval: OnInterfaceRemoval::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMbnInterfaceManagerEvents as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub trait IMbnMultiCarrierImpl: Sized {
    fn SetHomeProvider();
    fn GetPreferredProviders();
    fn GetVisibleProviders();
    fn GetSupportedCellularClasses();
    fn GetCurrentCellularClass();
    fn ScanNetwork();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl IMbnMultiCarrierVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMbnMultiCarrierImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMbnMultiCarrierVtbl {
        unsafe extern "system" fn SetHomeProvider<Impl: IMbnMultiCarrierImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, homeprovider: *const MBN_PROVIDER2, requestid: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetPreferredProviders<Impl: IMbnMultiCarrierImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, preferredmulticarrierproviders: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetVisibleProviders<Impl: IMbnMultiCarrierImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, age: *mut u32, visibleproviders: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetSupportedCellularClasses<Impl: IMbnMultiCarrierImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cellularclasses: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetCurrentCellularClass<Impl: IMbnMultiCarrierImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, currentcellularclass: *mut MBN_CELLULAR_CLASS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ScanNetwork<Impl: IMbnMultiCarrierImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, requestid: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            SetHomeProvider: SetHomeProvider::<Impl, IMPL_OFFSET>,
            GetPreferredProviders: GetPreferredProviders::<Impl, IMPL_OFFSET>,
            GetVisibleProviders: GetVisibleProviders::<Impl, IMPL_OFFSET>,
            GetSupportedCellularClasses: GetSupportedCellularClasses::<Impl, IMPL_OFFSET>,
            GetCurrentCellularClass: GetCurrentCellularClass::<Impl, IMPL_OFFSET>,
            ScanNetwork: ScanNetwork::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMbnMultiCarrier as ::windows::core::Interface>::IID
    }
}
pub trait IMbnMultiCarrierEventsImpl: Sized {
    fn OnSetHomeProviderComplete();
    fn OnCurrentCellularClassChange();
    fn OnPreferredProvidersChange();
    fn OnScanNetworkComplete();
    fn OnInterfaceCapabilityChange();
}
impl IMbnMultiCarrierEventsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMbnMultiCarrierEventsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMbnMultiCarrierEventsVtbl {
        unsafe extern "system" fn OnSetHomeProviderComplete<Impl: IMbnMultiCarrierEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, mbninterface: ::windows::core::RawPtr, requestid: u32, status: ::windows::core::HRESULT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn OnCurrentCellularClassChange<Impl: IMbnMultiCarrierEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, mbninterface: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn OnPreferredProvidersChange<Impl: IMbnMultiCarrierEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, mbninterface: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn OnScanNetworkComplete<Impl: IMbnMultiCarrierEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, mbninterface: ::windows::core::RawPtr, requestid: u32, status: ::windows::core::HRESULT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn OnInterfaceCapabilityChange<Impl: IMbnMultiCarrierEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, mbninterface: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            OnSetHomeProviderComplete: OnSetHomeProviderComplete::<Impl, IMPL_OFFSET>,
            OnCurrentCellularClassChange: OnCurrentCellularClassChange::<Impl, IMPL_OFFSET>,
            OnPreferredProvidersChange: OnPreferredProvidersChange::<Impl, IMPL_OFFSET>,
            OnScanNetworkComplete: OnScanNetworkComplete::<Impl, IMPL_OFFSET>,
            OnInterfaceCapabilityChange: OnInterfaceCapabilityChange::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMbnMultiCarrierEvents as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IMbnPinImpl: Sized {
    fn PinType();
    fn PinFormat();
    fn PinLengthMin();
    fn PinLengthMax();
    fn PinMode();
    fn Enable();
    fn Disable();
    fn Enter();
    fn Change();
    fn Unblock();
    fn GetPinManager();
}
#[cfg(feature = "Win32_Foundation")]
impl IMbnPinVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMbnPinImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMbnPinVtbl {
        unsafe extern "system" fn PinType<Impl: IMbnPinImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pintype: *mut MBN_PIN_TYPE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn PinFormat<Impl: IMbnPinImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinformat: *mut MBN_PIN_FORMAT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn PinLengthMin<Impl: IMbnPinImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinlengthmin: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn PinLengthMax<Impl: IMbnPinImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinlengthmax: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn PinMode<Impl: IMbnPinImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinmode: *mut MBN_PIN_MODE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Enable<Impl: IMbnPinImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pin: super::super::Foundation::PWSTR, requestid: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Disable<Impl: IMbnPinImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pin: super::super::Foundation::PWSTR, requestid: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Enter<Impl: IMbnPinImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pin: super::super::Foundation::PWSTR, requestid: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Change<Impl: IMbnPinImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pin: super::super::Foundation::PWSTR, newpin: super::super::Foundation::PWSTR, requestid: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Unblock<Impl: IMbnPinImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, puk: super::super::Foundation::PWSTR, newpin: super::super::Foundation::PWSTR, requestid: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetPinManager<Impl: IMbnPinImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinmanager: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            PinType: PinType::<Impl, IMPL_OFFSET>,
            PinFormat: PinFormat::<Impl, IMPL_OFFSET>,
            PinLengthMin: PinLengthMin::<Impl, IMPL_OFFSET>,
            PinLengthMax: PinLengthMax::<Impl, IMPL_OFFSET>,
            PinMode: PinMode::<Impl, IMPL_OFFSET>,
            Enable: Enable::<Impl, IMPL_OFFSET>,
            Disable: Disable::<Impl, IMPL_OFFSET>,
            Enter: Enter::<Impl, IMPL_OFFSET>,
            Change: Change::<Impl, IMPL_OFFSET>,
            Unblock: Unblock::<Impl, IMPL_OFFSET>,
            GetPinManager: GetPinManager::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMbnPin as ::windows::core::Interface>::IID
    }
}
pub trait IMbnPinEventsImpl: Sized {
    fn OnEnableComplete();
    fn OnDisableComplete();
    fn OnEnterComplete();
    fn OnChangeComplete();
    fn OnUnblockComplete();
}
impl IMbnPinEventsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMbnPinEventsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMbnPinEventsVtbl {
        unsafe extern "system" fn OnEnableComplete<Impl: IMbnPinEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pin: ::windows::core::RawPtr, pininfo: *const MBN_PIN_INFO, requestid: u32, status: ::windows::core::HRESULT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn OnDisableComplete<Impl: IMbnPinEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pin: ::windows::core::RawPtr, pininfo: *const MBN_PIN_INFO, requestid: u32, status: ::windows::core::HRESULT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn OnEnterComplete<Impl: IMbnPinEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pin: ::windows::core::RawPtr, pininfo: *const MBN_PIN_INFO, requestid: u32, status: ::windows::core::HRESULT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn OnChangeComplete<Impl: IMbnPinEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pin: ::windows::core::RawPtr, pininfo: *const MBN_PIN_INFO, requestid: u32, status: ::windows::core::HRESULT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn OnUnblockComplete<Impl: IMbnPinEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pin: ::windows::core::RawPtr, pininfo: *const MBN_PIN_INFO, requestid: u32, status: ::windows::core::HRESULT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            OnEnableComplete: OnEnableComplete::<Impl, IMPL_OFFSET>,
            OnDisableComplete: OnDisableComplete::<Impl, IMPL_OFFSET>,
            OnEnterComplete: OnEnterComplete::<Impl, IMPL_OFFSET>,
            OnChangeComplete: OnChangeComplete::<Impl, IMPL_OFFSET>,
            OnUnblockComplete: OnUnblockComplete::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMbnPinEvents as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IMbnPinManagerImpl: Sized {
    fn GetPinList();
    fn GetPin();
    fn GetPinState();
}
#[cfg(feature = "Win32_System_Com")]
impl IMbnPinManagerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMbnPinManagerImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMbnPinManagerVtbl {
        unsafe extern "system" fn GetPinList<Impl: IMbnPinManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinlist: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetPin<Impl: IMbnPinManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pintype: MBN_PIN_TYPE, pin: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetPinState<Impl: IMbnPinManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, requestid: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            GetPinList: GetPinList::<Impl, IMPL_OFFSET>,
            GetPin: GetPin::<Impl, IMPL_OFFSET>,
            GetPinState: GetPinState::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMbnPinManager as ::windows::core::Interface>::IID
    }
}
pub trait IMbnPinManagerEventsImpl: Sized {
    fn OnPinListAvailable();
    fn OnGetPinStateComplete();
}
impl IMbnPinManagerEventsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMbnPinManagerEventsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMbnPinManagerEventsVtbl {
        unsafe extern "system" fn OnPinListAvailable<Impl: IMbnPinManagerEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinmanager: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn OnGetPinStateComplete<Impl: IMbnPinManagerEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinmanager: ::windows::core::RawPtr, pininfo: MBN_PIN_INFO, requestid: u32, status: ::windows::core::HRESULT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            OnPinListAvailable: OnPinListAvailable::<Impl, IMPL_OFFSET>,
            OnGetPinStateComplete: OnGetPinStateComplete::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMbnPinManagerEvents as ::windows::core::Interface>::IID
    }
}
pub trait IMbnRadioImpl: Sized {
    fn SoftwareRadioState();
    fn HardwareRadioState();
    fn SetSoftwareRadioState();
}
impl IMbnRadioVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMbnRadioImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMbnRadioVtbl {
        unsafe extern "system" fn SoftwareRadioState<Impl: IMbnRadioImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, softwareradiostate: *mut MBN_RADIO) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn HardwareRadioState<Impl: IMbnRadioImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hardwareradiostate: *mut MBN_RADIO) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetSoftwareRadioState<Impl: IMbnRadioImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, radiostate: MBN_RADIO, requestid: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            SoftwareRadioState: SoftwareRadioState::<Impl, IMPL_OFFSET>,
            HardwareRadioState: HardwareRadioState::<Impl, IMPL_OFFSET>,
            SetSoftwareRadioState: SetSoftwareRadioState::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMbnRadio as ::windows::core::Interface>::IID
    }
}
pub trait IMbnRadioEventsImpl: Sized {
    fn OnRadioStateChange();
    fn OnSetSoftwareRadioStateComplete();
}
impl IMbnRadioEventsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMbnRadioEventsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMbnRadioEventsVtbl {
        unsafe extern "system" fn OnRadioStateChange<Impl: IMbnRadioEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newinterface: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn OnSetSoftwareRadioStateComplete<Impl: IMbnRadioEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newinterface: ::windows::core::RawPtr, requestid: u32, status: ::windows::core::HRESULT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            OnRadioStateChange: OnRadioStateChange::<Impl, IMPL_OFFSET>,
            OnSetSoftwareRadioStateComplete: OnSetSoftwareRadioStateComplete::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMbnRadioEvents as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IMbnRegistrationImpl: Sized {
    fn GetRegisterState();
    fn GetRegisterMode();
    fn GetProviderID();
    fn GetProviderName();
    fn GetRoamingText();
    fn GetAvailableDataClasses();
    fn GetCurrentDataClass();
    fn GetRegistrationNetworkError();
    fn GetPacketAttachNetworkError();
    fn SetRegisterMode();
}
#[cfg(feature = "Win32_Foundation")]
impl IMbnRegistrationVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMbnRegistrationImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMbnRegistrationVtbl {
        unsafe extern "system" fn GetRegisterState<Impl: IMbnRegistrationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, registerstate: *mut MBN_REGISTER_STATE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetRegisterMode<Impl: IMbnRegistrationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, registermode: *mut MBN_REGISTER_MODE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetProviderID<Impl: IMbnRegistrationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, providerid: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetProviderName<Impl: IMbnRegistrationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, providername: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetRoamingText<Impl: IMbnRegistrationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, roamingtext: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetAvailableDataClasses<Impl: IMbnRegistrationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, availabledataclasses: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetCurrentDataClass<Impl: IMbnRegistrationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, currentdataclass: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetRegistrationNetworkError<Impl: IMbnRegistrationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, registrationnetworkerror: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetPacketAttachNetworkError<Impl: IMbnRegistrationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, packetattachnetworkerror: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetRegisterMode<Impl: IMbnRegistrationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, registermode: MBN_REGISTER_MODE, providerid: super::super::Foundation::PWSTR, dataclass: u32, requestid: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            GetRegisterState: GetRegisterState::<Impl, IMPL_OFFSET>,
            GetRegisterMode: GetRegisterMode::<Impl, IMPL_OFFSET>,
            GetProviderID: GetProviderID::<Impl, IMPL_OFFSET>,
            GetProviderName: GetProviderName::<Impl, IMPL_OFFSET>,
            GetRoamingText: GetRoamingText::<Impl, IMPL_OFFSET>,
            GetAvailableDataClasses: GetAvailableDataClasses::<Impl, IMPL_OFFSET>,
            GetCurrentDataClass: GetCurrentDataClass::<Impl, IMPL_OFFSET>,
            GetRegistrationNetworkError: GetRegistrationNetworkError::<Impl, IMPL_OFFSET>,
            GetPacketAttachNetworkError: GetPacketAttachNetworkError::<Impl, IMPL_OFFSET>,
            SetRegisterMode: SetRegisterMode::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMbnRegistration as ::windows::core::Interface>::IID
    }
}
pub trait IMbnRegistrationEventsImpl: Sized {
    fn OnRegisterModeAvailable();
    fn OnRegisterStateChange();
    fn OnPacketServiceStateChange();
    fn OnSetRegisterModeComplete();
}
impl IMbnRegistrationEventsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMbnRegistrationEventsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMbnRegistrationEventsVtbl {
        unsafe extern "system" fn OnRegisterModeAvailable<Impl: IMbnRegistrationEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newinterface: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn OnRegisterStateChange<Impl: IMbnRegistrationEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newinterface: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn OnPacketServiceStateChange<Impl: IMbnRegistrationEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newinterface: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn OnSetRegisterModeComplete<Impl: IMbnRegistrationEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newinterface: ::windows::core::RawPtr, requestid: u32, status: ::windows::core::HRESULT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            OnRegisterModeAvailable: OnRegisterModeAvailable::<Impl, IMPL_OFFSET>,
            OnRegisterStateChange: OnRegisterStateChange::<Impl, IMPL_OFFSET>,
            OnPacketServiceStateChange: OnPacketServiceStateChange::<Impl, IMPL_OFFSET>,
            OnSetRegisterModeComplete: OnSetRegisterModeComplete::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMbnRegistrationEvents as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IMbnServiceActivationImpl: Sized {
    fn Activate();
}
#[cfg(feature = "Win32_System_Com")]
impl IMbnServiceActivationVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMbnServiceActivationImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMbnServiceActivationVtbl {
        unsafe extern "system" fn Activate<Impl: IMbnServiceActivationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, vendorspecificdata: *const super::super::System::Com::SAFEARRAY, requestid: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), Activate: Activate::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMbnServiceActivation as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IMbnServiceActivationEventsImpl: Sized {
    fn OnActivationComplete();
}
#[cfg(feature = "Win32_System_Com")]
impl IMbnServiceActivationEventsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMbnServiceActivationEventsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMbnServiceActivationEventsVtbl {
        unsafe extern "system" fn OnActivationComplete<Impl: IMbnServiceActivationEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, serviceactivation: ::windows::core::RawPtr, vendorspecificdata: *const super::super::System::Com::SAFEARRAY, requestid: u32, status: ::windows::core::HRESULT, networkerror: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), OnActivationComplete: OnActivationComplete::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMbnServiceActivationEvents as ::windows::core::Interface>::IID
    }
}
pub trait IMbnSignalImpl: Sized {
    fn GetSignalStrength();
    fn GetSignalError();
}
impl IMbnSignalVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMbnSignalImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMbnSignalVtbl {
        unsafe extern "system" fn GetSignalStrength<Impl: IMbnSignalImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, signalstrength: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetSignalError<Impl: IMbnSignalImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, signalerror: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            GetSignalStrength: GetSignalStrength::<Impl, IMPL_OFFSET>,
            GetSignalError: GetSignalError::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMbnSignal as ::windows::core::Interface>::IID
    }
}
pub trait IMbnSignalEventsImpl: Sized {
    fn OnSignalStateChange();
}
impl IMbnSignalEventsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMbnSignalEventsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMbnSignalEventsVtbl {
        unsafe extern "system" fn OnSignalStateChange<Impl: IMbnSignalEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newinterface: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), OnSignalStateChange: OnSignalStateChange::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMbnSignalEvents as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub trait IMbnSmsImpl: Sized {
    fn GetSmsConfiguration();
    fn SetSmsConfiguration();
    fn SmsSendPdu();
    fn SmsSendCdma();
    fn SmsSendCdmaPdu();
    fn SmsRead();
    fn SmsDelete();
    fn GetSmsStatus();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl IMbnSmsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMbnSmsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMbnSmsVtbl {
        unsafe extern "system" fn GetSmsConfiguration<Impl: IMbnSmsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, smsconfiguration: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetSmsConfiguration<Impl: IMbnSmsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, smsconfiguration: ::windows::core::RawPtr, requestid: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SmsSendPdu<Impl: IMbnSmsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdudata: super::super::Foundation::PWSTR, size: u8, requestid: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SmsSendCdma<Impl: IMbnSmsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, address: super::super::Foundation::PWSTR, encoding: MBN_SMS_CDMA_ENCODING, language: MBN_SMS_CDMA_LANG, sizeincharacters: u32, message: *const super::super::System::Com::SAFEARRAY, requestid: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SmsSendCdmaPdu<Impl: IMbnSmsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, message: *const super::super::System::Com::SAFEARRAY, requestid: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SmsRead<Impl: IMbnSmsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, smsfilter: *const MBN_SMS_FILTER, smsformat: MBN_SMS_FORMAT, requestid: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SmsDelete<Impl: IMbnSmsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, smsfilter: *const MBN_SMS_FILTER, requestid: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetSmsStatus<Impl: IMbnSmsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, smsstatusinfo: *mut MBN_SMS_STATUS_INFO) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            GetSmsConfiguration: GetSmsConfiguration::<Impl, IMPL_OFFSET>,
            SetSmsConfiguration: SetSmsConfiguration::<Impl, IMPL_OFFSET>,
            SmsSendPdu: SmsSendPdu::<Impl, IMPL_OFFSET>,
            SmsSendCdma: SmsSendCdma::<Impl, IMPL_OFFSET>,
            SmsSendCdmaPdu: SmsSendCdmaPdu::<Impl, IMPL_OFFSET>,
            SmsRead: SmsRead::<Impl, IMPL_OFFSET>,
            SmsDelete: SmsDelete::<Impl, IMPL_OFFSET>,
            GetSmsStatus: GetSmsStatus::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMbnSms as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IMbnSmsConfigurationImpl: Sized {
    fn ServiceCenterAddress();
    fn SetServiceCenterAddress();
    fn MaxMessageIndex();
    fn CdmaShortMsgSize();
    fn SmsFormat();
    fn SetSmsFormat();
}
#[cfg(feature = "Win32_Foundation")]
impl IMbnSmsConfigurationVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMbnSmsConfigurationImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMbnSmsConfigurationVtbl {
        unsafe extern "system" fn ServiceCenterAddress<Impl: IMbnSmsConfigurationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, scaddress: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetServiceCenterAddress<Impl: IMbnSmsConfigurationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, scaddress: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn MaxMessageIndex<Impl: IMbnSmsConfigurationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CdmaShortMsgSize<Impl: IMbnSmsConfigurationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, shortmsgsize: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SmsFormat<Impl: IMbnSmsConfigurationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, smsformat: *mut MBN_SMS_FORMAT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetSmsFormat<Impl: IMbnSmsConfigurationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, smsformat: MBN_SMS_FORMAT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            ServiceCenterAddress: ServiceCenterAddress::<Impl, IMPL_OFFSET>,
            SetServiceCenterAddress: SetServiceCenterAddress::<Impl, IMPL_OFFSET>,
            MaxMessageIndex: MaxMessageIndex::<Impl, IMPL_OFFSET>,
            CdmaShortMsgSize: CdmaShortMsgSize::<Impl, IMPL_OFFSET>,
            SmsFormat: SmsFormat::<Impl, IMPL_OFFSET>,
            SetSmsFormat: SetSmsFormat::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMbnSmsConfiguration as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IMbnSmsEventsImpl: Sized {
    fn OnSmsConfigurationChange();
    fn OnSetSmsConfigurationComplete();
    fn OnSmsSendComplete();
    fn OnSmsReadComplete();
    fn OnSmsNewClass0Message();
    fn OnSmsDeleteComplete();
    fn OnSmsStatusChange();
}
#[cfg(feature = "Win32_System_Com")]
impl IMbnSmsEventsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMbnSmsEventsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMbnSmsEventsVtbl {
        unsafe extern "system" fn OnSmsConfigurationChange<Impl: IMbnSmsEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, sms: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn OnSetSmsConfigurationComplete<Impl: IMbnSmsEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, sms: ::windows::core::RawPtr, requestid: u32, status: ::windows::core::HRESULT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn OnSmsSendComplete<Impl: IMbnSmsEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, sms: ::windows::core::RawPtr, requestid: u32, status: ::windows::core::HRESULT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn OnSmsReadComplete<Impl: IMbnSmsEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, sms: ::windows::core::RawPtr, smsformat: MBN_SMS_FORMAT, readmsgs: *const super::super::System::Com::SAFEARRAY, moremsgs: i16, requestid: u32, status: ::windows::core::HRESULT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn OnSmsNewClass0Message<Impl: IMbnSmsEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, sms: ::windows::core::RawPtr, smsformat: MBN_SMS_FORMAT, readmsgs: *const super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn OnSmsDeleteComplete<Impl: IMbnSmsEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, sms: ::windows::core::RawPtr, requestid: u32, status: ::windows::core::HRESULT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn OnSmsStatusChange<Impl: IMbnSmsEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, sms: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            OnSmsConfigurationChange: OnSmsConfigurationChange::<Impl, IMPL_OFFSET>,
            OnSetSmsConfigurationComplete: OnSetSmsConfigurationComplete::<Impl, IMPL_OFFSET>,
            OnSmsSendComplete: OnSmsSendComplete::<Impl, IMPL_OFFSET>,
            OnSmsReadComplete: OnSmsReadComplete::<Impl, IMPL_OFFSET>,
            OnSmsNewClass0Message: OnSmsNewClass0Message::<Impl, IMPL_OFFSET>,
            OnSmsDeleteComplete: OnSmsDeleteComplete::<Impl, IMPL_OFFSET>,
            OnSmsStatusChange: OnSmsStatusChange::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMbnSmsEvents as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub trait IMbnSmsReadMsgPduImpl: Sized {
    fn Index();
    fn Status();
    fn PduData();
    fn Message();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl IMbnSmsReadMsgPduVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMbnSmsReadMsgPduImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMbnSmsReadMsgPduVtbl {
        unsafe extern "system" fn Index<Impl: IMbnSmsReadMsgPduImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Status<Impl: IMbnSmsReadMsgPduImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, status: *mut MBN_MSG_STATUS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn PduData<Impl: IMbnSmsReadMsgPduImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdudata: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Message<Impl: IMbnSmsReadMsgPduImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, message: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            Index: Index::<Impl, IMPL_OFFSET>,
            Status: Status::<Impl, IMPL_OFFSET>,
            PduData: PduData::<Impl, IMPL_OFFSET>,
            Message: Message::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMbnSmsReadMsgPdu as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub trait IMbnSmsReadMsgTextCdmaImpl: Sized {
    fn Index();
    fn Status();
    fn Address();
    fn Timestamp();
    fn EncodingID();
    fn LanguageID();
    fn SizeInCharacters();
    fn Message();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl IMbnSmsReadMsgTextCdmaVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMbnSmsReadMsgTextCdmaImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMbnSmsReadMsgTextCdmaVtbl {
        unsafe extern "system" fn Index<Impl: IMbnSmsReadMsgTextCdmaImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Status<Impl: IMbnSmsReadMsgTextCdmaImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, status: *mut MBN_MSG_STATUS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Address<Impl: IMbnSmsReadMsgTextCdmaImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, address: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Timestamp<Impl: IMbnSmsReadMsgTextCdmaImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, timestamp: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn EncodingID<Impl: IMbnSmsReadMsgTextCdmaImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, encodingid: *mut MBN_SMS_CDMA_ENCODING) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn LanguageID<Impl: IMbnSmsReadMsgTextCdmaImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, languageid: *mut MBN_SMS_CDMA_LANG) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SizeInCharacters<Impl: IMbnSmsReadMsgTextCdmaImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, sizeincharacters: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Message<Impl: IMbnSmsReadMsgTextCdmaImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, message: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            Index: Index::<Impl, IMPL_OFFSET>,
            Status: Status::<Impl, IMPL_OFFSET>,
            Address: Address::<Impl, IMPL_OFFSET>,
            Timestamp: Timestamp::<Impl, IMPL_OFFSET>,
            EncodingID: EncodingID::<Impl, IMPL_OFFSET>,
            LanguageID: LanguageID::<Impl, IMPL_OFFSET>,
            SizeInCharacters: SizeInCharacters::<Impl, IMPL_OFFSET>,
            Message: Message::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMbnSmsReadMsgTextCdma as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub trait IMbnSubscriberInformationImpl: Sized {
    fn SubscriberID();
    fn SimIccID();
    fn TelephoneNumbers();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl IMbnSubscriberInformationVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMbnSubscriberInformationImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMbnSubscriberInformationVtbl {
        unsafe extern "system" fn SubscriberID<Impl: IMbnSubscriberInformationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, subscriberid: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SimIccID<Impl: IMbnSubscriberInformationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, simiccid: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn TelephoneNumbers<Impl: IMbnSubscriberInformationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, telephonenumbers: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            SubscriberID: SubscriberID::<Impl, IMPL_OFFSET>,
            SimIccID: SimIccID::<Impl, IMPL_OFFSET>,
            TelephoneNumbers: TelephoneNumbers::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMbnSubscriberInformation as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IMbnVendorSpecificEventsImpl: Sized {
    fn OnEventNotification();
    fn OnSetVendorSpecificComplete();
}
#[cfg(feature = "Win32_System_Com")]
impl IMbnVendorSpecificEventsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMbnVendorSpecificEventsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMbnVendorSpecificEventsVtbl {
        unsafe extern "system" fn OnEventNotification<Impl: IMbnVendorSpecificEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, vendoroperation: ::windows::core::RawPtr, vendorspecificdata: *const super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn OnSetVendorSpecificComplete<Impl: IMbnVendorSpecificEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, vendoroperation: ::windows::core::RawPtr, vendorspecificdata: *const super::super::System::Com::SAFEARRAY, requestid: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            OnEventNotification: OnEventNotification::<Impl, IMPL_OFFSET>,
            OnSetVendorSpecificComplete: OnSetVendorSpecificComplete::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMbnVendorSpecificEvents as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IMbnVendorSpecificOperationImpl: Sized {
    fn SetVendorSpecific();
}
#[cfg(feature = "Win32_System_Com")]
impl IMbnVendorSpecificOperationVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMbnVendorSpecificOperationImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMbnVendorSpecificOperationVtbl {
        unsafe extern "system" fn SetVendorSpecific<Impl: IMbnVendorSpecificOperationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, vendorspecificdata: *const super::super::System::Com::SAFEARRAY, requestid: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), SetVendorSpecific: SetVendorSpecific::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMbnVendorSpecificOperation as ::windows::core::Interface>::IID
    }
}
