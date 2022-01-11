#[cfg(feature = "Win32_Foundation")]
pub trait IDot11AdHocInterfaceImpl: Sized {
    fn GetDeviceSignature();
    fn GetFriendlyName();
    fn IsDot11d();
    fn IsAdHocCapable();
    fn IsRadioOn();
    fn GetActiveNetwork();
    fn GetIEnumSecuritySettings();
    fn GetIEnumDot11AdHocNetworks();
    fn GetStatus();
}
#[cfg(feature = "Win32_Foundation")]
impl IDot11AdHocInterfaceVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDot11AdHocInterfaceImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDot11AdHocInterfaceVtbl {
        unsafe extern "system" fn GetDeviceSignature<Impl: IDot11AdHocInterfaceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psignature: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetFriendlyName<Impl: IDot11AdHocInterfaceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppszname: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn IsDot11d<Impl: IDot11AdHocInterfaceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pf11d: *mut u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn IsAdHocCapable<Impl: IDot11AdHocInterfaceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfadhoccapable: *mut u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn IsRadioOn<Impl: IDot11AdHocInterfaceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfisradioon: *mut u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetActiveNetwork<Impl: IDot11AdHocInterfaceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppnetwork: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetIEnumSecuritySettings<Impl: IDot11AdHocInterfaceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetIEnumDot11AdHocNetworks<Impl: IDot11AdHocInterfaceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfilterguid: *const ::windows::core::GUID, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetStatus<Impl: IDot11AdHocInterfaceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pstate: *mut DOT11_ADHOC_NETWORK_CONNECTION_STATUS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            GetDeviceSignature: GetDeviceSignature::<Impl, IMPL_OFFSET>,
            GetFriendlyName: GetFriendlyName::<Impl, IMPL_OFFSET>,
            IsDot11d: IsDot11d::<Impl, IMPL_OFFSET>,
            IsAdHocCapable: IsAdHocCapable::<Impl, IMPL_OFFSET>,
            IsRadioOn: IsRadioOn::<Impl, IMPL_OFFSET>,
            GetActiveNetwork: GetActiveNetwork::<Impl, IMPL_OFFSET>,
            GetIEnumSecuritySettings: GetIEnumSecuritySettings::<Impl, IMPL_OFFSET>,
            GetIEnumDot11AdHocNetworks: GetIEnumDot11AdHocNetworks::<Impl, IMPL_OFFSET>,
            GetStatus: GetStatus::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDot11AdHocInterface as ::windows::core::Interface>::IID
    }
}
pub trait IDot11AdHocInterfaceNotificationSinkImpl: Sized {
    fn OnConnectionStatusChange();
}
impl IDot11AdHocInterfaceNotificationSinkVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDot11AdHocInterfaceNotificationSinkImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDot11AdHocInterfaceNotificationSinkVtbl {
        unsafe extern "system" fn OnConnectionStatusChange<Impl: IDot11AdHocInterfaceNotificationSinkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, estatus: DOT11_ADHOC_NETWORK_CONNECTION_STATUS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), OnConnectionStatusChange: OnConnectionStatusChange::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDot11AdHocInterfaceNotificationSink as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IDot11AdHocManagerImpl: Sized {
    fn CreateNetwork();
    fn CommitCreatedNetwork();
    fn GetIEnumDot11AdHocNetworks();
    fn GetIEnumDot11AdHocInterfaces();
    fn GetNetwork();
}
#[cfg(feature = "Win32_Foundation")]
impl IDot11AdHocManagerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDot11AdHocManagerImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDot11AdHocManagerVtbl {
        unsafe extern "system" fn CreateNetwork<Impl: IDot11AdHocManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: super::super::Foundation::PWSTR, password: super::super::Foundation::PWSTR, geographicalid: i32, pinterface: ::windows::core::RawPtr, psecurity: ::windows::core::RawPtr, pcontextguid: *const ::windows::core::GUID, piadhoc: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CommitCreatedNetwork<Impl: IDot11AdHocManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, piadhoc: ::windows::core::RawPtr, fsaveprofile: super::super::Foundation::BOOLEAN, fmakesavedprofileuserspecific: super::super::Foundation::BOOLEAN) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetIEnumDot11AdHocNetworks<Impl: IDot11AdHocManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcontextguid: *const ::windows::core::GUID, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetIEnumDot11AdHocInterfaces<Impl: IDot11AdHocManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetNetwork<Impl: IDot11AdHocManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, networksignature: *const ::windows::core::GUID, pnetwork: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            CreateNetwork: CreateNetwork::<Impl, IMPL_OFFSET>,
            CommitCreatedNetwork: CommitCreatedNetwork::<Impl, IMPL_OFFSET>,
            GetIEnumDot11AdHocNetworks: GetIEnumDot11AdHocNetworks::<Impl, IMPL_OFFSET>,
            GetIEnumDot11AdHocInterfaces: GetIEnumDot11AdHocInterfaces::<Impl, IMPL_OFFSET>,
            GetNetwork: GetNetwork::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDot11AdHocManager as ::windows::core::Interface>::IID
    }
}
pub trait IDot11AdHocManagerNotificationSinkImpl: Sized {
    fn OnNetworkAdd();
    fn OnNetworkRemove();
    fn OnInterfaceAdd();
    fn OnInterfaceRemove();
}
impl IDot11AdHocManagerNotificationSinkVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDot11AdHocManagerNotificationSinkImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDot11AdHocManagerNotificationSinkVtbl {
        unsafe extern "system" fn OnNetworkAdd<Impl: IDot11AdHocManagerNotificationSinkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, piadhocnetwork: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn OnNetworkRemove<Impl: IDot11AdHocManagerNotificationSinkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, signature: *const ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn OnInterfaceAdd<Impl: IDot11AdHocManagerNotificationSinkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, piadhocinterface: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn OnInterfaceRemove<Impl: IDot11AdHocManagerNotificationSinkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, signature: *const ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            OnNetworkAdd: OnNetworkAdd::<Impl, IMPL_OFFSET>,
            OnNetworkRemove: OnNetworkRemove::<Impl, IMPL_OFFSET>,
            OnInterfaceAdd: OnInterfaceAdd::<Impl, IMPL_OFFSET>,
            OnInterfaceRemove: OnInterfaceRemove::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDot11AdHocManagerNotificationSink as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IDot11AdHocNetworkImpl: Sized {
    fn GetStatus();
    fn GetSSID();
    fn HasProfile();
    fn GetProfileName();
    fn DeleteProfile();
    fn GetSignalQuality();
    fn GetSecuritySetting();
    fn GetContextGuid();
    fn GetSignature();
    fn GetInterface();
    fn Connect();
    fn Disconnect();
}
#[cfg(feature = "Win32_Foundation")]
impl IDot11AdHocNetworkVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDot11AdHocNetworkImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDot11AdHocNetworkVtbl {
        unsafe extern "system" fn GetStatus<Impl: IDot11AdHocNetworkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, estatus: *mut DOT11_ADHOC_NETWORK_CONNECTION_STATUS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetSSID<Impl: IDot11AdHocNetworkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppszwssid: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn HasProfile<Impl: IDot11AdHocNetworkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pf11d: *mut u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetProfileName<Impl: IDot11AdHocNetworkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppszwprofilename: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn DeleteProfile<Impl: IDot11AdHocNetworkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetSignalQuality<Impl: IDot11AdHocNetworkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pustrengthvalue: *mut u32, pustrengthmax: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetSecuritySetting<Impl: IDot11AdHocNetworkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, padhocsecuritysetting: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetContextGuid<Impl: IDot11AdHocNetworkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcontextguid: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetSignature<Impl: IDot11AdHocNetworkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psignature: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetInterface<Impl: IDot11AdHocNetworkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, padhocinterface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Connect<Impl: IDot11AdHocNetworkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, passphrase: super::super::Foundation::PWSTR, geographicalid: i32, fsaveprofile: super::super::Foundation::BOOLEAN, fmakesavedprofileuserspecific: super::super::Foundation::BOOLEAN) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Disconnect<Impl: IDot11AdHocNetworkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            GetStatus: GetStatus::<Impl, IMPL_OFFSET>,
            GetSSID: GetSSID::<Impl, IMPL_OFFSET>,
            HasProfile: HasProfile::<Impl, IMPL_OFFSET>,
            GetProfileName: GetProfileName::<Impl, IMPL_OFFSET>,
            DeleteProfile: DeleteProfile::<Impl, IMPL_OFFSET>,
            GetSignalQuality: GetSignalQuality::<Impl, IMPL_OFFSET>,
            GetSecuritySetting: GetSecuritySetting::<Impl, IMPL_OFFSET>,
            GetContextGuid: GetContextGuid::<Impl, IMPL_OFFSET>,
            GetSignature: GetSignature::<Impl, IMPL_OFFSET>,
            GetInterface: GetInterface::<Impl, IMPL_OFFSET>,
            Connect: Connect::<Impl, IMPL_OFFSET>,
            Disconnect: Disconnect::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDot11AdHocNetwork as ::windows::core::Interface>::IID
    }
}
pub trait IDot11AdHocNetworkNotificationSinkImpl: Sized {
    fn OnStatusChange();
    fn OnConnectFail();
}
impl IDot11AdHocNetworkNotificationSinkVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDot11AdHocNetworkNotificationSinkImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDot11AdHocNetworkNotificationSinkVtbl {
        unsafe extern "system" fn OnStatusChange<Impl: IDot11AdHocNetworkNotificationSinkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, estatus: DOT11_ADHOC_NETWORK_CONNECTION_STATUS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn OnConnectFail<Impl: IDot11AdHocNetworkNotificationSinkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, efailreason: DOT11_ADHOC_CONNECT_FAIL_REASON) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            OnStatusChange: OnStatusChange::<Impl, IMPL_OFFSET>,
            OnConnectFail: OnConnectFail::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDot11AdHocNetworkNotificationSink as ::windows::core::Interface>::IID
    }
}
pub trait IDot11AdHocSecuritySettingsImpl: Sized {
    fn GetDot11AuthAlgorithm();
    fn GetDot11CipherAlgorithm();
}
impl IDot11AdHocSecuritySettingsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDot11AdHocSecuritySettingsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDot11AdHocSecuritySettingsVtbl {
        unsafe extern "system" fn GetDot11AuthAlgorithm<Impl: IDot11AdHocSecuritySettingsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pauth: *mut DOT11_ADHOC_AUTH_ALGORITHM) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetDot11CipherAlgorithm<Impl: IDot11AdHocSecuritySettingsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcipher: *mut DOT11_ADHOC_CIPHER_ALGORITHM) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            GetDot11AuthAlgorithm: GetDot11AuthAlgorithm::<Impl, IMPL_OFFSET>,
            GetDot11CipherAlgorithm: GetDot11CipherAlgorithm::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDot11AdHocSecuritySettings as ::windows::core::Interface>::IID
    }
}
pub trait IEnumDot11AdHocInterfacesImpl: Sized {
    fn Next();
    fn Skip();
    fn Reset();
    fn Clone();
}
impl IEnumDot11AdHocInterfacesVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IEnumDot11AdHocInterfacesImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IEnumDot11AdHocInterfacesVtbl {
        unsafe extern "system" fn Next<Impl: IEnumDot11AdHocInterfacesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32, rgelt: *mut ::windows::core::RawPtr, pceltfetched: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Skip<Impl: IEnumDot11AdHocInterfacesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Reset<Impl: IEnumDot11AdHocInterfacesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Clone<Impl: IEnumDot11AdHocInterfacesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            Next: Next::<Impl, IMPL_OFFSET>,
            Skip: Skip::<Impl, IMPL_OFFSET>,
            Reset: Reset::<Impl, IMPL_OFFSET>,
            Clone: Clone::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IEnumDot11AdHocInterfaces as ::windows::core::Interface>::IID
    }
}
pub trait IEnumDot11AdHocNetworksImpl: Sized {
    fn Next();
    fn Skip();
    fn Reset();
    fn Clone();
}
impl IEnumDot11AdHocNetworksVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IEnumDot11AdHocNetworksImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IEnumDot11AdHocNetworksVtbl {
        unsafe extern "system" fn Next<Impl: IEnumDot11AdHocNetworksImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32, rgelt: *mut ::windows::core::RawPtr, pceltfetched: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Skip<Impl: IEnumDot11AdHocNetworksImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Reset<Impl: IEnumDot11AdHocNetworksImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Clone<Impl: IEnumDot11AdHocNetworksImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            Next: Next::<Impl, IMPL_OFFSET>,
            Skip: Skip::<Impl, IMPL_OFFSET>,
            Reset: Reset::<Impl, IMPL_OFFSET>,
            Clone: Clone::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IEnumDot11AdHocNetworks as ::windows::core::Interface>::IID
    }
}
pub trait IEnumDot11AdHocSecuritySettingsImpl: Sized {
    fn Next();
    fn Skip();
    fn Reset();
    fn Clone();
}
impl IEnumDot11AdHocSecuritySettingsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IEnumDot11AdHocSecuritySettingsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IEnumDot11AdHocSecuritySettingsVtbl {
        unsafe extern "system" fn Next<Impl: IEnumDot11AdHocSecuritySettingsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32, rgelt: *mut ::windows::core::RawPtr, pceltfetched: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Skip<Impl: IEnumDot11AdHocSecuritySettingsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Reset<Impl: IEnumDot11AdHocSecuritySettingsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Clone<Impl: IEnumDot11AdHocSecuritySettingsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            Next: Next::<Impl, IMPL_OFFSET>,
            Skip: Skip::<Impl, IMPL_OFFSET>,
            Reset: Reset::<Impl, IMPL_OFFSET>,
            Clone: Clone::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IEnumDot11AdHocSecuritySettings as ::windows::core::Interface>::IID
    }
}
