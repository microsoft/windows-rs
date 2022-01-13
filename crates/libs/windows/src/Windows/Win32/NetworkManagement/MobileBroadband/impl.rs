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
    fn ConnectionID(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn InterfaceID(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn Connect(&mut self, connectionmode: MBN_CONNECTION_MODE, strprofile: super::super::Foundation::PWSTR) -> ::windows::core::Result<u32>;
    fn Disconnect(&mut self) -> ::windows::core::Result<u32>;
    fn GetConnectionState(&mut self, connectionstate: *mut MBN_ACTIVATION_STATE, profilename: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn GetVoiceCallState(&mut self) -> ::windows::core::Result<MBN_VOICE_CALL_STATE>;
    fn GetActivationNetworkError(&mut self) -> ::windows::core::Result<u32>;
}
#[cfg(feature = "Win32_Foundation")]
impl IMbnConnectionVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMbnConnectionImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMbnConnectionVtbl {
        unsafe extern "system" fn ConnectionID<Impl: IMbnConnectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, connectionid: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ConnectionID() {
                ::core::result::Result::Ok(ok__) => {
                    *connectionid = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn InterfaceID<Impl: IMbnConnectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, interfaceid: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).InterfaceID() {
                ::core::result::Result::Ok(ok__) => {
                    *interfaceid = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Connect<Impl: IMbnConnectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, connectionmode: MBN_CONNECTION_MODE, strprofile: super::super::Foundation::PWSTR, requestid: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Connect(::core::mem::transmute_copy(&connectionmode), ::core::mem::transmute_copy(&strprofile)) {
                ::core::result::Result::Ok(ok__) => {
                    *requestid = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Disconnect<Impl: IMbnConnectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, requestid: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Disconnect() {
                ::core::result::Result::Ok(ok__) => {
                    *requestid = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetConnectionState<Impl: IMbnConnectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, connectionstate: *mut MBN_ACTIVATION_STATE, profilename: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetConnectionState(::core::mem::transmute_copy(&connectionstate), ::core::mem::transmute_copy(&profilename)).into()
        }
        unsafe extern "system" fn GetVoiceCallState<Impl: IMbnConnectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, voicecallstate: *mut MBN_VOICE_CALL_STATE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetVoiceCallState() {
                ::core::result::Result::Ok(ok__) => {
                    *voicecallstate = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetActivationNetworkError<Impl: IMbnConnectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, networkerror: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetActivationNetworkError() {
                ::core::result::Result::Ok(ok__) => {
                    *networkerror = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
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
    fn GetProvisionedContexts(&mut self) -> ::windows::core::Result<*mut super::super::System::Com::SAFEARRAY>;
    fn SetProvisionedContext(&mut self, provisionedcontexts: MBN_CONTEXT, providerid: super::super::Foundation::PWSTR) -> ::windows::core::Result<u32>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl IMbnConnectionContextVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMbnConnectionContextImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMbnConnectionContextVtbl {
        unsafe extern "system" fn GetProvisionedContexts<Impl: IMbnConnectionContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, provisionedcontexts: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetProvisionedContexts() {
                ::core::result::Result::Ok(ok__) => {
                    *provisionedcontexts = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetProvisionedContext<Impl: IMbnConnectionContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, provisionedcontexts: ::core::mem::ManuallyDrop<MBN_CONTEXT>, providerid: super::super::Foundation::PWSTR, requestid: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetProvisionedContext(::core::mem::transmute_copy(&provisionedcontexts), ::core::mem::transmute_copy(&providerid)) {
                ::core::result::Result::Ok(ok__) => {
                    *requestid = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
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
    fn OnProvisionedContextListChange(&mut self, newinterface: ::core::option::Option<IMbnConnectionContext>) -> ::windows::core::Result<()>;
    fn OnSetProvisionedContextComplete(&mut self, newinterface: ::core::option::Option<IMbnConnectionContext>, requestid: u32, status: ::windows::core::HRESULT) -> ::windows::core::Result<()>;
}
impl IMbnConnectionContextEventsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMbnConnectionContextEventsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMbnConnectionContextEventsVtbl {
        unsafe extern "system" fn OnProvisionedContextListChange<Impl: IMbnConnectionContextEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newinterface: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnProvisionedContextListChange(::core::mem::transmute(&newinterface)).into()
        }
        unsafe extern "system" fn OnSetProvisionedContextComplete<Impl: IMbnConnectionContextEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newinterface: ::windows::core::RawPtr, requestid: u32, status: ::windows::core::HRESULT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnSetProvisionedContextComplete(::core::mem::transmute(&newinterface), ::core::mem::transmute_copy(&requestid), ::core::mem::transmute_copy(&status)).into()
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
    fn OnConnectComplete(&mut self, newconnection: ::core::option::Option<IMbnConnection>, requestid: u32, status: ::windows::core::HRESULT) -> ::windows::core::Result<()>;
    fn OnDisconnectComplete(&mut self, newconnection: ::core::option::Option<IMbnConnection>, requestid: u32, status: ::windows::core::HRESULT) -> ::windows::core::Result<()>;
    fn OnConnectStateChange(&mut self, newconnection: ::core::option::Option<IMbnConnection>) -> ::windows::core::Result<()>;
    fn OnVoiceCallStateChange(&mut self, newconnection: ::core::option::Option<IMbnConnection>) -> ::windows::core::Result<()>;
}
impl IMbnConnectionEventsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMbnConnectionEventsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMbnConnectionEventsVtbl {
        unsafe extern "system" fn OnConnectComplete<Impl: IMbnConnectionEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newconnection: ::windows::core::RawPtr, requestid: u32, status: ::windows::core::HRESULT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnConnectComplete(::core::mem::transmute(&newconnection), ::core::mem::transmute_copy(&requestid), ::core::mem::transmute_copy(&status)).into()
        }
        unsafe extern "system" fn OnDisconnectComplete<Impl: IMbnConnectionEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newconnection: ::windows::core::RawPtr, requestid: u32, status: ::windows::core::HRESULT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnDisconnectComplete(::core::mem::transmute(&newconnection), ::core::mem::transmute_copy(&requestid), ::core::mem::transmute_copy(&status)).into()
        }
        unsafe extern "system" fn OnConnectStateChange<Impl: IMbnConnectionEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newconnection: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnConnectStateChange(::core::mem::transmute(&newconnection)).into()
        }
        unsafe extern "system" fn OnVoiceCallStateChange<Impl: IMbnConnectionEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newconnection: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnVoiceCallStateChange(::core::mem::transmute(&newconnection)).into()
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
    fn GetConnection(&mut self, connectionid: super::super::Foundation::PWSTR) -> ::windows::core::Result<IMbnConnection>;
    fn GetConnections(&mut self) -> ::windows::core::Result<*mut super::super::System::Com::SAFEARRAY>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl IMbnConnectionManagerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMbnConnectionManagerImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMbnConnectionManagerVtbl {
        unsafe extern "system" fn GetConnection<Impl: IMbnConnectionManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, connectionid: super::super::Foundation::PWSTR, mbnconnection: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetConnection(::core::mem::transmute_copy(&connectionid)) {
                ::core::result::Result::Ok(ok__) => {
                    *mbnconnection = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetConnections<Impl: IMbnConnectionManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, mbnconnections: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetConnections() {
                ::core::result::Result::Ok(ok__) => {
                    *mbnconnections = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
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
    fn OnConnectionArrival(&mut self, newconnection: ::core::option::Option<IMbnConnection>) -> ::windows::core::Result<()>;
    fn OnConnectionRemoval(&mut self, oldconnection: ::core::option::Option<IMbnConnection>) -> ::windows::core::Result<()>;
}
impl IMbnConnectionManagerEventsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMbnConnectionManagerEventsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMbnConnectionManagerEventsVtbl {
        unsafe extern "system" fn OnConnectionArrival<Impl: IMbnConnectionManagerEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newconnection: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnConnectionArrival(::core::mem::transmute(&newconnection)).into()
        }
        unsafe extern "system" fn OnConnectionRemoval<Impl: IMbnConnectionManagerEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, oldconnection: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnConnectionRemoval(::core::mem::transmute(&oldconnection)).into()
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
    fn GetProfileXmlData(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn UpdateProfile(&mut self, strprofile: super::super::Foundation::PWSTR) -> ::windows::core::Result<()>;
    fn Delete(&mut self) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IMbnConnectionProfileVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMbnConnectionProfileImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMbnConnectionProfileVtbl {
        unsafe extern "system" fn GetProfileXmlData<Impl: IMbnConnectionProfileImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, profiledata: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetProfileXmlData() {
                ::core::result::Result::Ok(ok__) => {
                    *profiledata = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UpdateProfile<Impl: IMbnConnectionProfileImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strprofile: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).UpdateProfile(::core::mem::transmute_copy(&strprofile)).into()
        }
        unsafe extern "system" fn Delete<Impl: IMbnConnectionProfileImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Delete().into()
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
    fn OnProfileUpdate(&mut self, newprofile: ::core::option::Option<IMbnConnectionProfile>) -> ::windows::core::Result<()>;
}
impl IMbnConnectionProfileEventsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMbnConnectionProfileEventsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMbnConnectionProfileEventsVtbl {
        unsafe extern "system" fn OnProfileUpdate<Impl: IMbnConnectionProfileEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newprofile: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnProfileUpdate(::core::mem::transmute(&newprofile)).into()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), OnProfileUpdate: OnProfileUpdate::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMbnConnectionProfileEvents as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub trait IMbnConnectionProfileManagerImpl: Sized {
    fn GetConnectionProfiles(&mut self, mbninterface: ::core::option::Option<IMbnInterface>) -> ::windows::core::Result<*mut super::super::System::Com::SAFEARRAY>;
    fn GetConnectionProfile(&mut self, mbninterface: ::core::option::Option<IMbnInterface>, profilename: super::super::Foundation::PWSTR) -> ::windows::core::Result<IMbnConnectionProfile>;
    fn CreateConnectionProfile(&mut self, xmlprofile: super::super::Foundation::PWSTR) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl IMbnConnectionProfileManagerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMbnConnectionProfileManagerImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMbnConnectionProfileManagerVtbl {
        unsafe extern "system" fn GetConnectionProfiles<Impl: IMbnConnectionProfileManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, mbninterface: ::windows::core::RawPtr, connectionprofiles: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetConnectionProfiles(::core::mem::transmute(&mbninterface)) {
                ::core::result::Result::Ok(ok__) => {
                    *connectionprofiles = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetConnectionProfile<Impl: IMbnConnectionProfileManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, mbninterface: ::windows::core::RawPtr, profilename: super::super::Foundation::PWSTR, connectionprofile: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetConnectionProfile(::core::mem::transmute(&mbninterface), ::core::mem::transmute_copy(&profilename)) {
                ::core::result::Result::Ok(ok__) => {
                    *connectionprofile = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateConnectionProfile<Impl: IMbnConnectionProfileManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, xmlprofile: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).CreateConnectionProfile(::core::mem::transmute_copy(&xmlprofile)).into()
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
    fn OnConnectionProfileArrival(&mut self, newconnectionprofile: ::core::option::Option<IMbnConnectionProfile>) -> ::windows::core::Result<()>;
    fn OnConnectionProfileRemoval(&mut self, oldconnectionprofile: ::core::option::Option<IMbnConnectionProfile>) -> ::windows::core::Result<()>;
}
impl IMbnConnectionProfileManagerEventsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMbnConnectionProfileManagerEventsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMbnConnectionProfileManagerEventsVtbl {
        unsafe extern "system" fn OnConnectionProfileArrival<Impl: IMbnConnectionProfileManagerEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newconnectionprofile: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnConnectionProfileArrival(::core::mem::transmute(&newconnectionprofile)).into()
        }
        unsafe extern "system" fn OnConnectionProfileRemoval<Impl: IMbnConnectionProfileManagerEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, oldconnectionprofile: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnConnectionProfileRemoval(::core::mem::transmute(&oldconnectionprofile)).into()
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
    fn QuerySupportedCommands(&mut self) -> ::windows::core::Result<u32>;
    fn OpenCommandSession(&mut self) -> ::windows::core::Result<u32>;
    fn CloseCommandSession(&mut self) -> ::windows::core::Result<u32>;
    fn SetCommand(&mut self, commandid: u32, deviceservicedata: *const super::super::System::Com::SAFEARRAY) -> ::windows::core::Result<u32>;
    fn QueryCommand(&mut self, commandid: u32, deviceservicedata: *const super::super::System::Com::SAFEARRAY) -> ::windows::core::Result<u32>;
    fn OpenDataSession(&mut self) -> ::windows::core::Result<u32>;
    fn CloseDataSession(&mut self) -> ::windows::core::Result<u32>;
    fn WriteData(&mut self, deviceservicedata: *const super::super::System::Com::SAFEARRAY) -> ::windows::core::Result<u32>;
    fn InterfaceID(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn DeviceServiceID(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn IsCommandSessionOpen(&mut self) -> ::windows::core::Result<super::super::Foundation::BOOL>;
    fn IsDataSessionOpen(&mut self) -> ::windows::core::Result<super::super::Foundation::BOOL>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl IMbnDeviceServiceVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMbnDeviceServiceImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMbnDeviceServiceVtbl {
        unsafe extern "system" fn QuerySupportedCommands<Impl: IMbnDeviceServiceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, requestid: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).QuerySupportedCommands() {
                ::core::result::Result::Ok(ok__) => {
                    *requestid = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OpenCommandSession<Impl: IMbnDeviceServiceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, requestid: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OpenCommandSession() {
                ::core::result::Result::Ok(ok__) => {
                    *requestid = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CloseCommandSession<Impl: IMbnDeviceServiceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, requestid: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CloseCommandSession() {
                ::core::result::Result::Ok(ok__) => {
                    *requestid = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCommand<Impl: IMbnDeviceServiceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, commandid: u32, deviceservicedata: *const super::super::System::Com::SAFEARRAY, requestid: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetCommand(::core::mem::transmute_copy(&commandid), ::core::mem::transmute_copy(&deviceservicedata)) {
                ::core::result::Result::Ok(ok__) => {
                    *requestid = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn QueryCommand<Impl: IMbnDeviceServiceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, commandid: u32, deviceservicedata: *const super::super::System::Com::SAFEARRAY, requestid: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).QueryCommand(::core::mem::transmute_copy(&commandid), ::core::mem::transmute_copy(&deviceservicedata)) {
                ::core::result::Result::Ok(ok__) => {
                    *requestid = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OpenDataSession<Impl: IMbnDeviceServiceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, requestid: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OpenDataSession() {
                ::core::result::Result::Ok(ok__) => {
                    *requestid = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CloseDataSession<Impl: IMbnDeviceServiceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, requestid: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CloseDataSession() {
                ::core::result::Result::Ok(ok__) => {
                    *requestid = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn WriteData<Impl: IMbnDeviceServiceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, deviceservicedata: *const super::super::System::Com::SAFEARRAY, requestid: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).WriteData(::core::mem::transmute_copy(&deviceservicedata)) {
                ::core::result::Result::Ok(ok__) => {
                    *requestid = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn InterfaceID<Impl: IMbnDeviceServiceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, interfaceid: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).InterfaceID() {
                ::core::result::Result::Ok(ok__) => {
                    *interfaceid = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DeviceServiceID<Impl: IMbnDeviceServiceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, deviceserviceid: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DeviceServiceID() {
                ::core::result::Result::Ok(ok__) => {
                    *deviceserviceid = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsCommandSessionOpen<Impl: IMbnDeviceServiceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsCommandSessionOpen() {
                ::core::result::Result::Ok(ok__) => {
                    *value = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsDataSessionOpen<Impl: IMbnDeviceServiceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsDataSessionOpen() {
                ::core::result::Result::Ok(ok__) => {
                    *value = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
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
    fn OnSessionsStateChange(&mut self, interfaceid: super::super::Foundation::BSTR, statechange: MBN_DEVICE_SERVICE_SESSIONS_STATE) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IMbnDeviceServiceStateEventsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMbnDeviceServiceStateEventsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMbnDeviceServiceStateEventsVtbl {
        unsafe extern "system" fn OnSessionsStateChange<Impl: IMbnDeviceServiceStateEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, interfaceid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, statechange: MBN_DEVICE_SERVICE_SESSIONS_STATE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnSessionsStateChange(::core::mem::transmute_copy(&interfaceid), ::core::mem::transmute_copy(&statechange)).into()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), OnSessionsStateChange: OnSessionsStateChange::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMbnDeviceServiceStateEvents as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub trait IMbnDeviceServicesContextImpl: Sized {
    fn EnumerateDeviceServices(&mut self) -> ::windows::core::Result<*mut super::super::System::Com::SAFEARRAY>;
    fn GetDeviceService(&mut self, deviceserviceid: super::super::Foundation::BSTR) -> ::windows::core::Result<IMbnDeviceService>;
    fn MaxCommandSize(&mut self) -> ::windows::core::Result<u32>;
    fn MaxDataSize(&mut self) -> ::windows::core::Result<u32>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl IMbnDeviceServicesContextVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMbnDeviceServicesContextImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMbnDeviceServicesContextVtbl {
        unsafe extern "system" fn EnumerateDeviceServices<Impl: IMbnDeviceServicesContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, deviceservices: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EnumerateDeviceServices() {
                ::core::result::Result::Ok(ok__) => {
                    *deviceservices = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDeviceService<Impl: IMbnDeviceServicesContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, deviceserviceid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, mbndeviceservice: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDeviceService(::core::mem::transmute_copy(&deviceserviceid)) {
                ::core::result::Result::Ok(ok__) => {
                    *mbndeviceservice = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MaxCommandSize<Impl: IMbnDeviceServicesContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, maxcommandsize: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MaxCommandSize() {
                ::core::result::Result::Ok(ok__) => {
                    *maxcommandsize = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MaxDataSize<Impl: IMbnDeviceServicesContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, maxdatasize: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MaxDataSize() {
                ::core::result::Result::Ok(ok__) => {
                    *maxdatasize = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
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
    fn OnQuerySupportedCommandsComplete(&mut self, deviceservice: ::core::option::Option<IMbnDeviceService>, commandidlist: *const super::super::System::Com::SAFEARRAY, status: ::windows::core::HRESULT, requestid: u32) -> ::windows::core::Result<()>;
    fn OnOpenCommandSessionComplete(&mut self, deviceservice: ::core::option::Option<IMbnDeviceService>, status: ::windows::core::HRESULT, requestid: u32) -> ::windows::core::Result<()>;
    fn OnCloseCommandSessionComplete(&mut self, deviceservice: ::core::option::Option<IMbnDeviceService>, status: ::windows::core::HRESULT, requestid: u32) -> ::windows::core::Result<()>;
    fn OnSetCommandComplete(&mut self, deviceservice: ::core::option::Option<IMbnDeviceService>, responseid: u32, deviceservicedata: *const super::super::System::Com::SAFEARRAY, status: ::windows::core::HRESULT, requestid: u32) -> ::windows::core::Result<()>;
    fn OnQueryCommandComplete(&mut self, deviceservice: ::core::option::Option<IMbnDeviceService>, responseid: u32, deviceservicedata: *const super::super::System::Com::SAFEARRAY, status: ::windows::core::HRESULT, requestid: u32) -> ::windows::core::Result<()>;
    fn OnEventNotification(&mut self, deviceservice: ::core::option::Option<IMbnDeviceService>, eventid: u32, deviceservicedata: *const super::super::System::Com::SAFEARRAY) -> ::windows::core::Result<()>;
    fn OnOpenDataSessionComplete(&mut self, deviceservice: ::core::option::Option<IMbnDeviceService>, status: ::windows::core::HRESULT, requestid: u32) -> ::windows::core::Result<()>;
    fn OnCloseDataSessionComplete(&mut self, deviceservice: ::core::option::Option<IMbnDeviceService>, status: ::windows::core::HRESULT, requestid: u32) -> ::windows::core::Result<()>;
    fn OnWriteDataComplete(&mut self, deviceservice: ::core::option::Option<IMbnDeviceService>, status: ::windows::core::HRESULT, requestid: u32) -> ::windows::core::Result<()>;
    fn OnReadData(&mut self, deviceservice: ::core::option::Option<IMbnDeviceService>, deviceservicedata: *const super::super::System::Com::SAFEARRAY) -> ::windows::core::Result<()>;
    fn OnInterfaceStateChange(&mut self, interfaceid: super::super::Foundation::BSTR, statechange: MBN_DEVICE_SERVICES_INTERFACE_STATE) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl IMbnDeviceServicesEventsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMbnDeviceServicesEventsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMbnDeviceServicesEventsVtbl {
        unsafe extern "system" fn OnQuerySupportedCommandsComplete<Impl: IMbnDeviceServicesEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, deviceservice: ::windows::core::RawPtr, commandidlist: *const super::super::System::Com::SAFEARRAY, status: ::windows::core::HRESULT, requestid: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnQuerySupportedCommandsComplete(::core::mem::transmute(&deviceservice), ::core::mem::transmute_copy(&commandidlist), ::core::mem::transmute_copy(&status), ::core::mem::transmute_copy(&requestid)).into()
        }
        unsafe extern "system" fn OnOpenCommandSessionComplete<Impl: IMbnDeviceServicesEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, deviceservice: ::windows::core::RawPtr, status: ::windows::core::HRESULT, requestid: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnOpenCommandSessionComplete(::core::mem::transmute(&deviceservice), ::core::mem::transmute_copy(&status), ::core::mem::transmute_copy(&requestid)).into()
        }
        unsafe extern "system" fn OnCloseCommandSessionComplete<Impl: IMbnDeviceServicesEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, deviceservice: ::windows::core::RawPtr, status: ::windows::core::HRESULT, requestid: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnCloseCommandSessionComplete(::core::mem::transmute(&deviceservice), ::core::mem::transmute_copy(&status), ::core::mem::transmute_copy(&requestid)).into()
        }
        unsafe extern "system" fn OnSetCommandComplete<Impl: IMbnDeviceServicesEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, deviceservice: ::windows::core::RawPtr, responseid: u32, deviceservicedata: *const super::super::System::Com::SAFEARRAY, status: ::windows::core::HRESULT, requestid: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnSetCommandComplete(::core::mem::transmute(&deviceservice), ::core::mem::transmute_copy(&responseid), ::core::mem::transmute_copy(&deviceservicedata), ::core::mem::transmute_copy(&status), ::core::mem::transmute_copy(&requestid)).into()
        }
        unsafe extern "system" fn OnQueryCommandComplete<Impl: IMbnDeviceServicesEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, deviceservice: ::windows::core::RawPtr, responseid: u32, deviceservicedata: *const super::super::System::Com::SAFEARRAY, status: ::windows::core::HRESULT, requestid: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnQueryCommandComplete(::core::mem::transmute(&deviceservice), ::core::mem::transmute_copy(&responseid), ::core::mem::transmute_copy(&deviceservicedata), ::core::mem::transmute_copy(&status), ::core::mem::transmute_copy(&requestid)).into()
        }
        unsafe extern "system" fn OnEventNotification<Impl: IMbnDeviceServicesEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, deviceservice: ::windows::core::RawPtr, eventid: u32, deviceservicedata: *const super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnEventNotification(::core::mem::transmute(&deviceservice), ::core::mem::transmute_copy(&eventid), ::core::mem::transmute_copy(&deviceservicedata)).into()
        }
        unsafe extern "system" fn OnOpenDataSessionComplete<Impl: IMbnDeviceServicesEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, deviceservice: ::windows::core::RawPtr, status: ::windows::core::HRESULT, requestid: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnOpenDataSessionComplete(::core::mem::transmute(&deviceservice), ::core::mem::transmute_copy(&status), ::core::mem::transmute_copy(&requestid)).into()
        }
        unsafe extern "system" fn OnCloseDataSessionComplete<Impl: IMbnDeviceServicesEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, deviceservice: ::windows::core::RawPtr, status: ::windows::core::HRESULT, requestid: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnCloseDataSessionComplete(::core::mem::transmute(&deviceservice), ::core::mem::transmute_copy(&status), ::core::mem::transmute_copy(&requestid)).into()
        }
        unsafe extern "system" fn OnWriteDataComplete<Impl: IMbnDeviceServicesEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, deviceservice: ::windows::core::RawPtr, status: ::windows::core::HRESULT, requestid: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnWriteDataComplete(::core::mem::transmute(&deviceservice), ::core::mem::transmute_copy(&status), ::core::mem::transmute_copy(&requestid)).into()
        }
        unsafe extern "system" fn OnReadData<Impl: IMbnDeviceServicesEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, deviceservice: ::windows::core::RawPtr, deviceservicedata: *const super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnReadData(::core::mem::transmute(&deviceservice), ::core::mem::transmute_copy(&deviceservicedata)).into()
        }
        unsafe extern "system" fn OnInterfaceStateChange<Impl: IMbnDeviceServicesEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, interfaceid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, statechange: MBN_DEVICE_SERVICES_INTERFACE_STATE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnInterfaceStateChange(::core::mem::transmute_copy(&interfaceid), ::core::mem::transmute_copy(&statechange)).into()
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
    fn GetDeviceServicesContext(&mut self, networkinterfaceid: super::super::Foundation::BSTR) -> ::windows::core::Result<IMbnDeviceServicesContext>;
}
#[cfg(feature = "Win32_Foundation")]
impl IMbnDeviceServicesManagerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMbnDeviceServicesManagerImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMbnDeviceServicesManagerVtbl {
        unsafe extern "system" fn GetDeviceServicesContext<Impl: IMbnDeviceServicesManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, networkinterfaceid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, mbndevicescontext: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDeviceServicesContext(::core::mem::transmute_copy(&networkinterfaceid)) {
                ::core::result::Result::Ok(ok__) => {
                    *mbndevicescontext = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), GetDeviceServicesContext: GetDeviceServicesContext::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMbnDeviceServicesManager as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub trait IMbnInterfaceImpl: Sized {
    fn InterfaceID(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn GetInterfaceCapability(&mut self) -> ::windows::core::Result<MBN_INTERFACE_CAPS>;
    fn GetSubscriberInformation(&mut self) -> ::windows::core::Result<IMbnSubscriberInformation>;
    fn GetReadyState(&mut self) -> ::windows::core::Result<MBN_READY_STATE>;
    fn InEmergencyMode(&mut self) -> ::windows::core::Result<i16>;
    fn GetHomeProvider(&mut self) -> ::windows::core::Result<MBN_PROVIDER>;
    fn GetPreferredProviders(&mut self) -> ::windows::core::Result<*mut super::super::System::Com::SAFEARRAY>;
    fn SetPreferredProviders(&mut self, preferredproviders: *const super::super::System::Com::SAFEARRAY) -> ::windows::core::Result<u32>;
    fn GetVisibleProviders(&mut self, age: *mut u32, visibleproviders: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::core::Result<()>;
    fn ScanNetwork(&mut self) -> ::windows::core::Result<u32>;
    fn GetConnection(&mut self) -> ::windows::core::Result<IMbnConnection>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl IMbnInterfaceVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMbnInterfaceImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMbnInterfaceVtbl {
        unsafe extern "system" fn InterfaceID<Impl: IMbnInterfaceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, interfaceid: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).InterfaceID() {
                ::core::result::Result::Ok(ok__) => {
                    *interfaceid = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetInterfaceCapability<Impl: IMbnInterfaceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, interfacecaps: *mut MBN_INTERFACE_CAPS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetInterfaceCapability() {
                ::core::result::Result::Ok(ok__) => {
                    *interfacecaps = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSubscriberInformation<Impl: IMbnInterfaceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, subscriberinformation: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetSubscriberInformation() {
                ::core::result::Result::Ok(ok__) => {
                    *subscriberinformation = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetReadyState<Impl: IMbnInterfaceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, readystate: *mut MBN_READY_STATE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetReadyState() {
                ::core::result::Result::Ok(ok__) => {
                    *readystate = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn InEmergencyMode<Impl: IMbnInterfaceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, emergencymode: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).InEmergencyMode() {
                ::core::result::Result::Ok(ok__) => {
                    *emergencymode = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetHomeProvider<Impl: IMbnInterfaceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, homeprovider: *mut MBN_PROVIDER) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetHomeProvider() {
                ::core::result::Result::Ok(ok__) => {
                    *homeprovider = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPreferredProviders<Impl: IMbnInterfaceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, preferredproviders: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetPreferredProviders() {
                ::core::result::Result::Ok(ok__) => {
                    *preferredproviders = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPreferredProviders<Impl: IMbnInterfaceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, preferredproviders: *const super::super::System::Com::SAFEARRAY, requestid: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetPreferredProviders(::core::mem::transmute_copy(&preferredproviders)) {
                ::core::result::Result::Ok(ok__) => {
                    *requestid = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetVisibleProviders<Impl: IMbnInterfaceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, age: *mut u32, visibleproviders: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetVisibleProviders(::core::mem::transmute_copy(&age), ::core::mem::transmute_copy(&visibleproviders)).into()
        }
        unsafe extern "system" fn ScanNetwork<Impl: IMbnInterfaceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, requestid: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ScanNetwork() {
                ::core::result::Result::Ok(ok__) => {
                    *requestid = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetConnection<Impl: IMbnInterfaceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, mbnconnection: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetConnection() {
                ::core::result::Result::Ok(ok__) => {
                    *mbnconnection = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
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
    fn OnInterfaceCapabilityAvailable(&mut self, newinterface: ::core::option::Option<IMbnInterface>) -> ::windows::core::Result<()>;
    fn OnSubscriberInformationChange(&mut self, newinterface: ::core::option::Option<IMbnInterface>) -> ::windows::core::Result<()>;
    fn OnReadyStateChange(&mut self, newinterface: ::core::option::Option<IMbnInterface>) -> ::windows::core::Result<()>;
    fn OnEmergencyModeChange(&mut self, newinterface: ::core::option::Option<IMbnInterface>) -> ::windows::core::Result<()>;
    fn OnHomeProviderAvailable(&mut self, newinterface: ::core::option::Option<IMbnInterface>) -> ::windows::core::Result<()>;
    fn OnPreferredProvidersChange(&mut self, newinterface: ::core::option::Option<IMbnInterface>) -> ::windows::core::Result<()>;
    fn OnSetPreferredProvidersComplete(&mut self, newinterface: ::core::option::Option<IMbnInterface>, requestid: u32, status: ::windows::core::HRESULT) -> ::windows::core::Result<()>;
    fn OnScanNetworkComplete(&mut self, newinterface: ::core::option::Option<IMbnInterface>, requestid: u32, status: ::windows::core::HRESULT) -> ::windows::core::Result<()>;
}
impl IMbnInterfaceEventsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMbnInterfaceEventsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMbnInterfaceEventsVtbl {
        unsafe extern "system" fn OnInterfaceCapabilityAvailable<Impl: IMbnInterfaceEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newinterface: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnInterfaceCapabilityAvailable(::core::mem::transmute(&newinterface)).into()
        }
        unsafe extern "system" fn OnSubscriberInformationChange<Impl: IMbnInterfaceEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newinterface: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnSubscriberInformationChange(::core::mem::transmute(&newinterface)).into()
        }
        unsafe extern "system" fn OnReadyStateChange<Impl: IMbnInterfaceEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newinterface: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnReadyStateChange(::core::mem::transmute(&newinterface)).into()
        }
        unsafe extern "system" fn OnEmergencyModeChange<Impl: IMbnInterfaceEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newinterface: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnEmergencyModeChange(::core::mem::transmute(&newinterface)).into()
        }
        unsafe extern "system" fn OnHomeProviderAvailable<Impl: IMbnInterfaceEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newinterface: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnHomeProviderAvailable(::core::mem::transmute(&newinterface)).into()
        }
        unsafe extern "system" fn OnPreferredProvidersChange<Impl: IMbnInterfaceEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newinterface: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnPreferredProvidersChange(::core::mem::transmute(&newinterface)).into()
        }
        unsafe extern "system" fn OnSetPreferredProvidersComplete<Impl: IMbnInterfaceEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newinterface: ::windows::core::RawPtr, requestid: u32, status: ::windows::core::HRESULT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnSetPreferredProvidersComplete(::core::mem::transmute(&newinterface), ::core::mem::transmute_copy(&requestid), ::core::mem::transmute_copy(&status)).into()
        }
        unsafe extern "system" fn OnScanNetworkComplete<Impl: IMbnInterfaceEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newinterface: ::windows::core::RawPtr, requestid: u32, status: ::windows::core::HRESULT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnScanNetworkComplete(::core::mem::transmute(&newinterface), ::core::mem::transmute_copy(&requestid), ::core::mem::transmute_copy(&status)).into()
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
    fn GetInterface(&mut self, interfaceid: super::super::Foundation::PWSTR) -> ::windows::core::Result<IMbnInterface>;
    fn GetInterfaces(&mut self) -> ::windows::core::Result<*mut super::super::System::Com::SAFEARRAY>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl IMbnInterfaceManagerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMbnInterfaceManagerImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMbnInterfaceManagerVtbl {
        unsafe extern "system" fn GetInterface<Impl: IMbnInterfaceManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, interfaceid: super::super::Foundation::PWSTR, mbninterface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetInterface(::core::mem::transmute_copy(&interfaceid)) {
                ::core::result::Result::Ok(ok__) => {
                    *mbninterface = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetInterfaces<Impl: IMbnInterfaceManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, mbninterfaces: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetInterfaces() {
                ::core::result::Result::Ok(ok__) => {
                    *mbninterfaces = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
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
    fn OnInterfaceArrival(&mut self, newinterface: ::core::option::Option<IMbnInterface>) -> ::windows::core::Result<()>;
    fn OnInterfaceRemoval(&mut self, oldinterface: ::core::option::Option<IMbnInterface>) -> ::windows::core::Result<()>;
}
impl IMbnInterfaceManagerEventsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMbnInterfaceManagerEventsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMbnInterfaceManagerEventsVtbl {
        unsafe extern "system" fn OnInterfaceArrival<Impl: IMbnInterfaceManagerEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newinterface: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnInterfaceArrival(::core::mem::transmute(&newinterface)).into()
        }
        unsafe extern "system" fn OnInterfaceRemoval<Impl: IMbnInterfaceManagerEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, oldinterface: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnInterfaceRemoval(::core::mem::transmute(&oldinterface)).into()
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
    fn SetHomeProvider(&mut self, homeprovider: *const MBN_PROVIDER2) -> ::windows::core::Result<u32>;
    fn GetPreferredProviders(&mut self) -> ::windows::core::Result<*mut super::super::System::Com::SAFEARRAY>;
    fn GetVisibleProviders(&mut self, age: *mut u32, visibleproviders: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::core::Result<()>;
    fn GetSupportedCellularClasses(&mut self) -> ::windows::core::Result<*mut super::super::System::Com::SAFEARRAY>;
    fn GetCurrentCellularClass(&mut self) -> ::windows::core::Result<MBN_CELLULAR_CLASS>;
    fn ScanNetwork(&mut self) -> ::windows::core::Result<u32>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl IMbnMultiCarrierVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMbnMultiCarrierImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMbnMultiCarrierVtbl {
        unsafe extern "system" fn SetHomeProvider<Impl: IMbnMultiCarrierImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, homeprovider: *const MBN_PROVIDER2, requestid: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetHomeProvider(::core::mem::transmute_copy(&homeprovider)) {
                ::core::result::Result::Ok(ok__) => {
                    *requestid = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPreferredProviders<Impl: IMbnMultiCarrierImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, preferredmulticarrierproviders: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetPreferredProviders() {
                ::core::result::Result::Ok(ok__) => {
                    *preferredmulticarrierproviders = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetVisibleProviders<Impl: IMbnMultiCarrierImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, age: *mut u32, visibleproviders: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetVisibleProviders(::core::mem::transmute_copy(&age), ::core::mem::transmute_copy(&visibleproviders)).into()
        }
        unsafe extern "system" fn GetSupportedCellularClasses<Impl: IMbnMultiCarrierImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cellularclasses: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetSupportedCellularClasses() {
                ::core::result::Result::Ok(ok__) => {
                    *cellularclasses = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCurrentCellularClass<Impl: IMbnMultiCarrierImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, currentcellularclass: *mut MBN_CELLULAR_CLASS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetCurrentCellularClass() {
                ::core::result::Result::Ok(ok__) => {
                    *currentcellularclass = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ScanNetwork<Impl: IMbnMultiCarrierImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, requestid: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ScanNetwork() {
                ::core::result::Result::Ok(ok__) => {
                    *requestid = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
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
    fn OnSetHomeProviderComplete(&mut self, mbninterface: ::core::option::Option<IMbnMultiCarrier>, requestid: u32, status: ::windows::core::HRESULT) -> ::windows::core::Result<()>;
    fn OnCurrentCellularClassChange(&mut self, mbninterface: ::core::option::Option<IMbnMultiCarrier>) -> ::windows::core::Result<()>;
    fn OnPreferredProvidersChange(&mut self, mbninterface: ::core::option::Option<IMbnMultiCarrier>) -> ::windows::core::Result<()>;
    fn OnScanNetworkComplete(&mut self, mbninterface: ::core::option::Option<IMbnMultiCarrier>, requestid: u32, status: ::windows::core::HRESULT) -> ::windows::core::Result<()>;
    fn OnInterfaceCapabilityChange(&mut self, mbninterface: ::core::option::Option<IMbnMultiCarrier>) -> ::windows::core::Result<()>;
}
impl IMbnMultiCarrierEventsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMbnMultiCarrierEventsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMbnMultiCarrierEventsVtbl {
        unsafe extern "system" fn OnSetHomeProviderComplete<Impl: IMbnMultiCarrierEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, mbninterface: ::windows::core::RawPtr, requestid: u32, status: ::windows::core::HRESULT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnSetHomeProviderComplete(::core::mem::transmute(&mbninterface), ::core::mem::transmute_copy(&requestid), ::core::mem::transmute_copy(&status)).into()
        }
        unsafe extern "system" fn OnCurrentCellularClassChange<Impl: IMbnMultiCarrierEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, mbninterface: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnCurrentCellularClassChange(::core::mem::transmute(&mbninterface)).into()
        }
        unsafe extern "system" fn OnPreferredProvidersChange<Impl: IMbnMultiCarrierEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, mbninterface: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnPreferredProvidersChange(::core::mem::transmute(&mbninterface)).into()
        }
        unsafe extern "system" fn OnScanNetworkComplete<Impl: IMbnMultiCarrierEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, mbninterface: ::windows::core::RawPtr, requestid: u32, status: ::windows::core::HRESULT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnScanNetworkComplete(::core::mem::transmute(&mbninterface), ::core::mem::transmute_copy(&requestid), ::core::mem::transmute_copy(&status)).into()
        }
        unsafe extern "system" fn OnInterfaceCapabilityChange<Impl: IMbnMultiCarrierEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, mbninterface: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnInterfaceCapabilityChange(::core::mem::transmute(&mbninterface)).into()
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
    fn PinType(&mut self) -> ::windows::core::Result<MBN_PIN_TYPE>;
    fn PinFormat(&mut self) -> ::windows::core::Result<MBN_PIN_FORMAT>;
    fn PinLengthMin(&mut self) -> ::windows::core::Result<u32>;
    fn PinLengthMax(&mut self) -> ::windows::core::Result<u32>;
    fn PinMode(&mut self) -> ::windows::core::Result<MBN_PIN_MODE>;
    fn Enable(&mut self, pin: super::super::Foundation::PWSTR) -> ::windows::core::Result<u32>;
    fn Disable(&mut self, pin: super::super::Foundation::PWSTR) -> ::windows::core::Result<u32>;
    fn Enter(&mut self, pin: super::super::Foundation::PWSTR) -> ::windows::core::Result<u32>;
    fn Change(&mut self, pin: super::super::Foundation::PWSTR, newpin: super::super::Foundation::PWSTR) -> ::windows::core::Result<u32>;
    fn Unblock(&mut self, puk: super::super::Foundation::PWSTR, newpin: super::super::Foundation::PWSTR) -> ::windows::core::Result<u32>;
    fn GetPinManager(&mut self) -> ::windows::core::Result<IMbnPinManager>;
}
#[cfg(feature = "Win32_Foundation")]
impl IMbnPinVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMbnPinImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMbnPinVtbl {
        unsafe extern "system" fn PinType<Impl: IMbnPinImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pintype: *mut MBN_PIN_TYPE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PinType() {
                ::core::result::Result::Ok(ok__) => {
                    *pintype = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PinFormat<Impl: IMbnPinImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinformat: *mut MBN_PIN_FORMAT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PinFormat() {
                ::core::result::Result::Ok(ok__) => {
                    *pinformat = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PinLengthMin<Impl: IMbnPinImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinlengthmin: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PinLengthMin() {
                ::core::result::Result::Ok(ok__) => {
                    *pinlengthmin = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PinLengthMax<Impl: IMbnPinImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinlengthmax: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PinLengthMax() {
                ::core::result::Result::Ok(ok__) => {
                    *pinlengthmax = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PinMode<Impl: IMbnPinImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinmode: *mut MBN_PIN_MODE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PinMode() {
                ::core::result::Result::Ok(ok__) => {
                    *pinmode = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Enable<Impl: IMbnPinImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pin: super::super::Foundation::PWSTR, requestid: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Enable(::core::mem::transmute_copy(&pin)) {
                ::core::result::Result::Ok(ok__) => {
                    *requestid = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Disable<Impl: IMbnPinImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pin: super::super::Foundation::PWSTR, requestid: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Disable(::core::mem::transmute_copy(&pin)) {
                ::core::result::Result::Ok(ok__) => {
                    *requestid = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Enter<Impl: IMbnPinImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pin: super::super::Foundation::PWSTR, requestid: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Enter(::core::mem::transmute_copy(&pin)) {
                ::core::result::Result::Ok(ok__) => {
                    *requestid = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Change<Impl: IMbnPinImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pin: super::super::Foundation::PWSTR, newpin: super::super::Foundation::PWSTR, requestid: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Change(::core::mem::transmute_copy(&pin), ::core::mem::transmute_copy(&newpin)) {
                ::core::result::Result::Ok(ok__) => {
                    *requestid = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Unblock<Impl: IMbnPinImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, puk: super::super::Foundation::PWSTR, newpin: super::super::Foundation::PWSTR, requestid: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Unblock(::core::mem::transmute_copy(&puk), ::core::mem::transmute_copy(&newpin)) {
                ::core::result::Result::Ok(ok__) => {
                    *requestid = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPinManager<Impl: IMbnPinImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinmanager: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetPinManager() {
                ::core::result::Result::Ok(ok__) => {
                    *pinmanager = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
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
    fn OnEnableComplete(&mut self, pin: ::core::option::Option<IMbnPin>, pininfo: *const MBN_PIN_INFO, requestid: u32, status: ::windows::core::HRESULT) -> ::windows::core::Result<()>;
    fn OnDisableComplete(&mut self, pin: ::core::option::Option<IMbnPin>, pininfo: *const MBN_PIN_INFO, requestid: u32, status: ::windows::core::HRESULT) -> ::windows::core::Result<()>;
    fn OnEnterComplete(&mut self, pin: ::core::option::Option<IMbnPin>, pininfo: *const MBN_PIN_INFO, requestid: u32, status: ::windows::core::HRESULT) -> ::windows::core::Result<()>;
    fn OnChangeComplete(&mut self, pin: ::core::option::Option<IMbnPin>, pininfo: *const MBN_PIN_INFO, requestid: u32, status: ::windows::core::HRESULT) -> ::windows::core::Result<()>;
    fn OnUnblockComplete(&mut self, pin: ::core::option::Option<IMbnPin>, pininfo: *const MBN_PIN_INFO, requestid: u32, status: ::windows::core::HRESULT) -> ::windows::core::Result<()>;
}
impl IMbnPinEventsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMbnPinEventsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMbnPinEventsVtbl {
        unsafe extern "system" fn OnEnableComplete<Impl: IMbnPinEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pin: ::windows::core::RawPtr, pininfo: *const MBN_PIN_INFO, requestid: u32, status: ::windows::core::HRESULT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnEnableComplete(::core::mem::transmute(&pin), ::core::mem::transmute_copy(&pininfo), ::core::mem::transmute_copy(&requestid), ::core::mem::transmute_copy(&status)).into()
        }
        unsafe extern "system" fn OnDisableComplete<Impl: IMbnPinEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pin: ::windows::core::RawPtr, pininfo: *const MBN_PIN_INFO, requestid: u32, status: ::windows::core::HRESULT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnDisableComplete(::core::mem::transmute(&pin), ::core::mem::transmute_copy(&pininfo), ::core::mem::transmute_copy(&requestid), ::core::mem::transmute_copy(&status)).into()
        }
        unsafe extern "system" fn OnEnterComplete<Impl: IMbnPinEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pin: ::windows::core::RawPtr, pininfo: *const MBN_PIN_INFO, requestid: u32, status: ::windows::core::HRESULT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnEnterComplete(::core::mem::transmute(&pin), ::core::mem::transmute_copy(&pininfo), ::core::mem::transmute_copy(&requestid), ::core::mem::transmute_copy(&status)).into()
        }
        unsafe extern "system" fn OnChangeComplete<Impl: IMbnPinEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pin: ::windows::core::RawPtr, pininfo: *const MBN_PIN_INFO, requestid: u32, status: ::windows::core::HRESULT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnChangeComplete(::core::mem::transmute(&pin), ::core::mem::transmute_copy(&pininfo), ::core::mem::transmute_copy(&requestid), ::core::mem::transmute_copy(&status)).into()
        }
        unsafe extern "system" fn OnUnblockComplete<Impl: IMbnPinEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pin: ::windows::core::RawPtr, pininfo: *const MBN_PIN_INFO, requestid: u32, status: ::windows::core::HRESULT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnUnblockComplete(::core::mem::transmute(&pin), ::core::mem::transmute_copy(&pininfo), ::core::mem::transmute_copy(&requestid), ::core::mem::transmute_copy(&status)).into()
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
    fn GetPinList(&mut self) -> ::windows::core::Result<*mut super::super::System::Com::SAFEARRAY>;
    fn GetPin(&mut self, pintype: MBN_PIN_TYPE) -> ::windows::core::Result<IMbnPin>;
    fn GetPinState(&mut self) -> ::windows::core::Result<u32>;
}
#[cfg(feature = "Win32_System_Com")]
impl IMbnPinManagerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMbnPinManagerImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMbnPinManagerVtbl {
        unsafe extern "system" fn GetPinList<Impl: IMbnPinManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinlist: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetPinList() {
                ::core::result::Result::Ok(ok__) => {
                    *pinlist = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPin<Impl: IMbnPinManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pintype: MBN_PIN_TYPE, pin: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetPin(::core::mem::transmute_copy(&pintype)) {
                ::core::result::Result::Ok(ok__) => {
                    *pin = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPinState<Impl: IMbnPinManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, requestid: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetPinState() {
                ::core::result::Result::Ok(ok__) => {
                    *requestid = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
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
    fn OnPinListAvailable(&mut self, pinmanager: ::core::option::Option<IMbnPinManager>) -> ::windows::core::Result<()>;
    fn OnGetPinStateComplete(&mut self, pinmanager: ::core::option::Option<IMbnPinManager>, pininfo: MBN_PIN_INFO, requestid: u32, status: ::windows::core::HRESULT) -> ::windows::core::Result<()>;
}
impl IMbnPinManagerEventsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMbnPinManagerEventsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMbnPinManagerEventsVtbl {
        unsafe extern "system" fn OnPinListAvailable<Impl: IMbnPinManagerEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinmanager: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnPinListAvailable(::core::mem::transmute(&pinmanager)).into()
        }
        unsafe extern "system" fn OnGetPinStateComplete<Impl: IMbnPinManagerEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinmanager: ::windows::core::RawPtr, pininfo: MBN_PIN_INFO, requestid: u32, status: ::windows::core::HRESULT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnGetPinStateComplete(::core::mem::transmute(&pinmanager), ::core::mem::transmute_copy(&pininfo), ::core::mem::transmute_copy(&requestid), ::core::mem::transmute_copy(&status)).into()
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
    fn SoftwareRadioState(&mut self) -> ::windows::core::Result<MBN_RADIO>;
    fn HardwareRadioState(&mut self) -> ::windows::core::Result<MBN_RADIO>;
    fn SetSoftwareRadioState(&mut self, radiostate: MBN_RADIO) -> ::windows::core::Result<u32>;
}
impl IMbnRadioVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMbnRadioImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMbnRadioVtbl {
        unsafe extern "system" fn SoftwareRadioState<Impl: IMbnRadioImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, softwareradiostate: *mut MBN_RADIO) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SoftwareRadioState() {
                ::core::result::Result::Ok(ok__) => {
                    *softwareradiostate = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn HardwareRadioState<Impl: IMbnRadioImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hardwareradiostate: *mut MBN_RADIO) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).HardwareRadioState() {
                ::core::result::Result::Ok(ok__) => {
                    *hardwareradiostate = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSoftwareRadioState<Impl: IMbnRadioImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, radiostate: MBN_RADIO, requestid: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetSoftwareRadioState(::core::mem::transmute_copy(&radiostate)) {
                ::core::result::Result::Ok(ok__) => {
                    *requestid = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
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
    fn OnRadioStateChange(&mut self, newinterface: ::core::option::Option<IMbnRadio>) -> ::windows::core::Result<()>;
    fn OnSetSoftwareRadioStateComplete(&mut self, newinterface: ::core::option::Option<IMbnRadio>, requestid: u32, status: ::windows::core::HRESULT) -> ::windows::core::Result<()>;
}
impl IMbnRadioEventsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMbnRadioEventsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMbnRadioEventsVtbl {
        unsafe extern "system" fn OnRadioStateChange<Impl: IMbnRadioEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newinterface: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnRadioStateChange(::core::mem::transmute(&newinterface)).into()
        }
        unsafe extern "system" fn OnSetSoftwareRadioStateComplete<Impl: IMbnRadioEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newinterface: ::windows::core::RawPtr, requestid: u32, status: ::windows::core::HRESULT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnSetSoftwareRadioStateComplete(::core::mem::transmute(&newinterface), ::core::mem::transmute_copy(&requestid), ::core::mem::transmute_copy(&status)).into()
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
    fn GetRegisterState(&mut self) -> ::windows::core::Result<MBN_REGISTER_STATE>;
    fn GetRegisterMode(&mut self) -> ::windows::core::Result<MBN_REGISTER_MODE>;
    fn GetProviderID(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn GetProviderName(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn GetRoamingText(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn GetAvailableDataClasses(&mut self) -> ::windows::core::Result<u32>;
    fn GetCurrentDataClass(&mut self) -> ::windows::core::Result<u32>;
    fn GetRegistrationNetworkError(&mut self) -> ::windows::core::Result<u32>;
    fn GetPacketAttachNetworkError(&mut self) -> ::windows::core::Result<u32>;
    fn SetRegisterMode(&mut self, registermode: MBN_REGISTER_MODE, providerid: super::super::Foundation::PWSTR, dataclass: u32) -> ::windows::core::Result<u32>;
}
#[cfg(feature = "Win32_Foundation")]
impl IMbnRegistrationVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMbnRegistrationImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMbnRegistrationVtbl {
        unsafe extern "system" fn GetRegisterState<Impl: IMbnRegistrationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, registerstate: *mut MBN_REGISTER_STATE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetRegisterState() {
                ::core::result::Result::Ok(ok__) => {
                    *registerstate = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetRegisterMode<Impl: IMbnRegistrationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, registermode: *mut MBN_REGISTER_MODE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetRegisterMode() {
                ::core::result::Result::Ok(ok__) => {
                    *registermode = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetProviderID<Impl: IMbnRegistrationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, providerid: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetProviderID() {
                ::core::result::Result::Ok(ok__) => {
                    *providerid = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetProviderName<Impl: IMbnRegistrationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, providername: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetProviderName() {
                ::core::result::Result::Ok(ok__) => {
                    *providername = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetRoamingText<Impl: IMbnRegistrationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, roamingtext: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetRoamingText() {
                ::core::result::Result::Ok(ok__) => {
                    *roamingtext = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetAvailableDataClasses<Impl: IMbnRegistrationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, availabledataclasses: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetAvailableDataClasses() {
                ::core::result::Result::Ok(ok__) => {
                    *availabledataclasses = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCurrentDataClass<Impl: IMbnRegistrationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, currentdataclass: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetCurrentDataClass() {
                ::core::result::Result::Ok(ok__) => {
                    *currentdataclass = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetRegistrationNetworkError<Impl: IMbnRegistrationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, registrationnetworkerror: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetRegistrationNetworkError() {
                ::core::result::Result::Ok(ok__) => {
                    *registrationnetworkerror = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPacketAttachNetworkError<Impl: IMbnRegistrationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, packetattachnetworkerror: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetPacketAttachNetworkError() {
                ::core::result::Result::Ok(ok__) => {
                    *packetattachnetworkerror = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetRegisterMode<Impl: IMbnRegistrationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, registermode: MBN_REGISTER_MODE, providerid: super::super::Foundation::PWSTR, dataclass: u32, requestid: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetRegisterMode(::core::mem::transmute_copy(&registermode), ::core::mem::transmute_copy(&providerid), ::core::mem::transmute_copy(&dataclass)) {
                ::core::result::Result::Ok(ok__) => {
                    *requestid = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
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
    fn OnRegisterModeAvailable(&mut self, newinterface: ::core::option::Option<IMbnRegistration>) -> ::windows::core::Result<()>;
    fn OnRegisterStateChange(&mut self, newinterface: ::core::option::Option<IMbnRegistration>) -> ::windows::core::Result<()>;
    fn OnPacketServiceStateChange(&mut self, newinterface: ::core::option::Option<IMbnRegistration>) -> ::windows::core::Result<()>;
    fn OnSetRegisterModeComplete(&mut self, newinterface: ::core::option::Option<IMbnRegistration>, requestid: u32, status: ::windows::core::HRESULT) -> ::windows::core::Result<()>;
}
impl IMbnRegistrationEventsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMbnRegistrationEventsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMbnRegistrationEventsVtbl {
        unsafe extern "system" fn OnRegisterModeAvailable<Impl: IMbnRegistrationEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newinterface: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnRegisterModeAvailable(::core::mem::transmute(&newinterface)).into()
        }
        unsafe extern "system" fn OnRegisterStateChange<Impl: IMbnRegistrationEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newinterface: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnRegisterStateChange(::core::mem::transmute(&newinterface)).into()
        }
        unsafe extern "system" fn OnPacketServiceStateChange<Impl: IMbnRegistrationEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newinterface: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnPacketServiceStateChange(::core::mem::transmute(&newinterface)).into()
        }
        unsafe extern "system" fn OnSetRegisterModeComplete<Impl: IMbnRegistrationEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newinterface: ::windows::core::RawPtr, requestid: u32, status: ::windows::core::HRESULT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnSetRegisterModeComplete(::core::mem::transmute(&newinterface), ::core::mem::transmute_copy(&requestid), ::core::mem::transmute_copy(&status)).into()
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
    fn Activate(&mut self, vendorspecificdata: *const super::super::System::Com::SAFEARRAY) -> ::windows::core::Result<u32>;
}
#[cfg(feature = "Win32_System_Com")]
impl IMbnServiceActivationVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMbnServiceActivationImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMbnServiceActivationVtbl {
        unsafe extern "system" fn Activate<Impl: IMbnServiceActivationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, vendorspecificdata: *const super::super::System::Com::SAFEARRAY, requestid: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Activate(::core::mem::transmute_copy(&vendorspecificdata)) {
                ::core::result::Result::Ok(ok__) => {
                    *requestid = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), Activate: Activate::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMbnServiceActivation as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IMbnServiceActivationEventsImpl: Sized {
    fn OnActivationComplete(&mut self, serviceactivation: ::core::option::Option<IMbnServiceActivation>, vendorspecificdata: *const super::super::System::Com::SAFEARRAY, requestid: u32, status: ::windows::core::HRESULT, networkerror: u32) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl IMbnServiceActivationEventsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMbnServiceActivationEventsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMbnServiceActivationEventsVtbl {
        unsafe extern "system" fn OnActivationComplete<Impl: IMbnServiceActivationEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, serviceactivation: ::windows::core::RawPtr, vendorspecificdata: *const super::super::System::Com::SAFEARRAY, requestid: u32, status: ::windows::core::HRESULT, networkerror: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnActivationComplete(::core::mem::transmute(&serviceactivation), ::core::mem::transmute_copy(&vendorspecificdata), ::core::mem::transmute_copy(&requestid), ::core::mem::transmute_copy(&status), ::core::mem::transmute_copy(&networkerror)).into()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), OnActivationComplete: OnActivationComplete::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMbnServiceActivationEvents as ::windows::core::Interface>::IID
    }
}
pub trait IMbnSignalImpl: Sized {
    fn GetSignalStrength(&mut self) -> ::windows::core::Result<u32>;
    fn GetSignalError(&mut self) -> ::windows::core::Result<u32>;
}
impl IMbnSignalVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMbnSignalImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMbnSignalVtbl {
        unsafe extern "system" fn GetSignalStrength<Impl: IMbnSignalImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, signalstrength: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetSignalStrength() {
                ::core::result::Result::Ok(ok__) => {
                    *signalstrength = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSignalError<Impl: IMbnSignalImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, signalerror: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetSignalError() {
                ::core::result::Result::Ok(ok__) => {
                    *signalerror = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
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
    fn OnSignalStateChange(&mut self, newinterface: ::core::option::Option<IMbnSignal>) -> ::windows::core::Result<()>;
}
impl IMbnSignalEventsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMbnSignalEventsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMbnSignalEventsVtbl {
        unsafe extern "system" fn OnSignalStateChange<Impl: IMbnSignalEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newinterface: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnSignalStateChange(::core::mem::transmute(&newinterface)).into()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), OnSignalStateChange: OnSignalStateChange::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMbnSignalEvents as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub trait IMbnSmsImpl: Sized {
    fn GetSmsConfiguration(&mut self) -> ::windows::core::Result<IMbnSmsConfiguration>;
    fn SetSmsConfiguration(&mut self, smsconfiguration: ::core::option::Option<IMbnSmsConfiguration>) -> ::windows::core::Result<u32>;
    fn SmsSendPdu(&mut self, pdudata: super::super::Foundation::PWSTR, size: u8) -> ::windows::core::Result<u32>;
    fn SmsSendCdma(&mut self, address: super::super::Foundation::PWSTR, encoding: MBN_SMS_CDMA_ENCODING, language: MBN_SMS_CDMA_LANG, sizeincharacters: u32, message: *const super::super::System::Com::SAFEARRAY) -> ::windows::core::Result<u32>;
    fn SmsSendCdmaPdu(&mut self, message: *const super::super::System::Com::SAFEARRAY) -> ::windows::core::Result<u32>;
    fn SmsRead(&mut self, smsfilter: *const MBN_SMS_FILTER, smsformat: MBN_SMS_FORMAT) -> ::windows::core::Result<u32>;
    fn SmsDelete(&mut self, smsfilter: *const MBN_SMS_FILTER) -> ::windows::core::Result<u32>;
    fn GetSmsStatus(&mut self) -> ::windows::core::Result<MBN_SMS_STATUS_INFO>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl IMbnSmsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMbnSmsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMbnSmsVtbl {
        unsafe extern "system" fn GetSmsConfiguration<Impl: IMbnSmsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, smsconfiguration: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetSmsConfiguration() {
                ::core::result::Result::Ok(ok__) => {
                    *smsconfiguration = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSmsConfiguration<Impl: IMbnSmsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, smsconfiguration: ::windows::core::RawPtr, requestid: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetSmsConfiguration(::core::mem::transmute(&smsconfiguration)) {
                ::core::result::Result::Ok(ok__) => {
                    *requestid = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SmsSendPdu<Impl: IMbnSmsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdudata: super::super::Foundation::PWSTR, size: u8, requestid: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SmsSendPdu(::core::mem::transmute_copy(&pdudata), ::core::mem::transmute_copy(&size)) {
                ::core::result::Result::Ok(ok__) => {
                    *requestid = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SmsSendCdma<Impl: IMbnSmsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, address: super::super::Foundation::PWSTR, encoding: MBN_SMS_CDMA_ENCODING, language: MBN_SMS_CDMA_LANG, sizeincharacters: u32, message: *const super::super::System::Com::SAFEARRAY, requestid: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SmsSendCdma(::core::mem::transmute_copy(&address), ::core::mem::transmute_copy(&encoding), ::core::mem::transmute_copy(&language), ::core::mem::transmute_copy(&sizeincharacters), ::core::mem::transmute_copy(&message)) {
                ::core::result::Result::Ok(ok__) => {
                    *requestid = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SmsSendCdmaPdu<Impl: IMbnSmsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, message: *const super::super::System::Com::SAFEARRAY, requestid: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SmsSendCdmaPdu(::core::mem::transmute_copy(&message)) {
                ::core::result::Result::Ok(ok__) => {
                    *requestid = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SmsRead<Impl: IMbnSmsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, smsfilter: *const MBN_SMS_FILTER, smsformat: MBN_SMS_FORMAT, requestid: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SmsRead(::core::mem::transmute_copy(&smsfilter), ::core::mem::transmute_copy(&smsformat)) {
                ::core::result::Result::Ok(ok__) => {
                    *requestid = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SmsDelete<Impl: IMbnSmsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, smsfilter: *const MBN_SMS_FILTER, requestid: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SmsDelete(::core::mem::transmute_copy(&smsfilter)) {
                ::core::result::Result::Ok(ok__) => {
                    *requestid = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSmsStatus<Impl: IMbnSmsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, smsstatusinfo: *mut MBN_SMS_STATUS_INFO) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetSmsStatus() {
                ::core::result::Result::Ok(ok__) => {
                    *smsstatusinfo = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
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
    fn ServiceCenterAddress(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetServiceCenterAddress(&mut self, scaddress: super::super::Foundation::PWSTR) -> ::windows::core::Result<()>;
    fn MaxMessageIndex(&mut self) -> ::windows::core::Result<u32>;
    fn CdmaShortMsgSize(&mut self) -> ::windows::core::Result<u32>;
    fn SmsFormat(&mut self) -> ::windows::core::Result<MBN_SMS_FORMAT>;
    fn SetSmsFormat(&mut self, smsformat: MBN_SMS_FORMAT) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IMbnSmsConfigurationVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMbnSmsConfigurationImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMbnSmsConfigurationVtbl {
        unsafe extern "system" fn ServiceCenterAddress<Impl: IMbnSmsConfigurationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, scaddress: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ServiceCenterAddress() {
                ::core::result::Result::Ok(ok__) => {
                    *scaddress = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetServiceCenterAddress<Impl: IMbnSmsConfigurationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, scaddress: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetServiceCenterAddress(::core::mem::transmute_copy(&scaddress)).into()
        }
        unsafe extern "system" fn MaxMessageIndex<Impl: IMbnSmsConfigurationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MaxMessageIndex() {
                ::core::result::Result::Ok(ok__) => {
                    *index = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CdmaShortMsgSize<Impl: IMbnSmsConfigurationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, shortmsgsize: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CdmaShortMsgSize() {
                ::core::result::Result::Ok(ok__) => {
                    *shortmsgsize = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SmsFormat<Impl: IMbnSmsConfigurationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, smsformat: *mut MBN_SMS_FORMAT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SmsFormat() {
                ::core::result::Result::Ok(ok__) => {
                    *smsformat = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSmsFormat<Impl: IMbnSmsConfigurationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, smsformat: MBN_SMS_FORMAT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetSmsFormat(::core::mem::transmute_copy(&smsformat)).into()
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
    fn OnSmsConfigurationChange(&mut self, sms: ::core::option::Option<IMbnSms>) -> ::windows::core::Result<()>;
    fn OnSetSmsConfigurationComplete(&mut self, sms: ::core::option::Option<IMbnSms>, requestid: u32, status: ::windows::core::HRESULT) -> ::windows::core::Result<()>;
    fn OnSmsSendComplete(&mut self, sms: ::core::option::Option<IMbnSms>, requestid: u32, status: ::windows::core::HRESULT) -> ::windows::core::Result<()>;
    fn OnSmsReadComplete(&mut self, sms: ::core::option::Option<IMbnSms>, smsformat: MBN_SMS_FORMAT, readmsgs: *const super::super::System::Com::SAFEARRAY, moremsgs: i16, requestid: u32, status: ::windows::core::HRESULT) -> ::windows::core::Result<()>;
    fn OnSmsNewClass0Message(&mut self, sms: ::core::option::Option<IMbnSms>, smsformat: MBN_SMS_FORMAT, readmsgs: *const super::super::System::Com::SAFEARRAY) -> ::windows::core::Result<()>;
    fn OnSmsDeleteComplete(&mut self, sms: ::core::option::Option<IMbnSms>, requestid: u32, status: ::windows::core::HRESULT) -> ::windows::core::Result<()>;
    fn OnSmsStatusChange(&mut self, sms: ::core::option::Option<IMbnSms>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl IMbnSmsEventsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMbnSmsEventsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMbnSmsEventsVtbl {
        unsafe extern "system" fn OnSmsConfigurationChange<Impl: IMbnSmsEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, sms: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnSmsConfigurationChange(::core::mem::transmute(&sms)).into()
        }
        unsafe extern "system" fn OnSetSmsConfigurationComplete<Impl: IMbnSmsEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, sms: ::windows::core::RawPtr, requestid: u32, status: ::windows::core::HRESULT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnSetSmsConfigurationComplete(::core::mem::transmute(&sms), ::core::mem::transmute_copy(&requestid), ::core::mem::transmute_copy(&status)).into()
        }
        unsafe extern "system" fn OnSmsSendComplete<Impl: IMbnSmsEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, sms: ::windows::core::RawPtr, requestid: u32, status: ::windows::core::HRESULT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnSmsSendComplete(::core::mem::transmute(&sms), ::core::mem::transmute_copy(&requestid), ::core::mem::transmute_copy(&status)).into()
        }
        unsafe extern "system" fn OnSmsReadComplete<Impl: IMbnSmsEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, sms: ::windows::core::RawPtr, smsformat: MBN_SMS_FORMAT, readmsgs: *const super::super::System::Com::SAFEARRAY, moremsgs: i16, requestid: u32, status: ::windows::core::HRESULT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnSmsReadComplete(::core::mem::transmute(&sms), ::core::mem::transmute_copy(&smsformat), ::core::mem::transmute_copy(&readmsgs), ::core::mem::transmute_copy(&moremsgs), ::core::mem::transmute_copy(&requestid), ::core::mem::transmute_copy(&status)).into()
        }
        unsafe extern "system" fn OnSmsNewClass0Message<Impl: IMbnSmsEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, sms: ::windows::core::RawPtr, smsformat: MBN_SMS_FORMAT, readmsgs: *const super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnSmsNewClass0Message(::core::mem::transmute(&sms), ::core::mem::transmute_copy(&smsformat), ::core::mem::transmute_copy(&readmsgs)).into()
        }
        unsafe extern "system" fn OnSmsDeleteComplete<Impl: IMbnSmsEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, sms: ::windows::core::RawPtr, requestid: u32, status: ::windows::core::HRESULT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnSmsDeleteComplete(::core::mem::transmute(&sms), ::core::mem::transmute_copy(&requestid), ::core::mem::transmute_copy(&status)).into()
        }
        unsafe extern "system" fn OnSmsStatusChange<Impl: IMbnSmsEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, sms: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnSmsStatusChange(::core::mem::transmute(&sms)).into()
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
    fn Index(&mut self) -> ::windows::core::Result<u32>;
    fn Status(&mut self) -> ::windows::core::Result<MBN_MSG_STATUS>;
    fn PduData(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn Message(&mut self) -> ::windows::core::Result<*mut super::super::System::Com::SAFEARRAY>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl IMbnSmsReadMsgPduVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMbnSmsReadMsgPduImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMbnSmsReadMsgPduVtbl {
        unsafe extern "system" fn Index<Impl: IMbnSmsReadMsgPduImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Index() {
                ::core::result::Result::Ok(ok__) => {
                    *index = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Status<Impl: IMbnSmsReadMsgPduImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, status: *mut MBN_MSG_STATUS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Status() {
                ::core::result::Result::Ok(ok__) => {
                    *status = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PduData<Impl: IMbnSmsReadMsgPduImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdudata: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PduData() {
                ::core::result::Result::Ok(ok__) => {
                    *pdudata = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Message<Impl: IMbnSmsReadMsgPduImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, message: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Message() {
                ::core::result::Result::Ok(ok__) => {
                    *message = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
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
    fn Index(&mut self) -> ::windows::core::Result<u32>;
    fn Status(&mut self) -> ::windows::core::Result<MBN_MSG_STATUS>;
    fn Address(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn Timestamp(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn EncodingID(&mut self) -> ::windows::core::Result<MBN_SMS_CDMA_ENCODING>;
    fn LanguageID(&mut self) -> ::windows::core::Result<MBN_SMS_CDMA_LANG>;
    fn SizeInCharacters(&mut self) -> ::windows::core::Result<u32>;
    fn Message(&mut self) -> ::windows::core::Result<*mut super::super::System::Com::SAFEARRAY>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl IMbnSmsReadMsgTextCdmaVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMbnSmsReadMsgTextCdmaImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMbnSmsReadMsgTextCdmaVtbl {
        unsafe extern "system" fn Index<Impl: IMbnSmsReadMsgTextCdmaImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Index() {
                ::core::result::Result::Ok(ok__) => {
                    *index = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Status<Impl: IMbnSmsReadMsgTextCdmaImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, status: *mut MBN_MSG_STATUS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Status() {
                ::core::result::Result::Ok(ok__) => {
                    *status = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Address<Impl: IMbnSmsReadMsgTextCdmaImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, address: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Address() {
                ::core::result::Result::Ok(ok__) => {
                    *address = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Timestamp<Impl: IMbnSmsReadMsgTextCdmaImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, timestamp: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Timestamp() {
                ::core::result::Result::Ok(ok__) => {
                    *timestamp = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EncodingID<Impl: IMbnSmsReadMsgTextCdmaImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, encodingid: *mut MBN_SMS_CDMA_ENCODING) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EncodingID() {
                ::core::result::Result::Ok(ok__) => {
                    *encodingid = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LanguageID<Impl: IMbnSmsReadMsgTextCdmaImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, languageid: *mut MBN_SMS_CDMA_LANG) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LanguageID() {
                ::core::result::Result::Ok(ok__) => {
                    *languageid = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SizeInCharacters<Impl: IMbnSmsReadMsgTextCdmaImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, sizeincharacters: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SizeInCharacters() {
                ::core::result::Result::Ok(ok__) => {
                    *sizeincharacters = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Message<Impl: IMbnSmsReadMsgTextCdmaImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, message: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Message() {
                ::core::result::Result::Ok(ok__) => {
                    *message = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
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
    fn SubscriberID(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SimIccID(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn TelephoneNumbers(&mut self) -> ::windows::core::Result<*mut super::super::System::Com::SAFEARRAY>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl IMbnSubscriberInformationVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMbnSubscriberInformationImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMbnSubscriberInformationVtbl {
        unsafe extern "system" fn SubscriberID<Impl: IMbnSubscriberInformationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, subscriberid: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SubscriberID() {
                ::core::result::Result::Ok(ok__) => {
                    *subscriberid = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SimIccID<Impl: IMbnSubscriberInformationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, simiccid: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SimIccID() {
                ::core::result::Result::Ok(ok__) => {
                    *simiccid = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TelephoneNumbers<Impl: IMbnSubscriberInformationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, telephonenumbers: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TelephoneNumbers() {
                ::core::result::Result::Ok(ok__) => {
                    *telephonenumbers = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
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
    fn OnEventNotification(&mut self, vendoroperation: ::core::option::Option<IMbnVendorSpecificOperation>, vendorspecificdata: *const super::super::System::Com::SAFEARRAY) -> ::windows::core::Result<()>;
    fn OnSetVendorSpecificComplete(&mut self, vendoroperation: ::core::option::Option<IMbnVendorSpecificOperation>, vendorspecificdata: *const super::super::System::Com::SAFEARRAY, requestid: u32) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl IMbnVendorSpecificEventsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMbnVendorSpecificEventsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMbnVendorSpecificEventsVtbl {
        unsafe extern "system" fn OnEventNotification<Impl: IMbnVendorSpecificEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, vendoroperation: ::windows::core::RawPtr, vendorspecificdata: *const super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnEventNotification(::core::mem::transmute(&vendoroperation), ::core::mem::transmute_copy(&vendorspecificdata)).into()
        }
        unsafe extern "system" fn OnSetVendorSpecificComplete<Impl: IMbnVendorSpecificEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, vendoroperation: ::windows::core::RawPtr, vendorspecificdata: *const super::super::System::Com::SAFEARRAY, requestid: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnSetVendorSpecificComplete(::core::mem::transmute(&vendoroperation), ::core::mem::transmute_copy(&vendorspecificdata), ::core::mem::transmute_copy(&requestid)).into()
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
    fn SetVendorSpecific(&mut self, vendorspecificdata: *const super::super::System::Com::SAFEARRAY) -> ::windows::core::Result<u32>;
}
#[cfg(feature = "Win32_System_Com")]
impl IMbnVendorSpecificOperationVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMbnVendorSpecificOperationImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMbnVendorSpecificOperationVtbl {
        unsafe extern "system" fn SetVendorSpecific<Impl: IMbnVendorSpecificOperationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, vendorspecificdata: *const super::super::System::Com::SAFEARRAY, requestid: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetVendorSpecific(::core::mem::transmute_copy(&vendorspecificdata)) {
                ::core::result::Result::Ok(ok__) => {
                    *requestid = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), SetVendorSpecific: SetVendorSpecific::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMbnVendorSpecificOperation as ::windows::core::Interface>::IID
    }
}
