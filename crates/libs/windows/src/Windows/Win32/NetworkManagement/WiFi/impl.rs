#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`, `\"implement\"`*"]
pub trait IDot11AdHocInterface_Impl: Sized {
    fn GetDeviceSignature(&self, psignature: *mut ::windows::core::GUID) -> ::windows::core::Result<()>;
    fn GetFriendlyName(&self) -> ::windows::core::Result<::windows::core::PWSTR>;
    fn IsDot11d(&self, pf11d: *mut u8) -> ::windows::core::Result<()>;
    fn IsAdHocCapable(&self, pfadhoccapable: *mut u8) -> ::windows::core::Result<()>;
    fn IsRadioOn(&self, pfisradioon: *mut u8) -> ::windows::core::Result<()>;
    fn GetActiveNetwork(&self) -> ::windows::core::Result<IDot11AdHocNetwork>;
    fn GetIEnumSecuritySettings(&self) -> ::windows::core::Result<IEnumDot11AdHocSecuritySettings>;
    fn GetIEnumDot11AdHocNetworks(&self, pfilterguid: *const ::windows::core::GUID) -> ::windows::core::Result<IEnumDot11AdHocNetworks>;
    fn GetStatus(&self, pstate: *mut DOT11_ADHOC_NETWORK_CONNECTION_STATUS) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IDot11AdHocInterface {}
impl IDot11AdHocInterface_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDot11AdHocInterface_Impl, const OFFSET: isize>() -> IDot11AdHocInterface_Vtbl {
        unsafe extern "system" fn GetDeviceSignature<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDot11AdHocInterface_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psignature: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetDeviceSignature(::core::mem::transmute_copy(&psignature)).into()
        }
        unsafe extern "system" fn GetFriendlyName<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDot11AdHocInterface_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppszname: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetFriendlyName() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppszname, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsDot11d<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDot11AdHocInterface_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pf11d: *mut u8) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.IsDot11d(::core::mem::transmute_copy(&pf11d)).into()
        }
        unsafe extern "system" fn IsAdHocCapable<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDot11AdHocInterface_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfadhoccapable: *mut u8) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.IsAdHocCapable(::core::mem::transmute_copy(&pfadhoccapable)).into()
        }
        unsafe extern "system" fn IsRadioOn<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDot11AdHocInterface_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfisradioon: *mut u8) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.IsRadioOn(::core::mem::transmute_copy(&pfisradioon)).into()
        }
        unsafe extern "system" fn GetActiveNetwork<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDot11AdHocInterface_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppnetwork: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetActiveNetwork() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppnetwork, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetIEnumSecuritySettings<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDot11AdHocInterface_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetIEnumSecuritySettings() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppenum, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetIEnumDot11AdHocNetworks<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDot11AdHocInterface_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfilterguid: *const ::windows::core::GUID, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetIEnumDot11AdHocNetworks(::core::mem::transmute_copy(&pfilterguid)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppenum, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetStatus<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDot11AdHocInterface_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pstate: *mut DOT11_ADHOC_NETWORK_CONNECTION_STATUS) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetStatus(::core::mem::transmute_copy(&pstate)).into()
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
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
        iid == &<IDot11AdHocInterface as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`, `\"implement\"`*"]
