#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IDummyMBNUCMExt_Impl: Sized + super::super::System::Com::IDispatch_Impl {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for IDummyMBNUCMExt {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IDummyMBNUCMExt_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDummyMBNUCMExt_Impl, const OFFSET: isize>() -> IDummyMBNUCMExt_Vtbl {
        Self { base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDummyMBNUCMExt as ::windows::core::ComInterface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`, `\"implement\"`*"]
pub trait IMbnConnection_Impl: Sized {
    fn ConnectionID(&self) -> ::windows::core::Result<::windows::core::BSTR>;
    fn InterfaceID(&self) -> ::windows::core::Result<::windows::core::BSTR>;
    fn Connect(&self, connectionmode: MBN_CONNECTION_MODE, strprofile: &::windows::core::PCWSTR) -> ::windows::core::Result<u32>;
    fn Disconnect(&self) -> ::windows::core::Result<u32>;
    fn GetConnectionState(&self, connectionstate: *mut MBN_ACTIVATION_STATE, profilename: *mut ::windows::core::BSTR) -> ::windows::core::Result<()>;
    fn GetVoiceCallState(&self) -> ::windows::core::Result<MBN_VOICE_CALL_STATE>;
    fn GetActivationNetworkError(&self) -> ::windows::core::Result<u32>;
}
impl ::windows::core::RuntimeName for IMbnConnection {}
impl IMbnConnection_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMbnConnection_Impl, const OFFSET: isize>() -> IMbnConnection_Vtbl {
        unsafe extern "system" fn ConnectionID<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMbnConnection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, connectionid: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.ConnectionID() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(connectionid, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn InterfaceID<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMbnConnection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, interfaceid: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.InterfaceID() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(interfaceid, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Connect<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMbnConnection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, connectionmode: MBN_CONNECTION_MODE, strprofile: ::windows::core::PCWSTR, requestid: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Connect(::core::mem::transmute_copy(&connectionmode), ::core::mem::transmute(&strprofile)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(requestid, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Disconnect<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMbnConnection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, requestid: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Disconnect() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(requestid, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetConnectionState<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMbnConnection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, connectionstate: *mut MBN_ACTIVATION_STATE, profilename: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetConnectionState(::core::mem::transmute_copy(&connectionstate), ::core::mem::transmute_copy(&profilename)).into()
        }
        unsafe extern "system" fn GetVoiceCallState<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMbnConnection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, voicecallstate: *mut MBN_VOICE_CALL_STATE) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetVoiceCallState() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(voicecallstate, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetActivationNetworkError<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMbnConnection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, networkerror: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetActivationNetworkError() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(networkerror, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
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
        iid == &<IMbnConnection as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`, `\"Win32_System_Com\"`, `\"implement\"`*"]
#[cfg(feature = "Win32_System_Com")]
pub trait IMbnConnectionContext_Impl: Sized {
    fn GetProvisionedContexts(&self) -> ::windows::core::Result<*mut super::super::System::Com::SAFEARRAY>;
    fn SetProvisionedContext(&self, provisionedcontexts: &MBN_CONTEXT, providerid: &::windows::core::PCWSTR) -> ::windows::core::Result<u32>;
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for IMbnConnectionContext {}
#[cfg(feature = "Win32_System_Com")]
impl IMbnConnectionContext_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMbnConnectionContext_Impl, const OFFSET: isize>() -> IMbnConnectionContext_Vtbl {
        unsafe extern "system" fn GetProvisionedContexts<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMbnConnectionContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, provisionedcontexts: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetProvisionedContexts() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(provisionedcontexts, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetProvisionedContext<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMbnConnectionContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, provisionedcontexts: MBN_CONTEXT, providerid: ::windows::core::PCWSTR, requestid: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.SetProvisionedContext(::core::mem::transmute(&provisionedcontexts), ::core::mem::transmute(&providerid)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(requestid, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetProvisionedContexts: GetProvisionedContexts::<Identity, Impl, OFFSET>,
            SetProvisionedContext: SetProvisionedContext::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMbnConnectionContext as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`, `\"implement\"`*"]
pub trait IMbnConnectionContextEvents_Impl: Sized {
    fn OnProvisionedContextListChange(&self, newinterface: ::core::option::Option<&IMbnConnectionContext>) -> ::windows::core::Result<()>;
    fn OnSetProvisionedContextComplete(&self, newinterface: ::core::option::Option<&IMbnConnectionContext>, requestid: u32, status: ::windows::core::HRESULT) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IMbnConnectionContextEvents {}
impl IMbnConnectionContextEvents_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMbnConnectionContextEvents_Impl, const OFFSET: isize>() -> IMbnConnectionContextEvents_Vtbl {
        unsafe extern "system" fn OnProvisionedContextListChange<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMbnConnectionContextEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newinterface: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnProvisionedContextListChange(::windows::core::from_raw_borrowed(&newinterface)).into()
        }
        unsafe extern "system" fn OnSetProvisionedContextComplete<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMbnConnectionContextEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newinterface: *mut ::core::ffi::c_void, requestid: u32, status: ::windows::core::HRESULT) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnSetProvisionedContextComplete(::windows::core::from_raw_borrowed(&newinterface), ::core::mem::transmute_copy(&requestid), ::core::mem::transmute_copy(&status)).into()
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            OnProvisionedContextListChange: OnProvisionedContextListChange::<Identity, Impl, OFFSET>,
            OnSetProvisionedContextComplete: OnSetProvisionedContextComplete::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMbnConnectionContextEvents as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`, `\"implement\"`*"]
pub trait IMbnConnectionEvents_Impl: Sized {
    fn OnConnectComplete(&self, newconnection: ::core::option::Option<&IMbnConnection>, requestid: u32, status: ::windows::core::HRESULT) -> ::windows::core::Result<()>;
    fn OnDisconnectComplete(&self, newconnection: ::core::option::Option<&IMbnConnection>, requestid: u32, status: ::windows::core::HRESULT) -> ::windows::core::Result<()>;
    fn OnConnectStateChange(&self, newconnection: ::core::option::Option<&IMbnConnection>) -> ::windows::core::Result<()>;
    fn OnVoiceCallStateChange(&self, newconnection: ::core::option::Option<&IMbnConnection>) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IMbnConnectionEvents {}
impl IMbnConnectionEvents_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMbnConnectionEvents_Impl, const OFFSET: isize>() -> IMbnConnectionEvents_Vtbl {
        unsafe extern "system" fn OnConnectComplete<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMbnConnectionEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newconnection: *mut ::core::ffi::c_void, requestid: u32, status: ::windows::core::HRESULT) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnConnectComplete(::windows::core::from_raw_borrowed(&newconnection), ::core::mem::transmute_copy(&requestid), ::core::mem::transmute_copy(&status)).into()
        }
        unsafe extern "system" fn OnDisconnectComplete<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMbnConnectionEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newconnection: *mut ::core::ffi::c_void, requestid: u32, status: ::windows::core::HRESULT) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnDisconnectComplete(::windows::core::from_raw_borrowed(&newconnection), ::core::mem::transmute_copy(&requestid), ::core::mem::transmute_copy(&status)).into()
        }
        unsafe extern "system" fn OnConnectStateChange<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMbnConnectionEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newconnection: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnConnectStateChange(::windows::core::from_raw_borrowed(&newconnection)).into()
        }
        unsafe extern "system" fn OnVoiceCallStateChange<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMbnConnectionEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newconnection: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnVoiceCallStateChange(::windows::core::from_raw_borrowed(&newconnection)).into()
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            OnConnectComplete: OnConnectComplete::<Identity, Impl, OFFSET>,
            OnDisconnectComplete: OnDisconnectComplete::<Identity, Impl, OFFSET>,
            OnConnectStateChange: OnConnectStateChange::<Identity, Impl, OFFSET>,
            OnVoiceCallStateChange: OnVoiceCallStateChange::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMbnConnectionEvents as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`, `\"Win32_System_Com\"`, `\"implement\"`*"]
#[cfg(feature = "Win32_System_Com")]
pub trait IMbnConnectionManager_Impl: Sized {
    fn GetConnection(&self, connectionid: &::windows::core::PCWSTR) -> ::windows::core::Result<IMbnConnection>;
    fn GetConnections(&self) -> ::windows::core::Result<*mut super::super::System::Com::SAFEARRAY>;
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for IMbnConnectionManager {}
#[cfg(feature = "Win32_System_Com")]
impl IMbnConnectionManager_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMbnConnectionManager_Impl, const OFFSET: isize>() -> IMbnConnectionManager_Vtbl {
        unsafe extern "system" fn GetConnection<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMbnConnectionManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, connectionid: ::windows::core::PCWSTR, mbnconnection: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetConnection(::core::mem::transmute(&connectionid)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(mbnconnection, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetConnections<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMbnConnectionManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, mbnconnections: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetConnections() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(mbnconnections, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetConnection: GetConnection::<Identity, Impl, OFFSET>,
            GetConnections: GetConnections::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMbnConnectionManager as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`, `\"implement\"`*"]
pub trait IMbnConnectionManagerEvents_Impl: Sized {
    fn OnConnectionArrival(&self, newconnection: ::core::option::Option<&IMbnConnection>) -> ::windows::core::Result<()>;
    fn OnConnectionRemoval(&self, oldconnection: ::core::option::Option<&IMbnConnection>) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IMbnConnectionManagerEvents {}
impl IMbnConnectionManagerEvents_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMbnConnectionManagerEvents_Impl, const OFFSET: isize>() -> IMbnConnectionManagerEvents_Vtbl {
        unsafe extern "system" fn OnConnectionArrival<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMbnConnectionManagerEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newconnection: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnConnectionArrival(::windows::core::from_raw_borrowed(&newconnection)).into()
        }
        unsafe extern "system" fn OnConnectionRemoval<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMbnConnectionManagerEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, oldconnection: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnConnectionRemoval(::windows::core::from_raw_borrowed(&oldconnection)).into()
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            OnConnectionArrival: OnConnectionArrival::<Identity, Impl, OFFSET>,
            OnConnectionRemoval: OnConnectionRemoval::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMbnConnectionManagerEvents as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`, `\"implement\"`*"]
pub trait IMbnConnectionProfile_Impl: Sized {
    fn GetProfileXmlData(&self) -> ::windows::core::Result<::windows::core::BSTR>;
    fn UpdateProfile(&self, strprofile: &::windows::core::PCWSTR) -> ::windows::core::Result<()>;
    fn Delete(&self) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IMbnConnectionProfile {}
impl IMbnConnectionProfile_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMbnConnectionProfile_Impl, const OFFSET: isize>() -> IMbnConnectionProfile_Vtbl {
        unsafe extern "system" fn GetProfileXmlData<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMbnConnectionProfile_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, profiledata: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetProfileXmlData() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(profiledata, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UpdateProfile<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMbnConnectionProfile_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strprofile: ::windows::core::PCWSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.UpdateProfile(::core::mem::transmute(&strprofile)).into()
        }
        unsafe extern "system" fn Delete<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMbnConnectionProfile_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Delete().into()
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetProfileXmlData: GetProfileXmlData::<Identity, Impl, OFFSET>,
            UpdateProfile: UpdateProfile::<Identity, Impl, OFFSET>,
            Delete: Delete::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMbnConnectionProfile as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`, `\"implement\"`*"]
pub trait IMbnConnectionProfileEvents_Impl: Sized {
    fn OnProfileUpdate(&self, newprofile: ::core::option::Option<&IMbnConnectionProfile>) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IMbnConnectionProfileEvents {}
impl IMbnConnectionProfileEvents_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMbnConnectionProfileEvents_Impl, const OFFSET: isize>() -> IMbnConnectionProfileEvents_Vtbl {
        unsafe extern "system" fn OnProfileUpdate<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMbnConnectionProfileEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newprofile: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnProfileUpdate(::windows::core::from_raw_borrowed(&newprofile)).into()
        }
        Self { base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(), OnProfileUpdate: OnProfileUpdate::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMbnConnectionProfileEvents as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`, `\"Win32_System_Com\"`, `\"implement\"`*"]
#[cfg(feature = "Win32_System_Com")]
pub trait IMbnConnectionProfileManager_Impl: Sized {
    fn GetConnectionProfiles(&self, mbninterface: ::core::option::Option<&IMbnInterface>) -> ::windows::core::Result<*mut super::super::System::Com::SAFEARRAY>;
    fn GetConnectionProfile(&self, mbninterface: ::core::option::Option<&IMbnInterface>, profilename: &::windows::core::PCWSTR) -> ::windows::core::Result<IMbnConnectionProfile>;
    fn CreateConnectionProfile(&self, xmlprofile: &::windows::core::PCWSTR) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for IMbnConnectionProfileManager {}
#[cfg(feature = "Win32_System_Com")]
impl IMbnConnectionProfileManager_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMbnConnectionProfileManager_Impl, const OFFSET: isize>() -> IMbnConnectionProfileManager_Vtbl {
        unsafe extern "system" fn GetConnectionProfiles<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMbnConnectionProfileManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, mbninterface: *mut ::core::ffi::c_void, connectionprofiles: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetConnectionProfiles(::windows::core::from_raw_borrowed(&mbninterface)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(connectionprofiles, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetConnectionProfile<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMbnConnectionProfileManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, mbninterface: *mut ::core::ffi::c_void, profilename: ::windows::core::PCWSTR, connectionprofile: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetConnectionProfile(::windows::core::from_raw_borrowed(&mbninterface), ::core::mem::transmute(&profilename)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(connectionprofile, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateConnectionProfile<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMbnConnectionProfileManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, xmlprofile: ::windows::core::PCWSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.CreateConnectionProfile(::core::mem::transmute(&xmlprofile)).into()
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetConnectionProfiles: GetConnectionProfiles::<Identity, Impl, OFFSET>,
            GetConnectionProfile: GetConnectionProfile::<Identity, Impl, OFFSET>,
            CreateConnectionProfile: CreateConnectionProfile::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMbnConnectionProfileManager as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`, `\"implement\"`*"]
pub trait IMbnConnectionProfileManagerEvents_Impl: Sized {
    fn OnConnectionProfileArrival(&self, newconnectionprofile: ::core::option::Option<&IMbnConnectionProfile>) -> ::windows::core::Result<()>;
    fn OnConnectionProfileRemoval(&self, oldconnectionprofile: ::core::option::Option<&IMbnConnectionProfile>) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IMbnConnectionProfileManagerEvents {}
impl IMbnConnectionProfileManagerEvents_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMbnConnectionProfileManagerEvents_Impl, const OFFSET: isize>() -> IMbnConnectionProfileManagerEvents_Vtbl {
        unsafe extern "system" fn OnConnectionProfileArrival<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMbnConnectionProfileManagerEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newconnectionprofile: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnConnectionProfileArrival(::windows::core::from_raw_borrowed(&newconnectionprofile)).into()
        }
        unsafe extern "system" fn OnConnectionProfileRemoval<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMbnConnectionProfileManagerEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, oldconnectionprofile: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnConnectionProfileRemoval(::windows::core::from_raw_borrowed(&oldconnectionprofile)).into()
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            OnConnectionProfileArrival: OnConnectionProfileArrival::<Identity, Impl, OFFSET>,
            OnConnectionProfileRemoval: OnConnectionProfileRemoval::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMbnConnectionProfileManagerEvents as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub trait IMbnDeviceService_Impl: Sized {
    fn QuerySupportedCommands(&self) -> ::windows::core::Result<u32>;
    fn OpenCommandSession(&self) -> ::windows::core::Result<u32>;
    fn CloseCommandSession(&self) -> ::windows::core::Result<u32>;
    fn SetCommand(&self, commandid: u32, deviceservicedata: *const super::super::System::Com::SAFEARRAY) -> ::windows::core::Result<u32>;
    fn QueryCommand(&self, commandid: u32, deviceservicedata: *const super::super::System::Com::SAFEARRAY) -> ::windows::core::Result<u32>;
    fn OpenDataSession(&self) -> ::windows::core::Result<u32>;
    fn CloseDataSession(&self) -> ::windows::core::Result<u32>;
    fn WriteData(&self, deviceservicedata: *const super::super::System::Com::SAFEARRAY) -> ::windows::core::Result<u32>;
    fn InterfaceID(&self) -> ::windows::core::Result<::windows::core::BSTR>;
    fn DeviceServiceID(&self) -> ::windows::core::Result<::windows::core::BSTR>;
    fn IsCommandSessionOpen(&self) -> ::windows::core::Result<super::super::Foundation::BOOL>;
    fn IsDataSessionOpen(&self) -> ::windows::core::Result<super::super::Foundation::BOOL>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::windows::core::RuntimeName for IMbnDeviceService {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl IMbnDeviceService_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMbnDeviceService_Impl, const OFFSET: isize>() -> IMbnDeviceService_Vtbl {
        unsafe extern "system" fn QuerySupportedCommands<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMbnDeviceService_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, requestid: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.QuerySupportedCommands() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(requestid, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OpenCommandSession<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMbnDeviceService_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, requestid: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.OpenCommandSession() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(requestid, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CloseCommandSession<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMbnDeviceService_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, requestid: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CloseCommandSession() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(requestid, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCommand<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMbnDeviceService_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, commandid: u32, deviceservicedata: *const super::super::System::Com::SAFEARRAY, requestid: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.SetCommand(::core::mem::transmute_copy(&commandid), ::core::mem::transmute_copy(&deviceservicedata)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(requestid, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn QueryCommand<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMbnDeviceService_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, commandid: u32, deviceservicedata: *const super::super::System::Com::SAFEARRAY, requestid: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.QueryCommand(::core::mem::transmute_copy(&commandid), ::core::mem::transmute_copy(&deviceservicedata)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(requestid, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OpenDataSession<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMbnDeviceService_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, requestid: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.OpenDataSession() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(requestid, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CloseDataSession<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMbnDeviceService_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, requestid: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CloseDataSession() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(requestid, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn WriteData<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMbnDeviceService_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, deviceservicedata: *const super::super::System::Com::SAFEARRAY, requestid: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.WriteData(::core::mem::transmute_copy(&deviceservicedata)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(requestid, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn InterfaceID<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMbnDeviceService_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, interfaceid: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.InterfaceID() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(interfaceid, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DeviceServiceID<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMbnDeviceService_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, deviceserviceid: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.DeviceServiceID() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(deviceserviceid, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsCommandSessionOpen<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMbnDeviceService_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.IsCommandSessionOpen() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(value, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsDataSessionOpen<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMbnDeviceService_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.IsDataSessionOpen() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(value, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
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
        iid == &<IMbnDeviceService as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`, `\"implement\"`*"]
pub trait IMbnDeviceServiceStateEvents_Impl: Sized {
    fn OnSessionsStateChange(&self, interfaceid: &::windows::core::BSTR, statechange: MBN_DEVICE_SERVICE_SESSIONS_STATE) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IMbnDeviceServiceStateEvents {}
impl IMbnDeviceServiceStateEvents_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMbnDeviceServiceStateEvents_Impl, const OFFSET: isize>() -> IMbnDeviceServiceStateEvents_Vtbl {
        unsafe extern "system" fn OnSessionsStateChange<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMbnDeviceServiceStateEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, interfaceid: ::std::mem::MaybeUninit<::windows::core::BSTR>, statechange: MBN_DEVICE_SERVICE_SESSIONS_STATE) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnSessionsStateChange(::core::mem::transmute(&interfaceid), ::core::mem::transmute_copy(&statechange)).into()
        }
        Self { base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(), OnSessionsStateChange: OnSessionsStateChange::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMbnDeviceServiceStateEvents as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`, `\"Win32_System_Com\"`, `\"implement\"`*"]
#[cfg(feature = "Win32_System_Com")]
pub trait IMbnDeviceServicesContext_Impl: Sized {
    fn EnumerateDeviceServices(&self) -> ::windows::core::Result<*mut super::super::System::Com::SAFEARRAY>;
    fn GetDeviceService(&self, deviceserviceid: &::windows::core::BSTR) -> ::windows::core::Result<IMbnDeviceService>;
    fn MaxCommandSize(&self) -> ::windows::core::Result<u32>;
    fn MaxDataSize(&self) -> ::windows::core::Result<u32>;
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for IMbnDeviceServicesContext {}
#[cfg(feature = "Win32_System_Com")]
impl IMbnDeviceServicesContext_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMbnDeviceServicesContext_Impl, const OFFSET: isize>() -> IMbnDeviceServicesContext_Vtbl {
        unsafe extern "system" fn EnumerateDeviceServices<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMbnDeviceServicesContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, deviceservices: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.EnumerateDeviceServices() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(deviceservices, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDeviceService<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMbnDeviceServicesContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, deviceserviceid: ::std::mem::MaybeUninit<::windows::core::BSTR>, mbndeviceservice: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetDeviceService(::core::mem::transmute(&deviceserviceid)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(mbndeviceservice, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MaxCommandSize<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMbnDeviceServicesContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, maxcommandsize: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.MaxCommandSize() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(maxcommandsize, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MaxDataSize<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMbnDeviceServicesContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, maxdatasize: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.MaxDataSize() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(maxdatasize, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            EnumerateDeviceServices: EnumerateDeviceServices::<Identity, Impl, OFFSET>,
            GetDeviceService: GetDeviceService::<Identity, Impl, OFFSET>,
            MaxCommandSize: MaxCommandSize::<Identity, Impl, OFFSET>,
            MaxDataSize: MaxDataSize::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMbnDeviceServicesContext as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`, `\"Win32_System_Com\"`, `\"implement\"`*"]
#[cfg(feature = "Win32_System_Com")]
pub trait IMbnDeviceServicesEvents_Impl: Sized {
    fn OnQuerySupportedCommandsComplete(&self, deviceservice: ::core::option::Option<&IMbnDeviceService>, commandidlist: *const super::super::System::Com::SAFEARRAY, status: ::windows::core::HRESULT, requestid: u32) -> ::windows::core::Result<()>;
    fn OnOpenCommandSessionComplete(&self, deviceservice: ::core::option::Option<&IMbnDeviceService>, status: ::windows::core::HRESULT, requestid: u32) -> ::windows::core::Result<()>;
    fn OnCloseCommandSessionComplete(&self, deviceservice: ::core::option::Option<&IMbnDeviceService>, status: ::windows::core::HRESULT, requestid: u32) -> ::windows::core::Result<()>;
    fn OnSetCommandComplete(&self, deviceservice: ::core::option::Option<&IMbnDeviceService>, responseid: u32, deviceservicedata: *const super::super::System::Com::SAFEARRAY, status: ::windows::core::HRESULT, requestid: u32) -> ::windows::core::Result<()>;
    fn OnQueryCommandComplete(&self, deviceservice: ::core::option::Option<&IMbnDeviceService>, responseid: u32, deviceservicedata: *const super::super::System::Com::SAFEARRAY, status: ::windows::core::HRESULT, requestid: u32) -> ::windows::core::Result<()>;
    fn OnEventNotification(&self, deviceservice: ::core::option::Option<&IMbnDeviceService>, eventid: u32, deviceservicedata: *const super::super::System::Com::SAFEARRAY) -> ::windows::core::Result<()>;
    fn OnOpenDataSessionComplete(&self, deviceservice: ::core::option::Option<&IMbnDeviceService>, status: ::windows::core::HRESULT, requestid: u32) -> ::windows::core::Result<()>;
    fn OnCloseDataSessionComplete(&self, deviceservice: ::core::option::Option<&IMbnDeviceService>, status: ::windows::core::HRESULT, requestid: u32) -> ::windows::core::Result<()>;
    fn OnWriteDataComplete(&self, deviceservice: ::core::option::Option<&IMbnDeviceService>, status: ::windows::core::HRESULT, requestid: u32) -> ::windows::core::Result<()>;
    fn OnReadData(&self, deviceservice: ::core::option::Option<&IMbnDeviceService>, deviceservicedata: *const super::super::System::Com::SAFEARRAY) -> ::windows::core::Result<()>;
    fn OnInterfaceStateChange(&self, interfaceid: &::windows::core::BSTR, statechange: MBN_DEVICE_SERVICES_INTERFACE_STATE) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for IMbnDeviceServicesEvents {}
#[cfg(feature = "Win32_System_Com")]
impl IMbnDeviceServicesEvents_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMbnDeviceServicesEvents_Impl, const OFFSET: isize>() -> IMbnDeviceServicesEvents_Vtbl {
        unsafe extern "system" fn OnQuerySupportedCommandsComplete<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMbnDeviceServicesEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, deviceservice: *mut ::core::ffi::c_void, commandidlist: *const super::super::System::Com::SAFEARRAY, status: ::windows::core::HRESULT, requestid: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnQuerySupportedCommandsComplete(::windows::core::from_raw_borrowed(&deviceservice), ::core::mem::transmute_copy(&commandidlist), ::core::mem::transmute_copy(&status), ::core::mem::transmute_copy(&requestid)).into()
        }
        unsafe extern "system" fn OnOpenCommandSessionComplete<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMbnDeviceServicesEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, deviceservice: *mut ::core::ffi::c_void, status: ::windows::core::HRESULT, requestid: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnOpenCommandSessionComplete(::windows::core::from_raw_borrowed(&deviceservice), ::core::mem::transmute_copy(&status), ::core::mem::transmute_copy(&requestid)).into()
        }
        unsafe extern "system" fn OnCloseCommandSessionComplete<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMbnDeviceServicesEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, deviceservice: *mut ::core::ffi::c_void, status: ::windows::core::HRESULT, requestid: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnCloseCommandSessionComplete(::windows::core::from_raw_borrowed(&deviceservice), ::core::mem::transmute_copy(&status), ::core::mem::transmute_copy(&requestid)).into()
        }
        unsafe extern "system" fn OnSetCommandComplete<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMbnDeviceServicesEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, deviceservice: *mut ::core::ffi::c_void, responseid: u32, deviceservicedata: *const super::super::System::Com::SAFEARRAY, status: ::windows::core::HRESULT, requestid: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnSetCommandComplete(::windows::core::from_raw_borrowed(&deviceservice), ::core::mem::transmute_copy(&responseid), ::core::mem::transmute_copy(&deviceservicedata), ::core::mem::transmute_copy(&status), ::core::mem::transmute_copy(&requestid)).into()
        }
        unsafe extern "system" fn OnQueryCommandComplete<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMbnDeviceServicesEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, deviceservice: *mut ::core::ffi::c_void, responseid: u32, deviceservicedata: *const super::super::System::Com::SAFEARRAY, status: ::windows::core::HRESULT, requestid: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnQueryCommandComplete(::windows::core::from_raw_borrowed(&deviceservice), ::core::mem::transmute_copy(&responseid), ::core::mem::transmute_copy(&deviceservicedata), ::core::mem::transmute_copy(&status), ::core::mem::transmute_copy(&requestid)).into()
        }
        unsafe extern "system" fn OnEventNotification<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMbnDeviceServicesEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, deviceservice: *mut ::core::ffi::c_void, eventid: u32, deviceservicedata: *const super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnEventNotification(::windows::core::from_raw_borrowed(&deviceservice), ::core::mem::transmute_copy(&eventid), ::core::mem::transmute_copy(&deviceservicedata)).into()
        }
        unsafe extern "system" fn OnOpenDataSessionComplete<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMbnDeviceServicesEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, deviceservice: *mut ::core::ffi::c_void, status: ::windows::core::HRESULT, requestid: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnOpenDataSessionComplete(::windows::core::from_raw_borrowed(&deviceservice), ::core::mem::transmute_copy(&status), ::core::mem::transmute_copy(&requestid)).into()
        }
        unsafe extern "system" fn OnCloseDataSessionComplete<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMbnDeviceServicesEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, deviceservice: *mut ::core::ffi::c_void, status: ::windows::core::HRESULT, requestid: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnCloseDataSessionComplete(::windows::core::from_raw_borrowed(&deviceservice), ::core::mem::transmute_copy(&status), ::core::mem::transmute_copy(&requestid)).into()
        }
        unsafe extern "system" fn OnWriteDataComplete<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMbnDeviceServicesEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, deviceservice: *mut ::core::ffi::c_void, status: ::windows::core::HRESULT, requestid: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnWriteDataComplete(::windows::core::from_raw_borrowed(&deviceservice), ::core::mem::transmute_copy(&status), ::core::mem::transmute_copy(&requestid)).into()
        }
        unsafe extern "system" fn OnReadData<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMbnDeviceServicesEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, deviceservice: *mut ::core::ffi::c_void, deviceservicedata: *const super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnReadData(::windows::core::from_raw_borrowed(&deviceservice), ::core::mem::transmute_copy(&deviceservicedata)).into()
        }
        unsafe extern "system" fn OnInterfaceStateChange<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMbnDeviceServicesEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, interfaceid: ::std::mem::MaybeUninit<::windows::core::BSTR>, statechange: MBN_DEVICE_SERVICES_INTERFACE_STATE) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnInterfaceStateChange(::core::mem::transmute(&interfaceid), ::core::mem::transmute_copy(&statechange)).into()
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
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
        iid == &<IMbnDeviceServicesEvents as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`, `\"implement\"`*"]
pub trait IMbnDeviceServicesManager_Impl: Sized {
    fn GetDeviceServicesContext(&self, networkinterfaceid: &::windows::core::BSTR) -> ::windows::core::Result<IMbnDeviceServicesContext>;
}
impl ::windows::core::RuntimeName for IMbnDeviceServicesManager {}
impl IMbnDeviceServicesManager_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMbnDeviceServicesManager_Impl, const OFFSET: isize>() -> IMbnDeviceServicesManager_Vtbl {
        unsafe extern "system" fn GetDeviceServicesContext<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMbnDeviceServicesManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, networkinterfaceid: ::std::mem::MaybeUninit<::windows::core::BSTR>, mbndevicescontext: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetDeviceServicesContext(::core::mem::transmute(&networkinterfaceid)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(mbndevicescontext, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(), GetDeviceServicesContext: GetDeviceServicesContext::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMbnDeviceServicesManager as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub trait IMbnInterface_Impl: Sized {
    fn InterfaceID(&self) -> ::windows::core::Result<::windows::core::BSTR>;
    fn GetInterfaceCapability(&self, interfacecaps: *mut MBN_INTERFACE_CAPS) -> ::windows::core::Result<()>;
    fn GetSubscriberInformation(&self) -> ::windows::core::Result<IMbnSubscriberInformation>;
    fn GetReadyState(&self) -> ::windows::core::Result<MBN_READY_STATE>;
    fn InEmergencyMode(&self) -> ::windows::core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn GetHomeProvider(&self) -> ::windows::core::Result<MBN_PROVIDER>;
    fn GetPreferredProviders(&self) -> ::windows::core::Result<*mut super::super::System::Com::SAFEARRAY>;
    fn SetPreferredProviders(&self, preferredproviders: *const super::super::System::Com::SAFEARRAY) -> ::windows::core::Result<u32>;
    fn GetVisibleProviders(&self, age: *mut u32, visibleproviders: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::core::Result<()>;
    fn ScanNetwork(&self) -> ::windows::core::Result<u32>;
    fn GetConnection(&self) -> ::windows::core::Result<IMbnConnection>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::windows::core::RuntimeName for IMbnInterface {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl IMbnInterface_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMbnInterface_Impl, const OFFSET: isize>() -> IMbnInterface_Vtbl {
        unsafe extern "system" fn InterfaceID<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMbnInterface_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, interfaceid: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.InterfaceID() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(interfaceid, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetInterfaceCapability<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMbnInterface_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, interfacecaps: *mut MBN_INTERFACE_CAPS) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetInterfaceCapability(::core::mem::transmute_copy(&interfacecaps)).into()
        }
        unsafe extern "system" fn GetSubscriberInformation<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMbnInterface_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, subscriberinformation: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetSubscriberInformation() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(subscriberinformation, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetReadyState<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMbnInterface_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, readystate: *mut MBN_READY_STATE) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetReadyState() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(readystate, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn InEmergencyMode<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMbnInterface_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, emergencymode: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.InEmergencyMode() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(emergencymode, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetHomeProvider<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMbnInterface_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, homeprovider: *mut MBN_PROVIDER) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetHomeProvider() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(homeprovider, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPreferredProviders<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMbnInterface_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, preferredproviders: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetPreferredProviders() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(preferredproviders, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPreferredProviders<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMbnInterface_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, preferredproviders: *const super::super::System::Com::SAFEARRAY, requestid: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.SetPreferredProviders(::core::mem::transmute_copy(&preferredproviders)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(requestid, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetVisibleProviders<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMbnInterface_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, age: *mut u32, visibleproviders: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetVisibleProviders(::core::mem::transmute_copy(&age), ::core::mem::transmute_copy(&visibleproviders)).into()
        }
        unsafe extern "system" fn ScanNetwork<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMbnInterface_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, requestid: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.ScanNetwork() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(requestid, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetConnection<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMbnInterface_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, mbnconnection: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetConnection() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(mbnconnection, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
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
        iid == &<IMbnInterface as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`, `\"implement\"`*"]
pub trait IMbnInterfaceEvents_Impl: Sized {
    fn OnInterfaceCapabilityAvailable(&self, newinterface: ::core::option::Option<&IMbnInterface>) -> ::windows::core::Result<()>;
    fn OnSubscriberInformationChange(&self, newinterface: ::core::option::Option<&IMbnInterface>) -> ::windows::core::Result<()>;
    fn OnReadyStateChange(&self, newinterface: ::core::option::Option<&IMbnInterface>) -> ::windows::core::Result<()>;
    fn OnEmergencyModeChange(&self, newinterface: ::core::option::Option<&IMbnInterface>) -> ::windows::core::Result<()>;
    fn OnHomeProviderAvailable(&self, newinterface: ::core::option::Option<&IMbnInterface>) -> ::windows::core::Result<()>;
    fn OnPreferredProvidersChange(&self, newinterface: ::core::option::Option<&IMbnInterface>) -> ::windows::core::Result<()>;
    fn OnSetPreferredProvidersComplete(&self, newinterface: ::core::option::Option<&IMbnInterface>, requestid: u32, status: ::windows::core::HRESULT) -> ::windows::core::Result<()>;
    fn OnScanNetworkComplete(&self, newinterface: ::core::option::Option<&IMbnInterface>, requestid: u32, status: ::windows::core::HRESULT) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IMbnInterfaceEvents {}
impl IMbnInterfaceEvents_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMbnInterfaceEvents_Impl, const OFFSET: isize>() -> IMbnInterfaceEvents_Vtbl {
        unsafe extern "system" fn OnInterfaceCapabilityAvailable<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMbnInterfaceEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newinterface: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnInterfaceCapabilityAvailable(::windows::core::from_raw_borrowed(&newinterface)).into()
        }
        unsafe extern "system" fn OnSubscriberInformationChange<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMbnInterfaceEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newinterface: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnSubscriberInformationChange(::windows::core::from_raw_borrowed(&newinterface)).into()
        }
        unsafe extern "system" fn OnReadyStateChange<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMbnInterfaceEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newinterface: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnReadyStateChange(::windows::core::from_raw_borrowed(&newinterface)).into()
        }
        unsafe extern "system" fn OnEmergencyModeChange<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMbnInterfaceEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newinterface: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnEmergencyModeChange(::windows::core::from_raw_borrowed(&newinterface)).into()
        }
        unsafe extern "system" fn OnHomeProviderAvailable<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMbnInterfaceEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newinterface: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnHomeProviderAvailable(::windows::core::from_raw_borrowed(&newinterface)).into()
        }
        unsafe extern "system" fn OnPreferredProvidersChange<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMbnInterfaceEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newinterface: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnPreferredProvidersChange(::windows::core::from_raw_borrowed(&newinterface)).into()
        }
        unsafe extern "system" fn OnSetPreferredProvidersComplete<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMbnInterfaceEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newinterface: *mut ::core::ffi::c_void, requestid: u32, status: ::windows::core::HRESULT) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnSetPreferredProvidersComplete(::windows::core::from_raw_borrowed(&newinterface), ::core::mem::transmute_copy(&requestid), ::core::mem::transmute_copy(&status)).into()
        }
        unsafe extern "system" fn OnScanNetworkComplete<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMbnInterfaceEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newinterface: *mut ::core::ffi::c_void, requestid: u32, status: ::windows::core::HRESULT) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnScanNetworkComplete(::windows::core::from_raw_borrowed(&newinterface), ::core::mem::transmute_copy(&requestid), ::core::mem::transmute_copy(&status)).into()
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
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
        iid == &<IMbnInterfaceEvents as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`, `\"Win32_System_Com\"`, `\"implement\"`*"]
#[cfg(feature = "Win32_System_Com")]
pub trait IMbnInterfaceManager_Impl: Sized {
    fn GetInterface(&self, interfaceid: &::windows::core::PCWSTR) -> ::windows::core::Result<IMbnInterface>;
    fn GetInterfaces(&self) -> ::windows::core::Result<*mut super::super::System::Com::SAFEARRAY>;
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for IMbnInterfaceManager {}
#[cfg(feature = "Win32_System_Com")]
impl IMbnInterfaceManager_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMbnInterfaceManager_Impl, const OFFSET: isize>() -> IMbnInterfaceManager_Vtbl {
        unsafe extern "system" fn GetInterface<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMbnInterfaceManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, interfaceid: ::windows::core::PCWSTR, mbninterface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetInterface(::core::mem::transmute(&interfaceid)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(mbninterface, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetInterfaces<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMbnInterfaceManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, mbninterfaces: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetInterfaces() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(mbninterfaces, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetInterface: GetInterface::<Identity, Impl, OFFSET>,
            GetInterfaces: GetInterfaces::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMbnInterfaceManager as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`, `\"implement\"`*"]
pub trait IMbnInterfaceManagerEvents_Impl: Sized {
    fn OnInterfaceArrival(&self, newinterface: ::core::option::Option<&IMbnInterface>) -> ::windows::core::Result<()>;
    fn OnInterfaceRemoval(&self, oldinterface: ::core::option::Option<&IMbnInterface>) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IMbnInterfaceManagerEvents {}
impl IMbnInterfaceManagerEvents_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMbnInterfaceManagerEvents_Impl, const OFFSET: isize>() -> IMbnInterfaceManagerEvents_Vtbl {
        unsafe extern "system" fn OnInterfaceArrival<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMbnInterfaceManagerEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newinterface: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnInterfaceArrival(::windows::core::from_raw_borrowed(&newinterface)).into()
        }
        unsafe extern "system" fn OnInterfaceRemoval<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMbnInterfaceManagerEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, oldinterface: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnInterfaceRemoval(::windows::core::from_raw_borrowed(&oldinterface)).into()
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            OnInterfaceArrival: OnInterfaceArrival::<Identity, Impl, OFFSET>,
            OnInterfaceRemoval: OnInterfaceRemoval::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMbnInterfaceManagerEvents as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`, `\"Win32_System_Com\"`, `\"implement\"`*"]
#[cfg(feature = "Win32_System_Com")]
pub trait IMbnMultiCarrier_Impl: Sized {
    fn SetHomeProvider(&self, homeprovider: *const MBN_PROVIDER2) -> ::windows::core::Result<u32>;
    fn GetPreferredProviders(&self) -> ::windows::core::Result<*mut super::super::System::Com::SAFEARRAY>;
    fn GetVisibleProviders(&self, age: *mut u32, visibleproviders: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::core::Result<()>;
    fn GetSupportedCellularClasses(&self) -> ::windows::core::Result<*mut super::super::System::Com::SAFEARRAY>;
    fn GetCurrentCellularClass(&self) -> ::windows::core::Result<MBN_CELLULAR_CLASS>;
    fn ScanNetwork(&self) -> ::windows::core::Result<u32>;
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for IMbnMultiCarrier {}
#[cfg(feature = "Win32_System_Com")]
impl IMbnMultiCarrier_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMbnMultiCarrier_Impl, const OFFSET: isize>() -> IMbnMultiCarrier_Vtbl {
        unsafe extern "system" fn SetHomeProvider<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMbnMultiCarrier_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, homeprovider: *const MBN_PROVIDER2, requestid: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.SetHomeProvider(::core::mem::transmute_copy(&homeprovider)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(requestid, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPreferredProviders<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMbnMultiCarrier_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, preferredmulticarrierproviders: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetPreferredProviders() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(preferredmulticarrierproviders, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetVisibleProviders<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMbnMultiCarrier_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, age: *mut u32, visibleproviders: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetVisibleProviders(::core::mem::transmute_copy(&age), ::core::mem::transmute_copy(&visibleproviders)).into()
        }
        unsafe extern "system" fn GetSupportedCellularClasses<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMbnMultiCarrier_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cellularclasses: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetSupportedCellularClasses() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(cellularclasses, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCurrentCellularClass<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMbnMultiCarrier_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, currentcellularclass: *mut MBN_CELLULAR_CLASS) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetCurrentCellularClass() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(currentcellularclass, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ScanNetwork<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMbnMultiCarrier_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, requestid: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.ScanNetwork() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(requestid, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            SetHomeProvider: SetHomeProvider::<Identity, Impl, OFFSET>,
            GetPreferredProviders: GetPreferredProviders::<Identity, Impl, OFFSET>,
            GetVisibleProviders: GetVisibleProviders::<Identity, Impl, OFFSET>,
            GetSupportedCellularClasses: GetSupportedCellularClasses::<Identity, Impl, OFFSET>,
            GetCurrentCellularClass: GetCurrentCellularClass::<Identity, Impl, OFFSET>,
            ScanNetwork: ScanNetwork::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMbnMultiCarrier as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`, `\"implement\"`*"]
pub trait IMbnMultiCarrierEvents_Impl: Sized {
    fn OnSetHomeProviderComplete(&self, mbninterface: ::core::option::Option<&IMbnMultiCarrier>, requestid: u32, status: ::windows::core::HRESULT) -> ::windows::core::Result<()>;
    fn OnCurrentCellularClassChange(&self, mbninterface: ::core::option::Option<&IMbnMultiCarrier>) -> ::windows::core::Result<()>;
    fn OnPreferredProvidersChange(&self, mbninterface: ::core::option::Option<&IMbnMultiCarrier>) -> ::windows::core::Result<()>;
    fn OnScanNetworkComplete(&self, mbninterface: ::core::option::Option<&IMbnMultiCarrier>, requestid: u32, status: ::windows::core::HRESULT) -> ::windows::core::Result<()>;
    fn OnInterfaceCapabilityChange(&self, mbninterface: ::core::option::Option<&IMbnMultiCarrier>) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IMbnMultiCarrierEvents {}
impl IMbnMultiCarrierEvents_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMbnMultiCarrierEvents_Impl, const OFFSET: isize>() -> IMbnMultiCarrierEvents_Vtbl {
        unsafe extern "system" fn OnSetHomeProviderComplete<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMbnMultiCarrierEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, mbninterface: *mut ::core::ffi::c_void, requestid: u32, status: ::windows::core::HRESULT) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnSetHomeProviderComplete(::windows::core::from_raw_borrowed(&mbninterface), ::core::mem::transmute_copy(&requestid), ::core::mem::transmute_copy(&status)).into()
        }
        unsafe extern "system" fn OnCurrentCellularClassChange<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMbnMultiCarrierEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, mbninterface: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnCurrentCellularClassChange(::windows::core::from_raw_borrowed(&mbninterface)).into()
        }
        unsafe extern "system" fn OnPreferredProvidersChange<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMbnMultiCarrierEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, mbninterface: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnPreferredProvidersChange(::windows::core::from_raw_borrowed(&mbninterface)).into()
        }
        unsafe extern "system" fn OnScanNetworkComplete<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMbnMultiCarrierEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, mbninterface: *mut ::core::ffi::c_void, requestid: u32, status: ::windows::core::HRESULT) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnScanNetworkComplete(::windows::core::from_raw_borrowed(&mbninterface), ::core::mem::transmute_copy(&requestid), ::core::mem::transmute_copy(&status)).into()
        }
        unsafe extern "system" fn OnInterfaceCapabilityChange<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMbnMultiCarrierEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, mbninterface: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnInterfaceCapabilityChange(::windows::core::from_raw_borrowed(&mbninterface)).into()
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            OnSetHomeProviderComplete: OnSetHomeProviderComplete::<Identity, Impl, OFFSET>,
            OnCurrentCellularClassChange: OnCurrentCellularClassChange::<Identity, Impl, OFFSET>,
            OnPreferredProvidersChange: OnPreferredProvidersChange::<Identity, Impl, OFFSET>,
            OnScanNetworkComplete: OnScanNetworkComplete::<Identity, Impl, OFFSET>,
            OnInterfaceCapabilityChange: OnInterfaceCapabilityChange::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMbnMultiCarrierEvents as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`, `\"implement\"`*"]
pub trait IMbnPin_Impl: Sized {
    fn PinType(&self) -> ::windows::core::Result<MBN_PIN_TYPE>;
    fn PinFormat(&self) -> ::windows::core::Result<MBN_PIN_FORMAT>;
    fn PinLengthMin(&self) -> ::windows::core::Result<u32>;
    fn PinLengthMax(&self) -> ::windows::core::Result<u32>;
    fn PinMode(&self) -> ::windows::core::Result<MBN_PIN_MODE>;
    fn Enable(&self, pin: &::windows::core::PCWSTR) -> ::windows::core::Result<u32>;
    fn Disable(&self, pin: &::windows::core::PCWSTR) -> ::windows::core::Result<u32>;
    fn Enter(&self, pin: &::windows::core::PCWSTR) -> ::windows::core::Result<u32>;
    fn Change(&self, pin: &::windows::core::PCWSTR, newpin: &::windows::core::PCWSTR) -> ::windows::core::Result<u32>;
    fn Unblock(&self, puk: &::windows::core::PCWSTR, newpin: &::windows::core::PCWSTR) -> ::windows::core::Result<u32>;
    fn GetPinManager(&self) -> ::windows::core::Result<IMbnPinManager>;
}
impl ::windows::core::RuntimeName for IMbnPin {}
impl IMbnPin_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMbnPin_Impl, const OFFSET: isize>() -> IMbnPin_Vtbl {
        unsafe extern "system" fn PinType<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMbnPin_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pintype: *mut MBN_PIN_TYPE) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.PinType() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pintype, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PinFormat<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMbnPin_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinformat: *mut MBN_PIN_FORMAT) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.PinFormat() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pinformat, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PinLengthMin<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMbnPin_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinlengthmin: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.PinLengthMin() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pinlengthmin, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PinLengthMax<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMbnPin_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinlengthmax: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.PinLengthMax() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pinlengthmax, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PinMode<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMbnPin_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinmode: *mut MBN_PIN_MODE) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.PinMode() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pinmode, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Enable<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMbnPin_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pin: ::windows::core::PCWSTR, requestid: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Enable(::core::mem::transmute(&pin)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(requestid, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Disable<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMbnPin_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pin: ::windows::core::PCWSTR, requestid: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Disable(::core::mem::transmute(&pin)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(requestid, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Enter<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMbnPin_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pin: ::windows::core::PCWSTR, requestid: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Enter(::core::mem::transmute(&pin)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(requestid, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Change<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMbnPin_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pin: ::windows::core::PCWSTR, newpin: ::windows::core::PCWSTR, requestid: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Change(::core::mem::transmute(&pin), ::core::mem::transmute(&newpin)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(requestid, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Unblock<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMbnPin_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, puk: ::windows::core::PCWSTR, newpin: ::windows::core::PCWSTR, requestid: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Unblock(::core::mem::transmute(&puk), ::core::mem::transmute(&newpin)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(requestid, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPinManager<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMbnPin_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinmanager: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetPinManager() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pinmanager, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
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
        iid == &<IMbnPin as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`, `\"implement\"`*"]
pub trait IMbnPinEvents_Impl: Sized {
    fn OnEnableComplete(&self, pin: ::core::option::Option<&IMbnPin>, pininfo: *const MBN_PIN_INFO, requestid: u32, status: ::windows::core::HRESULT) -> ::windows::core::Result<()>;
    fn OnDisableComplete(&self, pin: ::core::option::Option<&IMbnPin>, pininfo: *const MBN_PIN_INFO, requestid: u32, status: ::windows::core::HRESULT) -> ::windows::core::Result<()>;
    fn OnEnterComplete(&self, pin: ::core::option::Option<&IMbnPin>, pininfo: *const MBN_PIN_INFO, requestid: u32, status: ::windows::core::HRESULT) -> ::windows::core::Result<()>;
    fn OnChangeComplete(&self, pin: ::core::option::Option<&IMbnPin>, pininfo: *const MBN_PIN_INFO, requestid: u32, status: ::windows::core::HRESULT) -> ::windows::core::Result<()>;
    fn OnUnblockComplete(&self, pin: ::core::option::Option<&IMbnPin>, pininfo: *const MBN_PIN_INFO, requestid: u32, status: ::windows::core::HRESULT) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IMbnPinEvents {}
impl IMbnPinEvents_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMbnPinEvents_Impl, const OFFSET: isize>() -> IMbnPinEvents_Vtbl {
        unsafe extern "system" fn OnEnableComplete<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMbnPinEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pin: *mut ::core::ffi::c_void, pininfo: *const MBN_PIN_INFO, requestid: u32, status: ::windows::core::HRESULT) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnEnableComplete(::windows::core::from_raw_borrowed(&pin), ::core::mem::transmute_copy(&pininfo), ::core::mem::transmute_copy(&requestid), ::core::mem::transmute_copy(&status)).into()
        }
        unsafe extern "system" fn OnDisableComplete<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMbnPinEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pin: *mut ::core::ffi::c_void, pininfo: *const MBN_PIN_INFO, requestid: u32, status: ::windows::core::HRESULT) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnDisableComplete(::windows::core::from_raw_borrowed(&pin), ::core::mem::transmute_copy(&pininfo), ::core::mem::transmute_copy(&requestid), ::core::mem::transmute_copy(&status)).into()
        }
        unsafe extern "system" fn OnEnterComplete<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMbnPinEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pin: *mut ::core::ffi::c_void, pininfo: *const MBN_PIN_INFO, requestid: u32, status: ::windows::core::HRESULT) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnEnterComplete(::windows::core::from_raw_borrowed(&pin), ::core::mem::transmute_copy(&pininfo), ::core::mem::transmute_copy(&requestid), ::core::mem::transmute_copy(&status)).into()
        }
        unsafe extern "system" fn OnChangeComplete<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMbnPinEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pin: *mut ::core::ffi::c_void, pininfo: *const MBN_PIN_INFO, requestid: u32, status: ::windows::core::HRESULT) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnChangeComplete(::windows::core::from_raw_borrowed(&pin), ::core::mem::transmute_copy(&pininfo), ::core::mem::transmute_copy(&requestid), ::core::mem::transmute_copy(&status)).into()
        }
        unsafe extern "system" fn OnUnblockComplete<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMbnPinEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pin: *mut ::core::ffi::c_void, pininfo: *const MBN_PIN_INFO, requestid: u32, status: ::windows::core::HRESULT) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnUnblockComplete(::windows::core::from_raw_borrowed(&pin), ::core::mem::transmute_copy(&pininfo), ::core::mem::transmute_copy(&requestid), ::core::mem::transmute_copy(&status)).into()
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            OnEnableComplete: OnEnableComplete::<Identity, Impl, OFFSET>,
            OnDisableComplete: OnDisableComplete::<Identity, Impl, OFFSET>,
            OnEnterComplete: OnEnterComplete::<Identity, Impl, OFFSET>,
            OnChangeComplete: OnChangeComplete::<Identity, Impl, OFFSET>,
            OnUnblockComplete: OnUnblockComplete::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMbnPinEvents as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`, `\"Win32_System_Com\"`, `\"implement\"`*"]
#[cfg(feature = "Win32_System_Com")]
pub trait IMbnPinManager_Impl: Sized {
    fn GetPinList(&self) -> ::windows::core::Result<*mut super::super::System::Com::SAFEARRAY>;
    fn GetPin(&self, pintype: MBN_PIN_TYPE) -> ::windows::core::Result<IMbnPin>;
    fn GetPinState(&self) -> ::windows::core::Result<u32>;
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for IMbnPinManager {}
#[cfg(feature = "Win32_System_Com")]
impl IMbnPinManager_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMbnPinManager_Impl, const OFFSET: isize>() -> IMbnPinManager_Vtbl {
        unsafe extern "system" fn GetPinList<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMbnPinManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinlist: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetPinList() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pinlist, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPin<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMbnPinManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pintype: MBN_PIN_TYPE, pin: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetPin(::core::mem::transmute_copy(&pintype)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pin, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPinState<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMbnPinManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, requestid: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetPinState() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(requestid, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetPinList: GetPinList::<Identity, Impl, OFFSET>,
            GetPin: GetPin::<Identity, Impl, OFFSET>,
            GetPinState: GetPinState::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMbnPinManager as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`, `\"implement\"`*"]
pub trait IMbnPinManagerEvents_Impl: Sized {
    fn OnPinListAvailable(&self, pinmanager: ::core::option::Option<&IMbnPinManager>) -> ::windows::core::Result<()>;
    fn OnGetPinStateComplete(&self, pinmanager: ::core::option::Option<&IMbnPinManager>, pininfo: &MBN_PIN_INFO, requestid: u32, status: ::windows::core::HRESULT) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IMbnPinManagerEvents {}
impl IMbnPinManagerEvents_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMbnPinManagerEvents_Impl, const OFFSET: isize>() -> IMbnPinManagerEvents_Vtbl {
        unsafe extern "system" fn OnPinListAvailable<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMbnPinManagerEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinmanager: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnPinListAvailable(::windows::core::from_raw_borrowed(&pinmanager)).into()
        }
        unsafe extern "system" fn OnGetPinStateComplete<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMbnPinManagerEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinmanager: *mut ::core::ffi::c_void, pininfo: MBN_PIN_INFO, requestid: u32, status: ::windows::core::HRESULT) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnGetPinStateComplete(::windows::core::from_raw_borrowed(&pinmanager), ::core::mem::transmute(&pininfo), ::core::mem::transmute_copy(&requestid), ::core::mem::transmute_copy(&status)).into()
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            OnPinListAvailable: OnPinListAvailable::<Identity, Impl, OFFSET>,
            OnGetPinStateComplete: OnGetPinStateComplete::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMbnPinManagerEvents as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`, `\"implement\"`*"]
pub trait IMbnRadio_Impl: Sized {
    fn SoftwareRadioState(&self) -> ::windows::core::Result<MBN_RADIO>;
    fn HardwareRadioState(&self) -> ::windows::core::Result<MBN_RADIO>;
    fn SetSoftwareRadioState(&self, radiostate: MBN_RADIO) -> ::windows::core::Result<u32>;
}
impl ::windows::core::RuntimeName for IMbnRadio {}
impl IMbnRadio_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMbnRadio_Impl, const OFFSET: isize>() -> IMbnRadio_Vtbl {
        unsafe extern "system" fn SoftwareRadioState<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMbnRadio_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, softwareradiostate: *mut MBN_RADIO) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.SoftwareRadioState() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(softwareradiostate, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn HardwareRadioState<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMbnRadio_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hardwareradiostate: *mut MBN_RADIO) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.HardwareRadioState() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(hardwareradiostate, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSoftwareRadioState<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMbnRadio_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, radiostate: MBN_RADIO, requestid: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.SetSoftwareRadioState(::core::mem::transmute_copy(&radiostate)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(requestid, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            SoftwareRadioState: SoftwareRadioState::<Identity, Impl, OFFSET>,
            HardwareRadioState: HardwareRadioState::<Identity, Impl, OFFSET>,
            SetSoftwareRadioState: SetSoftwareRadioState::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMbnRadio as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`, `\"implement\"`*"]
pub trait IMbnRadioEvents_Impl: Sized {
    fn OnRadioStateChange(&self, newinterface: ::core::option::Option<&IMbnRadio>) -> ::windows::core::Result<()>;
    fn OnSetSoftwareRadioStateComplete(&self, newinterface: ::core::option::Option<&IMbnRadio>, requestid: u32, status: ::windows::core::HRESULT) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IMbnRadioEvents {}
impl IMbnRadioEvents_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMbnRadioEvents_Impl, const OFFSET: isize>() -> IMbnRadioEvents_Vtbl {
        unsafe extern "system" fn OnRadioStateChange<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMbnRadioEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newinterface: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnRadioStateChange(::windows::core::from_raw_borrowed(&newinterface)).into()
        }
        unsafe extern "system" fn OnSetSoftwareRadioStateComplete<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMbnRadioEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newinterface: *mut ::core::ffi::c_void, requestid: u32, status: ::windows::core::HRESULT) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnSetSoftwareRadioStateComplete(::windows::core::from_raw_borrowed(&newinterface), ::core::mem::transmute_copy(&requestid), ::core::mem::transmute_copy(&status)).into()
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            OnRadioStateChange: OnRadioStateChange::<Identity, Impl, OFFSET>,
            OnSetSoftwareRadioStateComplete: OnSetSoftwareRadioStateComplete::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMbnRadioEvents as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`, `\"implement\"`*"]
pub trait IMbnRegistration_Impl: Sized {
    fn GetRegisterState(&self) -> ::windows::core::Result<MBN_REGISTER_STATE>;
    fn GetRegisterMode(&self) -> ::windows::core::Result<MBN_REGISTER_MODE>;
    fn GetProviderID(&self) -> ::windows::core::Result<::windows::core::BSTR>;
    fn GetProviderName(&self) -> ::windows::core::Result<::windows::core::BSTR>;
    fn GetRoamingText(&self) -> ::windows::core::Result<::windows::core::BSTR>;
    fn GetAvailableDataClasses(&self) -> ::windows::core::Result<u32>;
    fn GetCurrentDataClass(&self) -> ::windows::core::Result<u32>;
    fn GetRegistrationNetworkError(&self) -> ::windows::core::Result<u32>;
    fn GetPacketAttachNetworkError(&self) -> ::windows::core::Result<u32>;
    fn SetRegisterMode(&self, registermode: MBN_REGISTER_MODE, providerid: &::windows::core::PCWSTR, dataclass: u32) -> ::windows::core::Result<u32>;
}
impl ::windows::core::RuntimeName for IMbnRegistration {}
impl IMbnRegistration_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMbnRegistration_Impl, const OFFSET: isize>() -> IMbnRegistration_Vtbl {
        unsafe extern "system" fn GetRegisterState<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMbnRegistration_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, registerstate: *mut MBN_REGISTER_STATE) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetRegisterState() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(registerstate, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetRegisterMode<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMbnRegistration_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, registermode: *mut MBN_REGISTER_MODE) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetRegisterMode() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(registermode, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetProviderID<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMbnRegistration_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, providerid: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetProviderID() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(providerid, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetProviderName<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMbnRegistration_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, providername: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetProviderName() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(providername, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetRoamingText<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMbnRegistration_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, roamingtext: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetRoamingText() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(roamingtext, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetAvailableDataClasses<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMbnRegistration_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, availabledataclasses: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetAvailableDataClasses() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(availabledataclasses, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCurrentDataClass<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMbnRegistration_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, currentdataclass: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetCurrentDataClass() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(currentdataclass, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetRegistrationNetworkError<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMbnRegistration_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, registrationnetworkerror: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetRegistrationNetworkError() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(registrationnetworkerror, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPacketAttachNetworkError<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMbnRegistration_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, packetattachnetworkerror: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetPacketAttachNetworkError() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(packetattachnetworkerror, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetRegisterMode<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMbnRegistration_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, registermode: MBN_REGISTER_MODE, providerid: ::windows::core::PCWSTR, dataclass: u32, requestid: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.SetRegisterMode(::core::mem::transmute_copy(&registermode), ::core::mem::transmute(&providerid), ::core::mem::transmute_copy(&dataclass)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(requestid, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
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
        iid == &<IMbnRegistration as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`, `\"implement\"`*"]
pub trait IMbnRegistrationEvents_Impl: Sized {
    fn OnRegisterModeAvailable(&self, newinterface: ::core::option::Option<&IMbnRegistration>) -> ::windows::core::Result<()>;
    fn OnRegisterStateChange(&self, newinterface: ::core::option::Option<&IMbnRegistration>) -> ::windows::core::Result<()>;
    fn OnPacketServiceStateChange(&self, newinterface: ::core::option::Option<&IMbnRegistration>) -> ::windows::core::Result<()>;
    fn OnSetRegisterModeComplete(&self, newinterface: ::core::option::Option<&IMbnRegistration>, requestid: u32, status: ::windows::core::HRESULT) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IMbnRegistrationEvents {}
impl IMbnRegistrationEvents_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMbnRegistrationEvents_Impl, const OFFSET: isize>() -> IMbnRegistrationEvents_Vtbl {
        unsafe extern "system" fn OnRegisterModeAvailable<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMbnRegistrationEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newinterface: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnRegisterModeAvailable(::windows::core::from_raw_borrowed(&newinterface)).into()
        }
        unsafe extern "system" fn OnRegisterStateChange<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMbnRegistrationEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newinterface: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnRegisterStateChange(::windows::core::from_raw_borrowed(&newinterface)).into()
        }
        unsafe extern "system" fn OnPacketServiceStateChange<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMbnRegistrationEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newinterface: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnPacketServiceStateChange(::windows::core::from_raw_borrowed(&newinterface)).into()
        }
        unsafe extern "system" fn OnSetRegisterModeComplete<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMbnRegistrationEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newinterface: *mut ::core::ffi::c_void, requestid: u32, status: ::windows::core::HRESULT) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnSetRegisterModeComplete(::windows::core::from_raw_borrowed(&newinterface), ::core::mem::transmute_copy(&requestid), ::core::mem::transmute_copy(&status)).into()
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            OnRegisterModeAvailable: OnRegisterModeAvailable::<Identity, Impl, OFFSET>,
            OnRegisterStateChange: OnRegisterStateChange::<Identity, Impl, OFFSET>,
            OnPacketServiceStateChange: OnPacketServiceStateChange::<Identity, Impl, OFFSET>,
            OnSetRegisterModeComplete: OnSetRegisterModeComplete::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMbnRegistrationEvents as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`, `\"Win32_System_Com\"`, `\"implement\"`*"]
#[cfg(feature = "Win32_System_Com")]
pub trait IMbnServiceActivation_Impl: Sized {
    fn Activate(&self, vendorspecificdata: *const super::super::System::Com::SAFEARRAY) -> ::windows::core::Result<u32>;
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for IMbnServiceActivation {}
#[cfg(feature = "Win32_System_Com")]
impl IMbnServiceActivation_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMbnServiceActivation_Impl, const OFFSET: isize>() -> IMbnServiceActivation_Vtbl {
        unsafe extern "system" fn Activate<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMbnServiceActivation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, vendorspecificdata: *const super::super::System::Com::SAFEARRAY, requestid: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Activate(::core::mem::transmute_copy(&vendorspecificdata)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(requestid, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(), Activate: Activate::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMbnServiceActivation as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`, `\"Win32_System_Com\"`, `\"implement\"`*"]
#[cfg(feature = "Win32_System_Com")]
pub trait IMbnServiceActivationEvents_Impl: Sized {
    fn OnActivationComplete(&self, serviceactivation: ::core::option::Option<&IMbnServiceActivation>, vendorspecificdata: *const super::super::System::Com::SAFEARRAY, requestid: u32, status: ::windows::core::HRESULT, networkerror: u32) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for IMbnServiceActivationEvents {}
#[cfg(feature = "Win32_System_Com")]
impl IMbnServiceActivationEvents_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMbnServiceActivationEvents_Impl, const OFFSET: isize>() -> IMbnServiceActivationEvents_Vtbl {
        unsafe extern "system" fn OnActivationComplete<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMbnServiceActivationEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, serviceactivation: *mut ::core::ffi::c_void, vendorspecificdata: *const super::super::System::Com::SAFEARRAY, requestid: u32, status: ::windows::core::HRESULT, networkerror: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnActivationComplete(::windows::core::from_raw_borrowed(&serviceactivation), ::core::mem::transmute_copy(&vendorspecificdata), ::core::mem::transmute_copy(&requestid), ::core::mem::transmute_copy(&status), ::core::mem::transmute_copy(&networkerror)).into()
        }
        Self { base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(), OnActivationComplete: OnActivationComplete::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMbnServiceActivationEvents as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`, `\"implement\"`*"]
pub trait IMbnSignal_Impl: Sized {
    fn GetSignalStrength(&self) -> ::windows::core::Result<u32>;
    fn GetSignalError(&self) -> ::windows::core::Result<u32>;
}
impl ::windows::core::RuntimeName for IMbnSignal {}
impl IMbnSignal_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMbnSignal_Impl, const OFFSET: isize>() -> IMbnSignal_Vtbl {
        unsafe extern "system" fn GetSignalStrength<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMbnSignal_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, signalstrength: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetSignalStrength() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(signalstrength, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSignalError<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMbnSignal_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, signalerror: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetSignalError() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(signalerror, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetSignalStrength: GetSignalStrength::<Identity, Impl, OFFSET>,
            GetSignalError: GetSignalError::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMbnSignal as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`, `\"implement\"`*"]
pub trait IMbnSignalEvents_Impl: Sized {
    fn OnSignalStateChange(&self, newinterface: ::core::option::Option<&IMbnSignal>) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IMbnSignalEvents {}
impl IMbnSignalEvents_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMbnSignalEvents_Impl, const OFFSET: isize>() -> IMbnSignalEvents_Vtbl {
        unsafe extern "system" fn OnSignalStateChange<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMbnSignalEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newinterface: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnSignalStateChange(::windows::core::from_raw_borrowed(&newinterface)).into()
        }
        Self { base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(), OnSignalStateChange: OnSignalStateChange::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMbnSignalEvents as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`, `\"Win32_System_Com\"`, `\"implement\"`*"]
#[cfg(feature = "Win32_System_Com")]
pub trait IMbnSms_Impl: Sized {
    fn GetSmsConfiguration(&self) -> ::windows::core::Result<IMbnSmsConfiguration>;
    fn SetSmsConfiguration(&self, smsconfiguration: ::core::option::Option<&IMbnSmsConfiguration>) -> ::windows::core::Result<u32>;
    fn SmsSendPdu(&self, pdudata: &::windows::core::PCWSTR, size: u8) -> ::windows::core::Result<u32>;
    fn SmsSendCdma(&self, address: &::windows::core::PCWSTR, encoding: MBN_SMS_CDMA_ENCODING, language: MBN_SMS_CDMA_LANG, sizeincharacters: u32, message: *const super::super::System::Com::SAFEARRAY) -> ::windows::core::Result<u32>;
    fn SmsSendCdmaPdu(&self, message: *const super::super::System::Com::SAFEARRAY) -> ::windows::core::Result<u32>;
    fn SmsRead(&self, smsfilter: *const MBN_SMS_FILTER, smsformat: MBN_SMS_FORMAT) -> ::windows::core::Result<u32>;
    fn SmsDelete(&self, smsfilter: *const MBN_SMS_FILTER) -> ::windows::core::Result<u32>;
    fn GetSmsStatus(&self) -> ::windows::core::Result<MBN_SMS_STATUS_INFO>;
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for IMbnSms {}
#[cfg(feature = "Win32_System_Com")]
impl IMbnSms_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMbnSms_Impl, const OFFSET: isize>() -> IMbnSms_Vtbl {
        unsafe extern "system" fn GetSmsConfiguration<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMbnSms_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, smsconfiguration: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetSmsConfiguration() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(smsconfiguration, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSmsConfiguration<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMbnSms_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, smsconfiguration: *mut ::core::ffi::c_void, requestid: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.SetSmsConfiguration(::windows::core::from_raw_borrowed(&smsconfiguration)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(requestid, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SmsSendPdu<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMbnSms_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdudata: ::windows::core::PCWSTR, size: u8, requestid: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.SmsSendPdu(::core::mem::transmute(&pdudata), ::core::mem::transmute_copy(&size)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(requestid, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SmsSendCdma<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMbnSms_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, address: ::windows::core::PCWSTR, encoding: MBN_SMS_CDMA_ENCODING, language: MBN_SMS_CDMA_LANG, sizeincharacters: u32, message: *const super::super::System::Com::SAFEARRAY, requestid: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.SmsSendCdma(::core::mem::transmute(&address), ::core::mem::transmute_copy(&encoding), ::core::mem::transmute_copy(&language), ::core::mem::transmute_copy(&sizeincharacters), ::core::mem::transmute_copy(&message)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(requestid, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SmsSendCdmaPdu<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMbnSms_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, message: *const super::super::System::Com::SAFEARRAY, requestid: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.SmsSendCdmaPdu(::core::mem::transmute_copy(&message)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(requestid, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SmsRead<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMbnSms_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, smsfilter: *const MBN_SMS_FILTER, smsformat: MBN_SMS_FORMAT, requestid: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.SmsRead(::core::mem::transmute_copy(&smsfilter), ::core::mem::transmute_copy(&smsformat)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(requestid, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SmsDelete<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMbnSms_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, smsfilter: *const MBN_SMS_FILTER, requestid: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.SmsDelete(::core::mem::transmute_copy(&smsfilter)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(requestid, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSmsStatus<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMbnSms_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, smsstatusinfo: *mut MBN_SMS_STATUS_INFO) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetSmsStatus() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(smsstatusinfo, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
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
        iid == &<IMbnSms as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`, `\"implement\"`*"]
pub trait IMbnSmsConfiguration_Impl: Sized {
    fn ServiceCenterAddress(&self) -> ::windows::core::Result<::windows::core::BSTR>;
    fn SetServiceCenterAddress(&self, scaddress: &::windows::core::PCWSTR) -> ::windows::core::Result<()>;
    fn MaxMessageIndex(&self) -> ::windows::core::Result<u32>;
    fn CdmaShortMsgSize(&self) -> ::windows::core::Result<u32>;
    fn SmsFormat(&self) -> ::windows::core::Result<MBN_SMS_FORMAT>;
    fn SetSmsFormat(&self, smsformat: MBN_SMS_FORMAT) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IMbnSmsConfiguration {}
impl IMbnSmsConfiguration_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMbnSmsConfiguration_Impl, const OFFSET: isize>() -> IMbnSmsConfiguration_Vtbl {
        unsafe extern "system" fn ServiceCenterAddress<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMbnSmsConfiguration_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, scaddress: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.ServiceCenterAddress() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(scaddress, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetServiceCenterAddress<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMbnSmsConfiguration_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, scaddress: ::windows::core::PCWSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetServiceCenterAddress(::core::mem::transmute(&scaddress)).into()
        }
        unsafe extern "system" fn MaxMessageIndex<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMbnSmsConfiguration_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.MaxMessageIndex() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(index, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CdmaShortMsgSize<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMbnSmsConfiguration_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, shortmsgsize: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CdmaShortMsgSize() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(shortmsgsize, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SmsFormat<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMbnSmsConfiguration_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, smsformat: *mut MBN_SMS_FORMAT) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.SmsFormat() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(smsformat, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSmsFormat<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMbnSmsConfiguration_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, smsformat: MBN_SMS_FORMAT) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetSmsFormat(::core::mem::transmute_copy(&smsformat)).into()
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            ServiceCenterAddress: ServiceCenterAddress::<Identity, Impl, OFFSET>,
            SetServiceCenterAddress: SetServiceCenterAddress::<Identity, Impl, OFFSET>,
            MaxMessageIndex: MaxMessageIndex::<Identity, Impl, OFFSET>,
            CdmaShortMsgSize: CdmaShortMsgSize::<Identity, Impl, OFFSET>,
            SmsFormat: SmsFormat::<Identity, Impl, OFFSET>,
            SetSmsFormat: SetSmsFormat::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMbnSmsConfiguration as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub trait IMbnSmsEvents_Impl: Sized {
    fn OnSmsConfigurationChange(&self, sms: ::core::option::Option<&IMbnSms>) -> ::windows::core::Result<()>;
    fn OnSetSmsConfigurationComplete(&self, sms: ::core::option::Option<&IMbnSms>, requestid: u32, status: ::windows::core::HRESULT) -> ::windows::core::Result<()>;
    fn OnSmsSendComplete(&self, sms: ::core::option::Option<&IMbnSms>, requestid: u32, status: ::windows::core::HRESULT) -> ::windows::core::Result<()>;
    fn OnSmsReadComplete(&self, sms: ::core::option::Option<&IMbnSms>, smsformat: MBN_SMS_FORMAT, readmsgs: *const super::super::System::Com::SAFEARRAY, moremsgs: super::super::Foundation::VARIANT_BOOL, requestid: u32, status: ::windows::core::HRESULT) -> ::windows::core::Result<()>;
    fn OnSmsNewClass0Message(&self, sms: ::core::option::Option<&IMbnSms>, smsformat: MBN_SMS_FORMAT, readmsgs: *const super::super::System::Com::SAFEARRAY) -> ::windows::core::Result<()>;
    fn OnSmsDeleteComplete(&self, sms: ::core::option::Option<&IMbnSms>, requestid: u32, status: ::windows::core::HRESULT) -> ::windows::core::Result<()>;
    fn OnSmsStatusChange(&self, sms: ::core::option::Option<&IMbnSms>) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::windows::core::RuntimeName for IMbnSmsEvents {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl IMbnSmsEvents_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMbnSmsEvents_Impl, const OFFSET: isize>() -> IMbnSmsEvents_Vtbl {
        unsafe extern "system" fn OnSmsConfigurationChange<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMbnSmsEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, sms: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnSmsConfigurationChange(::windows::core::from_raw_borrowed(&sms)).into()
        }
        unsafe extern "system" fn OnSetSmsConfigurationComplete<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMbnSmsEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, sms: *mut ::core::ffi::c_void, requestid: u32, status: ::windows::core::HRESULT) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnSetSmsConfigurationComplete(::windows::core::from_raw_borrowed(&sms), ::core::mem::transmute_copy(&requestid), ::core::mem::transmute_copy(&status)).into()
        }
        unsafe extern "system" fn OnSmsSendComplete<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMbnSmsEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, sms: *mut ::core::ffi::c_void, requestid: u32, status: ::windows::core::HRESULT) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnSmsSendComplete(::windows::core::from_raw_borrowed(&sms), ::core::mem::transmute_copy(&requestid), ::core::mem::transmute_copy(&status)).into()
        }
        unsafe extern "system" fn OnSmsReadComplete<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMbnSmsEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, sms: *mut ::core::ffi::c_void, smsformat: MBN_SMS_FORMAT, readmsgs: *const super::super::System::Com::SAFEARRAY, moremsgs: super::super::Foundation::VARIANT_BOOL, requestid: u32, status: ::windows::core::HRESULT) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnSmsReadComplete(::windows::core::from_raw_borrowed(&sms), ::core::mem::transmute_copy(&smsformat), ::core::mem::transmute_copy(&readmsgs), ::core::mem::transmute_copy(&moremsgs), ::core::mem::transmute_copy(&requestid), ::core::mem::transmute_copy(&status)).into()
        }
        unsafe extern "system" fn OnSmsNewClass0Message<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMbnSmsEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, sms: *mut ::core::ffi::c_void, smsformat: MBN_SMS_FORMAT, readmsgs: *const super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnSmsNewClass0Message(::windows::core::from_raw_borrowed(&sms), ::core::mem::transmute_copy(&smsformat), ::core::mem::transmute_copy(&readmsgs)).into()
        }
        unsafe extern "system" fn OnSmsDeleteComplete<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMbnSmsEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, sms: *mut ::core::ffi::c_void, requestid: u32, status: ::windows::core::HRESULT) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnSmsDeleteComplete(::windows::core::from_raw_borrowed(&sms), ::core::mem::transmute_copy(&requestid), ::core::mem::transmute_copy(&status)).into()
        }
        unsafe extern "system" fn OnSmsStatusChange<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMbnSmsEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, sms: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnSmsStatusChange(::windows::core::from_raw_borrowed(&sms)).into()
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
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
        iid == &<IMbnSmsEvents as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`, `\"Win32_System_Com\"`, `\"implement\"`*"]
#[cfg(feature = "Win32_System_Com")]
pub trait IMbnSmsReadMsgPdu_Impl: Sized {
    fn Index(&self) -> ::windows::core::Result<u32>;
    fn Status(&self) -> ::windows::core::Result<MBN_MSG_STATUS>;
    fn PduData(&self) -> ::windows::core::Result<::windows::core::BSTR>;
    fn Message(&self) -> ::windows::core::Result<*mut super::super::System::Com::SAFEARRAY>;
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for IMbnSmsReadMsgPdu {}
#[cfg(feature = "Win32_System_Com")]
impl IMbnSmsReadMsgPdu_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMbnSmsReadMsgPdu_Impl, const OFFSET: isize>() -> IMbnSmsReadMsgPdu_Vtbl {
        unsafe extern "system" fn Index<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMbnSmsReadMsgPdu_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Index() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(index, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Status<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMbnSmsReadMsgPdu_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, status: *mut MBN_MSG_STATUS) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Status() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(status, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PduData<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMbnSmsReadMsgPdu_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdudata: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.PduData() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pdudata, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Message<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMbnSmsReadMsgPdu_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, message: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Message() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(message, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Index: Index::<Identity, Impl, OFFSET>,
            Status: Status::<Identity, Impl, OFFSET>,
            PduData: PduData::<Identity, Impl, OFFSET>,
            Message: Message::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMbnSmsReadMsgPdu as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`, `\"Win32_System_Com\"`, `\"implement\"`*"]
#[cfg(feature = "Win32_System_Com")]
pub trait IMbnSmsReadMsgTextCdma_Impl: Sized {
    fn Index(&self) -> ::windows::core::Result<u32>;
    fn Status(&self) -> ::windows::core::Result<MBN_MSG_STATUS>;
    fn Address(&self) -> ::windows::core::Result<::windows::core::BSTR>;
    fn Timestamp(&self) -> ::windows::core::Result<::windows::core::BSTR>;
    fn EncodingID(&self) -> ::windows::core::Result<MBN_SMS_CDMA_ENCODING>;
    fn LanguageID(&self) -> ::windows::core::Result<MBN_SMS_CDMA_LANG>;
    fn SizeInCharacters(&self) -> ::windows::core::Result<u32>;
    fn Message(&self) -> ::windows::core::Result<*mut super::super::System::Com::SAFEARRAY>;
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for IMbnSmsReadMsgTextCdma {}
#[cfg(feature = "Win32_System_Com")]
impl IMbnSmsReadMsgTextCdma_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMbnSmsReadMsgTextCdma_Impl, const OFFSET: isize>() -> IMbnSmsReadMsgTextCdma_Vtbl {
        unsafe extern "system" fn Index<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMbnSmsReadMsgTextCdma_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Index() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(index, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Status<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMbnSmsReadMsgTextCdma_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, status: *mut MBN_MSG_STATUS) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Status() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(status, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Address<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMbnSmsReadMsgTextCdma_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, address: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Address() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(address, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Timestamp<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMbnSmsReadMsgTextCdma_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, timestamp: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Timestamp() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(timestamp, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EncodingID<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMbnSmsReadMsgTextCdma_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, encodingid: *mut MBN_SMS_CDMA_ENCODING) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.EncodingID() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(encodingid, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LanguageID<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMbnSmsReadMsgTextCdma_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, languageid: *mut MBN_SMS_CDMA_LANG) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.LanguageID() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(languageid, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SizeInCharacters<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMbnSmsReadMsgTextCdma_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, sizeincharacters: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.SizeInCharacters() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(sizeincharacters, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Message<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMbnSmsReadMsgTextCdma_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, message: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Message() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(message, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
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
        iid == &<IMbnSmsReadMsgTextCdma as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`, `\"Win32_System_Com\"`, `\"implement\"`*"]
#[cfg(feature = "Win32_System_Com")]
pub trait IMbnSubscriberInformation_Impl: Sized {
    fn SubscriberID(&self) -> ::windows::core::Result<::windows::core::BSTR>;
    fn SimIccID(&self) -> ::windows::core::Result<::windows::core::BSTR>;
    fn TelephoneNumbers(&self) -> ::windows::core::Result<*mut super::super::System::Com::SAFEARRAY>;
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for IMbnSubscriberInformation {}
#[cfg(feature = "Win32_System_Com")]
impl IMbnSubscriberInformation_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMbnSubscriberInformation_Impl, const OFFSET: isize>() -> IMbnSubscriberInformation_Vtbl {
        unsafe extern "system" fn SubscriberID<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMbnSubscriberInformation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, subscriberid: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.SubscriberID() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(subscriberid, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SimIccID<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMbnSubscriberInformation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, simiccid: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.SimIccID() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(simiccid, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TelephoneNumbers<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMbnSubscriberInformation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, telephonenumbers: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.TelephoneNumbers() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(telephonenumbers, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            SubscriberID: SubscriberID::<Identity, Impl, OFFSET>,
            SimIccID: SimIccID::<Identity, Impl, OFFSET>,
            TelephoneNumbers: TelephoneNumbers::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMbnSubscriberInformation as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`, `\"Win32_System_Com\"`, `\"implement\"`*"]
#[cfg(feature = "Win32_System_Com")]
pub trait IMbnVendorSpecificEvents_Impl: Sized {
    fn OnEventNotification(&self, vendoroperation: ::core::option::Option<&IMbnVendorSpecificOperation>, vendorspecificdata: *const super::super::System::Com::SAFEARRAY) -> ::windows::core::Result<()>;
    fn OnSetVendorSpecificComplete(&self, vendoroperation: ::core::option::Option<&IMbnVendorSpecificOperation>, vendorspecificdata: *const super::super::System::Com::SAFEARRAY, requestid: u32) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for IMbnVendorSpecificEvents {}
#[cfg(feature = "Win32_System_Com")]
impl IMbnVendorSpecificEvents_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMbnVendorSpecificEvents_Impl, const OFFSET: isize>() -> IMbnVendorSpecificEvents_Vtbl {
        unsafe extern "system" fn OnEventNotification<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMbnVendorSpecificEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, vendoroperation: *mut ::core::ffi::c_void, vendorspecificdata: *const super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnEventNotification(::windows::core::from_raw_borrowed(&vendoroperation), ::core::mem::transmute_copy(&vendorspecificdata)).into()
        }
        unsafe extern "system" fn OnSetVendorSpecificComplete<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMbnVendorSpecificEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, vendoroperation: *mut ::core::ffi::c_void, vendorspecificdata: *const super::super::System::Com::SAFEARRAY, requestid: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnSetVendorSpecificComplete(::windows::core::from_raw_borrowed(&vendoroperation), ::core::mem::transmute_copy(&vendorspecificdata), ::core::mem::transmute_copy(&requestid)).into()
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            OnEventNotification: OnEventNotification::<Identity, Impl, OFFSET>,
            OnSetVendorSpecificComplete: OnSetVendorSpecificComplete::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMbnVendorSpecificEvents as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_MobileBroadband\"`, `\"Win32_System_Com\"`, `\"implement\"`*"]
#[cfg(feature = "Win32_System_Com")]
pub trait IMbnVendorSpecificOperation_Impl: Sized {
    fn SetVendorSpecific(&self, vendorspecificdata: *const super::super::System::Com::SAFEARRAY) -> ::windows::core::Result<u32>;
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for IMbnVendorSpecificOperation {}
#[cfg(feature = "Win32_System_Com")]
impl IMbnVendorSpecificOperation_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMbnVendorSpecificOperation_Impl, const OFFSET: isize>() -> IMbnVendorSpecificOperation_Vtbl {
        unsafe extern "system" fn SetVendorSpecific<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMbnVendorSpecificOperation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, vendorspecificdata: *const super::super::System::Com::SAFEARRAY, requestid: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.SetVendorSpecific(::core::mem::transmute_copy(&vendorspecificdata)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(requestid, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(), SetVendorSpecific: SetVendorSpecific::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMbnVendorSpecificOperation as ::windows::core::ComInterface>::IID
    }
}
