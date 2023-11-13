#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IComponentAuthenticate(::windows_core::IUnknown);
impl IComponentAuthenticate {
    pub unsafe fn SACAuth(&self, dwprotocolid: u32, dwpass: u32, pbdatain: &[u8], ppbdataout: *mut *mut u8, pdwdataoutlen: *mut u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SACAuth)(::windows_core::Interface::as_raw(self), dwprotocolid, dwpass, ::core::mem::transmute(pbdatain.as_ptr()), pbdatain.len().try_into().unwrap(), ppbdataout, pdwdataoutlen).ok()
    }
    pub unsafe fn SACGetProtocols(&self, ppdwprotocols: *mut *mut u32, pdwprotocolcount: *mut u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SACGetProtocols)(::windows_core::Interface::as_raw(self), ppdwprotocols, pdwprotocolcount).ok()
    }
}
::windows_core::imp::interface_hierarchy!(IComponentAuthenticate, ::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IComponentAuthenticate {
    type Vtable = IComponentAuthenticate_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IComponentAuthenticate {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xa9889c00_6d2b_11d3_8496_00c04f79dbc0);
}
#[repr(C)]
#[doc(hidden)]
pub struct IComponentAuthenticate_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub SACAuth: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwprotocolid: u32, dwpass: u32, pbdatain: *const u8, dwdatainlen: u32, ppbdataout: *mut *mut u8, pdwdataoutlen: *mut u32) -> ::windows_core::HRESULT,
    pub SACGetProtocols: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppdwprotocols: *mut *mut u32, pdwprotocolcount: *mut u32) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IMDSPDevice(::windows_core::IUnknown);
impl IMDSPDevice {
    pub unsafe fn GetName(&self, pwszname: &mut [u16]) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetName)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pwszname.as_ptr()), pwszname.len().try_into().unwrap()).ok()
    }
    pub unsafe fn GetManufacturer(&self, pwszname: &mut [u16]) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetManufacturer)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pwszname.as_ptr()), pwszname.len().try_into().unwrap()).ok()
    }
    pub unsafe fn GetVersion(&self) -> ::windows_core::Result<u32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetVersion)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetType(&self) -> ::windows_core::Result<u32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetType)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetSerialNumber(&self, pserialnumber: *mut WMDMID, abmac: &mut [u8; 8]) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetSerialNumber)(::windows_core::Interface::as_raw(self), pserialnumber, ::core::mem::transmute(abmac.as_ptr())).ok()
    }
    pub unsafe fn GetPowerSource(&self, pdwpowersource: *mut u32, pdwpercentremaining: *mut u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetPowerSource)(::windows_core::Interface::as_raw(self), pdwpowersource, pdwpercentremaining).ok()
    }
    pub unsafe fn GetStatus(&self) -> ::windows_core::Result<u32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetStatus)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetDeviceIcon(&self) -> ::windows_core::Result<u32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetDeviceIcon)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn EnumStorage(&self) -> ::windows_core::Result<IMDSPEnumStorage> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).EnumStorage)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Media_Audio\"`"]
    #[cfg(feature = "Win32_Media_Audio")]
    pub unsafe fn GetFormatSupport(&self, pformatex: *mut *mut super::Audio::WAVEFORMATEX, pnformatcount: *mut u32, pppwszmimetype: *mut *mut ::windows_core::PWSTR, pnmimetypecount: *mut u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetFormatSupport)(::windows_core::Interface::as_raw(self), pformatex, pnformatcount, pppwszmimetype, pnmimetypecount).ok()
    }
    pub unsafe fn SendOpaqueCommand(&self, pcommand: *mut OPAQUECOMMAND) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SendOpaqueCommand)(::windows_core::Interface::as_raw(self), pcommand).ok()
    }
}
::windows_core::imp::interface_hierarchy!(IMDSPDevice, ::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IMDSPDevice {
    type Vtable = IMDSPDevice_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IMDSPDevice {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x1dcb3a12_33ed_11d3_8470_00c04f79dbc0);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMDSPDevice_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub GetName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwszname: ::windows_core::PWSTR, nmaxchars: u32) -> ::windows_core::HRESULT,
    pub GetManufacturer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwszname: ::windows_core::PWSTR, nmaxchars: u32) -> ::windows_core::HRESULT,
    pub GetVersion: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdwversion: *mut u32) -> ::windows_core::HRESULT,
    pub GetType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdwtype: *mut u32) -> ::windows_core::HRESULT,
    pub GetSerialNumber: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pserialnumber: *mut WMDMID, abmac: *mut u8) -> ::windows_core::HRESULT,
    pub GetPowerSource: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdwpowersource: *mut u32, pdwpercentremaining: *mut u32) -> ::windows_core::HRESULT,
    pub GetStatus: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdwstatus: *mut u32) -> ::windows_core::HRESULT,
    pub GetDeviceIcon: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hicon: *mut u32) -> ::windows_core::HRESULT,
    pub EnumStorage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppenumstorage: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_Media_Audio")]
    pub GetFormatSupport: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pformatex: *mut *mut super::Audio::WAVEFORMATEX, pnformatcount: *mut u32, pppwszmimetype: *mut *mut ::windows_core::PWSTR, pnmimetypecount: *mut u32) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Media_Audio"))]
    GetFormatSupport: usize,
    pub SendOpaqueCommand: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcommand: *mut OPAQUECOMMAND) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IMDSPDevice2(::windows_core::IUnknown);
impl IMDSPDevice2 {
    pub unsafe fn GetName(&self, pwszname: &mut [u16]) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.GetName)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pwszname.as_ptr()), pwszname.len().try_into().unwrap()).ok()
    }
    pub unsafe fn GetManufacturer(&self, pwszname: &mut [u16]) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.GetManufacturer)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pwszname.as_ptr()), pwszname.len().try_into().unwrap()).ok()
    }
    pub unsafe fn GetVersion(&self) -> ::windows_core::Result<u32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetVersion)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetType(&self) -> ::windows_core::Result<u32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetType)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetSerialNumber(&self, pserialnumber: *mut WMDMID, abmac: &mut [u8; 8]) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.GetSerialNumber)(::windows_core::Interface::as_raw(self), pserialnumber, ::core::mem::transmute(abmac.as_ptr())).ok()
    }
    pub unsafe fn GetPowerSource(&self, pdwpowersource: *mut u32, pdwpercentremaining: *mut u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.GetPowerSource)(::windows_core::Interface::as_raw(self), pdwpowersource, pdwpercentremaining).ok()
    }
    pub unsafe fn GetStatus(&self) -> ::windows_core::Result<u32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetStatus)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetDeviceIcon(&self) -> ::windows_core::Result<u32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetDeviceIcon)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn EnumStorage(&self) -> ::windows_core::Result<IMDSPEnumStorage> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.EnumStorage)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Media_Audio\"`"]
    #[cfg(feature = "Win32_Media_Audio")]
    pub unsafe fn GetFormatSupport(&self, pformatex: *mut *mut super::Audio::WAVEFORMATEX, pnformatcount: *mut u32, pppwszmimetype: *mut *mut ::windows_core::PWSTR, pnmimetypecount: *mut u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.GetFormatSupport)(::windows_core::Interface::as_raw(self), pformatex, pnformatcount, pppwszmimetype, pnmimetypecount).ok()
    }
    pub unsafe fn SendOpaqueCommand(&self, pcommand: *mut OPAQUECOMMAND) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.SendOpaqueCommand)(::windows_core::Interface::as_raw(self), pcommand).ok()
    }
    pub unsafe fn GetStorage<P0>(&self, pszstoragename: P0) -> ::windows_core::Result<IMDSPStorage>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetStorage)(::windows_core::Interface::as_raw(self), pszstoragename.into_param().abi(), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`, `\"Win32_Media_Audio\"`, `\"Win32_Media_MediaFoundation\"`"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_Media_Audio", feature = "Win32_Media_MediaFoundation"))]
    pub unsafe fn GetFormatSupport2(&self, dwflags: u32, ppaudioformatex: *mut *mut super::Audio::WAVEFORMATEX, pnaudioformatcount: *mut u32, ppvideoformatex: *mut *mut super::MediaFoundation::VIDEOINFOHEADER, pnvideoformatcount: *mut u32, ppfiletype: *mut *mut WMFILECAPABILITIES, pnfiletypecount: *mut u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetFormatSupport2)(::windows_core::Interface::as_raw(self), dwflags, ppaudioformatex, pnaudioformatcount, ppvideoformatex, pnvideoformatcount, ppfiletype, pnfiletypecount).ok()
    }
    #[doc = "Required features: `\"Win32_System_Ole\"`"]
    #[cfg(feature = "Win32_System_Ole")]
    pub unsafe fn GetSpecifyPropertyPages(&self, ppspecifyproppages: *mut ::core::option::Option<super::super::System::Ole::ISpecifyPropertyPages>, pppunknowns: *mut *mut ::core::option::Option<::windows_core::IUnknown>, pcunks: *mut u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetSpecifyPropertyPages)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(ppspecifyproppages), pppunknowns, pcunks).ok()
    }
    pub unsafe fn GetCanonicalName(&self, pwszpnpname: &mut [u16]) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetCanonicalName)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pwszpnpname.as_ptr()), pwszpnpname.len().try_into().unwrap()).ok()
    }
}
::windows_core::imp::interface_hierarchy!(IMDSPDevice2, ::windows_core::IUnknown, IMDSPDevice);
unsafe impl ::windows_core::Interface for IMDSPDevice2 {
    type Vtable = IMDSPDevice2_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IMDSPDevice2 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x420d16ad_c97d_4e00_82aa_00e9f4335ddd);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMDSPDevice2_Vtbl {
    pub base__: IMDSPDevice_Vtbl,
    pub GetStorage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszstoragename: ::windows_core::PCWSTR, ppstorage: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_Media_Audio", feature = "Win32_Media_MediaFoundation"))]
    pub GetFormatSupport2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwflags: u32, ppaudioformatex: *mut *mut super::Audio::WAVEFORMATEX, pnaudioformatcount: *mut u32, ppvideoformatex: *mut *mut super::MediaFoundation::VIDEOINFOHEADER, pnvideoformatcount: *mut u32, ppfiletype: *mut *mut WMFILECAPABILITIES, pnfiletypecount: *mut u32) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_Media_Audio", feature = "Win32_Media_MediaFoundation")))]
    GetFormatSupport2: usize,
    #[cfg(feature = "Win32_System_Ole")]
    pub GetSpecifyPropertyPages: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppspecifyproppages: *mut *mut ::core::ffi::c_void, pppunknowns: *mut *mut ::core::option::Option<::windows_core::IUnknown>, pcunks: *mut u32) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Ole"))]
    GetSpecifyPropertyPages: usize,
    pub GetCanonicalName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwszpnpname: ::windows_core::PWSTR, nmaxchars: u32) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IMDSPDevice3(::windows_core::IUnknown);
impl IMDSPDevice3 {
    pub unsafe fn GetName(&self, pwszname: &mut [u16]) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.GetName)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pwszname.as_ptr()), pwszname.len().try_into().unwrap()).ok()
    }
    pub unsafe fn GetManufacturer(&self, pwszname: &mut [u16]) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.GetManufacturer)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pwszname.as_ptr()), pwszname.len().try_into().unwrap()).ok()
    }
    pub unsafe fn GetVersion(&self) -> ::windows_core::Result<u32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.GetVersion)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetType(&self) -> ::windows_core::Result<u32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.GetType)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetSerialNumber(&self, pserialnumber: *mut WMDMID, abmac: &mut [u8; 8]) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.GetSerialNumber)(::windows_core::Interface::as_raw(self), pserialnumber, ::core::mem::transmute(abmac.as_ptr())).ok()
    }
    pub unsafe fn GetPowerSource(&self, pdwpowersource: *mut u32, pdwpercentremaining: *mut u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.GetPowerSource)(::windows_core::Interface::as_raw(self), pdwpowersource, pdwpercentremaining).ok()
    }
    pub unsafe fn GetStatus(&self) -> ::windows_core::Result<u32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.GetStatus)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetDeviceIcon(&self) -> ::windows_core::Result<u32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.GetDeviceIcon)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn EnumStorage(&self) -> ::windows_core::Result<IMDSPEnumStorage> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.EnumStorage)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Media_Audio\"`"]
    #[cfg(feature = "Win32_Media_Audio")]
    pub unsafe fn GetFormatSupport(&self, pformatex: *mut *mut super::Audio::WAVEFORMATEX, pnformatcount: *mut u32, pppwszmimetype: *mut *mut ::windows_core::PWSTR, pnmimetypecount: *mut u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.GetFormatSupport)(::windows_core::Interface::as_raw(self), pformatex, pnformatcount, pppwszmimetype, pnmimetypecount).ok()
    }
    pub unsafe fn SendOpaqueCommand(&self, pcommand: *mut OPAQUECOMMAND) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.SendOpaqueCommand)(::windows_core::Interface::as_raw(self), pcommand).ok()
    }
    pub unsafe fn GetStorage<P0>(&self, pszstoragename: P0) -> ::windows_core::Result<IMDSPStorage>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetStorage)(::windows_core::Interface::as_raw(self), pszstoragename.into_param().abi(), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`, `\"Win32_Media_Audio\"`, `\"Win32_Media_MediaFoundation\"`"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_Media_Audio", feature = "Win32_Media_MediaFoundation"))]
    pub unsafe fn GetFormatSupport2(&self, dwflags: u32, ppaudioformatex: *mut *mut super::Audio::WAVEFORMATEX, pnaudioformatcount: *mut u32, ppvideoformatex: *mut *mut super::MediaFoundation::VIDEOINFOHEADER, pnvideoformatcount: *mut u32, ppfiletype: *mut *mut WMFILECAPABILITIES, pnfiletypecount: *mut u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.GetFormatSupport2)(::windows_core::Interface::as_raw(self), dwflags, ppaudioformatex, pnaudioformatcount, ppvideoformatex, pnvideoformatcount, ppfiletype, pnfiletypecount).ok()
    }
    #[doc = "Required features: `\"Win32_System_Ole\"`"]
    #[cfg(feature = "Win32_System_Ole")]
    pub unsafe fn GetSpecifyPropertyPages(&self, ppspecifyproppages: *mut ::core::option::Option<super::super::System::Ole::ISpecifyPropertyPages>, pppunknowns: *mut *mut ::core::option::Option<::windows_core::IUnknown>, pcunks: *mut u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.GetSpecifyPropertyPages)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(ppspecifyproppages), pppunknowns, pcunks).ok()
    }
    pub unsafe fn GetCanonicalName(&self, pwszpnpname: &mut [u16]) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.GetCanonicalName)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pwszpnpname.as_ptr()), pwszpnpname.len().try_into().unwrap()).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com_StructuredStorage\"`, `\"Win32_System_Variant\"`"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant"))]
    pub unsafe fn GetProperty<P0>(&self, pwszpropname: P0) -> ::windows_core::Result<super::super::System::Com::StructuredStorage::PROPVARIANT>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetProperty)(::windows_core::Interface::as_raw(self), pwszpropname.into_param().abi(), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com_StructuredStorage\"`, `\"Win32_System_Variant\"`"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant"))]
    pub unsafe fn SetProperty<P0>(&self, pwszpropname: P0, pvalue: *const super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        (::windows_core::Interface::vtable(self).SetProperty)(::windows_core::Interface::as_raw(self), pwszpropname.into_param().abi(), pvalue).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com_StructuredStorage\"`, `\"Win32_System_Variant\"`"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant"))]
    pub unsafe fn GetFormatCapability(&self, format: WMDM_FORMATCODE) -> ::windows_core::Result<WMDM_FORMAT_CAPABILITY> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetFormatCapability)(::windows_core::Interface::as_raw(self), format, &mut result__).from_abi(result__)
    }
    pub unsafe fn DeviceIoControl(&self, dwiocontrolcode: u32, lpinbuffer: &[u8], lpoutbuffer: *mut u8, pnoutbuffersize: *mut u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).DeviceIoControl)(::windows_core::Interface::as_raw(self), dwiocontrolcode, ::core::mem::transmute(lpinbuffer.as_ptr()), lpinbuffer.len().try_into().unwrap(), lpoutbuffer, pnoutbuffersize).ok()
    }
    pub unsafe fn FindStorage<P0>(&self, findscope: WMDM_FIND_SCOPE, pwszuniqueid: P0) -> ::windows_core::Result<IMDSPStorage>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).FindStorage)(::windows_core::Interface::as_raw(self), findscope, pwszuniqueid.into_param().abi(), &mut result__).from_abi(result__)
    }
}
::windows_core::imp::interface_hierarchy!(IMDSPDevice3, ::windows_core::IUnknown, IMDSPDevice, IMDSPDevice2);
unsafe impl ::windows_core::Interface for IMDSPDevice3 {
    type Vtable = IMDSPDevice3_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IMDSPDevice3 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x1a839845_fc55_487c_976f_ee38ac0e8c4e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMDSPDevice3_Vtbl {
    pub base__: IMDSPDevice2_Vtbl,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant"))]
    pub GetProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwszpropname: ::windows_core::PCWSTR, pvalue: *mut super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant")))]
    GetProperty: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant"))]
    pub SetProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwszpropname: ::windows_core::PCWSTR, pvalue: *const super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant")))]
    SetProperty: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant"))]
    pub GetFormatCapability: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, format: WMDM_FORMATCODE, pformatsupport: *mut WMDM_FORMAT_CAPABILITY) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant")))]
    GetFormatCapability: usize,
    pub DeviceIoControl: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwiocontrolcode: u32, lpinbuffer: *const u8, ninbuffersize: u32, lpoutbuffer: *mut u8, pnoutbuffersize: *mut u32) -> ::windows_core::HRESULT,
    pub FindStorage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, findscope: WMDM_FIND_SCOPE, pwszuniqueid: ::windows_core::PCWSTR, ppstorage: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IMDSPDeviceControl(::windows_core::IUnknown);
impl IMDSPDeviceControl {
    pub unsafe fn GetDCStatus(&self) -> ::windows_core::Result<u32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetDCStatus)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetCapabilities(&self) -> ::windows_core::Result<u32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetCapabilities)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn Play(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Play)(::windows_core::Interface::as_raw(self)).ok()
    }
    #[doc = "Required features: `\"Win32_Media_Audio\"`"]
    #[cfg(feature = "Win32_Media_Audio")]
    pub unsafe fn Record(&self, pformat: *const super::Audio::WAVEFORMATEX) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Record)(::windows_core::Interface::as_raw(self), pformat).ok()
    }
    pub unsafe fn Pause(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Pause)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Resume(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Resume)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Stop(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Stop)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Seek(&self, fumode: u32, noffset: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Seek)(::windows_core::Interface::as_raw(self), fumode, noffset).ok()
    }
}
::windows_core::imp::interface_hierarchy!(IMDSPDeviceControl, ::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IMDSPDeviceControl {
    type Vtable = IMDSPDeviceControl_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IMDSPDeviceControl {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x1dcb3a14_33ed_11d3_8470_00c04f79dbc0);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMDSPDeviceControl_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub GetDCStatus: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdwstatus: *mut u32) -> ::windows_core::HRESULT,
    pub GetCapabilities: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdwcapabilitiesmask: *mut u32) -> ::windows_core::HRESULT,
    pub Play: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_Media_Audio")]
    pub Record: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pformat: *const super::Audio::WAVEFORMATEX) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Media_Audio"))]
    Record: usize,
    pub Pause: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub Resume: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub Stop: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub Seek: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, fumode: u32, noffset: i32) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IMDSPDirectTransfer(::windows_core::IUnknown);
impl IMDSPDirectTransfer {
    pub unsafe fn TransferToDevice<P0, P1, P2, P3, P4>(&self, pwszsourcefilepath: P0, psourceoperation: P1, fuflags: u32, pwszdestinationname: P2, psourcemetadata: P3, ptransferprogress: P4) -> ::windows_core::Result<IMDSPStorage>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
        P1: ::windows_core::IntoParam<IWMDMOperation>,
        P2: ::windows_core::IntoParam<::windows_core::PCWSTR>,
        P3: ::windows_core::IntoParam<IWMDMMetaData>,
        P4: ::windows_core::IntoParam<IWMDMProgress>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).TransferToDevice)(::windows_core::Interface::as_raw(self), pwszsourcefilepath.into_param().abi(), psourceoperation.into_param().abi(), fuflags, pwszdestinationname.into_param().abi(), psourcemetadata.into_param().abi(), ptransferprogress.into_param().abi(), &mut result__).from_abi(result__)
    }
}
::windows_core::imp::interface_hierarchy!(IMDSPDirectTransfer, ::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IMDSPDirectTransfer {
    type Vtable = IMDSPDirectTransfer_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IMDSPDirectTransfer {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xc2fe57a8_9304_478c_9ee4_47e397b912d7);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMDSPDirectTransfer_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub TransferToDevice: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwszsourcefilepath: ::windows_core::PCWSTR, psourceoperation: *mut ::core::ffi::c_void, fuflags: u32, pwszdestinationname: ::windows_core::PCWSTR, psourcemetadata: *mut ::core::ffi::c_void, ptransferprogress: *mut ::core::ffi::c_void, ppnewobject: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IMDSPEnumDevice(::windows_core::IUnknown);
impl IMDSPEnumDevice {
    pub unsafe fn Next(&self, ppdevice: &mut [::core::option::Option<IMDSPDevice>], pceltfetched: *mut u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Next)(::windows_core::Interface::as_raw(self), ppdevice.len().try_into().unwrap(), ::core::mem::transmute(ppdevice.as_ptr()), pceltfetched).ok()
    }
    pub unsafe fn Skip(&self, celt: u32) -> ::windows_core::Result<u32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).Skip)(::windows_core::Interface::as_raw(self), celt, &mut result__).from_abi(result__)
    }
    pub unsafe fn Reset(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Reset)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Clone(&self) -> ::windows_core::Result<IMDSPEnumDevice> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).Clone)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
::windows_core::imp::interface_hierarchy!(IMDSPEnumDevice, ::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IMDSPEnumDevice {
    type Vtable = IMDSPEnumDevice_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IMDSPEnumDevice {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x1dcb3a11_33ed_11d3_8470_00c04f79dbc0);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMDSPEnumDevice_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub Next: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, celt: u32, ppdevice: *mut *mut ::core::ffi::c_void, pceltfetched: *mut u32) -> ::windows_core::HRESULT,
    pub Skip: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, celt: u32, pceltfetched: *mut u32) -> ::windows_core::HRESULT,
    pub Reset: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub Clone: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppenumdevice: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IMDSPEnumStorage(::windows_core::IUnknown);
impl IMDSPEnumStorage {
    pub unsafe fn Next(&self, ppstorage: &mut [::core::option::Option<IMDSPStorage>], pceltfetched: *mut u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Next)(::windows_core::Interface::as_raw(self), ppstorage.len().try_into().unwrap(), ::core::mem::transmute(ppstorage.as_ptr()), pceltfetched).ok()
    }
    pub unsafe fn Skip(&self, celt: u32) -> ::windows_core::Result<u32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).Skip)(::windows_core::Interface::as_raw(self), celt, &mut result__).from_abi(result__)
    }
    pub unsafe fn Reset(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Reset)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Clone(&self) -> ::windows_core::Result<IMDSPEnumStorage> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).Clone)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
::windows_core::imp::interface_hierarchy!(IMDSPEnumStorage, ::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IMDSPEnumStorage {
    type Vtable = IMDSPEnumStorage_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IMDSPEnumStorage {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x1dcb3a15_33ed_11d3_8470_00c04f79dbc0);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMDSPEnumStorage_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub Next: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, celt: u32, ppstorage: *mut *mut ::core::ffi::c_void, pceltfetched: *mut u32) -> ::windows_core::HRESULT,
    pub Skip: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, celt: u32, pceltfetched: *mut u32) -> ::windows_core::HRESULT,
    pub Reset: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub Clone: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppenumstorage: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IMDSPObject(::windows_core::IUnknown);
impl IMDSPObject {
    pub unsafe fn Open(&self, fumode: u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Open)(::windows_core::Interface::as_raw(self), fumode).ok()
    }
    pub unsafe fn Read(&self, pdata: *mut u8, pdwsize: *mut u32, abmac: &mut [u8; 8]) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Read)(::windows_core::Interface::as_raw(self), pdata, pdwsize, ::core::mem::transmute(abmac.as_ptr())).ok()
    }
    pub unsafe fn Write(&self, pdata: *const u8, pdwsize: *mut u32, abmac: &mut [u8; 8]) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Write)(::windows_core::Interface::as_raw(self), pdata, pdwsize, ::core::mem::transmute(abmac.as_ptr())).ok()
    }
    pub unsafe fn Delete<P0>(&self, fumode: u32, pprogress: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IWMDMProgress>,
    {
        (::windows_core::Interface::vtable(self).Delete)(::windows_core::Interface::as_raw(self), fumode, pprogress.into_param().abi()).ok()
    }
    pub unsafe fn Seek(&self, fuflags: u32, dwoffset: u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Seek)(::windows_core::Interface::as_raw(self), fuflags, dwoffset).ok()
    }
    pub unsafe fn Rename<P0, P1>(&self, pwsznewname: P0, pprogress: P1) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
        P1: ::windows_core::IntoParam<IWMDMProgress>,
    {
        (::windows_core::Interface::vtable(self).Rename)(::windows_core::Interface::as_raw(self), pwsznewname.into_param().abi(), pprogress.into_param().abi()).ok()
    }
    pub unsafe fn Move<P0, P1>(&self, fumode: u32, pprogress: P0, ptarget: P1) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IWMDMProgress>,
        P1: ::windows_core::IntoParam<IMDSPStorage>,
    {
        (::windows_core::Interface::vtable(self).Move)(::windows_core::Interface::as_raw(self), fumode, pprogress.into_param().abi(), ptarget.into_param().abi()).ok()
    }
    pub unsafe fn Close(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Close)(::windows_core::Interface::as_raw(self)).ok()
    }
}
::windows_core::imp::interface_hierarchy!(IMDSPObject, ::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IMDSPObject {
    type Vtable = IMDSPObject_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IMDSPObject {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x1dcb3a18_33ed_11d3_8470_00c04f79dbc0);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMDSPObject_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub Open: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, fumode: u32) -> ::windows_core::HRESULT,
    pub Read: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdata: *mut u8, pdwsize: *mut u32, abmac: *mut u8) -> ::windows_core::HRESULT,
    pub Write: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdata: *const u8, pdwsize: *mut u32, abmac: *mut u8) -> ::windows_core::HRESULT,
    pub Delete: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, fumode: u32, pprogress: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub Seek: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, fuflags: u32, dwoffset: u32) -> ::windows_core::HRESULT,
    pub Rename: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwsznewname: ::windows_core::PCWSTR, pprogress: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub Move: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, fumode: u32, pprogress: *mut ::core::ffi::c_void, ptarget: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub Close: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IMDSPObject2(::windows_core::IUnknown);
impl IMDSPObject2 {
    pub unsafe fn Open(&self, fumode: u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.Open)(::windows_core::Interface::as_raw(self), fumode).ok()
    }
    pub unsafe fn Read(&self, pdata: *mut u8, pdwsize: *mut u32, abmac: &mut [u8; 8]) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.Read)(::windows_core::Interface::as_raw(self), pdata, pdwsize, ::core::mem::transmute(abmac.as_ptr())).ok()
    }
    pub unsafe fn Write(&self, pdata: *const u8, pdwsize: *mut u32, abmac: &mut [u8; 8]) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.Write)(::windows_core::Interface::as_raw(self), pdata, pdwsize, ::core::mem::transmute(abmac.as_ptr())).ok()
    }
    pub unsafe fn Delete<P0>(&self, fumode: u32, pprogress: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IWMDMProgress>,
    {
        (::windows_core::Interface::vtable(self).base__.Delete)(::windows_core::Interface::as_raw(self), fumode, pprogress.into_param().abi()).ok()
    }
    pub unsafe fn Seek(&self, fuflags: u32, dwoffset: u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.Seek)(::windows_core::Interface::as_raw(self), fuflags, dwoffset).ok()
    }
    pub unsafe fn Rename<P0, P1>(&self, pwsznewname: P0, pprogress: P1) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
        P1: ::windows_core::IntoParam<IWMDMProgress>,
    {
        (::windows_core::Interface::vtable(self).base__.Rename)(::windows_core::Interface::as_raw(self), pwsznewname.into_param().abi(), pprogress.into_param().abi()).ok()
    }
    pub unsafe fn Move<P0, P1>(&self, fumode: u32, pprogress: P0, ptarget: P1) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IWMDMProgress>,
        P1: ::windows_core::IntoParam<IMDSPStorage>,
    {
        (::windows_core::Interface::vtable(self).base__.Move)(::windows_core::Interface::as_raw(self), fumode, pprogress.into_param().abi(), ptarget.into_param().abi()).ok()
    }
    pub unsafe fn Close(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.Close)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn ReadOnClearChannel(&self, pdata: *mut u8, pdwsize: *mut u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).ReadOnClearChannel)(::windows_core::Interface::as_raw(self), pdata, pdwsize).ok()
    }
    pub unsafe fn WriteOnClearChannel(&self, pdata: *const u8, pdwsize: *mut u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).WriteOnClearChannel)(::windows_core::Interface::as_raw(self), pdata, pdwsize).ok()
    }
}
::windows_core::imp::interface_hierarchy!(IMDSPObject2, ::windows_core::IUnknown, IMDSPObject);
unsafe impl ::windows_core::Interface for IMDSPObject2 {
    type Vtable = IMDSPObject2_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IMDSPObject2 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x3f34cd3e_5907_4341_9af9_97f4187c3aa5);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMDSPObject2_Vtbl {
    pub base__: IMDSPObject_Vtbl,
    pub ReadOnClearChannel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdata: *mut u8, pdwsize: *mut u32) -> ::windows_core::HRESULT,
    pub WriteOnClearChannel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdata: *const u8, pdwsize: *mut u32) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IMDSPObjectInfo(::windows_core::IUnknown);
impl IMDSPObjectInfo {
    pub unsafe fn GetPlayLength(&self) -> ::windows_core::Result<u32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetPlayLength)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetPlayLength(&self, dwlength: u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetPlayLength)(::windows_core::Interface::as_raw(self), dwlength).ok()
    }
    pub unsafe fn GetPlayOffset(&self) -> ::windows_core::Result<u32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetPlayOffset)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetPlayOffset(&self, dwoffset: u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetPlayOffset)(::windows_core::Interface::as_raw(self), dwoffset).ok()
    }
    pub unsafe fn GetTotalLength(&self) -> ::windows_core::Result<u32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetTotalLength)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetLastPlayPosition(&self) -> ::windows_core::Result<u32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetLastPlayPosition)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetLongestPlayPosition(&self) -> ::windows_core::Result<u32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetLongestPlayPosition)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
