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
impl ::windows::core::RuntimeName for IDot11AdHocInterface {
    const NAME: &'static str = "Windows.Win32.NetworkManagement.WiFi.IDot11AdHocInterface";
}
impl IDot11AdHocInterfaceVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDot11AdHocInterfaceImpl, const OFFSET: isize>() -> IDot11AdHocInterfaceVtbl {
        unsafe extern "system" fn GetDeviceSignature<Impl: IDot11AdHocInterfaceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psignature: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDeviceSignature(&*(&psignature as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFriendlyName<Impl: IDot11AdHocInterfaceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppszname: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetFriendlyName(::core::mem::transmute_copy(&ppszname)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsDot11d<Impl: IDot11AdHocInterfaceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pf11d: *mut u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsDot11d(pf11d) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsAdHocCapable<Impl: IDot11AdHocInterfaceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfadhoccapable: *mut u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsAdHocCapable(pfadhoccapable) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsRadioOn<Impl: IDot11AdHocInterfaceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfisradioon: *mut u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsRadioOn(pfisradioon) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetActiveNetwork<Impl: IDot11AdHocInterfaceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppnetwork: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetActiveNetwork(::core::mem::transmute_copy(&ppnetwork)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetIEnumSecuritySettings<Impl: IDot11AdHocInterfaceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetIEnumSecuritySettings(::core::mem::transmute_copy(&ppenum)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetIEnumDot11AdHocNetworks<Impl: IDot11AdHocInterfaceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfilterguid: *const ::windows::core::GUID, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetIEnumDot11AdHocNetworks(&*(&pfilterguid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&ppenum)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetStatus<Impl: IDot11AdHocInterfaceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pstate: *mut DOT11_ADHOC_NETWORK_CONNECTION_STATUS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetStatus(pstate) {
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
            ::windows::core::GetRuntimeClassName::<IDot11AdHocInterface>,
            ::windows::core::GetTrustLevel,
            GetDeviceSignature::<Impl, OFFSET>,
            GetFriendlyName::<Impl, OFFSET>,
            IsDot11d::<Impl, OFFSET>,
            IsAdHocCapable::<Impl, OFFSET>,
            IsRadioOn::<Impl, OFFSET>,
            GetActiveNetwork::<Impl, OFFSET>,
            GetIEnumSecuritySettings::<Impl, OFFSET>,
            GetIEnumDot11AdHocNetworks::<Impl, OFFSET>,
            GetStatus::<Impl, OFFSET>,
        )
    }
}
pub trait IDot11AdHocInterfaceNotificationSinkImpl: Sized {
    fn OnConnectionStatusChange();
}
impl ::windows::core::RuntimeName for IDot11AdHocInterfaceNotificationSink {
    const NAME: &'static str = "Windows.Win32.NetworkManagement.WiFi.IDot11AdHocInterfaceNotificationSink";
}
impl IDot11AdHocInterfaceNotificationSinkVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDot11AdHocInterfaceNotificationSinkImpl, const OFFSET: isize>() -> IDot11AdHocInterfaceNotificationSinkVtbl {
        unsafe extern "system" fn OnConnectionStatusChange<Impl: IDot11AdHocInterfaceNotificationSinkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, estatus: DOT11_ADHOC_NETWORK_CONNECTION_STATUS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OnConnectionStatusChange(estatus) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IDot11AdHocInterfaceNotificationSink>, ::windows::core::GetTrustLevel, OnConnectionStatusChange::<Impl, OFFSET>)
    }
}
pub trait IDot11AdHocManagerImpl: Sized {
    fn CreateNetwork();
    fn CommitCreatedNetwork();
    fn GetIEnumDot11AdHocNetworks();
    fn GetIEnumDot11AdHocInterfaces();
    fn GetNetwork();
}
impl ::windows::core::RuntimeName for IDot11AdHocManager {
    const NAME: &'static str = "Windows.Win32.NetworkManagement.WiFi.IDot11AdHocManager";
}
impl IDot11AdHocManagerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDot11AdHocManagerImpl, const OFFSET: isize>() -> IDot11AdHocManagerVtbl {
        unsafe extern "system" fn CreateNetwork<Impl: IDot11AdHocManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: super::super::Foundation::PWSTR, password: super::super::Foundation::PWSTR, geographicalid: i32, pinterface: ::windows::core::RawPtr, psecurity: ::windows::core::RawPtr, pcontextguid: *const ::windows::core::GUID, piadhoc: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateNetwork(
                &*(&name as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&password as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType),
                geographicalid,
                &*(&pinterface as *const <IDot11AdHocInterface as ::windows::core::Abi>::Abi as *const <IDot11AdHocInterface as ::windows::core::DefaultType>::DefaultType),
                &*(&psecurity as *const <IDot11AdHocSecuritySettings as ::windows::core::Abi>::Abi as *const <IDot11AdHocSecuritySettings as ::windows::core::DefaultType>::DefaultType),
                &*(&pcontextguid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType),
                ::core::mem::transmute_copy(&piadhoc),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CommitCreatedNetwork<Impl: IDot11AdHocManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, piadhoc: ::windows::core::RawPtr, fsaveprofile: super::super::Foundation::BOOLEAN, fmakesavedprofileuserspecific: super::super::Foundation::BOOLEAN) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CommitCreatedNetwork(
                &*(&piadhoc as *const <IDot11AdHocNetwork as ::windows::core::Abi>::Abi as *const <IDot11AdHocNetwork as ::windows::core::DefaultType>::DefaultType),
                &*(&fsaveprofile as *const <super::super::Foundation::BOOLEAN as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BOOLEAN as ::windows::core::DefaultType>::DefaultType),
                &*(&fmakesavedprofileuserspecific as *const <super::super::Foundation::BOOLEAN as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BOOLEAN as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetIEnumDot11AdHocNetworks<Impl: IDot11AdHocManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcontextguid: *const ::windows::core::GUID, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetIEnumDot11AdHocNetworks(&*(&pcontextguid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&ppenum)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetIEnumDot11AdHocInterfaces<Impl: IDot11AdHocManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetIEnumDot11AdHocInterfaces(::core::mem::transmute_copy(&ppenum)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetNetwork<Impl: IDot11AdHocManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, networksignature: *const ::windows::core::GUID, pnetwork: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetNetwork(&*(&networksignature as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&pnetwork)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IDot11AdHocManager>, ::windows::core::GetTrustLevel, CreateNetwork::<Impl, OFFSET>, CommitCreatedNetwork::<Impl, OFFSET>, GetIEnumDot11AdHocNetworks::<Impl, OFFSET>, GetIEnumDot11AdHocInterfaces::<Impl, OFFSET>, GetNetwork::<Impl, OFFSET>)
    }
}
pub trait IDot11AdHocManagerNotificationSinkImpl: Sized {
    fn OnNetworkAdd();
    fn OnNetworkRemove();
    fn OnInterfaceAdd();
    fn OnInterfaceRemove();
}
impl ::windows::core::RuntimeName for IDot11AdHocManagerNotificationSink {
    const NAME: &'static str = "Windows.Win32.NetworkManagement.WiFi.IDot11AdHocManagerNotificationSink";
}
impl IDot11AdHocManagerNotificationSinkVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDot11AdHocManagerNotificationSinkImpl, const OFFSET: isize>() -> IDot11AdHocManagerNotificationSinkVtbl {
        unsafe extern "system" fn OnNetworkAdd<Impl: IDot11AdHocManagerNotificationSinkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, piadhocnetwork: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OnNetworkAdd(&*(&piadhocnetwork as *const <IDot11AdHocNetwork as ::windows::core::Abi>::Abi as *const <IDot11AdHocNetwork as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OnNetworkRemove<Impl: IDot11AdHocManagerNotificationSinkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, signature: *const ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OnNetworkRemove(&*(&signature as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OnInterfaceAdd<Impl: IDot11AdHocManagerNotificationSinkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, piadhocinterface: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OnInterfaceAdd(&*(&piadhocinterface as *const <IDot11AdHocInterface as ::windows::core::Abi>::Abi as *const <IDot11AdHocInterface as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OnInterfaceRemove<Impl: IDot11AdHocManagerNotificationSinkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, signature: *const ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OnInterfaceRemove(&*(&signature as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IDot11AdHocManagerNotificationSink>, ::windows::core::GetTrustLevel, OnNetworkAdd::<Impl, OFFSET>, OnNetworkRemove::<Impl, OFFSET>, OnInterfaceAdd::<Impl, OFFSET>, OnInterfaceRemove::<Impl, OFFSET>)
    }
}
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
impl ::windows::core::RuntimeName for IDot11AdHocNetwork {
    const NAME: &'static str = "Windows.Win32.NetworkManagement.WiFi.IDot11AdHocNetwork";
}
impl IDot11AdHocNetworkVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDot11AdHocNetworkImpl, const OFFSET: isize>() -> IDot11AdHocNetworkVtbl {
        unsafe extern "system" fn GetStatus<Impl: IDot11AdHocNetworkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, estatus: *mut DOT11_ADHOC_NETWORK_CONNECTION_STATUS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetStatus(estatus) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSSID<Impl: IDot11AdHocNetworkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppszwssid: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetSSID(::core::mem::transmute_copy(&ppszwssid)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn HasProfile<Impl: IDot11AdHocNetworkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pf11d: *mut u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).HasProfile(pf11d) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetProfileName<Impl: IDot11AdHocNetworkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppszwprofilename: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetProfileName(::core::mem::transmute_copy(&ppszwprofilename)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DeleteProfile<Impl: IDot11AdHocNetworkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DeleteProfile() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSignalQuality<Impl: IDot11AdHocNetworkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pustrengthvalue: *mut u32, pustrengthmax: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetSignalQuality(::core::mem::transmute_copy(&pustrengthvalue), ::core::mem::transmute_copy(&pustrengthmax)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSecuritySetting<Impl: IDot11AdHocNetworkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, padhocsecuritysetting: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetSecuritySetting(::core::mem::transmute_copy(&padhocsecuritysetting)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetContextGuid<Impl: IDot11AdHocNetworkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcontextguid: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetContextGuid(&*(&pcontextguid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSignature<Impl: IDot11AdHocNetworkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psignature: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetSignature(&*(&psignature as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetInterface<Impl: IDot11AdHocNetworkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, padhocinterface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetInterface(::core::mem::transmute_copy(&padhocinterface)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Connect<Impl: IDot11AdHocNetworkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, passphrase: super::super::Foundation::PWSTR, geographicalid: i32, fsaveprofile: super::super::Foundation::BOOLEAN, fmakesavedprofileuserspecific: super::super::Foundation::BOOLEAN) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Connect(
                &*(&passphrase as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType),
                geographicalid,
                &*(&fsaveprofile as *const <super::super::Foundation::BOOLEAN as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BOOLEAN as ::windows::core::DefaultType>::DefaultType),
                &*(&fmakesavedprofileuserspecific as *const <super::super::Foundation::BOOLEAN as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BOOLEAN as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Disconnect<Impl: IDot11AdHocNetworkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
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
        Self(
            ::windows::core::QueryInterface::<Identity, OFFSET>,
            ::windows::core::AddRef::<Identity, OFFSET>,
            ::windows::core::Release::<Identity, OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<IDot11AdHocNetwork>,
            ::windows::core::GetTrustLevel,
            GetStatus::<Impl, OFFSET>,
            GetSSID::<Impl, OFFSET>,
            HasProfile::<Impl, OFFSET>,
            GetProfileName::<Impl, OFFSET>,
            DeleteProfile::<Impl, OFFSET>,
            GetSignalQuality::<Impl, OFFSET>,
            GetSecuritySetting::<Impl, OFFSET>,
            GetContextGuid::<Impl, OFFSET>,
            GetSignature::<Impl, OFFSET>,
            GetInterface::<Impl, OFFSET>,
            Connect::<Impl, OFFSET>,
            Disconnect::<Impl, OFFSET>,
        )
    }
}
pub trait IDot11AdHocNetworkNotificationSinkImpl: Sized {
    fn OnStatusChange();
    fn OnConnectFail();
}
impl ::windows::core::RuntimeName for IDot11AdHocNetworkNotificationSink {
    const NAME: &'static str = "Windows.Win32.NetworkManagement.WiFi.IDot11AdHocNetworkNotificationSink";
}
impl IDot11AdHocNetworkNotificationSinkVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDot11AdHocNetworkNotificationSinkImpl, const OFFSET: isize>() -> IDot11AdHocNetworkNotificationSinkVtbl {
        unsafe extern "system" fn OnStatusChange<Impl: IDot11AdHocNetworkNotificationSinkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, estatus: DOT11_ADHOC_NETWORK_CONNECTION_STATUS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OnStatusChange(estatus) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OnConnectFail<Impl: IDot11AdHocNetworkNotificationSinkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, efailreason: DOT11_ADHOC_CONNECT_FAIL_REASON) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OnConnectFail(efailreason) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IDot11AdHocNetworkNotificationSink>, ::windows::core::GetTrustLevel, OnStatusChange::<Impl, OFFSET>, OnConnectFail::<Impl, OFFSET>)
    }
}
pub trait IDot11AdHocSecuritySettingsImpl: Sized {
    fn GetDot11AuthAlgorithm();
    fn GetDot11CipherAlgorithm();
}
impl ::windows::core::RuntimeName for IDot11AdHocSecuritySettings {
    const NAME: &'static str = "Windows.Win32.NetworkManagement.WiFi.IDot11AdHocSecuritySettings";
}
impl IDot11AdHocSecuritySettingsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDot11AdHocSecuritySettingsImpl, const OFFSET: isize>() -> IDot11AdHocSecuritySettingsVtbl {
        unsafe extern "system" fn GetDot11AuthAlgorithm<Impl: IDot11AdHocSecuritySettingsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pauth: *mut DOT11_ADHOC_AUTH_ALGORITHM) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDot11AuthAlgorithm(pauth) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDot11CipherAlgorithm<Impl: IDot11AdHocSecuritySettingsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcipher: *mut DOT11_ADHOC_CIPHER_ALGORITHM) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDot11CipherAlgorithm(pcipher) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IDot11AdHocSecuritySettings>, ::windows::core::GetTrustLevel, GetDot11AuthAlgorithm::<Impl, OFFSET>, GetDot11CipherAlgorithm::<Impl, OFFSET>)
    }
}
pub trait IEnumDot11AdHocInterfacesImpl: Sized {
    fn Next();
    fn Skip();
    fn Reset();
    fn Clone();
}
impl ::windows::core::RuntimeName for IEnumDot11AdHocInterfaces {
    const NAME: &'static str = "Windows.Win32.NetworkManagement.WiFi.IEnumDot11AdHocInterfaces";
}
impl IEnumDot11AdHocInterfacesVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IEnumDot11AdHocInterfacesImpl, const OFFSET: isize>() -> IEnumDot11AdHocInterfacesVtbl {
        unsafe extern "system" fn Next<Impl: IEnumDot11AdHocInterfacesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32, rgelt: *mut ::windows::core::RawPtr, pceltfetched: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Next(celt, ::core::mem::transmute_copy(&rgelt), ::core::mem::transmute_copy(&pceltfetched)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Skip<Impl: IEnumDot11AdHocInterfacesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Skip(celt) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Reset<Impl: IEnumDot11AdHocInterfacesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Clone<Impl: IEnumDot11AdHocInterfacesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Clone(::core::mem::transmute_copy(&ppenum)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IEnumDot11AdHocInterfaces>, ::windows::core::GetTrustLevel, Next::<Impl, OFFSET>, Skip::<Impl, OFFSET>, Reset::<Impl, OFFSET>, Clone::<Impl, OFFSET>)
    }
}
pub trait IEnumDot11AdHocNetworksImpl: Sized {
    fn Next();
    fn Skip();
    fn Reset();
    fn Clone();
}
impl ::windows::core::RuntimeName for IEnumDot11AdHocNetworks {
    const NAME: &'static str = "Windows.Win32.NetworkManagement.WiFi.IEnumDot11AdHocNetworks";
}
impl IEnumDot11AdHocNetworksVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IEnumDot11AdHocNetworksImpl, const OFFSET: isize>() -> IEnumDot11AdHocNetworksVtbl {
        unsafe extern "system" fn Next<Impl: IEnumDot11AdHocNetworksImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32, rgelt: *mut ::windows::core::RawPtr, pceltfetched: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Next(celt, ::core::mem::transmute_copy(&rgelt), ::core::mem::transmute_copy(&pceltfetched)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Skip<Impl: IEnumDot11AdHocNetworksImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Skip(celt) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Reset<Impl: IEnumDot11AdHocNetworksImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Clone<Impl: IEnumDot11AdHocNetworksImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Clone(::core::mem::transmute_copy(&ppenum)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IEnumDot11AdHocNetworks>, ::windows::core::GetTrustLevel, Next::<Impl, OFFSET>, Skip::<Impl, OFFSET>, Reset::<Impl, OFFSET>, Clone::<Impl, OFFSET>)
    }
}
pub trait IEnumDot11AdHocSecuritySettingsImpl: Sized {
    fn Next();
    fn Skip();
    fn Reset();
    fn Clone();
}
impl ::windows::core::RuntimeName for IEnumDot11AdHocSecuritySettings {
    const NAME: &'static str = "Windows.Win32.NetworkManagement.WiFi.IEnumDot11AdHocSecuritySettings";
}
impl IEnumDot11AdHocSecuritySettingsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IEnumDot11AdHocSecuritySettingsImpl, const OFFSET: isize>() -> IEnumDot11AdHocSecuritySettingsVtbl {
        unsafe extern "system" fn Next<Impl: IEnumDot11AdHocSecuritySettingsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32, rgelt: *mut ::windows::core::RawPtr, pceltfetched: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Next(celt, ::core::mem::transmute_copy(&rgelt), ::core::mem::transmute_copy(&pceltfetched)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Skip<Impl: IEnumDot11AdHocSecuritySettingsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Skip(celt) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Reset<Impl: IEnumDot11AdHocSecuritySettingsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Clone<Impl: IEnumDot11AdHocSecuritySettingsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Clone(::core::mem::transmute_copy(&ppenum)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IEnumDot11AdHocSecuritySettings>, ::windows::core::GetTrustLevel, Next::<Impl, OFFSET>, Skip::<Impl, OFFSET>, Reset::<Impl, OFFSET>, Clone::<Impl, OFFSET>)
    }
}
