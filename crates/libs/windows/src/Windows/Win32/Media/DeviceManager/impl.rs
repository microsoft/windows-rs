pub trait IComponentAuthenticate_Impl: Sized {
    fn SACAuth(&self, dwprotocolid: u32, dwpass: u32, pbdatain: *const u8, dwdatainlen: u32, ppbdataout: *mut *mut u8, pdwdataoutlen: *mut u32) -> ::windows::core::Result<()>;
    fn SACGetProtocols(&self, ppdwprotocols: *mut *mut u32, pdwprotocolcount: *mut u32) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IComponentAuthenticate {}
impl IComponentAuthenticate_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IComponentAuthenticate_Impl, const OFFSET: isize>() -> IComponentAuthenticate_Vtbl {
        unsafe extern "system" fn SACAuth<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IComponentAuthenticate_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwprotocolid: u32, dwpass: u32, pbdatain: *const u8, dwdatainlen: u32, ppbdataout: *mut *mut u8, pdwdataoutlen: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SACAuth(::core::mem::transmute_copy(&dwprotocolid), ::core::mem::transmute_copy(&dwpass), ::core::mem::transmute_copy(&pbdatain), ::core::mem::transmute_copy(&dwdatainlen), ::core::mem::transmute_copy(&ppbdataout), ::core::mem::transmute_copy(&pdwdataoutlen)).into()
        }
        unsafe extern "system" fn SACGetProtocols<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IComponentAuthenticate_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppdwprotocols: *mut *mut u32, pdwprotocolcount: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SACGetProtocols(::core::mem::transmute_copy(&ppdwprotocols), ::core::mem::transmute_copy(&pdwprotocolcount)).into()
        }
        Self {
            base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            SACAuth: SACAuth::<Identity, Impl, OFFSET>,
            SACGetProtocols: SACGetProtocols::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IComponentAuthenticate as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Media_Audio")]
pub trait IMDSPDevice_Impl: Sized {
    fn GetName(&self, pwszname: ::windows::core::PWSTR, nmaxchars: u32) -> ::windows::core::Result<()>;
    fn GetManufacturer(&self, pwszname: ::windows::core::PWSTR, nmaxchars: u32) -> ::windows::core::Result<()>;
    fn GetVersion(&self) -> ::windows::core::Result<u32>;
    fn GetType(&self) -> ::windows::core::Result<u32>;
    fn GetSerialNumber(&self, pserialnumber: *mut WMDMID, abmac: *mut u8) -> ::windows::core::Result<()>;
    fn GetPowerSource(&self, pdwpowersource: *mut u32, pdwpercentremaining: *mut u32) -> ::windows::core::Result<()>;
    fn GetStatus(&self) -> ::windows::core::Result<u32>;
    fn GetDeviceIcon(&self) -> ::windows::core::Result<u32>;
    fn EnumStorage(&self) -> ::windows::core::Result<IMDSPEnumStorage>;
    fn GetFormatSupport(&self, pformatex: *mut *mut super::Audio::WAVEFORMATEX, pnformatcount: *mut u32, pppwszmimetype: *mut *mut ::windows::core::PWSTR, pnmimetypecount: *mut u32) -> ::windows::core::Result<()>;
    fn SendOpaqueCommand(&self, pcommand: *mut OPAQUECOMMAND) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Media_Audio")]
impl ::windows::core::RuntimeName for IMDSPDevice {}
#[cfg(feature = "Win32_Media_Audio")]
impl IMDSPDevice_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMDSPDevice_Impl, const OFFSET: isize>() -> IMDSPDevice_Vtbl {
        unsafe extern "system" fn GetName<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMDSPDevice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwszname: ::windows::core::PWSTR, nmaxchars: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetName(::core::mem::transmute_copy(&pwszname), ::core::mem::transmute_copy(&nmaxchars)).into()
        }
        unsafe extern "system" fn GetManufacturer<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMDSPDevice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwszname: ::windows::core::PWSTR, nmaxchars: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetManufacturer(::core::mem::transmute_copy(&pwszname), ::core::mem::transmute_copy(&nmaxchars)).into()
        }
        unsafe extern "system" fn GetVersion<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMDSPDevice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwversion: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetVersion() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pdwversion, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetType<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMDSPDevice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwtype: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetType() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pdwtype, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSerialNumber<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMDSPDevice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pserialnumber: *mut WMDMID, abmac: *mut u8) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetSerialNumber(::core::mem::transmute_copy(&pserialnumber), ::core::mem::transmute_copy(&abmac)).into()
        }
        unsafe extern "system" fn GetPowerSource<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMDSPDevice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwpowersource: *mut u32, pdwpercentremaining: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetPowerSource(::core::mem::transmute_copy(&pdwpowersource), ::core::mem::transmute_copy(&pdwpercentremaining)).into()
        }
        unsafe extern "system" fn GetStatus<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMDSPDevice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwstatus: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetStatus() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pdwstatus, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDeviceIcon<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMDSPDevice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hicon: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetDeviceIcon() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(hicon, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnumStorage<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMDSPDevice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenumstorage: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.EnumStorage() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppenumstorage, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFormatSupport<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMDSPDevice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pformatex: *mut *mut super::Audio::WAVEFORMATEX, pnformatcount: *mut u32, pppwszmimetype: *mut *mut ::windows::core::PWSTR, pnmimetypecount: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetFormatSupport(::core::mem::transmute_copy(&pformatex), ::core::mem::transmute_copy(&pnformatcount), ::core::mem::transmute_copy(&pppwszmimetype), ::core::mem::transmute_copy(&pnmimetypecount)).into()
        }
        unsafe extern "system" fn SendOpaqueCommand<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMDSPDevice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcommand: *mut OPAQUECOMMAND) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SendOpaqueCommand(::core::mem::transmute_copy(&pcommand)).into()
        }
        Self {
            base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            GetName: GetName::<Identity, Impl, OFFSET>,
            GetManufacturer: GetManufacturer::<Identity, Impl, OFFSET>,
            GetVersion: GetVersion::<Identity, Impl, OFFSET>,
            GetType: GetType::<Identity, Impl, OFFSET>,
            GetSerialNumber: GetSerialNumber::<Identity, Impl, OFFSET>,
            GetPowerSource: GetPowerSource::<Identity, Impl, OFFSET>,
            GetStatus: GetStatus::<Identity, Impl, OFFSET>,
            GetDeviceIcon: GetDeviceIcon::<Identity, Impl, OFFSET>,
            EnumStorage: EnumStorage::<Identity, Impl, OFFSET>,
            GetFormatSupport: GetFormatSupport::<Identity, Impl, OFFSET>,
            SendOpaqueCommand: SendOpaqueCommand::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMDSPDevice as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_Media_Audio", feature = "Win32_Media_MediaFoundation", feature = "Win32_System_Ole"))]
pub trait IMDSPDevice2_Impl: Sized + IMDSPDevice_Impl {
    fn GetStorage(&self, pszstoragename: &::windows::core::PCWSTR) -> ::windows::core::Result<IMDSPStorage>;
    fn GetFormatSupport2(&self, dwflags: u32, ppaudioformatex: *mut *mut super::Audio::WAVEFORMATEX, pnaudioformatcount: *mut u32, ppvideoformatex: *mut *mut super::MediaFoundation::VIDEOINFOHEADER, pnvideoformatcount: *mut u32, ppfiletype: *mut *mut WMFILECAPABILITIES, pnfiletypecount: *mut u32) -> ::windows::core::Result<()>;
    fn GetSpecifyPropertyPages(&self, ppspecifyproppages: *mut ::core::option::Option<super::super::System::Ole::ISpecifyPropertyPages>, pppunknowns: *mut *mut ::core::option::Option<::windows::core::IUnknown>, pcunks: *mut u32) -> ::windows::core::Result<()>;
    fn GetCanonicalName(&self, pwszpnpname: ::windows::core::PWSTR, nmaxchars: u32) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_Media_Audio", feature = "Win32_Media_MediaFoundation", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for IMDSPDevice2 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_Media_Audio", feature = "Win32_Media_MediaFoundation", feature = "Win32_System_Ole"))]