pub trait IDot11AdHocInterfaceNotificationSink_Impl: Sized {
    fn OnConnectionStatusChange(&self, estatus: DOT11_ADHOC_NETWORK_CONNECTION_STATUS) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IDot11AdHocInterfaceNotificationSink {}
impl IDot11AdHocInterfaceNotificationSink_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDot11AdHocInterfaceNotificationSink_Impl, const OFFSET: isize>() -> IDot11AdHocInterfaceNotificationSink_Vtbl {
        unsafe extern "system" fn OnConnectionStatusChange<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDot11AdHocInterfaceNotificationSink_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, estatus: DOT11_ADHOC_NETWORK_CONNECTION_STATUS) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnConnectionStatusChange(::core::mem::transmute_copy(&estatus)).into()
        }
        Self { base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(), OnConnectionStatusChange: OnConnectionStatusChange::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDot11AdHocInterfaceNotificationSink as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`, `\"Win32_Foundation\"`, `\"implement\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub trait IDot11AdHocManager_Impl: Sized {
    fn CreateNetwork(&self, name: &::windows::core::PCWSTR, password: &::windows::core::PCWSTR, geographicalid: i32, pinterface: ::core::option::Option<&IDot11AdHocInterface>, psecurity: ::core::option::Option<&IDot11AdHocSecuritySettings>, pcontextguid: *const ::windows::core::GUID) -> ::windows::core::Result<IDot11AdHocNetwork>;
    fn CommitCreatedNetwork(&self, piadhoc: ::core::option::Option<&IDot11AdHocNetwork>, fsaveprofile: super::super::Foundation::BOOLEAN, fmakesavedprofileuserspecific: super::super::Foundation::BOOLEAN) -> ::windows::core::Result<()>;
    fn GetIEnumDot11AdHocNetworks(&self, pcontextguid: *const ::windows::core::GUID) -> ::windows::core::Result<IEnumDot11AdHocNetworks>;
    fn GetIEnumDot11AdHocInterfaces(&self) -> ::windows::core::Result<IEnumDot11AdHocInterfaces>;
    fn GetNetwork(&self, networksignature: *const ::windows::core::GUID) -> ::windows::core::Result<IDot11AdHocNetwork>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::RuntimeName for IDot11AdHocManager {}
#[cfg(feature = "Win32_Foundation")]
impl IDot11AdHocManager_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDot11AdHocManager_Impl, const OFFSET: isize>() -> IDot11AdHocManager_Vtbl {
        unsafe extern "system" fn CreateNetwork<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDot11AdHocManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::windows::core::PCWSTR, password: ::windows::core::PCWSTR, geographicalid: i32, pinterface: *mut ::core::ffi::c_void, psecurity: *mut ::core::ffi::c_void, pcontextguid: *const ::windows::core::GUID, piadhoc: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CreateNetwork(::core::mem::transmute(&name), ::core::mem::transmute(&password), ::core::mem::transmute_copy(&geographicalid), ::windows::core::from_raw_borrowed(&pinterface), ::windows::core::from_raw_borrowed(&psecurity), ::core::mem::transmute_copy(&pcontextguid)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(piadhoc, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CommitCreatedNetwork<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDot11AdHocManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, piadhoc: *mut ::core::ffi::c_void, fsaveprofile: super::super::Foundation::BOOLEAN, fmakesavedprofileuserspecific: super::super::Foundation::BOOLEAN) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.CommitCreatedNetwork(::windows::core::from_raw_borrowed(&piadhoc), ::core::mem::transmute_copy(&fsaveprofile), ::core::mem::transmute_copy(&fmakesavedprofileuserspecific)).into()
        }
        unsafe extern "system" fn GetIEnumDot11AdHocNetworks<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDot11AdHocManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcontextguid: *const ::windows::core::GUID, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetIEnumDot11AdHocNetworks(::core::mem::transmute_copy(&pcontextguid)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppenum, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetIEnumDot11AdHocInterfaces<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDot11AdHocManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetIEnumDot11AdHocInterfaces() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppenum, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetNetwork<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDot11AdHocManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, networksignature: *const ::windows::core::GUID, pnetwork: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetNetwork(::core::mem::transmute_copy(&networksignature)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pnetwork, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            CreateNetwork: CreateNetwork::<Identity, Impl, OFFSET>,
            CommitCreatedNetwork: CommitCreatedNetwork::<Identity, Impl, OFFSET>,
            GetIEnumDot11AdHocNetworks: GetIEnumDot11AdHocNetworks::<Identity, Impl, OFFSET>,
            GetIEnumDot11AdHocInterfaces: GetIEnumDot11AdHocInterfaces::<Identity, Impl, OFFSET>,
            GetNetwork: GetNetwork::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDot11AdHocManager as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`, `\"implement\"`*"]
pub trait IDot11AdHocManagerNotificationSink_Impl: Sized {
    fn OnNetworkAdd(&self, piadhocnetwork: ::core::option::Option<&IDot11AdHocNetwork>) -> ::windows::core::Result<()>;
    fn OnNetworkRemove(&self, signature: *const ::windows::core::GUID) -> ::windows::core::Result<()>;
    fn OnInterfaceAdd(&self, piadhocinterface: ::core::option::Option<&IDot11AdHocInterface>) -> ::windows::core::Result<()>;
    fn OnInterfaceRemove(&self, signature: *const ::windows::core::GUID) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IDot11AdHocManagerNotificationSink {}
impl IDot11AdHocManagerNotificationSink_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDot11AdHocManagerNotificationSink_Impl, const OFFSET: isize>() -> IDot11AdHocManagerNotificationSink_Vtbl {
        unsafe extern "system" fn OnNetworkAdd<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDot11AdHocManagerNotificationSink_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, piadhocnetwork: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnNetworkAdd(::windows::core::from_raw_borrowed(&piadhocnetwork)).into()
        }
        unsafe extern "system" fn OnNetworkRemove<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDot11AdHocManagerNotificationSink_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, signature: *const ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnNetworkRemove(::core::mem::transmute_copy(&signature)).into()
        }
        unsafe extern "system" fn OnInterfaceAdd<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDot11AdHocManagerNotificationSink_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, piadhocinterface: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnInterfaceAdd(::windows::core::from_raw_borrowed(&piadhocinterface)).into()
        }
        unsafe extern "system" fn OnInterfaceRemove<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDot11AdHocManagerNotificationSink_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, signature: *const ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnInterfaceRemove(::core::mem::transmute_copy(&signature)).into()
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            OnNetworkAdd: OnNetworkAdd::<Identity, Impl, OFFSET>,
            OnNetworkRemove: OnNetworkRemove::<Identity, Impl, OFFSET>,
            OnInterfaceAdd: OnInterfaceAdd::<Identity, Impl, OFFSET>,
            OnInterfaceRemove: OnInterfaceRemove::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDot11AdHocManagerNotificationSink as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`, `\"Win32_Foundation\"`, `\"implement\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub trait IDot11AdHocNetwork_Impl: Sized {
    fn GetStatus(&self, estatus: *mut DOT11_ADHOC_NETWORK_CONNECTION_STATUS) -> ::windows::core::Result<()>;
    fn GetSSID(&self) -> ::windows::core::Result<::windows::core::PWSTR>;
    fn HasProfile(&self, pf11d: *mut u8) -> ::windows::core::Result<()>;
    fn GetProfileName(&self) -> ::windows::core::Result<::windows::core::PWSTR>;
    fn DeleteProfile(&self) -> ::windows::core::Result<()>;
    fn GetSignalQuality(&self, pustrengthvalue: *mut u32, pustrengthmax: *mut u32) -> ::windows::core::Result<()>;
    fn GetSecuritySetting(&self) -> ::windows::core::Result<IDot11AdHocSecuritySettings>;
    fn GetContextGuid(&self, pcontextguid: *mut ::windows::core::GUID) -> ::windows::core::Result<()>;
    fn GetSignature(&self, psignature: *mut ::windows::core::GUID) -> ::windows::core::Result<()>;
    fn GetInterface(&self) -> ::windows::core::Result<IDot11AdHocInterface>;
    fn Connect(&self, passphrase: &::windows::core::PCWSTR, geographicalid: i32, fsaveprofile: super::super::Foundation::BOOLEAN, fmakesavedprofileuserspecific: super::super::Foundation::BOOLEAN) -> ::windows::core::Result<()>;
    fn Disconnect(&self) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::RuntimeName for IDot11AdHocNetwork {}
#[cfg(feature = "Win32_Foundation")]
impl IDot11AdHocNetwork_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDot11AdHocNetwork_Impl, const OFFSET: isize>() -> IDot11AdHocNetwork_Vtbl {
        unsafe extern "system" fn GetStatus<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDot11AdHocNetwork_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, estatus: *mut DOT11_ADHOC_NETWORK_CONNECTION_STATUS) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetStatus(::core::mem::transmute_copy(&estatus)).into()
        }
        unsafe extern "system" fn GetSSID<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDot11AdHocNetwork_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppszwssid: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetSSID() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppszwssid, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn HasProfile<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDot11AdHocNetwork_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pf11d: *mut u8) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.HasProfile(::core::mem::transmute_copy(&pf11d)).into()
        }
        unsafe extern "system" fn GetProfileName<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDot11AdHocNetwork_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppszwprofilename: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetProfileName() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppszwprofilename, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DeleteProfile<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDot11AdHocNetwork_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.DeleteProfile().into()
        }
        unsafe extern "system" fn GetSignalQuality<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDot11AdHocNetwork_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pustrengthvalue: *mut u32, pustrengthmax: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetSignalQuality(::core::mem::transmute_copy(&pustrengthvalue), ::core::mem::transmute_copy(&pustrengthmax)).into()
        }
        unsafe extern "system" fn GetSecuritySetting<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDot11AdHocNetwork_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, padhocsecuritysetting: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetSecuritySetting() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(padhocsecuritysetting, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetContextGuid<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDot11AdHocNetwork_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcontextguid: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetContextGuid(::core::mem::transmute_copy(&pcontextguid)).into()
        }
        unsafe extern "system" fn GetSignature<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDot11AdHocNetwork_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psignature: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetSignature(::core::mem::transmute_copy(&psignature)).into()
        }
        unsafe extern "system" fn GetInterface<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDot11AdHocNetwork_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, padhocinterface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetInterface() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(padhocinterface, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Connect<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDot11AdHocNetwork_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, passphrase: ::windows::core::PCWSTR, geographicalid: i32, fsaveprofile: super::super::Foundation::BOOLEAN, fmakesavedprofileuserspecific: super::super::Foundation::BOOLEAN) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Connect(::core::mem::transmute(&passphrase), ::core::mem::transmute_copy(&geographicalid), ::core::mem::transmute_copy(&fsaveprofile), ::core::mem::transmute_copy(&fmakesavedprofileuserspecific)).into()
        }
        unsafe extern "system" fn Disconnect<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDot11AdHocNetwork_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Disconnect().into()
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
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
        iid == &<IDot11AdHocNetwork as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`, `\"implement\"`*"]
