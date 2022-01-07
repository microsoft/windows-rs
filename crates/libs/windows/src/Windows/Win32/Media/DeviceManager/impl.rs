pub trait IComponentAuthenticateImpl: Sized {
    fn SACAuth();
    fn SACGetProtocols();
}
impl ::windows::core::RuntimeName for IComponentAuthenticate {
    const NAME: &'static str = "Windows.Win32.Media.DeviceManager.IComponentAuthenticate";
}
impl IComponentAuthenticateVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IComponentAuthenticateImpl, const OFFSET: isize>() -> IComponentAuthenticateVtbl {
        unsafe extern "system" fn SACAuth<Impl: IComponentAuthenticateImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwprotocolid: u32, dwpass: u32, pbdatain: *const u8, dwdatainlen: u32, ppbdataout: *mut *mut u8, pdwdataoutlen: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SACAuth(dwprotocolid, dwpass, pbdatain, dwdatainlen, ::core::mem::transmute_copy(&ppbdataout), ::core::mem::transmute_copy(&pdwdataoutlen)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SACGetProtocols<Impl: IComponentAuthenticateImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppdwprotocols: *mut *mut u32, pdwprotocolcount: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SACGetProtocols(::core::mem::transmute_copy(&ppdwprotocols), ::core::mem::transmute_copy(&pdwprotocolcount)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IComponentAuthenticate>, ::windows::core::GetTrustLevel, SACAuth::<Impl, OFFSET>, SACGetProtocols::<Impl, OFFSET>)
    }
}
pub trait IMDSPDeviceImpl: Sized {
    fn GetName();
    fn GetManufacturer();
    fn GetVersion();
    fn GetType();
    fn GetSerialNumber();
    fn GetPowerSource();
    fn GetStatus();
    fn GetDeviceIcon();
    fn EnumStorage();
    fn GetFormatSupport();
    fn SendOpaqueCommand();
}
impl ::windows::core::RuntimeName for IMDSPDevice {
    const NAME: &'static str = "Windows.Win32.Media.DeviceManager.IMDSPDevice";
}
impl IMDSPDeviceVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMDSPDeviceImpl, const OFFSET: isize>() -> IMDSPDeviceVtbl {
        unsafe extern "system" fn GetName<Impl: IMDSPDeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwszname: super::super::Foundation::PWSTR, nmaxchars: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetName(::core::mem::transmute_copy(&pwszname), nmaxchars) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetManufacturer<Impl: IMDSPDeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwszname: super::super::Foundation::PWSTR, nmaxchars: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetManufacturer(::core::mem::transmute_copy(&pwszname), nmaxchars) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetVersion<Impl: IMDSPDeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwversion: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetVersion(::core::mem::transmute_copy(&pdwversion)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetType<Impl: IMDSPDeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwtype: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetType(::core::mem::transmute_copy(&pdwtype)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSerialNumber<Impl: IMDSPDeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pserialnumber: *mut WMDMID, abmac: *mut u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetSerialNumber(::core::mem::transmute_copy(&pserialnumber), abmac) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPowerSource<Impl: IMDSPDeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwpowersource: *mut u32, pdwpercentremaining: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetPowerSource(::core::mem::transmute_copy(&pdwpowersource), ::core::mem::transmute_copy(&pdwpercentremaining)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetStatus<Impl: IMDSPDeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwstatus: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetStatus(::core::mem::transmute_copy(&pdwstatus)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDeviceIcon<Impl: IMDSPDeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hicon: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDeviceIcon(::core::mem::transmute_copy(&hicon)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnumStorage<Impl: IMDSPDeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenumstorage: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EnumStorage(::core::mem::transmute_copy(&ppenumstorage)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFormatSupport<Impl: IMDSPDeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pformatex: *mut *mut _WAVEFORMATEX, pnformatcount: *mut u32, pppwszmimetype: *mut *mut super::super::Foundation::PWSTR, pnmimetypecount: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetFormatSupport(::core::mem::transmute_copy(&pformatex), ::core::mem::transmute_copy(&pnformatcount), ::core::mem::transmute_copy(&pppwszmimetype), ::core::mem::transmute_copy(&pnmimetypecount)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SendOpaqueCommand<Impl: IMDSPDeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcommand: *mut OPAQUECOMMAND) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SendOpaqueCommand(&*(&pcommand as *const <OPAQUECOMMAND as ::windows::core::Abi>::Abi as *const <OPAQUECOMMAND as ::windows::core::DefaultType>::DefaultType)) {
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
            ::windows::core::GetRuntimeClassName::<IMDSPDevice>,
            ::windows::core::GetTrustLevel,
            GetName::<Impl, OFFSET>,
            GetManufacturer::<Impl, OFFSET>,
            GetVersion::<Impl, OFFSET>,
            GetType::<Impl, OFFSET>,
            GetSerialNumber::<Impl, OFFSET>,
            GetPowerSource::<Impl, OFFSET>,
            GetStatus::<Impl, OFFSET>,
            GetDeviceIcon::<Impl, OFFSET>,
            EnumStorage::<Impl, OFFSET>,
            GetFormatSupport::<Impl, OFFSET>,
            SendOpaqueCommand::<Impl, OFFSET>,
        )
    }
}
pub trait IMDSPDevice2Impl: Sized + IMDSPDeviceImpl {
    fn GetStorage();
    fn GetFormatSupport2();
    fn GetSpecifyPropertyPages();
    fn GetCanonicalName();
}
impl ::windows::core::RuntimeName for IMDSPDevice2 {
    const NAME: &'static str = "Windows.Win32.Media.DeviceManager.IMDSPDevice2";
}
impl IMDSPDevice2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMDSPDevice2Impl, const OFFSET: isize>() -> IMDSPDevice2Vtbl {
        unsafe extern "system" fn GetStorage<Impl: IMDSPDevice2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszstoragename: super::super::Foundation::PWSTR, ppstorage: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetStorage(&*(&pszstoragename as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&ppstorage)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFormatSupport2<Impl: IMDSPDevice2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwflags: u32, ppaudioformatex: *mut *mut _WAVEFORMATEX, pnaudioformatcount: *mut u32, ppvideoformatex: *mut *mut _VIDEOINFOHEADER, pnvideoformatcount: *mut u32, ppfiletype: *mut *mut WMFILECAPABILITIES, pnfiletypecount: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetFormatSupport2(dwflags, ::core::mem::transmute_copy(&ppaudioformatex), ::core::mem::transmute_copy(&pnaudioformatcount), ::core::mem::transmute_copy(&ppvideoformatex), ::core::mem::transmute_copy(&pnvideoformatcount), ::core::mem::transmute_copy(&ppfiletype), ::core::mem::transmute_copy(&pnfiletypecount)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSpecifyPropertyPages<Impl: IMDSPDevice2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppspecifyproppages: *mut ::windows::core::RawPtr, pppunknowns: *mut *mut *mut ::core::ffi::c_void, pcunks: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetSpecifyPropertyPages(::core::mem::transmute_copy(&ppspecifyproppages), ::core::mem::transmute_copy(&pppunknowns), ::core::mem::transmute_copy(&pcunks)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCanonicalName<Impl: IMDSPDevice2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwszpnpname: super::super::Foundation::PWSTR, nmaxchars: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetCanonicalName(::core::mem::transmute_copy(&pwszpnpname), nmaxchars) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IMDSPDevice2>, ::windows::core::GetTrustLevel, GetStorage::<Impl, OFFSET>, GetFormatSupport2::<Impl, OFFSET>, GetSpecifyPropertyPages::<Impl, OFFSET>, GetCanonicalName::<Impl, OFFSET>)
    }
}
pub trait IMDSPDevice3Impl: Sized + IMDSPDevice2Impl + IMDSPDeviceImpl {
    fn GetProperty();
    fn SetProperty();
    fn GetFormatCapability();
    fn DeviceIoControl();
    fn FindStorage();
}
impl ::windows::core::RuntimeName for IMDSPDevice3 {
    const NAME: &'static str = "Windows.Win32.Media.DeviceManager.IMDSPDevice3";
}
impl IMDSPDevice3Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMDSPDevice3Impl, const OFFSET: isize>() -> IMDSPDevice3Vtbl {
        unsafe extern "system" fn GetProperty<Impl: IMDSPDevice3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwszpropname: super::super::Foundation::PWSTR, pvalue: *mut super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetProperty(&*(&pwszpropname as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&pvalue)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetProperty<Impl: IMDSPDevice3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwszpropname: super::super::Foundation::PWSTR, pvalue: *const super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetProperty(&*(&pwszpropname as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), &*(&pvalue as *const <super::super::System::Com::StructuredStorage::PROPVARIANT as ::windows::core::Abi>::Abi as *const <super::super::System::Com::StructuredStorage::PROPVARIANT as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFormatCapability<Impl: IMDSPDevice3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, format: WMDM_FORMATCODE, pformatsupport: *mut WMDM_FORMAT_CAPABILITY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetFormatCapability(format, ::core::mem::transmute_copy(&pformatsupport)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DeviceIoControl<Impl: IMDSPDevice3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwiocontrolcode: u32, lpinbuffer: *const u8, ninbuffersize: u32, lpoutbuffer: *mut u8, pnoutbuffersize: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DeviceIoControl(dwiocontrolcode, lpinbuffer, ninbuffersize, ::core::mem::transmute_copy(&lpoutbuffer), pnoutbuffersize) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FindStorage<Impl: IMDSPDevice3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, findscope: WMDM_FIND_SCOPE, pwszuniqueid: super::super::Foundation::PWSTR, ppstorage: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FindStorage(findscope, &*(&pwszuniqueid as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&ppstorage)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IMDSPDevice3>, ::windows::core::GetTrustLevel, GetProperty::<Impl, OFFSET>, SetProperty::<Impl, OFFSET>, GetFormatCapability::<Impl, OFFSET>, DeviceIoControl::<Impl, OFFSET>, FindStorage::<Impl, OFFSET>)
    }
}
pub trait IMDSPDeviceControlImpl: Sized {
    fn GetDCStatus();
    fn GetCapabilities();
    fn Play();
    fn Record();
    fn Pause();
    fn Resume();
    fn Stop();
    fn Seek();
}
impl ::windows::core::RuntimeName for IMDSPDeviceControl {
    const NAME: &'static str = "Windows.Win32.Media.DeviceManager.IMDSPDeviceControl";
}
impl IMDSPDeviceControlVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMDSPDeviceControlImpl, const OFFSET: isize>() -> IMDSPDeviceControlVtbl {
        unsafe extern "system" fn GetDCStatus<Impl: IMDSPDeviceControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwstatus: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDCStatus(::core::mem::transmute_copy(&pdwstatus)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCapabilities<Impl: IMDSPDeviceControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwcapabilitiesmask: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetCapabilities(::core::mem::transmute_copy(&pdwcapabilitiesmask)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Play<Impl: IMDSPDeviceControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Play() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Record<Impl: IMDSPDeviceControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pformat: *const _WAVEFORMATEX) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Record(&*(&pformat as *const <_WAVEFORMATEX as ::windows::core::Abi>::Abi as *const <_WAVEFORMATEX as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Pause<Impl: IMDSPDeviceControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Pause() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Resume<Impl: IMDSPDeviceControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Resume() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Stop<Impl: IMDSPDeviceControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Stop() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Seek<Impl: IMDSPDeviceControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fumode: u32, noffset: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Seek(fumode, noffset) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IMDSPDeviceControl>, ::windows::core::GetTrustLevel, GetDCStatus::<Impl, OFFSET>, GetCapabilities::<Impl, OFFSET>, Play::<Impl, OFFSET>, Record::<Impl, OFFSET>, Pause::<Impl, OFFSET>, Resume::<Impl, OFFSET>, Stop::<Impl, OFFSET>, Seek::<Impl, OFFSET>)
    }
}
pub trait IMDSPDirectTransferImpl: Sized {
    fn TransferToDevice();
}
impl ::windows::core::RuntimeName for IMDSPDirectTransfer {
    const NAME: &'static str = "Windows.Win32.Media.DeviceManager.IMDSPDirectTransfer";
}
impl IMDSPDirectTransferVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMDSPDirectTransferImpl, const OFFSET: isize>() -> IMDSPDirectTransferVtbl {
        unsafe extern "system" fn TransferToDevice<Impl: IMDSPDirectTransferImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwszsourcefilepath: super::super::Foundation::PWSTR, psourceoperation: ::windows::core::RawPtr, fuflags: u32, pwszdestinationname: super::super::Foundation::PWSTR, psourcemetadata: ::windows::core::RawPtr, ptransferprogress: ::windows::core::RawPtr, ppnewobject: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TransferToDevice(
                &*(&pwszsourcefilepath as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&psourceoperation as *const <IWMDMOperation as ::windows::core::Abi>::Abi as *const <IWMDMOperation as ::windows::core::DefaultType>::DefaultType),
                fuflags,
                &*(&pwszdestinationname as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&psourcemetadata as *const <IWMDMMetaData as ::windows::core::Abi>::Abi as *const <IWMDMMetaData as ::windows::core::DefaultType>::DefaultType),
                &*(&ptransferprogress as *const <IWMDMProgress as ::windows::core::Abi>::Abi as *const <IWMDMProgress as ::windows::core::DefaultType>::DefaultType),
                ::core::mem::transmute_copy(&ppnewobject),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IMDSPDirectTransfer>, ::windows::core::GetTrustLevel, TransferToDevice::<Impl, OFFSET>)
    }
}
pub trait IMDSPEnumDeviceImpl: Sized {
    fn Next();
    fn Skip();
    fn Reset();
    fn Clone();
}
impl ::windows::core::RuntimeName for IMDSPEnumDevice {
    const NAME: &'static str = "Windows.Win32.Media.DeviceManager.IMDSPEnumDevice";
}
impl IMDSPEnumDeviceVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMDSPEnumDeviceImpl, const OFFSET: isize>() -> IMDSPEnumDeviceVtbl {
        unsafe extern "system" fn Next<Impl: IMDSPEnumDeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32, ppdevice: *mut ::windows::core::RawPtr, pceltfetched: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Next(celt, ::core::mem::transmute_copy(&ppdevice), ::core::mem::transmute_copy(&pceltfetched)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Skip<Impl: IMDSPEnumDeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32, pceltfetched: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Skip(celt, ::core::mem::transmute_copy(&pceltfetched)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Reset<Impl: IMDSPEnumDeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Clone<Impl: IMDSPEnumDeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenumdevice: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Clone(::core::mem::transmute_copy(&ppenumdevice)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IMDSPEnumDevice>, ::windows::core::GetTrustLevel, Next::<Impl, OFFSET>, Skip::<Impl, OFFSET>, Reset::<Impl, OFFSET>, Clone::<Impl, OFFSET>)
    }
}
pub trait IMDSPEnumStorageImpl: Sized {
    fn Next();
    fn Skip();
    fn Reset();
    fn Clone();
}
impl ::windows::core::RuntimeName for IMDSPEnumStorage {
    const NAME: &'static str = "Windows.Win32.Media.DeviceManager.IMDSPEnumStorage";
}
impl IMDSPEnumStorageVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMDSPEnumStorageImpl, const OFFSET: isize>() -> IMDSPEnumStorageVtbl {
        unsafe extern "system" fn Next<Impl: IMDSPEnumStorageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32, ppstorage: *mut ::windows::core::RawPtr, pceltfetched: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Next(celt, ::core::mem::transmute_copy(&ppstorage), ::core::mem::transmute_copy(&pceltfetched)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Skip<Impl: IMDSPEnumStorageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32, pceltfetched: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Skip(celt, ::core::mem::transmute_copy(&pceltfetched)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Reset<Impl: IMDSPEnumStorageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Clone<Impl: IMDSPEnumStorageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenumstorage: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Clone(::core::mem::transmute_copy(&ppenumstorage)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IMDSPEnumStorage>, ::windows::core::GetTrustLevel, Next::<Impl, OFFSET>, Skip::<Impl, OFFSET>, Reset::<Impl, OFFSET>, Clone::<Impl, OFFSET>)
    }
}
pub trait IMDSPObjectImpl: Sized {
    fn Open();
    fn Read();
    fn Write();
    fn Delete();
    fn Seek();
    fn Rename();
    fn Move();
    fn Close();
}
impl ::windows::core::RuntimeName for IMDSPObject {
    const NAME: &'static str = "Windows.Win32.Media.DeviceManager.IMDSPObject";
}
impl IMDSPObjectVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMDSPObjectImpl, const OFFSET: isize>() -> IMDSPObjectVtbl {
        unsafe extern "system" fn Open<Impl: IMDSPObjectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fumode: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Open(fumode) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Read<Impl: IMDSPObjectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdata: *mut u8, pdwsize: *mut u32, abmac: *mut u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Read(::core::mem::transmute_copy(&pdata), pdwsize, abmac) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Write<Impl: IMDSPObjectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdata: *const u8, pdwsize: *mut u32, abmac: *mut u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Write(pdata, pdwsize, abmac) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Delete<Impl: IMDSPObjectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fumode: u32, pprogress: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Delete(fumode, &*(&pprogress as *const <IWMDMProgress as ::windows::core::Abi>::Abi as *const <IWMDMProgress as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Seek<Impl: IMDSPObjectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fuflags: u32, dwoffset: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Seek(fuflags, dwoffset) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Rename<Impl: IMDSPObjectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwsznewname: super::super::Foundation::PWSTR, pprogress: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Rename(&*(&pwsznewname as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), &*(&pprogress as *const <IWMDMProgress as ::windows::core::Abi>::Abi as *const <IWMDMProgress as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Move<Impl: IMDSPObjectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fumode: u32, pprogress: ::windows::core::RawPtr, ptarget: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Move(fumode, &*(&pprogress as *const <IWMDMProgress as ::windows::core::Abi>::Abi as *const <IWMDMProgress as ::windows::core::DefaultType>::DefaultType), &*(&ptarget as *const <IMDSPStorage as ::windows::core::Abi>::Abi as *const <IMDSPStorage as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Close<Impl: IMDSPObjectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
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
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IMDSPObject>, ::windows::core::GetTrustLevel, Open::<Impl, OFFSET>, Read::<Impl, OFFSET>, Write::<Impl, OFFSET>, Delete::<Impl, OFFSET>, Seek::<Impl, OFFSET>, Rename::<Impl, OFFSET>, Move::<Impl, OFFSET>, Close::<Impl, OFFSET>)
    }
}
pub trait IMDSPObject2Impl: Sized + IMDSPObjectImpl {
    fn ReadOnClearChannel();
    fn WriteOnClearChannel();
}
impl ::windows::core::RuntimeName for IMDSPObject2 {
    const NAME: &'static str = "Windows.Win32.Media.DeviceManager.IMDSPObject2";
}
impl IMDSPObject2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMDSPObject2Impl, const OFFSET: isize>() -> IMDSPObject2Vtbl {
        unsafe extern "system" fn ReadOnClearChannel<Impl: IMDSPObject2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdata: *mut u8, pdwsize: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ReadOnClearChannel(::core::mem::transmute_copy(&pdata), pdwsize) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn WriteOnClearChannel<Impl: IMDSPObject2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdata: *const u8, pdwsize: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).WriteOnClearChannel(pdata, pdwsize) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IMDSPObject2>, ::windows::core::GetTrustLevel, ReadOnClearChannel::<Impl, OFFSET>, WriteOnClearChannel::<Impl, OFFSET>)
    }
}
pub trait IMDSPObjectInfoImpl: Sized {
    fn GetPlayLength();
    fn SetPlayLength();
    fn GetPlayOffset();
    fn SetPlayOffset();
    fn GetTotalLength();
    fn GetLastPlayPosition();
    fn GetLongestPlayPosition();
}
impl ::windows::core::RuntimeName for IMDSPObjectInfo {
    const NAME: &'static str = "Windows.Win32.Media.DeviceManager.IMDSPObjectInfo";
}
impl IMDSPObjectInfoVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMDSPObjectInfoImpl, const OFFSET: isize>() -> IMDSPObjectInfoVtbl {
        unsafe extern "system" fn GetPlayLength<Impl: IMDSPObjectInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwlength: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetPlayLength(::core::mem::transmute_copy(&pdwlength)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPlayLength<Impl: IMDSPObjectInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwlength: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetPlayLength(dwlength) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPlayOffset<Impl: IMDSPObjectInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwoffset: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetPlayOffset(::core::mem::transmute_copy(&pdwoffset)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPlayOffset<Impl: IMDSPObjectInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwoffset: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetPlayOffset(dwoffset) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetTotalLength<Impl: IMDSPObjectInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwlength: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetTotalLength(::core::mem::transmute_copy(&pdwlength)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetLastPlayPosition<Impl: IMDSPObjectInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwlastpos: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetLastPlayPosition(::core::mem::transmute_copy(&pdwlastpos)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetLongestPlayPosition<Impl: IMDSPObjectInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwlongestpos: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetLongestPlayPosition(::core::mem::transmute_copy(&pdwlongestpos)) {
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
            ::windows::core::GetRuntimeClassName::<IMDSPObjectInfo>,
            ::windows::core::GetTrustLevel,
            GetPlayLength::<Impl, OFFSET>,
            SetPlayLength::<Impl, OFFSET>,
            GetPlayOffset::<Impl, OFFSET>,
            SetPlayOffset::<Impl, OFFSET>,
            GetTotalLength::<Impl, OFFSET>,
            GetLastPlayPosition::<Impl, OFFSET>,
            GetLongestPlayPosition::<Impl, OFFSET>,
        )
    }
}
pub trait IMDSPRevokedImpl: Sized {
    fn GetRevocationURL();
}
impl ::windows::core::RuntimeName for IMDSPRevoked {
    const NAME: &'static str = "Windows.Win32.Media.DeviceManager.IMDSPRevoked";
}
impl IMDSPRevokedVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMDSPRevokedImpl, const OFFSET: isize>() -> IMDSPRevokedVtbl {
        unsafe extern "system" fn GetRevocationURL<Impl: IMDSPRevokedImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppwszrevocationurl: *mut super::super::Foundation::PWSTR, pdwbufferlen: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetRevocationURL(&*(&ppwszrevocationurl as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), pdwbufferlen) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IMDSPRevoked>, ::windows::core::GetTrustLevel, GetRevocationURL::<Impl, OFFSET>)
    }
}
pub trait IMDSPStorageImpl: Sized {
    fn SetAttributes();
    fn GetStorageGlobals();
    fn GetAttributes();
    fn GetName();
    fn GetDate();
    fn GetSize();
    fn GetRights();
    fn CreateStorage();
    fn EnumStorage();
    fn SendOpaqueCommand();
}
impl ::windows::core::RuntimeName for IMDSPStorage {
    const NAME: &'static str = "Windows.Win32.Media.DeviceManager.IMDSPStorage";
}
impl IMDSPStorageVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMDSPStorageImpl, const OFFSET: isize>() -> IMDSPStorageVtbl {
        unsafe extern "system" fn SetAttributes<Impl: IMDSPStorageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwattributes: u32, pformat: *const _WAVEFORMATEX) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetAttributes(dwattributes, &*(&pformat as *const <_WAVEFORMATEX as ::windows::core::Abi>::Abi as *const <_WAVEFORMATEX as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetStorageGlobals<Impl: IMDSPStorageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppstorageglobals: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetStorageGlobals(::core::mem::transmute_copy(&ppstorageglobals)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetAttributes<Impl: IMDSPStorageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwattributes: *mut u32, pformat: *mut _WAVEFORMATEX) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetAttributes(::core::mem::transmute_copy(&pdwattributes), &*(&pformat as *const <_WAVEFORMATEX as ::windows::core::Abi>::Abi as *const <_WAVEFORMATEX as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetName<Impl: IMDSPStorageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwszname: super::super::Foundation::PWSTR, nmaxchars: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetName(::core::mem::transmute_copy(&pwszname), nmaxchars) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDate<Impl: IMDSPStorageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdatetimeutc: *mut WMDMDATETIME) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDate(::core::mem::transmute_copy(&pdatetimeutc)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSize<Impl: IMDSPStorageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwsizelow: *mut u32, pdwsizehigh: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetSize(::core::mem::transmute_copy(&pdwsizelow), ::core::mem::transmute_copy(&pdwsizehigh)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetRights<Impl: IMDSPStorageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pprights: *mut *mut WMDMRIGHTS, pnrightscount: *mut u32, abmac: *mut u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetRights(::core::mem::transmute_copy(&pprights), ::core::mem::transmute_copy(&pnrightscount), abmac) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateStorage<Impl: IMDSPStorageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwattributes: u32, pformat: *const _WAVEFORMATEX, pwszname: super::super::Foundation::PWSTR, ppnewstorage: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateStorage(dwattributes, &*(&pformat as *const <_WAVEFORMATEX as ::windows::core::Abi>::Abi as *const <_WAVEFORMATEX as ::windows::core::DefaultType>::DefaultType), &*(&pwszname as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&ppnewstorage)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnumStorage<Impl: IMDSPStorageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenumstorage: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EnumStorage(::core::mem::transmute_copy(&ppenumstorage)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SendOpaqueCommand<Impl: IMDSPStorageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcommand: *mut OPAQUECOMMAND) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SendOpaqueCommand(&*(&pcommand as *const <OPAQUECOMMAND as ::windows::core::Abi>::Abi as *const <OPAQUECOMMAND as ::windows::core::DefaultType>::DefaultType)) {
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
            ::windows::core::GetRuntimeClassName::<IMDSPStorage>,
            ::windows::core::GetTrustLevel,
            SetAttributes::<Impl, OFFSET>,
            GetStorageGlobals::<Impl, OFFSET>,
            GetAttributes::<Impl, OFFSET>,
            GetName::<Impl, OFFSET>,
            GetDate::<Impl, OFFSET>,
            GetSize::<Impl, OFFSET>,
            GetRights::<Impl, OFFSET>,
            CreateStorage::<Impl, OFFSET>,
            EnumStorage::<Impl, OFFSET>,
            SendOpaqueCommand::<Impl, OFFSET>,
        )
    }
}
pub trait IMDSPStorage2Impl: Sized + IMDSPStorageImpl {
    fn GetStorage();
    fn CreateStorage2();
    fn SetAttributes2();
    fn GetAttributes2();
}
impl ::windows::core::RuntimeName for IMDSPStorage2 {
    const NAME: &'static str = "Windows.Win32.Media.DeviceManager.IMDSPStorage2";
}
impl IMDSPStorage2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMDSPStorage2Impl, const OFFSET: isize>() -> IMDSPStorage2Vtbl {
        unsafe extern "system" fn GetStorage<Impl: IMDSPStorage2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszstoragename: super::super::Foundation::PWSTR, ppstorage: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetStorage(&*(&pszstoragename as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&ppstorage)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateStorage2<Impl: IMDSPStorage2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwattributes: u32, dwattributesex: u32, paudioformat: *const _WAVEFORMATEX, pvideoformat: *const _VIDEOINFOHEADER, pwszname: super::super::Foundation::PWSTR, qwfilesize: u64, ppnewstorage: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateStorage2(
                dwattributes,
                dwattributesex,
                &*(&paudioformat as *const <_WAVEFORMATEX as ::windows::core::Abi>::Abi as *const <_WAVEFORMATEX as ::windows::core::DefaultType>::DefaultType),
                &*(&pvideoformat as *const <_VIDEOINFOHEADER as ::windows::core::Abi>::Abi as *const <_VIDEOINFOHEADER as ::windows::core::DefaultType>::DefaultType),
                &*(&pwszname as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType),
                qwfilesize,
                ::core::mem::transmute_copy(&ppnewstorage),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAttributes2<Impl: IMDSPStorage2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwattributes: u32, dwattributesex: u32, paudioformat: *const _WAVEFORMATEX, pvideoformat: *const _VIDEOINFOHEADER) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetAttributes2(dwattributes, dwattributesex, &*(&paudioformat as *const <_WAVEFORMATEX as ::windows::core::Abi>::Abi as *const <_WAVEFORMATEX as ::windows::core::DefaultType>::DefaultType), &*(&pvideoformat as *const <_VIDEOINFOHEADER as ::windows::core::Abi>::Abi as *const <_VIDEOINFOHEADER as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetAttributes2<Impl: IMDSPStorage2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwattributes: *mut u32, pdwattributesex: *mut u32, paudioformat: *mut _WAVEFORMATEX, pvideoformat: *mut _VIDEOINFOHEADER) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetAttributes2(::core::mem::transmute_copy(&pdwattributes), ::core::mem::transmute_copy(&pdwattributesex), &*(&paudioformat as *const <_WAVEFORMATEX as ::windows::core::Abi>::Abi as *const <_WAVEFORMATEX as ::windows::core::DefaultType>::DefaultType), &*(&pvideoformat as *const <_VIDEOINFOHEADER as ::windows::core::Abi>::Abi as *const <_VIDEOINFOHEADER as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IMDSPStorage2>, ::windows::core::GetTrustLevel, GetStorage::<Impl, OFFSET>, CreateStorage2::<Impl, OFFSET>, SetAttributes2::<Impl, OFFSET>, GetAttributes2::<Impl, OFFSET>)
    }
}
pub trait IMDSPStorage3Impl: Sized + IMDSPStorage2Impl + IMDSPStorageImpl {
    fn GetMetadata();
    fn SetMetadata();
}
impl ::windows::core::RuntimeName for IMDSPStorage3 {
    const NAME: &'static str = "Windows.Win32.Media.DeviceManager.IMDSPStorage3";
}
impl IMDSPStorage3Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMDSPStorage3Impl, const OFFSET: isize>() -> IMDSPStorage3Vtbl {
        unsafe extern "system" fn GetMetadata<Impl: IMDSPStorage3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pmetadata: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetMetadata(&*(&pmetadata as *const <IWMDMMetaData as ::windows::core::Abi>::Abi as *const <IWMDMMetaData as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMetadata<Impl: IMDSPStorage3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pmetadata: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetMetadata(&*(&pmetadata as *const <IWMDMMetaData as ::windows::core::Abi>::Abi as *const <IWMDMMetaData as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IMDSPStorage3>, ::windows::core::GetTrustLevel, GetMetadata::<Impl, OFFSET>, SetMetadata::<Impl, OFFSET>)
    }
}
pub trait IMDSPStorage4Impl: Sized + IMDSPStorage3Impl + IMDSPStorage2Impl + IMDSPStorageImpl {
    fn SetReferences();
    fn GetReferences();
    fn CreateStorageWithMetadata();
    fn GetSpecifiedMetadata();
    fn FindStorage();
    fn GetParent();
}
impl ::windows::core::RuntimeName for IMDSPStorage4 {
    const NAME: &'static str = "Windows.Win32.Media.DeviceManager.IMDSPStorage4";
}
impl IMDSPStorage4Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMDSPStorage4Impl, const OFFSET: isize>() -> IMDSPStorage4Vtbl {
        unsafe extern "system" fn SetReferences<Impl: IMDSPStorage4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwrefs: u32, ppispstorage: *const ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetReferences(dwrefs, &*(&ppispstorage as *const <IMDSPStorage as ::windows::core::Abi>::Abi as *const <IMDSPStorage as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetReferences<Impl: IMDSPStorage4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwrefs: *mut u32, pppispstorage: *mut *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetReferences(::core::mem::transmute_copy(&pdwrefs), ::core::mem::transmute_copy(&pppispstorage)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateStorageWithMetadata<Impl: IMDSPStorage4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwattributes: u32, pwszname: super::super::Foundation::PWSTR, pmetadata: ::windows::core::RawPtr, qwfilesize: u64, ppnewstorage: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateStorageWithMetadata(dwattributes, &*(&pwszname as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), &*(&pmetadata as *const <IWMDMMetaData as ::windows::core::Abi>::Abi as *const <IWMDMMetaData as ::windows::core::DefaultType>::DefaultType), qwfilesize, ::core::mem::transmute_copy(&ppnewstorage)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSpecifiedMetadata<Impl: IMDSPStorage4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cproperties: u32, ppwszpropnames: *const super::super::Foundation::PWSTR, pmetadata: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetSpecifiedMetadata(cproperties, &*(&ppwszpropnames as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), &*(&pmetadata as *const <IWMDMMetaData as ::windows::core::Abi>::Abi as *const <IWMDMMetaData as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FindStorage<Impl: IMDSPStorage4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, findscope: WMDM_FIND_SCOPE, pwszuniqueid: super::super::Foundation::PWSTR, ppstorage: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FindStorage(findscope, &*(&pwszuniqueid as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&ppstorage)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetParent<Impl: IMDSPStorage4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppstorage: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetParent(::core::mem::transmute_copy(&ppstorage)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IMDSPStorage4>, ::windows::core::GetTrustLevel, SetReferences::<Impl, OFFSET>, GetReferences::<Impl, OFFSET>, CreateStorageWithMetadata::<Impl, OFFSET>, GetSpecifiedMetadata::<Impl, OFFSET>, FindStorage::<Impl, OFFSET>, GetParent::<Impl, OFFSET>)
    }
}
pub trait IMDSPStorageGlobalsImpl: Sized {
    fn GetCapabilities();
    fn GetSerialNumber();
    fn GetTotalSize();
    fn GetTotalFree();
    fn GetTotalBad();
    fn GetStatus();
    fn Initialize();
    fn GetDevice();
    fn GetRootStorage();
}
impl ::windows::core::RuntimeName for IMDSPStorageGlobals {
    const NAME: &'static str = "Windows.Win32.Media.DeviceManager.IMDSPStorageGlobals";
}
impl IMDSPStorageGlobalsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMDSPStorageGlobalsImpl, const OFFSET: isize>() -> IMDSPStorageGlobalsVtbl {
        unsafe extern "system" fn GetCapabilities<Impl: IMDSPStorageGlobalsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwcapabilities: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetCapabilities(::core::mem::transmute_copy(&pdwcapabilities)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSerialNumber<Impl: IMDSPStorageGlobalsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pserialnum: *mut WMDMID, abmac: *mut u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetSerialNumber(::core::mem::transmute_copy(&pserialnum), abmac) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetTotalSize<Impl: IMDSPStorageGlobalsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwtotalsizelow: *mut u32, pdwtotalsizehigh: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetTotalSize(::core::mem::transmute_copy(&pdwtotalsizelow), ::core::mem::transmute_copy(&pdwtotalsizehigh)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetTotalFree<Impl: IMDSPStorageGlobalsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwfreelow: *mut u32, pdwfreehigh: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetTotalFree(::core::mem::transmute_copy(&pdwfreelow), ::core::mem::transmute_copy(&pdwfreehigh)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetTotalBad<Impl: IMDSPStorageGlobalsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwbadlow: *mut u32, pdwbadhigh: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetTotalBad(::core::mem::transmute_copy(&pdwbadlow), ::core::mem::transmute_copy(&pdwbadhigh)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetStatus<Impl: IMDSPStorageGlobalsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwstatus: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetStatus(::core::mem::transmute_copy(&pdwstatus)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Initialize<Impl: IMDSPStorageGlobalsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fumode: u32, pprogress: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Initialize(fumode, &*(&pprogress as *const <IWMDMProgress as ::windows::core::Abi>::Abi as *const <IWMDMProgress as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDevice<Impl: IMDSPStorageGlobalsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppdevice: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDevice(::core::mem::transmute_copy(&ppdevice)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetRootStorage<Impl: IMDSPStorageGlobalsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pproot: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetRootStorage(::core::mem::transmute_copy(&pproot)) {
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
            ::windows::core::GetRuntimeClassName::<IMDSPStorageGlobals>,
            ::windows::core::GetTrustLevel,
            GetCapabilities::<Impl, OFFSET>,
            GetSerialNumber::<Impl, OFFSET>,
            GetTotalSize::<Impl, OFFSET>,
            GetTotalFree::<Impl, OFFSET>,
            GetTotalBad::<Impl, OFFSET>,
            GetStatus::<Impl, OFFSET>,
            Initialize::<Impl, OFFSET>,
            GetDevice::<Impl, OFFSET>,
            GetRootStorage::<Impl, OFFSET>,
        )
    }
}
pub trait IMDServiceProviderImpl: Sized {
    fn GetDeviceCount();
    fn EnumDevices();
}
impl ::windows::core::RuntimeName for IMDServiceProvider {
    const NAME: &'static str = "Windows.Win32.Media.DeviceManager.IMDServiceProvider";
}
impl IMDServiceProviderVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMDServiceProviderImpl, const OFFSET: isize>() -> IMDServiceProviderVtbl {
        unsafe extern "system" fn GetDeviceCount<Impl: IMDServiceProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwcount: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDeviceCount(::core::mem::transmute_copy(&pdwcount)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnumDevices<Impl: IMDServiceProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenumdevice: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EnumDevices(::core::mem::transmute_copy(&ppenumdevice)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IMDServiceProvider>, ::windows::core::GetTrustLevel, GetDeviceCount::<Impl, OFFSET>, EnumDevices::<Impl, OFFSET>)
    }
}
pub trait IMDServiceProvider2Impl: Sized + IMDServiceProviderImpl {
    fn CreateDevice();
}
impl ::windows::core::RuntimeName for IMDServiceProvider2 {
    const NAME: &'static str = "Windows.Win32.Media.DeviceManager.IMDServiceProvider2";
}
impl IMDServiceProvider2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMDServiceProvider2Impl, const OFFSET: isize>() -> IMDServiceProvider2Vtbl {
        unsafe extern "system" fn CreateDevice<Impl: IMDServiceProvider2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwszdevicepath: super::super::Foundation::PWSTR, pdwcount: *mut u32, pppdevicearray: *mut *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateDevice(&*(&pwszdevicepath as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&pdwcount), ::core::mem::transmute_copy(&pppdevicearray)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IMDServiceProvider2>, ::windows::core::GetTrustLevel, CreateDevice::<Impl, OFFSET>)
    }
}
pub trait IMDServiceProvider3Impl: Sized + IMDServiceProvider2Impl + IMDServiceProviderImpl {
    fn SetDeviceEnumPreference();
}
impl ::windows::core::RuntimeName for IMDServiceProvider3 {
    const NAME: &'static str = "Windows.Win32.Media.DeviceManager.IMDServiceProvider3";
}
impl IMDServiceProvider3Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMDServiceProvider3Impl, const OFFSET: isize>() -> IMDServiceProvider3Vtbl {
        unsafe extern "system" fn SetDeviceEnumPreference<Impl: IMDServiceProvider3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwenumpref: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetDeviceEnumPreference(dwenumpref) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IMDServiceProvider3>, ::windows::core::GetTrustLevel, SetDeviceEnumPreference::<Impl, OFFSET>)
    }
}
pub trait ISCPSecureAuthenticateImpl: Sized {
    fn GetSecureQuery();
}
impl ::windows::core::RuntimeName for ISCPSecureAuthenticate {
    const NAME: &'static str = "Windows.Win32.Media.DeviceManager.ISCPSecureAuthenticate";
}
impl ISCPSecureAuthenticateVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISCPSecureAuthenticateImpl, const OFFSET: isize>() -> ISCPSecureAuthenticateVtbl {
        unsafe extern "system" fn GetSecureQuery<Impl: ISCPSecureAuthenticateImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppsecurequery: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetSecureQuery(::core::mem::transmute_copy(&ppsecurequery)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ISCPSecureAuthenticate>, ::windows::core::GetTrustLevel, GetSecureQuery::<Impl, OFFSET>)
    }
}
pub trait ISCPSecureAuthenticate2Impl: Sized + ISCPSecureAuthenticateImpl {
    fn GetSCPSession();
}
impl ::windows::core::RuntimeName for ISCPSecureAuthenticate2 {
    const NAME: &'static str = "Windows.Win32.Media.DeviceManager.ISCPSecureAuthenticate2";
}
impl ISCPSecureAuthenticate2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISCPSecureAuthenticate2Impl, const OFFSET: isize>() -> ISCPSecureAuthenticate2Vtbl {
        unsafe extern "system" fn GetSCPSession<Impl: ISCPSecureAuthenticate2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppscpsession: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetSCPSession(::core::mem::transmute_copy(&ppscpsession)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ISCPSecureAuthenticate2>, ::windows::core::GetTrustLevel, GetSCPSession::<Impl, OFFSET>)
    }
}
pub trait ISCPSecureExchangeImpl: Sized {
    fn TransferContainerData();
    fn ObjectData();
    fn TransferComplete();
}
impl ::windows::core::RuntimeName for ISCPSecureExchange {
    const NAME: &'static str = "Windows.Win32.Media.DeviceManager.ISCPSecureExchange";
}
impl ISCPSecureExchangeVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISCPSecureExchangeImpl, const OFFSET: isize>() -> ISCPSecureExchangeVtbl {
        unsafe extern "system" fn TransferContainerData<Impl: ISCPSecureExchangeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdata: *const u8, dwsize: u32, pfureadyflags: *mut u32, abmac: *mut u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TransferContainerData(pdata, dwsize, ::core::mem::transmute_copy(&pfureadyflags), abmac) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ObjectData<Impl: ISCPSecureExchangeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdata: *mut u8, pdwsize: *mut u32, abmac: *mut u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ObjectData(::core::mem::transmute_copy(&pdata), pdwsize, abmac) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TransferComplete<Impl: ISCPSecureExchangeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TransferComplete() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ISCPSecureExchange>, ::windows::core::GetTrustLevel, TransferContainerData::<Impl, OFFSET>, ObjectData::<Impl, OFFSET>, TransferComplete::<Impl, OFFSET>)
    }
}
pub trait ISCPSecureExchange2Impl: Sized + ISCPSecureExchangeImpl {
    fn TransferContainerData2();
}
impl ::windows::core::RuntimeName for ISCPSecureExchange2 {
    const NAME: &'static str = "Windows.Win32.Media.DeviceManager.ISCPSecureExchange2";
}
impl ISCPSecureExchange2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISCPSecureExchange2Impl, const OFFSET: isize>() -> ISCPSecureExchange2Vtbl {
        unsafe extern "system" fn TransferContainerData2<Impl: ISCPSecureExchange2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdata: *const u8, dwsize: u32, pprogresscallback: ::windows::core::RawPtr, pfureadyflags: *mut u32, abmac: *mut u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TransferContainerData2(pdata, dwsize, &*(&pprogresscallback as *const <IWMDMProgress3 as ::windows::core::Abi>::Abi as *const <IWMDMProgress3 as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&pfureadyflags), abmac) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ISCPSecureExchange2>, ::windows::core::GetTrustLevel, TransferContainerData2::<Impl, OFFSET>)
    }
}
pub trait ISCPSecureExchange3Impl: Sized + ISCPSecureExchange2Impl + ISCPSecureExchangeImpl {
    fn TransferContainerDataOnClearChannel();
    fn GetObjectDataOnClearChannel();
    fn TransferCompleteForDevice();
}
impl ::windows::core::RuntimeName for ISCPSecureExchange3 {
    const NAME: &'static str = "Windows.Win32.Media.DeviceManager.ISCPSecureExchange3";
}
impl ISCPSecureExchange3Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISCPSecureExchange3Impl, const OFFSET: isize>() -> ISCPSecureExchange3Vtbl {
        unsafe extern "system" fn TransferContainerDataOnClearChannel<Impl: ISCPSecureExchange3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdevice: ::windows::core::RawPtr, pdata: *const u8, dwsize: u32, pprogresscallback: ::windows::core::RawPtr, pfureadyflags: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TransferContainerDataOnClearChannel(&*(&pdevice as *const <IMDSPDevice as ::windows::core::Abi>::Abi as *const <IMDSPDevice as ::windows::core::DefaultType>::DefaultType), pdata, dwsize, &*(&pprogresscallback as *const <IWMDMProgress3 as ::windows::core::Abi>::Abi as *const <IWMDMProgress3 as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&pfureadyflags)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetObjectDataOnClearChannel<Impl: ISCPSecureExchange3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdevice: ::windows::core::RawPtr, pdata: *mut u8, pdwsize: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetObjectDataOnClearChannel(&*(&pdevice as *const <IMDSPDevice as ::windows::core::Abi>::Abi as *const <IMDSPDevice as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&pdata), pdwsize) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TransferCompleteForDevice<Impl: ISCPSecureExchange3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdevice: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TransferCompleteForDevice(&*(&pdevice as *const <IMDSPDevice as ::windows::core::Abi>::Abi as *const <IMDSPDevice as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ISCPSecureExchange3>, ::windows::core::GetTrustLevel, TransferContainerDataOnClearChannel::<Impl, OFFSET>, GetObjectDataOnClearChannel::<Impl, OFFSET>, TransferCompleteForDevice::<Impl, OFFSET>)
    }
}
pub trait ISCPSecureQueryImpl: Sized {
    fn GetDataDemands();
    fn ExamineData();
    fn MakeDecision();
    fn GetRights();
}
impl ::windows::core::RuntimeName for ISCPSecureQuery {
    const NAME: &'static str = "Windows.Win32.Media.DeviceManager.ISCPSecureQuery";
}
impl ISCPSecureQueryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISCPSecureQueryImpl, const OFFSET: isize>() -> ISCPSecureQueryVtbl {
        unsafe extern "system" fn GetDataDemands<Impl: ISCPSecureQueryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfuflags: *mut u32, pdwminrightsdata: *mut u32, pdwminexaminedata: *mut u32, pdwmindecidedata: *mut u32, abmac: *mut u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDataDemands(::core::mem::transmute_copy(&pfuflags), ::core::mem::transmute_copy(&pdwminrightsdata), ::core::mem::transmute_copy(&pdwminexaminedata), ::core::mem::transmute_copy(&pdwmindecidedata), abmac) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ExamineData<Impl: ISCPSecureQueryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fuflags: u32, pwszextension: super::super::Foundation::PWSTR, pdata: *const u8, dwsize: u32, abmac: *mut u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ExamineData(fuflags, &*(&pwszextension as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), pdata, dwsize, abmac) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MakeDecision<Impl: ISCPSecureQueryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fuflags: u32, pdata: *const u8, dwsize: u32, dwappsec: u32, pbspsessionkey: *const u8, dwsessionkeylen: u32, pstorageglobals: ::windows::core::RawPtr, ppexchange: *mut ::windows::core::RawPtr, abmac: *mut u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MakeDecision(fuflags, pdata, dwsize, dwappsec, pbspsessionkey, dwsessionkeylen, &*(&pstorageglobals as *const <IMDSPStorageGlobals as ::windows::core::Abi>::Abi as *const <IMDSPStorageGlobals as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&ppexchange), abmac) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetRights<Impl: ISCPSecureQueryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdata: *const u8, dwsize: u32, pbspsessionkey: *const u8, dwsessionkeylen: u32, pstgglobals: ::windows::core::RawPtr, pprights: *mut *mut WMDMRIGHTS, pnrightscount: *mut u32, abmac: *mut u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetRights(pdata, dwsize, pbspsessionkey, dwsessionkeylen, &*(&pstgglobals as *const <IMDSPStorageGlobals as ::windows::core::Abi>::Abi as *const <IMDSPStorageGlobals as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&pprights), ::core::mem::transmute_copy(&pnrightscount), abmac) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ISCPSecureQuery>, ::windows::core::GetTrustLevel, GetDataDemands::<Impl, OFFSET>, ExamineData::<Impl, OFFSET>, MakeDecision::<Impl, OFFSET>, GetRights::<Impl, OFFSET>)
    }
}
pub trait ISCPSecureQuery2Impl: Sized + ISCPSecureQueryImpl {
    fn MakeDecision2();
}
impl ::windows::core::RuntimeName for ISCPSecureQuery2 {
    const NAME: &'static str = "Windows.Win32.Media.DeviceManager.ISCPSecureQuery2";
}
impl ISCPSecureQuery2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISCPSecureQuery2Impl, const OFFSET: isize>() -> ISCPSecureQuery2Vtbl {
        unsafe extern "system" fn MakeDecision2<Impl: ISCPSecureQuery2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fuflags: u32, pdata: *const u8, dwsize: u32, dwappsec: u32, pbspsessionkey: *const u8, dwsessionkeylen: u32, pstorageglobals: ::windows::core::RawPtr, pappcertapp: *const u8, dwappcertapplen: u32, pappcertsp: *const u8, dwappcertsplen: u32, pszrevocationurl: *mut super::super::Foundation::PWSTR, pdwrevocationurllen: *mut u32, pdwrevocationbitflag: *mut u32, pqwfilesize: *mut u64, punknown: *mut ::core::ffi::c_void, ppexchange: *mut ::windows::core::RawPtr, abmac: *mut u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MakeDecision2(
                fuflags,
                pdata,
                dwsize,
                dwappsec,
                pbspsessionkey,
                dwsessionkeylen,
                &*(&pstorageglobals as *const <IMDSPStorageGlobals as ::windows::core::Abi>::Abi as *const <IMDSPStorageGlobals as ::windows::core::DefaultType>::DefaultType),
                pappcertapp,
                dwappcertapplen,
                pappcertsp,
                dwappcertsplen,
                &*(&pszrevocationurl as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType),
                pdwrevocationurllen,
                ::core::mem::transmute_copy(&pdwrevocationbitflag),
                pqwfilesize,
                &*(&punknown as *const <::windows::core::IUnknown as ::windows::core::Abi>::Abi as *const <::windows::core::IUnknown as ::windows::core::DefaultType>::DefaultType),
                ::core::mem::transmute_copy(&ppexchange),
                abmac,
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ISCPSecureQuery2>, ::windows::core::GetTrustLevel, MakeDecision2::<Impl, OFFSET>)
    }
}
pub trait ISCPSecureQuery3Impl: Sized + ISCPSecureQuery2Impl + ISCPSecureQueryImpl {
    fn GetRightsOnClearChannel();
    fn MakeDecisionOnClearChannel();
}
impl ::windows::core::RuntimeName for ISCPSecureQuery3 {
    const NAME: &'static str = "Windows.Win32.Media.DeviceManager.ISCPSecureQuery3";
}
impl ISCPSecureQuery3Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISCPSecureQuery3Impl, const OFFSET: isize>() -> ISCPSecureQuery3Vtbl {
        unsafe extern "system" fn GetRightsOnClearChannel<Impl: ISCPSecureQuery3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdata: *const u8, dwsize: u32, pbspsessionkey: *const u8, dwsessionkeylen: u32, pstgglobals: ::windows::core::RawPtr, pprogresscallback: ::windows::core::RawPtr, pprights: *mut *mut WMDMRIGHTS, pnrightscount: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetRightsOnClearChannel(pdata, dwsize, pbspsessionkey, dwsessionkeylen, &*(&pstgglobals as *const <IMDSPStorageGlobals as ::windows::core::Abi>::Abi as *const <IMDSPStorageGlobals as ::windows::core::DefaultType>::DefaultType), &*(&pprogresscallback as *const <IWMDMProgress3 as ::windows::core::Abi>::Abi as *const <IWMDMProgress3 as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&pprights), ::core::mem::transmute_copy(&pnrightscount)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MakeDecisionOnClearChannel<Impl: ISCPSecureQuery3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fuflags: u32, pdata: *const u8, dwsize: u32, dwappsec: u32, pbspsessionkey: *const u8, dwsessionkeylen: u32, pstorageglobals: ::windows::core::RawPtr, pprogresscallback: ::windows::core::RawPtr, pappcertapp: *const u8, dwappcertapplen: u32, pappcertsp: *const u8, dwappcertsplen: u32, pszrevocationurl: *mut super::super::Foundation::PWSTR, pdwrevocationurllen: *mut u32, pdwrevocationbitflag: *mut u32, pqwfilesize: *mut u64, punknown: *mut ::core::ffi::c_void, ppexchange: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MakeDecisionOnClearChannel(
                fuflags,
                pdata,
                dwsize,
                dwappsec,
                pbspsessionkey,
                dwsessionkeylen,
                &*(&pstorageglobals as *const <IMDSPStorageGlobals as ::windows::core::Abi>::Abi as *const <IMDSPStorageGlobals as ::windows::core::DefaultType>::DefaultType),
                &*(&pprogresscallback as *const <IWMDMProgress3 as ::windows::core::Abi>::Abi as *const <IWMDMProgress3 as ::windows::core::DefaultType>::DefaultType),
                pappcertapp,
                dwappcertapplen,
                pappcertsp,
                dwappcertsplen,
                &*(&pszrevocationurl as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType),
                pdwrevocationurllen,
                ::core::mem::transmute_copy(&pdwrevocationbitflag),
                pqwfilesize,
                &*(&punknown as *const <::windows::core::IUnknown as ::windows::core::Abi>::Abi as *const <::windows::core::IUnknown as ::windows::core::DefaultType>::DefaultType),
                ::core::mem::transmute_copy(&ppexchange),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ISCPSecureQuery3>, ::windows::core::GetTrustLevel, GetRightsOnClearChannel::<Impl, OFFSET>, MakeDecisionOnClearChannel::<Impl, OFFSET>)
    }
}
pub trait ISCPSessionImpl: Sized {
    fn BeginSession();
    fn EndSession();
    fn GetSecureQuery();
}
impl ::windows::core::RuntimeName for ISCPSession {
    const NAME: &'static str = "Windows.Win32.Media.DeviceManager.ISCPSession";
}
impl ISCPSessionVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISCPSessionImpl, const OFFSET: isize>() -> ISCPSessionVtbl {
        unsafe extern "system" fn BeginSession<Impl: ISCPSessionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pidevice: ::windows::core::RawPtr, pctx: *const u8, dwsizectx: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).BeginSession(&*(&pidevice as *const <IMDSPDevice as ::windows::core::Abi>::Abi as *const <IMDSPDevice as ::windows::core::DefaultType>::DefaultType), pctx, dwsizectx) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EndSession<Impl: ISCPSessionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pctx: *const u8, dwsizectx: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EndSession(pctx, dwsizectx) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSecureQuery<Impl: ISCPSessionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppsecurequery: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetSecureQuery(::core::mem::transmute_copy(&ppsecurequery)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ISCPSession>, ::windows::core::GetTrustLevel, BeginSession::<Impl, OFFSET>, EndSession::<Impl, OFFSET>, GetSecureQuery::<Impl, OFFSET>)
    }
}
pub trait IWMDMDeviceImpl: Sized {
    fn GetName();
    fn GetManufacturer();
    fn GetVersion();
    fn GetType();
    fn GetSerialNumber();
    fn GetPowerSource();
    fn GetStatus();
    fn GetDeviceIcon();
    fn EnumStorage();
    fn GetFormatSupport();
    fn SendOpaqueCommand();
}
impl ::windows::core::RuntimeName for IWMDMDevice {
    const NAME: &'static str = "Windows.Win32.Media.DeviceManager.IWMDMDevice";
}
impl IWMDMDeviceVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMDMDeviceImpl, const OFFSET: isize>() -> IWMDMDeviceVtbl {
        unsafe extern "system" fn GetName<Impl: IWMDMDeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwszname: super::super::Foundation::PWSTR, nmaxchars: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetName(::core::mem::transmute_copy(&pwszname), nmaxchars) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetManufacturer<Impl: IWMDMDeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwszname: super::super::Foundation::PWSTR, nmaxchars: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetManufacturer(::core::mem::transmute_copy(&pwszname), nmaxchars) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetVersion<Impl: IWMDMDeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwversion: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetVersion(::core::mem::transmute_copy(&pdwversion)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetType<Impl: IWMDMDeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwtype: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetType(::core::mem::transmute_copy(&pdwtype)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSerialNumber<Impl: IWMDMDeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pserialnumber: *mut WMDMID, abmac: *mut u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetSerialNumber(::core::mem::transmute_copy(&pserialnumber), abmac) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPowerSource<Impl: IWMDMDeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwpowersource: *mut u32, pdwpercentremaining: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetPowerSource(::core::mem::transmute_copy(&pdwpowersource), ::core::mem::transmute_copy(&pdwpercentremaining)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetStatus<Impl: IWMDMDeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwstatus: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetStatus(::core::mem::transmute_copy(&pdwstatus)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDeviceIcon<Impl: IWMDMDeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hicon: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDeviceIcon(::core::mem::transmute_copy(&hicon)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnumStorage<Impl: IWMDMDeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenumstorage: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EnumStorage(::core::mem::transmute_copy(&ppenumstorage)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFormatSupport<Impl: IWMDMDeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppformatex: *mut *mut _WAVEFORMATEX, pnformatcount: *mut u32, pppwszmimetype: *mut *mut super::super::Foundation::PWSTR, pnmimetypecount: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetFormatSupport(::core::mem::transmute_copy(&ppformatex), ::core::mem::transmute_copy(&pnformatcount), ::core::mem::transmute_copy(&pppwszmimetype), ::core::mem::transmute_copy(&pnmimetypecount)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SendOpaqueCommand<Impl: IWMDMDeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcommand: *mut OPAQUECOMMAND) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SendOpaqueCommand(&*(&pcommand as *const <OPAQUECOMMAND as ::windows::core::Abi>::Abi as *const <OPAQUECOMMAND as ::windows::core::DefaultType>::DefaultType)) {
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
            ::windows::core::GetRuntimeClassName::<IWMDMDevice>,
            ::windows::core::GetTrustLevel,
            GetName::<Impl, OFFSET>,
            GetManufacturer::<Impl, OFFSET>,
            GetVersion::<Impl, OFFSET>,
            GetType::<Impl, OFFSET>,
            GetSerialNumber::<Impl, OFFSET>,
            GetPowerSource::<Impl, OFFSET>,
            GetStatus::<Impl, OFFSET>,
            GetDeviceIcon::<Impl, OFFSET>,
            EnumStorage::<Impl, OFFSET>,
            GetFormatSupport::<Impl, OFFSET>,
            SendOpaqueCommand::<Impl, OFFSET>,
        )
    }
}
pub trait IWMDMDevice2Impl: Sized + IWMDMDeviceImpl {
    fn GetStorage();
    fn GetFormatSupport2();
    fn GetSpecifyPropertyPages();
    fn GetCanonicalName();
}
impl ::windows::core::RuntimeName for IWMDMDevice2 {
    const NAME: &'static str = "Windows.Win32.Media.DeviceManager.IWMDMDevice2";
}
impl IWMDMDevice2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMDMDevice2Impl, const OFFSET: isize>() -> IWMDMDevice2Vtbl {
        unsafe extern "system" fn GetStorage<Impl: IWMDMDevice2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszstoragename: super::super::Foundation::PWSTR, ppstorage: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetStorage(&*(&pszstoragename as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&ppstorage)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFormatSupport2<Impl: IWMDMDevice2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwflags: u32, ppaudioformatex: *mut *mut _WAVEFORMATEX, pnaudioformatcount: *mut u32, ppvideoformatex: *mut *mut _VIDEOINFOHEADER, pnvideoformatcount: *mut u32, ppfiletype: *mut *mut WMFILECAPABILITIES, pnfiletypecount: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetFormatSupport2(dwflags, ::core::mem::transmute_copy(&ppaudioformatex), ::core::mem::transmute_copy(&pnaudioformatcount), ::core::mem::transmute_copy(&ppvideoformatex), ::core::mem::transmute_copy(&pnvideoformatcount), ::core::mem::transmute_copy(&ppfiletype), ::core::mem::transmute_copy(&pnfiletypecount)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSpecifyPropertyPages<Impl: IWMDMDevice2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppspecifyproppages: *mut ::windows::core::RawPtr, pppunknowns: *mut *mut *mut ::core::ffi::c_void, pcunks: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetSpecifyPropertyPages(::core::mem::transmute_copy(&ppspecifyproppages), ::core::mem::transmute_copy(&pppunknowns), ::core::mem::transmute_copy(&pcunks)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCanonicalName<Impl: IWMDMDevice2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwszpnpname: super::super::Foundation::PWSTR, nmaxchars: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetCanonicalName(::core::mem::transmute_copy(&pwszpnpname), nmaxchars) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IWMDMDevice2>, ::windows::core::GetTrustLevel, GetStorage::<Impl, OFFSET>, GetFormatSupport2::<Impl, OFFSET>, GetSpecifyPropertyPages::<Impl, OFFSET>, GetCanonicalName::<Impl, OFFSET>)
    }
}
pub trait IWMDMDevice3Impl: Sized + IWMDMDevice2Impl + IWMDMDeviceImpl {
    fn GetProperty();
    fn SetProperty();
    fn GetFormatCapability();
    fn DeviceIoControl();
    fn FindStorage();
}
impl ::windows::core::RuntimeName for IWMDMDevice3 {
    const NAME: &'static str = "Windows.Win32.Media.DeviceManager.IWMDMDevice3";
}
impl IWMDMDevice3Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMDMDevice3Impl, const OFFSET: isize>() -> IWMDMDevice3Vtbl {
        unsafe extern "system" fn GetProperty<Impl: IWMDMDevice3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwszpropname: super::super::Foundation::PWSTR, pvalue: *mut super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetProperty(&*(&pwszpropname as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&pvalue)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetProperty<Impl: IWMDMDevice3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwszpropname: super::super::Foundation::PWSTR, pvalue: *const super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetProperty(&*(&pwszpropname as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), &*(&pvalue as *const <super::super::System::Com::StructuredStorage::PROPVARIANT as ::windows::core::Abi>::Abi as *const <super::super::System::Com::StructuredStorage::PROPVARIANT as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFormatCapability<Impl: IWMDMDevice3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, format: WMDM_FORMATCODE, pformatsupport: *mut WMDM_FORMAT_CAPABILITY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetFormatCapability(format, ::core::mem::transmute_copy(&pformatsupport)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DeviceIoControl<Impl: IWMDMDevice3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwiocontrolcode: u32, lpinbuffer: *const u8, ninbuffersize: u32, lpoutbuffer: *mut u8, pnoutbuffersize: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DeviceIoControl(dwiocontrolcode, lpinbuffer, ninbuffersize, ::core::mem::transmute_copy(&lpoutbuffer), pnoutbuffersize) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FindStorage<Impl: IWMDMDevice3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, findscope: WMDM_FIND_SCOPE, pwszuniqueid: super::super::Foundation::PWSTR, ppstorage: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FindStorage(findscope, &*(&pwszuniqueid as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&ppstorage)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IWMDMDevice3>, ::windows::core::GetTrustLevel, GetProperty::<Impl, OFFSET>, SetProperty::<Impl, OFFSET>, GetFormatCapability::<Impl, OFFSET>, DeviceIoControl::<Impl, OFFSET>, FindStorage::<Impl, OFFSET>)
    }
}
pub trait IWMDMDeviceControlImpl: Sized {
    fn GetStatus();
    fn GetCapabilities();
    fn Play();
    fn Record();
    fn Pause();
    fn Resume();
    fn Stop();
    fn Seek();
}
impl ::windows::core::RuntimeName for IWMDMDeviceControl {
    const NAME: &'static str = "Windows.Win32.Media.DeviceManager.IWMDMDeviceControl";
}
impl IWMDMDeviceControlVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMDMDeviceControlImpl, const OFFSET: isize>() -> IWMDMDeviceControlVtbl {
        unsafe extern "system" fn GetStatus<Impl: IWMDMDeviceControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwstatus: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetStatus(::core::mem::transmute_copy(&pdwstatus)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCapabilities<Impl: IWMDMDeviceControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwcapabilitiesmask: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetCapabilities(::core::mem::transmute_copy(&pdwcapabilitiesmask)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Play<Impl: IWMDMDeviceControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Play() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Record<Impl: IWMDMDeviceControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pformat: *const _WAVEFORMATEX) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Record(&*(&pformat as *const <_WAVEFORMATEX as ::windows::core::Abi>::Abi as *const <_WAVEFORMATEX as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Pause<Impl: IWMDMDeviceControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Pause() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Resume<Impl: IWMDMDeviceControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Resume() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Stop<Impl: IWMDMDeviceControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Stop() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Seek<Impl: IWMDMDeviceControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fumode: u32, noffset: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Seek(fumode, noffset) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IWMDMDeviceControl>, ::windows::core::GetTrustLevel, GetStatus::<Impl, OFFSET>, GetCapabilities::<Impl, OFFSET>, Play::<Impl, OFFSET>, Record::<Impl, OFFSET>, Pause::<Impl, OFFSET>, Resume::<Impl, OFFSET>, Stop::<Impl, OFFSET>, Seek::<Impl, OFFSET>)
    }
}
pub trait IWMDMDeviceSessionImpl: Sized {
    fn BeginSession();
    fn EndSession();
}
impl ::windows::core::RuntimeName for IWMDMDeviceSession {
    const NAME: &'static str = "Windows.Win32.Media.DeviceManager.IWMDMDeviceSession";
}
impl IWMDMDeviceSessionVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMDMDeviceSessionImpl, const OFFSET: isize>() -> IWMDMDeviceSessionVtbl {
        unsafe extern "system" fn BeginSession<Impl: IWMDMDeviceSessionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, r#type: WMDM_SESSION_TYPE, pctx: *const u8, dwsizectx: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).BeginSession(r#type, pctx, dwsizectx) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EndSession<Impl: IWMDMDeviceSessionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, r#type: WMDM_SESSION_TYPE, pctx: *const u8, dwsizectx: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EndSession(r#type, pctx, dwsizectx) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IWMDMDeviceSession>, ::windows::core::GetTrustLevel, BeginSession::<Impl, OFFSET>, EndSession::<Impl, OFFSET>)
    }
}
pub trait IWMDMEnumDeviceImpl: Sized {
    fn Next();
    fn Skip();
    fn Reset();
    fn Clone();
}
impl ::windows::core::RuntimeName for IWMDMEnumDevice {
    const NAME: &'static str = "Windows.Win32.Media.DeviceManager.IWMDMEnumDevice";
}
impl IWMDMEnumDeviceVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMDMEnumDeviceImpl, const OFFSET: isize>() -> IWMDMEnumDeviceVtbl {
        unsafe extern "system" fn Next<Impl: IWMDMEnumDeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32, ppdevice: *mut ::windows::core::RawPtr, pceltfetched: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Next(celt, ::core::mem::transmute_copy(&ppdevice), ::core::mem::transmute_copy(&pceltfetched)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Skip<Impl: IWMDMEnumDeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32, pceltfetched: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Skip(celt, ::core::mem::transmute_copy(&pceltfetched)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Reset<Impl: IWMDMEnumDeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Clone<Impl: IWMDMEnumDeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenumdevice: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Clone(::core::mem::transmute_copy(&ppenumdevice)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IWMDMEnumDevice>, ::windows::core::GetTrustLevel, Next::<Impl, OFFSET>, Skip::<Impl, OFFSET>, Reset::<Impl, OFFSET>, Clone::<Impl, OFFSET>)
    }
}
pub trait IWMDMEnumStorageImpl: Sized {
    fn Next();
    fn Skip();
    fn Reset();
    fn Clone();
}
impl ::windows::core::RuntimeName for IWMDMEnumStorage {
    const NAME: &'static str = "Windows.Win32.Media.DeviceManager.IWMDMEnumStorage";
}
impl IWMDMEnumStorageVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMDMEnumStorageImpl, const OFFSET: isize>() -> IWMDMEnumStorageVtbl {
        unsafe extern "system" fn Next<Impl: IWMDMEnumStorageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32, ppstorage: *mut ::windows::core::RawPtr, pceltfetched: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Next(celt, ::core::mem::transmute_copy(&ppstorage), ::core::mem::transmute_copy(&pceltfetched)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Skip<Impl: IWMDMEnumStorageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32, pceltfetched: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Skip(celt, ::core::mem::transmute_copy(&pceltfetched)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Reset<Impl: IWMDMEnumStorageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Clone<Impl: IWMDMEnumStorageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenumstorage: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Clone(::core::mem::transmute_copy(&ppenumstorage)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IWMDMEnumStorage>, ::windows::core::GetTrustLevel, Next::<Impl, OFFSET>, Skip::<Impl, OFFSET>, Reset::<Impl, OFFSET>, Clone::<Impl, OFFSET>)
    }
}
pub trait IWMDMLoggerImpl: Sized {
    fn IsEnabled();
    fn Enable();
    fn GetLogFileName();
    fn SetLogFileName();
    fn LogString();
    fn LogDword();
    fn Reset();
    fn GetSizeParams();
    fn SetSizeParams();
}
impl ::windows::core::RuntimeName for IWMDMLogger {
    const NAME: &'static str = "Windows.Win32.Media.DeviceManager.IWMDMLogger";
}
impl IWMDMLoggerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMDMLoggerImpl, const OFFSET: isize>() -> IWMDMLoggerVtbl {
        unsafe extern "system" fn IsEnabled<Impl: IWMDMLoggerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfenabled: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsEnabled(::core::mem::transmute_copy(&pfenabled)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Enable<Impl: IWMDMLoggerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fenable: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Enable(&*(&fenable as *const <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetLogFileName<Impl: IWMDMLoggerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszfilename: super::super::Foundation::PSTR, nmaxchars: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetLogFileName(::core::mem::transmute_copy(&pszfilename), nmaxchars) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetLogFileName<Impl: IWMDMLoggerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszfilename: super::super::Foundation::PSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetLogFileName(&*(&pszfilename as *const <super::super::Foundation::PSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LogString<Impl: IWMDMLoggerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwflags: u32, pszsrcname: super::super::Foundation::PSTR, pszlog: super::super::Foundation::PSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LogString(dwflags, &*(&pszsrcname as *const <super::super::Foundation::PSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PSTR as ::windows::core::DefaultType>::DefaultType), &*(&pszlog as *const <super::super::Foundation::PSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LogDword<Impl: IWMDMLoggerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwflags: u32, pszsrcname: super::super::Foundation::PSTR, pszlogformat: super::super::Foundation::PSTR, dwlog: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LogDword(dwflags, &*(&pszsrcname as *const <super::super::Foundation::PSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PSTR as ::windows::core::DefaultType>::DefaultType), &*(&pszlogformat as *const <super::super::Foundation::PSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PSTR as ::windows::core::DefaultType>::DefaultType), dwlog) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Reset<Impl: IWMDMLoggerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn GetSizeParams<Impl: IWMDMLoggerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwmaxsize: *mut u32, pdwshrinktosize: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetSizeParams(::core::mem::transmute_copy(&pdwmaxsize), ::core::mem::transmute_copy(&pdwshrinktosize)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSizeParams<Impl: IWMDMLoggerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwmaxsize: u32, dwshrinktosize: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetSizeParams(dwmaxsize, dwshrinktosize) {
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
            ::windows::core::GetRuntimeClassName::<IWMDMLogger>,
            ::windows::core::GetTrustLevel,
            IsEnabled::<Impl, OFFSET>,
            Enable::<Impl, OFFSET>,
            GetLogFileName::<Impl, OFFSET>,
            SetLogFileName::<Impl, OFFSET>,
            LogString::<Impl, OFFSET>,
            LogDword::<Impl, OFFSET>,
            Reset::<Impl, OFFSET>,
            GetSizeParams::<Impl, OFFSET>,
            SetSizeParams::<Impl, OFFSET>,
        )
    }
}
pub trait IWMDMMetaDataImpl: Sized {
    fn AddItem();
    fn QueryByName();
    fn QueryByIndex();
    fn GetItemCount();
}
impl ::windows::core::RuntimeName for IWMDMMetaData {
    const NAME: &'static str = "Windows.Win32.Media.DeviceManager.IWMDMMetaData";
}
impl IWMDMMetaDataVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMDMMetaDataImpl, const OFFSET: isize>() -> IWMDMMetaDataVtbl {
        unsafe extern "system" fn AddItem<Impl: IWMDMMetaDataImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, r#type: WMDM_TAG_DATATYPE, pwsztagname: super::super::Foundation::PWSTR, pvalue: *const u8, ilength: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AddItem(r#type, &*(&pwsztagname as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), pvalue, ilength) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn QueryByName<Impl: IWMDMMetaDataImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwsztagname: super::super::Foundation::PWSTR, ptype: *mut WMDM_TAG_DATATYPE, pvalue: *mut *mut u8, pcblength: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).QueryByName(&*(&pwsztagname as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&ptype), ::core::mem::transmute_copy(&pvalue), ::core::mem::transmute_copy(&pcblength)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn QueryByIndex<Impl: IWMDMMetaDataImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, iindex: u32, ppwszname: *mut *mut u16, ptype: *mut WMDM_TAG_DATATYPE, ppvalue: *mut *mut u8, pcblength: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).QueryByIndex(iindex, ::core::mem::transmute_copy(&ppwszname), ::core::mem::transmute_copy(&ptype), ::core::mem::transmute_copy(&ppvalue), ::core::mem::transmute_copy(&pcblength)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetItemCount<Impl: IWMDMMetaDataImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, icount: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetItemCount(::core::mem::transmute_copy(&icount)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IWMDMMetaData>, ::windows::core::GetTrustLevel, AddItem::<Impl, OFFSET>, QueryByName::<Impl, OFFSET>, QueryByIndex::<Impl, OFFSET>, GetItemCount::<Impl, OFFSET>)
    }
}
pub trait IWMDMNotificationImpl: Sized {
    fn WMDMMessage();
}
impl ::windows::core::RuntimeName for IWMDMNotification {
    const NAME: &'static str = "Windows.Win32.Media.DeviceManager.IWMDMNotification";
}
impl IWMDMNotificationVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMDMNotificationImpl, const OFFSET: isize>() -> IWMDMNotificationVtbl {
        unsafe extern "system" fn WMDMMessage<Impl: IWMDMNotificationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwmessagetype: u32, pwszcanonicalname: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).WMDMMessage(dwmessagetype, &*(&pwszcanonicalname as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IWMDMNotification>, ::windows::core::GetTrustLevel, WMDMMessage::<Impl, OFFSET>)
    }
}
pub trait IWMDMObjectInfoImpl: Sized {
    fn GetPlayLength();
    fn SetPlayLength();
    fn GetPlayOffset();
    fn SetPlayOffset();
    fn GetTotalLength();
    fn GetLastPlayPosition();
    fn GetLongestPlayPosition();
}
impl ::windows::core::RuntimeName for IWMDMObjectInfo {
    const NAME: &'static str = "Windows.Win32.Media.DeviceManager.IWMDMObjectInfo";
}
impl IWMDMObjectInfoVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMDMObjectInfoImpl, const OFFSET: isize>() -> IWMDMObjectInfoVtbl {
        unsafe extern "system" fn GetPlayLength<Impl: IWMDMObjectInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwlength: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetPlayLength(::core::mem::transmute_copy(&pdwlength)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPlayLength<Impl: IWMDMObjectInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwlength: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetPlayLength(dwlength) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPlayOffset<Impl: IWMDMObjectInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwoffset: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetPlayOffset(::core::mem::transmute_copy(&pdwoffset)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPlayOffset<Impl: IWMDMObjectInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwoffset: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetPlayOffset(dwoffset) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetTotalLength<Impl: IWMDMObjectInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwlength: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetTotalLength(::core::mem::transmute_copy(&pdwlength)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetLastPlayPosition<Impl: IWMDMObjectInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwlastpos: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetLastPlayPosition(::core::mem::transmute_copy(&pdwlastpos)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetLongestPlayPosition<Impl: IWMDMObjectInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwlongestpos: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetLongestPlayPosition(::core::mem::transmute_copy(&pdwlongestpos)) {
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
            ::windows::core::GetRuntimeClassName::<IWMDMObjectInfo>,
            ::windows::core::GetTrustLevel,
            GetPlayLength::<Impl, OFFSET>,
            SetPlayLength::<Impl, OFFSET>,
            GetPlayOffset::<Impl, OFFSET>,
            SetPlayOffset::<Impl, OFFSET>,
            GetTotalLength::<Impl, OFFSET>,
            GetLastPlayPosition::<Impl, OFFSET>,
            GetLongestPlayPosition::<Impl, OFFSET>,
        )
    }
}
pub trait IWMDMOperationImpl: Sized {
    fn BeginRead();
    fn BeginWrite();
    fn GetObjectName();
    fn SetObjectName();
    fn GetObjectAttributes();
    fn SetObjectAttributes();
    fn GetObjectTotalSize();
    fn SetObjectTotalSize();
    fn TransferObjectData();
    fn End();
}
impl ::windows::core::RuntimeName for IWMDMOperation {
    const NAME: &'static str = "Windows.Win32.Media.DeviceManager.IWMDMOperation";
}
impl IWMDMOperationVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMDMOperationImpl, const OFFSET: isize>() -> IWMDMOperationVtbl {
        unsafe extern "system" fn BeginRead<Impl: IWMDMOperationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).BeginRead() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BeginWrite<Impl: IWMDMOperationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).BeginWrite() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetObjectName<Impl: IWMDMOperationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwszname: super::super::Foundation::PWSTR, nmaxchars: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetObjectName(::core::mem::transmute_copy(&pwszname), nmaxchars) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetObjectName<Impl: IWMDMOperationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwszname: super::super::Foundation::PWSTR, nmaxchars: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetObjectName(&*(&pwszname as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), nmaxchars) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetObjectAttributes<Impl: IWMDMOperationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwattributes: *mut u32, pformat: *mut _WAVEFORMATEX) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetObjectAttributes(::core::mem::transmute_copy(&pdwattributes), &*(&pformat as *const <_WAVEFORMATEX as ::windows::core::Abi>::Abi as *const <_WAVEFORMATEX as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetObjectAttributes<Impl: IWMDMOperationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwattributes: u32, pformat: *const _WAVEFORMATEX) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetObjectAttributes(dwattributes, &*(&pformat as *const <_WAVEFORMATEX as ::windows::core::Abi>::Abi as *const <_WAVEFORMATEX as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetObjectTotalSize<Impl: IWMDMOperationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwsize: *mut u32, pdwsizehigh: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetObjectTotalSize(::core::mem::transmute_copy(&pdwsize), ::core::mem::transmute_copy(&pdwsizehigh)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetObjectTotalSize<Impl: IWMDMOperationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwsize: u32, dwsizehigh: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetObjectTotalSize(dwsize, dwsizehigh) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TransferObjectData<Impl: IWMDMOperationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdata: *mut u8, pdwsize: *mut u32, abmac: *mut u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TransferObjectData(pdata, pdwsize, abmac) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn End<Impl: IWMDMOperationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, phcompletioncode: *const ::windows::core::HRESULT, pnewobject: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).End(phcompletioncode, &*(&pnewobject as *const <::windows::core::IUnknown as ::windows::core::Abi>::Abi as *const <::windows::core::IUnknown as ::windows::core::DefaultType>::DefaultType)) {
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
            ::windows::core::GetRuntimeClassName::<IWMDMOperation>,
            ::windows::core::GetTrustLevel,
            BeginRead::<Impl, OFFSET>,
            BeginWrite::<Impl, OFFSET>,
            GetObjectName::<Impl, OFFSET>,
            SetObjectName::<Impl, OFFSET>,
            GetObjectAttributes::<Impl, OFFSET>,
            SetObjectAttributes::<Impl, OFFSET>,
            GetObjectTotalSize::<Impl, OFFSET>,
            SetObjectTotalSize::<Impl, OFFSET>,
            TransferObjectData::<Impl, OFFSET>,
            End::<Impl, OFFSET>,
        )
    }
}
pub trait IWMDMOperation2Impl: Sized + IWMDMOperationImpl {
    fn SetObjectAttributes2();
    fn GetObjectAttributes2();
}
impl ::windows::core::RuntimeName for IWMDMOperation2 {
    const NAME: &'static str = "Windows.Win32.Media.DeviceManager.IWMDMOperation2";
}
impl IWMDMOperation2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMDMOperation2Impl, const OFFSET: isize>() -> IWMDMOperation2Vtbl {
        unsafe extern "system" fn SetObjectAttributes2<Impl: IWMDMOperation2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwattributes: u32, dwattributesex: u32, pformat: *const _WAVEFORMATEX, pvideoformat: *const _VIDEOINFOHEADER) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetObjectAttributes2(dwattributes, dwattributesex, &*(&pformat as *const <_WAVEFORMATEX as ::windows::core::Abi>::Abi as *const <_WAVEFORMATEX as ::windows::core::DefaultType>::DefaultType), &*(&pvideoformat as *const <_VIDEOINFOHEADER as ::windows::core::Abi>::Abi as *const <_VIDEOINFOHEADER as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetObjectAttributes2<Impl: IWMDMOperation2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwattributes: *mut u32, pdwattributesex: *mut u32, paudioformat: *mut _WAVEFORMATEX, pvideoformat: *mut _VIDEOINFOHEADER) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetObjectAttributes2(::core::mem::transmute_copy(&pdwattributes), ::core::mem::transmute_copy(&pdwattributesex), &*(&paudioformat as *const <_WAVEFORMATEX as ::windows::core::Abi>::Abi as *const <_WAVEFORMATEX as ::windows::core::DefaultType>::DefaultType), &*(&pvideoformat as *const <_VIDEOINFOHEADER as ::windows::core::Abi>::Abi as *const <_VIDEOINFOHEADER as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IWMDMOperation2>, ::windows::core::GetTrustLevel, SetObjectAttributes2::<Impl, OFFSET>, GetObjectAttributes2::<Impl, OFFSET>)
    }
}
pub trait IWMDMOperation3Impl: Sized + IWMDMOperationImpl {
    fn TransferObjectDataOnClearChannel();
}
impl ::windows::core::RuntimeName for IWMDMOperation3 {
    const NAME: &'static str = "Windows.Win32.Media.DeviceManager.IWMDMOperation3";
}
impl IWMDMOperation3Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMDMOperation3Impl, const OFFSET: isize>() -> IWMDMOperation3Vtbl {
        unsafe extern "system" fn TransferObjectDataOnClearChannel<Impl: IWMDMOperation3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdata: *mut u8, pdwsize: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TransferObjectDataOnClearChannel(pdata, pdwsize) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IWMDMOperation3>, ::windows::core::GetTrustLevel, TransferObjectDataOnClearChannel::<Impl, OFFSET>)
    }
}
pub trait IWMDMProgressImpl: Sized {
    fn Begin();
    fn Progress();
    fn End();
}
impl ::windows::core::RuntimeName for IWMDMProgress {
    const NAME: &'static str = "Windows.Win32.Media.DeviceManager.IWMDMProgress";
}
impl IWMDMProgressVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMDMProgressImpl, const OFFSET: isize>() -> IWMDMProgressVtbl {
        unsafe extern "system" fn Begin<Impl: IWMDMProgressImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwestimatedticks: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Begin(dwestimatedticks) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Progress<Impl: IWMDMProgressImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwtranspiredticks: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Progress(dwtranspiredticks) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn End<Impl: IWMDMProgressImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).End() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IWMDMProgress>, ::windows::core::GetTrustLevel, Begin::<Impl, OFFSET>, Progress::<Impl, OFFSET>, End::<Impl, OFFSET>)
    }
}
pub trait IWMDMProgress2Impl: Sized + IWMDMProgressImpl {
    fn End2();
}
impl ::windows::core::RuntimeName for IWMDMProgress2 {
    const NAME: &'static str = "Windows.Win32.Media.DeviceManager.IWMDMProgress2";
}
impl IWMDMProgress2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMDMProgress2Impl, const OFFSET: isize>() -> IWMDMProgress2Vtbl {
        unsafe extern "system" fn End2<Impl: IWMDMProgress2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hrcompletioncode: ::windows::core::HRESULT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).End2(hrcompletioncode) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IWMDMProgress2>, ::windows::core::GetTrustLevel, End2::<Impl, OFFSET>)
    }
}
pub trait IWMDMProgress3Impl: Sized + IWMDMProgress2Impl + IWMDMProgressImpl {
    fn Begin3();
    fn Progress3();
    fn End3();
}
impl ::windows::core::RuntimeName for IWMDMProgress3 {
    const NAME: &'static str = "Windows.Win32.Media.DeviceManager.IWMDMProgress3";
}
impl IWMDMProgress3Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMDMProgress3Impl, const OFFSET: isize>() -> IWMDMProgress3Vtbl {
        unsafe extern "system" fn Begin3<Impl: IWMDMProgress3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, eventid: ::windows::core::GUID, dwestimatedticks: u32, pcontext: *mut OPAQUECOMMAND) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Begin3(&*(&eventid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), dwestimatedticks, &*(&pcontext as *const <OPAQUECOMMAND as ::windows::core::Abi>::Abi as *const <OPAQUECOMMAND as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Progress3<Impl: IWMDMProgress3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, eventid: ::windows::core::GUID, dwtranspiredticks: u32, pcontext: *mut OPAQUECOMMAND) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Progress3(&*(&eventid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), dwtranspiredticks, &*(&pcontext as *const <OPAQUECOMMAND as ::windows::core::Abi>::Abi as *const <OPAQUECOMMAND as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn End3<Impl: IWMDMProgress3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, eventid: ::windows::core::GUID, hrcompletioncode: ::windows::core::HRESULT, pcontext: *mut OPAQUECOMMAND) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).End3(&*(&eventid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), hrcompletioncode, &*(&pcontext as *const <OPAQUECOMMAND as ::windows::core::Abi>::Abi as *const <OPAQUECOMMAND as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IWMDMProgress3>, ::windows::core::GetTrustLevel, Begin3::<Impl, OFFSET>, Progress3::<Impl, OFFSET>, End3::<Impl, OFFSET>)
    }
}
pub trait IWMDMRevokedImpl: Sized {
    fn GetRevocationURL();
}
impl ::windows::core::RuntimeName for IWMDMRevoked {
    const NAME: &'static str = "Windows.Win32.Media.DeviceManager.IWMDMRevoked";
}
impl IWMDMRevokedVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMDMRevokedImpl, const OFFSET: isize>() -> IWMDMRevokedVtbl {
        unsafe extern "system" fn GetRevocationURL<Impl: IWMDMRevokedImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppwszrevocationurl: *mut super::super::Foundation::PWSTR, pdwbufferlen: *mut u32, pdwrevokedbitflag: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetRevocationURL(&*(&ppwszrevocationurl as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), pdwbufferlen, ::core::mem::transmute_copy(&pdwrevokedbitflag)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IWMDMRevoked>, ::windows::core::GetTrustLevel, GetRevocationURL::<Impl, OFFSET>)
    }
}
pub trait IWMDMStorageImpl: Sized {
    fn SetAttributes();
    fn GetStorageGlobals();
    fn GetAttributes();
    fn GetName();
    fn GetDate();
    fn GetSize();
    fn GetRights();
    fn EnumStorage();
    fn SendOpaqueCommand();
}
impl ::windows::core::RuntimeName for IWMDMStorage {
    const NAME: &'static str = "Windows.Win32.Media.DeviceManager.IWMDMStorage";
}
impl IWMDMStorageVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMDMStorageImpl, const OFFSET: isize>() -> IWMDMStorageVtbl {
        unsafe extern "system" fn SetAttributes<Impl: IWMDMStorageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwattributes: u32, pformat: *const _WAVEFORMATEX) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetAttributes(dwattributes, &*(&pformat as *const <_WAVEFORMATEX as ::windows::core::Abi>::Abi as *const <_WAVEFORMATEX as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetStorageGlobals<Impl: IWMDMStorageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppstorageglobals: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetStorageGlobals(::core::mem::transmute_copy(&ppstorageglobals)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetAttributes<Impl: IWMDMStorageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwattributes: *mut u32, pformat: *mut _WAVEFORMATEX) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetAttributes(::core::mem::transmute_copy(&pdwattributes), &*(&pformat as *const <_WAVEFORMATEX as ::windows::core::Abi>::Abi as *const <_WAVEFORMATEX as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetName<Impl: IWMDMStorageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwszname: super::super::Foundation::PWSTR, nmaxchars: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetName(::core::mem::transmute_copy(&pwszname), nmaxchars) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDate<Impl: IWMDMStorageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdatetimeutc: *mut WMDMDATETIME) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDate(::core::mem::transmute_copy(&pdatetimeutc)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSize<Impl: IWMDMStorageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwsizelow: *mut u32, pdwsizehigh: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetSize(::core::mem::transmute_copy(&pdwsizelow), ::core::mem::transmute_copy(&pdwsizehigh)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetRights<Impl: IWMDMStorageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pprights: *mut *mut WMDMRIGHTS, pnrightscount: *mut u32, abmac: *mut u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetRights(::core::mem::transmute_copy(&pprights), ::core::mem::transmute_copy(&pnrightscount), abmac) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnumStorage<Impl: IWMDMStorageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, penumstorage: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EnumStorage(::core::mem::transmute_copy(&penumstorage)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SendOpaqueCommand<Impl: IWMDMStorageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcommand: *mut OPAQUECOMMAND) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SendOpaqueCommand(&*(&pcommand as *const <OPAQUECOMMAND as ::windows::core::Abi>::Abi as *const <OPAQUECOMMAND as ::windows::core::DefaultType>::DefaultType)) {
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
            ::windows::core::GetRuntimeClassName::<IWMDMStorage>,
            ::windows::core::GetTrustLevel,
            SetAttributes::<Impl, OFFSET>,
            GetStorageGlobals::<Impl, OFFSET>,
            GetAttributes::<Impl, OFFSET>,
            GetName::<Impl, OFFSET>,
            GetDate::<Impl, OFFSET>,
            GetSize::<Impl, OFFSET>,
            GetRights::<Impl, OFFSET>,
            EnumStorage::<Impl, OFFSET>,
            SendOpaqueCommand::<Impl, OFFSET>,
        )
    }
}
pub trait IWMDMStorage2Impl: Sized + IWMDMStorageImpl {
    fn GetStorage();
    fn SetAttributes2();
    fn GetAttributes2();
}
impl ::windows::core::RuntimeName for IWMDMStorage2 {
    const NAME: &'static str = "Windows.Win32.Media.DeviceManager.IWMDMStorage2";
}
impl IWMDMStorage2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMDMStorage2Impl, const OFFSET: isize>() -> IWMDMStorage2Vtbl {
        unsafe extern "system" fn GetStorage<Impl: IWMDMStorage2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszstoragename: super::super::Foundation::PWSTR, ppstorage: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetStorage(&*(&pszstoragename as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&ppstorage)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAttributes2<Impl: IWMDMStorage2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwattributes: u32, dwattributesex: u32, pformat: *const _WAVEFORMATEX, pvideoformat: *const _VIDEOINFOHEADER) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetAttributes2(dwattributes, dwattributesex, &*(&pformat as *const <_WAVEFORMATEX as ::windows::core::Abi>::Abi as *const <_WAVEFORMATEX as ::windows::core::DefaultType>::DefaultType), &*(&pvideoformat as *const <_VIDEOINFOHEADER as ::windows::core::Abi>::Abi as *const <_VIDEOINFOHEADER as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetAttributes2<Impl: IWMDMStorage2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwattributes: *mut u32, pdwattributesex: *mut u32, paudioformat: *mut _WAVEFORMATEX, pvideoformat: *mut _VIDEOINFOHEADER) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetAttributes2(::core::mem::transmute_copy(&pdwattributes), ::core::mem::transmute_copy(&pdwattributesex), &*(&paudioformat as *const <_WAVEFORMATEX as ::windows::core::Abi>::Abi as *const <_WAVEFORMATEX as ::windows::core::DefaultType>::DefaultType), &*(&pvideoformat as *const <_VIDEOINFOHEADER as ::windows::core::Abi>::Abi as *const <_VIDEOINFOHEADER as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IWMDMStorage2>, ::windows::core::GetTrustLevel, GetStorage::<Impl, OFFSET>, SetAttributes2::<Impl, OFFSET>, GetAttributes2::<Impl, OFFSET>)
    }
}
pub trait IWMDMStorage3Impl: Sized + IWMDMStorage2Impl + IWMDMStorageImpl {
    fn GetMetadata();
    fn SetMetadata();
    fn CreateEmptyMetadataObject();
    fn SetEnumPreference();
}
impl ::windows::core::RuntimeName for IWMDMStorage3 {
    const NAME: &'static str = "Windows.Win32.Media.DeviceManager.IWMDMStorage3";
}
impl IWMDMStorage3Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMDMStorage3Impl, const OFFSET: isize>() -> IWMDMStorage3Vtbl {
        unsafe extern "system" fn GetMetadata<Impl: IWMDMStorage3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppmetadata: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetMetadata(::core::mem::transmute_copy(&ppmetadata)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMetadata<Impl: IWMDMStorage3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pmetadata: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetMetadata(&*(&pmetadata as *const <IWMDMMetaData as ::windows::core::Abi>::Abi as *const <IWMDMMetaData as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateEmptyMetadataObject<Impl: IWMDMStorage3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppmetadata: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateEmptyMetadataObject(::core::mem::transmute_copy(&ppmetadata)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetEnumPreference<Impl: IWMDMStorage3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pmode: *mut WMDM_STORAGE_ENUM_MODE, nviews: u32, pviews: *const WMDMMetadataView) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetEnumPreference(pmode, nviews, &*(&pviews as *const <WMDMMetadataView as ::windows::core::Abi>::Abi as *const <WMDMMetadataView as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IWMDMStorage3>, ::windows::core::GetTrustLevel, GetMetadata::<Impl, OFFSET>, SetMetadata::<Impl, OFFSET>, CreateEmptyMetadataObject::<Impl, OFFSET>, SetEnumPreference::<Impl, OFFSET>)
    }
}
pub trait IWMDMStorage4Impl: Sized + IWMDMStorage3Impl + IWMDMStorage2Impl + IWMDMStorageImpl {
    fn SetReferences();
    fn GetReferences();
    fn GetRightsWithProgress();
    fn GetSpecifiedMetadata();
    fn FindStorage();
    fn GetParent();
}
impl ::windows::core::RuntimeName for IWMDMStorage4 {
    const NAME: &'static str = "Windows.Win32.Media.DeviceManager.IWMDMStorage4";
}
impl IWMDMStorage4Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMDMStorage4Impl, const OFFSET: isize>() -> IWMDMStorage4Vtbl {
        unsafe extern "system" fn SetReferences<Impl: IWMDMStorage4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwrefs: u32, ppiwmdmstorage: *const ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetReferences(dwrefs, &*(&ppiwmdmstorage as *const <IWMDMStorage as ::windows::core::Abi>::Abi as *const <IWMDMStorage as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetReferences<Impl: IWMDMStorage4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwrefs: *mut u32, pppiwmdmstorage: *mut *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetReferences(::core::mem::transmute_copy(&pdwrefs), ::core::mem::transmute_copy(&pppiwmdmstorage)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetRightsWithProgress<Impl: IWMDMStorage4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, piprogresscallback: ::windows::core::RawPtr, pprights: *mut *mut WMDMRIGHTS, pnrightscount: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetRightsWithProgress(&*(&piprogresscallback as *const <IWMDMProgress3 as ::windows::core::Abi>::Abi as *const <IWMDMProgress3 as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&pprights), ::core::mem::transmute_copy(&pnrightscount)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSpecifiedMetadata<Impl: IWMDMStorage4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cproperties: u32, ppwszpropnames: *const super::super::Foundation::PWSTR, ppmetadata: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetSpecifiedMetadata(cproperties, &*(&ppwszpropnames as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&ppmetadata)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FindStorage<Impl: IWMDMStorage4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, findscope: WMDM_FIND_SCOPE, pwszuniqueid: super::super::Foundation::PWSTR, ppstorage: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FindStorage(findscope, &*(&pwszuniqueid as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&ppstorage)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetParent<Impl: IWMDMStorage4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppstorage: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetParent(::core::mem::transmute_copy(&ppstorage)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IWMDMStorage4>, ::windows::core::GetTrustLevel, SetReferences::<Impl, OFFSET>, GetReferences::<Impl, OFFSET>, GetRightsWithProgress::<Impl, OFFSET>, GetSpecifiedMetadata::<Impl, OFFSET>, FindStorage::<Impl, OFFSET>, GetParent::<Impl, OFFSET>)
    }
}
pub trait IWMDMStorageControlImpl: Sized {
    fn Insert();
    fn Delete();
    fn Rename();
    fn Read();
    fn Move();
}
impl ::windows::core::RuntimeName for IWMDMStorageControl {
    const NAME: &'static str = "Windows.Win32.Media.DeviceManager.IWMDMStorageControl";
}
impl IWMDMStorageControlVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMDMStorageControlImpl, const OFFSET: isize>() -> IWMDMStorageControlVtbl {
        unsafe extern "system" fn Insert<Impl: IWMDMStorageControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fumode: u32, pwszfile: super::super::Foundation::PWSTR, poperation: ::windows::core::RawPtr, pprogress: ::windows::core::RawPtr, ppnewobject: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Insert(
                fumode,
                &*(&pwszfile as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&poperation as *const <IWMDMOperation as ::windows::core::Abi>::Abi as *const <IWMDMOperation as ::windows::core::DefaultType>::DefaultType),
                &*(&pprogress as *const <IWMDMProgress as ::windows::core::Abi>::Abi as *const <IWMDMProgress as ::windows::core::DefaultType>::DefaultType),
                ::core::mem::transmute_copy(&ppnewobject),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Delete<Impl: IWMDMStorageControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fumode: u32, pprogress: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Delete(fumode, &*(&pprogress as *const <IWMDMProgress as ::windows::core::Abi>::Abi as *const <IWMDMProgress as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Rename<Impl: IWMDMStorageControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fumode: u32, pwsznewname: super::super::Foundation::PWSTR, pprogress: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Rename(fumode, &*(&pwsznewname as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), &*(&pprogress as *const <IWMDMProgress as ::windows::core::Abi>::Abi as *const <IWMDMProgress as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Read<Impl: IWMDMStorageControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fumode: u32, pwszfile: super::super::Foundation::PWSTR, pprogress: ::windows::core::RawPtr, poperation: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Read(fumode, &*(&pwszfile as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), &*(&pprogress as *const <IWMDMProgress as ::windows::core::Abi>::Abi as *const <IWMDMProgress as ::windows::core::DefaultType>::DefaultType), &*(&poperation as *const <IWMDMOperation as ::windows::core::Abi>::Abi as *const <IWMDMOperation as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Move<Impl: IWMDMStorageControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fumode: u32, ptargetobject: ::windows::core::RawPtr, pprogress: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Move(fumode, &*(&ptargetobject as *const <IWMDMStorage as ::windows::core::Abi>::Abi as *const <IWMDMStorage as ::windows::core::DefaultType>::DefaultType), &*(&pprogress as *const <IWMDMProgress as ::windows::core::Abi>::Abi as *const <IWMDMProgress as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IWMDMStorageControl>, ::windows::core::GetTrustLevel, Insert::<Impl, OFFSET>, Delete::<Impl, OFFSET>, Rename::<Impl, OFFSET>, Read::<Impl, OFFSET>, Move::<Impl, OFFSET>)
    }
}
pub trait IWMDMStorageControl2Impl: Sized + IWMDMStorageControlImpl {
    fn Insert2();
}
impl ::windows::core::RuntimeName for IWMDMStorageControl2 {
    const NAME: &'static str = "Windows.Win32.Media.DeviceManager.IWMDMStorageControl2";
}
impl IWMDMStorageControl2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMDMStorageControl2Impl, const OFFSET: isize>() -> IWMDMStorageControl2Vtbl {
        unsafe extern "system" fn Insert2<Impl: IWMDMStorageControl2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fumode: u32, pwszfilesource: super::super::Foundation::PWSTR, pwszfiledest: super::super::Foundation::PWSTR, poperation: ::windows::core::RawPtr, pprogress: ::windows::core::RawPtr, punknown: *mut ::core::ffi::c_void, ppnewobject: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Insert2(
                fumode,
                &*(&pwszfilesource as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&pwszfiledest as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&poperation as *const <IWMDMOperation as ::windows::core::Abi>::Abi as *const <IWMDMOperation as ::windows::core::DefaultType>::DefaultType),
                &*(&pprogress as *const <IWMDMProgress as ::windows::core::Abi>::Abi as *const <IWMDMProgress as ::windows::core::DefaultType>::DefaultType),
                &*(&punknown as *const <::windows::core::IUnknown as ::windows::core::Abi>::Abi as *const <::windows::core::IUnknown as ::windows::core::DefaultType>::DefaultType),
                &*(&ppnewobject as *const <IWMDMStorage as ::windows::core::Abi>::Abi as *const <IWMDMStorage as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IWMDMStorageControl2>, ::windows::core::GetTrustLevel, Insert2::<Impl, OFFSET>)
    }
}
pub trait IWMDMStorageControl3Impl: Sized + IWMDMStorageControl2Impl + IWMDMStorageControlImpl {
    fn Insert3();
}
impl ::windows::core::RuntimeName for IWMDMStorageControl3 {
    const NAME: &'static str = "Windows.Win32.Media.DeviceManager.IWMDMStorageControl3";
}
impl IWMDMStorageControl3Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMDMStorageControl3Impl, const OFFSET: isize>() -> IWMDMStorageControl3Vtbl {
        unsafe extern "system" fn Insert3<Impl: IWMDMStorageControl3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fumode: u32, futype: u32, pwszfilesource: super::super::Foundation::PWSTR, pwszfiledest: super::super::Foundation::PWSTR, poperation: ::windows::core::RawPtr, pprogress: ::windows::core::RawPtr, pmetadata: ::windows::core::RawPtr, punknown: *mut ::core::ffi::c_void, ppnewobject: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Insert3(
                fumode,
                futype,
                &*(&pwszfilesource as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&pwszfiledest as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&poperation as *const <IWMDMOperation as ::windows::core::Abi>::Abi as *const <IWMDMOperation as ::windows::core::DefaultType>::DefaultType),
                &*(&pprogress as *const <IWMDMProgress as ::windows::core::Abi>::Abi as *const <IWMDMProgress as ::windows::core::DefaultType>::DefaultType),
                &*(&pmetadata as *const <IWMDMMetaData as ::windows::core::Abi>::Abi as *const <IWMDMMetaData as ::windows::core::DefaultType>::DefaultType),
                &*(&punknown as *const <::windows::core::IUnknown as ::windows::core::Abi>::Abi as *const <::windows::core::IUnknown as ::windows::core::DefaultType>::DefaultType),
                &*(&ppnewobject as *const <IWMDMStorage as ::windows::core::Abi>::Abi as *const <IWMDMStorage as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IWMDMStorageControl3>, ::windows::core::GetTrustLevel, Insert3::<Impl, OFFSET>)
    }
}
pub trait IWMDMStorageGlobalsImpl: Sized {
    fn GetCapabilities();
    fn GetSerialNumber();
    fn GetTotalSize();
    fn GetTotalFree();
    fn GetTotalBad();
    fn GetStatus();
    fn Initialize();
}
impl ::windows::core::RuntimeName for IWMDMStorageGlobals {
    const NAME: &'static str = "Windows.Win32.Media.DeviceManager.IWMDMStorageGlobals";
}
impl IWMDMStorageGlobalsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMDMStorageGlobalsImpl, const OFFSET: isize>() -> IWMDMStorageGlobalsVtbl {
        unsafe extern "system" fn GetCapabilities<Impl: IWMDMStorageGlobalsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwcapabilities: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetCapabilities(::core::mem::transmute_copy(&pdwcapabilities)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSerialNumber<Impl: IWMDMStorageGlobalsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pserialnum: *mut WMDMID, abmac: *mut u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetSerialNumber(::core::mem::transmute_copy(&pserialnum), abmac) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetTotalSize<Impl: IWMDMStorageGlobalsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwtotalsizelow: *mut u32, pdwtotalsizehigh: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetTotalSize(::core::mem::transmute_copy(&pdwtotalsizelow), ::core::mem::transmute_copy(&pdwtotalsizehigh)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetTotalFree<Impl: IWMDMStorageGlobalsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwfreelow: *mut u32, pdwfreehigh: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetTotalFree(::core::mem::transmute_copy(&pdwfreelow), ::core::mem::transmute_copy(&pdwfreehigh)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetTotalBad<Impl: IWMDMStorageGlobalsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwbadlow: *mut u32, pdwbadhigh: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetTotalBad(::core::mem::transmute_copy(&pdwbadlow), ::core::mem::transmute_copy(&pdwbadhigh)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetStatus<Impl: IWMDMStorageGlobalsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwstatus: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetStatus(::core::mem::transmute_copy(&pdwstatus)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Initialize<Impl: IWMDMStorageGlobalsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fumode: u32, pprogress: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Initialize(fumode, &*(&pprogress as *const <IWMDMProgress as ::windows::core::Abi>::Abi as *const <IWMDMProgress as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IWMDMStorageGlobals>, ::windows::core::GetTrustLevel, GetCapabilities::<Impl, OFFSET>, GetSerialNumber::<Impl, OFFSET>, GetTotalSize::<Impl, OFFSET>, GetTotalFree::<Impl, OFFSET>, GetTotalBad::<Impl, OFFSET>, GetStatus::<Impl, OFFSET>, Initialize::<Impl, OFFSET>)
    }
}
pub trait IWMDeviceManagerImpl: Sized {
    fn GetRevision();
    fn GetDeviceCount();
    fn EnumDevices();
}
impl ::windows::core::RuntimeName for IWMDeviceManager {
    const NAME: &'static str = "Windows.Win32.Media.DeviceManager.IWMDeviceManager";
}
impl IWMDeviceManagerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMDeviceManagerImpl, const OFFSET: isize>() -> IWMDeviceManagerVtbl {
        unsafe extern "system" fn GetRevision<Impl: IWMDeviceManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwrevision: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetRevision(::core::mem::transmute_copy(&pdwrevision)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDeviceCount<Impl: IWMDeviceManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwcount: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDeviceCount(::core::mem::transmute_copy(&pdwcount)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnumDevices<Impl: IWMDeviceManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenumdevice: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EnumDevices(::core::mem::transmute_copy(&ppenumdevice)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IWMDeviceManager>, ::windows::core::GetTrustLevel, GetRevision::<Impl, OFFSET>, GetDeviceCount::<Impl, OFFSET>, EnumDevices::<Impl, OFFSET>)
    }
}
pub trait IWMDeviceManager2Impl: Sized + IWMDeviceManagerImpl {
    fn GetDeviceFromCanonicalName();
    fn EnumDevices2();
    fn Reinitialize();
}
impl ::windows::core::RuntimeName for IWMDeviceManager2 {
    const NAME: &'static str = "Windows.Win32.Media.DeviceManager.IWMDeviceManager2";
}
impl IWMDeviceManager2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMDeviceManager2Impl, const OFFSET: isize>() -> IWMDeviceManager2Vtbl {
        unsafe extern "system" fn GetDeviceFromCanonicalName<Impl: IWMDeviceManager2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwszcanonicalname: super::super::Foundation::PWSTR, ppdevice: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDeviceFromCanonicalName(&*(&pwszcanonicalname as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&ppdevice)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnumDevices2<Impl: IWMDeviceManager2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenumdevice: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EnumDevices2(::core::mem::transmute_copy(&ppenumdevice)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Reinitialize<Impl: IWMDeviceManager2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Reinitialize() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IWMDeviceManager2>, ::windows::core::GetTrustLevel, GetDeviceFromCanonicalName::<Impl, OFFSET>, EnumDevices2::<Impl, OFFSET>, Reinitialize::<Impl, OFFSET>)
    }
}
pub trait IWMDeviceManager3Impl: Sized + IWMDeviceManager2Impl + IWMDeviceManagerImpl {
    fn SetDeviceEnumPreference();
}
impl ::windows::core::RuntimeName for IWMDeviceManager3 {
    const NAME: &'static str = "Windows.Win32.Media.DeviceManager.IWMDeviceManager3";
}
impl IWMDeviceManager3Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMDeviceManager3Impl, const OFFSET: isize>() -> IWMDeviceManager3Vtbl {
        unsafe extern "system" fn SetDeviceEnumPreference<Impl: IWMDeviceManager3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwenumpref: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetDeviceEnumPreference(dwenumpref) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IWMDeviceManager3>, ::windows::core::GetTrustLevel, SetDeviceEnumPreference::<Impl, OFFSET>)
    }
}