::windows_core::imp::interface_hierarchy!(IMDSPObjectInfo, ::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IMDSPObjectInfo {
    type Vtable = IMDSPObjectInfo_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IMDSPObjectInfo {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x1dcb3a19_33ed_11d3_8470_00c04f79dbc0);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMDSPObjectInfo_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub GetPlayLength: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdwlength: *mut u32) -> ::windows_core::HRESULT,
    pub SetPlayLength: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwlength: u32) -> ::windows_core::HRESULT,
    pub GetPlayOffset: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdwoffset: *mut u32) -> ::windows_core::HRESULT,
    pub SetPlayOffset: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwoffset: u32) -> ::windows_core::HRESULT,
    pub GetTotalLength: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdwlength: *mut u32) -> ::windows_core::HRESULT,
    pub GetLastPlayPosition: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdwlastpos: *mut u32) -> ::windows_core::HRESULT,
    pub GetLongestPlayPosition: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdwlongestpos: *mut u32) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IMDSPRevoked(::windows_core::IUnknown);
impl IMDSPRevoked {
    pub unsafe fn GetRevocationURL(&self, ppwszrevocationurl: *mut ::windows_core::PWSTR, pdwbufferlen: *mut u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetRevocationURL)(::windows_core::Interface::as_raw(self), ppwszrevocationurl, pdwbufferlen).ok()
    }
}
::windows_core::imp::interface_hierarchy!(IMDSPRevoked, ::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IMDSPRevoked {
    type Vtable = IMDSPRevoked_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IMDSPRevoked {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xa4e8f2d4_3f31_464d_b53d_4fc335998184);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMDSPRevoked_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub GetRevocationURL: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppwszrevocationurl: *mut ::windows_core::PWSTR, pdwbufferlen: *mut u32) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IMDSPStorage(::windows_core::IUnknown);
impl IMDSPStorage {
    #[doc = "Required features: `\"Win32_Media_Audio\"`"]
    #[cfg(feature = "Win32_Media_Audio")]
    pub unsafe fn SetAttributes(&self, dwattributes: u32, pformat: ::core::option::Option<*const super::Audio::WAVEFORMATEX>) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetAttributes)(::windows_core::Interface::as_raw(self), dwattributes, ::core::mem::transmute(pformat.unwrap_or(::std::ptr::null()))).ok()
    }
    pub unsafe fn GetStorageGlobals(&self) -> ::windows_core::Result<IMDSPStorageGlobals> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetStorageGlobals)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Media_Audio\"`"]
    #[cfg(feature = "Win32_Media_Audio")]
    pub unsafe fn GetAttributes(&self, pdwattributes: *mut u32, pformat: ::core::option::Option<*mut super::Audio::WAVEFORMATEX>) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetAttributes)(::windows_core::Interface::as_raw(self), pdwattributes, ::core::mem::transmute(pformat.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    pub unsafe fn GetName(&self, pwszname: &mut [u16]) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetName)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pwszname.as_ptr()), pwszname.len().try_into().unwrap()).ok()
    }
    pub unsafe fn GetDate(&self) -> ::windows_core::Result<WMDMDATETIME> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetDate)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetSize(&self, pdwsizelow: *mut u32, pdwsizehigh: *mut u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetSize)(::windows_core::Interface::as_raw(self), pdwsizelow, pdwsizehigh).ok()
    }
    pub unsafe fn GetRights(&self, pprights: *mut *mut WMDMRIGHTS, pnrightscount: *mut u32, abmac: &mut [u8; 8]) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetRights)(::windows_core::Interface::as_raw(self), pprights, pnrightscount, ::core::mem::transmute(abmac.as_ptr())).ok()
    }
    #[doc = "Required features: `\"Win32_Media_Audio\"`"]
    #[cfg(feature = "Win32_Media_Audio")]
    pub unsafe fn CreateStorage<P0>(&self, dwattributes: u32, pformat: ::core::option::Option<*const super::Audio::WAVEFORMATEX>, pwszname: P0) -> ::windows_core::Result<IMDSPStorage>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).CreateStorage)(::windows_core::Interface::as_raw(self), dwattributes, ::core::mem::transmute(pformat.unwrap_or(::std::ptr::null())), pwszname.into_param().abi(), &mut result__).from_abi(result__)
    }
    pub unsafe fn EnumStorage(&self) -> ::windows_core::Result<IMDSPEnumStorage> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).EnumStorage)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SendOpaqueCommand(&self, pcommand: *mut OPAQUECOMMAND) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SendOpaqueCommand)(::windows_core::Interface::as_raw(self), pcommand).ok()
    }
}
::windows_core::imp::interface_hierarchy!(IMDSPStorage, ::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IMDSPStorage {
    type Vtable = IMDSPStorage_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IMDSPStorage {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x1dcb3a16_33ed_11d3_8470_00c04f79dbc0);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMDSPStorage_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_Media_Audio")]
    pub SetAttributes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwattributes: u32, pformat: *const super::Audio::WAVEFORMATEX) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Media_Audio"))]
    SetAttributes: usize,
    pub GetStorageGlobals: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppstorageglobals: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_Media_Audio")]
    pub GetAttributes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdwattributes: *mut u32, pformat: *mut super::Audio::WAVEFORMATEX) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Media_Audio"))]
    GetAttributes: usize,
    pub GetName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwszname: ::windows_core::PWSTR, nmaxchars: u32) -> ::windows_core::HRESULT,
    pub GetDate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdatetimeutc: *mut WMDMDATETIME) -> ::windows_core::HRESULT,
    pub GetSize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdwsizelow: *mut u32, pdwsizehigh: *mut u32) -> ::windows_core::HRESULT,
    pub GetRights: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pprights: *mut *mut WMDMRIGHTS, pnrightscount: *mut u32, abmac: *mut u8) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_Media_Audio")]
    pub CreateStorage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwattributes: u32, pformat: *const super::Audio::WAVEFORMATEX, pwszname: ::windows_core::PCWSTR, ppnewstorage: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Media_Audio"))]
    CreateStorage: usize,
    pub EnumStorage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppenumstorage: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub SendOpaqueCommand: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcommand: *mut OPAQUECOMMAND) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IMDSPStorage2(::windows_core::IUnknown);
impl IMDSPStorage2 {
    #[doc = "Required features: `\"Win32_Media_Audio\"`"]
    #[cfg(feature = "Win32_Media_Audio")]
    pub unsafe fn SetAttributes(&self, dwattributes: u32, pformat: ::core::option::Option<*const super::Audio::WAVEFORMATEX>) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.SetAttributes)(::windows_core::Interface::as_raw(self), dwattributes, ::core::mem::transmute(pformat.unwrap_or(::std::ptr::null()))).ok()
    }
    pub unsafe fn GetStorageGlobals(&self) -> ::windows_core::Result<IMDSPStorageGlobals> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetStorageGlobals)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Media_Audio\"`"]
    #[cfg(feature = "Win32_Media_Audio")]
    pub unsafe fn GetAttributes(&self, pdwattributes: *mut u32, pformat: ::core::option::Option<*mut super::Audio::WAVEFORMATEX>) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.GetAttributes)(::windows_core::Interface::as_raw(self), pdwattributes, ::core::mem::transmute(pformat.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    pub unsafe fn GetName(&self, pwszname: &mut [u16]) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.GetName)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pwszname.as_ptr()), pwszname.len().try_into().unwrap()).ok()
    }
    pub unsafe fn GetDate(&self) -> ::windows_core::Result<WMDMDATETIME> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetDate)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetSize(&self, pdwsizelow: *mut u32, pdwsizehigh: *mut u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.GetSize)(::windows_core::Interface::as_raw(self), pdwsizelow, pdwsizehigh).ok()
    }
    pub unsafe fn GetRights(&self, pprights: *mut *mut WMDMRIGHTS, pnrightscount: *mut u32, abmac: &mut [u8; 8]) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.GetRights)(::windows_core::Interface::as_raw(self), pprights, pnrightscount, ::core::mem::transmute(abmac.as_ptr())).ok()
    }
    #[doc = "Required features: `\"Win32_Media_Audio\"`"]
    #[cfg(feature = "Win32_Media_Audio")]
    pub unsafe fn CreateStorage<P0>(&self, dwattributes: u32, pformat: ::core::option::Option<*const super::Audio::WAVEFORMATEX>, pwszname: P0) -> ::windows_core::Result<IMDSPStorage>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.CreateStorage)(::windows_core::Interface::as_raw(self), dwattributes, ::core::mem::transmute(pformat.unwrap_or(::std::ptr::null())), pwszname.into_param().abi(), &mut result__).from_abi(result__)
    }
    pub unsafe fn EnumStorage(&self) -> ::windows_core::Result<IMDSPEnumStorage> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.EnumStorage)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SendOpaqueCommand(&self, pcommand: *mut OPAQUECOMMAND) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.SendOpaqueCommand)(::windows_core::Interface::as_raw(self), pcommand).ok()
    }
    pub unsafe fn GetStorage<P0>(&self, pszstoragename: P0) -> ::windows_core::Result<IMDSPStorage>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetStorage)(::windows_core::Interface::as_raw(self), pszstoragename.into_param().abi(), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`, `\"Win32_Media_Audio\"`, `\"Win32_Media_MediaFoundation\"`"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_Media_Audio", feature = "Win32_Media_MediaFoundation"))]
    pub unsafe fn CreateStorage2<P0>(&self, dwattributes: u32, dwattributesex: u32, paudioformat: ::core::option::Option<*const super::Audio::WAVEFORMATEX>, pvideoformat: ::core::option::Option<*const super::MediaFoundation::VIDEOINFOHEADER>, pwszname: P0, qwfilesize: u64) -> ::windows_core::Result<IMDSPStorage>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).CreateStorage2)(::windows_core::Interface::as_raw(self), dwattributes, dwattributesex, ::core::mem::transmute(paudioformat.unwrap_or(::std::ptr::null())), ::core::mem::transmute(pvideoformat.unwrap_or(::std::ptr::null())), pwszname.into_param().abi(), qwfilesize, &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`, `\"Win32_Media_Audio\"`, `\"Win32_Media_MediaFoundation\"`"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_Media_Audio", feature = "Win32_Media_MediaFoundation"))]
    pub unsafe fn SetAttributes2(&self, dwattributes: u32, dwattributesex: u32, paudioformat: ::core::option::Option<*const super::Audio::WAVEFORMATEX>, pvideoformat: ::core::option::Option<*const super::MediaFoundation::VIDEOINFOHEADER>) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetAttributes2)(::windows_core::Interface::as_raw(self), dwattributes, dwattributesex, ::core::mem::transmute(paudioformat.unwrap_or(::std::ptr::null())), ::core::mem::transmute(pvideoformat.unwrap_or(::std::ptr::null()))).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`, `\"Win32_Media_Audio\"`, `\"Win32_Media_MediaFoundation\"`"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_Media_Audio", feature = "Win32_Media_MediaFoundation"))]
    pub unsafe fn GetAttributes2(&self, pdwattributes: *mut u32, pdwattributesex: *mut u32, paudioformat: ::core::option::Option<*mut super::Audio::WAVEFORMATEX>, pvideoformat: ::core::option::Option<*mut super::MediaFoundation::VIDEOINFOHEADER>) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetAttributes2)(::windows_core::Interface::as_raw(self), pdwattributes, pdwattributesex, ::core::mem::transmute(paudioformat.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(pvideoformat.unwrap_or(::std::ptr::null_mut()))).ok()
    }
}
::windows_core::imp::interface_hierarchy!(IMDSPStorage2, ::windows_core::IUnknown, IMDSPStorage);
unsafe impl ::windows_core::Interface for IMDSPStorage2 {
    type Vtable = IMDSPStorage2_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IMDSPStorage2 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x0a5e07a5_6454_4451_9c36_1c6ae7e2b1d6);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMDSPStorage2_Vtbl {
    pub base__: IMDSPStorage_Vtbl,
    pub GetStorage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszstoragename: ::windows_core::PCWSTR, ppstorage: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_Media_Audio", feature = "Win32_Media_MediaFoundation"))]
    pub CreateStorage2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwattributes: u32, dwattributesex: u32, paudioformat: *const super::Audio::WAVEFORMATEX, pvideoformat: *const super::MediaFoundation::VIDEOINFOHEADER, pwszname: ::windows_core::PCWSTR, qwfilesize: u64, ppnewstorage: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_Media_Audio", feature = "Win32_Media_MediaFoundation")))]
    CreateStorage2: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_Media_Audio", feature = "Win32_Media_MediaFoundation"))]
    pub SetAttributes2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwattributes: u32, dwattributesex: u32, paudioformat: *const super::Audio::WAVEFORMATEX, pvideoformat: *const super::MediaFoundation::VIDEOINFOHEADER) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_Media_Audio", feature = "Win32_Media_MediaFoundation")))]
    SetAttributes2: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_Media_Audio", feature = "Win32_Media_MediaFoundation"))]
    pub GetAttributes2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdwattributes: *mut u32, pdwattributesex: *mut u32, paudioformat: *mut super::Audio::WAVEFORMATEX, pvideoformat: *mut super::MediaFoundation::VIDEOINFOHEADER) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_Media_Audio", feature = "Win32_Media_MediaFoundation")))]
    GetAttributes2: usize,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IMDSPStorage3(::windows_core::IUnknown);
impl IMDSPStorage3 {
    #[doc = "Required features: `\"Win32_Media_Audio\"`"]
    #[cfg(feature = "Win32_Media_Audio")]
    pub unsafe fn SetAttributes(&self, dwattributes: u32, pformat: ::core::option::Option<*const super::Audio::WAVEFORMATEX>) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.SetAttributes)(::windows_core::Interface::as_raw(self), dwattributes, ::core::mem::transmute(pformat.unwrap_or(::std::ptr::null()))).ok()
    }
    pub unsafe fn GetStorageGlobals(&self) -> ::windows_core::Result<IMDSPStorageGlobals> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.GetStorageGlobals)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Media_Audio\"`"]
    #[cfg(feature = "Win32_Media_Audio")]
    pub unsafe fn GetAttributes(&self, pdwattributes: *mut u32, pformat: ::core::option::Option<*mut super::Audio::WAVEFORMATEX>) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.GetAttributes)(::windows_core::Interface::as_raw(self), pdwattributes, ::core::mem::transmute(pformat.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    pub unsafe fn GetName(&self, pwszname: &mut [u16]) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.GetName)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pwszname.as_ptr()), pwszname.len().try_into().unwrap()).ok()
    }
    pub unsafe fn GetDate(&self) -> ::windows_core::Result<WMDMDATETIME> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.GetDate)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetSize(&self, pdwsizelow: *mut u32, pdwsizehigh: *mut u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.GetSize)(::windows_core::Interface::as_raw(self), pdwsizelow, pdwsizehigh).ok()
    }
    pub unsafe fn GetRights(&self, pprights: *mut *mut WMDMRIGHTS, pnrightscount: *mut u32, abmac: &mut [u8; 8]) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.GetRights)(::windows_core::Interface::as_raw(self), pprights, pnrightscount, ::core::mem::transmute(abmac.as_ptr())).ok()
    }
    #[doc = "Required features: `\"Win32_Media_Audio\"`"]
    #[cfg(feature = "Win32_Media_Audio")]
    pub unsafe fn CreateStorage<P0>(&self, dwattributes: u32, pformat: ::core::option::Option<*const super::Audio::WAVEFORMATEX>, pwszname: P0) -> ::windows_core::Result<IMDSPStorage>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.CreateStorage)(::windows_core::Interface::as_raw(self), dwattributes, ::core::mem::transmute(pformat.unwrap_or(::std::ptr::null())), pwszname.into_param().abi(), &mut result__).from_abi(result__)
    }
    pub unsafe fn EnumStorage(&self) -> ::windows_core::Result<IMDSPEnumStorage> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.EnumStorage)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SendOpaqueCommand(&self, pcommand: *mut OPAQUECOMMAND) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.SendOpaqueCommand)(::windows_core::Interface::as_raw(self), pcommand).ok()
    }
    pub unsafe fn GetStorage<P0>(&self, pszstoragename: P0) -> ::windows_core::Result<IMDSPStorage>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetStorage)(::windows_core::Interface::as_raw(self), pszstoragename.into_param().abi(), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`, `\"Win32_Media_Audio\"`, `\"Win32_Media_MediaFoundation\"`"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_Media_Audio", feature = "Win32_Media_MediaFoundation"))]
    pub unsafe fn CreateStorage2<P0>(&self, dwattributes: u32, dwattributesex: u32, paudioformat: ::core::option::Option<*const super::Audio::WAVEFORMATEX>, pvideoformat: ::core::option::Option<*const super::MediaFoundation::VIDEOINFOHEADER>, pwszname: P0, qwfilesize: u64) -> ::windows_core::Result<IMDSPStorage>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.CreateStorage2)(::windows_core::Interface::as_raw(self), dwattributes, dwattributesex, ::core::mem::transmute(paudioformat.unwrap_or(::std::ptr::null())), ::core::mem::transmute(pvideoformat.unwrap_or(::std::ptr::null())), pwszname.into_param().abi(), qwfilesize, &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`, `\"Win32_Media_Audio\"`, `\"Win32_Media_MediaFoundation\"`"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_Media_Audio", feature = "Win32_Media_MediaFoundation"))]
    pub unsafe fn SetAttributes2(&self, dwattributes: u32, dwattributesex: u32, paudioformat: ::core::option::Option<*const super::Audio::WAVEFORMATEX>, pvideoformat: ::core::option::Option<*const super::MediaFoundation::VIDEOINFOHEADER>) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.SetAttributes2)(::windows_core::Interface::as_raw(self), dwattributes, dwattributesex, ::core::mem::transmute(paudioformat.unwrap_or(::std::ptr::null())), ::core::mem::transmute(pvideoformat.unwrap_or(::std::ptr::null()))).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`, `\"Win32_Media_Audio\"`, `\"Win32_Media_MediaFoundation\"`"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_Media_Audio", feature = "Win32_Media_MediaFoundation"))]
    pub unsafe fn GetAttributes2(&self, pdwattributes: *mut u32, pdwattributesex: *mut u32, paudioformat: ::core::option::Option<*mut super::Audio::WAVEFORMATEX>, pvideoformat: ::core::option::Option<*mut super::MediaFoundation::VIDEOINFOHEADER>) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.GetAttributes2)(::windows_core::Interface::as_raw(self), pdwattributes, pdwattributesex, ::core::mem::transmute(paudioformat.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(pvideoformat.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    pub unsafe fn GetMetadata<P0>(&self, pmetadata: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IWMDMMetaData>,
    {
        (::windows_core::Interface::vtable(self).GetMetadata)(::windows_core::Interface::as_raw(self), pmetadata.into_param().abi()).ok()
    }
    pub unsafe fn SetMetadata<P0>(&self, pmetadata: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IWMDMMetaData>,
    {
        (::windows_core::Interface::vtable(self).SetMetadata)(::windows_core::Interface::as_raw(self), pmetadata.into_param().abi()).ok()
    }
}
::windows_core::imp::interface_hierarchy!(IMDSPStorage3, ::windows_core::IUnknown, IMDSPStorage, IMDSPStorage2);
unsafe impl ::windows_core::Interface for IMDSPStorage3 {
    type Vtable = IMDSPStorage3_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IMDSPStorage3 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x6c669867_97ed_4a67_9706_1c5529d2a414);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMDSPStorage3_Vtbl {
    pub base__: IMDSPStorage2_Vtbl,
    pub GetMetadata: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pmetadata: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub SetMetadata: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pmetadata: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IMDSPStorage4(::windows_core::IUnknown);
impl IMDSPStorage4 {
    #[doc = "Required features: `\"Win32_Media_Audio\"`"]
    #[cfg(feature = "Win32_Media_Audio")]
    pub unsafe fn SetAttributes(&self, dwattributes: u32, pformat: ::core::option::Option<*const super::Audio::WAVEFORMATEX>) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.base__.SetAttributes)(::windows_core::Interface::as_raw(self), dwattributes, ::core::mem::transmute(pformat.unwrap_or(::std::ptr::null()))).ok()
    }
    pub unsafe fn GetStorageGlobals(&self) -> ::windows_core::Result<IMDSPStorageGlobals> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.base__.GetStorageGlobals)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Media_Audio\"`"]
    #[cfg(feature = "Win32_Media_Audio")]
    pub unsafe fn GetAttributes(&self, pdwattributes: *mut u32, pformat: ::core::option::Option<*mut super::Audio::WAVEFORMATEX>) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.base__.GetAttributes)(::windows_core::Interface::as_raw(self), pdwattributes, ::core::mem::transmute(pformat.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    pub unsafe fn GetName(&self, pwszname: &mut [u16]) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.base__.GetName)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pwszname.as_ptr()), pwszname.len().try_into().unwrap()).ok()
    }
    pub unsafe fn GetDate(&self) -> ::windows_core::Result<WMDMDATETIME> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.base__.GetDate)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetSize(&self, pdwsizelow: *mut u32, pdwsizehigh: *mut u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.base__.GetSize)(::windows_core::Interface::as_raw(self), pdwsizelow, pdwsizehigh).ok()
    }
    pub unsafe fn GetRights(&self, pprights: *mut *mut WMDMRIGHTS, pnrightscount: *mut u32, abmac: &mut [u8; 8]) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.base__.GetRights)(::windows_core::Interface::as_raw(self), pprights, pnrightscount, ::core::mem::transmute(abmac.as_ptr())).ok()
    }
    #[doc = "Required features: `\"Win32_Media_Audio\"`"]
    #[cfg(feature = "Win32_Media_Audio")]
    pub unsafe fn CreateStorage<P0>(&self, dwattributes: u32, pformat: ::core::option::Option<*const super::Audio::WAVEFORMATEX>, pwszname: P0) -> ::windows_core::Result<IMDSPStorage>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.base__.CreateStorage)(::windows_core::Interface::as_raw(self), dwattributes, ::core::mem::transmute(pformat.unwrap_or(::std::ptr::null())), pwszname.into_param().abi(), &mut result__).from_abi(result__)
    }
    pub unsafe fn EnumStorage(&self) -> ::windows_core::Result<IMDSPEnumStorage> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.base__.EnumStorage)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SendOpaqueCommand(&self, pcommand: *mut OPAQUECOMMAND) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.base__.SendOpaqueCommand)(::windows_core::Interface::as_raw(self), pcommand).ok()
    }
    pub unsafe fn GetStorage<P0>(&self, pszstoragename: P0) -> ::windows_core::Result<IMDSPStorage>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.GetStorage)(::windows_core::Interface::as_raw(self), pszstoragename.into_param().abi(), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`, `\"Win32_Media_Audio\"`, `\"Win32_Media_MediaFoundation\"`"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_Media_Audio", feature = "Win32_Media_MediaFoundation"))]
    pub unsafe fn CreateStorage2<P0>(&self, dwattributes: u32, dwattributesex: u32, paudioformat: ::core::option::Option<*const super::Audio::WAVEFORMATEX>, pvideoformat: ::core::option::Option<*const super::MediaFoundation::VIDEOINFOHEADER>, pwszname: P0, qwfilesize: u64) -> ::windows_core::Result<IMDSPStorage>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.CreateStorage2)(::windows_core::Interface::as_raw(self), dwattributes, dwattributesex, ::core::mem::transmute(paudioformat.unwrap_or(::std::ptr::null())), ::core::mem::transmute(pvideoformat.unwrap_or(::std::ptr::null())), pwszname.into_param().abi(), qwfilesize, &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`, `\"Win32_Media_Audio\"`, `\"Win32_Media_MediaFoundation\"`"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_Media_Audio", feature = "Win32_Media_MediaFoundation"))]
    pub unsafe fn SetAttributes2(&self, dwattributes: u32, dwattributesex: u32, paudioformat: ::core::option::Option<*const super::Audio::WAVEFORMATEX>, pvideoformat: ::core::option::Option<*const super::MediaFoundation::VIDEOINFOHEADER>) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.SetAttributes2)(::windows_core::Interface::as_raw(self), dwattributes, dwattributesex, ::core::mem::transmute(paudioformat.unwrap_or(::std::ptr::null())), ::core::mem::transmute(pvideoformat.unwrap_or(::std::ptr::null()))).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`, `\"Win32_Media_Audio\"`, `\"Win32_Media_MediaFoundation\"`"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_Media_Audio", feature = "Win32_Media_MediaFoundation"))]
    pub unsafe fn GetAttributes2(&self, pdwattributes: *mut u32, pdwattributesex: *mut u32, paudioformat: ::core::option::Option<*mut super::Audio::WAVEFORMATEX>, pvideoformat: ::core::option::Option<*mut super::MediaFoundation::VIDEOINFOHEADER>) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.GetAttributes2)(::windows_core::Interface::as_raw(self), pdwattributes, pdwattributesex, ::core::mem::transmute(paudioformat.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(pvideoformat.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    pub unsafe fn GetMetadata<P0>(&self, pmetadata: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IWMDMMetaData>,
    {
        (::windows_core::Interface::vtable(self).base__.GetMetadata)(::windows_core::Interface::as_raw(self), pmetadata.into_param().abi()).ok()
    }
    pub unsafe fn SetMetadata<P0>(&self, pmetadata: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IWMDMMetaData>,
    {
        (::windows_core::Interface::vtable(self).base__.SetMetadata)(::windows_core::Interface::as_raw(self), pmetadata.into_param().abi()).ok()
    }
    pub unsafe fn SetReferences(&self, ppispstorage: ::core::option::Option<&[::core::option::Option<IMDSPStorage>]>) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetReferences)(::windows_core::Interface::as_raw(self), ppispstorage.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), ::core::mem::transmute(ppispstorage.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr()))).ok()
    }
    pub unsafe fn GetReferences(&self, pdwrefs: *mut u32, pppispstorage: *mut *mut ::core::option::Option<IMDSPStorage>) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetReferences)(::windows_core::Interface::as_raw(self), pdwrefs, pppispstorage).ok()
    }
    pub unsafe fn CreateStorageWithMetadata<P0, P1>(&self, dwattributes: u32, pwszname: P0, pmetadata: P1, qwfilesize: u64) -> ::windows_core::Result<IMDSPStorage>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
        P1: ::windows_core::IntoParam<IWMDMMetaData>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).CreateStorageWithMetadata)(::windows_core::Interface::as_raw(self), dwattributes, pwszname.into_param().abi(), pmetadata.into_param().abi(), qwfilesize, &mut result__).from_abi(result__)
    }
    pub unsafe fn GetSpecifiedMetadata<P0>(&self, ppwszpropnames: &[::windows_core::PCWSTR], pmetadata: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IWMDMMetaData>,
    {
        (::windows_core::Interface::vtable(self).GetSpecifiedMetadata)(::windows_core::Interface::as_raw(self), ppwszpropnames.len().try_into().unwrap(), ::core::mem::transmute(ppwszpropnames.as_ptr()), pmetadata.into_param().abi()).ok()
    }
    pub unsafe fn FindStorage<P0>(&self, findscope: WMDM_FIND_SCOPE, pwszuniqueid: P0) -> ::windows_core::Result<IMDSPStorage>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).FindStorage)(::windows_core::Interface::as_raw(self), findscope, pwszuniqueid.into_param().abi(), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetParent(&self) -> ::windows_core::Result<IMDSPStorage> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetParent)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
::windows_core::imp::interface_hierarchy!(IMDSPStorage4, ::windows_core::IUnknown, IMDSPStorage, IMDSPStorage2, IMDSPStorage3);
unsafe impl ::windows_core::Interface for IMDSPStorage4 {
    type Vtable = IMDSPStorage4_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IMDSPStorage4 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x3133b2c4_515c_481b_b1ce_39327ecb4f74);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMDSPStorage4_Vtbl {
    pub base__: IMDSPStorage3_Vtbl,
    pub SetReferences: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwrefs: u32, ppispstorage: *const *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub GetReferences: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdwrefs: *mut u32, pppispstorage: *mut *mut ::core::option::Option<IMDSPStorage>) -> ::windows_core::HRESULT,
    pub CreateStorageWithMetadata: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwattributes: u32, pwszname: ::windows_core::PCWSTR, pmetadata: *mut ::core::ffi::c_void, qwfilesize: u64, ppnewstorage: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub GetSpecifiedMetadata: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cproperties: u32, ppwszpropnames: *const ::windows_core::PCWSTR, pmetadata: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub FindStorage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, findscope: WMDM_FIND_SCOPE, pwszuniqueid: ::windows_core::PCWSTR, ppstorage: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub GetParent: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppstorage: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IMDSPStorageGlobals(::windows_core::IUnknown);
impl IMDSPStorageGlobals {
    pub unsafe fn GetCapabilities(&self) -> ::windows_core::Result<u32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetCapabilities)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetSerialNumber(&self, pserialnum: *mut WMDMID, abmac: &mut [u8; 8]) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetSerialNumber)(::windows_core::Interface::as_raw(self), pserialnum, ::core::mem::transmute(abmac.as_ptr())).ok()
    }
    pub unsafe fn GetTotalSize(&self, pdwtotalsizelow: *mut u32, pdwtotalsizehigh: *mut u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetTotalSize)(::windows_core::Interface::as_raw(self), pdwtotalsizelow, pdwtotalsizehigh).ok()
    }
    pub unsafe fn GetTotalFree(&self, pdwfreelow: *mut u32, pdwfreehigh: *mut u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetTotalFree)(::windows_core::Interface::as_raw(self), pdwfreelow, pdwfreehigh).ok()
    }
    pub unsafe fn GetTotalBad(&self, pdwbadlow: *mut u32, pdwbadhigh: *mut u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetTotalBad)(::windows_core::Interface::as_raw(self), pdwbadlow, pdwbadhigh).ok()
    }
    pub unsafe fn GetStatus(&self) -> ::windows_core::Result<u32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetStatus)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn Initialize<P0>(&self, fumode: u32, pprogress: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IWMDMProgress>,
    {
        (::windows_core::Interface::vtable(self).Initialize)(::windows_core::Interface::as_raw(self), fumode, pprogress.into_param().abi()).ok()
    }
    pub unsafe fn GetDevice(&self) -> ::windows_core::Result<IMDSPDevice> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetDevice)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetRootStorage(&self) -> ::windows_core::Result<IMDSPStorage> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetRootStorage)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
::windows_core::imp::interface_hierarchy!(IMDSPStorageGlobals, ::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IMDSPStorageGlobals {
    type Vtable = IMDSPStorageGlobals_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IMDSPStorageGlobals {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x1dcb3a17_33ed_11d3_8470_00c04f79dbc0);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMDSPStorageGlobals_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub GetCapabilities: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdwcapabilities: *mut u32) -> ::windows_core::HRESULT,
    pub GetSerialNumber: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pserialnum: *mut WMDMID, abmac: *mut u8) -> ::windows_core::HRESULT,
    pub GetTotalSize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdwtotalsizelow: *mut u32, pdwtotalsizehigh: *mut u32) -> ::windows_core::HRESULT,
    pub GetTotalFree: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdwfreelow: *mut u32, pdwfreehigh: *mut u32) -> ::windows_core::HRESULT,
    pub GetTotalBad: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdwbadlow: *mut u32, pdwbadhigh: *mut u32) -> ::windows_core::HRESULT,
    pub GetStatus: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdwstatus: *mut u32) -> ::windows_core::HRESULT,
    pub Initialize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, fumode: u32, pprogress: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub GetDevice: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppdevice: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub GetRootStorage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pproot: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IMDServiceProvider(::windows_core::IUnknown);
