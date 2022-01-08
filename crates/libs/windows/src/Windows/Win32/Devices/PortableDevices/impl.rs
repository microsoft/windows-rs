pub trait IConnectionRequestCallbackImpl: Sized {
    fn OnComplete();
}
impl ::windows::core::RuntimeName for IConnectionRequestCallback {
    const NAME: &'static str = "Windows.Win32.Devices.PortableDevices.IConnectionRequestCallback";
}
impl IConnectionRequestCallbackVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IConnectionRequestCallbackImpl, const OFFSET: isize>() -> IConnectionRequestCallbackVtbl {
        unsafe extern "system" fn OnComplete<Impl: IConnectionRequestCallbackImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hrstatus: ::windows::core::HRESULT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OnComplete(hrstatus) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IConnectionRequestCallback>, ::windows::core::GetTrustLevel, OnComplete::<Impl, OFFSET>)
    }
}
pub trait IEnumPortableDeviceConnectorsImpl: Sized {
    fn Next();
    fn Skip();
    fn Reset();
    fn Clone();
}
impl ::windows::core::RuntimeName for IEnumPortableDeviceConnectors {
    const NAME: &'static str = "Windows.Win32.Devices.PortableDevices.IEnumPortableDeviceConnectors";
}
impl IEnumPortableDeviceConnectorsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IEnumPortableDeviceConnectorsImpl, const OFFSET: isize>() -> IEnumPortableDeviceConnectorsVtbl {
        unsafe extern "system" fn Next<Impl: IEnumPortableDeviceConnectorsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, crequested: u32, pconnectors: *mut ::windows::core::RawPtr, pcfetched: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Next(crequested, ::core::mem::transmute_copy(&pconnectors), pcfetched) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Skip<Impl: IEnumPortableDeviceConnectorsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cconnectors: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Skip(cconnectors) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Reset<Impl: IEnumPortableDeviceConnectorsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Clone<Impl: IEnumPortableDeviceConnectorsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IEnumPortableDeviceConnectors>, ::windows::core::GetTrustLevel, Next::<Impl, OFFSET>, Skip::<Impl, OFFSET>, Reset::<Impl, OFFSET>, Clone::<Impl, OFFSET>)
    }
}
pub trait IEnumPortableDeviceObjectIDsImpl: Sized {
    fn Next();
    fn Skip();
    fn Reset();
    fn Clone();
    fn Cancel();
}
impl ::windows::core::RuntimeName for IEnumPortableDeviceObjectIDs {
    const NAME: &'static str = "Windows.Win32.Devices.PortableDevices.IEnumPortableDeviceObjectIDs";
}
impl IEnumPortableDeviceObjectIDsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IEnumPortableDeviceObjectIDsImpl, const OFFSET: isize>() -> IEnumPortableDeviceObjectIDsVtbl {
        unsafe extern "system" fn Next<Impl: IEnumPortableDeviceObjectIDsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cobjects: u32, pobjids: *mut super::super::Foundation::PWSTR, pcfetched: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Next(cobjects, ::core::mem::transmute_copy(&pobjids), pcfetched) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Skip<Impl: IEnumPortableDeviceObjectIDsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cobjects: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Skip(cobjects) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Reset<Impl: IEnumPortableDeviceObjectIDsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Clone<Impl: IEnumPortableDeviceObjectIDsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Cancel<Impl: IEnumPortableDeviceObjectIDsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Cancel() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IEnumPortableDeviceObjectIDs>, ::windows::core::GetTrustLevel, Next::<Impl, OFFSET>, Skip::<Impl, OFFSET>, Reset::<Impl, OFFSET>, Clone::<Impl, OFFSET>, Cancel::<Impl, OFFSET>)
    }
}
pub trait IMediaRadioManagerImpl: Sized {
    fn GetRadioInstances();
    fn OnSystemRadioStateChange();
}
impl ::windows::core::RuntimeName for IMediaRadioManager {
    const NAME: &'static str = "Windows.Win32.Devices.PortableDevices.IMediaRadioManager";
}
impl IMediaRadioManagerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMediaRadioManagerImpl, const OFFSET: isize>() -> IMediaRadioManagerVtbl {
        unsafe extern "system" fn GetRadioInstances<Impl: IMediaRadioManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppcollection: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetRadioInstances(::core::mem::transmute_copy(&ppcollection)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OnSystemRadioStateChange<Impl: IMediaRadioManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, sysradiostate: SYSTEM_RADIO_STATE, utimeoutsec: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OnSystemRadioStateChange(sysradiostate, utimeoutsec) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IMediaRadioManager>, ::windows::core::GetTrustLevel, GetRadioInstances::<Impl, OFFSET>, OnSystemRadioStateChange::<Impl, OFFSET>)
    }
}
pub trait IMediaRadioManagerNotifySinkImpl: Sized {
    fn OnInstanceAdd();
    fn OnInstanceRemove();
    fn OnInstanceRadioChange();
}
impl ::windows::core::RuntimeName for IMediaRadioManagerNotifySink {
    const NAME: &'static str = "Windows.Win32.Devices.PortableDevices.IMediaRadioManagerNotifySink";
}
impl IMediaRadioManagerNotifySinkVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMediaRadioManagerNotifySinkImpl, const OFFSET: isize>() -> IMediaRadioManagerNotifySinkVtbl {
        unsafe extern "system" fn OnInstanceAdd<Impl: IMediaRadioManagerNotifySinkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pradioinstance: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OnInstanceAdd(&*(&pradioinstance as *const <IRadioInstance as ::windows::core::Abi>::Abi as *const <IRadioInstance as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OnInstanceRemove<Impl: IMediaRadioManagerNotifySinkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrradioinstanceid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OnInstanceRemove(&*(&bstrradioinstanceid as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OnInstanceRadioChange<Impl: IMediaRadioManagerNotifySinkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrradioinstanceid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, radiostate: DEVICE_RADIO_STATE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OnInstanceRadioChange(&*(&bstrradioinstanceid as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), radiostate) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IMediaRadioManagerNotifySink>, ::windows::core::GetTrustLevel, OnInstanceAdd::<Impl, OFFSET>, OnInstanceRemove::<Impl, OFFSET>, OnInstanceRadioChange::<Impl, OFFSET>)
    }
}
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
impl ::windows::core::RuntimeName for IPortableDevice {
    const NAME: &'static str = "Windows.Win32.Devices.PortableDevices.IPortableDevice";
}
impl IPortableDeviceVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPortableDeviceImpl, const OFFSET: isize>() -> IPortableDeviceVtbl {
        unsafe extern "system" fn Open<Impl: IPortableDeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszpnpdeviceid: super::super::Foundation::PWSTR, pclientinfo: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Open(&*(&pszpnpdeviceid as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), &*(&pclientinfo as *const <IPortableDeviceValues as ::windows::core::Abi>::Abi as *const <IPortableDeviceValues as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SendCommand<Impl: IPortableDeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwflags: u32, pparameters: ::windows::core::RawPtr, ppresults: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SendCommand(dwflags, &*(&pparameters as *const <IPortableDeviceValues as ::windows::core::Abi>::Abi as *const <IPortableDeviceValues as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&ppresults)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Content<Impl: IPortableDeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppcontent: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Content(::core::mem::transmute_copy(&ppcontent)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Capabilities<Impl: IPortableDeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppcapabilities: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Capabilities(::core::mem::transmute_copy(&ppcapabilities)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Cancel<Impl: IPortableDeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Cancel() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Close<Impl: IPortableDeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Close() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Advise<Impl: IPortableDeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwflags: u32, pcallback: ::windows::core::RawPtr, pparameters: ::windows::core::RawPtr, ppszcookie: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Advise(dwflags, &*(&pcallback as *const <IPortableDeviceEventCallback as ::windows::core::Abi>::Abi as *const <IPortableDeviceEventCallback as ::windows::core::DefaultType>::DefaultType), &*(&pparameters as *const <IPortableDeviceValues as ::windows::core::Abi>::Abi as *const <IPortableDeviceValues as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&ppszcookie)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Unadvise<Impl: IPortableDeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszcookie: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Unadvise(&*(&pszcookie as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPnPDeviceID<Impl: IPortableDeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppszpnpdeviceid: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetPnPDeviceID(::core::mem::transmute_copy(&ppszpnpdeviceid)) {
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
            ::windows::core::GetRuntimeClassName::<IPortableDevice>,
            ::windows::core::GetTrustLevel,
            Open::<Impl, OFFSET>,
            SendCommand::<Impl, OFFSET>,
            Content::<Impl, OFFSET>,
            Capabilities::<Impl, OFFSET>,
            Cancel::<Impl, OFFSET>,
            Close::<Impl, OFFSET>,
            Advise::<Impl, OFFSET>,
            Unadvise::<Impl, OFFSET>,
            GetPnPDeviceID::<Impl, OFFSET>,
        )
    }
}
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
impl ::windows::core::RuntimeName for IPortableDeviceCapabilities {
    const NAME: &'static str = "Windows.Win32.Devices.PortableDevices.IPortableDeviceCapabilities";
}
impl IPortableDeviceCapabilitiesVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPortableDeviceCapabilitiesImpl, const OFFSET: isize>() -> IPortableDeviceCapabilitiesVtbl {
        unsafe extern "system" fn GetSupportedCommands<Impl: IPortableDeviceCapabilitiesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppcommands: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetSupportedCommands(::core::mem::transmute_copy(&ppcommands)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCommandOptions<Impl: IPortableDeviceCapabilitiesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, command: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, ppoptions: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetCommandOptions(&*(&command as *const <super::super::UI::Shell::PropertiesSystem::PROPERTYKEY as ::windows::core::Abi>::Abi as *const <super::super::UI::Shell::PropertiesSystem::PROPERTYKEY as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&ppoptions)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFunctionalCategories<Impl: IPortableDeviceCapabilitiesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppcategories: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetFunctionalCategories(::core::mem::transmute_copy(&ppcategories)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFunctionalObjects<Impl: IPortableDeviceCapabilitiesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, category: &::windows::core::GUID, ppobjectids: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetFunctionalObjects(&*(&category as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&ppobjectids)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSupportedContentTypes<Impl: IPortableDeviceCapabilitiesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, category: &::windows::core::GUID, ppcontenttypes: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetSupportedContentTypes(&*(&category as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&ppcontenttypes)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSupportedFormats<Impl: IPortableDeviceCapabilitiesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, contenttype: &::windows::core::GUID, ppformats: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetSupportedFormats(&*(&contenttype as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&ppformats)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSupportedFormatProperties<Impl: IPortableDeviceCapabilitiesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, format: &::windows::core::GUID, ppkeys: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetSupportedFormatProperties(&*(&format as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&ppkeys)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFixedPropertyAttributes<Impl: IPortableDeviceCapabilitiesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, format: &::windows::core::GUID, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, ppattributes: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetFixedPropertyAttributes(&*(&format as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), &*(&key as *const <super::super::UI::Shell::PropertiesSystem::PROPERTYKEY as ::windows::core::Abi>::Abi as *const <super::super::UI::Shell::PropertiesSystem::PROPERTYKEY as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&ppattributes)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Cancel<Impl: IPortableDeviceCapabilitiesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Cancel() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSupportedEvents<Impl: IPortableDeviceCapabilitiesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppevents: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetSupportedEvents(::core::mem::transmute_copy(&ppevents)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetEventOptions<Impl: IPortableDeviceCapabilitiesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, event: &::windows::core::GUID, ppoptions: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetEventOptions(&*(&event as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&ppoptions)) {
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
            ::windows::core::GetRuntimeClassName::<IPortableDeviceCapabilities>,
            ::windows::core::GetTrustLevel,
            GetSupportedCommands::<Impl, OFFSET>,
            GetCommandOptions::<Impl, OFFSET>,
            GetFunctionalCategories::<Impl, OFFSET>,
            GetFunctionalObjects::<Impl, OFFSET>,
            GetSupportedContentTypes::<Impl, OFFSET>,
            GetSupportedFormats::<Impl, OFFSET>,
            GetSupportedFormatProperties::<Impl, OFFSET>,
            GetFixedPropertyAttributes::<Impl, OFFSET>,
            Cancel::<Impl, OFFSET>,
            GetSupportedEvents::<Impl, OFFSET>,
            GetEventOptions::<Impl, OFFSET>,
        )
    }
}
pub trait IPortableDeviceConnectorImpl: Sized {
    fn Connect();
    fn Disconnect();
    fn Cancel();
    fn GetProperty();
    fn SetProperty();
    fn GetPnPID();
}
impl ::windows::core::RuntimeName for IPortableDeviceConnector {
    const NAME: &'static str = "Windows.Win32.Devices.PortableDevices.IPortableDeviceConnector";
}
impl IPortableDeviceConnectorVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPortableDeviceConnectorImpl, const OFFSET: isize>() -> IPortableDeviceConnectorVtbl {
        unsafe extern "system" fn Connect<Impl: IPortableDeviceConnectorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcallback: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Connect(&*(&pcallback as *const <IConnectionRequestCallback as ::windows::core::Abi>::Abi as *const <IConnectionRequestCallback as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Disconnect<Impl: IPortableDeviceConnectorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcallback: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Disconnect(&*(&pcallback as *const <IConnectionRequestCallback as ::windows::core::Abi>::Abi as *const <IConnectionRequestCallback as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Cancel<Impl: IPortableDeviceConnectorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcallback: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Cancel(&*(&pcallback as *const <IConnectionRequestCallback as ::windows::core::Abi>::Abi as *const <IConnectionRequestCallback as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetProperty<Impl: IPortableDeviceConnectorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppropertykey: *const super::Properties::DEVPROPKEY, ppropertytype: *mut u32, ppdata: *mut *mut u8, pcbdata: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetProperty(&*(&ppropertykey as *const <super::Properties::DEVPROPKEY as ::windows::core::Abi>::Abi as *const <super::Properties::DEVPROPKEY as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&ppropertytype), ::core::mem::transmute_copy(&ppdata), ::core::mem::transmute_copy(&pcbdata)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetProperty<Impl: IPortableDeviceConnectorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppropertykey: *const super::Properties::DEVPROPKEY, propertytype: u32, pdata: *const u8, cbdata: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetProperty(&*(&ppropertykey as *const <super::Properties::DEVPROPKEY as ::windows::core::Abi>::Abi as *const <super::Properties::DEVPROPKEY as ::windows::core::DefaultType>::DefaultType), propertytype, pdata, cbdata) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPnPID<Impl: IPortableDeviceConnectorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppwszpnpid: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetPnPID(::core::mem::transmute_copy(&ppwszpnpid)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IPortableDeviceConnector>, ::windows::core::GetTrustLevel, Connect::<Impl, OFFSET>, Disconnect::<Impl, OFFSET>, Cancel::<Impl, OFFSET>, GetProperty::<Impl, OFFSET>, SetProperty::<Impl, OFFSET>, GetPnPID::<Impl, OFFSET>)
    }
}
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
impl ::windows::core::RuntimeName for IPortableDeviceContent {
    const NAME: &'static str = "Windows.Win32.Devices.PortableDevices.IPortableDeviceContent";
}
impl IPortableDeviceContentVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPortableDeviceContentImpl, const OFFSET: isize>() -> IPortableDeviceContentVtbl {
        unsafe extern "system" fn EnumObjects<Impl: IPortableDeviceContentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwflags: u32, pszparentobjectid: super::super::Foundation::PWSTR, pfilter: ::windows::core::RawPtr, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EnumObjects(dwflags, &*(&pszparentobjectid as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), &*(&pfilter as *const <IPortableDeviceValues as ::windows::core::Abi>::Abi as *const <IPortableDeviceValues as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&ppenum)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Properties<Impl: IPortableDeviceContentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppproperties: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Properties(::core::mem::transmute_copy(&ppproperties)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Transfer<Impl: IPortableDeviceContentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppresources: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Transfer(::core::mem::transmute_copy(&ppresources)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateObjectWithPropertiesOnly<Impl: IPortableDeviceContentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalues: ::windows::core::RawPtr, ppszobjectid: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateObjectWithPropertiesOnly(&*(&pvalues as *const <IPortableDeviceValues as ::windows::core::Abi>::Abi as *const <IPortableDeviceValues as ::windows::core::DefaultType>::DefaultType), &*(&ppszobjectid as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateObjectWithPropertiesAndData<Impl: IPortableDeviceContentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalues: ::windows::core::RawPtr, ppdata: *mut ::windows::core::RawPtr, pdwoptimalwritebuffersize: *mut u32, ppszcookie: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateObjectWithPropertiesAndData(&*(&pvalues as *const <IPortableDeviceValues as ::windows::core::Abi>::Abi as *const <IPortableDeviceValues as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&ppdata), pdwoptimalwritebuffersize, &*(&ppszcookie as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Delete<Impl: IPortableDeviceContentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwoptions: u32, pobjectids: ::windows::core::RawPtr, ppresults: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Delete(dwoptions, &*(&pobjectids as *const <IPortableDevicePropVariantCollection as ::windows::core::Abi>::Abi as *const <IPortableDevicePropVariantCollection as ::windows::core::DefaultType>::DefaultType), &*(&ppresults as *const <IPortableDevicePropVariantCollection as ::windows::core::Abi>::Abi as *const <IPortableDevicePropVariantCollection as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetObjectIDsFromPersistentUniqueIDs<Impl: IPortableDeviceContentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppersistentuniqueids: ::windows::core::RawPtr, ppobjectids: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetObjectIDsFromPersistentUniqueIDs(&*(&ppersistentuniqueids as *const <IPortableDevicePropVariantCollection as ::windows::core::Abi>::Abi as *const <IPortableDevicePropVariantCollection as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&ppobjectids)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Cancel<Impl: IPortableDeviceContentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Cancel() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Move<Impl: IPortableDeviceContentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pobjectids: ::windows::core::RawPtr, pszdestinationfolderobjectid: super::super::Foundation::PWSTR, ppresults: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Move(
                &*(&pobjectids as *const <IPortableDevicePropVariantCollection as ::windows::core::Abi>::Abi as *const <IPortableDevicePropVariantCollection as ::windows::core::DefaultType>::DefaultType),
                &*(&pszdestinationfolderobjectid as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&ppresults as *const <IPortableDevicePropVariantCollection as ::windows::core::Abi>::Abi as *const <IPortableDevicePropVariantCollection as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Copy<Impl: IPortableDeviceContentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pobjectids: ::windows::core::RawPtr, pszdestinationfolderobjectid: super::super::Foundation::PWSTR, ppresults: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Copy(
                &*(&pobjectids as *const <IPortableDevicePropVariantCollection as ::windows::core::Abi>::Abi as *const <IPortableDevicePropVariantCollection as ::windows::core::DefaultType>::DefaultType),
                &*(&pszdestinationfolderobjectid as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&ppresults as *const <IPortableDevicePropVariantCollection as ::windows::core::Abi>::Abi as *const <IPortableDevicePropVariantCollection as ::windows::core::DefaultType>::DefaultType),
            ) {
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
            ::windows::core::GetRuntimeClassName::<IPortableDeviceContent>,
            ::windows::core::GetTrustLevel,
            EnumObjects::<Impl, OFFSET>,
            Properties::<Impl, OFFSET>,
            Transfer::<Impl, OFFSET>,
            CreateObjectWithPropertiesOnly::<Impl, OFFSET>,
            CreateObjectWithPropertiesAndData::<Impl, OFFSET>,
            Delete::<Impl, OFFSET>,
            GetObjectIDsFromPersistentUniqueIDs::<Impl, OFFSET>,
            Cancel::<Impl, OFFSET>,
            Move::<Impl, OFFSET>,
            Copy::<Impl, OFFSET>,
        )
    }
}
pub trait IPortableDeviceContent2Impl: Sized + IPortableDeviceContentImpl {
    fn UpdateObjectWithPropertiesAndData();
}
impl ::windows::core::RuntimeName for IPortableDeviceContent2 {
    const NAME: &'static str = "Windows.Win32.Devices.PortableDevices.IPortableDeviceContent2";
}
impl IPortableDeviceContent2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPortableDeviceContent2Impl, const OFFSET: isize>() -> IPortableDeviceContent2Vtbl {
        unsafe extern "system" fn UpdateObjectWithPropertiesAndData<Impl: IPortableDeviceContent2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszobjectid: super::super::Foundation::PWSTR, pproperties: ::windows::core::RawPtr, ppdata: *mut ::windows::core::RawPtr, pdwoptimalwritebuffersize: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).UpdateObjectWithPropertiesAndData(&*(&pszobjectid as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), &*(&pproperties as *const <IPortableDeviceValues as ::windows::core::Abi>::Abi as *const <IPortableDeviceValues as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&ppdata), pdwoptimalwritebuffersize) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IPortableDeviceContent2>, ::windows::core::GetTrustLevel, UpdateObjectWithPropertiesAndData::<Impl, OFFSET>)
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IPortableDeviceDataStreamImpl: Sized + IStreamImpl + ISequentialStreamImpl {
    fn GetObjectID();
    fn Cancel();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for IPortableDeviceDataStream {
    const NAME: &'static str = "Windows.Win32.Devices.PortableDevices.IPortableDeviceDataStream";
}
#[cfg(feature = "Win32_System_Com")]
impl IPortableDeviceDataStreamVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPortableDeviceDataStreamImpl, const OFFSET: isize>() -> IPortableDeviceDataStreamVtbl {
        unsafe extern "system" fn GetObjectID<Impl: IPortableDeviceDataStreamImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppszobjectid: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetObjectID(::core::mem::transmute_copy(&ppszobjectid)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Cancel<Impl: IPortableDeviceDataStreamImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Cancel() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IPortableDeviceDataStream>, ::windows::core::GetTrustLevel, GetObjectID::<Impl, OFFSET>, Cancel::<Impl, OFFSET>)
    }
}
pub trait IPortableDeviceDispatchFactoryImpl: Sized {
    fn GetDeviceDispatch();
}
impl ::windows::core::RuntimeName for IPortableDeviceDispatchFactory {
    const NAME: &'static str = "Windows.Win32.Devices.PortableDevices.IPortableDeviceDispatchFactory";
}
impl IPortableDeviceDispatchFactoryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPortableDeviceDispatchFactoryImpl, const OFFSET: isize>() -> IPortableDeviceDispatchFactoryVtbl {
        unsafe extern "system" fn GetDeviceDispatch<Impl: IPortableDeviceDispatchFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszpnpdeviceid: super::super::Foundation::PWSTR, ppdevicedispatch: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDeviceDispatch(&*(&pszpnpdeviceid as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&ppdevicedispatch)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IPortableDeviceDispatchFactory>, ::windows::core::GetTrustLevel, GetDeviceDispatch::<Impl, OFFSET>)
    }
}
pub trait IPortableDeviceEventCallbackImpl: Sized {
    fn OnEvent();
}
impl ::windows::core::RuntimeName for IPortableDeviceEventCallback {
    const NAME: &'static str = "Windows.Win32.Devices.PortableDevices.IPortableDeviceEventCallback";
}
impl IPortableDeviceEventCallbackVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPortableDeviceEventCallbackImpl, const OFFSET: isize>() -> IPortableDeviceEventCallbackVtbl {
        unsafe extern "system" fn OnEvent<Impl: IPortableDeviceEventCallbackImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, peventparameters: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OnEvent(&*(&peventparameters as *const <IPortableDeviceValues as ::windows::core::Abi>::Abi as *const <IPortableDeviceValues as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IPortableDeviceEventCallback>, ::windows::core::GetTrustLevel, OnEvent::<Impl, OFFSET>)
    }
}
pub trait IPortableDeviceKeyCollectionImpl: Sized {
    fn GetCount();
    fn GetAt();
    fn Add();
    fn Clear();
    fn RemoveAt();
}
impl ::windows::core::RuntimeName for IPortableDeviceKeyCollection {
    const NAME: &'static str = "Windows.Win32.Devices.PortableDevices.IPortableDeviceKeyCollection";
}
impl IPortableDeviceKeyCollectionVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPortableDeviceKeyCollectionImpl, const OFFSET: isize>() -> IPortableDeviceKeyCollectionVtbl {
        unsafe extern "system" fn GetCount<Impl: IPortableDeviceKeyCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcelems: *const u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetCount(pcelems) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetAt<Impl: IPortableDeviceKeyCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwindex: u32, pkey: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetAt(dwindex, &*(&pkey as *const <super::super::UI::Shell::PropertiesSystem::PROPERTYKEY as ::windows::core::Abi>::Abi as *const <super::super::UI::Shell::PropertiesSystem::PROPERTYKEY as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Add<Impl: IPortableDeviceKeyCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Add(&*(&key as *const <super::super::UI::Shell::PropertiesSystem::PROPERTYKEY as ::windows::core::Abi>::Abi as *const <super::super::UI::Shell::PropertiesSystem::PROPERTYKEY as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Clear<Impl: IPortableDeviceKeyCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Clear() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveAt<Impl: IPortableDeviceKeyCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwindex: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RemoveAt(dwindex) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IPortableDeviceKeyCollection>, ::windows::core::GetTrustLevel, GetCount::<Impl, OFFSET>, GetAt::<Impl, OFFSET>, Add::<Impl, OFFSET>, Clear::<Impl, OFFSET>, RemoveAt::<Impl, OFFSET>)
    }
}
pub trait IPortableDeviceManagerImpl: Sized {
    fn GetDevices();
    fn RefreshDeviceList();
    fn GetDeviceFriendlyName();
    fn GetDeviceDescription();
    fn GetDeviceManufacturer();
    fn GetDeviceProperty();
    fn GetPrivateDevices();
}
impl ::windows::core::RuntimeName for IPortableDeviceManager {
    const NAME: &'static str = "Windows.Win32.Devices.PortableDevices.IPortableDeviceManager";
}
impl IPortableDeviceManagerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPortableDeviceManagerImpl, const OFFSET: isize>() -> IPortableDeviceManagerVtbl {
        unsafe extern "system" fn GetDevices<Impl: IPortableDeviceManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppnpdeviceids: *mut super::super::Foundation::PWSTR, pcpnpdeviceids: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDevices(&*(&ppnpdeviceids as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), pcpnpdeviceids) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RefreshDeviceList<Impl: IPortableDeviceManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RefreshDeviceList() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDeviceFriendlyName<Impl: IPortableDeviceManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszpnpdeviceid: super::super::Foundation::PWSTR, pdevicefriendlyname: super::super::Foundation::PWSTR, pcchdevicefriendlyname: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDeviceFriendlyName(&*(&pszpnpdeviceid as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), &*(&pdevicefriendlyname as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), pcchdevicefriendlyname) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDeviceDescription<Impl: IPortableDeviceManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszpnpdeviceid: super::super::Foundation::PWSTR, pdevicedescription: super::super::Foundation::PWSTR, pcchdevicedescription: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDeviceDescription(&*(&pszpnpdeviceid as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), &*(&pdevicedescription as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), pcchdevicedescription) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDeviceManufacturer<Impl: IPortableDeviceManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszpnpdeviceid: super::super::Foundation::PWSTR, pdevicemanufacturer: super::super::Foundation::PWSTR, pcchdevicemanufacturer: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDeviceManufacturer(&*(&pszpnpdeviceid as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), &*(&pdevicemanufacturer as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), pcchdevicemanufacturer) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDeviceProperty<Impl: IPortableDeviceManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszpnpdeviceid: super::super::Foundation::PWSTR, pszdevicepropertyname: super::super::Foundation::PWSTR, pdata: *mut u8, pcbdata: *mut u32, pdwtype: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDeviceProperty(&*(&pszpnpdeviceid as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), &*(&pszdevicepropertyname as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), pdata, pcbdata, pdwtype) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPrivateDevices<Impl: IPortableDeviceManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppnpdeviceids: *mut super::super::Foundation::PWSTR, pcpnpdeviceids: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetPrivateDevices(&*(&ppnpdeviceids as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), pcpnpdeviceids) {
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
            ::windows::core::GetRuntimeClassName::<IPortableDeviceManager>,
            ::windows::core::GetTrustLevel,
            GetDevices::<Impl, OFFSET>,
            RefreshDeviceList::<Impl, OFFSET>,
            GetDeviceFriendlyName::<Impl, OFFSET>,
            GetDeviceDescription::<Impl, OFFSET>,
            GetDeviceManufacturer::<Impl, OFFSET>,
            GetDeviceProperty::<Impl, OFFSET>,
            GetPrivateDevices::<Impl, OFFSET>,
        )
    }
}
pub trait IPortableDevicePropVariantCollectionImpl: Sized {
    fn GetCount();
    fn GetAt();
    fn Add();
    fn GetType();
    fn ChangeType();
    fn Clear();
    fn RemoveAt();
}
impl ::windows::core::RuntimeName for IPortableDevicePropVariantCollection {
    const NAME: &'static str = "Windows.Win32.Devices.PortableDevices.IPortableDevicePropVariantCollection";
}
impl IPortableDevicePropVariantCollectionVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPortableDevicePropVariantCollectionImpl, const OFFSET: isize>() -> IPortableDevicePropVariantCollectionVtbl {
        unsafe extern "system" fn GetCount<Impl: IPortableDevicePropVariantCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcelems: *const u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetCount(pcelems) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetAt<Impl: IPortableDevicePropVariantCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwindex: u32, pvalue: *const super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetAt(dwindex, &*(&pvalue as *const <super::super::System::Com::StructuredStorage::PROPVARIANT as ::windows::core::Abi>::Abi as *const <super::super::System::Com::StructuredStorage::PROPVARIANT as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Add<Impl: IPortableDevicePropVariantCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *const super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Add(&*(&pvalue as *const <super::super::System::Com::StructuredStorage::PROPVARIANT as ::windows::core::Abi>::Abi as *const <super::super::System::Com::StructuredStorage::PROPVARIANT as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetType<Impl: IPortableDevicePropVariantCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvt: *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetType(::core::mem::transmute_copy(&pvt)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ChangeType<Impl: IPortableDevicePropVariantCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, vt: u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ChangeType(vt) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Clear<Impl: IPortableDevicePropVariantCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Clear() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveAt<Impl: IPortableDevicePropVariantCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwindex: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RemoveAt(dwindex) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IPortableDevicePropVariantCollection>, ::windows::core::GetTrustLevel, GetCount::<Impl, OFFSET>, GetAt::<Impl, OFFSET>, Add::<Impl, OFFSET>, GetType::<Impl, OFFSET>, ChangeType::<Impl, OFFSET>, Clear::<Impl, OFFSET>, RemoveAt::<Impl, OFFSET>)
    }
}
pub trait IPortableDevicePropertiesImpl: Sized {
    fn GetSupportedProperties();
    fn GetPropertyAttributes();
    fn GetValues();
    fn SetValues();
    fn Delete();
    fn Cancel();
}
impl ::windows::core::RuntimeName for IPortableDeviceProperties {
    const NAME: &'static str = "Windows.Win32.Devices.PortableDevices.IPortableDeviceProperties";
}
impl IPortableDevicePropertiesVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPortableDevicePropertiesImpl, const OFFSET: isize>() -> IPortableDevicePropertiesVtbl {
        unsafe extern "system" fn GetSupportedProperties<Impl: IPortableDevicePropertiesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszobjectid: super::super::Foundation::PWSTR, ppkeys: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetSupportedProperties(&*(&pszobjectid as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&ppkeys)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPropertyAttributes<Impl: IPortableDevicePropertiesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszobjectid: super::super::Foundation::PWSTR, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, ppattributes: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetPropertyAttributes(&*(&pszobjectid as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), &*(&key as *const <super::super::UI::Shell::PropertiesSystem::PROPERTYKEY as ::windows::core::Abi>::Abi as *const <super::super::UI::Shell::PropertiesSystem::PROPERTYKEY as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&ppattributes)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetValues<Impl: IPortableDevicePropertiesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszobjectid: super::super::Foundation::PWSTR, pkeys: ::windows::core::RawPtr, ppvalues: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetValues(&*(&pszobjectid as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), &*(&pkeys as *const <IPortableDeviceKeyCollection as ::windows::core::Abi>::Abi as *const <IPortableDeviceKeyCollection as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&ppvalues)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetValues<Impl: IPortableDevicePropertiesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszobjectid: super::super::Foundation::PWSTR, pvalues: ::windows::core::RawPtr, ppresults: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetValues(&*(&pszobjectid as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), &*(&pvalues as *const <IPortableDeviceValues as ::windows::core::Abi>::Abi as *const <IPortableDeviceValues as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&ppresults)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Delete<Impl: IPortableDevicePropertiesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszobjectid: super::super::Foundation::PWSTR, pkeys: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Delete(&*(&pszobjectid as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), &*(&pkeys as *const <IPortableDeviceKeyCollection as ::windows::core::Abi>::Abi as *const <IPortableDeviceKeyCollection as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Cancel<Impl: IPortableDevicePropertiesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Cancel() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IPortableDeviceProperties>, ::windows::core::GetTrustLevel, GetSupportedProperties::<Impl, OFFSET>, GetPropertyAttributes::<Impl, OFFSET>, GetValues::<Impl, OFFSET>, SetValues::<Impl, OFFSET>, Delete::<Impl, OFFSET>, Cancel::<Impl, OFFSET>)
    }
}
pub trait IPortableDevicePropertiesBulkImpl: Sized {
    fn QueueGetValuesByObjectList();
    fn QueueGetValuesByObjectFormat();
    fn QueueSetValuesByObjectList();
    fn Start();
    fn Cancel();
}
impl ::windows::core::RuntimeName for IPortableDevicePropertiesBulk {
    const NAME: &'static str = "Windows.Win32.Devices.PortableDevices.IPortableDevicePropertiesBulk";
}
impl IPortableDevicePropertiesBulkVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPortableDevicePropertiesBulkImpl, const OFFSET: isize>() -> IPortableDevicePropertiesBulkVtbl {
        unsafe extern "system" fn QueueGetValuesByObjectList<Impl: IPortableDevicePropertiesBulkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pobjectids: ::windows::core::RawPtr, pkeys: ::windows::core::RawPtr, pcallback: ::windows::core::RawPtr, pcontext: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).QueueGetValuesByObjectList(
                &*(&pobjectids as *const <IPortableDevicePropVariantCollection as ::windows::core::Abi>::Abi as *const <IPortableDevicePropVariantCollection as ::windows::core::DefaultType>::DefaultType),
                &*(&pkeys as *const <IPortableDeviceKeyCollection as ::windows::core::Abi>::Abi as *const <IPortableDeviceKeyCollection as ::windows::core::DefaultType>::DefaultType),
                &*(&pcallback as *const <IPortableDevicePropertiesBulkCallback as ::windows::core::Abi>::Abi as *const <IPortableDevicePropertiesBulkCallback as ::windows::core::DefaultType>::DefaultType),
                ::core::mem::transmute_copy(&pcontext),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn QueueGetValuesByObjectFormat<Impl: IPortableDevicePropertiesBulkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pguidobjectformat: &::windows::core::GUID, pszparentobjectid: super::super::Foundation::PWSTR, dwdepth: u32, pkeys: ::windows::core::RawPtr, pcallback: ::windows::core::RawPtr, pcontext: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).QueueGetValuesByObjectFormat(
                &*(&pguidobjectformat as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType),
                &*(&pszparentobjectid as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType),
                dwdepth,
                &*(&pkeys as *const <IPortableDeviceKeyCollection as ::windows::core::Abi>::Abi as *const <IPortableDeviceKeyCollection as ::windows::core::DefaultType>::DefaultType),
                &*(&pcallback as *const <IPortableDevicePropertiesBulkCallback as ::windows::core::Abi>::Abi as *const <IPortableDevicePropertiesBulkCallback as ::windows::core::DefaultType>::DefaultType),
                ::core::mem::transmute_copy(&pcontext),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn QueueSetValuesByObjectList<Impl: IPortableDevicePropertiesBulkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pobjectvalues: ::windows::core::RawPtr, pcallback: ::windows::core::RawPtr, pcontext: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).QueueSetValuesByObjectList(&*(&pobjectvalues as *const <IPortableDeviceValuesCollection as ::windows::core::Abi>::Abi as *const <IPortableDeviceValuesCollection as ::windows::core::DefaultType>::DefaultType), &*(&pcallback as *const <IPortableDevicePropertiesBulkCallback as ::windows::core::Abi>::Abi as *const <IPortableDevicePropertiesBulkCallback as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&pcontext)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Start<Impl: IPortableDevicePropertiesBulkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcontext: &::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Start(&*(&pcontext as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Cancel<Impl: IPortableDevicePropertiesBulkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcontext: &::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Cancel(&*(&pcontext as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IPortableDevicePropertiesBulk>, ::windows::core::GetTrustLevel, QueueGetValuesByObjectList::<Impl, OFFSET>, QueueGetValuesByObjectFormat::<Impl, OFFSET>, QueueSetValuesByObjectList::<Impl, OFFSET>, Start::<Impl, OFFSET>, Cancel::<Impl, OFFSET>)
    }
}
pub trait IPortableDevicePropertiesBulkCallbackImpl: Sized {
    fn OnStart();
    fn OnProgress();
    fn OnEnd();
}
impl ::windows::core::RuntimeName for IPortableDevicePropertiesBulkCallback {
    const NAME: &'static str = "Windows.Win32.Devices.PortableDevices.IPortableDevicePropertiesBulkCallback";
}
impl IPortableDevicePropertiesBulkCallbackVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPortableDevicePropertiesBulkCallbackImpl, const OFFSET: isize>() -> IPortableDevicePropertiesBulkCallbackVtbl {
        unsafe extern "system" fn OnStart<Impl: IPortableDevicePropertiesBulkCallbackImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcontext: &::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OnStart(&*(&pcontext as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OnProgress<Impl: IPortableDevicePropertiesBulkCallbackImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcontext: &::windows::core::GUID, presults: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OnProgress(&*(&pcontext as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), &*(&presults as *const <IPortableDeviceValuesCollection as ::windows::core::Abi>::Abi as *const <IPortableDeviceValuesCollection as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OnEnd<Impl: IPortableDevicePropertiesBulkCallbackImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcontext: &::windows::core::GUID, hrstatus: ::windows::core::HRESULT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OnEnd(&*(&pcontext as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), hrstatus) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IPortableDevicePropertiesBulkCallback>, ::windows::core::GetTrustLevel, OnStart::<Impl, OFFSET>, OnProgress::<Impl, OFFSET>, OnEnd::<Impl, OFFSET>)
    }
}
pub trait IPortableDeviceResourcesImpl: Sized {
    fn GetSupportedResources();
    fn GetResourceAttributes();
    fn GetStream();
    fn Delete();
    fn Cancel();
    fn CreateResource();
}
impl ::windows::core::RuntimeName for IPortableDeviceResources {
    const NAME: &'static str = "Windows.Win32.Devices.PortableDevices.IPortableDeviceResources";
}
impl IPortableDeviceResourcesVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPortableDeviceResourcesImpl, const OFFSET: isize>() -> IPortableDeviceResourcesVtbl {
        unsafe extern "system" fn GetSupportedResources<Impl: IPortableDeviceResourcesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszobjectid: super::super::Foundation::PWSTR, ppkeys: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetSupportedResources(&*(&pszobjectid as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&ppkeys)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetResourceAttributes<Impl: IPortableDeviceResourcesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszobjectid: super::super::Foundation::PWSTR, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, ppresourceattributes: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetResourceAttributes(&*(&pszobjectid as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), &*(&key as *const <super::super::UI::Shell::PropertiesSystem::PROPERTYKEY as ::windows::core::Abi>::Abi as *const <super::super::UI::Shell::PropertiesSystem::PROPERTYKEY as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&ppresourceattributes)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetStream<Impl: IPortableDeviceResourcesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszobjectid: super::super::Foundation::PWSTR, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, dwmode: u32, pdwoptimalbuffersize: *mut u32, ppstream: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetStream(&*(&pszobjectid as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), &*(&key as *const <super::super::UI::Shell::PropertiesSystem::PROPERTYKEY as ::windows::core::Abi>::Abi as *const <super::super::UI::Shell::PropertiesSystem::PROPERTYKEY as ::windows::core::DefaultType>::DefaultType), dwmode, pdwoptimalbuffersize, ::core::mem::transmute_copy(&ppstream)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Delete<Impl: IPortableDeviceResourcesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszobjectid: super::super::Foundation::PWSTR, pkeys: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Delete(&*(&pszobjectid as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), &*(&pkeys as *const <IPortableDeviceKeyCollection as ::windows::core::Abi>::Abi as *const <IPortableDeviceKeyCollection as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Cancel<Impl: IPortableDeviceResourcesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Cancel() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateResource<Impl: IPortableDeviceResourcesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, presourceattributes: ::windows::core::RawPtr, ppdata: *mut ::windows::core::RawPtr, pdwoptimalwritebuffersize: *mut u32, ppszcookie: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateResource(&*(&presourceattributes as *const <IPortableDeviceValues as ::windows::core::Abi>::Abi as *const <IPortableDeviceValues as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&ppdata), pdwoptimalwritebuffersize, &*(&ppszcookie as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IPortableDeviceResources>, ::windows::core::GetTrustLevel, GetSupportedResources::<Impl, OFFSET>, GetResourceAttributes::<Impl, OFFSET>, GetStream::<Impl, OFFSET>, Delete::<Impl, OFFSET>, Cancel::<Impl, OFFSET>, CreateResource::<Impl, OFFSET>)
    }
}
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
impl ::windows::core::RuntimeName for IPortableDeviceService {
    const NAME: &'static str = "Windows.Win32.Devices.PortableDevices.IPortableDeviceService";
}
impl IPortableDeviceServiceVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPortableDeviceServiceImpl, const OFFSET: isize>() -> IPortableDeviceServiceVtbl {
        unsafe extern "system" fn Open<Impl: IPortableDeviceServiceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszpnpserviceid: super::super::Foundation::PWSTR, pclientinfo: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Open(&*(&pszpnpserviceid as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), &*(&pclientinfo as *const <IPortableDeviceValues as ::windows::core::Abi>::Abi as *const <IPortableDeviceValues as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Capabilities<Impl: IPortableDeviceServiceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppcapabilities: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Capabilities(::core::mem::transmute_copy(&ppcapabilities)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Content<Impl: IPortableDeviceServiceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppcontent: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Content(::core::mem::transmute_copy(&ppcontent)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Methods<Impl: IPortableDeviceServiceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppmethods: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Methods(::core::mem::transmute_copy(&ppmethods)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Cancel<Impl: IPortableDeviceServiceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Cancel() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Close<Impl: IPortableDeviceServiceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Close() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetServiceObjectID<Impl: IPortableDeviceServiceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppszserviceobjectid: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetServiceObjectID(::core::mem::transmute_copy(&ppszserviceobjectid)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPnPServiceID<Impl: IPortableDeviceServiceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppszpnpserviceid: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetPnPServiceID(::core::mem::transmute_copy(&ppszpnpserviceid)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Advise<Impl: IPortableDeviceServiceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwflags: u32, pcallback: ::windows::core::RawPtr, pparameters: ::windows::core::RawPtr, ppszcookie: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Advise(dwflags, &*(&pcallback as *const <IPortableDeviceEventCallback as ::windows::core::Abi>::Abi as *const <IPortableDeviceEventCallback as ::windows::core::DefaultType>::DefaultType), &*(&pparameters as *const <IPortableDeviceValues as ::windows::core::Abi>::Abi as *const <IPortableDeviceValues as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&ppszcookie)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Unadvise<Impl: IPortableDeviceServiceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszcookie: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Unadvise(&*(&pszcookie as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SendCommand<Impl: IPortableDeviceServiceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwflags: u32, pparameters: ::windows::core::RawPtr, ppresults: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SendCommand(dwflags, &*(&pparameters as *const <IPortableDeviceValues as ::windows::core::Abi>::Abi as *const <IPortableDeviceValues as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&ppresults)) {
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
            ::windows::core::GetRuntimeClassName::<IPortableDeviceService>,
            ::windows::core::GetTrustLevel,
            Open::<Impl, OFFSET>,
            Capabilities::<Impl, OFFSET>,
            Content::<Impl, OFFSET>,
            Methods::<Impl, OFFSET>,
            Cancel::<Impl, OFFSET>,
            Close::<Impl, OFFSET>,
            GetServiceObjectID::<Impl, OFFSET>,
            GetPnPServiceID::<Impl, OFFSET>,
            Advise::<Impl, OFFSET>,
            Unadvise::<Impl, OFFSET>,
            SendCommand::<Impl, OFFSET>,
        )
    }
}
pub trait IPortableDeviceServiceActivationImpl: Sized {
    fn OpenAsync();
    fn CancelOpenAsync();
}
impl ::windows::core::RuntimeName for IPortableDeviceServiceActivation {
    const NAME: &'static str = "Windows.Win32.Devices.PortableDevices.IPortableDeviceServiceActivation";
}
impl IPortableDeviceServiceActivationVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPortableDeviceServiceActivationImpl, const OFFSET: isize>() -> IPortableDeviceServiceActivationVtbl {
        unsafe extern "system" fn OpenAsync<Impl: IPortableDeviceServiceActivationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszpnpserviceid: super::super::Foundation::PWSTR, pclientinfo: ::windows::core::RawPtr, pcallback: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OpenAsync(
                &*(&pszpnpserviceid as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&pclientinfo as *const <IPortableDeviceValues as ::windows::core::Abi>::Abi as *const <IPortableDeviceValues as ::windows::core::DefaultType>::DefaultType),
                &*(&pcallback as *const <IPortableDeviceServiceOpenCallback as ::windows::core::Abi>::Abi as *const <IPortableDeviceServiceOpenCallback as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CancelOpenAsync<Impl: IPortableDeviceServiceActivationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CancelOpenAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IPortableDeviceServiceActivation>, ::windows::core::GetTrustLevel, OpenAsync::<Impl, OFFSET>, CancelOpenAsync::<Impl, OFFSET>)
    }
}
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
impl ::windows::core::RuntimeName for IPortableDeviceServiceCapabilities {
    const NAME: &'static str = "Windows.Win32.Devices.PortableDevices.IPortableDeviceServiceCapabilities";
}
impl IPortableDeviceServiceCapabilitiesVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPortableDeviceServiceCapabilitiesImpl, const OFFSET: isize>() -> IPortableDeviceServiceCapabilitiesVtbl {
        unsafe extern "system" fn GetSupportedMethods<Impl: IPortableDeviceServiceCapabilitiesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppmethods: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetSupportedMethods(::core::mem::transmute_copy(&ppmethods)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSupportedMethodsByFormat<Impl: IPortableDeviceServiceCapabilitiesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, format: &::windows::core::GUID, ppmethods: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetSupportedMethodsByFormat(&*(&format as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&ppmethods)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetMethodAttributes<Impl: IPortableDeviceServiceCapabilitiesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, method: &::windows::core::GUID, ppattributes: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetMethodAttributes(&*(&method as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&ppattributes)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetMethodParameterAttributes<Impl: IPortableDeviceServiceCapabilitiesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, method: &::windows::core::GUID, parameter: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, ppattributes: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetMethodParameterAttributes(&*(&method as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), &*(&parameter as *const <super::super::UI::Shell::PropertiesSystem::PROPERTYKEY as ::windows::core::Abi>::Abi as *const <super::super::UI::Shell::PropertiesSystem::PROPERTYKEY as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&ppattributes)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSupportedFormats<Impl: IPortableDeviceServiceCapabilitiesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppformats: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetSupportedFormats(::core::mem::transmute_copy(&ppformats)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFormatAttributes<Impl: IPortableDeviceServiceCapabilitiesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, format: &::windows::core::GUID, ppattributes: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetFormatAttributes(&*(&format as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&ppattributes)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSupportedFormatProperties<Impl: IPortableDeviceServiceCapabilitiesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, format: &::windows::core::GUID, ppkeys: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetSupportedFormatProperties(&*(&format as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&ppkeys)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFormatPropertyAttributes<Impl: IPortableDeviceServiceCapabilitiesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, format: &::windows::core::GUID, property: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, ppattributes: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetFormatPropertyAttributes(&*(&format as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), &*(&property as *const <super::super::UI::Shell::PropertiesSystem::PROPERTYKEY as ::windows::core::Abi>::Abi as *const <super::super::UI::Shell::PropertiesSystem::PROPERTYKEY as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&ppattributes)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSupportedEvents<Impl: IPortableDeviceServiceCapabilitiesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppevents: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetSupportedEvents(::core::mem::transmute_copy(&ppevents)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetEventAttributes<Impl: IPortableDeviceServiceCapabilitiesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, event: &::windows::core::GUID, ppattributes: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetEventAttributes(&*(&event as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&ppattributes)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetEventParameterAttributes<Impl: IPortableDeviceServiceCapabilitiesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, event: &::windows::core::GUID, parameter: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, ppattributes: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetEventParameterAttributes(&*(&event as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), &*(&parameter as *const <super::super::UI::Shell::PropertiesSystem::PROPERTYKEY as ::windows::core::Abi>::Abi as *const <super::super::UI::Shell::PropertiesSystem::PROPERTYKEY as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&ppattributes)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetInheritedServices<Impl: IPortableDeviceServiceCapabilitiesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwinheritancetype: u32, ppservices: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetInheritedServices(dwinheritancetype, ::core::mem::transmute_copy(&ppservices)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFormatRenderingProfiles<Impl: IPortableDeviceServiceCapabilitiesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, format: &::windows::core::GUID, pprenderingprofiles: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetFormatRenderingProfiles(&*(&format as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&pprenderingprofiles)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSupportedCommands<Impl: IPortableDeviceServiceCapabilitiesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppcommands: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetSupportedCommands(::core::mem::transmute_copy(&ppcommands)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCommandOptions<Impl: IPortableDeviceServiceCapabilitiesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, command: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, ppoptions: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetCommandOptions(&*(&command as *const <super::super::UI::Shell::PropertiesSystem::PROPERTYKEY as ::windows::core::Abi>::Abi as *const <super::super::UI::Shell::PropertiesSystem::PROPERTYKEY as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&ppoptions)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Cancel<Impl: IPortableDeviceServiceCapabilitiesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Cancel() {
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
            ::windows::core::GetRuntimeClassName::<IPortableDeviceServiceCapabilities>,
            ::windows::core::GetTrustLevel,
            GetSupportedMethods::<Impl, OFFSET>,
            GetSupportedMethodsByFormat::<Impl, OFFSET>,
            GetMethodAttributes::<Impl, OFFSET>,
            GetMethodParameterAttributes::<Impl, OFFSET>,
            GetSupportedFormats::<Impl, OFFSET>,
            GetFormatAttributes::<Impl, OFFSET>,
            GetSupportedFormatProperties::<Impl, OFFSET>,
            GetFormatPropertyAttributes::<Impl, OFFSET>,
            GetSupportedEvents::<Impl, OFFSET>,
            GetEventAttributes::<Impl, OFFSET>,
            GetEventParameterAttributes::<Impl, OFFSET>,
            GetInheritedServices::<Impl, OFFSET>,
            GetFormatRenderingProfiles::<Impl, OFFSET>,
            GetSupportedCommands::<Impl, OFFSET>,
            GetCommandOptions::<Impl, OFFSET>,
            Cancel::<Impl, OFFSET>,
        )
    }
}
pub trait IPortableDeviceServiceManagerImpl: Sized {
    fn GetDeviceServices();
    fn GetDeviceForService();
}
impl ::windows::core::RuntimeName for IPortableDeviceServiceManager {
    const NAME: &'static str = "Windows.Win32.Devices.PortableDevices.IPortableDeviceServiceManager";
}
impl IPortableDeviceServiceManagerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPortableDeviceServiceManagerImpl, const OFFSET: isize>() -> IPortableDeviceServiceManagerVtbl {
        unsafe extern "system" fn GetDeviceServices<Impl: IPortableDeviceServiceManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszpnpdeviceid: super::super::Foundation::PWSTR, guidservicecategory: &::windows::core::GUID, pservices: *mut super::super::Foundation::PWSTR, pcservices: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDeviceServices(
                &*(&pszpnpdeviceid as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&guidservicecategory as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType),
                &*(&pservices as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType),
                pcservices,
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDeviceForService<Impl: IPortableDeviceServiceManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszpnpserviceid: super::super::Foundation::PWSTR, ppszpnpdeviceid: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDeviceForService(&*(&pszpnpserviceid as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&ppszpnpdeviceid)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IPortableDeviceServiceManager>, ::windows::core::GetTrustLevel, GetDeviceServices::<Impl, OFFSET>, GetDeviceForService::<Impl, OFFSET>)
    }
}
pub trait IPortableDeviceServiceMethodCallbackImpl: Sized {
    fn OnComplete();
}
impl ::windows::core::RuntimeName for IPortableDeviceServiceMethodCallback {
    const NAME: &'static str = "Windows.Win32.Devices.PortableDevices.IPortableDeviceServiceMethodCallback";
}
impl IPortableDeviceServiceMethodCallbackVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPortableDeviceServiceMethodCallbackImpl, const OFFSET: isize>() -> IPortableDeviceServiceMethodCallbackVtbl {
        unsafe extern "system" fn OnComplete<Impl: IPortableDeviceServiceMethodCallbackImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hrstatus: ::windows::core::HRESULT, presults: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OnComplete(hrstatus, &*(&presults as *const <IPortableDeviceValues as ::windows::core::Abi>::Abi as *const <IPortableDeviceValues as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IPortableDeviceServiceMethodCallback>, ::windows::core::GetTrustLevel, OnComplete::<Impl, OFFSET>)
    }
}
pub trait IPortableDeviceServiceMethodsImpl: Sized {
    fn Invoke();
    fn InvokeAsync();
    fn Cancel();
}
impl ::windows::core::RuntimeName for IPortableDeviceServiceMethods {
    const NAME: &'static str = "Windows.Win32.Devices.PortableDevices.IPortableDeviceServiceMethods";
}
impl IPortableDeviceServiceMethodsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPortableDeviceServiceMethodsImpl, const OFFSET: isize>() -> IPortableDeviceServiceMethodsVtbl {
        unsafe extern "system" fn Invoke<Impl: IPortableDeviceServiceMethodsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, method: &::windows::core::GUID, pparameters: ::windows::core::RawPtr, ppresults: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Invoke(&*(&method as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), &*(&pparameters as *const <IPortableDeviceValues as ::windows::core::Abi>::Abi as *const <IPortableDeviceValues as ::windows::core::DefaultType>::DefaultType), &*(&ppresults as *const <IPortableDeviceValues as ::windows::core::Abi>::Abi as *const <IPortableDeviceValues as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn InvokeAsync<Impl: IPortableDeviceServiceMethodsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, method: &::windows::core::GUID, pparameters: ::windows::core::RawPtr, pcallback: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).InvokeAsync(
                &*(&method as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType),
                &*(&pparameters as *const <IPortableDeviceValues as ::windows::core::Abi>::Abi as *const <IPortableDeviceValues as ::windows::core::DefaultType>::DefaultType),
                &*(&pcallback as *const <IPortableDeviceServiceMethodCallback as ::windows::core::Abi>::Abi as *const <IPortableDeviceServiceMethodCallback as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Cancel<Impl: IPortableDeviceServiceMethodsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcallback: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Cancel(&*(&pcallback as *const <IPortableDeviceServiceMethodCallback as ::windows::core::Abi>::Abi as *const <IPortableDeviceServiceMethodCallback as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IPortableDeviceServiceMethods>, ::windows::core::GetTrustLevel, Invoke::<Impl, OFFSET>, InvokeAsync::<Impl, OFFSET>, Cancel::<Impl, OFFSET>)
    }
}
pub trait IPortableDeviceServiceOpenCallbackImpl: Sized {
    fn OnComplete();
}
impl ::windows::core::RuntimeName for IPortableDeviceServiceOpenCallback {
    const NAME: &'static str = "Windows.Win32.Devices.PortableDevices.IPortableDeviceServiceOpenCallback";
}
impl IPortableDeviceServiceOpenCallbackVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPortableDeviceServiceOpenCallbackImpl, const OFFSET: isize>() -> IPortableDeviceServiceOpenCallbackVtbl {
        unsafe extern "system" fn OnComplete<Impl: IPortableDeviceServiceOpenCallbackImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hrstatus: ::windows::core::HRESULT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OnComplete(hrstatus) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IPortableDeviceServiceOpenCallback>, ::windows::core::GetTrustLevel, OnComplete::<Impl, OFFSET>)
    }
}
pub trait IPortableDeviceUnitsStreamImpl: Sized {
    fn SeekInUnits();
    fn Cancel();
}
impl ::windows::core::RuntimeName for IPortableDeviceUnitsStream {
    const NAME: &'static str = "Windows.Win32.Devices.PortableDevices.IPortableDeviceUnitsStream";
}
impl IPortableDeviceUnitsStreamVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPortableDeviceUnitsStreamImpl, const OFFSET: isize>() -> IPortableDeviceUnitsStreamVtbl {
        unsafe extern "system" fn SeekInUnits<Impl: IPortableDeviceUnitsStreamImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dlibmove: i64, units: WPD_STREAM_UNITS, dworigin: u32, plibnewposition: *mut u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SeekInUnits(dlibmove, units, dworigin, ::core::mem::transmute_copy(&plibnewposition)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Cancel<Impl: IPortableDeviceUnitsStreamImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Cancel() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IPortableDeviceUnitsStream>, ::windows::core::GetTrustLevel, SeekInUnits::<Impl, OFFSET>, Cancel::<Impl, OFFSET>)
    }
}
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
impl ::windows::core::RuntimeName for IPortableDeviceValues {
    const NAME: &'static str = "Windows.Win32.Devices.PortableDevices.IPortableDeviceValues";
}
impl IPortableDeviceValuesVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPortableDeviceValuesImpl, const OFFSET: isize>() -> IPortableDeviceValuesVtbl {
        unsafe extern "system" fn GetCount<Impl: IPortableDeviceValuesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcelt: *const u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetCount(pcelt) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetAt<Impl: IPortableDeviceValuesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32, pkey: *mut super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, pvalue: *mut super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetAt(index, &*(&pkey as *const <super::super::UI::Shell::PropertiesSystem::PROPERTYKEY as ::windows::core::Abi>::Abi as *const <super::super::UI::Shell::PropertiesSystem::PROPERTYKEY as ::windows::core::DefaultType>::DefaultType), &*(&pvalue as *const <super::super::System::Com::StructuredStorage::PROPVARIANT as ::windows::core::Abi>::Abi as *const <super::super::System::Com::StructuredStorage::PROPVARIANT as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetValue<Impl: IPortableDeviceValuesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, pvalue: *const super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetValue(&*(&key as *const <super::super::UI::Shell::PropertiesSystem::PROPERTYKEY as ::windows::core::Abi>::Abi as *const <super::super::UI::Shell::PropertiesSystem::PROPERTYKEY as ::windows::core::DefaultType>::DefaultType), &*(&pvalue as *const <super::super::System::Com::StructuredStorage::PROPVARIANT as ::windows::core::Abi>::Abi as *const <super::super::System::Com::StructuredStorage::PROPVARIANT as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetValue<Impl: IPortableDeviceValuesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, pvalue: *mut super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetValue(&*(&key as *const <super::super::UI::Shell::PropertiesSystem::PROPERTYKEY as ::windows::core::Abi>::Abi as *const <super::super::UI::Shell::PropertiesSystem::PROPERTYKEY as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&pvalue)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetStringValue<Impl: IPortableDeviceValuesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, value: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetStringValue(&*(&key as *const <super::super::UI::Shell::PropertiesSystem::PROPERTYKEY as ::windows::core::Abi>::Abi as *const <super::super::UI::Shell::PropertiesSystem::PROPERTYKEY as ::windows::core::DefaultType>::DefaultType), &*(&value as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetStringValue<Impl: IPortableDeviceValuesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, pvalue: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetStringValue(&*(&key as *const <super::super::UI::Shell::PropertiesSystem::PROPERTYKEY as ::windows::core::Abi>::Abi as *const <super::super::UI::Shell::PropertiesSystem::PROPERTYKEY as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&pvalue)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetUnsignedIntegerValue<Impl: IPortableDeviceValuesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, value: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetUnsignedIntegerValue(&*(&key as *const <super::super::UI::Shell::PropertiesSystem::PROPERTYKEY as ::windows::core::Abi>::Abi as *const <super::super::UI::Shell::PropertiesSystem::PROPERTYKEY as ::windows::core::DefaultType>::DefaultType), value) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetUnsignedIntegerValue<Impl: IPortableDeviceValuesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, pvalue: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetUnsignedIntegerValue(&*(&key as *const <super::super::UI::Shell::PropertiesSystem::PROPERTYKEY as ::windows::core::Abi>::Abi as *const <super::super::UI::Shell::PropertiesSystem::PROPERTYKEY as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&pvalue)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSignedIntegerValue<Impl: IPortableDeviceValuesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, value: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetSignedIntegerValue(&*(&key as *const <super::super::UI::Shell::PropertiesSystem::PROPERTYKEY as ::windows::core::Abi>::Abi as *const <super::super::UI::Shell::PropertiesSystem::PROPERTYKEY as ::windows::core::DefaultType>::DefaultType), value) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSignedIntegerValue<Impl: IPortableDeviceValuesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, pvalue: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetSignedIntegerValue(&*(&key as *const <super::super::UI::Shell::PropertiesSystem::PROPERTYKEY as ::windows::core::Abi>::Abi as *const <super::super::UI::Shell::PropertiesSystem::PROPERTYKEY as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&pvalue)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetUnsignedLargeIntegerValue<Impl: IPortableDeviceValuesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, value: u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetUnsignedLargeIntegerValue(&*(&key as *const <super::super::UI::Shell::PropertiesSystem::PROPERTYKEY as ::windows::core::Abi>::Abi as *const <super::super::UI::Shell::PropertiesSystem::PROPERTYKEY as ::windows::core::DefaultType>::DefaultType), value) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetUnsignedLargeIntegerValue<Impl: IPortableDeviceValuesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, pvalue: *mut u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetUnsignedLargeIntegerValue(&*(&key as *const <super::super::UI::Shell::PropertiesSystem::PROPERTYKEY as ::windows::core::Abi>::Abi as *const <super::super::UI::Shell::PropertiesSystem::PROPERTYKEY as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&pvalue)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSignedLargeIntegerValue<Impl: IPortableDeviceValuesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, value: i64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetSignedLargeIntegerValue(&*(&key as *const <super::super::UI::Shell::PropertiesSystem::PROPERTYKEY as ::windows::core::Abi>::Abi as *const <super::super::UI::Shell::PropertiesSystem::PROPERTYKEY as ::windows::core::DefaultType>::DefaultType), value) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSignedLargeIntegerValue<Impl: IPortableDeviceValuesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, pvalue: *mut i64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetSignedLargeIntegerValue(&*(&key as *const <super::super::UI::Shell::PropertiesSystem::PROPERTYKEY as ::windows::core::Abi>::Abi as *const <super::super::UI::Shell::PropertiesSystem::PROPERTYKEY as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&pvalue)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetFloatValue<Impl: IPortableDeviceValuesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, value: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetFloatValue(&*(&key as *const <super::super::UI::Shell::PropertiesSystem::PROPERTYKEY as ::windows::core::Abi>::Abi as *const <super::super::UI::Shell::PropertiesSystem::PROPERTYKEY as ::windows::core::DefaultType>::DefaultType), value) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFloatValue<Impl: IPortableDeviceValuesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, pvalue: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetFloatValue(&*(&key as *const <super::super::UI::Shell::PropertiesSystem::PROPERTYKEY as ::windows::core::Abi>::Abi as *const <super::super::UI::Shell::PropertiesSystem::PROPERTYKEY as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&pvalue)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetErrorValue<Impl: IPortableDeviceValuesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, value: ::windows::core::HRESULT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetErrorValue(&*(&key as *const <super::super::UI::Shell::PropertiesSystem::PROPERTYKEY as ::windows::core::Abi>::Abi as *const <super::super::UI::Shell::PropertiesSystem::PROPERTYKEY as ::windows::core::DefaultType>::DefaultType), value) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetErrorValue<Impl: IPortableDeviceValuesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, pvalue: *mut ::windows::core::HRESULT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetErrorValue(&*(&key as *const <super::super::UI::Shell::PropertiesSystem::PROPERTYKEY as ::windows::core::Abi>::Abi as *const <super::super::UI::Shell::PropertiesSystem::PROPERTYKEY as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&pvalue)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetKeyValue<Impl: IPortableDeviceValuesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, value: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetKeyValue(&*(&key as *const <super::super::UI::Shell::PropertiesSystem::PROPERTYKEY as ::windows::core::Abi>::Abi as *const <super::super::UI::Shell::PropertiesSystem::PROPERTYKEY as ::windows::core::DefaultType>::DefaultType), &*(&value as *const <super::super::UI::Shell::PropertiesSystem::PROPERTYKEY as ::windows::core::Abi>::Abi as *const <super::super::UI::Shell::PropertiesSystem::PROPERTYKEY as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetKeyValue<Impl: IPortableDeviceValuesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, pvalue: *mut super::super::UI::Shell::PropertiesSystem::PROPERTYKEY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetKeyValue(&*(&key as *const <super::super::UI::Shell::PropertiesSystem::PROPERTYKEY as ::windows::core::Abi>::Abi as *const <super::super::UI::Shell::PropertiesSystem::PROPERTYKEY as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&pvalue)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetBoolValue<Impl: IPortableDeviceValuesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, value: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetBoolValue(&*(&key as *const <super::super::UI::Shell::PropertiesSystem::PROPERTYKEY as ::windows::core::Abi>::Abi as *const <super::super::UI::Shell::PropertiesSystem::PROPERTYKEY as ::windows::core::DefaultType>::DefaultType), &*(&value as *const <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetBoolValue<Impl: IPortableDeviceValuesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, pvalue: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetBoolValue(&*(&key as *const <super::super::UI::Shell::PropertiesSystem::PROPERTYKEY as ::windows::core::Abi>::Abi as *const <super::super::UI::Shell::PropertiesSystem::PROPERTYKEY as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&pvalue)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIUnknownValue<Impl: IPortableDeviceValuesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, pvalue: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetIUnknownValue(&*(&key as *const <super::super::UI::Shell::PropertiesSystem::PROPERTYKEY as ::windows::core::Abi>::Abi as *const <super::super::UI::Shell::PropertiesSystem::PROPERTYKEY as ::windows::core::DefaultType>::DefaultType), &*(&pvalue as *const <::windows::core::IUnknown as ::windows::core::Abi>::Abi as *const <::windows::core::IUnknown as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetIUnknownValue<Impl: IPortableDeviceValuesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, ppvalue: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetIUnknownValue(&*(&key as *const <super::super::UI::Shell::PropertiesSystem::PROPERTYKEY as ::windows::core::Abi>::Abi as *const <super::super::UI::Shell::PropertiesSystem::PROPERTYKEY as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&ppvalue)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetGuidValue<Impl: IPortableDeviceValuesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, value: &::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetGuidValue(&*(&key as *const <super::super::UI::Shell::PropertiesSystem::PROPERTYKEY as ::windows::core::Abi>::Abi as *const <super::super::UI::Shell::PropertiesSystem::PROPERTYKEY as ::windows::core::DefaultType>::DefaultType), &*(&value as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetGuidValue<Impl: IPortableDeviceValuesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, pvalue: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetGuidValue(&*(&key as *const <super::super::UI::Shell::PropertiesSystem::PROPERTYKEY as ::windows::core::Abi>::Abi as *const <super::super::UI::Shell::PropertiesSystem::PROPERTYKEY as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&pvalue)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetBufferValue<Impl: IPortableDeviceValuesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, pvalue: *const u8, cbvalue: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetBufferValue(&*(&key as *const <super::super::UI::Shell::PropertiesSystem::PROPERTYKEY as ::windows::core::Abi>::Abi as *const <super::super::UI::Shell::PropertiesSystem::PROPERTYKEY as ::windows::core::DefaultType>::DefaultType), pvalue, cbvalue) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetBufferValue<Impl: IPortableDeviceValuesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, ppvalue: *mut *mut u8, pcbvalue: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetBufferValue(&*(&key as *const <super::super::UI::Shell::PropertiesSystem::PROPERTYKEY as ::windows::core::Abi>::Abi as *const <super::super::UI::Shell::PropertiesSystem::PROPERTYKEY as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&ppvalue), ::core::mem::transmute_copy(&pcbvalue)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIPortableDeviceValuesValue<Impl: IPortableDeviceValuesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, pvalue: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetIPortableDeviceValuesValue(&*(&key as *const <super::super::UI::Shell::PropertiesSystem::PROPERTYKEY as ::windows::core::Abi>::Abi as *const <super::super::UI::Shell::PropertiesSystem::PROPERTYKEY as ::windows::core::DefaultType>::DefaultType), &*(&pvalue as *const <IPortableDeviceValues as ::windows::core::Abi>::Abi as *const <IPortableDeviceValues as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetIPortableDeviceValuesValue<Impl: IPortableDeviceValuesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, ppvalue: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetIPortableDeviceValuesValue(&*(&key as *const <super::super::UI::Shell::PropertiesSystem::PROPERTYKEY as ::windows::core::Abi>::Abi as *const <super::super::UI::Shell::PropertiesSystem::PROPERTYKEY as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&ppvalue)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIPortableDevicePropVariantCollectionValue<Impl: IPortableDeviceValuesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, pvalue: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetIPortableDevicePropVariantCollectionValue(&*(&key as *const <super::super::UI::Shell::PropertiesSystem::PROPERTYKEY as ::windows::core::Abi>::Abi as *const <super::super::UI::Shell::PropertiesSystem::PROPERTYKEY as ::windows::core::DefaultType>::DefaultType), &*(&pvalue as *const <IPortableDevicePropVariantCollection as ::windows::core::Abi>::Abi as *const <IPortableDevicePropVariantCollection as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetIPortableDevicePropVariantCollectionValue<Impl: IPortableDeviceValuesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, ppvalue: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetIPortableDevicePropVariantCollectionValue(&*(&key as *const <super::super::UI::Shell::PropertiesSystem::PROPERTYKEY as ::windows::core::Abi>::Abi as *const <super::super::UI::Shell::PropertiesSystem::PROPERTYKEY as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&ppvalue)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIPortableDeviceKeyCollectionValue<Impl: IPortableDeviceValuesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, pvalue: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetIPortableDeviceKeyCollectionValue(&*(&key as *const <super::super::UI::Shell::PropertiesSystem::PROPERTYKEY as ::windows::core::Abi>::Abi as *const <super::super::UI::Shell::PropertiesSystem::PROPERTYKEY as ::windows::core::DefaultType>::DefaultType), &*(&pvalue as *const <IPortableDeviceKeyCollection as ::windows::core::Abi>::Abi as *const <IPortableDeviceKeyCollection as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetIPortableDeviceKeyCollectionValue<Impl: IPortableDeviceValuesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, ppvalue: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetIPortableDeviceKeyCollectionValue(&*(&key as *const <super::super::UI::Shell::PropertiesSystem::PROPERTYKEY as ::windows::core::Abi>::Abi as *const <super::super::UI::Shell::PropertiesSystem::PROPERTYKEY as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&ppvalue)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIPortableDeviceValuesCollectionValue<Impl: IPortableDeviceValuesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, pvalue: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetIPortableDeviceValuesCollectionValue(&*(&key as *const <super::super::UI::Shell::PropertiesSystem::PROPERTYKEY as ::windows::core::Abi>::Abi as *const <super::super::UI::Shell::PropertiesSystem::PROPERTYKEY as ::windows::core::DefaultType>::DefaultType), &*(&pvalue as *const <IPortableDeviceValuesCollection as ::windows::core::Abi>::Abi as *const <IPortableDeviceValuesCollection as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetIPortableDeviceValuesCollectionValue<Impl: IPortableDeviceValuesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, ppvalue: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetIPortableDeviceValuesCollectionValue(&*(&key as *const <super::super::UI::Shell::PropertiesSystem::PROPERTYKEY as ::windows::core::Abi>::Abi as *const <super::super::UI::Shell::PropertiesSystem::PROPERTYKEY as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&ppvalue)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveValue<Impl: IPortableDeviceValuesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RemoveValue(&*(&key as *const <super::super::UI::Shell::PropertiesSystem::PROPERTYKEY as ::windows::core::Abi>::Abi as *const <super::super::UI::Shell::PropertiesSystem::PROPERTYKEY as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CopyValuesFromPropertyStore<Impl: IPortableDeviceValuesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pstore: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CopyValuesFromPropertyStore(&*(&pstore as *const <super::super::UI::Shell::PropertiesSystem::IPropertyStore as ::windows::core::Abi>::Abi as *const <super::super::UI::Shell::PropertiesSystem::IPropertyStore as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CopyValuesToPropertyStore<Impl: IPortableDeviceValuesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pstore: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CopyValuesToPropertyStore(&*(&pstore as *const <super::super::UI::Shell::PropertiesSystem::IPropertyStore as ::windows::core::Abi>::Abi as *const <super::super::UI::Shell::PropertiesSystem::IPropertyStore as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Clear<Impl: IPortableDeviceValuesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Clear() {
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
            ::windows::core::GetRuntimeClassName::<IPortableDeviceValues>,
            ::windows::core::GetTrustLevel,
            GetCount::<Impl, OFFSET>,
            GetAt::<Impl, OFFSET>,
            SetValue::<Impl, OFFSET>,
            GetValue::<Impl, OFFSET>,
            SetStringValue::<Impl, OFFSET>,
            GetStringValue::<Impl, OFFSET>,
            SetUnsignedIntegerValue::<Impl, OFFSET>,
            GetUnsignedIntegerValue::<Impl, OFFSET>,
            SetSignedIntegerValue::<Impl, OFFSET>,
            GetSignedIntegerValue::<Impl, OFFSET>,
            SetUnsignedLargeIntegerValue::<Impl, OFFSET>,
            GetUnsignedLargeIntegerValue::<Impl, OFFSET>,
            SetSignedLargeIntegerValue::<Impl, OFFSET>,
            GetSignedLargeIntegerValue::<Impl, OFFSET>,
            SetFloatValue::<Impl, OFFSET>,
            GetFloatValue::<Impl, OFFSET>,
            SetErrorValue::<Impl, OFFSET>,
            GetErrorValue::<Impl, OFFSET>,
            SetKeyValue::<Impl, OFFSET>,
            GetKeyValue::<Impl, OFFSET>,
            SetBoolValue::<Impl, OFFSET>,
            GetBoolValue::<Impl, OFFSET>,
            SetIUnknownValue::<Impl, OFFSET>,
            GetIUnknownValue::<Impl, OFFSET>,
            SetGuidValue::<Impl, OFFSET>,
            GetGuidValue::<Impl, OFFSET>,
            SetBufferValue::<Impl, OFFSET>,
            GetBufferValue::<Impl, OFFSET>,
            SetIPortableDeviceValuesValue::<Impl, OFFSET>,
            GetIPortableDeviceValuesValue::<Impl, OFFSET>,
            SetIPortableDevicePropVariantCollectionValue::<Impl, OFFSET>,
            GetIPortableDevicePropVariantCollectionValue::<Impl, OFFSET>,
            SetIPortableDeviceKeyCollectionValue::<Impl, OFFSET>,
            GetIPortableDeviceKeyCollectionValue::<Impl, OFFSET>,
            SetIPortableDeviceValuesCollectionValue::<Impl, OFFSET>,
            GetIPortableDeviceValuesCollectionValue::<Impl, OFFSET>,
            RemoveValue::<Impl, OFFSET>,
            CopyValuesFromPropertyStore::<Impl, OFFSET>,
            CopyValuesToPropertyStore::<Impl, OFFSET>,
            Clear::<Impl, OFFSET>,
        )
    }
}
pub trait IPortableDeviceValuesCollectionImpl: Sized {
    fn GetCount();
    fn GetAt();
    fn Add();
    fn Clear();
    fn RemoveAt();
}
impl ::windows::core::RuntimeName for IPortableDeviceValuesCollection {
    const NAME: &'static str = "Windows.Win32.Devices.PortableDevices.IPortableDeviceValuesCollection";
}
impl IPortableDeviceValuesCollectionVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPortableDeviceValuesCollectionImpl, const OFFSET: isize>() -> IPortableDeviceValuesCollectionVtbl {
        unsafe extern "system" fn GetCount<Impl: IPortableDeviceValuesCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcelems: *const u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetCount(pcelems) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetAt<Impl: IPortableDeviceValuesCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwindex: u32, ppvalues: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetAt(dwindex, ::core::mem::transmute_copy(&ppvalues)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Add<Impl: IPortableDeviceValuesCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalues: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Add(&*(&pvalues as *const <IPortableDeviceValues as ::windows::core::Abi>::Abi as *const <IPortableDeviceValues as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Clear<Impl: IPortableDeviceValuesCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Clear() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveAt<Impl: IPortableDeviceValuesCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwindex: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RemoveAt(dwindex) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IPortableDeviceValuesCollection>, ::windows::core::GetTrustLevel, GetCount::<Impl, OFFSET>, GetAt::<Impl, OFFSET>, Add::<Impl, OFFSET>, Clear::<Impl, OFFSET>, RemoveAt::<Impl, OFFSET>)
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IPortableDeviceWebControlImpl: Sized + IDispatchImpl {
    fn GetDeviceFromId();
    fn GetDeviceFromIdAsync();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for IPortableDeviceWebControl {
    const NAME: &'static str = "Windows.Win32.Devices.PortableDevices.IPortableDeviceWebControl";
}
#[cfg(feature = "Win32_System_Com")]
impl IPortableDeviceWebControlVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPortableDeviceWebControlImpl, const OFFSET: isize>() -> IPortableDeviceWebControlVtbl {
        unsafe extern "system" fn GetDeviceFromId<Impl: IPortableDeviceWebControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, deviceid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ppdevice: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDeviceFromId(&*(&deviceid as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&ppdevice)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDeviceFromIdAsync<Impl: IPortableDeviceWebControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, deviceid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pcompletionhandler: ::windows::core::RawPtr, perrorhandler: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDeviceFromIdAsync(
                &*(&deviceid as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&pcompletionhandler as *const <super::super::System::Com::IDispatch as ::windows::core::Abi>::Abi as *const <super::super::System::Com::IDispatch as ::windows::core::DefaultType>::DefaultType),
                &*(&perrorhandler as *const <super::super::System::Com::IDispatch as ::windows::core::Abi>::Abi as *const <super::super::System::Com::IDispatch as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IPortableDeviceWebControl>, ::windows::core::GetTrustLevel, GetDeviceFromId::<Impl, OFFSET>, GetDeviceFromIdAsync::<Impl, OFFSET>)
    }
}
pub trait IRadioInstanceImpl: Sized {
    fn GetRadioManagerSignature();
    fn GetInstanceSignature();
    fn GetFriendlyName();
    fn GetRadioState();
    fn SetRadioState();
    fn IsMultiComm();
    fn IsAssociatingDevice();
}
impl ::windows::core::RuntimeName for IRadioInstance {
    const NAME: &'static str = "Windows.Win32.Devices.PortableDevices.IRadioInstance";
}
impl IRadioInstanceVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRadioInstanceImpl, const OFFSET: isize>() -> IRadioInstanceVtbl {
        unsafe extern "system" fn GetRadioManagerSignature<Impl: IRadioInstanceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pguidsignature: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetRadioManagerSignature(::core::mem::transmute_copy(&pguidsignature)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetInstanceSignature<Impl: IRadioInstanceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrid: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetInstanceSignature(::core::mem::transmute_copy(&pbstrid)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFriendlyName<Impl: IRadioInstanceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lcid: u32, pbstrname: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetFriendlyName(lcid, ::core::mem::transmute_copy(&pbstrname)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetRadioState<Impl: IRadioInstanceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pradiostate: *mut DEVICE_RADIO_STATE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetRadioState(::core::mem::transmute_copy(&pradiostate)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetRadioState<Impl: IRadioInstanceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, radiostate: DEVICE_RADIO_STATE, utimeoutsec: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetRadioState(radiostate, utimeoutsec) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsMultiComm<Impl: IRadioInstanceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsMultiComm() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsAssociatingDevice<Impl: IRadioInstanceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsAssociatingDevice() {
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
            ::windows::core::GetRuntimeClassName::<IRadioInstance>,
            ::windows::core::GetTrustLevel,
            GetRadioManagerSignature::<Impl, OFFSET>,
            GetInstanceSignature::<Impl, OFFSET>,
            GetFriendlyName::<Impl, OFFSET>,
            GetRadioState::<Impl, OFFSET>,
            SetRadioState::<Impl, OFFSET>,
            IsMultiComm::<Impl, OFFSET>,
            IsAssociatingDevice::<Impl, OFFSET>,
        )
    }
}
pub trait IRadioInstanceCollectionImpl: Sized {
    fn GetCount();
    fn GetAt();
}
impl ::windows::core::RuntimeName for IRadioInstanceCollection {
    const NAME: &'static str = "Windows.Win32.Devices.PortableDevices.IRadioInstanceCollection";
}
impl IRadioInstanceCollectionVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRadioInstanceCollectionImpl, const OFFSET: isize>() -> IRadioInstanceCollectionVtbl {
        unsafe extern "system" fn GetCount<Impl: IRadioInstanceCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcinstance: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetCount(::core::mem::transmute_copy(&pcinstance)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetAt<Impl: IRadioInstanceCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, uindex: u32, ppradioinstance: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetAt(uindex, ::core::mem::transmute_copy(&ppradioinstance)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IRadioInstanceCollection>, ::windows::core::GetTrustLevel, GetCount::<Impl, OFFSET>, GetAt::<Impl, OFFSET>)
    }
}
pub trait IWpdSerializerImpl: Sized {
    fn GetIPortableDeviceValuesFromBuffer();
    fn WriteIPortableDeviceValuesToBuffer();
    fn GetBufferFromIPortableDeviceValues();
    fn GetSerializedSize();
}
impl ::windows::core::RuntimeName for IWpdSerializer {
    const NAME: &'static str = "Windows.Win32.Devices.PortableDevices.IWpdSerializer";
}
impl IWpdSerializerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWpdSerializerImpl, const OFFSET: isize>() -> IWpdSerializerVtbl {
        unsafe extern "system" fn GetIPortableDeviceValuesFromBuffer<Impl: IWpdSerializerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbuffer: *const u8, dwinputbufferlength: u32, ppparams: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetIPortableDeviceValuesFromBuffer(pbuffer, dwinputbufferlength, ::core::mem::transmute_copy(&ppparams)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn WriteIPortableDeviceValuesToBuffer<Impl: IWpdSerializerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwoutputbufferlength: u32, presults: ::windows::core::RawPtr, pbuffer: *mut u8, pdwbyteswritten: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).WriteIPortableDeviceValuesToBuffer(dwoutputbufferlength, &*(&presults as *const <IPortableDeviceValues as ::windows::core::Abi>::Abi as *const <IPortableDeviceValues as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&pbuffer), ::core::mem::transmute_copy(&pdwbyteswritten)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetBufferFromIPortableDeviceValues<Impl: IWpdSerializerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psource: ::windows::core::RawPtr, ppbuffer: *mut *mut u8, pdwbuffersize: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetBufferFromIPortableDeviceValues(&*(&psource as *const <IPortableDeviceValues as ::windows::core::Abi>::Abi as *const <IPortableDeviceValues as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&ppbuffer), ::core::mem::transmute_copy(&pdwbuffersize)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSerializedSize<Impl: IWpdSerializerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psource: ::windows::core::RawPtr, pdwsize: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetSerializedSize(&*(&psource as *const <IPortableDeviceValues as ::windows::core::Abi>::Abi as *const <IPortableDeviceValues as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&pdwsize)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IWpdSerializer>, ::windows::core::GetTrustLevel, GetIPortableDeviceValuesFromBuffer::<Impl, OFFSET>, WriteIPortableDeviceValuesToBuffer::<Impl, OFFSET>, GetBufferFromIPortableDeviceValues::<Impl, OFFSET>, GetSerializedSize::<Impl, OFFSET>)
    }
}