pub trait IDot11AdHocNetworkNotificationSink_Impl: Sized {
    fn OnStatusChange(&self, estatus: DOT11_ADHOC_NETWORK_CONNECTION_STATUS) -> ::windows::core::Result<()>;
    fn OnConnectFail(&self, efailreason: DOT11_ADHOC_CONNECT_FAIL_REASON) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IDot11AdHocNetworkNotificationSink {}
impl IDot11AdHocNetworkNotificationSink_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDot11AdHocNetworkNotificationSink_Impl, const OFFSET: isize>() -> IDot11AdHocNetworkNotificationSink_Vtbl {
        unsafe extern "system" fn OnStatusChange<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDot11AdHocNetworkNotificationSink_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, estatus: DOT11_ADHOC_NETWORK_CONNECTION_STATUS) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnStatusChange(::core::mem::transmute_copy(&estatus)).into()
        }
        unsafe extern "system" fn OnConnectFail<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDot11AdHocNetworkNotificationSink_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, efailreason: DOT11_ADHOC_CONNECT_FAIL_REASON) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnConnectFail(::core::mem::transmute_copy(&efailreason)).into()
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            OnStatusChange: OnStatusChange::<Identity, Impl, OFFSET>,
            OnConnectFail: OnConnectFail::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDot11AdHocNetworkNotificationSink as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`, `\"implement\"`*"]