impl IMDServiceProvider {
    pub unsafe fn GetDeviceCount(&self) -> ::windows_core::Result<u32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetDeviceCount)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn EnumDevices(&self) -> ::windows_core::Result<IMDSPEnumDevice> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).EnumDevices)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
::windows_core::imp::interface_hierarchy!(IMDServiceProvider, ::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IMDServiceProvider {
    type Vtable = IMDServiceProvider_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IMDServiceProvider {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x1dcb3a10_33ed_11d3_8470_00c04f79dbc0);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMDServiceProvider_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub GetDeviceCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdwcount: *mut u32) -> ::windows_core::HRESULT,
    pub EnumDevices: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppenumdevice: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IMDServiceProvider2(::windows_core::IUnknown);
impl IMDServiceProvider2 {
    pub unsafe fn GetDeviceCount(&self) -> ::windows_core::Result<u32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetDeviceCount)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn EnumDevices(&self) -> ::windows_core::Result<IMDSPEnumDevice> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.EnumDevices)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn CreateDevice<P0>(&self, pwszdevicepath: P0, pdwcount: *mut u32, pppdevicearray: *mut *mut ::core::option::Option<IMDSPDevice>) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        (::windows_core::Interface::vtable(self).CreateDevice)(::windows_core::Interface::as_raw(self), pwszdevicepath.into_param().abi(), pdwcount, pppdevicearray).ok()
    }
}
::windows_core::imp::interface_hierarchy!(IMDServiceProvider2, ::windows_core::IUnknown, IMDServiceProvider);
unsafe impl ::windows_core::Interface for IMDServiceProvider2 {
    type Vtable = IMDServiceProvider2_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IMDServiceProvider2 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xb2fa24b7_cda3_4694_9862_413ae1a34819);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMDServiceProvider2_Vtbl {
    pub base__: IMDServiceProvider_Vtbl,
    pub CreateDevice: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwszdevicepath: ::windows_core::PCWSTR, pdwcount: *mut u32, pppdevicearray: *mut *mut ::core::option::Option<IMDSPDevice>) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IMDServiceProvider3(::windows_core::IUnknown);
impl IMDServiceProvider3 {
    pub unsafe fn GetDeviceCount(&self) -> ::windows_core::Result<u32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.GetDeviceCount)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn EnumDevices(&self) -> ::windows_core::Result<IMDSPEnumDevice> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.EnumDevices)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn CreateDevice<P0>(&self, pwszdevicepath: P0, pdwcount: *mut u32, pppdevicearray: *mut *mut ::core::option::Option<IMDSPDevice>) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        (::windows_core::Interface::vtable(self).base__.CreateDevice)(::windows_core::Interface::as_raw(self), pwszdevicepath.into_param().abi(), pdwcount, pppdevicearray).ok()
    }
    pub unsafe fn SetDeviceEnumPreference(&self, dwenumpref: u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetDeviceEnumPreference)(::windows_core::Interface::as_raw(self), dwenumpref).ok()
    }
}
::windows_core::imp::interface_hierarchy!(IMDServiceProvider3, ::windows_core::IUnknown, IMDServiceProvider, IMDServiceProvider2);
unsafe impl ::windows_core::Interface for IMDServiceProvider3 {
    type Vtable = IMDServiceProvider3_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IMDServiceProvider3 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x4ed13ef3_a971_4d19_9f51_0e1826b2da57);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMDServiceProvider3_Vtbl {
    pub base__: IMDServiceProvider2_Vtbl,
    pub SetDeviceEnumPreference: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwenumpref: u32) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct ISCPSecureAuthenticate(::windows_core::IUnknown);
impl ISCPSecureAuthenticate {
    pub unsafe fn GetSecureQuery(&self) -> ::windows_core::Result<ISCPSecureQuery> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetSecureQuery)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
::windows_core::imp::interface_hierarchy!(ISCPSecureAuthenticate, ::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ISCPSecureAuthenticate {
    type Vtable = ISCPSecureAuthenticate_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ISCPSecureAuthenticate {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x1dcb3a0f_33ed_11d3_8470_00c04f79dbc0);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISCPSecureAuthenticate_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub GetSecureQuery: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppsecurequery: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct ISCPSecureAuthenticate2(::windows_core::IUnknown);
impl ISCPSecureAuthenticate2 {
    pub unsafe fn GetSecureQuery(&self) -> ::windows_core::Result<ISCPSecureQuery> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetSecureQuery)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetSCPSession(&self) -> ::windows_core::Result<ISCPSession> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetSCPSession)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
::windows_core::imp::interface_hierarchy!(ISCPSecureAuthenticate2, ::windows_core::IUnknown, ISCPSecureAuthenticate);
unsafe impl ::windows_core::Interface for ISCPSecureAuthenticate2 {
    type Vtable = ISCPSecureAuthenticate2_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ISCPSecureAuthenticate2 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xb580cfae_1672_47e2_acaa_44bbecbcae5b);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISCPSecureAuthenticate2_Vtbl {
    pub base__: ISCPSecureAuthenticate_Vtbl,
    pub GetSCPSession: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppscpsession: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct ISCPSecureExchange(::windows_core::IUnknown);
impl ISCPSecureExchange {
    pub unsafe fn TransferContainerData(&self, pdata: &[u8], pfureadyflags: *mut u32, abmac: &mut [u8; 8]) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).TransferContainerData)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pdata.as_ptr()), pdata.len().try_into().unwrap(), pfureadyflags, ::core::mem::transmute(abmac.as_ptr())).ok()
    }
    pub unsafe fn ObjectData(&self, pdata: *mut u8, pdwsize: *mut u32, abmac: &mut [u8; 8]) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).ObjectData)(::windows_core::Interface::as_raw(self), pdata, pdwsize, ::core::mem::transmute(abmac.as_ptr())).ok()
    }
    pub unsafe fn TransferComplete(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).TransferComplete)(::windows_core::Interface::as_raw(self)).ok()
    }
}
::windows_core::imp::interface_hierarchy!(ISCPSecureExchange, ::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ISCPSecureExchange {
    type Vtable = ISCPSecureExchange_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ISCPSecureExchange {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x1dcb3a0e_33ed_11d3_8470_00c04f79dbc0);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISCPSecureExchange_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub TransferContainerData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdata: *const u8, dwsize: u32, pfureadyflags: *mut u32, abmac: *mut u8) -> ::windows_core::HRESULT,
    pub ObjectData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdata: *mut u8, pdwsize: *mut u32, abmac: *mut u8) -> ::windows_core::HRESULT,
    pub TransferComplete: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct ISCPSecureExchange2(::windows_core::IUnknown);
impl ISCPSecureExchange2 {
    pub unsafe fn TransferContainerData(&self, pdata: &[u8], pfureadyflags: *mut u32, abmac: &mut [u8; 8]) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.TransferContainerData)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pdata.as_ptr()), pdata.len().try_into().unwrap(), pfureadyflags, ::core::mem::transmute(abmac.as_ptr())).ok()
    }
    pub unsafe fn ObjectData(&self, pdata: *mut u8, pdwsize: *mut u32, abmac: &mut [u8; 8]) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.ObjectData)(::windows_core::Interface::as_raw(self), pdata, pdwsize, ::core::mem::transmute(abmac.as_ptr())).ok()
    }
    pub unsafe fn TransferComplete(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.TransferComplete)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn TransferContainerData2<P0>(&self, pdata: &[u8], pprogresscallback: P0, pfureadyflags: *mut u32, abmac: &mut [u8; 8]) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IWMDMProgress3>,
    {
        (::windows_core::Interface::vtable(self).TransferContainerData2)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pdata.as_ptr()), pdata.len().try_into().unwrap(), pprogresscallback.into_param().abi(), pfureadyflags, ::core::mem::transmute(abmac.as_ptr())).ok()
    }
}
::windows_core::imp::interface_hierarchy!(ISCPSecureExchange2, ::windows_core::IUnknown, ISCPSecureExchange);
unsafe impl ::windows_core::Interface for ISCPSecureExchange2 {
    type Vtable = ISCPSecureExchange2_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ISCPSecureExchange2 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x6c62fc7b_2690_483f_9d44_0a20cb35577c);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISCPSecureExchange2_Vtbl {
    pub base__: ISCPSecureExchange_Vtbl,
    pub TransferContainerData2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdata: *const u8, dwsize: u32, pprogresscallback: *mut ::core::ffi::c_void, pfureadyflags: *mut u32, abmac: *mut u8) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct ISCPSecureExchange3(::windows_core::IUnknown);
impl ISCPSecureExchange3 {
    pub unsafe fn TransferContainerData(&self, pdata: &[u8], pfureadyflags: *mut u32, abmac: &mut [u8; 8]) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.TransferContainerData)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pdata.as_ptr()), pdata.len().try_into().unwrap(), pfureadyflags, ::core::mem::transmute(abmac.as_ptr())).ok()
    }
    pub unsafe fn ObjectData(&self, pdata: *mut u8, pdwsize: *mut u32, abmac: &mut [u8; 8]) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.ObjectData)(::windows_core::Interface::as_raw(self), pdata, pdwsize, ::core::mem::transmute(abmac.as_ptr())).ok()
    }
    pub unsafe fn TransferComplete(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.TransferComplete)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn TransferContainerData2<P0>(&self, pdata: &[u8], pprogresscallback: P0, pfureadyflags: *mut u32, abmac: &mut [u8; 8]) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IWMDMProgress3>,
    {
        (::windows_core::Interface::vtable(self).base__.TransferContainerData2)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pdata.as_ptr()), pdata.len().try_into().unwrap(), pprogresscallback.into_param().abi(), pfureadyflags, ::core::mem::transmute(abmac.as_ptr())).ok()
    }
    pub unsafe fn TransferContainerDataOnClearChannel<P0, P1>(&self, pdevice: P0, pdata: &[u8], pprogresscallback: P1) -> ::windows_core::Result<u32>
    where
        P0: ::windows_core::IntoParam<IMDSPDevice>,
        P1: ::windows_core::IntoParam<IWMDMProgress3>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).TransferContainerDataOnClearChannel)(::windows_core::Interface::as_raw(self), pdevice.into_param().abi(), ::core::mem::transmute(pdata.as_ptr()), pdata.len().try_into().unwrap(), pprogresscallback.into_param().abi(), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetObjectDataOnClearChannel<P0>(&self, pdevice: P0, pdata: *mut u8, pdwsize: *mut u32) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IMDSPDevice>,
    {
        (::windows_core::Interface::vtable(self).GetObjectDataOnClearChannel)(::windows_core::Interface::as_raw(self), pdevice.into_param().abi(), pdata, pdwsize).ok()
    }
    pub unsafe fn TransferCompleteForDevice<P0>(&self, pdevice: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IMDSPDevice>,
    {
        (::windows_core::Interface::vtable(self).TransferCompleteForDevice)(::windows_core::Interface::as_raw(self), pdevice.into_param().abi()).ok()
    }
}
::windows_core::imp::interface_hierarchy!(ISCPSecureExchange3, ::windows_core::IUnknown, ISCPSecureExchange, ISCPSecureExchange2);
unsafe impl ::windows_core::Interface for ISCPSecureExchange3 {
    type Vtable = ISCPSecureExchange3_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ISCPSecureExchange3 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xab4e77e4_8908_4b17_bd2a_b1dbe6dd69e1);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISCPSecureExchange3_Vtbl {
    pub base__: ISCPSecureExchange2_Vtbl,
    pub TransferContainerDataOnClearChannel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdevice: *mut ::core::ffi::c_void, pdata: *const u8, dwsize: u32, pprogresscallback: *mut ::core::ffi::c_void, pfureadyflags: *mut u32) -> ::windows_core::HRESULT,
    pub GetObjectDataOnClearChannel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdevice: *mut ::core::ffi::c_void, pdata: *mut u8, pdwsize: *mut u32) -> ::windows_core::HRESULT,
    pub TransferCompleteForDevice: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdevice: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct ISCPSecureQuery(::windows_core::IUnknown);
impl ISCPSecureQuery {
    pub unsafe fn GetDataDemands(&self, pfuflags: *mut u32, pdwminrightsdata: *mut u32, pdwminexaminedata: *mut u32, pdwmindecidedata: *mut u32, abmac: &mut [u8; 8]) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetDataDemands)(::windows_core::Interface::as_raw(self), pfuflags, pdwminrightsdata, pdwminexaminedata, pdwmindecidedata, ::core::mem::transmute(abmac.as_ptr())).ok()
    }
    pub unsafe fn ExamineData<P0>(&self, fuflags: u32, pwszextension: P0, pdata: &[u8], abmac: &mut [u8; 8]) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        (::windows_core::Interface::vtable(self).ExamineData)(::windows_core::Interface::as_raw(self), fuflags, pwszextension.into_param().abi(), ::core::mem::transmute(pdata.as_ptr()), pdata.len().try_into().unwrap(), ::core::mem::transmute(abmac.as_ptr())).ok()
    }
    pub unsafe fn MakeDecision<P0>(&self, fuflags: u32, pdata: &[u8], dwappsec: u32, pbspsessionkey: &[u8], pstorageglobals: P0, ppexchange: *mut ::core::option::Option<ISCPSecureExchange>, abmac: &mut [u8; 8]) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IMDSPStorageGlobals>,
    {
        (::windows_core::Interface::vtable(self).MakeDecision)(::windows_core::Interface::as_raw(self), fuflags, ::core::mem::transmute(pdata.as_ptr()), pdata.len().try_into().unwrap(), dwappsec, ::core::mem::transmute(pbspsessionkey.as_ptr()), pbspsessionkey.len().try_into().unwrap(), pstorageglobals.into_param().abi(), ::core::mem::transmute(ppexchange), ::core::mem::transmute(abmac.as_ptr())).ok()
    }
    pub unsafe fn GetRights<P0>(&self, pdata: &[u8], pbspsessionkey: &[u8], pstgglobals: P0, pprights: *mut *mut WMDMRIGHTS, pnrightscount: *mut u32, abmac: &mut [u8; 8]) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IMDSPStorageGlobals>,
    {
        (::windows_core::Interface::vtable(self).GetRights)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pdata.as_ptr()), pdata.len().try_into().unwrap(), ::core::mem::transmute(pbspsessionkey.as_ptr()), pbspsessionkey.len().try_into().unwrap(), pstgglobals.into_param().abi(), pprights, pnrightscount, ::core::mem::transmute(abmac.as_ptr())).ok()
    }
}
::windows_core::imp::interface_hierarchy!(ISCPSecureQuery, ::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ISCPSecureQuery {
    type Vtable = ISCPSecureQuery_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ISCPSecureQuery {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x1dcb3a0d_33ed_11d3_8470_00c04f79dbc0);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISCPSecureQuery_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub GetDataDemands: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pfuflags: *mut u32, pdwminrightsdata: *mut u32, pdwminexaminedata: *mut u32, pdwmindecidedata: *mut u32, abmac: *mut u8) -> ::windows_core::HRESULT,
    pub ExamineData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, fuflags: u32, pwszextension: ::windows_core::PCWSTR, pdata: *const u8, dwsize: u32, abmac: *mut u8) -> ::windows_core::HRESULT,
    pub MakeDecision: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, fuflags: u32, pdata: *const u8, dwsize: u32, dwappsec: u32, pbspsessionkey: *const u8, dwsessionkeylen: u32, pstorageglobals: *mut ::core::ffi::c_void, ppexchange: *mut *mut ::core::ffi::c_void, abmac: *mut u8) -> ::windows_core::HRESULT,
    pub GetRights: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdata: *const u8, dwsize: u32, pbspsessionkey: *const u8, dwsessionkeylen: u32, pstgglobals: *mut ::core::ffi::c_void, pprights: *mut *mut WMDMRIGHTS, pnrightscount: *mut u32, abmac: *mut u8) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct ISCPSecureQuery2(::windows_core::IUnknown);
impl ISCPSecureQuery2 {
    pub unsafe fn GetDataDemands(&self, pfuflags: *mut u32, pdwminrightsdata: *mut u32, pdwminexaminedata: *mut u32, pdwmindecidedata: *mut u32, abmac: &mut [u8; 8]) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.GetDataDemands)(::windows_core::Interface::as_raw(self), pfuflags, pdwminrightsdata, pdwminexaminedata, pdwmindecidedata, ::core::mem::transmute(abmac.as_ptr())).ok()
    }
    pub unsafe fn ExamineData<P0>(&self, fuflags: u32, pwszextension: P0, pdata: &[u8], abmac: &mut [u8; 8]) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        (::windows_core::Interface::vtable(self).base__.ExamineData)(::windows_core::Interface::as_raw(self), fuflags, pwszextension.into_param().abi(), ::core::mem::transmute(pdata.as_ptr()), pdata.len().try_into().unwrap(), ::core::mem::transmute(abmac.as_ptr())).ok()
    }
    pub unsafe fn MakeDecision<P0>(&self, fuflags: u32, pdata: &[u8], dwappsec: u32, pbspsessionkey: &[u8], pstorageglobals: P0, ppexchange: *mut ::core::option::Option<ISCPSecureExchange>, abmac: &mut [u8; 8]) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IMDSPStorageGlobals>,
    {
        (::windows_core::Interface::vtable(self).base__.MakeDecision)(::windows_core::Interface::as_raw(self), fuflags, ::core::mem::transmute(pdata.as_ptr()), pdata.len().try_into().unwrap(), dwappsec, ::core::mem::transmute(pbspsessionkey.as_ptr()), pbspsessionkey.len().try_into().unwrap(), pstorageglobals.into_param().abi(), ::core::mem::transmute(ppexchange), ::core::mem::transmute(abmac.as_ptr())).ok()
    }
    pub unsafe fn GetRights<P0>(&self, pdata: &[u8], pbspsessionkey: &[u8], pstgglobals: P0, pprights: *mut *mut WMDMRIGHTS, pnrightscount: *mut u32, abmac: &mut [u8; 8]) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IMDSPStorageGlobals>,
    {
        (::windows_core::Interface::vtable(self).base__.GetRights)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pdata.as_ptr()), pdata.len().try_into().unwrap(), ::core::mem::transmute(pbspsessionkey.as_ptr()), pbspsessionkey.len().try_into().unwrap(), pstgglobals.into_param().abi(), pprights, pnrightscount, ::core::mem::transmute(abmac.as_ptr())).ok()
    }
    pub unsafe fn MakeDecision2<P0, P1>(&self, fuflags: u32, pdata: &[u8], dwappsec: u32, pbspsessionkey: &[u8], pstorageglobals: P0, pappcertapp: &[u8], pappcertsp: &[u8], pszrevocationurl: *mut ::windows_core::PWSTR, pdwrevocationurllen: *mut u32, pdwrevocationbitflag: *mut u32, pqwfilesize: ::core::option::Option<*mut u64>, punknown: P1, ppexchange: *mut ::core::option::Option<ISCPSecureExchange>, abmac: &mut [u8; 8]) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IMDSPStorageGlobals>,
        P1: ::windows_core::IntoParam<::windows_core::IUnknown>,
    {
        (::windows_core::Interface::vtable(self).MakeDecision2)(
            ::windows_core::Interface::as_raw(self),
            fuflags,
            ::core::mem::transmute(pdata.as_ptr()),
            pdata.len().try_into().unwrap(),
            dwappsec,
            ::core::mem::transmute(pbspsessionkey.as_ptr()),
            pbspsessionkey.len().try_into().unwrap(),
            pstorageglobals.into_param().abi(),
            ::core::mem::transmute(pappcertapp.as_ptr()),
            pappcertapp.len().try_into().unwrap(),
            ::core::mem::transmute(pappcertsp.as_ptr()),
            pappcertsp.len().try_into().unwrap(),
            pszrevocationurl,
            pdwrevocationurllen,
            pdwrevocationbitflag,
            ::core::mem::transmute(pqwfilesize.unwrap_or(::std::ptr::null_mut())),
            punknown.into_param().abi(),
            ::core::mem::transmute(ppexchange),
            ::core::mem::transmute(abmac.as_ptr()),
        )
        .ok()
    }
}
::windows_core::imp::interface_hierarchy!(ISCPSecureQuery2, ::windows_core::IUnknown, ISCPSecureQuery);
unsafe impl ::windows_core::Interface for ISCPSecureQuery2 {
    type Vtable = ISCPSecureQuery2_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ISCPSecureQuery2 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xebe17e25_4fd7_4632_af46_6d93d4fcc72e);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISCPSecureQuery2_Vtbl {
    pub base__: ISCPSecureQuery_Vtbl,
    pub MakeDecision2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, fuflags: u32, pdata: *const u8, dwsize: u32, dwappsec: u32, pbspsessionkey: *const u8, dwsessionkeylen: u32, pstorageglobals: *mut ::core::ffi::c_void, pappcertapp: *const u8, dwappcertapplen: u32, pappcertsp: *const u8, dwappcertsplen: u32, pszrevocationurl: *mut ::windows_core::PWSTR, pdwrevocationurllen: *mut u32, pdwrevocationbitflag: *mut u32, pqwfilesize: *mut u64, punknown: *mut ::core::ffi::c_void, ppexchange: *mut *mut ::core::ffi::c_void, abmac: *mut u8) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct ISCPSecureQuery3(::windows_core::IUnknown);
impl ISCPSecureQuery3 {
    pub unsafe fn GetDataDemands(&self, pfuflags: *mut u32, pdwminrightsdata: *mut u32, pdwminexaminedata: *mut u32, pdwmindecidedata: *mut u32, abmac: &mut [u8; 8]) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.GetDataDemands)(::windows_core::Interface::as_raw(self), pfuflags, pdwminrightsdata, pdwminexaminedata, pdwmindecidedata, ::core::mem::transmute(abmac.as_ptr())).ok()
    }
    pub unsafe fn ExamineData<P0>(&self, fuflags: u32, pwszextension: P0, pdata: &[u8], abmac: &mut [u8; 8]) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        (::windows_core::Interface::vtable(self).base__.base__.ExamineData)(::windows_core::Interface::as_raw(self), fuflags, pwszextension.into_param().abi(), ::core::mem::transmute(pdata.as_ptr()), pdata.len().try_into().unwrap(), ::core::mem::transmute(abmac.as_ptr())).ok()
    }
    pub unsafe fn MakeDecision<P0>(&self, fuflags: u32, pdata: &[u8], dwappsec: u32, pbspsessionkey: &[u8], pstorageglobals: P0, ppexchange: *mut ::core::option::Option<ISCPSecureExchange>, abmac: &mut [u8; 8]) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IMDSPStorageGlobals>,
    {
        (::windows_core::Interface::vtable(self).base__.base__.MakeDecision)(::windows_core::Interface::as_raw(self), fuflags, ::core::mem::transmute(pdata.as_ptr()), pdata.len().try_into().unwrap(), dwappsec, ::core::mem::transmute(pbspsessionkey.as_ptr()), pbspsessionkey.len().try_into().unwrap(), pstorageglobals.into_param().abi(), ::core::mem::transmute(ppexchange), ::core::mem::transmute(abmac.as_ptr())).ok()
    }
    pub unsafe fn GetRights<P0>(&self, pdata: &[u8], pbspsessionkey: &[u8], pstgglobals: P0, pprights: *mut *mut WMDMRIGHTS, pnrightscount: *mut u32, abmac: &mut [u8; 8]) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IMDSPStorageGlobals>,
    {
        (::windows_core::Interface::vtable(self).base__.base__.GetRights)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pdata.as_ptr()), pdata.len().try_into().unwrap(), ::core::mem::transmute(pbspsessionkey.as_ptr()), pbspsessionkey.len().try_into().unwrap(), pstgglobals.into_param().abi(), pprights, pnrightscount, ::core::mem::transmute(abmac.as_ptr())).ok()
    }
    pub unsafe fn MakeDecision2<P0, P1>(&self, fuflags: u32, pdata: &[u8], dwappsec: u32, pbspsessionkey: &[u8], pstorageglobals: P0, pappcertapp: &[u8], pappcertsp: &[u8], pszrevocationurl: *mut ::windows_core::PWSTR, pdwrevocationurllen: *mut u32, pdwrevocationbitflag: *mut u32, pqwfilesize: ::core::option::Option<*mut u64>, punknown: P1, ppexchange: *mut ::core::option::Option<ISCPSecureExchange>, abmac: &mut [u8; 8]) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IMDSPStorageGlobals>,
        P1: ::windows_core::IntoParam<::windows_core::IUnknown>,
    {
        (::windows_core::Interface::vtable(self).base__.MakeDecision2)(
            ::windows_core::Interface::as_raw(self),
            fuflags,
            ::core::mem::transmute(pdata.as_ptr()),
            pdata.len().try_into().unwrap(),
            dwappsec,
            ::core::mem::transmute(pbspsessionkey.as_ptr()),
            pbspsessionkey.len().try_into().unwrap(),
            pstorageglobals.into_param().abi(),
            ::core::mem::transmute(pappcertapp.as_ptr()),
            pappcertapp.len().try_into().unwrap(),
            ::core::mem::transmute(pappcertsp.as_ptr()),
            pappcertsp.len().try_into().unwrap(),
            pszrevocationurl,
            pdwrevocationurllen,
            pdwrevocationbitflag,
            ::core::mem::transmute(pqwfilesize.unwrap_or(::std::ptr::null_mut())),
            punknown.into_param().abi(),
            ::core::mem::transmute(ppexchange),
            ::core::mem::transmute(abmac.as_ptr()),
        )
        .ok()
    }
    pub unsafe fn GetRightsOnClearChannel<P0, P1>(&self, pdata: &[u8], pbspsessionkey: &[u8], pstgglobals: P0, pprogresscallback: P1, pprights: *mut *mut WMDMRIGHTS, pnrightscount: *mut u32) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IMDSPStorageGlobals>,
        P1: ::windows_core::IntoParam<IWMDMProgress3>,
    {
        (::windows_core::Interface::vtable(self).GetRightsOnClearChannel)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pdata.as_ptr()), pdata.len().try_into().unwrap(), ::core::mem::transmute(pbspsessionkey.as_ptr()), pbspsessionkey.len().try_into().unwrap(), pstgglobals.into_param().abi(), pprogresscallback.into_param().abi(), pprights, pnrightscount).ok()
    }
    pub unsafe fn MakeDecisionOnClearChannel<P0, P1, P2>(&self, fuflags: u32, pdata: &[u8], dwappsec: u32, pbspsessionkey: &[u8], pstorageglobals: P0, pprogresscallback: P1, pappcertapp: &[u8], pappcertsp: &[u8], pszrevocationurl: *mut ::windows_core::PWSTR, pdwrevocationurllen: *mut u32, pdwrevocationbitflag: *mut u32, pqwfilesize: ::core::option::Option<*mut u64>, punknown: P2, ppexchange: *mut ::core::option::Option<ISCPSecureExchange>) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IMDSPStorageGlobals>,
        P1: ::windows_core::IntoParam<IWMDMProgress3>,
        P2: ::windows_core::IntoParam<::windows_core::IUnknown>,
    {
        (::windows_core::Interface::vtable(self).MakeDecisionOnClearChannel)(
            ::windows_core::Interface::as_raw(self),
            fuflags,
            ::core::mem::transmute(pdata.as_ptr()),
            pdata.len().try_into().unwrap(),
            dwappsec,
            ::core::mem::transmute(pbspsessionkey.as_ptr()),
            pbspsessionkey.len().try_into().unwrap(),
            pstorageglobals.into_param().abi(),
            pprogresscallback.into_param().abi(),
            ::core::mem::transmute(pappcertapp.as_ptr()),
            pappcertapp.len().try_into().unwrap(),
            ::core::mem::transmute(pappcertsp.as_ptr()),
            pappcertsp.len().try_into().unwrap(),
            pszrevocationurl,
            pdwrevocationurllen,
            pdwrevocationbitflag,
            ::core::mem::transmute(pqwfilesize.unwrap_or(::std::ptr::null_mut())),
            punknown.into_param().abi(),
            ::core::mem::transmute(ppexchange),
        )
        .ok()
    }
}
::windows_core::imp::interface_hierarchy!(ISCPSecureQuery3, ::windows_core::IUnknown, ISCPSecureQuery, ISCPSecureQuery2);
unsafe impl ::windows_core::Interface for ISCPSecureQuery3 {
    type Vtable = ISCPSecureQuery3_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ISCPSecureQuery3 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xb7edd1a2_4dab_484b_b3c5_ad39b8b4c0b1);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISCPSecureQuery3_Vtbl {
    pub base__: ISCPSecureQuery2_Vtbl,
    pub GetRightsOnClearChannel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdata: *const u8, dwsize: u32, pbspsessionkey: *const u8, dwsessionkeylen: u32, pstgglobals: *mut ::core::ffi::c_void, pprogresscallback: *mut ::core::ffi::c_void, pprights: *mut *mut WMDMRIGHTS, pnrightscount: *mut u32) -> ::windows_core::HRESULT,
    pub MakeDecisionOnClearChannel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, fuflags: u32, pdata: *const u8, dwsize: u32, dwappsec: u32, pbspsessionkey: *const u8, dwsessionkeylen: u32, pstorageglobals: *mut ::core::ffi::c_void, pprogresscallback: *mut ::core::ffi::c_void, pappcertapp: *const u8, dwappcertapplen: u32, pappcertsp: *const u8, dwappcertsplen: u32, pszrevocationurl: *mut ::windows_core::PWSTR, pdwrevocationurllen: *mut u32, pdwrevocationbitflag: *mut u32, pqwfilesize: *mut u64, punknown: *mut ::core::ffi::c_void, ppexchange: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct ISCPSession(::windows_core::IUnknown);
impl ISCPSession {
    pub unsafe fn BeginSession<P0>(&self, pidevice: P0, pctx: &[u8]) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IMDSPDevice>,
    {
        (::windows_core::Interface::vtable(self).BeginSession)(::windows_core::Interface::as_raw(self), pidevice.into_param().abi(), ::core::mem::transmute(pctx.as_ptr()), pctx.len().try_into().unwrap()).ok()
    }
    pub unsafe fn EndSession(&self, pctx: &[u8]) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).EndSession)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pctx.as_ptr()), pctx.len().try_into().unwrap()).ok()
    }
    pub unsafe fn GetSecureQuery(&self) -> ::windows_core::Result<ISCPSecureQuery> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetSecureQuery)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
::windows_core::imp::interface_hierarchy!(ISCPSession, ::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ISCPSession {
    type Vtable = ISCPSession_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ISCPSession {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x88a3e6ed_eee4_4619_bbb3_fd4fb62715d1);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISCPSession_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub BeginSession: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pidevice: *mut ::core::ffi::c_void, pctx: *const u8, dwsizectx: u32) -> ::windows_core::HRESULT,
    pub EndSession: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pctx: *const u8, dwsizectx: u32) -> ::windows_core::HRESULT,
    pub GetSecureQuery: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppsecurequery: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IWMDMDevice(::windows_core::IUnknown);
