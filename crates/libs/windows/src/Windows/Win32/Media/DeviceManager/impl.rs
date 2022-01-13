pub trait IComponentAuthenticateImpl: Sized {
    fn SACAuth(&mut self, dwprotocolid: u32, dwpass: u32, pbdatain: *const u8, dwdatainlen: u32, ppbdataout: *mut *mut u8, pdwdataoutlen: *mut u32) -> ::windows::core::Result<()>;
    fn SACGetProtocols(&mut self, ppdwprotocols: *mut *mut u32, pdwprotocolcount: *mut u32) -> ::windows::core::Result<()>;
}
impl IComponentAuthenticateVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IComponentAuthenticateImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IComponentAuthenticateVtbl {
        unsafe extern "system" fn SACAuth<Impl: IComponentAuthenticateImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwprotocolid: u32, dwpass: u32, pbdatain: *const u8, dwdatainlen: u32, ppbdataout: *mut *mut u8, pdwdataoutlen: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SACAuth(::core::mem::transmute_copy(&dwprotocolid), ::core::mem::transmute_copy(&dwpass), ::core::mem::transmute_copy(&pbdatain), ::core::mem::transmute_copy(&dwdatainlen), ::core::mem::transmute_copy(&ppbdataout), ::core::mem::transmute_copy(&pdwdataoutlen)).into()
        }
        unsafe extern "system" fn SACGetProtocols<Impl: IComponentAuthenticateImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppdwprotocols: *mut *mut u32, pdwprotocolcount: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SACGetProtocols(::core::mem::transmute_copy(&ppdwprotocols), ::core::mem::transmute_copy(&pdwprotocolcount)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            SACAuth: SACAuth::<Impl, IMPL_OFFSET>,
            SACGetProtocols: SACGetProtocols::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IComponentAuthenticate as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IMDSPDeviceImpl: Sized {
    fn GetName(&mut self, pwszname: super::super::Foundation::PWSTR, nmaxchars: u32) -> ::windows::core::Result<()>;
    fn GetManufacturer(&mut self, pwszname: super::super::Foundation::PWSTR, nmaxchars: u32) -> ::windows::core::Result<()>;
    fn GetVersion(&mut self) -> ::windows::core::Result<u32>;
    fn GetType(&mut self) -> ::windows::core::Result<u32>;
    fn GetSerialNumber(&mut self, pserialnumber: *mut WMDMID, abmac: *mut u8) -> ::windows::core::Result<()>;
    fn GetPowerSource(&mut self, pdwpowersource: *mut u32, pdwpercentremaining: *mut u32) -> ::windows::core::Result<()>;
    fn GetStatus(&mut self) -> ::windows::core::Result<u32>;
    fn GetDeviceIcon(&mut self) -> ::windows::core::Result<u32>;
    fn EnumStorage(&mut self) -> ::windows::core::Result<IMDSPEnumStorage>;
    fn GetFormatSupport(&mut self, pformatex: *mut *mut _WAVEFORMATEX, pnformatcount: *mut u32, pppwszmimetype: *mut *mut super::super::Foundation::PWSTR, pnmimetypecount: *mut u32) -> ::windows::core::Result<()>;
    fn SendOpaqueCommand(&mut self, pcommand: *mut OPAQUECOMMAND) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IMDSPDeviceVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMDSPDeviceImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMDSPDeviceVtbl {
        unsafe extern "system" fn GetName<Impl: IMDSPDeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwszname: super::super::Foundation::PWSTR, nmaxchars: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetName(::core::mem::transmute_copy(&pwszname), ::core::mem::transmute_copy(&nmaxchars)).into()
        }
        unsafe extern "system" fn GetManufacturer<Impl: IMDSPDeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwszname: super::super::Foundation::PWSTR, nmaxchars: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetManufacturer(::core::mem::transmute_copy(&pwszname), ::core::mem::transmute_copy(&nmaxchars)).into()
        }
        unsafe extern "system" fn GetVersion<Impl: IMDSPDeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwversion: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetVersion() {
                ::core::result::Result::Ok(ok__) => {
                    *pdwversion = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetType<Impl: IMDSPDeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwtype: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetType() {
                ::core::result::Result::Ok(ok__) => {
                    *pdwtype = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSerialNumber<Impl: IMDSPDeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pserialnumber: *mut WMDMID, abmac: *mut u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetSerialNumber(::core::mem::transmute_copy(&pserialnumber), ::core::mem::transmute_copy(&abmac)).into()
        }
        unsafe extern "system" fn GetPowerSource<Impl: IMDSPDeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwpowersource: *mut u32, pdwpercentremaining: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetPowerSource(::core::mem::transmute_copy(&pdwpowersource), ::core::mem::transmute_copy(&pdwpercentremaining)).into()
        }
        unsafe extern "system" fn GetStatus<Impl: IMDSPDeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwstatus: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetStatus() {
                ::core::result::Result::Ok(ok__) => {
                    *pdwstatus = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDeviceIcon<Impl: IMDSPDeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hicon: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDeviceIcon() {
                ::core::result::Result::Ok(ok__) => {
                    *hicon = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnumStorage<Impl: IMDSPDeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenumstorage: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EnumStorage() {
                ::core::result::Result::Ok(ok__) => {
                    *ppenumstorage = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFormatSupport<Impl: IMDSPDeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pformatex: *mut *mut _WAVEFORMATEX, pnformatcount: *mut u32, pppwszmimetype: *mut *mut super::super::Foundation::PWSTR, pnmimetypecount: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetFormatSupport(::core::mem::transmute_copy(&pformatex), ::core::mem::transmute_copy(&pnformatcount), ::core::mem::transmute_copy(&pppwszmimetype), ::core::mem::transmute_copy(&pnmimetypecount)).into()
        }
        unsafe extern "system" fn SendOpaqueCommand<Impl: IMDSPDeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcommand: *mut OPAQUECOMMAND) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SendOpaqueCommand(::core::mem::transmute_copy(&pcommand)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            GetName: GetName::<Impl, IMPL_OFFSET>,
            GetManufacturer: GetManufacturer::<Impl, IMPL_OFFSET>,
            GetVersion: GetVersion::<Impl, IMPL_OFFSET>,
            GetType: GetType::<Impl, IMPL_OFFSET>,
            GetSerialNumber: GetSerialNumber::<Impl, IMPL_OFFSET>,
            GetPowerSource: GetPowerSource::<Impl, IMPL_OFFSET>,
            GetStatus: GetStatus::<Impl, IMPL_OFFSET>,
            GetDeviceIcon: GetDeviceIcon::<Impl, IMPL_OFFSET>,
            EnumStorage: EnumStorage::<Impl, IMPL_OFFSET>,
            GetFormatSupport: GetFormatSupport::<Impl, IMPL_OFFSET>,
            SendOpaqueCommand: SendOpaqueCommand::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMDSPDevice as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
pub trait IMDSPDevice2Impl: Sized + IMDSPDeviceImpl {
    fn GetStorage(&mut self, pszstoragename: super::super::Foundation::PWSTR) -> ::windows::core::Result<IMDSPStorage>;
    fn GetFormatSupport2(&mut self, dwflags: u32, ppaudioformatex: *mut *mut _WAVEFORMATEX, pnaudioformatcount: *mut u32, ppvideoformatex: *mut *mut _VIDEOINFOHEADER, pnvideoformatcount: *mut u32, ppfiletype: *mut *mut WMFILECAPABILITIES, pnfiletypecount: *mut u32) -> ::windows::core::Result<()>;
    fn GetSpecifyPropertyPages(&mut self, ppspecifyproppages: *mut ::core::option::Option<super::super::System::Ole::ISpecifyPropertyPages>, pppunknowns: *mut *mut ::core::option::Option<::windows::core::IUnknown>, pcunks: *mut u32) -> ::windows::core::Result<()>;
    fn GetCanonicalName(&mut self, pwszpnpname: super::super::Foundation::PWSTR, nmaxchars: u32) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
impl IMDSPDevice2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMDSPDevice2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMDSPDevice2Vtbl {
        unsafe extern "system" fn GetStorage<Impl: IMDSPDevice2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszstoragename: super::super::Foundation::PWSTR, ppstorage: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetStorage(::core::mem::transmute_copy(&pszstoragename)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppstorage = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFormatSupport2<Impl: IMDSPDevice2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwflags: u32, ppaudioformatex: *mut *mut _WAVEFORMATEX, pnaudioformatcount: *mut u32, ppvideoformatex: *mut *mut _VIDEOINFOHEADER, pnvideoformatcount: *mut u32, ppfiletype: *mut *mut WMFILECAPABILITIES, pnfiletypecount: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetFormatSupport2(::core::mem::transmute_copy(&dwflags), ::core::mem::transmute_copy(&ppaudioformatex), ::core::mem::transmute_copy(&pnaudioformatcount), ::core::mem::transmute_copy(&ppvideoformatex), ::core::mem::transmute_copy(&pnvideoformatcount), ::core::mem::transmute_copy(&ppfiletype), ::core::mem::transmute_copy(&pnfiletypecount)).into()
        }
        unsafe extern "system" fn GetSpecifyPropertyPages<Impl: IMDSPDevice2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppspecifyproppages: *mut ::windows::core::RawPtr, pppunknowns: *mut *mut *mut ::core::ffi::c_void, pcunks: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetSpecifyPropertyPages(::core::mem::transmute_copy(&ppspecifyproppages), ::core::mem::transmute_copy(&pppunknowns), ::core::mem::transmute_copy(&pcunks)).into()
        }
        unsafe extern "system" fn GetCanonicalName<Impl: IMDSPDevice2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwszpnpname: super::super::Foundation::PWSTR, nmaxchars: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetCanonicalName(::core::mem::transmute_copy(&pwszpnpname), ::core::mem::transmute_copy(&nmaxchars)).into()
        }
        Self {
            base: IMDSPDeviceVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            GetStorage: GetStorage::<Impl, IMPL_OFFSET>,
            GetFormatSupport2: GetFormatSupport2::<Impl, IMPL_OFFSET>,
            GetSpecifyPropertyPages: GetSpecifyPropertyPages::<Impl, IMPL_OFFSET>,
            GetCanonicalName: GetCanonicalName::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMDSPDevice2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Ole"))]
pub trait IMDSPDevice3Impl: Sized + IMDSPDeviceImpl + IMDSPDevice2Impl {
    fn GetProperty(&mut self, pwszpropname: super::super::Foundation::PWSTR) -> ::windows::core::Result<super::super::System::Com::StructuredStorage::PROPVARIANT>;
    fn SetProperty(&mut self, pwszpropname: super::super::Foundation::PWSTR, pvalue: *const super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows::core::Result<()>;
    fn GetFormatCapability(&mut self, format: WMDM_FORMATCODE) -> ::windows::core::Result<WMDM_FORMAT_CAPABILITY>;
    fn DeviceIoControl(&mut self, dwiocontrolcode: u32, lpinbuffer: *const u8, ninbuffersize: u32, lpoutbuffer: *mut u8, pnoutbuffersize: *mut u32) -> ::windows::core::Result<()>;
    fn FindStorage(&mut self, findscope: WMDM_FIND_SCOPE, pwszuniqueid: super::super::Foundation::PWSTR) -> ::windows::core::Result<IMDSPStorage>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Ole"))]
impl IMDSPDevice3Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMDSPDevice3Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMDSPDevice3Vtbl {
        unsafe extern "system" fn GetProperty<Impl: IMDSPDevice3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwszpropname: super::super::Foundation::PWSTR, pvalue: *mut super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetProperty(::core::mem::transmute_copy(&pwszpropname)) {
                ::core::result::Result::Ok(ok__) => {
                    *pvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetProperty<Impl: IMDSPDevice3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwszpropname: super::super::Foundation::PWSTR, pvalue: *const super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetProperty(::core::mem::transmute_copy(&pwszpropname), ::core::mem::transmute_copy(&pvalue)).into()
        }
        unsafe extern "system" fn GetFormatCapability<Impl: IMDSPDevice3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, format: WMDM_FORMATCODE, pformatsupport: *mut WMDM_FORMAT_CAPABILITY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetFormatCapability(::core::mem::transmute_copy(&format)) {
                ::core::result::Result::Ok(ok__) => {
                    *pformatsupport = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DeviceIoControl<Impl: IMDSPDevice3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwiocontrolcode: u32, lpinbuffer: *const u8, ninbuffersize: u32, lpoutbuffer: *mut u8, pnoutbuffersize: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).DeviceIoControl(::core::mem::transmute_copy(&dwiocontrolcode), ::core::mem::transmute_copy(&lpinbuffer), ::core::mem::transmute_copy(&ninbuffersize), ::core::mem::transmute_copy(&lpoutbuffer), ::core::mem::transmute_copy(&pnoutbuffersize)).into()
        }
        unsafe extern "system" fn FindStorage<Impl: IMDSPDevice3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, findscope: WMDM_FIND_SCOPE, pwszuniqueid: super::super::Foundation::PWSTR, ppstorage: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FindStorage(::core::mem::transmute_copy(&findscope), ::core::mem::transmute_copy(&pwszuniqueid)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppstorage = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: IMDSPDevice2Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            GetProperty: GetProperty::<Impl, IMPL_OFFSET>,
            SetProperty: SetProperty::<Impl, IMPL_OFFSET>,
            GetFormatCapability: GetFormatCapability::<Impl, IMPL_OFFSET>,
            DeviceIoControl: DeviceIoControl::<Impl, IMPL_OFFSET>,
            FindStorage: FindStorage::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMDSPDevice3 as ::windows::core::Interface>::IID
    }
}
pub trait IMDSPDeviceControlImpl: Sized {
    fn GetDCStatus(&mut self) -> ::windows::core::Result<u32>;
    fn GetCapabilities(&mut self) -> ::windows::core::Result<u32>;
    fn Play(&mut self) -> ::windows::core::Result<()>;
    fn Record(&mut self, pformat: *const _WAVEFORMATEX) -> ::windows::core::Result<()>;
    fn Pause(&mut self) -> ::windows::core::Result<()>;
    fn Resume(&mut self) -> ::windows::core::Result<()>;
    fn Stop(&mut self) -> ::windows::core::Result<()>;
    fn Seek(&mut self, fumode: u32, noffset: i32) -> ::windows::core::Result<()>;
}
impl IMDSPDeviceControlVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMDSPDeviceControlImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMDSPDeviceControlVtbl {
        unsafe extern "system" fn GetDCStatus<Impl: IMDSPDeviceControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwstatus: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDCStatus() {
                ::core::result::Result::Ok(ok__) => {
                    *pdwstatus = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCapabilities<Impl: IMDSPDeviceControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwcapabilitiesmask: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetCapabilities() {
                ::core::result::Result::Ok(ok__) => {
                    *pdwcapabilitiesmask = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Play<Impl: IMDSPDeviceControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Play().into()
        }
        unsafe extern "system" fn Record<Impl: IMDSPDeviceControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pformat: *const _WAVEFORMATEX) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Record(::core::mem::transmute_copy(&pformat)).into()
        }
        unsafe extern "system" fn Pause<Impl: IMDSPDeviceControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Pause().into()
        }
        unsafe extern "system" fn Resume<Impl: IMDSPDeviceControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Resume().into()
        }
        unsafe extern "system" fn Stop<Impl: IMDSPDeviceControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Stop().into()
        }
        unsafe extern "system" fn Seek<Impl: IMDSPDeviceControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fumode: u32, noffset: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Seek(::core::mem::transmute_copy(&fumode), ::core::mem::transmute_copy(&noffset)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            GetDCStatus: GetDCStatus::<Impl, IMPL_OFFSET>,
            GetCapabilities: GetCapabilities::<Impl, IMPL_OFFSET>,
            Play: Play::<Impl, IMPL_OFFSET>,
            Record: Record::<Impl, IMPL_OFFSET>,
            Pause: Pause::<Impl, IMPL_OFFSET>,
            Resume: Resume::<Impl, IMPL_OFFSET>,
            Stop: Stop::<Impl, IMPL_OFFSET>,
            Seek: Seek::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMDSPDeviceControl as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IMDSPDirectTransferImpl: Sized {
    fn TransferToDevice(&mut self, pwszsourcefilepath: super::super::Foundation::PWSTR, psourceoperation: ::core::option::Option<IWMDMOperation>, fuflags: u32, pwszdestinationname: super::super::Foundation::PWSTR, psourcemetadata: ::core::option::Option<IWMDMMetaData>, ptransferprogress: ::core::option::Option<IWMDMProgress>) -> ::windows::core::Result<IMDSPStorage>;
}
#[cfg(feature = "Win32_Foundation")]
impl IMDSPDirectTransferVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMDSPDirectTransferImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMDSPDirectTransferVtbl {
        unsafe extern "system" fn TransferToDevice<Impl: IMDSPDirectTransferImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwszsourcefilepath: super::super::Foundation::PWSTR, psourceoperation: ::windows::core::RawPtr, fuflags: u32, pwszdestinationname: super::super::Foundation::PWSTR, psourcemetadata: ::windows::core::RawPtr, ptransferprogress: ::windows::core::RawPtr, ppnewobject: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TransferToDevice(::core::mem::transmute_copy(&pwszsourcefilepath), ::core::mem::transmute(&psourceoperation), ::core::mem::transmute_copy(&fuflags), ::core::mem::transmute_copy(&pwszdestinationname), ::core::mem::transmute(&psourcemetadata), ::core::mem::transmute(&ptransferprogress)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppnewobject = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), TransferToDevice: TransferToDevice::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMDSPDirectTransfer as ::windows::core::Interface>::IID
    }
}
pub trait IMDSPEnumDeviceImpl: Sized {
    fn Next(&mut self, celt: u32, ppdevice: *mut ::core::option::Option<IMDSPDevice>, pceltfetched: *mut u32) -> ::windows::core::Result<()>;
    fn Skip(&mut self, celt: u32) -> ::windows::core::Result<u32>;
    fn Reset(&mut self) -> ::windows::core::Result<()>;
    fn Clone(&mut self) -> ::windows::core::Result<IMDSPEnumDevice>;
}
impl IMDSPEnumDeviceVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMDSPEnumDeviceImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMDSPEnumDeviceVtbl {
        unsafe extern "system" fn Next<Impl: IMDSPEnumDeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32, ppdevice: *mut ::windows::core::RawPtr, pceltfetched: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Next(::core::mem::transmute_copy(&celt), ::core::mem::transmute_copy(&ppdevice), ::core::mem::transmute_copy(&pceltfetched)).into()
        }
        unsafe extern "system" fn Skip<Impl: IMDSPEnumDeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32, pceltfetched: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Skip(::core::mem::transmute_copy(&celt)) {
                ::core::result::Result::Ok(ok__) => {
                    *pceltfetched = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Reset<Impl: IMDSPEnumDeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Reset().into()
        }
        unsafe extern "system" fn Clone<Impl: IMDSPEnumDeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenumdevice: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Clone() {
                ::core::result::Result::Ok(ok__) => {
                    *ppenumdevice = ::core::mem::transmute(ok__);
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
        iid == &<IMDSPEnumDevice as ::windows::core::Interface>::IID
    }
}
pub trait IMDSPEnumStorageImpl: Sized {
    fn Next(&mut self, celt: u32, ppstorage: *mut ::core::option::Option<IMDSPStorage>, pceltfetched: *mut u32) -> ::windows::core::Result<()>;
    fn Skip(&mut self, celt: u32) -> ::windows::core::Result<u32>;
    fn Reset(&mut self) -> ::windows::core::Result<()>;
    fn Clone(&mut self) -> ::windows::core::Result<IMDSPEnumStorage>;
}
impl IMDSPEnumStorageVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMDSPEnumStorageImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMDSPEnumStorageVtbl {
        unsafe extern "system" fn Next<Impl: IMDSPEnumStorageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32, ppstorage: *mut ::windows::core::RawPtr, pceltfetched: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Next(::core::mem::transmute_copy(&celt), ::core::mem::transmute_copy(&ppstorage), ::core::mem::transmute_copy(&pceltfetched)).into()
        }
        unsafe extern "system" fn Skip<Impl: IMDSPEnumStorageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32, pceltfetched: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Skip(::core::mem::transmute_copy(&celt)) {
                ::core::result::Result::Ok(ok__) => {
                    *pceltfetched = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Reset<Impl: IMDSPEnumStorageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Reset().into()
        }
        unsafe extern "system" fn Clone<Impl: IMDSPEnumStorageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenumstorage: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Clone() {
                ::core::result::Result::Ok(ok__) => {
                    *ppenumstorage = ::core::mem::transmute(ok__);
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
        iid == &<IMDSPEnumStorage as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IMDSPObjectImpl: Sized {
    fn Open(&mut self, fumode: u32) -> ::windows::core::Result<()>;
    fn Read(&mut self, pdata: *mut u8, pdwsize: *mut u32, abmac: *mut u8) -> ::windows::core::Result<()>;
    fn Write(&mut self, pdata: *const u8, pdwsize: *mut u32, abmac: *mut u8) -> ::windows::core::Result<()>;
    fn Delete(&mut self, fumode: u32, pprogress: ::core::option::Option<IWMDMProgress>) -> ::windows::core::Result<()>;
    fn Seek(&mut self, fuflags: u32, dwoffset: u32) -> ::windows::core::Result<()>;
    fn Rename(&mut self, pwsznewname: super::super::Foundation::PWSTR, pprogress: ::core::option::Option<IWMDMProgress>) -> ::windows::core::Result<()>;
    fn Move(&mut self, fumode: u32, pprogress: ::core::option::Option<IWMDMProgress>, ptarget: ::core::option::Option<IMDSPStorage>) -> ::windows::core::Result<()>;
    fn Close(&mut self) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IMDSPObjectVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMDSPObjectImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMDSPObjectVtbl {
        unsafe extern "system" fn Open<Impl: IMDSPObjectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fumode: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Open(::core::mem::transmute_copy(&fumode)).into()
        }
        unsafe extern "system" fn Read<Impl: IMDSPObjectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdata: *mut u8, pdwsize: *mut u32, abmac: *mut u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Read(::core::mem::transmute_copy(&pdata), ::core::mem::transmute_copy(&pdwsize), ::core::mem::transmute_copy(&abmac)).into()
        }
        unsafe extern "system" fn Write<Impl: IMDSPObjectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdata: *const u8, pdwsize: *mut u32, abmac: *mut u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Write(::core::mem::transmute_copy(&pdata), ::core::mem::transmute_copy(&pdwsize), ::core::mem::transmute_copy(&abmac)).into()
        }
        unsafe extern "system" fn Delete<Impl: IMDSPObjectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fumode: u32, pprogress: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Delete(::core::mem::transmute_copy(&fumode), ::core::mem::transmute(&pprogress)).into()
        }
        unsafe extern "system" fn Seek<Impl: IMDSPObjectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fuflags: u32, dwoffset: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Seek(::core::mem::transmute_copy(&fuflags), ::core::mem::transmute_copy(&dwoffset)).into()
        }
        unsafe extern "system" fn Rename<Impl: IMDSPObjectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwsznewname: super::super::Foundation::PWSTR, pprogress: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Rename(::core::mem::transmute_copy(&pwsznewname), ::core::mem::transmute(&pprogress)).into()
        }
        unsafe extern "system" fn Move<Impl: IMDSPObjectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fumode: u32, pprogress: ::windows::core::RawPtr, ptarget: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Move(::core::mem::transmute_copy(&fumode), ::core::mem::transmute(&pprogress), ::core::mem::transmute(&ptarget)).into()
        }
        unsafe extern "system" fn Close<Impl: IMDSPObjectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Close().into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            Open: Open::<Impl, IMPL_OFFSET>,
            Read: Read::<Impl, IMPL_OFFSET>,
            Write: Write::<Impl, IMPL_OFFSET>,
            Delete: Delete::<Impl, IMPL_OFFSET>,
            Seek: Seek::<Impl, IMPL_OFFSET>,
            Rename: Rename::<Impl, IMPL_OFFSET>,
            Move: Move::<Impl, IMPL_OFFSET>,
            Close: Close::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMDSPObject as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IMDSPObject2Impl: Sized + IMDSPObjectImpl {
    fn ReadOnClearChannel(&mut self, pdata: *mut u8, pdwsize: *mut u32) -> ::windows::core::Result<()>;
    fn WriteOnClearChannel(&mut self, pdata: *const u8, pdwsize: *mut u32) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IMDSPObject2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMDSPObject2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMDSPObject2Vtbl {
        unsafe extern "system" fn ReadOnClearChannel<Impl: IMDSPObject2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdata: *mut u8, pdwsize: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ReadOnClearChannel(::core::mem::transmute_copy(&pdata), ::core::mem::transmute_copy(&pdwsize)).into()
        }
        unsafe extern "system" fn WriteOnClearChannel<Impl: IMDSPObject2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdata: *const u8, pdwsize: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).WriteOnClearChannel(::core::mem::transmute_copy(&pdata), ::core::mem::transmute_copy(&pdwsize)).into()
        }
        Self {
            base: IMDSPObjectVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            ReadOnClearChannel: ReadOnClearChannel::<Impl, IMPL_OFFSET>,
            WriteOnClearChannel: WriteOnClearChannel::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMDSPObject2 as ::windows::core::Interface>::IID
    }
}
pub trait IMDSPObjectInfoImpl: Sized {
    fn GetPlayLength(&mut self) -> ::windows::core::Result<u32>;
    fn SetPlayLength(&mut self, dwlength: u32) -> ::windows::core::Result<()>;
    fn GetPlayOffset(&mut self) -> ::windows::core::Result<u32>;
    fn SetPlayOffset(&mut self, dwoffset: u32) -> ::windows::core::Result<()>;
    fn GetTotalLength(&mut self) -> ::windows::core::Result<u32>;
    fn GetLastPlayPosition(&mut self) -> ::windows::core::Result<u32>;
    fn GetLongestPlayPosition(&mut self) -> ::windows::core::Result<u32>;
}
impl IMDSPObjectInfoVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMDSPObjectInfoImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMDSPObjectInfoVtbl {
        unsafe extern "system" fn GetPlayLength<Impl: IMDSPObjectInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwlength: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetPlayLength() {
                ::core::result::Result::Ok(ok__) => {
                    *pdwlength = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPlayLength<Impl: IMDSPObjectInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwlength: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetPlayLength(::core::mem::transmute_copy(&dwlength)).into()
        }
        unsafe extern "system" fn GetPlayOffset<Impl: IMDSPObjectInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwoffset: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetPlayOffset() {
                ::core::result::Result::Ok(ok__) => {
                    *pdwoffset = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPlayOffset<Impl: IMDSPObjectInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwoffset: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetPlayOffset(::core::mem::transmute_copy(&dwoffset)).into()
        }
        unsafe extern "system" fn GetTotalLength<Impl: IMDSPObjectInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwlength: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetTotalLength() {
                ::core::result::Result::Ok(ok__) => {
                    *pdwlength = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetLastPlayPosition<Impl: IMDSPObjectInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwlastpos: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetLastPlayPosition() {
                ::core::result::Result::Ok(ok__) => {
                    *pdwlastpos = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetLongestPlayPosition<Impl: IMDSPObjectInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwlongestpos: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetLongestPlayPosition() {
                ::core::result::Result::Ok(ok__) => {
                    *pdwlongestpos = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            GetPlayLength: GetPlayLength::<Impl, IMPL_OFFSET>,
            SetPlayLength: SetPlayLength::<Impl, IMPL_OFFSET>,
            GetPlayOffset: GetPlayOffset::<Impl, IMPL_OFFSET>,
            SetPlayOffset: SetPlayOffset::<Impl, IMPL_OFFSET>,
            GetTotalLength: GetTotalLength::<Impl, IMPL_OFFSET>,
            GetLastPlayPosition: GetLastPlayPosition::<Impl, IMPL_OFFSET>,
            GetLongestPlayPosition: GetLongestPlayPosition::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMDSPObjectInfo as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IMDSPRevokedImpl: Sized {
    fn GetRevocationURL(&mut self, ppwszrevocationurl: *mut super::super::Foundation::PWSTR, pdwbufferlen: *mut u32) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IMDSPRevokedVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMDSPRevokedImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMDSPRevokedVtbl {
        unsafe extern "system" fn GetRevocationURL<Impl: IMDSPRevokedImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppwszrevocationurl: *mut super::super::Foundation::PWSTR, pdwbufferlen: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetRevocationURL(::core::mem::transmute_copy(&ppwszrevocationurl), ::core::mem::transmute_copy(&pdwbufferlen)).into()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), GetRevocationURL: GetRevocationURL::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMDSPRevoked as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IMDSPStorageImpl: Sized {
    fn SetAttributes(&mut self, dwattributes: u32, pformat: *const _WAVEFORMATEX) -> ::windows::core::Result<()>;
    fn GetStorageGlobals(&mut self) -> ::windows::core::Result<IMDSPStorageGlobals>;
    fn GetAttributes(&mut self, pdwattributes: *mut u32, pformat: *mut _WAVEFORMATEX) -> ::windows::core::Result<()>;
    fn GetName(&mut self, pwszname: super::super::Foundation::PWSTR, nmaxchars: u32) -> ::windows::core::Result<()>;
    fn GetDate(&mut self) -> ::windows::core::Result<WMDMDATETIME>;
    fn GetSize(&mut self, pdwsizelow: *mut u32, pdwsizehigh: *mut u32) -> ::windows::core::Result<()>;
    fn GetRights(&mut self, pprights: *mut *mut WMDMRIGHTS, pnrightscount: *mut u32, abmac: *mut u8) -> ::windows::core::Result<()>;
    fn CreateStorage(&mut self, dwattributes: u32, pformat: *const _WAVEFORMATEX, pwszname: super::super::Foundation::PWSTR) -> ::windows::core::Result<IMDSPStorage>;
    fn EnumStorage(&mut self) -> ::windows::core::Result<IMDSPEnumStorage>;
    fn SendOpaqueCommand(&mut self, pcommand: *mut OPAQUECOMMAND) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IMDSPStorageVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMDSPStorageImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMDSPStorageVtbl {
        unsafe extern "system" fn SetAttributes<Impl: IMDSPStorageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwattributes: u32, pformat: *const _WAVEFORMATEX) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetAttributes(::core::mem::transmute_copy(&dwattributes), ::core::mem::transmute_copy(&pformat)).into()
        }
        unsafe extern "system" fn GetStorageGlobals<Impl: IMDSPStorageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppstorageglobals: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetStorageGlobals() {
                ::core::result::Result::Ok(ok__) => {
                    *ppstorageglobals = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetAttributes<Impl: IMDSPStorageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwattributes: *mut u32, pformat: *mut _WAVEFORMATEX) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetAttributes(::core::mem::transmute_copy(&pdwattributes), ::core::mem::transmute_copy(&pformat)).into()
        }
        unsafe extern "system" fn GetName<Impl: IMDSPStorageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwszname: super::super::Foundation::PWSTR, nmaxchars: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetName(::core::mem::transmute_copy(&pwszname), ::core::mem::transmute_copy(&nmaxchars)).into()
        }
        unsafe extern "system" fn GetDate<Impl: IMDSPStorageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdatetimeutc: *mut WMDMDATETIME) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDate() {
                ::core::result::Result::Ok(ok__) => {
                    *pdatetimeutc = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSize<Impl: IMDSPStorageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwsizelow: *mut u32, pdwsizehigh: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetSize(::core::mem::transmute_copy(&pdwsizelow), ::core::mem::transmute_copy(&pdwsizehigh)).into()
        }
        unsafe extern "system" fn GetRights<Impl: IMDSPStorageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pprights: *mut *mut WMDMRIGHTS, pnrightscount: *mut u32, abmac: *mut u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetRights(::core::mem::transmute_copy(&pprights), ::core::mem::transmute_copy(&pnrightscount), ::core::mem::transmute_copy(&abmac)).into()
        }
        unsafe extern "system" fn CreateStorage<Impl: IMDSPStorageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwattributes: u32, pformat: *const _WAVEFORMATEX, pwszname: super::super::Foundation::PWSTR, ppnewstorage: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateStorage(::core::mem::transmute_copy(&dwattributes), ::core::mem::transmute_copy(&pformat), ::core::mem::transmute_copy(&pwszname)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppnewstorage = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnumStorage<Impl: IMDSPStorageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenumstorage: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EnumStorage() {
                ::core::result::Result::Ok(ok__) => {
                    *ppenumstorage = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SendOpaqueCommand<Impl: IMDSPStorageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcommand: *mut OPAQUECOMMAND) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SendOpaqueCommand(::core::mem::transmute_copy(&pcommand)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            SetAttributes: SetAttributes::<Impl, IMPL_OFFSET>,
            GetStorageGlobals: GetStorageGlobals::<Impl, IMPL_OFFSET>,
            GetAttributes: GetAttributes::<Impl, IMPL_OFFSET>,
            GetName: GetName::<Impl, IMPL_OFFSET>,
            GetDate: GetDate::<Impl, IMPL_OFFSET>,
            GetSize: GetSize::<Impl, IMPL_OFFSET>,
            GetRights: GetRights::<Impl, IMPL_OFFSET>,
            CreateStorage: CreateStorage::<Impl, IMPL_OFFSET>,
            EnumStorage: EnumStorage::<Impl, IMPL_OFFSET>,
            SendOpaqueCommand: SendOpaqueCommand::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMDSPStorage as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IMDSPStorage2Impl: Sized + IMDSPStorageImpl {
    fn GetStorage(&mut self, pszstoragename: super::super::Foundation::PWSTR) -> ::windows::core::Result<IMDSPStorage>;
    fn CreateStorage2(&mut self, dwattributes: u32, dwattributesex: u32, paudioformat: *const _WAVEFORMATEX, pvideoformat: *const _VIDEOINFOHEADER, pwszname: super::super::Foundation::PWSTR, qwfilesize: u64) -> ::windows::core::Result<IMDSPStorage>;
    fn SetAttributes2(&mut self, dwattributes: u32, dwattributesex: u32, paudioformat: *const _WAVEFORMATEX, pvideoformat: *const _VIDEOINFOHEADER) -> ::windows::core::Result<()>;
    fn GetAttributes2(&mut self, pdwattributes: *mut u32, pdwattributesex: *mut u32, paudioformat: *mut _WAVEFORMATEX, pvideoformat: *mut _VIDEOINFOHEADER) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IMDSPStorage2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMDSPStorage2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMDSPStorage2Vtbl {
        unsafe extern "system" fn GetStorage<Impl: IMDSPStorage2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszstoragename: super::super::Foundation::PWSTR, ppstorage: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetStorage(::core::mem::transmute_copy(&pszstoragename)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppstorage = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateStorage2<Impl: IMDSPStorage2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwattributes: u32, dwattributesex: u32, paudioformat: *const _WAVEFORMATEX, pvideoformat: *const _VIDEOINFOHEADER, pwszname: super::super::Foundation::PWSTR, qwfilesize: u64, ppnewstorage: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateStorage2(::core::mem::transmute_copy(&dwattributes), ::core::mem::transmute_copy(&dwattributesex), ::core::mem::transmute_copy(&paudioformat), ::core::mem::transmute_copy(&pvideoformat), ::core::mem::transmute_copy(&pwszname), ::core::mem::transmute_copy(&qwfilesize)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppnewstorage = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAttributes2<Impl: IMDSPStorage2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwattributes: u32, dwattributesex: u32, paudioformat: *const _WAVEFORMATEX, pvideoformat: *const _VIDEOINFOHEADER) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetAttributes2(::core::mem::transmute_copy(&dwattributes), ::core::mem::transmute_copy(&dwattributesex), ::core::mem::transmute_copy(&paudioformat), ::core::mem::transmute_copy(&pvideoformat)).into()
        }
        unsafe extern "system" fn GetAttributes2<Impl: IMDSPStorage2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwattributes: *mut u32, pdwattributesex: *mut u32, paudioformat: *mut _WAVEFORMATEX, pvideoformat: *mut _VIDEOINFOHEADER) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetAttributes2(::core::mem::transmute_copy(&pdwattributes), ::core::mem::transmute_copy(&pdwattributesex), ::core::mem::transmute_copy(&paudioformat), ::core::mem::transmute_copy(&pvideoformat)).into()
        }
        Self {
            base: IMDSPStorageVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            GetStorage: GetStorage::<Impl, IMPL_OFFSET>,
            CreateStorage2: CreateStorage2::<Impl, IMPL_OFFSET>,
            SetAttributes2: SetAttributes2::<Impl, IMPL_OFFSET>,
            GetAttributes2: GetAttributes2::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMDSPStorage2 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IMDSPStorage3Impl: Sized + IMDSPStorageImpl + IMDSPStorage2Impl {
    fn GetMetadata(&mut self, pmetadata: ::core::option::Option<IWMDMMetaData>) -> ::windows::core::Result<()>;
    fn SetMetadata(&mut self, pmetadata: ::core::option::Option<IWMDMMetaData>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IMDSPStorage3Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMDSPStorage3Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMDSPStorage3Vtbl {
        unsafe extern "system" fn GetMetadata<Impl: IMDSPStorage3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pmetadata: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetMetadata(::core::mem::transmute(&pmetadata)).into()
        }
        unsafe extern "system" fn SetMetadata<Impl: IMDSPStorage3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pmetadata: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetMetadata(::core::mem::transmute(&pmetadata)).into()
        }
        Self {
            base: IMDSPStorage2Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            GetMetadata: GetMetadata::<Impl, IMPL_OFFSET>,
            SetMetadata: SetMetadata::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMDSPStorage3 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IMDSPStorage4Impl: Sized + IMDSPStorageImpl + IMDSPStorage2Impl + IMDSPStorage3Impl {
    fn SetReferences(&mut self, dwrefs: u32, ppispstorage: *const ::core::option::Option<IMDSPStorage>) -> ::windows::core::Result<()>;
    fn GetReferences(&mut self, pdwrefs: *mut u32, pppispstorage: *mut *mut ::core::option::Option<IMDSPStorage>) -> ::windows::core::Result<()>;
    fn CreateStorageWithMetadata(&mut self, dwattributes: u32, pwszname: super::super::Foundation::PWSTR, pmetadata: ::core::option::Option<IWMDMMetaData>, qwfilesize: u64) -> ::windows::core::Result<IMDSPStorage>;
    fn GetSpecifiedMetadata(&mut self, cproperties: u32, ppwszpropnames: *const super::super::Foundation::PWSTR, pmetadata: ::core::option::Option<IWMDMMetaData>) -> ::windows::core::Result<()>;
    fn FindStorage(&mut self, findscope: WMDM_FIND_SCOPE, pwszuniqueid: super::super::Foundation::PWSTR) -> ::windows::core::Result<IMDSPStorage>;
    fn GetParent(&mut self) -> ::windows::core::Result<IMDSPStorage>;
}
#[cfg(feature = "Win32_Foundation")]
impl IMDSPStorage4Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMDSPStorage4Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMDSPStorage4Vtbl {
        unsafe extern "system" fn SetReferences<Impl: IMDSPStorage4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwrefs: u32, ppispstorage: *const ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetReferences(::core::mem::transmute_copy(&dwrefs), ::core::mem::transmute_copy(&ppispstorage)).into()
        }
        unsafe extern "system" fn GetReferences<Impl: IMDSPStorage4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwrefs: *mut u32, pppispstorage: *mut *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetReferences(::core::mem::transmute_copy(&pdwrefs), ::core::mem::transmute_copy(&pppispstorage)).into()
        }
        unsafe extern "system" fn CreateStorageWithMetadata<Impl: IMDSPStorage4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwattributes: u32, pwszname: super::super::Foundation::PWSTR, pmetadata: ::windows::core::RawPtr, qwfilesize: u64, ppnewstorage: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateStorageWithMetadata(::core::mem::transmute_copy(&dwattributes), ::core::mem::transmute_copy(&pwszname), ::core::mem::transmute(&pmetadata), ::core::mem::transmute_copy(&qwfilesize)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppnewstorage = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSpecifiedMetadata<Impl: IMDSPStorage4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cproperties: u32, ppwszpropnames: *const super::super::Foundation::PWSTR, pmetadata: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetSpecifiedMetadata(::core::mem::transmute_copy(&cproperties), ::core::mem::transmute_copy(&ppwszpropnames), ::core::mem::transmute(&pmetadata)).into()
        }
        unsafe extern "system" fn FindStorage<Impl: IMDSPStorage4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, findscope: WMDM_FIND_SCOPE, pwszuniqueid: super::super::Foundation::PWSTR, ppstorage: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FindStorage(::core::mem::transmute_copy(&findscope), ::core::mem::transmute_copy(&pwszuniqueid)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppstorage = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetParent<Impl: IMDSPStorage4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppstorage: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetParent() {
                ::core::result::Result::Ok(ok__) => {
                    *ppstorage = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: IMDSPStorage3Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            SetReferences: SetReferences::<Impl, IMPL_OFFSET>,
            GetReferences: GetReferences::<Impl, IMPL_OFFSET>,
            CreateStorageWithMetadata: CreateStorageWithMetadata::<Impl, IMPL_OFFSET>,
            GetSpecifiedMetadata: GetSpecifiedMetadata::<Impl, IMPL_OFFSET>,
            FindStorage: FindStorage::<Impl, IMPL_OFFSET>,
            GetParent: GetParent::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMDSPStorage4 as ::windows::core::Interface>::IID
    }
}
pub trait IMDSPStorageGlobalsImpl: Sized {
    fn GetCapabilities(&mut self) -> ::windows::core::Result<u32>;
    fn GetSerialNumber(&mut self, pserialnum: *mut WMDMID, abmac: *mut u8) -> ::windows::core::Result<()>;
    fn GetTotalSize(&mut self, pdwtotalsizelow: *mut u32, pdwtotalsizehigh: *mut u32) -> ::windows::core::Result<()>;
    fn GetTotalFree(&mut self, pdwfreelow: *mut u32, pdwfreehigh: *mut u32) -> ::windows::core::Result<()>;
    fn GetTotalBad(&mut self, pdwbadlow: *mut u32, pdwbadhigh: *mut u32) -> ::windows::core::Result<()>;
    fn GetStatus(&mut self) -> ::windows::core::Result<u32>;
    fn Initialize(&mut self, fumode: u32, pprogress: ::core::option::Option<IWMDMProgress>) -> ::windows::core::Result<()>;
    fn GetDevice(&mut self) -> ::windows::core::Result<IMDSPDevice>;
    fn GetRootStorage(&mut self) -> ::windows::core::Result<IMDSPStorage>;
}
impl IMDSPStorageGlobalsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMDSPStorageGlobalsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMDSPStorageGlobalsVtbl {
        unsafe extern "system" fn GetCapabilities<Impl: IMDSPStorageGlobalsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwcapabilities: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetCapabilities() {
                ::core::result::Result::Ok(ok__) => {
                    *pdwcapabilities = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSerialNumber<Impl: IMDSPStorageGlobalsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pserialnum: *mut WMDMID, abmac: *mut u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetSerialNumber(::core::mem::transmute_copy(&pserialnum), ::core::mem::transmute_copy(&abmac)).into()
        }
        unsafe extern "system" fn GetTotalSize<Impl: IMDSPStorageGlobalsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwtotalsizelow: *mut u32, pdwtotalsizehigh: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetTotalSize(::core::mem::transmute_copy(&pdwtotalsizelow), ::core::mem::transmute_copy(&pdwtotalsizehigh)).into()
        }
        unsafe extern "system" fn GetTotalFree<Impl: IMDSPStorageGlobalsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwfreelow: *mut u32, pdwfreehigh: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetTotalFree(::core::mem::transmute_copy(&pdwfreelow), ::core::mem::transmute_copy(&pdwfreehigh)).into()
        }
        unsafe extern "system" fn GetTotalBad<Impl: IMDSPStorageGlobalsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwbadlow: *mut u32, pdwbadhigh: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetTotalBad(::core::mem::transmute_copy(&pdwbadlow), ::core::mem::transmute_copy(&pdwbadhigh)).into()
        }
        unsafe extern "system" fn GetStatus<Impl: IMDSPStorageGlobalsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwstatus: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetStatus() {
                ::core::result::Result::Ok(ok__) => {
                    *pdwstatus = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Initialize<Impl: IMDSPStorageGlobalsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fumode: u32, pprogress: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Initialize(::core::mem::transmute_copy(&fumode), ::core::mem::transmute(&pprogress)).into()
        }
        unsafe extern "system" fn GetDevice<Impl: IMDSPStorageGlobalsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppdevice: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDevice() {
                ::core::result::Result::Ok(ok__) => {
                    *ppdevice = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetRootStorage<Impl: IMDSPStorageGlobalsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pproot: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetRootStorage() {
                ::core::result::Result::Ok(ok__) => {
                    *pproot = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            GetCapabilities: GetCapabilities::<Impl, IMPL_OFFSET>,
            GetSerialNumber: GetSerialNumber::<Impl, IMPL_OFFSET>,
            GetTotalSize: GetTotalSize::<Impl, IMPL_OFFSET>,
            GetTotalFree: GetTotalFree::<Impl, IMPL_OFFSET>,
            GetTotalBad: GetTotalBad::<Impl, IMPL_OFFSET>,
            GetStatus: GetStatus::<Impl, IMPL_OFFSET>,
            Initialize: Initialize::<Impl, IMPL_OFFSET>,
            GetDevice: GetDevice::<Impl, IMPL_OFFSET>,
            GetRootStorage: GetRootStorage::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMDSPStorageGlobals as ::windows::core::Interface>::IID
    }
}
pub trait IMDServiceProviderImpl: Sized {
    fn GetDeviceCount(&mut self) -> ::windows::core::Result<u32>;
    fn EnumDevices(&mut self) -> ::windows::core::Result<IMDSPEnumDevice>;
}
impl IMDServiceProviderVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMDServiceProviderImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMDServiceProviderVtbl {
        unsafe extern "system" fn GetDeviceCount<Impl: IMDServiceProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwcount: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDeviceCount() {
                ::core::result::Result::Ok(ok__) => {
                    *pdwcount = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnumDevices<Impl: IMDServiceProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenumdevice: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EnumDevices() {
                ::core::result::Result::Ok(ok__) => {
                    *ppenumdevice = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            GetDeviceCount: GetDeviceCount::<Impl, IMPL_OFFSET>,
            EnumDevices: EnumDevices::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMDServiceProvider as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IMDServiceProvider2Impl: Sized + IMDServiceProviderImpl {
    fn CreateDevice(&mut self, pwszdevicepath: super::super::Foundation::PWSTR, pdwcount: *mut u32, pppdevicearray: *mut *mut ::core::option::Option<IMDSPDevice>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IMDServiceProvider2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMDServiceProvider2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMDServiceProvider2Vtbl {
        unsafe extern "system" fn CreateDevice<Impl: IMDServiceProvider2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwszdevicepath: super::super::Foundation::PWSTR, pdwcount: *mut u32, pppdevicearray: *mut *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).CreateDevice(::core::mem::transmute_copy(&pwszdevicepath), ::core::mem::transmute_copy(&pdwcount), ::core::mem::transmute_copy(&pppdevicearray)).into()
        }
        Self { base: IMDServiceProviderVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(), CreateDevice: CreateDevice::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMDServiceProvider2 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IMDServiceProvider3Impl: Sized + IMDServiceProviderImpl + IMDServiceProvider2Impl {
    fn SetDeviceEnumPreference(&mut self, dwenumpref: u32) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IMDServiceProvider3Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMDServiceProvider3Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMDServiceProvider3Vtbl {
        unsafe extern "system" fn SetDeviceEnumPreference<Impl: IMDServiceProvider3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwenumpref: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetDeviceEnumPreference(::core::mem::transmute_copy(&dwenumpref)).into()
        }
        Self {
            base: IMDServiceProvider2Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            SetDeviceEnumPreference: SetDeviceEnumPreference::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMDServiceProvider3 as ::windows::core::Interface>::IID
    }
}
pub trait ISCPSecureAuthenticateImpl: Sized {
    fn GetSecureQuery(&mut self) -> ::windows::core::Result<ISCPSecureQuery>;
}
impl ISCPSecureAuthenticateVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISCPSecureAuthenticateImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISCPSecureAuthenticateVtbl {
        unsafe extern "system" fn GetSecureQuery<Impl: ISCPSecureAuthenticateImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppsecurequery: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetSecureQuery() {
                ::core::result::Result::Ok(ok__) => {
                    *ppsecurequery = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), GetSecureQuery: GetSecureQuery::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISCPSecureAuthenticate as ::windows::core::Interface>::IID
    }
}
pub trait ISCPSecureAuthenticate2Impl: Sized + ISCPSecureAuthenticateImpl {
    fn GetSCPSession(&mut self) -> ::windows::core::Result<ISCPSession>;
}
impl ISCPSecureAuthenticate2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISCPSecureAuthenticate2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISCPSecureAuthenticate2Vtbl {
        unsafe extern "system" fn GetSCPSession<Impl: ISCPSecureAuthenticate2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppscpsession: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetSCPSession() {
                ::core::result::Result::Ok(ok__) => {
                    *ppscpsession = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: ISCPSecureAuthenticateVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(), GetSCPSession: GetSCPSession::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISCPSecureAuthenticate2 as ::windows::core::Interface>::IID
    }
}
pub trait ISCPSecureExchangeImpl: Sized {
    fn TransferContainerData(&mut self, pdata: *const u8, dwsize: u32, pfureadyflags: *mut u32, abmac: *mut u8) -> ::windows::core::Result<()>;
    fn ObjectData(&mut self, pdata: *mut u8, pdwsize: *mut u32, abmac: *mut u8) -> ::windows::core::Result<()>;
    fn TransferComplete(&mut self) -> ::windows::core::Result<()>;
}
impl ISCPSecureExchangeVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISCPSecureExchangeImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISCPSecureExchangeVtbl {
        unsafe extern "system" fn TransferContainerData<Impl: ISCPSecureExchangeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdata: *const u8, dwsize: u32, pfureadyflags: *mut u32, abmac: *mut u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).TransferContainerData(::core::mem::transmute_copy(&pdata), ::core::mem::transmute_copy(&dwsize), ::core::mem::transmute_copy(&pfureadyflags), ::core::mem::transmute_copy(&abmac)).into()
        }
        unsafe extern "system" fn ObjectData<Impl: ISCPSecureExchangeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdata: *mut u8, pdwsize: *mut u32, abmac: *mut u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ObjectData(::core::mem::transmute_copy(&pdata), ::core::mem::transmute_copy(&pdwsize), ::core::mem::transmute_copy(&abmac)).into()
        }
        unsafe extern "system" fn TransferComplete<Impl: ISCPSecureExchangeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).TransferComplete().into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            TransferContainerData: TransferContainerData::<Impl, IMPL_OFFSET>,
            ObjectData: ObjectData::<Impl, IMPL_OFFSET>,
            TransferComplete: TransferComplete::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISCPSecureExchange as ::windows::core::Interface>::IID
    }
}
pub trait ISCPSecureExchange2Impl: Sized + ISCPSecureExchangeImpl {
    fn TransferContainerData2(&mut self, pdata: *const u8, dwsize: u32, pprogresscallback: ::core::option::Option<IWMDMProgress3>, pfureadyflags: *mut u32, abmac: *mut u8) -> ::windows::core::Result<()>;
}
impl ISCPSecureExchange2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISCPSecureExchange2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISCPSecureExchange2Vtbl {
        unsafe extern "system" fn TransferContainerData2<Impl: ISCPSecureExchange2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdata: *const u8, dwsize: u32, pprogresscallback: ::windows::core::RawPtr, pfureadyflags: *mut u32, abmac: *mut u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).TransferContainerData2(::core::mem::transmute_copy(&pdata), ::core::mem::transmute_copy(&dwsize), ::core::mem::transmute(&pprogresscallback), ::core::mem::transmute_copy(&pfureadyflags), ::core::mem::transmute_copy(&abmac)).into()
        }
        Self {
            base: ISCPSecureExchangeVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            TransferContainerData2: TransferContainerData2::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISCPSecureExchange2 as ::windows::core::Interface>::IID
    }
}
pub trait ISCPSecureExchange3Impl: Sized + ISCPSecureExchangeImpl + ISCPSecureExchange2Impl {
    fn TransferContainerDataOnClearChannel(&mut self, pdevice: ::core::option::Option<IMDSPDevice>, pdata: *const u8, dwsize: u32, pprogresscallback: ::core::option::Option<IWMDMProgress3>) -> ::windows::core::Result<u32>;
    fn GetObjectDataOnClearChannel(&mut self, pdevice: ::core::option::Option<IMDSPDevice>, pdata: *mut u8, pdwsize: *mut u32) -> ::windows::core::Result<()>;
    fn TransferCompleteForDevice(&mut self, pdevice: ::core::option::Option<IMDSPDevice>) -> ::windows::core::Result<()>;
}
impl ISCPSecureExchange3Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISCPSecureExchange3Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISCPSecureExchange3Vtbl {
        unsafe extern "system" fn TransferContainerDataOnClearChannel<Impl: ISCPSecureExchange3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdevice: ::windows::core::RawPtr, pdata: *const u8, dwsize: u32, pprogresscallback: ::windows::core::RawPtr, pfureadyflags: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TransferContainerDataOnClearChannel(::core::mem::transmute(&pdevice), ::core::mem::transmute_copy(&pdata), ::core::mem::transmute_copy(&dwsize), ::core::mem::transmute(&pprogresscallback)) {
                ::core::result::Result::Ok(ok__) => {
                    *pfureadyflags = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetObjectDataOnClearChannel<Impl: ISCPSecureExchange3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdevice: ::windows::core::RawPtr, pdata: *mut u8, pdwsize: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetObjectDataOnClearChannel(::core::mem::transmute(&pdevice), ::core::mem::transmute_copy(&pdata), ::core::mem::transmute_copy(&pdwsize)).into()
        }
        unsafe extern "system" fn TransferCompleteForDevice<Impl: ISCPSecureExchange3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdevice: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).TransferCompleteForDevice(::core::mem::transmute(&pdevice)).into()
        }
        Self {
            base: ISCPSecureExchange2Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            TransferContainerDataOnClearChannel: TransferContainerDataOnClearChannel::<Impl, IMPL_OFFSET>,
            GetObjectDataOnClearChannel: GetObjectDataOnClearChannel::<Impl, IMPL_OFFSET>,
            TransferCompleteForDevice: TransferCompleteForDevice::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISCPSecureExchange3 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait ISCPSecureQueryImpl: Sized {
    fn GetDataDemands(&mut self, pfuflags: *mut u32, pdwminrightsdata: *mut u32, pdwminexaminedata: *mut u32, pdwmindecidedata: *mut u32, abmac: *mut u8) -> ::windows::core::Result<()>;
    fn ExamineData(&mut self, fuflags: u32, pwszextension: super::super::Foundation::PWSTR, pdata: *const u8, dwsize: u32, abmac: *mut u8) -> ::windows::core::Result<()>;
    fn MakeDecision(&mut self, fuflags: u32, pdata: *const u8, dwsize: u32, dwappsec: u32, pbspsessionkey: *const u8, dwsessionkeylen: u32, pstorageglobals: ::core::option::Option<IMDSPStorageGlobals>, ppexchange: *mut ::core::option::Option<ISCPSecureExchange>, abmac: *mut u8) -> ::windows::core::Result<()>;
    fn GetRights(&mut self, pdata: *const u8, dwsize: u32, pbspsessionkey: *const u8, dwsessionkeylen: u32, pstgglobals: ::core::option::Option<IMDSPStorageGlobals>, pprights: *mut *mut WMDMRIGHTS, pnrightscount: *mut u32, abmac: *mut u8) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ISCPSecureQueryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISCPSecureQueryImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISCPSecureQueryVtbl {
        unsafe extern "system" fn GetDataDemands<Impl: ISCPSecureQueryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfuflags: *mut u32, pdwminrightsdata: *mut u32, pdwminexaminedata: *mut u32, pdwmindecidedata: *mut u32, abmac: *mut u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetDataDemands(::core::mem::transmute_copy(&pfuflags), ::core::mem::transmute_copy(&pdwminrightsdata), ::core::mem::transmute_copy(&pdwminexaminedata), ::core::mem::transmute_copy(&pdwmindecidedata), ::core::mem::transmute_copy(&abmac)).into()
        }
        unsafe extern "system" fn ExamineData<Impl: ISCPSecureQueryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fuflags: u32, pwszextension: super::super::Foundation::PWSTR, pdata: *const u8, dwsize: u32, abmac: *mut u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ExamineData(::core::mem::transmute_copy(&fuflags), ::core::mem::transmute_copy(&pwszextension), ::core::mem::transmute_copy(&pdata), ::core::mem::transmute_copy(&dwsize), ::core::mem::transmute_copy(&abmac)).into()
        }
        unsafe extern "system" fn MakeDecision<Impl: ISCPSecureQueryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fuflags: u32, pdata: *const u8, dwsize: u32, dwappsec: u32, pbspsessionkey: *const u8, dwsessionkeylen: u32, pstorageglobals: ::windows::core::RawPtr, ppexchange: *mut ::windows::core::RawPtr, abmac: *mut u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).MakeDecision(::core::mem::transmute_copy(&fuflags), ::core::mem::transmute_copy(&pdata), ::core::mem::transmute_copy(&dwsize), ::core::mem::transmute_copy(&dwappsec), ::core::mem::transmute_copy(&pbspsessionkey), ::core::mem::transmute_copy(&dwsessionkeylen), ::core::mem::transmute(&pstorageglobals), ::core::mem::transmute_copy(&ppexchange), ::core::mem::transmute_copy(&abmac)).into()
        }
        unsafe extern "system" fn GetRights<Impl: ISCPSecureQueryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdata: *const u8, dwsize: u32, pbspsessionkey: *const u8, dwsessionkeylen: u32, pstgglobals: ::windows::core::RawPtr, pprights: *mut *mut WMDMRIGHTS, pnrightscount: *mut u32, abmac: *mut u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetRights(::core::mem::transmute_copy(&pdata), ::core::mem::transmute_copy(&dwsize), ::core::mem::transmute_copy(&pbspsessionkey), ::core::mem::transmute_copy(&dwsessionkeylen), ::core::mem::transmute(&pstgglobals), ::core::mem::transmute_copy(&pprights), ::core::mem::transmute_copy(&pnrightscount), ::core::mem::transmute_copy(&abmac)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            GetDataDemands: GetDataDemands::<Impl, IMPL_OFFSET>,
            ExamineData: ExamineData::<Impl, IMPL_OFFSET>,
            MakeDecision: MakeDecision::<Impl, IMPL_OFFSET>,
            GetRights: GetRights::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISCPSecureQuery as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait ISCPSecureQuery2Impl: Sized + ISCPSecureQueryImpl {
    fn MakeDecision2(&mut self, fuflags: u32, pdata: *const u8, dwsize: u32, dwappsec: u32, pbspsessionkey: *const u8, dwsessionkeylen: u32, pstorageglobals: ::core::option::Option<IMDSPStorageGlobals>, pappcertapp: *const u8, dwappcertapplen: u32, pappcertsp: *const u8, dwappcertsplen: u32, pszrevocationurl: *mut super::super::Foundation::PWSTR, pdwrevocationurllen: *mut u32, pdwrevocationbitflag: *mut u32, pqwfilesize: *mut u64, punknown: ::core::option::Option<::windows::core::IUnknown>, ppexchange: *mut ::core::option::Option<ISCPSecureExchange>, abmac: *mut u8) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ISCPSecureQuery2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISCPSecureQuery2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISCPSecureQuery2Vtbl {
        unsafe extern "system" fn MakeDecision2<Impl: ISCPSecureQuery2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fuflags: u32, pdata: *const u8, dwsize: u32, dwappsec: u32, pbspsessionkey: *const u8, dwsessionkeylen: u32, pstorageglobals: ::windows::core::RawPtr, pappcertapp: *const u8, dwappcertapplen: u32, pappcertsp: *const u8, dwappcertsplen: u32, pszrevocationurl: *mut super::super::Foundation::PWSTR, pdwrevocationurllen: *mut u32, pdwrevocationbitflag: *mut u32, pqwfilesize: *mut u64, punknown: *mut ::core::ffi::c_void, ppexchange: *mut ::windows::core::RawPtr, abmac: *mut u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this)
                .MakeDecision2(
                    ::core::mem::transmute_copy(&fuflags),
                    ::core::mem::transmute_copy(&pdata),
                    ::core::mem::transmute_copy(&dwsize),
                    ::core::mem::transmute_copy(&dwappsec),
                    ::core::mem::transmute_copy(&pbspsessionkey),
                    ::core::mem::transmute_copy(&dwsessionkeylen),
                    ::core::mem::transmute(&pstorageglobals),
                    ::core::mem::transmute_copy(&pappcertapp),
                    ::core::mem::transmute_copy(&dwappcertapplen),
                    ::core::mem::transmute_copy(&pappcertsp),
                    ::core::mem::transmute_copy(&dwappcertsplen),
                    ::core::mem::transmute_copy(&pszrevocationurl),
                    ::core::mem::transmute_copy(&pdwrevocationurllen),
                    ::core::mem::transmute_copy(&pdwrevocationbitflag),
                    ::core::mem::transmute_copy(&pqwfilesize),
                    ::core::mem::transmute(&punknown),
                    ::core::mem::transmute_copy(&ppexchange),
                    ::core::mem::transmute_copy(&abmac),
                )
                .into()
        }
        Self { base: ISCPSecureQueryVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(), MakeDecision2: MakeDecision2::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISCPSecureQuery2 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait ISCPSecureQuery3Impl: Sized + ISCPSecureQueryImpl + ISCPSecureQuery2Impl {
    fn GetRightsOnClearChannel(&mut self, pdata: *const u8, dwsize: u32, pbspsessionkey: *const u8, dwsessionkeylen: u32, pstgglobals: ::core::option::Option<IMDSPStorageGlobals>, pprogresscallback: ::core::option::Option<IWMDMProgress3>, pprights: *mut *mut WMDMRIGHTS, pnrightscount: *mut u32) -> ::windows::core::Result<()>;
    fn MakeDecisionOnClearChannel(&mut self, fuflags: u32, pdata: *const u8, dwsize: u32, dwappsec: u32, pbspsessionkey: *const u8, dwsessionkeylen: u32, pstorageglobals: ::core::option::Option<IMDSPStorageGlobals>, pprogresscallback: ::core::option::Option<IWMDMProgress3>, pappcertapp: *const u8, dwappcertapplen: u32, pappcertsp: *const u8, dwappcertsplen: u32, pszrevocationurl: *mut super::super::Foundation::PWSTR, pdwrevocationurllen: *mut u32, pdwrevocationbitflag: *mut u32, pqwfilesize: *mut u64, punknown: ::core::option::Option<::windows::core::IUnknown>, ppexchange: *mut ::core::option::Option<ISCPSecureExchange>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ISCPSecureQuery3Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISCPSecureQuery3Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISCPSecureQuery3Vtbl {
        unsafe extern "system" fn GetRightsOnClearChannel<Impl: ISCPSecureQuery3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdata: *const u8, dwsize: u32, pbspsessionkey: *const u8, dwsessionkeylen: u32, pstgglobals: ::windows::core::RawPtr, pprogresscallback: ::windows::core::RawPtr, pprights: *mut *mut WMDMRIGHTS, pnrightscount: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetRightsOnClearChannel(::core::mem::transmute_copy(&pdata), ::core::mem::transmute_copy(&dwsize), ::core::mem::transmute_copy(&pbspsessionkey), ::core::mem::transmute_copy(&dwsessionkeylen), ::core::mem::transmute(&pstgglobals), ::core::mem::transmute(&pprogresscallback), ::core::mem::transmute_copy(&pprights), ::core::mem::transmute_copy(&pnrightscount)).into()
        }
        unsafe extern "system" fn MakeDecisionOnClearChannel<Impl: ISCPSecureQuery3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fuflags: u32, pdata: *const u8, dwsize: u32, dwappsec: u32, pbspsessionkey: *const u8, dwsessionkeylen: u32, pstorageglobals: ::windows::core::RawPtr, pprogresscallback: ::windows::core::RawPtr, pappcertapp: *const u8, dwappcertapplen: u32, pappcertsp: *const u8, dwappcertsplen: u32, pszrevocationurl: *mut super::super::Foundation::PWSTR, pdwrevocationurllen: *mut u32, pdwrevocationbitflag: *mut u32, pqwfilesize: *mut u64, punknown: *mut ::core::ffi::c_void, ppexchange: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this)
                .MakeDecisionOnClearChannel(
                    ::core::mem::transmute_copy(&fuflags),
                    ::core::mem::transmute_copy(&pdata),
                    ::core::mem::transmute_copy(&dwsize),
                    ::core::mem::transmute_copy(&dwappsec),
                    ::core::mem::transmute_copy(&pbspsessionkey),
                    ::core::mem::transmute_copy(&dwsessionkeylen),
                    ::core::mem::transmute(&pstorageglobals),
                    ::core::mem::transmute(&pprogresscallback),
                    ::core::mem::transmute_copy(&pappcertapp),
                    ::core::mem::transmute_copy(&dwappcertapplen),
                    ::core::mem::transmute_copy(&pappcertsp),
                    ::core::mem::transmute_copy(&dwappcertsplen),
                    ::core::mem::transmute_copy(&pszrevocationurl),
                    ::core::mem::transmute_copy(&pdwrevocationurllen),
                    ::core::mem::transmute_copy(&pdwrevocationbitflag),
                    ::core::mem::transmute_copy(&pqwfilesize),
                    ::core::mem::transmute(&punknown),
                    ::core::mem::transmute_copy(&ppexchange),
                )
                .into()
        }
        Self {
            base: ISCPSecureQuery2Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            GetRightsOnClearChannel: GetRightsOnClearChannel::<Impl, IMPL_OFFSET>,
            MakeDecisionOnClearChannel: MakeDecisionOnClearChannel::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISCPSecureQuery3 as ::windows::core::Interface>::IID
    }
}
pub trait ISCPSessionImpl: Sized {
    fn BeginSession(&mut self, pidevice: ::core::option::Option<IMDSPDevice>, pctx: *const u8, dwsizectx: u32) -> ::windows::core::Result<()>;
    fn EndSession(&mut self, pctx: *const u8, dwsizectx: u32) -> ::windows::core::Result<()>;
    fn GetSecureQuery(&mut self) -> ::windows::core::Result<ISCPSecureQuery>;
}
impl ISCPSessionVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISCPSessionImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISCPSessionVtbl {
        unsafe extern "system" fn BeginSession<Impl: ISCPSessionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pidevice: ::windows::core::RawPtr, pctx: *const u8, dwsizectx: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).BeginSession(::core::mem::transmute(&pidevice), ::core::mem::transmute_copy(&pctx), ::core::mem::transmute_copy(&dwsizectx)).into()
        }
        unsafe extern "system" fn EndSession<Impl: ISCPSessionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pctx: *const u8, dwsizectx: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).EndSession(::core::mem::transmute_copy(&pctx), ::core::mem::transmute_copy(&dwsizectx)).into()
        }
        unsafe extern "system" fn GetSecureQuery<Impl: ISCPSessionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppsecurequery: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetSecureQuery() {
                ::core::result::Result::Ok(ok__) => {
                    *ppsecurequery = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            BeginSession: BeginSession::<Impl, IMPL_OFFSET>,
            EndSession: EndSession::<Impl, IMPL_OFFSET>,
            GetSecureQuery: GetSecureQuery::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISCPSession as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IWMDMDeviceImpl: Sized {
    fn GetName(&mut self, pwszname: super::super::Foundation::PWSTR, nmaxchars: u32) -> ::windows::core::Result<()>;
    fn GetManufacturer(&mut self, pwszname: super::super::Foundation::PWSTR, nmaxchars: u32) -> ::windows::core::Result<()>;
    fn GetVersion(&mut self) -> ::windows::core::Result<u32>;
    fn GetType(&mut self) -> ::windows::core::Result<u32>;
    fn GetSerialNumber(&mut self, pserialnumber: *mut WMDMID, abmac: *mut u8) -> ::windows::core::Result<()>;
    fn GetPowerSource(&mut self, pdwpowersource: *mut u32, pdwpercentremaining: *mut u32) -> ::windows::core::Result<()>;
    fn GetStatus(&mut self) -> ::windows::core::Result<u32>;
    fn GetDeviceIcon(&mut self) -> ::windows::core::Result<u32>;
    fn EnumStorage(&mut self) -> ::windows::core::Result<IWMDMEnumStorage>;
    fn GetFormatSupport(&mut self, ppformatex: *mut *mut _WAVEFORMATEX, pnformatcount: *mut u32, pppwszmimetype: *mut *mut super::super::Foundation::PWSTR, pnmimetypecount: *mut u32) -> ::windows::core::Result<()>;
    fn SendOpaqueCommand(&mut self, pcommand: *mut OPAQUECOMMAND) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IWMDMDeviceVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMDMDeviceImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWMDMDeviceVtbl {
        unsafe extern "system" fn GetName<Impl: IWMDMDeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwszname: super::super::Foundation::PWSTR, nmaxchars: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetName(::core::mem::transmute_copy(&pwszname), ::core::mem::transmute_copy(&nmaxchars)).into()
        }
        unsafe extern "system" fn GetManufacturer<Impl: IWMDMDeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwszname: super::super::Foundation::PWSTR, nmaxchars: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetManufacturer(::core::mem::transmute_copy(&pwszname), ::core::mem::transmute_copy(&nmaxchars)).into()
        }
        unsafe extern "system" fn GetVersion<Impl: IWMDMDeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwversion: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetVersion() {
                ::core::result::Result::Ok(ok__) => {
                    *pdwversion = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetType<Impl: IWMDMDeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwtype: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetType() {
                ::core::result::Result::Ok(ok__) => {
                    *pdwtype = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSerialNumber<Impl: IWMDMDeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pserialnumber: *mut WMDMID, abmac: *mut u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetSerialNumber(::core::mem::transmute_copy(&pserialnumber), ::core::mem::transmute_copy(&abmac)).into()
        }
        unsafe extern "system" fn GetPowerSource<Impl: IWMDMDeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwpowersource: *mut u32, pdwpercentremaining: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetPowerSource(::core::mem::transmute_copy(&pdwpowersource), ::core::mem::transmute_copy(&pdwpercentremaining)).into()
        }
        unsafe extern "system" fn GetStatus<Impl: IWMDMDeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwstatus: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetStatus() {
                ::core::result::Result::Ok(ok__) => {
                    *pdwstatus = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDeviceIcon<Impl: IWMDMDeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hicon: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDeviceIcon() {
                ::core::result::Result::Ok(ok__) => {
                    *hicon = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnumStorage<Impl: IWMDMDeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenumstorage: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EnumStorage() {
                ::core::result::Result::Ok(ok__) => {
                    *ppenumstorage = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFormatSupport<Impl: IWMDMDeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppformatex: *mut *mut _WAVEFORMATEX, pnformatcount: *mut u32, pppwszmimetype: *mut *mut super::super::Foundation::PWSTR, pnmimetypecount: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetFormatSupport(::core::mem::transmute_copy(&ppformatex), ::core::mem::transmute_copy(&pnformatcount), ::core::mem::transmute_copy(&pppwszmimetype), ::core::mem::transmute_copy(&pnmimetypecount)).into()
        }
        unsafe extern "system" fn SendOpaqueCommand<Impl: IWMDMDeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcommand: *mut OPAQUECOMMAND) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SendOpaqueCommand(::core::mem::transmute_copy(&pcommand)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            GetName: GetName::<Impl, IMPL_OFFSET>,
            GetManufacturer: GetManufacturer::<Impl, IMPL_OFFSET>,
            GetVersion: GetVersion::<Impl, IMPL_OFFSET>,
            GetType: GetType::<Impl, IMPL_OFFSET>,
            GetSerialNumber: GetSerialNumber::<Impl, IMPL_OFFSET>,
            GetPowerSource: GetPowerSource::<Impl, IMPL_OFFSET>,
            GetStatus: GetStatus::<Impl, IMPL_OFFSET>,
            GetDeviceIcon: GetDeviceIcon::<Impl, IMPL_OFFSET>,
            EnumStorage: EnumStorage::<Impl, IMPL_OFFSET>,
            GetFormatSupport: GetFormatSupport::<Impl, IMPL_OFFSET>,
            SendOpaqueCommand: SendOpaqueCommand::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWMDMDevice as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
pub trait IWMDMDevice2Impl: Sized + IWMDMDeviceImpl {
    fn GetStorage(&mut self, pszstoragename: super::super::Foundation::PWSTR) -> ::windows::core::Result<IWMDMStorage>;
    fn GetFormatSupport2(&mut self, dwflags: u32, ppaudioformatex: *mut *mut _WAVEFORMATEX, pnaudioformatcount: *mut u32, ppvideoformatex: *mut *mut _VIDEOINFOHEADER, pnvideoformatcount: *mut u32, ppfiletype: *mut *mut WMFILECAPABILITIES, pnfiletypecount: *mut u32) -> ::windows::core::Result<()>;
    fn GetSpecifyPropertyPages(&mut self, ppspecifyproppages: *mut ::core::option::Option<super::super::System::Ole::ISpecifyPropertyPages>, pppunknowns: *mut *mut ::core::option::Option<::windows::core::IUnknown>, pcunks: *mut u32) -> ::windows::core::Result<()>;
    fn GetCanonicalName(&mut self, pwszpnpname: super::super::Foundation::PWSTR, nmaxchars: u32) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
impl IWMDMDevice2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMDMDevice2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWMDMDevice2Vtbl {
        unsafe extern "system" fn GetStorage<Impl: IWMDMDevice2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszstoragename: super::super::Foundation::PWSTR, ppstorage: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetStorage(::core::mem::transmute_copy(&pszstoragename)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppstorage = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFormatSupport2<Impl: IWMDMDevice2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwflags: u32, ppaudioformatex: *mut *mut _WAVEFORMATEX, pnaudioformatcount: *mut u32, ppvideoformatex: *mut *mut _VIDEOINFOHEADER, pnvideoformatcount: *mut u32, ppfiletype: *mut *mut WMFILECAPABILITIES, pnfiletypecount: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetFormatSupport2(::core::mem::transmute_copy(&dwflags), ::core::mem::transmute_copy(&ppaudioformatex), ::core::mem::transmute_copy(&pnaudioformatcount), ::core::mem::transmute_copy(&ppvideoformatex), ::core::mem::transmute_copy(&pnvideoformatcount), ::core::mem::transmute_copy(&ppfiletype), ::core::mem::transmute_copy(&pnfiletypecount)).into()
        }
        unsafe extern "system" fn GetSpecifyPropertyPages<Impl: IWMDMDevice2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppspecifyproppages: *mut ::windows::core::RawPtr, pppunknowns: *mut *mut *mut ::core::ffi::c_void, pcunks: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetSpecifyPropertyPages(::core::mem::transmute_copy(&ppspecifyproppages), ::core::mem::transmute_copy(&pppunknowns), ::core::mem::transmute_copy(&pcunks)).into()
        }
        unsafe extern "system" fn GetCanonicalName<Impl: IWMDMDevice2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwszpnpname: super::super::Foundation::PWSTR, nmaxchars: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetCanonicalName(::core::mem::transmute_copy(&pwszpnpname), ::core::mem::transmute_copy(&nmaxchars)).into()
        }
        Self {
            base: IWMDMDeviceVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            GetStorage: GetStorage::<Impl, IMPL_OFFSET>,
            GetFormatSupport2: GetFormatSupport2::<Impl, IMPL_OFFSET>,
            GetSpecifyPropertyPages: GetSpecifyPropertyPages::<Impl, IMPL_OFFSET>,
            GetCanonicalName: GetCanonicalName::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWMDMDevice2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Ole"))]
pub trait IWMDMDevice3Impl: Sized + IWMDMDeviceImpl + IWMDMDevice2Impl {
    fn GetProperty(&mut self, pwszpropname: super::super::Foundation::PWSTR) -> ::windows::core::Result<super::super::System::Com::StructuredStorage::PROPVARIANT>;
    fn SetProperty(&mut self, pwszpropname: super::super::Foundation::PWSTR, pvalue: *const super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows::core::Result<()>;
    fn GetFormatCapability(&mut self, format: WMDM_FORMATCODE) -> ::windows::core::Result<WMDM_FORMAT_CAPABILITY>;
    fn DeviceIoControl(&mut self, dwiocontrolcode: u32, lpinbuffer: *const u8, ninbuffersize: u32, lpoutbuffer: *mut u8, pnoutbuffersize: *mut u32) -> ::windows::core::Result<()>;
    fn FindStorage(&mut self, findscope: WMDM_FIND_SCOPE, pwszuniqueid: super::super::Foundation::PWSTR) -> ::windows::core::Result<IWMDMStorage>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Ole"))]
impl IWMDMDevice3Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMDMDevice3Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWMDMDevice3Vtbl {
        unsafe extern "system" fn GetProperty<Impl: IWMDMDevice3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwszpropname: super::super::Foundation::PWSTR, pvalue: *mut super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetProperty(::core::mem::transmute_copy(&pwszpropname)) {
                ::core::result::Result::Ok(ok__) => {
                    *pvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetProperty<Impl: IWMDMDevice3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwszpropname: super::super::Foundation::PWSTR, pvalue: *const super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetProperty(::core::mem::transmute_copy(&pwszpropname), ::core::mem::transmute_copy(&pvalue)).into()
        }
        unsafe extern "system" fn GetFormatCapability<Impl: IWMDMDevice3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, format: WMDM_FORMATCODE, pformatsupport: *mut WMDM_FORMAT_CAPABILITY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetFormatCapability(::core::mem::transmute_copy(&format)) {
                ::core::result::Result::Ok(ok__) => {
                    *pformatsupport = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DeviceIoControl<Impl: IWMDMDevice3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwiocontrolcode: u32, lpinbuffer: *const u8, ninbuffersize: u32, lpoutbuffer: *mut u8, pnoutbuffersize: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).DeviceIoControl(::core::mem::transmute_copy(&dwiocontrolcode), ::core::mem::transmute_copy(&lpinbuffer), ::core::mem::transmute_copy(&ninbuffersize), ::core::mem::transmute_copy(&lpoutbuffer), ::core::mem::transmute_copy(&pnoutbuffersize)).into()
        }
        unsafe extern "system" fn FindStorage<Impl: IWMDMDevice3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, findscope: WMDM_FIND_SCOPE, pwszuniqueid: super::super::Foundation::PWSTR, ppstorage: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FindStorage(::core::mem::transmute_copy(&findscope), ::core::mem::transmute_copy(&pwszuniqueid)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppstorage = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: IWMDMDevice2Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            GetProperty: GetProperty::<Impl, IMPL_OFFSET>,
            SetProperty: SetProperty::<Impl, IMPL_OFFSET>,
            GetFormatCapability: GetFormatCapability::<Impl, IMPL_OFFSET>,
            DeviceIoControl: DeviceIoControl::<Impl, IMPL_OFFSET>,
            FindStorage: FindStorage::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWMDMDevice3 as ::windows::core::Interface>::IID
    }
}
pub trait IWMDMDeviceControlImpl: Sized {
    fn GetStatus(&mut self) -> ::windows::core::Result<u32>;
    fn GetCapabilities(&mut self) -> ::windows::core::Result<u32>;
    fn Play(&mut self) -> ::windows::core::Result<()>;
    fn Record(&mut self, pformat: *const _WAVEFORMATEX) -> ::windows::core::Result<()>;
    fn Pause(&mut self) -> ::windows::core::Result<()>;
    fn Resume(&mut self) -> ::windows::core::Result<()>;
    fn Stop(&mut self) -> ::windows::core::Result<()>;
    fn Seek(&mut self, fumode: u32, noffset: i32) -> ::windows::core::Result<()>;
}
impl IWMDMDeviceControlVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMDMDeviceControlImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWMDMDeviceControlVtbl {
        unsafe extern "system" fn GetStatus<Impl: IWMDMDeviceControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwstatus: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetStatus() {
                ::core::result::Result::Ok(ok__) => {
                    *pdwstatus = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCapabilities<Impl: IWMDMDeviceControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwcapabilitiesmask: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetCapabilities() {
                ::core::result::Result::Ok(ok__) => {
                    *pdwcapabilitiesmask = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Play<Impl: IWMDMDeviceControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Play().into()
        }
        unsafe extern "system" fn Record<Impl: IWMDMDeviceControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pformat: *const _WAVEFORMATEX) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Record(::core::mem::transmute_copy(&pformat)).into()
        }
        unsafe extern "system" fn Pause<Impl: IWMDMDeviceControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Pause().into()
        }
        unsafe extern "system" fn Resume<Impl: IWMDMDeviceControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Resume().into()
        }
        unsafe extern "system" fn Stop<Impl: IWMDMDeviceControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Stop().into()
        }
        unsafe extern "system" fn Seek<Impl: IWMDMDeviceControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fumode: u32, noffset: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Seek(::core::mem::transmute_copy(&fumode), ::core::mem::transmute_copy(&noffset)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            GetStatus: GetStatus::<Impl, IMPL_OFFSET>,
            GetCapabilities: GetCapabilities::<Impl, IMPL_OFFSET>,
            Play: Play::<Impl, IMPL_OFFSET>,
            Record: Record::<Impl, IMPL_OFFSET>,
            Pause: Pause::<Impl, IMPL_OFFSET>,
            Resume: Resume::<Impl, IMPL_OFFSET>,
            Stop: Stop::<Impl, IMPL_OFFSET>,
            Seek: Seek::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWMDMDeviceControl as ::windows::core::Interface>::IID
    }
}
pub trait IWMDMDeviceSessionImpl: Sized {
    fn BeginSession(&mut self, r#type: WMDM_SESSION_TYPE, pctx: *const u8, dwsizectx: u32) -> ::windows::core::Result<()>;
    fn EndSession(&mut self, r#type: WMDM_SESSION_TYPE, pctx: *const u8, dwsizectx: u32) -> ::windows::core::Result<()>;
}
impl IWMDMDeviceSessionVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMDMDeviceSessionImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWMDMDeviceSessionVtbl {
        unsafe extern "system" fn BeginSession<Impl: IWMDMDeviceSessionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, r#type: WMDM_SESSION_TYPE, pctx: *const u8, dwsizectx: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).BeginSession(::core::mem::transmute_copy(&r#type), ::core::mem::transmute_copy(&pctx), ::core::mem::transmute_copy(&dwsizectx)).into()
        }
        unsafe extern "system" fn EndSession<Impl: IWMDMDeviceSessionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, r#type: WMDM_SESSION_TYPE, pctx: *const u8, dwsizectx: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).EndSession(::core::mem::transmute_copy(&r#type), ::core::mem::transmute_copy(&pctx), ::core::mem::transmute_copy(&dwsizectx)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            BeginSession: BeginSession::<Impl, IMPL_OFFSET>,
            EndSession: EndSession::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWMDMDeviceSession as ::windows::core::Interface>::IID
    }
}
pub trait IWMDMEnumDeviceImpl: Sized {
    fn Next(&mut self, celt: u32, ppdevice: *mut ::core::option::Option<IWMDMDevice>, pceltfetched: *mut u32) -> ::windows::core::Result<()>;
    fn Skip(&mut self, celt: u32) -> ::windows::core::Result<u32>;
    fn Reset(&mut self) -> ::windows::core::Result<()>;
    fn Clone(&mut self) -> ::windows::core::Result<IWMDMEnumDevice>;
}
impl IWMDMEnumDeviceVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMDMEnumDeviceImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWMDMEnumDeviceVtbl {
        unsafe extern "system" fn Next<Impl: IWMDMEnumDeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32, ppdevice: *mut ::windows::core::RawPtr, pceltfetched: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Next(::core::mem::transmute_copy(&celt), ::core::mem::transmute_copy(&ppdevice), ::core::mem::transmute_copy(&pceltfetched)).into()
        }
        unsafe extern "system" fn Skip<Impl: IWMDMEnumDeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32, pceltfetched: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Skip(::core::mem::transmute_copy(&celt)) {
                ::core::result::Result::Ok(ok__) => {
                    *pceltfetched = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Reset<Impl: IWMDMEnumDeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Reset().into()
        }
        unsafe extern "system" fn Clone<Impl: IWMDMEnumDeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenumdevice: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Clone() {
                ::core::result::Result::Ok(ok__) => {
                    *ppenumdevice = ::core::mem::transmute(ok__);
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
        iid == &<IWMDMEnumDevice as ::windows::core::Interface>::IID
    }
}
pub trait IWMDMEnumStorageImpl: Sized {
    fn Next(&mut self, celt: u32, ppstorage: *mut ::core::option::Option<IWMDMStorage>, pceltfetched: *mut u32) -> ::windows::core::Result<()>;
    fn Skip(&mut self, celt: u32) -> ::windows::core::Result<u32>;
    fn Reset(&mut self) -> ::windows::core::Result<()>;
    fn Clone(&mut self) -> ::windows::core::Result<IWMDMEnumStorage>;
}
impl IWMDMEnumStorageVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMDMEnumStorageImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWMDMEnumStorageVtbl {
        unsafe extern "system" fn Next<Impl: IWMDMEnumStorageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32, ppstorage: *mut ::windows::core::RawPtr, pceltfetched: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Next(::core::mem::transmute_copy(&celt), ::core::mem::transmute_copy(&ppstorage), ::core::mem::transmute_copy(&pceltfetched)).into()
        }
        unsafe extern "system" fn Skip<Impl: IWMDMEnumStorageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32, pceltfetched: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Skip(::core::mem::transmute_copy(&celt)) {
                ::core::result::Result::Ok(ok__) => {
                    *pceltfetched = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Reset<Impl: IWMDMEnumStorageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Reset().into()
        }
        unsafe extern "system" fn Clone<Impl: IWMDMEnumStorageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenumstorage: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Clone() {
                ::core::result::Result::Ok(ok__) => {
                    *ppenumstorage = ::core::mem::transmute(ok__);
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
        iid == &<IWMDMEnumStorage as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IWMDMLoggerImpl: Sized {
    fn IsEnabled(&mut self) -> ::windows::core::Result<super::super::Foundation::BOOL>;
    fn Enable(&mut self, fenable: super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn GetLogFileName(&mut self, pszfilename: super::super::Foundation::PSTR, nmaxchars: u32) -> ::windows::core::Result<()>;
    fn SetLogFileName(&mut self, pszfilename: super::super::Foundation::PSTR) -> ::windows::core::Result<()>;
    fn LogString(&mut self, dwflags: u32, pszsrcname: super::super::Foundation::PSTR, pszlog: super::super::Foundation::PSTR) -> ::windows::core::Result<()>;
    fn LogDword(&mut self, dwflags: u32, pszsrcname: super::super::Foundation::PSTR, pszlogformat: super::super::Foundation::PSTR, dwlog: u32) -> ::windows::core::Result<()>;
    fn Reset(&mut self) -> ::windows::core::Result<()>;
    fn GetSizeParams(&mut self, pdwmaxsize: *mut u32, pdwshrinktosize: *mut u32) -> ::windows::core::Result<()>;
    fn SetSizeParams(&mut self, dwmaxsize: u32, dwshrinktosize: u32) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IWMDMLoggerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMDMLoggerImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWMDMLoggerVtbl {
        unsafe extern "system" fn IsEnabled<Impl: IWMDMLoggerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfenabled: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsEnabled() {
                ::core::result::Result::Ok(ok__) => {
                    *pfenabled = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Enable<Impl: IWMDMLoggerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fenable: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Enable(::core::mem::transmute_copy(&fenable)).into()
        }
        unsafe extern "system" fn GetLogFileName<Impl: IWMDMLoggerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszfilename: super::super::Foundation::PSTR, nmaxchars: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetLogFileName(::core::mem::transmute_copy(&pszfilename), ::core::mem::transmute_copy(&nmaxchars)).into()
        }
        unsafe extern "system" fn SetLogFileName<Impl: IWMDMLoggerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszfilename: super::super::Foundation::PSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetLogFileName(::core::mem::transmute_copy(&pszfilename)).into()
        }
        unsafe extern "system" fn LogString<Impl: IWMDMLoggerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwflags: u32, pszsrcname: super::super::Foundation::PSTR, pszlog: super::super::Foundation::PSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).LogString(::core::mem::transmute_copy(&dwflags), ::core::mem::transmute_copy(&pszsrcname), ::core::mem::transmute_copy(&pszlog)).into()
        }
        unsafe extern "system" fn LogDword<Impl: IWMDMLoggerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwflags: u32, pszsrcname: super::super::Foundation::PSTR, pszlogformat: super::super::Foundation::PSTR, dwlog: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).LogDword(::core::mem::transmute_copy(&dwflags), ::core::mem::transmute_copy(&pszsrcname), ::core::mem::transmute_copy(&pszlogformat), ::core::mem::transmute_copy(&dwlog)).into()
        }
        unsafe extern "system" fn Reset<Impl: IWMDMLoggerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Reset().into()
        }
        unsafe extern "system" fn GetSizeParams<Impl: IWMDMLoggerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwmaxsize: *mut u32, pdwshrinktosize: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetSizeParams(::core::mem::transmute_copy(&pdwmaxsize), ::core::mem::transmute_copy(&pdwshrinktosize)).into()
        }
        unsafe extern "system" fn SetSizeParams<Impl: IWMDMLoggerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwmaxsize: u32, dwshrinktosize: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetSizeParams(::core::mem::transmute_copy(&dwmaxsize), ::core::mem::transmute_copy(&dwshrinktosize)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            IsEnabled: IsEnabled::<Impl, IMPL_OFFSET>,
            Enable: Enable::<Impl, IMPL_OFFSET>,
            GetLogFileName: GetLogFileName::<Impl, IMPL_OFFSET>,
            SetLogFileName: SetLogFileName::<Impl, IMPL_OFFSET>,
            LogString: LogString::<Impl, IMPL_OFFSET>,
            LogDword: LogDword::<Impl, IMPL_OFFSET>,
            Reset: Reset::<Impl, IMPL_OFFSET>,
            GetSizeParams: GetSizeParams::<Impl, IMPL_OFFSET>,
            SetSizeParams: SetSizeParams::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWMDMLogger as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IWMDMMetaDataImpl: Sized {
    fn AddItem(&mut self, r#type: WMDM_TAG_DATATYPE, pwsztagname: super::super::Foundation::PWSTR, pvalue: *const u8, ilength: u32) -> ::windows::core::Result<()>;
    fn QueryByName(&mut self, pwsztagname: super::super::Foundation::PWSTR, ptype: *mut WMDM_TAG_DATATYPE, pvalue: *mut *mut u8, pcblength: *mut u32) -> ::windows::core::Result<()>;
    fn QueryByIndex(&mut self, iindex: u32, ppwszname: *mut *mut u16, ptype: *mut WMDM_TAG_DATATYPE, ppvalue: *mut *mut u8, pcblength: *mut u32) -> ::windows::core::Result<()>;
    fn GetItemCount(&mut self) -> ::windows::core::Result<u32>;
}
#[cfg(feature = "Win32_Foundation")]
impl IWMDMMetaDataVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMDMMetaDataImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWMDMMetaDataVtbl {
        unsafe extern "system" fn AddItem<Impl: IWMDMMetaDataImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, r#type: WMDM_TAG_DATATYPE, pwsztagname: super::super::Foundation::PWSTR, pvalue: *const u8, ilength: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AddItem(::core::mem::transmute_copy(&r#type), ::core::mem::transmute_copy(&pwsztagname), ::core::mem::transmute_copy(&pvalue), ::core::mem::transmute_copy(&ilength)).into()
        }
        unsafe extern "system" fn QueryByName<Impl: IWMDMMetaDataImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwsztagname: super::super::Foundation::PWSTR, ptype: *mut WMDM_TAG_DATATYPE, pvalue: *mut *mut u8, pcblength: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).QueryByName(::core::mem::transmute_copy(&pwsztagname), ::core::mem::transmute_copy(&ptype), ::core::mem::transmute_copy(&pvalue), ::core::mem::transmute_copy(&pcblength)).into()
        }
        unsafe extern "system" fn QueryByIndex<Impl: IWMDMMetaDataImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, iindex: u32, ppwszname: *mut *mut u16, ptype: *mut WMDM_TAG_DATATYPE, ppvalue: *mut *mut u8, pcblength: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).QueryByIndex(::core::mem::transmute_copy(&iindex), ::core::mem::transmute_copy(&ppwszname), ::core::mem::transmute_copy(&ptype), ::core::mem::transmute_copy(&ppvalue), ::core::mem::transmute_copy(&pcblength)).into()
        }
        unsafe extern "system" fn GetItemCount<Impl: IWMDMMetaDataImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, icount: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetItemCount() {
                ::core::result::Result::Ok(ok__) => {
                    *icount = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            AddItem: AddItem::<Impl, IMPL_OFFSET>,
            QueryByName: QueryByName::<Impl, IMPL_OFFSET>,
            QueryByIndex: QueryByIndex::<Impl, IMPL_OFFSET>,
            GetItemCount: GetItemCount::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWMDMMetaData as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IWMDMNotificationImpl: Sized {
    fn WMDMMessage(&mut self, dwmessagetype: u32, pwszcanonicalname: super::super::Foundation::PWSTR) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IWMDMNotificationVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMDMNotificationImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWMDMNotificationVtbl {
        unsafe extern "system" fn WMDMMessage<Impl: IWMDMNotificationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwmessagetype: u32, pwszcanonicalname: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).WMDMMessage(::core::mem::transmute_copy(&dwmessagetype), ::core::mem::transmute_copy(&pwszcanonicalname)).into()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), WMDMMessage: WMDMMessage::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWMDMNotification as ::windows::core::Interface>::IID
    }
}
pub trait IWMDMObjectInfoImpl: Sized {
    fn GetPlayLength(&mut self) -> ::windows::core::Result<u32>;
    fn SetPlayLength(&mut self, dwlength: u32) -> ::windows::core::Result<()>;
    fn GetPlayOffset(&mut self) -> ::windows::core::Result<u32>;
    fn SetPlayOffset(&mut self, dwoffset: u32) -> ::windows::core::Result<()>;
    fn GetTotalLength(&mut self) -> ::windows::core::Result<u32>;
    fn GetLastPlayPosition(&mut self) -> ::windows::core::Result<u32>;
    fn GetLongestPlayPosition(&mut self) -> ::windows::core::Result<u32>;
}
impl IWMDMObjectInfoVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMDMObjectInfoImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWMDMObjectInfoVtbl {
        unsafe extern "system" fn GetPlayLength<Impl: IWMDMObjectInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwlength: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetPlayLength() {
                ::core::result::Result::Ok(ok__) => {
                    *pdwlength = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPlayLength<Impl: IWMDMObjectInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwlength: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetPlayLength(::core::mem::transmute_copy(&dwlength)).into()
        }
        unsafe extern "system" fn GetPlayOffset<Impl: IWMDMObjectInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwoffset: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetPlayOffset() {
                ::core::result::Result::Ok(ok__) => {
                    *pdwoffset = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPlayOffset<Impl: IWMDMObjectInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwoffset: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetPlayOffset(::core::mem::transmute_copy(&dwoffset)).into()
        }
        unsafe extern "system" fn GetTotalLength<Impl: IWMDMObjectInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwlength: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetTotalLength() {
                ::core::result::Result::Ok(ok__) => {
                    *pdwlength = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetLastPlayPosition<Impl: IWMDMObjectInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwlastpos: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetLastPlayPosition() {
                ::core::result::Result::Ok(ok__) => {
                    *pdwlastpos = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetLongestPlayPosition<Impl: IWMDMObjectInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwlongestpos: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetLongestPlayPosition() {
                ::core::result::Result::Ok(ok__) => {
                    *pdwlongestpos = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            GetPlayLength: GetPlayLength::<Impl, IMPL_OFFSET>,
            SetPlayLength: SetPlayLength::<Impl, IMPL_OFFSET>,
            GetPlayOffset: GetPlayOffset::<Impl, IMPL_OFFSET>,
            SetPlayOffset: SetPlayOffset::<Impl, IMPL_OFFSET>,
            GetTotalLength: GetTotalLength::<Impl, IMPL_OFFSET>,
            GetLastPlayPosition: GetLastPlayPosition::<Impl, IMPL_OFFSET>,
            GetLongestPlayPosition: GetLongestPlayPosition::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWMDMObjectInfo as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IWMDMOperationImpl: Sized {
    fn BeginRead(&mut self) -> ::windows::core::Result<()>;
    fn BeginWrite(&mut self) -> ::windows::core::Result<()>;
    fn GetObjectName(&mut self, pwszname: super::super::Foundation::PWSTR, nmaxchars: u32) -> ::windows::core::Result<()>;
    fn SetObjectName(&mut self, pwszname: super::super::Foundation::PWSTR, nmaxchars: u32) -> ::windows::core::Result<()>;
    fn GetObjectAttributes(&mut self, pdwattributes: *mut u32, pformat: *mut _WAVEFORMATEX) -> ::windows::core::Result<()>;
    fn SetObjectAttributes(&mut self, dwattributes: u32, pformat: *const _WAVEFORMATEX) -> ::windows::core::Result<()>;
    fn GetObjectTotalSize(&mut self, pdwsize: *mut u32, pdwsizehigh: *mut u32) -> ::windows::core::Result<()>;
    fn SetObjectTotalSize(&mut self, dwsize: u32, dwsizehigh: u32) -> ::windows::core::Result<()>;
    fn TransferObjectData(&mut self, pdata: *mut u8, pdwsize: *mut u32, abmac: *mut u8) -> ::windows::core::Result<()>;
    fn End(&mut self, phcompletioncode: *const ::windows::core::HRESULT, pnewobject: ::core::option::Option<::windows::core::IUnknown>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IWMDMOperationVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMDMOperationImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWMDMOperationVtbl {
        unsafe extern "system" fn BeginRead<Impl: IWMDMOperationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).BeginRead().into()
        }
        unsafe extern "system" fn BeginWrite<Impl: IWMDMOperationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).BeginWrite().into()
        }
        unsafe extern "system" fn GetObjectName<Impl: IWMDMOperationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwszname: super::super::Foundation::PWSTR, nmaxchars: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetObjectName(::core::mem::transmute_copy(&pwszname), ::core::mem::transmute_copy(&nmaxchars)).into()
        }
        unsafe extern "system" fn SetObjectName<Impl: IWMDMOperationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwszname: super::super::Foundation::PWSTR, nmaxchars: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetObjectName(::core::mem::transmute_copy(&pwszname), ::core::mem::transmute_copy(&nmaxchars)).into()
        }
        unsafe extern "system" fn GetObjectAttributes<Impl: IWMDMOperationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwattributes: *mut u32, pformat: *mut _WAVEFORMATEX) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetObjectAttributes(::core::mem::transmute_copy(&pdwattributes), ::core::mem::transmute_copy(&pformat)).into()
        }
        unsafe extern "system" fn SetObjectAttributes<Impl: IWMDMOperationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwattributes: u32, pformat: *const _WAVEFORMATEX) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetObjectAttributes(::core::mem::transmute_copy(&dwattributes), ::core::mem::transmute_copy(&pformat)).into()
        }
        unsafe extern "system" fn GetObjectTotalSize<Impl: IWMDMOperationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwsize: *mut u32, pdwsizehigh: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetObjectTotalSize(::core::mem::transmute_copy(&pdwsize), ::core::mem::transmute_copy(&pdwsizehigh)).into()
        }
        unsafe extern "system" fn SetObjectTotalSize<Impl: IWMDMOperationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwsize: u32, dwsizehigh: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetObjectTotalSize(::core::mem::transmute_copy(&dwsize), ::core::mem::transmute_copy(&dwsizehigh)).into()
        }
        unsafe extern "system" fn TransferObjectData<Impl: IWMDMOperationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdata: *mut u8, pdwsize: *mut u32, abmac: *mut u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).TransferObjectData(::core::mem::transmute_copy(&pdata), ::core::mem::transmute_copy(&pdwsize), ::core::mem::transmute_copy(&abmac)).into()
        }
        unsafe extern "system" fn End<Impl: IWMDMOperationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, phcompletioncode: *const ::windows::core::HRESULT, pnewobject: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).End(::core::mem::transmute_copy(&phcompletioncode), ::core::mem::transmute(&pnewobject)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            BeginRead: BeginRead::<Impl, IMPL_OFFSET>,
            BeginWrite: BeginWrite::<Impl, IMPL_OFFSET>,
            GetObjectName: GetObjectName::<Impl, IMPL_OFFSET>,
            SetObjectName: SetObjectName::<Impl, IMPL_OFFSET>,
            GetObjectAttributes: GetObjectAttributes::<Impl, IMPL_OFFSET>,
            SetObjectAttributes: SetObjectAttributes::<Impl, IMPL_OFFSET>,
            GetObjectTotalSize: GetObjectTotalSize::<Impl, IMPL_OFFSET>,
            SetObjectTotalSize: SetObjectTotalSize::<Impl, IMPL_OFFSET>,
            TransferObjectData: TransferObjectData::<Impl, IMPL_OFFSET>,
            End: End::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWMDMOperation as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IWMDMOperation2Impl: Sized + IWMDMOperationImpl {
    fn SetObjectAttributes2(&mut self, dwattributes: u32, dwattributesex: u32, pformat: *const _WAVEFORMATEX, pvideoformat: *const _VIDEOINFOHEADER) -> ::windows::core::Result<()>;
    fn GetObjectAttributes2(&mut self, pdwattributes: *mut u32, pdwattributesex: *mut u32, paudioformat: *mut _WAVEFORMATEX, pvideoformat: *mut _VIDEOINFOHEADER) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IWMDMOperation2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMDMOperation2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWMDMOperation2Vtbl {
        unsafe extern "system" fn SetObjectAttributes2<Impl: IWMDMOperation2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwattributes: u32, dwattributesex: u32, pformat: *const _WAVEFORMATEX, pvideoformat: *const _VIDEOINFOHEADER) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetObjectAttributes2(::core::mem::transmute_copy(&dwattributes), ::core::mem::transmute_copy(&dwattributesex), ::core::mem::transmute_copy(&pformat), ::core::mem::transmute_copy(&pvideoformat)).into()
        }
        unsafe extern "system" fn GetObjectAttributes2<Impl: IWMDMOperation2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwattributes: *mut u32, pdwattributesex: *mut u32, paudioformat: *mut _WAVEFORMATEX, pvideoformat: *mut _VIDEOINFOHEADER) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetObjectAttributes2(::core::mem::transmute_copy(&pdwattributes), ::core::mem::transmute_copy(&pdwattributesex), ::core::mem::transmute_copy(&paudioformat), ::core::mem::transmute_copy(&pvideoformat)).into()
        }
        Self {
            base: IWMDMOperationVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            SetObjectAttributes2: SetObjectAttributes2::<Impl, IMPL_OFFSET>,
            GetObjectAttributes2: GetObjectAttributes2::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWMDMOperation2 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IWMDMOperation3Impl: Sized + IWMDMOperationImpl {
    fn TransferObjectDataOnClearChannel(&mut self, pdata: *mut u8, pdwsize: *mut u32) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IWMDMOperation3Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMDMOperation3Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWMDMOperation3Vtbl {
        unsafe extern "system" fn TransferObjectDataOnClearChannel<Impl: IWMDMOperation3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdata: *mut u8, pdwsize: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).TransferObjectDataOnClearChannel(::core::mem::transmute_copy(&pdata), ::core::mem::transmute_copy(&pdwsize)).into()
        }
        Self {
            base: IWMDMOperationVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            TransferObjectDataOnClearChannel: TransferObjectDataOnClearChannel::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWMDMOperation3 as ::windows::core::Interface>::IID
    }
}
pub trait IWMDMProgressImpl: Sized {
    fn Begin(&mut self, dwestimatedticks: u32) -> ::windows::core::Result<()>;
    fn Progress(&mut self, dwtranspiredticks: u32) -> ::windows::core::Result<()>;
    fn End(&mut self) -> ::windows::core::Result<()>;
}
impl IWMDMProgressVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMDMProgressImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWMDMProgressVtbl {
        unsafe extern "system" fn Begin<Impl: IWMDMProgressImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwestimatedticks: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Begin(::core::mem::transmute_copy(&dwestimatedticks)).into()
        }
        unsafe extern "system" fn Progress<Impl: IWMDMProgressImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwtranspiredticks: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Progress(::core::mem::transmute_copy(&dwtranspiredticks)).into()
        }
        unsafe extern "system" fn End<Impl: IWMDMProgressImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).End().into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            Begin: Begin::<Impl, IMPL_OFFSET>,
            Progress: Progress::<Impl, IMPL_OFFSET>,
            End: End::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWMDMProgress as ::windows::core::Interface>::IID
    }
}
pub trait IWMDMProgress2Impl: Sized + IWMDMProgressImpl {
    fn End2(&mut self, hrcompletioncode: ::windows::core::HRESULT) -> ::windows::core::Result<()>;
}
impl IWMDMProgress2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMDMProgress2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWMDMProgress2Vtbl {
        unsafe extern "system" fn End2<Impl: IWMDMProgress2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hrcompletioncode: ::windows::core::HRESULT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).End2(::core::mem::transmute_copy(&hrcompletioncode)).into()
        }
        Self { base: IWMDMProgressVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(), End2: End2::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWMDMProgress2 as ::windows::core::Interface>::IID
    }
}
pub trait IWMDMProgress3Impl: Sized + IWMDMProgressImpl + IWMDMProgress2Impl {
    fn Begin3(&mut self, eventid: ::windows::core::GUID, dwestimatedticks: u32, pcontext: *mut OPAQUECOMMAND) -> ::windows::core::Result<()>;
    fn Progress3(&mut self, eventid: ::windows::core::GUID, dwtranspiredticks: u32, pcontext: *mut OPAQUECOMMAND) -> ::windows::core::Result<()>;
    fn End3(&mut self, eventid: ::windows::core::GUID, hrcompletioncode: ::windows::core::HRESULT, pcontext: *mut OPAQUECOMMAND) -> ::windows::core::Result<()>;
}
impl IWMDMProgress3Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMDMProgress3Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWMDMProgress3Vtbl {
        unsafe extern "system" fn Begin3<Impl: IWMDMProgress3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, eventid: ::windows::core::GUID, dwestimatedticks: u32, pcontext: *mut OPAQUECOMMAND) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Begin3(::core::mem::transmute_copy(&eventid), ::core::mem::transmute_copy(&dwestimatedticks), ::core::mem::transmute_copy(&pcontext)).into()
        }
        unsafe extern "system" fn Progress3<Impl: IWMDMProgress3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, eventid: ::windows::core::GUID, dwtranspiredticks: u32, pcontext: *mut OPAQUECOMMAND) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Progress3(::core::mem::transmute_copy(&eventid), ::core::mem::transmute_copy(&dwtranspiredticks), ::core::mem::transmute_copy(&pcontext)).into()
        }
        unsafe extern "system" fn End3<Impl: IWMDMProgress3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, eventid: ::windows::core::GUID, hrcompletioncode: ::windows::core::HRESULT, pcontext: *mut OPAQUECOMMAND) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).End3(::core::mem::transmute_copy(&eventid), ::core::mem::transmute_copy(&hrcompletioncode), ::core::mem::transmute_copy(&pcontext)).into()
        }
        Self {
            base: IWMDMProgress2Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            Begin3: Begin3::<Impl, IMPL_OFFSET>,
            Progress3: Progress3::<Impl, IMPL_OFFSET>,
            End3: End3::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWMDMProgress3 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IWMDMRevokedImpl: Sized {
    fn GetRevocationURL(&mut self, ppwszrevocationurl: *mut super::super::Foundation::PWSTR, pdwbufferlen: *mut u32, pdwrevokedbitflag: *mut u32) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IWMDMRevokedVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMDMRevokedImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWMDMRevokedVtbl {
        unsafe extern "system" fn GetRevocationURL<Impl: IWMDMRevokedImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppwszrevocationurl: *mut super::super::Foundation::PWSTR, pdwbufferlen: *mut u32, pdwrevokedbitflag: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetRevocationURL(::core::mem::transmute_copy(&ppwszrevocationurl), ::core::mem::transmute_copy(&pdwbufferlen), ::core::mem::transmute_copy(&pdwrevokedbitflag)).into()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), GetRevocationURL: GetRevocationURL::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWMDMRevoked as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IWMDMStorageImpl: Sized {
    fn SetAttributes(&mut self, dwattributes: u32, pformat: *const _WAVEFORMATEX) -> ::windows::core::Result<()>;
    fn GetStorageGlobals(&mut self) -> ::windows::core::Result<IWMDMStorageGlobals>;
    fn GetAttributes(&mut self, pdwattributes: *mut u32, pformat: *mut _WAVEFORMATEX) -> ::windows::core::Result<()>;
    fn GetName(&mut self, pwszname: super::super::Foundation::PWSTR, nmaxchars: u32) -> ::windows::core::Result<()>;
    fn GetDate(&mut self) -> ::windows::core::Result<WMDMDATETIME>;
    fn GetSize(&mut self, pdwsizelow: *mut u32, pdwsizehigh: *mut u32) -> ::windows::core::Result<()>;
    fn GetRights(&mut self, pprights: *mut *mut WMDMRIGHTS, pnrightscount: *mut u32, abmac: *mut u8) -> ::windows::core::Result<()>;
    fn EnumStorage(&mut self) -> ::windows::core::Result<IWMDMEnumStorage>;
    fn SendOpaqueCommand(&mut self, pcommand: *mut OPAQUECOMMAND) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IWMDMStorageVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMDMStorageImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWMDMStorageVtbl {
        unsafe extern "system" fn SetAttributes<Impl: IWMDMStorageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwattributes: u32, pformat: *const _WAVEFORMATEX) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetAttributes(::core::mem::transmute_copy(&dwattributes), ::core::mem::transmute_copy(&pformat)).into()
        }
        unsafe extern "system" fn GetStorageGlobals<Impl: IWMDMStorageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppstorageglobals: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetStorageGlobals() {
                ::core::result::Result::Ok(ok__) => {
                    *ppstorageglobals = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetAttributes<Impl: IWMDMStorageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwattributes: *mut u32, pformat: *mut _WAVEFORMATEX) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetAttributes(::core::mem::transmute_copy(&pdwattributes), ::core::mem::transmute_copy(&pformat)).into()
        }
        unsafe extern "system" fn GetName<Impl: IWMDMStorageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwszname: super::super::Foundation::PWSTR, nmaxchars: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetName(::core::mem::transmute_copy(&pwszname), ::core::mem::transmute_copy(&nmaxchars)).into()
        }
        unsafe extern "system" fn GetDate<Impl: IWMDMStorageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdatetimeutc: *mut WMDMDATETIME) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDate() {
                ::core::result::Result::Ok(ok__) => {
                    *pdatetimeutc = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSize<Impl: IWMDMStorageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwsizelow: *mut u32, pdwsizehigh: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetSize(::core::mem::transmute_copy(&pdwsizelow), ::core::mem::transmute_copy(&pdwsizehigh)).into()
        }
        unsafe extern "system" fn GetRights<Impl: IWMDMStorageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pprights: *mut *mut WMDMRIGHTS, pnrightscount: *mut u32, abmac: *mut u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetRights(::core::mem::transmute_copy(&pprights), ::core::mem::transmute_copy(&pnrightscount), ::core::mem::transmute_copy(&abmac)).into()
        }
        unsafe extern "system" fn EnumStorage<Impl: IWMDMStorageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, penumstorage: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EnumStorage() {
                ::core::result::Result::Ok(ok__) => {
                    *penumstorage = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SendOpaqueCommand<Impl: IWMDMStorageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcommand: *mut OPAQUECOMMAND) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SendOpaqueCommand(::core::mem::transmute_copy(&pcommand)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            SetAttributes: SetAttributes::<Impl, IMPL_OFFSET>,
            GetStorageGlobals: GetStorageGlobals::<Impl, IMPL_OFFSET>,
            GetAttributes: GetAttributes::<Impl, IMPL_OFFSET>,
            GetName: GetName::<Impl, IMPL_OFFSET>,
            GetDate: GetDate::<Impl, IMPL_OFFSET>,
            GetSize: GetSize::<Impl, IMPL_OFFSET>,
            GetRights: GetRights::<Impl, IMPL_OFFSET>,
            EnumStorage: EnumStorage::<Impl, IMPL_OFFSET>,
            SendOpaqueCommand: SendOpaqueCommand::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWMDMStorage as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IWMDMStorage2Impl: Sized + IWMDMStorageImpl {
    fn GetStorage(&mut self, pszstoragename: super::super::Foundation::PWSTR) -> ::windows::core::Result<IWMDMStorage>;
    fn SetAttributes2(&mut self, dwattributes: u32, dwattributesex: u32, pformat: *const _WAVEFORMATEX, pvideoformat: *const _VIDEOINFOHEADER) -> ::windows::core::Result<()>;
    fn GetAttributes2(&mut self, pdwattributes: *mut u32, pdwattributesex: *mut u32, paudioformat: *mut _WAVEFORMATEX, pvideoformat: *mut _VIDEOINFOHEADER) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IWMDMStorage2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMDMStorage2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWMDMStorage2Vtbl {
        unsafe extern "system" fn GetStorage<Impl: IWMDMStorage2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszstoragename: super::super::Foundation::PWSTR, ppstorage: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetStorage(::core::mem::transmute_copy(&pszstoragename)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppstorage = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAttributes2<Impl: IWMDMStorage2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwattributes: u32, dwattributesex: u32, pformat: *const _WAVEFORMATEX, pvideoformat: *const _VIDEOINFOHEADER) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetAttributes2(::core::mem::transmute_copy(&dwattributes), ::core::mem::transmute_copy(&dwattributesex), ::core::mem::transmute_copy(&pformat), ::core::mem::transmute_copy(&pvideoformat)).into()
        }
        unsafe extern "system" fn GetAttributes2<Impl: IWMDMStorage2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwattributes: *mut u32, pdwattributesex: *mut u32, paudioformat: *mut _WAVEFORMATEX, pvideoformat: *mut _VIDEOINFOHEADER) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetAttributes2(::core::mem::transmute_copy(&pdwattributes), ::core::mem::transmute_copy(&pdwattributesex), ::core::mem::transmute_copy(&paudioformat), ::core::mem::transmute_copy(&pvideoformat)).into()
        }
        Self {
            base: IWMDMStorageVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            GetStorage: GetStorage::<Impl, IMPL_OFFSET>,
            SetAttributes2: SetAttributes2::<Impl, IMPL_OFFSET>,
            GetAttributes2: GetAttributes2::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWMDMStorage2 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IWMDMStorage3Impl: Sized + IWMDMStorageImpl + IWMDMStorage2Impl {
    fn GetMetadata(&mut self) -> ::windows::core::Result<IWMDMMetaData>;
    fn SetMetadata(&mut self, pmetadata: ::core::option::Option<IWMDMMetaData>) -> ::windows::core::Result<()>;
    fn CreateEmptyMetadataObject(&mut self) -> ::windows::core::Result<IWMDMMetaData>;
    fn SetEnumPreference(&mut self, pmode: *mut WMDM_STORAGE_ENUM_MODE, nviews: u32, pviews: *const WMDMMetadataView) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IWMDMStorage3Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMDMStorage3Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWMDMStorage3Vtbl {
        unsafe extern "system" fn GetMetadata<Impl: IWMDMStorage3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppmetadata: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetMetadata() {
                ::core::result::Result::Ok(ok__) => {
                    *ppmetadata = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMetadata<Impl: IWMDMStorage3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pmetadata: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetMetadata(::core::mem::transmute(&pmetadata)).into()
        }
        unsafe extern "system" fn CreateEmptyMetadataObject<Impl: IWMDMStorage3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppmetadata: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateEmptyMetadataObject() {
                ::core::result::Result::Ok(ok__) => {
                    *ppmetadata = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetEnumPreference<Impl: IWMDMStorage3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pmode: *mut WMDM_STORAGE_ENUM_MODE, nviews: u32, pviews: *const WMDMMetadataView) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetEnumPreference(::core::mem::transmute_copy(&pmode), ::core::mem::transmute_copy(&nviews), ::core::mem::transmute_copy(&pviews)).into()
        }
        Self {
            base: IWMDMStorage2Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            GetMetadata: GetMetadata::<Impl, IMPL_OFFSET>,
            SetMetadata: SetMetadata::<Impl, IMPL_OFFSET>,
            CreateEmptyMetadataObject: CreateEmptyMetadataObject::<Impl, IMPL_OFFSET>,
            SetEnumPreference: SetEnumPreference::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWMDMStorage3 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IWMDMStorage4Impl: Sized + IWMDMStorageImpl + IWMDMStorage2Impl + IWMDMStorage3Impl {
    fn SetReferences(&mut self, dwrefs: u32, ppiwmdmstorage: *const ::core::option::Option<IWMDMStorage>) -> ::windows::core::Result<()>;
    fn GetReferences(&mut self, pdwrefs: *mut u32, pppiwmdmstorage: *mut *mut ::core::option::Option<IWMDMStorage>) -> ::windows::core::Result<()>;
    fn GetRightsWithProgress(&mut self, piprogresscallback: ::core::option::Option<IWMDMProgress3>, pprights: *mut *mut WMDMRIGHTS, pnrightscount: *mut u32) -> ::windows::core::Result<()>;
    fn GetSpecifiedMetadata(&mut self, cproperties: u32, ppwszpropnames: *const super::super::Foundation::PWSTR) -> ::windows::core::Result<IWMDMMetaData>;
    fn FindStorage(&mut self, findscope: WMDM_FIND_SCOPE, pwszuniqueid: super::super::Foundation::PWSTR) -> ::windows::core::Result<IWMDMStorage>;
    fn GetParent(&mut self) -> ::windows::core::Result<IWMDMStorage>;
}
#[cfg(feature = "Win32_Foundation")]
impl IWMDMStorage4Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMDMStorage4Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWMDMStorage4Vtbl {
        unsafe extern "system" fn SetReferences<Impl: IWMDMStorage4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwrefs: u32, ppiwmdmstorage: *const ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetReferences(::core::mem::transmute_copy(&dwrefs), ::core::mem::transmute_copy(&ppiwmdmstorage)).into()
        }
        unsafe extern "system" fn GetReferences<Impl: IWMDMStorage4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwrefs: *mut u32, pppiwmdmstorage: *mut *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetReferences(::core::mem::transmute_copy(&pdwrefs), ::core::mem::transmute_copy(&pppiwmdmstorage)).into()
        }
        unsafe extern "system" fn GetRightsWithProgress<Impl: IWMDMStorage4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, piprogresscallback: ::windows::core::RawPtr, pprights: *mut *mut WMDMRIGHTS, pnrightscount: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetRightsWithProgress(::core::mem::transmute(&piprogresscallback), ::core::mem::transmute_copy(&pprights), ::core::mem::transmute_copy(&pnrightscount)).into()
        }
        unsafe extern "system" fn GetSpecifiedMetadata<Impl: IWMDMStorage4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cproperties: u32, ppwszpropnames: *const super::super::Foundation::PWSTR, ppmetadata: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetSpecifiedMetadata(::core::mem::transmute_copy(&cproperties), ::core::mem::transmute_copy(&ppwszpropnames)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppmetadata = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FindStorage<Impl: IWMDMStorage4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, findscope: WMDM_FIND_SCOPE, pwszuniqueid: super::super::Foundation::PWSTR, ppstorage: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FindStorage(::core::mem::transmute_copy(&findscope), ::core::mem::transmute_copy(&pwszuniqueid)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppstorage = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetParent<Impl: IWMDMStorage4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppstorage: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetParent() {
                ::core::result::Result::Ok(ok__) => {
                    *ppstorage = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: IWMDMStorage3Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            SetReferences: SetReferences::<Impl, IMPL_OFFSET>,
            GetReferences: GetReferences::<Impl, IMPL_OFFSET>,
            GetRightsWithProgress: GetRightsWithProgress::<Impl, IMPL_OFFSET>,
            GetSpecifiedMetadata: GetSpecifiedMetadata::<Impl, IMPL_OFFSET>,
            FindStorage: FindStorage::<Impl, IMPL_OFFSET>,
            GetParent: GetParent::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWMDMStorage4 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IWMDMStorageControlImpl: Sized {
    fn Insert(&mut self, fumode: u32, pwszfile: super::super::Foundation::PWSTR, poperation: ::core::option::Option<IWMDMOperation>, pprogress: ::core::option::Option<IWMDMProgress>) -> ::windows::core::Result<IWMDMStorage>;
    fn Delete(&mut self, fumode: u32, pprogress: ::core::option::Option<IWMDMProgress>) -> ::windows::core::Result<()>;
    fn Rename(&mut self, fumode: u32, pwsznewname: super::super::Foundation::PWSTR, pprogress: ::core::option::Option<IWMDMProgress>) -> ::windows::core::Result<()>;
    fn Read(&mut self, fumode: u32, pwszfile: super::super::Foundation::PWSTR, pprogress: ::core::option::Option<IWMDMProgress>, poperation: ::core::option::Option<IWMDMOperation>) -> ::windows::core::Result<()>;
    fn Move(&mut self, fumode: u32, ptargetobject: ::core::option::Option<IWMDMStorage>, pprogress: ::core::option::Option<IWMDMProgress>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IWMDMStorageControlVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMDMStorageControlImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWMDMStorageControlVtbl {
        unsafe extern "system" fn Insert<Impl: IWMDMStorageControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fumode: u32, pwszfile: super::super::Foundation::PWSTR, poperation: ::windows::core::RawPtr, pprogress: ::windows::core::RawPtr, ppnewobject: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Insert(::core::mem::transmute_copy(&fumode), ::core::mem::transmute_copy(&pwszfile), ::core::mem::transmute(&poperation), ::core::mem::transmute(&pprogress)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppnewobject = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Delete<Impl: IWMDMStorageControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fumode: u32, pprogress: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Delete(::core::mem::transmute_copy(&fumode), ::core::mem::transmute(&pprogress)).into()
        }
        unsafe extern "system" fn Rename<Impl: IWMDMStorageControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fumode: u32, pwsznewname: super::super::Foundation::PWSTR, pprogress: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Rename(::core::mem::transmute_copy(&fumode), ::core::mem::transmute_copy(&pwsznewname), ::core::mem::transmute(&pprogress)).into()
        }
        unsafe extern "system" fn Read<Impl: IWMDMStorageControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fumode: u32, pwszfile: super::super::Foundation::PWSTR, pprogress: ::windows::core::RawPtr, poperation: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Read(::core::mem::transmute_copy(&fumode), ::core::mem::transmute_copy(&pwszfile), ::core::mem::transmute(&pprogress), ::core::mem::transmute(&poperation)).into()
        }
        unsafe extern "system" fn Move<Impl: IWMDMStorageControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fumode: u32, ptargetobject: ::windows::core::RawPtr, pprogress: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Move(::core::mem::transmute_copy(&fumode), ::core::mem::transmute(&ptargetobject), ::core::mem::transmute(&pprogress)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            Insert: Insert::<Impl, IMPL_OFFSET>,
            Delete: Delete::<Impl, IMPL_OFFSET>,
            Rename: Rename::<Impl, IMPL_OFFSET>,
            Read: Read::<Impl, IMPL_OFFSET>,
            Move: Move::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWMDMStorageControl as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IWMDMStorageControl2Impl: Sized + IWMDMStorageControlImpl {
    fn Insert2(&mut self, fumode: u32, pwszfilesource: super::super::Foundation::PWSTR, pwszfiledest: super::super::Foundation::PWSTR, poperation: ::core::option::Option<IWMDMOperation>, pprogress: ::core::option::Option<IWMDMProgress>, punknown: ::core::option::Option<::windows::core::IUnknown>, ppnewobject: *mut ::core::option::Option<IWMDMStorage>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IWMDMStorageControl2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMDMStorageControl2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWMDMStorageControl2Vtbl {
        unsafe extern "system" fn Insert2<Impl: IWMDMStorageControl2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fumode: u32, pwszfilesource: super::super::Foundation::PWSTR, pwszfiledest: super::super::Foundation::PWSTR, poperation: ::windows::core::RawPtr, pprogress: ::windows::core::RawPtr, punknown: *mut ::core::ffi::c_void, ppnewobject: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Insert2(::core::mem::transmute_copy(&fumode), ::core::mem::transmute_copy(&pwszfilesource), ::core::mem::transmute_copy(&pwszfiledest), ::core::mem::transmute(&poperation), ::core::mem::transmute(&pprogress), ::core::mem::transmute(&punknown), ::core::mem::transmute_copy(&ppnewobject)).into()
        }
        Self { base: IWMDMStorageControlVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(), Insert2: Insert2::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWMDMStorageControl2 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IWMDMStorageControl3Impl: Sized + IWMDMStorageControlImpl + IWMDMStorageControl2Impl {
    fn Insert3(&mut self, fumode: u32, futype: u32, pwszfilesource: super::super::Foundation::PWSTR, pwszfiledest: super::super::Foundation::PWSTR, poperation: ::core::option::Option<IWMDMOperation>, pprogress: ::core::option::Option<IWMDMProgress>, pmetadata: ::core::option::Option<IWMDMMetaData>, punknown: ::core::option::Option<::windows::core::IUnknown>, ppnewobject: *mut ::core::option::Option<IWMDMStorage>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IWMDMStorageControl3Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMDMStorageControl3Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWMDMStorageControl3Vtbl {
        unsafe extern "system" fn Insert3<Impl: IWMDMStorageControl3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fumode: u32, futype: u32, pwszfilesource: super::super::Foundation::PWSTR, pwszfiledest: super::super::Foundation::PWSTR, poperation: ::windows::core::RawPtr, pprogress: ::windows::core::RawPtr, pmetadata: ::windows::core::RawPtr, punknown: *mut ::core::ffi::c_void, ppnewobject: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Insert3(::core::mem::transmute_copy(&fumode), ::core::mem::transmute_copy(&futype), ::core::mem::transmute_copy(&pwszfilesource), ::core::mem::transmute_copy(&pwszfiledest), ::core::mem::transmute(&poperation), ::core::mem::transmute(&pprogress), ::core::mem::transmute(&pmetadata), ::core::mem::transmute(&punknown), ::core::mem::transmute_copy(&ppnewobject)).into()
        }
        Self { base: IWMDMStorageControl2Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(), Insert3: Insert3::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWMDMStorageControl3 as ::windows::core::Interface>::IID
    }
}
pub trait IWMDMStorageGlobalsImpl: Sized {
    fn GetCapabilities(&mut self) -> ::windows::core::Result<u32>;
    fn GetSerialNumber(&mut self, pserialnum: *mut WMDMID, abmac: *mut u8) -> ::windows::core::Result<()>;
    fn GetTotalSize(&mut self, pdwtotalsizelow: *mut u32, pdwtotalsizehigh: *mut u32) -> ::windows::core::Result<()>;
    fn GetTotalFree(&mut self, pdwfreelow: *mut u32, pdwfreehigh: *mut u32) -> ::windows::core::Result<()>;
    fn GetTotalBad(&mut self, pdwbadlow: *mut u32, pdwbadhigh: *mut u32) -> ::windows::core::Result<()>;
    fn GetStatus(&mut self) -> ::windows::core::Result<u32>;
    fn Initialize(&mut self, fumode: u32, pprogress: ::core::option::Option<IWMDMProgress>) -> ::windows::core::Result<()>;
}
impl IWMDMStorageGlobalsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMDMStorageGlobalsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWMDMStorageGlobalsVtbl {
        unsafe extern "system" fn GetCapabilities<Impl: IWMDMStorageGlobalsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwcapabilities: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetCapabilities() {
                ::core::result::Result::Ok(ok__) => {
                    *pdwcapabilities = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSerialNumber<Impl: IWMDMStorageGlobalsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pserialnum: *mut WMDMID, abmac: *mut u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetSerialNumber(::core::mem::transmute_copy(&pserialnum), ::core::mem::transmute_copy(&abmac)).into()
        }
        unsafe extern "system" fn GetTotalSize<Impl: IWMDMStorageGlobalsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwtotalsizelow: *mut u32, pdwtotalsizehigh: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetTotalSize(::core::mem::transmute_copy(&pdwtotalsizelow), ::core::mem::transmute_copy(&pdwtotalsizehigh)).into()
        }
        unsafe extern "system" fn GetTotalFree<Impl: IWMDMStorageGlobalsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwfreelow: *mut u32, pdwfreehigh: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetTotalFree(::core::mem::transmute_copy(&pdwfreelow), ::core::mem::transmute_copy(&pdwfreehigh)).into()
        }
        unsafe extern "system" fn GetTotalBad<Impl: IWMDMStorageGlobalsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwbadlow: *mut u32, pdwbadhigh: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetTotalBad(::core::mem::transmute_copy(&pdwbadlow), ::core::mem::transmute_copy(&pdwbadhigh)).into()
        }
        unsafe extern "system" fn GetStatus<Impl: IWMDMStorageGlobalsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwstatus: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetStatus() {
                ::core::result::Result::Ok(ok__) => {
                    *pdwstatus = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Initialize<Impl: IWMDMStorageGlobalsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fumode: u32, pprogress: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Initialize(::core::mem::transmute_copy(&fumode), ::core::mem::transmute(&pprogress)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            GetCapabilities: GetCapabilities::<Impl, IMPL_OFFSET>,
            GetSerialNumber: GetSerialNumber::<Impl, IMPL_OFFSET>,
            GetTotalSize: GetTotalSize::<Impl, IMPL_OFFSET>,
            GetTotalFree: GetTotalFree::<Impl, IMPL_OFFSET>,
            GetTotalBad: GetTotalBad::<Impl, IMPL_OFFSET>,
            GetStatus: GetStatus::<Impl, IMPL_OFFSET>,
            Initialize: Initialize::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWMDMStorageGlobals as ::windows::core::Interface>::IID
    }
}
pub trait IWMDeviceManagerImpl: Sized {
    fn GetRevision(&mut self) -> ::windows::core::Result<u32>;
    fn GetDeviceCount(&mut self) -> ::windows::core::Result<u32>;
    fn EnumDevices(&mut self) -> ::windows::core::Result<IWMDMEnumDevice>;
}
impl IWMDeviceManagerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMDeviceManagerImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWMDeviceManagerVtbl {
        unsafe extern "system" fn GetRevision<Impl: IWMDeviceManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwrevision: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetRevision() {
                ::core::result::Result::Ok(ok__) => {
                    *pdwrevision = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDeviceCount<Impl: IWMDeviceManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwcount: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDeviceCount() {
                ::core::result::Result::Ok(ok__) => {
                    *pdwcount = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnumDevices<Impl: IWMDeviceManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenumdevice: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EnumDevices() {
                ::core::result::Result::Ok(ok__) => {
                    *ppenumdevice = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            GetRevision: GetRevision::<Impl, IMPL_OFFSET>,
            GetDeviceCount: GetDeviceCount::<Impl, IMPL_OFFSET>,
            EnumDevices: EnumDevices::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWMDeviceManager as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IWMDeviceManager2Impl: Sized + IWMDeviceManagerImpl {
    fn GetDeviceFromCanonicalName(&mut self, pwszcanonicalname: super::super::Foundation::PWSTR) -> ::windows::core::Result<IWMDMDevice>;
    fn EnumDevices2(&mut self) -> ::windows::core::Result<IWMDMEnumDevice>;
    fn Reinitialize(&mut self) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IWMDeviceManager2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMDeviceManager2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWMDeviceManager2Vtbl {
        unsafe extern "system" fn GetDeviceFromCanonicalName<Impl: IWMDeviceManager2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwszcanonicalname: super::super::Foundation::PWSTR, ppdevice: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDeviceFromCanonicalName(::core::mem::transmute_copy(&pwszcanonicalname)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppdevice = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnumDevices2<Impl: IWMDeviceManager2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenumdevice: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EnumDevices2() {
                ::core::result::Result::Ok(ok__) => {
                    *ppenumdevice = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Reinitialize<Impl: IWMDeviceManager2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Reinitialize().into()
        }
        Self {
            base: IWMDeviceManagerVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            GetDeviceFromCanonicalName: GetDeviceFromCanonicalName::<Impl, IMPL_OFFSET>,
            EnumDevices2: EnumDevices2::<Impl, IMPL_OFFSET>,
            Reinitialize: Reinitialize::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWMDeviceManager2 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IWMDeviceManager3Impl: Sized + IWMDeviceManagerImpl + IWMDeviceManager2Impl {
    fn SetDeviceEnumPreference(&mut self, dwenumpref: u32) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IWMDeviceManager3Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMDeviceManager3Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWMDeviceManager3Vtbl {
        unsafe extern "system" fn SetDeviceEnumPreference<Impl: IWMDeviceManager3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwenumpref: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetDeviceEnumPreference(::core::mem::transmute_copy(&dwenumpref)).into()
        }
        Self {
            base: IWMDeviceManager2Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            SetDeviceEnumPreference: SetDeviceEnumPreference::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWMDeviceManager3 as ::windows::core::Interface>::IID
    }
}