pub trait IDot11AdHocSecuritySettings_Impl: Sized {
    fn GetDot11AuthAlgorithm(&self, pauth: *mut DOT11_ADHOC_AUTH_ALGORITHM) -> ::windows::core::Result<()>;
    fn GetDot11CipherAlgorithm(&self, pcipher: *mut DOT11_ADHOC_CIPHER_ALGORITHM) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IDot11AdHocSecuritySettings {}
impl IDot11AdHocSecuritySettings_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDot11AdHocSecuritySettings_Impl, const OFFSET: isize>() -> IDot11AdHocSecuritySettings_Vtbl {
        unsafe extern "system" fn GetDot11AuthAlgorithm<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDot11AdHocSecuritySettings_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pauth: *mut DOT11_ADHOC_AUTH_ALGORITHM) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetDot11AuthAlgorithm(::core::mem::transmute_copy(&pauth)).into()
        }
        unsafe extern "system" fn GetDot11CipherAlgorithm<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDot11AdHocSecuritySettings_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcipher: *mut DOT11_ADHOC_CIPHER_ALGORITHM) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetDot11CipherAlgorithm(::core::mem::transmute_copy(&pcipher)).into()
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetDot11AuthAlgorithm: GetDot11AuthAlgorithm::<Identity, Impl, OFFSET>,
            GetDot11CipherAlgorithm: GetDot11CipherAlgorithm::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDot11AdHocSecuritySettings as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`, `\"implement\"`*"]
pub trait IEnumDot11AdHocInterfaces_Impl: Sized {
    fn Next(&self, celt: u32, rgelt: *mut ::core::option::Option<IDot11AdHocInterface>, pceltfetched: *mut u32) -> ::windows::core::Result<()>;
    fn Skip(&self, celt: u32) -> ::windows::core::Result<()>;
    fn Reset(&self) -> ::windows::core::Result<()>;
    fn Clone(&self) -> ::windows::core::Result<IEnumDot11AdHocInterfaces>;
}
impl ::windows::core::RuntimeName for IEnumDot11AdHocInterfaces {}
impl IEnumDot11AdHocInterfaces_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IEnumDot11AdHocInterfaces_Impl, const OFFSET: isize>() -> IEnumDot11AdHocInterfaces_Vtbl {
        unsafe extern "system" fn Next<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IEnumDot11AdHocInterfaces_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32, rgelt: *mut *mut ::core::ffi::c_void, pceltfetched: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Next(::core::mem::transmute_copy(&celt), ::core::mem::transmute_copy(&rgelt), ::core::mem::transmute_copy(&pceltfetched)).into()
        }
        unsafe extern "system" fn Skip<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IEnumDot11AdHocInterfaces_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Skip(::core::mem::transmute_copy(&celt)).into()
        }
        unsafe extern "system" fn Reset<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IEnumDot11AdHocInterfaces_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Reset().into()
        }
        unsafe extern "system" fn Clone<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IEnumDot11AdHocInterfaces_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Clone() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppenum, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Next: Next::<Identity, Impl, OFFSET>,
            Skip: Skip::<Identity, Impl, OFFSET>,
            Reset: Reset::<Identity, Impl, OFFSET>,
            Clone: Clone::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IEnumDot11AdHocInterfaces as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`, `\"implement\"`*"]