impl IWMDMDevice {
    pub unsafe fn GetName(&self, pwszname: &mut [u16]) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetName)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pwszname.as_ptr()), pwszname.len().try_into().unwrap()).ok()
    }
    pub unsafe fn GetManufacturer(&self, pwszname: &mut [u16]) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetManufacturer)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pwszname.as_ptr()), pwszname.len().try_into().unwrap()).ok()
    }
    pub unsafe fn GetVersion(&self) -> ::windows_core::Result<u32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetVersion)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetType(&self) -> ::windows_core::Result<u32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetType)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetSerialNumber(&self, pserialnumber: *mut WMDMID, abmac: &mut [u8; 8]) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetSerialNumber)(::windows_core::Interface::as_raw(self), pserialnumber, ::core::mem::transmute(abmac.as_ptr())).ok()
    }
    pub unsafe fn GetPowerSource(&self, pdwpowersource: *mut u32, pdwpercentremaining: *mut u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetPowerSource)(::windows_core::Interface::as_raw(self), pdwpowersource, pdwpercentremaining).ok()
    }
    pub unsafe fn GetStatus(&self) -> ::windows_core::Result<u32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetStatus)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetDeviceIcon(&self) -> ::windows_core::Result<u32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetDeviceIcon)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn EnumStorage(&self) -> ::windows_core::Result<IWMDMEnumStorage> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).EnumStorage)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Media_Audio\"`"]
    #[cfg(feature = "Win32_Media_Audio")]
    pub unsafe fn GetFormatSupport(&self, ppformatex: *mut *mut super::Audio::WAVEFORMATEX, pnformatcount: *mut u32, pppwszmimetype: *mut *mut ::windows_core::PWSTR, pnmimetypecount: *mut u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetFormatSupport)(::windows_core::Interface::as_raw(self), ppformatex, pnformatcount, pppwszmimetype, pnmimetypecount).ok()
    }
    pub unsafe fn SendOpaqueCommand(&self, pcommand: *mut OPAQUECOMMAND) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SendOpaqueCommand)(::windows_core::Interface::as_raw(self), pcommand).ok()
    }
}
::windows_core::imp::interface_hierarchy!(IWMDMDevice, ::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IWMDMDevice {
    type Vtable = IWMDMDevice_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IWMDMDevice {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x1dcb3a02_33ed_11d3_8470_00c04f79dbc0);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMDMDevice_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub GetName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwszname: ::windows_core::PWSTR, nmaxchars: u32) -> ::windows_core::HRESULT,
    pub GetManufacturer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwszname: ::windows_core::PWSTR, nmaxchars: u32) -> ::windows_core::HRESULT,
    pub GetVersion: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdwversion: *mut u32) -> ::windows_core::HRESULT,
    pub GetType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdwtype: *mut u32) -> ::windows_core::HRESULT,
    pub GetSerialNumber: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pserialnumber: *mut WMDMID, abmac: *mut u8) -> ::windows_core::HRESULT,
    pub GetPowerSource: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdwpowersource: *mut u32, pdwpercentremaining: *mut u32) -> ::windows_core::HRESULT,
    pub GetStatus: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdwstatus: *mut u32) -> ::windows_core::HRESULT,
    pub GetDeviceIcon: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hicon: *mut u32) -> ::windows_core::HRESULT,
    pub EnumStorage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppenumstorage: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_Media_Audio")]
    pub GetFormatSupport: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppformatex: *mut *mut super::Audio::WAVEFORMATEX, pnformatcount: *mut u32, pppwszmimetype: *mut *mut ::windows_core::PWSTR, pnmimetypecount: *mut u32) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Media_Audio"))]
    GetFormatSupport: usize,
    pub SendOpaqueCommand: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcommand: *mut OPAQUECOMMAND) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IWMDMDevice2(::windows_core::IUnknown);
impl IWMDMDevice2 {
    pub unsafe fn GetName(&self, pwszname: &mut [u16]) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.GetName)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pwszname.as_ptr()), pwszname.len().try_into().unwrap()).ok()
    }
    pub unsafe fn GetManufacturer(&self, pwszname: &mut [u16]) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.GetManufacturer)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pwszname.as_ptr()), pwszname.len().try_into().unwrap()).ok()
    }
    pub unsafe fn GetVersion(&self) -> ::windows_core::Result<u32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetVersion)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetType(&self) -> ::windows_core::Result<u32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetType)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetSerialNumber(&self, pserialnumber: *mut WMDMID, abmac: &mut [u8; 8]) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.GetSerialNumber)(::windows_core::Interface::as_raw(self), pserialnumber, ::core::mem::transmute(abmac.as_ptr())).ok()
    }
    pub unsafe fn GetPowerSource(&self, pdwpowersource: *mut u32, pdwpercentremaining: *mut u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.GetPowerSource)(::windows_core::Interface::as_raw(self), pdwpowersource, pdwpercentremaining).ok()
    }
    pub unsafe fn GetStatus(&self) -> ::windows_core::Result<u32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetStatus)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetDeviceIcon(&self) -> ::windows_core::Result<u32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetDeviceIcon)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn EnumStorage(&self) -> ::windows_core::Result<IWMDMEnumStorage> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.EnumStorage)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Media_Audio\"`"]
    #[cfg(feature = "Win32_Media_Audio")]
    pub unsafe fn GetFormatSupport(&self, ppformatex: *mut *mut super::Audio::WAVEFORMATEX, pnformatcount: *mut u32, pppwszmimetype: *mut *mut ::windows_core::PWSTR, pnmimetypecount: *mut u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.GetFormatSupport)(::windows_core::Interface::as_raw(self), ppformatex, pnformatcount, pppwszmimetype, pnmimetypecount).ok()
    }
    pub unsafe fn SendOpaqueCommand(&self, pcommand: *mut OPAQUECOMMAND) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.SendOpaqueCommand)(::windows_core::Interface::as_raw(self), pcommand).ok()
    }
    pub unsafe fn GetStorage<P0>(&self, pszstoragename: P0) -> ::windows_core::Result<IWMDMStorage>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetStorage)(::windows_core::Interface::as_raw(self), pszstoragename.into_param().abi(), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`, `\"Win32_Media_Audio\"`, `\"Win32_Media_MediaFoundation\"`"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_Media_Audio", feature = "Win32_Media_MediaFoundation"))]
    pub unsafe fn GetFormatSupport2(&self, dwflags: u32, ppaudioformatex: *mut *mut super::Audio::WAVEFORMATEX, pnaudioformatcount: *mut u32, ppvideoformatex: *mut *mut super::MediaFoundation::VIDEOINFOHEADER, pnvideoformatcount: *mut u32, ppfiletype: *mut *mut WMFILECAPABILITIES, pnfiletypecount: *mut u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetFormatSupport2)(::windows_core::Interface::as_raw(self), dwflags, ppaudioformatex, pnaudioformatcount, ppvideoformatex, pnvideoformatcount, ppfiletype, pnfiletypecount).ok()
    }
    #[doc = "Required features: `\"Win32_System_Ole\"`"]
    #[cfg(feature = "Win32_System_Ole")]
    pub unsafe fn GetSpecifyPropertyPages(&self, ppspecifyproppages: *mut ::core::option::Option<super::super::System::Ole::ISpecifyPropertyPages>, pppunknowns: *mut *mut ::core::option::Option<::windows_core::IUnknown>, pcunks: *mut u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetSpecifyPropertyPages)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(ppspecifyproppages), pppunknowns, pcunks).ok()
    }
    pub unsafe fn GetCanonicalName(&self, pwszpnpname: &mut [u16]) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetCanonicalName)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pwszpnpname.as_ptr()), pwszpnpname.len().try_into().unwrap()).ok()
    }
}
::windows_core::imp::interface_hierarchy!(IWMDMDevice2, ::windows_core::IUnknown, IWMDMDevice);
unsafe impl ::windows_core::Interface for IWMDMDevice2 {
    type Vtable = IWMDMDevice2_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IWMDMDevice2 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xe34f3d37_9d67_4fc1_9252_62d28b2f8b55);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMDMDevice2_Vtbl {
    pub base__: IWMDMDevice_Vtbl,
    pub GetStorage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszstoragename: ::windows_core::PCWSTR, ppstorage: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_Media_Audio", feature = "Win32_Media_MediaFoundation"))]
    pub GetFormatSupport2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwflags: u32, ppaudioformatex: *mut *mut super::Audio::WAVEFORMATEX, pnaudioformatcount: *mut u32, ppvideoformatex: *mut *mut super::MediaFoundation::VIDEOINFOHEADER, pnvideoformatcount: *mut u32, ppfiletype: *mut *mut WMFILECAPABILITIES, pnfiletypecount: *mut u32) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_Media_Audio", feature = "Win32_Media_MediaFoundation")))]
    GetFormatSupport2: usize,
    #[cfg(feature = "Win32_System_Ole")]
    pub GetSpecifyPropertyPages: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppspecifyproppages: *mut *mut ::core::ffi::c_void, pppunknowns: *mut *mut ::core::option::Option<::windows_core::IUnknown>, pcunks: *mut u32) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Ole"))]
    GetSpecifyPropertyPages: usize,
    pub GetCanonicalName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwszpnpname: ::windows_core::PWSTR, nmaxchars: u32) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IWMDMDevice3(::windows_core::IUnknown);
impl IWMDMDevice3 {
    pub unsafe fn GetName(&self, pwszname: &mut [u16]) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.GetName)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pwszname.as_ptr()), pwszname.len().try_into().unwrap()).ok()
    }
    pub unsafe fn GetManufacturer(&self, pwszname: &mut [u16]) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.GetManufacturer)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pwszname.as_ptr()), pwszname.len().try_into().unwrap()).ok()
    }
    pub unsafe fn GetVersion(&self) -> ::windows_core::Result<u32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.GetVersion)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetType(&self) -> ::windows_core::Result<u32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.GetType)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetSerialNumber(&self, pserialnumber: *mut WMDMID, abmac: &mut [u8; 8]) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.GetSerialNumber)(::windows_core::Interface::as_raw(self), pserialnumber, ::core::mem::transmute(abmac.as_ptr())).ok()
    }
    pub unsafe fn GetPowerSource(&self, pdwpowersource: *mut u32, pdwpercentremaining: *mut u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.GetPowerSource)(::windows_core::Interface::as_raw(self), pdwpowersource, pdwpercentremaining).ok()
    }
    pub unsafe fn GetStatus(&self) -> ::windows_core::Result<u32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.GetStatus)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetDeviceIcon(&self) -> ::windows_core::Result<u32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.GetDeviceIcon)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn EnumStorage(&self) -> ::windows_core::Result<IWMDMEnumStorage> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.EnumStorage)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Media_Audio\"`"]
    #[cfg(feature = "Win32_Media_Audio")]
    pub unsafe fn GetFormatSupport(&self, ppformatex: *mut *mut super::Audio::WAVEFORMATEX, pnformatcount: *mut u32, pppwszmimetype: *mut *mut ::windows_core::PWSTR, pnmimetypecount: *mut u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.GetFormatSupport)(::windows_core::Interface::as_raw(self), ppformatex, pnformatcount, pppwszmimetype, pnmimetypecount).ok()
    }
    pub unsafe fn SendOpaqueCommand(&self, pcommand: *mut OPAQUECOMMAND) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.SendOpaqueCommand)(::windows_core::Interface::as_raw(self), pcommand).ok()
    }
    pub unsafe fn GetStorage<P0>(&self, pszstoragename: P0) -> ::windows_core::Result<IWMDMStorage>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetStorage)(::windows_core::Interface::as_raw(self), pszstoragename.into_param().abi(), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`, `\"Win32_Media_Audio\"`, `\"Win32_Media_MediaFoundation\"`"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_Media_Audio", feature = "Win32_Media_MediaFoundation"))]
    pub unsafe fn GetFormatSupport2(&self, dwflags: u32, ppaudioformatex: *mut *mut super::Audio::WAVEFORMATEX, pnaudioformatcount: *mut u32, ppvideoformatex: *mut *mut super::MediaFoundation::VIDEOINFOHEADER, pnvideoformatcount: *mut u32, ppfiletype: *mut *mut WMFILECAPABILITIES, pnfiletypecount: *mut u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.GetFormatSupport2)(::windows_core::Interface::as_raw(self), dwflags, ppaudioformatex, pnaudioformatcount, ppvideoformatex, pnvideoformatcount, ppfiletype, pnfiletypecount).ok()
    }
    #[doc = "Required features: `\"Win32_System_Ole\"`"]
    #[cfg(feature = "Win32_System_Ole")]
    pub unsafe fn GetSpecifyPropertyPages(&self, ppspecifyproppages: *mut ::core::option::Option<super::super::System::Ole::ISpecifyPropertyPages>, pppunknowns: *mut *mut ::core::option::Option<::windows_core::IUnknown>, pcunks: *mut u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.GetSpecifyPropertyPages)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(ppspecifyproppages), pppunknowns, pcunks).ok()
    }
    pub unsafe fn GetCanonicalName(&self, pwszpnpname: &mut [u16]) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.GetCanonicalName)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pwszpnpname.as_ptr()), pwszpnpname.len().try_into().unwrap()).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com_StructuredStorage\"`, `\"Win32_System_Variant\"`"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant"))]
    pub unsafe fn GetProperty<P0>(&self, pwszpropname: P0) -> ::windows_core::Result<super::super::System::Com::StructuredStorage::PROPVARIANT>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetProperty)(::windows_core::Interface::as_raw(self), pwszpropname.into_param().abi(), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com_StructuredStorage\"`, `\"Win32_System_Variant\"`"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant"))]
    pub unsafe fn SetProperty<P0>(&self, pwszpropname: P0, pvalue: *const super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        (::windows_core::Interface::vtable(self).SetProperty)(::windows_core::Interface::as_raw(self), pwszpropname.into_param().abi(), pvalue).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com_StructuredStorage\"`, `\"Win32_System_Variant\"`"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant"))]
    pub unsafe fn GetFormatCapability(&self, format: WMDM_FORMATCODE) -> ::windows_core::Result<WMDM_FORMAT_CAPABILITY> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetFormatCapability)(::windows_core::Interface::as_raw(self), format, &mut result__).from_abi(result__)
    }
    pub unsafe fn DeviceIoControl(&self, dwiocontrolcode: u32, lpinbuffer: &[u8], lpoutbuffer: *mut u8, pnoutbuffersize: *mut u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).DeviceIoControl)(::windows_core::Interface::as_raw(self), dwiocontrolcode, ::core::mem::transmute(lpinbuffer.as_ptr()), lpinbuffer.len().try_into().unwrap(), lpoutbuffer, pnoutbuffersize).ok()
    }
    pub unsafe fn FindStorage<P0>(&self, findscope: WMDM_FIND_SCOPE, pwszuniqueid: P0) -> ::windows_core::Result<IWMDMStorage>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).FindStorage)(::windows_core::Interface::as_raw(self), findscope, pwszuniqueid.into_param().abi(), &mut result__).from_abi(result__)
    }
}
::windows_core::imp::interface_hierarchy!(IWMDMDevice3, ::windows_core::IUnknown, IWMDMDevice, IWMDMDevice2);
unsafe impl ::windows_core::Interface for IWMDMDevice3 {
    type Vtable = IWMDMDevice3_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IWMDMDevice3 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x6c03e4fe_05db_4dda_9e3c_06233a6d5d65);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMDMDevice3_Vtbl {
    pub base__: IWMDMDevice2_Vtbl,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant"))]
    pub GetProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwszpropname: ::windows_core::PCWSTR, pvalue: *mut super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant")))]
    GetProperty: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant"))]
    pub SetProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwszpropname: ::windows_core::PCWSTR, pvalue: *const super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant")))]
    SetProperty: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant"))]
    pub GetFormatCapability: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, format: WMDM_FORMATCODE, pformatsupport: *mut WMDM_FORMAT_CAPABILITY) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant")))]
    GetFormatCapability: usize,
    pub DeviceIoControl: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwiocontrolcode: u32, lpinbuffer: *const u8, ninbuffersize: u32, lpoutbuffer: *mut u8, pnoutbuffersize: *mut u32) -> ::windows_core::HRESULT,
    pub FindStorage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, findscope: WMDM_FIND_SCOPE, pwszuniqueid: ::windows_core::PCWSTR, ppstorage: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IWMDMDeviceControl(::windows_core::IUnknown);
impl IWMDMDeviceControl {
    pub unsafe fn GetStatus(&self) -> ::windows_core::Result<u32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetStatus)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetCapabilities(&self) -> ::windows_core::Result<u32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetCapabilities)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn Play(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Play)(::windows_core::Interface::as_raw(self)).ok()
    }
    #[doc = "Required features: `\"Win32_Media_Audio\"`"]
    #[cfg(feature = "Win32_Media_Audio")]
    pub unsafe fn Record(&self, pformat: *const super::Audio::WAVEFORMATEX) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Record)(::windows_core::Interface::as_raw(self), pformat).ok()
    }
    pub unsafe fn Pause(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Pause)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Resume(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Resume)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Stop(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Stop)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Seek(&self, fumode: u32, noffset: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Seek)(::windows_core::Interface::as_raw(self), fumode, noffset).ok()
    }
}
::windows_core::imp::interface_hierarchy!(IWMDMDeviceControl, ::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IWMDMDeviceControl {
    type Vtable = IWMDMDeviceControl_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IWMDMDeviceControl {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x1dcb3a04_33ed_11d3_8470_00c04f79dbc0);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMDMDeviceControl_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub GetStatus: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdwstatus: *mut u32) -> ::windows_core::HRESULT,
    pub GetCapabilities: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdwcapabilitiesmask: *mut u32) -> ::windows_core::HRESULT,
    pub Play: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_Media_Audio")]
    pub Record: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pformat: *const super::Audio::WAVEFORMATEX) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Media_Audio"))]
    Record: usize,
    pub Pause: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub Resume: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub Stop: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub Seek: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, fumode: u32, noffset: i32) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IWMDMDeviceSession(::windows_core::IUnknown);
impl IWMDMDeviceSession {
    pub unsafe fn BeginSession(&self, r#type: WMDM_SESSION_TYPE, pctx: ::core::option::Option<&[u8]>) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).BeginSession)(::windows_core::Interface::as_raw(self), r#type, ::core::mem::transmute(pctx.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())), pctx.as_deref().map_or(0, |slice| slice.len().try_into().unwrap())).ok()
    }
    pub unsafe fn EndSession(&self, r#type: WMDM_SESSION_TYPE, pctx: ::core::option::Option<&[u8]>) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).EndSession)(::windows_core::Interface::as_raw(self), r#type, ::core::mem::transmute(pctx.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())), pctx.as_deref().map_or(0, |slice| slice.len().try_into().unwrap())).ok()
    }
}
::windows_core::imp::interface_hierarchy!(IWMDMDeviceSession, ::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IWMDMDeviceSession {
    type Vtable = IWMDMDeviceSession_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IWMDMDeviceSession {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x82af0a65_9d96_412c_83e5_3c43e4b06cc7);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMDMDeviceSession_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub BeginSession: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, r#type: WMDM_SESSION_TYPE, pctx: *const u8, dwsizectx: u32) -> ::windows_core::HRESULT,
    pub EndSession: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, r#type: WMDM_SESSION_TYPE, pctx: *const u8, dwsizectx: u32) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IWMDMEnumDevice(::windows_core::IUnknown);
impl IWMDMEnumDevice {
    pub unsafe fn Next(&self, ppdevice: &mut [::core::option::Option<IWMDMDevice>], pceltfetched: *mut u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Next)(::windows_core::Interface::as_raw(self), ppdevice.len().try_into().unwrap(), ::core::mem::transmute(ppdevice.as_ptr()), pceltfetched).ok()
    }
    pub unsafe fn Skip(&self, celt: u32) -> ::windows_core::Result<u32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).Skip)(::windows_core::Interface::as_raw(self), celt, &mut result__).from_abi(result__)
    }
    pub unsafe fn Reset(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Reset)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Clone(&self) -> ::windows_core::Result<IWMDMEnumDevice> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).Clone)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
::windows_core::imp::interface_hierarchy!(IWMDMEnumDevice, ::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IWMDMEnumDevice {
    type Vtable = IWMDMEnumDevice_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IWMDMEnumDevice {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x1dcb3a01_33ed_11d3_8470_00c04f79dbc0);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMDMEnumDevice_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub Next: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, celt: u32, ppdevice: *mut *mut ::core::ffi::c_void, pceltfetched: *mut u32) -> ::windows_core::HRESULT,
    pub Skip: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, celt: u32, pceltfetched: *mut u32) -> ::windows_core::HRESULT,
    pub Reset: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub Clone: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppenumdevice: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IWMDMEnumStorage(::windows_core::IUnknown);
impl IWMDMEnumStorage {
    pub unsafe fn Next(&self, ppstorage: &mut [::core::option::Option<IWMDMStorage>], pceltfetched: *mut u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Next)(::windows_core::Interface::as_raw(self), ppstorage.len().try_into().unwrap(), ::core::mem::transmute(ppstorage.as_ptr()), pceltfetched).ok()
    }
    pub unsafe fn Skip(&self, celt: u32) -> ::windows_core::Result<u32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).Skip)(::windows_core::Interface::as_raw(self), celt, &mut result__).from_abi(result__)
    }
    pub unsafe fn Reset(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Reset)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Clone(&self) -> ::windows_core::Result<IWMDMEnumStorage> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).Clone)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
::windows_core::imp::interface_hierarchy!(IWMDMEnumStorage, ::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IWMDMEnumStorage {
    type Vtable = IWMDMEnumStorage_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IWMDMEnumStorage {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x1dcb3a05_33ed_11d3_8470_00c04f79dbc0);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMDMEnumStorage_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub Next: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, celt: u32, ppstorage: *mut *mut ::core::ffi::c_void, pceltfetched: *mut u32) -> ::windows_core::HRESULT,
    pub Skip: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, celt: u32, pceltfetched: *mut u32) -> ::windows_core::HRESULT,
    pub Reset: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub Clone: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppenumstorage: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IWMDMLogger(::windows_core::IUnknown);
impl IWMDMLogger {
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsEnabled(&self) -> ::windows_core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).IsEnabled)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Enable<P0>(&self, fenable: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::BOOL>,
    {
        (::windows_core::Interface::vtable(self).Enable)(::windows_core::Interface::as_raw(self), fenable.into_param().abi()).ok()
    }
    pub unsafe fn GetLogFileName(&self, pszfilename: ::windows_core::PSTR, nmaxchars: u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetLogFileName)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pszfilename), nmaxchars).ok()
    }
    pub unsafe fn SetLogFileName<P0>(&self, pszfilename: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCSTR>,
    {
        (::windows_core::Interface::vtable(self).SetLogFileName)(::windows_core::Interface::as_raw(self), pszfilename.into_param().abi()).ok()
    }
    pub unsafe fn LogString<P0, P1>(&self, dwflags: u32, pszsrcname: P0, pszlog: P1) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCSTR>,
        P1: ::windows_core::IntoParam<::windows_core::PCSTR>,
    {
        (::windows_core::Interface::vtable(self).LogString)(::windows_core::Interface::as_raw(self), dwflags, pszsrcname.into_param().abi(), pszlog.into_param().abi()).ok()
    }
    pub unsafe fn LogDword<P0, P1>(&self, dwflags: u32, pszsrcname: P0, pszlogformat: P1, dwlog: u32) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCSTR>,
        P1: ::windows_core::IntoParam<::windows_core::PCSTR>,
    {
        (::windows_core::Interface::vtable(self).LogDword)(::windows_core::Interface::as_raw(self), dwflags, pszsrcname.into_param().abi(), pszlogformat.into_param().abi(), dwlog).ok()
    }
    pub unsafe fn Reset(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Reset)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn GetSizeParams(&self, pdwmaxsize: *mut u32, pdwshrinktosize: *mut u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetSizeParams)(::windows_core::Interface::as_raw(self), pdwmaxsize, pdwshrinktosize).ok()
    }
    pub unsafe fn SetSizeParams(&self, dwmaxsize: u32, dwshrinktosize: u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetSizeParams)(::windows_core::Interface::as_raw(self), dwmaxsize, dwshrinktosize).ok()
    }
}
::windows_core::imp::interface_hierarchy!(IWMDMLogger, ::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IWMDMLogger {
    type Vtable = IWMDMLogger_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IWMDMLogger {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x110a3200_5a79_11d3_8d78_444553540000);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMDMLogger_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub IsEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pfenabled: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    IsEnabled: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub Enable: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, fenable: super::super::Foundation::BOOL) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Enable: usize,
    pub GetLogFileName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszfilename: ::windows_core::PSTR, nmaxchars: u32) -> ::windows_core::HRESULT,
    pub SetLogFileName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszfilename: ::windows_core::PCSTR) -> ::windows_core::HRESULT,
    pub LogString: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwflags: u32, pszsrcname: ::windows_core::PCSTR, pszlog: ::windows_core::PCSTR) -> ::windows_core::HRESULT,
    pub LogDword: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwflags: u32, pszsrcname: ::windows_core::PCSTR, pszlogformat: ::windows_core::PCSTR, dwlog: u32) -> ::windows_core::HRESULT,
    pub Reset: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub GetSizeParams: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdwmaxsize: *mut u32, pdwshrinktosize: *mut u32) -> ::windows_core::HRESULT,
    pub SetSizeParams: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwmaxsize: u32, dwshrinktosize: u32) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IWMDMMetaData(::windows_core::IUnknown);
impl IWMDMMetaData {
    pub unsafe fn AddItem<P0>(&self, r#type: WMDM_TAG_DATATYPE, pwsztagname: P0, pvalue: ::core::option::Option<&[u8]>) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        (::windows_core::Interface::vtable(self).AddItem)(::windows_core::Interface::as_raw(self), r#type, pwsztagname.into_param().abi(), ::core::mem::transmute(pvalue.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())), pvalue.as_deref().map_or(0, |slice| slice.len().try_into().unwrap())).ok()
    }
    pub unsafe fn QueryByName<P0>(&self, pwsztagname: P0, ptype: *mut WMDM_TAG_DATATYPE, pvalue: *mut *mut u8, pcblength: *mut u32) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        (::windows_core::Interface::vtable(self).QueryByName)(::windows_core::Interface::as_raw(self), pwsztagname.into_param().abi(), ptype, pvalue, pcblength).ok()
    }
    pub unsafe fn QueryByIndex(&self, iindex: u32, ppwszname: *mut *mut u16, ptype: *mut WMDM_TAG_DATATYPE, ppvalue: *mut *mut u8, pcblength: *mut u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).QueryByIndex)(::windows_core::Interface::as_raw(self), iindex, ppwszname, ptype, ppvalue, pcblength).ok()
    }
    pub unsafe fn GetItemCount(&self) -> ::windows_core::Result<u32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetItemCount)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
::windows_core::imp::interface_hierarchy!(IWMDMMetaData, ::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IWMDMMetaData {
    type Vtable = IWMDMMetaData_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IWMDMMetaData {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xec3b0663_0951_460a_9a80_0dceed3c043c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMDMMetaData_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub AddItem: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, r#type: WMDM_TAG_DATATYPE, pwsztagname: ::windows_core::PCWSTR, pvalue: *const u8, ilength: u32) -> ::windows_core::HRESULT,
    pub QueryByName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwsztagname: ::windows_core::PCWSTR, ptype: *mut WMDM_TAG_DATATYPE, pvalue: *mut *mut u8, pcblength: *mut u32) -> ::windows_core::HRESULT,
    pub QueryByIndex: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iindex: u32, ppwszname: *mut *mut u16, ptype: *mut WMDM_TAG_DATATYPE, ppvalue: *mut *mut u8, pcblength: *mut u32) -> ::windows_core::HRESULT,
    pub GetItemCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, icount: *mut u32) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IWMDMNotification(::windows_core::IUnknown);
impl IWMDMNotification {
    pub unsafe fn WMDMMessage<P0>(&self, dwmessagetype: u32, pwszcanonicalname: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        (::windows_core::Interface::vtable(self).WMDMMessage)(::windows_core::Interface::as_raw(self), dwmessagetype, pwszcanonicalname.into_param().abi()).ok()
    }
}
::windows_core::imp::interface_hierarchy!(IWMDMNotification, ::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IWMDMNotification {
    type Vtable = IWMDMNotification_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IWMDMNotification {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x3f5e95c0_0f43_4ed4_93d2_c89a45d59b81);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMDMNotification_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub WMDMMessage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwmessagetype: u32, pwszcanonicalname: ::windows_core::PCWSTR) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IWMDMObjectInfo(::windows_core::IUnknown);
impl IWMDMObjectInfo {
    pub unsafe fn GetPlayLength(&self) -> ::windows_core::Result<u32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetPlayLength)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetPlayLength(&self, dwlength: u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetPlayLength)(::windows_core::Interface::as_raw(self), dwlength).ok()
    }
    pub unsafe fn GetPlayOffset(&self) -> ::windows_core::Result<u32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetPlayOffset)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetPlayOffset(&self, dwoffset: u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetPlayOffset)(::windows_core::Interface::as_raw(self), dwoffset).ok()
    }
    pub unsafe fn GetTotalLength(&self) -> ::windows_core::Result<u32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetTotalLength)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetLastPlayPosition(&self) -> ::windows_core::Result<u32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetLastPlayPosition)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetLongestPlayPosition(&self) -> ::windows_core::Result<u32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetLongestPlayPosition)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
::windows_core::imp::interface_hierarchy!(IWMDMObjectInfo, ::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IWMDMObjectInfo {
    type Vtable = IWMDMObjectInfo_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IWMDMObjectInfo {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x1dcb3a09_33ed_11d3_8470_00c04f79dbc0);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMDMObjectInfo_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub GetPlayLength: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdwlength: *mut u32) -> ::windows_core::HRESULT,
    pub SetPlayLength: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwlength: u32) -> ::windows_core::HRESULT,
    pub GetPlayOffset: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdwoffset: *mut u32) -> ::windows_core::HRESULT,
    pub SetPlayOffset: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwoffset: u32) -> ::windows_core::HRESULT,
    pub GetTotalLength: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdwlength: *mut u32) -> ::windows_core::HRESULT,
    pub GetLastPlayPosition: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdwlastpos: *mut u32) -> ::windows_core::HRESULT,
    pub GetLongestPlayPosition: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdwlongestpos: *mut u32) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IWMDMOperation(::windows_core::IUnknown);