impl IMDSPDevice2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMDSPDevice2_Impl, const OFFSET: isize>() -> IMDSPDevice2_Vtbl {
        unsafe extern "system" fn GetStorage<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMDSPDevice2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszstoragename: ::windows::core::PCWSTR, ppstorage: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetStorage(::core::mem::transmute(&pszstoragename)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppstorage, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFormatSupport2<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMDSPDevice2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwflags: u32, ppaudioformatex: *mut *mut super::Audio::WAVEFORMATEX, pnaudioformatcount: *mut u32, ppvideoformatex: *mut *mut super::MediaFoundation::VIDEOINFOHEADER, pnvideoformatcount: *mut u32, ppfiletype: *mut *mut WMFILECAPABILITIES, pnfiletypecount: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetFormatSupport2(::core::mem::transmute_copy(&dwflags), ::core::mem::transmute_copy(&ppaudioformatex), ::core::mem::transmute_copy(&pnaudioformatcount), ::core::mem::transmute_copy(&ppvideoformatex), ::core::mem::transmute_copy(&pnvideoformatcount), ::core::mem::transmute_copy(&ppfiletype), ::core::mem::transmute_copy(&pnfiletypecount)).into()
        }
        unsafe extern "system" fn GetSpecifyPropertyPages<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMDSPDevice2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppspecifyproppages: *mut *mut ::core::ffi::c_void, pppunknowns: *mut *mut *mut ::core::ffi::c_void, pcunks: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetSpecifyPropertyPages(::core::mem::transmute_copy(&ppspecifyproppages), ::core::mem::transmute_copy(&pppunknowns), ::core::mem::transmute_copy(&pcunks)).into()
        }
        unsafe extern "system" fn GetCanonicalName<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMDSPDevice2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwszpnpname: ::windows::core::PWSTR, nmaxchars: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetCanonicalName(::core::mem::transmute_copy(&pwszpnpname), ::core::mem::transmute_copy(&nmaxchars)).into()
        }
        Self {
            base__: IMDSPDevice_Vtbl::new::<Identity, Impl, OFFSET>(),
            GetStorage: GetStorage::<Identity, Impl, OFFSET>,
            GetFormatSupport2: GetFormatSupport2::<Identity, Impl, OFFSET>,
            GetSpecifyPropertyPages: GetSpecifyPropertyPages::<Identity, Impl, OFFSET>,
            GetCanonicalName: GetCanonicalName::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMDSPDevice2 as ::windows::core::Interface>::IID || iid == &<IMDSPDevice as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_Media_Audio", feature = "Win32_Media_MediaFoundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Ole"))]
pub trait IMDSPDevice3_Impl: Sized + IMDSPDevice_Impl + IMDSPDevice2_Impl {
    fn GetProperty(&self, pwszpropname: &::windows::core::PCWSTR) -> ::windows::core::Result<super::super::System::Com::StructuredStorage::PROPVARIANT>;
    fn SetProperty(&self, pwszpropname: &::windows::core::PCWSTR, pvalue: *const super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows::core::Result<()>;
    fn GetFormatCapability(&self, format: WMDM_FORMATCODE) -> ::windows::core::Result<WMDM_FORMAT_CAPABILITY>;
    fn DeviceIoControl(&self, dwiocontrolcode: u32, lpinbuffer: *const u8, ninbuffersize: u32, lpoutbuffer: *mut u8, pnoutbuffersize: *mut u32) -> ::windows::core::Result<()>;
    fn FindStorage(&self, findscope: WMDM_FIND_SCOPE, pwszuniqueid: &::windows::core::PCWSTR) -> ::windows::core::Result<IMDSPStorage>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_Media_Audio", feature = "Win32_Media_MediaFoundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for IMDSPDevice3 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_Media_Audio", feature = "Win32_Media_MediaFoundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Ole"))]
impl IMDSPDevice3_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMDSPDevice3_Impl, const OFFSET: isize>() -> IMDSPDevice3_Vtbl {
        unsafe extern "system" fn GetProperty<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMDSPDevice3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwszpropname: ::windows::core::PCWSTR, pvalue: *mut super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetProperty(::core::mem::transmute(&pwszpropname)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pvalue, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetProperty<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMDSPDevice3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwszpropname: ::windows::core::PCWSTR, pvalue: *const super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetProperty(::core::mem::transmute(&pwszpropname), ::core::mem::transmute_copy(&pvalue)).into()
        }
        unsafe extern "system" fn GetFormatCapability<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMDSPDevice3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, format: WMDM_FORMATCODE, pformatsupport: *mut WMDM_FORMAT_CAPABILITY) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetFormatCapability(::core::mem::transmute_copy(&format)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pformatsupport, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DeviceIoControl<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMDSPDevice3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwiocontrolcode: u32, lpinbuffer: *const u8, ninbuffersize: u32, lpoutbuffer: *mut u8, pnoutbuffersize: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.DeviceIoControl(::core::mem::transmute_copy(&dwiocontrolcode), ::core::mem::transmute_copy(&lpinbuffer), ::core::mem::transmute_copy(&ninbuffersize), ::core::mem::transmute_copy(&lpoutbuffer), ::core::mem::transmute_copy(&pnoutbuffersize)).into()
        }
        unsafe extern "system" fn FindStorage<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMDSPDevice3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, findscope: WMDM_FIND_SCOPE, pwszuniqueid: ::windows::core::PCWSTR, ppstorage: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.FindStorage(::core::mem::transmute_copy(&findscope), ::core::mem::transmute(&pwszuniqueid)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppstorage, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: IMDSPDevice2_Vtbl::new::<Identity, Impl, OFFSET>(),
            GetProperty: GetProperty::<Identity, Impl, OFFSET>,
            SetProperty: SetProperty::<Identity, Impl, OFFSET>,
            GetFormatCapability: GetFormatCapability::<Identity, Impl, OFFSET>,
            DeviceIoControl: DeviceIoControl::<Identity, Impl, OFFSET>,
            FindStorage: FindStorage::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMDSPDevice3 as ::windows::core::Interface>::IID || iid == &<IMDSPDevice as ::windows::core::Interface>::IID || iid == &<IMDSPDevice2 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Media_Audio")]
pub trait IMDSPDeviceControl_Impl: Sized {
    fn GetDCStatus(&self) -> ::windows::core::Result<u32>;
    fn GetCapabilities(&self) -> ::windows::core::Result<u32>;
    fn Play(&self) -> ::windows::core::Result<()>;
    fn Record(&self, pformat: *const super::Audio::WAVEFORMATEX) -> ::windows::core::Result<()>;
    fn Pause(&self) -> ::windows::core::Result<()>;
    fn Resume(&self) -> ::windows::core::Result<()>;
    fn Stop(&self) -> ::windows::core::Result<()>;
    fn Seek(&self, fumode: u32, noffset: i32) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Media_Audio")]
impl ::windows::core::RuntimeName for IMDSPDeviceControl {}
#[cfg(feature = "Win32_Media_Audio")]
impl IMDSPDeviceControl_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMDSPDeviceControl_Impl, const OFFSET: isize>() -> IMDSPDeviceControl_Vtbl {
        unsafe extern "system" fn GetDCStatus<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMDSPDeviceControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwstatus: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetDCStatus() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pdwstatus, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCapabilities<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMDSPDeviceControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwcapabilitiesmask: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetCapabilities() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pdwcapabilitiesmask, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Play<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMDSPDeviceControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Play().into()
        }
        unsafe extern "system" fn Record<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMDSPDeviceControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pformat: *const super::Audio::WAVEFORMATEX) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Record(::core::mem::transmute_copy(&pformat)).into()
        }
        unsafe extern "system" fn Pause<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMDSPDeviceControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Pause().into()
        }
        unsafe extern "system" fn Resume<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMDSPDeviceControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Resume().into()
        }
        unsafe extern "system" fn Stop<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMDSPDeviceControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Stop().into()
        }
        unsafe extern "system" fn Seek<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMDSPDeviceControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fumode: u32, noffset: i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Seek(::core::mem::transmute_copy(&fumode), ::core::mem::transmute_copy(&noffset)).into()
        }
        Self {
            base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            GetDCStatus: GetDCStatus::<Identity, Impl, OFFSET>,
            GetCapabilities: GetCapabilities::<Identity, Impl, OFFSET>,
            Play: Play::<Identity, Impl, OFFSET>,
            Record: Record::<Identity, Impl, OFFSET>,
            Pause: Pause::<Identity, Impl, OFFSET>,
            Resume: Resume::<Identity, Impl, OFFSET>,
            Stop: Stop::<Identity, Impl, OFFSET>,
            Seek: Seek::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMDSPDeviceControl as ::windows::core::Interface>::IID
    }
}
pub trait IMDSPDirectTransfer_Impl: Sized {
    fn TransferToDevice(&self, pwszsourcefilepath: &::windows::core::PCWSTR, psourceoperation: &::core::option::Option<IWMDMOperation>, fuflags: u32, pwszdestinationname: &::windows::core::PCWSTR, psourcemetadata: &::core::option::Option<IWMDMMetaData>, ptransferprogress: &::core::option::Option<IWMDMProgress>) -> ::windows::core::Result<IMDSPStorage>;
}
impl ::windows::core::RuntimeName for IMDSPDirectTransfer {}
impl IMDSPDirectTransfer_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMDSPDirectTransfer_Impl, const OFFSET: isize>() -> IMDSPDirectTransfer_Vtbl {
        unsafe extern "system" fn TransferToDevice<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMDSPDirectTransfer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwszsourcefilepath: ::windows::core::PCWSTR, psourceoperation: *mut ::core::ffi::c_void, fuflags: u32, pwszdestinationname: ::windows::core::PCWSTR, psourcemetadata: *mut ::core::ffi::c_void, ptransferprogress: *mut ::core::ffi::c_void, ppnewobject: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.TransferToDevice(::core::mem::transmute(&pwszsourcefilepath), ::core::mem::transmute(&psourceoperation), ::core::mem::transmute_copy(&fuflags), ::core::mem::transmute(&pwszdestinationname), ::core::mem::transmute(&psourcemetadata), ::core::mem::transmute(&ptransferprogress)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppnewobject, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(), TransferToDevice: TransferToDevice::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMDSPDirectTransfer as ::windows::core::Interface>::IID
    }
}
pub trait IMDSPEnumDevice_Impl: Sized {
    fn Next(&self, celt: u32, ppdevice: *mut ::core::option::Option<IMDSPDevice>, pceltfetched: *mut u32) -> ::windows::core::Result<()>;
    fn Skip(&self, celt: u32) -> ::windows::core::Result<u32>;
    fn Reset(&self) -> ::windows::core::Result<()>;
    fn Clone(&self) -> ::windows::core::Result<IMDSPEnumDevice>;
}
impl ::windows::core::RuntimeName for IMDSPEnumDevice {}
impl IMDSPEnumDevice_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMDSPEnumDevice_Impl, const OFFSET: isize>() -> IMDSPEnumDevice_Vtbl {
        unsafe extern "system" fn Next<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMDSPEnumDevice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32, ppdevice: *mut *mut ::core::ffi::c_void, pceltfetched: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Next(::core::mem::transmute_copy(&celt), ::core::mem::transmute_copy(&ppdevice), ::core::mem::transmute_copy(&pceltfetched)).into()
        }
        unsafe extern "system" fn Skip<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMDSPEnumDevice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32, pceltfetched: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Skip(::core::mem::transmute_copy(&celt)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pceltfetched, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Reset<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMDSPEnumDevice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Reset().into()
        }
        unsafe extern "system" fn Clone<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMDSPEnumDevice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenumdevice: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Clone() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppenumdevice, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            Next: Next::<Identity, Impl, OFFSET>,
            Skip: Skip::<Identity, Impl, OFFSET>,
            Reset: Reset::<Identity, Impl, OFFSET>,
            Clone: Clone::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMDSPEnumDevice as ::windows::core::Interface>::IID
    }
}
pub trait IMDSPEnumStorage_Impl: Sized {
    fn Next(&self, celt: u32, ppstorage: *mut ::core::option::Option<IMDSPStorage>, pceltfetched: *mut u32) -> ::windows::core::Result<()>;
    fn Skip(&self, celt: u32) -> ::windows::core::Result<u32>;
    fn Reset(&self) -> ::windows::core::Result<()>;
    fn Clone(&self) -> ::windows::core::Result<IMDSPEnumStorage>;
}
impl ::windows::core::RuntimeName for IMDSPEnumStorage {}
impl IMDSPEnumStorage_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMDSPEnumStorage_Impl, const OFFSET: isize>() -> IMDSPEnumStorage_Vtbl {
        unsafe extern "system" fn Next<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMDSPEnumStorage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32, ppstorage: *mut *mut ::core::ffi::c_void, pceltfetched: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Next(::core::mem::transmute_copy(&celt), ::core::mem::transmute_copy(&ppstorage), ::core::mem::transmute_copy(&pceltfetched)).into()
        }
        unsafe extern "system" fn Skip<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMDSPEnumStorage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32, pceltfetched: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Skip(::core::mem::transmute_copy(&celt)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pceltfetched, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Reset<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMDSPEnumStorage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Reset().into()
        }
        unsafe extern "system" fn Clone<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMDSPEnumStorage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenumstorage: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Clone() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppenumstorage, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            Next: Next::<Identity, Impl, OFFSET>,
            Skip: Skip::<Identity, Impl, OFFSET>,
            Reset: Reset::<Identity, Impl, OFFSET>,
            Clone: Clone::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMDSPEnumStorage as ::windows::core::Interface>::IID
    }
}
pub trait IMDSPObject_Impl: Sized {
    fn Open(&self, fumode: u32) -> ::windows::core::Result<()>;
    fn Read(&self, pdata: *mut u8, pdwsize: *mut u32, abmac: *mut u8) -> ::windows::core::Result<()>;
    fn Write(&self, pdata: *const u8, pdwsize: *mut u32, abmac: *mut u8) -> ::windows::core::Result<()>;
    fn Delete(&self, fumode: u32, pprogress: &::core::option::Option<IWMDMProgress>) -> ::windows::core::Result<()>;
    fn Seek(&self, fuflags: u32, dwoffset: u32) -> ::windows::core::Result<()>;
    fn Rename(&self, pwsznewname: &::windows::core::PCWSTR, pprogress: &::core::option::Option<IWMDMProgress>) -> ::windows::core::Result<()>;
    fn Move(&self, fumode: u32, pprogress: &::core::option::Option<IWMDMProgress>, ptarget: &::core::option::Option<IMDSPStorage>) -> ::windows::core::Result<()>;
    fn Close(&self) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IMDSPObject {}
impl IMDSPObject_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMDSPObject_Impl, const OFFSET: isize>() -> IMDSPObject_Vtbl {
        unsafe extern "system" fn Open<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMDSPObject_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fumode: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Open(::core::mem::transmute_copy(&fumode)).into()
        }
        unsafe extern "system" fn Read<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMDSPObject_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdata: *mut u8, pdwsize: *mut u32, abmac: *mut u8) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Read(::core::mem::transmute_copy(&pdata), ::core::mem::transmute_copy(&pdwsize), ::core::mem::transmute_copy(&abmac)).into()
        }
        unsafe extern "system" fn Write<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMDSPObject_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdata: *const u8, pdwsize: *mut u32, abmac: *mut u8) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Write(::core::mem::transmute_copy(&pdata), ::core::mem::transmute_copy(&pdwsize), ::core::mem::transmute_copy(&abmac)).into()
        }
        unsafe extern "system" fn Delete<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMDSPObject_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fumode: u32, pprogress: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Delete(::core::mem::transmute_copy(&fumode), ::core::mem::transmute(&pprogress)).into()
        }
        unsafe extern "system" fn Seek<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMDSPObject_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fuflags: u32, dwoffset: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Seek(::core::mem::transmute_copy(&fuflags), ::core::mem::transmute_copy(&dwoffset)).into()
        }
        unsafe extern "system" fn Rename<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMDSPObject_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwsznewname: ::windows::core::PCWSTR, pprogress: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Rename(::core::mem::transmute(&pwsznewname), ::core::mem::transmute(&pprogress)).into()
        }
        unsafe extern "system" fn Move<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMDSPObject_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fumode: u32, pprogress: *mut ::core::ffi::c_void, ptarget: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Move(::core::mem::transmute_copy(&fumode), ::core::mem::transmute(&pprogress), ::core::mem::transmute(&ptarget)).into()
        }
        unsafe extern "system" fn Close<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMDSPObject_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Close().into()
        }
        Self {
            base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            Open: Open::<Identity, Impl, OFFSET>,
            Read: Read::<Identity, Impl, OFFSET>,
            Write: Write::<Identity, Impl, OFFSET>,
            Delete: Delete::<Identity, Impl, OFFSET>,
            Seek: Seek::<Identity, Impl, OFFSET>,
            Rename: Rename::<Identity, Impl, OFFSET>,
            Move: Move::<Identity, Impl, OFFSET>,
            Close: Close::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMDSPObject as ::windows::core::Interface>::IID
    }
}
pub trait IMDSPObject2_Impl: Sized + IMDSPObject_Impl {
    fn ReadOnClearChannel(&self, pdata: *mut u8, pdwsize: *mut u32) -> ::windows::core::Result<()>;
    fn WriteOnClearChannel(&self, pdata: *const u8, pdwsize: *mut u32) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IMDSPObject2 {}
impl IMDSPObject2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMDSPObject2_Impl, const OFFSET: isize>() -> IMDSPObject2_Vtbl {
        unsafe extern "system" fn ReadOnClearChannel<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMDSPObject2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdata: *mut u8, pdwsize: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.ReadOnClearChannel(::core::mem::transmute_copy(&pdata), ::core::mem::transmute_copy(&pdwsize)).into()
        }
        unsafe extern "system" fn WriteOnClearChannel<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMDSPObject2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdata: *const u8, pdwsize: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.WriteOnClearChannel(::core::mem::transmute_copy(&pdata), ::core::mem::transmute_copy(&pdwsize)).into()
        }
        Self {
            base__: IMDSPObject_Vtbl::new::<Identity, Impl, OFFSET>(),
            ReadOnClearChannel: ReadOnClearChannel::<Identity, Impl, OFFSET>,
            WriteOnClearChannel: WriteOnClearChannel::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMDSPObject2 as ::windows::core::Interface>::IID || iid == &<IMDSPObject as ::windows::core::Interface>::IID
    }
}
pub trait IMDSPObjectInfo_Impl: Sized {
    fn GetPlayLength(&self) -> ::windows::core::Result<u32>;
    fn SetPlayLength(&self, dwlength: u32) -> ::windows::core::Result<()>;
    fn GetPlayOffset(&self) -> ::windows::core::Result<u32>;
    fn SetPlayOffset(&self, dwoffset: u32) -> ::windows::core::Result<()>;
    fn GetTotalLength(&self) -> ::windows::core::Result<u32>;
    fn GetLastPlayPosition(&self) -> ::windows::core::Result<u32>;
    fn GetLongestPlayPosition(&self) -> ::windows::core::Result<u32>;
}
impl ::windows::core::RuntimeName for IMDSPObjectInfo {}
impl IMDSPObjectInfo_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMDSPObjectInfo_Impl, const OFFSET: isize>() -> IMDSPObjectInfo_Vtbl {
        unsafe extern "system" fn GetPlayLength<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMDSPObjectInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwlength: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetPlayLength() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pdwlength, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPlayLength<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMDSPObjectInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwlength: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetPlayLength(::core::mem::transmute_copy(&dwlength)).into()
        }
        unsafe extern "system" fn GetPlayOffset<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMDSPObjectInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwoffset: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetPlayOffset() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pdwoffset, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPlayOffset<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMDSPObjectInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwoffset: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetPlayOffset(::core::mem::transmute_copy(&dwoffset)).into()
        }
        unsafe extern "system" fn GetTotalLength<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMDSPObjectInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwlength: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetTotalLength() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pdwlength, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetLastPlayPosition<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMDSPObjectInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwlastpos: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetLastPlayPosition() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pdwlastpos, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetLongestPlayPosition<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMDSPObjectInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwlongestpos: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetLongestPlayPosition() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pdwlongestpos, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            GetPlayLength: GetPlayLength::<Identity, Impl, OFFSET>,
            SetPlayLength: SetPlayLength::<Identity, Impl, OFFSET>,
            GetPlayOffset: GetPlayOffset::<Identity, Impl, OFFSET>,
            SetPlayOffset: SetPlayOffset::<Identity, Impl, OFFSET>,
            GetTotalLength: GetTotalLength::<Identity, Impl, OFFSET>,
            GetLastPlayPosition: GetLastPlayPosition::<Identity, Impl, OFFSET>,
            GetLongestPlayPosition: GetLongestPlayPosition::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMDSPObjectInfo as ::windows::core::Interface>::IID
    }
}
pub trait IMDSPRevoked_Impl: Sized {
    fn GetRevocationURL(&self, ppwszrevocationurl: *mut ::windows::core::PWSTR, pdwbufferlen: *mut u32) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IMDSPRevoked {}
impl IMDSPRevoked_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMDSPRevoked_Impl, const OFFSET: isize>() -> IMDSPRevoked_Vtbl {
        unsafe extern "system" fn GetRevocationURL<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMDSPRevoked_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppwszrevocationurl: *mut ::windows::core::PWSTR, pdwbufferlen: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetRevocationURL(::core::mem::transmute_copy(&ppwszrevocationurl), ::core::mem::transmute_copy(&pdwbufferlen)).into()
        }
        Self { base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(), GetRevocationURL: GetRevocationURL::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMDSPRevoked as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Media_Audio")]
pub trait IMDSPStorage_Impl: Sized {
    fn SetAttributes(&self, dwattributes: u32, pformat: *const super::Audio::WAVEFORMATEX) -> ::windows::core::Result<()>;
    fn GetStorageGlobals(&self) -> ::windows::core::Result<IMDSPStorageGlobals>;
    fn GetAttributes(&self, pdwattributes: *mut u32, pformat: *mut super::Audio::WAVEFORMATEX) -> ::windows::core::Result<()>;
    fn GetName(&self, pwszname: ::windows::core::PWSTR, nmaxchars: u32) -> ::windows::core::Result<()>;
    fn GetDate(&self) -> ::windows::core::Result<WMDMDATETIME>;
    fn GetSize(&self, pdwsizelow: *mut u32, pdwsizehigh: *mut u32) -> ::windows::core::Result<()>;
    fn GetRights(&self, pprights: *mut *mut WMDMRIGHTS, pnrightscount: *mut u32, abmac: *mut u8) -> ::windows::core::Result<()>;
    fn CreateStorage(&self, dwattributes: u32, pformat: *const super::Audio::WAVEFORMATEX, pwszname: &::windows::core::PCWSTR) -> ::windows::core::Result<IMDSPStorage>;
    fn EnumStorage(&self) -> ::windows::core::Result<IMDSPEnumStorage>;
    fn SendOpaqueCommand(&self, pcommand: *mut OPAQUECOMMAND) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Media_Audio")]
impl ::windows::core::RuntimeName for IMDSPStorage {}
#[cfg(feature = "Win32_Media_Audio")]
impl IMDSPStorage_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMDSPStorage_Impl, const OFFSET: isize>() -> IMDSPStorage_Vtbl {
        unsafe extern "system" fn SetAttributes<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMDSPStorage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwattributes: u32, pformat: *const super::Audio::WAVEFORMATEX) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetAttributes(::core::mem::transmute_copy(&dwattributes), ::core::mem::transmute_copy(&pformat)).into()
        }
        unsafe extern "system" fn GetStorageGlobals<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMDSPStorage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppstorageglobals: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetStorageGlobals() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppstorageglobals, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetAttributes<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMDSPStorage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwattributes: *mut u32, pformat: *mut super::Audio::WAVEFORMATEX) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetAttributes(::core::mem::transmute_copy(&pdwattributes), ::core::mem::transmute_copy(&pformat)).into()
        }
        unsafe extern "system" fn GetName<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMDSPStorage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwszname: ::windows::core::PWSTR, nmaxchars: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetName(::core::mem::transmute_copy(&pwszname), ::core::mem::transmute_copy(&nmaxchars)).into()
        }
        unsafe extern "system" fn GetDate<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMDSPStorage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdatetimeutc: *mut WMDMDATETIME) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetDate() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pdatetimeutc, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSize<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMDSPStorage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwsizelow: *mut u32, pdwsizehigh: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetSize(::core::mem::transmute_copy(&pdwsizelow), ::core::mem::transmute_copy(&pdwsizehigh)).into()
        }
        unsafe extern "system" fn GetRights<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMDSPStorage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pprights: *mut *mut WMDMRIGHTS, pnrightscount: *mut u32, abmac: *mut u8) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetRights(::core::mem::transmute_copy(&pprights), ::core::mem::transmute_copy(&pnrightscount), ::core::mem::transmute_copy(&abmac)).into()
        }
        unsafe extern "system" fn CreateStorage<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMDSPStorage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwattributes: u32, pformat: *const super::Audio::WAVEFORMATEX, pwszname: ::windows::core::PCWSTR, ppnewstorage: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CreateStorage(::core::mem::transmute_copy(&dwattributes), ::core::mem::transmute_copy(&pformat), ::core::mem::transmute(&pwszname)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppnewstorage, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnumStorage<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMDSPStorage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenumstorage: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.EnumStorage() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppenumstorage, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SendOpaqueCommand<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMDSPStorage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcommand: *mut OPAQUECOMMAND) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SendOpaqueCommand(::core::mem::transmute_copy(&pcommand)).into()
        }
        Self {
            base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            SetAttributes: SetAttributes::<Identity, Impl, OFFSET>,
            GetStorageGlobals: GetStorageGlobals::<Identity, Impl, OFFSET>,
            GetAttributes: GetAttributes::<Identity, Impl, OFFSET>,
            GetName: GetName::<Identity, Impl, OFFSET>,
            GetDate: GetDate::<Identity, Impl, OFFSET>,
            GetSize: GetSize::<Identity, Impl, OFFSET>,
            GetRights: GetRights::<Identity, Impl, OFFSET>,
            CreateStorage: CreateStorage::<Identity, Impl, OFFSET>,
            EnumStorage: EnumStorage::<Identity, Impl, OFFSET>,
            SendOpaqueCommand: SendOpaqueCommand::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMDSPStorage as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_Media_Audio", feature = "Win32_Media_MediaFoundation"))]
pub trait IMDSPStorage2_Impl: Sized + IMDSPStorage_Impl {
    fn GetStorage(&self, pszstoragename: &::windows::core::PCWSTR) -> ::windows::core::Result<IMDSPStorage>;
    fn CreateStorage2(&self, dwattributes: u32, dwattributesex: u32, paudioformat: *const super::Audio::WAVEFORMATEX, pvideoformat: *const super::MediaFoundation::VIDEOINFOHEADER, pwszname: &::windows::core::PCWSTR, qwfilesize: u64) -> ::windows::core::Result<IMDSPStorage>;
    fn SetAttributes2(&self, dwattributes: u32, dwattributesex: u32, paudioformat: *const super::Audio::WAVEFORMATEX, pvideoformat: *const super::MediaFoundation::VIDEOINFOHEADER) -> ::windows::core::Result<()>;
    fn GetAttributes2(&self, pdwattributes: *mut u32, pdwattributesex: *mut u32, paudioformat: *mut super::Audio::WAVEFORMATEX, pvideoformat: *mut super::MediaFoundation::VIDEOINFOHEADER) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_Media_Audio", feature = "Win32_Media_MediaFoundation"))]
impl ::windows::core::RuntimeName for IMDSPStorage2 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_Media_Audio", feature = "Win32_Media_MediaFoundation"))]
impl IMDSPStorage2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMDSPStorage2_Impl, const OFFSET: isize>() -> IMDSPStorage2_Vtbl {
        unsafe extern "system" fn GetStorage<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMDSPStorage2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszstoragename: ::windows::core::PCWSTR, ppstorage: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetStorage(::core::mem::transmute(&pszstoragename)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppstorage, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateStorage2<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMDSPStorage2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwattributes: u32, dwattributesex: u32, paudioformat: *const super::Audio::WAVEFORMATEX, pvideoformat: *const super::MediaFoundation::VIDEOINFOHEADER, pwszname: ::windows::core::PCWSTR, qwfilesize: u64, ppnewstorage: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CreateStorage2(::core::mem::transmute_copy(&dwattributes), ::core::mem::transmute_copy(&dwattributesex), ::core::mem::transmute_copy(&paudioformat), ::core::mem::transmute_copy(&pvideoformat), ::core::mem::transmute(&pwszname), ::core::mem::transmute_copy(&qwfilesize)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppnewstorage, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAttributes2<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMDSPStorage2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwattributes: u32, dwattributesex: u32, paudioformat: *const super::Audio::WAVEFORMATEX, pvideoformat: *const super::MediaFoundation::VIDEOINFOHEADER) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetAttributes2(::core::mem::transmute_copy(&dwattributes), ::core::mem::transmute_copy(&dwattributesex), ::core::mem::transmute_copy(&paudioformat), ::core::mem::transmute_copy(&pvideoformat)).into()
        }
        unsafe extern "system" fn GetAttributes2<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMDSPStorage2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwattributes: *mut u32, pdwattributesex: *mut u32, paudioformat: *mut super::Audio::WAVEFORMATEX, pvideoformat: *mut super::MediaFoundation::VIDEOINFOHEADER) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetAttributes2(::core::mem::transmute_copy(&pdwattributes), ::core::mem::transmute_copy(&pdwattributesex), ::core::mem::transmute_copy(&paudioformat), ::core::mem::transmute_copy(&pvideoformat)).into()
        }
        Self {
            base__: IMDSPStorage_Vtbl::new::<Identity, Impl, OFFSET>(),
            GetStorage: GetStorage::<Identity, Impl, OFFSET>,
            CreateStorage2: CreateStorage2::<Identity, Impl, OFFSET>,
            SetAttributes2: SetAttributes2::<Identity, Impl, OFFSET>,
            GetAttributes2: GetAttributes2::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMDSPStorage2 as ::windows::core::Interface>::IID || iid == &<IMDSPStorage as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_Media_Audio", feature = "Win32_Media_MediaFoundation"))]
pub trait IMDSPStorage3_Impl: Sized + IMDSPStorage_Impl + IMDSPStorage2_Impl {
    fn GetMetadata(&self, pmetadata: &::core::option::Option<IWMDMMetaData>) -> ::windows::core::Result<()>;
    fn SetMetadata(&self, pmetadata: &::core::option::Option<IWMDMMetaData>) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_Media_Audio", feature = "Win32_Media_MediaFoundation"))]
impl ::windows::core::RuntimeName for IMDSPStorage3 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_Media_Audio", feature = "Win32_Media_MediaFoundation"))]
impl IMDSPStorage3_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMDSPStorage3_Impl, const OFFSET: isize>() -> IMDSPStorage3_Vtbl {
        unsafe extern "system" fn GetMetadata<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMDSPStorage3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pmetadata: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetMetadata(::core::mem::transmute(&pmetadata)).into()
        }
        unsafe extern "system" fn SetMetadata<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMDSPStorage3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pmetadata: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetMetadata(::core::mem::transmute(&pmetadata)).into()
        }
        Self {
            base__: IMDSPStorage2_Vtbl::new::<Identity, Impl, OFFSET>(),
            GetMetadata: GetMetadata::<Identity, Impl, OFFSET>,
            SetMetadata: SetMetadata::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMDSPStorage3 as ::windows::core::Interface>::IID || iid == &<IMDSPStorage as ::windows::core::Interface>::IID || iid == &<IMDSPStorage2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_Media_Audio", feature = "Win32_Media_MediaFoundation"))]
pub trait IMDSPStorage4_Impl: Sized + IMDSPStorage_Impl + IMDSPStorage2_Impl + IMDSPStorage3_Impl {
    fn SetReferences(&self, dwrefs: u32, ppispstorage: *const ::core::option::Option<IMDSPStorage>) -> ::windows::core::Result<()>;
    fn GetReferences(&self, pdwrefs: *mut u32, pppispstorage: *mut *mut ::core::option::Option<IMDSPStorage>) -> ::windows::core::Result<()>;
    fn CreateStorageWithMetadata(&self, dwattributes: u32, pwszname: &::windows::core::PCWSTR, pmetadata: &::core::option::Option<IWMDMMetaData>, qwfilesize: u64) -> ::windows::core::Result<IMDSPStorage>;
    fn GetSpecifiedMetadata(&self, cproperties: u32, ppwszpropnames: *const ::windows::core::PWSTR, pmetadata: &::core::option::Option<IWMDMMetaData>) -> ::windows::core::Result<()>;
    fn FindStorage(&self, findscope: WMDM_FIND_SCOPE, pwszuniqueid: &::windows::core::PCWSTR) -> ::windows::core::Result<IMDSPStorage>;
    fn GetParent(&self) -> ::windows::core::Result<IMDSPStorage>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_Media_Audio", feature = "Win32_Media_MediaFoundation"))]
impl ::windows::core::RuntimeName for IMDSPStorage4 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_Media_Audio", feature = "Win32_Media_MediaFoundation"))]
impl IMDSPStorage4_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMDSPStorage4_Impl, const OFFSET: isize>() -> IMDSPStorage4_Vtbl {
        unsafe extern "system" fn SetReferences<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMDSPStorage4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwrefs: u32, ppispstorage: *const *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetReferences(::core::mem::transmute_copy(&dwrefs), ::core::mem::transmute_copy(&ppispstorage)).into()
        }
        unsafe extern "system" fn GetReferences<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMDSPStorage4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwrefs: *mut u32, pppispstorage: *mut *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetReferences(::core::mem::transmute_copy(&pdwrefs), ::core::mem::transmute_copy(&pppispstorage)).into()
        }
        unsafe extern "system" fn CreateStorageWithMetadata<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMDSPStorage4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwattributes: u32, pwszname: ::windows::core::PCWSTR, pmetadata: *mut ::core::ffi::c_void, qwfilesize: u64, ppnewstorage: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CreateStorageWithMetadata(::core::mem::transmute_copy(&dwattributes), ::core::mem::transmute(&pwszname), ::core::mem::transmute(&pmetadata), ::core::mem::transmute_copy(&qwfilesize)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppnewstorage, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSpecifiedMetadata<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMDSPStorage4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cproperties: u32, ppwszpropnames: *const ::windows::core::PWSTR, pmetadata: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetSpecifiedMetadata(::core::mem::transmute_copy(&cproperties), ::core::mem::transmute_copy(&ppwszpropnames), ::core::mem::transmute(&pmetadata)).into()
        }
        unsafe extern "system" fn FindStorage<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMDSPStorage4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, findscope: WMDM_FIND_SCOPE, pwszuniqueid: ::windows::core::PCWSTR, ppstorage: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.FindStorage(::core::mem::transmute_copy(&findscope), ::core::mem::transmute(&pwszuniqueid)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppstorage, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetParent<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMDSPStorage4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppstorage: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetParent() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppstorage, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: IMDSPStorage3_Vtbl::new::<Identity, Impl, OFFSET>(),
            SetReferences: SetReferences::<Identity, Impl, OFFSET>,
            GetReferences: GetReferences::<Identity, Impl, OFFSET>,
            CreateStorageWithMetadata: CreateStorageWithMetadata::<Identity, Impl, OFFSET>,
            GetSpecifiedMetadata: GetSpecifiedMetadata::<Identity, Impl, OFFSET>,
            FindStorage: FindStorage::<Identity, Impl, OFFSET>,
            GetParent: GetParent::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMDSPStorage4 as ::windows::core::Interface>::IID || iid == &<IMDSPStorage as ::windows::core::Interface>::IID || iid == &<IMDSPStorage2 as ::windows::core::Interface>::IID || iid == &<IMDSPStorage3 as ::windows::core::Interface>::IID
    }
}
pub trait IMDSPStorageGlobals_Impl: Sized {
    fn GetCapabilities(&self) -> ::windows::core::Result<u32>;
    fn GetSerialNumber(&self, pserialnum: *mut WMDMID, abmac: *mut u8) -> ::windows::core::Result<()>;
    fn GetTotalSize(&self, pdwtotalsizelow: *mut u32, pdwtotalsizehigh: *mut u32) -> ::windows::core::Result<()>;
    fn GetTotalFree(&self, pdwfreelow: *mut u32, pdwfreehigh: *mut u32) -> ::windows::core::Result<()>;
    fn GetTotalBad(&self, pdwbadlow: *mut u32, pdwbadhigh: *mut u32) -> ::windows::core::Result<()>;
    fn GetStatus(&self) -> ::windows::core::Result<u32>;
    fn Initialize(&self, fumode: u32, pprogress: &::core::option::Option<IWMDMProgress>) -> ::windows::core::Result<()>;
    fn GetDevice(&self) -> ::windows::core::Result<IMDSPDevice>;
    fn GetRootStorage(&self) -> ::windows::core::Result<IMDSPStorage>;
}
impl ::windows::core::RuntimeName for IMDSPStorageGlobals {}
impl IMDSPStorageGlobals_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMDSPStorageGlobals_Impl, const OFFSET: isize>() -> IMDSPStorageGlobals_Vtbl {
        unsafe extern "system" fn GetCapabilities<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMDSPStorageGlobals_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwcapabilities: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetCapabilities() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pdwcapabilities, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSerialNumber<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMDSPStorageGlobals_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pserialnum: *mut WMDMID, abmac: *mut u8) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetSerialNumber(::core::mem::transmute_copy(&pserialnum), ::core::mem::transmute_copy(&abmac)).into()
        }
        unsafe extern "system" fn GetTotalSize<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMDSPStorageGlobals_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwtotalsizelow: *mut u32, pdwtotalsizehigh: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetTotalSize(::core::mem::transmute_copy(&pdwtotalsizelow), ::core::mem::transmute_copy(&pdwtotalsizehigh)).into()
        }
        unsafe extern "system" fn GetTotalFree<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMDSPStorageGlobals_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwfreelow: *mut u32, pdwfreehigh: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetTotalFree(::core::mem::transmute_copy(&pdwfreelow), ::core::mem::transmute_copy(&pdwfreehigh)).into()
        }
        unsafe extern "system" fn GetTotalBad<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMDSPStorageGlobals_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwbadlow: *mut u32, pdwbadhigh: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetTotalBad(::core::mem::transmute_copy(&pdwbadlow), ::core::mem::transmute_copy(&pdwbadhigh)).into()
        }
        unsafe extern "system" fn GetStatus<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMDSPStorageGlobals_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwstatus: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetStatus() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pdwstatus, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Initialize<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMDSPStorageGlobals_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fumode: u32, pprogress: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Initialize(::core::mem::transmute_copy(&fumode), ::core::mem::transmute(&pprogress)).into()
        }
        unsafe extern "system" fn GetDevice<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMDSPStorageGlobals_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppdevice: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetDevice() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppdevice, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetRootStorage<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMDSPStorageGlobals_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pproot: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetRootStorage() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pproot, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            GetCapabilities: GetCapabilities::<Identity, Impl, OFFSET>,
            GetSerialNumber: GetSerialNumber::<Identity, Impl, OFFSET>,
            GetTotalSize: GetTotalSize::<Identity, Impl, OFFSET>,
            GetTotalFree: GetTotalFree::<Identity, Impl, OFFSET>,
            GetTotalBad: GetTotalBad::<Identity, Impl, OFFSET>,
            GetStatus: GetStatus::<Identity, Impl, OFFSET>,
            Initialize: Initialize::<Identity, Impl, OFFSET>,
            GetDevice: GetDevice::<Identity, Impl, OFFSET>,
            GetRootStorage: GetRootStorage::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMDSPStorageGlobals as ::windows::core::Interface>::IID
    }
}
pub trait IMDServiceProvider_Impl: Sized {
    fn GetDeviceCount(&self) -> ::windows::core::Result<u32>;
    fn EnumDevices(&self) -> ::windows::core::Result<IMDSPEnumDevice>;
}
impl ::windows::core::RuntimeName for IMDServiceProvider {}
impl IMDServiceProvider_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMDServiceProvider_Impl, const OFFSET: isize>() -> IMDServiceProvider_Vtbl {
        unsafe extern "system" fn GetDeviceCount<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMDServiceProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwcount: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetDeviceCount() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pdwcount, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnumDevices<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMDServiceProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenumdevice: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.EnumDevices() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppenumdevice, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            GetDeviceCount: GetDeviceCount::<Identity, Impl, OFFSET>,
            EnumDevices: EnumDevices::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMDServiceProvider as ::windows::core::Interface>::IID
    }
}
pub trait IMDServiceProvider2_Impl: Sized + IMDServiceProvider_Impl {
    fn CreateDevice(&self, pwszdevicepath: &::windows::core::PCWSTR, pdwcount: *mut u32, pppdevicearray: *mut *mut ::core::option::Option<IMDSPDevice>) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IMDServiceProvider2 {}
impl IMDServiceProvider2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMDServiceProvider2_Impl, const OFFSET: isize>() -> IMDServiceProvider2_Vtbl {
        unsafe extern "system" fn CreateDevice<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMDServiceProvider2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwszdevicepath: ::windows::core::PCWSTR, pdwcount: *mut u32, pppdevicearray: *mut *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.CreateDevice(::core::mem::transmute(&pwszdevicepath), ::core::mem::transmute_copy(&pdwcount), ::core::mem::transmute_copy(&pppdevicearray)).into()
        }
        Self { base__: IMDServiceProvider_Vtbl::new::<Identity, Impl, OFFSET>(), CreateDevice: CreateDevice::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMDServiceProvider2 as ::windows::core::Interface>::IID || iid == &<IMDServiceProvider as ::windows::core::Interface>::IID
    }
}
pub trait IMDServiceProvider3_Impl: Sized + IMDServiceProvider_Impl + IMDServiceProvider2_Impl {
    fn SetDeviceEnumPreference(&self, dwenumpref: u32) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IMDServiceProvider3 {}
impl IMDServiceProvider3_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMDServiceProvider3_Impl, const OFFSET: isize>() -> IMDServiceProvider3_Vtbl {
        unsafe extern "system" fn SetDeviceEnumPreference<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMDServiceProvider3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwenumpref: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetDeviceEnumPreference(::core::mem::transmute_copy(&dwenumpref)).into()
        }
        Self { base__: IMDServiceProvider2_Vtbl::new::<Identity, Impl, OFFSET>(), SetDeviceEnumPreference: SetDeviceEnumPreference::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMDServiceProvider3 as ::windows::core::Interface>::IID || iid == &<IMDServiceProvider as ::windows::core::Interface>::IID || iid == &<IMDServiceProvider2 as ::windows::core::Interface>::IID
    }
}
pub trait ISCPSecureAuthenticate_Impl: Sized {
    fn GetSecureQuery(&self) -> ::windows::core::Result<ISCPSecureQuery>;
}
impl ::windows::core::RuntimeName for ISCPSecureAuthenticate {}
impl ISCPSecureAuthenticate_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISCPSecureAuthenticate_Impl, const OFFSET: isize>() -> ISCPSecureAuthenticate_Vtbl {
        unsafe extern "system" fn GetSecureQuery<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISCPSecureAuthenticate_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppsecurequery: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetSecureQuery() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppsecurequery, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(), GetSecureQuery: GetSecureQuery::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISCPSecureAuthenticate as ::windows::core::Interface>::IID
    }
}
pub trait ISCPSecureAuthenticate2_Impl: Sized + ISCPSecureAuthenticate_Impl {
    fn GetSCPSession(&self) -> ::windows::core::Result<ISCPSession>;
}
impl ::windows::core::RuntimeName for ISCPSecureAuthenticate2 {}
impl ISCPSecureAuthenticate2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISCPSecureAuthenticate2_Impl, const OFFSET: isize>() -> ISCPSecureAuthenticate2_Vtbl {
        unsafe extern "system" fn GetSCPSession<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISCPSecureAuthenticate2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppscpsession: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetSCPSession() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppscpsession, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base__: ISCPSecureAuthenticate_Vtbl::new::<Identity, Impl, OFFSET>(), GetSCPSession: GetSCPSession::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISCPSecureAuthenticate2 as ::windows::core::Interface>::IID || iid == &<ISCPSecureAuthenticate as ::windows::core::Interface>::IID
    }
}
pub trait ISCPSecureExchange_Impl: Sized {
    fn TransferContainerData(&self, pdata: *const u8, dwsize: u32, pfureadyflags: *mut u32, abmac: *mut u8) -> ::windows::core::Result<()>;
    fn ObjectData(&self, pdata: *mut u8, pdwsize: *mut u32, abmac: *mut u8) -> ::windows::core::Result<()>;
    fn TransferComplete(&self) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for ISCPSecureExchange {}
impl ISCPSecureExchange_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISCPSecureExchange_Impl, const OFFSET: isize>() -> ISCPSecureExchange_Vtbl {
        unsafe extern "system" fn TransferContainerData<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISCPSecureExchange_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdata: *const u8, dwsize: u32, pfureadyflags: *mut u32, abmac: *mut u8) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.TransferContainerData(::core::mem::transmute_copy(&pdata), ::core::mem::transmute_copy(&dwsize), ::core::mem::transmute_copy(&pfureadyflags), ::core::mem::transmute_copy(&abmac)).into()
        }
        unsafe extern "system" fn ObjectData<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISCPSecureExchange_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdata: *mut u8, pdwsize: *mut u32, abmac: *mut u8) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.ObjectData(::core::mem::transmute_copy(&pdata), ::core::mem::transmute_copy(&pdwsize), ::core::mem::transmute_copy(&abmac)).into()
        }
        unsafe extern "system" fn TransferComplete<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISCPSecureExchange_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.TransferComplete().into()
        }
        Self {
            base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            TransferContainerData: TransferContainerData::<Identity, Impl, OFFSET>,
            ObjectData: ObjectData::<Identity, Impl, OFFSET>,
            TransferComplete: TransferComplete::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISCPSecureExchange as ::windows::core::Interface>::IID
    }
}
pub trait ISCPSecureExchange2_Impl: Sized + ISCPSecureExchange_Impl {
    fn TransferContainerData2(&self, pdata: *const u8, dwsize: u32, pprogresscallback: &::core::option::Option<IWMDMProgress3>, pfureadyflags: *mut u32, abmac: *mut u8) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for ISCPSecureExchange2 {}
impl ISCPSecureExchange2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISCPSecureExchange2_Impl, const OFFSET: isize>() -> ISCPSecureExchange2_Vtbl {
        unsafe extern "system" fn TransferContainerData2<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISCPSecureExchange2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdata: *const u8, dwsize: u32, pprogresscallback: *mut ::core::ffi::c_void, pfureadyflags: *mut u32, abmac: *mut u8) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.TransferContainerData2(::core::mem::transmute_copy(&pdata), ::core::mem::transmute_copy(&dwsize), ::core::mem::transmute(&pprogresscallback), ::core::mem::transmute_copy(&pfureadyflags), ::core::mem::transmute_copy(&abmac)).into()
        }
        Self { base__: ISCPSecureExchange_Vtbl::new::<Identity, Impl, OFFSET>(), TransferContainerData2: TransferContainerData2::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISCPSecureExchange2 as ::windows::core::Interface>::IID || iid == &<ISCPSecureExchange as ::windows::core::Interface>::IID
    }
}
pub trait ISCPSecureExchange3_Impl: Sized + ISCPSecureExchange_Impl + ISCPSecureExchange2_Impl {
    fn TransferContainerDataOnClearChannel(&self, pdevice: &::core::option::Option<IMDSPDevice>, pdata: *const u8, dwsize: u32, pprogresscallback: &::core::option::Option<IWMDMProgress3>) -> ::windows::core::Result<u32>;
    fn GetObjectDataOnClearChannel(&self, pdevice: &::core::option::Option<IMDSPDevice>, pdata: *mut u8, pdwsize: *mut u32) -> ::windows::core::Result<()>;
    fn TransferCompleteForDevice(&self, pdevice: &::core::option::Option<IMDSPDevice>) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for ISCPSecureExchange3 {}
impl ISCPSecureExchange3_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISCPSecureExchange3_Impl, const OFFSET: isize>() -> ISCPSecureExchange3_Vtbl {
        unsafe extern "system" fn TransferContainerDataOnClearChannel<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISCPSecureExchange3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdevice: *mut ::core::ffi::c_void, pdata: *const u8, dwsize: u32, pprogresscallback: *mut ::core::ffi::c_void, pfureadyflags: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.TransferContainerDataOnClearChannel(::core::mem::transmute(&pdevice), ::core::mem::transmute_copy(&pdata), ::core::mem::transmute_copy(&dwsize), ::core::mem::transmute(&pprogresscallback)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pfureadyflags, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetObjectDataOnClearChannel<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISCPSecureExchange3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdevice: *mut ::core::ffi::c_void, pdata: *mut u8, pdwsize: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetObjectDataOnClearChannel(::core::mem::transmute(&pdevice), ::core::mem::transmute_copy(&pdata), ::core::mem::transmute_copy(&pdwsize)).into()
        }
        unsafe extern "system" fn TransferCompleteForDevice<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISCPSecureExchange3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdevice: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.TransferCompleteForDevice(::core::mem::transmute(&pdevice)).into()
        }
        Self {
            base__: ISCPSecureExchange2_Vtbl::new::<Identity, Impl, OFFSET>(),
            TransferContainerDataOnClearChannel: TransferContainerDataOnClearChannel::<Identity, Impl, OFFSET>,
            GetObjectDataOnClearChannel: GetObjectDataOnClearChannel::<Identity, Impl, OFFSET>,
            TransferCompleteForDevice: TransferCompleteForDevice::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISCPSecureExchange3 as ::windows::core::Interface>::IID || iid == &<ISCPSecureExchange as ::windows::core::Interface>::IID || iid == &<ISCPSecureExchange2 as ::windows::core::Interface>::IID
    }
}
pub trait ISCPSecureQuery_Impl: Sized {
    fn GetDataDemands(&self, pfuflags: *mut u32, pdwminrightsdata: *mut u32, pdwminexaminedata: *mut u32, pdwmindecidedata: *mut u32, abmac: *mut u8) -> ::windows::core::Result<()>;
    fn ExamineData(&self, fuflags: u32, pwszextension: &::windows::core::PCWSTR, pdata: *const u8, dwsize: u32, abmac: *mut u8) -> ::windows::core::Result<()>;
    fn MakeDecision(&self, fuflags: u32, pdata: *const u8, dwsize: u32, dwappsec: u32, pbspsessionkey: *const u8, dwsessionkeylen: u32, pstorageglobals: &::core::option::Option<IMDSPStorageGlobals>, ppexchange: *mut ::core::option::Option<ISCPSecureExchange>, abmac: *mut u8) -> ::windows::core::Result<()>;
    fn GetRights(&self, pdata: *const u8, dwsize: u32, pbspsessionkey: *const u8, dwsessionkeylen: u32, pstgglobals: &::core::option::Option<IMDSPStorageGlobals>, pprights: *mut *mut WMDMRIGHTS, pnrightscount: *mut u32, abmac: *mut u8) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for ISCPSecureQuery {}
impl ISCPSecureQuery_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISCPSecureQuery_Impl, const OFFSET: isize>() -> ISCPSecureQuery_Vtbl {
        unsafe extern "system" fn GetDataDemands<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISCPSecureQuery_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfuflags: *mut u32, pdwminrightsdata: *mut u32, pdwminexaminedata: *mut u32, pdwmindecidedata: *mut u32, abmac: *mut u8) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetDataDemands(::core::mem::transmute_copy(&pfuflags), ::core::mem::transmute_copy(&pdwminrightsdata), ::core::mem::transmute_copy(&pdwminexaminedata), ::core::mem::transmute_copy(&pdwmindecidedata), ::core::mem::transmute_copy(&abmac)).into()
        }
        unsafe extern "system" fn ExamineData<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISCPSecureQuery_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fuflags: u32, pwszextension: ::windows::core::PCWSTR, pdata: *const u8, dwsize: u32, abmac: *mut u8) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.ExamineData(::core::mem::transmute_copy(&fuflags), ::core::mem::transmute(&pwszextension), ::core::mem::transmute_copy(&pdata), ::core::mem::transmute_copy(&dwsize), ::core::mem::transmute_copy(&abmac)).into()
        }
        unsafe extern "system" fn MakeDecision<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISCPSecureQuery_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fuflags: u32, pdata: *const u8, dwsize: u32, dwappsec: u32, pbspsessionkey: *const u8, dwsessionkeylen: u32, pstorageglobals: *mut ::core::ffi::c_void, ppexchange: *mut *mut ::core::ffi::c_void, abmac: *mut u8) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.MakeDecision(::core::mem::transmute_copy(&fuflags), ::core::mem::transmute_copy(&pdata), ::core::mem::transmute_copy(&dwsize), ::core::mem::transmute_copy(&dwappsec), ::core::mem::transmute_copy(&pbspsessionkey), ::core::mem::transmute_copy(&dwsessionkeylen), ::core::mem::transmute(&pstorageglobals), ::core::mem::transmute_copy(&ppexchange), ::core::mem::transmute_copy(&abmac)).into()
        }
        unsafe extern "system" fn GetRights<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISCPSecureQuery_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdata: *const u8, dwsize: u32, pbspsessionkey: *const u8, dwsessionkeylen: u32, pstgglobals: *mut ::core::ffi::c_void, pprights: *mut *mut WMDMRIGHTS, pnrightscount: *mut u32, abmac: *mut u8) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetRights(::core::mem::transmute_copy(&pdata), ::core::mem::transmute_copy(&dwsize), ::core::mem::transmute_copy(&pbspsessionkey), ::core::mem::transmute_copy(&dwsessionkeylen), ::core::mem::transmute(&pstgglobals), ::core::mem::transmute_copy(&pprights), ::core::mem::transmute_copy(&pnrightscount), ::core::mem::transmute_copy(&abmac)).into()
        }
        Self {
            base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            GetDataDemands: GetDataDemands::<Identity, Impl, OFFSET>,
            ExamineData: ExamineData::<Identity, Impl, OFFSET>,
            MakeDecision: MakeDecision::<Identity, Impl, OFFSET>,
            GetRights: GetRights::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISCPSecureQuery as ::windows::core::Interface>::IID
    }
}
pub trait ISCPSecureQuery2_Impl: Sized + ISCPSecureQuery_Impl {
    fn MakeDecision2(&self, fuflags: u32, pdata: *const u8, dwsize: u32, dwappsec: u32, pbspsessionkey: *const u8, dwsessionkeylen: u32, pstorageglobals: &::core::option::Option<IMDSPStorageGlobals>, pappcertapp: *const u8, dwappcertapplen: u32, pappcertsp: *const u8, dwappcertsplen: u32, pszrevocationurl: *mut ::windows::core::PWSTR, pdwrevocationurllen: *mut u32, pdwrevocationbitflag: *mut u32, pqwfilesize: *mut u64, punknown: &::core::option::Option<::windows::core::IUnknown>, ppexchange: *mut ::core::option::Option<ISCPSecureExchange>, abmac: *mut u8) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for ISCPSecureQuery2 {}
impl ISCPSecureQuery2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISCPSecureQuery2_Impl, const OFFSET: isize>() -> ISCPSecureQuery2_Vtbl {
        unsafe extern "system" fn MakeDecision2<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISCPSecureQuery2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fuflags: u32, pdata: *const u8, dwsize: u32, dwappsec: u32, pbspsessionkey: *const u8, dwsessionkeylen: u32, pstorageglobals: *mut ::core::ffi::c_void, pappcertapp: *const u8, dwappcertapplen: u32, pappcertsp: *const u8, dwappcertsplen: u32, pszrevocationurl: *mut ::windows::core::PWSTR, pdwrevocationurllen: *mut u32, pdwrevocationbitflag: *mut u32, pqwfilesize: *mut u64, punknown: *mut ::core::ffi::c_void, ppexchange: *mut *mut ::core::ffi::c_void, abmac: *mut u8) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.MakeDecision2(
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
        Self { base__: ISCPSecureQuery_Vtbl::new::<Identity, Impl, OFFSET>(), MakeDecision2: MakeDecision2::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISCPSecureQuery2 as ::windows::core::Interface>::IID || iid == &<ISCPSecureQuery as ::windows::core::Interface>::IID
    }
}
pub trait ISCPSecureQuery3_Impl: Sized + ISCPSecureQuery_Impl + ISCPSecureQuery2_Impl {
    fn GetRightsOnClearChannel(&self, pdata: *const u8, dwsize: u32, pbspsessionkey: *const u8, dwsessionkeylen: u32, pstgglobals: &::core::option::Option<IMDSPStorageGlobals>, pprogresscallback: &::core::option::Option<IWMDMProgress3>, pprights: *mut *mut WMDMRIGHTS, pnrightscount: *mut u32) -> ::windows::core::Result<()>;
    fn MakeDecisionOnClearChannel(&self, fuflags: u32, pdata: *const u8, dwsize: u32, dwappsec: u32, pbspsessionkey: *const u8, dwsessionkeylen: u32, pstorageglobals: &::core::option::Option<IMDSPStorageGlobals>, pprogresscallback: &::core::option::Option<IWMDMProgress3>, pappcertapp: *const u8, dwappcertapplen: u32, pappcertsp: *const u8, dwappcertsplen: u32, pszrevocationurl: *mut ::windows::core::PWSTR, pdwrevocationurllen: *mut u32, pdwrevocationbitflag: *mut u32, pqwfilesize: *mut u64, punknown: &::core::option::Option<::windows::core::IUnknown>, ppexchange: *mut ::core::option::Option<ISCPSecureExchange>) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for ISCPSecureQuery3 {}
impl ISCPSecureQuery3_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISCPSecureQuery3_Impl, const OFFSET: isize>() -> ISCPSecureQuery3_Vtbl {
        unsafe extern "system" fn GetRightsOnClearChannel<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISCPSecureQuery3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdata: *const u8, dwsize: u32, pbspsessionkey: *const u8, dwsessionkeylen: u32, pstgglobals: *mut ::core::ffi::c_void, pprogresscallback: *mut ::core::ffi::c_void, pprights: *mut *mut WMDMRIGHTS, pnrightscount: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetRightsOnClearChannel(::core::mem::transmute_copy(&pdata), ::core::mem::transmute_copy(&dwsize), ::core::mem::transmute_copy(&pbspsessionkey), ::core::mem::transmute_copy(&dwsessionkeylen), ::core::mem::transmute(&pstgglobals), ::core::mem::transmute(&pprogresscallback), ::core::mem::transmute_copy(&pprights), ::core::mem::transmute_copy(&pnrightscount)).into()
        }
        unsafe extern "system" fn MakeDecisionOnClearChannel<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISCPSecureQuery3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fuflags: u32, pdata: *const u8, dwsize: u32, dwappsec: u32, pbspsessionkey: *const u8, dwsessionkeylen: u32, pstorageglobals: *mut ::core::ffi::c_void, pprogresscallback: *mut ::core::ffi::c_void, pappcertapp: *const u8, dwappcertapplen: u32, pappcertsp: *const u8, dwappcertsplen: u32, pszrevocationurl: *mut ::windows::core::PWSTR, pdwrevocationurllen: *mut u32, pdwrevocationbitflag: *mut u32, pqwfilesize: *mut u64, punknown: *mut ::core::ffi::c_void, ppexchange: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.MakeDecisionOnClearChannel(
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
            base__: ISCPSecureQuery2_Vtbl::new::<Identity, Impl, OFFSET>(),
            GetRightsOnClearChannel: GetRightsOnClearChannel::<Identity, Impl, OFFSET>,
            MakeDecisionOnClearChannel: MakeDecisionOnClearChannel::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISCPSecureQuery3 as ::windows::core::Interface>::IID || iid == &<ISCPSecureQuery as ::windows::core::Interface>::IID || iid == &<ISCPSecureQuery2 as ::windows::core::Interface>::IID
    }
}
pub trait ISCPSession_Impl: Sized {
    fn BeginSession(&self, pidevice: &::core::option::Option<IMDSPDevice>, pctx: *const u8, dwsizectx: u32) -> ::windows::core::Result<()>;
    fn EndSession(&self, pctx: *const u8, dwsizectx: u32) -> ::windows::core::Result<()>;
    fn GetSecureQuery(&self) -> ::windows::core::Result<ISCPSecureQuery>;
}
impl ::windows::core::RuntimeName for ISCPSession {}
impl ISCPSession_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISCPSession_Impl, const OFFSET: isize>() -> ISCPSession_Vtbl {
        unsafe extern "system" fn BeginSession<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISCPSession_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pidevice: *mut ::core::ffi::c_void, pctx: *const u8, dwsizectx: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.BeginSession(::core::mem::transmute(&pidevice), ::core::mem::transmute_copy(&pctx), ::core::mem::transmute_copy(&dwsizectx)).into()
        }
        unsafe extern "system" fn EndSession<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISCPSession_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pctx: *const u8, dwsizectx: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.EndSession(::core::mem::transmute_copy(&pctx), ::core::mem::transmute_copy(&dwsizectx)).into()
        }
        unsafe extern "system" fn GetSecureQuery<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISCPSession_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppsecurequery: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetSecureQuery() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppsecurequery, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            BeginSession: BeginSession::<Identity, Impl, OFFSET>,
            EndSession: EndSession::<Identity, Impl, OFFSET>,
            GetSecureQuery: GetSecureQuery::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISCPSession as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Media_Audio")]
