#[cfg(feature = "Win32_Foundation")]
pub trait IDot11AdHocInterface_Impl: Sized {
    fn GetDeviceSignature(&mut self, psignature: *mut ::windows::core::GUID) -> ::windows::core::Result<()>;
    fn GetFriendlyName(&mut self) -> ::windows::core::Result<super::super::Foundation::PWSTR>;
    fn IsDot11d(&mut self, pf11d: *mut u8) -> ::windows::core::Result<()>;
    fn IsAdHocCapable(&mut self, pfadhoccapable: *mut u8) -> ::windows::core::Result<()>;
    fn IsRadioOn(&mut self, pfisradioon: *mut u8) -> ::windows::core::Result<()>;
    fn GetActiveNetwork(&mut self) -> ::windows::core::Result<IDot11AdHocNetwork>;
    fn GetIEnumSecuritySettings(&mut self) -> ::windows::core::Result<IEnumDot11AdHocSecuritySettings>;
    fn GetIEnumDot11AdHocNetworks(&mut self, pfilterguid: *const ::windows::core::GUID) -> ::windows::core::Result<IEnumDot11AdHocNetworks>;
    fn GetStatus(&mut self, pstate: *mut DOT11_ADHOC_NETWORK_CONNECTION_STATUS) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IDot11AdHocInterface_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDot11AdHocInterface_Impl, const OFFSET: isize>() -> IDot11AdHocInterface_Vtbl {
        unsafe extern "system" fn GetDeviceSignature<Identity: ::windows::core::IUnknownImpl, Impl: IDot11AdHocInterface_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psignature: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetDeviceSignature(::core::mem::transmute_copy(&psignature)).into()
        }
        unsafe extern "system" fn GetFriendlyName<Identity: ::windows::core::IUnknownImpl, Impl: IDot11AdHocInterface_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppszname: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetFriendlyName() {
                ::core::result::Result::Ok(ok__) => {
                    *ppszname = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsDot11d<Identity: ::windows::core::IUnknownImpl, Impl: IDot11AdHocInterface_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pf11d: *mut u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).IsDot11d(::core::mem::transmute_copy(&pf11d)).into()
        }
        unsafe extern "system" fn IsAdHocCapable<Identity: ::windows::core::IUnknownImpl, Impl: IDot11AdHocInterface_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfadhoccapable: *mut u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).IsAdHocCapable(::core::mem::transmute_copy(&pfadhoccapable)).into()
        }
        unsafe extern "system" fn IsRadioOn<Identity: ::windows::core::IUnknownImpl, Impl: IDot11AdHocInterface_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfisradioon: *mut u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).IsRadioOn(::core::mem::transmute_copy(&pfisradioon)).into()
        }
        unsafe extern "system" fn GetActiveNetwork<Identity: ::windows::core::IUnknownImpl, Impl: IDot11AdHocInterface_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppnetwork: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetActiveNetwork() {
                ::core::result::Result::Ok(ok__) => {
                    *ppnetwork = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetIEnumSecuritySettings<Identity: ::windows::core::IUnknownImpl, Impl: IDot11AdHocInterface_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetIEnumSecuritySettings() {
                ::core::result::Result::Ok(ok__) => {
                    *ppenum = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetIEnumDot11AdHocNetworks<Identity: ::windows::core::IUnknownImpl, Impl: IDot11AdHocInterface_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfilterguid: *const ::windows::core::GUID, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetIEnumDot11AdHocNetworks(::core::mem::transmute_copy(&pfilterguid)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppenum = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetStatus<Identity: ::windows::core::IUnknownImpl, Impl: IDot11AdHocInterface_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pstate: *mut DOT11_ADHOC_NETWORK_CONNECTION_STATUS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetStatus(::core::mem::transmute_copy(&pstate)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            GetDeviceSignature: GetDeviceSignature::<Identity, Impl, OFFSET>,
            GetFriendlyName: GetFriendlyName::<Identity, Impl, OFFSET>,
            IsDot11d: IsDot11d::<Identity, Impl, OFFSET>,
            IsAdHocCapable: IsAdHocCapable::<Identity, Impl, OFFSET>,
            IsRadioOn: IsRadioOn::<Identity, Impl, OFFSET>,
            GetActiveNetwork: GetActiveNetwork::<Identity, Impl, OFFSET>,
            GetIEnumSecuritySettings: GetIEnumSecuritySettings::<Identity, Impl, OFFSET>,
            GetIEnumDot11AdHocNetworks: GetIEnumDot11AdHocNetworks::<Identity, Impl, OFFSET>,
            GetStatus: GetStatus::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDot11AdHocInterface as ::windows::core::Interface>::IID
    }
}
pub trait IDot11AdHocInterfaceNotificationSink_Impl: Sized {
    fn OnConnectionStatusChange(&mut self, estatus: DOT11_ADHOC_NETWORK_CONNECTION_STATUS) -> ::windows::core::Result<()>;
}
impl IDot11AdHocInterfaceNotificationSink_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDot11AdHocInterfaceNotificationSink_Impl, const OFFSET: isize>() -> IDot11AdHocInterfaceNotificationSink_Vtbl {
        unsafe extern "system" fn OnConnectionStatusChange<Identity: ::windows::core::IUnknownImpl, Impl: IDot11AdHocInterfaceNotificationSink_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, estatus: DOT11_ADHOC_NETWORK_CONNECTION_STATUS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).OnConnectionStatusChange(::core::mem::transmute_copy(&estatus)).into()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(), OnConnectionStatusChange: OnConnectionStatusChange::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDot11AdHocInterfaceNotificationSink as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IDot11AdHocManager_Impl: Sized {
    fn CreateNetwork(&mut self, name: super::super::Foundation::PWSTR, password: super::super::Foundation::PWSTR, geographicalid: i32, pinterface: &::core::option::Option<IDot11AdHocInterface>, psecurity: &::core::option::Option<IDot11AdHocSecuritySettings>, pcontextguid: *const ::windows::core::GUID) -> ::windows::core::Result<IDot11AdHocNetwork>;
    fn CommitCreatedNetwork(&mut self, piadhoc: &::core::option::Option<IDot11AdHocNetwork>, fsaveprofile: super::super::Foundation::BOOLEAN, fmakesavedprofileuserspecific: super::super::Foundation::BOOLEAN) -> ::windows::core::Result<()>;
    fn GetIEnumDot11AdHocNetworks(&mut self, pcontextguid: *const ::windows::core::GUID) -> ::windows::core::Result<IEnumDot11AdHocNetworks>;
    fn GetIEnumDot11AdHocInterfaces(&mut self) -> ::windows::core::Result<IEnumDot11AdHocInterfaces>;
    fn GetNetwork(&mut self, networksignature: *const ::windows::core::GUID) -> ::windows::core::Result<IDot11AdHocNetwork>;
}
#[cfg(feature = "Win32_Foundation")]
impl IDot11AdHocManager_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDot11AdHocManager_Impl, const OFFSET: isize>() -> IDot11AdHocManager_Vtbl {
        unsafe extern "system" fn CreateNetwork<Identity: ::windows::core::IUnknownImpl, Impl: IDot11AdHocManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: super::super::Foundation::PWSTR, password: super::super::Foundation::PWSTR, geographicalid: i32, pinterface: ::windows::core::RawPtr, psecurity: ::windows::core::RawPtr, pcontextguid: *const ::windows::core::GUID, piadhoc: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CreateNetwork(::core::mem::transmute_copy(&name), ::core::mem::transmute_copy(&password), ::core::mem::transmute_copy(&geographicalid), ::core::mem::transmute(&pinterface), ::core::mem::transmute(&psecurity), ::core::mem::transmute_copy(&pcontextguid)) {
                ::core::result::Result::Ok(ok__) => {
                    *piadhoc = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CommitCreatedNetwork<Identity: ::windows::core::IUnknownImpl, Impl: IDot11AdHocManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, piadhoc: ::windows::core::RawPtr, fsaveprofile: super::super::Foundation::BOOLEAN, fmakesavedprofileuserspecific: super::super::Foundation::BOOLEAN) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).CommitCreatedNetwork(::core::mem::transmute(&piadhoc), ::core::mem::transmute_copy(&fsaveprofile), ::core::mem::transmute_copy(&fmakesavedprofileuserspecific)).into()
        }
        unsafe extern "system" fn GetIEnumDot11AdHocNetworks<Identity: ::windows::core::IUnknownImpl, Impl: IDot11AdHocManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcontextguid: *const ::windows::core::GUID, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetIEnumDot11AdHocNetworks(::core::mem::transmute_copy(&pcontextguid)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppenum = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetIEnumDot11AdHocInterfaces<Identity: ::windows::core::IUnknownImpl, Impl: IDot11AdHocManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetIEnumDot11AdHocInterfaces() {
                ::core::result::Result::Ok(ok__) => {
                    *ppenum = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetNetwork<Identity: ::windows::core::IUnknownImpl, Impl: IDot11AdHocManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, networksignature: *const ::windows::core::GUID, pnetwork: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetNetwork(::core::mem::transmute_copy(&networksignature)) {
                ::core::result::Result::Ok(ok__) => {
                    *pnetwork = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            CreateNetwork: CreateNetwork::<Identity, Impl, OFFSET>,
            CommitCreatedNetwork: CommitCreatedNetwork::<Identity, Impl, OFFSET>,
            GetIEnumDot11AdHocNetworks: GetIEnumDot11AdHocNetworks::<Identity, Impl, OFFSET>,
            GetIEnumDot11AdHocInterfaces: GetIEnumDot11AdHocInterfaces::<Identity, Impl, OFFSET>,
            GetNetwork: GetNetwork::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDot11AdHocManager as ::windows::core::Interface>::IID
    }
}
pub trait IDot11AdHocManagerNotificationSink_Impl: Sized {
    fn OnNetworkAdd(&mut self, piadhocnetwork: &::core::option::Option<IDot11AdHocNetwork>) -> ::windows::core::Result<()>;
    fn OnNetworkRemove(&mut self, signature: *const ::windows::core::GUID) -> ::windows::core::Result<()>;
    fn OnInterfaceAdd(&mut self, piadhocinterface: &::core::option::Option<IDot11AdHocInterface>) -> ::windows::core::Result<()>;
    fn OnInterfaceRemove(&mut self, signature: *const ::windows::core::GUID) -> ::windows::core::Result<()>;
}
impl IDot11AdHocManagerNotificationSink_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDot11AdHocManagerNotificationSink_Impl, const OFFSET: isize>() -> IDot11AdHocManagerNotificationSink_Vtbl {
        unsafe extern "system" fn OnNetworkAdd<Identity: ::windows::core::IUnknownImpl, Impl: IDot11AdHocManagerNotificationSink_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, piadhocnetwork: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).OnNetworkAdd(::core::mem::transmute(&piadhocnetwork)).into()
        }
        unsafe extern "system" fn OnNetworkRemove<Identity: ::windows::core::IUnknownImpl, Impl: IDot11AdHocManagerNotificationSink_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, signature: *const ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).OnNetworkRemove(::core::mem::transmute_copy(&signature)).into()
        }
        unsafe extern "system" fn OnInterfaceAdd<Identity: ::windows::core::IUnknownImpl, Impl: IDot11AdHocManagerNotificationSink_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, piadhocinterface: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).OnInterfaceAdd(::core::mem::transmute(&piadhocinterface)).into()
        }
        unsafe extern "system" fn OnInterfaceRemove<Identity: ::windows::core::IUnknownImpl, Impl: IDot11AdHocManagerNotificationSink_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, signature: *const ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).OnInterfaceRemove(::core::mem::transmute_copy(&signature)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            OnNetworkAdd: OnNetworkAdd::<Identity, Impl, OFFSET>,
            OnNetworkRemove: OnNetworkRemove::<Identity, Impl, OFFSET>,
            OnInterfaceAdd: OnInterfaceAdd::<Identity, Impl, OFFSET>,
            OnInterfaceRemove: OnInterfaceRemove::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDot11AdHocManagerNotificationSink as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IDot11AdHocNetwork_Impl: Sized {
    fn GetStatus(&mut self, estatus: *mut DOT11_ADHOC_NETWORK_CONNECTION_STATUS) -> ::windows::core::Result<()>;
    fn GetSSID(&mut self) -> ::windows::core::Result<super::super::Foundation::PWSTR>;
    fn HasProfile(&mut self, pf11d: *mut u8) -> ::windows::core::Result<()>;
    fn GetProfileName(&mut self) -> ::windows::core::Result<super::super::Foundation::PWSTR>;
    fn DeleteProfile(&mut self) -> ::windows::core::Result<()>;
    fn GetSignalQuality(&mut self, pustrengthvalue: *mut u32, pustrengthmax: *mut u32) -> ::windows::core::Result<()>;
    fn GetSecuritySetting(&mut self) -> ::windows::core::Result<IDot11AdHocSecuritySettings>;
    fn GetContextGuid(&mut self, pcontextguid: *mut ::windows::core::GUID) -> ::windows::core::Result<()>;
    fn GetSignature(&mut self, psignature: *mut ::windows::core::GUID) -> ::windows::core::Result<()>;
    fn GetInterface(&mut self) -> ::windows::core::Result<IDot11AdHocInterface>;
    fn Connect(&mut self, passphrase: super::super::Foundation::PWSTR, geographicalid: i32, fsaveprofile: super::super::Foundation::BOOLEAN, fmakesavedprofileuserspecific: super::super::Foundation::BOOLEAN) -> ::windows::core::Result<()>;
    fn Disconnect(&mut self) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IDot11AdHocNetwork_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDot11AdHocNetwork_Impl, const OFFSET: isize>() -> IDot11AdHocNetwork_Vtbl {
        unsafe extern "system" fn GetStatus<Identity: ::windows::core::IUnknownImpl, Impl: IDot11AdHocNetwork_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, estatus: *mut DOT11_ADHOC_NETWORK_CONNECTION_STATUS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetStatus(::core::mem::transmute_copy(&estatus)).into()
        }
        unsafe extern "system" fn GetSSID<Identity: ::windows::core::IUnknownImpl, Impl: IDot11AdHocNetwork_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppszwssid: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetSSID() {
                ::core::result::Result::Ok(ok__) => {
                    *ppszwssid = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn HasProfile<Identity: ::windows::core::IUnknownImpl, Impl: IDot11AdHocNetwork_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pf11d: *mut u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).HasProfile(::core::mem::transmute_copy(&pf11d)).into()
        }
        unsafe extern "system" fn GetProfileName<Identity: ::windows::core::IUnknownImpl, Impl: IDot11AdHocNetwork_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppszwprofilename: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetProfileName() {
                ::core::result::Result::Ok(ok__) => {
                    *ppszwprofilename = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DeleteProfile<Identity: ::windows::core::IUnknownImpl, Impl: IDot11AdHocNetwork_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).DeleteProfile().into()
        }
        unsafe extern "system" fn GetSignalQuality<Identity: ::windows::core::IUnknownImpl, Impl: IDot11AdHocNetwork_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pustrengthvalue: *mut u32, pustrengthmax: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetSignalQuality(::core::mem::transmute_copy(&pustrengthvalue), ::core::mem::transmute_copy(&pustrengthmax)).into()
        }
        unsafe extern "system" fn GetSecuritySetting<Identity: ::windows::core::IUnknownImpl, Impl: IDot11AdHocNetwork_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, padhocsecuritysetting: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetSecuritySetting() {
                ::core::result::Result::Ok(ok__) => {
                    *padhocsecuritysetting = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetContextGuid<Identity: ::windows::core::IUnknownImpl, Impl: IDot11AdHocNetwork_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcontextguid: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetContextGuid(::core::mem::transmute_copy(&pcontextguid)).into()
        }
        unsafe extern "system" fn GetSignature<Identity: ::windows::core::IUnknownImpl, Impl: IDot11AdHocNetwork_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psignature: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetSignature(::core::mem::transmute_copy(&psignature)).into()
        }
        unsafe extern "system" fn GetInterface<Identity: ::windows::core::IUnknownImpl, Impl: IDot11AdHocNetwork_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, padhocinterface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetInterface() {
                ::core::result::Result::Ok(ok__) => {
                    *padhocinterface = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Connect<Identity: ::windows::core::IUnknownImpl, Impl: IDot11AdHocNetwork_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, passphrase: super::super::Foundation::PWSTR, geographicalid: i32, fsaveprofile: super::super::Foundation::BOOLEAN, fmakesavedprofileuserspecific: super::super::Foundation::BOOLEAN) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Connect(::core::mem::transmute_copy(&passphrase), ::core::mem::transmute_copy(&geographicalid), ::core::mem::transmute_copy(&fsaveprofile), ::core::mem::transmute_copy(&fmakesavedprofileuserspecific)).into()
        }
        unsafe extern "system" fn Disconnect<Identity: ::windows::core::IUnknownImpl, Impl: IDot11AdHocNetwork_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Disconnect().into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            GetStatus: GetStatus::<Identity, Impl, OFFSET>,
            GetSSID: GetSSID::<Identity, Impl, OFFSET>,
            HasProfile: HasProfile::<Identity, Impl, OFFSET>,
            GetProfileName: GetProfileName::<Identity, Impl, OFFSET>,
            DeleteProfile: DeleteProfile::<Identity, Impl, OFFSET>,
            GetSignalQuality: GetSignalQuality::<Identity, Impl, OFFSET>,
            GetSecuritySetting: GetSecuritySetting::<Identity, Impl, OFFSET>,
            GetContextGuid: GetContextGuid::<Identity, Impl, OFFSET>,
            GetSignature: GetSignature::<Identity, Impl, OFFSET>,
            GetInterface: GetInterface::<Identity, Impl, OFFSET>,
            Connect: Connect::<Identity, Impl, OFFSET>,
            Disconnect: Disconnect::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDot11AdHocNetwork as ::windows::core::Interface>::IID
    }
}
pub trait IDot11AdHocNetworkNotificationSink_Impl: Sized {
    fn OnStatusChange(&mut self, estatus: DOT11_ADHOC_NETWORK_CONNECTION_STATUS) -> ::windows::core::Result<()>;
    fn OnConnectFail(&mut self, efailreason: DOT11_ADHOC_CONNECT_FAIL_REASON) -> ::windows::core::Result<()>;
}
impl IDot11AdHocNetworkNotificationSink_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDot11AdHocNetworkNotificationSink_Impl, const OFFSET: isize>() -> IDot11AdHocNetworkNotificationSink_Vtbl {
        unsafe extern "system" fn OnStatusChange<Identity: ::windows::core::IUnknownImpl, Impl: IDot11AdHocNetworkNotificationSink_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, estatus: DOT11_ADHOC_NETWORK_CONNECTION_STATUS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).OnStatusChange(::core::mem::transmute_copy(&estatus)).into()
        }
        unsafe extern "system" fn OnConnectFail<Identity: ::windows::core::IUnknownImpl, Impl: IDot11AdHocNetworkNotificationSink_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, efailreason: DOT11_ADHOC_CONNECT_FAIL_REASON) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).OnConnectFail(::core::mem::transmute_copy(&efailreason)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            OnStatusChange: OnStatusChange::<Identity, Impl, OFFSET>,
            OnConnectFail: OnConnectFail::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDot11AdHocNetworkNotificationSink as ::windows::core::Interface>::IID
    }
}
pub trait IDot11AdHocSecuritySettings_Impl: Sized {
    fn GetDot11AuthAlgorithm(&mut self, pauth: *mut DOT11_ADHOC_AUTH_ALGORITHM) -> ::windows::core::Result<()>;
    fn GetDot11CipherAlgorithm(&mut self, pcipher: *mut DOT11_ADHOC_CIPHER_ALGORITHM) -> ::windows::core::Result<()>;
}
impl IDot11AdHocSecuritySettings_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDot11AdHocSecuritySettings_Impl, const OFFSET: isize>() -> IDot11AdHocSecuritySettings_Vtbl {
        unsafe extern "system" fn GetDot11AuthAlgorithm<Identity: ::windows::core::IUnknownImpl, Impl: IDot11AdHocSecuritySettings_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pauth: *mut DOT11_ADHOC_AUTH_ALGORITHM) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetDot11AuthAlgorithm(::core::mem::transmute_copy(&pauth)).into()
        }
        unsafe extern "system" fn GetDot11CipherAlgorithm<Identity: ::windows::core::IUnknownImpl, Impl: IDot11AdHocSecuritySettings_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcipher: *mut DOT11_ADHOC_CIPHER_ALGORITHM) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetDot11CipherAlgorithm(::core::mem::transmute_copy(&pcipher)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            GetDot11AuthAlgorithm: GetDot11AuthAlgorithm::<Identity, Impl, OFFSET>,
            GetDot11CipherAlgorithm: GetDot11CipherAlgorithm::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDot11AdHocSecuritySettings as ::windows::core::Interface>::IID
    }
}
pub trait IEnumDot11AdHocInterfaces_Impl: Sized {
    fn Next(&mut self, celt: u32, rgelt: *mut ::core::option::Option<IDot11AdHocInterface>, pceltfetched: *mut u32) -> ::windows::core::Result<()>;
    fn Skip(&mut self, celt: u32) -> ::windows::core::Result<()>;
    fn Reset(&mut self) -> ::windows::core::Result<()>;
    fn Clone(&mut self) -> ::windows::core::Result<IEnumDot11AdHocInterfaces>;
}
impl IEnumDot11AdHocInterfaces_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IEnumDot11AdHocInterfaces_Impl, const OFFSET: isize>() -> IEnumDot11AdHocInterfaces_Vtbl {
        unsafe extern "system" fn Next<Identity: ::windows::core::IUnknownImpl, Impl: IEnumDot11AdHocInterfaces_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32, rgelt: *mut ::windows::core::RawPtr, pceltfetched: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Next(::core::mem::transmute_copy(&celt), ::core::mem::transmute_copy(&rgelt), ::core::mem::transmute_copy(&pceltfetched)).into()
        }
        unsafe extern "system" fn Skip<Identity: ::windows::core::IUnknownImpl, Impl: IEnumDot11AdHocInterfaces_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Skip(::core::mem::transmute_copy(&celt)).into()
        }
        unsafe extern "system" fn Reset<Identity: ::windows::core::IUnknownImpl, Impl: IEnumDot11AdHocInterfaces_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Reset().into()
        }
        unsafe extern "system" fn Clone<Identity: ::windows::core::IUnknownImpl, Impl: IEnumDot11AdHocInterfaces_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Clone() {
                ::core::result::Result::Ok(ok__) => {
                    *ppenum = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            Next: Next::<Identity, Impl, OFFSET>,
            Skip: Skip::<Identity, Impl, OFFSET>,
            Reset: Reset::<Identity, Impl, OFFSET>,
            Clone: Clone::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IEnumDot11AdHocInterfaces as ::windows::core::Interface>::IID
    }
}
pub trait IEnumDot11AdHocNetworks_Impl: Sized {
    fn Next(&mut self, celt: u32, rgelt: *mut ::core::option::Option<IDot11AdHocNetwork>, pceltfetched: *mut u32) -> ::windows::core::Result<()>;
    fn Skip(&mut self, celt: u32) -> ::windows::core::Result<()>;
    fn Reset(&mut self) -> ::windows::core::Result<()>;
    fn Clone(&mut self) -> ::windows::core::Result<IEnumDot11AdHocNetworks>;
}
impl IEnumDot11AdHocNetworks_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IEnumDot11AdHocNetworks_Impl, const OFFSET: isize>() -> IEnumDot11AdHocNetworks_Vtbl {
        unsafe extern "system" fn Next<Identity: ::windows::core::IUnknownImpl, Impl: IEnumDot11AdHocNetworks_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32, rgelt: *mut ::windows::core::RawPtr, pceltfetched: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Next(::core::mem::transmute_copy(&celt), ::core::mem::transmute_copy(&rgelt), ::core::mem::transmute_copy(&pceltfetched)).into()
        }
        unsafe extern "system" fn Skip<Identity: ::windows::core::IUnknownImpl, Impl: IEnumDot11AdHocNetworks_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Skip(::core::mem::transmute_copy(&celt)).into()
        }
        unsafe extern "system" fn Reset<Identity: ::windows::core::IUnknownImpl, Impl: IEnumDot11AdHocNetworks_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Reset().into()
        }
        unsafe extern "system" fn Clone<Identity: ::windows::core::IUnknownImpl, Impl: IEnumDot11AdHocNetworks_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Clone() {
                ::core::result::Result::Ok(ok__) => {
                    *ppenum = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            Next: Next::<Identity, Impl, OFFSET>,
            Skip: Skip::<Identity, Impl, OFFSET>,
            Reset: Reset::<Identity, Impl, OFFSET>,
            Clone: Clone::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IEnumDot11AdHocNetworks as ::windows::core::Interface>::IID
    }
}
pub trait IEnumDot11AdHocSecuritySettings_Impl: Sized {
    fn Next(&mut self, celt: u32, rgelt: *mut ::core::option::Option<IDot11AdHocSecuritySettings>, pceltfetched: *mut u32) -> ::windows::core::Result<()>;
    fn Skip(&mut self, celt: u32) -> ::windows::core::Result<()>;
    fn Reset(&mut self) -> ::windows::core::Result<()>;
    fn Clone(&mut self) -> ::windows::core::Result<IEnumDot11AdHocSecuritySettings>;
}
impl IEnumDot11AdHocSecuritySettings_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IEnumDot11AdHocSecuritySettings_Impl, const OFFSET: isize>() -> IEnumDot11AdHocSecuritySettings_Vtbl {
        unsafe extern "system" fn Next<Identity: ::windows::core::IUnknownImpl, Impl: IEnumDot11AdHocSecuritySettings_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32, rgelt: *mut ::windows::core::RawPtr, pceltfetched: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Next(::core::mem::transmute_copy(&celt), ::core::mem::transmute_copy(&rgelt), ::core::mem::transmute_copy(&pceltfetched)).into()
        }
        unsafe extern "system" fn Skip<Identity: ::windows::core::IUnknownImpl, Impl: IEnumDot11AdHocSecuritySettings_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Skip(::core::mem::transmute_copy(&celt)).into()
        }
        unsafe extern "system" fn Reset<Identity: ::windows::core::IUnknownImpl, Impl: IEnumDot11AdHocSecuritySettings_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Reset().into()
        }
        unsafe extern "system" fn Clone<Identity: ::windows::core::IUnknownImpl, Impl: IEnumDot11AdHocSecuritySettings_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Clone() {
                ::core::result::Result::Ok(ok__) => {
                    *ppenum = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            Next: Next::<Identity, Impl, OFFSET>,
            Skip: Skip::<Identity, Impl, OFFSET>,
            Reset: Reset::<Identity, Impl, OFFSET>,
            Clone: Clone::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IEnumDot11AdHocSecuritySettings as ::windows::core::Interface>::IID
    }
}