impl IWMDMOperation {
    pub unsafe fn BeginRead(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).BeginRead)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn BeginWrite(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).BeginWrite)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn GetObjectName(&self, pwszname: &mut [u16]) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetObjectName)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pwszname.as_ptr()), pwszname.len().try_into().unwrap()).ok()
    }
    pub unsafe fn SetObjectName(&self, pwszname: &[u16]) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetObjectName)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pwszname.as_ptr()), pwszname.len().try_into().unwrap()).ok()
    }
    #[doc = "Required features: `\"Win32_Media_Audio\"`"]
    #[cfg(feature = "Win32_Media_Audio")]
    pub unsafe fn GetObjectAttributes(&self, pdwattributes: *mut u32, pformat: ::core::option::Option<*mut super::Audio::WAVEFORMATEX>) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetObjectAttributes)(::windows_core::Interface::as_raw(self), pdwattributes, ::core::mem::transmute(pformat.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    #[doc = "Required features: `\"Win32_Media_Audio\"`"]
    #[cfg(feature = "Win32_Media_Audio")]
    pub unsafe fn SetObjectAttributes(&self, dwattributes: u32, pformat: ::core::option::Option<*const super::Audio::WAVEFORMATEX>) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetObjectAttributes)(::windows_core::Interface::as_raw(self), dwattributes, ::core::mem::transmute(pformat.unwrap_or(::std::ptr::null()))).ok()
    }
    pub unsafe fn GetObjectTotalSize(&self, pdwsize: *mut u32, pdwsizehigh: *mut u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetObjectTotalSize)(::windows_core::Interface::as_raw(self), pdwsize, pdwsizehigh).ok()
    }
    pub unsafe fn SetObjectTotalSize(&self, dwsize: u32, dwsizehigh: u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetObjectTotalSize)(::windows_core::Interface::as_raw(self), dwsize, dwsizehigh).ok()
    }
    pub unsafe fn TransferObjectData(&self, pdata: *mut u8, pdwsize: *mut u32, abmac: &mut [u8; 8]) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).TransferObjectData)(::windows_core::Interface::as_raw(self), pdata, pdwsize, ::core::mem::transmute(abmac.as_ptr())).ok()
    }
    pub unsafe fn End<P0>(&self, phcompletioncode: *const ::windows_core::HRESULT, pnewobject: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::IUnknown>,
    {
        (::windows_core::Interface::vtable(self).End)(::windows_core::Interface::as_raw(self), phcompletioncode, pnewobject.into_param().abi()).ok()
    }
}
::windows_core::imp::interface_hierarchy!(IWMDMOperation, ::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IWMDMOperation {
    type Vtable = IWMDMOperation_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IWMDMOperation {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x1dcb3a0b_33ed_11d3_8470_00c04f79dbc0);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMDMOperation_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub BeginRead: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub BeginWrite: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub GetObjectName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwszname: ::windows_core::PWSTR, nmaxchars: u32) -> ::windows_core::HRESULT,
    pub SetObjectName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwszname: ::windows_core::PCWSTR, nmaxchars: u32) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_Media_Audio")]
    pub GetObjectAttributes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdwattributes: *mut u32, pformat: *mut super::Audio::WAVEFORMATEX) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Media_Audio"))]
    GetObjectAttributes: usize,
    #[cfg(feature = "Win32_Media_Audio")]
    pub SetObjectAttributes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwattributes: u32, pformat: *const super::Audio::WAVEFORMATEX) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Media_Audio"))]
    SetObjectAttributes: usize,
    pub GetObjectTotalSize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdwsize: *mut u32, pdwsizehigh: *mut u32) -> ::windows_core::HRESULT,
    pub SetObjectTotalSize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwsize: u32, dwsizehigh: u32) -> ::windows_core::HRESULT,
    pub TransferObjectData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdata: *mut u8, pdwsize: *mut u32, abmac: *mut u8) -> ::windows_core::HRESULT,
    pub End: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, phcompletioncode: *const ::windows_core::HRESULT, pnewobject: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IWMDMOperation2(::windows_core::IUnknown);
impl IWMDMOperation2 {
    pub unsafe fn BeginRead(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.BeginRead)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn BeginWrite(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.BeginWrite)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn GetObjectName(&self, pwszname: &mut [u16]) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.GetObjectName)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pwszname.as_ptr()), pwszname.len().try_into().unwrap()).ok()
    }
    pub unsafe fn SetObjectName(&self, pwszname: &[u16]) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.SetObjectName)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pwszname.as_ptr()), pwszname.len().try_into().unwrap()).ok()
    }
    #[doc = "Required features: `\"Win32_Media_Audio\"`"]
    #[cfg(feature = "Win32_Media_Audio")]
    pub unsafe fn GetObjectAttributes(&self, pdwattributes: *mut u32, pformat: ::core::option::Option<*mut super::Audio::WAVEFORMATEX>) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.GetObjectAttributes)(::windows_core::Interface::as_raw(self), pdwattributes, ::core::mem::transmute(pformat.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    #[doc = "Required features: `\"Win32_Media_Audio\"`"]
    #[cfg(feature = "Win32_Media_Audio")]
    pub unsafe fn SetObjectAttributes(&self, dwattributes: u32, pformat: ::core::option::Option<*const super::Audio::WAVEFORMATEX>) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.SetObjectAttributes)(::windows_core::Interface::as_raw(self), dwattributes, ::core::mem::transmute(pformat.unwrap_or(::std::ptr::null()))).ok()
    }
    pub unsafe fn GetObjectTotalSize(&self, pdwsize: *mut u32, pdwsizehigh: *mut u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.GetObjectTotalSize)(::windows_core::Interface::as_raw(self), pdwsize, pdwsizehigh).ok()
    }
    pub unsafe fn SetObjectTotalSize(&self, dwsize: u32, dwsizehigh: u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.SetObjectTotalSize)(::windows_core::Interface::as_raw(self), dwsize, dwsizehigh).ok()
    }
    pub unsafe fn TransferObjectData(&self, pdata: *mut u8, pdwsize: *mut u32, abmac: &mut [u8; 8]) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.TransferObjectData)(::windows_core::Interface::as_raw(self), pdata, pdwsize, ::core::mem::transmute(abmac.as_ptr())).ok()
    }
    pub unsafe fn End<P0>(&self, phcompletioncode: *const ::windows_core::HRESULT, pnewobject: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::IUnknown>,
    {
        (::windows_core::Interface::vtable(self).base__.End)(::windows_core::Interface::as_raw(self), phcompletioncode, pnewobject.into_param().abi()).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`, `\"Win32_Media_Audio\"`, `\"Win32_Media_MediaFoundation\"`"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_Media_Audio", feature = "Win32_Media_MediaFoundation"))]
    pub unsafe fn SetObjectAttributes2(&self, dwattributes: u32, dwattributesex: u32, pformat: ::core::option::Option<*const super::Audio::WAVEFORMATEX>, pvideoformat: ::core::option::Option<*const super::MediaFoundation::VIDEOINFOHEADER>) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetObjectAttributes2)(::windows_core::Interface::as_raw(self), dwattributes, dwattributesex, ::core::mem::transmute(pformat.unwrap_or(::std::ptr::null())), ::core::mem::transmute(pvideoformat.unwrap_or(::std::ptr::null()))).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`, `\"Win32_Media_Audio\"`, `\"Win32_Media_MediaFoundation\"`"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_Media_Audio", feature = "Win32_Media_MediaFoundation"))]
    pub unsafe fn GetObjectAttributes2(&self, pdwattributes: *mut u32, pdwattributesex: *mut u32, paudioformat: ::core::option::Option<*mut super::Audio::WAVEFORMATEX>, pvideoformat: ::core::option::Option<*mut super::MediaFoundation::VIDEOINFOHEADER>) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetObjectAttributes2)(::windows_core::Interface::as_raw(self), pdwattributes, pdwattributesex, ::core::mem::transmute(paudioformat.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(pvideoformat.unwrap_or(::std::ptr::null_mut()))).ok()
    }
}
::windows_core::imp::interface_hierarchy!(IWMDMOperation2, ::windows_core::IUnknown, IWMDMOperation);
unsafe impl ::windows_core::Interface for IWMDMOperation2 {
    type Vtable = IWMDMOperation2_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IWMDMOperation2 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x33445b48_7df7_425c_ad8f_0fc6d82f9f75);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMDMOperation2_Vtbl {
    pub base__: IWMDMOperation_Vtbl,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_Media_Audio", feature = "Win32_Media_MediaFoundation"))]
    pub SetObjectAttributes2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwattributes: u32, dwattributesex: u32, pformat: *const super::Audio::WAVEFORMATEX, pvideoformat: *const super::MediaFoundation::VIDEOINFOHEADER) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_Media_Audio", feature = "Win32_Media_MediaFoundation")))]
    SetObjectAttributes2: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_Media_Audio", feature = "Win32_Media_MediaFoundation"))]
    pub GetObjectAttributes2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdwattributes: *mut u32, pdwattributesex: *mut u32, paudioformat: *mut super::Audio::WAVEFORMATEX, pvideoformat: *mut super::MediaFoundation::VIDEOINFOHEADER) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_Media_Audio", feature = "Win32_Media_MediaFoundation")))]
    GetObjectAttributes2: usize,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IWMDMOperation3(::windows_core::IUnknown);
impl IWMDMOperation3 {
    pub unsafe fn BeginRead(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.BeginRead)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn BeginWrite(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.BeginWrite)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn GetObjectName(&self, pwszname: &mut [u16]) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.GetObjectName)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pwszname.as_ptr()), pwszname.len().try_into().unwrap()).ok()
    }
    pub unsafe fn SetObjectName(&self, pwszname: &[u16]) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.SetObjectName)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pwszname.as_ptr()), pwszname.len().try_into().unwrap()).ok()
    }
    #[doc = "Required features: `\"Win32_Media_Audio\"`"]
    #[cfg(feature = "Win32_Media_Audio")]
    pub unsafe fn GetObjectAttributes(&self, pdwattributes: *mut u32, pformat: ::core::option::Option<*mut super::Audio::WAVEFORMATEX>) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.GetObjectAttributes)(::windows_core::Interface::as_raw(self), pdwattributes, ::core::mem::transmute(pformat.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    #[doc = "Required features: `\"Win32_Media_Audio\"`"]
    #[cfg(feature = "Win32_Media_Audio")]
    pub unsafe fn SetObjectAttributes(&self, dwattributes: u32, pformat: ::core::option::Option<*const super::Audio::WAVEFORMATEX>) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.SetObjectAttributes)(::windows_core::Interface::as_raw(self), dwattributes, ::core::mem::transmute(pformat.unwrap_or(::std::ptr::null()))).ok()
    }
    pub unsafe fn GetObjectTotalSize(&self, pdwsize: *mut u32, pdwsizehigh: *mut u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.GetObjectTotalSize)(::windows_core::Interface::as_raw(self), pdwsize, pdwsizehigh).ok()
    }
    pub unsafe fn SetObjectTotalSize(&self, dwsize: u32, dwsizehigh: u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.SetObjectTotalSize)(::windows_core::Interface::as_raw(self), dwsize, dwsizehigh).ok()
    }
    pub unsafe fn TransferObjectData(&self, pdata: *mut u8, pdwsize: *mut u32, abmac: &mut [u8; 8]) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.TransferObjectData)(::windows_core::Interface::as_raw(self), pdata, pdwsize, ::core::mem::transmute(abmac.as_ptr())).ok()
    }
    pub unsafe fn End<P0>(&self, phcompletioncode: *const ::windows_core::HRESULT, pnewobject: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::IUnknown>,
    {
        (::windows_core::Interface::vtable(self).base__.End)(::windows_core::Interface::as_raw(self), phcompletioncode, pnewobject.into_param().abi()).ok()
    }
    pub unsafe fn TransferObjectDataOnClearChannel(&self, pdata: *mut u8, pdwsize: *mut u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).TransferObjectDataOnClearChannel)(::windows_core::Interface::as_raw(self), pdata, pdwsize).ok()
    }
}
::windows_core::imp::interface_hierarchy!(IWMDMOperation3, ::windows_core::IUnknown, IWMDMOperation);
unsafe impl ::windows_core::Interface for IWMDMOperation3 {
    type Vtable = IWMDMOperation3_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IWMDMOperation3 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xd1f9b46a_9ca8_46d8_9d0f_1ec9bae54919);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMDMOperation3_Vtbl {
    pub base__: IWMDMOperation_Vtbl,
    pub TransferObjectDataOnClearChannel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdata: *mut u8, pdwsize: *mut u32) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IWMDMProgress(::windows_core::IUnknown);
impl IWMDMProgress {
    pub unsafe fn Begin(&self, dwestimatedticks: u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Begin)(::windows_core::Interface::as_raw(self), dwestimatedticks).ok()
    }
    pub unsafe fn Progress(&self, dwtranspiredticks: u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Progress)(::windows_core::Interface::as_raw(self), dwtranspiredticks).ok()
    }
    pub unsafe fn End(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).End)(::windows_core::Interface::as_raw(self)).ok()
    }
}
::windows_core::imp::interface_hierarchy!(IWMDMProgress, ::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IWMDMProgress {
    type Vtable = IWMDMProgress_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IWMDMProgress {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x1dcb3a0c_33ed_11d3_8470_00c04f79dbc0);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMDMProgress_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub Begin: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwestimatedticks: u32) -> ::windows_core::HRESULT,
    pub Progress: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwtranspiredticks: u32) -> ::windows_core::HRESULT,
    pub End: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IWMDMProgress2(::windows_core::IUnknown);
impl IWMDMProgress2 {
    pub unsafe fn Begin(&self, dwestimatedticks: u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.Begin)(::windows_core::Interface::as_raw(self), dwestimatedticks).ok()
    }
    pub unsafe fn Progress(&self, dwtranspiredticks: u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.Progress)(::windows_core::Interface::as_raw(self), dwtranspiredticks).ok()
    }
    pub unsafe fn End(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.End)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn End2(&self, hrcompletioncode: ::windows_core::HRESULT) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).End2)(::windows_core::Interface::as_raw(self), hrcompletioncode).ok()
    }
}
::windows_core::imp::interface_hierarchy!(IWMDMProgress2, ::windows_core::IUnknown, IWMDMProgress);
unsafe impl ::windows_core::Interface for IWMDMProgress2 {
    type Vtable = IWMDMProgress2_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IWMDMProgress2 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x3a43f550_b383_4e92_b04a_e6bbc660fefc);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMDMProgress2_Vtbl {
    pub base__: IWMDMProgress_Vtbl,
    pub End2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hrcompletioncode: ::windows_core::HRESULT) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IWMDMProgress3(::windows_core::IUnknown);
impl IWMDMProgress3 {
    pub unsafe fn Begin(&self, dwestimatedticks: u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.Begin)(::windows_core::Interface::as_raw(self), dwestimatedticks).ok()
    }
    pub unsafe fn Progress(&self, dwtranspiredticks: u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.Progress)(::windows_core::Interface::as_raw(self), dwtranspiredticks).ok()
    }
    pub unsafe fn End(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.End)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn End2(&self, hrcompletioncode: ::windows_core::HRESULT) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.End2)(::windows_core::Interface::as_raw(self), hrcompletioncode).ok()
    }
    pub unsafe fn Begin3(&self, eventid: ::windows_core::GUID, dwestimatedticks: u32, pcontext: ::core::option::Option<*mut OPAQUECOMMAND>) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Begin3)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(eventid), dwestimatedticks, ::core::mem::transmute(pcontext.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    pub unsafe fn Progress3(&self, eventid: ::windows_core::GUID, dwtranspiredticks: u32, pcontext: ::core::option::Option<*mut OPAQUECOMMAND>) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Progress3)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(eventid), dwtranspiredticks, ::core::mem::transmute(pcontext.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    pub unsafe fn End3(&self, eventid: ::windows_core::GUID, hrcompletioncode: ::windows_core::HRESULT, pcontext: ::core::option::Option<*mut OPAQUECOMMAND>) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).End3)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(eventid), hrcompletioncode, ::core::mem::transmute(pcontext.unwrap_or(::std::ptr::null_mut()))).ok()
    }
}
::windows_core::imp::interface_hierarchy!(IWMDMProgress3, ::windows_core::IUnknown, IWMDMProgress, IWMDMProgress2);
unsafe impl ::windows_core::Interface for IWMDMProgress3 {
    type Vtable = IWMDMProgress3_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IWMDMProgress3 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x21de01cb_3bb4_4929_b21a_17af3f80f658);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMDMProgress3_Vtbl {
    pub base__: IWMDMProgress2_Vtbl,
    pub Begin3: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, eventid: ::windows_core::GUID, dwestimatedticks: u32, pcontext: *mut OPAQUECOMMAND) -> ::windows_core::HRESULT,
    pub Progress3: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, eventid: ::windows_core::GUID, dwtranspiredticks: u32, pcontext: *mut OPAQUECOMMAND) -> ::windows_core::HRESULT,
    pub End3: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, eventid: ::windows_core::GUID, hrcompletioncode: ::windows_core::HRESULT, pcontext: *mut OPAQUECOMMAND) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IWMDMRevoked(::windows_core::IUnknown);
impl IWMDMRevoked {
    pub unsafe fn GetRevocationURL(&self, ppwszrevocationurl: *mut ::windows_core::PWSTR, pdwbufferlen: *mut u32, pdwrevokedbitflag: *mut u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetRevocationURL)(::windows_core::Interface::as_raw(self), ppwszrevocationurl, pdwbufferlen, pdwrevokedbitflag).ok()
    }
}
::windows_core::imp::interface_hierarchy!(IWMDMRevoked, ::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IWMDMRevoked {
    type Vtable = IWMDMRevoked_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IWMDMRevoked {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xebeccedb_88ee_4e55_b6a4_8d9f07d696aa);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMDMRevoked_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub GetRevocationURL: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppwszrevocationurl: *mut ::windows_core::PWSTR, pdwbufferlen: *mut u32, pdwrevokedbitflag: *mut u32) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IWMDMStorage(::windows_core::IUnknown);
impl IWMDMStorage {
    #[doc = "Required features: `\"Win32_Media_Audio\"`"]
    #[cfg(feature = "Win32_Media_Audio")]
    pub unsafe fn SetAttributes(&self, dwattributes: u32, pformat: ::core::option::Option<*const super::Audio::WAVEFORMATEX>) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetAttributes)(::windows_core::Interface::as_raw(self), dwattributes, ::core::mem::transmute(pformat.unwrap_or(::std::ptr::null()))).ok()
    }
    pub unsafe fn GetStorageGlobals(&self) -> ::windows_core::Result<IWMDMStorageGlobals> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetStorageGlobals)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Media_Audio\"`"]
    #[cfg(feature = "Win32_Media_Audio")]
    pub unsafe fn GetAttributes(&self, pdwattributes: *mut u32, pformat: ::core::option::Option<*mut super::Audio::WAVEFORMATEX>) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetAttributes)(::windows_core::Interface::as_raw(self), pdwattributes, ::core::mem::transmute(pformat.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    pub unsafe fn GetName(&self, pwszname: &mut [u16]) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetName)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pwszname.as_ptr()), pwszname.len().try_into().unwrap()).ok()
    }
    pub unsafe fn GetDate(&self) -> ::windows_core::Result<WMDMDATETIME> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetDate)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetSize(&self, pdwsizelow: *mut u32, pdwsizehigh: *mut u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetSize)(::windows_core::Interface::as_raw(self), pdwsizelow, pdwsizehigh).ok()
    }
    pub unsafe fn GetRights(&self, pprights: *mut *mut WMDMRIGHTS, pnrightscount: *mut u32, abmac: &mut [u8; 8]) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetRights)(::windows_core::Interface::as_raw(self), pprights, pnrightscount, ::core::mem::transmute(abmac.as_ptr())).ok()
    }
    pub unsafe fn EnumStorage(&self) -> ::windows_core::Result<IWMDMEnumStorage> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).EnumStorage)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SendOpaqueCommand(&self, pcommand: *mut OPAQUECOMMAND) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SendOpaqueCommand)(::windows_core::Interface::as_raw(self), pcommand).ok()
    }
}
::windows_core::imp::interface_hierarchy!(IWMDMStorage, ::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IWMDMStorage {
    type Vtable = IWMDMStorage_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IWMDMStorage {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x1dcb3a06_33ed_11d3_8470_00c04f79dbc0);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMDMStorage_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_Media_Audio")]
    pub SetAttributes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwattributes: u32, pformat: *const super::Audio::WAVEFORMATEX) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Media_Audio"))]
    SetAttributes: usize,
    pub GetStorageGlobals: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppstorageglobals: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_Media_Audio")]
    pub GetAttributes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdwattributes: *mut u32, pformat: *mut super::Audio::WAVEFORMATEX) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Media_Audio"))]
    GetAttributes: usize,
    pub GetName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwszname: ::windows_core::PWSTR, nmaxchars: u32) -> ::windows_core::HRESULT,
    pub GetDate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdatetimeutc: *mut WMDMDATETIME) -> ::windows_core::HRESULT,
    pub GetSize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdwsizelow: *mut u32, pdwsizehigh: *mut u32) -> ::windows_core::HRESULT,
    pub GetRights: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pprights: *mut *mut WMDMRIGHTS, pnrightscount: *mut u32, abmac: *mut u8) -> ::windows_core::HRESULT,
    pub EnumStorage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, penumstorage: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub SendOpaqueCommand: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcommand: *mut OPAQUECOMMAND) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IWMDMStorage2(::windows_core::IUnknown);
impl IWMDMStorage2 {
    #[doc = "Required features: `\"Win32_Media_Audio\"`"]
    #[cfg(feature = "Win32_Media_Audio")]
    pub unsafe fn SetAttributes(&self, dwattributes: u32, pformat: ::core::option::Option<*const super::Audio::WAVEFORMATEX>) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.SetAttributes)(::windows_core::Interface::as_raw(self), dwattributes, ::core::mem::transmute(pformat.unwrap_or(::std::ptr::null()))).ok()
    }
    pub unsafe fn GetStorageGlobals(&self) -> ::windows_core::Result<IWMDMStorageGlobals> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetStorageGlobals)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Media_Audio\"`"]
    #[cfg(feature = "Win32_Media_Audio")]
    pub unsafe fn GetAttributes(&self, pdwattributes: *mut u32, pformat: ::core::option::Option<*mut super::Audio::WAVEFORMATEX>) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.GetAttributes)(::windows_core::Interface::as_raw(self), pdwattributes, ::core::mem::transmute(pformat.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    pub unsafe fn GetName(&self, pwszname: &mut [u16]) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.GetName)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pwszname.as_ptr()), pwszname.len().try_into().unwrap()).ok()
    }
    pub unsafe fn GetDate(&self) -> ::windows_core::Result<WMDMDATETIME> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetDate)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetSize(&self, pdwsizelow: *mut u32, pdwsizehigh: *mut u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.GetSize)(::windows_core::Interface::as_raw(self), pdwsizelow, pdwsizehigh).ok()
    }
    pub unsafe fn GetRights(&self, pprights: *mut *mut WMDMRIGHTS, pnrightscount: *mut u32, abmac: &mut [u8; 8]) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.GetRights)(::windows_core::Interface::as_raw(self), pprights, pnrightscount, ::core::mem::transmute(abmac.as_ptr())).ok()
    }
    pub unsafe fn EnumStorage(&self) -> ::windows_core::Result<IWMDMEnumStorage> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.EnumStorage)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SendOpaqueCommand(&self, pcommand: *mut OPAQUECOMMAND) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.SendOpaqueCommand)(::windows_core::Interface::as_raw(self), pcommand).ok()
    }
    pub unsafe fn GetStorage<P0>(&self, pszstoragename: P0) -> ::windows_core::Result<IWMDMStorage>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetStorage)(::windows_core::Interface::as_raw(self), pszstoragename.into_param().abi(), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`, `\"Win32_Media_Audio\"`, `\"Win32_Media_MediaFoundation\"`"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_Media_Audio", feature = "Win32_Media_MediaFoundation"))]
    pub unsafe fn SetAttributes2(&self, dwattributes: u32, dwattributesex: u32, pformat: ::core::option::Option<*const super::Audio::WAVEFORMATEX>, pvideoformat: ::core::option::Option<*const super::MediaFoundation::VIDEOINFOHEADER>) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetAttributes2)(::windows_core::Interface::as_raw(self), dwattributes, dwattributesex, ::core::mem::transmute(pformat.unwrap_or(::std::ptr::null())), ::core::mem::transmute(pvideoformat.unwrap_or(::std::ptr::null()))).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`, `\"Win32_Media_Audio\"`, `\"Win32_Media_MediaFoundation\"`"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_Media_Audio", feature = "Win32_Media_MediaFoundation"))]
    pub unsafe fn GetAttributes2(&self, pdwattributes: *mut u32, pdwattributesex: *mut u32, paudioformat: ::core::option::Option<*mut super::Audio::WAVEFORMATEX>, pvideoformat: ::core::option::Option<*mut super::MediaFoundation::VIDEOINFOHEADER>) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetAttributes2)(::windows_core::Interface::as_raw(self), pdwattributes, pdwattributesex, ::core::mem::transmute(paudioformat.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(pvideoformat.unwrap_or(::std::ptr::null_mut()))).ok()
    }
}
::windows_core::imp::interface_hierarchy!(IWMDMStorage2, ::windows_core::IUnknown, IWMDMStorage);
unsafe impl ::windows_core::Interface for IWMDMStorage2 {
    type Vtable = IWMDMStorage2_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IWMDMStorage2 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x1ed5a144_5cd5_4683_9eff_72cbdb2d9533);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMDMStorage2_Vtbl {
    pub base__: IWMDMStorage_Vtbl,
    pub GetStorage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszstoragename: ::windows_core::PCWSTR, ppstorage: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_Media_Audio", feature = "Win32_Media_MediaFoundation"))]
    pub SetAttributes2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwattributes: u32, dwattributesex: u32, pformat: *const super::Audio::WAVEFORMATEX, pvideoformat: *const super::MediaFoundation::VIDEOINFOHEADER) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_Media_Audio", feature = "Win32_Media_MediaFoundation")))]
    SetAttributes2: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_Media_Audio", feature = "Win32_Media_MediaFoundation"))]
    pub GetAttributes2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdwattributes: *mut u32, pdwattributesex: *mut u32, paudioformat: *mut super::Audio::WAVEFORMATEX, pvideoformat: *mut super::MediaFoundation::VIDEOINFOHEADER) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_Media_Audio", feature = "Win32_Media_MediaFoundation")))]
    GetAttributes2: usize,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IWMDMStorage3(::windows_core::IUnknown);
