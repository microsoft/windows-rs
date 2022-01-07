#[cfg(feature = "Win32_System_Com")]
pub trait IDummyMBNUCMExtImpl: Sized + IDispatchImpl {}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for IDummyMBNUCMExt {
    const NAME: &'static str = "Windows.Win32.NetworkManagement.MobileBroadband.IDummyMBNUCMExt";
}
#[cfg(feature = "Win32_System_Com")]
impl IDummyMBNUCMExtVtbl {
    pub const fn new<Impl: IDummyMBNUCMExtImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IDummyMBNUCMExtVtbl {
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IDummyMBNUCMExt>, base.5)
    }
}
pub trait IMbnConnectionImpl: Sized {
    fn ConnectionID();
    fn InterfaceID();
    fn Connect();
    fn Disconnect();
    fn GetConnectionState();
    fn GetVoiceCallState();
    fn GetActivationNetworkError();
}
impl ::windows::core::RuntimeName for IMbnConnection {
    const NAME: &'static str = "Windows.Win32.NetworkManagement.MobileBroadband.IMbnConnection";
}
impl IMbnConnectionVtbl {
    pub const fn new<Impl: IMbnConnectionImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IMbnConnectionVtbl {
        unsafe extern "system" fn ConnectionID<Impl: IMbnConnectionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, connectionid: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ConnectionID(::core::mem::transmute_copy(&connectionid)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn InterfaceID<Impl: IMbnConnectionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, interfaceid: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).InterfaceID(::core::mem::transmute_copy(&interfaceid)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Connect<Impl: IMbnConnectionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, connectionmode: MBN_CONNECTION_MODE, strprofile: super::super::Foundation::PWSTR, requestid: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Connect(connectionmode, &*(&strprofile as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&requestid)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Disconnect<Impl: IMbnConnectionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, requestid: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Disconnect(::core::mem::transmute_copy(&requestid)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetConnectionState<Impl: IMbnConnectionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, connectionstate: *mut MBN_ACTIVATION_STATE, profilename: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetConnectionState(::core::mem::transmute_copy(&connectionstate), ::core::mem::transmute_copy(&profilename)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetVoiceCallState<Impl: IMbnConnectionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, voicecallstate: *mut MBN_VOICE_CALL_STATE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetVoiceCallState(::core::mem::transmute_copy(&voicecallstate)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetActivationNetworkError<Impl: IMbnConnectionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, networkerror: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetActivationNetworkError(::core::mem::transmute_copy(&networkerror)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IMbnConnection>, base.5, ConnectionID::<Impl, OFFSET>, InterfaceID::<Impl, OFFSET>, Connect::<Impl, OFFSET>, Disconnect::<Impl, OFFSET>, GetConnectionState::<Impl, OFFSET>, GetVoiceCallState::<Impl, OFFSET>, GetActivationNetworkError::<Impl, OFFSET>)
    }
}
pub trait IMbnConnectionContextImpl: Sized {
    fn GetProvisionedContexts();
    fn SetProvisionedContext();
}
impl ::windows::core::RuntimeName for IMbnConnectionContext {
    const NAME: &'static str = "Windows.Win32.NetworkManagement.MobileBroadband.IMbnConnectionContext";
}
impl IMbnConnectionContextVtbl {
    pub const fn new<Impl: IMbnConnectionContextImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IMbnConnectionContextVtbl {
        unsafe extern "system" fn GetProvisionedContexts<Impl: IMbnConnectionContextImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, provisionedcontexts: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetProvisionedContexts(::core::mem::transmute_copy(&provisionedcontexts)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetProvisionedContext<Impl: IMbnConnectionContextImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, provisionedcontexts: ::core::mem::ManuallyDrop<MBN_CONTEXT>, providerid: super::super::Foundation::PWSTR, requestid: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetProvisionedContext(&*(&provisionedcontexts as *const <MBN_CONTEXT as ::windows::core::Abi>::Abi as *const <MBN_CONTEXT as ::windows::core::DefaultType>::DefaultType), &*(&providerid as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&requestid)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IMbnConnectionContext>, base.5, GetProvisionedContexts::<Impl, OFFSET>, SetProvisionedContext::<Impl, OFFSET>)
    }
}
pub trait IMbnConnectionContextEventsImpl: Sized {
    fn OnProvisionedContextListChange();
    fn OnSetProvisionedContextComplete();
}
impl ::windows::core::RuntimeName for IMbnConnectionContextEvents {
    const NAME: &'static str = "Windows.Win32.NetworkManagement.MobileBroadband.IMbnConnectionContextEvents";
}
impl IMbnConnectionContextEventsVtbl {
    pub const fn new<Impl: IMbnConnectionContextEventsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IMbnConnectionContextEventsVtbl {
        unsafe extern "system" fn OnProvisionedContextListChange<Impl: IMbnConnectionContextEventsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, newinterface: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).OnProvisionedContextListChange(&*(&newinterface as *const <IMbnConnectionContext as ::windows::core::Abi>::Abi as *const <IMbnConnectionContext as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OnSetProvisionedContextComplete<Impl: IMbnConnectionContextEventsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, newinterface: ::windows::core::RawPtr, requestid: u32, status: ::windows::core::HRESULT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).OnSetProvisionedContextComplete(&*(&newinterface as *const <IMbnConnectionContext as ::windows::core::Abi>::Abi as *const <IMbnConnectionContext as ::windows::core::DefaultType>::DefaultType), requestid, status) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IMbnConnectionContextEvents>, base.5, OnProvisionedContextListChange::<Impl, OFFSET>, OnSetProvisionedContextComplete::<Impl, OFFSET>)
    }
}
pub trait IMbnConnectionEventsImpl: Sized {
    fn OnConnectComplete();
    fn OnDisconnectComplete();
    fn OnConnectStateChange();
    fn OnVoiceCallStateChange();
}
impl ::windows::core::RuntimeName for IMbnConnectionEvents {
    const NAME: &'static str = "Windows.Win32.NetworkManagement.MobileBroadband.IMbnConnectionEvents";
}
impl IMbnConnectionEventsVtbl {
    pub const fn new<Impl: IMbnConnectionEventsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IMbnConnectionEventsVtbl {
        unsafe extern "system" fn OnConnectComplete<Impl: IMbnConnectionEventsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, newconnection: ::windows::core::RawPtr, requestid: u32, status: ::windows::core::HRESULT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).OnConnectComplete(&*(&newconnection as *const <IMbnConnection as ::windows::core::Abi>::Abi as *const <IMbnConnection as ::windows::core::DefaultType>::DefaultType), requestid, status) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OnDisconnectComplete<Impl: IMbnConnectionEventsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, newconnection: ::windows::core::RawPtr, requestid: u32, status: ::windows::core::HRESULT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).OnDisconnectComplete(&*(&newconnection as *const <IMbnConnection as ::windows::core::Abi>::Abi as *const <IMbnConnection as ::windows::core::DefaultType>::DefaultType), requestid, status) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OnConnectStateChange<Impl: IMbnConnectionEventsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, newconnection: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).OnConnectStateChange(&*(&newconnection as *const <IMbnConnection as ::windows::core::Abi>::Abi as *const <IMbnConnection as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OnVoiceCallStateChange<Impl: IMbnConnectionEventsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, newconnection: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).OnVoiceCallStateChange(&*(&newconnection as *const <IMbnConnection as ::windows::core::Abi>::Abi as *const <IMbnConnection as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IMbnConnectionEvents>, base.5, OnConnectComplete::<Impl, OFFSET>, OnDisconnectComplete::<Impl, OFFSET>, OnConnectStateChange::<Impl, OFFSET>, OnVoiceCallStateChange::<Impl, OFFSET>)
    }
}
pub trait IMbnConnectionManagerImpl: Sized {
    fn GetConnection();
    fn GetConnections();
}
impl ::windows::core::RuntimeName for IMbnConnectionManager {
    const NAME: &'static str = "Windows.Win32.NetworkManagement.MobileBroadband.IMbnConnectionManager";
}
impl IMbnConnectionManagerVtbl {
    pub const fn new<Impl: IMbnConnectionManagerImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IMbnConnectionManagerVtbl {
        unsafe extern "system" fn GetConnection<Impl: IMbnConnectionManagerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, connectionid: super::super::Foundation::PWSTR, mbnconnection: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetConnection(&*(&connectionid as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&mbnconnection)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetConnections<Impl: IMbnConnectionManagerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, mbnconnections: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetConnections(::core::mem::transmute_copy(&mbnconnections)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IMbnConnectionManager>, base.5, GetConnection::<Impl, OFFSET>, GetConnections::<Impl, OFFSET>)
    }
}
pub trait IMbnConnectionManagerEventsImpl: Sized {
    fn OnConnectionArrival();
    fn OnConnectionRemoval();
}
impl ::windows::core::RuntimeName for IMbnConnectionManagerEvents {
    const NAME: &'static str = "Windows.Win32.NetworkManagement.MobileBroadband.IMbnConnectionManagerEvents";
}
impl IMbnConnectionManagerEventsVtbl {
    pub const fn new<Impl: IMbnConnectionManagerEventsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IMbnConnectionManagerEventsVtbl {
        unsafe extern "system" fn OnConnectionArrival<Impl: IMbnConnectionManagerEventsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, newconnection: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).OnConnectionArrival(&*(&newconnection as *const <IMbnConnection as ::windows::core::Abi>::Abi as *const <IMbnConnection as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OnConnectionRemoval<Impl: IMbnConnectionManagerEventsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, oldconnection: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).OnConnectionRemoval(&*(&oldconnection as *const <IMbnConnection as ::windows::core::Abi>::Abi as *const <IMbnConnection as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IMbnConnectionManagerEvents>, base.5, OnConnectionArrival::<Impl, OFFSET>, OnConnectionRemoval::<Impl, OFFSET>)
    }
}
pub trait IMbnConnectionProfileImpl: Sized {
    fn GetProfileXmlData();
    fn UpdateProfile();
    fn Delete();
}
impl ::windows::core::RuntimeName for IMbnConnectionProfile {
    const NAME: &'static str = "Windows.Win32.NetworkManagement.MobileBroadband.IMbnConnectionProfile";
}
impl IMbnConnectionProfileVtbl {
    pub const fn new<Impl: IMbnConnectionProfileImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IMbnConnectionProfileVtbl {
        unsafe extern "system" fn GetProfileXmlData<Impl: IMbnConnectionProfileImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, profiledata: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetProfileXmlData(::core::mem::transmute_copy(&profiledata)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UpdateProfile<Impl: IMbnConnectionProfileImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, strprofile: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).UpdateProfile(&*(&strprofile as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Delete<Impl: IMbnConnectionProfileImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Delete() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IMbnConnectionProfile>, base.5, GetProfileXmlData::<Impl, OFFSET>, UpdateProfile::<Impl, OFFSET>, Delete::<Impl, OFFSET>)
    }
}
pub trait IMbnConnectionProfileEventsImpl: Sized {
    fn OnProfileUpdate();
}
impl ::windows::core::RuntimeName for IMbnConnectionProfileEvents {
    const NAME: &'static str = "Windows.Win32.NetworkManagement.MobileBroadband.IMbnConnectionProfileEvents";
}
impl IMbnConnectionProfileEventsVtbl {
    pub const fn new<Impl: IMbnConnectionProfileEventsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IMbnConnectionProfileEventsVtbl {
        unsafe extern "system" fn OnProfileUpdate<Impl: IMbnConnectionProfileEventsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, newprofile: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).OnProfileUpdate(&*(&newprofile as *const <IMbnConnectionProfile as ::windows::core::Abi>::Abi as *const <IMbnConnectionProfile as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IMbnConnectionProfileEvents>, base.5, OnProfileUpdate::<Impl, OFFSET>)
    }
}
pub trait IMbnConnectionProfileManagerImpl: Sized {
    fn GetConnectionProfiles();
    fn GetConnectionProfile();
    fn CreateConnectionProfile();
}
impl ::windows::core::RuntimeName for IMbnConnectionProfileManager {
    const NAME: &'static str = "Windows.Win32.NetworkManagement.MobileBroadband.IMbnConnectionProfileManager";
}
impl IMbnConnectionProfileManagerVtbl {
    pub const fn new<Impl: IMbnConnectionProfileManagerImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IMbnConnectionProfileManagerVtbl {
        unsafe extern "system" fn GetConnectionProfiles<Impl: IMbnConnectionProfileManagerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, mbninterface: ::windows::core::RawPtr, connectionprofiles: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetConnectionProfiles(&*(&mbninterface as *const <IMbnInterface as ::windows::core::Abi>::Abi as *const <IMbnInterface as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&connectionprofiles)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetConnectionProfile<Impl: IMbnConnectionProfileManagerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, mbninterface: ::windows::core::RawPtr, profilename: super::super::Foundation::PWSTR, connectionprofile: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetConnectionProfile(&*(&mbninterface as *const <IMbnInterface as ::windows::core::Abi>::Abi as *const <IMbnInterface as ::windows::core::DefaultType>::DefaultType), &*(&profilename as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&connectionprofile)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateConnectionProfile<Impl: IMbnConnectionProfileManagerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, xmlprofile: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateConnectionProfile(&*(&xmlprofile as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IMbnConnectionProfileManager>, base.5, GetConnectionProfiles::<Impl, OFFSET>, GetConnectionProfile::<Impl, OFFSET>, CreateConnectionProfile::<Impl, OFFSET>)
    }
}
pub trait IMbnConnectionProfileManagerEventsImpl: Sized {
    fn OnConnectionProfileArrival();
    fn OnConnectionProfileRemoval();
}
impl ::windows::core::RuntimeName for IMbnConnectionProfileManagerEvents {
    const NAME: &'static str = "Windows.Win32.NetworkManagement.MobileBroadband.IMbnConnectionProfileManagerEvents";
}
impl IMbnConnectionProfileManagerEventsVtbl {
    pub const fn new<Impl: IMbnConnectionProfileManagerEventsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IMbnConnectionProfileManagerEventsVtbl {
        unsafe extern "system" fn OnConnectionProfileArrival<Impl: IMbnConnectionProfileManagerEventsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, newconnectionprofile: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).OnConnectionProfileArrival(&*(&newconnectionprofile as *const <IMbnConnectionProfile as ::windows::core::Abi>::Abi as *const <IMbnConnectionProfile as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OnConnectionProfileRemoval<Impl: IMbnConnectionProfileManagerEventsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, oldconnectionprofile: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).OnConnectionProfileRemoval(&*(&oldconnectionprofile as *const <IMbnConnectionProfile as ::windows::core::Abi>::Abi as *const <IMbnConnectionProfile as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IMbnConnectionProfileManagerEvents>, base.5, OnConnectionProfileArrival::<Impl, OFFSET>, OnConnectionProfileRemoval::<Impl, OFFSET>)
    }
}
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
impl ::windows::core::RuntimeName for IMbnDeviceService {
    const NAME: &'static str = "Windows.Win32.NetworkManagement.MobileBroadband.IMbnDeviceService";
}
impl IMbnDeviceServiceVtbl {
    pub const fn new<Impl: IMbnDeviceServiceImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IMbnDeviceServiceVtbl {
        unsafe extern "system" fn QuerySupportedCommands<Impl: IMbnDeviceServiceImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, requestid: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).QuerySupportedCommands(::core::mem::transmute_copy(&requestid)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OpenCommandSession<Impl: IMbnDeviceServiceImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, requestid: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).OpenCommandSession(::core::mem::transmute_copy(&requestid)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CloseCommandSession<Impl: IMbnDeviceServiceImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, requestid: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CloseCommandSession(::core::mem::transmute_copy(&requestid)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCommand<Impl: IMbnDeviceServiceImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, commandid: u32, deviceservicedata: *const super::super::System::Com::SAFEARRAY, requestid: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetCommand(commandid, &*(&deviceservicedata as *const <super::super::System::Com::SAFEARRAY as ::windows::core::Abi>::Abi as *const <super::super::System::Com::SAFEARRAY as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&requestid)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn QueryCommand<Impl: IMbnDeviceServiceImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, commandid: u32, deviceservicedata: *const super::super::System::Com::SAFEARRAY, requestid: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).QueryCommand(commandid, &*(&deviceservicedata as *const <super::super::System::Com::SAFEARRAY as ::windows::core::Abi>::Abi as *const <super::super::System::Com::SAFEARRAY as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&requestid)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OpenDataSession<Impl: IMbnDeviceServiceImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, requestid: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).OpenDataSession(::core::mem::transmute_copy(&requestid)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CloseDataSession<Impl: IMbnDeviceServiceImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, requestid: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CloseDataSession(::core::mem::transmute_copy(&requestid)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn WriteData<Impl: IMbnDeviceServiceImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, deviceservicedata: *const super::super::System::Com::SAFEARRAY, requestid: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).WriteData(&*(&deviceservicedata as *const <super::super::System::Com::SAFEARRAY as ::windows::core::Abi>::Abi as *const <super::super::System::Com::SAFEARRAY as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&requestid)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn InterfaceID<Impl: IMbnDeviceServiceImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, interfaceid: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).InterfaceID(::core::mem::transmute_copy(&interfaceid)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DeviceServiceID<Impl: IMbnDeviceServiceImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, deviceserviceid: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).DeviceServiceID(::core::mem::transmute_copy(&deviceserviceid)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsCommandSessionOpen<Impl: IMbnDeviceServiceImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).IsCommandSessionOpen(::core::mem::transmute_copy(&value)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsDataSessionOpen<Impl: IMbnDeviceServiceImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).IsDataSessionOpen(::core::mem::transmute_copy(&value)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(
            base.0,
            base.1,
            base.2,
            base.3,
            ::windows::core::GetRuntimeClassName::<IMbnDeviceService>,
            base.5,
            QuerySupportedCommands::<Impl, OFFSET>,
            OpenCommandSession::<Impl, OFFSET>,
            CloseCommandSession::<Impl, OFFSET>,
            SetCommand::<Impl, OFFSET>,
            QueryCommand::<Impl, OFFSET>,
            OpenDataSession::<Impl, OFFSET>,
            CloseDataSession::<Impl, OFFSET>,
            WriteData::<Impl, OFFSET>,
            InterfaceID::<Impl, OFFSET>,
            DeviceServiceID::<Impl, OFFSET>,
            IsCommandSessionOpen::<Impl, OFFSET>,
            IsDataSessionOpen::<Impl, OFFSET>,
        )
    }
}
pub trait IMbnDeviceServiceStateEventsImpl: Sized {
    fn OnSessionsStateChange();
}
impl ::windows::core::RuntimeName for IMbnDeviceServiceStateEvents {
    const NAME: &'static str = "Windows.Win32.NetworkManagement.MobileBroadband.IMbnDeviceServiceStateEvents";
}
impl IMbnDeviceServiceStateEventsVtbl {
    pub const fn new<Impl: IMbnDeviceServiceStateEventsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IMbnDeviceServiceStateEventsVtbl {
        unsafe extern "system" fn OnSessionsStateChange<Impl: IMbnDeviceServiceStateEventsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, interfaceid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, statechange: MBN_DEVICE_SERVICE_SESSIONS_STATE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).OnSessionsStateChange(&*(&interfaceid as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), statechange) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IMbnDeviceServiceStateEvents>, base.5, OnSessionsStateChange::<Impl, OFFSET>)
    }
}
pub trait IMbnDeviceServicesContextImpl: Sized {
    fn EnumerateDeviceServices();
    fn GetDeviceService();
    fn MaxCommandSize();
    fn MaxDataSize();
}
impl ::windows::core::RuntimeName for IMbnDeviceServicesContext {
    const NAME: &'static str = "Windows.Win32.NetworkManagement.MobileBroadband.IMbnDeviceServicesContext";
}
impl IMbnDeviceServicesContextVtbl {
    pub const fn new<Impl: IMbnDeviceServicesContextImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IMbnDeviceServicesContextVtbl {
        unsafe extern "system" fn EnumerateDeviceServices<Impl: IMbnDeviceServicesContextImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, deviceservices: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).EnumerateDeviceServices(::core::mem::transmute_copy(&deviceservices)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDeviceService<Impl: IMbnDeviceServicesContextImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, deviceserviceid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, mbndeviceservice: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetDeviceService(&*(&deviceserviceid as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&mbndeviceservice)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MaxCommandSize<Impl: IMbnDeviceServicesContextImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, maxcommandsize: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).MaxCommandSize(::core::mem::transmute_copy(&maxcommandsize)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MaxDataSize<Impl: IMbnDeviceServicesContextImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, maxdatasize: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).MaxDataSize(::core::mem::transmute_copy(&maxdatasize)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IMbnDeviceServicesContext>, base.5, EnumerateDeviceServices::<Impl, OFFSET>, GetDeviceService::<Impl, OFFSET>, MaxCommandSize::<Impl, OFFSET>, MaxDataSize::<Impl, OFFSET>)
    }
}
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
impl ::windows::core::RuntimeName for IMbnDeviceServicesEvents {
    const NAME: &'static str = "Windows.Win32.NetworkManagement.MobileBroadband.IMbnDeviceServicesEvents";
}
impl IMbnDeviceServicesEventsVtbl {
    pub const fn new<Impl: IMbnDeviceServicesEventsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IMbnDeviceServicesEventsVtbl {
        unsafe extern "system" fn OnQuerySupportedCommandsComplete<Impl: IMbnDeviceServicesEventsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, deviceservice: ::windows::core::RawPtr, commandidlist: *const super::super::System::Com::SAFEARRAY, status: ::windows::core::HRESULT, requestid: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).OnQuerySupportedCommandsComplete(&*(&deviceservice as *const <IMbnDeviceService as ::windows::core::Abi>::Abi as *const <IMbnDeviceService as ::windows::core::DefaultType>::DefaultType), &*(&commandidlist as *const <super::super::System::Com::SAFEARRAY as ::windows::core::Abi>::Abi as *const <super::super::System::Com::SAFEARRAY as ::windows::core::DefaultType>::DefaultType), status, requestid) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OnOpenCommandSessionComplete<Impl: IMbnDeviceServicesEventsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, deviceservice: ::windows::core::RawPtr, status: ::windows::core::HRESULT, requestid: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).OnOpenCommandSessionComplete(&*(&deviceservice as *const <IMbnDeviceService as ::windows::core::Abi>::Abi as *const <IMbnDeviceService as ::windows::core::DefaultType>::DefaultType), status, requestid) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OnCloseCommandSessionComplete<Impl: IMbnDeviceServicesEventsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, deviceservice: ::windows::core::RawPtr, status: ::windows::core::HRESULT, requestid: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).OnCloseCommandSessionComplete(&*(&deviceservice as *const <IMbnDeviceService as ::windows::core::Abi>::Abi as *const <IMbnDeviceService as ::windows::core::DefaultType>::DefaultType), status, requestid) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OnSetCommandComplete<Impl: IMbnDeviceServicesEventsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, deviceservice: ::windows::core::RawPtr, responseid: u32, deviceservicedata: *const super::super::System::Com::SAFEARRAY, status: ::windows::core::HRESULT, requestid: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).OnSetCommandComplete(&*(&deviceservice as *const <IMbnDeviceService as ::windows::core::Abi>::Abi as *const <IMbnDeviceService as ::windows::core::DefaultType>::DefaultType), responseid, &*(&deviceservicedata as *const <super::super::System::Com::SAFEARRAY as ::windows::core::Abi>::Abi as *const <super::super::System::Com::SAFEARRAY as ::windows::core::DefaultType>::DefaultType), status, requestid) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OnQueryCommandComplete<Impl: IMbnDeviceServicesEventsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, deviceservice: ::windows::core::RawPtr, responseid: u32, deviceservicedata: *const super::super::System::Com::SAFEARRAY, status: ::windows::core::HRESULT, requestid: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).OnQueryCommandComplete(&*(&deviceservice as *const <IMbnDeviceService as ::windows::core::Abi>::Abi as *const <IMbnDeviceService as ::windows::core::DefaultType>::DefaultType), responseid, &*(&deviceservicedata as *const <super::super::System::Com::SAFEARRAY as ::windows::core::Abi>::Abi as *const <super::super::System::Com::SAFEARRAY as ::windows::core::DefaultType>::DefaultType), status, requestid) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OnEventNotification<Impl: IMbnDeviceServicesEventsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, deviceservice: ::windows::core::RawPtr, eventid: u32, deviceservicedata: *const super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).OnEventNotification(&*(&deviceservice as *const <IMbnDeviceService as ::windows::core::Abi>::Abi as *const <IMbnDeviceService as ::windows::core::DefaultType>::DefaultType), eventid, &*(&deviceservicedata as *const <super::super::System::Com::SAFEARRAY as ::windows::core::Abi>::Abi as *const <super::super::System::Com::SAFEARRAY as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OnOpenDataSessionComplete<Impl: IMbnDeviceServicesEventsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, deviceservice: ::windows::core::RawPtr, status: ::windows::core::HRESULT, requestid: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).OnOpenDataSessionComplete(&*(&deviceservice as *const <IMbnDeviceService as ::windows::core::Abi>::Abi as *const <IMbnDeviceService as ::windows::core::DefaultType>::DefaultType), status, requestid) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OnCloseDataSessionComplete<Impl: IMbnDeviceServicesEventsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, deviceservice: ::windows::core::RawPtr, status: ::windows::core::HRESULT, requestid: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).OnCloseDataSessionComplete(&*(&deviceservice as *const <IMbnDeviceService as ::windows::core::Abi>::Abi as *const <IMbnDeviceService as ::windows::core::DefaultType>::DefaultType), status, requestid) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OnWriteDataComplete<Impl: IMbnDeviceServicesEventsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, deviceservice: ::windows::core::RawPtr, status: ::windows::core::HRESULT, requestid: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).OnWriteDataComplete(&*(&deviceservice as *const <IMbnDeviceService as ::windows::core::Abi>::Abi as *const <IMbnDeviceService as ::windows::core::DefaultType>::DefaultType), status, requestid) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OnReadData<Impl: IMbnDeviceServicesEventsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, deviceservice: ::windows::core::RawPtr, deviceservicedata: *const super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).OnReadData(&*(&deviceservice as *const <IMbnDeviceService as ::windows::core::Abi>::Abi as *const <IMbnDeviceService as ::windows::core::DefaultType>::DefaultType), &*(&deviceservicedata as *const <super::super::System::Com::SAFEARRAY as ::windows::core::Abi>::Abi as *const <super::super::System::Com::SAFEARRAY as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OnInterfaceStateChange<Impl: IMbnDeviceServicesEventsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, interfaceid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, statechange: MBN_DEVICE_SERVICES_INTERFACE_STATE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).OnInterfaceStateChange(&*(&interfaceid as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), statechange) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(
            base.0,
            base.1,
            base.2,
            base.3,
            ::windows::core::GetRuntimeClassName::<IMbnDeviceServicesEvents>,
            base.5,
            OnQuerySupportedCommandsComplete::<Impl, OFFSET>,
            OnOpenCommandSessionComplete::<Impl, OFFSET>,
            OnCloseCommandSessionComplete::<Impl, OFFSET>,
            OnSetCommandComplete::<Impl, OFFSET>,
            OnQueryCommandComplete::<Impl, OFFSET>,
            OnEventNotification::<Impl, OFFSET>,
            OnOpenDataSessionComplete::<Impl, OFFSET>,
            OnCloseDataSessionComplete::<Impl, OFFSET>,
            OnWriteDataComplete::<Impl, OFFSET>,
            OnReadData::<Impl, OFFSET>,
            OnInterfaceStateChange::<Impl, OFFSET>,
        )
    }
}
pub trait IMbnDeviceServicesManagerImpl: Sized {
    fn GetDeviceServicesContext();
}
impl ::windows::core::RuntimeName for IMbnDeviceServicesManager {
    const NAME: &'static str = "Windows.Win32.NetworkManagement.MobileBroadband.IMbnDeviceServicesManager";
}
impl IMbnDeviceServicesManagerVtbl {
    pub const fn new<Impl: IMbnDeviceServicesManagerImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IMbnDeviceServicesManagerVtbl {
        unsafe extern "system" fn GetDeviceServicesContext<Impl: IMbnDeviceServicesManagerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, networkinterfaceid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, mbndevicescontext: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetDeviceServicesContext(&*(&networkinterfaceid as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&mbndevicescontext)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IMbnDeviceServicesManager>, base.5, GetDeviceServicesContext::<Impl, OFFSET>)
    }
}
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
impl ::windows::core::RuntimeName for IMbnInterface {
    const NAME: &'static str = "Windows.Win32.NetworkManagement.MobileBroadband.IMbnInterface";
}
impl IMbnInterfaceVtbl {
    pub const fn new<Impl: IMbnInterfaceImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IMbnInterfaceVtbl {
        unsafe extern "system" fn InterfaceID<Impl: IMbnInterfaceImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, interfaceid: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).InterfaceID(::core::mem::transmute_copy(&interfaceid)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetInterfaceCapability<Impl: IMbnInterfaceImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, interfacecaps: *mut MBN_INTERFACE_CAPS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetInterfaceCapability(::core::mem::transmute_copy(&interfacecaps)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSubscriberInformation<Impl: IMbnInterfaceImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, subscriberinformation: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetSubscriberInformation(::core::mem::transmute_copy(&subscriberinformation)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetReadyState<Impl: IMbnInterfaceImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, readystate: *mut MBN_READY_STATE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetReadyState(::core::mem::transmute_copy(&readystate)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn InEmergencyMode<Impl: IMbnInterfaceImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, emergencymode: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).InEmergencyMode(::core::mem::transmute_copy(&emergencymode)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetHomeProvider<Impl: IMbnInterfaceImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, homeprovider: *mut MBN_PROVIDER) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetHomeProvider(::core::mem::transmute_copy(&homeprovider)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPreferredProviders<Impl: IMbnInterfaceImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, preferredproviders: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetPreferredProviders(::core::mem::transmute_copy(&preferredproviders)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPreferredProviders<Impl: IMbnInterfaceImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, preferredproviders: *const super::super::System::Com::SAFEARRAY, requestid: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetPreferredProviders(&*(&preferredproviders as *const <super::super::System::Com::SAFEARRAY as ::windows::core::Abi>::Abi as *const <super::super::System::Com::SAFEARRAY as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&requestid)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetVisibleProviders<Impl: IMbnInterfaceImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, age: *mut u32, visibleproviders: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetVisibleProviders(::core::mem::transmute_copy(&age), ::core::mem::transmute_copy(&visibleproviders)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ScanNetwork<Impl: IMbnInterfaceImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, requestid: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ScanNetwork(::core::mem::transmute_copy(&requestid)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetConnection<Impl: IMbnInterfaceImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, mbnconnection: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetConnection(::core::mem::transmute_copy(&mbnconnection)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IMbnInterface>, base.5, InterfaceID::<Impl, OFFSET>, GetInterfaceCapability::<Impl, OFFSET>, GetSubscriberInformation::<Impl, OFFSET>, GetReadyState::<Impl, OFFSET>, InEmergencyMode::<Impl, OFFSET>, GetHomeProvider::<Impl, OFFSET>, GetPreferredProviders::<Impl, OFFSET>, SetPreferredProviders::<Impl, OFFSET>, GetVisibleProviders::<Impl, OFFSET>, ScanNetwork::<Impl, OFFSET>, GetConnection::<Impl, OFFSET>)
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
impl ::windows::core::RuntimeName for IMbnInterfaceEvents {
    const NAME: &'static str = "Windows.Win32.NetworkManagement.MobileBroadband.IMbnInterfaceEvents";
}
impl IMbnInterfaceEventsVtbl {
    pub const fn new<Impl: IMbnInterfaceEventsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IMbnInterfaceEventsVtbl {
        unsafe extern "system" fn OnInterfaceCapabilityAvailable<Impl: IMbnInterfaceEventsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, newinterface: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).OnInterfaceCapabilityAvailable(&*(&newinterface as *const <IMbnInterface as ::windows::core::Abi>::Abi as *const <IMbnInterface as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OnSubscriberInformationChange<Impl: IMbnInterfaceEventsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, newinterface: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).OnSubscriberInformationChange(&*(&newinterface as *const <IMbnInterface as ::windows::core::Abi>::Abi as *const <IMbnInterface as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OnReadyStateChange<Impl: IMbnInterfaceEventsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, newinterface: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).OnReadyStateChange(&*(&newinterface as *const <IMbnInterface as ::windows::core::Abi>::Abi as *const <IMbnInterface as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OnEmergencyModeChange<Impl: IMbnInterfaceEventsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, newinterface: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).OnEmergencyModeChange(&*(&newinterface as *const <IMbnInterface as ::windows::core::Abi>::Abi as *const <IMbnInterface as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OnHomeProviderAvailable<Impl: IMbnInterfaceEventsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, newinterface: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).OnHomeProviderAvailable(&*(&newinterface as *const <IMbnInterface as ::windows::core::Abi>::Abi as *const <IMbnInterface as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OnPreferredProvidersChange<Impl: IMbnInterfaceEventsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, newinterface: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).OnPreferredProvidersChange(&*(&newinterface as *const <IMbnInterface as ::windows::core::Abi>::Abi as *const <IMbnInterface as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OnSetPreferredProvidersComplete<Impl: IMbnInterfaceEventsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, newinterface: ::windows::core::RawPtr, requestid: u32, status: ::windows::core::HRESULT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).OnSetPreferredProvidersComplete(&*(&newinterface as *const <IMbnInterface as ::windows::core::Abi>::Abi as *const <IMbnInterface as ::windows::core::DefaultType>::DefaultType), requestid, status) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OnScanNetworkComplete<Impl: IMbnInterfaceEventsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, newinterface: ::windows::core::RawPtr, requestid: u32, status: ::windows::core::HRESULT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).OnScanNetworkComplete(&*(&newinterface as *const <IMbnInterface as ::windows::core::Abi>::Abi as *const <IMbnInterface as ::windows::core::DefaultType>::DefaultType), requestid, status) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IMbnInterfaceEvents>, base.5, OnInterfaceCapabilityAvailable::<Impl, OFFSET>, OnSubscriberInformationChange::<Impl, OFFSET>, OnReadyStateChange::<Impl, OFFSET>, OnEmergencyModeChange::<Impl, OFFSET>, OnHomeProviderAvailable::<Impl, OFFSET>, OnPreferredProvidersChange::<Impl, OFFSET>, OnSetPreferredProvidersComplete::<Impl, OFFSET>, OnScanNetworkComplete::<Impl, OFFSET>)
    }
}
pub trait IMbnInterfaceManagerImpl: Sized {
    fn GetInterface();
    fn GetInterfaces();
}
impl ::windows::core::RuntimeName for IMbnInterfaceManager {
    const NAME: &'static str = "Windows.Win32.NetworkManagement.MobileBroadband.IMbnInterfaceManager";
}
impl IMbnInterfaceManagerVtbl {
    pub const fn new<Impl: IMbnInterfaceManagerImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IMbnInterfaceManagerVtbl {
        unsafe extern "system" fn GetInterface<Impl: IMbnInterfaceManagerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, interfaceid: super::super::Foundation::PWSTR, mbninterface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetInterface(&*(&interfaceid as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&mbninterface)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetInterfaces<Impl: IMbnInterfaceManagerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, mbninterfaces: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetInterfaces(::core::mem::transmute_copy(&mbninterfaces)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IMbnInterfaceManager>, base.5, GetInterface::<Impl, OFFSET>, GetInterfaces::<Impl, OFFSET>)
    }
}
pub trait IMbnInterfaceManagerEventsImpl: Sized {
    fn OnInterfaceArrival();
    fn OnInterfaceRemoval();
}
impl ::windows::core::RuntimeName for IMbnInterfaceManagerEvents {
    const NAME: &'static str = "Windows.Win32.NetworkManagement.MobileBroadband.IMbnInterfaceManagerEvents";
}
impl IMbnInterfaceManagerEventsVtbl {
    pub const fn new<Impl: IMbnInterfaceManagerEventsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IMbnInterfaceManagerEventsVtbl {
        unsafe extern "system" fn OnInterfaceArrival<Impl: IMbnInterfaceManagerEventsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, newinterface: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).OnInterfaceArrival(&*(&newinterface as *const <IMbnInterface as ::windows::core::Abi>::Abi as *const <IMbnInterface as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OnInterfaceRemoval<Impl: IMbnInterfaceManagerEventsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, oldinterface: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).OnInterfaceRemoval(&*(&oldinterface as *const <IMbnInterface as ::windows::core::Abi>::Abi as *const <IMbnInterface as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IMbnInterfaceManagerEvents>, base.5, OnInterfaceArrival::<Impl, OFFSET>, OnInterfaceRemoval::<Impl, OFFSET>)
    }
}
pub trait IMbnMultiCarrierImpl: Sized {
    fn SetHomeProvider();
    fn GetPreferredProviders();
    fn GetVisibleProviders();
    fn GetSupportedCellularClasses();
    fn GetCurrentCellularClass();
    fn ScanNetwork();
}
impl ::windows::core::RuntimeName for IMbnMultiCarrier {
    const NAME: &'static str = "Windows.Win32.NetworkManagement.MobileBroadband.IMbnMultiCarrier";
}
impl IMbnMultiCarrierVtbl {
    pub const fn new<Impl: IMbnMultiCarrierImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IMbnMultiCarrierVtbl {
        unsafe extern "system" fn SetHomeProvider<Impl: IMbnMultiCarrierImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, homeprovider: *const MBN_PROVIDER2, requestid: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetHomeProvider(&*(&homeprovider as *const <MBN_PROVIDER2 as ::windows::core::Abi>::Abi as *const <MBN_PROVIDER2 as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&requestid)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPreferredProviders<Impl: IMbnMultiCarrierImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, preferredmulticarrierproviders: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetPreferredProviders(::core::mem::transmute_copy(&preferredmulticarrierproviders)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetVisibleProviders<Impl: IMbnMultiCarrierImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, age: *mut u32, visibleproviders: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetVisibleProviders(::core::mem::transmute_copy(&age), ::core::mem::transmute_copy(&visibleproviders)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSupportedCellularClasses<Impl: IMbnMultiCarrierImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, cellularclasses: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetSupportedCellularClasses(::core::mem::transmute_copy(&cellularclasses)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCurrentCellularClass<Impl: IMbnMultiCarrierImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, currentcellularclass: *mut MBN_CELLULAR_CLASS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetCurrentCellularClass(::core::mem::transmute_copy(&currentcellularclass)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ScanNetwork<Impl: IMbnMultiCarrierImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, requestid: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ScanNetwork(::core::mem::transmute_copy(&requestid)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IMbnMultiCarrier>, base.5, SetHomeProvider::<Impl, OFFSET>, GetPreferredProviders::<Impl, OFFSET>, GetVisibleProviders::<Impl, OFFSET>, GetSupportedCellularClasses::<Impl, OFFSET>, GetCurrentCellularClass::<Impl, OFFSET>, ScanNetwork::<Impl, OFFSET>)
    }
}
pub trait IMbnMultiCarrierEventsImpl: Sized {
    fn OnSetHomeProviderComplete();
    fn OnCurrentCellularClassChange();
    fn OnPreferredProvidersChange();
    fn OnScanNetworkComplete();
    fn OnInterfaceCapabilityChange();
}
impl ::windows::core::RuntimeName for IMbnMultiCarrierEvents {
    const NAME: &'static str = "Windows.Win32.NetworkManagement.MobileBroadband.IMbnMultiCarrierEvents";
}
impl IMbnMultiCarrierEventsVtbl {
    pub const fn new<Impl: IMbnMultiCarrierEventsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IMbnMultiCarrierEventsVtbl {
        unsafe extern "system" fn OnSetHomeProviderComplete<Impl: IMbnMultiCarrierEventsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, mbninterface: ::windows::core::RawPtr, requestid: u32, status: ::windows::core::HRESULT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).OnSetHomeProviderComplete(&*(&mbninterface as *const <IMbnMultiCarrier as ::windows::core::Abi>::Abi as *const <IMbnMultiCarrier as ::windows::core::DefaultType>::DefaultType), requestid, status) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OnCurrentCellularClassChange<Impl: IMbnMultiCarrierEventsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, mbninterface: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).OnCurrentCellularClassChange(&*(&mbninterface as *const <IMbnMultiCarrier as ::windows::core::Abi>::Abi as *const <IMbnMultiCarrier as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OnPreferredProvidersChange<Impl: IMbnMultiCarrierEventsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, mbninterface: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).OnPreferredProvidersChange(&*(&mbninterface as *const <IMbnMultiCarrier as ::windows::core::Abi>::Abi as *const <IMbnMultiCarrier as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OnScanNetworkComplete<Impl: IMbnMultiCarrierEventsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, mbninterface: ::windows::core::RawPtr, requestid: u32, status: ::windows::core::HRESULT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).OnScanNetworkComplete(&*(&mbninterface as *const <IMbnMultiCarrier as ::windows::core::Abi>::Abi as *const <IMbnMultiCarrier as ::windows::core::DefaultType>::DefaultType), requestid, status) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OnInterfaceCapabilityChange<Impl: IMbnMultiCarrierEventsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, mbninterface: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).OnInterfaceCapabilityChange(&*(&mbninterface as *const <IMbnMultiCarrier as ::windows::core::Abi>::Abi as *const <IMbnMultiCarrier as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IMbnMultiCarrierEvents>, base.5, OnSetHomeProviderComplete::<Impl, OFFSET>, OnCurrentCellularClassChange::<Impl, OFFSET>, OnPreferredProvidersChange::<Impl, OFFSET>, OnScanNetworkComplete::<Impl, OFFSET>, OnInterfaceCapabilityChange::<Impl, OFFSET>)
    }
}
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
impl ::windows::core::RuntimeName for IMbnPin {
    const NAME: &'static str = "Windows.Win32.NetworkManagement.MobileBroadband.IMbnPin";
}
impl IMbnPinVtbl {
    pub const fn new<Impl: IMbnPinImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IMbnPinVtbl {
        unsafe extern "system" fn PinType<Impl: IMbnPinImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pintype: *mut MBN_PIN_TYPE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).PinType(::core::mem::transmute_copy(&pintype)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PinFormat<Impl: IMbnPinImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pinformat: *mut MBN_PIN_FORMAT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).PinFormat(::core::mem::transmute_copy(&pinformat)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PinLengthMin<Impl: IMbnPinImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pinlengthmin: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).PinLengthMin(::core::mem::transmute_copy(&pinlengthmin)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PinLengthMax<Impl: IMbnPinImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pinlengthmax: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).PinLengthMax(::core::mem::transmute_copy(&pinlengthmax)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PinMode<Impl: IMbnPinImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pinmode: *mut MBN_PIN_MODE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).PinMode(::core::mem::transmute_copy(&pinmode)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Enable<Impl: IMbnPinImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pin: super::super::Foundation::PWSTR, requestid: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Enable(&*(&pin as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&requestid)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Disable<Impl: IMbnPinImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pin: super::super::Foundation::PWSTR, requestid: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Disable(&*(&pin as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&requestid)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Enter<Impl: IMbnPinImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pin: super::super::Foundation::PWSTR, requestid: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Enter(&*(&pin as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&requestid)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Change<Impl: IMbnPinImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pin: super::super::Foundation::PWSTR, newpin: super::super::Foundation::PWSTR, requestid: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Change(&*(&pin as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), &*(&newpin as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&requestid)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Unblock<Impl: IMbnPinImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, puk: super::super::Foundation::PWSTR, newpin: super::super::Foundation::PWSTR, requestid: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Unblock(&*(&puk as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), &*(&newpin as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&requestid)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPinManager<Impl: IMbnPinImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pinmanager: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetPinManager(::core::mem::transmute_copy(&pinmanager)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IMbnPin>, base.5, PinType::<Impl, OFFSET>, PinFormat::<Impl, OFFSET>, PinLengthMin::<Impl, OFFSET>, PinLengthMax::<Impl, OFFSET>, PinMode::<Impl, OFFSET>, Enable::<Impl, OFFSET>, Disable::<Impl, OFFSET>, Enter::<Impl, OFFSET>, Change::<Impl, OFFSET>, Unblock::<Impl, OFFSET>, GetPinManager::<Impl, OFFSET>)
    }
}
pub trait IMbnPinEventsImpl: Sized {
    fn OnEnableComplete();
    fn OnDisableComplete();
    fn OnEnterComplete();
    fn OnChangeComplete();
    fn OnUnblockComplete();
}
impl ::windows::core::RuntimeName for IMbnPinEvents {
    const NAME: &'static str = "Windows.Win32.NetworkManagement.MobileBroadband.IMbnPinEvents";
}
impl IMbnPinEventsVtbl {
    pub const fn new<Impl: IMbnPinEventsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IMbnPinEventsVtbl {
        unsafe extern "system" fn OnEnableComplete<Impl: IMbnPinEventsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pin: ::windows::core::RawPtr, pininfo: *const MBN_PIN_INFO, requestid: u32, status: ::windows::core::HRESULT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).OnEnableComplete(&*(&pin as *const <IMbnPin as ::windows::core::Abi>::Abi as *const <IMbnPin as ::windows::core::DefaultType>::DefaultType), &*(&pininfo as *const <MBN_PIN_INFO as ::windows::core::Abi>::Abi as *const <MBN_PIN_INFO as ::windows::core::DefaultType>::DefaultType), requestid, status) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OnDisableComplete<Impl: IMbnPinEventsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pin: ::windows::core::RawPtr, pininfo: *const MBN_PIN_INFO, requestid: u32, status: ::windows::core::HRESULT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).OnDisableComplete(&*(&pin as *const <IMbnPin as ::windows::core::Abi>::Abi as *const <IMbnPin as ::windows::core::DefaultType>::DefaultType), &*(&pininfo as *const <MBN_PIN_INFO as ::windows::core::Abi>::Abi as *const <MBN_PIN_INFO as ::windows::core::DefaultType>::DefaultType), requestid, status) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OnEnterComplete<Impl: IMbnPinEventsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pin: ::windows::core::RawPtr, pininfo: *const MBN_PIN_INFO, requestid: u32, status: ::windows::core::HRESULT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).OnEnterComplete(&*(&pin as *const <IMbnPin as ::windows::core::Abi>::Abi as *const <IMbnPin as ::windows::core::DefaultType>::DefaultType), &*(&pininfo as *const <MBN_PIN_INFO as ::windows::core::Abi>::Abi as *const <MBN_PIN_INFO as ::windows::core::DefaultType>::DefaultType), requestid, status) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OnChangeComplete<Impl: IMbnPinEventsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pin: ::windows::core::RawPtr, pininfo: *const MBN_PIN_INFO, requestid: u32, status: ::windows::core::HRESULT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).OnChangeComplete(&*(&pin as *const <IMbnPin as ::windows::core::Abi>::Abi as *const <IMbnPin as ::windows::core::DefaultType>::DefaultType), &*(&pininfo as *const <MBN_PIN_INFO as ::windows::core::Abi>::Abi as *const <MBN_PIN_INFO as ::windows::core::DefaultType>::DefaultType), requestid, status) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OnUnblockComplete<Impl: IMbnPinEventsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pin: ::windows::core::RawPtr, pininfo: *const MBN_PIN_INFO, requestid: u32, status: ::windows::core::HRESULT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).OnUnblockComplete(&*(&pin as *const <IMbnPin as ::windows::core::Abi>::Abi as *const <IMbnPin as ::windows::core::DefaultType>::DefaultType), &*(&pininfo as *const <MBN_PIN_INFO as ::windows::core::Abi>::Abi as *const <MBN_PIN_INFO as ::windows::core::DefaultType>::DefaultType), requestid, status) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IMbnPinEvents>, base.5, OnEnableComplete::<Impl, OFFSET>, OnDisableComplete::<Impl, OFFSET>, OnEnterComplete::<Impl, OFFSET>, OnChangeComplete::<Impl, OFFSET>, OnUnblockComplete::<Impl, OFFSET>)
    }
}
pub trait IMbnPinManagerImpl: Sized {
    fn GetPinList();
    fn GetPin();
    fn GetPinState();
}
impl ::windows::core::RuntimeName for IMbnPinManager {
    const NAME: &'static str = "Windows.Win32.NetworkManagement.MobileBroadband.IMbnPinManager";
}
impl IMbnPinManagerVtbl {
    pub const fn new<Impl: IMbnPinManagerImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IMbnPinManagerVtbl {
        unsafe extern "system" fn GetPinList<Impl: IMbnPinManagerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pinlist: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetPinList(::core::mem::transmute_copy(&pinlist)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPin<Impl: IMbnPinManagerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pintype: MBN_PIN_TYPE, pin: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetPin(pintype, ::core::mem::transmute_copy(&pin)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPinState<Impl: IMbnPinManagerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, requestid: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetPinState(::core::mem::transmute_copy(&requestid)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IMbnPinManager>, base.5, GetPinList::<Impl, OFFSET>, GetPin::<Impl, OFFSET>, GetPinState::<Impl, OFFSET>)
    }
}
pub trait IMbnPinManagerEventsImpl: Sized {
    fn OnPinListAvailable();
    fn OnGetPinStateComplete();
}
impl ::windows::core::RuntimeName for IMbnPinManagerEvents {
    const NAME: &'static str = "Windows.Win32.NetworkManagement.MobileBroadband.IMbnPinManagerEvents";
}
impl IMbnPinManagerEventsVtbl {
    pub const fn new<Impl: IMbnPinManagerEventsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IMbnPinManagerEventsVtbl {
        unsafe extern "system" fn OnPinListAvailable<Impl: IMbnPinManagerEventsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pinmanager: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).OnPinListAvailable(&*(&pinmanager as *const <IMbnPinManager as ::windows::core::Abi>::Abi as *const <IMbnPinManager as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OnGetPinStateComplete<Impl: IMbnPinManagerEventsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pinmanager: ::windows::core::RawPtr, pininfo: MBN_PIN_INFO, requestid: u32, status: ::windows::core::HRESULT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).OnGetPinStateComplete(&*(&pinmanager as *const <IMbnPinManager as ::windows::core::Abi>::Abi as *const <IMbnPinManager as ::windows::core::DefaultType>::DefaultType), &*(&pininfo as *const <MBN_PIN_INFO as ::windows::core::Abi>::Abi as *const <MBN_PIN_INFO as ::windows::core::DefaultType>::DefaultType), requestid, status) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IMbnPinManagerEvents>, base.5, OnPinListAvailable::<Impl, OFFSET>, OnGetPinStateComplete::<Impl, OFFSET>)
    }
}
pub trait IMbnRadioImpl: Sized {
    fn SoftwareRadioState();
    fn HardwareRadioState();
    fn SetSoftwareRadioState();
}
impl ::windows::core::RuntimeName for IMbnRadio {
    const NAME: &'static str = "Windows.Win32.NetworkManagement.MobileBroadband.IMbnRadio";
}
impl IMbnRadioVtbl {
    pub const fn new<Impl: IMbnRadioImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IMbnRadioVtbl {
        unsafe extern "system" fn SoftwareRadioState<Impl: IMbnRadioImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, softwareradiostate: *mut MBN_RADIO) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SoftwareRadioState(::core::mem::transmute_copy(&softwareradiostate)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn HardwareRadioState<Impl: IMbnRadioImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hardwareradiostate: *mut MBN_RADIO) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).HardwareRadioState(::core::mem::transmute_copy(&hardwareradiostate)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSoftwareRadioState<Impl: IMbnRadioImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, radiostate: MBN_RADIO, requestid: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetSoftwareRadioState(radiostate, ::core::mem::transmute_copy(&requestid)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IMbnRadio>, base.5, SoftwareRadioState::<Impl, OFFSET>, HardwareRadioState::<Impl, OFFSET>, SetSoftwareRadioState::<Impl, OFFSET>)
    }
}
pub trait IMbnRadioEventsImpl: Sized {
    fn OnRadioStateChange();
    fn OnSetSoftwareRadioStateComplete();
}
impl ::windows::core::RuntimeName for IMbnRadioEvents {
    const NAME: &'static str = "Windows.Win32.NetworkManagement.MobileBroadband.IMbnRadioEvents";
}
impl IMbnRadioEventsVtbl {
    pub const fn new<Impl: IMbnRadioEventsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IMbnRadioEventsVtbl {
        unsafe extern "system" fn OnRadioStateChange<Impl: IMbnRadioEventsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, newinterface: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).OnRadioStateChange(&*(&newinterface as *const <IMbnRadio as ::windows::core::Abi>::Abi as *const <IMbnRadio as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OnSetSoftwareRadioStateComplete<Impl: IMbnRadioEventsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, newinterface: ::windows::core::RawPtr, requestid: u32, status: ::windows::core::HRESULT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).OnSetSoftwareRadioStateComplete(&*(&newinterface as *const <IMbnRadio as ::windows::core::Abi>::Abi as *const <IMbnRadio as ::windows::core::DefaultType>::DefaultType), requestid, status) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IMbnRadioEvents>, base.5, OnRadioStateChange::<Impl, OFFSET>, OnSetSoftwareRadioStateComplete::<Impl, OFFSET>)
    }
}
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
impl ::windows::core::RuntimeName for IMbnRegistration {
    const NAME: &'static str = "Windows.Win32.NetworkManagement.MobileBroadband.IMbnRegistration";
}
impl IMbnRegistrationVtbl {
    pub const fn new<Impl: IMbnRegistrationImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IMbnRegistrationVtbl {
        unsafe extern "system" fn GetRegisterState<Impl: IMbnRegistrationImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, registerstate: *mut MBN_REGISTER_STATE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetRegisterState(::core::mem::transmute_copy(&registerstate)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetRegisterMode<Impl: IMbnRegistrationImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, registermode: *mut MBN_REGISTER_MODE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetRegisterMode(::core::mem::transmute_copy(&registermode)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetProviderID<Impl: IMbnRegistrationImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, providerid: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetProviderID(::core::mem::transmute_copy(&providerid)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetProviderName<Impl: IMbnRegistrationImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, providername: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetProviderName(::core::mem::transmute_copy(&providername)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetRoamingText<Impl: IMbnRegistrationImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, roamingtext: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetRoamingText(::core::mem::transmute_copy(&roamingtext)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetAvailableDataClasses<Impl: IMbnRegistrationImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, availabledataclasses: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetAvailableDataClasses(::core::mem::transmute_copy(&availabledataclasses)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCurrentDataClass<Impl: IMbnRegistrationImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, currentdataclass: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetCurrentDataClass(::core::mem::transmute_copy(&currentdataclass)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetRegistrationNetworkError<Impl: IMbnRegistrationImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, registrationnetworkerror: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetRegistrationNetworkError(::core::mem::transmute_copy(&registrationnetworkerror)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPacketAttachNetworkError<Impl: IMbnRegistrationImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, packetattachnetworkerror: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetPacketAttachNetworkError(::core::mem::transmute_copy(&packetattachnetworkerror)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetRegisterMode<Impl: IMbnRegistrationImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, registermode: MBN_REGISTER_MODE, providerid: super::super::Foundation::PWSTR, dataclass: u32, requestid: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetRegisterMode(registermode, &*(&providerid as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), dataclass, ::core::mem::transmute_copy(&requestid)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IMbnRegistration>, base.5, GetRegisterState::<Impl, OFFSET>, GetRegisterMode::<Impl, OFFSET>, GetProviderID::<Impl, OFFSET>, GetProviderName::<Impl, OFFSET>, GetRoamingText::<Impl, OFFSET>, GetAvailableDataClasses::<Impl, OFFSET>, GetCurrentDataClass::<Impl, OFFSET>, GetRegistrationNetworkError::<Impl, OFFSET>, GetPacketAttachNetworkError::<Impl, OFFSET>, SetRegisterMode::<Impl, OFFSET>)
    }
}
pub trait IMbnRegistrationEventsImpl: Sized {
    fn OnRegisterModeAvailable();
    fn OnRegisterStateChange();
    fn OnPacketServiceStateChange();
    fn OnSetRegisterModeComplete();
}
impl ::windows::core::RuntimeName for IMbnRegistrationEvents {
    const NAME: &'static str = "Windows.Win32.NetworkManagement.MobileBroadband.IMbnRegistrationEvents";
}
impl IMbnRegistrationEventsVtbl {
    pub const fn new<Impl: IMbnRegistrationEventsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IMbnRegistrationEventsVtbl {
        unsafe extern "system" fn OnRegisterModeAvailable<Impl: IMbnRegistrationEventsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, newinterface: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).OnRegisterModeAvailable(&*(&newinterface as *const <IMbnRegistration as ::windows::core::Abi>::Abi as *const <IMbnRegistration as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OnRegisterStateChange<Impl: IMbnRegistrationEventsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, newinterface: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).OnRegisterStateChange(&*(&newinterface as *const <IMbnRegistration as ::windows::core::Abi>::Abi as *const <IMbnRegistration as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OnPacketServiceStateChange<Impl: IMbnRegistrationEventsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, newinterface: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).OnPacketServiceStateChange(&*(&newinterface as *const <IMbnRegistration as ::windows::core::Abi>::Abi as *const <IMbnRegistration as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OnSetRegisterModeComplete<Impl: IMbnRegistrationEventsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, newinterface: ::windows::core::RawPtr, requestid: u32, status: ::windows::core::HRESULT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).OnSetRegisterModeComplete(&*(&newinterface as *const <IMbnRegistration as ::windows::core::Abi>::Abi as *const <IMbnRegistration as ::windows::core::DefaultType>::DefaultType), requestid, status) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IMbnRegistrationEvents>, base.5, OnRegisterModeAvailable::<Impl, OFFSET>, OnRegisterStateChange::<Impl, OFFSET>, OnPacketServiceStateChange::<Impl, OFFSET>, OnSetRegisterModeComplete::<Impl, OFFSET>)
    }
}
pub trait IMbnServiceActivationImpl: Sized {
    fn Activate();
}
impl ::windows::core::RuntimeName for IMbnServiceActivation {
    const NAME: &'static str = "Windows.Win32.NetworkManagement.MobileBroadband.IMbnServiceActivation";
}
impl IMbnServiceActivationVtbl {
    pub const fn new<Impl: IMbnServiceActivationImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IMbnServiceActivationVtbl {
        unsafe extern "system" fn Activate<Impl: IMbnServiceActivationImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, vendorspecificdata: *const super::super::System::Com::SAFEARRAY, requestid: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Activate(&*(&vendorspecificdata as *const <super::super::System::Com::SAFEARRAY as ::windows::core::Abi>::Abi as *const <super::super::System::Com::SAFEARRAY as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&requestid)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IMbnServiceActivation>, base.5, Activate::<Impl, OFFSET>)
    }
}
pub trait IMbnServiceActivationEventsImpl: Sized {
    fn OnActivationComplete();
}
impl ::windows::core::RuntimeName for IMbnServiceActivationEvents {
    const NAME: &'static str = "Windows.Win32.NetworkManagement.MobileBroadband.IMbnServiceActivationEvents";
}
impl IMbnServiceActivationEventsVtbl {
    pub const fn new<Impl: IMbnServiceActivationEventsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IMbnServiceActivationEventsVtbl {
        unsafe extern "system" fn OnActivationComplete<Impl: IMbnServiceActivationEventsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, serviceactivation: ::windows::core::RawPtr, vendorspecificdata: *const super::super::System::Com::SAFEARRAY, requestid: u32, status: ::windows::core::HRESULT, networkerror: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).OnActivationComplete(&*(&serviceactivation as *const <IMbnServiceActivation as ::windows::core::Abi>::Abi as *const <IMbnServiceActivation as ::windows::core::DefaultType>::DefaultType), &*(&vendorspecificdata as *const <super::super::System::Com::SAFEARRAY as ::windows::core::Abi>::Abi as *const <super::super::System::Com::SAFEARRAY as ::windows::core::DefaultType>::DefaultType), requestid, status, networkerror) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IMbnServiceActivationEvents>, base.5, OnActivationComplete::<Impl, OFFSET>)
    }
}
pub trait IMbnSignalImpl: Sized {
    fn GetSignalStrength();
    fn GetSignalError();
}
impl ::windows::core::RuntimeName for IMbnSignal {
    const NAME: &'static str = "Windows.Win32.NetworkManagement.MobileBroadband.IMbnSignal";
}
impl IMbnSignalVtbl {
    pub const fn new<Impl: IMbnSignalImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IMbnSignalVtbl {
        unsafe extern "system" fn GetSignalStrength<Impl: IMbnSignalImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, signalstrength: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetSignalStrength(::core::mem::transmute_copy(&signalstrength)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSignalError<Impl: IMbnSignalImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, signalerror: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetSignalError(::core::mem::transmute_copy(&signalerror)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IMbnSignal>, base.5, GetSignalStrength::<Impl, OFFSET>, GetSignalError::<Impl, OFFSET>)
    }
}
pub trait IMbnSignalEventsImpl: Sized {
    fn OnSignalStateChange();
}
impl ::windows::core::RuntimeName for IMbnSignalEvents {
    const NAME: &'static str = "Windows.Win32.NetworkManagement.MobileBroadband.IMbnSignalEvents";
}
impl IMbnSignalEventsVtbl {
    pub const fn new<Impl: IMbnSignalEventsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IMbnSignalEventsVtbl {
        unsafe extern "system" fn OnSignalStateChange<Impl: IMbnSignalEventsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, newinterface: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).OnSignalStateChange(&*(&newinterface as *const <IMbnSignal as ::windows::core::Abi>::Abi as *const <IMbnSignal as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IMbnSignalEvents>, base.5, OnSignalStateChange::<Impl, OFFSET>)
    }
}
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
impl ::windows::core::RuntimeName for IMbnSms {
    const NAME: &'static str = "Windows.Win32.NetworkManagement.MobileBroadband.IMbnSms";
}
impl IMbnSmsVtbl {
    pub const fn new<Impl: IMbnSmsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IMbnSmsVtbl {
        unsafe extern "system" fn GetSmsConfiguration<Impl: IMbnSmsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, smsconfiguration: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetSmsConfiguration(::core::mem::transmute_copy(&smsconfiguration)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSmsConfiguration<Impl: IMbnSmsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, smsconfiguration: ::windows::core::RawPtr, requestid: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetSmsConfiguration(&*(&smsconfiguration as *const <IMbnSmsConfiguration as ::windows::core::Abi>::Abi as *const <IMbnSmsConfiguration as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&requestid)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SmsSendPdu<Impl: IMbnSmsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdudata: super::super::Foundation::PWSTR, size: u8, requestid: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SmsSendPdu(&*(&pdudata as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), size, ::core::mem::transmute_copy(&requestid)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SmsSendCdma<Impl: IMbnSmsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, address: super::super::Foundation::PWSTR, encoding: MBN_SMS_CDMA_ENCODING, language: MBN_SMS_CDMA_LANG, sizeincharacters: u32, message: *const super::super::System::Com::SAFEARRAY, requestid: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SmsSendCdma(&*(&address as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), encoding, language, sizeincharacters, &*(&message as *const <super::super::System::Com::SAFEARRAY as ::windows::core::Abi>::Abi as *const <super::super::System::Com::SAFEARRAY as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&requestid)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SmsSendCdmaPdu<Impl: IMbnSmsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, message: *const super::super::System::Com::SAFEARRAY, requestid: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SmsSendCdmaPdu(&*(&message as *const <super::super::System::Com::SAFEARRAY as ::windows::core::Abi>::Abi as *const <super::super::System::Com::SAFEARRAY as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&requestid)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SmsRead<Impl: IMbnSmsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, smsfilter: *const MBN_SMS_FILTER, smsformat: MBN_SMS_FORMAT, requestid: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SmsRead(&*(&smsfilter as *const <MBN_SMS_FILTER as ::windows::core::Abi>::Abi as *const <MBN_SMS_FILTER as ::windows::core::DefaultType>::DefaultType), smsformat, ::core::mem::transmute_copy(&requestid)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SmsDelete<Impl: IMbnSmsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, smsfilter: *const MBN_SMS_FILTER, requestid: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SmsDelete(&*(&smsfilter as *const <MBN_SMS_FILTER as ::windows::core::Abi>::Abi as *const <MBN_SMS_FILTER as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&requestid)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSmsStatus<Impl: IMbnSmsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, smsstatusinfo: *mut MBN_SMS_STATUS_INFO) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetSmsStatus(::core::mem::transmute_copy(&smsstatusinfo)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IMbnSms>, base.5, GetSmsConfiguration::<Impl, OFFSET>, SetSmsConfiguration::<Impl, OFFSET>, SmsSendPdu::<Impl, OFFSET>, SmsSendCdma::<Impl, OFFSET>, SmsSendCdmaPdu::<Impl, OFFSET>, SmsRead::<Impl, OFFSET>, SmsDelete::<Impl, OFFSET>, GetSmsStatus::<Impl, OFFSET>)
    }
}
pub trait IMbnSmsConfigurationImpl: Sized {
    fn ServiceCenterAddress();
    fn SetServiceCenterAddress();
    fn MaxMessageIndex();
    fn CdmaShortMsgSize();
    fn SmsFormat();
    fn SetSmsFormat();
}
impl ::windows::core::RuntimeName for IMbnSmsConfiguration {
    const NAME: &'static str = "Windows.Win32.NetworkManagement.MobileBroadband.IMbnSmsConfiguration";
}
impl IMbnSmsConfigurationVtbl {
    pub const fn new<Impl: IMbnSmsConfigurationImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IMbnSmsConfigurationVtbl {
        unsafe extern "system" fn ServiceCenterAddress<Impl: IMbnSmsConfigurationImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, scaddress: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ServiceCenterAddress(::core::mem::transmute_copy(&scaddress)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetServiceCenterAddress<Impl: IMbnSmsConfigurationImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, scaddress: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetServiceCenterAddress(&*(&scaddress as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MaxMessageIndex<Impl: IMbnSmsConfigurationImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, index: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).MaxMessageIndex(::core::mem::transmute_copy(&index)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CdmaShortMsgSize<Impl: IMbnSmsConfigurationImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, shortmsgsize: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CdmaShortMsgSize(::core::mem::transmute_copy(&shortmsgsize)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SmsFormat<Impl: IMbnSmsConfigurationImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, smsformat: *mut MBN_SMS_FORMAT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SmsFormat(::core::mem::transmute_copy(&smsformat)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSmsFormat<Impl: IMbnSmsConfigurationImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, smsformat: MBN_SMS_FORMAT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetSmsFormat(smsformat) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IMbnSmsConfiguration>, base.5, ServiceCenterAddress::<Impl, OFFSET>, SetServiceCenterAddress::<Impl, OFFSET>, MaxMessageIndex::<Impl, OFFSET>, CdmaShortMsgSize::<Impl, OFFSET>, SmsFormat::<Impl, OFFSET>, SetSmsFormat::<Impl, OFFSET>)
    }
}
pub trait IMbnSmsEventsImpl: Sized {
    fn OnSmsConfigurationChange();
    fn OnSetSmsConfigurationComplete();
    fn OnSmsSendComplete();
    fn OnSmsReadComplete();
    fn OnSmsNewClass0Message();
    fn OnSmsDeleteComplete();
    fn OnSmsStatusChange();
}
impl ::windows::core::RuntimeName for IMbnSmsEvents {
    const NAME: &'static str = "Windows.Win32.NetworkManagement.MobileBroadband.IMbnSmsEvents";
}
impl IMbnSmsEventsVtbl {
    pub const fn new<Impl: IMbnSmsEventsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IMbnSmsEventsVtbl {
        unsafe extern "system" fn OnSmsConfigurationChange<Impl: IMbnSmsEventsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, sms: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).OnSmsConfigurationChange(&*(&sms as *const <IMbnSms as ::windows::core::Abi>::Abi as *const <IMbnSms as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OnSetSmsConfigurationComplete<Impl: IMbnSmsEventsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, sms: ::windows::core::RawPtr, requestid: u32, status: ::windows::core::HRESULT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).OnSetSmsConfigurationComplete(&*(&sms as *const <IMbnSms as ::windows::core::Abi>::Abi as *const <IMbnSms as ::windows::core::DefaultType>::DefaultType), requestid, status) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OnSmsSendComplete<Impl: IMbnSmsEventsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, sms: ::windows::core::RawPtr, requestid: u32, status: ::windows::core::HRESULT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).OnSmsSendComplete(&*(&sms as *const <IMbnSms as ::windows::core::Abi>::Abi as *const <IMbnSms as ::windows::core::DefaultType>::DefaultType), requestid, status) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OnSmsReadComplete<Impl: IMbnSmsEventsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, sms: ::windows::core::RawPtr, smsformat: MBN_SMS_FORMAT, readmsgs: *const super::super::System::Com::SAFEARRAY, moremsgs: i16, requestid: u32, status: ::windows::core::HRESULT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).OnSmsReadComplete(&*(&sms as *const <IMbnSms as ::windows::core::Abi>::Abi as *const <IMbnSms as ::windows::core::DefaultType>::DefaultType), smsformat, &*(&readmsgs as *const <super::super::System::Com::SAFEARRAY as ::windows::core::Abi>::Abi as *const <super::super::System::Com::SAFEARRAY as ::windows::core::DefaultType>::DefaultType), moremsgs, requestid, status) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OnSmsNewClass0Message<Impl: IMbnSmsEventsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, sms: ::windows::core::RawPtr, smsformat: MBN_SMS_FORMAT, readmsgs: *const super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).OnSmsNewClass0Message(&*(&sms as *const <IMbnSms as ::windows::core::Abi>::Abi as *const <IMbnSms as ::windows::core::DefaultType>::DefaultType), smsformat, &*(&readmsgs as *const <super::super::System::Com::SAFEARRAY as ::windows::core::Abi>::Abi as *const <super::super::System::Com::SAFEARRAY as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OnSmsDeleteComplete<Impl: IMbnSmsEventsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, sms: ::windows::core::RawPtr, requestid: u32, status: ::windows::core::HRESULT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).OnSmsDeleteComplete(&*(&sms as *const <IMbnSms as ::windows::core::Abi>::Abi as *const <IMbnSms as ::windows::core::DefaultType>::DefaultType), requestid, status) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OnSmsStatusChange<Impl: IMbnSmsEventsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, sms: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).OnSmsStatusChange(&*(&sms as *const <IMbnSms as ::windows::core::Abi>::Abi as *const <IMbnSms as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IMbnSmsEvents>, base.5, OnSmsConfigurationChange::<Impl, OFFSET>, OnSetSmsConfigurationComplete::<Impl, OFFSET>, OnSmsSendComplete::<Impl, OFFSET>, OnSmsReadComplete::<Impl, OFFSET>, OnSmsNewClass0Message::<Impl, OFFSET>, OnSmsDeleteComplete::<Impl, OFFSET>, OnSmsStatusChange::<Impl, OFFSET>)
    }
}
pub trait IMbnSmsReadMsgPduImpl: Sized {
    fn Index();
    fn Status();
    fn PduData();
    fn Message();
}
impl ::windows::core::RuntimeName for IMbnSmsReadMsgPdu {
    const NAME: &'static str = "Windows.Win32.NetworkManagement.MobileBroadband.IMbnSmsReadMsgPdu";
}
impl IMbnSmsReadMsgPduVtbl {
    pub const fn new<Impl: IMbnSmsReadMsgPduImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IMbnSmsReadMsgPduVtbl {
        unsafe extern "system" fn Index<Impl: IMbnSmsReadMsgPduImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, index: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Index(::core::mem::transmute_copy(&index)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Status<Impl: IMbnSmsReadMsgPduImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, status: *mut MBN_MSG_STATUS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Status(::core::mem::transmute_copy(&status)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PduData<Impl: IMbnSmsReadMsgPduImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdudata: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).PduData(::core::mem::transmute_copy(&pdudata)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Message<Impl: IMbnSmsReadMsgPduImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, message: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Message(::core::mem::transmute_copy(&message)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IMbnSmsReadMsgPdu>, base.5, Index::<Impl, OFFSET>, Status::<Impl, OFFSET>, PduData::<Impl, OFFSET>, Message::<Impl, OFFSET>)
    }
}
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
impl ::windows::core::RuntimeName for IMbnSmsReadMsgTextCdma {
    const NAME: &'static str = "Windows.Win32.NetworkManagement.MobileBroadband.IMbnSmsReadMsgTextCdma";
}
impl IMbnSmsReadMsgTextCdmaVtbl {
    pub const fn new<Impl: IMbnSmsReadMsgTextCdmaImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IMbnSmsReadMsgTextCdmaVtbl {
        unsafe extern "system" fn Index<Impl: IMbnSmsReadMsgTextCdmaImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, index: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Index(::core::mem::transmute_copy(&index)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Status<Impl: IMbnSmsReadMsgTextCdmaImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, status: *mut MBN_MSG_STATUS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Status(::core::mem::transmute_copy(&status)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Address<Impl: IMbnSmsReadMsgTextCdmaImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, address: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Address(::core::mem::transmute_copy(&address)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Timestamp<Impl: IMbnSmsReadMsgTextCdmaImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, timestamp: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Timestamp(::core::mem::transmute_copy(&timestamp)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EncodingID<Impl: IMbnSmsReadMsgTextCdmaImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, encodingid: *mut MBN_SMS_CDMA_ENCODING) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).EncodingID(::core::mem::transmute_copy(&encodingid)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LanguageID<Impl: IMbnSmsReadMsgTextCdmaImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, languageid: *mut MBN_SMS_CDMA_LANG) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).LanguageID(::core::mem::transmute_copy(&languageid)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SizeInCharacters<Impl: IMbnSmsReadMsgTextCdmaImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, sizeincharacters: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SizeInCharacters(::core::mem::transmute_copy(&sizeincharacters)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Message<Impl: IMbnSmsReadMsgTextCdmaImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, message: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Message(::core::mem::transmute_copy(&message)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IMbnSmsReadMsgTextCdma>, base.5, Index::<Impl, OFFSET>, Status::<Impl, OFFSET>, Address::<Impl, OFFSET>, Timestamp::<Impl, OFFSET>, EncodingID::<Impl, OFFSET>, LanguageID::<Impl, OFFSET>, SizeInCharacters::<Impl, OFFSET>, Message::<Impl, OFFSET>)
    }
}
pub trait IMbnSubscriberInformationImpl: Sized {
    fn SubscriberID();
    fn SimIccID();
    fn TelephoneNumbers();
}
impl ::windows::core::RuntimeName for IMbnSubscriberInformation {
    const NAME: &'static str = "Windows.Win32.NetworkManagement.MobileBroadband.IMbnSubscriberInformation";
}
impl IMbnSubscriberInformationVtbl {
    pub const fn new<Impl: IMbnSubscriberInformationImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IMbnSubscriberInformationVtbl {
        unsafe extern "system" fn SubscriberID<Impl: IMbnSubscriberInformationImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, subscriberid: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SubscriberID(::core::mem::transmute_copy(&subscriberid)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SimIccID<Impl: IMbnSubscriberInformationImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, simiccid: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SimIccID(::core::mem::transmute_copy(&simiccid)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TelephoneNumbers<Impl: IMbnSubscriberInformationImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, telephonenumbers: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).TelephoneNumbers(::core::mem::transmute_copy(&telephonenumbers)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IMbnSubscriberInformation>, base.5, SubscriberID::<Impl, OFFSET>, SimIccID::<Impl, OFFSET>, TelephoneNumbers::<Impl, OFFSET>)
    }
}
pub trait IMbnVendorSpecificEventsImpl: Sized {
    fn OnEventNotification();
    fn OnSetVendorSpecificComplete();
}
impl ::windows::core::RuntimeName for IMbnVendorSpecificEvents {
    const NAME: &'static str = "Windows.Win32.NetworkManagement.MobileBroadband.IMbnVendorSpecificEvents";
}
impl IMbnVendorSpecificEventsVtbl {
    pub const fn new<Impl: IMbnVendorSpecificEventsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IMbnVendorSpecificEventsVtbl {
        unsafe extern "system" fn OnEventNotification<Impl: IMbnVendorSpecificEventsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, vendoroperation: ::windows::core::RawPtr, vendorspecificdata: *const super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).OnEventNotification(&*(&vendoroperation as *const <IMbnVendorSpecificOperation as ::windows::core::Abi>::Abi as *const <IMbnVendorSpecificOperation as ::windows::core::DefaultType>::DefaultType), &*(&vendorspecificdata as *const <super::super::System::Com::SAFEARRAY as ::windows::core::Abi>::Abi as *const <super::super::System::Com::SAFEARRAY as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OnSetVendorSpecificComplete<Impl: IMbnVendorSpecificEventsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, vendoroperation: ::windows::core::RawPtr, vendorspecificdata: *const super::super::System::Com::SAFEARRAY, requestid: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).OnSetVendorSpecificComplete(&*(&vendoroperation as *const <IMbnVendorSpecificOperation as ::windows::core::Abi>::Abi as *const <IMbnVendorSpecificOperation as ::windows::core::DefaultType>::DefaultType), &*(&vendorspecificdata as *const <super::super::System::Com::SAFEARRAY as ::windows::core::Abi>::Abi as *const <super::super::System::Com::SAFEARRAY as ::windows::core::DefaultType>::DefaultType), requestid) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IMbnVendorSpecificEvents>, base.5, OnEventNotification::<Impl, OFFSET>, OnSetVendorSpecificComplete::<Impl, OFFSET>)
    }
}
pub trait IMbnVendorSpecificOperationImpl: Sized {
    fn SetVendorSpecific();
}
impl ::windows::core::RuntimeName for IMbnVendorSpecificOperation {
    const NAME: &'static str = "Windows.Win32.NetworkManagement.MobileBroadband.IMbnVendorSpecificOperation";
}
impl IMbnVendorSpecificOperationVtbl {
    pub const fn new<Impl: IMbnVendorSpecificOperationImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IMbnVendorSpecificOperationVtbl {
        unsafe extern "system" fn SetVendorSpecific<Impl: IMbnVendorSpecificOperationImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, vendorspecificdata: *const super::super::System::Com::SAFEARRAY, requestid: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetVendorSpecific(&*(&vendorspecificdata as *const <super::super::System::Com::SAFEARRAY as ::windows::core::Abi>::Abi as *const <super::super::System::Com::SAFEARRAY as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&requestid)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IMbnVendorSpecificOperation>, base.5, SetVendorSpecific::<Impl, OFFSET>)
    }
}