pub trait IWMDMDevice_Impl: Sized {
    fn GetName(&self, pwszname: ::windows::core::PWSTR, nmaxchars: u32) -> ::windows::core::Result<()>;
    fn GetManufacturer(&self, pwszname: ::windows::core::PWSTR, nmaxchars: u32) -> ::windows::core::Result<()>;
    fn GetVersion(&self) -> ::windows::core::Result<u32>;
    fn GetType(&self) -> ::windows::core::Result<u32>;
    fn GetSerialNumber(&self, pserialnumber: *mut WMDMID, abmac: *mut u8) -> ::windows::core::Result<()>;
    fn GetPowerSource(&self, pdwpowersource: *mut u32, pdwpercentremaining: *mut u32) -> ::windows::core::Result<()>;
    fn GetStatus(&self) -> ::windows::core::Result<u32>;
    fn GetDeviceIcon(&self) -> ::windows::core::Result<u32>;
    fn EnumStorage(&self) -> ::windows::core::Result<IWMDMEnumStorage>;
    fn GetFormatSupport(&self, ppformatex: *mut *mut super::Audio::WAVEFORMATEX, pnformatcount: *mut u32, pppwszmimetype: *mut *mut ::windows::core::PWSTR, pnmimetypecount: *mut u32) -> ::windows::core::Result<()>;
    fn SendOpaqueCommand(&self, pcommand: *mut OPAQUECOMMAND) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Media_Audio")]
impl ::windows::core::RuntimeName for IWMDMDevice {}
#[cfg(feature = "Win32_Media_Audio")]
impl IWMDMDevice_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMDMDevice_Impl, const OFFSET: isize>() -> IWMDMDevice_Vtbl {
        unsafe extern "system" fn GetName<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMDMDevice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwszname: ::windows::core::PWSTR, nmaxchars: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetName(::core::mem::transmute_copy(&pwszname), ::core::mem::transmute_copy(&nmaxchars)).into()
        }
        unsafe extern "system" fn GetManufacturer<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMDMDevice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwszname: ::windows::core::PWSTR, nmaxchars: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetManufacturer(::core::mem::transmute_copy(&pwszname), ::core::mem::transmute_copy(&nmaxchars)).into()
        }
        unsafe extern "system" fn GetVersion<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMDMDevice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwversion: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetVersion() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pdwversion, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetType<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMDMDevice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwtype: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetType() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pdwtype, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSerialNumber<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMDMDevice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pserialnumber: *mut WMDMID, abmac: *mut u8) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetSerialNumber(::core::mem::transmute_copy(&pserialnumber), ::core::mem::transmute_copy(&abmac)).into()
        }
        unsafe extern "system" fn GetPowerSource<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMDMDevice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwpowersource: *mut u32, pdwpercentremaining: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetPowerSource(::core::mem::transmute_copy(&pdwpowersource), ::core::mem::transmute_copy(&pdwpercentremaining)).into()
        }
        unsafe extern "system" fn GetStatus<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMDMDevice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwstatus: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetStatus() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pdwstatus, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDeviceIcon<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMDMDevice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hicon: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetDeviceIcon() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(hicon, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnumStorage<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMDMDevice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenumstorage: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.EnumStorage() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppenumstorage, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFormatSupport<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMDMDevice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppformatex: *mut *mut super::Audio::WAVEFORMATEX, pnformatcount: *mut u32, pppwszmimetype: *mut *mut ::windows::core::PWSTR, pnmimetypecount: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetFormatSupport(::core::mem::transmute_copy(&ppformatex), ::core::mem::transmute_copy(&pnformatcount), ::core::mem::transmute_copy(&pppwszmimetype), ::core::mem::transmute_copy(&pnmimetypecount)).into()
        }
        unsafe extern "system" fn SendOpaqueCommand<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMDMDevice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcommand: *mut OPAQUECOMMAND) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SendOpaqueCommand(::core::mem::transmute_copy(&pcommand)).into()
        }
        Self {
            base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            GetName: GetName::<Identity, Impl, OFFSET>,
            GetManufacturer: GetManufacturer::<Identity, Impl, OFFSET>,
            GetVersion: GetVersion::<Identity, Impl, OFFSET>,
            GetType: GetType::<Identity, Impl, OFFSET>,
            GetSerialNumber: GetSerialNumber::<Identity, Impl, OFFSET>,
            GetPowerSource: GetPowerSource::<Identity, Impl, OFFSET>,
            GetStatus: GetStatus::<Identity, Impl, OFFSET>,
            GetDeviceIcon: GetDeviceIcon::<Identity, Impl, OFFSET>,
            EnumStorage: EnumStorage::<Identity, Impl, OFFSET>,
            GetFormatSupport: GetFormatSupport::<Identity, Impl, OFFSET>,
            SendOpaqueCommand: SendOpaqueCommand::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWMDMDevice as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_Media_Audio", feature = "Win32_Media_MediaFoundation", feature = "Win32_System_Ole"))]
pub trait IWMDMDevice2_Impl: Sized + IWMDMDevice_Impl {
    fn GetStorage(&self, pszstoragename: &::windows::core::PCWSTR) -> ::windows::core::Result<IWMDMStorage>;
    fn GetFormatSupport2(&self, dwflags: u32, ppaudioformatex: *mut *mut super::Audio::WAVEFORMATEX, pnaudioformatcount: *mut u32, ppvideoformatex: *mut *mut super::MediaFoundation::VIDEOINFOHEADER, pnvideoformatcount: *mut u32, ppfiletype: *mut *mut WMFILECAPABILITIES, pnfiletypecount: *mut u32) -> ::windows::core::Result<()>;
    fn GetSpecifyPropertyPages(&self, ppspecifyproppages: *mut ::core::option::Option<super::super::System::Ole::ISpecifyPropertyPages>, pppunknowns: *mut *mut ::core::option::Option<::windows::core::IUnknown>, pcunks: *mut u32) -> ::windows::core::Result<()>;
    fn GetCanonicalName(&self, pwszpnpname: ::windows::core::PWSTR, nmaxchars: u32) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_Media_Audio", feature = "Win32_Media_MediaFoundation", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for IWMDMDevice2 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_Media_Audio", feature = "Win32_Media_MediaFoundation", feature = "Win32_System_Ole"))]