impl IWMDMStorage3 {
    #[doc = "Required features: `\"Win32_Media_Audio\"`"]
    #[cfg(feature = "Win32_Media_Audio")]
    pub unsafe fn SetAttributes(&self, dwattributes: u32, pformat: ::core::option::Option<*const super::Audio::WAVEFORMATEX>) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.SetAttributes)(::windows_core::Interface::as_raw(self), dwattributes, ::core::mem::transmute(pformat.unwrap_or(::std::ptr::null()))).ok()
    }
    pub unsafe fn GetStorageGlobals(&self) -> ::windows_core::Result<IWMDMStorageGlobals> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.GetStorageGlobals)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Media_Audio\"`"]
    #[cfg(feature = "Win32_Media_Audio")]
    pub unsafe fn GetAttributes(&self, pdwattributes: *mut u32, pformat: ::core::option::Option<*mut super::Audio::WAVEFORMATEX>) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.GetAttributes)(::windows_core::Interface::as_raw(self), pdwattributes, ::core::mem::transmute(pformat.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    pub unsafe fn GetName(&self, pwszname: &mut [u16]) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.GetName)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pwszname.as_ptr()), pwszname.len().try_into().unwrap()).ok()
    }
    pub unsafe fn GetDate(&self) -> ::windows_core::Result<WMDMDATETIME> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.GetDate)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetSize(&self, pdwsizelow: *mut u32, pdwsizehigh: *mut u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.GetSize)(::windows_core::Interface::as_raw(self), pdwsizelow, pdwsizehigh).ok()
    }
    pub unsafe fn GetRights(&self, pprights: *mut *mut WMDMRIGHTS, pnrightscount: *mut u32, abmac: &mut [u8; 8]) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.GetRights)(::windows_core::Interface::as_raw(self), pprights, pnrightscount, ::core::mem::transmute(abmac.as_ptr())).ok()
    }
    pub unsafe fn EnumStorage(&self) -> ::windows_core::Result<IWMDMEnumStorage> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.EnumStorage)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SendOpaqueCommand(&self, pcommand: *mut OPAQUECOMMAND) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.SendOpaqueCommand)(::windows_core::Interface::as_raw(self), pcommand).ok()
    }
    pub unsafe fn GetStorage<P0>(&self, pszstoragename: P0) -> ::windows_core::Result<IWMDMStorage>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetStorage)(::windows_core::Interface::as_raw(self), pszstoragename.into_param().abi(), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`, `\"Win32_Media_Audio\"`, `\"Win32_Media_MediaFoundation\"`"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_Media_Audio", feature = "Win32_Media_MediaFoundation"))]
    pub unsafe fn SetAttributes2(&self, dwattributes: u32, dwattributesex: u32, pformat: ::core::option::Option<*const super::Audio::WAVEFORMATEX>, pvideoformat: ::core::option::Option<*const super::MediaFoundation::VIDEOINFOHEADER>) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.SetAttributes2)(::windows_core::Interface::as_raw(self), dwattributes, dwattributesex, ::core::mem::transmute(pformat.unwrap_or(::std::ptr::null())), ::core::mem::transmute(pvideoformat.unwrap_or(::std::ptr::null()))).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`, `\"Win32_Media_Audio\"`, `\"Win32_Media_MediaFoundation\"`"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_Media_Audio", feature = "Win32_Media_MediaFoundation"))]
    pub unsafe fn GetAttributes2(&self, pdwattributes: *mut u32, pdwattributesex: *mut u32, paudioformat: ::core::option::Option<*mut super::Audio::WAVEFORMATEX>, pvideoformat: ::core::option::Option<*mut super::MediaFoundation::VIDEOINFOHEADER>) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.GetAttributes2)(::windows_core::Interface::as_raw(self), pdwattributes, pdwattributesex, ::core::mem::transmute(paudioformat.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(pvideoformat.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    pub unsafe fn GetMetadata(&self) -> ::windows_core::Result<IWMDMMetaData> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetMetadata)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetMetadata<P0>(&self, pmetadata: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IWMDMMetaData>,
    {
        (::windows_core::Interface::vtable(self).SetMetadata)(::windows_core::Interface::as_raw(self), pmetadata.into_param().abi()).ok()
    }
    pub unsafe fn CreateEmptyMetadataObject(&self) -> ::windows_core::Result<IWMDMMetaData> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).CreateEmptyMetadataObject)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetEnumPreference(&self, pmode: *mut WMDM_STORAGE_ENUM_MODE, pviews: ::core::option::Option<&[WMDMMetadataView]>) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetEnumPreference)(::windows_core::Interface::as_raw(self), pmode, pviews.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), ::core::mem::transmute(pviews.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr()))).ok()
    }
}
::windows_core::imp::interface_hierarchy!(IWMDMStorage3, ::windows_core::IUnknown, IWMDMStorage, IWMDMStorage2);
unsafe impl ::windows_core::Interface for IWMDMStorage3 {
    type Vtable = IWMDMStorage3_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IWMDMStorage3 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x97717eea_926a_464e_96a4_247b0216026e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMDMStorage3_Vtbl {
    pub base__: IWMDMStorage2_Vtbl,
    pub GetMetadata: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppmetadata: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub SetMetadata: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pmetadata: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub CreateEmptyMetadataObject: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppmetadata: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub SetEnumPreference: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pmode: *mut WMDM_STORAGE_ENUM_MODE, nviews: u32, pviews: *const WMDMMetadataView) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IWMDMStorage4(::windows_core::IUnknown);
impl IWMDMStorage4 {
    #[doc = "Required features: `\"Win32_Media_Audio\"`"]
    #[cfg(feature = "Win32_Media_Audio")]
    pub unsafe fn SetAttributes(&self, dwattributes: u32, pformat: ::core::option::Option<*const super::Audio::WAVEFORMATEX>) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.base__.SetAttributes)(::windows_core::Interface::as_raw(self), dwattributes, ::core::mem::transmute(pformat.unwrap_or(::std::ptr::null()))).ok()
    }
    pub unsafe fn GetStorageGlobals(&self) -> ::windows_core::Result<IWMDMStorageGlobals> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.base__.GetStorageGlobals)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Media_Audio\"`"]
    #[cfg(feature = "Win32_Media_Audio")]
    pub unsafe fn GetAttributes(&self, pdwattributes: *mut u32, pformat: ::core::option::Option<*mut super::Audio::WAVEFORMATEX>) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.base__.GetAttributes)(::windows_core::Interface::as_raw(self), pdwattributes, ::core::mem::transmute(pformat.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    pub unsafe fn GetName(&self, pwszname: &mut [u16]) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.base__.GetName)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pwszname.as_ptr()), pwszname.len().try_into().unwrap()).ok()
    }
    pub unsafe fn GetDate(&self) -> ::windows_core::Result<WMDMDATETIME> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.base__.GetDate)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetSize(&self, pdwsizelow: *mut u32, pdwsizehigh: *mut u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.base__.GetSize)(::windows_core::Interface::as_raw(self), pdwsizelow, pdwsizehigh).ok()
    }
    pub unsafe fn GetRights(&self, pprights: *mut *mut WMDMRIGHTS, pnrightscount: *mut u32, abmac: &mut [u8; 8]) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.base__.GetRights)(::windows_core::Interface::as_raw(self), pprights, pnrightscount, ::core::mem::transmute(abmac.as_ptr())).ok()
    }
    pub unsafe fn EnumStorage(&self) -> ::windows_core::Result<IWMDMEnumStorage> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.base__.EnumStorage)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SendOpaqueCommand(&self, pcommand: *mut OPAQUECOMMAND) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.base__.SendOpaqueCommand)(::windows_core::Interface::as_raw(self), pcommand).ok()
    }
    pub unsafe fn GetStorage<P0>(&self, pszstoragename: P0) -> ::windows_core::Result<IWMDMStorage>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.GetStorage)(::windows_core::Interface::as_raw(self), pszstoragename.into_param().abi(), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`, `\"Win32_Media_Audio\"`, `\"Win32_Media_MediaFoundation\"`"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_Media_Audio", feature = "Win32_Media_MediaFoundation"))]
    pub unsafe fn SetAttributes2(&self, dwattributes: u32, dwattributesex: u32, pformat: ::core::option::Option<*const super::Audio::WAVEFORMATEX>, pvideoformat: ::core::option::Option<*const super::MediaFoundation::VIDEOINFOHEADER>) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.SetAttributes2)(::windows_core::Interface::as_raw(self), dwattributes, dwattributesex, ::core::mem::transmute(pformat.unwrap_or(::std::ptr::null())), ::core::mem::transmute(pvideoformat.unwrap_or(::std::ptr::null()))).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`, `\"Win32_Media_Audio\"`, `\"Win32_Media_MediaFoundation\"`"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_Media_Audio", feature = "Win32_Media_MediaFoundation"))]
    pub unsafe fn GetAttributes2(&self, pdwattributes: *mut u32, pdwattributesex: *mut u32, paudioformat: ::core::option::Option<*mut super::Audio::WAVEFORMATEX>, pvideoformat: ::core::option::Option<*mut super::MediaFoundation::VIDEOINFOHEADER>) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.GetAttributes2)(::windows_core::Interface::as_raw(self), pdwattributes, pdwattributesex, ::core::mem::transmute(paudioformat.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(pvideoformat.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    pub unsafe fn GetMetadata(&self) -> ::windows_core::Result<IWMDMMetaData> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetMetadata)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetMetadata<P0>(&self, pmetadata: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IWMDMMetaData>,
    {
        (::windows_core::Interface::vtable(self).base__.SetMetadata)(::windows_core::Interface::as_raw(self), pmetadata.into_param().abi()).ok()
    }
    pub unsafe fn CreateEmptyMetadataObject(&self) -> ::windows_core::Result<IWMDMMetaData> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.CreateEmptyMetadataObject)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetEnumPreference(&self, pmode: *mut WMDM_STORAGE_ENUM_MODE, pviews: ::core::option::Option<&[WMDMMetadataView]>) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.SetEnumPreference)(::windows_core::Interface::as_raw(self), pmode, pviews.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), ::core::mem::transmute(pviews.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr()))).ok()
    }
    pub unsafe fn SetReferences(&self, ppiwmdmstorage: ::core::option::Option<&[::core::option::Option<IWMDMStorage>]>) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetReferences)(::windows_core::Interface::as_raw(self), ppiwmdmstorage.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), ::core::mem::transmute(ppiwmdmstorage.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr()))).ok()
    }
    pub unsafe fn GetReferences(&self, pdwrefs: *mut u32, pppiwmdmstorage: *mut *mut ::core::option::Option<IWMDMStorage>) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetReferences)(::windows_core::Interface::as_raw(self), pdwrefs, pppiwmdmstorage).ok()
    }
    pub unsafe fn GetRightsWithProgress<P0>(&self, piprogresscallback: P0, pprights: *mut *mut WMDMRIGHTS, pnrightscount: *mut u32) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IWMDMProgress3>,
    {
        (::windows_core::Interface::vtable(self).GetRightsWithProgress)(::windows_core::Interface::as_raw(self), piprogresscallback.into_param().abi(), pprights, pnrightscount).ok()
    }
    pub unsafe fn GetSpecifiedMetadata(&self, ppwszpropnames: &[::windows_core::PCWSTR]) -> ::windows_core::Result<IWMDMMetaData> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetSpecifiedMetadata)(::windows_core::Interface::as_raw(self), ppwszpropnames.len().try_into().unwrap(), ::core::mem::transmute(ppwszpropnames.as_ptr()), &mut result__).from_abi(result__)
    }
    pub unsafe fn FindStorage<P0>(&self, findscope: WMDM_FIND_SCOPE, pwszuniqueid: P0) -> ::windows_core::Result<IWMDMStorage>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).FindStorage)(::windows_core::Interface::as_raw(self), findscope, pwszuniqueid.into_param().abi(), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetParent(&self) -> ::windows_core::Result<IWMDMStorage> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetParent)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
::windows_core::imp::interface_hierarchy!(IWMDMStorage4, ::windows_core::IUnknown, IWMDMStorage, IWMDMStorage2, IWMDMStorage3);
unsafe impl ::windows_core::Interface for IWMDMStorage4 {
    type Vtable = IWMDMStorage4_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IWMDMStorage4 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xc225bac5_a03a_40b8_9a23_91cf478c64a6);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMDMStorage4_Vtbl {
    pub base__: IWMDMStorage3_Vtbl,
    pub SetReferences: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwrefs: u32, ppiwmdmstorage: *const *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub GetReferences: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdwrefs: *mut u32, pppiwmdmstorage: *mut *mut ::core::option::Option<IWMDMStorage>) -> ::windows_core::HRESULT,
    pub GetRightsWithProgress: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, piprogresscallback: *mut ::core::ffi::c_void, pprights: *mut *mut WMDMRIGHTS, pnrightscount: *mut u32) -> ::windows_core::HRESULT,
    pub GetSpecifiedMetadata: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cproperties: u32, ppwszpropnames: *const ::windows_core::PCWSTR, ppmetadata: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub FindStorage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, findscope: WMDM_FIND_SCOPE, pwszuniqueid: ::windows_core::PCWSTR, ppstorage: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub GetParent: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppstorage: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IWMDMStorageControl(::windows_core::IUnknown);
impl IWMDMStorageControl {
    pub unsafe fn Insert<P0, P1, P2>(&self, fumode: u32, pwszfile: P0, poperation: P1, pprogress: P2) -> ::windows_core::Result<IWMDMStorage>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
        P1: ::windows_core::IntoParam<IWMDMOperation>,
        P2: ::windows_core::IntoParam<IWMDMProgress>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).Insert)(::windows_core::Interface::as_raw(self), fumode, pwszfile.into_param().abi(), poperation.into_param().abi(), pprogress.into_param().abi(), &mut result__).from_abi(result__)
    }
    pub unsafe fn Delete<P0>(&self, fumode: u32, pprogress: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IWMDMProgress>,
    {
        (::windows_core::Interface::vtable(self).Delete)(::windows_core::Interface::as_raw(self), fumode, pprogress.into_param().abi()).ok()
    }
    pub unsafe fn Rename<P0, P1>(&self, fumode: u32, pwsznewname: P0, pprogress: P1) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
        P1: ::windows_core::IntoParam<IWMDMProgress>,
    {
        (::windows_core::Interface::vtable(self).Rename)(::windows_core::Interface::as_raw(self), fumode, pwsznewname.into_param().abi(), pprogress.into_param().abi()).ok()
    }
    pub unsafe fn Read<P0, P1, P2>(&self, fumode: u32, pwszfile: P0, pprogress: P1, poperation: P2) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
        P1: ::windows_core::IntoParam<IWMDMProgress>,
        P2: ::windows_core::IntoParam<IWMDMOperation>,
    {
        (::windows_core::Interface::vtable(self).Read)(::windows_core::Interface::as_raw(self), fumode, pwszfile.into_param().abi(), pprogress.into_param().abi(), poperation.into_param().abi()).ok()
    }
    pub unsafe fn Move<P0, P1>(&self, fumode: u32, ptargetobject: P0, pprogress: P1) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IWMDMStorage>,
        P1: ::windows_core::IntoParam<IWMDMProgress>,
    {
        (::windows_core::Interface::vtable(self).Move)(::windows_core::Interface::as_raw(self), fumode, ptargetobject.into_param().abi(), pprogress.into_param().abi()).ok()
    }
}
::windows_core::imp::interface_hierarchy!(IWMDMStorageControl, ::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IWMDMStorageControl {
    type Vtable = IWMDMStorageControl_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IWMDMStorageControl {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x1dcb3a08_33ed_11d3_8470_00c04f79dbc0);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMDMStorageControl_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub Insert: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, fumode: u32, pwszfile: ::windows_core::PCWSTR, poperation: *mut ::core::ffi::c_void, pprogress: *mut ::core::ffi::c_void, ppnewobject: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub Delete: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, fumode: u32, pprogress: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub Rename: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, fumode: u32, pwsznewname: ::windows_core::PCWSTR, pprogress: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub Read: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, fumode: u32, pwszfile: ::windows_core::PCWSTR, pprogress: *mut ::core::ffi::c_void, poperation: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub Move: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, fumode: u32, ptargetobject: *mut ::core::ffi::c_void, pprogress: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IWMDMStorageControl2(::windows_core::IUnknown);
impl IWMDMStorageControl2 {
    pub unsafe fn Insert<P0, P1, P2>(&self, fumode: u32, pwszfile: P0, poperation: P1, pprogress: P2) -> ::windows_core::Result<IWMDMStorage>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
        P1: ::windows_core::IntoParam<IWMDMOperation>,
        P2: ::windows_core::IntoParam<IWMDMProgress>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.Insert)(::windows_core::Interface::as_raw(self), fumode, pwszfile.into_param().abi(), poperation.into_param().abi(), pprogress.into_param().abi(), &mut result__).from_abi(result__)
    }
    pub unsafe fn Delete<P0>(&self, fumode: u32, pprogress: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IWMDMProgress>,
    {
        (::windows_core::Interface::vtable(self).base__.Delete)(::windows_core::Interface::as_raw(self), fumode, pprogress.into_param().abi()).ok()
    }
    pub unsafe fn Rename<P0, P1>(&self, fumode: u32, pwsznewname: P0, pprogress: P1) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
        P1: ::windows_core::IntoParam<IWMDMProgress>,
    {
        (::windows_core::Interface::vtable(self).base__.Rename)(::windows_core::Interface::as_raw(self), fumode, pwsznewname.into_param().abi(), pprogress.into_param().abi()).ok()
    }
    pub unsafe fn Read<P0, P1, P2>(&self, fumode: u32, pwszfile: P0, pprogress: P1, poperation: P2) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
        P1: ::windows_core::IntoParam<IWMDMProgress>,
        P2: ::windows_core::IntoParam<IWMDMOperation>,
    {
        (::windows_core::Interface::vtable(self).base__.Read)(::windows_core::Interface::as_raw(self), fumode, pwszfile.into_param().abi(), pprogress.into_param().abi(), poperation.into_param().abi()).ok()
    }
    pub unsafe fn Move<P0, P1>(&self, fumode: u32, ptargetobject: P0, pprogress: P1) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IWMDMStorage>,
        P1: ::windows_core::IntoParam<IWMDMProgress>,
    {
        (::windows_core::Interface::vtable(self).base__.Move)(::windows_core::Interface::as_raw(self), fumode, ptargetobject.into_param().abi(), pprogress.into_param().abi()).ok()
    }
    pub unsafe fn Insert2<P0, P1, P2, P3, P4>(&self, fumode: u32, pwszfilesource: P0, pwszfiledest: P1, poperation: P2, pprogress: P3, punknown: P4, ppnewobject: ::core::option::Option<*mut ::core::option::Option<IWMDMStorage>>) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
        P1: ::windows_core::IntoParam<::windows_core::PCWSTR>,
        P2: ::windows_core::IntoParam<IWMDMOperation>,
        P3: ::windows_core::IntoParam<IWMDMProgress>,
        P4: ::windows_core::IntoParam<::windows_core::IUnknown>,
    {
        (::windows_core::Interface::vtable(self).Insert2)(::windows_core::Interface::as_raw(self), fumode, pwszfilesource.into_param().abi(), pwszfiledest.into_param().abi(), poperation.into_param().abi(), pprogress.into_param().abi(), punknown.into_param().abi(), ::core::mem::transmute(ppnewobject.unwrap_or(::std::ptr::null_mut()))).ok()
    }
}
::windows_core::imp::interface_hierarchy!(IWMDMStorageControl2, ::windows_core::IUnknown, IWMDMStorageControl);
unsafe impl ::windows_core::Interface for IWMDMStorageControl2 {
    type Vtable = IWMDMStorageControl2_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IWMDMStorageControl2 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x972c2e88_bd6c_4125_8e09_84f837e637b6);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMDMStorageControl2_Vtbl {
    pub base__: IWMDMStorageControl_Vtbl,
    pub Insert2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, fumode: u32, pwszfilesource: ::windows_core::PCWSTR, pwszfiledest: ::windows_core::PCWSTR, poperation: *mut ::core::ffi::c_void, pprogress: *mut ::core::ffi::c_void, punknown: *mut ::core::ffi::c_void, ppnewobject: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IWMDMStorageControl3(::windows_core::IUnknown);
impl IWMDMStorageControl3 {
    pub unsafe fn Insert<P0, P1, P2>(&self, fumode: u32, pwszfile: P0, poperation: P1, pprogress: P2) -> ::windows_core::Result<IWMDMStorage>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
        P1: ::windows_core::IntoParam<IWMDMOperation>,
        P2: ::windows_core::IntoParam<IWMDMProgress>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.Insert)(::windows_core::Interface::as_raw(self), fumode, pwszfile.into_param().abi(), poperation.into_param().abi(), pprogress.into_param().abi(), &mut result__).from_abi(result__)
    }
    pub unsafe fn Delete<P0>(&self, fumode: u32, pprogress: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IWMDMProgress>,
    {
        (::windows_core::Interface::vtable(self).base__.base__.Delete)(::windows_core::Interface::as_raw(self), fumode, pprogress.into_param().abi()).ok()
    }
    pub unsafe fn Rename<P0, P1>(&self, fumode: u32, pwsznewname: P0, pprogress: P1) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
        P1: ::windows_core::IntoParam<IWMDMProgress>,
    {
        (::windows_core::Interface::vtable(self).base__.base__.Rename)(::windows_core::Interface::as_raw(self), fumode, pwsznewname.into_param().abi(), pprogress.into_param().abi()).ok()
    }
    pub unsafe fn Read<P0, P1, P2>(&self, fumode: u32, pwszfile: P0, pprogress: P1, poperation: P2) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
        P1: ::windows_core::IntoParam<IWMDMProgress>,
        P2: ::windows_core::IntoParam<IWMDMOperation>,
    {
        (::windows_core::Interface::vtable(self).base__.base__.Read)(::windows_core::Interface::as_raw(self), fumode, pwszfile.into_param().abi(), pprogress.into_param().abi(), poperation.into_param().abi()).ok()
    }
    pub unsafe fn Move<P0, P1>(&self, fumode: u32, ptargetobject: P0, pprogress: P1) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IWMDMStorage>,
        P1: ::windows_core::IntoParam<IWMDMProgress>,
    {
        (::windows_core::Interface::vtable(self).base__.base__.Move)(::windows_core::Interface::as_raw(self), fumode, ptargetobject.into_param().abi(), pprogress.into_param().abi()).ok()
    }
    pub unsafe fn Insert2<P0, P1, P2, P3, P4>(&self, fumode: u32, pwszfilesource: P0, pwszfiledest: P1, poperation: P2, pprogress: P3, punknown: P4, ppnewobject: ::core::option::Option<*mut ::core::option::Option<IWMDMStorage>>) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
        P1: ::windows_core::IntoParam<::windows_core::PCWSTR>,
        P2: ::windows_core::IntoParam<IWMDMOperation>,
        P3: ::windows_core::IntoParam<IWMDMProgress>,
        P4: ::windows_core::IntoParam<::windows_core::IUnknown>,
    {
        (::windows_core::Interface::vtable(self).base__.Insert2)(::windows_core::Interface::as_raw(self), fumode, pwszfilesource.into_param().abi(), pwszfiledest.into_param().abi(), poperation.into_param().abi(), pprogress.into_param().abi(), punknown.into_param().abi(), ::core::mem::transmute(ppnewobject.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    pub unsafe fn Insert3<P0, P1, P2, P3, P4, P5>(&self, fumode: u32, futype: u32, pwszfilesource: P0, pwszfiledest: P1, poperation: P2, pprogress: P3, pmetadata: P4, punknown: P5, ppnewobject: ::core::option::Option<*mut ::core::option::Option<IWMDMStorage>>) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
        P1: ::windows_core::IntoParam<::windows_core::PCWSTR>,
        P2: ::windows_core::IntoParam<IWMDMOperation>,
        P3: ::windows_core::IntoParam<IWMDMProgress>,
        P4: ::windows_core::IntoParam<IWMDMMetaData>,
        P5: ::windows_core::IntoParam<::windows_core::IUnknown>,
    {
        (::windows_core::Interface::vtable(self).Insert3)(::windows_core::Interface::as_raw(self), fumode, futype, pwszfilesource.into_param().abi(), pwszfiledest.into_param().abi(), poperation.into_param().abi(), pprogress.into_param().abi(), pmetadata.into_param().abi(), punknown.into_param().abi(), ::core::mem::transmute(ppnewobject.unwrap_or(::std::ptr::null_mut()))).ok()
    }
}
::windows_core::imp::interface_hierarchy!(IWMDMStorageControl3, ::windows_core::IUnknown, IWMDMStorageControl, IWMDMStorageControl2);
unsafe impl ::windows_core::Interface for IWMDMStorageControl3 {
    type Vtable = IWMDMStorageControl3_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IWMDMStorageControl3 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xb3266365_d4f3_4696_8d53_bd27ec60993a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMDMStorageControl3_Vtbl {
    pub base__: IWMDMStorageControl2_Vtbl,
    pub Insert3: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, fumode: u32, futype: u32, pwszfilesource: ::windows_core::PCWSTR, pwszfiledest: ::windows_core::PCWSTR, poperation: *mut ::core::ffi::c_void, pprogress: *mut ::core::ffi::c_void, pmetadata: *mut ::core::ffi::c_void, punknown: *mut ::core::ffi::c_void, ppnewobject: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IWMDMStorageGlobals(::windows_core::IUnknown);
impl IWMDMStorageGlobals {
    pub unsafe fn GetCapabilities(&self) -> ::windows_core::Result<u32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetCapabilities)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetSerialNumber(&self, pserialnum: *mut WMDMID, abmac: &mut [u8; 8]) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetSerialNumber)(::windows_core::Interface::as_raw(self), pserialnum, ::core::mem::transmute(abmac.as_ptr())).ok()
    }
    pub unsafe fn GetTotalSize(&self, pdwtotalsizelow: *mut u32, pdwtotalsizehigh: *mut u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetTotalSize)(::windows_core::Interface::as_raw(self), pdwtotalsizelow, pdwtotalsizehigh).ok()
    }
    pub unsafe fn GetTotalFree(&self, pdwfreelow: *mut u32, pdwfreehigh: *mut u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetTotalFree)(::windows_core::Interface::as_raw(self), pdwfreelow, pdwfreehigh).ok()
    }
    pub unsafe fn GetTotalBad(&self, pdwbadlow: *mut u32, pdwbadhigh: *mut u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetTotalBad)(::windows_core::Interface::as_raw(self), pdwbadlow, pdwbadhigh).ok()
    }
    pub unsafe fn GetStatus(&self) -> ::windows_core::Result<u32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetStatus)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn Initialize<P0>(&self, fumode: u32, pprogress: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IWMDMProgress>,
    {
        (::windows_core::Interface::vtable(self).Initialize)(::windows_core::Interface::as_raw(self), fumode, pprogress.into_param().abi()).ok()
    }
}
::windows_core::imp::interface_hierarchy!(IWMDMStorageGlobals, ::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IWMDMStorageGlobals {
    type Vtable = IWMDMStorageGlobals_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IWMDMStorageGlobals {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x1dcb3a07_33ed_11d3_8470_00c04f79dbc0);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMDMStorageGlobals_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub GetCapabilities: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdwcapabilities: *mut u32) -> ::windows_core::HRESULT,
    pub GetSerialNumber: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pserialnum: *mut WMDMID, abmac: *mut u8) -> ::windows_core::HRESULT,
    pub GetTotalSize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdwtotalsizelow: *mut u32, pdwtotalsizehigh: *mut u32) -> ::windows_core::HRESULT,
    pub GetTotalFree: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdwfreelow: *mut u32, pdwfreehigh: *mut u32) -> ::windows_core::HRESULT,
    pub GetTotalBad: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdwbadlow: *mut u32, pdwbadhigh: *mut u32) -> ::windows_core::HRESULT,
    pub GetStatus: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdwstatus: *mut u32) -> ::windows_core::HRESULT,
    pub Initialize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, fumode: u32, pprogress: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IWMDeviceManager(::windows_core::IUnknown);
impl IWMDeviceManager {
    pub unsafe fn GetRevision(&self) -> ::windows_core::Result<u32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetRevision)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetDeviceCount(&self) -> ::windows_core::Result<u32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetDeviceCount)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn EnumDevices(&self) -> ::windows_core::Result<IWMDMEnumDevice> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).EnumDevices)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
::windows_core::imp::interface_hierarchy!(IWMDeviceManager, ::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IWMDeviceManager {
    type Vtable = IWMDeviceManager_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IWMDeviceManager {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x1dcb3a00_33ed_11d3_8470_00c04f79dbc0);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMDeviceManager_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub GetRevision: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdwrevision: *mut u32) -> ::windows_core::HRESULT,
    pub GetDeviceCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdwcount: *mut u32) -> ::windows_core::HRESULT,
    pub EnumDevices: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppenumdevice: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IWMDeviceManager2(::windows_core::IUnknown);
