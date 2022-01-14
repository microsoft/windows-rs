pub trait IConnectionRequestCallback_Impl: Sized {
    fn OnComplete(&mut self, hrstatus: ::windows::core::HRESULT) -> ::windows::core::Result<()>;
}
impl IConnectionRequestCallback_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IConnectionRequestCallback_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IConnectionRequestCallback_Vtbl {
        unsafe extern "system" fn OnComplete<Impl: IConnectionRequestCallback_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hrstatus: ::windows::core::HRESULT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnComplete(::core::mem::transmute_copy(&hrstatus)).into()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), OnComplete: OnComplete::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IConnectionRequestCallback as ::windows::core::Interface>::IID
    }
}
pub trait IEnumPortableDeviceConnectors_Impl: Sized {
    fn Next(&mut self, crequested: u32, pconnectors: *mut ::core::option::Option<IPortableDeviceConnector>, pcfetched: *mut u32) -> ::windows::core::Result<()>;
    fn Skip(&mut self, cconnectors: u32) -> ::windows::core::Result<()>;
    fn Reset(&mut self) -> ::windows::core::Result<()>;
    fn Clone(&mut self) -> ::windows::core::Result<IEnumPortableDeviceConnectors>;
}
impl IEnumPortableDeviceConnectors_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IEnumPortableDeviceConnectors_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IEnumPortableDeviceConnectors_Vtbl {
        unsafe extern "system" fn Next<Impl: IEnumPortableDeviceConnectors_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, crequested: u32, pconnectors: *mut ::windows::core::RawPtr, pcfetched: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Next(::core::mem::transmute_copy(&crequested), ::core::mem::transmute_copy(&pconnectors), ::core::mem::transmute_copy(&pcfetched)).into()
        }
        unsafe extern "system" fn Skip<Impl: IEnumPortableDeviceConnectors_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cconnectors: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Skip(::core::mem::transmute_copy(&cconnectors)).into()
        }
        unsafe extern "system" fn Reset<Impl: IEnumPortableDeviceConnectors_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Reset().into()
        }
        unsafe extern "system" fn Clone<Impl: IEnumPortableDeviceConnectors_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Clone() {
                ::core::result::Result::Ok(ok__) => {
                    *ppenum = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
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
        iid == &<IEnumPortableDeviceConnectors as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IEnumPortableDeviceObjectIDs_Impl: Sized {
    fn Next(&mut self, cobjects: u32, pobjids: *mut super::super::Foundation::PWSTR, pcfetched: *mut u32) -> ::windows::core::HRESULT;
    fn Skip(&mut self, cobjects: u32) -> ::windows::core::HRESULT;
    fn Reset(&mut self) -> ::windows::core::Result<()>;
    fn Clone(&mut self) -> ::windows::core::Result<IEnumPortableDeviceObjectIDs>;
    fn Cancel(&mut self) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IEnumPortableDeviceObjectIDs_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IEnumPortableDeviceObjectIDs_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IEnumPortableDeviceObjectIDs_Vtbl {
        unsafe extern "system" fn Next<Impl: IEnumPortableDeviceObjectIDs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cobjects: u32, pobjids: *mut super::super::Foundation::PWSTR, pcfetched: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Next(::core::mem::transmute_copy(&cobjects), ::core::mem::transmute_copy(&pobjids), ::core::mem::transmute_copy(&pcfetched))
        }
        unsafe extern "system" fn Skip<Impl: IEnumPortableDeviceObjectIDs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cobjects: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Skip(::core::mem::transmute_copy(&cobjects))
        }
        unsafe extern "system" fn Reset<Impl: IEnumPortableDeviceObjectIDs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Reset().into()
        }
        unsafe extern "system" fn Clone<Impl: IEnumPortableDeviceObjectIDs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Clone() {
                ::core::result::Result::Ok(ok__) => {
                    *ppenum = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Cancel<Impl: IEnumPortableDeviceObjectIDs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Cancel().into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            Next: Next::<Impl, IMPL_OFFSET>,
            Skip: Skip::<Impl, IMPL_OFFSET>,
            Reset: Reset::<Impl, IMPL_OFFSET>,
            Clone: Clone::<Impl, IMPL_OFFSET>,
            Cancel: Cancel::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IEnumPortableDeviceObjectIDs as ::windows::core::Interface>::IID
    }
}
pub trait IMediaRadioManager_Impl: Sized {
    fn GetRadioInstances(&mut self) -> ::windows::core::Result<IRadioInstanceCollection>;
    fn OnSystemRadioStateChange(&mut self, sysradiostate: SYSTEM_RADIO_STATE, utimeoutsec: u32) -> ::windows::core::Result<()>;
}
impl IMediaRadioManager_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMediaRadioManager_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMediaRadioManager_Vtbl {
        unsafe extern "system" fn GetRadioInstances<Impl: IMediaRadioManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppcollection: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetRadioInstances() {
                ::core::result::Result::Ok(ok__) => {
                    *ppcollection = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OnSystemRadioStateChange<Impl: IMediaRadioManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, sysradiostate: SYSTEM_RADIO_STATE, utimeoutsec: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnSystemRadioStateChange(::core::mem::transmute_copy(&sysradiostate), ::core::mem::transmute_copy(&utimeoutsec)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            GetRadioInstances: GetRadioInstances::<Impl, IMPL_OFFSET>,
            OnSystemRadioStateChange: OnSystemRadioStateChange::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMediaRadioManager as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IMediaRadioManagerNotifySink_Impl: Sized {
    fn OnInstanceAdd(&mut self, pradioinstance: ::core::option::Option<IRadioInstance>) -> ::windows::core::Result<()>;
    fn OnInstanceRemove(&mut self, bstrradioinstanceid: super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn OnInstanceRadioChange(&mut self, bstrradioinstanceid: super::super::Foundation::BSTR, radiostate: DEVICE_RADIO_STATE) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IMediaRadioManagerNotifySink_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMediaRadioManagerNotifySink_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMediaRadioManagerNotifySink_Vtbl {
        unsafe extern "system" fn OnInstanceAdd<Impl: IMediaRadioManagerNotifySink_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pradioinstance: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnInstanceAdd(::core::mem::transmute(&pradioinstance)).into()
        }
        unsafe extern "system" fn OnInstanceRemove<Impl: IMediaRadioManagerNotifySink_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrradioinstanceid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnInstanceRemove(::core::mem::transmute_copy(&bstrradioinstanceid)).into()
        }
        unsafe extern "system" fn OnInstanceRadioChange<Impl: IMediaRadioManagerNotifySink_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrradioinstanceid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, radiostate: DEVICE_RADIO_STATE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnInstanceRadioChange(::core::mem::transmute_copy(&bstrradioinstanceid), ::core::mem::transmute_copy(&radiostate)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            OnInstanceAdd: OnInstanceAdd::<Impl, IMPL_OFFSET>,
            OnInstanceRemove: OnInstanceRemove::<Impl, IMPL_OFFSET>,
            OnInstanceRadioChange: OnInstanceRadioChange::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMediaRadioManagerNotifySink as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IPortableDevice_Impl: Sized {
    fn Open(&mut self, pszpnpdeviceid: super::super::Foundation::PWSTR, pclientinfo: ::core::option::Option<IPortableDeviceValues>) -> ::windows::core::Result<()>;
    fn SendCommand(&mut self, dwflags: u32, pparameters: ::core::option::Option<IPortableDeviceValues>) -> ::windows::core::Result<IPortableDeviceValues>;
    fn Content(&mut self) -> ::windows::core::Result<IPortableDeviceContent>;
    fn Capabilities(&mut self) -> ::windows::core::Result<IPortableDeviceCapabilities>;
    fn Cancel(&mut self) -> ::windows::core::Result<()>;
    fn Close(&mut self) -> ::windows::core::Result<()>;
    fn Advise(&mut self, dwflags: u32, pcallback: ::core::option::Option<IPortableDeviceEventCallback>, pparameters: ::core::option::Option<IPortableDeviceValues>) -> ::windows::core::Result<super::super::Foundation::PWSTR>;
    fn Unadvise(&mut self, pszcookie: super::super::Foundation::PWSTR) -> ::windows::core::Result<()>;
    fn GetPnPDeviceID(&mut self) -> ::windows::core::Result<super::super::Foundation::PWSTR>;
}
#[cfg(feature = "Win32_Foundation")]
impl IPortableDevice_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPortableDevice_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPortableDevice_Vtbl {
        unsafe extern "system" fn Open<Impl: IPortableDevice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszpnpdeviceid: super::super::Foundation::PWSTR, pclientinfo: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Open(::core::mem::transmute_copy(&pszpnpdeviceid), ::core::mem::transmute(&pclientinfo)).into()
        }
        unsafe extern "system" fn SendCommand<Impl: IPortableDevice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwflags: u32, pparameters: ::windows::core::RawPtr, ppresults: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SendCommand(::core::mem::transmute_copy(&dwflags), ::core::mem::transmute(&pparameters)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppresults = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Content<Impl: IPortableDevice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppcontent: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Content() {
                ::core::result::Result::Ok(ok__) => {
                    *ppcontent = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Capabilities<Impl: IPortableDevice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppcapabilities: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Capabilities() {
                ::core::result::Result::Ok(ok__) => {
                    *ppcapabilities = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Cancel<Impl: IPortableDevice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Cancel().into()
        }
        unsafe extern "system" fn Close<Impl: IPortableDevice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Close().into()
        }
        unsafe extern "system" fn Advise<Impl: IPortableDevice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwflags: u32, pcallback: ::windows::core::RawPtr, pparameters: ::windows::core::RawPtr, ppszcookie: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Advise(::core::mem::transmute_copy(&dwflags), ::core::mem::transmute(&pcallback), ::core::mem::transmute(&pparameters)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppszcookie = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Unadvise<Impl: IPortableDevice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszcookie: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Unadvise(::core::mem::transmute_copy(&pszcookie)).into()
        }
        unsafe extern "system" fn GetPnPDeviceID<Impl: IPortableDevice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppszpnpdeviceid: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetPnPDeviceID() {
                ::core::result::Result::Ok(ok__) => {
                    *ppszpnpdeviceid = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            Open: Open::<Impl, IMPL_OFFSET>,
            SendCommand: SendCommand::<Impl, IMPL_OFFSET>,
            Content: Content::<Impl, IMPL_OFFSET>,
            Capabilities: Capabilities::<Impl, IMPL_OFFSET>,
            Cancel: Cancel::<Impl, IMPL_OFFSET>,
            Close: Close::<Impl, IMPL_OFFSET>,
            Advise: Advise::<Impl, IMPL_OFFSET>,
            Unadvise: Unadvise::<Impl, IMPL_OFFSET>,
            GetPnPDeviceID: GetPnPDeviceID::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPortableDevice as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub trait IPortableDeviceCapabilities_Impl: Sized {
    fn GetSupportedCommands(&mut self) -> ::windows::core::Result<IPortableDeviceKeyCollection>;
    fn GetCommandOptions(&mut self, command: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY) -> ::windows::core::Result<IPortableDeviceValues>;
    fn GetFunctionalCategories(&mut self) -> ::windows::core::Result<IPortableDevicePropVariantCollection>;
    fn GetFunctionalObjects(&mut self, category: *const ::windows::core::GUID) -> ::windows::core::Result<IPortableDevicePropVariantCollection>;
    fn GetSupportedContentTypes(&mut self, category: *const ::windows::core::GUID) -> ::windows::core::Result<IPortableDevicePropVariantCollection>;
    fn GetSupportedFormats(&mut self, contenttype: *const ::windows::core::GUID) -> ::windows::core::Result<IPortableDevicePropVariantCollection>;
    fn GetSupportedFormatProperties(&mut self, format: *const ::windows::core::GUID) -> ::windows::core::Result<IPortableDeviceKeyCollection>;
    fn GetFixedPropertyAttributes(&mut self, format: *const ::windows::core::GUID, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY) -> ::windows::core::Result<IPortableDeviceValues>;
    fn Cancel(&mut self) -> ::windows::core::Result<()>;
    fn GetSupportedEvents(&mut self) -> ::windows::core::Result<IPortableDevicePropVariantCollection>;
    fn GetEventOptions(&mut self, event: *const ::windows::core::GUID) -> ::windows::core::Result<IPortableDeviceValues>;
}
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
impl IPortableDeviceCapabilities_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPortableDeviceCapabilities_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPortableDeviceCapabilities_Vtbl {
        unsafe extern "system" fn GetSupportedCommands<Impl: IPortableDeviceCapabilities_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppcommands: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetSupportedCommands() {
                ::core::result::Result::Ok(ok__) => {
                    *ppcommands = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCommandOptions<Impl: IPortableDeviceCapabilities_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, command: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, ppoptions: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetCommandOptions(::core::mem::transmute_copy(&command)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppoptions = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFunctionalCategories<Impl: IPortableDeviceCapabilities_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppcategories: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetFunctionalCategories() {
                ::core::result::Result::Ok(ok__) => {
                    *ppcategories = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFunctionalObjects<Impl: IPortableDeviceCapabilities_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, category: *const ::windows::core::GUID, ppobjectids: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetFunctionalObjects(::core::mem::transmute_copy(&category)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppobjectids = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSupportedContentTypes<Impl: IPortableDeviceCapabilities_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, category: *const ::windows::core::GUID, ppcontenttypes: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetSupportedContentTypes(::core::mem::transmute_copy(&category)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppcontenttypes = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSupportedFormats<Impl: IPortableDeviceCapabilities_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, contenttype: *const ::windows::core::GUID, ppformats: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetSupportedFormats(::core::mem::transmute_copy(&contenttype)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppformats = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSupportedFormatProperties<Impl: IPortableDeviceCapabilities_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, format: *const ::windows::core::GUID, ppkeys: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetSupportedFormatProperties(::core::mem::transmute_copy(&format)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppkeys = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFixedPropertyAttributes<Impl: IPortableDeviceCapabilities_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, format: *const ::windows::core::GUID, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, ppattributes: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetFixedPropertyAttributes(::core::mem::transmute_copy(&format), ::core::mem::transmute_copy(&key)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppattributes = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Cancel<Impl: IPortableDeviceCapabilities_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Cancel().into()
        }
        unsafe extern "system" fn GetSupportedEvents<Impl: IPortableDeviceCapabilities_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppevents: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetSupportedEvents() {
                ::core::result::Result::Ok(ok__) => {
                    *ppevents = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetEventOptions<Impl: IPortableDeviceCapabilities_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, event: *const ::windows::core::GUID, ppoptions: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetEventOptions(::core::mem::transmute_copy(&event)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppoptions = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            GetSupportedCommands: GetSupportedCommands::<Impl, IMPL_OFFSET>,
            GetCommandOptions: GetCommandOptions::<Impl, IMPL_OFFSET>,
            GetFunctionalCategories: GetFunctionalCategories::<Impl, IMPL_OFFSET>,
            GetFunctionalObjects: GetFunctionalObjects::<Impl, IMPL_OFFSET>,
            GetSupportedContentTypes: GetSupportedContentTypes::<Impl, IMPL_OFFSET>,
            GetSupportedFormats: GetSupportedFormats::<Impl, IMPL_OFFSET>,
            GetSupportedFormatProperties: GetSupportedFormatProperties::<Impl, IMPL_OFFSET>,
            GetFixedPropertyAttributes: GetFixedPropertyAttributes::<Impl, IMPL_OFFSET>,
            Cancel: Cancel::<Impl, IMPL_OFFSET>,
            GetSupportedEvents: GetSupportedEvents::<Impl, IMPL_OFFSET>,
            GetEventOptions: GetEventOptions::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPortableDeviceCapabilities as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Devices_Properties", feature = "Win32_Foundation"))]
pub trait IPortableDeviceConnector_Impl: Sized {
    fn Connect(&mut self, pcallback: ::core::option::Option<IConnectionRequestCallback>) -> ::windows::core::Result<()>;
    fn Disconnect(&mut self, pcallback: ::core::option::Option<IConnectionRequestCallback>) -> ::windows::core::Result<()>;
    fn Cancel(&mut self, pcallback: ::core::option::Option<IConnectionRequestCallback>) -> ::windows::core::Result<()>;
    fn GetProperty(&mut self, ppropertykey: *const super::Properties::DEVPROPKEY, ppropertytype: *mut u32, ppdata: *mut *mut u8, pcbdata: *mut u32) -> ::windows::core::Result<()>;
    fn SetProperty(&mut self, ppropertykey: *const super::Properties::DEVPROPKEY, propertytype: u32, pdata: *const u8, cbdata: u32) -> ::windows::core::Result<()>;
    fn GetPnPID(&mut self) -> ::windows::core::Result<super::super::Foundation::PWSTR>;
}
#[cfg(all(feature = "Win32_Devices_Properties", feature = "Win32_Foundation"))]
impl IPortableDeviceConnector_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPortableDeviceConnector_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPortableDeviceConnector_Vtbl {
        unsafe extern "system" fn Connect<Impl: IPortableDeviceConnector_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcallback: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Connect(::core::mem::transmute(&pcallback)).into()
        }
        unsafe extern "system" fn Disconnect<Impl: IPortableDeviceConnector_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcallback: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Disconnect(::core::mem::transmute(&pcallback)).into()
        }
        unsafe extern "system" fn Cancel<Impl: IPortableDeviceConnector_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcallback: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Cancel(::core::mem::transmute(&pcallback)).into()
        }
        unsafe extern "system" fn GetProperty<Impl: IPortableDeviceConnector_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppropertykey: *const super::Properties::DEVPROPKEY, ppropertytype: *mut u32, ppdata: *mut *mut u8, pcbdata: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetProperty(::core::mem::transmute_copy(&ppropertykey), ::core::mem::transmute_copy(&ppropertytype), ::core::mem::transmute_copy(&ppdata), ::core::mem::transmute_copy(&pcbdata)).into()
        }
        unsafe extern "system" fn SetProperty<Impl: IPortableDeviceConnector_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppropertykey: *const super::Properties::DEVPROPKEY, propertytype: u32, pdata: *const u8, cbdata: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetProperty(::core::mem::transmute_copy(&ppropertykey), ::core::mem::transmute_copy(&propertytype), ::core::mem::transmute_copy(&pdata), ::core::mem::transmute_copy(&cbdata)).into()
        }
        unsafe extern "system" fn GetPnPID<Impl: IPortableDeviceConnector_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppwszpnpid: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetPnPID() {
                ::core::result::Result::Ok(ok__) => {
                    *ppwszpnpid = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            Connect: Connect::<Impl, IMPL_OFFSET>,
            Disconnect: Disconnect::<Impl, IMPL_OFFSET>,
            Cancel: Cancel::<Impl, IMPL_OFFSET>,
            GetProperty: GetProperty::<Impl, IMPL_OFFSET>,
            SetProperty: SetProperty::<Impl, IMPL_OFFSET>,
            GetPnPID: GetPnPID::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPortableDeviceConnector as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub trait IPortableDeviceContent_Impl: Sized {
    fn EnumObjects(&mut self, dwflags: u32, pszparentobjectid: super::super::Foundation::PWSTR, pfilter: ::core::option::Option<IPortableDeviceValues>) -> ::windows::core::Result<IEnumPortableDeviceObjectIDs>;
    fn Properties(&mut self) -> ::windows::core::Result<IPortableDeviceProperties>;
    fn Transfer(&mut self) -> ::windows::core::Result<IPortableDeviceResources>;
    fn CreateObjectWithPropertiesOnly(&mut self, pvalues: ::core::option::Option<IPortableDeviceValues>, ppszobjectid: *mut super::super::Foundation::PWSTR) -> ::windows::core::Result<()>;
    fn CreateObjectWithPropertiesAndData(&mut self, pvalues: ::core::option::Option<IPortableDeviceValues>, ppdata: *mut ::core::option::Option<super::super::System::Com::IStream>, pdwoptimalwritebuffersize: *mut u32, ppszcookie: *mut super::super::Foundation::PWSTR) -> ::windows::core::Result<()>;
    fn Delete(&mut self, dwoptions: u32, pobjectids: ::core::option::Option<IPortableDevicePropVariantCollection>, ppresults: *mut ::core::option::Option<IPortableDevicePropVariantCollection>) -> ::windows::core::Result<()>;
    fn GetObjectIDsFromPersistentUniqueIDs(&mut self, ppersistentuniqueids: ::core::option::Option<IPortableDevicePropVariantCollection>) -> ::windows::core::Result<IPortableDevicePropVariantCollection>;
    fn Cancel(&mut self) -> ::windows::core::Result<()>;
    fn Move(&mut self, pobjectids: ::core::option::Option<IPortableDevicePropVariantCollection>, pszdestinationfolderobjectid: super::super::Foundation::PWSTR, ppresults: *mut ::core::option::Option<IPortableDevicePropVariantCollection>) -> ::windows::core::Result<()>;
    fn Copy(&mut self, pobjectids: ::core::option::Option<IPortableDevicePropVariantCollection>, pszdestinationfolderobjectid: super::super::Foundation::PWSTR, ppresults: *mut ::core::option::Option<IPortableDevicePropVariantCollection>) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl IPortableDeviceContent_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPortableDeviceContent_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPortableDeviceContent_Vtbl {
        unsafe extern "system" fn EnumObjects<Impl: IPortableDeviceContent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwflags: u32, pszparentobjectid: super::super::Foundation::PWSTR, pfilter: ::windows::core::RawPtr, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EnumObjects(::core::mem::transmute_copy(&dwflags), ::core::mem::transmute_copy(&pszparentobjectid), ::core::mem::transmute(&pfilter)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppenum = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Properties<Impl: IPortableDeviceContent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppproperties: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Properties() {
                ::core::result::Result::Ok(ok__) => {
                    *ppproperties = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Transfer<Impl: IPortableDeviceContent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppresources: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Transfer() {
                ::core::result::Result::Ok(ok__) => {
                    *ppresources = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateObjectWithPropertiesOnly<Impl: IPortableDeviceContent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalues: ::windows::core::RawPtr, ppszobjectid: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).CreateObjectWithPropertiesOnly(::core::mem::transmute(&pvalues), ::core::mem::transmute_copy(&ppszobjectid)).into()
        }
        unsafe extern "system" fn CreateObjectWithPropertiesAndData<Impl: IPortableDeviceContent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalues: ::windows::core::RawPtr, ppdata: *mut ::windows::core::RawPtr, pdwoptimalwritebuffersize: *mut u32, ppszcookie: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).CreateObjectWithPropertiesAndData(::core::mem::transmute(&pvalues), ::core::mem::transmute_copy(&ppdata), ::core::mem::transmute_copy(&pdwoptimalwritebuffersize), ::core::mem::transmute_copy(&ppszcookie)).into()
        }
        unsafe extern "system" fn Delete<Impl: IPortableDeviceContent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwoptions: u32, pobjectids: ::windows::core::RawPtr, ppresults: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Delete(::core::mem::transmute_copy(&dwoptions), ::core::mem::transmute(&pobjectids), ::core::mem::transmute_copy(&ppresults)).into()
        }
        unsafe extern "system" fn GetObjectIDsFromPersistentUniqueIDs<Impl: IPortableDeviceContent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppersistentuniqueids: ::windows::core::RawPtr, ppobjectids: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetObjectIDsFromPersistentUniqueIDs(::core::mem::transmute(&ppersistentuniqueids)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppobjectids = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Cancel<Impl: IPortableDeviceContent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Cancel().into()
        }
        unsafe extern "system" fn Move<Impl: IPortableDeviceContent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pobjectids: ::windows::core::RawPtr, pszdestinationfolderobjectid: super::super::Foundation::PWSTR, ppresults: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Move(::core::mem::transmute(&pobjectids), ::core::mem::transmute_copy(&pszdestinationfolderobjectid), ::core::mem::transmute_copy(&ppresults)).into()
        }
        unsafe extern "system" fn Copy<Impl: IPortableDeviceContent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pobjectids: ::windows::core::RawPtr, pszdestinationfolderobjectid: super::super::Foundation::PWSTR, ppresults: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Copy(::core::mem::transmute(&pobjectids), ::core::mem::transmute_copy(&pszdestinationfolderobjectid), ::core::mem::transmute_copy(&ppresults)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            EnumObjects: EnumObjects::<Impl, IMPL_OFFSET>,
            Properties: Properties::<Impl, IMPL_OFFSET>,
            Transfer: Transfer::<Impl, IMPL_OFFSET>,
            CreateObjectWithPropertiesOnly: CreateObjectWithPropertiesOnly::<Impl, IMPL_OFFSET>,
            CreateObjectWithPropertiesAndData: CreateObjectWithPropertiesAndData::<Impl, IMPL_OFFSET>,
            Delete: Delete::<Impl, IMPL_OFFSET>,
            GetObjectIDsFromPersistentUniqueIDs: GetObjectIDsFromPersistentUniqueIDs::<Impl, IMPL_OFFSET>,
            Cancel: Cancel::<Impl, IMPL_OFFSET>,
            Move: Move::<Impl, IMPL_OFFSET>,
            Copy: Copy::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPortableDeviceContent as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub trait IPortableDeviceContent2_Impl: Sized + IPortableDeviceContent_Impl {
    fn UpdateObjectWithPropertiesAndData(&mut self, pszobjectid: super::super::Foundation::PWSTR, pproperties: ::core::option::Option<IPortableDeviceValues>, ppdata: *mut ::core::option::Option<super::super::System::Com::IStream>, pdwoptimalwritebuffersize: *mut u32) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl IPortableDeviceContent2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPortableDeviceContent2_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPortableDeviceContent2_Vtbl {
        unsafe extern "system" fn UpdateObjectWithPropertiesAndData<Impl: IPortableDeviceContent2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszobjectid: super::super::Foundation::PWSTR, pproperties: ::windows::core::RawPtr, ppdata: *mut ::windows::core::RawPtr, pdwoptimalwritebuffersize: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).UpdateObjectWithPropertiesAndData(::core::mem::transmute_copy(&pszobjectid), ::core::mem::transmute(&pproperties), ::core::mem::transmute_copy(&ppdata), ::core::mem::transmute_copy(&pdwoptimalwritebuffersize)).into()
        }
        Self {
            base: IPortableDeviceContent_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            UpdateObjectWithPropertiesAndData: UpdateObjectWithPropertiesAndData::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPortableDeviceContent2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
pub trait IPortableDeviceDataStream_Impl: Sized + super::super::System::Com::ISequentialStream_Impl + super::super::System::Com::IStream_Impl {
    fn GetObjectID(&mut self) -> ::windows::core::Result<super::super::Foundation::PWSTR>;
    fn Cancel(&mut self) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
impl IPortableDeviceDataStream_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPortableDeviceDataStream_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPortableDeviceDataStream_Vtbl {
        unsafe extern "system" fn GetObjectID<Impl: IPortableDeviceDataStream_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppszobjectid: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetObjectID() {
                ::core::result::Result::Ok(ok__) => {
                    *ppszobjectid = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Cancel<Impl: IPortableDeviceDataStream_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Cancel().into()
        }
        Self {
            base: super::super::System::Com::IStream_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            GetObjectID: GetObjectID::<Impl, IMPL_OFFSET>,
            Cancel: Cancel::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPortableDeviceDataStream as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub trait IPortableDeviceDispatchFactory_Impl: Sized {
    fn GetDeviceDispatch(&mut self, pszpnpdeviceid: super::super::Foundation::PWSTR) -> ::windows::core::Result<super::super::System::Com::IDispatch>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl IPortableDeviceDispatchFactory_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPortableDeviceDispatchFactory_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPortableDeviceDispatchFactory_Vtbl {
        unsafe extern "system" fn GetDeviceDispatch<Impl: IPortableDeviceDispatchFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszpnpdeviceid: super::super::Foundation::PWSTR, ppdevicedispatch: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDeviceDispatch(::core::mem::transmute_copy(&pszpnpdeviceid)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppdevicedispatch = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), GetDeviceDispatch: GetDeviceDispatch::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPortableDeviceDispatchFactory as ::windows::core::Interface>::IID
    }
}
pub trait IPortableDeviceEventCallback_Impl: Sized {
    fn OnEvent(&mut self, peventparameters: ::core::option::Option<IPortableDeviceValues>) -> ::windows::core::Result<()>;
}
impl IPortableDeviceEventCallback_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPortableDeviceEventCallback_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPortableDeviceEventCallback_Vtbl {
        unsafe extern "system" fn OnEvent<Impl: IPortableDeviceEventCallback_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, peventparameters: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnEvent(::core::mem::transmute(&peventparameters)).into()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), OnEvent: OnEvent::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPortableDeviceEventCallback as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub trait IPortableDeviceKeyCollection_Impl: Sized {
    fn GetCount(&mut self, pcelems: *const u32) -> ::windows::core::Result<()>;
    fn GetAt(&mut self, dwindex: u32, pkey: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY) -> ::windows::core::Result<()>;
    fn Add(&mut self, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY) -> ::windows::core::Result<()>;
    fn Clear(&mut self) -> ::windows::core::Result<()>;
    fn RemoveAt(&mut self, dwindex: u32) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
impl IPortableDeviceKeyCollection_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPortableDeviceKeyCollection_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPortableDeviceKeyCollection_Vtbl {
        unsafe extern "system" fn GetCount<Impl: IPortableDeviceKeyCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcelems: *const u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetCount(::core::mem::transmute_copy(&pcelems)).into()
        }
        unsafe extern "system" fn GetAt<Impl: IPortableDeviceKeyCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwindex: u32, pkey: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetAt(::core::mem::transmute_copy(&dwindex), ::core::mem::transmute_copy(&pkey)).into()
        }
        unsafe extern "system" fn Add<Impl: IPortableDeviceKeyCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Add(::core::mem::transmute_copy(&key)).into()
        }
        unsafe extern "system" fn Clear<Impl: IPortableDeviceKeyCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Clear().into()
        }
        unsafe extern "system" fn RemoveAt<Impl: IPortableDeviceKeyCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwindex: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveAt(::core::mem::transmute_copy(&dwindex)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            GetCount: GetCount::<Impl, IMPL_OFFSET>,
            GetAt: GetAt::<Impl, IMPL_OFFSET>,
            Add: Add::<Impl, IMPL_OFFSET>,
            Clear: Clear::<Impl, IMPL_OFFSET>,
            RemoveAt: RemoveAt::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPortableDeviceKeyCollection as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IPortableDeviceManager_Impl: Sized {
    fn GetDevices(&mut self, ppnpdeviceids: *mut super::super::Foundation::PWSTR, pcpnpdeviceids: *mut u32) -> ::windows::core::Result<()>;
    fn RefreshDeviceList(&mut self) -> ::windows::core::Result<()>;
    fn GetDeviceFriendlyName(&mut self, pszpnpdeviceid: super::super::Foundation::PWSTR, pdevicefriendlyname: super::super::Foundation::PWSTR, pcchdevicefriendlyname: *mut u32) -> ::windows::core::Result<()>;
    fn GetDeviceDescription(&mut self, pszpnpdeviceid: super::super::Foundation::PWSTR, pdevicedescription: super::super::Foundation::PWSTR, pcchdevicedescription: *mut u32) -> ::windows::core::Result<()>;
    fn GetDeviceManufacturer(&mut self, pszpnpdeviceid: super::super::Foundation::PWSTR, pdevicemanufacturer: super::super::Foundation::PWSTR, pcchdevicemanufacturer: *mut u32) -> ::windows::core::Result<()>;
    fn GetDeviceProperty(&mut self, pszpnpdeviceid: super::super::Foundation::PWSTR, pszdevicepropertyname: super::super::Foundation::PWSTR, pdata: *mut u8, pcbdata: *mut u32, pdwtype: *mut u32) -> ::windows::core::Result<()>;
    fn GetPrivateDevices(&mut self, ppnpdeviceids: *mut super::super::Foundation::PWSTR, pcpnpdeviceids: *mut u32) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IPortableDeviceManager_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPortableDeviceManager_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPortableDeviceManager_Vtbl {
        unsafe extern "system" fn GetDevices<Impl: IPortableDeviceManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppnpdeviceids: *mut super::super::Foundation::PWSTR, pcpnpdeviceids: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetDevices(::core::mem::transmute_copy(&ppnpdeviceids), ::core::mem::transmute_copy(&pcpnpdeviceids)).into()
        }
        unsafe extern "system" fn RefreshDeviceList<Impl: IPortableDeviceManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RefreshDeviceList().into()
        }
        unsafe extern "system" fn GetDeviceFriendlyName<Impl: IPortableDeviceManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszpnpdeviceid: super::super::Foundation::PWSTR, pdevicefriendlyname: super::super::Foundation::PWSTR, pcchdevicefriendlyname: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetDeviceFriendlyName(::core::mem::transmute_copy(&pszpnpdeviceid), ::core::mem::transmute_copy(&pdevicefriendlyname), ::core::mem::transmute_copy(&pcchdevicefriendlyname)).into()
        }
        unsafe extern "system" fn GetDeviceDescription<Impl: IPortableDeviceManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszpnpdeviceid: super::super::Foundation::PWSTR, pdevicedescription: super::super::Foundation::PWSTR, pcchdevicedescription: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetDeviceDescription(::core::mem::transmute_copy(&pszpnpdeviceid), ::core::mem::transmute_copy(&pdevicedescription), ::core::mem::transmute_copy(&pcchdevicedescription)).into()
        }
        unsafe extern "system" fn GetDeviceManufacturer<Impl: IPortableDeviceManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszpnpdeviceid: super::super::Foundation::PWSTR, pdevicemanufacturer: super::super::Foundation::PWSTR, pcchdevicemanufacturer: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetDeviceManufacturer(::core::mem::transmute_copy(&pszpnpdeviceid), ::core::mem::transmute_copy(&pdevicemanufacturer), ::core::mem::transmute_copy(&pcchdevicemanufacturer)).into()
        }
        unsafe extern "system" fn GetDeviceProperty<Impl: IPortableDeviceManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszpnpdeviceid: super::super::Foundation::PWSTR, pszdevicepropertyname: super::super::Foundation::PWSTR, pdata: *mut u8, pcbdata: *mut u32, pdwtype: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetDeviceProperty(::core::mem::transmute_copy(&pszpnpdeviceid), ::core::mem::transmute_copy(&pszdevicepropertyname), ::core::mem::transmute_copy(&pdata), ::core::mem::transmute_copy(&pcbdata), ::core::mem::transmute_copy(&pdwtype)).into()
        }
        unsafe extern "system" fn GetPrivateDevices<Impl: IPortableDeviceManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppnpdeviceids: *mut super::super::Foundation::PWSTR, pcpnpdeviceids: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetPrivateDevices(::core::mem::transmute_copy(&ppnpdeviceids), ::core::mem::transmute_copy(&pcpnpdeviceids)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            GetDevices: GetDevices::<Impl, IMPL_OFFSET>,
            RefreshDeviceList: RefreshDeviceList::<Impl, IMPL_OFFSET>,
            GetDeviceFriendlyName: GetDeviceFriendlyName::<Impl, IMPL_OFFSET>,
            GetDeviceDescription: GetDeviceDescription::<Impl, IMPL_OFFSET>,
            GetDeviceManufacturer: GetDeviceManufacturer::<Impl, IMPL_OFFSET>,
            GetDeviceProperty: GetDeviceProperty::<Impl, IMPL_OFFSET>,
            GetPrivateDevices: GetPrivateDevices::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPortableDeviceManager as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
pub trait IPortableDevicePropVariantCollection_Impl: Sized {
    fn GetCount(&mut self, pcelems: *const u32) -> ::windows::core::Result<()>;
    fn GetAt(&mut self, dwindex: u32, pvalue: *const super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows::core::Result<()>;
    fn Add(&mut self, pvalue: *const super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows::core::Result<()>;
    fn GetType(&mut self) -> ::windows::core::Result<u16>;
    fn ChangeType(&mut self, vt: u16) -> ::windows::core::Result<()>;
    fn Clear(&mut self) -> ::windows::core::Result<()>;
    fn RemoveAt(&mut self, dwindex: u32) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
impl IPortableDevicePropVariantCollection_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPortableDevicePropVariantCollection_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPortableDevicePropVariantCollection_Vtbl {
        unsafe extern "system" fn GetCount<Impl: IPortableDevicePropVariantCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcelems: *const u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetCount(::core::mem::transmute_copy(&pcelems)).into()
        }
        unsafe extern "system" fn GetAt<Impl: IPortableDevicePropVariantCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwindex: u32, pvalue: *const super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetAt(::core::mem::transmute_copy(&dwindex), ::core::mem::transmute_copy(&pvalue)).into()
        }
        unsafe extern "system" fn Add<Impl: IPortableDevicePropVariantCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *const super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Add(::core::mem::transmute_copy(&pvalue)).into()
        }
        unsafe extern "system" fn GetType<Impl: IPortableDevicePropVariantCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvt: *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetType() {
                ::core::result::Result::Ok(ok__) => {
                    *pvt = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ChangeType<Impl: IPortableDevicePropVariantCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, vt: u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ChangeType(::core::mem::transmute_copy(&vt)).into()
        }
        unsafe extern "system" fn Clear<Impl: IPortableDevicePropVariantCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Clear().into()
        }
        unsafe extern "system" fn RemoveAt<Impl: IPortableDevicePropVariantCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwindex: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveAt(::core::mem::transmute_copy(&dwindex)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            GetCount: GetCount::<Impl, IMPL_OFFSET>,
            GetAt: GetAt::<Impl, IMPL_OFFSET>,
            Add: Add::<Impl, IMPL_OFFSET>,
            GetType: GetType::<Impl, IMPL_OFFSET>,
            ChangeType: ChangeType::<Impl, IMPL_OFFSET>,
            Clear: Clear::<Impl, IMPL_OFFSET>,
            RemoveAt: RemoveAt::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPortableDevicePropVariantCollection as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Shell_PropertiesSystem"))]
pub trait IPortableDeviceProperties_Impl: Sized {
    fn GetSupportedProperties(&mut self, pszobjectid: super::super::Foundation::PWSTR) -> ::windows::core::Result<IPortableDeviceKeyCollection>;
    fn GetPropertyAttributes(&mut self, pszobjectid: super::super::Foundation::PWSTR, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY) -> ::windows::core::Result<IPortableDeviceValues>;
    fn GetValues(&mut self, pszobjectid: super::super::Foundation::PWSTR, pkeys: ::core::option::Option<IPortableDeviceKeyCollection>) -> ::windows::core::Result<IPortableDeviceValues>;
    fn SetValues(&mut self, pszobjectid: super::super::Foundation::PWSTR, pvalues: ::core::option::Option<IPortableDeviceValues>) -> ::windows::core::Result<IPortableDeviceValues>;
    fn Delete(&mut self, pszobjectid: super::super::Foundation::PWSTR, pkeys: ::core::option::Option<IPortableDeviceKeyCollection>) -> ::windows::core::Result<()>;
    fn Cancel(&mut self) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Shell_PropertiesSystem"))]
impl IPortableDeviceProperties_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPortableDeviceProperties_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPortableDeviceProperties_Vtbl {
        unsafe extern "system" fn GetSupportedProperties<Impl: IPortableDeviceProperties_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszobjectid: super::super::Foundation::PWSTR, ppkeys: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetSupportedProperties(::core::mem::transmute_copy(&pszobjectid)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppkeys = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPropertyAttributes<Impl: IPortableDeviceProperties_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszobjectid: super::super::Foundation::PWSTR, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, ppattributes: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetPropertyAttributes(::core::mem::transmute_copy(&pszobjectid), ::core::mem::transmute_copy(&key)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppattributes = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetValues<Impl: IPortableDeviceProperties_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszobjectid: super::super::Foundation::PWSTR, pkeys: ::windows::core::RawPtr, ppvalues: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetValues(::core::mem::transmute_copy(&pszobjectid), ::core::mem::transmute(&pkeys)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppvalues = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetValues<Impl: IPortableDeviceProperties_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszobjectid: super::super::Foundation::PWSTR, pvalues: ::windows::core::RawPtr, ppresults: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetValues(::core::mem::transmute_copy(&pszobjectid), ::core::mem::transmute(&pvalues)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppresults = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Delete<Impl: IPortableDeviceProperties_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszobjectid: super::super::Foundation::PWSTR, pkeys: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Delete(::core::mem::transmute_copy(&pszobjectid), ::core::mem::transmute(&pkeys)).into()
        }
        unsafe extern "system" fn Cancel<Impl: IPortableDeviceProperties_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Cancel().into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            GetSupportedProperties: GetSupportedProperties::<Impl, IMPL_OFFSET>,
            GetPropertyAttributes: GetPropertyAttributes::<Impl, IMPL_OFFSET>,
            GetValues: GetValues::<Impl, IMPL_OFFSET>,
            SetValues: SetValues::<Impl, IMPL_OFFSET>,
            Delete: Delete::<Impl, IMPL_OFFSET>,
            Cancel: Cancel::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPortableDeviceProperties as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IPortableDevicePropertiesBulk_Impl: Sized {
    fn QueueGetValuesByObjectList(&mut self, pobjectids: ::core::option::Option<IPortableDevicePropVariantCollection>, pkeys: ::core::option::Option<IPortableDeviceKeyCollection>, pcallback: ::core::option::Option<IPortableDevicePropertiesBulkCallback>) -> ::windows::core::Result<::windows::core::GUID>;
    fn QueueGetValuesByObjectFormat(&mut self, pguidobjectformat: *const ::windows::core::GUID, pszparentobjectid: super::super::Foundation::PWSTR, dwdepth: u32, pkeys: ::core::option::Option<IPortableDeviceKeyCollection>, pcallback: ::core::option::Option<IPortableDevicePropertiesBulkCallback>) -> ::windows::core::Result<::windows::core::GUID>;
    fn QueueSetValuesByObjectList(&mut self, pobjectvalues: ::core::option::Option<IPortableDeviceValuesCollection>, pcallback: ::core::option::Option<IPortableDevicePropertiesBulkCallback>) -> ::windows::core::Result<::windows::core::GUID>;
    fn Start(&mut self, pcontext: *const ::windows::core::GUID) -> ::windows::core::Result<()>;
    fn Cancel(&mut self, pcontext: *const ::windows::core::GUID) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IPortableDevicePropertiesBulk_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPortableDevicePropertiesBulk_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPortableDevicePropertiesBulk_Vtbl {
        unsafe extern "system" fn QueueGetValuesByObjectList<Impl: IPortableDevicePropertiesBulk_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pobjectids: ::windows::core::RawPtr, pkeys: ::windows::core::RawPtr, pcallback: ::windows::core::RawPtr, pcontext: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).QueueGetValuesByObjectList(::core::mem::transmute(&pobjectids), ::core::mem::transmute(&pkeys), ::core::mem::transmute(&pcallback)) {
                ::core::result::Result::Ok(ok__) => {
                    *pcontext = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn QueueGetValuesByObjectFormat<Impl: IPortableDevicePropertiesBulk_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pguidobjectformat: *const ::windows::core::GUID, pszparentobjectid: super::super::Foundation::PWSTR, dwdepth: u32, pkeys: ::windows::core::RawPtr, pcallback: ::windows::core::RawPtr, pcontext: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).QueueGetValuesByObjectFormat(::core::mem::transmute_copy(&pguidobjectformat), ::core::mem::transmute_copy(&pszparentobjectid), ::core::mem::transmute_copy(&dwdepth), ::core::mem::transmute(&pkeys), ::core::mem::transmute(&pcallback)) {
                ::core::result::Result::Ok(ok__) => {
                    *pcontext = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn QueueSetValuesByObjectList<Impl: IPortableDevicePropertiesBulk_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pobjectvalues: ::windows::core::RawPtr, pcallback: ::windows::core::RawPtr, pcontext: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).QueueSetValuesByObjectList(::core::mem::transmute(&pobjectvalues), ::core::mem::transmute(&pcallback)) {
                ::core::result::Result::Ok(ok__) => {
                    *pcontext = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Start<Impl: IPortableDevicePropertiesBulk_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcontext: *const ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Start(::core::mem::transmute_copy(&pcontext)).into()
        }
        unsafe extern "system" fn Cancel<Impl: IPortableDevicePropertiesBulk_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcontext: *const ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Cancel(::core::mem::transmute_copy(&pcontext)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            QueueGetValuesByObjectList: QueueGetValuesByObjectList::<Impl, IMPL_OFFSET>,
            QueueGetValuesByObjectFormat: QueueGetValuesByObjectFormat::<Impl, IMPL_OFFSET>,
            QueueSetValuesByObjectList: QueueSetValuesByObjectList::<Impl, IMPL_OFFSET>,
            Start: Start::<Impl, IMPL_OFFSET>,
            Cancel: Cancel::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPortableDevicePropertiesBulk as ::windows::core::Interface>::IID
    }
}
pub trait IPortableDevicePropertiesBulkCallback_Impl: Sized {
    fn OnStart(&mut self, pcontext: *const ::windows::core::GUID) -> ::windows::core::Result<()>;
    fn OnProgress(&mut self, pcontext: *const ::windows::core::GUID, presults: ::core::option::Option<IPortableDeviceValuesCollection>) -> ::windows::core::Result<()>;
    fn OnEnd(&mut self, pcontext: *const ::windows::core::GUID, hrstatus: ::windows::core::HRESULT) -> ::windows::core::Result<()>;
}
impl IPortableDevicePropertiesBulkCallback_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPortableDevicePropertiesBulkCallback_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPortableDevicePropertiesBulkCallback_Vtbl {
        unsafe extern "system" fn OnStart<Impl: IPortableDevicePropertiesBulkCallback_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcontext: *const ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnStart(::core::mem::transmute_copy(&pcontext)).into()
        }
        unsafe extern "system" fn OnProgress<Impl: IPortableDevicePropertiesBulkCallback_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcontext: *const ::windows::core::GUID, presults: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnProgress(::core::mem::transmute_copy(&pcontext), ::core::mem::transmute(&presults)).into()
        }
        unsafe extern "system" fn OnEnd<Impl: IPortableDevicePropertiesBulkCallback_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcontext: *const ::windows::core::GUID, hrstatus: ::windows::core::HRESULT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnEnd(::core::mem::transmute_copy(&pcontext), ::core::mem::transmute_copy(&hrstatus)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            OnStart: OnStart::<Impl, IMPL_OFFSET>,
            OnProgress: OnProgress::<Impl, IMPL_OFFSET>,
            OnEnd: OnEnd::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPortableDevicePropertiesBulkCallback as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_UI_Shell_PropertiesSystem"))]
pub trait IPortableDeviceResources_Impl: Sized {
    fn GetSupportedResources(&mut self, pszobjectid: super::super::Foundation::PWSTR) -> ::windows::core::Result<IPortableDeviceKeyCollection>;
    fn GetResourceAttributes(&mut self, pszobjectid: super::super::Foundation::PWSTR, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY) -> ::windows::core::Result<IPortableDeviceValues>;
    fn GetStream(&mut self, pszobjectid: super::super::Foundation::PWSTR, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, dwmode: u32, pdwoptimalbuffersize: *mut u32, ppstream: *mut ::core::option::Option<super::super::System::Com::IStream>) -> ::windows::core::Result<()>;
    fn Delete(&mut self, pszobjectid: super::super::Foundation::PWSTR, pkeys: ::core::option::Option<IPortableDeviceKeyCollection>) -> ::windows::core::Result<()>;
    fn Cancel(&mut self) -> ::windows::core::Result<()>;
    fn CreateResource(&mut self, presourceattributes: ::core::option::Option<IPortableDeviceValues>, ppdata: *mut ::core::option::Option<super::super::System::Com::IStream>, pdwoptimalwritebuffersize: *mut u32, ppszcookie: *mut super::super::Foundation::PWSTR) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_UI_Shell_PropertiesSystem"))]
impl IPortableDeviceResources_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPortableDeviceResources_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPortableDeviceResources_Vtbl {
        unsafe extern "system" fn GetSupportedResources<Impl: IPortableDeviceResources_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszobjectid: super::super::Foundation::PWSTR, ppkeys: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetSupportedResources(::core::mem::transmute_copy(&pszobjectid)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppkeys = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetResourceAttributes<Impl: IPortableDeviceResources_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszobjectid: super::super::Foundation::PWSTR, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, ppresourceattributes: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetResourceAttributes(::core::mem::transmute_copy(&pszobjectid), ::core::mem::transmute_copy(&key)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppresourceattributes = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetStream<Impl: IPortableDeviceResources_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszobjectid: super::super::Foundation::PWSTR, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, dwmode: u32, pdwoptimalbuffersize: *mut u32, ppstream: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetStream(::core::mem::transmute_copy(&pszobjectid), ::core::mem::transmute_copy(&key), ::core::mem::transmute_copy(&dwmode), ::core::mem::transmute_copy(&pdwoptimalbuffersize), ::core::mem::transmute_copy(&ppstream)).into()
        }
        unsafe extern "system" fn Delete<Impl: IPortableDeviceResources_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszobjectid: super::super::Foundation::PWSTR, pkeys: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Delete(::core::mem::transmute_copy(&pszobjectid), ::core::mem::transmute(&pkeys)).into()
        }
        unsafe extern "system" fn Cancel<Impl: IPortableDeviceResources_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Cancel().into()
        }
        unsafe extern "system" fn CreateResource<Impl: IPortableDeviceResources_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, presourceattributes: ::windows::core::RawPtr, ppdata: *mut ::windows::core::RawPtr, pdwoptimalwritebuffersize: *mut u32, ppszcookie: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).CreateResource(::core::mem::transmute(&presourceattributes), ::core::mem::transmute_copy(&ppdata), ::core::mem::transmute_copy(&pdwoptimalwritebuffersize), ::core::mem::transmute_copy(&ppszcookie)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            GetSupportedResources: GetSupportedResources::<Impl, IMPL_OFFSET>,
            GetResourceAttributes: GetResourceAttributes::<Impl, IMPL_OFFSET>,
            GetStream: GetStream::<Impl, IMPL_OFFSET>,
            Delete: Delete::<Impl, IMPL_OFFSET>,
            Cancel: Cancel::<Impl, IMPL_OFFSET>,
            CreateResource: CreateResource::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPortableDeviceResources as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IPortableDeviceService_Impl: Sized {
    fn Open(&mut self, pszpnpserviceid: super::super::Foundation::PWSTR, pclientinfo: ::core::option::Option<IPortableDeviceValues>) -> ::windows::core::Result<()>;
    fn Capabilities(&mut self) -> ::windows::core::Result<IPortableDeviceServiceCapabilities>;
    fn Content(&mut self) -> ::windows::core::Result<IPortableDeviceContent2>;
    fn Methods(&mut self) -> ::windows::core::Result<IPortableDeviceServiceMethods>;
    fn Cancel(&mut self) -> ::windows::core::Result<()>;
    fn Close(&mut self) -> ::windows::core::Result<()>;
    fn GetServiceObjectID(&mut self) -> ::windows::core::Result<super::super::Foundation::PWSTR>;
    fn GetPnPServiceID(&mut self) -> ::windows::core::Result<super::super::Foundation::PWSTR>;
    fn Advise(&mut self, dwflags: u32, pcallback: ::core::option::Option<IPortableDeviceEventCallback>, pparameters: ::core::option::Option<IPortableDeviceValues>) -> ::windows::core::Result<super::super::Foundation::PWSTR>;
    fn Unadvise(&mut self, pszcookie: super::super::Foundation::PWSTR) -> ::windows::core::Result<()>;
    fn SendCommand(&mut self, dwflags: u32, pparameters: ::core::option::Option<IPortableDeviceValues>) -> ::windows::core::Result<IPortableDeviceValues>;
}
#[cfg(feature = "Win32_Foundation")]
impl IPortableDeviceService_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPortableDeviceService_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPortableDeviceService_Vtbl {
        unsafe extern "system" fn Open<Impl: IPortableDeviceService_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszpnpserviceid: super::super::Foundation::PWSTR, pclientinfo: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Open(::core::mem::transmute_copy(&pszpnpserviceid), ::core::mem::transmute(&pclientinfo)).into()
        }
        unsafe extern "system" fn Capabilities<Impl: IPortableDeviceService_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppcapabilities: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Capabilities() {
                ::core::result::Result::Ok(ok__) => {
                    *ppcapabilities = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Content<Impl: IPortableDeviceService_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppcontent: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Content() {
                ::core::result::Result::Ok(ok__) => {
                    *ppcontent = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Methods<Impl: IPortableDeviceService_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppmethods: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Methods() {
                ::core::result::Result::Ok(ok__) => {
                    *ppmethods = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Cancel<Impl: IPortableDeviceService_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Cancel().into()
        }
        unsafe extern "system" fn Close<Impl: IPortableDeviceService_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Close().into()
        }
        unsafe extern "system" fn GetServiceObjectID<Impl: IPortableDeviceService_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppszserviceobjectid: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetServiceObjectID() {
                ::core::result::Result::Ok(ok__) => {
                    *ppszserviceobjectid = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPnPServiceID<Impl: IPortableDeviceService_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppszpnpserviceid: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetPnPServiceID() {
                ::core::result::Result::Ok(ok__) => {
                    *ppszpnpserviceid = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Advise<Impl: IPortableDeviceService_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwflags: u32, pcallback: ::windows::core::RawPtr, pparameters: ::windows::core::RawPtr, ppszcookie: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Advise(::core::mem::transmute_copy(&dwflags), ::core::mem::transmute(&pcallback), ::core::mem::transmute(&pparameters)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppszcookie = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Unadvise<Impl: IPortableDeviceService_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszcookie: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Unadvise(::core::mem::transmute_copy(&pszcookie)).into()
        }
        unsafe extern "system" fn SendCommand<Impl: IPortableDeviceService_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwflags: u32, pparameters: ::windows::core::RawPtr, ppresults: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SendCommand(::core::mem::transmute_copy(&dwflags), ::core::mem::transmute(&pparameters)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppresults = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            Open: Open::<Impl, IMPL_OFFSET>,
            Capabilities: Capabilities::<Impl, IMPL_OFFSET>,
            Content: Content::<Impl, IMPL_OFFSET>,
            Methods: Methods::<Impl, IMPL_OFFSET>,
            Cancel: Cancel::<Impl, IMPL_OFFSET>,
            Close: Close::<Impl, IMPL_OFFSET>,
            GetServiceObjectID: GetServiceObjectID::<Impl, IMPL_OFFSET>,
            GetPnPServiceID: GetPnPServiceID::<Impl, IMPL_OFFSET>,
            Advise: Advise::<Impl, IMPL_OFFSET>,
            Unadvise: Unadvise::<Impl, IMPL_OFFSET>,
            SendCommand: SendCommand::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPortableDeviceService as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IPortableDeviceServiceActivation_Impl: Sized {
    fn OpenAsync(&mut self, pszpnpserviceid: super::super::Foundation::PWSTR, pclientinfo: ::core::option::Option<IPortableDeviceValues>, pcallback: ::core::option::Option<IPortableDeviceServiceOpenCallback>) -> ::windows::core::Result<()>;
    fn CancelOpenAsync(&mut self) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IPortableDeviceServiceActivation_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPortableDeviceServiceActivation_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPortableDeviceServiceActivation_Vtbl {
        unsafe extern "system" fn OpenAsync<Impl: IPortableDeviceServiceActivation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszpnpserviceid: super::super::Foundation::PWSTR, pclientinfo: ::windows::core::RawPtr, pcallback: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OpenAsync(::core::mem::transmute_copy(&pszpnpserviceid), ::core::mem::transmute(&pclientinfo), ::core::mem::transmute(&pcallback)).into()
        }
        unsafe extern "system" fn CancelOpenAsync<Impl: IPortableDeviceServiceActivation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).CancelOpenAsync().into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            OpenAsync: OpenAsync::<Impl, IMPL_OFFSET>,
            CancelOpenAsync: CancelOpenAsync::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPortableDeviceServiceActivation as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub trait IPortableDeviceServiceCapabilities_Impl: Sized {
    fn GetSupportedMethods(&mut self) -> ::windows::core::Result<IPortableDevicePropVariantCollection>;
    fn GetSupportedMethodsByFormat(&mut self, format: *const ::windows::core::GUID) -> ::windows::core::Result<IPortableDevicePropVariantCollection>;
    fn GetMethodAttributes(&mut self, method: *const ::windows::core::GUID) -> ::windows::core::Result<IPortableDeviceValues>;
    fn GetMethodParameterAttributes(&mut self, method: *const ::windows::core::GUID, parameter: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY) -> ::windows::core::Result<IPortableDeviceValues>;
    fn GetSupportedFormats(&mut self) -> ::windows::core::Result<IPortableDevicePropVariantCollection>;
    fn GetFormatAttributes(&mut self, format: *const ::windows::core::GUID) -> ::windows::core::Result<IPortableDeviceValues>;
    fn GetSupportedFormatProperties(&mut self, format: *const ::windows::core::GUID) -> ::windows::core::Result<IPortableDeviceKeyCollection>;
    fn GetFormatPropertyAttributes(&mut self, format: *const ::windows::core::GUID, property: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY) -> ::windows::core::Result<IPortableDeviceValues>;
    fn GetSupportedEvents(&mut self) -> ::windows::core::Result<IPortableDevicePropVariantCollection>;
    fn GetEventAttributes(&mut self, event: *const ::windows::core::GUID) -> ::windows::core::Result<IPortableDeviceValues>;
    fn GetEventParameterAttributes(&mut self, event: *const ::windows::core::GUID, parameter: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY) -> ::windows::core::Result<IPortableDeviceValues>;
    fn GetInheritedServices(&mut self, dwinheritancetype: u32) -> ::windows::core::Result<IPortableDevicePropVariantCollection>;
    fn GetFormatRenderingProfiles(&mut self, format: *const ::windows::core::GUID) -> ::windows::core::Result<IPortableDeviceValuesCollection>;
    fn GetSupportedCommands(&mut self) -> ::windows::core::Result<IPortableDeviceKeyCollection>;
    fn GetCommandOptions(&mut self, command: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY) -> ::windows::core::Result<IPortableDeviceValues>;
    fn Cancel(&mut self) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
impl IPortableDeviceServiceCapabilities_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPortableDeviceServiceCapabilities_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPortableDeviceServiceCapabilities_Vtbl {
        unsafe extern "system" fn GetSupportedMethods<Impl: IPortableDeviceServiceCapabilities_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppmethods: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetSupportedMethods() {
                ::core::result::Result::Ok(ok__) => {
                    *ppmethods = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSupportedMethodsByFormat<Impl: IPortableDeviceServiceCapabilities_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, format: *const ::windows::core::GUID, ppmethods: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetSupportedMethodsByFormat(::core::mem::transmute_copy(&format)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppmethods = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetMethodAttributes<Impl: IPortableDeviceServiceCapabilities_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, method: *const ::windows::core::GUID, ppattributes: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetMethodAttributes(::core::mem::transmute_copy(&method)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppattributes = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetMethodParameterAttributes<Impl: IPortableDeviceServiceCapabilities_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, method: *const ::windows::core::GUID, parameter: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, ppattributes: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetMethodParameterAttributes(::core::mem::transmute_copy(&method), ::core::mem::transmute_copy(&parameter)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppattributes = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSupportedFormats<Impl: IPortableDeviceServiceCapabilities_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppformats: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetSupportedFormats() {
                ::core::result::Result::Ok(ok__) => {
                    *ppformats = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFormatAttributes<Impl: IPortableDeviceServiceCapabilities_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, format: *const ::windows::core::GUID, ppattributes: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetFormatAttributes(::core::mem::transmute_copy(&format)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppattributes = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSupportedFormatProperties<Impl: IPortableDeviceServiceCapabilities_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, format: *const ::windows::core::GUID, ppkeys: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetSupportedFormatProperties(::core::mem::transmute_copy(&format)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppkeys = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFormatPropertyAttributes<Impl: IPortableDeviceServiceCapabilities_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, format: *const ::windows::core::GUID, property: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, ppattributes: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetFormatPropertyAttributes(::core::mem::transmute_copy(&format), ::core::mem::transmute_copy(&property)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppattributes = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSupportedEvents<Impl: IPortableDeviceServiceCapabilities_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppevents: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetSupportedEvents() {
                ::core::result::Result::Ok(ok__) => {
                    *ppevents = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetEventAttributes<Impl: IPortableDeviceServiceCapabilities_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, event: *const ::windows::core::GUID, ppattributes: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetEventAttributes(::core::mem::transmute_copy(&event)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppattributes = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetEventParameterAttributes<Impl: IPortableDeviceServiceCapabilities_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, event: *const ::windows::core::GUID, parameter: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, ppattributes: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetEventParameterAttributes(::core::mem::transmute_copy(&event), ::core::mem::transmute_copy(&parameter)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppattributes = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetInheritedServices<Impl: IPortableDeviceServiceCapabilities_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwinheritancetype: u32, ppservices: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetInheritedServices(::core::mem::transmute_copy(&dwinheritancetype)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppservices = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFormatRenderingProfiles<Impl: IPortableDeviceServiceCapabilities_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, format: *const ::windows::core::GUID, pprenderingprofiles: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetFormatRenderingProfiles(::core::mem::transmute_copy(&format)) {
                ::core::result::Result::Ok(ok__) => {
                    *pprenderingprofiles = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSupportedCommands<Impl: IPortableDeviceServiceCapabilities_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppcommands: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetSupportedCommands() {
                ::core::result::Result::Ok(ok__) => {
                    *ppcommands = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCommandOptions<Impl: IPortableDeviceServiceCapabilities_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, command: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, ppoptions: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetCommandOptions(::core::mem::transmute_copy(&command)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppoptions = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Cancel<Impl: IPortableDeviceServiceCapabilities_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Cancel().into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            GetSupportedMethods: GetSupportedMethods::<Impl, IMPL_OFFSET>,
            GetSupportedMethodsByFormat: GetSupportedMethodsByFormat::<Impl, IMPL_OFFSET>,
            GetMethodAttributes: GetMethodAttributes::<Impl, IMPL_OFFSET>,
            GetMethodParameterAttributes: GetMethodParameterAttributes::<Impl, IMPL_OFFSET>,
            GetSupportedFormats: GetSupportedFormats::<Impl, IMPL_OFFSET>,
            GetFormatAttributes: GetFormatAttributes::<Impl, IMPL_OFFSET>,
            GetSupportedFormatProperties: GetSupportedFormatProperties::<Impl, IMPL_OFFSET>,
            GetFormatPropertyAttributes: GetFormatPropertyAttributes::<Impl, IMPL_OFFSET>,
            GetSupportedEvents: GetSupportedEvents::<Impl, IMPL_OFFSET>,
            GetEventAttributes: GetEventAttributes::<Impl, IMPL_OFFSET>,
            GetEventParameterAttributes: GetEventParameterAttributes::<Impl, IMPL_OFFSET>,
            GetInheritedServices: GetInheritedServices::<Impl, IMPL_OFFSET>,
            GetFormatRenderingProfiles: GetFormatRenderingProfiles::<Impl, IMPL_OFFSET>,
            GetSupportedCommands: GetSupportedCommands::<Impl, IMPL_OFFSET>,
            GetCommandOptions: GetCommandOptions::<Impl, IMPL_OFFSET>,
            Cancel: Cancel::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPortableDeviceServiceCapabilities as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IPortableDeviceServiceManager_Impl: Sized {
    fn GetDeviceServices(&mut self, pszpnpdeviceid: super::super::Foundation::PWSTR, guidservicecategory: *const ::windows::core::GUID, pservices: *mut super::super::Foundation::PWSTR, pcservices: *mut u32) -> ::windows::core::Result<()>;
    fn GetDeviceForService(&mut self, pszpnpserviceid: super::super::Foundation::PWSTR) -> ::windows::core::Result<super::super::Foundation::PWSTR>;
}
#[cfg(feature = "Win32_Foundation")]
impl IPortableDeviceServiceManager_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPortableDeviceServiceManager_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPortableDeviceServiceManager_Vtbl {
        unsafe extern "system" fn GetDeviceServices<Impl: IPortableDeviceServiceManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszpnpdeviceid: super::super::Foundation::PWSTR, guidservicecategory: *const ::windows::core::GUID, pservices: *mut super::super::Foundation::PWSTR, pcservices: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetDeviceServices(::core::mem::transmute_copy(&pszpnpdeviceid), ::core::mem::transmute_copy(&guidservicecategory), ::core::mem::transmute_copy(&pservices), ::core::mem::transmute_copy(&pcservices)).into()
        }
        unsafe extern "system" fn GetDeviceForService<Impl: IPortableDeviceServiceManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszpnpserviceid: super::super::Foundation::PWSTR, ppszpnpdeviceid: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDeviceForService(::core::mem::transmute_copy(&pszpnpserviceid)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppszpnpdeviceid = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            GetDeviceServices: GetDeviceServices::<Impl, IMPL_OFFSET>,
            GetDeviceForService: GetDeviceForService::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPortableDeviceServiceManager as ::windows::core::Interface>::IID
    }
}
pub trait IPortableDeviceServiceMethodCallback_Impl: Sized {
    fn OnComplete(&mut self, hrstatus: ::windows::core::HRESULT, presults: ::core::option::Option<IPortableDeviceValues>) -> ::windows::core::Result<()>;
}
impl IPortableDeviceServiceMethodCallback_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPortableDeviceServiceMethodCallback_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPortableDeviceServiceMethodCallback_Vtbl {
        unsafe extern "system" fn OnComplete<Impl: IPortableDeviceServiceMethodCallback_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hrstatus: ::windows::core::HRESULT, presults: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnComplete(::core::mem::transmute_copy(&hrstatus), ::core::mem::transmute(&presults)).into()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), OnComplete: OnComplete::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPortableDeviceServiceMethodCallback as ::windows::core::Interface>::IID
    }
}
pub trait IPortableDeviceServiceMethods_Impl: Sized {
    fn Invoke(&mut self, method: *const ::windows::core::GUID, pparameters: ::core::option::Option<IPortableDeviceValues>, ppresults: *mut ::core::option::Option<IPortableDeviceValues>) -> ::windows::core::Result<()>;
    fn InvokeAsync(&mut self, method: *const ::windows::core::GUID, pparameters: ::core::option::Option<IPortableDeviceValues>, pcallback: ::core::option::Option<IPortableDeviceServiceMethodCallback>) -> ::windows::core::Result<()>;
    fn Cancel(&mut self, pcallback: ::core::option::Option<IPortableDeviceServiceMethodCallback>) -> ::windows::core::Result<()>;
}
impl IPortableDeviceServiceMethods_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPortableDeviceServiceMethods_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPortableDeviceServiceMethods_Vtbl {
        unsafe extern "system" fn Invoke<Impl: IPortableDeviceServiceMethods_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, method: *const ::windows::core::GUID, pparameters: ::windows::core::RawPtr, ppresults: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Invoke(::core::mem::transmute_copy(&method), ::core::mem::transmute(&pparameters), ::core::mem::transmute_copy(&ppresults)).into()
        }
        unsafe extern "system" fn InvokeAsync<Impl: IPortableDeviceServiceMethods_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, method: *const ::windows::core::GUID, pparameters: ::windows::core::RawPtr, pcallback: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).InvokeAsync(::core::mem::transmute_copy(&method), ::core::mem::transmute(&pparameters), ::core::mem::transmute(&pcallback)).into()
        }
        unsafe extern "system" fn Cancel<Impl: IPortableDeviceServiceMethods_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcallback: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Cancel(::core::mem::transmute(&pcallback)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            Invoke: Invoke::<Impl, IMPL_OFFSET>,
            InvokeAsync: InvokeAsync::<Impl, IMPL_OFFSET>,
            Cancel: Cancel::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPortableDeviceServiceMethods as ::windows::core::Interface>::IID
    }
}
pub trait IPortableDeviceServiceOpenCallback_Impl: Sized {
    fn OnComplete(&mut self, hrstatus: ::windows::core::HRESULT) -> ::windows::core::Result<()>;
}
impl IPortableDeviceServiceOpenCallback_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPortableDeviceServiceOpenCallback_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPortableDeviceServiceOpenCallback_Vtbl {
        unsafe extern "system" fn OnComplete<Impl: IPortableDeviceServiceOpenCallback_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hrstatus: ::windows::core::HRESULT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnComplete(::core::mem::transmute_copy(&hrstatus)).into()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), OnComplete: OnComplete::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPortableDeviceServiceOpenCallback as ::windows::core::Interface>::IID
    }
}
pub trait IPortableDeviceUnitsStream_Impl: Sized {
    fn SeekInUnits(&mut self, dlibmove: i64, units: WPD_STREAM_UNITS, dworigin: u32) -> ::windows::core::Result<u64>;
    fn Cancel(&mut self) -> ::windows::core::Result<()>;
}
impl IPortableDeviceUnitsStream_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPortableDeviceUnitsStream_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPortableDeviceUnitsStream_Vtbl {
        unsafe extern "system" fn SeekInUnits<Impl: IPortableDeviceUnitsStream_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dlibmove: i64, units: WPD_STREAM_UNITS, dworigin: u32, plibnewposition: *mut u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SeekInUnits(::core::mem::transmute_copy(&dlibmove), ::core::mem::transmute_copy(&units), ::core::mem::transmute_copy(&dworigin)) {
                ::core::result::Result::Ok(ok__) => {
                    *plibnewposition = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Cancel<Impl: IPortableDeviceUnitsStream_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Cancel().into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            SeekInUnits: SeekInUnits::<Impl, IMPL_OFFSET>,
            Cancel: Cancel::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPortableDeviceUnitsStream as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_UI_Shell_PropertiesSystem"))]
pub trait IPortableDeviceValues_Impl: Sized {
    fn GetCount(&mut self, pcelt: *const u32) -> ::windows::core::Result<()>;
    fn GetAt(&mut self, index: u32, pkey: *mut super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, pvalue: *mut super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows::core::Result<()>;
    fn SetValue(&mut self, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, pvalue: *const super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows::core::Result<()>;
    fn GetValue(&mut self, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY) -> ::windows::core::Result<super::super::System::Com::StructuredStorage::PROPVARIANT>;
    fn SetStringValue(&mut self, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, value: super::super::Foundation::PWSTR) -> ::windows::core::Result<()>;
    fn GetStringValue(&mut self, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY) -> ::windows::core::Result<super::super::Foundation::PWSTR>;
    fn SetUnsignedIntegerValue(&mut self, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, value: u32) -> ::windows::core::Result<()>;
    fn GetUnsignedIntegerValue(&mut self, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY) -> ::windows::core::Result<u32>;
    fn SetSignedIntegerValue(&mut self, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, value: i32) -> ::windows::core::Result<()>;
    fn GetSignedIntegerValue(&mut self, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY) -> ::windows::core::Result<i32>;
    fn SetUnsignedLargeIntegerValue(&mut self, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, value: u64) -> ::windows::core::Result<()>;
    fn GetUnsignedLargeIntegerValue(&mut self, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY) -> ::windows::core::Result<u64>;
    fn SetSignedLargeIntegerValue(&mut self, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, value: i64) -> ::windows::core::Result<()>;
    fn GetSignedLargeIntegerValue(&mut self, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY) -> ::windows::core::Result<i64>;
    fn SetFloatValue(&mut self, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, value: f32) -> ::windows::core::Result<()>;
    fn GetFloatValue(&mut self, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY) -> ::windows::core::Result<f32>;
    fn SetErrorValue(&mut self, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, value: ::windows::core::HRESULT) -> ::windows::core::Result<()>;
    fn GetErrorValue(&mut self, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY) -> ::windows::core::Result<::windows::core::HRESULT>;
    fn SetKeyValue(&mut self, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, value: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY) -> ::windows::core::Result<()>;
    fn GetKeyValue(&mut self, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY) -> ::windows::core::Result<super::super::UI::Shell::PropertiesSystem::PROPERTYKEY>;
    fn SetBoolValue(&mut self, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, value: super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn GetBoolValue(&mut self, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY) -> ::windows::core::Result<super::super::Foundation::BOOL>;
    fn SetIUnknownValue(&mut self, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, pvalue: ::core::option::Option<::windows::core::IUnknown>) -> ::windows::core::Result<()>;
    fn GetIUnknownValue(&mut self, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY) -> ::windows::core::Result<::windows::core::IUnknown>;
    fn SetGuidValue(&mut self, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, value: *const ::windows::core::GUID) -> ::windows::core::Result<()>;
    fn GetGuidValue(&mut self, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY) -> ::windows::core::Result<::windows::core::GUID>;
    fn SetBufferValue(&mut self, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, pvalue: *const u8, cbvalue: u32) -> ::windows::core::Result<()>;
    fn GetBufferValue(&mut self, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, ppvalue: *mut *mut u8, pcbvalue: *mut u32) -> ::windows::core::Result<()>;
    fn SetIPortableDeviceValuesValue(&mut self, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, pvalue: ::core::option::Option<IPortableDeviceValues>) -> ::windows::core::Result<()>;
    fn GetIPortableDeviceValuesValue(&mut self, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY) -> ::windows::core::Result<IPortableDeviceValues>;
    fn SetIPortableDevicePropVariantCollectionValue(&mut self, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, pvalue: ::core::option::Option<IPortableDevicePropVariantCollection>) -> ::windows::core::Result<()>;
    fn GetIPortableDevicePropVariantCollectionValue(&mut self, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY) -> ::windows::core::Result<IPortableDevicePropVariantCollection>;
    fn SetIPortableDeviceKeyCollectionValue(&mut self, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, pvalue: ::core::option::Option<IPortableDeviceKeyCollection>) -> ::windows::core::Result<()>;
    fn GetIPortableDeviceKeyCollectionValue(&mut self, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY) -> ::windows::core::Result<IPortableDeviceKeyCollection>;
    fn SetIPortableDeviceValuesCollectionValue(&mut self, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, pvalue: ::core::option::Option<IPortableDeviceValuesCollection>) -> ::windows::core::Result<()>;
    fn GetIPortableDeviceValuesCollectionValue(&mut self, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY) -> ::windows::core::Result<IPortableDeviceValuesCollection>;
    fn RemoveValue(&mut self, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY) -> ::windows::core::Result<()>;
    fn CopyValuesFromPropertyStore(&mut self, pstore: ::core::option::Option<super::super::UI::Shell::PropertiesSystem::IPropertyStore>) -> ::windows::core::Result<()>;
    fn CopyValuesToPropertyStore(&mut self, pstore: ::core::option::Option<super::super::UI::Shell::PropertiesSystem::IPropertyStore>) -> ::windows::core::Result<()>;
    fn Clear(&mut self) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_UI_Shell_PropertiesSystem"))]
impl IPortableDeviceValues_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPortableDeviceValues_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPortableDeviceValues_Vtbl {
        unsafe extern "system" fn GetCount<Impl: IPortableDeviceValues_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcelt: *const u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetCount(::core::mem::transmute_copy(&pcelt)).into()
        }
        unsafe extern "system" fn GetAt<Impl: IPortableDeviceValues_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32, pkey: *mut super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, pvalue: *mut super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetAt(::core::mem::transmute_copy(&index), ::core::mem::transmute_copy(&pkey), ::core::mem::transmute_copy(&pvalue)).into()
        }
        unsafe extern "system" fn SetValue<Impl: IPortableDeviceValues_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, pvalue: *const super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetValue(::core::mem::transmute_copy(&key), ::core::mem::transmute_copy(&pvalue)).into()
        }
        unsafe extern "system" fn GetValue<Impl: IPortableDeviceValues_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, pvalue: *mut super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetValue(::core::mem::transmute_copy(&key)) {
                ::core::result::Result::Ok(ok__) => {
                    *pvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetStringValue<Impl: IPortableDeviceValues_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, value: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetStringValue(::core::mem::transmute_copy(&key), ::core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn GetStringValue<Impl: IPortableDeviceValues_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, pvalue: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetStringValue(::core::mem::transmute_copy(&key)) {
                ::core::result::Result::Ok(ok__) => {
                    *pvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetUnsignedIntegerValue<Impl: IPortableDeviceValues_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, value: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetUnsignedIntegerValue(::core::mem::transmute_copy(&key), ::core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn GetUnsignedIntegerValue<Impl: IPortableDeviceValues_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, pvalue: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetUnsignedIntegerValue(::core::mem::transmute_copy(&key)) {
                ::core::result::Result::Ok(ok__) => {
                    *pvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSignedIntegerValue<Impl: IPortableDeviceValues_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, value: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetSignedIntegerValue(::core::mem::transmute_copy(&key), ::core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn GetSignedIntegerValue<Impl: IPortableDeviceValues_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, pvalue: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetSignedIntegerValue(::core::mem::transmute_copy(&key)) {
                ::core::result::Result::Ok(ok__) => {
                    *pvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetUnsignedLargeIntegerValue<Impl: IPortableDeviceValues_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, value: u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetUnsignedLargeIntegerValue(::core::mem::transmute_copy(&key), ::core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn GetUnsignedLargeIntegerValue<Impl: IPortableDeviceValues_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, pvalue: *mut u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetUnsignedLargeIntegerValue(::core::mem::transmute_copy(&key)) {
                ::core::result::Result::Ok(ok__) => {
                    *pvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSignedLargeIntegerValue<Impl: IPortableDeviceValues_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, value: i64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetSignedLargeIntegerValue(::core::mem::transmute_copy(&key), ::core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn GetSignedLargeIntegerValue<Impl: IPortableDeviceValues_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, pvalue: *mut i64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetSignedLargeIntegerValue(::core::mem::transmute_copy(&key)) {
                ::core::result::Result::Ok(ok__) => {
                    *pvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetFloatValue<Impl: IPortableDeviceValues_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, value: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetFloatValue(::core::mem::transmute_copy(&key), ::core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn GetFloatValue<Impl: IPortableDeviceValues_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, pvalue: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetFloatValue(::core::mem::transmute_copy(&key)) {
                ::core::result::Result::Ok(ok__) => {
                    *pvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetErrorValue<Impl: IPortableDeviceValues_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, value: ::windows::core::HRESULT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetErrorValue(::core::mem::transmute_copy(&key), ::core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn GetErrorValue<Impl: IPortableDeviceValues_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, pvalue: *mut ::windows::core::HRESULT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetErrorValue(::core::mem::transmute_copy(&key)) {
                ::core::result::Result::Ok(ok__) => {
                    *pvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetKeyValue<Impl: IPortableDeviceValues_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, value: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetKeyValue(::core::mem::transmute_copy(&key), ::core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn GetKeyValue<Impl: IPortableDeviceValues_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, pvalue: *mut super::super::UI::Shell::PropertiesSystem::PROPERTYKEY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetKeyValue(::core::mem::transmute_copy(&key)) {
                ::core::result::Result::Ok(ok__) => {
                    *pvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetBoolValue<Impl: IPortableDeviceValues_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, value: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetBoolValue(::core::mem::transmute_copy(&key), ::core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn GetBoolValue<Impl: IPortableDeviceValues_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, pvalue: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetBoolValue(::core::mem::transmute_copy(&key)) {
                ::core::result::Result::Ok(ok__) => {
                    *pvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIUnknownValue<Impl: IPortableDeviceValues_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, pvalue: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetIUnknownValue(::core::mem::transmute_copy(&key), ::core::mem::transmute(&pvalue)).into()
        }
        unsafe extern "system" fn GetIUnknownValue<Impl: IPortableDeviceValues_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, ppvalue: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetIUnknownValue(::core::mem::transmute_copy(&key)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetGuidValue<Impl: IPortableDeviceValues_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, value: *const ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetGuidValue(::core::mem::transmute_copy(&key), ::core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn GetGuidValue<Impl: IPortableDeviceValues_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, pvalue: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetGuidValue(::core::mem::transmute_copy(&key)) {
                ::core::result::Result::Ok(ok__) => {
                    *pvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetBufferValue<Impl: IPortableDeviceValues_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, pvalue: *const u8, cbvalue: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetBufferValue(::core::mem::transmute_copy(&key), ::core::mem::transmute_copy(&pvalue), ::core::mem::transmute_copy(&cbvalue)).into()
        }
        unsafe extern "system" fn GetBufferValue<Impl: IPortableDeviceValues_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, ppvalue: *mut *mut u8, pcbvalue: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetBufferValue(::core::mem::transmute_copy(&key), ::core::mem::transmute_copy(&ppvalue), ::core::mem::transmute_copy(&pcbvalue)).into()
        }
        unsafe extern "system" fn SetIPortableDeviceValuesValue<Impl: IPortableDeviceValues_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, pvalue: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetIPortableDeviceValuesValue(::core::mem::transmute_copy(&key), ::core::mem::transmute(&pvalue)).into()
        }
        unsafe extern "system" fn GetIPortableDeviceValuesValue<Impl: IPortableDeviceValues_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, ppvalue: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetIPortableDeviceValuesValue(::core::mem::transmute_copy(&key)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIPortableDevicePropVariantCollectionValue<Impl: IPortableDeviceValues_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, pvalue: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetIPortableDevicePropVariantCollectionValue(::core::mem::transmute_copy(&key), ::core::mem::transmute(&pvalue)).into()
        }
        unsafe extern "system" fn GetIPortableDevicePropVariantCollectionValue<Impl: IPortableDeviceValues_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, ppvalue: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetIPortableDevicePropVariantCollectionValue(::core::mem::transmute_copy(&key)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIPortableDeviceKeyCollectionValue<Impl: IPortableDeviceValues_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, pvalue: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetIPortableDeviceKeyCollectionValue(::core::mem::transmute_copy(&key), ::core::mem::transmute(&pvalue)).into()
        }
        unsafe extern "system" fn GetIPortableDeviceKeyCollectionValue<Impl: IPortableDeviceValues_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, ppvalue: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetIPortableDeviceKeyCollectionValue(::core::mem::transmute_copy(&key)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIPortableDeviceValuesCollectionValue<Impl: IPortableDeviceValues_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, pvalue: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetIPortableDeviceValuesCollectionValue(::core::mem::transmute_copy(&key), ::core::mem::transmute(&pvalue)).into()
        }
        unsafe extern "system" fn GetIPortableDeviceValuesCollectionValue<Impl: IPortableDeviceValues_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, ppvalue: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetIPortableDeviceValuesCollectionValue(::core::mem::transmute_copy(&key)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveValue<Impl: IPortableDeviceValues_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveValue(::core::mem::transmute_copy(&key)).into()
        }
        unsafe extern "system" fn CopyValuesFromPropertyStore<Impl: IPortableDeviceValues_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pstore: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).CopyValuesFromPropertyStore(::core::mem::transmute(&pstore)).into()
        }
        unsafe extern "system" fn CopyValuesToPropertyStore<Impl: IPortableDeviceValues_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pstore: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).CopyValuesToPropertyStore(::core::mem::transmute(&pstore)).into()
        }
        unsafe extern "system" fn Clear<Impl: IPortableDeviceValues_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Clear().into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            GetCount: GetCount::<Impl, IMPL_OFFSET>,
            GetAt: GetAt::<Impl, IMPL_OFFSET>,
            SetValue: SetValue::<Impl, IMPL_OFFSET>,
            GetValue: GetValue::<Impl, IMPL_OFFSET>,
            SetStringValue: SetStringValue::<Impl, IMPL_OFFSET>,
            GetStringValue: GetStringValue::<Impl, IMPL_OFFSET>,
            SetUnsignedIntegerValue: SetUnsignedIntegerValue::<Impl, IMPL_OFFSET>,
            GetUnsignedIntegerValue: GetUnsignedIntegerValue::<Impl, IMPL_OFFSET>,
            SetSignedIntegerValue: SetSignedIntegerValue::<Impl, IMPL_OFFSET>,
            GetSignedIntegerValue: GetSignedIntegerValue::<Impl, IMPL_OFFSET>,
            SetUnsignedLargeIntegerValue: SetUnsignedLargeIntegerValue::<Impl, IMPL_OFFSET>,
            GetUnsignedLargeIntegerValue: GetUnsignedLargeIntegerValue::<Impl, IMPL_OFFSET>,
            SetSignedLargeIntegerValue: SetSignedLargeIntegerValue::<Impl, IMPL_OFFSET>,
            GetSignedLargeIntegerValue: GetSignedLargeIntegerValue::<Impl, IMPL_OFFSET>,
            SetFloatValue: SetFloatValue::<Impl, IMPL_OFFSET>,
            GetFloatValue: GetFloatValue::<Impl, IMPL_OFFSET>,
            SetErrorValue: SetErrorValue::<Impl, IMPL_OFFSET>,
            GetErrorValue: GetErrorValue::<Impl, IMPL_OFFSET>,
            SetKeyValue: SetKeyValue::<Impl, IMPL_OFFSET>,
            GetKeyValue: GetKeyValue::<Impl, IMPL_OFFSET>,
            SetBoolValue: SetBoolValue::<Impl, IMPL_OFFSET>,
            GetBoolValue: GetBoolValue::<Impl, IMPL_OFFSET>,
            SetIUnknownValue: SetIUnknownValue::<Impl, IMPL_OFFSET>,
            GetIUnknownValue: GetIUnknownValue::<Impl, IMPL_OFFSET>,
            SetGuidValue: SetGuidValue::<Impl, IMPL_OFFSET>,
            GetGuidValue: GetGuidValue::<Impl, IMPL_OFFSET>,
            SetBufferValue: SetBufferValue::<Impl, IMPL_OFFSET>,
            GetBufferValue: GetBufferValue::<Impl, IMPL_OFFSET>,
            SetIPortableDeviceValuesValue: SetIPortableDeviceValuesValue::<Impl, IMPL_OFFSET>,
            GetIPortableDeviceValuesValue: GetIPortableDeviceValuesValue::<Impl, IMPL_OFFSET>,
            SetIPortableDevicePropVariantCollectionValue: SetIPortableDevicePropVariantCollectionValue::<Impl, IMPL_OFFSET>,
            GetIPortableDevicePropVariantCollectionValue: GetIPortableDevicePropVariantCollectionValue::<Impl, IMPL_OFFSET>,
            SetIPortableDeviceKeyCollectionValue: SetIPortableDeviceKeyCollectionValue::<Impl, IMPL_OFFSET>,
            GetIPortableDeviceKeyCollectionValue: GetIPortableDeviceKeyCollectionValue::<Impl, IMPL_OFFSET>,
            SetIPortableDeviceValuesCollectionValue: SetIPortableDeviceValuesCollectionValue::<Impl, IMPL_OFFSET>,
            GetIPortableDeviceValuesCollectionValue: GetIPortableDeviceValuesCollectionValue::<Impl, IMPL_OFFSET>,
            RemoveValue: RemoveValue::<Impl, IMPL_OFFSET>,
            CopyValuesFromPropertyStore: CopyValuesFromPropertyStore::<Impl, IMPL_OFFSET>,
            CopyValuesToPropertyStore: CopyValuesToPropertyStore::<Impl, IMPL_OFFSET>,
            Clear: Clear::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPortableDeviceValues as ::windows::core::Interface>::IID
    }
}
pub trait IPortableDeviceValuesCollection_Impl: Sized {
    fn GetCount(&mut self, pcelems: *const u32) -> ::windows::core::Result<()>;
    fn GetAt(&mut self, dwindex: u32) -> ::windows::core::Result<IPortableDeviceValues>;
    fn Add(&mut self, pvalues: ::core::option::Option<IPortableDeviceValues>) -> ::windows::core::Result<()>;
    fn Clear(&mut self) -> ::windows::core::Result<()>;
    fn RemoveAt(&mut self, dwindex: u32) -> ::windows::core::Result<()>;
}
impl IPortableDeviceValuesCollection_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPortableDeviceValuesCollection_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPortableDeviceValuesCollection_Vtbl {
        unsafe extern "system" fn GetCount<Impl: IPortableDeviceValuesCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcelems: *const u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetCount(::core::mem::transmute_copy(&pcelems)).into()
        }
        unsafe extern "system" fn GetAt<Impl: IPortableDeviceValuesCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwindex: u32, ppvalues: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetAt(::core::mem::transmute_copy(&dwindex)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppvalues = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Add<Impl: IPortableDeviceValuesCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalues: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Add(::core::mem::transmute(&pvalues)).into()
        }
        unsafe extern "system" fn Clear<Impl: IPortableDeviceValuesCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Clear().into()
        }
        unsafe extern "system" fn RemoveAt<Impl: IPortableDeviceValuesCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwindex: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveAt(::core::mem::transmute_copy(&dwindex)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            GetCount: GetCount::<Impl, IMPL_OFFSET>,
            GetAt: GetAt::<Impl, IMPL_OFFSET>,
            Add: Add::<Impl, IMPL_OFFSET>,
            Clear: Clear::<Impl, IMPL_OFFSET>,
            RemoveAt: RemoveAt::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPortableDeviceValuesCollection as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IPortableDeviceWebControl_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn GetDeviceFromId(&mut self, deviceid: super::super::Foundation::BSTR) -> ::windows::core::Result<super::super::System::Com::IDispatch>;
    fn GetDeviceFromIdAsync(&mut self, deviceid: super::super::Foundation::BSTR, pcompletionhandler: ::core::option::Option<super::super::System::Com::IDispatch>, perrorhandler: ::core::option::Option<super::super::System::Com::IDispatch>) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IPortableDeviceWebControl_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPortableDeviceWebControl_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPortableDeviceWebControl_Vtbl {
        unsafe extern "system" fn GetDeviceFromId<Impl: IPortableDeviceWebControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, deviceid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ppdevice: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDeviceFromId(::core::mem::transmute_copy(&deviceid)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppdevice = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDeviceFromIdAsync<Impl: IPortableDeviceWebControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, deviceid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pcompletionhandler: ::windows::core::RawPtr, perrorhandler: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetDeviceFromIdAsync(::core::mem::transmute_copy(&deviceid), ::core::mem::transmute(&pcompletionhandler), ::core::mem::transmute(&perrorhandler)).into()
        }
        Self {
            base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            GetDeviceFromId: GetDeviceFromId::<Impl, IMPL_OFFSET>,
            GetDeviceFromIdAsync: GetDeviceFromIdAsync::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPortableDeviceWebControl as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IRadioInstance_Impl: Sized {
    fn GetRadioManagerSignature(&mut self) -> ::windows::core::Result<::windows::core::GUID>;
    fn GetInstanceSignature(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn GetFriendlyName(&mut self, lcid: u32) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn GetRadioState(&mut self) -> ::windows::core::Result<DEVICE_RADIO_STATE>;
    fn SetRadioState(&mut self, radiostate: DEVICE_RADIO_STATE, utimeoutsec: u32) -> ::windows::core::Result<()>;
    fn IsMultiComm(&mut self) -> super::super::Foundation::BOOL;
    fn IsAssociatingDevice(&mut self) -> super::super::Foundation::BOOL;
}
#[cfg(feature = "Win32_Foundation")]
impl IRadioInstance_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRadioInstance_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IRadioInstance_Vtbl {
        unsafe extern "system" fn GetRadioManagerSignature<Impl: IRadioInstance_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pguidsignature: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetRadioManagerSignature() {
                ::core::result::Result::Ok(ok__) => {
                    *pguidsignature = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetInstanceSignature<Impl: IRadioInstance_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrid: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetInstanceSignature() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstrid = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFriendlyName<Impl: IRadioInstance_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lcid: u32, pbstrname: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetFriendlyName(::core::mem::transmute_copy(&lcid)) {
                ::core::result::Result::Ok(ok__) => {
                    *pbstrname = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetRadioState<Impl: IRadioInstance_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pradiostate: *mut DEVICE_RADIO_STATE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetRadioState() {
                ::core::result::Result::Ok(ok__) => {
                    *pradiostate = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetRadioState<Impl: IRadioInstance_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, radiostate: DEVICE_RADIO_STATE, utimeoutsec: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetRadioState(::core::mem::transmute_copy(&radiostate), ::core::mem::transmute_copy(&utimeoutsec)).into()
        }
        unsafe extern "system" fn IsMultiComm<Impl: IRadioInstance_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).IsMultiComm()
        }
        unsafe extern "system" fn IsAssociatingDevice<Impl: IRadioInstance_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).IsAssociatingDevice()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            GetRadioManagerSignature: GetRadioManagerSignature::<Impl, IMPL_OFFSET>,
            GetInstanceSignature: GetInstanceSignature::<Impl, IMPL_OFFSET>,
            GetFriendlyName: GetFriendlyName::<Impl, IMPL_OFFSET>,
            GetRadioState: GetRadioState::<Impl, IMPL_OFFSET>,
            SetRadioState: SetRadioState::<Impl, IMPL_OFFSET>,
            IsMultiComm: IsMultiComm::<Impl, IMPL_OFFSET>,
            IsAssociatingDevice: IsAssociatingDevice::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRadioInstance as ::windows::core::Interface>::IID
    }
}
pub trait IRadioInstanceCollection_Impl: Sized {
    fn GetCount(&mut self) -> ::windows::core::Result<u32>;
    fn GetAt(&mut self, uindex: u32) -> ::windows::core::Result<IRadioInstance>;
}
impl IRadioInstanceCollection_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRadioInstanceCollection_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IRadioInstanceCollection_Vtbl {
        unsafe extern "system" fn GetCount<Impl: IRadioInstanceCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcinstance: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetCount() {
                ::core::result::Result::Ok(ok__) => {
                    *pcinstance = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetAt<Impl: IRadioInstanceCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, uindex: u32, ppradioinstance: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetAt(::core::mem::transmute_copy(&uindex)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppradioinstance = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), GetCount: GetCount::<Impl, IMPL_OFFSET>, GetAt: GetAt::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRadioInstanceCollection as ::windows::core::Interface>::IID
    }
}
pub trait IWpdSerializer_Impl: Sized {
    fn GetIPortableDeviceValuesFromBuffer(&mut self, pbuffer: *const u8, dwinputbufferlength: u32) -> ::windows::core::Result<IPortableDeviceValues>;
    fn WriteIPortableDeviceValuesToBuffer(&mut self, dwoutputbufferlength: u32, presults: ::core::option::Option<IPortableDeviceValues>, pbuffer: *mut u8, pdwbyteswritten: *mut u32) -> ::windows::core::Result<()>;
    fn GetBufferFromIPortableDeviceValues(&mut self, psource: ::core::option::Option<IPortableDeviceValues>, ppbuffer: *mut *mut u8, pdwbuffersize: *mut u32) -> ::windows::core::Result<()>;
    fn GetSerializedSize(&mut self, psource: ::core::option::Option<IPortableDeviceValues>) -> ::windows::core::Result<u32>;
}
impl IWpdSerializer_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWpdSerializer_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWpdSerializer_Vtbl {
        unsafe extern "system" fn GetIPortableDeviceValuesFromBuffer<Impl: IWpdSerializer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbuffer: *const u8, dwinputbufferlength: u32, ppparams: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetIPortableDeviceValuesFromBuffer(::core::mem::transmute_copy(&pbuffer), ::core::mem::transmute_copy(&dwinputbufferlength)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppparams = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn WriteIPortableDeviceValuesToBuffer<Impl: IWpdSerializer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwoutputbufferlength: u32, presults: ::windows::core::RawPtr, pbuffer: *mut u8, pdwbyteswritten: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).WriteIPortableDeviceValuesToBuffer(::core::mem::transmute_copy(&dwoutputbufferlength), ::core::mem::transmute(&presults), ::core::mem::transmute_copy(&pbuffer), ::core::mem::transmute_copy(&pdwbyteswritten)).into()
        }
        unsafe extern "system" fn GetBufferFromIPortableDeviceValues<Impl: IWpdSerializer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psource: ::windows::core::RawPtr, ppbuffer: *mut *mut u8, pdwbuffersize: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetBufferFromIPortableDeviceValues(::core::mem::transmute(&psource), ::core::mem::transmute_copy(&ppbuffer), ::core::mem::transmute_copy(&pdwbuffersize)).into()
        }
        unsafe extern "system" fn GetSerializedSize<Impl: IWpdSerializer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psource: ::windows::core::RawPtr, pdwsize: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetSerializedSize(::core::mem::transmute(&psource)) {
                ::core::result::Result::Ok(ok__) => {
                    *pdwsize = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            GetIPortableDeviceValuesFromBuffer: GetIPortableDeviceValuesFromBuffer::<Impl, IMPL_OFFSET>,
            WriteIPortableDeviceValuesToBuffer: WriteIPortableDeviceValuesToBuffer::<Impl, IMPL_OFFSET>,
            GetBufferFromIPortableDeviceValues: GetBufferFromIPortableDeviceValues::<Impl, IMPL_OFFSET>,
            GetSerializedSize: GetSerializedSize::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWpdSerializer as ::windows::core::Interface>::IID
    }
}