impl IWMDMDevice2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMDMDevice2_Impl, const OFFSET: isize>() -> IWMDMDevice2_Vtbl {
        unsafe extern "system" fn GetStorage<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMDMDevice2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszstoragename: ::windows::core::PCWSTR, ppstorage: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetStorage(::core::mem::transmute(&pszstoragename)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppstorage, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFormatSupport2<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMDMDevice2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwflags: u32, ppaudioformatex: *mut *mut super::Audio::WAVEFORMATEX, pnaudioformatcount: *mut u32, ppvideoformatex: *mut *mut super::MediaFoundation::VIDEOINFOHEADER, pnvideoformatcount: *mut u32, ppfiletype: *mut *mut WMFILECAPABILITIES, pnfiletypecount: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetFormatSupport2(::core::mem::transmute_copy(&dwflags), ::core::mem::transmute_copy(&ppaudioformatex), ::core::mem::transmute_copy(&pnaudioformatcount), ::core::mem::transmute_copy(&ppvideoformatex), ::core::mem::transmute_copy(&pnvideoformatcount), ::core::mem::transmute_copy(&ppfiletype), ::core::mem::transmute_copy(&pnfiletypecount)).into()
        }
        unsafe extern "system" fn GetSpecifyPropertyPages<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMDMDevice2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppspecifyproppages: *mut *mut ::core::ffi::c_void, pppunknowns: *mut *mut *mut ::core::ffi::c_void, pcunks: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetSpecifyPropertyPages(::core::mem::transmute_copy(&ppspecifyproppages), ::core::mem::transmute_copy(&pppunknowns), ::core::mem::transmute_copy(&pcunks)).into()
        }
        unsafe extern "system" fn GetCanonicalName<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMDMDevice2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwszpnpname: ::windows::core::PWSTR, nmaxchars: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetCanonicalName(::core::mem::transmute_copy(&pwszpnpname), ::core::mem::transmute_copy(&nmaxchars)).into()
        }
        Self {
            base__: IWMDMDevice_Vtbl::new::<Identity, Impl, OFFSET>(),
            GetStorage: GetStorage::<Identity, Impl, OFFSET>,
            GetFormatSupport2: GetFormatSupport2::<Identity, Impl, OFFSET>,
            GetSpecifyPropertyPages: GetSpecifyPropertyPages::<Identity, Impl, OFFSET>,
            GetCanonicalName: GetCanonicalName::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWMDMDevice2 as ::windows::core::Interface>::IID || iid == &<IWMDMDevice as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_Media_Audio", feature = "Win32_Media_MediaFoundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Ole"))]
pub trait IWMDMDevice3_Impl: Sized + IWMDMDevice_Impl + IWMDMDevice2_Impl {
    fn GetProperty(&self, pwszpropname: &::windows::core::PCWSTR) -> ::windows::core::Result<super::super::System::Com::StructuredStorage::PROPVARIANT>;
    fn SetProperty(&self, pwszpropname: &::windows::core::PCWSTR, pvalue: *const super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows::core::Result<()>;
    fn GetFormatCapability(&self, format: WMDM_FORMATCODE) -> ::windows::core::Result<WMDM_FORMAT_CAPABILITY>;
    fn DeviceIoControl(&self, dwiocontrolcode: u32, lpinbuffer: *const u8, ninbuffersize: u32, lpoutbuffer: *mut u8, pnoutbuffersize: *mut u32) -> ::windows::core::Result<()>;
    fn FindStorage(&self, findscope: WMDM_FIND_SCOPE, pwszuniqueid: &::windows::core::PCWSTR) -> ::windows::core::Result<IWMDMStorage>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_Media_Audio", feature = "Win32_Media_MediaFoundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for IWMDMDevice3 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_Media_Audio", feature = "Win32_Media_MediaFoundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Ole"))]
impl IWMDMDevice3_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMDMDevice3_Impl, const OFFSET: isize>() -> IWMDMDevice3_Vtbl {
        unsafe extern "system" fn GetProperty<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMDMDevice3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwszpropname: ::windows::core::PCWSTR, pvalue: *mut super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetProperty(::core::mem::transmute(&pwszpropname)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pvalue, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetProperty<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMDMDevice3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwszpropname: ::windows::core::PCWSTR, pvalue: *const super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetProperty(::core::mem::transmute(&pwszpropname), ::core::mem::transmute_copy(&pvalue)).into()
        }
        unsafe extern "system" fn GetFormatCapability<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMDMDevice3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, format: WMDM_FORMATCODE, pformatsupport: *mut WMDM_FORMAT_CAPABILITY) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetFormatCapability(::core::mem::transmute_copy(&format)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pformatsupport, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DeviceIoControl<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMDMDevice3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwiocontrolcode: u32, lpinbuffer: *const u8, ninbuffersize: u32, lpoutbuffer: *mut u8, pnoutbuffersize: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.DeviceIoControl(::core::mem::transmute_copy(&dwiocontrolcode), ::core::mem::transmute_copy(&lpinbuffer), ::core::mem::transmute_copy(&ninbuffersize), ::core::mem::transmute_copy(&lpoutbuffer), ::core::mem::transmute_copy(&pnoutbuffersize)).into()
        }
        unsafe extern "system" fn FindStorage<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMDMDevice3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, findscope: WMDM_FIND_SCOPE, pwszuniqueid: ::windows::core::PCWSTR, ppstorage: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.FindStorage(::core::mem::transmute_copy(&findscope), ::core::mem::transmute(&pwszuniqueid)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppstorage, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: IWMDMDevice2_Vtbl::new::<Identity, Impl, OFFSET>(),
            GetProperty: GetProperty::<Identity, Impl, OFFSET>,
            SetProperty: SetProperty::<Identity, Impl, OFFSET>,
            GetFormatCapability: GetFormatCapability::<Identity, Impl, OFFSET>,
            DeviceIoControl: DeviceIoControl::<Identity, Impl, OFFSET>,
            FindStorage: FindStorage::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWMDMDevice3 as ::windows::core::Interface>::IID || iid == &<IWMDMDevice as ::windows::core::Interface>::IID || iid == &<IWMDMDevice2 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Media_Audio")]
pub trait IWMDMDeviceControl_Impl: Sized {
    fn GetStatus(&self) -> ::windows::core::Result<u32>;
    fn GetCapabilities(&self) -> ::windows::core::Result<u32>;
    fn Play(&self) -> ::windows::core::Result<()>;
    fn Record(&self, pformat: *const super::Audio::WAVEFORMATEX) -> ::windows::core::Result<()>;
    fn Pause(&self) -> ::windows::core::Result<()>;
    fn Resume(&self) -> ::windows::core::Result<()>;
    fn Stop(&self) -> ::windows::core::Result<()>;
    fn Seek(&self, fumode: u32, noffset: i32) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Media_Audio")]
impl ::windows::core::RuntimeName for IWMDMDeviceControl {}
#[cfg(feature = "Win32_Media_Audio")]
impl IWMDMDeviceControl_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMDMDeviceControl_Impl, const OFFSET: isize>() -> IWMDMDeviceControl_Vtbl {
        unsafe extern "system" fn GetStatus<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMDMDeviceControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwstatus: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetStatus() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pdwstatus, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCapabilities<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMDMDeviceControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwcapabilitiesmask: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetCapabilities() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pdwcapabilitiesmask, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Play<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMDMDeviceControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Play().into()
        }
        unsafe extern "system" fn Record<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMDMDeviceControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pformat: *const super::Audio::WAVEFORMATEX) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Record(::core::mem::transmute_copy(&pformat)).into()
        }
        unsafe extern "system" fn Pause<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMDMDeviceControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Pause().into()
        }
        unsafe extern "system" fn Resume<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMDMDeviceControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Resume().into()
        }
        unsafe extern "system" fn Stop<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMDMDeviceControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Stop().into()
        }
        unsafe extern "system" fn Seek<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMDMDeviceControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fumode: u32, noffset: i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Seek(::core::mem::transmute_copy(&fumode), ::core::mem::transmute_copy(&noffset)).into()
        }
        Self {
            base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            GetStatus: GetStatus::<Identity, Impl, OFFSET>,
            GetCapabilities: GetCapabilities::<Identity, Impl, OFFSET>,
            Play: Play::<Identity, Impl, OFFSET>,
            Record: Record::<Identity, Impl, OFFSET>,
            Pause: Pause::<Identity, Impl, OFFSET>,
            Resume: Resume::<Identity, Impl, OFFSET>,
            Stop: Stop::<Identity, Impl, OFFSET>,
            Seek: Seek::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWMDMDeviceControl as ::windows::core::Interface>::IID
    }
}
pub trait IWMDMDeviceSession_Impl: Sized {
    fn BeginSession(&self, r#type: WMDM_SESSION_TYPE, pctx: *const u8, dwsizectx: u32) -> ::windows::core::Result<()>;
    fn EndSession(&self, r#type: WMDM_SESSION_TYPE, pctx: *const u8, dwsizectx: u32) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IWMDMDeviceSession {}
impl IWMDMDeviceSession_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMDMDeviceSession_Impl, const OFFSET: isize>() -> IWMDMDeviceSession_Vtbl {
        unsafe extern "system" fn BeginSession<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMDMDeviceSession_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, r#type: WMDM_SESSION_TYPE, pctx: *const u8, dwsizectx: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.BeginSession(::core::mem::transmute_copy(&r#type), ::core::mem::transmute_copy(&pctx), ::core::mem::transmute_copy(&dwsizectx)).into()
        }
        unsafe extern "system" fn EndSession<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMDMDeviceSession_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, r#type: WMDM_SESSION_TYPE, pctx: *const u8, dwsizectx: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.EndSession(::core::mem::transmute_copy(&r#type), ::core::mem::transmute_copy(&pctx), ::core::mem::transmute_copy(&dwsizectx)).into()
        }
        Self {
            base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            BeginSession: BeginSession::<Identity, Impl, OFFSET>,
            EndSession: EndSession::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWMDMDeviceSession as ::windows::core::Interface>::IID
    }
}
pub trait IWMDMEnumDevice_Impl: Sized {
    fn Next(&self, celt: u32, ppdevice: *mut ::core::option::Option<IWMDMDevice>, pceltfetched: *mut u32) -> ::windows::core::Result<()>;
    fn Skip(&self, celt: u32) -> ::windows::core::Result<u32>;
    fn Reset(&self) -> ::windows::core::Result<()>;
    fn Clone(&self) -> ::windows::core::Result<IWMDMEnumDevice>;
}
impl ::windows::core::RuntimeName for IWMDMEnumDevice {}
impl IWMDMEnumDevice_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMDMEnumDevice_Impl, const OFFSET: isize>() -> IWMDMEnumDevice_Vtbl {
        unsafe extern "system" fn Next<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMDMEnumDevice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32, ppdevice: *mut *mut ::core::ffi::c_void, pceltfetched: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Next(::core::mem::transmute_copy(&celt), ::core::mem::transmute_copy(&ppdevice), ::core::mem::transmute_copy(&pceltfetched)).into()
        }
        unsafe extern "system" fn Skip<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMDMEnumDevice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32, pceltfetched: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Skip(::core::mem::transmute_copy(&celt)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pceltfetched, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Reset<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMDMEnumDevice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Reset().into()
        }
        unsafe extern "system" fn Clone<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMDMEnumDevice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenumdevice: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Clone() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppenumdevice, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            Next: Next::<Identity, Impl, OFFSET>,
            Skip: Skip::<Identity, Impl, OFFSET>,
            Reset: Reset::<Identity, Impl, OFFSET>,
            Clone: Clone::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWMDMEnumDevice as ::windows::core::Interface>::IID
    }
}
pub trait IWMDMEnumStorage_Impl: Sized {
    fn Next(&self, celt: u32, ppstorage: *mut ::core::option::Option<IWMDMStorage>, pceltfetched: *mut u32) -> ::windows::core::Result<()>;
    fn Skip(&self, celt: u32) -> ::windows::core::Result<u32>;
    fn Reset(&self) -> ::windows::core::Result<()>;
    fn Clone(&self) -> ::windows::core::Result<IWMDMEnumStorage>;
}
impl ::windows::core::RuntimeName for IWMDMEnumStorage {}
impl IWMDMEnumStorage_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMDMEnumStorage_Impl, const OFFSET: isize>() -> IWMDMEnumStorage_Vtbl {
        unsafe extern "system" fn Next<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMDMEnumStorage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32, ppstorage: *mut *mut ::core::ffi::c_void, pceltfetched: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Next(::core::mem::transmute_copy(&celt), ::core::mem::transmute_copy(&ppstorage), ::core::mem::transmute_copy(&pceltfetched)).into()
        }
        unsafe extern "system" fn Skip<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMDMEnumStorage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32, pceltfetched: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Skip(::core::mem::transmute_copy(&celt)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pceltfetched, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Reset<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMDMEnumStorage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Reset().into()
        }
        unsafe extern "system" fn Clone<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMDMEnumStorage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenumstorage: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Clone() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppenumstorage, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            Next: Next::<Identity, Impl, OFFSET>,
            Skip: Skip::<Identity, Impl, OFFSET>,
            Reset: Reset::<Identity, Impl, OFFSET>,
            Clone: Clone::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWMDMEnumStorage as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IWMDMLogger_Impl: Sized {
    fn IsEnabled(&self) -> ::windows::core::Result<super::super::Foundation::BOOL>;
    fn Enable(&self, fenable: super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn GetLogFileName(&self, pszfilename: ::windows::core::PSTR, nmaxchars: u32) -> ::windows::core::Result<()>;
    fn SetLogFileName(&self, pszfilename: &::windows::core::PCSTR) -> ::windows::core::Result<()>;
    fn LogString(&self, dwflags: u32, pszsrcname: &::windows::core::PCSTR, pszlog: &::windows::core::PCSTR) -> ::windows::core::Result<()>;
    fn LogDword(&self, dwflags: u32, pszsrcname: &::windows::core::PCSTR, pszlogformat: &::windows::core::PCSTR, dwlog: u32) -> ::windows::core::Result<()>;
    fn Reset(&self) -> ::windows::core::Result<()>;
    fn GetSizeParams(&self, pdwmaxsize: *mut u32, pdwshrinktosize: *mut u32) -> ::windows::core::Result<()>;
    fn SetSizeParams(&self, dwmaxsize: u32, dwshrinktosize: u32) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::RuntimeName for IWMDMLogger {}
#[cfg(feature = "Win32_Foundation")]
impl IWMDMLogger_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMDMLogger_Impl, const OFFSET: isize>() -> IWMDMLogger_Vtbl {
        unsafe extern "system" fn IsEnabled<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMDMLogger_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfenabled: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.IsEnabled() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pfenabled, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Enable<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMDMLogger_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fenable: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Enable(::core::mem::transmute_copy(&fenable)).into()
        }
        unsafe extern "system" fn GetLogFileName<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMDMLogger_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszfilename: ::windows::core::PSTR, nmaxchars: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetLogFileName(::core::mem::transmute_copy(&pszfilename), ::core::mem::transmute_copy(&nmaxchars)).into()
        }
        unsafe extern "system" fn SetLogFileName<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMDMLogger_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszfilename: ::windows::core::PCSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetLogFileName(::core::mem::transmute(&pszfilename)).into()
        }
        unsafe extern "system" fn LogString<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMDMLogger_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwflags: u32, pszsrcname: ::windows::core::PCSTR, pszlog: ::windows::core::PCSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.LogString(::core::mem::transmute_copy(&dwflags), ::core::mem::transmute(&pszsrcname), ::core::mem::transmute(&pszlog)).into()
        }
        unsafe extern "system" fn LogDword<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMDMLogger_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwflags: u32, pszsrcname: ::windows::core::PCSTR, pszlogformat: ::windows::core::PCSTR, dwlog: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.LogDword(::core::mem::transmute_copy(&dwflags), ::core::mem::transmute(&pszsrcname), ::core::mem::transmute(&pszlogformat), ::core::mem::transmute_copy(&dwlog)).into()
        }
        unsafe extern "system" fn Reset<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMDMLogger_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Reset().into()
        }
        unsafe extern "system" fn GetSizeParams<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMDMLogger_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwmaxsize: *mut u32, pdwshrinktosize: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetSizeParams(::core::mem::transmute_copy(&pdwmaxsize), ::core::mem::transmute_copy(&pdwshrinktosize)).into()
        }
        unsafe extern "system" fn SetSizeParams<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMDMLogger_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwmaxsize: u32, dwshrinktosize: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetSizeParams(::core::mem::transmute_copy(&dwmaxsize), ::core::mem::transmute_copy(&dwshrinktosize)).into()
        }
        Self {
            base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            IsEnabled: IsEnabled::<Identity, Impl, OFFSET>,
            Enable: Enable::<Identity, Impl, OFFSET>,
            GetLogFileName: GetLogFileName::<Identity, Impl, OFFSET>,
            SetLogFileName: SetLogFileName::<Identity, Impl, OFFSET>,
            LogString: LogString::<Identity, Impl, OFFSET>,
            LogDword: LogDword::<Identity, Impl, OFFSET>,
            Reset: Reset::<Identity, Impl, OFFSET>,
            GetSizeParams: GetSizeParams::<Identity, Impl, OFFSET>,
            SetSizeParams: SetSizeParams::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWMDMLogger as ::windows::core::Interface>::IID
    }
}
pub trait IWMDMMetaData_Impl: Sized {
    fn AddItem(&self, r#type: WMDM_TAG_DATATYPE, pwsztagname: &::windows::core::PCWSTR, pvalue: *const u8, ilength: u32) -> ::windows::core::Result<()>;
    fn QueryByName(&self, pwsztagname: &::windows::core::PCWSTR, ptype: *mut WMDM_TAG_DATATYPE, pvalue: *mut *mut u8, pcblength: *mut u32) -> ::windows::core::Result<()>;
    fn QueryByIndex(&self, iindex: u32, ppwszname: *mut *mut u16, ptype: *mut WMDM_TAG_DATATYPE, ppvalue: *mut *mut u8, pcblength: *mut u32) -> ::windows::core::Result<()>;
    fn GetItemCount(&self) -> ::windows::core::Result<u32>;
}
impl ::windows::core::RuntimeName for IWMDMMetaData {}
impl IWMDMMetaData_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMDMMetaData_Impl, const OFFSET: isize>() -> IWMDMMetaData_Vtbl {
        unsafe extern "system" fn AddItem<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMDMMetaData_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, r#type: WMDM_TAG_DATATYPE, pwsztagname: ::windows::core::PCWSTR, pvalue: *const u8, ilength: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.AddItem(::core::mem::transmute_copy(&r#type), ::core::mem::transmute(&pwsztagname), ::core::mem::transmute_copy(&pvalue), ::core::mem::transmute_copy(&ilength)).into()
        }
        unsafe extern "system" fn QueryByName<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMDMMetaData_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwsztagname: ::windows::core::PCWSTR, ptype: *mut WMDM_TAG_DATATYPE, pvalue: *mut *mut u8, pcblength: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.QueryByName(::core::mem::transmute(&pwsztagname), ::core::mem::transmute_copy(&ptype), ::core::mem::transmute_copy(&pvalue), ::core::mem::transmute_copy(&pcblength)).into()
        }
        unsafe extern "system" fn QueryByIndex<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMDMMetaData_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, iindex: u32, ppwszname: *mut *mut u16, ptype: *mut WMDM_TAG_DATATYPE, ppvalue: *mut *mut u8, pcblength: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.QueryByIndex(::core::mem::transmute_copy(&iindex), ::core::mem::transmute_copy(&ppwszname), ::core::mem::transmute_copy(&ptype), ::core::mem::transmute_copy(&ppvalue), ::core::mem::transmute_copy(&pcblength)).into()
        }
        unsafe extern "system" fn GetItemCount<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMDMMetaData_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, icount: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetItemCount() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(icount, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            AddItem: AddItem::<Identity, Impl, OFFSET>,
            QueryByName: QueryByName::<Identity, Impl, OFFSET>,
            QueryByIndex: QueryByIndex::<Identity, Impl, OFFSET>,
            GetItemCount: GetItemCount::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWMDMMetaData as ::windows::core::Interface>::IID
    }
}
pub trait IWMDMNotification_Impl: Sized {
    fn WMDMMessage(&self, dwmessagetype: u32, pwszcanonicalname: &::windows::core::PCWSTR) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IWMDMNotification {}
impl IWMDMNotification_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMDMNotification_Impl, const OFFSET: isize>() -> IWMDMNotification_Vtbl {
        unsafe extern "system" fn WMDMMessage<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMDMNotification_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwmessagetype: u32, pwszcanonicalname: ::windows::core::PCWSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.WMDMMessage(::core::mem::transmute_copy(&dwmessagetype), ::core::mem::transmute(&pwszcanonicalname)).into()
        }
        Self { base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(), WMDMMessage: WMDMMessage::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWMDMNotification as ::windows::core::Interface>::IID
    }
}
pub trait IWMDMObjectInfo_Impl: Sized {
    fn GetPlayLength(&self) -> ::windows::core::Result<u32>;
    fn SetPlayLength(&self, dwlength: u32) -> ::windows::core::Result<()>;
    fn GetPlayOffset(&self) -> ::windows::core::Result<u32>;
    fn SetPlayOffset(&self, dwoffset: u32) -> ::windows::core::Result<()>;
    fn GetTotalLength(&self) -> ::windows::core::Result<u32>;
    fn GetLastPlayPosition(&self) -> ::windows::core::Result<u32>;
    fn GetLongestPlayPosition(&self) -> ::windows::core::Result<u32>;
}
impl ::windows::core::RuntimeName for IWMDMObjectInfo {}
impl IWMDMObjectInfo_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMDMObjectInfo_Impl, const OFFSET: isize>() -> IWMDMObjectInfo_Vtbl {
        unsafe extern "system" fn GetPlayLength<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMDMObjectInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwlength: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetPlayLength() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pdwlength, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPlayLength<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMDMObjectInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwlength: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetPlayLength(::core::mem::transmute_copy(&dwlength)).into()
        }
        unsafe extern "system" fn GetPlayOffset<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMDMObjectInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwoffset: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetPlayOffset() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pdwoffset, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPlayOffset<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMDMObjectInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwoffset: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetPlayOffset(::core::mem::transmute_copy(&dwoffset)).into()
        }
        unsafe extern "system" fn GetTotalLength<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMDMObjectInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwlength: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetTotalLength() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pdwlength, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetLastPlayPosition<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMDMObjectInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwlastpos: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetLastPlayPosition() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pdwlastpos, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetLongestPlayPosition<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMDMObjectInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwlongestpos: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetLongestPlayPosition() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pdwlongestpos, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            GetPlayLength: GetPlayLength::<Identity, Impl, OFFSET>,
            SetPlayLength: SetPlayLength::<Identity, Impl, OFFSET>,
            GetPlayOffset: GetPlayOffset::<Identity, Impl, OFFSET>,
            SetPlayOffset: SetPlayOffset::<Identity, Impl, OFFSET>,
            GetTotalLength: GetTotalLength::<Identity, Impl, OFFSET>,
            GetLastPlayPosition: GetLastPlayPosition::<Identity, Impl, OFFSET>,
            GetLongestPlayPosition: GetLongestPlayPosition::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWMDMObjectInfo as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Media_Audio")]