pub trait IEnumDot11AdHocNetworks_Impl: Sized {
    fn Next(&self, celt: u32, rgelt: *mut ::core::option::Option<IDot11AdHocNetwork>, pceltfetched: *mut u32) -> ::windows::core::Result<()>;
    fn Skip(&self, celt: u32) -> ::windows::core::Result<()>;
    fn Reset(&self) -> ::windows::core::Result<()>;
    fn Clone(&self) -> ::windows::core::Result<IEnumDot11AdHocNetworks>;
}
impl ::windows::core::RuntimeName for IEnumDot11AdHocNetworks {}
impl IEnumDot11AdHocNetworks_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IEnumDot11AdHocNetworks_Impl, const OFFSET: isize>() -> IEnumDot11AdHocNetworks_Vtbl {
        unsafe extern "system" fn Next<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IEnumDot11AdHocNetworks_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32, rgelt: *mut *mut ::core::ffi::c_void, pceltfetched: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Next(::core::mem::transmute_copy(&celt), ::core::mem::transmute_copy(&rgelt), ::core::mem::transmute_copy(&pceltfetched)).into()
        }
        unsafe extern "system" fn Skip<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IEnumDot11AdHocNetworks_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Skip(::core::mem::transmute_copy(&celt)).into()
        }
        unsafe extern "system" fn Reset<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IEnumDot11AdHocNetworks_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Reset().into()
        }
        unsafe extern "system" fn Clone<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IEnumDot11AdHocNetworks_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Clone() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppenum, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Next: Next::<Identity, Impl, OFFSET>,
            Skip: Skip::<Identity, Impl, OFFSET>,
            Reset: Reset::<Identity, Impl, OFFSET>,
            Clone: Clone::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IEnumDot11AdHocNetworks as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`, `\"implement\"`*"]
pub trait IEnumDot11AdHocSecuritySettings_Impl: Sized {
    fn Next(&self, celt: u32, rgelt: *mut ::core::option::Option<IDot11AdHocSecuritySettings>, pceltfetched: *mut u32) -> ::windows::core::Result<()>;
    fn Skip(&self, celt: u32) -> ::windows::core::Result<()>;
    fn Reset(&self) -> ::windows::core::Result<()>;
    fn Clone(&self) -> ::windows::core::Result<IEnumDot11AdHocSecuritySettings>;
}
impl ::windows::core::RuntimeName for IEnumDot11AdHocSecuritySettings {}
impl IEnumDot11AdHocSecuritySettings_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IEnumDot11AdHocSecuritySettings_Impl, const OFFSET: isize>() -> IEnumDot11AdHocSecuritySettings_Vtbl {
        unsafe extern "system" fn Next<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IEnumDot11AdHocSecuritySettings_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32, rgelt: *mut *mut ::core::ffi::c_void, pceltfetched: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Next(::core::mem::transmute_copy(&celt), ::core::mem::transmute_copy(&rgelt), ::core::mem::transmute_copy(&pceltfetched)).into()
        }
        unsafe extern "system" fn Skip<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IEnumDot11AdHocSecuritySettings_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Skip(::core::mem::transmute_copy(&celt)).into()
        }
        unsafe extern "system" fn Reset<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IEnumDot11AdHocSecuritySettings_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Reset().into()
        }
        unsafe extern "system" fn Clone<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IEnumDot11AdHocSecuritySettings_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Clone() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppenum, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Next: Next::<Identity, Impl, OFFSET>,
            Skip: Skip::<Identity, Impl, OFFSET>,
            Reset: Reset::<Identity, Impl, OFFSET>,
            Clone: Clone::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IEnumDot11AdHocSecuritySettings as ::windows::core::ComInterface>::IID
    }
}