impl IWMDeviceManager2 {
    pub unsafe fn GetRevision(&self) -> ::windows_core::Result<u32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetRevision)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetDeviceCount(&self) -> ::windows_core::Result<u32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetDeviceCount)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn EnumDevices(&self) -> ::windows_core::Result<IWMDMEnumDevice> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.EnumDevices)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetDeviceFromCanonicalName<P0>(&self, pwszcanonicalname: P0) -> ::windows_core::Result<IWMDMDevice>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetDeviceFromCanonicalName)(::windows_core::Interface::as_raw(self), pwszcanonicalname.into_param().abi(), &mut result__).from_abi(result__)
    }
    pub unsafe fn EnumDevices2(&self) -> ::windows_core::Result<IWMDMEnumDevice> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).EnumDevices2)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn Reinitialize(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Reinitialize)(::windows_core::Interface::as_raw(self)).ok()
    }
}
::windows_core::imp::interface_hierarchy!(IWMDeviceManager2, ::windows_core::IUnknown, IWMDeviceManager);
unsafe impl ::windows_core::Interface for IWMDeviceManager2 {
    type Vtable = IWMDeviceManager2_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IWMDeviceManager2 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x923e5249_8731_4c5b_9b1c_b8b60b6e46af);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMDeviceManager2_Vtbl {
    pub base__: IWMDeviceManager_Vtbl,
    pub GetDeviceFromCanonicalName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwszcanonicalname: ::windows_core::PCWSTR, ppdevice: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub EnumDevices2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppenumdevice: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub Reinitialize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IWMDeviceManager3(::windows_core::IUnknown);
impl IWMDeviceManager3 {
    pub unsafe fn GetRevision(&self) -> ::windows_core::Result<u32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.GetRevision)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetDeviceCount(&self) -> ::windows_core::Result<u32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.GetDeviceCount)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn EnumDevices(&self) -> ::windows_core::Result<IWMDMEnumDevice> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.EnumDevices)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetDeviceFromCanonicalName<P0>(&self, pwszcanonicalname: P0) -> ::windows_core::Result<IWMDMDevice>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetDeviceFromCanonicalName)(::windows_core::Interface::as_raw(self), pwszcanonicalname.into_param().abi(), &mut result__).from_abi(result__)
    }
    pub unsafe fn EnumDevices2(&self) -> ::windows_core::Result<IWMDMEnumDevice> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.EnumDevices2)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn Reinitialize(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.Reinitialize)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn SetDeviceEnumPreference(&self, dwenumpref: u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetDeviceEnumPreference)(::windows_core::Interface::as_raw(self), dwenumpref).ok()
    }
}
::windows_core::imp::interface_hierarchy!(IWMDeviceManager3, ::windows_core::IUnknown, IWMDeviceManager, IWMDeviceManager2);
unsafe impl ::windows_core::Interface for IWMDeviceManager3 {
    type Vtable = IWMDeviceManager3_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IWMDeviceManager3 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xaf185c41_100d_46ed_be2e_9ce8c44594ef);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMDeviceManager3_Vtbl {
    pub base__: IWMDeviceManager2_Vtbl,
    pub SetDeviceEnumPreference: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwenumpref: u32) -> ::windows_core::HRESULT,
}
pub const ALLOW_OUTOFBAND_NOTIFICATION: u32 = 2u32;
pub const DO_NOT_VIRTUALIZE_STORAGES_AS_DEVICES: u32 = 1u32;
pub const ENUM_MODE_METADATA_VIEWS: WMDM_STORAGE_ENUM_MODE = WMDM_STORAGE_ENUM_MODE(2i32);
pub const ENUM_MODE_RAW: WMDM_STORAGE_ENUM_MODE = WMDM_STORAGE_ENUM_MODE(0i32);
pub const ENUM_MODE_USE_DEVICE_PREF: WMDM_STORAGE_ENUM_MODE = WMDM_STORAGE_ENUM_MODE(1i32);
pub const EVENT_WMDM_CONTENT_TRANSFER: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x339c9bf4_bcfe_4ed8_94df_eaf8c26ab61b);
pub const IOCTL_MTP_CUSTOM_COMMAND: u32 = 827348045u32;
pub const MDSP_READ: u32 = 1u32;
pub const MDSP_SEEK_BOF: u32 = 1u32;
pub const MDSP_SEEK_CUR: u32 = 2u32;
pub const MDSP_SEEK_EOF: u32 = 4u32;
pub const MDSP_WRITE: u32 = 2u32;
pub const MTP_COMMAND_MAX_PARAMS: u32 = 5u32;
pub const MTP_NEXTPHASE_NO_DATA: u32 = 3u32;
pub const MTP_NEXTPHASE_READ_DATA: u32 = 1u32;
pub const MTP_NEXTPHASE_WRITE_DATA: u32 = 2u32;
pub const MTP_RESPONSE_MAX_PARAMS: u32 = 5u32;
pub const MTP_RESPONSE_OK: u16 = 8193u16;
pub const MediaDevMgr: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x25baad81_3560_11d3_8471_00c04f79dbc0);
pub const MediaDevMgrClassFactory: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x50040c1d_bdbf_4924_b873_f14d6c5bfd66);
pub const RSA_KEY_LEN: u32 = 64u32;
pub const SAC_CERT_V1: u32 = 2u32;
pub const SAC_CERT_X509: u32 = 1u32;
pub const SAC_MAC_LEN: u32 = 8u32;
pub const SAC_PROTOCOL_V1: u32 = 2u32;
pub const SAC_PROTOCOL_WMDM: u32 = 1u32;
pub const SAC_SESSION_KEYLEN: u32 = 8u32;
pub const SCP_EVENTID_ACQSECURECLOCK: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x86248cc9_4a59_43e2_9146_48a7f3f4140c);
pub const SCP_EVENTID_DRMINFO: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x213dd287_41d2_432b_9e3f_3b4f7b3581dd);
pub const SCP_EVENTID_NEEDTOINDIV: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x87a507c7_b469_4386_b976_d5d1ce538a6f);
pub const SCP_PARAMID_DRMVERSION: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x41d0155d_7cc7_4217_ada9_005074624da4);
pub const WMDMDevice: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x807b3cdf_357a_11d3_8471_00c04f79dbc0);
pub const WMDMDeviceEnum: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x430e35af_3971_11d3_8474_00c04f79dbc0);
pub const WMDMID_LENGTH: u32 = 128u32;
pub const WMDMLogger: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x110a3202_5a79_11d3_8d78_444553540000);
pub const WMDMStorage: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x807b3ce0_357a_11d3_8471_00c04f79dbc0);
pub const WMDMStorageEnum: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xeb401a3b_3af7_11d3_8474_00c04f79dbc0);
pub const WMDMStorageGlobal: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x807b3ce1_357a_11d3_8471_00c04f79dbc0);
pub const WMDM_APP_REVOKED: u32 = 2u32;
pub const WMDM_CONTENT_FILE: u32 = 4u32;
pub const WMDM_CONTENT_FOLDER: u32 = 8u32;
pub const WMDM_CONTENT_OPERATIONINTERFACE: u32 = 16u32;
pub const WMDM_DEVICECAP_CANPAUSE: u32 = 16u32;
pub const WMDM_DEVICECAP_CANPLAY: u32 = 1u32;
pub const WMDM_DEVICECAP_CANRECORD: u32 = 4u32;
pub const WMDM_DEVICECAP_CANRESUME: u32 = 32u32;
pub const WMDM_DEVICECAP_CANSEEK: u32 = 128u32;
pub const WMDM_DEVICECAP_CANSTOP: u32 = 64u32;
pub const WMDM_DEVICECAP_CANSTREAMPLAY: u32 = 2u32;
pub const WMDM_DEVICECAP_CANSTREAMRECORD: u32 = 8u32;
pub const WMDM_DEVICECAP_HASSECURECLOCK: u32 = 256u32;
pub const WMDM_DEVICE_PROTOCOL_MSC: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xa4d2c26c_a881_44bb_bd5d_1f703c71f7a9);
pub const WMDM_DEVICE_PROTOCOL_MTP: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x979e54e5_0afc_4604_8d93_dc798a4bcf45);
pub const WMDM_DEVICE_PROTOCOL_RAPI: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x2a11ed91_8c8f_41e4_82d1_8386e003561c);
pub const WMDM_DEVICE_TYPE_DECODE: u32 = 4u32;
pub const WMDM_DEVICE_TYPE_ENCODE: u32 = 8u32;
pub const WMDM_DEVICE_TYPE_FILELISTRESYNC: u32 = 512u32;
pub const WMDM_DEVICE_TYPE_NONREENTRANT: u32 = 256u32;
pub const WMDM_DEVICE_TYPE_NONSDMI: u32 = 128u32;
pub const WMDM_DEVICE_TYPE_PLAYBACK: u32 = 1u32;
pub const WMDM_DEVICE_TYPE_RECORD: u32 = 2u32;
pub const WMDM_DEVICE_TYPE_SDMI: u32 = 64u32;
pub const WMDM_DEVICE_TYPE_STORAGE: u32 = 16u32;
pub const WMDM_DEVICE_TYPE_VIEW_PREF_METADATAVIEW: u32 = 1024u32;
pub const WMDM_DEVICE_TYPE_VIRTUAL: u32 = 32u32;
pub const WMDM_ENUM_PROP_VALID_VALUES_ANY: WMDM_ENUM_PROP_VALID_VALUES_FORM = WMDM_ENUM_PROP_VALID_VALUES_FORM(0i32);
pub const WMDM_ENUM_PROP_VALID_VALUES_ENUM: WMDM_ENUM_PROP_VALID_VALUES_FORM = WMDM_ENUM_PROP_VALID_VALUES_FORM(2i32);
pub const WMDM_ENUM_PROP_VALID_VALUES_RANGE: WMDM_ENUM_PROP_VALID_VALUES_FORM = WMDM_ENUM_PROP_VALID_VALUES_FORM(1i32);
pub const WMDM_E_BUFFERTOOSMALL: i32 = -2147201016i32;
pub const WMDM_E_BUSY: i32 = -2147201024i32;
pub const WMDM_E_CALL_OUT_OF_SEQUENCE: i32 = -2147201017i32;
pub const WMDM_E_CANTOPEN_PMSN_SERVICE_PIPE: i32 = -2147201005i32;
pub const WMDM_E_INCORRECT_APPSEC: i32 = -2147201008i32;
pub const WMDM_E_INCORRECT_RIGHTS: i32 = -2147201007i32;
pub const WMDM_E_INTERFACEDEAD: i32 = -2147201023i32;
pub const WMDM_E_INVALIDTYPE: i32 = -2147201022i32;
pub const WMDM_E_LICENSE_EXPIRED: i32 = -2147201006i32;
pub const WMDM_E_LICENSE_NOTEXIST: i32 = -2147201009i32;
pub const WMDM_E_MAC_CHECK_FAILED: i32 = -2147201014i32;
pub const WMDM_E_MOREDATA: i32 = -2147201015i32;
pub const WMDM_E_NORIGHTS: i32 = -2147201018i32;
pub const WMDM_E_NOTCERTIFIED: i32 = -2147201019i32;
pub const WMDM_E_NOTSUPPORTED: i32 = -2147201020i32;
pub const WMDM_E_PROCESSFAILED: i32 = -2147201021i32;
pub const WMDM_E_REVOKED: i32 = -2147201010i32;
pub const WMDM_E_SDMI_NOMORECOPIES: i32 = -2147201011i32;
pub const WMDM_E_SDMI_TRIGGER: i32 = -2147201012i32;
pub const WMDM_E_TOO_MANY_SESSIONS: i32 = -2147201005i32;
pub const WMDM_E_USER_CANCELLED: i32 = -2147201013i32;
pub const WMDM_FILE_ATTR_AUDIO: u32 = 4096u32;
pub const WMDM_FILE_ATTR_AUDIOBOOK: u32 = 2097152u32;
pub const WMDM_FILE_ATTR_CANDELETE: u32 = 32768u32;
pub const WMDM_FILE_ATTR_CANMOVE: u32 = 65536u32;
pub const WMDM_FILE_ATTR_CANPLAY: u32 = 16384u32;
pub const WMDM_FILE_ATTR_CANREAD: u32 = 262144u32;
pub const WMDM_FILE_ATTR_CANRENAME: u32 = 131072u32;
pub const WMDM_FILE_ATTR_DATA: u32 = 8192u32;
pub const WMDM_FILE_ATTR_FILE: u32 = 32u32;
pub const WMDM_FILE_ATTR_FOLDER: u32 = 8u32;
pub const WMDM_FILE_ATTR_HIDDEN: u32 = 4194304u32;
pub const WMDM_FILE_ATTR_LINK: u32 = 16u32;
pub const WMDM_FILE_ATTR_MUSIC: u32 = 524288u32;
pub const WMDM_FILE_ATTR_READONLY: u32 = 16777216u32;
pub const WMDM_FILE_ATTR_SYSTEM: u32 = 8388608u32;
pub const WMDM_FILE_ATTR_VIDEO: u32 = 64u32;
pub const WMDM_FILE_CREATE_OVERWRITE: u32 = 1048576u32;
pub const WMDM_FIND_SCOPE_GLOBAL: WMDM_FIND_SCOPE = WMDM_FIND_SCOPE(0i32);
pub const WMDM_FIND_SCOPE_IMMEDIATE_CHILDREN: WMDM_FIND_SCOPE = WMDM_FIND_SCOPE(1i32);
pub const WMDM_FORMATCODE_3G2: WMDM_FORMATCODE = WMDM_FORMATCODE(47493i32);
pub const WMDM_FORMATCODE_3G2A: WMDM_FORMATCODE = WMDM_FORMATCODE(860303937i32);
pub const WMDM_FORMATCODE_3GP: WMDM_FORMATCODE = WMDM_FORMATCODE(47492i32);
pub const WMDM_FORMATCODE_3GPA: WMDM_FORMATCODE = WMDM_FORMATCODE(860311617i32);
pub const WMDM_FORMATCODE_AAC: WMDM_FORMATCODE = WMDM_FORMATCODE(47363i32);
pub const WMDM_FORMATCODE_ABSTRACTAUDIOALBUM: WMDM_FORMATCODE = WMDM_FORMATCODE(47619i32);
pub const WMDM_FORMATCODE_ABSTRACTAUDIOVIDEOPLAYLIST: WMDM_FORMATCODE = WMDM_FORMATCODE(47621i32);
pub const WMDM_FORMATCODE_ABSTRACTCALENDARITEM: WMDM_FORMATCODE = WMDM_FORMATCODE(48641i32);
pub const WMDM_FORMATCODE_ABSTRACTCHAPTEREDPRODUCTION: WMDM_FORMATCODE = WMDM_FORMATCODE(47624i32);
pub const WMDM_FORMATCODE_ABSTRACTCONTACT: WMDM_FORMATCODE = WMDM_FORMATCODE(48001i32);
pub const WMDM_FORMATCODE_ABSTRACTCONTACTGROUP: WMDM_FORMATCODE = WMDM_FORMATCODE(47622i32);
pub const WMDM_FORMATCODE_ABSTRACTDOCUMENT: WMDM_FORMATCODE = WMDM_FORMATCODE(47745i32);
pub const WMDM_FORMATCODE_ABSTRACTIMAGEALBUM: WMDM_FORMATCODE = WMDM_FORMATCODE(47618i32);
pub const WMDM_FORMATCODE_ABSTRACTMESSAGE: WMDM_FORMATCODE = WMDM_FORMATCODE(47873i32);
pub const WMDM_FORMATCODE_ABSTRACTMESSAGEFOLDER: WMDM_FORMATCODE = WMDM_FORMATCODE(47623i32);
pub const WMDM_FORMATCODE_ABSTRACTMULTIMEDIAALBUM: WMDM_FORMATCODE = WMDM_FORMATCODE(47617i32);
pub const WMDM_FORMATCODE_ABSTRACTVIDEOALBUM: WMDM_FORMATCODE = WMDM_FORMATCODE(47620i32);
pub const WMDM_FORMATCODE_AIFF: WMDM_FORMATCODE = WMDM_FORMATCODE(12295i32);
pub const WMDM_FORMATCODE_ALLIMAGES: WMDM_FORMATCODE = WMDM_FORMATCODE(-1i32);
pub const WMDM_FORMATCODE_AMR: WMDM_FORMATCODE = WMDM_FORMATCODE(47368i32);
pub const WMDM_FORMATCODE_ASF: WMDM_FORMATCODE = WMDM_FORMATCODE(12300i32);
pub const WMDM_FORMATCODE_ASSOCIATION: WMDM_FORMATCODE = WMDM_FORMATCODE(12289i32);
pub const WMDM_FORMATCODE_ASXPLAYLIST: WMDM_FORMATCODE = WMDM_FORMATCODE(47635i32);
pub const WMDM_FORMATCODE_ATSCTS: WMDM_FORMATCODE = WMDM_FORMATCODE(47495i32);
pub const WMDM_FORMATCODE_AUDIBLE: WMDM_FORMATCODE = WMDM_FORMATCODE(47364i32);
pub const WMDM_FORMATCODE_AVCHD: WMDM_FORMATCODE = WMDM_FORMATCODE(47494i32);
pub const WMDM_FORMATCODE_AVI: WMDM_FORMATCODE = WMDM_FORMATCODE(12298i32);
pub const WMDM_FORMATCODE_DPOF: WMDM_FORMATCODE = WMDM_FORMATCODE(12294i32);
pub const WMDM_FORMATCODE_DVBTS: WMDM_FORMATCODE = WMDM_FORMATCODE(47496i32);
pub const WMDM_FORMATCODE_EXECUTABLE: WMDM_FORMATCODE = WMDM_FORMATCODE(12291i32);
pub const WMDM_FORMATCODE_FLAC: WMDM_FORMATCODE = WMDM_FORMATCODE(47366i32);
pub const WMDM_FORMATCODE_HTML: WMDM_FORMATCODE = WMDM_FORMATCODE(12293i32);
pub const WMDM_FORMATCODE_IMAGE_BMP: WMDM_FORMATCODE = WMDM_FORMATCODE(14340i32);
pub const WMDM_FORMATCODE_IMAGE_CIFF: WMDM_FORMATCODE = WMDM_FORMATCODE(14341i32);
pub const WMDM_FORMATCODE_IMAGE_EXIF: WMDM_FORMATCODE = WMDM_FORMATCODE(14337i32);
pub const WMDM_FORMATCODE_IMAGE_FLASHPIX: WMDM_FORMATCODE = WMDM_FORMATCODE(14339i32);
pub const WMDM_FORMATCODE_IMAGE_GIF: WMDM_FORMATCODE = WMDM_FORMATCODE(14343i32);
pub const WMDM_FORMATCODE_IMAGE_JFIF: WMDM_FORMATCODE = WMDM_FORMATCODE(14344i32);
pub const WMDM_FORMATCODE_IMAGE_JP2: WMDM_FORMATCODE = WMDM_FORMATCODE(14351i32);
pub const WMDM_FORMATCODE_IMAGE_JPX: WMDM_FORMATCODE = WMDM_FORMATCODE(14352i32);
pub const WMDM_FORMATCODE_IMAGE_PCD: WMDM_FORMATCODE = WMDM_FORMATCODE(14345i32);
pub const WMDM_FORMATCODE_IMAGE_PICT: WMDM_FORMATCODE = WMDM_FORMATCODE(14346i32);
pub const WMDM_FORMATCODE_IMAGE_PNG: WMDM_FORMATCODE = WMDM_FORMATCODE(14347i32);
pub const WMDM_FORMATCODE_IMAGE_RESERVED_FIRST: WMDM_FORMATCODE = WMDM_FORMATCODE(14353i32);
pub const WMDM_FORMATCODE_IMAGE_RESERVED_LAST: WMDM_FORMATCODE = WMDM_FORMATCODE(16383i32);
pub const WMDM_FORMATCODE_IMAGE_TIFF: WMDM_FORMATCODE = WMDM_FORMATCODE(14349i32);
pub const WMDM_FORMATCODE_IMAGE_TIFFEP: WMDM_FORMATCODE = WMDM_FORMATCODE(14338i32);
pub const WMDM_FORMATCODE_IMAGE_TIFFIT: WMDM_FORMATCODE = WMDM_FORMATCODE(14350i32);
pub const WMDM_FORMATCODE_IMAGE_UNDEFINED: WMDM_FORMATCODE = WMDM_FORMATCODE(14336i32);
pub const WMDM_FORMATCODE_JPEGXR: WMDM_FORMATCODE = WMDM_FORMATCODE(47108i32);
pub const WMDM_FORMATCODE_M3UPLAYLIST: WMDM_FORMATCODE = WMDM_FORMATCODE(47633i32);
pub const WMDM_FORMATCODE_M4A: WMDM_FORMATCODE = WMDM_FORMATCODE(1297101889i32);
pub const WMDM_FORMATCODE_MEDIA_CAST: WMDM_FORMATCODE = WMDM_FORMATCODE(47627i32);
pub const WMDM_FORMATCODE_MHTCOMPILEDHTMLDOCUMENT: WMDM_FORMATCODE = WMDM_FORMATCODE(47748i32);
pub const WMDM_FORMATCODE_MICROSOFTEXCELSPREADSHEET: WMDM_FORMATCODE = WMDM_FORMATCODE(47749i32);
pub const WMDM_FORMATCODE_MICROSOFTPOWERPOINTDOCUMENT: WMDM_FORMATCODE = WMDM_FORMATCODE(47750i32);
pub const WMDM_FORMATCODE_MICROSOFTWORDDOCUMENT: WMDM_FORMATCODE = WMDM_FORMATCODE(47747i32);
pub const WMDM_FORMATCODE_MK3D: WMDM_FORMATCODE = WMDM_FORMATCODE(47499i32);
pub const WMDM_FORMATCODE_MKA: WMDM_FORMATCODE = WMDM_FORMATCODE(47498i32);
pub const WMDM_FORMATCODE_MKV: WMDM_FORMATCODE = WMDM_FORMATCODE(47497i32);
pub const WMDM_FORMATCODE_MP2: WMDM_FORMATCODE = WMDM_FORMATCODE(47491i32);
pub const WMDM_FORMATCODE_MP3: WMDM_FORMATCODE = WMDM_FORMATCODE(12297i32);
pub const WMDM_FORMATCODE_MP4: WMDM_FORMATCODE = WMDM_FORMATCODE(47490i32);
pub const WMDM_FORMATCODE_MPEG: WMDM_FORMATCODE = WMDM_FORMATCODE(12299i32);
pub const WMDM_FORMATCODE_MPLPLAYLIST: WMDM_FORMATCODE = WMDM_FORMATCODE(47634i32);
pub const WMDM_FORMATCODE_NOTUSED: WMDM_FORMATCODE = WMDM_FORMATCODE(0i32);
pub const WMDM_FORMATCODE_OGG: WMDM_FORMATCODE = WMDM_FORMATCODE(47362i32);
pub const WMDM_FORMATCODE_PLSPLAYLIST: WMDM_FORMATCODE = WMDM_FORMATCODE(47636i32);
pub const WMDM_FORMATCODE_QCELP: WMDM_FORMATCODE = WMDM_FORMATCODE(47367i32);
pub const WMDM_FORMATCODE_RESERVED_FIRST: WMDM_FORMATCODE = WMDM_FORMATCODE(12301i32);
pub const WMDM_FORMATCODE_RESERVED_LAST: WMDM_FORMATCODE = WMDM_FORMATCODE(14335i32);
pub const WMDM_FORMATCODE_SCRIPT: WMDM_FORMATCODE = WMDM_FORMATCODE(12290i32);
pub const WMDM_FORMATCODE_SECTION: WMDM_FORMATCODE = WMDM_FORMATCODE(48770i32);
pub const WMDM_FORMATCODE_TEXT: WMDM_FORMATCODE = WMDM_FORMATCODE(12292i32);
pub const WMDM_FORMATCODE_UNDEFINED: WMDM_FORMATCODE = WMDM_FORMATCODE(12288i32);
pub const WMDM_FORMATCODE_UNDEFINEDAUDIO: WMDM_FORMATCODE = WMDM_FORMATCODE(47360i32);
pub const WMDM_FORMATCODE_UNDEFINEDCALENDARITEM: WMDM_FORMATCODE = WMDM_FORMATCODE(48640i32);
pub const WMDM_FORMATCODE_UNDEFINEDCOLLECTION: WMDM_FORMATCODE = WMDM_FORMATCODE(47616i32);
pub const WMDM_FORMATCODE_UNDEFINEDCONTACT: WMDM_FORMATCODE = WMDM_FORMATCODE(48000i32);
pub const WMDM_FORMATCODE_UNDEFINEDDOCUMENT: WMDM_FORMATCODE = WMDM_FORMATCODE(47744i32);
pub const WMDM_FORMATCODE_UNDEFINEDFIRMWARE: WMDM_FORMATCODE = WMDM_FORMATCODE(47106i32);
pub const WMDM_FORMATCODE_UNDEFINEDMESSAGE: WMDM_FORMATCODE = WMDM_FORMATCODE(47872i32);
pub const WMDM_FORMATCODE_UNDEFINEDVIDEO: WMDM_FORMATCODE = WMDM_FORMATCODE(47488i32);
pub const WMDM_FORMATCODE_UNDEFINEDWINDOWSEXECUTABLE: WMDM_FORMATCODE = WMDM_FORMATCODE(48768i32);
pub const WMDM_FORMATCODE_VCALENDAR1: WMDM_FORMATCODE = WMDM_FORMATCODE(48642i32);
pub const WMDM_FORMATCODE_VCALENDAR2: WMDM_FORMATCODE = WMDM_FORMATCODE(48643i32);
pub const WMDM_FORMATCODE_VCARD2: WMDM_FORMATCODE = WMDM_FORMATCODE(48002i32);
pub const WMDM_FORMATCODE_VCARD3: WMDM_FORMATCODE = WMDM_FORMATCODE(48003i32);
pub const WMDM_FORMATCODE_WAVE: WMDM_FORMATCODE = WMDM_FORMATCODE(12296i32);
pub const WMDM_FORMATCODE_WBMP: WMDM_FORMATCODE = WMDM_FORMATCODE(47107i32);
pub const WMDM_FORMATCODE_WINDOWSIMAGEFORMAT: WMDM_FORMATCODE = WMDM_FORMATCODE(47233i32);
pub const WMDM_FORMATCODE_WMA: WMDM_FORMATCODE = WMDM_FORMATCODE(47361i32);
pub const WMDM_FORMATCODE_WMV: WMDM_FORMATCODE = WMDM_FORMATCODE(47489i32);
pub const WMDM_FORMATCODE_WPLPLAYLIST: WMDM_FORMATCODE = WMDM_FORMATCODE(47632i32);
pub const WMDM_FORMATCODE_XMLDOCUMENT: WMDM_FORMATCODE = WMDM_FORMATCODE(47746i32);
pub const WMDM_GET_FORMAT_SUPPORT_AUDIO: u32 = 1u32;
pub const WMDM_GET_FORMAT_SUPPORT_FILE: u32 = 4u32;
pub const WMDM_GET_FORMAT_SUPPORT_VIDEO: u32 = 2u32;
pub const WMDM_LOG_NOTIMESTAMP: u32 = 16u32;
pub const WMDM_LOG_SEV_ERROR: u32 = 4u32;
pub const WMDM_LOG_SEV_INFO: u32 = 1u32;
pub const WMDM_LOG_SEV_WARN: u32 = 2u32;
pub const WMDM_MAC_LENGTH: u32 = 8u32;
pub const WMDM_MODE_BLOCK: u32 = 1u32;
pub const WMDM_MODE_PROGRESS: u32 = 64u32;
pub const WMDM_MODE_QUERY: u32 = 32u32;
pub const WMDM_MODE_RECURSIVE: u32 = 4096u32;
pub const WMDM_MODE_THREAD: u32 = 2u32;
pub const WMDM_MODE_TRANSFER_PROTECTED: u32 = 128u32;
pub const WMDM_MODE_TRANSFER_UNPROTECTED: u32 = 256u32;
pub const WMDM_MSG_DEVICE_ARRIVAL: WMDMMessage = WMDMMessage(0i32);
pub const WMDM_MSG_DEVICE_REMOVAL: WMDMMessage = WMDMMessage(1i32);
pub const WMDM_MSG_MEDIA_ARRIVAL: WMDMMessage = WMDMMessage(2i32);
pub const WMDM_MSG_MEDIA_REMOVAL: WMDMMessage = WMDMMessage(3i32);
pub const WMDM_POWER_CAP_BATTERY: u32 = 1u32;
pub const WMDM_POWER_CAP_EXTERNAL: u32 = 2u32;
pub const WMDM_POWER_IS_BATTERY: u32 = 4u32;
pub const WMDM_POWER_IS_EXTERNAL: u32 = 8u32;
pub const WMDM_POWER_PERCENT_AVAILABLE: u32 = 16u32;
pub const WMDM_RIGHTS_COPY_TO_CD: u32 = 8u32;
pub const WMDM_RIGHTS_COPY_TO_NON_SDMI_DEVICE: u32 = 2u32;
pub const WMDM_RIGHTS_COPY_TO_SDMI_DEVICE: u32 = 16u32;
pub const WMDM_RIGHTS_EXPIRATIONDATE: u32 = 2u32;
pub const WMDM_RIGHTS_FREESERIALIDS: u32 = 8u32;
pub const WMDM_RIGHTS_GROUPID: u32 = 4u32;
pub const WMDM_RIGHTS_NAMEDSERIALIDS: u32 = 16u32;
pub const WMDM_RIGHTS_PLAYBACKCOUNT: u32 = 1u32;
pub const WMDM_RIGHTS_PLAY_ON_PC: u32 = 1u32;
pub const WMDM_SCP_DECIDE_DATA: i32 = 8i32;
pub const WMDM_SCP_DRMINFO_NOT_DRMPROTECTED: i32 = 0i32;
pub const WMDM_SCP_DRMINFO_V1HEADER: i32 = 1i32;
pub const WMDM_SCP_DRMINFO_V2HEADER: i32 = 2i32;
pub const WMDM_SCP_EXAMINE_DATA: i32 = 2i32;
pub const WMDM_SCP_EXAMINE_EXTENSION: i32 = 1i32;
pub const WMDM_SCP_NO_MORE_CHANGES: i32 = 64i32;
pub const WMDM_SCP_PROTECTED_OUTPUT: i32 = 16i32;
pub const WMDM_SCP_REVOKED: u32 = 8u32;
pub const WMDM_SCP_RIGHTS_DATA: i32 = 64i32;
pub const WMDM_SCP_TRANSFER_OBJECTDATA: i32 = 32i32;
pub const WMDM_SCP_UNPROTECTED_OUTPUT: i32 = 32i32;
pub const WMDM_SEEK_BEGIN: u32 = 1u32;
pub const WMDM_SEEK_CURRENT: u32 = 2u32;
pub const WMDM_SEEK_END: u32 = 8u32;
pub const WMDM_SEEK_REMOTECONTROL: u32 = 1u32;
pub const WMDM_SEEK_STREAMINGAUDIO: u32 = 2u32;
pub const WMDM_SERVICE_PROVIDER_VENDOR_MICROSOFT: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x7de8686d_78ee_43ea_a496_c625ac91cc5d);
pub const WMDM_SESSION_CUSTOM: WMDM_SESSION_TYPE = WMDM_SESSION_TYPE(4096i32);
pub const WMDM_SESSION_DELETE: WMDM_SESSION_TYPE = WMDM_SESSION_TYPE(256i32);
pub const WMDM_SESSION_NONE: WMDM_SESSION_TYPE = WMDM_SESSION_TYPE(0i32);
pub const WMDM_SESSION_TRANSFER_FROM_DEVICE: WMDM_SESSION_TYPE = WMDM_SESSION_TYPE(16i32);
pub const WMDM_SESSION_TRANSFER_TO_DEVICE: WMDM_SESSION_TYPE = WMDM_SESSION_TYPE(1i32);
pub const WMDM_SP_REVOKED: u32 = 4u32;
pub const WMDM_STATUS_BUSY: u32 = 2u32;
pub const WMDM_STATUS_DEVICECONTROL_PAUSED: u32 = 32u32;
pub const WMDM_STATUS_DEVICECONTROL_PLAYING: u32 = 8u32;
pub const WMDM_STATUS_DEVICECONTROL_RECORDING: u32 = 16u32;
pub const WMDM_STATUS_DEVICECONTROL_REMOTE: u32 = 64u32;
pub const WMDM_STATUS_DEVICECONTROL_STREAM: u32 = 128u32;
pub const WMDM_STATUS_DEVICE_NOTPRESENT: u32 = 4u32;
pub const WMDM_STATUS_READY: u32 = 1u32;
pub const WMDM_STATUS_STORAGECONTROL_APPENDING: u32 = 32768u32;
pub const WMDM_STATUS_STORAGECONTROL_DELETING: u32 = 16384u32;
pub const WMDM_STATUS_STORAGECONTROL_INSERTING: u32 = 8192u32;
pub const WMDM_STATUS_STORAGECONTROL_MOVING: u32 = 65536u32;
pub const WMDM_STATUS_STORAGECONTROL_READING: u32 = 131072u32;
pub const WMDM_STATUS_STORAGE_BROKEN: u32 = 1024u32;
pub const WMDM_STATUS_STORAGE_INITIALIZING: u32 = 512u32;
pub const WMDM_STATUS_STORAGE_NOTPRESENT: u32 = 256u32;
pub const WMDM_STATUS_STORAGE_NOTSUPPORTED: u32 = 2048u32;
pub const WMDM_STATUS_STORAGE_UNFORMATTED: u32 = 4096u32;
pub const WMDM_STORAGECAP_FILELIMITEXISTS: u32 = 32u32;
pub const WMDM_STORAGECAP_FILESINFOLDERS: u32 = 8u32;
pub const WMDM_STORAGECAP_FILESINROOT: u32 = 2u32;
pub const WMDM_STORAGECAP_FOLDERLIMITEXISTS: u32 = 16u32;
pub const WMDM_STORAGECAP_FOLDERSINFOLDERS: u32 = 4u32;
pub const WMDM_STORAGECAP_FOLDERSINROOT: u32 = 1u32;
pub const WMDM_STORAGECAP_NOT_INITIALIZABLE: u32 = 64u32;
pub const WMDM_STORAGECONTROL_INSERTAFTER: u32 = 1024u32;
pub const WMDM_STORAGECONTROL_INSERTBEFORE: u32 = 512u32;
pub const WMDM_STORAGECONTROL_INSERTINTO: u32 = 2048u32;
pub const WMDM_STORAGE_ATTR_CANEDITMETADATA: u32 = 128u32;
pub const WMDM_STORAGE_ATTR_FILESYSTEM: u32 = 1u32;
pub const WMDM_STORAGE_ATTR_FOLDERS: u32 = 256u32;
pub const WMDM_STORAGE_ATTR_HAS_FILES: u32 = 67108864u32;
pub const WMDM_STORAGE_ATTR_HAS_FOLDERS: u32 = 33554432u32;
pub const WMDM_STORAGE_ATTR_NONREMOVABLE: u32 = 4u32;
pub const WMDM_STORAGE_ATTR_REMOVABLE: u32 = 2u32;
pub const WMDM_STORAGE_ATTR_VIRTUAL: u32 = 536870912u32;
pub const WMDM_STORAGE_CONTAINS_DEFAULT: u32 = 268435456u32;
pub const WMDM_STORAGE_IS_DEFAULT: u32 = 134217728u32;
pub const WMDM_S_NOT_ALL_PROPERTIES_APPLIED: i32 = 282625i32;
pub const WMDM_S_NOT_ALL_PROPERTIES_RETRIEVED: i32 = 282626i32;
pub const WMDM_TYPE_BINARY: WMDM_TAG_DATATYPE = WMDM_TAG_DATATYPE(2i32);
pub const WMDM_TYPE_BOOL: WMDM_TAG_DATATYPE = WMDM_TAG_DATATYPE(3i32);
pub const WMDM_TYPE_DATE: WMDM_TAG_DATATYPE = WMDM_TAG_DATATYPE(7i32);
pub const WMDM_TYPE_DWORD: WMDM_TAG_DATATYPE = WMDM_TAG_DATATYPE(0i32);
pub const WMDM_TYPE_GUID: WMDM_TAG_DATATYPE = WMDM_TAG_DATATYPE(6i32);
pub const WMDM_TYPE_QWORD: WMDM_TAG_DATATYPE = WMDM_TAG_DATATYPE(4i32);
pub const WMDM_TYPE_STRING: WMDM_TAG_DATATYPE = WMDM_TAG_DATATYPE(1i32);
pub const WMDM_TYPE_WORD: WMDM_TAG_DATATYPE = WMDM_TAG_DATATYPE(5i32);
pub const WMDM_WMDM_REVOKED: u32 = 1u32;
pub const g_wszAudioWAVECodec: ::windows_core::PCWSTR = ::windows_core::w!("WMDM/AudioWAVECodec");
pub const g_wszVideoFourCCCodec: ::windows_core::PCWSTR = ::windows_core::w!("WMDM/VideoFourCCCodec");
pub const g_wszWMDMAlbumArt: ::windows_core::PCWSTR = ::windows_core::w!("WMDM/AlbumArt");
pub const g_wszWMDMAlbumArtist: ::windows_core::PCWSTR = ::windows_core::w!("WMDM/AlbumArtist");
pub const g_wszWMDMAlbumCoverData: ::windows_core::PCWSTR = ::windows_core::w!("WMDM/AlbumCoverData");
pub const g_wszWMDMAlbumCoverDuration: ::windows_core::PCWSTR = ::windows_core::w!("WMDM/AlbumCoverDuration");
pub const g_wszWMDMAlbumCoverFormat: ::windows_core::PCWSTR = ::windows_core::w!("WMDM/AlbumCoverFormat");
pub const g_wszWMDMAlbumCoverHeight: ::windows_core::PCWSTR = ::windows_core::w!("WMDM/AlbumCoverHeight");
pub const g_wszWMDMAlbumCoverSize: ::windows_core::PCWSTR = ::windows_core::w!("WMDM/AlbumCoverSize");
pub const g_wszWMDMAlbumCoverWidth: ::windows_core::PCWSTR = ::windows_core::w!("WMDM/AlbumCoverWidth");
pub const g_wszWMDMAlbumTitle: ::windows_core::PCWSTR = ::windows_core::w!("WMDM/AlbumTitle");
pub const g_wszWMDMAudioBitDepth: ::windows_core::PCWSTR = ::windows_core::w!("WMDM/AudioBitDepth");
pub const g_wszWMDMAuthor: ::windows_core::PCWSTR = ::windows_core::w!("WMDM/Author");
pub const g_wszWMDMAuthorDate: ::windows_core::PCWSTR = ::windows_core::w!("WMDM/AuthorDate");
pub const g_wszWMDMBitRateType: ::windows_core::PCWSTR = ::windows_core::w!("WMDM/BitRateType");
pub const g_wszWMDMBitrate: ::windows_core::PCWSTR = ::windows_core::w!("WMDM/Bitrate");
pub const g_wszWMDMBlockAlignment: ::windows_core::PCWSTR = ::windows_core::w!("WMDM/BlockAlignment");
pub const g_wszWMDMBufferSize: ::windows_core::PCWSTR = ::windows_core::w!("WMDM/BufferSize");
pub const g_wszWMDMBuyNow: ::windows_core::PCWSTR = ::windows_core::w!("WMDM/BuyNow");
pub const g_wszWMDMByteBookmark: ::windows_core::PCWSTR = ::windows_core::w!("WMDM/ByteBookmark");
pub const g_wszWMDMCategory: ::windows_core::PCWSTR = ::windows_core::w!("WMDM/Category");
pub const g_wszWMDMCodec: ::windows_core::PCWSTR = ::windows_core::w!("WMDM/Codec");
pub const g_wszWMDMCollectionID: ::windows_core::PCWSTR = ::windows_core::w!("WMDM/CollectionID");
pub const g_wszWMDMComposer: ::windows_core::PCWSTR = ::windows_core::w!("WMDM/Composer");
pub const g_wszWMDMDRMId: ::windows_core::PCWSTR = ::windows_core::w!("WMDM/DRMId");
pub const g_wszWMDMDataLength: ::windows_core::PCWSTR = ::windows_core::w!("WMDM/DataLength");
pub const g_wszWMDMDataOffset: ::windows_core::PCWSTR = ::windows_core::w!("WMDM/DataOffset");
pub const g_wszWMDMDataUnits: ::windows_core::PCWSTR = ::windows_core::w!("WMDM/DataUnits");
pub const g_wszWMDMDescription: ::windows_core::PCWSTR = ::windows_core::w!("WMDM/Description");
pub const g_wszWMDMDestinationURL: ::windows_core::PCWSTR = ::windows_core::w!("WMDM/DestinationURL");
pub const g_wszWMDMDeviceFirmwareVersion: ::windows_core::PCWSTR = ::windows_core::w!("WMDM/DeviceFirmwareVersion");
pub const g_wszWMDMDeviceFriendlyName: ::windows_core::PCWSTR = ::windows_core::w!("WMDM/DeviceFriendlyName");
pub const g_wszWMDMDeviceModelName: ::windows_core::PCWSTR = ::windows_core::w!("WMDM/DeviceModelName");
pub const g_wszWMDMDevicePlayCount: ::windows_core::PCWSTR = ::windows_core::w!("WMDM/DevicePlayCount");
pub const g_wszWMDMDeviceProtocol: ::windows_core::PCWSTR = ::windows_core::w!("WMDM/DeviceProtocol");
pub const g_wszWMDMDeviceRevocationInfo: ::windows_core::PCWSTR = ::windows_core::w!("WMDM/DeviceRevocationInfo");
pub const g_wszWMDMDeviceServiceProviderVendor: ::windows_core::PCWSTR = ::windows_core::w!("WMDM/DeviceServiceProviderVendor");
pub const g_wszWMDMDeviceVendorExtension: ::windows_core::PCWSTR = ::windows_core::w!("WMDM/DeviceVendorExtension");
pub const g_wszWMDMDuration: ::windows_core::PCWSTR = ::windows_core::w!("WMDM/Duration");
pub const g_wszWMDMEditor: ::windows_core::PCWSTR = ::windows_core::w!("WMDM/Editor");
pub const g_wszWMDMEncodingProfile: ::windows_core::PCWSTR = ::windows_core::w!("WMDM/EncodingProfile");
pub const g_wszWMDMFileAttributes: ::windows_core::PCWSTR = ::windows_core::w!("WMDM/FileAttributes");
pub const g_wszWMDMFileCreationDate: ::windows_core::PCWSTR = ::windows_core::w!("WMDM/FileCreationDate");
pub const g_wszWMDMFileName: ::windows_core::PCWSTR = ::windows_core::w!("WMDM/FileName");
pub const g_wszWMDMFileSize: ::windows_core::PCWSTR = ::windows_core::w!("WMDM/FileSize");
pub const g_wszWMDMFormatCode: ::windows_core::PCWSTR = ::windows_core::w!("WMDM/FormatCode");
pub const g_wszWMDMFormatsSupported: ::windows_core::PCWSTR = ::windows_core::w!("WMDM/FormatsSupported");
pub const g_wszWMDMFormatsSupportedAreOrdered: ::windows_core::PCWSTR = ::windows_core::w!("WMDM/FormatsSupportedAreOrdered");
pub const g_wszWMDMFrameRate: ::windows_core::PCWSTR = ::windows_core::w!("WMDM/FrameRate");
pub const g_wszWMDMGenre: ::windows_core::PCWSTR = ::windows_core::w!("WMDM/Genre");
pub const g_wszWMDMHeight: ::windows_core::PCWSTR = ::windows_core::w!("WMDM/Height");
pub const g_wszWMDMIsProtected: ::windows_core::PCWSTR = ::windows_core::w!("WMDM/IsProtected");
pub const g_wszWMDMIsRepeat: ::windows_core::PCWSTR = ::windows_core::w!("WMDM/IsRepeat");
pub const g_wszWMDMKeyFrameDistance: ::windows_core::PCWSTR = ::windows_core::w!("WMDM/KeyFrameDistance");
pub const g_wszWMDMLastModifiedDate: ::windows_core::PCWSTR = ::windows_core::w!("WMDM/LastModifiedDate");
pub const g_wszWMDMMediaClassSecondaryID: ::windows_core::PCWSTR = ::windows_core::w!("WMDM/MediaClassSecondaryID");
pub const g_wszWMDMMediaCredits: ::windows_core::PCWSTR = ::windows_core::w!("WMDM/MediaCredits");
pub const g_wszWMDMMediaGuid: ::windows_core::PCWSTR = ::windows_core::w!("WMDM/MediaGuid");
pub const g_wszWMDMMediaOriginalBroadcastDateTime: ::windows_core::PCWSTR = ::windows_core::w!("WMDM/MediaOriginalBroadcastDateTime");
pub const g_wszWMDMMediaOriginalChannel: ::windows_core::PCWSTR = ::windows_core::w!("WMDM/MediaOriginalChannel");
pub const g_wszWMDMMediaStationName: ::windows_core::PCWSTR = ::windows_core::w!("WMDM/MediaStationName");
pub const g_wszWMDMMetaGenre: ::windows_core::PCWSTR = ::windows_core::w!("WMDM/MetaGenre");
pub const g_wszWMDMNonConsumable: ::windows_core::PCWSTR = ::windows_core::w!("WMDM/NonConsumable");
pub const g_wszWMDMNumChannels: ::windows_core::PCWSTR = ::windows_core::w!("WMDM/NumChannels");
pub const g_wszWMDMObjectBookmark: ::windows_core::PCWSTR = ::windows_core::w!("WMDM/ObjectBookmark");
pub const g_wszWMDMOwner: ::windows_core::PCWSTR = ::windows_core::w!("WMDM/Owner");
pub const g_wszWMDMParentalRating: ::windows_core::PCWSTR = ::windows_core::w!("WMDM/ParentalRating");
pub const g_wszWMDMPersistentUniqueID: ::windows_core::PCWSTR = ::windows_core::w!("WMDM/PersistentUniqueID");
pub const g_wszWMDMPlayCount: ::windows_core::PCWSTR = ::windows_core::w!("WMDM/PlayCount");
pub const g_wszWMDMProviderCopyright: ::windows_core::PCWSTR = ::windows_core::w!("WMDM/ProviderCopyright");
pub const g_wszWMDMQualitySetting: ::windows_core::PCWSTR = ::windows_core::w!("WMDM/QualitySetting");
pub const g_wszWMDMSampleRate: ::windows_core::PCWSTR = ::windows_core::w!("WMDM/SampleRate");
pub const g_wszWMDMScanType: ::windows_core::PCWSTR = ::windows_core::w!("WMDM/ScanType");
pub const g_wszWMDMSourceURL: ::windows_core::PCWSTR = ::windows_core::w!("WMDM/SourceURL");
pub const g_wszWMDMSubTitle: ::windows_core::PCWSTR = ::windows_core::w!("WMDM/SubTitle");
pub const g_wszWMDMSubTitleDescription: ::windows_core::PCWSTR = ::windows_core::w!("WMDM/SubTitleDescription");
pub const g_wszWMDMSupportedDeviceProperties: ::windows_core::PCWSTR = ::windows_core::w!("WMDM/SupportedDeviceProperties");
pub const g_wszWMDMSyncID: ::windows_core::PCWSTR = ::windows_core::w!("WMDM/SyncID");
pub const g_wszWMDMSyncRelationshipID: ::windows_core::PCWSTR = ::windows_core::w!("WMDM/SyncRelationshipID");
pub const g_wszWMDMSyncTime: ::windows_core::PCWSTR = ::windows_core::w!("WMDM/SyncTime");
pub const g_wszWMDMTimeBookmark: ::windows_core::PCWSTR = ::windows_core::w!("WMDM/TimeBookmark");
pub const g_wszWMDMTimeToLive: ::windows_core::PCWSTR = ::windows_core::w!("WMDM/TimeToLive");
pub const g_wszWMDMTitle: ::windows_core::PCWSTR = ::windows_core::w!("WMDM/Title");
pub const g_wszWMDMTotalBitrate: ::windows_core::PCWSTR = ::windows_core::w!("WMDM/TotalBitrate");
pub const g_wszWMDMTrack: ::windows_core::PCWSTR = ::windows_core::w!("WMDM/Track");
pub const g_wszWMDMTrackMood: ::windows_core::PCWSTR = ::windows_core::w!("WMDM/TrackMood");
pub const g_wszWMDMUserEffectiveRating: ::windows_core::PCWSTR = ::windows_core::w!("WMDM/UserEffectiveRating");
pub const g_wszWMDMUserLastPlayTime: ::windows_core::PCWSTR = ::windows_core::w!("WMDM/UserLastPlayTime");
pub const g_wszWMDMUserRating: ::windows_core::PCWSTR = ::windows_core::w!("WMDM/UserRating");
pub const g_wszWMDMUserRatingOnDevice: ::windows_core::PCWSTR = ::windows_core::w!("WMDM/UserRatingOnDevice");
pub const g_wszWMDMVideoBitrate: ::windows_core::PCWSTR = ::windows_core::w!("WMDM/VideoBitrate");
pub const g_wszWMDMWebmaster: ::windows_core::PCWSTR = ::windows_core::w!("WMDM/Webmaster");
pub const g_wszWMDMWidth: ::windows_core::PCWSTR = ::windows_core::w!("WMDM/Width");
pub const g_wszWMDMYear: ::windows_core::PCWSTR = ::windows_core::w!("WMDM/Year");
pub const g_wszWMDMediaClassPrimaryID: ::windows_core::PCWSTR = ::windows_core::w!("WMDM/MediaClassPrimaryID");
pub const g_wszWPDPassthroughPropertyValues: ::windows_core::PCWSTR = ::windows_core::w!("WPD/PassthroughPropertyValues");
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct WMDMMessage(pub i32);
impl ::core::marker::Copy for WMDMMessage {}
impl ::core::clone::Clone for WMDMMessage {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WMDMMessage {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for WMDMMessage {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for WMDMMessage {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WMDMMessage").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct WMDM_ENUM_PROP_VALID_VALUES_FORM(pub i32);
impl ::core::marker::Copy for WMDM_ENUM_PROP_VALID_VALUES_FORM {}
impl ::core::clone::Clone for WMDM_ENUM_PROP_VALID_VALUES_FORM {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WMDM_ENUM_PROP_VALID_VALUES_FORM {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for WMDM_ENUM_PROP_VALID_VALUES_FORM {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for WMDM_ENUM_PROP_VALID_VALUES_FORM {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WMDM_ENUM_PROP_VALID_VALUES_FORM").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct WMDM_FIND_SCOPE(pub i32);
impl ::core::marker::Copy for WMDM_FIND_SCOPE {}
impl ::core::clone::Clone for WMDM_FIND_SCOPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WMDM_FIND_SCOPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for WMDM_FIND_SCOPE {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for WMDM_FIND_SCOPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WMDM_FIND_SCOPE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct WMDM_FORMATCODE(pub i32);
impl ::core::marker::Copy for WMDM_FORMATCODE {}
impl ::core::clone::Clone for WMDM_FORMATCODE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WMDM_FORMATCODE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for WMDM_FORMATCODE {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for WMDM_FORMATCODE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WMDM_FORMATCODE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct WMDM_SESSION_TYPE(pub i32);
impl ::core::marker::Copy for WMDM_SESSION_TYPE {}
impl ::core::clone::Clone for WMDM_SESSION_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WMDM_SESSION_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for WMDM_SESSION_TYPE {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for WMDM_SESSION_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WMDM_SESSION_TYPE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct WMDM_STORAGE_ENUM_MODE(pub i32);
impl ::core::marker::Copy for WMDM_STORAGE_ENUM_MODE {}
impl ::core::clone::Clone for WMDM_STORAGE_ENUM_MODE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WMDM_STORAGE_ENUM_MODE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for WMDM_STORAGE_ENUM_MODE {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for WMDM_STORAGE_ENUM_MODE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WMDM_STORAGE_ENUM_MODE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct WMDM_TAG_DATATYPE(pub i32);
impl ::core::marker::Copy for WMDM_TAG_DATATYPE {}
impl ::core::clone::Clone for WMDM_TAG_DATATYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WMDM_TAG_DATATYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for WMDM_TAG_DATATYPE {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for WMDM_TAG_DATATYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WMDM_TAG_DATATYPE").field(&self.0).finish()
    }
}
#[repr(C)]
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub struct MACINFO {
    pub fUsed: super::super::Foundation::BOOL,
    pub abMacState: [u8; 36],
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for MACINFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for MACINFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for MACINFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MACINFO").field("fUsed", &self.fUsed).field("abMacState", &self.abMacState).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::TypeKind for MACINFO {
    type TypeKind = ::windows_core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for MACINFO {
    fn eq(&self, other: &Self) -> bool {
        self.fUsed == other.fUsed && self.abMacState == other.abMacState
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for MACINFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for MACINFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
pub struct MTP_COMMAND_DATA_IN {
    pub OpCode: u16,
    pub NumParams: u32,
    pub Params: [u32; 5],
    pub NextPhase: u32,
    pub CommandWriteDataSize: u32,
    pub CommandWriteData: [u8; 1],
}
impl ::core::marker::Copy for MTP_COMMAND_DATA_IN {}
impl ::core::clone::Clone for MTP_COMMAND_DATA_IN {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::windows_core::TypeKind for MTP_COMMAND_DATA_IN {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::default::Default for MTP_COMMAND_DATA_IN {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
pub struct MTP_COMMAND_DATA_OUT {
    pub ResponseCode: u16,
    pub NumParams: u32,
    pub Params: [u32; 5],
    pub CommandReadDataSize: u32,
    pub CommandReadData: [u8; 1],
}
impl ::core::marker::Copy for MTP_COMMAND_DATA_OUT {}
impl ::core::clone::Clone for MTP_COMMAND_DATA_OUT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::windows_core::TypeKind for MTP_COMMAND_DATA_OUT {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::default::Default for MTP_COMMAND_DATA_OUT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct OPAQUECOMMAND {
    pub guidCommand: ::windows_core::GUID,
    pub dwDataLen: u32,
    pub pData: *mut u8,
    pub abMAC: [u8; 20],
}
impl ::core::marker::Copy for OPAQUECOMMAND {}
impl ::core::clone::Clone for OPAQUECOMMAND {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for OPAQUECOMMAND {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("OPAQUECOMMAND").field("guidCommand", &self.guidCommand).field("dwDataLen", &self.dwDataLen).field("pData", &self.pData).field("abMAC", &self.abMAC).finish()
    }
}
impl ::windows_core::TypeKind for OPAQUECOMMAND {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for OPAQUECOMMAND {
    fn eq(&self, other: &Self) -> bool {
        self.guidCommand == other.guidCommand && self.dwDataLen == other.dwDataLen && self.pData == other.pData && self.abMAC == other.abMAC
    }
}
impl ::core::cmp::Eq for OPAQUECOMMAND {}
impl ::core::default::Default for OPAQUECOMMAND {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct WMDMDATETIME {
    pub wYear: u16,
    pub wMonth: u16,
    pub wDay: u16,
    pub wHour: u16,
    pub wMinute: u16,
    pub wSecond: u16,
}
impl ::core::marker::Copy for WMDMDATETIME {}
impl ::core::clone::Clone for WMDMDATETIME {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WMDMDATETIME {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WMDMDATETIME").field("wYear", &self.wYear).field("wMonth", &self.wMonth).field("wDay", &self.wDay).field("wHour", &self.wHour).field("wMinute", &self.wMinute).field("wSecond", &self.wSecond).finish()
    }
}
impl ::windows_core::TypeKind for WMDMDATETIME {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for WMDMDATETIME {
    fn eq(&self, other: &Self) -> bool {
        self.wYear == other.wYear && self.wMonth == other.wMonth && self.wDay == other.wDay && self.wHour == other.wHour && self.wMinute == other.wMinute && self.wSecond == other.wSecond
    }
}
impl ::core::cmp::Eq for WMDMDATETIME {}
impl ::core::default::Default for WMDMDATETIME {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub union WMDMDetermineMaxPropStringLen {
    pub sz001: [u16; 27],
    pub sz002: [u16; 31],
    pub sz003: [u16; 14],
    pub sz004: [u16; 16],
    pub sz005: [u16; 22],
    pub sz006: [u16; 14],
    pub sz007: [u16; 20],
    pub sz008: [u16; 20],
    pub sz009: [u16; 22],
    pub sz010: [u16; 11],
    pub sz011: [u16; 12],
    pub sz012: [u16; 17],
    pub sz013: [u16; 17],
    pub sz014: [u16; 16],
    pub sz015: [u16; 17],
    pub sz016: [u16; 11],
    pub sz017: [u16; 11],
    pub sz018: [u16; 15],
    pub sz019: [u16; 22],
    pub sz020: [u16; 20],
    pub sz021: [u16; 22],
    pub sz022: [u16; 21],
    pub sz023: [u16; 24],
    pub sz024: [u16; 20],
    pub sz025: [u16; 10],
    pub sz026: [u16; 14],
    pub sz027: [u16; 11],
    pub sz028: [u16; 11],
    pub sz029: [u16; 13],
    pub sz030: [u16; 17],
    pub sz031: [u16; 16],
    pub sz032: [u16; 17],
    pub sz033: [u16; 20],
    pub sz034: [u16; 19],
    pub sz035: [u16; 18],
    pub sz036: [u16; 18],
    pub sz037: [u16; 15],
    pub sz041: [u16; 14],
    pub sz043: [u16; 22],
    pub sz044: [u16; 16],
    pub sz045: [u16; 20],
    pub sz046: [u16; 14],
    pub sz047: [u16; 14],
    pub sz048: [u16; 12],
    pub sz049: [u16; 25],
    pub sz050: [u16; 26],
    pub sz051: [u16; 25],
    pub sz052: [u16; 16],
    pub sz053: [u16; 24],
    pub sz054: [u16; 15],
    pub sz055: [u16; 21],
    pub sz056: [u16; 16],
    pub sz057: [u16; 22],
    pub sz058: [u16; 14],
    pub sz059: [u16; 25],
    pub sz060: [u16; 18],
    pub sz061: [u16; 22],
    pub sz062: [u16; 26],
    pub sz063: [u16; 36],
    pub sz064: [u16; 23],
    pub sz065: [u16; 12],
    pub sz066: [u16; 24],
    pub sz067: [u16; 11],
    pub sz068: [u16; 12],
    pub sz069: [u16; 14],
    pub sz070: [u16; 20],
    pub sz071: [u16; 15],
    pub sz072: [u16; 14],
    pub sz073: [u16; 31],
    pub sz074: [u16; 24],
    pub sz075: [u16; 22],
    pub sz076: [u16; 24],
    pub sz077: [u16; 21],
    pub sz078: [u16; 27],
    pub sz079: [u16; 27],
    pub sz080: [u16; 20],
    pub sz081: [u16; 33],
    pub sz082: [u16; 21],
    pub sz083: [u16; 32],
    pub sz084: [u16; 26],
    pub sz085: [u16; 18],
    pub sz086: [u16; 30],
}
impl ::core::marker::Copy for WMDMDetermineMaxPropStringLen {}
impl ::core::clone::Clone for WMDMDetermineMaxPropStringLen {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::windows_core::TypeKind for WMDMDetermineMaxPropStringLen {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::default::Default for WMDMDetermineMaxPropStringLen {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct WMDMID {
    pub cbSize: u32,
    pub dwVendorID: u32,
    pub pID: [u8; 128],
    pub SerialNumberLength: u32,
}
impl ::core::marker::Copy for WMDMID {}
impl ::core::clone::Clone for WMDMID {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WMDMID {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WMDMID").field("cbSize", &self.cbSize).field("dwVendorID", &self.dwVendorID).field("pID", &self.pID).field("SerialNumberLength", &self.SerialNumberLength).finish()
    }
}
impl ::windows_core::TypeKind for WMDMID {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for WMDMID {
    fn eq(&self, other: &Self) -> bool {
        self.cbSize == other.cbSize && self.dwVendorID == other.dwVendorID && self.pID == other.pID && self.SerialNumberLength == other.SerialNumberLength
    }
}
impl ::core::cmp::Eq for WMDMID {}
impl ::core::default::Default for WMDMID {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct WMDMMetadataView {
    pub pwszViewName: ::windows_core::PWSTR,
    pub nDepth: u32,
    pub ppwszTags: *mut *mut u16,
}
impl ::core::marker::Copy for WMDMMetadataView {}
impl ::core::clone::Clone for WMDMMetadataView {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WMDMMetadataView {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WMDMMetadataView").field("pwszViewName", &self.pwszViewName).field("nDepth", &self.nDepth).field("ppwszTags", &self.ppwszTags).finish()
    }
}
impl ::windows_core::TypeKind for WMDMMetadataView {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for WMDMMetadataView {
    fn eq(&self, other: &Self) -> bool {
        self.pwszViewName == other.pwszViewName && self.nDepth == other.nDepth && self.ppwszTags == other.ppwszTags
    }
}
impl ::core::cmp::Eq for WMDMMetadataView {}
impl ::core::default::Default for WMDMMetadataView {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct WMDMRIGHTS {
    pub cbSize: u32,
    pub dwContentType: u32,
    pub fuFlags: u32,
    pub fuRights: u32,
    pub dwAppSec: u32,
    pub dwPlaybackCount: u32,
    pub ExpirationDate: WMDMDATETIME,
}
impl ::core::marker::Copy for WMDMRIGHTS {}
impl ::core::clone::Clone for WMDMRIGHTS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WMDMRIGHTS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WMDMRIGHTS").field("cbSize", &self.cbSize).field("dwContentType", &self.dwContentType).field("fuFlags", &self.fuFlags).field("fuRights", &self.fuRights).field("dwAppSec", &self.dwAppSec).field("dwPlaybackCount", &self.dwPlaybackCount).field("ExpirationDate", &self.ExpirationDate).finish()
    }
}
impl ::windows_core::TypeKind for WMDMRIGHTS {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for WMDMRIGHTS {
    fn eq(&self, other: &Self) -> bool {
        self.cbSize == other.cbSize && self.dwContentType == other.dwContentType && self.fuFlags == other.fuFlags && self.fuRights == other.fuRights && self.dwAppSec == other.dwAppSec && self.dwPlaybackCount == other.dwPlaybackCount && self.ExpirationDate == other.ExpirationDate
    }
}
impl ::core::cmp::Eq for WMDMRIGHTS {}
impl ::core::default::Default for WMDMRIGHTS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com_StructuredStorage\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant"))]
pub struct WMDM_FORMAT_CAPABILITY {
    pub nPropConfig: u32,
    pub pConfigs: *mut WMDM_PROP_CONFIG,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant"))]
impl ::core::marker::Copy for WMDM_FORMAT_CAPABILITY {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant"))]
impl ::core::clone::Clone for WMDM_FORMAT_CAPABILITY {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant"))]
impl ::core::fmt::Debug for WMDM_FORMAT_CAPABILITY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WMDM_FORMAT_CAPABILITY").field("nPropConfig", &self.nPropConfig).field("pConfigs", &self.pConfigs).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant"))]
impl ::windows_core::TypeKind for WMDM_FORMAT_CAPABILITY {
    type TypeKind = ::windows_core::CopyType;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant"))]
impl ::core::cmp::PartialEq for WMDM_FORMAT_CAPABILITY {
    fn eq(&self, other: &Self) -> bool {
        self.nPropConfig == other.nPropConfig && self.pConfigs == other.pConfigs
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant"))]
impl ::core::cmp::Eq for WMDM_FORMAT_CAPABILITY {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant"))]
impl ::core::default::Default for WMDM_FORMAT_CAPABILITY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com_StructuredStorage\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant"))]
pub struct WMDM_PROP_CONFIG {
    pub nPreference: u32,
    pub nPropDesc: u32,
    pub pPropDesc: *mut WMDM_PROP_DESC,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant"))]
impl ::core::marker::Copy for WMDM_PROP_CONFIG {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant"))]
impl ::core::clone::Clone for WMDM_PROP_CONFIG {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant"))]
impl ::core::fmt::Debug for WMDM_PROP_CONFIG {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WMDM_PROP_CONFIG").field("nPreference", &self.nPreference).field("nPropDesc", &self.nPropDesc).field("pPropDesc", &self.pPropDesc).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant"))]
impl ::windows_core::TypeKind for WMDM_PROP_CONFIG {
    type TypeKind = ::windows_core::CopyType;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant"))]
impl ::core::cmp::PartialEq for WMDM_PROP_CONFIG {
    fn eq(&self, other: &Self) -> bool {
        self.nPreference == other.nPreference && self.nPropDesc == other.nPropDesc && self.pPropDesc == other.pPropDesc
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant"))]
impl ::core::cmp::Eq for WMDM_PROP_CONFIG {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant"))]
impl ::core::default::Default for WMDM_PROP_CONFIG {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com_StructuredStorage\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant"))]
pub struct WMDM_PROP_DESC {
    pub pwszPropName: ::windows_core::PWSTR,
    pub ValidValuesForm: WMDM_ENUM_PROP_VALID_VALUES_FORM,
    pub ValidValues: WMDM_PROP_DESC_0,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant"))]
impl ::core::clone::Clone for WMDM_PROP_DESC {
    fn clone(&self) -> Self {
        unsafe { ::core::mem::transmute_copy(self) }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant"))]
impl ::windows_core::TypeKind for WMDM_PROP_DESC {
    type TypeKind = ::windows_core::CopyType;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant"))]
impl ::core::default::Default for WMDM_PROP_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com_StructuredStorage\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant"))]
pub union WMDM_PROP_DESC_0 {
    pub ValidValuesRange: ::std::mem::ManuallyDrop<WMDM_PROP_VALUES_RANGE>,
    pub EnumeratedValidValues: WMDM_PROP_VALUES_ENUM,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant"))]
impl ::core::clone::Clone for WMDM_PROP_DESC_0 {
    fn clone(&self) -> Self {
        unsafe { ::core::mem::transmute_copy(self) }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant"))]
impl ::windows_core::TypeKind for WMDM_PROP_DESC_0 {
    type TypeKind = ::windows_core::CopyType;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant"))]
impl ::core::default::Default for WMDM_PROP_DESC_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com_StructuredStorage\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant"))]
pub struct WMDM_PROP_VALUES_ENUM {
    pub cEnumValues: u32,
    pub pValues: *mut super::super::System::Com::StructuredStorage::PROPVARIANT,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant"))]
impl ::core::marker::Copy for WMDM_PROP_VALUES_ENUM {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant"))]
impl ::core::clone::Clone for WMDM_PROP_VALUES_ENUM {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant"))]
impl ::core::fmt::Debug for WMDM_PROP_VALUES_ENUM {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WMDM_PROP_VALUES_ENUM").field("cEnumValues", &self.cEnumValues).field("pValues", &self.pValues).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant"))]
impl ::windows_core::TypeKind for WMDM_PROP_VALUES_ENUM {
    type TypeKind = ::windows_core::CopyType;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant"))]
impl ::core::cmp::PartialEq for WMDM_PROP_VALUES_ENUM {
    fn eq(&self, other: &Self) -> bool {
        self.cEnumValues == other.cEnumValues && self.pValues == other.pValues
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant"))]
impl ::core::cmp::Eq for WMDM_PROP_VALUES_ENUM {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant"))]
impl ::core::default::Default for WMDM_PROP_VALUES_ENUM {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com_StructuredStorage\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant"))]
pub struct WMDM_PROP_VALUES_RANGE {
    pub rangeMin: super::super::System::Com::StructuredStorage::PROPVARIANT,
    pub rangeMax: super::super::System::Com::StructuredStorage::PROPVARIANT,
    pub rangeStep: super::super::System::Com::StructuredStorage::PROPVARIANT,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant"))]
impl ::core::clone::Clone for WMDM_PROP_VALUES_RANGE {
    fn clone(&self) -> Self {
        unsafe { ::core::mem::transmute_copy(self) }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant"))]
impl ::windows_core::TypeKind for WMDM_PROP_VALUES_RANGE {
    type TypeKind = ::windows_core::CopyType;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant"))]
impl ::core::default::Default for WMDM_PROP_VALUES_RANGE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct WMFILECAPABILITIES {
    pub pwszMimeType: ::windows_core::PWSTR,
    pub dwReserved: u32,
}
impl ::core::marker::Copy for WMFILECAPABILITIES {}
impl ::core::clone::Clone for WMFILECAPABILITIES {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WMFILECAPABILITIES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WMFILECAPABILITIES").field("pwszMimeType", &self.pwszMimeType).field("dwReserved", &self.dwReserved).finish()
    }
}
impl ::windows_core::TypeKind for WMFILECAPABILITIES {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for WMFILECAPABILITIES {
    fn eq(&self, other: &Self) -> bool {
        self.pwszMimeType == other.pwszMimeType && self.dwReserved == other.dwReserved
    }
}
impl ::core::cmp::Eq for WMFILECAPABILITIES {}
impl ::core::default::Default for WMFILECAPABILITIES {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