pub trait IWMDMOperation_Impl: Sized {
    fn BeginRead(&self) -> ::windows::core::Result<()>;
    fn BeginWrite(&self) -> ::windows::core::Result<()>;
    fn GetObjectName(&self, pwszname: ::windows::core::PWSTR, nmaxchars: u32) -> ::windows::core::Result<()>;
    fn SetObjectName(&self, pwszname: &::windows::core::PCWSTR, nmaxchars: u32) -> ::windows::core::Result<()>;
    fn GetObjectAttributes(&self, pdwattributes: *mut u32, pformat: *mut super::Audio::WAVEFORMATEX) -> ::windows::core::Result<()>;
    fn SetObjectAttributes(&self, dwattributes: u32, pformat: *const super::Audio::WAVEFORMATEX) -> ::windows::core::Result<()>;
    fn GetObjectTotalSize(&self, pdwsize: *mut u32, pdwsizehigh: *mut u32) -> ::windows::core::Result<()>;
    fn SetObjectTotalSize(&self, dwsize: u32, dwsizehigh: u32) -> ::windows::core::Result<()>;
    fn TransferObjectData(&self, pdata: *mut u8, pdwsize: *mut u32, abmac: *mut u8) -> ::windows::core::Result<()>;
    fn End(&self, phcompletioncode: *const ::windows::core::HRESULT, pnewobject: &::core::option::Option<::windows::core::IUnknown>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Media_Audio")]
impl ::windows::core::RuntimeName for IWMDMOperation {}
#[cfg(feature = "Win32_Media_Audio")]
impl IWMDMOperation_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMDMOperation_Impl, const OFFSET: isize>() -> IWMDMOperation_Vtbl {
        unsafe extern "system" fn BeginRead<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMDMOperation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.BeginRead().into()
        }
        unsafe extern "system" fn BeginWrite<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMDMOperation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.BeginWrite().into()
        }
        unsafe extern "system" fn GetObjectName<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMDMOperation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwszname: ::windows::core::PWSTR, nmaxchars: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetObjectName(::core::mem::transmute_copy(&pwszname), ::core::mem::transmute_copy(&nmaxchars)).into()
        }
        unsafe extern "system" fn SetObjectName<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMDMOperation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwszname: ::windows::core::PCWSTR, nmaxchars: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetObjectName(::core::mem::transmute(&pwszname), ::core::mem::transmute_copy(&nmaxchars)).into()
        }
        unsafe extern "system" fn GetObjectAttributes<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMDMOperation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwattributes: *mut u32, pformat: *mut super::Audio::WAVEFORMATEX) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetObjectAttributes(::core::mem::transmute_copy(&pdwattributes), ::core::mem::transmute_copy(&pformat)).into()
        }
        unsafe extern "system" fn SetObjectAttributes<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMDMOperation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwattributes: u32, pformat: *const super::Audio::WAVEFORMATEX) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetObjectAttributes(::core::mem::transmute_copy(&dwattributes), ::core::mem::transmute_copy(&pformat)).into()
        }
        unsafe extern "system" fn GetObjectTotalSize<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMDMOperation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwsize: *mut u32, pdwsizehigh: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetObjectTotalSize(::core::mem::transmute_copy(&pdwsize), ::core::mem::transmute_copy(&pdwsizehigh)).into()
        }
        unsafe extern "system" fn SetObjectTotalSize<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMDMOperation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwsize: u32, dwsizehigh: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetObjectTotalSize(::core::mem::transmute_copy(&dwsize), ::core::mem::transmute_copy(&dwsizehigh)).into()
        }
        unsafe extern "system" fn TransferObjectData<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMDMOperation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdata: *mut u8, pdwsize: *mut u32, abmac: *mut u8) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.TransferObjectData(::core::mem::transmute_copy(&pdata), ::core::mem::transmute_copy(&pdwsize), ::core::mem::transmute_copy(&abmac)).into()
        }
        unsafe extern "system" fn End<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMDMOperation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, phcompletioncode: *const ::windows::core::HRESULT, pnewobject: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.End(::core::mem::transmute_copy(&phcompletioncode), ::core::mem::transmute(&pnewobject)).into()
        }
        Self {
            base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            BeginRead: BeginRead::<Identity, Impl, OFFSET>,
            BeginWrite: BeginWrite::<Identity, Impl, OFFSET>,
            GetObjectName: GetObjectName::<Identity, Impl, OFFSET>,
            SetObjectName: SetObjectName::<Identity, Impl, OFFSET>,
            GetObjectAttributes: GetObjectAttributes::<Identity, Impl, OFFSET>,
            SetObjectAttributes: SetObjectAttributes::<Identity, Impl, OFFSET>,
            GetObjectTotalSize: GetObjectTotalSize::<Identity, Impl, OFFSET>,
            SetObjectTotalSize: SetObjectTotalSize::<Identity, Impl, OFFSET>,
            TransferObjectData: TransferObjectData::<Identity, Impl, OFFSET>,
            End: End::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWMDMOperation as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_Media_Audio", feature = "Win32_Media_MediaFoundation"))]
pub trait IWMDMOperation2_Impl: Sized + IWMDMOperation_Impl {
    fn SetObjectAttributes2(&self, dwattributes: u32, dwattributesex: u32, pformat: *const super::Audio::WAVEFORMATEX, pvideoformat: *const super::MediaFoundation::VIDEOINFOHEADER) -> ::windows::core::Result<()>;
    fn GetObjectAttributes2(&self, pdwattributes: *mut u32, pdwattributesex: *mut u32, paudioformat: *mut super::Audio::WAVEFORMATEX, pvideoformat: *mut super::MediaFoundation::VIDEOINFOHEADER) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_Media_Audio", feature = "Win32_Media_MediaFoundation"))]
impl ::windows::core::RuntimeName for IWMDMOperation2 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_Media_Audio", feature = "Win32_Media_MediaFoundation"))]
impl IWMDMOperation2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMDMOperation2_Impl, const OFFSET: isize>() -> IWMDMOperation2_Vtbl {
        unsafe extern "system" fn SetObjectAttributes2<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMDMOperation2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwattributes: u32, dwattributesex: u32, pformat: *const super::Audio::WAVEFORMATEX, pvideoformat: *const super::MediaFoundation::VIDEOINFOHEADER) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetObjectAttributes2(::core::mem::transmute_copy(&dwattributes), ::core::mem::transmute_copy(&dwattributesex), ::core::mem::transmute_copy(&pformat), ::core::mem::transmute_copy(&pvideoformat)).into()
        }
        unsafe extern "system" fn GetObjectAttributes2<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMDMOperation2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwattributes: *mut u32, pdwattributesex: *mut u32, paudioformat: *mut super::Audio::WAVEFORMATEX, pvideoformat: *mut super::MediaFoundation::VIDEOINFOHEADER) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetObjectAttributes2(::core::mem::transmute_copy(&pdwattributes), ::core::mem::transmute_copy(&pdwattributesex), ::core::mem::transmute_copy(&paudioformat), ::core::mem::transmute_copy(&pvideoformat)).into()
        }
        Self {
            base__: IWMDMOperation_Vtbl::new::<Identity, Impl, OFFSET>(),
            SetObjectAttributes2: SetObjectAttributes2::<Identity, Impl, OFFSET>,
            GetObjectAttributes2: GetObjectAttributes2::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWMDMOperation2 as ::windows::core::Interface>::IID || iid == &<IWMDMOperation as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Media_Audio")]
pub trait IWMDMOperation3_Impl: Sized + IWMDMOperation_Impl {
    fn TransferObjectDataOnClearChannel(&self, pdata: *mut u8, pdwsize: *mut u32) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Media_Audio")]
impl ::windows::core::RuntimeName for IWMDMOperation3 {}
#[cfg(feature = "Win32_Media_Audio")]
impl IWMDMOperation3_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMDMOperation3_Impl, const OFFSET: isize>() -> IWMDMOperation3_Vtbl {
        unsafe extern "system" fn TransferObjectDataOnClearChannel<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMDMOperation3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdata: *mut u8, pdwsize: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.TransferObjectDataOnClearChannel(::core::mem::transmute_copy(&pdata), ::core::mem::transmute_copy(&pdwsize)).into()
        }
        Self {
            base__: IWMDMOperation_Vtbl::new::<Identity, Impl, OFFSET>(),
            TransferObjectDataOnClearChannel: TransferObjectDataOnClearChannel::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWMDMOperation3 as ::windows::core::Interface>::IID || iid == &<IWMDMOperation as ::windows::core::Interface>::IID
    }
}
pub trait IWMDMProgress_Impl: Sized {
    fn Begin(&self, dwestimatedticks: u32) -> ::windows::core::Result<()>;
    fn Progress(&self, dwtranspiredticks: u32) -> ::windows::core::Result<()>;
    fn End(&self) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IWMDMProgress {}
impl IWMDMProgress_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMDMProgress_Impl, const OFFSET: isize>() -> IWMDMProgress_Vtbl {
        unsafe extern "system" fn Begin<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMDMProgress_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwestimatedticks: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Begin(::core::mem::transmute_copy(&dwestimatedticks)).into()
        }
        unsafe extern "system" fn Progress<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMDMProgress_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwtranspiredticks: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Progress(::core::mem::transmute_copy(&dwtranspiredticks)).into()
        }
        unsafe extern "system" fn End<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMDMProgress_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.End().into()
        }
        Self {
            base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            Begin: Begin::<Identity, Impl, OFFSET>,
            Progress: Progress::<Identity, Impl, OFFSET>,
            End: End::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWMDMProgress as ::windows::core::Interface>::IID
    }
}
pub trait IWMDMProgress2_Impl: Sized + IWMDMProgress_Impl {
    fn End2(&self, hrcompletioncode: ::windows::core::HRESULT) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IWMDMProgress2 {}
impl IWMDMProgress2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMDMProgress2_Impl, const OFFSET: isize>() -> IWMDMProgress2_Vtbl {
        unsafe extern "system" fn End2<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMDMProgress2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hrcompletioncode: ::windows::core::HRESULT) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.End2(::core::mem::transmute_copy(&hrcompletioncode)).into()
        }
        Self { base__: IWMDMProgress_Vtbl::new::<Identity, Impl, OFFSET>(), End2: End2::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWMDMProgress2 as ::windows::core::Interface>::IID || iid == &<IWMDMProgress as ::windows::core::Interface>::IID
    }
}
pub trait IWMDMProgress3_Impl: Sized + IWMDMProgress_Impl + IWMDMProgress2_Impl {
    fn Begin3(&self, eventid: &::windows::core::GUID, dwestimatedticks: u32, pcontext: *mut OPAQUECOMMAND) -> ::windows::core::Result<()>;
    fn Progress3(&self, eventid: &::windows::core::GUID, dwtranspiredticks: u32, pcontext: *mut OPAQUECOMMAND) -> ::windows::core::Result<()>;
    fn End3(&self, eventid: &::windows::core::GUID, hrcompletioncode: ::windows::core::HRESULT, pcontext: *mut OPAQUECOMMAND) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IWMDMProgress3 {}
impl IWMDMProgress3_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMDMProgress3_Impl, const OFFSET: isize>() -> IWMDMProgress3_Vtbl {
        unsafe extern "system" fn Begin3<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMDMProgress3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, eventid: ::windows::core::GUID, dwestimatedticks: u32, pcontext: *mut OPAQUECOMMAND) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Begin3(::core::mem::transmute(&eventid), ::core::mem::transmute_copy(&dwestimatedticks), ::core::mem::transmute_copy(&pcontext)).into()
        }
        unsafe extern "system" fn Progress3<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMDMProgress3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, eventid: ::windows::core::GUID, dwtranspiredticks: u32, pcontext: *mut OPAQUECOMMAND) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Progress3(::core::mem::transmute(&eventid), ::core::mem::transmute_copy(&dwtranspiredticks), ::core::mem::transmute_copy(&pcontext)).into()
        }
        unsafe extern "system" fn End3<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMDMProgress3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, eventid: ::windows::core::GUID, hrcompletioncode: ::windows::core::HRESULT, pcontext: *mut OPAQUECOMMAND) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.End3(::core::mem::transmute(&eventid), ::core::mem::transmute_copy(&hrcompletioncode), ::core::mem::transmute_copy(&pcontext)).into()
        }
        Self {
            base__: IWMDMProgress2_Vtbl::new::<Identity, Impl, OFFSET>(),
            Begin3: Begin3::<Identity, Impl, OFFSET>,
            Progress3: Progress3::<Identity, Impl, OFFSET>,
            End3: End3::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWMDMProgress3 as ::windows::core::Interface>::IID || iid == &<IWMDMProgress as ::windows::core::Interface>::IID || iid == &<IWMDMProgress2 as ::windows::core::Interface>::IID
    }
}
pub trait IWMDMRevoked_Impl: Sized {
    fn GetRevocationURL(&self, ppwszrevocationurl: *mut ::windows::core::PWSTR, pdwbufferlen: *mut u32, pdwrevokedbitflag: *mut u32) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IWMDMRevoked {}
impl IWMDMRevoked_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMDMRevoked_Impl, const OFFSET: isize>() -> IWMDMRevoked_Vtbl {
        unsafe extern "system" fn GetRevocationURL<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMDMRevoked_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppwszrevocationurl: *mut ::windows::core::PWSTR, pdwbufferlen: *mut u32, pdwrevokedbitflag: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetRevocationURL(::core::mem::transmute_copy(&ppwszrevocationurl), ::core::mem::transmute_copy(&pdwbufferlen), ::core::mem::transmute_copy(&pdwrevokedbitflag)).into()
        }
        Self { base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(), GetRevocationURL: GetRevocationURL::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWMDMRevoked as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Media_Audio")]
pub trait IWMDMStorage_Impl: Sized {
    fn SetAttributes(&self, dwattributes: u32, pformat: *const super::Audio::WAVEFORMATEX) -> ::windows::core::Result<()>;
    fn GetStorageGlobals(&self) -> ::windows::core::Result<IWMDMStorageGlobals>;
    fn GetAttributes(&self, pdwattributes: *mut u32, pformat: *mut super::Audio::WAVEFORMATEX) -> ::windows::core::Result<()>;
    fn GetName(&self, pwszname: ::windows::core::PWSTR, nmaxchars: u32) -> ::windows::core::Result<()>;
    fn GetDate(&self) -> ::windows::core::Result<WMDMDATETIME>;
    fn GetSize(&self, pdwsizelow: *mut u32, pdwsizehigh: *mut u32) -> ::windows::core::Result<()>;
    fn GetRights(&self, pprights: *mut *mut WMDMRIGHTS, pnrightscount: *mut u32, abmac: *mut u8) -> ::windows::core::Result<()>;
    fn EnumStorage(&self) -> ::windows::core::Result<IWMDMEnumStorage>;
    fn SendOpaqueCommand(&self, pcommand: *mut OPAQUECOMMAND) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Media_Audio")]
impl ::windows::core::RuntimeName for IWMDMStorage {}
#[cfg(feature = "Win32_Media_Audio")]
impl IWMDMStorage_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMDMStorage_Impl, const OFFSET: isize>() -> IWMDMStorage_Vtbl {
        unsafe extern "system" fn SetAttributes<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMDMStorage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwattributes: u32, pformat: *const super::Audio::WAVEFORMATEX) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetAttributes(::core::mem::transmute_copy(&dwattributes), ::core::mem::transmute_copy(&pformat)).into()
        }
        unsafe extern "system" fn GetStorageGlobals<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMDMStorage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppstorageglobals: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetStorageGlobals() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppstorageglobals, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetAttributes<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMDMStorage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwattributes: *mut u32, pformat: *mut super::Audio::WAVEFORMATEX) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetAttributes(::core::mem::transmute_copy(&pdwattributes), ::core::mem::transmute_copy(&pformat)).into()
        }
        unsafe extern "system" fn GetName<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMDMStorage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwszname: ::windows::core::PWSTR, nmaxchars: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetName(::core::mem::transmute_copy(&pwszname), ::core::mem::transmute_copy(&nmaxchars)).into()
        }
        unsafe extern "system" fn GetDate<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMDMStorage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdatetimeutc: *mut WMDMDATETIME) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetDate() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pdatetimeutc, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSize<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMDMStorage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwsizelow: *mut u32, pdwsizehigh: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetSize(::core::mem::transmute_copy(&pdwsizelow), ::core::mem::transmute_copy(&pdwsizehigh)).into()
        }
        unsafe extern "system" fn GetRights<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMDMStorage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pprights: *mut *mut WMDMRIGHTS, pnrightscount: *mut u32, abmac: *mut u8) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetRights(::core::mem::transmute_copy(&pprights), ::core::mem::transmute_copy(&pnrightscount), ::core::mem::transmute_copy(&abmac)).into()
        }
        unsafe extern "system" fn EnumStorage<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMDMStorage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, penumstorage: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.EnumStorage() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(penumstorage, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SendOpaqueCommand<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMDMStorage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcommand: *mut OPAQUECOMMAND) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SendOpaqueCommand(::core::mem::transmute_copy(&pcommand)).into()
        }
        Self {
            base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            SetAttributes: SetAttributes::<Identity, Impl, OFFSET>,
            GetStorageGlobals: GetStorageGlobals::<Identity, Impl, OFFSET>,
            GetAttributes: GetAttributes::<Identity, Impl, OFFSET>,
            GetName: GetName::<Identity, Impl, OFFSET>,
            GetDate: GetDate::<Identity, Impl, OFFSET>,
            GetSize: GetSize::<Identity, Impl, OFFSET>,
            GetRights: GetRights::<Identity, Impl, OFFSET>,
            EnumStorage: EnumStorage::<Identity, Impl, OFFSET>,
            SendOpaqueCommand: SendOpaqueCommand::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWMDMStorage as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_Media_Audio", feature = "Win32_Media_MediaFoundation"))]
pub trait IWMDMStorage2_Impl: Sized + IWMDMStorage_Impl {
    fn GetStorage(&self, pszstoragename: &::windows::core::PCWSTR) -> ::windows::core::Result<IWMDMStorage>;
    fn SetAttributes2(&self, dwattributes: u32, dwattributesex: u32, pformat: *const super::Audio::WAVEFORMATEX, pvideoformat: *const super::MediaFoundation::VIDEOINFOHEADER) -> ::windows::core::Result<()>;
    fn GetAttributes2(&self, pdwattributes: *mut u32, pdwattributesex: *mut u32, paudioformat: *mut super::Audio::WAVEFORMATEX, pvideoformat: *mut super::MediaFoundation::VIDEOINFOHEADER) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_Media_Audio", feature = "Win32_Media_MediaFoundation"))]
impl ::windows::core::RuntimeName for IWMDMStorage2 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_Media_Audio", feature = "Win32_Media_MediaFoundation"))]
impl IWMDMStorage2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMDMStorage2_Impl, const OFFSET: isize>() -> IWMDMStorage2_Vtbl {
        unsafe extern "system" fn GetStorage<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMDMStorage2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszstoragename: ::windows::core::PCWSTR, ppstorage: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetStorage(::core::mem::transmute(&pszstoragename)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppstorage, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAttributes2<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMDMStorage2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwattributes: u32, dwattributesex: u32, pformat: *const super::Audio::WAVEFORMATEX, pvideoformat: *const super::MediaFoundation::VIDEOINFOHEADER) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetAttributes2(::core::mem::transmute_copy(&dwattributes), ::core::mem::transmute_copy(&dwattributesex), ::core::mem::transmute_copy(&pformat), ::core::mem::transmute_copy(&pvideoformat)).into()
        }
        unsafe extern "system" fn GetAttributes2<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMDMStorage2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwattributes: *mut u32, pdwattributesex: *mut u32, paudioformat: *mut super::Audio::WAVEFORMATEX, pvideoformat: *mut super::MediaFoundation::VIDEOINFOHEADER) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetAttributes2(::core::mem::transmute_copy(&pdwattributes), ::core::mem::transmute_copy(&pdwattributesex), ::core::mem::transmute_copy(&paudioformat), ::core::mem::transmute_copy(&pvideoformat)).into()
        }
        Self {
            base__: IWMDMStorage_Vtbl::new::<Identity, Impl, OFFSET>(),
            GetStorage: GetStorage::<Identity, Impl, OFFSET>,
            SetAttributes2: SetAttributes2::<Identity, Impl, OFFSET>,
            GetAttributes2: GetAttributes2::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWMDMStorage2 as ::windows::core::Interface>::IID || iid == &<IWMDMStorage as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_Media_Audio", feature = "Win32_Media_MediaFoundation"))]
pub trait IWMDMStorage3_Impl: Sized + IWMDMStorage_Impl + IWMDMStorage2_Impl {
    fn GetMetadata(&self) -> ::windows::core::Result<IWMDMMetaData>;
    fn SetMetadata(&self, pmetadata: &::core::option::Option<IWMDMMetaData>) -> ::windows::core::Result<()>;
    fn CreateEmptyMetadataObject(&self) -> ::windows::core::Result<IWMDMMetaData>;
    fn SetEnumPreference(&self, pmode: *mut WMDM_STORAGE_ENUM_MODE, nviews: u32, pviews: *const WMDMMetadataView) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_Media_Audio", feature = "Win32_Media_MediaFoundation"))]
impl ::windows::core::RuntimeName for IWMDMStorage3 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_Media_Audio", feature = "Win32_Media_MediaFoundation"))]
impl IWMDMStorage3_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMDMStorage3_Impl, const OFFSET: isize>() -> IWMDMStorage3_Vtbl {
        unsafe extern "system" fn GetMetadata<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMDMStorage3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppmetadata: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetMetadata() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppmetadata, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMetadata<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMDMStorage3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pmetadata: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetMetadata(::core::mem::transmute(&pmetadata)).into()
        }
        unsafe extern "system" fn CreateEmptyMetadataObject<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMDMStorage3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppmetadata: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CreateEmptyMetadataObject() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppmetadata, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetEnumPreference<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMDMStorage3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pmode: *mut WMDM_STORAGE_ENUM_MODE, nviews: u32, pviews: *const WMDMMetadataView) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetEnumPreference(::core::mem::transmute_copy(&pmode), ::core::mem::transmute_copy(&nviews), ::core::mem::transmute_copy(&pviews)).into()
        }
        Self {
            base__: IWMDMStorage2_Vtbl::new::<Identity, Impl, OFFSET>(),
            GetMetadata: GetMetadata::<Identity, Impl, OFFSET>,
            SetMetadata: SetMetadata::<Identity, Impl, OFFSET>,
            CreateEmptyMetadataObject: CreateEmptyMetadataObject::<Identity, Impl, OFFSET>,
            SetEnumPreference: SetEnumPreference::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWMDMStorage3 as ::windows::core::Interface>::IID || iid == &<IWMDMStorage as ::windows::core::Interface>::IID || iid == &<IWMDMStorage2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_Media_Audio", feature = "Win32_Media_MediaFoundation"))]
pub trait IWMDMStorage4_Impl: Sized + IWMDMStorage_Impl + IWMDMStorage2_Impl + IWMDMStorage3_Impl {
    fn SetReferences(&self, dwrefs: u32, ppiwmdmstorage: *const ::core::option::Option<IWMDMStorage>) -> ::windows::core::Result<()>;
    fn GetReferences(&self, pdwrefs: *mut u32, pppiwmdmstorage: *mut *mut ::core::option::Option<IWMDMStorage>) -> ::windows::core::Result<()>;
    fn GetRightsWithProgress(&self, piprogresscallback: &::core::option::Option<IWMDMProgress3>, pprights: *mut *mut WMDMRIGHTS, pnrightscount: *mut u32) -> ::windows::core::Result<()>;
    fn GetSpecifiedMetadata(&self, cproperties: u32, ppwszpropnames: *const ::windows::core::PWSTR) -> ::windows::core::Result<IWMDMMetaData>;
    fn FindStorage(&self, findscope: WMDM_FIND_SCOPE, pwszuniqueid: &::windows::core::PCWSTR) -> ::windows::core::Result<IWMDMStorage>;
    fn GetParent(&self) -> ::windows::core::Result<IWMDMStorage>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_Media_Audio", feature = "Win32_Media_MediaFoundation"))]
impl ::windows::core::RuntimeName for IWMDMStorage4 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_Media_Audio", feature = "Win32_Media_MediaFoundation"))]
impl IWMDMStorage4_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMDMStorage4_Impl, const OFFSET: isize>() -> IWMDMStorage4_Vtbl {
        unsafe extern "system" fn SetReferences<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMDMStorage4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwrefs: u32, ppiwmdmstorage: *const *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetReferences(::core::mem::transmute_copy(&dwrefs), ::core::mem::transmute_copy(&ppiwmdmstorage)).into()
        }
        unsafe extern "system" fn GetReferences<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMDMStorage4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwrefs: *mut u32, pppiwmdmstorage: *mut *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetReferences(::core::mem::transmute_copy(&pdwrefs), ::core::mem::transmute_copy(&pppiwmdmstorage)).into()
        }
        unsafe extern "system" fn GetRightsWithProgress<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMDMStorage4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, piprogresscallback: *mut ::core::ffi::c_void, pprights: *mut *mut WMDMRIGHTS, pnrightscount: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetRightsWithProgress(::core::mem::transmute(&piprogresscallback), ::core::mem::transmute_copy(&pprights), ::core::mem::transmute_copy(&pnrightscount)).into()
        }
        unsafe extern "system" fn GetSpecifiedMetadata<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMDMStorage4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cproperties: u32, ppwszpropnames: *const ::windows::core::PWSTR, ppmetadata: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetSpecifiedMetadata(::core::mem::transmute_copy(&cproperties), ::core::mem::transmute_copy(&ppwszpropnames)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppmetadata, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FindStorage<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMDMStorage4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, findscope: WMDM_FIND_SCOPE, pwszuniqueid: ::windows::core::PCWSTR, ppstorage: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.FindStorage(::core::mem::transmute_copy(&findscope), ::core::mem::transmute(&pwszuniqueid)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppstorage, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetParent<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMDMStorage4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppstorage: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetParent() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppstorage, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: IWMDMStorage3_Vtbl::new::<Identity, Impl, OFFSET>(),
            SetReferences: SetReferences::<Identity, Impl, OFFSET>,
            GetReferences: GetReferences::<Identity, Impl, OFFSET>,
            GetRightsWithProgress: GetRightsWithProgress::<Identity, Impl, OFFSET>,
            GetSpecifiedMetadata: GetSpecifiedMetadata::<Identity, Impl, OFFSET>,
            FindStorage: FindStorage::<Identity, Impl, OFFSET>,
            GetParent: GetParent::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWMDMStorage4 as ::windows::core::Interface>::IID || iid == &<IWMDMStorage as ::windows::core::Interface>::IID || iid == &<IWMDMStorage2 as ::windows::core::Interface>::IID || iid == &<IWMDMStorage3 as ::windows::core::Interface>::IID
    }
}
pub trait IWMDMStorageControl_Impl: Sized {
    fn Insert(&self, fumode: u32, pwszfile: &::windows::core::PCWSTR, poperation: &::core::option::Option<IWMDMOperation>, pprogress: &::core::option::Option<IWMDMProgress>) -> ::windows::core::Result<IWMDMStorage>;
    fn Delete(&self, fumode: u32, pprogress: &::core::option::Option<IWMDMProgress>) -> ::windows::core::Result<()>;
    fn Rename(&self, fumode: u32, pwsznewname: &::windows::core::PCWSTR, pprogress: &::core::option::Option<IWMDMProgress>) -> ::windows::core::Result<()>;
    fn Read(&self, fumode: u32, pwszfile: &::windows::core::PCWSTR, pprogress: &::core::option::Option<IWMDMProgress>, poperation: &::core::option::Option<IWMDMOperation>) -> ::windows::core::Result<()>;
    fn Move(&self, fumode: u32, ptargetobject: &::core::option::Option<IWMDMStorage>, pprogress: &::core::option::Option<IWMDMProgress>) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IWMDMStorageControl {}
impl IWMDMStorageControl_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMDMStorageControl_Impl, const OFFSET: isize>() -> IWMDMStorageControl_Vtbl {
        unsafe extern "system" fn Insert<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMDMStorageControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fumode: u32, pwszfile: ::windows::core::PCWSTR, poperation: *mut ::core::ffi::c_void, pprogress: *mut ::core::ffi::c_void, ppnewobject: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Insert(::core::mem::transmute_copy(&fumode), ::core::mem::transmute(&pwszfile), ::core::mem::transmute(&poperation), ::core::mem::transmute(&pprogress)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppnewobject, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Delete<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMDMStorageControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fumode: u32, pprogress: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Delete(::core::mem::transmute_copy(&fumode), ::core::mem::transmute(&pprogress)).into()
        }
        unsafe extern "system" fn Rename<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMDMStorageControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fumode: u32, pwsznewname: ::windows::core::PCWSTR, pprogress: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Rename(::core::mem::transmute_copy(&fumode), ::core::mem::transmute(&pwsznewname), ::core::mem::transmute(&pprogress)).into()
        }
        unsafe extern "system" fn Read<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMDMStorageControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fumode: u32, pwszfile: ::windows::core::PCWSTR, pprogress: *mut ::core::ffi::c_void, poperation: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Read(::core::mem::transmute_copy(&fumode), ::core::mem::transmute(&pwszfile), ::core::mem::transmute(&pprogress), ::core::mem::transmute(&poperation)).into()
        }
        unsafe extern "system" fn Move<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMDMStorageControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fumode: u32, ptargetobject: *mut ::core::ffi::c_void, pprogress: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Move(::core::mem::transmute_copy(&fumode), ::core::mem::transmute(&ptargetobject), ::core::mem::transmute(&pprogress)).into()
        }
        Self {
            base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            Insert: Insert::<Identity, Impl, OFFSET>,
            Delete: Delete::<Identity, Impl, OFFSET>,
            Rename: Rename::<Identity, Impl, OFFSET>,
            Read: Read::<Identity, Impl, OFFSET>,
            Move: Move::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWMDMStorageControl as ::windows::core::Interface>::IID
    }
}
pub trait IWMDMStorageControl2_Impl: Sized + IWMDMStorageControl_Impl {
    fn Insert2(&self, fumode: u32, pwszfilesource: &::windows::core::PCWSTR, pwszfiledest: &::windows::core::PCWSTR, poperation: &::core::option::Option<IWMDMOperation>, pprogress: &::core::option::Option<IWMDMProgress>, punknown: &::core::option::Option<::windows::core::IUnknown>, ppnewobject: *mut ::core::option::Option<IWMDMStorage>) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IWMDMStorageControl2 {}
impl IWMDMStorageControl2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMDMStorageControl2_Impl, const OFFSET: isize>() -> IWMDMStorageControl2_Vtbl {
        unsafe extern "system" fn Insert2<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMDMStorageControl2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fumode: u32, pwszfilesource: ::windows::core::PCWSTR, pwszfiledest: ::windows::core::PCWSTR, poperation: *mut ::core::ffi::c_void, pprogress: *mut ::core::ffi::c_void, punknown: *mut ::core::ffi::c_void, ppnewobject: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Insert2(::core::mem::transmute_copy(&fumode), ::core::mem::transmute(&pwszfilesource), ::core::mem::transmute(&pwszfiledest), ::core::mem::transmute(&poperation), ::core::mem::transmute(&pprogress), ::core::mem::transmute(&punknown), ::core::mem::transmute_copy(&ppnewobject)).into()
        }
        Self { base__: IWMDMStorageControl_Vtbl::new::<Identity, Impl, OFFSET>(), Insert2: Insert2::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWMDMStorageControl2 as ::windows::core::Interface>::IID || iid == &<IWMDMStorageControl as ::windows::core::Interface>::IID
    }
}
pub trait IWMDMStorageControl3_Impl: Sized + IWMDMStorageControl_Impl + IWMDMStorageControl2_Impl {
    fn Insert3(&self, fumode: u32, futype: u32, pwszfilesource: &::windows::core::PCWSTR, pwszfiledest: &::windows::core::PCWSTR, poperation: &::core::option::Option<IWMDMOperation>, pprogress: &::core::option::Option<IWMDMProgress>, pmetadata: &::core::option::Option<IWMDMMetaData>, punknown: &::core::option::Option<::windows::core::IUnknown>, ppnewobject: *mut ::core::option::Option<IWMDMStorage>) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IWMDMStorageControl3 {}
impl IWMDMStorageControl3_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMDMStorageControl3_Impl, const OFFSET: isize>() -> IWMDMStorageControl3_Vtbl {
        unsafe extern "system" fn Insert3<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMDMStorageControl3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fumode: u32, futype: u32, pwszfilesource: ::windows::core::PCWSTR, pwszfiledest: ::windows::core::PCWSTR, poperation: *mut ::core::ffi::c_void, pprogress: *mut ::core::ffi::c_void, pmetadata: *mut ::core::ffi::c_void, punknown: *mut ::core::ffi::c_void, ppnewobject: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Insert3(::core::mem::transmute_copy(&fumode), ::core::mem::transmute_copy(&futype), ::core::mem::transmute(&pwszfilesource), ::core::mem::transmute(&pwszfiledest), ::core::mem::transmute(&poperation), ::core::mem::transmute(&pprogress), ::core::mem::transmute(&pmetadata), ::core::mem::transmute(&punknown), ::core::mem::transmute_copy(&ppnewobject)).into()
        }
        Self { base__: IWMDMStorageControl2_Vtbl::new::<Identity, Impl, OFFSET>(), Insert3: Insert3::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWMDMStorageControl3 as ::windows::core::Interface>::IID || iid == &<IWMDMStorageControl as ::windows::core::Interface>::IID || iid == &<IWMDMStorageControl2 as ::windows::core::Interface>::IID
    }
}
pub trait IWMDMStorageGlobals_Impl: Sized {
    fn GetCapabilities(&self) -> ::windows::core::Result<u32>;
    fn GetSerialNumber(&self, pserialnum: *mut WMDMID, abmac: *mut u8) -> ::windows::core::Result<()>;
    fn GetTotalSize(&self, pdwtotalsizelow: *mut u32, pdwtotalsizehigh: *mut u32) -> ::windows::core::Result<()>;
    fn GetTotalFree(&self, pdwfreelow: *mut u32, pdwfreehigh: *mut u32) -> ::windows::core::Result<()>;
    fn GetTotalBad(&self, pdwbadlow: *mut u32, pdwbadhigh: *mut u32) -> ::windows::core::Result<()>;
    fn GetStatus(&self) -> ::windows::core::Result<u32>;
    fn Initialize(&self, fumode: u32, pprogress: &::core::option::Option<IWMDMProgress>) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IWMDMStorageGlobals {}
impl IWMDMStorageGlobals_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMDMStorageGlobals_Impl, const OFFSET: isize>() -> IWMDMStorageGlobals_Vtbl {
        unsafe extern "system" fn GetCapabilities<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMDMStorageGlobals_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwcapabilities: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetCapabilities() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pdwcapabilities, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSerialNumber<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMDMStorageGlobals_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pserialnum: *mut WMDMID, abmac: *mut u8) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetSerialNumber(::core::mem::transmute_copy(&pserialnum), ::core::mem::transmute_copy(&abmac)).into()
        }
        unsafe extern "system" fn GetTotalSize<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMDMStorageGlobals_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwtotalsizelow: *mut u32, pdwtotalsizehigh: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetTotalSize(::core::mem::transmute_copy(&pdwtotalsizelow), ::core::mem::transmute_copy(&pdwtotalsizehigh)).into()
        }
        unsafe extern "system" fn GetTotalFree<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMDMStorageGlobals_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwfreelow: *mut u32, pdwfreehigh: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetTotalFree(::core::mem::transmute_copy(&pdwfreelow), ::core::mem::transmute_copy(&pdwfreehigh)).into()
        }
        unsafe extern "system" fn GetTotalBad<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMDMStorageGlobals_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwbadlow: *mut u32, pdwbadhigh: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetTotalBad(::core::mem::transmute_copy(&pdwbadlow), ::core::mem::transmute_copy(&pdwbadhigh)).into()
        }
        unsafe extern "system" fn GetStatus<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMDMStorageGlobals_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwstatus: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetStatus() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pdwstatus, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Initialize<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMDMStorageGlobals_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fumode: u32, pprogress: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Initialize(::core::mem::transmute_copy(&fumode), ::core::mem::transmute(&pprogress)).into()
        }
        Self {
            base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            GetCapabilities: GetCapabilities::<Identity, Impl, OFFSET>,
            GetSerialNumber: GetSerialNumber::<Identity, Impl, OFFSET>,
            GetTotalSize: GetTotalSize::<Identity, Impl, OFFSET>,
            GetTotalFree: GetTotalFree::<Identity, Impl, OFFSET>,
            GetTotalBad: GetTotalBad::<Identity, Impl, OFFSET>,
            GetStatus: GetStatus::<Identity, Impl, OFFSET>,
            Initialize: Initialize::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWMDMStorageGlobals as ::windows::core::Interface>::IID
    }
}
pub trait IWMDeviceManager_Impl: Sized {
    fn GetRevision(&self) -> ::windows::core::Result<u32>;
    fn GetDeviceCount(&self) -> ::windows::core::Result<u32>;
    fn EnumDevices(&self) -> ::windows::core::Result<IWMDMEnumDevice>;
}
impl ::windows::core::RuntimeName for IWMDeviceManager {}
impl IWMDeviceManager_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMDeviceManager_Impl, const OFFSET: isize>() -> IWMDeviceManager_Vtbl {
        unsafe extern "system" fn GetRevision<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMDeviceManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwrevision: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetRevision() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pdwrevision, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDeviceCount<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMDeviceManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwcount: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetDeviceCount() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pdwcount, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnumDevices<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMDeviceManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenumdevice: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.EnumDevices() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppenumdevice, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            GetRevision: GetRevision::<Identity, Impl, OFFSET>,
            GetDeviceCount: GetDeviceCount::<Identity, Impl, OFFSET>,
            EnumDevices: EnumDevices::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWMDeviceManager as ::windows::core::Interface>::IID
    }
}
pub trait IWMDeviceManager2_Impl: Sized + IWMDeviceManager_Impl {
    fn GetDeviceFromCanonicalName(&self, pwszcanonicalname: &::windows::core::PCWSTR) -> ::windows::core::Result<IWMDMDevice>;
    fn EnumDevices2(&self) -> ::windows::core::Result<IWMDMEnumDevice>;
    fn Reinitialize(&self) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IWMDeviceManager2 {}
impl IWMDeviceManager2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMDeviceManager2_Impl, const OFFSET: isize>() -> IWMDeviceManager2_Vtbl {
        unsafe extern "system" fn GetDeviceFromCanonicalName<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMDeviceManager2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwszcanonicalname: ::windows::core::PCWSTR, ppdevice: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetDeviceFromCanonicalName(::core::mem::transmute(&pwszcanonicalname)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppdevice, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnumDevices2<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMDeviceManager2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenumdevice: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.EnumDevices2() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppenumdevice, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Reinitialize<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMDeviceManager2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Reinitialize().into()
        }
        Self {
            base__: IWMDeviceManager_Vtbl::new::<Identity, Impl, OFFSET>(),
            GetDeviceFromCanonicalName: GetDeviceFromCanonicalName::<Identity, Impl, OFFSET>,
            EnumDevices2: EnumDevices2::<Identity, Impl, OFFSET>,
            Reinitialize: Reinitialize::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWMDeviceManager2 as ::windows::core::Interface>::IID || iid == &<IWMDeviceManager as ::windows::core::Interface>::IID
    }
}
pub trait IWMDeviceManager3_Impl: Sized + IWMDeviceManager_Impl + IWMDeviceManager2_Impl {
    fn SetDeviceEnumPreference(&self, dwenumpref: u32) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IWMDeviceManager3 {}
impl IWMDeviceManager3_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMDeviceManager3_Impl, const OFFSET: isize>() -> IWMDeviceManager3_Vtbl {
        unsafe extern "system" fn SetDeviceEnumPreference<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMDeviceManager3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwenumpref: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetDeviceEnumPreference(::core::mem::transmute_copy(&dwenumpref)).into()
        }
        Self { base__: IWMDeviceManager2_Vtbl::new::<Identity, Impl, OFFSET>(), SetDeviceEnumPreference: SetDeviceEnumPreference::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWMDeviceManager3 as ::windows::core::Interface>::IID || iid == &<IWMDeviceManager as ::windows::core::Interface>::IID || iid == &<IWMDeviceManager2 as ::windows::core::Interface>::IID
    }
}
