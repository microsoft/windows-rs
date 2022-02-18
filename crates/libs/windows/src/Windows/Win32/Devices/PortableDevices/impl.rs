pub trait IConnectionRequestCallback_Impl: Sized {
    fn OnComplete(&self, hrstatus: ::windows::core::HRESULT) -> ::windows::core::Result<()>;
}
impl IConnectionRequestCallback_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IConnectionRequestCallback_Impl, const OFFSET: isize>() -> IConnectionRequestCallback_Vtbl {
        unsafe extern "system" fn OnComplete<Identity: ::windows::core::IUnknownImpl, Impl: IConnectionRequestCallback_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hrstatus: ::windows::core::HRESULT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).OnComplete(::core::mem::transmute_copy(&hrstatus)).into()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(), OnComplete: OnComplete::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IConnectionRequestCallback as ::windows::core::Interface>::IID
    }
}
pub trait IEnumPortableDeviceConnectors_Impl: Sized {
    fn Next(&self, crequested: u32, pconnectors: *mut ::core::option::Option<IPortableDeviceConnector>, pcfetched: *mut u32) -> ::windows::core::Result<()>;
    fn Skip(&self, cconnectors: u32) -> ::windows::core::Result<()>;
    fn Reset(&self) -> ::windows::core::Result<()>;
    fn Clone(&self) -> ::windows::core::Result<IEnumPortableDeviceConnectors>;
}
impl IEnumPortableDeviceConnectors_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IEnumPortableDeviceConnectors_Impl, const OFFSET: isize>() -> IEnumPortableDeviceConnectors_Vtbl {
        unsafe extern "system" fn Next<Identity: ::windows::core::IUnknownImpl, Impl: IEnumPortableDeviceConnectors_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, crequested: u32, pconnectors: *mut ::windows::core::RawPtr, pcfetched: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Next(::core::mem::transmute_copy(&crequested), ::core::mem::transmute_copy(&pconnectors), ::core::mem::transmute_copy(&pcfetched)).into()
        }
        unsafe extern "system" fn Skip<Identity: ::windows::core::IUnknownImpl, Impl: IEnumPortableDeviceConnectors_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cconnectors: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Skip(::core::mem::transmute_copy(&cconnectors)).into()
        }
        unsafe extern "system" fn Reset<Identity: ::windows::core::IUnknownImpl, Impl: IEnumPortableDeviceConnectors_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Reset().into()
        }
        unsafe extern "system" fn Clone<Identity: ::windows::core::IUnknownImpl, Impl: IEnumPortableDeviceConnectors_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        iid == &<IEnumPortableDeviceConnectors as ::windows::core::Interface>::IID
    }
}
pub trait IEnumPortableDeviceObjectIDs_Impl: Sized {
    fn Next(&self, cobjects: u32, pobjids: *mut ::windows::core::PWSTR, pcfetched: *mut u32) -> ::windows::core::HRESULT;
    fn Skip(&self, cobjects: u32) -> ::windows::core::HRESULT;
    fn Reset(&self) -> ::windows::core::Result<()>;
    fn Clone(&self) -> ::windows::core::Result<IEnumPortableDeviceObjectIDs>;
    fn Cancel(&self) -> ::windows::core::Result<()>;
}
impl IEnumPortableDeviceObjectIDs_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IEnumPortableDeviceObjectIDs_Impl, const OFFSET: isize>() -> IEnumPortableDeviceObjectIDs_Vtbl {
        unsafe extern "system" fn Next<Identity: ::windows::core::IUnknownImpl, Impl: IEnumPortableDeviceObjectIDs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cobjects: u32, pobjids: *mut ::windows::core::PWSTR, pcfetched: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Next(::core::mem::transmute_copy(&cobjects), ::core::mem::transmute_copy(&pobjids), ::core::mem::transmute_copy(&pcfetched))
        }
        unsafe extern "system" fn Skip<Identity: ::windows::core::IUnknownImpl, Impl: IEnumPortableDeviceObjectIDs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cobjects: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Skip(::core::mem::transmute_copy(&cobjects))
        }
        unsafe extern "system" fn Reset<Identity: ::windows::core::IUnknownImpl, Impl: IEnumPortableDeviceObjectIDs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Reset().into()
        }
        unsafe extern "system" fn Clone<Identity: ::windows::core::IUnknownImpl, Impl: IEnumPortableDeviceObjectIDs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Cancel<Identity: ::windows::core::IUnknownImpl, Impl: IEnumPortableDeviceObjectIDs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Cancel().into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            Next: Next::<Identity, Impl, OFFSET>,
            Skip: Skip::<Identity, Impl, OFFSET>,
            Reset: Reset::<Identity, Impl, OFFSET>,
            Clone: Clone::<Identity, Impl, OFFSET>,
            Cancel: Cancel::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IEnumPortableDeviceObjectIDs as ::windows::core::Interface>::IID
    }
}
pub trait IMediaRadioManager_Impl: Sized {
    fn GetRadioInstances(&self) -> ::windows::core::Result<IRadioInstanceCollection>;
    fn OnSystemRadioStateChange(&self, sysradiostate: SYSTEM_RADIO_STATE, utimeoutsec: u32) -> ::windows::core::Result<()>;
}
impl IMediaRadioManager_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMediaRadioManager_Impl, const OFFSET: isize>() -> IMediaRadioManager_Vtbl {
        unsafe extern "system" fn GetRadioInstances<Identity: ::windows::core::IUnknownImpl, Impl: IMediaRadioManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppcollection: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetRadioInstances() {
                ::core::result::Result::Ok(ok__) => {
                    *ppcollection = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OnSystemRadioStateChange<Identity: ::windows::core::IUnknownImpl, Impl: IMediaRadioManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, sysradiostate: SYSTEM_RADIO_STATE, utimeoutsec: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).OnSystemRadioStateChange(::core::mem::transmute_copy(&sysradiostate), ::core::mem::transmute_copy(&utimeoutsec)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            GetRadioInstances: GetRadioInstances::<Identity, Impl, OFFSET>,
            OnSystemRadioStateChange: OnSystemRadioStateChange::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMediaRadioManager as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IMediaRadioManagerNotifySink_Impl: Sized {
    fn OnInstanceAdd(&self, pradioinstance: &::core::option::Option<IRadioInstance>) -> ::windows::core::Result<()>;
    fn OnInstanceRemove(&self, bstrradioinstanceid: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn OnInstanceRadioChange(&self, bstrradioinstanceid: &super::super::Foundation::BSTR, radiostate: DEVICE_RADIO_STATE) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IMediaRadioManagerNotifySink_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMediaRadioManagerNotifySink_Impl, const OFFSET: isize>() -> IMediaRadioManagerNotifySink_Vtbl {
        unsafe extern "system" fn OnInstanceAdd<Identity: ::windows::core::IUnknownImpl, Impl: IMediaRadioManagerNotifySink_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pradioinstance: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).OnInstanceAdd(::core::mem::transmute(&pradioinstance)).into()
        }
        unsafe extern "system" fn OnInstanceRemove<Identity: ::windows::core::IUnknownImpl, Impl: IMediaRadioManagerNotifySink_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrradioinstanceid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).OnInstanceRemove(::core::mem::transmute(&bstrradioinstanceid)).into()
        }
        unsafe extern "system" fn OnInstanceRadioChange<Identity: ::windows::core::IUnknownImpl, Impl: IMediaRadioManagerNotifySink_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrradioinstanceid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, radiostate: DEVICE_RADIO_STATE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).OnInstanceRadioChange(::core::mem::transmute(&bstrradioinstanceid), ::core::mem::transmute_copy(&radiostate)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            OnInstanceAdd: OnInstanceAdd::<Identity, Impl, OFFSET>,
            OnInstanceRemove: OnInstanceRemove::<Identity, Impl, OFFSET>,
            OnInstanceRadioChange: OnInstanceRadioChange::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMediaRadioManagerNotifySink as ::windows::core::Interface>::IID
    }
}
pub trait IPortableDevice_Impl: Sized {
    fn Open(&self, pszpnpdeviceid: &::windows::core::PCWSTR, pclientinfo: &::core::option::Option<IPortableDeviceValues>) -> ::windows::core::Result<()>;
    fn SendCommand(&self, dwflags: u32, pparameters: &::core::option::Option<IPortableDeviceValues>) -> ::windows::core::Result<IPortableDeviceValues>;
    fn Content(&self) -> ::windows::core::Result<IPortableDeviceContent>;
    fn Capabilities(&self) -> ::windows::core::Result<IPortableDeviceCapabilities>;
    fn Cancel(&self) -> ::windows::core::Result<()>;
    fn Close(&self) -> ::windows::core::Result<()>;
    fn Advise(&self, dwflags: u32, pcallback: &::core::option::Option<IPortableDeviceEventCallback>, pparameters: &::core::option::Option<IPortableDeviceValues>) -> ::windows::core::Result<::windows::core::PWSTR>;
    fn Unadvise(&self, pszcookie: &::windows::core::PCWSTR) -> ::windows::core::Result<()>;
    fn GetPnPDeviceID(&self) -> ::windows::core::Result<::windows::core::PWSTR>;
}
impl IPortableDevice_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPortableDevice_Impl, const OFFSET: isize>() -> IPortableDevice_Vtbl {
        unsafe extern "system" fn Open<Identity: ::windows::core::IUnknownImpl, Impl: IPortableDevice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszpnpdeviceid: ::windows::core::PCWSTR, pclientinfo: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Open(::core::mem::transmute(&pszpnpdeviceid), ::core::mem::transmute(&pclientinfo)).into()
        }
        unsafe extern "system" fn SendCommand<Identity: ::windows::core::IUnknownImpl, Impl: IPortableDevice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwflags: u32, pparameters: ::windows::core::RawPtr, ppresults: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).SendCommand(::core::mem::transmute_copy(&dwflags), ::core::mem::transmute(&pparameters)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppresults = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Content<Identity: ::windows::core::IUnknownImpl, Impl: IPortableDevice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppcontent: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Content() {
                ::core::result::Result::Ok(ok__) => {
                    *ppcontent = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Capabilities<Identity: ::windows::core::IUnknownImpl, Impl: IPortableDevice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppcapabilities: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Capabilities() {
                ::core::result::Result::Ok(ok__) => {
                    *ppcapabilities = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Cancel<Identity: ::windows::core::IUnknownImpl, Impl: IPortableDevice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Cancel().into()
        }
        unsafe extern "system" fn Close<Identity: ::windows::core::IUnknownImpl, Impl: IPortableDevice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Close().into()
        }
        unsafe extern "system" fn Advise<Identity: ::windows::core::IUnknownImpl, Impl: IPortableDevice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwflags: u32, pcallback: ::windows::core::RawPtr, pparameters: ::windows::core::RawPtr, ppszcookie: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Advise(::core::mem::transmute_copy(&dwflags), ::core::mem::transmute(&pcallback), ::core::mem::transmute(&pparameters)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppszcookie = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Unadvise<Identity: ::windows::core::IUnknownImpl, Impl: IPortableDevice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszcookie: ::windows::core::PCWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Unadvise(::core::mem::transmute(&pszcookie)).into()
        }
        unsafe extern "system" fn GetPnPDeviceID<Identity: ::windows::core::IUnknownImpl, Impl: IPortableDevice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppszpnpdeviceid: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetPnPDeviceID() {
                ::core::result::Result::Ok(ok__) => {
                    *ppszpnpdeviceid = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            Open: Open::<Identity, Impl, OFFSET>,
            SendCommand: SendCommand::<Identity, Impl, OFFSET>,
            Content: Content::<Identity, Impl, OFFSET>,
            Capabilities: Capabilities::<Identity, Impl, OFFSET>,
            Cancel: Cancel::<Identity, Impl, OFFSET>,
            Close: Close::<Identity, Impl, OFFSET>,
            Advise: Advise::<Identity, Impl, OFFSET>,
            Unadvise: Unadvise::<Identity, Impl, OFFSET>,
            GetPnPDeviceID: GetPnPDeviceID::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPortableDevice as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub trait IPortableDeviceCapabilities_Impl: Sized {
    fn GetSupportedCommands(&self) -> ::windows::core::Result<IPortableDeviceKeyCollection>;
    fn GetCommandOptions(&self, command: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY) -> ::windows::core::Result<IPortableDeviceValues>;
    fn GetFunctionalCategories(&self) -> ::windows::core::Result<IPortableDevicePropVariantCollection>;
    fn GetFunctionalObjects(&self, category: *const ::windows::core::GUID) -> ::windows::core::Result<IPortableDevicePropVariantCollection>;
    fn GetSupportedContentTypes(&self, category: *const ::windows::core::GUID) -> ::windows::core::Result<IPortableDevicePropVariantCollection>;
    fn GetSupportedFormats(&self, contenttype: *const ::windows::core::GUID) -> ::windows::core::Result<IPortableDevicePropVariantCollection>;
    fn GetSupportedFormatProperties(&self, format: *const ::windows::core::GUID) -> ::windows::core::Result<IPortableDeviceKeyCollection>;
    fn GetFixedPropertyAttributes(&self, format: *const ::windows::core::GUID, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY) -> ::windows::core::Result<IPortableDeviceValues>;
    fn Cancel(&self) -> ::windows::core::Result<()>;
    fn GetSupportedEvents(&self) -> ::windows::core::Result<IPortableDevicePropVariantCollection>;
    fn GetEventOptions(&self, event: *const ::windows::core::GUID) -> ::windows::core::Result<IPortableDeviceValues>;
}
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
impl IPortableDeviceCapabilities_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPortableDeviceCapabilities_Impl, const OFFSET: isize>() -> IPortableDeviceCapabilities_Vtbl {
        unsafe extern "system" fn GetSupportedCommands<Identity: ::windows::core::IUnknownImpl, Impl: IPortableDeviceCapabilities_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppcommands: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetSupportedCommands() {
                ::core::result::Result::Ok(ok__) => {
                    *ppcommands = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCommandOptions<Identity: ::windows::core::IUnknownImpl, Impl: IPortableDeviceCapabilities_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, command: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, ppoptions: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetCommandOptions(::core::mem::transmute_copy(&command)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppoptions = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFunctionalCategories<Identity: ::windows::core::IUnknownImpl, Impl: IPortableDeviceCapabilities_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppcategories: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetFunctionalCategories() {
                ::core::result::Result::Ok(ok__) => {
                    *ppcategories = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFunctionalObjects<Identity: ::windows::core::IUnknownImpl, Impl: IPortableDeviceCapabilities_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, category: *const ::windows::core::GUID, ppobjectids: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetFunctionalObjects(::core::mem::transmute_copy(&category)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppobjectids = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSupportedContentTypes<Identity: ::windows::core::IUnknownImpl, Impl: IPortableDeviceCapabilities_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, category: *const ::windows::core::GUID, ppcontenttypes: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetSupportedContentTypes(::core::mem::transmute_copy(&category)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppcontenttypes = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSupportedFormats<Identity: ::windows::core::IUnknownImpl, Impl: IPortableDeviceCapabilities_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, contenttype: *const ::windows::core::GUID, ppformats: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetSupportedFormats(::core::mem::transmute_copy(&contenttype)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppformats = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSupportedFormatProperties<Identity: ::windows::core::IUnknownImpl, Impl: IPortableDeviceCapabilities_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, format: *const ::windows::core::GUID, ppkeys: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetSupportedFormatProperties(::core::mem::transmute_copy(&format)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppkeys = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFixedPropertyAttributes<Identity: ::windows::core::IUnknownImpl, Impl: IPortableDeviceCapabilities_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, format: *const ::windows::core::GUID, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, ppattributes: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetFixedPropertyAttributes(::core::mem::transmute_copy(&format), ::core::mem::transmute_copy(&key)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppattributes = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Cancel<Identity: ::windows::core::IUnknownImpl, Impl: IPortableDeviceCapabilities_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Cancel().into()
        }
        unsafe extern "system" fn GetSupportedEvents<Identity: ::windows::core::IUnknownImpl, Impl: IPortableDeviceCapabilities_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppevents: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetSupportedEvents() {
                ::core::result::Result::Ok(ok__) => {
                    *ppevents = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetEventOptions<Identity: ::windows::core::IUnknownImpl, Impl: IPortableDeviceCapabilities_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, event: *const ::windows::core::GUID, ppoptions: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetEventOptions(::core::mem::transmute_copy(&event)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppoptions = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            GetSupportedCommands: GetSupportedCommands::<Identity, Impl, OFFSET>,
            GetCommandOptions: GetCommandOptions::<Identity, Impl, OFFSET>,
            GetFunctionalCategories: GetFunctionalCategories::<Identity, Impl, OFFSET>,
            GetFunctionalObjects: GetFunctionalObjects::<Identity, Impl, OFFSET>,
            GetSupportedContentTypes: GetSupportedContentTypes::<Identity, Impl, OFFSET>,
            GetSupportedFormats: GetSupportedFormats::<Identity, Impl, OFFSET>,
            GetSupportedFormatProperties: GetSupportedFormatProperties::<Identity, Impl, OFFSET>,
            GetFixedPropertyAttributes: GetFixedPropertyAttributes::<Identity, Impl, OFFSET>,
            Cancel: Cancel::<Identity, Impl, OFFSET>,
            GetSupportedEvents: GetSupportedEvents::<Identity, Impl, OFFSET>,
            GetEventOptions: GetEventOptions::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPortableDeviceCapabilities as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Devices_Properties")]
pub trait IPortableDeviceConnector_Impl: Sized {
    fn Connect(&self, pcallback: &::core::option::Option<IConnectionRequestCallback>) -> ::windows::core::Result<()>;
    fn Disconnect(&self, pcallback: &::core::option::Option<IConnectionRequestCallback>) -> ::windows::core::Result<()>;
    fn Cancel(&self, pcallback: &::core::option::Option<IConnectionRequestCallback>) -> ::windows::core::Result<()>;
    fn GetProperty(&self, ppropertykey: *const super::Properties::DEVPROPKEY, ppropertytype: *mut u32, ppdata: *mut *mut u8, pcbdata: *mut u32) -> ::windows::core::Result<()>;
    fn SetProperty(&self, ppropertykey: *const super::Properties::DEVPROPKEY, propertytype: u32, pdata: *const u8, cbdata: u32) -> ::windows::core::Result<()>;
    fn GetPnPID(&self) -> ::windows::core::Result<::windows::core::PWSTR>;
}
#[cfg(feature = "Win32_Devices_Properties")]
impl IPortableDeviceConnector_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPortableDeviceConnector_Impl, const OFFSET: isize>() -> IPortableDeviceConnector_Vtbl {
        unsafe extern "system" fn Connect<Identity: ::windows::core::IUnknownImpl, Impl: IPortableDeviceConnector_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcallback: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Connect(::core::mem::transmute(&pcallback)).into()
        }
        unsafe extern "system" fn Disconnect<Identity: ::windows::core::IUnknownImpl, Impl: IPortableDeviceConnector_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcallback: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Disconnect(::core::mem::transmute(&pcallback)).into()
        }
        unsafe extern "system" fn Cancel<Identity: ::windows::core::IUnknownImpl, Impl: IPortableDeviceConnector_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcallback: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Cancel(::core::mem::transmute(&pcallback)).into()
        }
        unsafe extern "system" fn GetProperty<Identity: ::windows::core::IUnknownImpl, Impl: IPortableDeviceConnector_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppropertykey: *const super::Properties::DEVPROPKEY, ppropertytype: *mut u32, ppdata: *mut *mut u8, pcbdata: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetProperty(::core::mem::transmute_copy(&ppropertykey), ::core::mem::transmute_copy(&ppropertytype), ::core::mem::transmute_copy(&ppdata), ::core::mem::transmute_copy(&pcbdata)).into()
        }
        unsafe extern "system" fn SetProperty<Identity: ::windows::core::IUnknownImpl, Impl: IPortableDeviceConnector_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppropertykey: *const super::Properties::DEVPROPKEY, propertytype: u32, pdata: *const u8, cbdata: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetProperty(::core::mem::transmute_copy(&ppropertykey), ::core::mem::transmute_copy(&propertytype), ::core::mem::transmute_copy(&pdata), ::core::mem::transmute_copy(&cbdata)).into()
        }
        unsafe extern "system" fn GetPnPID<Identity: ::windows::core::IUnknownImpl, Impl: IPortableDeviceConnector_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppwszpnpid: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetPnPID() {
                ::core::result::Result::Ok(ok__) => {
                    *ppwszpnpid = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            Connect: Connect::<Identity, Impl, OFFSET>,
            Disconnect: Disconnect::<Identity, Impl, OFFSET>,
            Cancel: Cancel::<Identity, Impl, OFFSET>,
            GetProperty: GetProperty::<Identity, Impl, OFFSET>,
            SetProperty: SetProperty::<Identity, Impl, OFFSET>,
            GetPnPID: GetPnPID::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPortableDeviceConnector as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IPortableDeviceContent_Impl: Sized {
    fn EnumObjects(&self, dwflags: u32, pszparentobjectid: &::windows::core::PCWSTR, pfilter: &::core::option::Option<IPortableDeviceValues>) -> ::windows::core::Result<IEnumPortableDeviceObjectIDs>;
    fn Properties(&self) -> ::windows::core::Result<IPortableDeviceProperties>;
    fn Transfer(&self) -> ::windows::core::Result<IPortableDeviceResources>;
    fn CreateObjectWithPropertiesOnly(&self, pvalues: &::core::option::Option<IPortableDeviceValues>, ppszobjectid: *mut ::windows::core::PWSTR) -> ::windows::core::Result<()>;
    fn CreateObjectWithPropertiesAndData(&self, pvalues: &::core::option::Option<IPortableDeviceValues>, ppdata: *mut ::core::option::Option<super::super::System::Com::IStream>, pdwoptimalwritebuffersize: *mut u32, ppszcookie: *mut ::windows::core::PWSTR) -> ::windows::core::Result<()>;
    fn Delete(&self, dwoptions: u32, pobjectids: &::core::option::Option<IPortableDevicePropVariantCollection>, ppresults: *mut ::core::option::Option<IPortableDevicePropVariantCollection>) -> ::windows::core::Result<()>;
    fn GetObjectIDsFromPersistentUniqueIDs(&self, ppersistentuniqueids: &::core::option::Option<IPortableDevicePropVariantCollection>) -> ::windows::core::Result<IPortableDevicePropVariantCollection>;
    fn Cancel(&self) -> ::windows::core::Result<()>;
    fn Move(&self, pobjectids: &::core::option::Option<IPortableDevicePropVariantCollection>, pszdestinationfolderobjectid: &::windows::core::PCWSTR, ppresults: *mut ::core::option::Option<IPortableDevicePropVariantCollection>) -> ::windows::core::Result<()>;
    fn Copy(&self, pobjectids: &::core::option::Option<IPortableDevicePropVariantCollection>, pszdestinationfolderobjectid: &::windows::core::PCWSTR, ppresults: *mut ::core::option::Option<IPortableDevicePropVariantCollection>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl IPortableDeviceContent_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPortableDeviceContent_Impl, const OFFSET: isize>() -> IPortableDeviceContent_Vtbl {
        unsafe extern "system" fn EnumObjects<Identity: ::windows::core::IUnknownImpl, Impl: IPortableDeviceContent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwflags: u32, pszparentobjectid: ::windows::core::PCWSTR, pfilter: ::windows::core::RawPtr, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).EnumObjects(::core::mem::transmute_copy(&dwflags), ::core::mem::transmute(&pszparentobjectid), ::core::mem::transmute(&pfilter)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppenum = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Properties<Identity: ::windows::core::IUnknownImpl, Impl: IPortableDeviceContent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppproperties: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Properties() {
                ::core::result::Result::Ok(ok__) => {
                    *ppproperties = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Transfer<Identity: ::windows::core::IUnknownImpl, Impl: IPortableDeviceContent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppresources: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Transfer() {
                ::core::result::Result::Ok(ok__) => {
                    *ppresources = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateObjectWithPropertiesOnly<Identity: ::windows::core::IUnknownImpl, Impl: IPortableDeviceContent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalues: ::windows::core::RawPtr, ppszobjectid: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).CreateObjectWithPropertiesOnly(::core::mem::transmute(&pvalues), ::core::mem::transmute_copy(&ppszobjectid)).into()
        }
        unsafe extern "system" fn CreateObjectWithPropertiesAndData<Identity: ::windows::core::IUnknownImpl, Impl: IPortableDeviceContent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalues: ::windows::core::RawPtr, ppdata: *mut ::windows::core::RawPtr, pdwoptimalwritebuffersize: *mut u32, ppszcookie: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).CreateObjectWithPropertiesAndData(::core::mem::transmute(&pvalues), ::core::mem::transmute_copy(&ppdata), ::core::mem::transmute_copy(&pdwoptimalwritebuffersize), ::core::mem::transmute_copy(&ppszcookie)).into()
        }
        unsafe extern "system" fn Delete<Identity: ::windows::core::IUnknownImpl, Impl: IPortableDeviceContent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwoptions: u32, pobjectids: ::windows::core::RawPtr, ppresults: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Delete(::core::mem::transmute_copy(&dwoptions), ::core::mem::transmute(&pobjectids), ::core::mem::transmute_copy(&ppresults)).into()
        }
        unsafe extern "system" fn GetObjectIDsFromPersistentUniqueIDs<Identity: ::windows::core::IUnknownImpl, Impl: IPortableDeviceContent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppersistentuniqueids: ::windows::core::RawPtr, ppobjectids: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetObjectIDsFromPersistentUniqueIDs(::core::mem::transmute(&ppersistentuniqueids)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppobjectids = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Cancel<Identity: ::windows::core::IUnknownImpl, Impl: IPortableDeviceContent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Cancel().into()
        }
        unsafe extern "system" fn Move<Identity: ::windows::core::IUnknownImpl, Impl: IPortableDeviceContent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pobjectids: ::windows::core::RawPtr, pszdestinationfolderobjectid: ::windows::core::PCWSTR, ppresults: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Move(::core::mem::transmute(&pobjectids), ::core::mem::transmute(&pszdestinationfolderobjectid), ::core::mem::transmute_copy(&ppresults)).into()
        }
        unsafe extern "system" fn Copy<Identity: ::windows::core::IUnknownImpl, Impl: IPortableDeviceContent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pobjectids: ::windows::core::RawPtr, pszdestinationfolderobjectid: ::windows::core::PCWSTR, ppresults: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Copy(::core::mem::transmute(&pobjectids), ::core::mem::transmute(&pszdestinationfolderobjectid), ::core::mem::transmute_copy(&ppresults)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            EnumObjects: EnumObjects::<Identity, Impl, OFFSET>,
            Properties: Properties::<Identity, Impl, OFFSET>,
            Transfer: Transfer::<Identity, Impl, OFFSET>,
            CreateObjectWithPropertiesOnly: CreateObjectWithPropertiesOnly::<Identity, Impl, OFFSET>,
            CreateObjectWithPropertiesAndData: CreateObjectWithPropertiesAndData::<Identity, Impl, OFFSET>,
            Delete: Delete::<Identity, Impl, OFFSET>,
            GetObjectIDsFromPersistentUniqueIDs: GetObjectIDsFromPersistentUniqueIDs::<Identity, Impl, OFFSET>,
            Cancel: Cancel::<Identity, Impl, OFFSET>,
            Move: Move::<Identity, Impl, OFFSET>,
            Copy: Copy::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPortableDeviceContent as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IPortableDeviceContent2_Impl: Sized + IPortableDeviceContent_Impl {
    fn UpdateObjectWithPropertiesAndData(&self, pszobjectid: &::windows::core::PCWSTR, pproperties: &::core::option::Option<IPortableDeviceValues>, ppdata: *mut ::core::option::Option<super::super::System::Com::IStream>, pdwoptimalwritebuffersize: *mut u32) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl IPortableDeviceContent2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPortableDeviceContent2_Impl, const OFFSET: isize>() -> IPortableDeviceContent2_Vtbl {
        unsafe extern "system" fn UpdateObjectWithPropertiesAndData<Identity: ::windows::core::IUnknownImpl, Impl: IPortableDeviceContent2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszobjectid: ::windows::core::PCWSTR, pproperties: ::windows::core::RawPtr, ppdata: *mut ::windows::core::RawPtr, pdwoptimalwritebuffersize: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).UpdateObjectWithPropertiesAndData(::core::mem::transmute(&pszobjectid), ::core::mem::transmute(&pproperties), ::core::mem::transmute_copy(&ppdata), ::core::mem::transmute_copy(&pdwoptimalwritebuffersize)).into()
        }
        Self {
            base: IPortableDeviceContent_Vtbl::new::<Identity, Impl, OFFSET>(),
            UpdateObjectWithPropertiesAndData: UpdateObjectWithPropertiesAndData::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPortableDeviceContent2 as ::windows::core::Interface>::IID || iid == &<IPortableDeviceContent as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
pub trait IPortableDeviceDataStream_Impl: Sized + super::super::System::Com::ISequentialStream_Impl + super::super::System::Com::IStream_Impl {
    fn GetObjectID(&self) -> ::windows::core::Result<::windows::core::PWSTR>;
    fn Cancel(&self) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
impl IPortableDeviceDataStream_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPortableDeviceDataStream_Impl, const OFFSET: isize>() -> IPortableDeviceDataStream_Vtbl {
        unsafe extern "system" fn GetObjectID<Identity: ::windows::core::IUnknownImpl, Impl: IPortableDeviceDataStream_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppszobjectid: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetObjectID() {
                ::core::result::Result::Ok(ok__) => {
                    *ppszobjectid = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Cancel<Identity: ::windows::core::IUnknownImpl, Impl: IPortableDeviceDataStream_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Cancel().into()
        }
        Self {
            base: super::super::System::Com::IStream_Vtbl::new::<Identity, Impl, OFFSET>(),
            GetObjectID: GetObjectID::<Identity, Impl, OFFSET>,
            Cancel: Cancel::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPortableDeviceDataStream as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::ISequentialStream as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IStream as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IPortableDeviceDispatchFactory_Impl: Sized {
    fn GetDeviceDispatch(&self, pszpnpdeviceid: &::windows::core::PCWSTR) -> ::windows::core::Result<super::super::System::Com::IDispatch>;
}
#[cfg(feature = "Win32_System_Com")]
impl IPortableDeviceDispatchFactory_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPortableDeviceDispatchFactory_Impl, const OFFSET: isize>() -> IPortableDeviceDispatchFactory_Vtbl {
        unsafe extern "system" fn GetDeviceDispatch<Identity: ::windows::core::IUnknownImpl, Impl: IPortableDeviceDispatchFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszpnpdeviceid: ::windows::core::PCWSTR, ppdevicedispatch: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetDeviceDispatch(::core::mem::transmute(&pszpnpdeviceid)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppdevicedispatch = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(), GetDeviceDispatch: GetDeviceDispatch::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPortableDeviceDispatchFactory as ::windows::core::Interface>::IID
    }
}
pub trait IPortableDeviceEventCallback_Impl: Sized {
    fn OnEvent(&self, peventparameters: &::core::option::Option<IPortableDeviceValues>) -> ::windows::core::Result<()>;
}
impl IPortableDeviceEventCallback_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPortableDeviceEventCallback_Impl, const OFFSET: isize>() -> IPortableDeviceEventCallback_Vtbl {
        unsafe extern "system" fn OnEvent<Identity: ::windows::core::IUnknownImpl, Impl: IPortableDeviceEventCallback_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, peventparameters: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).OnEvent(::core::mem::transmute(&peventparameters)).into()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(), OnEvent: OnEvent::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPortableDeviceEventCallback as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub trait IPortableDeviceKeyCollection_Impl: Sized {
    fn GetCount(&self, pcelems: *const u32) -> ::windows::core::Result<()>;
    fn GetAt(&self, dwindex: u32, pkey: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY) -> ::windows::core::Result<()>;
    fn Add(&self, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY) -> ::windows::core::Result<()>;
    fn Clear(&self) -> ::windows::core::Result<()>;
    fn RemoveAt(&self, dwindex: u32) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
impl IPortableDeviceKeyCollection_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPortableDeviceKeyCollection_Impl, const OFFSET: isize>() -> IPortableDeviceKeyCollection_Vtbl {
        unsafe extern "system" fn GetCount<Identity: ::windows::core::IUnknownImpl, Impl: IPortableDeviceKeyCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcelems: *const u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetCount(::core::mem::transmute_copy(&pcelems)).into()
        }
        unsafe extern "system" fn GetAt<Identity: ::windows::core::IUnknownImpl, Impl: IPortableDeviceKeyCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwindex: u32, pkey: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetAt(::core::mem::transmute_copy(&dwindex), ::core::mem::transmute_copy(&pkey)).into()
        }
        unsafe extern "system" fn Add<Identity: ::windows::core::IUnknownImpl, Impl: IPortableDeviceKeyCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Add(::core::mem::transmute_copy(&key)).into()
        }
        unsafe extern "system" fn Clear<Identity: ::windows::core::IUnknownImpl, Impl: IPortableDeviceKeyCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Clear().into()
        }
        unsafe extern "system" fn RemoveAt<Identity: ::windows::core::IUnknownImpl, Impl: IPortableDeviceKeyCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwindex: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).RemoveAt(::core::mem::transmute_copy(&dwindex)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            GetCount: GetCount::<Identity, Impl, OFFSET>,
            GetAt: GetAt::<Identity, Impl, OFFSET>,
            Add: Add::<Identity, Impl, OFFSET>,
            Clear: Clear::<Identity, Impl, OFFSET>,
            RemoveAt: RemoveAt::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPortableDeviceKeyCollection as ::windows::core::Interface>::IID
    }
}
pub trait IPortableDeviceManager_Impl: Sized {
    fn GetDevices(&self, ppnpdeviceids: *mut ::windows::core::PWSTR, pcpnpdeviceids: *mut u32) -> ::windows::core::Result<()>;
    fn RefreshDeviceList(&self) -> ::windows::core::Result<()>;
    fn GetDeviceFriendlyName(&self, pszpnpdeviceid: &::windows::core::PCWSTR, pdevicefriendlyname: &::windows::core::PWSTR, pcchdevicefriendlyname: *mut u32) -> ::windows::core::Result<()>;
    fn GetDeviceDescription(&self, pszpnpdeviceid: &::windows::core::PCWSTR, pdevicedescription: &::windows::core::PWSTR, pcchdevicedescription: *mut u32) -> ::windows::core::Result<()>;
    fn GetDeviceManufacturer(&self, pszpnpdeviceid: &::windows::core::PCWSTR, pdevicemanufacturer: &::windows::core::PWSTR, pcchdevicemanufacturer: *mut u32) -> ::windows::core::Result<()>;
    fn GetDeviceProperty(&self, pszpnpdeviceid: &::windows::core::PCWSTR, pszdevicepropertyname: &::windows::core::PCWSTR, pdata: *mut u8, pcbdata: *mut u32, pdwtype: *mut u32) -> ::windows::core::Result<()>;
    fn GetPrivateDevices(&self, ppnpdeviceids: *mut ::windows::core::PWSTR, pcpnpdeviceids: *mut u32) -> ::windows::core::Result<()>;
}
impl IPortableDeviceManager_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPortableDeviceManager_Impl, const OFFSET: isize>() -> IPortableDeviceManager_Vtbl {
        unsafe extern "system" fn GetDevices<Identity: ::windows::core::IUnknownImpl, Impl: IPortableDeviceManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppnpdeviceids: *mut ::windows::core::PWSTR, pcpnpdeviceids: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetDevices(::core::mem::transmute_copy(&ppnpdeviceids), ::core::mem::transmute_copy(&pcpnpdeviceids)).into()
        }
        unsafe extern "system" fn RefreshDeviceList<Identity: ::windows::core::IUnknownImpl, Impl: IPortableDeviceManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).RefreshDeviceList().into()
        }
        unsafe extern "system" fn GetDeviceFriendlyName<Identity: ::windows::core::IUnknownImpl, Impl: IPortableDeviceManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszpnpdeviceid: ::windows::core::PCWSTR, pdevicefriendlyname: ::windows::core::PWSTR, pcchdevicefriendlyname: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetDeviceFriendlyName(::core::mem::transmute(&pszpnpdeviceid), ::core::mem::transmute(&pdevicefriendlyname), ::core::mem::transmute_copy(&pcchdevicefriendlyname)).into()
        }
        unsafe extern "system" fn GetDeviceDescription<Identity: ::windows::core::IUnknownImpl, Impl: IPortableDeviceManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszpnpdeviceid: ::windows::core::PCWSTR, pdevicedescription: ::windows::core::PWSTR, pcchdevicedescription: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetDeviceDescription(::core::mem::transmute(&pszpnpdeviceid), ::core::mem::transmute(&pdevicedescription), ::core::mem::transmute_copy(&pcchdevicedescription)).into()
        }
        unsafe extern "system" fn GetDeviceManufacturer<Identity: ::windows::core::IUnknownImpl, Impl: IPortableDeviceManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszpnpdeviceid: ::windows::core::PCWSTR, pdevicemanufacturer: ::windows::core::PWSTR, pcchdevicemanufacturer: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetDeviceManufacturer(::core::mem::transmute(&pszpnpdeviceid), ::core::mem::transmute(&pdevicemanufacturer), ::core::mem::transmute_copy(&pcchdevicemanufacturer)).into()
        }
        unsafe extern "system" fn GetDeviceProperty<Identity: ::windows::core::IUnknownImpl, Impl: IPortableDeviceManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszpnpdeviceid: ::windows::core::PCWSTR, pszdevicepropertyname: ::windows::core::PCWSTR, pdata: *mut u8, pcbdata: *mut u32, pdwtype: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetDeviceProperty(::core::mem::transmute(&pszpnpdeviceid), ::core::mem::transmute(&pszdevicepropertyname), ::core::mem::transmute_copy(&pdata), ::core::mem::transmute_copy(&pcbdata), ::core::mem::transmute_copy(&pdwtype)).into()
        }
        unsafe extern "system" fn GetPrivateDevices<Identity: ::windows::core::IUnknownImpl, Impl: IPortableDeviceManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppnpdeviceids: *mut ::windows::core::PWSTR, pcpnpdeviceids: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetPrivateDevices(::core::mem::transmute_copy(&ppnpdeviceids), ::core::mem::transmute_copy(&pcpnpdeviceids)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            GetDevices: GetDevices::<Identity, Impl, OFFSET>,
            RefreshDeviceList: RefreshDeviceList::<Identity, Impl, OFFSET>,
            GetDeviceFriendlyName: GetDeviceFriendlyName::<Identity, Impl, OFFSET>,
            GetDeviceDescription: GetDeviceDescription::<Identity, Impl, OFFSET>,
            GetDeviceManufacturer: GetDeviceManufacturer::<Identity, Impl, OFFSET>,
            GetDeviceProperty: GetDeviceProperty::<Identity, Impl, OFFSET>,
            GetPrivateDevices: GetPrivateDevices::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPortableDeviceManager as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
pub trait IPortableDevicePropVariantCollection_Impl: Sized {
    fn GetCount(&self, pcelems: *const u32) -> ::windows::core::Result<()>;
    fn GetAt(&self, dwindex: u32, pvalue: *const super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows::core::Result<()>;
    fn Add(&self, pvalue: *const super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows::core::Result<()>;
    fn GetType(&self) -> ::windows::core::Result<u16>;
    fn ChangeType(&self, vt: u16) -> ::windows::core::Result<()>;
    fn Clear(&self) -> ::windows::core::Result<()>;
    fn RemoveAt(&self, dwindex: u32) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
impl IPortableDevicePropVariantCollection_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPortableDevicePropVariantCollection_Impl, const OFFSET: isize>() -> IPortableDevicePropVariantCollection_Vtbl {
        unsafe extern "system" fn GetCount<Identity: ::windows::core::IUnknownImpl, Impl: IPortableDevicePropVariantCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcelems: *const u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetCount(::core::mem::transmute_copy(&pcelems)).into()
        }
        unsafe extern "system" fn GetAt<Identity: ::windows::core::IUnknownImpl, Impl: IPortableDevicePropVariantCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwindex: u32, pvalue: *const super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetAt(::core::mem::transmute_copy(&dwindex), ::core::mem::transmute_copy(&pvalue)).into()
        }
        unsafe extern "system" fn Add<Identity: ::windows::core::IUnknownImpl, Impl: IPortableDevicePropVariantCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *const super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Add(::core::mem::transmute_copy(&pvalue)).into()
        }
        unsafe extern "system" fn GetType<Identity: ::windows::core::IUnknownImpl, Impl: IPortableDevicePropVariantCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvt: *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetType() {
                ::core::result::Result::Ok(ok__) => {
                    *pvt = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ChangeType<Identity: ::windows::core::IUnknownImpl, Impl: IPortableDevicePropVariantCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, vt: u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).ChangeType(::core::mem::transmute_copy(&vt)).into()
        }
        unsafe extern "system" fn Clear<Identity: ::windows::core::IUnknownImpl, Impl: IPortableDevicePropVariantCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Clear().into()
        }
        unsafe extern "system" fn RemoveAt<Identity: ::windows::core::IUnknownImpl, Impl: IPortableDevicePropVariantCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwindex: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).RemoveAt(::core::mem::transmute_copy(&dwindex)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            GetCount: GetCount::<Identity, Impl, OFFSET>,
            GetAt: GetAt::<Identity, Impl, OFFSET>,
            Add: Add::<Identity, Impl, OFFSET>,
            GetType: GetType::<Identity, Impl, OFFSET>,
            ChangeType: ChangeType::<Identity, Impl, OFFSET>,
            Clear: Clear::<Identity, Impl, OFFSET>,
            RemoveAt: RemoveAt::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPortableDevicePropVariantCollection as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub trait IPortableDeviceProperties_Impl: Sized {
    fn GetSupportedProperties(&self, pszobjectid: &::windows::core::PCWSTR) -> ::windows::core::Result<IPortableDeviceKeyCollection>;
    fn GetPropertyAttributes(&self, pszobjectid: &::windows::core::PCWSTR, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY) -> ::windows::core::Result<IPortableDeviceValues>;
    fn GetValues(&self, pszobjectid: &::windows::core::PCWSTR, pkeys: &::core::option::Option<IPortableDeviceKeyCollection>) -> ::windows::core::Result<IPortableDeviceValues>;
    fn SetValues(&self, pszobjectid: &::windows::core::PCWSTR, pvalues: &::core::option::Option<IPortableDeviceValues>) -> ::windows::core::Result<IPortableDeviceValues>;
    fn Delete(&self, pszobjectid: &::windows::core::PCWSTR, pkeys: &::core::option::Option<IPortableDeviceKeyCollection>) -> ::windows::core::Result<()>;
    fn Cancel(&self) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
impl IPortableDeviceProperties_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPortableDeviceProperties_Impl, const OFFSET: isize>() -> IPortableDeviceProperties_Vtbl {
        unsafe extern "system" fn GetSupportedProperties<Identity: ::windows::core::IUnknownImpl, Impl: IPortableDeviceProperties_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszobjectid: ::windows::core::PCWSTR, ppkeys: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetSupportedProperties(::core::mem::transmute(&pszobjectid)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppkeys = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPropertyAttributes<Identity: ::windows::core::IUnknownImpl, Impl: IPortableDeviceProperties_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszobjectid: ::windows::core::PCWSTR, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, ppattributes: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetPropertyAttributes(::core::mem::transmute(&pszobjectid), ::core::mem::transmute_copy(&key)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppattributes = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetValues<Identity: ::windows::core::IUnknownImpl, Impl: IPortableDeviceProperties_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszobjectid: ::windows::core::PCWSTR, pkeys: ::windows::core::RawPtr, ppvalues: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetValues(::core::mem::transmute(&pszobjectid), ::core::mem::transmute(&pkeys)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppvalues = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetValues<Identity: ::windows::core::IUnknownImpl, Impl: IPortableDeviceProperties_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszobjectid: ::windows::core::PCWSTR, pvalues: ::windows::core::RawPtr, ppresults: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).SetValues(::core::mem::transmute(&pszobjectid), ::core::mem::transmute(&pvalues)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppresults = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Delete<Identity: ::windows::core::IUnknownImpl, Impl: IPortableDeviceProperties_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszobjectid: ::windows::core::PCWSTR, pkeys: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Delete(::core::mem::transmute(&pszobjectid), ::core::mem::transmute(&pkeys)).into()
        }
        unsafe extern "system" fn Cancel<Identity: ::windows::core::IUnknownImpl, Impl: IPortableDeviceProperties_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Cancel().into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            GetSupportedProperties: GetSupportedProperties::<Identity, Impl, OFFSET>,
            GetPropertyAttributes: GetPropertyAttributes::<Identity, Impl, OFFSET>,
            GetValues: GetValues::<Identity, Impl, OFFSET>,
            SetValues: SetValues::<Identity, Impl, OFFSET>,
            Delete: Delete::<Identity, Impl, OFFSET>,
            Cancel: Cancel::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPortableDeviceProperties as ::windows::core::Interface>::IID
    }
}
pub trait IPortableDevicePropertiesBulk_Impl: Sized {
    fn QueueGetValuesByObjectList(&self, pobjectids: &::core::option::Option<IPortableDevicePropVariantCollection>, pkeys: &::core::option::Option<IPortableDeviceKeyCollection>, pcallback: &::core::option::Option<IPortableDevicePropertiesBulkCallback>) -> ::windows::core::Result<::windows::core::GUID>;
    fn QueueGetValuesByObjectFormat(&self, pguidobjectformat: *const ::windows::core::GUID, pszparentobjectid: &::windows::core::PCWSTR, dwdepth: u32, pkeys: &::core::option::Option<IPortableDeviceKeyCollection>, pcallback: &::core::option::Option<IPortableDevicePropertiesBulkCallback>) -> ::windows::core::Result<::windows::core::GUID>;
    fn QueueSetValuesByObjectList(&self, pobjectvalues: &::core::option::Option<IPortableDeviceValuesCollection>, pcallback: &::core::option::Option<IPortableDevicePropertiesBulkCallback>) -> ::windows::core::Result<::windows::core::GUID>;
    fn Start(&self, pcontext: *const ::windows::core::GUID) -> ::windows::core::Result<()>;
    fn Cancel(&self, pcontext: *const ::windows::core::GUID) -> ::windows::core::Result<()>;
}
impl IPortableDevicePropertiesBulk_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPortableDevicePropertiesBulk_Impl, const OFFSET: isize>() -> IPortableDevicePropertiesBulk_Vtbl {
        unsafe extern "system" fn QueueGetValuesByObjectList<Identity: ::windows::core::IUnknownImpl, Impl: IPortableDevicePropertiesBulk_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pobjectids: ::windows::core::RawPtr, pkeys: ::windows::core::RawPtr, pcallback: ::windows::core::RawPtr, pcontext: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).QueueGetValuesByObjectList(::core::mem::transmute(&pobjectids), ::core::mem::transmute(&pkeys), ::core::mem::transmute(&pcallback)) {
                ::core::result::Result::Ok(ok__) => {
                    *pcontext = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn QueueGetValuesByObjectFormat<Identity: ::windows::core::IUnknownImpl, Impl: IPortableDevicePropertiesBulk_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pguidobjectformat: *const ::windows::core::GUID, pszparentobjectid: ::windows::core::PCWSTR, dwdepth: u32, pkeys: ::windows::core::RawPtr, pcallback: ::windows::core::RawPtr, pcontext: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).QueueGetValuesByObjectFormat(::core::mem::transmute_copy(&pguidobjectformat), ::core::mem::transmute(&pszparentobjectid), ::core::mem::transmute_copy(&dwdepth), ::core::mem::transmute(&pkeys), ::core::mem::transmute(&pcallback)) {
                ::core::result::Result::Ok(ok__) => {
                    *pcontext = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn QueueSetValuesByObjectList<Identity: ::windows::core::IUnknownImpl, Impl: IPortableDevicePropertiesBulk_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pobjectvalues: ::windows::core::RawPtr, pcallback: ::windows::core::RawPtr, pcontext: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).QueueSetValuesByObjectList(::core::mem::transmute(&pobjectvalues), ::core::mem::transmute(&pcallback)) {
                ::core::result::Result::Ok(ok__) => {
                    *pcontext = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Start<Identity: ::windows::core::IUnknownImpl, Impl: IPortableDevicePropertiesBulk_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcontext: *const ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Start(::core::mem::transmute_copy(&pcontext)).into()
        }
        unsafe extern "system" fn Cancel<Identity: ::windows::core::IUnknownImpl, Impl: IPortableDevicePropertiesBulk_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcontext: *const ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Cancel(::core::mem::transmute_copy(&pcontext)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            QueueGetValuesByObjectList: QueueGetValuesByObjectList::<Identity, Impl, OFFSET>,
            QueueGetValuesByObjectFormat: QueueGetValuesByObjectFormat::<Identity, Impl, OFFSET>,
            QueueSetValuesByObjectList: QueueSetValuesByObjectList::<Identity, Impl, OFFSET>,
            Start: Start::<Identity, Impl, OFFSET>,
            Cancel: Cancel::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPortableDevicePropertiesBulk as ::windows::core::Interface>::IID
    }
}
pub trait IPortableDevicePropertiesBulkCallback_Impl: Sized {
    fn OnStart(&self, pcontext: *const ::windows::core::GUID) -> ::windows::core::Result<()>;
    fn OnProgress(&self, pcontext: *const ::windows::core::GUID, presults: &::core::option::Option<IPortableDeviceValuesCollection>) -> ::windows::core::Result<()>;
    fn OnEnd(&self, pcontext: *const ::windows::core::GUID, hrstatus: ::windows::core::HRESULT) -> ::windows::core::Result<()>;
}
impl IPortableDevicePropertiesBulkCallback_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPortableDevicePropertiesBulkCallback_Impl, const OFFSET: isize>() -> IPortableDevicePropertiesBulkCallback_Vtbl {
        unsafe extern "system" fn OnStart<Identity: ::windows::core::IUnknownImpl, Impl: IPortableDevicePropertiesBulkCallback_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcontext: *const ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).OnStart(::core::mem::transmute_copy(&pcontext)).into()
        }
        unsafe extern "system" fn OnProgress<Identity: ::windows::core::IUnknownImpl, Impl: IPortableDevicePropertiesBulkCallback_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcontext: *const ::windows::core::GUID, presults: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).OnProgress(::core::mem::transmute_copy(&pcontext), ::core::mem::transmute(&presults)).into()
        }
        unsafe extern "system" fn OnEnd<Identity: ::windows::core::IUnknownImpl, Impl: IPortableDevicePropertiesBulkCallback_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcontext: *const ::windows::core::GUID, hrstatus: ::windows::core::HRESULT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).OnEnd(::core::mem::transmute_copy(&pcontext), ::core::mem::transmute_copy(&hrstatus)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            OnStart: OnStart::<Identity, Impl, OFFSET>,
            OnProgress: OnProgress::<Identity, Impl, OFFSET>,
            OnEnd: OnEnd::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPortableDevicePropertiesBulkCallback as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_UI_Shell_PropertiesSystem"))]
pub trait IPortableDeviceResources_Impl: Sized {
    fn GetSupportedResources(&self, pszobjectid: &::windows::core::PCWSTR) -> ::windows::core::Result<IPortableDeviceKeyCollection>;
    fn GetResourceAttributes(&self, pszobjectid: &::windows::core::PCWSTR, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY) -> ::windows::core::Result<IPortableDeviceValues>;
    fn GetStream(&self, pszobjectid: &::windows::core::PCWSTR, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, dwmode: u32, pdwoptimalbuffersize: *mut u32, ppstream: *mut ::core::option::Option<super::super::System::Com::IStream>) -> ::windows::core::Result<()>;
    fn Delete(&self, pszobjectid: &::windows::core::PCWSTR, pkeys: &::core::option::Option<IPortableDeviceKeyCollection>) -> ::windows::core::Result<()>;
    fn Cancel(&self) -> ::windows::core::Result<()>;
    fn CreateResource(&self, presourceattributes: &::core::option::Option<IPortableDeviceValues>, ppdata: *mut ::core::option::Option<super::super::System::Com::IStream>, pdwoptimalwritebuffersize: *mut u32, ppszcookie: *mut ::windows::core::PWSTR) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_UI_Shell_PropertiesSystem"))]
impl IPortableDeviceResources_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPortableDeviceResources_Impl, const OFFSET: isize>() -> IPortableDeviceResources_Vtbl {
        unsafe extern "system" fn GetSupportedResources<Identity: ::windows::core::IUnknownImpl, Impl: IPortableDeviceResources_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszobjectid: ::windows::core::PCWSTR, ppkeys: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetSupportedResources(::core::mem::transmute(&pszobjectid)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppkeys = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetResourceAttributes<Identity: ::windows::core::IUnknownImpl, Impl: IPortableDeviceResources_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszobjectid: ::windows::core::PCWSTR, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, ppresourceattributes: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetResourceAttributes(::core::mem::transmute(&pszobjectid), ::core::mem::transmute_copy(&key)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppresourceattributes = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetStream<Identity: ::windows::core::IUnknownImpl, Impl: IPortableDeviceResources_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszobjectid: ::windows::core::PCWSTR, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, dwmode: u32, pdwoptimalbuffersize: *mut u32, ppstream: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetStream(::core::mem::transmute(&pszobjectid), ::core::mem::transmute_copy(&key), ::core::mem::transmute_copy(&dwmode), ::core::mem::transmute_copy(&pdwoptimalbuffersize), ::core::mem::transmute_copy(&ppstream)).into()
        }
        unsafe extern "system" fn Delete<Identity: ::windows::core::IUnknownImpl, Impl: IPortableDeviceResources_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszobjectid: ::windows::core::PCWSTR, pkeys: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Delete(::core::mem::transmute(&pszobjectid), ::core::mem::transmute(&pkeys)).into()
        }
        unsafe extern "system" fn Cancel<Identity: ::windows::core::IUnknownImpl, Impl: IPortableDeviceResources_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Cancel().into()
        }
        unsafe extern "system" fn CreateResource<Identity: ::windows::core::IUnknownImpl, Impl: IPortableDeviceResources_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, presourceattributes: ::windows::core::RawPtr, ppdata: *mut ::windows::core::RawPtr, pdwoptimalwritebuffersize: *mut u32, ppszcookie: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).CreateResource(::core::mem::transmute(&presourceattributes), ::core::mem::transmute_copy(&ppdata), ::core::mem::transmute_copy(&pdwoptimalwritebuffersize), ::core::mem::transmute_copy(&ppszcookie)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            GetSupportedResources: GetSupportedResources::<Identity, Impl, OFFSET>,
            GetResourceAttributes: GetResourceAttributes::<Identity, Impl, OFFSET>,
            GetStream: GetStream::<Identity, Impl, OFFSET>,
            Delete: Delete::<Identity, Impl, OFFSET>,
            Cancel: Cancel::<Identity, Impl, OFFSET>,
            CreateResource: CreateResource::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPortableDeviceResources as ::windows::core::Interface>::IID
    }
}
pub trait IPortableDeviceService_Impl: Sized {
    fn Open(&self, pszpnpserviceid: &::windows::core::PCWSTR, pclientinfo: &::core::option::Option<IPortableDeviceValues>) -> ::windows::core::Result<()>;
    fn Capabilities(&self) -> ::windows::core::Result<IPortableDeviceServiceCapabilities>;
    fn Content(&self) -> ::windows::core::Result<IPortableDeviceContent2>;
    fn Methods(&self) -> ::windows::core::Result<IPortableDeviceServiceMethods>;
    fn Cancel(&self) -> ::windows::core::Result<()>;
    fn Close(&self) -> ::windows::core::Result<()>;
    fn GetServiceObjectID(&self) -> ::windows::core::Result<::windows::core::PWSTR>;
    fn GetPnPServiceID(&self) -> ::windows::core::Result<::windows::core::PWSTR>;
    fn Advise(&self, dwflags: u32, pcallback: &::core::option::Option<IPortableDeviceEventCallback>, pparameters: &::core::option::Option<IPortableDeviceValues>) -> ::windows::core::Result<::windows::core::PWSTR>;
    fn Unadvise(&self, pszcookie: &::windows::core::PCWSTR) -> ::windows::core::Result<()>;
    fn SendCommand(&self, dwflags: u32, pparameters: &::core::option::Option<IPortableDeviceValues>) -> ::windows::core::Result<IPortableDeviceValues>;
}
impl IPortableDeviceService_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPortableDeviceService_Impl, const OFFSET: isize>() -> IPortableDeviceService_Vtbl {
        unsafe extern "system" fn Open<Identity: ::windows::core::IUnknownImpl, Impl: IPortableDeviceService_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszpnpserviceid: ::windows::core::PCWSTR, pclientinfo: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Open(::core::mem::transmute(&pszpnpserviceid), ::core::mem::transmute(&pclientinfo)).into()
        }
        unsafe extern "system" fn Capabilities<Identity: ::windows::core::IUnknownImpl, Impl: IPortableDeviceService_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppcapabilities: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Capabilities() {
                ::core::result::Result::Ok(ok__) => {
                    *ppcapabilities = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Content<Identity: ::windows::core::IUnknownImpl, Impl: IPortableDeviceService_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppcontent: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Content() {
                ::core::result::Result::Ok(ok__) => {
                    *ppcontent = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Methods<Identity: ::windows::core::IUnknownImpl, Impl: IPortableDeviceService_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppmethods: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Methods() {
                ::core::result::Result::Ok(ok__) => {
                    *ppmethods = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Cancel<Identity: ::windows::core::IUnknownImpl, Impl: IPortableDeviceService_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Cancel().into()
        }
        unsafe extern "system" fn Close<Identity: ::windows::core::IUnknownImpl, Impl: IPortableDeviceService_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Close().into()
        }
        unsafe extern "system" fn GetServiceObjectID<Identity: ::windows::core::IUnknownImpl, Impl: IPortableDeviceService_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppszserviceobjectid: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetServiceObjectID() {
                ::core::result::Result::Ok(ok__) => {
                    *ppszserviceobjectid = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPnPServiceID<Identity: ::windows::core::IUnknownImpl, Impl: IPortableDeviceService_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppszpnpserviceid: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetPnPServiceID() {
                ::core::result::Result::Ok(ok__) => {
                    *ppszpnpserviceid = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Advise<Identity: ::windows::core::IUnknownImpl, Impl: IPortableDeviceService_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwflags: u32, pcallback: ::windows::core::RawPtr, pparameters: ::windows::core::RawPtr, ppszcookie: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Advise(::core::mem::transmute_copy(&dwflags), ::core::mem::transmute(&pcallback), ::core::mem::transmute(&pparameters)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppszcookie = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Unadvise<Identity: ::windows::core::IUnknownImpl, Impl: IPortableDeviceService_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszcookie: ::windows::core::PCWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Unadvise(::core::mem::transmute(&pszcookie)).into()
        }
        unsafe extern "system" fn SendCommand<Identity: ::windows::core::IUnknownImpl, Impl: IPortableDeviceService_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwflags: u32, pparameters: ::windows::core::RawPtr, ppresults: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).SendCommand(::core::mem::transmute_copy(&dwflags), ::core::mem::transmute(&pparameters)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppresults = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            Open: Open::<Identity, Impl, OFFSET>,
            Capabilities: Capabilities::<Identity, Impl, OFFSET>,
            Content: Content::<Identity, Impl, OFFSET>,
            Methods: Methods::<Identity, Impl, OFFSET>,
            Cancel: Cancel::<Identity, Impl, OFFSET>,
            Close: Close::<Identity, Impl, OFFSET>,
            GetServiceObjectID: GetServiceObjectID::<Identity, Impl, OFFSET>,
            GetPnPServiceID: GetPnPServiceID::<Identity, Impl, OFFSET>,
            Advise: Advise::<Identity, Impl, OFFSET>,
            Unadvise: Unadvise::<Identity, Impl, OFFSET>,
            SendCommand: SendCommand::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPortableDeviceService as ::windows::core::Interface>::IID
    }
}
pub trait IPortableDeviceServiceActivation_Impl: Sized {
    fn OpenAsync(&self, pszpnpserviceid: &::windows::core::PCWSTR, pclientinfo: &::core::option::Option<IPortableDeviceValues>, pcallback: &::core::option::Option<IPortableDeviceServiceOpenCallback>) -> ::windows::core::Result<()>;
    fn CancelOpenAsync(&self) -> ::windows::core::Result<()>;
}
impl IPortableDeviceServiceActivation_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPortableDeviceServiceActivation_Impl, const OFFSET: isize>() -> IPortableDeviceServiceActivation_Vtbl {
        unsafe extern "system" fn OpenAsync<Identity: ::windows::core::IUnknownImpl, Impl: IPortableDeviceServiceActivation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszpnpserviceid: ::windows::core::PCWSTR, pclientinfo: ::windows::core::RawPtr, pcallback: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).OpenAsync(::core::mem::transmute(&pszpnpserviceid), ::core::mem::transmute(&pclientinfo), ::core::mem::transmute(&pcallback)).into()
        }
        unsafe extern "system" fn CancelOpenAsync<Identity: ::windows::core::IUnknownImpl, Impl: IPortableDeviceServiceActivation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).CancelOpenAsync().into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            OpenAsync: OpenAsync::<Identity, Impl, OFFSET>,
            CancelOpenAsync: CancelOpenAsync::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPortableDeviceServiceActivation as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub trait IPortableDeviceServiceCapabilities_Impl: Sized {
    fn GetSupportedMethods(&self) -> ::windows::core::Result<IPortableDevicePropVariantCollection>;
    fn GetSupportedMethodsByFormat(&self, format: *const ::windows::core::GUID) -> ::windows::core::Result<IPortableDevicePropVariantCollection>;
    fn GetMethodAttributes(&self, method: *const ::windows::core::GUID) -> ::windows::core::Result<IPortableDeviceValues>;
    fn GetMethodParameterAttributes(&self, method: *const ::windows::core::GUID, parameter: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY) -> ::windows::core::Result<IPortableDeviceValues>;
    fn GetSupportedFormats(&self) -> ::windows::core::Result<IPortableDevicePropVariantCollection>;
    fn GetFormatAttributes(&self, format: *const ::windows::core::GUID) -> ::windows::core::Result<IPortableDeviceValues>;
    fn GetSupportedFormatProperties(&self, format: *const ::windows::core::GUID) -> ::windows::core::Result<IPortableDeviceKeyCollection>;
    fn GetFormatPropertyAttributes(&self, format: *const ::windows::core::GUID, property: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY) -> ::windows::core::Result<IPortableDeviceValues>;
    fn GetSupportedEvents(&self) -> ::windows::core::Result<IPortableDevicePropVariantCollection>;
    fn GetEventAttributes(&self, event: *const ::windows::core::GUID) -> ::windows::core::Result<IPortableDeviceValues>;
    fn GetEventParameterAttributes(&self, event: *const ::windows::core::GUID, parameter: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY) -> ::windows::core::Result<IPortableDeviceValues>;
    fn GetInheritedServices(&self, dwinheritancetype: u32) -> ::windows::core::Result<IPortableDevicePropVariantCollection>;
    fn GetFormatRenderingProfiles(&self, format: *const ::windows::core::GUID) -> ::windows::core::Result<IPortableDeviceValuesCollection>;
    fn GetSupportedCommands(&self) -> ::windows::core::Result<IPortableDeviceKeyCollection>;
    fn GetCommandOptions(&self, command: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY) -> ::windows::core::Result<IPortableDeviceValues>;
    fn Cancel(&self) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
impl IPortableDeviceServiceCapabilities_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPortableDeviceServiceCapabilities_Impl, const OFFSET: isize>() -> IPortableDeviceServiceCapabilities_Vtbl {
        unsafe extern "system" fn GetSupportedMethods<Identity: ::windows::core::IUnknownImpl, Impl: IPortableDeviceServiceCapabilities_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppmethods: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetSupportedMethods() {
                ::core::result::Result::Ok(ok__) => {
                    *ppmethods = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSupportedMethodsByFormat<Identity: ::windows::core::IUnknownImpl, Impl: IPortableDeviceServiceCapabilities_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, format: *const ::windows::core::GUID, ppmethods: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetSupportedMethodsByFormat(::core::mem::transmute_copy(&format)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppmethods = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetMethodAttributes<Identity: ::windows::core::IUnknownImpl, Impl: IPortableDeviceServiceCapabilities_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, method: *const ::windows::core::GUID, ppattributes: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetMethodAttributes(::core::mem::transmute_copy(&method)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppattributes = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetMethodParameterAttributes<Identity: ::windows::core::IUnknownImpl, Impl: IPortableDeviceServiceCapabilities_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, method: *const ::windows::core::GUID, parameter: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, ppattributes: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetMethodParameterAttributes(::core::mem::transmute_copy(&method), ::core::mem::transmute_copy(&parameter)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppattributes = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSupportedFormats<Identity: ::windows::core::IUnknownImpl, Impl: IPortableDeviceServiceCapabilities_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppformats: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetSupportedFormats() {
                ::core::result::Result::Ok(ok__) => {
                    *ppformats = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFormatAttributes<Identity: ::windows::core::IUnknownImpl, Impl: IPortableDeviceServiceCapabilities_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, format: *const ::windows::core::GUID, ppattributes: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetFormatAttributes(::core::mem::transmute_copy(&format)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppattributes = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSupportedFormatProperties<Identity: ::windows::core::IUnknownImpl, Impl: IPortableDeviceServiceCapabilities_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, format: *const ::windows::core::GUID, ppkeys: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetSupportedFormatProperties(::core::mem::transmute_copy(&format)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppkeys = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFormatPropertyAttributes<Identity: ::windows::core::IUnknownImpl, Impl: IPortableDeviceServiceCapabilities_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, format: *const ::windows::core::GUID, property: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, ppattributes: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetFormatPropertyAttributes(::core::mem::transmute_copy(&format), ::core::mem::transmute_copy(&property)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppattributes = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSupportedEvents<Identity: ::windows::core::IUnknownImpl, Impl: IPortableDeviceServiceCapabilities_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppevents: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetSupportedEvents() {
                ::core::result::Result::Ok(ok__) => {
                    *ppevents = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetEventAttributes<Identity: ::windows::core::IUnknownImpl, Impl: IPortableDeviceServiceCapabilities_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, event: *const ::windows::core::GUID, ppattributes: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetEventAttributes(::core::mem::transmute_copy(&event)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppattributes = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetEventParameterAttributes<Identity: ::windows::core::IUnknownImpl, Impl: IPortableDeviceServiceCapabilities_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, event: *const ::windows::core::GUID, parameter: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, ppattributes: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetEventParameterAttributes(::core::mem::transmute_copy(&event), ::core::mem::transmute_copy(&parameter)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppattributes = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetInheritedServices<Identity: ::windows::core::IUnknownImpl, Impl: IPortableDeviceServiceCapabilities_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwinheritancetype: u32, ppservices: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetInheritedServices(::core::mem::transmute_copy(&dwinheritancetype)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppservices = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFormatRenderingProfiles<Identity: ::windows::core::IUnknownImpl, Impl: IPortableDeviceServiceCapabilities_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, format: *const ::windows::core::GUID, pprenderingprofiles: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetFormatRenderingProfiles(::core::mem::transmute_copy(&format)) {
                ::core::result::Result::Ok(ok__) => {
                    *pprenderingprofiles = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSupportedCommands<Identity: ::windows::core::IUnknownImpl, Impl: IPortableDeviceServiceCapabilities_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppcommands: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetSupportedCommands() {
                ::core::result::Result::Ok(ok__) => {
                    *ppcommands = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCommandOptions<Identity: ::windows::core::IUnknownImpl, Impl: IPortableDeviceServiceCapabilities_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, command: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, ppoptions: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetCommandOptions(::core::mem::transmute_copy(&command)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppoptions = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Cancel<Identity: ::windows::core::IUnknownImpl, Impl: IPortableDeviceServiceCapabilities_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Cancel().into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            GetSupportedMethods: GetSupportedMethods::<Identity, Impl, OFFSET>,
            GetSupportedMethodsByFormat: GetSupportedMethodsByFormat::<Identity, Impl, OFFSET>,
            GetMethodAttributes: GetMethodAttributes::<Identity, Impl, OFFSET>,
            GetMethodParameterAttributes: GetMethodParameterAttributes::<Identity, Impl, OFFSET>,
            GetSupportedFormats: GetSupportedFormats::<Identity, Impl, OFFSET>,
            GetFormatAttributes: GetFormatAttributes::<Identity, Impl, OFFSET>,
            GetSupportedFormatProperties: GetSupportedFormatProperties::<Identity, Impl, OFFSET>,
            GetFormatPropertyAttributes: GetFormatPropertyAttributes::<Identity, Impl, OFFSET>,
            GetSupportedEvents: GetSupportedEvents::<Identity, Impl, OFFSET>,
            GetEventAttributes: GetEventAttributes::<Identity, Impl, OFFSET>,
            GetEventParameterAttributes: GetEventParameterAttributes::<Identity, Impl, OFFSET>,
            GetInheritedServices: GetInheritedServices::<Identity, Impl, OFFSET>,
            GetFormatRenderingProfiles: GetFormatRenderingProfiles::<Identity, Impl, OFFSET>,
            GetSupportedCommands: GetSupportedCommands::<Identity, Impl, OFFSET>,
            GetCommandOptions: GetCommandOptions::<Identity, Impl, OFFSET>,
            Cancel: Cancel::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPortableDeviceServiceCapabilities as ::windows::core::Interface>::IID
    }
}
pub trait IPortableDeviceServiceManager_Impl: Sized {
    fn GetDeviceServices(&self, pszpnpdeviceid: &::windows::core::PCWSTR, guidservicecategory: *const ::windows::core::GUID, pservices: *mut ::windows::core::PWSTR, pcservices: *mut u32) -> ::windows::core::Result<()>;
    fn GetDeviceForService(&self, pszpnpserviceid: &::windows::core::PCWSTR) -> ::windows::core::Result<::windows::core::PWSTR>;
}
impl IPortableDeviceServiceManager_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPortableDeviceServiceManager_Impl, const OFFSET: isize>() -> IPortableDeviceServiceManager_Vtbl {
        unsafe extern "system" fn GetDeviceServices<Identity: ::windows::core::IUnknownImpl, Impl: IPortableDeviceServiceManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszpnpdeviceid: ::windows::core::PCWSTR, guidservicecategory: *const ::windows::core::GUID, pservices: *mut ::windows::core::PWSTR, pcservices: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetDeviceServices(::core::mem::transmute(&pszpnpdeviceid), ::core::mem::transmute_copy(&guidservicecategory), ::core::mem::transmute_copy(&pservices), ::core::mem::transmute_copy(&pcservices)).into()
        }
        unsafe extern "system" fn GetDeviceForService<Identity: ::windows::core::IUnknownImpl, Impl: IPortableDeviceServiceManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszpnpserviceid: ::windows::core::PCWSTR, ppszpnpdeviceid: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetDeviceForService(::core::mem::transmute(&pszpnpserviceid)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppszpnpdeviceid = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            GetDeviceServices: GetDeviceServices::<Identity, Impl, OFFSET>,
            GetDeviceForService: GetDeviceForService::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPortableDeviceServiceManager as ::windows::core::Interface>::IID
    }
}
pub trait IPortableDeviceServiceMethodCallback_Impl: Sized {
    fn OnComplete(&self, hrstatus: ::windows::core::HRESULT, presults: &::core::option::Option<IPortableDeviceValues>) -> ::windows::core::Result<()>;
}
impl IPortableDeviceServiceMethodCallback_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPortableDeviceServiceMethodCallback_Impl, const OFFSET: isize>() -> IPortableDeviceServiceMethodCallback_Vtbl {
        unsafe extern "system" fn OnComplete<Identity: ::windows::core::IUnknownImpl, Impl: IPortableDeviceServiceMethodCallback_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hrstatus: ::windows::core::HRESULT, presults: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).OnComplete(::core::mem::transmute_copy(&hrstatus), ::core::mem::transmute(&presults)).into()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(), OnComplete: OnComplete::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPortableDeviceServiceMethodCallback as ::windows::core::Interface>::IID
    }
}
pub trait IPortableDeviceServiceMethods_Impl: Sized {
    fn Invoke(&self, method: *const ::windows::core::GUID, pparameters: &::core::option::Option<IPortableDeviceValues>, ppresults: *mut ::core::option::Option<IPortableDeviceValues>) -> ::windows::core::Result<()>;
    fn InvokeAsync(&self, method: *const ::windows::core::GUID, pparameters: &::core::option::Option<IPortableDeviceValues>, pcallback: &::core::option::Option<IPortableDeviceServiceMethodCallback>) -> ::windows::core::Result<()>;
    fn Cancel(&self, pcallback: &::core::option::Option<IPortableDeviceServiceMethodCallback>) -> ::windows::core::Result<()>;
}
impl IPortableDeviceServiceMethods_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPortableDeviceServiceMethods_Impl, const OFFSET: isize>() -> IPortableDeviceServiceMethods_Vtbl {
        unsafe extern "system" fn Invoke<Identity: ::windows::core::IUnknownImpl, Impl: IPortableDeviceServiceMethods_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, method: *const ::windows::core::GUID, pparameters: ::windows::core::RawPtr, ppresults: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Invoke(::core::mem::transmute_copy(&method), ::core::mem::transmute(&pparameters), ::core::mem::transmute_copy(&ppresults)).into()
        }
        unsafe extern "system" fn InvokeAsync<Identity: ::windows::core::IUnknownImpl, Impl: IPortableDeviceServiceMethods_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, method: *const ::windows::core::GUID, pparameters: ::windows::core::RawPtr, pcallback: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).InvokeAsync(::core::mem::transmute_copy(&method), ::core::mem::transmute(&pparameters), ::core::mem::transmute(&pcallback)).into()
        }
        unsafe extern "system" fn Cancel<Identity: ::windows::core::IUnknownImpl, Impl: IPortableDeviceServiceMethods_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcallback: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Cancel(::core::mem::transmute(&pcallback)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            Invoke: Invoke::<Identity, Impl, OFFSET>,
            InvokeAsync: InvokeAsync::<Identity, Impl, OFFSET>,
            Cancel: Cancel::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPortableDeviceServiceMethods as ::windows::core::Interface>::IID
    }
}
pub trait IPortableDeviceServiceOpenCallback_Impl: Sized {
    fn OnComplete(&self, hrstatus: ::windows::core::HRESULT) -> ::windows::core::Result<()>;
}
impl IPortableDeviceServiceOpenCallback_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPortableDeviceServiceOpenCallback_Impl, const OFFSET: isize>() -> IPortableDeviceServiceOpenCallback_Vtbl {
        unsafe extern "system" fn OnComplete<Identity: ::windows::core::IUnknownImpl, Impl: IPortableDeviceServiceOpenCallback_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hrstatus: ::windows::core::HRESULT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).OnComplete(::core::mem::transmute_copy(&hrstatus)).into()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(), OnComplete: OnComplete::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPortableDeviceServiceOpenCallback as ::windows::core::Interface>::IID
    }
}
pub trait IPortableDeviceUnitsStream_Impl: Sized {
    fn SeekInUnits(&self, dlibmove: i64, units: WPD_STREAM_UNITS, dworigin: u32) -> ::windows::core::Result<u64>;
    fn Cancel(&self) -> ::windows::core::Result<()>;
}
impl IPortableDeviceUnitsStream_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPortableDeviceUnitsStream_Impl, const OFFSET: isize>() -> IPortableDeviceUnitsStream_Vtbl {
        unsafe extern "system" fn SeekInUnits<Identity: ::windows::core::IUnknownImpl, Impl: IPortableDeviceUnitsStream_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dlibmove: i64, units: WPD_STREAM_UNITS, dworigin: u32, plibnewposition: *mut u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).SeekInUnits(::core::mem::transmute_copy(&dlibmove), ::core::mem::transmute_copy(&units), ::core::mem::transmute_copy(&dworigin)) {
                ::core::result::Result::Ok(ok__) => {
                    *plibnewposition = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Cancel<Identity: ::windows::core::IUnknownImpl, Impl: IPortableDeviceUnitsStream_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Cancel().into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            SeekInUnits: SeekInUnits::<Identity, Impl, OFFSET>,
            Cancel: Cancel::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPortableDeviceUnitsStream as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_UI_Shell_PropertiesSystem"))]
pub trait IPortableDeviceValues_Impl: Sized {
    fn GetCount(&self, pcelt: *const u32) -> ::windows::core::Result<()>;
    fn GetAt(&self, index: u32, pkey: *mut super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, pvalue: *mut super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows::core::Result<()>;
    fn SetValue(&self, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, pvalue: *const super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows::core::Result<()>;
    fn GetValue(&self, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY) -> ::windows::core::Result<super::super::System::Com::StructuredStorage::PROPVARIANT>;
    fn SetStringValue(&self, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, value: &::windows::core::PCWSTR) -> ::windows::core::Result<()>;
    fn GetStringValue(&self, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY) -> ::windows::core::Result<::windows::core::PWSTR>;
    fn SetUnsignedIntegerValue(&self, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, value: u32) -> ::windows::core::Result<()>;
    fn GetUnsignedIntegerValue(&self, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY) -> ::windows::core::Result<u32>;
    fn SetSignedIntegerValue(&self, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, value: i32) -> ::windows::core::Result<()>;
    fn GetSignedIntegerValue(&self, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY) -> ::windows::core::Result<i32>;
    fn SetUnsignedLargeIntegerValue(&self, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, value: u64) -> ::windows::core::Result<()>;
    fn GetUnsignedLargeIntegerValue(&self, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY) -> ::windows::core::Result<u64>;
    fn SetSignedLargeIntegerValue(&self, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, value: i64) -> ::windows::core::Result<()>;
    fn GetSignedLargeIntegerValue(&self, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY) -> ::windows::core::Result<i64>;
    fn SetFloatValue(&self, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, value: f32) -> ::windows::core::Result<()>;
    fn GetFloatValue(&self, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY) -> ::windows::core::Result<f32>;
    fn SetErrorValue(&self, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, value: ::windows::core::HRESULT) -> ::windows::core::Result<()>;
    fn GetErrorValue(&self, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY) -> ::windows::core::Result<::windows::core::HRESULT>;
    fn SetKeyValue(&self, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, value: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY) -> ::windows::core::Result<()>;
    fn GetKeyValue(&self, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY) -> ::windows::core::Result<super::super::UI::Shell::PropertiesSystem::PROPERTYKEY>;
    fn SetBoolValue(&self, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, value: super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn GetBoolValue(&self, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY) -> ::windows::core::Result<super::super::Foundation::BOOL>;
    fn SetIUnknownValue(&self, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, pvalue: &::core::option::Option<::windows::core::IUnknown>) -> ::windows::core::Result<()>;
    fn GetIUnknownValue(&self, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY) -> ::windows::core::Result<::windows::core::IUnknown>;
    fn SetGuidValue(&self, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, value: *const ::windows::core::GUID) -> ::windows::core::Result<()>;
    fn GetGuidValue(&self, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY) -> ::windows::core::Result<::windows::core::GUID>;
    fn SetBufferValue(&self, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, pvalue: *const u8, cbvalue: u32) -> ::windows::core::Result<()>;
    fn GetBufferValue(&self, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, ppvalue: *mut *mut u8, pcbvalue: *mut u32) -> ::windows::core::Result<()>;
    fn SetIPortableDeviceValuesValue(&self, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, pvalue: &::core::option::Option<IPortableDeviceValues>) -> ::windows::core::Result<()>;
    fn GetIPortableDeviceValuesValue(&self, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY) -> ::windows::core::Result<IPortableDeviceValues>;
    fn SetIPortableDevicePropVariantCollectionValue(&self, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, pvalue: &::core::option::Option<IPortableDevicePropVariantCollection>) -> ::windows::core::Result<()>;
    fn GetIPortableDevicePropVariantCollectionValue(&self, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY) -> ::windows::core::Result<IPortableDevicePropVariantCollection>;
    fn SetIPortableDeviceKeyCollectionValue(&self, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, pvalue: &::core::option::Option<IPortableDeviceKeyCollection>) -> ::windows::core::Result<()>;
    fn GetIPortableDeviceKeyCollectionValue(&self, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY) -> ::windows::core::Result<IPortableDeviceKeyCollection>;
    fn SetIPortableDeviceValuesCollectionValue(&self, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, pvalue: &::core::option::Option<IPortableDeviceValuesCollection>) -> ::windows::core::Result<()>;
    fn GetIPortableDeviceValuesCollectionValue(&self, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY) -> ::windows::core::Result<IPortableDeviceValuesCollection>;
    fn RemoveValue(&self, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY) -> ::windows::core::Result<()>;
    fn CopyValuesFromPropertyStore(&self, pstore: &::core::option::Option<super::super::UI::Shell::PropertiesSystem::IPropertyStore>) -> ::windows::core::Result<()>;
    fn CopyValuesToPropertyStore(&self, pstore: &::core::option::Option<super::super::UI::Shell::PropertiesSystem::IPropertyStore>) -> ::windows::core::Result<()>;
    fn Clear(&self) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_UI_Shell_PropertiesSystem"))]
impl IPortableDeviceValues_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPortableDeviceValues_Impl, const OFFSET: isize>() -> IPortableDeviceValues_Vtbl {
        unsafe extern "system" fn GetCount<Identity: ::windows::core::IUnknownImpl, Impl: IPortableDeviceValues_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcelt: *const u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetCount(::core::mem::transmute_copy(&pcelt)).into()
        }
        unsafe extern "system" fn GetAt<Identity: ::windows::core::IUnknownImpl, Impl: IPortableDeviceValues_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32, pkey: *mut super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, pvalue: *mut super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetAt(::core::mem::transmute_copy(&index), ::core::mem::transmute_copy(&pkey), ::core::mem::transmute_copy(&pvalue)).into()
        }
        unsafe extern "system" fn SetValue<Identity: ::windows::core::IUnknownImpl, Impl: IPortableDeviceValues_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, pvalue: *const super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetValue(::core::mem::transmute_copy(&key), ::core::mem::transmute_copy(&pvalue)).into()
        }
        unsafe extern "system" fn GetValue<Identity: ::windows::core::IUnknownImpl, Impl: IPortableDeviceValues_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, pvalue: *mut super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetValue(::core::mem::transmute_copy(&key)) {
                ::core::result::Result::Ok(ok__) => {
                    *pvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetStringValue<Identity: ::windows::core::IUnknownImpl, Impl: IPortableDeviceValues_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, value: ::windows::core::PCWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetStringValue(::core::mem::transmute_copy(&key), ::core::mem::transmute(&value)).into()
        }
        unsafe extern "system" fn GetStringValue<Identity: ::windows::core::IUnknownImpl, Impl: IPortableDeviceValues_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, pvalue: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetStringValue(::core::mem::transmute_copy(&key)) {
                ::core::result::Result::Ok(ok__) => {
                    *pvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetUnsignedIntegerValue<Identity: ::windows::core::IUnknownImpl, Impl: IPortableDeviceValues_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, value: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetUnsignedIntegerValue(::core::mem::transmute_copy(&key), ::core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn GetUnsignedIntegerValue<Identity: ::windows::core::IUnknownImpl, Impl: IPortableDeviceValues_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, pvalue: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetUnsignedIntegerValue(::core::mem::transmute_copy(&key)) {
                ::core::result::Result::Ok(ok__) => {
                    *pvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSignedIntegerValue<Identity: ::windows::core::IUnknownImpl, Impl: IPortableDeviceValues_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, value: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetSignedIntegerValue(::core::mem::transmute_copy(&key), ::core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn GetSignedIntegerValue<Identity: ::windows::core::IUnknownImpl, Impl: IPortableDeviceValues_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, pvalue: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetSignedIntegerValue(::core::mem::transmute_copy(&key)) {
                ::core::result::Result::Ok(ok__) => {
                    *pvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetUnsignedLargeIntegerValue<Identity: ::windows::core::IUnknownImpl, Impl: IPortableDeviceValues_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, value: u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetUnsignedLargeIntegerValue(::core::mem::transmute_copy(&key), ::core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn GetUnsignedLargeIntegerValue<Identity: ::windows::core::IUnknownImpl, Impl: IPortableDeviceValues_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, pvalue: *mut u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetUnsignedLargeIntegerValue(::core::mem::transmute_copy(&key)) {
                ::core::result::Result::Ok(ok__) => {
                    *pvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSignedLargeIntegerValue<Identity: ::windows::core::IUnknownImpl, Impl: IPortableDeviceValues_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, value: i64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetSignedLargeIntegerValue(::core::mem::transmute_copy(&key), ::core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn GetSignedLargeIntegerValue<Identity: ::windows::core::IUnknownImpl, Impl: IPortableDeviceValues_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, pvalue: *mut i64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetSignedLargeIntegerValue(::core::mem::transmute_copy(&key)) {
                ::core::result::Result::Ok(ok__) => {
                    *pvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetFloatValue<Identity: ::windows::core::IUnknownImpl, Impl: IPortableDeviceValues_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, value: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetFloatValue(::core::mem::transmute_copy(&key), ::core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn GetFloatValue<Identity: ::windows::core::IUnknownImpl, Impl: IPortableDeviceValues_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, pvalue: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetFloatValue(::core::mem::transmute_copy(&key)) {
                ::core::result::Result::Ok(ok__) => {
                    *pvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetErrorValue<Identity: ::windows::core::IUnknownImpl, Impl: IPortableDeviceValues_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, value: ::windows::core::HRESULT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetErrorValue(::core::mem::transmute_copy(&key), ::core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn GetErrorValue<Identity: ::windows::core::IUnknownImpl, Impl: IPortableDeviceValues_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, pvalue: *mut ::windows::core::HRESULT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetErrorValue(::core::mem::transmute_copy(&key)) {
                ::core::result::Result::Ok(ok__) => {
                    *pvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetKeyValue<Identity: ::windows::core::IUnknownImpl, Impl: IPortableDeviceValues_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, value: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetKeyValue(::core::mem::transmute_copy(&key), ::core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn GetKeyValue<Identity: ::windows::core::IUnknownImpl, Impl: IPortableDeviceValues_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, pvalue: *mut super::super::UI::Shell::PropertiesSystem::PROPERTYKEY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetKeyValue(::core::mem::transmute_copy(&key)) {
                ::core::result::Result::Ok(ok__) => {
                    *pvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetBoolValue<Identity: ::windows::core::IUnknownImpl, Impl: IPortableDeviceValues_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, value: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetBoolValue(::core::mem::transmute_copy(&key), ::core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn GetBoolValue<Identity: ::windows::core::IUnknownImpl, Impl: IPortableDeviceValues_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, pvalue: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetBoolValue(::core::mem::transmute_copy(&key)) {
                ::core::result::Result::Ok(ok__) => {
                    *pvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIUnknownValue<Identity: ::windows::core::IUnknownImpl, Impl: IPortableDeviceValues_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, pvalue: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetIUnknownValue(::core::mem::transmute_copy(&key), ::core::mem::transmute(&pvalue)).into()
        }
        unsafe extern "system" fn GetIUnknownValue<Identity: ::windows::core::IUnknownImpl, Impl: IPortableDeviceValues_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, ppvalue: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetIUnknownValue(::core::mem::transmute_copy(&key)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetGuidValue<Identity: ::windows::core::IUnknownImpl, Impl: IPortableDeviceValues_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, value: *const ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetGuidValue(::core::mem::transmute_copy(&key), ::core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn GetGuidValue<Identity: ::windows::core::IUnknownImpl, Impl: IPortableDeviceValues_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, pvalue: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetGuidValue(::core::mem::transmute_copy(&key)) {
                ::core::result::Result::Ok(ok__) => {
                    *pvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetBufferValue<Identity: ::windows::core::IUnknownImpl, Impl: IPortableDeviceValues_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, pvalue: *const u8, cbvalue: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetBufferValue(::core::mem::transmute_copy(&key), ::core::mem::transmute_copy(&pvalue), ::core::mem::transmute_copy(&cbvalue)).into()
        }
        unsafe extern "system" fn GetBufferValue<Identity: ::windows::core::IUnknownImpl, Impl: IPortableDeviceValues_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, ppvalue: *mut *mut u8, pcbvalue: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetBufferValue(::core::mem::transmute_copy(&key), ::core::mem::transmute_copy(&ppvalue), ::core::mem::transmute_copy(&pcbvalue)).into()
        }
        unsafe extern "system" fn SetIPortableDeviceValuesValue<Identity: ::windows::core::IUnknownImpl, Impl: IPortableDeviceValues_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, pvalue: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetIPortableDeviceValuesValue(::core::mem::transmute_copy(&key), ::core::mem::transmute(&pvalue)).into()
        }
        unsafe extern "system" fn GetIPortableDeviceValuesValue<Identity: ::windows::core::IUnknownImpl, Impl: IPortableDeviceValues_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, ppvalue: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetIPortableDeviceValuesValue(::core::mem::transmute_copy(&key)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIPortableDevicePropVariantCollectionValue<Identity: ::windows::core::IUnknownImpl, Impl: IPortableDeviceValues_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, pvalue: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetIPortableDevicePropVariantCollectionValue(::core::mem::transmute_copy(&key), ::core::mem::transmute(&pvalue)).into()
        }
        unsafe extern "system" fn GetIPortableDevicePropVariantCollectionValue<Identity: ::windows::core::IUnknownImpl, Impl: IPortableDeviceValues_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, ppvalue: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetIPortableDevicePropVariantCollectionValue(::core::mem::transmute_copy(&key)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIPortableDeviceKeyCollectionValue<Identity: ::windows::core::IUnknownImpl, Impl: IPortableDeviceValues_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, pvalue: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetIPortableDeviceKeyCollectionValue(::core::mem::transmute_copy(&key), ::core::mem::transmute(&pvalue)).into()
        }
        unsafe extern "system" fn GetIPortableDeviceKeyCollectionValue<Identity: ::windows::core::IUnknownImpl, Impl: IPortableDeviceValues_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, ppvalue: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetIPortableDeviceKeyCollectionValue(::core::mem::transmute_copy(&key)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIPortableDeviceValuesCollectionValue<Identity: ::windows::core::IUnknownImpl, Impl: IPortableDeviceValues_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, pvalue: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetIPortableDeviceValuesCollectionValue(::core::mem::transmute_copy(&key), ::core::mem::transmute(&pvalue)).into()
        }
        unsafe extern "system" fn GetIPortableDeviceValuesCollectionValue<Identity: ::windows::core::IUnknownImpl, Impl: IPortableDeviceValues_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, ppvalue: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetIPortableDeviceValuesCollectionValue(::core::mem::transmute_copy(&key)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveValue<Identity: ::windows::core::IUnknownImpl, Impl: IPortableDeviceValues_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).RemoveValue(::core::mem::transmute_copy(&key)).into()
        }
        unsafe extern "system" fn CopyValuesFromPropertyStore<Identity: ::windows::core::IUnknownImpl, Impl: IPortableDeviceValues_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pstore: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).CopyValuesFromPropertyStore(::core::mem::transmute(&pstore)).into()
        }
        unsafe extern "system" fn CopyValuesToPropertyStore<Identity: ::windows::core::IUnknownImpl, Impl: IPortableDeviceValues_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pstore: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).CopyValuesToPropertyStore(::core::mem::transmute(&pstore)).into()
        }
        unsafe extern "system" fn Clear<Identity: ::windows::core::IUnknownImpl, Impl: IPortableDeviceValues_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Clear().into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            GetCount: GetCount::<Identity, Impl, OFFSET>,
            GetAt: GetAt::<Identity, Impl, OFFSET>,
            SetValue: SetValue::<Identity, Impl, OFFSET>,
            GetValue: GetValue::<Identity, Impl, OFFSET>,
            SetStringValue: SetStringValue::<Identity, Impl, OFFSET>,
            GetStringValue: GetStringValue::<Identity, Impl, OFFSET>,
            SetUnsignedIntegerValue: SetUnsignedIntegerValue::<Identity, Impl, OFFSET>,
            GetUnsignedIntegerValue: GetUnsignedIntegerValue::<Identity, Impl, OFFSET>,
            SetSignedIntegerValue: SetSignedIntegerValue::<Identity, Impl, OFFSET>,
            GetSignedIntegerValue: GetSignedIntegerValue::<Identity, Impl, OFFSET>,
            SetUnsignedLargeIntegerValue: SetUnsignedLargeIntegerValue::<Identity, Impl, OFFSET>,
            GetUnsignedLargeIntegerValue: GetUnsignedLargeIntegerValue::<Identity, Impl, OFFSET>,
            SetSignedLargeIntegerValue: SetSignedLargeIntegerValue::<Identity, Impl, OFFSET>,
            GetSignedLargeIntegerValue: GetSignedLargeIntegerValue::<Identity, Impl, OFFSET>,
            SetFloatValue: SetFloatValue::<Identity, Impl, OFFSET>,
            GetFloatValue: GetFloatValue::<Identity, Impl, OFFSET>,
            SetErrorValue: SetErrorValue::<Identity, Impl, OFFSET>,
            GetErrorValue: GetErrorValue::<Identity, Impl, OFFSET>,
            SetKeyValue: SetKeyValue::<Identity, Impl, OFFSET>,
            GetKeyValue: GetKeyValue::<Identity, Impl, OFFSET>,
            SetBoolValue: SetBoolValue::<Identity, Impl, OFFSET>,
            GetBoolValue: GetBoolValue::<Identity, Impl, OFFSET>,
            SetIUnknownValue: SetIUnknownValue::<Identity, Impl, OFFSET>,
            GetIUnknownValue: GetIUnknownValue::<Identity, Impl, OFFSET>,
            SetGuidValue: SetGuidValue::<Identity, Impl, OFFSET>,
            GetGuidValue: GetGuidValue::<Identity, Impl, OFFSET>,
            SetBufferValue: SetBufferValue::<Identity, Impl, OFFSET>,
            GetBufferValue: GetBufferValue::<Identity, Impl, OFFSET>,
            SetIPortableDeviceValuesValue: SetIPortableDeviceValuesValue::<Identity, Impl, OFFSET>,
            GetIPortableDeviceValuesValue: GetIPortableDeviceValuesValue::<Identity, Impl, OFFSET>,
            SetIPortableDevicePropVariantCollectionValue: SetIPortableDevicePropVariantCollectionValue::<Identity, Impl, OFFSET>,
            GetIPortableDevicePropVariantCollectionValue: GetIPortableDevicePropVariantCollectionValue::<Identity, Impl, OFFSET>,
            SetIPortableDeviceKeyCollectionValue: SetIPortableDeviceKeyCollectionValue::<Identity, Impl, OFFSET>,
            GetIPortableDeviceKeyCollectionValue: GetIPortableDeviceKeyCollectionValue::<Identity, Impl, OFFSET>,
            SetIPortableDeviceValuesCollectionValue: SetIPortableDeviceValuesCollectionValue::<Identity, Impl, OFFSET>,
            GetIPortableDeviceValuesCollectionValue: GetIPortableDeviceValuesCollectionValue::<Identity, Impl, OFFSET>,
            RemoveValue: RemoveValue::<Identity, Impl, OFFSET>,
            CopyValuesFromPropertyStore: CopyValuesFromPropertyStore::<Identity, Impl, OFFSET>,
            CopyValuesToPropertyStore: CopyValuesToPropertyStore::<Identity, Impl, OFFSET>,
            Clear: Clear::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPortableDeviceValues as ::windows::core::Interface>::IID
    }
}
pub trait IPortableDeviceValuesCollection_Impl: Sized {
    fn GetCount(&self, pcelems: *const u32) -> ::windows::core::Result<()>;
    fn GetAt(&self, dwindex: u32) -> ::windows::core::Result<IPortableDeviceValues>;
    fn Add(&self, pvalues: &::core::option::Option<IPortableDeviceValues>) -> ::windows::core::Result<()>;
    fn Clear(&self) -> ::windows::core::Result<()>;
    fn RemoveAt(&self, dwindex: u32) -> ::windows::core::Result<()>;
}
impl IPortableDeviceValuesCollection_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPortableDeviceValuesCollection_Impl, const OFFSET: isize>() -> IPortableDeviceValuesCollection_Vtbl {
        unsafe extern "system" fn GetCount<Identity: ::windows::core::IUnknownImpl, Impl: IPortableDeviceValuesCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcelems: *const u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetCount(::core::mem::transmute_copy(&pcelems)).into()
        }
        unsafe extern "system" fn GetAt<Identity: ::windows::core::IUnknownImpl, Impl: IPortableDeviceValuesCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwindex: u32, ppvalues: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetAt(::core::mem::transmute_copy(&dwindex)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppvalues = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Add<Identity: ::windows::core::IUnknownImpl, Impl: IPortableDeviceValuesCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalues: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Add(::core::mem::transmute(&pvalues)).into()
        }
        unsafe extern "system" fn Clear<Identity: ::windows::core::IUnknownImpl, Impl: IPortableDeviceValuesCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Clear().into()
        }
        unsafe extern "system" fn RemoveAt<Identity: ::windows::core::IUnknownImpl, Impl: IPortableDeviceValuesCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwindex: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).RemoveAt(::core::mem::transmute_copy(&dwindex)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            GetCount: GetCount::<Identity, Impl, OFFSET>,
            GetAt: GetAt::<Identity, Impl, OFFSET>,
            Add: Add::<Identity, Impl, OFFSET>,
            Clear: Clear::<Identity, Impl, OFFSET>,
            RemoveAt: RemoveAt::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPortableDeviceValuesCollection as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IPortableDeviceWebControl_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn GetDeviceFromId(&self, deviceid: &super::super::Foundation::BSTR) -> ::windows::core::Result<super::super::System::Com::IDispatch>;
    fn GetDeviceFromIdAsync(&self, deviceid: &super::super::Foundation::BSTR, pcompletionhandler: &::core::option::Option<super::super::System::Com::IDispatch>, perrorhandler: &::core::option::Option<super::super::System::Com::IDispatch>) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IPortableDeviceWebControl_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPortableDeviceWebControl_Impl, const OFFSET: isize>() -> IPortableDeviceWebControl_Vtbl {
        unsafe extern "system" fn GetDeviceFromId<Identity: ::windows::core::IUnknownImpl, Impl: IPortableDeviceWebControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, deviceid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ppdevice: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetDeviceFromId(::core::mem::transmute(&deviceid)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppdevice = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDeviceFromIdAsync<Identity: ::windows::core::IUnknownImpl, Impl: IPortableDeviceWebControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, deviceid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pcompletionhandler: ::windows::core::RawPtr, perrorhandler: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetDeviceFromIdAsync(::core::mem::transmute(&deviceid), ::core::mem::transmute(&pcompletionhandler), ::core::mem::transmute(&perrorhandler)).into()
        }
        Self {
            base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            GetDeviceFromId: GetDeviceFromId::<Identity, Impl, OFFSET>,
            GetDeviceFromIdAsync: GetDeviceFromIdAsync::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPortableDeviceWebControl as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IRadioInstance_Impl: Sized {
    fn GetRadioManagerSignature(&self) -> ::windows::core::Result<::windows::core::GUID>;
    fn GetInstanceSignature(&self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn GetFriendlyName(&self, lcid: u32) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn GetRadioState(&self) -> ::windows::core::Result<DEVICE_RADIO_STATE>;
    fn SetRadioState(&self, radiostate: DEVICE_RADIO_STATE, utimeoutsec: u32) -> ::windows::core::Result<()>;
    fn IsMultiComm(&self) -> super::super::Foundation::BOOL;
    fn IsAssociatingDevice(&self) -> super::super::Foundation::BOOL;
}
#[cfg(feature = "Win32_Foundation")]
impl IRadioInstance_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRadioInstance_Impl, const OFFSET: isize>() -> IRadioInstance_Vtbl {
        unsafe extern "system" fn GetRadioManagerSignature<Identity: ::windows::core::IUnknownImpl, Impl: IRadioInstance_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pguidsignature: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetRadioManagerSignature() {
                ::core::result::Result::Ok(ok__) => {
                    *pguidsignature = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetInstanceSignature<Identity: ::windows::core::IUnknownImpl, Impl: IRadioInstance_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrid: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetInstanceSignature() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstrid = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFriendlyName<Identity: ::windows::core::IUnknownImpl, Impl: IRadioInstance_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lcid: u32, pbstrname: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetFriendlyName(::core::mem::transmute_copy(&lcid)) {
                ::core::result::Result::Ok(ok__) => {
                    *pbstrname = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetRadioState<Identity: ::windows::core::IUnknownImpl, Impl: IRadioInstance_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pradiostate: *mut DEVICE_RADIO_STATE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetRadioState() {
                ::core::result::Result::Ok(ok__) => {
                    *pradiostate = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetRadioState<Identity: ::windows::core::IUnknownImpl, Impl: IRadioInstance_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, radiostate: DEVICE_RADIO_STATE, utimeoutsec: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetRadioState(::core::mem::transmute_copy(&radiostate), ::core::mem::transmute_copy(&utimeoutsec)).into()
        }
        unsafe extern "system" fn IsMultiComm<Identity: ::windows::core::IUnknownImpl, Impl: IRadioInstance_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).IsMultiComm()
        }
        unsafe extern "system" fn IsAssociatingDevice<Identity: ::windows::core::IUnknownImpl, Impl: IRadioInstance_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).IsAssociatingDevice()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            GetRadioManagerSignature: GetRadioManagerSignature::<Identity, Impl, OFFSET>,
            GetInstanceSignature: GetInstanceSignature::<Identity, Impl, OFFSET>,
            GetFriendlyName: GetFriendlyName::<Identity, Impl, OFFSET>,
            GetRadioState: GetRadioState::<Identity, Impl, OFFSET>,
            SetRadioState: SetRadioState::<Identity, Impl, OFFSET>,
            IsMultiComm: IsMultiComm::<Identity, Impl, OFFSET>,
            IsAssociatingDevice: IsAssociatingDevice::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRadioInstance as ::windows::core::Interface>::IID
    }
}
pub trait IRadioInstanceCollection_Impl: Sized {
    fn GetCount(&self) -> ::windows::core::Result<u32>;
    fn GetAt(&self, uindex: u32) -> ::windows::core::Result<IRadioInstance>;
}
impl IRadioInstanceCollection_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRadioInstanceCollection_Impl, const OFFSET: isize>() -> IRadioInstanceCollection_Vtbl {
        unsafe extern "system" fn GetCount<Identity: ::windows::core::IUnknownImpl, Impl: IRadioInstanceCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcinstance: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetCount() {
                ::core::result::Result::Ok(ok__) => {
                    *pcinstance = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetAt<Identity: ::windows::core::IUnknownImpl, Impl: IRadioInstanceCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, uindex: u32, ppradioinstance: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetAt(::core::mem::transmute_copy(&uindex)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppradioinstance = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            GetCount: GetCount::<Identity, Impl, OFFSET>,
            GetAt: GetAt::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRadioInstanceCollection as ::windows::core::Interface>::IID
    }
}
pub trait IWpdSerializer_Impl: Sized {
    fn GetIPortableDeviceValuesFromBuffer(&self, pbuffer: *const u8, dwinputbufferlength: u32) -> ::windows::core::Result<IPortableDeviceValues>;
    fn WriteIPortableDeviceValuesToBuffer(&self, dwoutputbufferlength: u32, presults: &::core::option::Option<IPortableDeviceValues>, pbuffer: *mut u8, pdwbyteswritten: *mut u32) -> ::windows::core::Result<()>;
    fn GetBufferFromIPortableDeviceValues(&self, psource: &::core::option::Option<IPortableDeviceValues>, ppbuffer: *mut *mut u8, pdwbuffersize: *mut u32) -> ::windows::core::Result<()>;
    fn GetSerializedSize(&self, psource: &::core::option::Option<IPortableDeviceValues>) -> ::windows::core::Result<u32>;
}
impl IWpdSerializer_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWpdSerializer_Impl, const OFFSET: isize>() -> IWpdSerializer_Vtbl {
        unsafe extern "system" fn GetIPortableDeviceValuesFromBuffer<Identity: ::windows::core::IUnknownImpl, Impl: IWpdSerializer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbuffer: *const u8, dwinputbufferlength: u32, ppparams: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetIPortableDeviceValuesFromBuffer(::core::mem::transmute_copy(&pbuffer), ::core::mem::transmute_copy(&dwinputbufferlength)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppparams = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn WriteIPortableDeviceValuesToBuffer<Identity: ::windows::core::IUnknownImpl, Impl: IWpdSerializer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwoutputbufferlength: u32, presults: ::windows::core::RawPtr, pbuffer: *mut u8, pdwbyteswritten: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).WriteIPortableDeviceValuesToBuffer(::core::mem::transmute_copy(&dwoutputbufferlength), ::core::mem::transmute(&presults), ::core::mem::transmute_copy(&pbuffer), ::core::mem::transmute_copy(&pdwbyteswritten)).into()
        }
        unsafe extern "system" fn GetBufferFromIPortableDeviceValues<Identity: ::windows::core::IUnknownImpl, Impl: IWpdSerializer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psource: ::windows::core::RawPtr, ppbuffer: *mut *mut u8, pdwbuffersize: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetBufferFromIPortableDeviceValues(::core::mem::transmute(&psource), ::core::mem::transmute_copy(&ppbuffer), ::core::mem::transmute_copy(&pdwbuffersize)).into()
        }
        unsafe extern "system" fn GetSerializedSize<Identity: ::windows::core::IUnknownImpl, Impl: IWpdSerializer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psource: ::windows::core::RawPtr, pdwsize: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetSerializedSize(::core::mem::transmute(&psource)) {
                ::core::result::Result::Ok(ok__) => {
                    *pdwsize = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            GetIPortableDeviceValuesFromBuffer: GetIPortableDeviceValuesFromBuffer::<Identity, Impl, OFFSET>,
            WriteIPortableDeviceValuesToBuffer: WriteIPortableDeviceValuesToBuffer::<Identity, Impl, OFFSET>,
            GetBufferFromIPortableDeviceValues: GetBufferFromIPortableDeviceValues::<Identity, Impl, OFFSET>,
            GetSerializedSize: GetSerializedSize::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWpdSerializer as ::windows::core::Interface>::IID
    }
}
