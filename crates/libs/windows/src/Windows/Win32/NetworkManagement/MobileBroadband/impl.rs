#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IDummyMBNUCMExt_Impl: Sized + super::super::System::Com::IDispatch_Impl {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IDummyMBNUCMExt_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDummyMBNUCMExt_Impl, const OFFSET: isize>() -> IDummyMBNUCMExt_Vtbl {
        Self { base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDummyMBNUCMExt as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IMbnConnection_Impl: Sized {
    fn ConnectionID(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn InterfaceID(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn Connect(&mut self, connectionmode: MBN_CONNECTION_MODE, strprofile: super::super::Foundation::PWSTR) -> ::windows::core::Result<u32>;
    fn Disconnect(&mut self) -> ::windows::core::Result<u32>;
    fn GetConnectionState(&mut self, connectionstate: *mut MBN_ACTIVATION_STATE, profilename: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn GetVoiceCallState(&mut self) -> ::windows::core::Result<MBN_VOICE_CALL_STATE>;
    fn GetActivationNetworkError(&mut self) -> ::windows::core::Result<u32>;
}
#[cfg(feature = "Win32_Foundation")]
impl IMbnConnection_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMbnConnection_Impl, const OFFSET: isize>() -> IMbnConnection_Vtbl {
        unsafe extern "system" fn ConnectionID<Identity: ::windows::core::IUnknownImpl, Impl: IMbnConnection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, connectionid: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).ConnectionID() {
                ::core::result::Result::Ok(ok__) => {
                    *connectionid = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn InterfaceID<Identity: ::windows::core::IUnknownImpl, Impl: IMbnConnection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, interfaceid: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).InterfaceID() {
                ::core::result::Result::Ok(ok__) => {
                    *interfaceid = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Connect<Identity: ::windows::core::IUnknownImpl, Impl: IMbnConnection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, connectionmode: MBN_CONNECTION_MODE, strprofile: super::super::Foundation::PWSTR, requestid: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Connect(::core::mem::transmute_copy(&connectionmode), ::core::mem::transmute_copy(&strprofile)) {
                ::core::result::Result::Ok(ok__) => {
                    *requestid = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Disconnect<Identity: ::windows::core::IUnknownImpl, Impl: IMbnConnection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, requestid: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Disconnect() {
                ::core::result::Result::Ok(ok__) => {
                    *requestid = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetConnectionState<Identity: ::windows::core::IUnknownImpl, Impl: IMbnConnection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, connectionstate: *mut MBN_ACTIVATION_STATE, profilename: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetConnectionState(::core::mem::transmute_copy(&connectionstate), ::core::mem::transmute_copy(&profilename)).into()
        }
        unsafe extern "system" fn GetVoiceCallState<Identity: ::windows::core::IUnknownImpl, Impl: IMbnConnection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, voicecallstate: *mut MBN_VOICE_CALL_STATE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetVoiceCallState() {
                ::core::result::Result::Ok(ok__) => {
                    *voicecallstate = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetActivationNetworkError<Identity: ::windows::core::IUnknownImpl, Impl: IMbnConnection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, networkerror: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetActivationNetworkError() {
                ::core::result::Result::Ok(ok__) => {
                    *networkerror = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            ConnectionID: ConnectionID::<Identity, Impl, OFFSET>,
            InterfaceID: InterfaceID::<Identity, Impl, OFFSET>,
            Connect: Connect::<Identity, Impl, OFFSET>,
            Disconnect: Disconnect::<Identity, Impl, OFFSET>,
            GetConnectionState: GetConnectionState::<Identity, Impl, OFFSET>,
            GetVoiceCallState: GetVoiceCallState::<Identity, Impl, OFFSET>,
            GetActivationNetworkError: GetActivationNetworkError::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMbnConnection as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub trait IMbnConnectionContext_Impl: Sized {
    fn GetProvisionedContexts(&mut self) -> ::windows::core::Result<*mut super::super::System::Com::SAFEARRAY>;
    fn SetProvisionedContext(&mut self, provisionedcontexts: &MBN_CONTEXT, providerid: super::super::Foundation::PWSTR) -> ::windows::core::Result<u32>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl IMbnConnectionContext_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMbnConnectionContext_Impl, const OFFSET: isize>() -> IMbnConnectionContext_Vtbl {
        unsafe extern "system" fn GetProvisionedContexts<Identity: ::windows::core::IUnknownImpl, Impl: IMbnConnectionContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, provisionedcontexts: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetProvisionedContexts() {
                ::core::result::Result::Ok(ok__) => {
                    *provisionedcontexts = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetProvisionedContext<Identity: ::windows::core::IUnknownImpl, Impl: IMbnConnectionContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, provisionedcontexts: ::core::mem::ManuallyDrop<MBN_CONTEXT>, providerid: super::super::Foundation::PWSTR, requestid: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).SetProvisionedContext(::core::mem::transmute_copy(&provisionedcontexts), ::core::mem::transmute_copy(&providerid)) {
                ::core::result::Result::Ok(ok__) => {
                    *requestid = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            GetProvisionedContexts: GetProvisionedContexts::<Identity, Impl, OFFSET>,
            SetProvisionedContext: SetProvisionedContext::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMbnConnectionContext as ::windows::core::Interface>::IID
    }
}
pub trait IMbnConnectionContextEvents_Impl: Sized {
    fn OnProvisionedContextListChange(&mut self, newinterface: &::core::option::Option<IMbnConnectionContext>) -> ::windows::core::Result<()>;
    fn OnSetProvisionedContextComplete(&mut self, newinterface: &::core::option::Option<IMbnConnectionContext>, requestid: u32, status: ::windows::core::HRESULT) -> ::windows::core::Result<()>;
}
impl IMbnConnectionContextEvents_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMbnConnectionContextEvents_Impl, const OFFSET: isize>() -> IMbnConnectionContextEvents_Vtbl {
        unsafe extern "system" fn OnProvisionedContextListChange<Identity: ::windows::core::IUnknownImpl, Impl: IMbnConnectionContextEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newinterface: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).OnProvisionedContextListChange(::core::mem::transmute(&newinterface)).into()
        }
        unsafe extern "system" fn OnSetProvisionedContextComplete<Identity: ::windows::core::IUnknownImpl, Impl: IMbnConnectionContextEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newinterface: ::windows::core::RawPtr, requestid: u32, status: ::windows::core::HRESULT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).OnSetProvisionedContextComplete(::core::mem::transmute(&newinterface), ::core::mem::transmute_copy(&requestid), ::core::mem::transmute_copy(&status)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            OnProvisionedContextListChange: OnProvisionedContextListChange::<Identity, Impl, OFFSET>,
            OnSetProvisionedContextComplete: OnSetProvisionedContextComplete::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMbnConnectionContextEvents as ::windows::core::Interface>::IID
    }
}
pub trait IMbnConnectionEvents_Impl: Sized {
    fn OnConnectComplete(&mut self, newconnection: &::core::option::Option<IMbnConnection>, requestid: u32, status: ::windows::core::HRESULT) -> ::windows::core::Result<()>;
    fn OnDisconnectComplete(&mut self, newconnection: &::core::option::Option<IMbnConnection>, requestid: u32, status: ::windows::core::HRESULT) -> ::windows::core::Result<()>;
    fn OnConnectStateChange(&mut self, newconnection: &::core::option::Option<IMbnConnection>) -> ::windows::core::Result<()>;
    fn OnVoiceCallStateChange(&mut self, newconnection: &::core::option::Option<IMbnConnection>) -> ::windows::core::Result<()>;
}
impl IMbnConnectionEvents_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMbnConnectionEvents_Impl, const OFFSET: isize>() -> IMbnConnectionEvents_Vtbl {
        unsafe extern "system" fn OnConnectComplete<Identity: ::windows::core::IUnknownImpl, Impl: IMbnConnectionEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newconnection: ::windows::core::RawPtr, requestid: u32, status: ::windows::core::HRESULT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).OnConnectComplete(::core::mem::transmute(&newconnection), ::core::mem::transmute_copy(&requestid), ::core::mem::transmute_copy(&status)).into()
        }
        unsafe extern "system" fn OnDisconnectComplete<Identity: ::windows::core::IUnknownImpl, Impl: IMbnConnectionEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newconnection: ::windows::core::RawPtr, requestid: u32, status: ::windows::core::HRESULT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).OnDisconnectComplete(::core::mem::transmute(&newconnection), ::core::mem::transmute_copy(&requestid), ::core::mem::transmute_copy(&status)).into()
        }
        unsafe extern "system" fn OnConnectStateChange<Identity: ::windows::core::IUnknownImpl, Impl: IMbnConnectionEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newconnection: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).OnConnectStateChange(::core::mem::transmute(&newconnection)).into()
        }
        unsafe extern "system" fn OnVoiceCallStateChange<Identity: ::windows::core::IUnknownImpl, Impl: IMbnConnectionEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newconnection: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).OnVoiceCallStateChange(::core::mem::transmute(&newconnection)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            OnConnectComplete: OnConnectComplete::<Identity, Impl, OFFSET>,
            OnDisconnectComplete: OnDisconnectComplete::<Identity, Impl, OFFSET>,
            OnConnectStateChange: OnConnectStateChange::<Identity, Impl, OFFSET>,
            OnVoiceCallStateChange: OnVoiceCallStateChange::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMbnConnectionEvents as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub trait IMbnConnectionManager_Impl: Sized {
    fn GetConnection(&mut self, connectionid: super::super::Foundation::PWSTR) -> ::windows::core::Result<IMbnConnection>;
    fn GetConnections(&mut self) -> ::windows::core::Result<*mut super::super::System::Com::SAFEARRAY>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl IMbnConnectionManager_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMbnConnectionManager_Impl, const OFFSET: isize>() -> IMbnConnectionManager_Vtbl {
        unsafe extern "system" fn GetConnection<Identity: ::windows::core::IUnknownImpl, Impl: IMbnConnectionManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, connectionid: super::super::Foundation::PWSTR, mbnconnection: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetConnection(::core::mem::transmute_copy(&connectionid)) {
                ::core::result::Result::Ok(ok__) => {
                    *mbnconnection = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetConnections<Identity: ::windows::core::IUnknownImpl, Impl: IMbnConnectionManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, mbnconnections: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetConnections() {
                ::core::result::Result::Ok(ok__) => {
                    *mbnconnections = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            GetConnection: GetConnection::<Identity, Impl, OFFSET>,
            GetConnections: GetConnections::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMbnConnectionManager as ::windows::core::Interface>::IID
    }
}
pub trait IMbnConnectionManagerEvents_Impl: Sized {
    fn OnConnectionArrival(&mut self, newconnection: &::core::option::Option<IMbnConnection>) -> ::windows::core::Result<()>;
    fn OnConnectionRemoval(&mut self, oldconnection: &::core::option::Option<IMbnConnection>) -> ::windows::core::Result<()>;
}
impl IMbnConnectionManagerEvents_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMbnConnectionManagerEvents_Impl, const OFFSET: isize>() -> IMbnConnectionManagerEvents_Vtbl {
        unsafe extern "system" fn OnConnectionArrival<Identity: ::windows::core::IUnknownImpl, Impl: IMbnConnectionManagerEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newconnection: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).OnConnectionArrival(::core::mem::transmute(&newconnection)).into()
        }
        unsafe extern "system" fn OnConnectionRemoval<Identity: ::windows::core::IUnknownImpl, Impl: IMbnConnectionManagerEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, oldconnection: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).OnConnectionRemoval(::core::mem::transmute(&oldconnection)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            OnConnectionArrival: OnConnectionArrival::<Identity, Impl, OFFSET>,
            OnConnectionRemoval: OnConnectionRemoval::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMbnConnectionManagerEvents as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IMbnConnectionProfile_Impl: Sized {
    fn GetProfileXmlData(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn UpdateProfile(&mut self, strprofile: super::super::Foundation::PWSTR) -> ::windows::core::Result<()>;
    fn Delete(&mut self) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IMbnConnectionProfile_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMbnConnectionProfile_Impl, const OFFSET: isize>() -> IMbnConnectionProfile_Vtbl {
        unsafe extern "system" fn GetProfileXmlData<Identity: ::windows::core::IUnknownImpl, Impl: IMbnConnectionProfile_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, profiledata: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetProfileXmlData() {
                ::core::result::Result::Ok(ok__) => {
                    *profiledata = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UpdateProfile<Identity: ::windows::core::IUnknownImpl, Impl: IMbnConnectionProfile_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strprofile: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).UpdateProfile(::core::mem::transmute_copy(&strprofile)).into()
        }
        unsafe extern "system" fn Delete<Identity: ::windows::core::IUnknownImpl, Impl: IMbnConnectionProfile_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Delete().into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            GetProfileXmlData: GetProfileXmlData::<Identity, Impl, OFFSET>,
            UpdateProfile: UpdateProfile::<Identity, Impl, OFFSET>,
            Delete: Delete::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMbnConnectionProfile as ::windows::core::Interface>::IID
    }
}
pub trait IMbnConnectionProfileEvents_Impl: Sized {
    fn OnProfileUpdate(&mut self, newprofile: &::core::option::Option<IMbnConnectionProfile>) -> ::windows::core::Result<()>;
}
impl IMbnConnectionProfileEvents_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMbnConnectionProfileEvents_Impl, const OFFSET: isize>() -> IMbnConnectionProfileEvents_Vtbl {
        unsafe extern "system" fn OnProfileUpdate<Identity: ::windows::core::IUnknownImpl, Impl: IMbnConnectionProfileEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newprofile: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).OnProfileUpdate(::core::mem::transmute(&newprofile)).into()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(), OnProfileUpdate: OnProfileUpdate::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMbnConnectionProfileEvents as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub trait IMbnConnectionProfileManager_Impl: Sized {
    fn GetConnectionProfiles(&mut self, mbninterface: &::core::option::Option<IMbnInterface>) -> ::windows::core::Result<*mut super::super::System::Com::SAFEARRAY>;
    fn GetConnectionProfile(&mut self, mbninterface: &::core::option::Option<IMbnInterface>, profilename: super::super::Foundation::PWSTR) -> ::windows::core::Result<IMbnConnectionProfile>;
    fn CreateConnectionProfile(&mut self, xmlprofile: super::super::Foundation::PWSTR) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl IMbnConnectionProfileManager_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMbnConnectionProfileManager_Impl, const OFFSET: isize>() -> IMbnConnectionProfileManager_Vtbl {
        unsafe extern "system" fn GetConnectionProfiles<Identity: ::windows::core::IUnknownImpl, Impl: IMbnConnectionProfileManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, mbninterface: ::windows::core::RawPtr, connectionprofiles: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetConnectionProfiles(::core::mem::transmute(&mbninterface)) {
                ::core::result::Result::Ok(ok__) => {
                    *connectionprofiles = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetConnectionProfile<Identity: ::windows::core::IUnknownImpl, Impl: IMbnConnectionProfileManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, mbninterface: ::windows::core::RawPtr, profilename: super::super::Foundation::PWSTR, connectionprofile: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetConnectionProfile(::core::mem::transmute(&mbninterface), ::core::mem::transmute_copy(&profilename)) {
                ::core::result::Result::Ok(ok__) => {
                    *connectionprofile = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateConnectionProfile<Identity: ::windows::core::IUnknownImpl, Impl: IMbnConnectionProfileManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, xmlprofile: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).CreateConnectionProfile(::core::mem::transmute_copy(&xmlprofile)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            GetConnectionProfiles: GetConnectionProfiles::<Identity, Impl, OFFSET>,
            GetConnectionProfile: GetConnectionProfile::<Identity, Impl, OFFSET>,
            CreateConnectionProfile: CreateConnectionProfile::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMbnConnectionProfileManager as ::windows::core::Interface>::IID
    }
}
pub trait IMbnConnectionProfileManagerEvents_Impl: Sized {
    fn OnConnectionProfileArrival(&mut self, newconnectionprofile: &::core::option::Option<IMbnConnectionProfile>) -> ::windows::core::Result<()>;
    fn OnConnectionProfileRemoval(&mut self, oldconnectionprofile: &::core::option::Option<IMbnConnectionProfile>) -> ::windows::core::Result<()>;
}
impl IMbnConnectionProfileManagerEvents_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMbnConnectionProfileManagerEvents_Impl, const OFFSET: isize>() -> IMbnConnectionProfileManagerEvents_Vtbl {
        unsafe extern "system" fn OnConnectionProfileArrival<Identity: ::windows::core::IUnknownImpl, Impl: IMbnConnectionProfileManagerEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newconnectionprofile: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).OnConnectionProfileArrival(::core::mem::transmute(&newconnectionprofile)).into()
        }
        unsafe extern "system" fn OnConnectionProfileRemoval<Identity: ::windows::core::IUnknownImpl, Impl: IMbnConnectionProfileManagerEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, oldconnectionprofile: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).OnConnectionProfileRemoval(::core::mem::transmute(&oldconnectionprofile)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            OnConnectionProfileArrival: OnConnectionProfileArrival::<Identity, Impl, OFFSET>,
            OnConnectionProfileRemoval: OnConnectionProfileRemoval::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMbnConnectionProfileManagerEvents as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub trait IMbnDeviceService_Impl: Sized {
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
impl IMbnDeviceService_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMbnDeviceService_Impl, const OFFSET: isize>() -> IMbnDeviceService_Vtbl {
        unsafe extern "system" fn QuerySupportedCommands<Identity: ::windows::core::IUnknownImpl, Impl: IMbnDeviceService_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, requestid: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).QuerySupportedCommands() {
                ::core::result::Result::Ok(ok__) => {
                    *requestid = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OpenCommandSession<Identity: ::windows::core::IUnknownImpl, Impl: IMbnDeviceService_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, requestid: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).OpenCommandSession() {
                ::core::result::Result::Ok(ok__) => {
                    *requestid = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CloseCommandSession<Identity: ::windows::core::IUnknownImpl, Impl: IMbnDeviceService_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, requestid: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CloseCommandSession() {
                ::core::result::Result::Ok(ok__) => {
                    *requestid = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCommand<Identity: ::windows::core::IUnknownImpl, Impl: IMbnDeviceService_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, commandid: u32, deviceservicedata: *const super::super::System::Com::SAFEARRAY, requestid: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).SetCommand(::core::mem::transmute_copy(&commandid), ::core::mem::transmute_copy(&deviceservicedata)) {
                ::core::result::Result::Ok(ok__) => {
                    *requestid = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn QueryCommand<Identity: ::windows::core::IUnknownImpl, Impl: IMbnDeviceService_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, commandid: u32, deviceservicedata: *const super::super::System::Com::SAFEARRAY, requestid: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).QueryCommand(::core::mem::transmute_copy(&commandid), ::core::mem::transmute_copy(&deviceservicedata)) {
                ::core::result::Result::Ok(ok__) => {
                    *requestid = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OpenDataSession<Identity: ::windows::core::IUnknownImpl, Impl: IMbnDeviceService_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, requestid: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).OpenDataSession() {
                ::core::result::Result::Ok(ok__) => {
                    *requestid = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CloseDataSession<Identity: ::windows::core::IUnknownImpl, Impl: IMbnDeviceService_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, requestid: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CloseDataSession() {
                ::core::result::Result::Ok(ok__) => {
                    *requestid = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn WriteData<Identity: ::windows::core::IUnknownImpl, Impl: IMbnDeviceService_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, deviceservicedata: *const super::super::System::Com::SAFEARRAY, requestid: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).WriteData(::core::mem::transmute_copy(&deviceservicedata)) {
                ::core::result::Result::Ok(ok__) => {
                    *requestid = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn InterfaceID<Identity: ::windows::core::IUnknownImpl, Impl: IMbnDeviceService_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, interfaceid: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).InterfaceID() {
                ::core::result::Result::Ok(ok__) => {
                    *interfaceid = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DeviceServiceID<Identity: ::windows::core::IUnknownImpl, Impl: IMbnDeviceService_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, deviceserviceid: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).DeviceServiceID() {
                ::core::result::Result::Ok(ok__) => {
                    *deviceserviceid = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsCommandSessionOpen<Identity: ::windows::core::IUnknownImpl, Impl: IMbnDeviceService_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).IsCommandSessionOpen() {
                ::core::result::Result::Ok(ok__) => {
                    *value = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsDataSessionOpen<Identity: ::windows::core::IUnknownImpl, Impl: IMbnDeviceService_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).IsDataSessionOpen() {
                ::core::result::Result::Ok(ok__) => {
                    *value = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            QuerySupportedCommands: QuerySupportedCommands::<Identity, Impl, OFFSET>,
            OpenCommandSession: OpenCommandSession::<Identity, Impl, OFFSET>,
            CloseCommandSession: CloseCommandSession::<Identity, Impl, OFFSET>,
            SetCommand: SetCommand::<Identity, Impl, OFFSET>,
            QueryCommand: QueryCommand::<Identity, Impl, OFFSET>,
            OpenDataSession: OpenDataSession::<Identity, Impl, OFFSET>,
            CloseDataSession: CloseDataSession::<Identity, Impl, OFFSET>,
            WriteData: WriteData::<Identity, Impl, OFFSET>,
            InterfaceID: InterfaceID::<Identity, Impl, OFFSET>,
            DeviceServiceID: DeviceServiceID::<Identity, Impl, OFFSET>,
            IsCommandSessionOpen: IsCommandSessionOpen::<Identity, Impl, OFFSET>,
            IsDataSessionOpen: IsDataSessionOpen::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMbnDeviceService as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IMbnDeviceServiceStateEvents_Impl: Sized {
    fn OnSessionsStateChange(&mut self, interfaceid: &super::super::Foundation::BSTR, statechange: MBN_DEVICE_SERVICE_SESSIONS_STATE) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IMbnDeviceServiceStateEvents_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMbnDeviceServiceStateEvents_Impl, const OFFSET: isize>() -> IMbnDeviceServiceStateEvents_Vtbl {
        unsafe extern "system" fn OnSessionsStateChange<Identity: ::windows::core::IUnknownImpl, Impl: IMbnDeviceServiceStateEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, interfaceid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, statechange: MBN_DEVICE_SERVICE_SESSIONS_STATE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).OnSessionsStateChange(::core::mem::transmute_copy(&interfaceid), ::core::mem::transmute_copy(&statechange)).into()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(), OnSessionsStateChange: OnSessionsStateChange::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMbnDeviceServiceStateEvents as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub trait IMbnDeviceServicesContext_Impl: Sized {
    fn EnumerateDeviceServices(&mut self) -> ::windows::core::Result<*mut super::super::System::Com::SAFEARRAY>;
    fn GetDeviceService(&mut self, deviceserviceid: &super::super::Foundation::BSTR) -> ::windows::core::Result<IMbnDeviceService>;
    fn MaxCommandSize(&mut self) -> ::windows::core::Result<u32>;
    fn MaxDataSize(&mut self) -> ::windows::core::Result<u32>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl IMbnDeviceServicesContext_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMbnDeviceServicesContext_Impl, const OFFSET: isize>() -> IMbnDeviceServicesContext_Vtbl {
        unsafe extern "system" fn EnumerateDeviceServices<Identity: ::windows::core::IUnknownImpl, Impl: IMbnDeviceServicesContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, deviceservices: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).EnumerateDeviceServices() {
                ::core::result::Result::Ok(ok__) => {
                    *deviceservices = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDeviceService<Identity: ::windows::core::IUnknownImpl, Impl: IMbnDeviceServicesContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, deviceserviceid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, mbndeviceservice: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetDeviceService(::core::mem::transmute_copy(&deviceserviceid)) {
                ::core::result::Result::Ok(ok__) => {
                    *mbndeviceservice = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MaxCommandSize<Identity: ::windows::core::IUnknownImpl, Impl: IMbnDeviceServicesContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, maxcommandsize: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).MaxCommandSize() {
                ::core::result::Result::Ok(ok__) => {
                    *maxcommandsize = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MaxDataSize<Identity: ::windows::core::IUnknownImpl, Impl: IMbnDeviceServicesContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, maxdatasize: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).MaxDataSize() {
                ::core::result::Result::Ok(ok__) => {
                    *maxdatasize = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            EnumerateDeviceServices: EnumerateDeviceServices::<Identity, Impl, OFFSET>,
            GetDeviceService: GetDeviceService::<Identity, Impl, OFFSET>,
            MaxCommandSize: MaxCommandSize::<Identity, Impl, OFFSET>,
            MaxDataSize: MaxDataSize::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMbnDeviceServicesContext as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub trait IMbnDeviceServicesEvents_Impl: Sized {
    fn OnQuerySupportedCommandsComplete(&mut self, deviceservice: &::core::option::Option<IMbnDeviceService>, commandidlist: *const super::super::System::Com::SAFEARRAY, status: ::windows::core::HRESULT, requestid: u32) -> ::windows::core::Result<()>;
    fn OnOpenCommandSessionComplete(&mut self, deviceservice: &::core::option::Option<IMbnDeviceService>, status: ::windows::core::HRESULT, requestid: u32) -> ::windows::core::Result<()>;
    fn OnCloseCommandSessionComplete(&mut self, deviceservice: &::core::option::Option<IMbnDeviceService>, status: ::windows::core::HRESULT, requestid: u32) -> ::windows::core::Result<()>;
    fn OnSetCommandComplete(&mut self, deviceservice: &::core::option::Option<IMbnDeviceService>, responseid: u32, deviceservicedata: *const super::super::System::Com::SAFEARRAY, status: ::windows::core::HRESULT, requestid: u32) -> ::windows::core::Result<()>;
    fn OnQueryCommandComplete(&mut self, deviceservice: &::core::option::Option<IMbnDeviceService>, responseid: u32, deviceservicedata: *const super::super::System::Com::SAFEARRAY, status: ::windows::core::HRESULT, requestid: u32) -> ::windows::core::Result<()>;
    fn OnEventNotification(&mut self, deviceservice: &::core::option::Option<IMbnDeviceService>, eventid: u32, deviceservicedata: *const super::super::System::Com::SAFEARRAY) -> ::windows::core::Result<()>;
    fn OnOpenDataSessionComplete(&mut self, deviceservice: &::core::option::Option<IMbnDeviceService>, status: ::windows::core::HRESULT, requestid: u32) -> ::windows::core::Result<()>;
    fn OnCloseDataSessionComplete(&mut self, deviceservice: &::core::option::Option<IMbnDeviceService>, status: ::windows::core::HRESULT, requestid: u32) -> ::windows::core::Result<()>;
    fn OnWriteDataComplete(&mut self, deviceservice: &::core::option::Option<IMbnDeviceService>, status: ::windows::core::HRESULT, requestid: u32) -> ::windows::core::Result<()>;
    fn OnReadData(&mut self, deviceservice: &::core::option::Option<IMbnDeviceService>, deviceservicedata: *const super::super::System::Com::SAFEARRAY) -> ::windows::core::Result<()>;
    fn OnInterfaceStateChange(&mut self, interfaceid: &super::super::Foundation::BSTR, statechange: MBN_DEVICE_SERVICES_INTERFACE_STATE) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl IMbnDeviceServicesEvents_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMbnDeviceServicesEvents_Impl, const OFFSET: isize>() -> IMbnDeviceServicesEvents_Vtbl {
        unsafe extern "system" fn OnQuerySupportedCommandsComplete<Identity: ::windows::core::IUnknownImpl, Impl: IMbnDeviceServicesEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, deviceservice: ::windows::core::RawPtr, commandidlist: *const super::super::System::Com::SAFEARRAY, status: ::windows::core::HRESULT, requestid: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).OnQuerySupportedCommandsComplete(::core::mem::transmute(&deviceservice), ::core::mem::transmute_copy(&commandidlist), ::core::mem::transmute_copy(&status), ::core::mem::transmute_copy(&requestid)).into()
        }
        unsafe extern "system" fn OnOpenCommandSessionComplete<Identity: ::windows::core::IUnknownImpl, Impl: IMbnDeviceServicesEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, deviceservice: ::windows::core::RawPtr, status: ::windows::core::HRESULT, requestid: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).OnOpenCommandSessionComplete(::core::mem::transmute(&deviceservice), ::core::mem::transmute_copy(&status), ::core::mem::transmute_copy(&requestid)).into()
        }
        unsafe extern "system" fn OnCloseCommandSessionComplete<Identity: ::windows::core::IUnknownImpl, Impl: IMbnDeviceServicesEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, deviceservice: ::windows::core::RawPtr, status: ::windows::core::HRESULT, requestid: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).OnCloseCommandSessionComplete(::core::mem::transmute(&deviceservice), ::core::mem::transmute_copy(&status), ::core::mem::transmute_copy(&requestid)).into()
        }
        unsafe extern "system" fn OnSetCommandComplete<Identity: ::windows::core::IUnknownImpl, Impl: IMbnDeviceServicesEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, deviceservice: ::windows::core::RawPtr, responseid: u32, deviceservicedata: *const super::super::System::Com::SAFEARRAY, status: ::windows::core::HRESULT, requestid: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).OnSetCommandComplete(::core::mem::transmute(&deviceservice), ::core::mem::transmute_copy(&responseid), ::core::mem::transmute_copy(&deviceservicedata), ::core::mem::transmute_copy(&status), ::core::mem::transmute_copy(&requestid)).into()
        }
        unsafe extern "system" fn OnQueryCommandComplete<Identity: ::windows::core::IUnknownImpl, Impl: IMbnDeviceServicesEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, deviceservice: ::windows::core::RawPtr, responseid: u32, deviceservicedata: *const super::super::System::Com::SAFEARRAY, status: ::windows::core::HRESULT, requestid: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).OnQueryCommandComplete(::core::mem::transmute(&deviceservice), ::core::mem::transmute_copy(&responseid), ::core::mem::transmute_copy(&deviceservicedata), ::core::mem::transmute_copy(&status), ::core::mem::transmute_copy(&requestid)).into()
        }
        unsafe extern "system" fn OnEventNotification<Identity: ::windows::core::IUnknownImpl, Impl: IMbnDeviceServicesEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, deviceservice: ::windows::core::RawPtr, eventid: u32, deviceservicedata: *const super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).OnEventNotification(::core::mem::transmute(&deviceservice), ::core::mem::transmute_copy(&eventid), ::core::mem::transmute_copy(&deviceservicedata)).into()
        }
        unsafe extern "system" fn OnOpenDataSessionComplete<Identity: ::windows::core::IUnknownImpl, Impl: IMbnDeviceServicesEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, deviceservice: ::windows::core::RawPtr, status: ::windows::core::HRESULT, requestid: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).OnOpenDataSessionComplete(::core::mem::transmute(&deviceservice), ::core::mem::transmute_copy(&status), ::core::mem::transmute_copy(&requestid)).into()
        }
        unsafe extern "system" fn OnCloseDataSessionComplete<Identity: ::windows::core::IUnknownImpl, Impl: IMbnDeviceServicesEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, deviceservice: ::windows::core::RawPtr, status: ::windows::core::HRESULT, requestid: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).OnCloseDataSessionComplete(::core::mem::transmute(&deviceservice), ::core::mem::transmute_copy(&status), ::core::mem::transmute_copy(&requestid)).into()
        }
        unsafe extern "system" fn OnWriteDataComplete<Identity: ::windows::core::IUnknownImpl, Impl: IMbnDeviceServicesEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, deviceservice: ::windows::core::RawPtr, status: ::windows::core::HRESULT, requestid: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).OnWriteDataComplete(::core::mem::transmute(&deviceservice), ::core::mem::transmute_copy(&status), ::core::mem::transmute_copy(&requestid)).into()
        }
        unsafe extern "system" fn OnReadData<Identity: ::windows::core::IUnknownImpl, Impl: IMbnDeviceServicesEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, deviceservice: ::windows::core::RawPtr, deviceservicedata: *const super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).OnReadData(::core::mem::transmute(&deviceservice), ::core::mem::transmute_copy(&deviceservicedata)).into()
        }
        unsafe extern "system" fn OnInterfaceStateChange<Identity: ::windows::core::IUnknownImpl, Impl: IMbnDeviceServicesEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, interfaceid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, statechange: MBN_DEVICE_SERVICES_INTERFACE_STATE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).OnInterfaceStateChange(::core::mem::transmute_copy(&interfaceid), ::core::mem::transmute_copy(&statechange)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            OnQuerySupportedCommandsComplete: OnQuerySupportedCommandsComplete::<Identity, Impl, OFFSET>,
            OnOpenCommandSessionComplete: OnOpenCommandSessionComplete::<Identity, Impl, OFFSET>,
            OnCloseCommandSessionComplete: OnCloseCommandSessionComplete::<Identity, Impl, OFFSET>,
            OnSetCommandComplete: OnSetCommandComplete::<Identity, Impl, OFFSET>,
            OnQueryCommandComplete: OnQueryCommandComplete::<Identity, Impl, OFFSET>,
            OnEventNotification: OnEventNotification::<Identity, Impl, OFFSET>,
            OnOpenDataSessionComplete: OnOpenDataSessionComplete::<Identity, Impl, OFFSET>,
            OnCloseDataSessionComplete: OnCloseDataSessionComplete::<Identity, Impl, OFFSET>,
            OnWriteDataComplete: OnWriteDataComplete::<Identity, Impl, OFFSET>,
            OnReadData: OnReadData::<Identity, Impl, OFFSET>,
            OnInterfaceStateChange: OnInterfaceStateChange::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMbnDeviceServicesEvents as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IMbnDeviceServicesManager_Impl: Sized {
    fn GetDeviceServicesContext(&mut self, networkinterfaceid: &super::super::Foundation::BSTR) -> ::windows::core::Result<IMbnDeviceServicesContext>;
}
#[cfg(feature = "Win32_Foundation")]
impl IMbnDeviceServicesManager_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMbnDeviceServicesManager_Impl, const OFFSET: isize>() -> IMbnDeviceServicesManager_Vtbl {
        unsafe extern "system" fn GetDeviceServicesContext<Identity: ::windows::core::IUnknownImpl, Impl: IMbnDeviceServicesManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, networkinterfaceid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, mbndevicescontext: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetDeviceServicesContext(::core::mem::transmute_copy(&networkinterfaceid)) {
                ::core::result::Result::Ok(ok__) => {
                    *mbndevicescontext = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(), GetDeviceServicesContext: GetDeviceServicesContext::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMbnDeviceServicesManager as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub trait IMbnInterface_Impl: Sized {
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
impl IMbnInterface_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMbnInterface_Impl, const OFFSET: isize>() -> IMbnInterface_Vtbl {
        unsafe extern "system" fn InterfaceID<Identity: ::windows::core::IUnknownImpl, Impl: IMbnInterface_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, interfaceid: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).InterfaceID() {
                ::core::result::Result::Ok(ok__) => {
                    *interfaceid = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetInterfaceCapability<Identity: ::windows::core::IUnknownImpl, Impl: IMbnInterface_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, interfacecaps: *mut MBN_INTERFACE_CAPS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetInterfaceCapability() {
                ::core::result::Result::Ok(ok__) => {
                    *interfacecaps = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSubscriberInformation<Identity: ::windows::core::IUnknownImpl, Impl: IMbnInterface_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, subscriberinformation: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetSubscriberInformation() {
                ::core::result::Result::Ok(ok__) => {
                    *subscriberinformation = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetReadyState<Identity: ::windows::core::IUnknownImpl, Impl: IMbnInterface_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, readystate: *mut MBN_READY_STATE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetReadyState() {
                ::core::result::Result::Ok(ok__) => {
                    *readystate = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn InEmergencyMode<Identity: ::windows::core::IUnknownImpl, Impl: IMbnInterface_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, emergencymode: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).InEmergencyMode() {
                ::core::result::Result::Ok(ok__) => {
                    *emergencymode = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetHomeProvider<Identity: ::windows::core::IUnknownImpl, Impl: IMbnInterface_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, homeprovider: *mut MBN_PROVIDER) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetHomeProvider() {
                ::core::result::Result::Ok(ok__) => {
                    *homeprovider = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPreferredProviders<Identity: ::windows::core::IUnknownImpl, Impl: IMbnInterface_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, preferredproviders: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetPreferredProviders() {
                ::core::result::Result::Ok(ok__) => {
                    *preferredproviders = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPreferredProviders<Identity: ::windows::core::IUnknownImpl, Impl: IMbnInterface_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, preferredproviders: *const super::super::System::Com::SAFEARRAY, requestid: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).SetPreferredProviders(::core::mem::transmute_copy(&preferredproviders)) {
                ::core::result::Result::Ok(ok__) => {
                    *requestid = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetVisibleProviders<Identity: ::windows::core::IUnknownImpl, Impl: IMbnInterface_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, age: *mut u32, visibleproviders: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetVisibleProviders(::core::mem::transmute_copy(&age), ::core::mem::transmute_copy(&visibleproviders)).into()
        }
        unsafe extern "system" fn ScanNetwork<Identity: ::windows::core::IUnknownImpl, Impl: IMbnInterface_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, requestid: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).ScanNetwork() {
                ::core::result::Result::Ok(ok__) => {
                    *requestid = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetConnection<Identity: ::windows::core::IUnknownImpl, Impl: IMbnInterface_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, mbnconnection: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetConnection() {
                ::core::result::Result::Ok(ok__) => {
                    *mbnconnection = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            InterfaceID: InterfaceID::<Identity, Impl, OFFSET>,
            GetInterfaceCapability: GetInterfaceCapability::<Identity, Impl, OFFSET>,
            GetSubscriberInformation: GetSubscriberInformation::<Identity, Impl, OFFSET>,
            GetReadyState: GetReadyState::<Identity, Impl, OFFSET>,
            InEmergencyMode: InEmergencyMode::<Identity, Impl, OFFSET>,
            GetHomeProvider: GetHomeProvider::<Identity, Impl, OFFSET>,
            GetPreferredProviders: GetPreferredProviders::<Identity, Impl, OFFSET>,
            SetPreferredProviders: SetPreferredProviders::<Identity, Impl, OFFSET>,
            GetVisibleProviders: GetVisibleProviders::<Identity, Impl, OFFSET>,
            ScanNetwork: ScanNetwork::<Identity, Impl, OFFSET>,
            GetConnection: GetConnection::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMbnInterface as ::windows::core::Interface>::IID
    }
}
pub trait IMbnInterfaceEvents_Impl: Sized {
    fn OnInterfaceCapabilityAvailable(&mut self, newinterface: &::core::option::Option<IMbnInterface>) -> ::windows::core::Result<()>;
    fn OnSubscriberInformationChange(&mut self, newinterface: &::core::option::Option<IMbnInterface>) -> ::windows::core::Result<()>;
    fn OnReadyStateChange(&mut self, newinterface: &::core::option::Option<IMbnInterface>) -> ::windows::core::Result<()>;
    fn OnEmergencyModeChange(&mut self, newinterface: &::core::option::Option<IMbnInterface>) -> ::windows::core::Result<()>;
    fn OnHomeProviderAvailable(&mut self, newinterface: &::core::option::Option<IMbnInterface>) -> ::windows::core::Result<()>;
    fn OnPreferredProvidersChange(&mut self, newinterface: &::core::option::Option<IMbnInterface>) -> ::windows::core::Result<()>;
    fn OnSetPreferredProvidersComplete(&mut self, newinterface: &::core::option::Option<IMbnInterface>, requestid: u32, status: ::windows::core::HRESULT) -> ::windows::core::Result<()>;
    fn OnScanNetworkComplete(&mut self, newinterface: &::core::option::Option<IMbnInterface>, requestid: u32, status: ::windows::core::HRESULT) -> ::windows::core::Result<()>;
}
impl IMbnInterfaceEvents_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMbnInterfaceEvents_Impl, const OFFSET: isize>() -> IMbnInterfaceEvents_Vtbl {
        unsafe extern "system" fn OnInterfaceCapabilityAvailable<Identity: ::windows::core::IUnknownImpl, Impl: IMbnInterfaceEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newinterface: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).OnInterfaceCapabilityAvailable(::core::mem::transmute(&newinterface)).into()
        }
        unsafe extern "system" fn OnSubscriberInformationChange<Identity: ::windows::core::IUnknownImpl, Impl: IMbnInterfaceEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newinterface: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).OnSubscriberInformationChange(::core::mem::transmute(&newinterface)).into()
        }
        unsafe extern "system" fn OnReadyStateChange<Identity: ::windows::core::IUnknownImpl, Impl: IMbnInterfaceEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newinterface: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).OnReadyStateChange(::core::mem::transmute(&newinterface)).into()
        }
        unsafe extern "system" fn OnEmergencyModeChange<Identity: ::windows::core::IUnknownImpl, Impl: IMbnInterfaceEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newinterface: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).OnEmergencyModeChange(::core::mem::transmute(&newinterface)).into()
        }
        unsafe extern "system" fn OnHomeProviderAvailable<Identity: ::windows::core::IUnknownImpl, Impl: IMbnInterfaceEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newinterface: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).OnHomeProviderAvailable(::core::mem::transmute(&newinterface)).into()
        }
        unsafe extern "system" fn OnPreferredProvidersChange<Identity: ::windows::core::IUnknownImpl, Impl: IMbnInterfaceEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newinterface: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).OnPreferredProvidersChange(::core::mem::transmute(&newinterface)).into()
        }
        unsafe extern "system" fn OnSetPreferredProvidersComplete<Identity: ::windows::core::IUnknownImpl, Impl: IMbnInterfaceEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newinterface: ::windows::core::RawPtr, requestid: u32, status: ::windows::core::HRESULT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).OnSetPreferredProvidersComplete(::core::mem::transmute(&newinterface), ::core::mem::transmute_copy(&requestid), ::core::mem::transmute_copy(&status)).into()
        }
        unsafe extern "system" fn OnScanNetworkComplete<Identity: ::windows::core::IUnknownImpl, Impl: IMbnInterfaceEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newinterface: ::windows::core::RawPtr, requestid: u32, status: ::windows::core::HRESULT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).OnScanNetworkComplete(::core::mem::transmute(&newinterface), ::core::mem::transmute_copy(&requestid), ::core::mem::transmute_copy(&status)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            OnInterfaceCapabilityAvailable: OnInterfaceCapabilityAvailable::<Identity, Impl, OFFSET>,
            OnSubscriberInformationChange: OnSubscriberInformationChange::<Identity, Impl, OFFSET>,
            OnReadyStateChange: OnReadyStateChange::<Identity, Impl, OFFSET>,
            OnEmergencyModeChange: OnEmergencyModeChange::<Identity, Impl, OFFSET>,
            OnHomeProviderAvailable: OnHomeProviderAvailable::<Identity, Impl, OFFSET>,
            OnPreferredProvidersChange: OnPreferredProvidersChange::<Identity, Impl, OFFSET>,
            OnSetPreferredProvidersComplete: OnSetPreferredProvidersComplete::<Identity, Impl, OFFSET>,
            OnScanNetworkComplete: OnScanNetworkComplete::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMbnInterfaceEvents as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub trait IMbnInterfaceManager_Impl: Sized {
    fn GetInterface(&mut self, interfaceid: super::super::Foundation::PWSTR) -> ::windows::core::Result<IMbnInterface>;
    fn GetInterfaces(&mut self) -> ::windows::core::Result<*mut super::super::System::Com::SAFEARRAY>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl IMbnInterfaceManager_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMbnInterfaceManager_Impl, const OFFSET: isize>() -> IMbnInterfaceManager_Vtbl {
        unsafe extern "system" fn GetInterface<Identity: ::windows::core::IUnknownImpl, Impl: IMbnInterfaceManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, interfaceid: super::super::Foundation::PWSTR, mbninterface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetInterface(::core::mem::transmute_copy(&interfaceid)) {
                ::core::result::Result::Ok(ok__) => {
                    *mbninterface = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetInterfaces<Identity: ::windows::core::IUnknownImpl, Impl: IMbnInterfaceManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, mbninterfaces: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetInterfaces() {
                ::core::result::Result::Ok(ok__) => {
                    *mbninterfaces = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            GetInterface: GetInterface::<Identity, Impl, OFFSET>,
            GetInterfaces: GetInterfaces::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMbnInterfaceManager as ::windows::core::Interface>::IID
    }
}
pub trait IMbnInterfaceManagerEvents_Impl: Sized {
    fn OnInterfaceArrival(&mut self, newinterface: &::core::option::Option<IMbnInterface>) -> ::windows::core::Result<()>;
    fn OnInterfaceRemoval(&mut self, oldinterface: &::core::option::Option<IMbnInterface>) -> ::windows::core::Result<()>;
}
impl IMbnInterfaceManagerEvents_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMbnInterfaceManagerEvents_Impl, const OFFSET: isize>() -> IMbnInterfaceManagerEvents_Vtbl {
        unsafe extern "system" fn OnInterfaceArrival<Identity: ::windows::core::IUnknownImpl, Impl: IMbnInterfaceManagerEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newinterface: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).OnInterfaceArrival(::core::mem::transmute(&newinterface)).into()
        }
        unsafe extern "system" fn OnInterfaceRemoval<Identity: ::windows::core::IUnknownImpl, Impl: IMbnInterfaceManagerEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, oldinterface: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).OnInterfaceRemoval(::core::mem::transmute(&oldinterface)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            OnInterfaceArrival: OnInterfaceArrival::<Identity, Impl, OFFSET>,
            OnInterfaceRemoval: OnInterfaceRemoval::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMbnInterfaceManagerEvents as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub trait IMbnMultiCarrier_Impl: Sized {
    fn SetHomeProvider(&mut self, homeprovider: *const MBN_PROVIDER2) -> ::windows::core::Result<u32>;
    fn GetPreferredProviders(&mut self) -> ::windows::core::Result<*mut super::super::System::Com::SAFEARRAY>;
    fn GetVisibleProviders(&mut self, age: *mut u32, visibleproviders: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::core::Result<()>;
    fn GetSupportedCellularClasses(&mut self) -> ::windows::core::Result<*mut super::super::System::Com::SAFEARRAY>;
    fn GetCurrentCellularClass(&mut self) -> ::windows::core::Result<MBN_CELLULAR_CLASS>;
    fn ScanNetwork(&mut self) -> ::windows::core::Result<u32>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl IMbnMultiCarrier_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMbnMultiCarrier_Impl, const OFFSET: isize>() -> IMbnMultiCarrier_Vtbl {
        unsafe extern "system" fn SetHomeProvider<Identity: ::windows::core::IUnknownImpl, Impl: IMbnMultiCarrier_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, homeprovider: *const MBN_PROVIDER2, requestid: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).SetHomeProvider(::core::mem::transmute_copy(&homeprovider)) {
                ::core::result::Result::Ok(ok__) => {
                    *requestid = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPreferredProviders<Identity: ::windows::core::IUnknownImpl, Impl: IMbnMultiCarrier_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, preferredmulticarrierproviders: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetPreferredProviders() {
                ::core::result::Result::Ok(ok__) => {
                    *preferredmulticarrierproviders = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetVisibleProviders<Identity: ::windows::core::IUnknownImpl, Impl: IMbnMultiCarrier_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, age: *mut u32, visibleproviders: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetVisibleProviders(::core::mem::transmute_copy(&age), ::core::mem::transmute_copy(&visibleproviders)).into()
        }
        unsafe extern "system" fn GetSupportedCellularClasses<Identity: ::windows::core::IUnknownImpl, Impl: IMbnMultiCarrier_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cellularclasses: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetSupportedCellularClasses() {
                ::core::result::Result::Ok(ok__) => {
                    *cellularclasses = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCurrentCellularClass<Identity: ::windows::core::IUnknownImpl, Impl: IMbnMultiCarrier_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, currentcellularclass: *mut MBN_CELLULAR_CLASS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetCurrentCellularClass() {
                ::core::result::Result::Ok(ok__) => {
                    *currentcellularclass = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ScanNetwork<Identity: ::windows::core::IUnknownImpl, Impl: IMbnMultiCarrier_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, requestid: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).ScanNetwork() {
                ::core::result::Result::Ok(ok__) => {
                    *requestid = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            SetHomeProvider: SetHomeProvider::<Identity, Impl, OFFSET>,
            GetPreferredProviders: GetPreferredProviders::<Identity, Impl, OFFSET>,
            GetVisibleProviders: GetVisibleProviders::<Identity, Impl, OFFSET>,
            GetSupportedCellularClasses: GetSupportedCellularClasses::<Identity, Impl, OFFSET>,
            GetCurrentCellularClass: GetCurrentCellularClass::<Identity, Impl, OFFSET>,
            ScanNetwork: ScanNetwork::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMbnMultiCarrier as ::windows::core::Interface>::IID
    }
}
pub trait IMbnMultiCarrierEvents_Impl: Sized {
    fn OnSetHomeProviderComplete(&mut self, mbninterface: &::core::option::Option<IMbnMultiCarrier>, requestid: u32, status: ::windows::core::HRESULT) -> ::windows::core::Result<()>;
    fn OnCurrentCellularClassChange(&mut self, mbninterface: &::core::option::Option<IMbnMultiCarrier>) -> ::windows::core::Result<()>;
    fn OnPreferredProvidersChange(&mut self, mbninterface: &::core::option::Option<IMbnMultiCarrier>) -> ::windows::core::Result<()>;
    fn OnScanNetworkComplete(&mut self, mbninterface: &::core::option::Option<IMbnMultiCarrier>, requestid: u32, status: ::windows::core::HRESULT) -> ::windows::core::Result<()>;
    fn OnInterfaceCapabilityChange(&mut self, mbninterface: &::core::option::Option<IMbnMultiCarrier>) -> ::windows::core::Result<()>;
}
impl IMbnMultiCarrierEvents_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMbnMultiCarrierEvents_Impl, const OFFSET: isize>() -> IMbnMultiCarrierEvents_Vtbl {
        unsafe extern "system" fn OnSetHomeProviderComplete<Identity: ::windows::core::IUnknownImpl, Impl: IMbnMultiCarrierEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, mbninterface: ::windows::core::RawPtr, requestid: u32, status: ::windows::core::HRESULT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).OnSetHomeProviderComplete(::core::mem::transmute(&mbninterface), ::core::mem::transmute_copy(&requestid), ::core::mem::transmute_copy(&status)).into()
        }
        unsafe extern "system" fn OnCurrentCellularClassChange<Identity: ::windows::core::IUnknownImpl, Impl: IMbnMultiCarrierEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, mbninterface: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).OnCurrentCellularClassChange(::core::mem::transmute(&mbninterface)).into()
        }
        unsafe extern "system" fn OnPreferredProvidersChange<Identity: ::windows::core::IUnknownImpl, Impl: IMbnMultiCarrierEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, mbninterface: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).OnPreferredProvidersChange(::core::mem::transmute(&mbninterface)).into()
        }
        unsafe extern "system" fn OnScanNetworkComplete<Identity: ::windows::core::IUnknownImpl, Impl: IMbnMultiCarrierEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, mbninterface: ::windows::core::RawPtr, requestid: u32, status: ::windows::core::HRESULT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).OnScanNetworkComplete(::core::mem::transmute(&mbninterface), ::core::mem::transmute_copy(&requestid), ::core::mem::transmute_copy(&status)).into()
        }
        unsafe extern "system" fn OnInterfaceCapabilityChange<Identity: ::windows::core::IUnknownImpl, Impl: IMbnMultiCarrierEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, mbninterface: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).OnInterfaceCapabilityChange(::core::mem::transmute(&mbninterface)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            OnSetHomeProviderComplete: OnSetHomeProviderComplete::<Identity, Impl, OFFSET>,
            OnCurrentCellularClassChange: OnCurrentCellularClassChange::<Identity, Impl, OFFSET>,
            OnPreferredProvidersChange: OnPreferredProvidersChange::<Identity, Impl, OFFSET>,
            OnScanNetworkComplete: OnScanNetworkComplete::<Identity, Impl, OFFSET>,
            OnInterfaceCapabilityChange: OnInterfaceCapabilityChange::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMbnMultiCarrierEvents as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IMbnPin_Impl: Sized {
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
impl IMbnPin_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMbnPin_Impl, const OFFSET: isize>() -> IMbnPin_Vtbl {
        unsafe extern "system" fn PinType<Identity: ::windows::core::IUnknownImpl, Impl: IMbnPin_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pintype: *mut MBN_PIN_TYPE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).PinType() {
                ::core::result::Result::Ok(ok__) => {
                    *pintype = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PinFormat<Identity: ::windows::core::IUnknownImpl, Impl: IMbnPin_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinformat: *mut MBN_PIN_FORMAT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).PinFormat() {
                ::core::result::Result::Ok(ok__) => {
                    *pinformat = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PinLengthMin<Identity: ::windows::core::IUnknownImpl, Impl: IMbnPin_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinlengthmin: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).PinLengthMin() {
                ::core::result::Result::Ok(ok__) => {
                    *pinlengthmin = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PinLengthMax<Identity: ::windows::core::IUnknownImpl, Impl: IMbnPin_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinlengthmax: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).PinLengthMax() {
                ::core::result::Result::Ok(ok__) => {
                    *pinlengthmax = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PinMode<Identity: ::windows::core::IUnknownImpl, Impl: IMbnPin_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinmode: *mut MBN_PIN_MODE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).PinMode() {
                ::core::result::Result::Ok(ok__) => {
                    *pinmode = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Enable<Identity: ::windows::core::IUnknownImpl, Impl: IMbnPin_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pin: super::super::Foundation::PWSTR, requestid: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Enable(::core::mem::transmute_copy(&pin)) {
                ::core::result::Result::Ok(ok__) => {
                    *requestid = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Disable<Identity: ::windows::core::IUnknownImpl, Impl: IMbnPin_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pin: super::super::Foundation::PWSTR, requestid: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Disable(::core::mem::transmute_copy(&pin)) {
                ::core::result::Result::Ok(ok__) => {
                    *requestid = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Enter<Identity: ::windows::core::IUnknownImpl, Impl: IMbnPin_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pin: super::super::Foundation::PWSTR, requestid: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Enter(::core::mem::transmute_copy(&pin)) {
                ::core::result::Result::Ok(ok__) => {
                    *requestid = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Change<Identity: ::windows::core::IUnknownImpl, Impl: IMbnPin_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pin: super::super::Foundation::PWSTR, newpin: super::super::Foundation::PWSTR, requestid: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Change(::core::mem::transmute_copy(&pin), ::core::mem::transmute_copy(&newpin)) {
                ::core::result::Result::Ok(ok__) => {
                    *requestid = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Unblock<Identity: ::windows::core::IUnknownImpl, Impl: IMbnPin_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, puk: super::super::Foundation::PWSTR, newpin: super::super::Foundation::PWSTR, requestid: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Unblock(::core::mem::transmute_copy(&puk), ::core::mem::transmute_copy(&newpin)) {
                ::core::result::Result::Ok(ok__) => {
                    *requestid = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPinManager<Identity: ::windows::core::IUnknownImpl, Impl: IMbnPin_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinmanager: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetPinManager() {
                ::core::result::Result::Ok(ok__) => {
                    *pinmanager = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            PinType: PinType::<Identity, Impl, OFFSET>,
            PinFormat: PinFormat::<Identity, Impl, OFFSET>,
            PinLengthMin: PinLengthMin::<Identity, Impl, OFFSET>,
            PinLengthMax: PinLengthMax::<Identity, Impl, OFFSET>,
            PinMode: PinMode::<Identity, Impl, OFFSET>,
            Enable: Enable::<Identity, Impl, OFFSET>,
            Disable: Disable::<Identity, Impl, OFFSET>,
            Enter: Enter::<Identity, Impl, OFFSET>,
            Change: Change::<Identity, Impl, OFFSET>,
            Unblock: Unblock::<Identity, Impl, OFFSET>,
            GetPinManager: GetPinManager::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMbnPin as ::windows::core::Interface>::IID
    }
}
pub trait IMbnPinEvents_Impl: Sized {
    fn OnEnableComplete(&mut self, pin: &::core::option::Option<IMbnPin>, pininfo: *const MBN_PIN_INFO, requestid: u32, status: ::windows::core::HRESULT) -> ::windows::core::Result<()>;
    fn OnDisableComplete(&mut self, pin: &::core::option::Option<IMbnPin>, pininfo: *const MBN_PIN_INFO, requestid: u32, status: ::windows::core::HRESULT) -> ::windows::core::Result<()>;
    fn OnEnterComplete(&mut self, pin: &::core::option::Option<IMbnPin>, pininfo: *const MBN_PIN_INFO, requestid: u32, status: ::windows::core::HRESULT) -> ::windows::core::Result<()>;
    fn OnChangeComplete(&mut self, pin: &::core::option::Option<IMbnPin>, pininfo: *const MBN_PIN_INFO, requestid: u32, status: ::windows::core::HRESULT) -> ::windows::core::Result<()>;
    fn OnUnblockComplete(&mut self, pin: &::core::option::Option<IMbnPin>, pininfo: *const MBN_PIN_INFO, requestid: u32, status: ::windows::core::HRESULT) -> ::windows::core::Result<()>;
}
impl IMbnPinEvents_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMbnPinEvents_Impl, const OFFSET: isize>() -> IMbnPinEvents_Vtbl {
        unsafe extern "system" fn OnEnableComplete<Identity: ::windows::core::IUnknownImpl, Impl: IMbnPinEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pin: ::windows::core::RawPtr, pininfo: *const MBN_PIN_INFO, requestid: u32, status: ::windows::core::HRESULT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).OnEnableComplete(::core::mem::transmute(&pin), ::core::mem::transmute_copy(&pininfo), ::core::mem::transmute_copy(&requestid), ::core::mem::transmute_copy(&status)).into()
        }
        unsafe extern "system" fn OnDisableComplete<Identity: ::windows::core::IUnknownImpl, Impl: IMbnPinEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pin: ::windows::core::RawPtr, pininfo: *const MBN_PIN_INFO, requestid: u32, status: ::windows::core::HRESULT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).OnDisableComplete(::core::mem::transmute(&pin), ::core::mem::transmute_copy(&pininfo), ::core::mem::transmute_copy(&requestid), ::core::mem::transmute_copy(&status)).into()
        }
        unsafe extern "system" fn OnEnterComplete<Identity: ::windows::core::IUnknownImpl, Impl: IMbnPinEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pin: ::windows::core::RawPtr, pininfo: *const MBN_PIN_INFO, requestid: u32, status: ::windows::core::HRESULT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).OnEnterComplete(::core::mem::transmute(&pin), ::core::mem::transmute_copy(&pininfo), ::core::mem::transmute_copy(&requestid), ::core::mem::transmute_copy(&status)).into()
        }
        unsafe extern "system" fn OnChangeComplete<Identity: ::windows::core::IUnknownImpl, Impl: IMbnPinEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pin: ::windows::core::RawPtr, pininfo: *const MBN_PIN_INFO, requestid: u32, status: ::windows::core::HRESULT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).OnChangeComplete(::core::mem::transmute(&pin), ::core::mem::transmute_copy(&pininfo), ::core::mem::transmute_copy(&requestid), ::core::mem::transmute_copy(&status)).into()
        }
        unsafe extern "system" fn OnUnblockComplete<Identity: ::windows::core::IUnknownImpl, Impl: IMbnPinEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pin: ::windows::core::RawPtr, pininfo: *const MBN_PIN_INFO, requestid: u32, status: ::windows::core::HRESULT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).OnUnblockComplete(::core::mem::transmute(&pin), ::core::mem::transmute_copy(&pininfo), ::core::mem::transmute_copy(&requestid), ::core::mem::transmute_copy(&status)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            OnEnableComplete: OnEnableComplete::<Identity, Impl, OFFSET>,
            OnDisableComplete: OnDisableComplete::<Identity, Impl, OFFSET>,
            OnEnterComplete: OnEnterComplete::<Identity, Impl, OFFSET>,
            OnChangeComplete: OnChangeComplete::<Identity, Impl, OFFSET>,
            OnUnblockComplete: OnUnblockComplete::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMbnPinEvents as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IMbnPinManager_Impl: Sized {
    fn GetPinList(&mut self) -> ::windows::core::Result<*mut super::super::System::Com::SAFEARRAY>;
    fn GetPin(&mut self, pintype: MBN_PIN_TYPE) -> ::windows::core::Result<IMbnPin>;
    fn GetPinState(&mut self) -> ::windows::core::Result<u32>;
}
#[cfg(feature = "Win32_System_Com")]
impl IMbnPinManager_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMbnPinManager_Impl, const OFFSET: isize>() -> IMbnPinManager_Vtbl {
        unsafe extern "system" fn GetPinList<Identity: ::windows::core::IUnknownImpl, Impl: IMbnPinManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinlist: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetPinList() {
                ::core::result::Result::Ok(ok__) => {
                    *pinlist = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPin<Identity: ::windows::core::IUnknownImpl, Impl: IMbnPinManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pintype: MBN_PIN_TYPE, pin: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetPin(::core::mem::transmute_copy(&pintype)) {
                ::core::result::Result::Ok(ok__) => {
                    *pin = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPinState<Identity: ::windows::core::IUnknownImpl, Impl: IMbnPinManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, requestid: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetPinState() {
                ::core::result::Result::Ok(ok__) => {
                    *requestid = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            GetPinList: GetPinList::<Identity, Impl, OFFSET>,
            GetPin: GetPin::<Identity, Impl, OFFSET>,
            GetPinState: GetPinState::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMbnPinManager as ::windows::core::Interface>::IID
    }
}
pub trait IMbnPinManagerEvents_Impl: Sized {
    fn OnPinListAvailable(&mut self, pinmanager: &::core::option::Option<IMbnPinManager>) -> ::windows::core::Result<()>;
    fn OnGetPinStateComplete(&mut self, pinmanager: &::core::option::Option<IMbnPinManager>, pininfo: &MBN_PIN_INFO, requestid: u32, status: ::windows::core::HRESULT) -> ::windows::core::Result<()>;
}
impl IMbnPinManagerEvents_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMbnPinManagerEvents_Impl, const OFFSET: isize>() -> IMbnPinManagerEvents_Vtbl {
        unsafe extern "system" fn OnPinListAvailable<Identity: ::windows::core::IUnknownImpl, Impl: IMbnPinManagerEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinmanager: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).OnPinListAvailable(::core::mem::transmute(&pinmanager)).into()
        }
        unsafe extern "system" fn OnGetPinStateComplete<Identity: ::windows::core::IUnknownImpl, Impl: IMbnPinManagerEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinmanager: ::windows::core::RawPtr, pininfo: MBN_PIN_INFO, requestid: u32, status: ::windows::core::HRESULT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).OnGetPinStateComplete(::core::mem::transmute(&pinmanager), ::core::mem::transmute_copy(&pininfo), ::core::mem::transmute_copy(&requestid), ::core::mem::transmute_copy(&status)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            OnPinListAvailable: OnPinListAvailable::<Identity, Impl, OFFSET>,
            OnGetPinStateComplete: OnGetPinStateComplete::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMbnPinManagerEvents as ::windows::core::Interface>::IID
    }
}
pub trait IMbnRadio_Impl: Sized {
    fn SoftwareRadioState(&mut self) -> ::windows::core::Result<MBN_RADIO>;
    fn HardwareRadioState(&mut self) -> ::windows::core::Result<MBN_RADIO>;
    fn SetSoftwareRadioState(&mut self, radiostate: MBN_RADIO) -> ::windows::core::Result<u32>;
}
impl IMbnRadio_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMbnRadio_Impl, const OFFSET: isize>() -> IMbnRadio_Vtbl {
        unsafe extern "system" fn SoftwareRadioState<Identity: ::windows::core::IUnknownImpl, Impl: IMbnRadio_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, softwareradiostate: *mut MBN_RADIO) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).SoftwareRadioState() {
                ::core::result::Result::Ok(ok__) => {
                    *softwareradiostate = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn HardwareRadioState<Identity: ::windows::core::IUnknownImpl, Impl: IMbnRadio_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hardwareradiostate: *mut MBN_RADIO) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).HardwareRadioState() {
                ::core::result::Result::Ok(ok__) => {
                    *hardwareradiostate = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSoftwareRadioState<Identity: ::windows::core::IUnknownImpl, Impl: IMbnRadio_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, radiostate: MBN_RADIO, requestid: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).SetSoftwareRadioState(::core::mem::transmute_copy(&radiostate)) {
                ::core::result::Result::Ok(ok__) => {
                    *requestid = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            SoftwareRadioState: SoftwareRadioState::<Identity, Impl, OFFSET>,
            HardwareRadioState: HardwareRadioState::<Identity, Impl, OFFSET>,
            SetSoftwareRadioState: SetSoftwareRadioState::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMbnRadio as ::windows::core::Interface>::IID
    }
}
pub trait IMbnRadioEvents_Impl: Sized {
    fn OnRadioStateChange(&mut self, newinterface: &::core::option::Option<IMbnRadio>) -> ::windows::core::Result<()>;
    fn OnSetSoftwareRadioStateComplete(&mut self, newinterface: &::core::option::Option<IMbnRadio>, requestid: u32, status: ::windows::core::HRESULT) -> ::windows::core::Result<()>;
}
impl IMbnRadioEvents_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMbnRadioEvents_Impl, const OFFSET: isize>() -> IMbnRadioEvents_Vtbl {
        unsafe extern "system" fn OnRadioStateChange<Identity: ::windows::core::IUnknownImpl, Impl: IMbnRadioEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newinterface: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).OnRadioStateChange(::core::mem::transmute(&newinterface)).into()
        }
        unsafe extern "system" fn OnSetSoftwareRadioStateComplete<Identity: ::windows::core::IUnknownImpl, Impl: IMbnRadioEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newinterface: ::windows::core::RawPtr, requestid: u32, status: ::windows::core::HRESULT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).OnSetSoftwareRadioStateComplete(::core::mem::transmute(&newinterface), ::core::mem::transmute_copy(&requestid), ::core::mem::transmute_copy(&status)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            OnRadioStateChange: OnRadioStateChange::<Identity, Impl, OFFSET>,
            OnSetSoftwareRadioStateComplete: OnSetSoftwareRadioStateComplete::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMbnRadioEvents as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IMbnRegistration_Impl: Sized {
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
impl IMbnRegistration_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMbnRegistration_Impl, const OFFSET: isize>() -> IMbnRegistration_Vtbl {
        unsafe extern "system" fn GetRegisterState<Identity: ::windows::core::IUnknownImpl, Impl: IMbnRegistration_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, registerstate: *mut MBN_REGISTER_STATE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetRegisterState() {
                ::core::result::Result::Ok(ok__) => {
                    *registerstate = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetRegisterMode<Identity: ::windows::core::IUnknownImpl, Impl: IMbnRegistration_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, registermode: *mut MBN_REGISTER_MODE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetRegisterMode() {
                ::core::result::Result::Ok(ok__) => {
                    *registermode = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetProviderID<Identity: ::windows::core::IUnknownImpl, Impl: IMbnRegistration_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, providerid: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetProviderID() {
                ::core::result::Result::Ok(ok__) => {
                    *providerid = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetProviderName<Identity: ::windows::core::IUnknownImpl, Impl: IMbnRegistration_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, providername: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetProviderName() {
                ::core::result::Result::Ok(ok__) => {
                    *providername = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetRoamingText<Identity: ::windows::core::IUnknownImpl, Impl: IMbnRegistration_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, roamingtext: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetRoamingText() {
                ::core::result::Result::Ok(ok__) => {
                    *roamingtext = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetAvailableDataClasses<Identity: ::windows::core::IUnknownImpl, Impl: IMbnRegistration_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, availabledataclasses: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetAvailableDataClasses() {
                ::core::result::Result::Ok(ok__) => {
                    *availabledataclasses = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCurrentDataClass<Identity: ::windows::core::IUnknownImpl, Impl: IMbnRegistration_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, currentdataclass: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetCurrentDataClass() {
                ::core::result::Result::Ok(ok__) => {
                    *currentdataclass = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetRegistrationNetworkError<Identity: ::windows::core::IUnknownImpl, Impl: IMbnRegistration_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, registrationnetworkerror: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetRegistrationNetworkError() {
                ::core::result::Result::Ok(ok__) => {
                    *registrationnetworkerror = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPacketAttachNetworkError<Identity: ::windows::core::IUnknownImpl, Impl: IMbnRegistration_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, packetattachnetworkerror: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetPacketAttachNetworkError() {
                ::core::result::Result::Ok(ok__) => {
                    *packetattachnetworkerror = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetRegisterMode<Identity: ::windows::core::IUnknownImpl, Impl: IMbnRegistration_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, registermode: MBN_REGISTER_MODE, providerid: super::super::Foundation::PWSTR, dataclass: u32, requestid: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).SetRegisterMode(::core::mem::transmute_copy(&registermode), ::core::mem::transmute_copy(&providerid), ::core::mem::transmute_copy(&dataclass)) {
                ::core::result::Result::Ok(ok__) => {
                    *requestid = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            GetRegisterState: GetRegisterState::<Identity, Impl, OFFSET>,
            GetRegisterMode: GetRegisterMode::<Identity, Impl, OFFSET>,
            GetProviderID: GetProviderID::<Identity, Impl, OFFSET>,
            GetProviderName: GetProviderName::<Identity, Impl, OFFSET>,
            GetRoamingText: GetRoamingText::<Identity, Impl, OFFSET>,
            GetAvailableDataClasses: GetAvailableDataClasses::<Identity, Impl, OFFSET>,
            GetCurrentDataClass: GetCurrentDataClass::<Identity, Impl, OFFSET>,
            GetRegistrationNetworkError: GetRegistrationNetworkError::<Identity, Impl, OFFSET>,
            GetPacketAttachNetworkError: GetPacketAttachNetworkError::<Identity, Impl, OFFSET>,
            SetRegisterMode: SetRegisterMode::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMbnRegistration as ::windows::core::Interface>::IID
    }
}
pub trait IMbnRegistrationEvents_Impl: Sized {
    fn OnRegisterModeAvailable(&mut self, newinterface: &::core::option::Option<IMbnRegistration>) -> ::windows::core::Result<()>;
    fn OnRegisterStateChange(&mut self, newinterface: &::core::option::Option<IMbnRegistration>) -> ::windows::core::Result<()>;
    fn OnPacketServiceStateChange(&mut self, newinterface: &::core::option::Option<IMbnRegistration>) -> ::windows::core::Result<()>;
    fn OnSetRegisterModeComplete(&mut self, newinterface: &::core::option::Option<IMbnRegistration>, requestid: u32, status: ::windows::core::HRESULT) -> ::windows::core::Result<()>;
}
impl IMbnRegistrationEvents_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMbnRegistrationEvents_Impl, const OFFSET: isize>() -> IMbnRegistrationEvents_Vtbl {
        unsafe extern "system" fn OnRegisterModeAvailable<Identity: ::windows::core::IUnknownImpl, Impl: IMbnRegistrationEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newinterface: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).OnRegisterModeAvailable(::core::mem::transmute(&newinterface)).into()
        }
        unsafe extern "system" fn OnRegisterStateChange<Identity: ::windows::core::IUnknownImpl, Impl: IMbnRegistrationEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newinterface: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).OnRegisterStateChange(::core::mem::transmute(&newinterface)).into()
        }
        unsafe extern "system" fn OnPacketServiceStateChange<Identity: ::windows::core::IUnknownImpl, Impl: IMbnRegistrationEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newinterface: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).OnPacketServiceStateChange(::core::mem::transmute(&newinterface)).into()
        }
        unsafe extern "system" fn OnSetRegisterModeComplete<Identity: ::windows::core::IUnknownImpl, Impl: IMbnRegistrationEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newinterface: ::windows::core::RawPtr, requestid: u32, status: ::windows::core::HRESULT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).OnSetRegisterModeComplete(::core::mem::transmute(&newinterface), ::core::mem::transmute_copy(&requestid), ::core::mem::transmute_copy(&status)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            OnRegisterModeAvailable: OnRegisterModeAvailable::<Identity, Impl, OFFSET>,
            OnRegisterStateChange: OnRegisterStateChange::<Identity, Impl, OFFSET>,
            OnPacketServiceStateChange: OnPacketServiceStateChange::<Identity, Impl, OFFSET>,
            OnSetRegisterModeComplete: OnSetRegisterModeComplete::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMbnRegistrationEvents as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IMbnServiceActivation_Impl: Sized {
    fn Activate(&mut self, vendorspecificdata: *const super::super::System::Com::SAFEARRAY) -> ::windows::core::Result<u32>;
}
#[cfg(feature = "Win32_System_Com")]
impl IMbnServiceActivation_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMbnServiceActivation_Impl, const OFFSET: isize>() -> IMbnServiceActivation_Vtbl {
        unsafe extern "system" fn Activate<Identity: ::windows::core::IUnknownImpl, Impl: IMbnServiceActivation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, vendorspecificdata: *const super::super::System::Com::SAFEARRAY, requestid: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Activate(::core::mem::transmute_copy(&vendorspecificdata)) {
                ::core::result::Result::Ok(ok__) => {
                    *requestid = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(), Activate: Activate::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMbnServiceActivation as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IMbnServiceActivationEvents_Impl: Sized {
    fn OnActivationComplete(&mut self, serviceactivation: &::core::option::Option<IMbnServiceActivation>, vendorspecificdata: *const super::super::System::Com::SAFEARRAY, requestid: u32, status: ::windows::core::HRESULT, networkerror: u32) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl IMbnServiceActivationEvents_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMbnServiceActivationEvents_Impl, const OFFSET: isize>() -> IMbnServiceActivationEvents_Vtbl {
        unsafe extern "system" fn OnActivationComplete<Identity: ::windows::core::IUnknownImpl, Impl: IMbnServiceActivationEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, serviceactivation: ::windows::core::RawPtr, vendorspecificdata: *const super::super::System::Com::SAFEARRAY, requestid: u32, status: ::windows::core::HRESULT, networkerror: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).OnActivationComplete(::core::mem::transmute(&serviceactivation), ::core::mem::transmute_copy(&vendorspecificdata), ::core::mem::transmute_copy(&requestid), ::core::mem::transmute_copy(&status), ::core::mem::transmute_copy(&networkerror)).into()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(), OnActivationComplete: OnActivationComplete::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMbnServiceActivationEvents as ::windows::core::Interface>::IID
    }
}
pub trait IMbnSignal_Impl: Sized {
    fn GetSignalStrength(&mut self) -> ::windows::core::Result<u32>;
    fn GetSignalError(&mut self) -> ::windows::core::Result<u32>;
}
impl IMbnSignal_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMbnSignal_Impl, const OFFSET: isize>() -> IMbnSignal_Vtbl {
        unsafe extern "system" fn GetSignalStrength<Identity: ::windows::core::IUnknownImpl, Impl: IMbnSignal_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, signalstrength: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetSignalStrength() {
                ::core::result::Result::Ok(ok__) => {
                    *signalstrength = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSignalError<Identity: ::windows::core::IUnknownImpl, Impl: IMbnSignal_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, signalerror: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetSignalError() {
                ::core::result::Result::Ok(ok__) => {
                    *signalerror = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            GetSignalStrength: GetSignalStrength::<Identity, Impl, OFFSET>,
            GetSignalError: GetSignalError::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMbnSignal as ::windows::core::Interface>::IID
    }
}
pub trait IMbnSignalEvents_Impl: Sized {
    fn OnSignalStateChange(&mut self, newinterface: &::core::option::Option<IMbnSignal>) -> ::windows::core::Result<()>;
}
impl IMbnSignalEvents_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMbnSignalEvents_Impl, const OFFSET: isize>() -> IMbnSignalEvents_Vtbl {
        unsafe extern "system" fn OnSignalStateChange<Identity: ::windows::core::IUnknownImpl, Impl: IMbnSignalEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newinterface: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).OnSignalStateChange(::core::mem::transmute(&newinterface)).into()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(), OnSignalStateChange: OnSignalStateChange::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMbnSignalEvents as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub trait IMbnSms_Impl: Sized {
    fn GetSmsConfiguration(&mut self) -> ::windows::core::Result<IMbnSmsConfiguration>;
    fn SetSmsConfiguration(&mut self, smsconfiguration: &::core::option::Option<IMbnSmsConfiguration>) -> ::windows::core::Result<u32>;
    fn SmsSendPdu(&mut self, pdudata: super::super::Foundation::PWSTR, size: u8) -> ::windows::core::Result<u32>;
    fn SmsSendCdma(&mut self, address: super::super::Foundation::PWSTR, encoding: MBN_SMS_CDMA_ENCODING, language: MBN_SMS_CDMA_LANG, sizeincharacters: u32, message: *const super::super::System::Com::SAFEARRAY) -> ::windows::core::Result<u32>;
    fn SmsSendCdmaPdu(&mut self, message: *const super::super::System::Com::SAFEARRAY) -> ::windows::core::Result<u32>;
    fn SmsRead(&mut self, smsfilter: *const MBN_SMS_FILTER, smsformat: MBN_SMS_FORMAT) -> ::windows::core::Result<u32>;
    fn SmsDelete(&mut self, smsfilter: *const MBN_SMS_FILTER) -> ::windows::core::Result<u32>;
    fn GetSmsStatus(&mut self) -> ::windows::core::Result<MBN_SMS_STATUS_INFO>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl IMbnSms_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMbnSms_Impl, const OFFSET: isize>() -> IMbnSms_Vtbl {
        unsafe extern "system" fn GetSmsConfiguration<Identity: ::windows::core::IUnknownImpl, Impl: IMbnSms_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, smsconfiguration: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetSmsConfiguration() {
                ::core::result::Result::Ok(ok__) => {
                    *smsconfiguration = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSmsConfiguration<Identity: ::windows::core::IUnknownImpl, Impl: IMbnSms_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, smsconfiguration: ::windows::core::RawPtr, requestid: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).SetSmsConfiguration(::core::mem::transmute(&smsconfiguration)) {
                ::core::result::Result::Ok(ok__) => {
                    *requestid = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SmsSendPdu<Identity: ::windows::core::IUnknownImpl, Impl: IMbnSms_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdudata: super::super::Foundation::PWSTR, size: u8, requestid: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).SmsSendPdu(::core::mem::transmute_copy(&pdudata), ::core::mem::transmute_copy(&size)) {
                ::core::result::Result::Ok(ok__) => {
                    *requestid = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SmsSendCdma<Identity: ::windows::core::IUnknownImpl, Impl: IMbnSms_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, address: super::super::Foundation::PWSTR, encoding: MBN_SMS_CDMA_ENCODING, language: MBN_SMS_CDMA_LANG, sizeincharacters: u32, message: *const super::super::System::Com::SAFEARRAY, requestid: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).SmsSendCdma(::core::mem::transmute_copy(&address), ::core::mem::transmute_copy(&encoding), ::core::mem::transmute_copy(&language), ::core::mem::transmute_copy(&sizeincharacters), ::core::mem::transmute_copy(&message)) {
                ::core::result::Result::Ok(ok__) => {
                    *requestid = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SmsSendCdmaPdu<Identity: ::windows::core::IUnknownImpl, Impl: IMbnSms_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, message: *const super::super::System::Com::SAFEARRAY, requestid: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).SmsSendCdmaPdu(::core::mem::transmute_copy(&message)) {
                ::core::result::Result::Ok(ok__) => {
                    *requestid = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SmsRead<Identity: ::windows::core::IUnknownImpl, Impl: IMbnSms_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, smsfilter: *const MBN_SMS_FILTER, smsformat: MBN_SMS_FORMAT, requestid: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).SmsRead(::core::mem::transmute_copy(&smsfilter), ::core::mem::transmute_copy(&smsformat)) {
                ::core::result::Result::Ok(ok__) => {
                    *requestid = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SmsDelete<Identity: ::windows::core::IUnknownImpl, Impl: IMbnSms_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, smsfilter: *const MBN_SMS_FILTER, requestid: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).SmsDelete(::core::mem::transmute_copy(&smsfilter)) {
                ::core::result::Result::Ok(ok__) => {
                    *requestid = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSmsStatus<Identity: ::windows::core::IUnknownImpl, Impl: IMbnSms_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, smsstatusinfo: *mut MBN_SMS_STATUS_INFO) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetSmsStatus() {
                ::core::result::Result::Ok(ok__) => {
                    *smsstatusinfo = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            GetSmsConfiguration: GetSmsConfiguration::<Identity, Impl, OFFSET>,
            SetSmsConfiguration: SetSmsConfiguration::<Identity, Impl, OFFSET>,
            SmsSendPdu: SmsSendPdu::<Identity, Impl, OFFSET>,
            SmsSendCdma: SmsSendCdma::<Identity, Impl, OFFSET>,
            SmsSendCdmaPdu: SmsSendCdmaPdu::<Identity, Impl, OFFSET>,
            SmsRead: SmsRead::<Identity, Impl, OFFSET>,
            SmsDelete: SmsDelete::<Identity, Impl, OFFSET>,
            GetSmsStatus: GetSmsStatus::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMbnSms as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IMbnSmsConfiguration_Impl: Sized {
    fn ServiceCenterAddress(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetServiceCenterAddress(&mut self, scaddress: super::super::Foundation::PWSTR) -> ::windows::core::Result<()>;
    fn MaxMessageIndex(&mut self) -> ::windows::core::Result<u32>;
    fn CdmaShortMsgSize(&mut self) -> ::windows::core::Result<u32>;
    fn SmsFormat(&mut self) -> ::windows::core::Result<MBN_SMS_FORMAT>;
    fn SetSmsFormat(&mut self, smsformat: MBN_SMS_FORMAT) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IMbnSmsConfiguration_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMbnSmsConfiguration_Impl, const OFFSET: isize>() -> IMbnSmsConfiguration_Vtbl {
        unsafe extern "system" fn ServiceCenterAddress<Identity: ::windows::core::IUnknownImpl, Impl: IMbnSmsConfiguration_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, scaddress: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).ServiceCenterAddress() {
                ::core::result::Result::Ok(ok__) => {
                    *scaddress = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetServiceCenterAddress<Identity: ::windows::core::IUnknownImpl, Impl: IMbnSmsConfiguration_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, scaddress: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetServiceCenterAddress(::core::mem::transmute_copy(&scaddress)).into()
        }
        unsafe extern "system" fn MaxMessageIndex<Identity: ::windows::core::IUnknownImpl, Impl: IMbnSmsConfiguration_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).MaxMessageIndex() {
                ::core::result::Result::Ok(ok__) => {
                    *index = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CdmaShortMsgSize<Identity: ::windows::core::IUnknownImpl, Impl: IMbnSmsConfiguration_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, shortmsgsize: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CdmaShortMsgSize() {
                ::core::result::Result::Ok(ok__) => {
                    *shortmsgsize = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SmsFormat<Identity: ::windows::core::IUnknownImpl, Impl: IMbnSmsConfiguration_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, smsformat: *mut MBN_SMS_FORMAT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).SmsFormat() {
                ::core::result::Result::Ok(ok__) => {
                    *smsformat = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSmsFormat<Identity: ::windows::core::IUnknownImpl, Impl: IMbnSmsConfiguration_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, smsformat: MBN_SMS_FORMAT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetSmsFormat(::core::mem::transmute_copy(&smsformat)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            ServiceCenterAddress: ServiceCenterAddress::<Identity, Impl, OFFSET>,
            SetServiceCenterAddress: SetServiceCenterAddress::<Identity, Impl, OFFSET>,
            MaxMessageIndex: MaxMessageIndex::<Identity, Impl, OFFSET>,
            CdmaShortMsgSize: CdmaShortMsgSize::<Identity, Impl, OFFSET>,
            SmsFormat: SmsFormat::<Identity, Impl, OFFSET>,
            SetSmsFormat: SetSmsFormat::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMbnSmsConfiguration as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IMbnSmsEvents_Impl: Sized {
    fn OnSmsConfigurationChange(&mut self, sms: &::core::option::Option<IMbnSms>) -> ::windows::core::Result<()>;
    fn OnSetSmsConfigurationComplete(&mut self, sms: &::core::option::Option<IMbnSms>, requestid: u32, status: ::windows::core::HRESULT) -> ::windows::core::Result<()>;
    fn OnSmsSendComplete(&mut self, sms: &::core::option::Option<IMbnSms>, requestid: u32, status: ::windows::core::HRESULT) -> ::windows::core::Result<()>;
    fn OnSmsReadComplete(&mut self, sms: &::core::option::Option<IMbnSms>, smsformat: MBN_SMS_FORMAT, readmsgs: *const super::super::System::Com::SAFEARRAY, moremsgs: i16, requestid: u32, status: ::windows::core::HRESULT) -> ::windows::core::Result<()>;
    fn OnSmsNewClass0Message(&mut self, sms: &::core::option::Option<IMbnSms>, smsformat: MBN_SMS_FORMAT, readmsgs: *const super::super::System::Com::SAFEARRAY) -> ::windows::core::Result<()>;
    fn OnSmsDeleteComplete(&mut self, sms: &::core::option::Option<IMbnSms>, requestid: u32, status: ::windows::core::HRESULT) -> ::windows::core::Result<()>;
    fn OnSmsStatusChange(&mut self, sms: &::core::option::Option<IMbnSms>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl IMbnSmsEvents_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMbnSmsEvents_Impl, const OFFSET: isize>() -> IMbnSmsEvents_Vtbl {
        unsafe extern "system" fn OnSmsConfigurationChange<Identity: ::windows::core::IUnknownImpl, Impl: IMbnSmsEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, sms: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).OnSmsConfigurationChange(::core::mem::transmute(&sms)).into()
        }
        unsafe extern "system" fn OnSetSmsConfigurationComplete<Identity: ::windows::core::IUnknownImpl, Impl: IMbnSmsEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, sms: ::windows::core::RawPtr, requestid: u32, status: ::windows::core::HRESULT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).OnSetSmsConfigurationComplete(::core::mem::transmute(&sms), ::core::mem::transmute_copy(&requestid), ::core::mem::transmute_copy(&status)).into()
        }
        unsafe extern "system" fn OnSmsSendComplete<Identity: ::windows::core::IUnknownImpl, Impl: IMbnSmsEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, sms: ::windows::core::RawPtr, requestid: u32, status: ::windows::core::HRESULT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).OnSmsSendComplete(::core::mem::transmute(&sms), ::core::mem::transmute_copy(&requestid), ::core::mem::transmute_copy(&status)).into()
        }
        unsafe extern "system" fn OnSmsReadComplete<Identity: ::windows::core::IUnknownImpl, Impl: IMbnSmsEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, sms: ::windows::core::RawPtr, smsformat: MBN_SMS_FORMAT, readmsgs: *const super::super::System::Com::SAFEARRAY, moremsgs: i16, requestid: u32, status: ::windows::core::HRESULT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).OnSmsReadComplete(::core::mem::transmute(&sms), ::core::mem::transmute_copy(&smsformat), ::core::mem::transmute_copy(&readmsgs), ::core::mem::transmute_copy(&moremsgs), ::core::mem::transmute_copy(&requestid), ::core::mem::transmute_copy(&status)).into()
        }
        unsafe extern "system" fn OnSmsNewClass0Message<Identity: ::windows::core::IUnknownImpl, Impl: IMbnSmsEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, sms: ::windows::core::RawPtr, smsformat: MBN_SMS_FORMAT, readmsgs: *const super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).OnSmsNewClass0Message(::core::mem::transmute(&sms), ::core::mem::transmute_copy(&smsformat), ::core::mem::transmute_copy(&readmsgs)).into()
        }
        unsafe extern "system" fn OnSmsDeleteComplete<Identity: ::windows::core::IUnknownImpl, Impl: IMbnSmsEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, sms: ::windows::core::RawPtr, requestid: u32, status: ::windows::core::HRESULT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).OnSmsDeleteComplete(::core::mem::transmute(&sms), ::core::mem::transmute_copy(&requestid), ::core::mem::transmute_copy(&status)).into()
        }
        unsafe extern "system" fn OnSmsStatusChange<Identity: ::windows::core::IUnknownImpl, Impl: IMbnSmsEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, sms: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).OnSmsStatusChange(::core::mem::transmute(&sms)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            OnSmsConfigurationChange: OnSmsConfigurationChange::<Identity, Impl, OFFSET>,
            OnSetSmsConfigurationComplete: OnSetSmsConfigurationComplete::<Identity, Impl, OFFSET>,
            OnSmsSendComplete: OnSmsSendComplete::<Identity, Impl, OFFSET>,
            OnSmsReadComplete: OnSmsReadComplete::<Identity, Impl, OFFSET>,
            OnSmsNewClass0Message: OnSmsNewClass0Message::<Identity, Impl, OFFSET>,
            OnSmsDeleteComplete: OnSmsDeleteComplete::<Identity, Impl, OFFSET>,
            OnSmsStatusChange: OnSmsStatusChange::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMbnSmsEvents as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub trait IMbnSmsReadMsgPdu_Impl: Sized {
    fn Index(&mut self) -> ::windows::core::Result<u32>;
    fn Status(&mut self) -> ::windows::core::Result<MBN_MSG_STATUS>;
    fn PduData(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn Message(&mut self) -> ::windows::core::Result<*mut super::super::System::Com::SAFEARRAY>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl IMbnSmsReadMsgPdu_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMbnSmsReadMsgPdu_Impl, const OFFSET: isize>() -> IMbnSmsReadMsgPdu_Vtbl {
        unsafe extern "system" fn Index<Identity: ::windows::core::IUnknownImpl, Impl: IMbnSmsReadMsgPdu_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Index() {
                ::core::result::Result::Ok(ok__) => {
                    *index = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Status<Identity: ::windows::core::IUnknownImpl, Impl: IMbnSmsReadMsgPdu_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, status: *mut MBN_MSG_STATUS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Status() {
                ::core::result::Result::Ok(ok__) => {
                    *status = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PduData<Identity: ::windows::core::IUnknownImpl, Impl: IMbnSmsReadMsgPdu_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdudata: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).PduData() {
                ::core::result::Result::Ok(ok__) => {
                    *pdudata = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Message<Identity: ::windows::core::IUnknownImpl, Impl: IMbnSmsReadMsgPdu_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, message: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Message() {
                ::core::result::Result::Ok(ok__) => {
                    *message = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            Index: Index::<Identity, Impl, OFFSET>,
            Status: Status::<Identity, Impl, OFFSET>,
            PduData: PduData::<Identity, Impl, OFFSET>,
            Message: Message::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMbnSmsReadMsgPdu as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub trait IMbnSmsReadMsgTextCdma_Impl: Sized {
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
impl IMbnSmsReadMsgTextCdma_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMbnSmsReadMsgTextCdma_Impl, const OFFSET: isize>() -> IMbnSmsReadMsgTextCdma_Vtbl {
        unsafe extern "system" fn Index<Identity: ::windows::core::IUnknownImpl, Impl: IMbnSmsReadMsgTextCdma_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Index() {
                ::core::result::Result::Ok(ok__) => {
                    *index = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Status<Identity: ::windows::core::IUnknownImpl, Impl: IMbnSmsReadMsgTextCdma_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, status: *mut MBN_MSG_STATUS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Status() {
                ::core::result::Result::Ok(ok__) => {
                    *status = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Address<Identity: ::windows::core::IUnknownImpl, Impl: IMbnSmsReadMsgTextCdma_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, address: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Address() {
                ::core::result::Result::Ok(ok__) => {
                    *address = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Timestamp<Identity: ::windows::core::IUnknownImpl, Impl: IMbnSmsReadMsgTextCdma_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, timestamp: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Timestamp() {
                ::core::result::Result::Ok(ok__) => {
                    *timestamp = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EncodingID<Identity: ::windows::core::IUnknownImpl, Impl: IMbnSmsReadMsgTextCdma_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, encodingid: *mut MBN_SMS_CDMA_ENCODING) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).EncodingID() {
                ::core::result::Result::Ok(ok__) => {
                    *encodingid = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LanguageID<Identity: ::windows::core::IUnknownImpl, Impl: IMbnSmsReadMsgTextCdma_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, languageid: *mut MBN_SMS_CDMA_LANG) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).LanguageID() {
                ::core::result::Result::Ok(ok__) => {
                    *languageid = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SizeInCharacters<Identity: ::windows::core::IUnknownImpl, Impl: IMbnSmsReadMsgTextCdma_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, sizeincharacters: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).SizeInCharacters() {
                ::core::result::Result::Ok(ok__) => {
                    *sizeincharacters = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Message<Identity: ::windows::core::IUnknownImpl, Impl: IMbnSmsReadMsgTextCdma_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, message: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Message() {
                ::core::result::Result::Ok(ok__) => {
                    *message = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            Index: Index::<Identity, Impl, OFFSET>,
            Status: Status::<Identity, Impl, OFFSET>,
            Address: Address::<Identity, Impl, OFFSET>,
            Timestamp: Timestamp::<Identity, Impl, OFFSET>,
            EncodingID: EncodingID::<Identity, Impl, OFFSET>,
            LanguageID: LanguageID::<Identity, Impl, OFFSET>,
            SizeInCharacters: SizeInCharacters::<Identity, Impl, OFFSET>,
            Message: Message::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMbnSmsReadMsgTextCdma as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub trait IMbnSubscriberInformation_Impl: Sized {
    fn SubscriberID(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SimIccID(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn TelephoneNumbers(&mut self) -> ::windows::core::Result<*mut super::super::System::Com::SAFEARRAY>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl IMbnSubscriberInformation_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMbnSubscriberInformation_Impl, const OFFSET: isize>() -> IMbnSubscriberInformation_Vtbl {
        unsafe extern "system" fn SubscriberID<Identity: ::windows::core::IUnknownImpl, Impl: IMbnSubscriberInformation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, subscriberid: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).SubscriberID() {
                ::core::result::Result::Ok(ok__) => {
                    *subscriberid = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SimIccID<Identity: ::windows::core::IUnknownImpl, Impl: IMbnSubscriberInformation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, simiccid: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).SimIccID() {
                ::core::result::Result::Ok(ok__) => {
                    *simiccid = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TelephoneNumbers<Identity: ::windows::core::IUnknownImpl, Impl: IMbnSubscriberInformation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, telephonenumbers: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).TelephoneNumbers() {
                ::core::result::Result::Ok(ok__) => {
                    *telephonenumbers = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            SubscriberID: SubscriberID::<Identity, Impl, OFFSET>,
            SimIccID: SimIccID::<Identity, Impl, OFFSET>,
            TelephoneNumbers: TelephoneNumbers::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMbnSubscriberInformation as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IMbnVendorSpecificEvents_Impl: Sized {
    fn OnEventNotification(&mut self, vendoroperation: &::core::option::Option<IMbnVendorSpecificOperation>, vendorspecificdata: *const super::super::System::Com::SAFEARRAY) -> ::windows::core::Result<()>;
    fn OnSetVendorSpecificComplete(&mut self, vendoroperation: &::core::option::Option<IMbnVendorSpecificOperation>, vendorspecificdata: *const super::super::System::Com::SAFEARRAY, requestid: u32) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl IMbnVendorSpecificEvents_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMbnVendorSpecificEvents_Impl, const OFFSET: isize>() -> IMbnVendorSpecificEvents_Vtbl {
        unsafe extern "system" fn OnEventNotification<Identity: ::windows::core::IUnknownImpl, Impl: IMbnVendorSpecificEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, vendoroperation: ::windows::core::RawPtr, vendorspecificdata: *const super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).OnEventNotification(::core::mem::transmute(&vendoroperation), ::core::mem::transmute_copy(&vendorspecificdata)).into()
        }
        unsafe extern "system" fn OnSetVendorSpecificComplete<Identity: ::windows::core::IUnknownImpl, Impl: IMbnVendorSpecificEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, vendoroperation: ::windows::core::RawPtr, vendorspecificdata: *const super::super::System::Com::SAFEARRAY, requestid: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).OnSetVendorSpecificComplete(::core::mem::transmute(&vendoroperation), ::core::mem::transmute_copy(&vendorspecificdata), ::core::mem::transmute_copy(&requestid)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            OnEventNotification: OnEventNotification::<Identity, Impl, OFFSET>,
            OnSetVendorSpecificComplete: OnSetVendorSpecificComplete::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMbnVendorSpecificEvents as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IMbnVendorSpecificOperation_Impl: Sized {
    fn SetVendorSpecific(&mut self, vendorspecificdata: *const super::super::System::Com::SAFEARRAY) -> ::windows::core::Result<u32>;
}
#[cfg(feature = "Win32_System_Com")]
impl IMbnVendorSpecificOperation_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMbnVendorSpecificOperation_Impl, const OFFSET: isize>() -> IMbnVendorSpecificOperation_Vtbl {
        unsafe extern "system" fn SetVendorSpecific<Identity: ::windows::core::IUnknownImpl, Impl: IMbnVendorSpecificOperation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, vendorspecificdata: *const super::super::System::Com::SAFEARRAY, requestid: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).SetVendorSpecific(::core::mem::transmute_copy(&vendorspecificdata)) {
                ::core::result::Result::Ok(ok__) => {
                    *requestid = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(), SetVendorSpecific: SetVendorSpecific::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMbnVendorSpecificOperation as ::windows::core::Interface>::IID
    }
}
