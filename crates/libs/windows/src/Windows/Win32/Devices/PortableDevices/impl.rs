pub trait IConnectionRequestCallbackImpl: Sized {
    fn OnComplete();
}
impl IConnectionRequestCallbackVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IConnectionRequestCallbackImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IConnectionRequestCallbackVtbl {
        unsafe extern "system" fn OnComplete<Impl: IConnectionRequestCallbackImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hrstatus: ::windows::core::HRESULT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, OnComplete::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IConnectionRequestCallback as ::windows::core::Interface>::IID
    }
}
pub trait IEnumPortableDeviceConnectorsImpl: Sized {
    fn Next();
    fn Skip();
    fn Reset();
    fn Clone();
}
impl IEnumPortableDeviceConnectorsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IEnumPortableDeviceConnectorsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IEnumPortableDeviceConnectorsVtbl {
        unsafe extern "system" fn Next<Impl: IEnumPortableDeviceConnectorsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, crequested: u32, pconnectors: *mut ::windows::core::RawPtr, pcfetched: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Skip<Impl: IEnumPortableDeviceConnectorsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cconnectors: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Reset<Impl: IEnumPortableDeviceConnectorsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Clone<Impl: IEnumPortableDeviceConnectorsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, Next::<Impl, IMPL_OFFSET>, Skip::<Impl, IMPL_OFFSET>, Reset::<Impl, IMPL_OFFSET>, Clone::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IEnumPortableDeviceConnectors as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IEnumPortableDeviceObjectIDsImpl: Sized {
    fn Next();
    fn Skip();
    fn Reset();
    fn Clone();
    fn Cancel();
}
#[cfg(feature = "Win32_Foundation")]
impl IEnumPortableDeviceObjectIDsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IEnumPortableDeviceObjectIDsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IEnumPortableDeviceObjectIDsVtbl {
        unsafe extern "system" fn Next<Impl: IEnumPortableDeviceObjectIDsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cobjects: u32, pobjids: *mut super::super::Foundation::PWSTR, pcfetched: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Skip<Impl: IEnumPortableDeviceObjectIDsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cobjects: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Reset<Impl: IEnumPortableDeviceObjectIDsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Clone<Impl: IEnumPortableDeviceObjectIDsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Cancel<Impl: IEnumPortableDeviceObjectIDsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, Next::<Impl, IMPL_OFFSET>, Skip::<Impl, IMPL_OFFSET>, Reset::<Impl, IMPL_OFFSET>, Clone::<Impl, IMPL_OFFSET>, Cancel::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IEnumPortableDeviceObjectIDs as ::windows::core::Interface>::IID
    }
}
pub trait IMediaRadioManagerImpl: Sized {
    fn GetRadioInstances();
    fn OnSystemRadioStateChange();
}
impl IMediaRadioManagerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMediaRadioManagerImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMediaRadioManagerVtbl {
        unsafe extern "system" fn GetRadioInstances<Impl: IMediaRadioManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppcollection: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn OnSystemRadioStateChange<Impl: IMediaRadioManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, sysradiostate: SYSTEM_RADIO_STATE, utimeoutsec: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetRadioInstances::<Impl, IMPL_OFFSET>, OnSystemRadioStateChange::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMediaRadioManager as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IMediaRadioManagerNotifySinkImpl: Sized {
    fn OnInstanceAdd();
    fn OnInstanceRemove();
    fn OnInstanceRadioChange();
}
#[cfg(feature = "Win32_Foundation")]
impl IMediaRadioManagerNotifySinkVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMediaRadioManagerNotifySinkImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMediaRadioManagerNotifySinkVtbl {
        unsafe extern "system" fn OnInstanceAdd<Impl: IMediaRadioManagerNotifySinkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pradioinstance: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn OnInstanceRemove<Impl: IMediaRadioManagerNotifySinkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrradioinstanceid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn OnInstanceRadioChange<Impl: IMediaRadioManagerNotifySinkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrradioinstanceid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, radiostate: DEVICE_RADIO_STATE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, OnInstanceAdd::<Impl, IMPL_OFFSET>, OnInstanceRemove::<Impl, IMPL_OFFSET>, OnInstanceRadioChange::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMediaRadioManagerNotifySink as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IPortableDeviceImpl: Sized {
    fn Open();
    fn SendCommand();
    fn Content();
    fn Capabilities();
    fn Cancel();
    fn Close();
    fn Advise();
    fn Unadvise();
    fn GetPnPDeviceID();
}
#[cfg(feature = "Win32_Foundation")]
impl IPortableDeviceVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPortableDeviceImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPortableDeviceVtbl {
        unsafe extern "system" fn Open<Impl: IPortableDeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszpnpdeviceid: super::super::Foundation::PWSTR, pclientinfo: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SendCommand<Impl: IPortableDeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwflags: u32, pparameters: ::windows::core::RawPtr, ppresults: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Content<Impl: IPortableDeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppcontent: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Capabilities<Impl: IPortableDeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppcapabilities: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Cancel<Impl: IPortableDeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Close<Impl: IPortableDeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Advise<Impl: IPortableDeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwflags: u32, pcallback: ::windows::core::RawPtr, pparameters: ::windows::core::RawPtr, ppszcookie: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Unadvise<Impl: IPortableDeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszcookie: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetPnPDeviceID<Impl: IPortableDeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppszpnpdeviceid: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, Open::<Impl, IMPL_OFFSET>, SendCommand::<Impl, IMPL_OFFSET>, Content::<Impl, IMPL_OFFSET>, Capabilities::<Impl, IMPL_OFFSET>, Cancel::<Impl, IMPL_OFFSET>, Close::<Impl, IMPL_OFFSET>, Advise::<Impl, IMPL_OFFSET>, Unadvise::<Impl, IMPL_OFFSET>, GetPnPDeviceID::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPortableDevice as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub trait IPortableDeviceCapabilitiesImpl: Sized {
    fn GetSupportedCommands();
    fn GetCommandOptions();
    fn GetFunctionalCategories();
    fn GetFunctionalObjects();
    fn GetSupportedContentTypes();
    fn GetSupportedFormats();
    fn GetSupportedFormatProperties();
    fn GetFixedPropertyAttributes();
    fn Cancel();
    fn GetSupportedEvents();
    fn GetEventOptions();
}
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
impl IPortableDeviceCapabilitiesVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPortableDeviceCapabilitiesImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPortableDeviceCapabilitiesVtbl {
        unsafe extern "system" fn GetSupportedCommands<Impl: IPortableDeviceCapabilitiesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppcommands: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetCommandOptions<Impl: IPortableDeviceCapabilitiesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, command: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, ppoptions: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetFunctionalCategories<Impl: IPortableDeviceCapabilitiesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppcategories: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetFunctionalObjects<Impl: IPortableDeviceCapabilitiesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, category: *const ::windows::core::GUID, ppobjectids: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetSupportedContentTypes<Impl: IPortableDeviceCapabilitiesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, category: *const ::windows::core::GUID, ppcontenttypes: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetSupportedFormats<Impl: IPortableDeviceCapabilitiesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, contenttype: *const ::windows::core::GUID, ppformats: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetSupportedFormatProperties<Impl: IPortableDeviceCapabilitiesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, format: *const ::windows::core::GUID, ppkeys: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetFixedPropertyAttributes<Impl: IPortableDeviceCapabilitiesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, format: *const ::windows::core::GUID, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, ppattributes: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Cancel<Impl: IPortableDeviceCapabilitiesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetSupportedEvents<Impl: IPortableDeviceCapabilitiesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppevents: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetEventOptions<Impl: IPortableDeviceCapabilitiesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, event: *const ::windows::core::GUID, ppoptions: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            GetSupportedCommands::<Impl, IMPL_OFFSET>,
            GetCommandOptions::<Impl, IMPL_OFFSET>,
            GetFunctionalCategories::<Impl, IMPL_OFFSET>,
            GetFunctionalObjects::<Impl, IMPL_OFFSET>,
            GetSupportedContentTypes::<Impl, IMPL_OFFSET>,
            GetSupportedFormats::<Impl, IMPL_OFFSET>,
            GetSupportedFormatProperties::<Impl, IMPL_OFFSET>,
            GetFixedPropertyAttributes::<Impl, IMPL_OFFSET>,
            Cancel::<Impl, IMPL_OFFSET>,
            GetSupportedEvents::<Impl, IMPL_OFFSET>,
            GetEventOptions::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPortableDeviceCapabilities as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Devices_Properties", feature = "Win32_Foundation"))]
pub trait IPortableDeviceConnectorImpl: Sized {
    fn Connect();
    fn Disconnect();
    fn Cancel();
    fn GetProperty();
    fn SetProperty();
    fn GetPnPID();
}
#[cfg(all(feature = "Win32_Devices_Properties", feature = "Win32_Foundation"))]
impl IPortableDeviceConnectorVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPortableDeviceConnectorImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPortableDeviceConnectorVtbl {
        unsafe extern "system" fn Connect<Impl: IPortableDeviceConnectorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcallback: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Disconnect<Impl: IPortableDeviceConnectorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcallback: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Cancel<Impl: IPortableDeviceConnectorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcallback: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetProperty<Impl: IPortableDeviceConnectorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppropertykey: *const super::Properties::DEVPROPKEY, ppropertytype: *mut u32, ppdata: *mut *mut u8, pcbdata: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetProperty<Impl: IPortableDeviceConnectorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppropertykey: *const super::Properties::DEVPROPKEY, propertytype: u32, pdata: *const u8, cbdata: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetPnPID<Impl: IPortableDeviceConnectorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppwszpnpid: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, Connect::<Impl, IMPL_OFFSET>, Disconnect::<Impl, IMPL_OFFSET>, Cancel::<Impl, IMPL_OFFSET>, GetProperty::<Impl, IMPL_OFFSET>, SetProperty::<Impl, IMPL_OFFSET>, GetPnPID::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPortableDeviceConnector as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub trait IPortableDeviceContentImpl: Sized {
    fn EnumObjects();
    fn Properties();
    fn Transfer();
    fn CreateObjectWithPropertiesOnly();
    fn CreateObjectWithPropertiesAndData();
    fn Delete();
    fn GetObjectIDsFromPersistentUniqueIDs();
    fn Cancel();
    fn Move();
    fn Copy();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl IPortableDeviceContentVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPortableDeviceContentImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPortableDeviceContentVtbl {
        unsafe extern "system" fn EnumObjects<Impl: IPortableDeviceContentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwflags: u32, pszparentobjectid: super::super::Foundation::PWSTR, pfilter: ::windows::core::RawPtr, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Properties<Impl: IPortableDeviceContentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppproperties: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Transfer<Impl: IPortableDeviceContentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppresources: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CreateObjectWithPropertiesOnly<Impl: IPortableDeviceContentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalues: ::windows::core::RawPtr, ppszobjectid: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CreateObjectWithPropertiesAndData<Impl: IPortableDeviceContentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalues: ::windows::core::RawPtr, ppdata: *mut ::windows::core::RawPtr, pdwoptimalwritebuffersize: *mut u32, ppszcookie: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Delete<Impl: IPortableDeviceContentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwoptions: u32, pobjectids: ::windows::core::RawPtr, ppresults: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetObjectIDsFromPersistentUniqueIDs<Impl: IPortableDeviceContentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppersistentuniqueids: ::windows::core::RawPtr, ppobjectids: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Cancel<Impl: IPortableDeviceContentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Move<Impl: IPortableDeviceContentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pobjectids: ::windows::core::RawPtr, pszdestinationfolderobjectid: super::super::Foundation::PWSTR, ppresults: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Copy<Impl: IPortableDeviceContentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pobjectids: ::windows::core::RawPtr, pszdestinationfolderobjectid: super::super::Foundation::PWSTR, ppresults: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            EnumObjects::<Impl, IMPL_OFFSET>,
            Properties::<Impl, IMPL_OFFSET>,
            Transfer::<Impl, IMPL_OFFSET>,
            CreateObjectWithPropertiesOnly::<Impl, IMPL_OFFSET>,
            CreateObjectWithPropertiesAndData::<Impl, IMPL_OFFSET>,
            Delete::<Impl, IMPL_OFFSET>,
            GetObjectIDsFromPersistentUniqueIDs::<Impl, IMPL_OFFSET>,
            Cancel::<Impl, IMPL_OFFSET>,
            Move::<Impl, IMPL_OFFSET>,
            Copy::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPortableDeviceContent as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub trait IPortableDeviceContent2Impl: Sized + IPortableDeviceContentImpl {
    fn UpdateObjectWithPropertiesAndData();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl IPortableDeviceContent2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPortableDeviceContent2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPortableDeviceContent2Vtbl {
        unsafe extern "system" fn UpdateObjectWithPropertiesAndData<Impl: IPortableDeviceContent2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszobjectid: super::super::Foundation::PWSTR, pproperties: ::windows::core::RawPtr, ppdata: *mut ::windows::core::RawPtr, pdwoptimalwritebuffersize: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            EnumObjects::<Impl, IMPL_OFFSET>,
            Properties::<Impl, IMPL_OFFSET>,
            Transfer::<Impl, IMPL_OFFSET>,
            CreateObjectWithPropertiesOnly::<Impl, IMPL_OFFSET>,
            CreateObjectWithPropertiesAndData::<Impl, IMPL_OFFSET>,
            Delete::<Impl, IMPL_OFFSET>,
            GetObjectIDsFromPersistentUniqueIDs::<Impl, IMPL_OFFSET>,
            Cancel::<Impl, IMPL_OFFSET>,
            Move::<Impl, IMPL_OFFSET>,
            Copy::<Impl, IMPL_OFFSET>,
            UpdateObjectWithPropertiesAndData::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPortableDeviceContent2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
pub trait IPortableDeviceDataStreamImpl: Sized + IStreamImpl + ISequentialStreamImpl {
    fn GetObjectID();
    fn Cancel();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
impl IPortableDeviceDataStreamVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPortableDeviceDataStreamImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPortableDeviceDataStreamVtbl {
        unsafe extern "system" fn GetObjectID<Impl: IPortableDeviceDataStreamImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppszobjectid: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Cancel<Impl: IPortableDeviceDataStreamImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            Read::<Impl, IMPL_OFFSET>,
            Write::<Impl, IMPL_OFFSET>,
            Seek::<Impl, IMPL_OFFSET>,
            SetSize::<Impl, IMPL_OFFSET>,
            CopyTo::<Impl, IMPL_OFFSET>,
            Commit::<Impl, IMPL_OFFSET>,
            Revert::<Impl, IMPL_OFFSET>,
            LockRegion::<Impl, IMPL_OFFSET>,
            UnlockRegion::<Impl, IMPL_OFFSET>,
            Stat::<Impl, IMPL_OFFSET>,
            Clone::<Impl, IMPL_OFFSET>,
            GetObjectID::<Impl, IMPL_OFFSET>,
            Cancel::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPortableDeviceDataStream as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub trait IPortableDeviceDispatchFactoryImpl: Sized {
    fn GetDeviceDispatch();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl IPortableDeviceDispatchFactoryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPortableDeviceDispatchFactoryImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPortableDeviceDispatchFactoryVtbl {
        unsafe extern "system" fn GetDeviceDispatch<Impl: IPortableDeviceDispatchFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszpnpdeviceid: super::super::Foundation::PWSTR, ppdevicedispatch: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetDeviceDispatch::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPortableDeviceDispatchFactory as ::windows::core::Interface>::IID
    }
}
pub trait IPortableDeviceEventCallbackImpl: Sized {
    fn OnEvent();
}
impl IPortableDeviceEventCallbackVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPortableDeviceEventCallbackImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPortableDeviceEventCallbackVtbl {
        unsafe extern "system" fn OnEvent<Impl: IPortableDeviceEventCallbackImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, peventparameters: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, OnEvent::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPortableDeviceEventCallback as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub trait IPortableDeviceKeyCollectionImpl: Sized {
    fn GetCount();
    fn GetAt();
    fn Add();
    fn Clear();
    fn RemoveAt();
}
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
impl IPortableDeviceKeyCollectionVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPortableDeviceKeyCollectionImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPortableDeviceKeyCollectionVtbl {
        unsafe extern "system" fn GetCount<Impl: IPortableDeviceKeyCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcelems: *const u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetAt<Impl: IPortableDeviceKeyCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwindex: u32, pkey: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Add<Impl: IPortableDeviceKeyCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Clear<Impl: IPortableDeviceKeyCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn RemoveAt<Impl: IPortableDeviceKeyCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwindex: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetCount::<Impl, IMPL_OFFSET>, GetAt::<Impl, IMPL_OFFSET>, Add::<Impl, IMPL_OFFSET>, Clear::<Impl, IMPL_OFFSET>, RemoveAt::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPortableDeviceKeyCollection as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IPortableDeviceManagerImpl: Sized {
    fn GetDevices();
    fn RefreshDeviceList();
    fn GetDeviceFriendlyName();
    fn GetDeviceDescription();
    fn GetDeviceManufacturer();
    fn GetDeviceProperty();
    fn GetPrivateDevices();
}
#[cfg(feature = "Win32_Foundation")]
impl IPortableDeviceManagerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPortableDeviceManagerImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPortableDeviceManagerVtbl {
        unsafe extern "system" fn GetDevices<Impl: IPortableDeviceManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppnpdeviceids: *mut super::super::Foundation::PWSTR, pcpnpdeviceids: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn RefreshDeviceList<Impl: IPortableDeviceManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetDeviceFriendlyName<Impl: IPortableDeviceManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszpnpdeviceid: super::super::Foundation::PWSTR, pdevicefriendlyname: super::super::Foundation::PWSTR, pcchdevicefriendlyname: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetDeviceDescription<Impl: IPortableDeviceManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszpnpdeviceid: super::super::Foundation::PWSTR, pdevicedescription: super::super::Foundation::PWSTR, pcchdevicedescription: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetDeviceManufacturer<Impl: IPortableDeviceManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszpnpdeviceid: super::super::Foundation::PWSTR, pdevicemanufacturer: super::super::Foundation::PWSTR, pcchdevicemanufacturer: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetDeviceProperty<Impl: IPortableDeviceManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszpnpdeviceid: super::super::Foundation::PWSTR, pszdevicepropertyname: super::super::Foundation::PWSTR, pdata: *mut u8, pcbdata: *mut u32, pdwtype: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetPrivateDevices<Impl: IPortableDeviceManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppnpdeviceids: *mut super::super::Foundation::PWSTR, pcpnpdeviceids: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetDevices::<Impl, IMPL_OFFSET>, RefreshDeviceList::<Impl, IMPL_OFFSET>, GetDeviceFriendlyName::<Impl, IMPL_OFFSET>, GetDeviceDescription::<Impl, IMPL_OFFSET>, GetDeviceManufacturer::<Impl, IMPL_OFFSET>, GetDeviceProperty::<Impl, IMPL_OFFSET>, GetPrivateDevices::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPortableDeviceManager as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
pub trait IPortableDevicePropVariantCollectionImpl: Sized {
    fn GetCount();
    fn GetAt();
    fn Add();
    fn GetType();
    fn ChangeType();
    fn Clear();
    fn RemoveAt();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
impl IPortableDevicePropVariantCollectionVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPortableDevicePropVariantCollectionImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPortableDevicePropVariantCollectionVtbl {
        unsafe extern "system" fn GetCount<Impl: IPortableDevicePropVariantCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcelems: *const u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetAt<Impl: IPortableDevicePropVariantCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwindex: u32, pvalue: *const super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Add<Impl: IPortableDevicePropVariantCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *const super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetType<Impl: IPortableDevicePropVariantCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvt: *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ChangeType<Impl: IPortableDevicePropVariantCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, vt: u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Clear<Impl: IPortableDevicePropVariantCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn RemoveAt<Impl: IPortableDevicePropVariantCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwindex: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetCount::<Impl, IMPL_OFFSET>, GetAt::<Impl, IMPL_OFFSET>, Add::<Impl, IMPL_OFFSET>, GetType::<Impl, IMPL_OFFSET>, ChangeType::<Impl, IMPL_OFFSET>, Clear::<Impl, IMPL_OFFSET>, RemoveAt::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPortableDevicePropVariantCollection as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Shell_PropertiesSystem"))]
pub trait IPortableDevicePropertiesImpl: Sized {
    fn GetSupportedProperties();
    fn GetPropertyAttributes();
    fn GetValues();
    fn SetValues();
    fn Delete();
    fn Cancel();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Shell_PropertiesSystem"))]
impl IPortableDevicePropertiesVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPortableDevicePropertiesImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPortableDevicePropertiesVtbl {
        unsafe extern "system" fn GetSupportedProperties<Impl: IPortableDevicePropertiesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszobjectid: super::super::Foundation::PWSTR, ppkeys: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetPropertyAttributes<Impl: IPortableDevicePropertiesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszobjectid: super::super::Foundation::PWSTR, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, ppattributes: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetValues<Impl: IPortableDevicePropertiesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszobjectid: super::super::Foundation::PWSTR, pkeys: ::windows::core::RawPtr, ppvalues: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetValues<Impl: IPortableDevicePropertiesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszobjectid: super::super::Foundation::PWSTR, pvalues: ::windows::core::RawPtr, ppresults: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Delete<Impl: IPortableDevicePropertiesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszobjectid: super::super::Foundation::PWSTR, pkeys: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Cancel<Impl: IPortableDevicePropertiesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetSupportedProperties::<Impl, IMPL_OFFSET>, GetPropertyAttributes::<Impl, IMPL_OFFSET>, GetValues::<Impl, IMPL_OFFSET>, SetValues::<Impl, IMPL_OFFSET>, Delete::<Impl, IMPL_OFFSET>, Cancel::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPortableDeviceProperties as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IPortableDevicePropertiesBulkImpl: Sized {
    fn QueueGetValuesByObjectList();
    fn QueueGetValuesByObjectFormat();
    fn QueueSetValuesByObjectList();
    fn Start();
    fn Cancel();
}
#[cfg(feature = "Win32_Foundation")]
impl IPortableDevicePropertiesBulkVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPortableDevicePropertiesBulkImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPortableDevicePropertiesBulkVtbl {
        unsafe extern "system" fn QueueGetValuesByObjectList<Impl: IPortableDevicePropertiesBulkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pobjectids: ::windows::core::RawPtr, pkeys: ::windows::core::RawPtr, pcallback: ::windows::core::RawPtr, pcontext: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn QueueGetValuesByObjectFormat<Impl: IPortableDevicePropertiesBulkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pguidobjectformat: *const ::windows::core::GUID, pszparentobjectid: super::super::Foundation::PWSTR, dwdepth: u32, pkeys: ::windows::core::RawPtr, pcallback: ::windows::core::RawPtr, pcontext: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn QueueSetValuesByObjectList<Impl: IPortableDevicePropertiesBulkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pobjectvalues: ::windows::core::RawPtr, pcallback: ::windows::core::RawPtr, pcontext: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Start<Impl: IPortableDevicePropertiesBulkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcontext: *const ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Cancel<Impl: IPortableDevicePropertiesBulkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcontext: *const ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, QueueGetValuesByObjectList::<Impl, IMPL_OFFSET>, QueueGetValuesByObjectFormat::<Impl, IMPL_OFFSET>, QueueSetValuesByObjectList::<Impl, IMPL_OFFSET>, Start::<Impl, IMPL_OFFSET>, Cancel::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPortableDevicePropertiesBulk as ::windows::core::Interface>::IID
    }
}
pub trait IPortableDevicePropertiesBulkCallbackImpl: Sized {
    fn OnStart();
    fn OnProgress();
    fn OnEnd();
}
impl IPortableDevicePropertiesBulkCallbackVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPortableDevicePropertiesBulkCallbackImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPortableDevicePropertiesBulkCallbackVtbl {
        unsafe extern "system" fn OnStart<Impl: IPortableDevicePropertiesBulkCallbackImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcontext: *const ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn OnProgress<Impl: IPortableDevicePropertiesBulkCallbackImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcontext: *const ::windows::core::GUID, presults: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn OnEnd<Impl: IPortableDevicePropertiesBulkCallbackImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcontext: *const ::windows::core::GUID, hrstatus: ::windows::core::HRESULT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, OnStart::<Impl, IMPL_OFFSET>, OnProgress::<Impl, IMPL_OFFSET>, OnEnd::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPortableDevicePropertiesBulkCallback as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_UI_Shell_PropertiesSystem"))]
pub trait IPortableDeviceResourcesImpl: Sized {
    fn GetSupportedResources();
    fn GetResourceAttributes();
    fn GetStream();
    fn Delete();
    fn Cancel();
    fn CreateResource();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_UI_Shell_PropertiesSystem"))]
impl IPortableDeviceResourcesVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPortableDeviceResourcesImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPortableDeviceResourcesVtbl {
        unsafe extern "system" fn GetSupportedResources<Impl: IPortableDeviceResourcesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszobjectid: super::super::Foundation::PWSTR, ppkeys: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetResourceAttributes<Impl: IPortableDeviceResourcesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszobjectid: super::super::Foundation::PWSTR, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, ppresourceattributes: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetStream<Impl: IPortableDeviceResourcesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszobjectid: super::super::Foundation::PWSTR, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, dwmode: u32, pdwoptimalbuffersize: *mut u32, ppstream: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Delete<Impl: IPortableDeviceResourcesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszobjectid: super::super::Foundation::PWSTR, pkeys: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Cancel<Impl: IPortableDeviceResourcesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CreateResource<Impl: IPortableDeviceResourcesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, presourceattributes: ::windows::core::RawPtr, ppdata: *mut ::windows::core::RawPtr, pdwoptimalwritebuffersize: *mut u32, ppszcookie: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetSupportedResources::<Impl, IMPL_OFFSET>, GetResourceAttributes::<Impl, IMPL_OFFSET>, GetStream::<Impl, IMPL_OFFSET>, Delete::<Impl, IMPL_OFFSET>, Cancel::<Impl, IMPL_OFFSET>, CreateResource::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPortableDeviceResources as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IPortableDeviceServiceImpl: Sized {
    fn Open();
    fn Capabilities();
    fn Content();
    fn Methods();
    fn Cancel();
    fn Close();
    fn GetServiceObjectID();
    fn GetPnPServiceID();
    fn Advise();
    fn Unadvise();
    fn SendCommand();
}
#[cfg(feature = "Win32_Foundation")]
impl IPortableDeviceServiceVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPortableDeviceServiceImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPortableDeviceServiceVtbl {
        unsafe extern "system" fn Open<Impl: IPortableDeviceServiceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszpnpserviceid: super::super::Foundation::PWSTR, pclientinfo: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Capabilities<Impl: IPortableDeviceServiceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppcapabilities: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Content<Impl: IPortableDeviceServiceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppcontent: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Methods<Impl: IPortableDeviceServiceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppmethods: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Cancel<Impl: IPortableDeviceServiceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Close<Impl: IPortableDeviceServiceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetServiceObjectID<Impl: IPortableDeviceServiceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppszserviceobjectid: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetPnPServiceID<Impl: IPortableDeviceServiceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppszpnpserviceid: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Advise<Impl: IPortableDeviceServiceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwflags: u32, pcallback: ::windows::core::RawPtr, pparameters: ::windows::core::RawPtr, ppszcookie: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Unadvise<Impl: IPortableDeviceServiceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszcookie: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SendCommand<Impl: IPortableDeviceServiceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwflags: u32, pparameters: ::windows::core::RawPtr, ppresults: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            Open::<Impl, IMPL_OFFSET>,
            Capabilities::<Impl, IMPL_OFFSET>,
            Content::<Impl, IMPL_OFFSET>,
            Methods::<Impl, IMPL_OFFSET>,
            Cancel::<Impl, IMPL_OFFSET>,
            Close::<Impl, IMPL_OFFSET>,
            GetServiceObjectID::<Impl, IMPL_OFFSET>,
            GetPnPServiceID::<Impl, IMPL_OFFSET>,
            Advise::<Impl, IMPL_OFFSET>,
            Unadvise::<Impl, IMPL_OFFSET>,
            SendCommand::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPortableDeviceService as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IPortableDeviceServiceActivationImpl: Sized {
    fn OpenAsync();
    fn CancelOpenAsync();
}
#[cfg(feature = "Win32_Foundation")]
impl IPortableDeviceServiceActivationVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPortableDeviceServiceActivationImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPortableDeviceServiceActivationVtbl {
        unsafe extern "system" fn OpenAsync<Impl: IPortableDeviceServiceActivationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszpnpserviceid: super::super::Foundation::PWSTR, pclientinfo: ::windows::core::RawPtr, pcallback: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CancelOpenAsync<Impl: IPortableDeviceServiceActivationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, OpenAsync::<Impl, IMPL_OFFSET>, CancelOpenAsync::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPortableDeviceServiceActivation as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub trait IPortableDeviceServiceCapabilitiesImpl: Sized {
    fn GetSupportedMethods();
    fn GetSupportedMethodsByFormat();
    fn GetMethodAttributes();
    fn GetMethodParameterAttributes();
    fn GetSupportedFormats();
    fn GetFormatAttributes();
    fn GetSupportedFormatProperties();
    fn GetFormatPropertyAttributes();
    fn GetSupportedEvents();
    fn GetEventAttributes();
    fn GetEventParameterAttributes();
    fn GetInheritedServices();
    fn GetFormatRenderingProfiles();
    fn GetSupportedCommands();
    fn GetCommandOptions();
    fn Cancel();
}
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
impl IPortableDeviceServiceCapabilitiesVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPortableDeviceServiceCapabilitiesImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPortableDeviceServiceCapabilitiesVtbl {
        unsafe extern "system" fn GetSupportedMethods<Impl: IPortableDeviceServiceCapabilitiesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppmethods: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetSupportedMethodsByFormat<Impl: IPortableDeviceServiceCapabilitiesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, format: *const ::windows::core::GUID, ppmethods: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetMethodAttributes<Impl: IPortableDeviceServiceCapabilitiesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, method: *const ::windows::core::GUID, ppattributes: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetMethodParameterAttributes<Impl: IPortableDeviceServiceCapabilitiesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, method: *const ::windows::core::GUID, parameter: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, ppattributes: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetSupportedFormats<Impl: IPortableDeviceServiceCapabilitiesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppformats: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetFormatAttributes<Impl: IPortableDeviceServiceCapabilitiesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, format: *const ::windows::core::GUID, ppattributes: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetSupportedFormatProperties<Impl: IPortableDeviceServiceCapabilitiesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, format: *const ::windows::core::GUID, ppkeys: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetFormatPropertyAttributes<Impl: IPortableDeviceServiceCapabilitiesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, format: *const ::windows::core::GUID, property: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, ppattributes: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetSupportedEvents<Impl: IPortableDeviceServiceCapabilitiesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppevents: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetEventAttributes<Impl: IPortableDeviceServiceCapabilitiesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, event: *const ::windows::core::GUID, ppattributes: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetEventParameterAttributes<Impl: IPortableDeviceServiceCapabilitiesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, event: *const ::windows::core::GUID, parameter: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, ppattributes: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetInheritedServices<Impl: IPortableDeviceServiceCapabilitiesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwinheritancetype: u32, ppservices: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetFormatRenderingProfiles<Impl: IPortableDeviceServiceCapabilitiesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, format: *const ::windows::core::GUID, pprenderingprofiles: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetSupportedCommands<Impl: IPortableDeviceServiceCapabilitiesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppcommands: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetCommandOptions<Impl: IPortableDeviceServiceCapabilitiesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, command: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, ppoptions: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Cancel<Impl: IPortableDeviceServiceCapabilitiesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            GetSupportedMethods::<Impl, IMPL_OFFSET>,
            GetSupportedMethodsByFormat::<Impl, IMPL_OFFSET>,
            GetMethodAttributes::<Impl, IMPL_OFFSET>,
            GetMethodParameterAttributes::<Impl, IMPL_OFFSET>,
            GetSupportedFormats::<Impl, IMPL_OFFSET>,
            GetFormatAttributes::<Impl, IMPL_OFFSET>,
            GetSupportedFormatProperties::<Impl, IMPL_OFFSET>,
            GetFormatPropertyAttributes::<Impl, IMPL_OFFSET>,
            GetSupportedEvents::<Impl, IMPL_OFFSET>,
            GetEventAttributes::<Impl, IMPL_OFFSET>,
            GetEventParameterAttributes::<Impl, IMPL_OFFSET>,
            GetInheritedServices::<Impl, IMPL_OFFSET>,
            GetFormatRenderingProfiles::<Impl, IMPL_OFFSET>,
            GetSupportedCommands::<Impl, IMPL_OFFSET>,
            GetCommandOptions::<Impl, IMPL_OFFSET>,
            Cancel::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPortableDeviceServiceCapabilities as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IPortableDeviceServiceManagerImpl: Sized {
    fn GetDeviceServices();
    fn GetDeviceForService();
}
#[cfg(feature = "Win32_Foundation")]
impl IPortableDeviceServiceManagerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPortableDeviceServiceManagerImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPortableDeviceServiceManagerVtbl {
        unsafe extern "system" fn GetDeviceServices<Impl: IPortableDeviceServiceManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszpnpdeviceid: super::super::Foundation::PWSTR, guidservicecategory: *const ::windows::core::GUID, pservices: *mut super::super::Foundation::PWSTR, pcservices: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetDeviceForService<Impl: IPortableDeviceServiceManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszpnpserviceid: super::super::Foundation::PWSTR, ppszpnpdeviceid: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetDeviceServices::<Impl, IMPL_OFFSET>, GetDeviceForService::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPortableDeviceServiceManager as ::windows::core::Interface>::IID
    }
}
pub trait IPortableDeviceServiceMethodCallbackImpl: Sized {
    fn OnComplete();
}
impl IPortableDeviceServiceMethodCallbackVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPortableDeviceServiceMethodCallbackImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPortableDeviceServiceMethodCallbackVtbl {
        unsafe extern "system" fn OnComplete<Impl: IPortableDeviceServiceMethodCallbackImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hrstatus: ::windows::core::HRESULT, presults: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, OnComplete::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPortableDeviceServiceMethodCallback as ::windows::core::Interface>::IID
    }
}
pub trait IPortableDeviceServiceMethodsImpl: Sized {
    fn Invoke();
    fn InvokeAsync();
    fn Cancel();
}
impl IPortableDeviceServiceMethodsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPortableDeviceServiceMethodsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPortableDeviceServiceMethodsVtbl {
        unsafe extern "system" fn Invoke<Impl: IPortableDeviceServiceMethodsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, method: *const ::windows::core::GUID, pparameters: ::windows::core::RawPtr, ppresults: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn InvokeAsync<Impl: IPortableDeviceServiceMethodsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, method: *const ::windows::core::GUID, pparameters: ::windows::core::RawPtr, pcallback: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Cancel<Impl: IPortableDeviceServiceMethodsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcallback: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, Invoke::<Impl, IMPL_OFFSET>, InvokeAsync::<Impl, IMPL_OFFSET>, Cancel::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPortableDeviceServiceMethods as ::windows::core::Interface>::IID
    }
}
pub trait IPortableDeviceServiceOpenCallbackImpl: Sized {
    fn OnComplete();
}
impl IPortableDeviceServiceOpenCallbackVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPortableDeviceServiceOpenCallbackImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPortableDeviceServiceOpenCallbackVtbl {
        unsafe extern "system" fn OnComplete<Impl: IPortableDeviceServiceOpenCallbackImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hrstatus: ::windows::core::HRESULT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, OnComplete::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPortableDeviceServiceOpenCallback as ::windows::core::Interface>::IID
    }
}
pub trait IPortableDeviceUnitsStreamImpl: Sized {
    fn SeekInUnits();
    fn Cancel();
}
impl IPortableDeviceUnitsStreamVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPortableDeviceUnitsStreamImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPortableDeviceUnitsStreamVtbl {
        unsafe extern "system" fn SeekInUnits<Impl: IPortableDeviceUnitsStreamImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dlibmove: i64, units: WPD_STREAM_UNITS, dworigin: u32, plibnewposition: *mut u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Cancel<Impl: IPortableDeviceUnitsStreamImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, SeekInUnits::<Impl, IMPL_OFFSET>, Cancel::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPortableDeviceUnitsStream as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_UI_Shell_PropertiesSystem"))]
pub trait IPortableDeviceValuesImpl: Sized {
    fn GetCount();
    fn GetAt();
    fn SetValue();
    fn GetValue();
    fn SetStringValue();
    fn GetStringValue();
    fn SetUnsignedIntegerValue();
    fn GetUnsignedIntegerValue();
    fn SetSignedIntegerValue();
    fn GetSignedIntegerValue();
    fn SetUnsignedLargeIntegerValue();
    fn GetUnsignedLargeIntegerValue();
    fn SetSignedLargeIntegerValue();
    fn GetSignedLargeIntegerValue();
    fn SetFloatValue();
    fn GetFloatValue();
    fn SetErrorValue();
    fn GetErrorValue();
    fn SetKeyValue();
    fn GetKeyValue();
    fn SetBoolValue();
    fn GetBoolValue();
    fn SetIUnknownValue();
    fn GetIUnknownValue();
    fn SetGuidValue();
    fn GetGuidValue();
    fn SetBufferValue();
    fn GetBufferValue();
    fn SetIPortableDeviceValuesValue();
    fn GetIPortableDeviceValuesValue();
    fn SetIPortableDevicePropVariantCollectionValue();
    fn GetIPortableDevicePropVariantCollectionValue();
    fn SetIPortableDeviceKeyCollectionValue();
    fn GetIPortableDeviceKeyCollectionValue();
    fn SetIPortableDeviceValuesCollectionValue();
    fn GetIPortableDeviceValuesCollectionValue();
    fn RemoveValue();
    fn CopyValuesFromPropertyStore();
    fn CopyValuesToPropertyStore();
    fn Clear();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_UI_Shell_PropertiesSystem"))]
impl IPortableDeviceValuesVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPortableDeviceValuesImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPortableDeviceValuesVtbl {
        unsafe extern "system" fn GetCount<Impl: IPortableDeviceValuesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcelt: *const u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetAt<Impl: IPortableDeviceValuesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32, pkey: *mut super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, pvalue: *mut super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetValue<Impl: IPortableDeviceValuesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, pvalue: *const super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetValue<Impl: IPortableDeviceValuesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, pvalue: *mut super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetStringValue<Impl: IPortableDeviceValuesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, value: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetStringValue<Impl: IPortableDeviceValuesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, pvalue: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetUnsignedIntegerValue<Impl: IPortableDeviceValuesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, value: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetUnsignedIntegerValue<Impl: IPortableDeviceValuesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, pvalue: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetSignedIntegerValue<Impl: IPortableDeviceValuesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, value: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetSignedIntegerValue<Impl: IPortableDeviceValuesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, pvalue: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetUnsignedLargeIntegerValue<Impl: IPortableDeviceValuesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, value: u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetUnsignedLargeIntegerValue<Impl: IPortableDeviceValuesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, pvalue: *mut u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetSignedLargeIntegerValue<Impl: IPortableDeviceValuesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, value: i64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetSignedLargeIntegerValue<Impl: IPortableDeviceValuesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, pvalue: *mut i64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetFloatValue<Impl: IPortableDeviceValuesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, value: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetFloatValue<Impl: IPortableDeviceValuesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, pvalue: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetErrorValue<Impl: IPortableDeviceValuesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, value: ::windows::core::HRESULT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetErrorValue<Impl: IPortableDeviceValuesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, pvalue: *mut ::windows::core::HRESULT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetKeyValue<Impl: IPortableDeviceValuesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, value: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetKeyValue<Impl: IPortableDeviceValuesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, pvalue: *mut super::super::UI::Shell::PropertiesSystem::PROPERTYKEY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetBoolValue<Impl: IPortableDeviceValuesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, value: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetBoolValue<Impl: IPortableDeviceValuesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, pvalue: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetIUnknownValue<Impl: IPortableDeviceValuesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, pvalue: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetIUnknownValue<Impl: IPortableDeviceValuesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, ppvalue: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetGuidValue<Impl: IPortableDeviceValuesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, value: *const ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetGuidValue<Impl: IPortableDeviceValuesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, pvalue: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetBufferValue<Impl: IPortableDeviceValuesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, pvalue: *const u8, cbvalue: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetBufferValue<Impl: IPortableDeviceValuesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, ppvalue: *mut *mut u8, pcbvalue: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetIPortableDeviceValuesValue<Impl: IPortableDeviceValuesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, pvalue: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetIPortableDeviceValuesValue<Impl: IPortableDeviceValuesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, ppvalue: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetIPortableDevicePropVariantCollectionValue<Impl: IPortableDeviceValuesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, pvalue: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetIPortableDevicePropVariantCollectionValue<Impl: IPortableDeviceValuesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, ppvalue: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetIPortableDeviceKeyCollectionValue<Impl: IPortableDeviceValuesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, pvalue: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetIPortableDeviceKeyCollectionValue<Impl: IPortableDeviceValuesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, ppvalue: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetIPortableDeviceValuesCollectionValue<Impl: IPortableDeviceValuesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, pvalue: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetIPortableDeviceValuesCollectionValue<Impl: IPortableDeviceValuesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, ppvalue: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn RemoveValue<Impl: IPortableDeviceValuesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CopyValuesFromPropertyStore<Impl: IPortableDeviceValuesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pstore: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CopyValuesToPropertyStore<Impl: IPortableDeviceValuesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pstore: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Clear<Impl: IPortableDeviceValuesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            GetCount::<Impl, IMPL_OFFSET>,
            GetAt::<Impl, IMPL_OFFSET>,
            SetValue::<Impl, IMPL_OFFSET>,
            GetValue::<Impl, IMPL_OFFSET>,
            SetStringValue::<Impl, IMPL_OFFSET>,
            GetStringValue::<Impl, IMPL_OFFSET>,
            SetUnsignedIntegerValue::<Impl, IMPL_OFFSET>,
            GetUnsignedIntegerValue::<Impl, IMPL_OFFSET>,
            SetSignedIntegerValue::<Impl, IMPL_OFFSET>,
            GetSignedIntegerValue::<Impl, IMPL_OFFSET>,
            SetUnsignedLargeIntegerValue::<Impl, IMPL_OFFSET>,
            GetUnsignedLargeIntegerValue::<Impl, IMPL_OFFSET>,
            SetSignedLargeIntegerValue::<Impl, IMPL_OFFSET>,
            GetSignedLargeIntegerValue::<Impl, IMPL_OFFSET>,
            SetFloatValue::<Impl, IMPL_OFFSET>,
            GetFloatValue::<Impl, IMPL_OFFSET>,
            SetErrorValue::<Impl, IMPL_OFFSET>,
            GetErrorValue::<Impl, IMPL_OFFSET>,
            SetKeyValue::<Impl, IMPL_OFFSET>,
            GetKeyValue::<Impl, IMPL_OFFSET>,
            SetBoolValue::<Impl, IMPL_OFFSET>,
            GetBoolValue::<Impl, IMPL_OFFSET>,
            SetIUnknownValue::<Impl, IMPL_OFFSET>,
            GetIUnknownValue::<Impl, IMPL_OFFSET>,
            SetGuidValue::<Impl, IMPL_OFFSET>,
            GetGuidValue::<Impl, IMPL_OFFSET>,
            SetBufferValue::<Impl, IMPL_OFFSET>,
            GetBufferValue::<Impl, IMPL_OFFSET>,
            SetIPortableDeviceValuesValue::<Impl, IMPL_OFFSET>,
            GetIPortableDeviceValuesValue::<Impl, IMPL_OFFSET>,
            SetIPortableDevicePropVariantCollectionValue::<Impl, IMPL_OFFSET>,
            GetIPortableDevicePropVariantCollectionValue::<Impl, IMPL_OFFSET>,
            SetIPortableDeviceKeyCollectionValue::<Impl, IMPL_OFFSET>,
            GetIPortableDeviceKeyCollectionValue::<Impl, IMPL_OFFSET>,
            SetIPortableDeviceValuesCollectionValue::<Impl, IMPL_OFFSET>,
            GetIPortableDeviceValuesCollectionValue::<Impl, IMPL_OFFSET>,
            RemoveValue::<Impl, IMPL_OFFSET>,
            CopyValuesFromPropertyStore::<Impl, IMPL_OFFSET>,
            CopyValuesToPropertyStore::<Impl, IMPL_OFFSET>,
            Clear::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPortableDeviceValues as ::windows::core::Interface>::IID
    }
}
pub trait IPortableDeviceValuesCollectionImpl: Sized {
    fn GetCount();
    fn GetAt();
    fn Add();
    fn Clear();
    fn RemoveAt();
}
impl IPortableDeviceValuesCollectionVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPortableDeviceValuesCollectionImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPortableDeviceValuesCollectionVtbl {
        unsafe extern "system" fn GetCount<Impl: IPortableDeviceValuesCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcelems: *const u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetAt<Impl: IPortableDeviceValuesCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwindex: u32, ppvalues: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Add<Impl: IPortableDeviceValuesCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalues: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Clear<Impl: IPortableDeviceValuesCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn RemoveAt<Impl: IPortableDeviceValuesCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwindex: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetCount::<Impl, IMPL_OFFSET>, GetAt::<Impl, IMPL_OFFSET>, Add::<Impl, IMPL_OFFSET>, Clear::<Impl, IMPL_OFFSET>, RemoveAt::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPortableDeviceValuesCollection as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IPortableDeviceWebControlImpl: Sized + IDispatchImpl {
    fn GetDeviceFromId();
    fn GetDeviceFromIdAsync();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IPortableDeviceWebControlVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPortableDeviceWebControlImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPortableDeviceWebControlVtbl {
        unsafe extern "system" fn GetDeviceFromId<Impl: IPortableDeviceWebControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, deviceid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ppdevice: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetDeviceFromIdAsync<Impl: IPortableDeviceWebControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, deviceid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pcompletionhandler: ::windows::core::RawPtr, perrorhandler: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetTypeInfoCount::<Impl, IMPL_OFFSET>, GetTypeInfo::<Impl, IMPL_OFFSET>, GetIDsOfNames::<Impl, IMPL_OFFSET>, Invoke::<Impl, IMPL_OFFSET>, GetDeviceFromId::<Impl, IMPL_OFFSET>, GetDeviceFromIdAsync::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPortableDeviceWebControl as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IRadioInstanceImpl: Sized {
    fn GetRadioManagerSignature();
    fn GetInstanceSignature();
    fn GetFriendlyName();
    fn GetRadioState();
    fn SetRadioState();
    fn IsMultiComm();
    fn IsAssociatingDevice();
}
#[cfg(feature = "Win32_Foundation")]
impl IRadioInstanceVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRadioInstanceImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IRadioInstanceVtbl {
        unsafe extern "system" fn GetRadioManagerSignature<Impl: IRadioInstanceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pguidsignature: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetInstanceSignature<Impl: IRadioInstanceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrid: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetFriendlyName<Impl: IRadioInstanceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lcid: u32, pbstrname: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetRadioState<Impl: IRadioInstanceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pradiostate: *mut DEVICE_RADIO_STATE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetRadioState<Impl: IRadioInstanceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, radiostate: DEVICE_RADIO_STATE, utimeoutsec: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn IsMultiComm<Impl: IRadioInstanceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn IsAssociatingDevice<Impl: IRadioInstanceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetRadioManagerSignature::<Impl, IMPL_OFFSET>, GetInstanceSignature::<Impl, IMPL_OFFSET>, GetFriendlyName::<Impl, IMPL_OFFSET>, GetRadioState::<Impl, IMPL_OFFSET>, SetRadioState::<Impl, IMPL_OFFSET>, IsMultiComm::<Impl, IMPL_OFFSET>, IsAssociatingDevice::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRadioInstance as ::windows::core::Interface>::IID
    }
}
pub trait IRadioInstanceCollectionImpl: Sized {
    fn GetCount();
    fn GetAt();
}
impl IRadioInstanceCollectionVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRadioInstanceCollectionImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IRadioInstanceCollectionVtbl {
        unsafe extern "system" fn GetCount<Impl: IRadioInstanceCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcinstance: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetAt<Impl: IRadioInstanceCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, uindex: u32, ppradioinstance: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetCount::<Impl, IMPL_OFFSET>, GetAt::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRadioInstanceCollection as ::windows::core::Interface>::IID
    }
}
pub trait IWpdSerializerImpl: Sized {
    fn GetIPortableDeviceValuesFromBuffer();
    fn WriteIPortableDeviceValuesToBuffer();
    fn GetBufferFromIPortableDeviceValues();
    fn GetSerializedSize();
}
impl IWpdSerializerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWpdSerializerImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWpdSerializerVtbl {
        unsafe extern "system" fn GetIPortableDeviceValuesFromBuffer<Impl: IWpdSerializerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbuffer: *const u8, dwinputbufferlength: u32, ppparams: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn WriteIPortableDeviceValuesToBuffer<Impl: IWpdSerializerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwoutputbufferlength: u32, presults: ::windows::core::RawPtr, pbuffer: *mut u8, pdwbyteswritten: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetBufferFromIPortableDeviceValues<Impl: IWpdSerializerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psource: ::windows::core::RawPtr, ppbuffer: *mut *mut u8, pdwbuffersize: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetSerializedSize<Impl: IWpdSerializerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psource: ::windows::core::RawPtr, pdwsize: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetIPortableDeviceValuesFromBuffer::<Impl, IMPL_OFFSET>, WriteIPortableDeviceValuesToBuffer::<Impl, IMPL_OFFSET>, GetBufferFromIPortableDeviceValues::<Impl, IMPL_OFFSET>, GetSerializedSize::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWpdSerializer as ::windows::core::Interface>::IID
    }
}
