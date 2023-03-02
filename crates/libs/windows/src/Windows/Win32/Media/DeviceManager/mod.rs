#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
#[repr(transparent)]
pub struct IComponentAuthenticate(::windows::core::IUnknown);
impl IComponentAuthenticate {
    pub unsafe fn SACAuth(&self, dwprotocolid: u32, dwpass: u32, pbdatain: &[u8], ppbdataout: *mut *mut u8, pdwdataoutlen: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SACAuth)(::windows::core::Interface::as_raw(self), dwprotocolid, dwpass, ::core::mem::transmute(pbdatain.as_ptr()), pbdatain.len() as _, ppbdataout, pdwdataoutlen).ok()
    }
    pub unsafe fn SACGetProtocols(&self, ppdwprotocols: *mut *mut u32, pdwprotocolcount: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SACGetProtocols)(::windows::core::Interface::as_raw(self), ppdwprotocols, pdwprotocolcount).ok()
    }
}
::windows::imp::interface_hierarchy!(IComponentAuthenticate, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IComponentAuthenticate {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IComponentAuthenticate {}
impl ::core::fmt::Debug for IComponentAuthenticate {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IComponentAuthenticate").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IComponentAuthenticate {
    type Vtable = IComponentAuthenticate_Vtbl;
}
impl ::core::clone::Clone for IComponentAuthenticate {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IComponentAuthenticate {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa9889c00_6d2b_11d3_8496_00c04f79dbc0);
}
#[repr(C)]
#[doc(hidden)]
pub struct IComponentAuthenticate_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub SACAuth: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwprotocolid: u32, dwpass: u32, pbdatain: *const u8, dwdatainlen: u32, ppbdataout: *mut *mut u8, pdwdataoutlen: *mut u32) -> ::windows::core::HRESULT,
    pub SACGetProtocols: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppdwprotocols: *mut *mut u32, pdwprotocolcount: *mut u32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
#[repr(transparent)]
pub struct IMDSPDevice(::windows::core::IUnknown);
impl IMDSPDevice {
    pub unsafe fn GetName(&self, pwszname: &mut [u16]) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetName)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pwszname.as_ptr()), pwszname.len() as _).ok()
    }
    pub unsafe fn GetManufacturer(&self, pwszname: &mut [u16]) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetManufacturer)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pwszname.as_ptr()), pwszname.len() as _).ok()
    }
    pub unsafe fn GetVersion(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::windows::core::zeroed::<u32>();
        (::windows::core::Interface::vtable(self).GetVersion)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetType(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::windows::core::zeroed::<u32>();
        (::windows::core::Interface::vtable(self).GetType)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetSerialNumber(&self, pserialnumber: *mut WMDMID, abmac: *mut u8) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetSerialNumber)(::windows::core::Interface::as_raw(self), pserialnumber, abmac).ok()
    }
    pub unsafe fn GetPowerSource(&self, pdwpowersource: *mut u32, pdwpercentremaining: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetPowerSource)(::windows::core::Interface::as_raw(self), pdwpowersource, pdwpercentremaining).ok()
    }
    pub unsafe fn GetStatus(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::windows::core::zeroed::<u32>();
        (::windows::core::Interface::vtable(self).GetStatus)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetDeviceIcon(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::windows::core::zeroed::<u32>();
        (::windows::core::Interface::vtable(self).GetDeviceIcon)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn EnumStorage(&self) -> ::windows::core::Result<IMDSPEnumStorage> {
        let mut result__ = ::windows::core::zeroed::<IMDSPEnumStorage>();
        (::windows::core::Interface::vtable(self).EnumStorage)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
    #[cfg(feature = "Win32_Media_Audio")]
    pub unsafe fn GetFormatSupport(&self, pformatex: *mut *mut super::Audio::WAVEFORMATEX, pnformatcount: *mut u32, pppwszmimetype: *mut *mut ::windows::core::PWSTR, pnmimetypecount: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetFormatSupport)(::windows::core::Interface::as_raw(self), pformatex, pnformatcount, pppwszmimetype, pnmimetypecount).ok()
    }
    pub unsafe fn SendOpaqueCommand(&self, pcommand: *mut OPAQUECOMMAND) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SendOpaqueCommand)(::windows::core::Interface::as_raw(self), pcommand).ok()
    }
}
::windows::imp::interface_hierarchy!(IMDSPDevice, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IMDSPDevice {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IMDSPDevice {}
impl ::core::fmt::Debug for IMDSPDevice {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMDSPDevice").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IMDSPDevice {
    type Vtable = IMDSPDevice_Vtbl;
}
impl ::core::clone::Clone for IMDSPDevice {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IMDSPDevice {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1dcb3a12_33ed_11d3_8470_00c04f79dbc0);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMDSPDevice_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub GetName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwszname: ::windows::core::PWSTR, nmaxchars: u32) -> ::windows::core::HRESULT,
    pub GetManufacturer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwszname: ::windows::core::PWSTR, nmaxchars: u32) -> ::windows::core::HRESULT,
    pub GetVersion: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdwversion: *mut u32) -> ::windows::core::HRESULT,
    pub GetType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdwtype: *mut u32) -> ::windows::core::HRESULT,
    pub GetSerialNumber: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pserialnumber: *mut WMDMID, abmac: *mut u8) -> ::windows::core::HRESULT,
    pub GetPowerSource: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdwpowersource: *mut u32, pdwpercentremaining: *mut u32) -> ::windows::core::HRESULT,
    pub GetStatus: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdwstatus: *mut u32) -> ::windows::core::HRESULT,
    pub GetDeviceIcon: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hicon: *mut u32) -> ::windows::core::HRESULT,
    pub EnumStorage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppenumstorage: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Media_Audio")]
    pub GetFormatSupport: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pformatex: *mut *mut super::Audio::WAVEFORMATEX, pnformatcount: *mut u32, pppwszmimetype: *mut *mut ::windows::core::PWSTR, pnmimetypecount: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Media_Audio"))]
    GetFormatSupport: usize,
    pub SendOpaqueCommand: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcommand: *mut OPAQUECOMMAND) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
#[repr(transparent)]
pub struct IMDSPDevice2(::windows::core::IUnknown);
impl IMDSPDevice2 {
    pub unsafe fn GetName(&self, pwszname: &mut [u16]) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.GetName)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pwszname.as_ptr()), pwszname.len() as _).ok()
    }
    pub unsafe fn GetManufacturer(&self, pwszname: &mut [u16]) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.GetManufacturer)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pwszname.as_ptr()), pwszname.len() as _).ok()
    }
    pub unsafe fn GetVersion(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::windows::core::zeroed::<u32>();
        (::windows::core::Interface::vtable(self).base__.GetVersion)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetType(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::windows::core::zeroed::<u32>();
        (::windows::core::Interface::vtable(self).base__.GetType)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetSerialNumber(&self, pserialnumber: *mut WMDMID, abmac: *mut u8) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.GetSerialNumber)(::windows::core::Interface::as_raw(self), pserialnumber, abmac).ok()
    }
    pub unsafe fn GetPowerSource(&self, pdwpowersource: *mut u32, pdwpercentremaining: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.GetPowerSource)(::windows::core::Interface::as_raw(self), pdwpowersource, pdwpercentremaining).ok()
    }
    pub unsafe fn GetStatus(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::windows::core::zeroed::<u32>();
        (::windows::core::Interface::vtable(self).base__.GetStatus)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetDeviceIcon(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::windows::core::zeroed::<u32>();
        (::windows::core::Interface::vtable(self).base__.GetDeviceIcon)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn EnumStorage(&self) -> ::windows::core::Result<IMDSPEnumStorage> {
        let mut result__ = ::windows::core::zeroed::<IMDSPEnumStorage>();
        (::windows::core::Interface::vtable(self).base__.EnumStorage)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
    #[cfg(feature = "Win32_Media_Audio")]
    pub unsafe fn GetFormatSupport(&self, pformatex: *mut *mut super::Audio::WAVEFORMATEX, pnformatcount: *mut u32, pppwszmimetype: *mut *mut ::windows::core::PWSTR, pnmimetypecount: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.GetFormatSupport)(::windows::core::Interface::as_raw(self), pformatex, pnformatcount, pppwszmimetype, pnmimetypecount).ok()
    }
    pub unsafe fn SendOpaqueCommand(&self, pcommand: *mut OPAQUECOMMAND) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.SendOpaqueCommand)(::windows::core::Interface::as_raw(self), pcommand).ok()
    }
    pub unsafe fn GetStorage<P0>(&self, pszstoragename: P0) -> ::windows::core::Result<IMDSPStorage>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        let mut result__ = ::windows::core::zeroed::<IMDSPStorage>();
        (::windows::core::Interface::vtable(self).GetStorage)(::windows::core::Interface::as_raw(self), pszstoragename.into_param().abi(), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`, `\"Win32_Media_Audio\"`, `\"Win32_Media_MediaFoundation\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_Media_Audio", feature = "Win32_Media_MediaFoundation"))]
    pub unsafe fn GetFormatSupport2(&self, dwflags: u32, ppaudioformatex: *mut *mut super::Audio::WAVEFORMATEX, pnaudioformatcount: *mut u32, ppvideoformatex: *mut *mut super::MediaFoundation::VIDEOINFOHEADER, pnvideoformatcount: *mut u32, ppfiletype: *mut *mut WMFILECAPABILITIES, pnfiletypecount: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetFormatSupport2)(::windows::core::Interface::as_raw(self), dwflags, ppaudioformatex, pnaudioformatcount, ppvideoformatex, pnvideoformatcount, ppfiletype, pnfiletypecount).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Ole\"`*"]
    #[cfg(feature = "Win32_System_Ole")]
    pub unsafe fn GetSpecifyPropertyPages(&self, ppspecifyproppages: *mut ::core::option::Option<super::super::System::Ole::ISpecifyPropertyPages>, pppunknowns: *mut *mut ::core::option::Option<::windows::core::IUnknown>, pcunks: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetSpecifyPropertyPages)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(ppspecifyproppages), pppunknowns, pcunks).ok()
    }
    pub unsafe fn GetCanonicalName(&self, pwszpnpname: &mut [u16]) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetCanonicalName)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pwszpnpname.as_ptr()), pwszpnpname.len() as _).ok()
    }
}
::windows::imp::interface_hierarchy!(IMDSPDevice2, ::windows::core::IUnknown, IMDSPDevice);
impl ::core::cmp::PartialEq for IMDSPDevice2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IMDSPDevice2 {}
impl ::core::fmt::Debug for IMDSPDevice2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMDSPDevice2").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IMDSPDevice2 {
    type Vtable = IMDSPDevice2_Vtbl;
}
impl ::core::clone::Clone for IMDSPDevice2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IMDSPDevice2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x420d16ad_c97d_4e00_82aa_00e9f4335ddd);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMDSPDevice2_Vtbl {
    pub base__: IMDSPDevice_Vtbl,
    pub GetStorage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszstoragename: ::windows::core::PCWSTR, ppstorage: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_Media_Audio", feature = "Win32_Media_MediaFoundation"))]
    pub GetFormatSupport2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwflags: u32, ppaudioformatex: *mut *mut super::Audio::WAVEFORMATEX, pnaudioformatcount: *mut u32, ppvideoformatex: *mut *mut super::MediaFoundation::VIDEOINFOHEADER, pnvideoformatcount: *mut u32, ppfiletype: *mut *mut WMFILECAPABILITIES, pnfiletypecount: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_Media_Audio", feature = "Win32_Media_MediaFoundation")))]
    GetFormatSupport2: usize,
    #[cfg(feature = "Win32_System_Ole")]
    pub GetSpecifyPropertyPages: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppspecifyproppages: *mut *mut ::core::ffi::c_void, pppunknowns: *mut *mut ::core::option::Option<::windows::core::IUnknown>, pcunks: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Ole"))]
    GetSpecifyPropertyPages: usize,
    pub GetCanonicalName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwszpnpname: ::windows::core::PWSTR, nmaxchars: u32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
#[repr(transparent)]
pub struct IMDSPDevice3(::windows::core::IUnknown);
impl IMDSPDevice3 {
    pub unsafe fn GetName(&self, pwszname: &mut [u16]) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.GetName)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pwszname.as_ptr()), pwszname.len() as _).ok()
    }
    pub unsafe fn GetManufacturer(&self, pwszname: &mut [u16]) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.GetManufacturer)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pwszname.as_ptr()), pwszname.len() as _).ok()
    }
    pub unsafe fn GetVersion(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::windows::core::zeroed::<u32>();
        (::windows::core::Interface::vtable(self).base__.base__.GetVersion)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetType(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::windows::core::zeroed::<u32>();
        (::windows::core::Interface::vtable(self).base__.base__.GetType)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetSerialNumber(&self, pserialnumber: *mut WMDMID, abmac: *mut u8) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.GetSerialNumber)(::windows::core::Interface::as_raw(self), pserialnumber, abmac).ok()
    }
    pub unsafe fn GetPowerSource(&self, pdwpowersource: *mut u32, pdwpercentremaining: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.GetPowerSource)(::windows::core::Interface::as_raw(self), pdwpowersource, pdwpercentremaining).ok()
    }
    pub unsafe fn GetStatus(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::windows::core::zeroed::<u32>();
        (::windows::core::Interface::vtable(self).base__.base__.GetStatus)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetDeviceIcon(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::windows::core::zeroed::<u32>();
        (::windows::core::Interface::vtable(self).base__.base__.GetDeviceIcon)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn EnumStorage(&self) -> ::windows::core::Result<IMDSPEnumStorage> {
        let mut result__ = ::windows::core::zeroed::<IMDSPEnumStorage>();
        (::windows::core::Interface::vtable(self).base__.base__.EnumStorage)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
    #[cfg(feature = "Win32_Media_Audio")]
    pub unsafe fn GetFormatSupport(&self, pformatex: *mut *mut super::Audio::WAVEFORMATEX, pnformatcount: *mut u32, pppwszmimetype: *mut *mut ::windows::core::PWSTR, pnmimetypecount: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.GetFormatSupport)(::windows::core::Interface::as_raw(self), pformatex, pnformatcount, pppwszmimetype, pnmimetypecount).ok()
    }
    pub unsafe fn SendOpaqueCommand(&self, pcommand: *mut OPAQUECOMMAND) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.SendOpaqueCommand)(::windows::core::Interface::as_raw(self), pcommand).ok()
    }
    pub unsafe fn GetStorage<P0>(&self, pszstoragename: P0) -> ::windows::core::Result<IMDSPStorage>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        let mut result__ = ::windows::core::zeroed::<IMDSPStorage>();
        (::windows::core::Interface::vtable(self).base__.GetStorage)(::windows::core::Interface::as_raw(self), pszstoragename.into_param().abi(), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`, `\"Win32_Media_Audio\"`, `\"Win32_Media_MediaFoundation\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_Media_Audio", feature = "Win32_Media_MediaFoundation"))]
    pub unsafe fn GetFormatSupport2(&self, dwflags: u32, ppaudioformatex: *mut *mut super::Audio::WAVEFORMATEX, pnaudioformatcount: *mut u32, ppvideoformatex: *mut *mut super::MediaFoundation::VIDEOINFOHEADER, pnvideoformatcount: *mut u32, ppfiletype: *mut *mut WMFILECAPABILITIES, pnfiletypecount: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.GetFormatSupport2)(::windows::core::Interface::as_raw(self), dwflags, ppaudioformatex, pnaudioformatcount, ppvideoformatex, pnvideoformatcount, ppfiletype, pnfiletypecount).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Ole\"`*"]
    #[cfg(feature = "Win32_System_Ole")]
    pub unsafe fn GetSpecifyPropertyPages(&self, ppspecifyproppages: *mut ::core::option::Option<super::super::System::Ole::ISpecifyPropertyPages>, pppunknowns: *mut *mut ::core::option::Option<::windows::core::IUnknown>, pcunks: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.GetSpecifyPropertyPages)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(ppspecifyproppages), pppunknowns, pcunks).ok()
    }
    pub unsafe fn GetCanonicalName(&self, pwszpnpname: &mut [u16]) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.GetCanonicalName)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pwszpnpname.as_ptr()), pwszpnpname.len() as _).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com_StructuredStorage\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
    pub unsafe fn GetProperty<P0>(&self, pwszpropname: P0) -> ::windows::core::Result<super::super::System::Com::StructuredStorage::PROPVARIANT>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        let mut result__ = ::windows::core::zeroed::<super::super::System::Com::StructuredStorage::PROPVARIANT>();
        (::windows::core::Interface::vtable(self).GetProperty)(::windows::core::Interface::as_raw(self), pwszpropname.into_param().abi(), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com_StructuredStorage\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
    pub unsafe fn SetProperty<P0>(&self, pwszpropname: P0, pvalue: *const super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).SetProperty)(::windows::core::Interface::as_raw(self), pwszpropname.into_param().abi(), pvalue).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com_StructuredStorage\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
    pub unsafe fn GetFormatCapability(&self, format: WMDM_FORMATCODE) -> ::windows::core::Result<WMDM_FORMAT_CAPABILITY> {
        let mut result__ = ::windows::core::zeroed::<WMDM_FORMAT_CAPABILITY>();
        (::windows::core::Interface::vtable(self).GetFormatCapability)(::windows::core::Interface::as_raw(self), format, &mut result__).from_abi(result__)
    }
    pub unsafe fn DeviceIoControl(&self, dwiocontrolcode: u32, lpinbuffer: &[u8], lpoutbuffer: *mut u8, pnoutbuffersize: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).DeviceIoControl)(::windows::core::Interface::as_raw(self), dwiocontrolcode, ::core::mem::transmute(lpinbuffer.as_ptr()), lpinbuffer.len() as _, lpoutbuffer, pnoutbuffersize).ok()
    }
    pub unsafe fn FindStorage<P0>(&self, findscope: WMDM_FIND_SCOPE, pwszuniqueid: P0) -> ::windows::core::Result<IMDSPStorage>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        let mut result__ = ::windows::core::zeroed::<IMDSPStorage>();
        (::windows::core::Interface::vtable(self).FindStorage)(::windows::core::Interface::as_raw(self), findscope, pwszuniqueid.into_param().abi(), &mut result__).from_abi(result__)
    }
}
::windows::imp::interface_hierarchy!(IMDSPDevice3, ::windows::core::IUnknown, IMDSPDevice, IMDSPDevice2);
impl ::core::cmp::PartialEq for IMDSPDevice3 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IMDSPDevice3 {}
impl ::core::fmt::Debug for IMDSPDevice3 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMDSPDevice3").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IMDSPDevice3 {
    type Vtable = IMDSPDevice3_Vtbl;
}
impl ::core::clone::Clone for IMDSPDevice3 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IMDSPDevice3 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1a839845_fc55_487c_976f_ee38ac0e8c4e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMDSPDevice3_Vtbl {
    pub base__: IMDSPDevice2_Vtbl,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
    pub GetProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwszpropname: ::windows::core::PCWSTR, pvalue: *mut super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage")))]
    GetProperty: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
    pub SetProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwszpropname: ::windows::core::PCWSTR, pvalue: *const super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage")))]
    SetProperty: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
    pub GetFormatCapability: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, format: WMDM_FORMATCODE, pformatsupport: *mut WMDM_FORMAT_CAPABILITY) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage")))]
    GetFormatCapability: usize,
    pub DeviceIoControl: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwiocontrolcode: u32, lpinbuffer: *const u8, ninbuffersize: u32, lpoutbuffer: *mut u8, pnoutbuffersize: *mut u32) -> ::windows::core::HRESULT,
    pub FindStorage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, findscope: WMDM_FIND_SCOPE, pwszuniqueid: ::windows::core::PCWSTR, ppstorage: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
#[repr(transparent)]
pub struct IMDSPDeviceControl(::windows::core::IUnknown);
impl IMDSPDeviceControl {
    pub unsafe fn GetDCStatus(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::windows::core::zeroed::<u32>();
        (::windows::core::Interface::vtable(self).GetDCStatus)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetCapabilities(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::windows::core::zeroed::<u32>();
        (::windows::core::Interface::vtable(self).GetCapabilities)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn Play(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Play)(::windows::core::Interface::as_raw(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
    #[cfg(feature = "Win32_Media_Audio")]
    pub unsafe fn Record(&self, pformat: *const super::Audio::WAVEFORMATEX) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Record)(::windows::core::Interface::as_raw(self), pformat).ok()
    }
    pub unsafe fn Pause(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Pause)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Resume(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Resume)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Stop(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Stop)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Seek(&self, fumode: u32, noffset: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Seek)(::windows::core::Interface::as_raw(self), fumode, noffset).ok()
    }
}
::windows::imp::interface_hierarchy!(IMDSPDeviceControl, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IMDSPDeviceControl {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IMDSPDeviceControl {}
impl ::core::fmt::Debug for IMDSPDeviceControl {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMDSPDeviceControl").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IMDSPDeviceControl {
    type Vtable = IMDSPDeviceControl_Vtbl;
}
impl ::core::clone::Clone for IMDSPDeviceControl {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IMDSPDeviceControl {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1dcb3a14_33ed_11d3_8470_00c04f79dbc0);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMDSPDeviceControl_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub GetDCStatus: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdwstatus: *mut u32) -> ::windows::core::HRESULT,
    pub GetCapabilities: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdwcapabilitiesmask: *mut u32) -> ::windows::core::HRESULT,
    pub Play: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Media_Audio")]
    pub Record: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pformat: *const super::Audio::WAVEFORMATEX) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Media_Audio"))]
    Record: usize,
    pub Pause: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Resume: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Stop: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Seek: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, fumode: u32, noffset: i32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
#[repr(transparent)]
pub struct IMDSPDirectTransfer(::windows::core::IUnknown);
impl IMDSPDirectTransfer {
    pub unsafe fn TransferToDevice<P0, P1, P2, P3, P4>(&self, pwszsourcefilepath: P0, psourceoperation: P1, fuflags: u32, pwszdestinationname: P2, psourcemetadata: P3, ptransferprogress: P4) -> ::windows::core::Result<IMDSPStorage>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
        P1: ::windows::core::IntoParam<IWMDMOperation>,
        P2: ::windows::core::IntoParam<::windows::core::PCWSTR>,
        P3: ::windows::core::IntoParam<IWMDMMetaData>,
        P4: ::windows::core::IntoParam<IWMDMProgress>,
    {
        let mut result__ = ::windows::core::zeroed::<IMDSPStorage>();
        (::windows::core::Interface::vtable(self).TransferToDevice)(::windows::core::Interface::as_raw(self), pwszsourcefilepath.into_param().abi(), psourceoperation.into_param().abi(), fuflags, pwszdestinationname.into_param().abi(), psourcemetadata.into_param().abi(), ptransferprogress.into_param().abi(), &mut result__).from_abi(result__)
    }
}
::windows::imp::interface_hierarchy!(IMDSPDirectTransfer, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IMDSPDirectTransfer {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IMDSPDirectTransfer {}
impl ::core::fmt::Debug for IMDSPDirectTransfer {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMDSPDirectTransfer").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IMDSPDirectTransfer {
    type Vtable = IMDSPDirectTransfer_Vtbl;
}
impl ::core::clone::Clone for IMDSPDirectTransfer {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IMDSPDirectTransfer {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc2fe57a8_9304_478c_9ee4_47e397b912d7);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMDSPDirectTransfer_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub TransferToDevice: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwszsourcefilepath: ::windows::core::PCWSTR, psourceoperation: *mut ::core::ffi::c_void, fuflags: u32, pwszdestinationname: ::windows::core::PCWSTR, psourcemetadata: *mut ::core::ffi::c_void, ptransferprogress: *mut ::core::ffi::c_void, ppnewobject: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
#[repr(transparent)]
pub struct IMDSPEnumDevice(::windows::core::IUnknown);
impl IMDSPEnumDevice {
    pub unsafe fn Next(&self, celt: u32, ppdevice: *mut ::core::option::Option<IMDSPDevice>, pceltfetched: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Next)(::windows::core::Interface::as_raw(self), celt, ::core::mem::transmute(ppdevice), pceltfetched).ok()
    }
    pub unsafe fn Skip(&self, celt: u32) -> ::windows::core::Result<u32> {
        let mut result__ = ::windows::core::zeroed::<u32>();
        (::windows::core::Interface::vtable(self).Skip)(::windows::core::Interface::as_raw(self), celt, &mut result__).from_abi(result__)
    }
    pub unsafe fn Reset(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Reset)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Clone(&self) -> ::windows::core::Result<IMDSPEnumDevice> {
        let mut result__ = ::windows::core::zeroed::<IMDSPEnumDevice>();
        (::windows::core::Interface::vtable(self).Clone)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
::windows::imp::interface_hierarchy!(IMDSPEnumDevice, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IMDSPEnumDevice {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IMDSPEnumDevice {}
impl ::core::fmt::Debug for IMDSPEnumDevice {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMDSPEnumDevice").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IMDSPEnumDevice {
    type Vtable = IMDSPEnumDevice_Vtbl;
}
impl ::core::clone::Clone for IMDSPEnumDevice {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IMDSPEnumDevice {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1dcb3a11_33ed_11d3_8470_00c04f79dbc0);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMDSPEnumDevice_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub Next: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, celt: u32, ppdevice: *mut *mut ::core::ffi::c_void, pceltfetched: *mut u32) -> ::windows::core::HRESULT,
    pub Skip: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, celt: u32, pceltfetched: *mut u32) -> ::windows::core::HRESULT,
    pub Reset: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Clone: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppenumdevice: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
#[repr(transparent)]
pub struct IMDSPEnumStorage(::windows::core::IUnknown);
impl IMDSPEnumStorage {
    pub unsafe fn Next(&self, celt: u32, ppstorage: *mut ::core::option::Option<IMDSPStorage>, pceltfetched: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Next)(::windows::core::Interface::as_raw(self), celt, ::core::mem::transmute(ppstorage), pceltfetched).ok()
    }
    pub unsafe fn Skip(&self, celt: u32) -> ::windows::core::Result<u32> {
        let mut result__ = ::windows::core::zeroed::<u32>();
        (::windows::core::Interface::vtable(self).Skip)(::windows::core::Interface::as_raw(self), celt, &mut result__).from_abi(result__)
    }
    pub unsafe fn Reset(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Reset)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Clone(&self) -> ::windows::core::Result<IMDSPEnumStorage> {
        let mut result__ = ::windows::core::zeroed::<IMDSPEnumStorage>();
        (::windows::core::Interface::vtable(self).Clone)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
::windows::imp::interface_hierarchy!(IMDSPEnumStorage, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IMDSPEnumStorage {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IMDSPEnumStorage {}
impl ::core::fmt::Debug for IMDSPEnumStorage {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMDSPEnumStorage").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IMDSPEnumStorage {
    type Vtable = IMDSPEnumStorage_Vtbl;
}
impl ::core::clone::Clone for IMDSPEnumStorage {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IMDSPEnumStorage {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1dcb3a15_33ed_11d3_8470_00c04f79dbc0);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMDSPEnumStorage_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub Next: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, celt: u32, ppstorage: *mut *mut ::core::ffi::c_void, pceltfetched: *mut u32) -> ::windows::core::HRESULT,
    pub Skip: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, celt: u32, pceltfetched: *mut u32) -> ::windows::core::HRESULT,
    pub Reset: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Clone: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppenumstorage: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
#[repr(transparent)]
pub struct IMDSPObject(::windows::core::IUnknown);
impl IMDSPObject {
    pub unsafe fn Open(&self, fumode: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Open)(::windows::core::Interface::as_raw(self), fumode).ok()
    }
    pub unsafe fn Read(&self, pdata: *mut u8, pdwsize: *mut u32, abmac: *mut u8) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Read)(::windows::core::Interface::as_raw(self), pdata, pdwsize, abmac).ok()
    }
    pub unsafe fn Write(&self, pdata: *const u8, pdwsize: *mut u32, abmac: *mut u8) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Write)(::windows::core::Interface::as_raw(self), pdata, pdwsize, abmac).ok()
    }
    pub unsafe fn Delete<P0>(&self, fumode: u32, pprogress: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<IWMDMProgress>,
    {
        (::windows::core::Interface::vtable(self).Delete)(::windows::core::Interface::as_raw(self), fumode, pprogress.into_param().abi()).ok()
    }
    pub unsafe fn Seek(&self, fuflags: u32, dwoffset: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Seek)(::windows::core::Interface::as_raw(self), fuflags, dwoffset).ok()
    }
    pub unsafe fn Rename<P0, P1>(&self, pwsznewname: P0, pprogress: P1) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
        P1: ::windows::core::IntoParam<IWMDMProgress>,
    {
        (::windows::core::Interface::vtable(self).Rename)(::windows::core::Interface::as_raw(self), pwsznewname.into_param().abi(), pprogress.into_param().abi()).ok()
    }
    pub unsafe fn Move<P0, P1>(&self, fumode: u32, pprogress: P0, ptarget: P1) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<IWMDMProgress>,
        P1: ::windows::core::IntoParam<IMDSPStorage>,
    {
        (::windows::core::Interface::vtable(self).Move)(::windows::core::Interface::as_raw(self), fumode, pprogress.into_param().abi(), ptarget.into_param().abi()).ok()
    }
    pub unsafe fn Close(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Close)(::windows::core::Interface::as_raw(self)).ok()
    }
}
::windows::imp::interface_hierarchy!(IMDSPObject, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IMDSPObject {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IMDSPObject {}
impl ::core::fmt::Debug for IMDSPObject {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMDSPObject").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IMDSPObject {
    type Vtable = IMDSPObject_Vtbl;
}
impl ::core::clone::Clone for IMDSPObject {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IMDSPObject {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1dcb3a18_33ed_11d3_8470_00c04f79dbc0);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMDSPObject_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub Open: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, fumode: u32) -> ::windows::core::HRESULT,
    pub Read: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdata: *mut u8, pdwsize: *mut u32, abmac: *mut u8) -> ::windows::core::HRESULT,
    pub Write: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdata: *const u8, pdwsize: *mut u32, abmac: *mut u8) -> ::windows::core::HRESULT,
    pub Delete: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, fumode: u32, pprogress: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Seek: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, fuflags: u32, dwoffset: u32) -> ::windows::core::HRESULT,
    pub Rename: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwsznewname: ::windows::core::PCWSTR, pprogress: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Move: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, fumode: u32, pprogress: *mut ::core::ffi::c_void, ptarget: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Close: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
#[repr(transparent)]
pub struct IMDSPObject2(::windows::core::IUnknown);
impl IMDSPObject2 {
    pub unsafe fn Open(&self, fumode: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.Open)(::windows::core::Interface::as_raw(self), fumode).ok()
    }
    pub unsafe fn Read(&self, pdata: *mut u8, pdwsize: *mut u32, abmac: *mut u8) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.Read)(::windows::core::Interface::as_raw(self), pdata, pdwsize, abmac).ok()
    }
    pub unsafe fn Write(&self, pdata: *const u8, pdwsize: *mut u32, abmac: *mut u8) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.Write)(::windows::core::Interface::as_raw(self), pdata, pdwsize, abmac).ok()
    }
    pub unsafe fn Delete<P0>(&self, fumode: u32, pprogress: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<IWMDMProgress>,
    {
        (::windows::core::Interface::vtable(self).base__.Delete)(::windows::core::Interface::as_raw(self), fumode, pprogress.into_param().abi()).ok()
    }
    pub unsafe fn Seek(&self, fuflags: u32, dwoffset: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.Seek)(::windows::core::Interface::as_raw(self), fuflags, dwoffset).ok()
    }
    pub unsafe fn Rename<P0, P1>(&self, pwsznewname: P0, pprogress: P1) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
        P1: ::windows::core::IntoParam<IWMDMProgress>,
    {
        (::windows::core::Interface::vtable(self).base__.Rename)(::windows::core::Interface::as_raw(self), pwsznewname.into_param().abi(), pprogress.into_param().abi()).ok()
    }
    pub unsafe fn Move<P0, P1>(&self, fumode: u32, pprogress: P0, ptarget: P1) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<IWMDMProgress>,
        P1: ::windows::core::IntoParam<IMDSPStorage>,
    {
        (::windows::core::Interface::vtable(self).base__.Move)(::windows::core::Interface::as_raw(self), fumode, pprogress.into_param().abi(), ptarget.into_param().abi()).ok()
    }
    pub unsafe fn Close(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.Close)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn ReadOnClearChannel(&self, pdata: *mut u8, pdwsize: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).ReadOnClearChannel)(::windows::core::Interface::as_raw(self), pdata, pdwsize).ok()
    }
    pub unsafe fn WriteOnClearChannel(&self, pdata: *const u8, pdwsize: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).WriteOnClearChannel)(::windows::core::Interface::as_raw(self), pdata, pdwsize).ok()
    }
}
::windows::imp::interface_hierarchy!(IMDSPObject2, ::windows::core::IUnknown, IMDSPObject);
impl ::core::cmp::PartialEq for IMDSPObject2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IMDSPObject2 {}
impl ::core::fmt::Debug for IMDSPObject2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMDSPObject2").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IMDSPObject2 {
    type Vtable = IMDSPObject2_Vtbl;
}
impl ::core::clone::Clone for IMDSPObject2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IMDSPObject2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3f34cd3e_5907_4341_9af9_97f4187c3aa5);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMDSPObject2_Vtbl {
    pub base__: IMDSPObject_Vtbl,
    pub ReadOnClearChannel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdata: *mut u8, pdwsize: *mut u32) -> ::windows::core::HRESULT,
    pub WriteOnClearChannel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdata: *const u8, pdwsize: *mut u32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
#[repr(transparent)]
pub struct IMDSPObjectInfo(::windows::core::IUnknown);
impl IMDSPObjectInfo {
    pub unsafe fn GetPlayLength(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::windows::core::zeroed::<u32>();
        (::windows::core::Interface::vtable(self).GetPlayLength)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetPlayLength(&self, dwlength: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetPlayLength)(::windows::core::Interface::as_raw(self), dwlength).ok()
    }
    pub unsafe fn GetPlayOffset(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::windows::core::zeroed::<u32>();
        (::windows::core::Interface::vtable(self).GetPlayOffset)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetPlayOffset(&self, dwoffset: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetPlayOffset)(::windows::core::Interface::as_raw(self), dwoffset).ok()
    }
    pub unsafe fn GetTotalLength(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::windows::core::zeroed::<u32>();
        (::windows::core::Interface::vtable(self).GetTotalLength)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetLastPlayPosition(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::windows::core::zeroed::<u32>();
        (::windows::core::Interface::vtable(self).GetLastPlayPosition)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetLongestPlayPosition(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::windows::core::zeroed::<u32>();
        (::windows::core::Interface::vtable(self).GetLongestPlayPosition)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
::windows::imp::interface_hierarchy!(IMDSPObjectInfo, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IMDSPObjectInfo {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IMDSPObjectInfo {}
impl ::core::fmt::Debug for IMDSPObjectInfo {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMDSPObjectInfo").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IMDSPObjectInfo {
    type Vtable = IMDSPObjectInfo_Vtbl;
}
impl ::core::clone::Clone for IMDSPObjectInfo {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IMDSPObjectInfo {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1dcb3a19_33ed_11d3_8470_00c04f79dbc0);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMDSPObjectInfo_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub GetPlayLength: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdwlength: *mut u32) -> ::windows::core::HRESULT,
    pub SetPlayLength: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwlength: u32) -> ::windows::core::HRESULT,
    pub GetPlayOffset: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdwoffset: *mut u32) -> ::windows::core::HRESULT,
    pub SetPlayOffset: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwoffset: u32) -> ::windows::core::HRESULT,
    pub GetTotalLength: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdwlength: *mut u32) -> ::windows::core::HRESULT,
    pub GetLastPlayPosition: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdwlastpos: *mut u32) -> ::windows::core::HRESULT,
    pub GetLongestPlayPosition: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdwlongestpos: *mut u32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
#[repr(transparent)]
pub struct IMDSPRevoked(::windows::core::IUnknown);
impl IMDSPRevoked {
    pub unsafe fn GetRevocationURL(&self, ppwszrevocationurl: *mut ::windows::core::PWSTR, pdwbufferlen: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetRevocationURL)(::windows::core::Interface::as_raw(self), ppwszrevocationurl, pdwbufferlen).ok()
    }
}
::windows::imp::interface_hierarchy!(IMDSPRevoked, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IMDSPRevoked {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IMDSPRevoked {}
impl ::core::fmt::Debug for IMDSPRevoked {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMDSPRevoked").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IMDSPRevoked {
    type Vtable = IMDSPRevoked_Vtbl;
}
impl ::core::clone::Clone for IMDSPRevoked {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IMDSPRevoked {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa4e8f2d4_3f31_464d_b53d_4fc335998184);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMDSPRevoked_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub GetRevocationURL: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppwszrevocationurl: *mut ::windows::core::PWSTR, pdwbufferlen: *mut u32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
#[repr(transparent)]
pub struct IMDSPStorage(::windows::core::IUnknown);
impl IMDSPStorage {
    #[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
    #[cfg(feature = "Win32_Media_Audio")]
    pub unsafe fn SetAttributes(&self, dwattributes: u32, pformat: ::core::option::Option<*const super::Audio::WAVEFORMATEX>) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetAttributes)(::windows::core::Interface::as_raw(self), dwattributes, ::core::mem::transmute(pformat.unwrap_or(::std::ptr::null()))).ok()
    }
    pub unsafe fn GetStorageGlobals(&self) -> ::windows::core::Result<IMDSPStorageGlobals> {
        let mut result__ = ::windows::core::zeroed::<IMDSPStorageGlobals>();
        (::windows::core::Interface::vtable(self).GetStorageGlobals)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
    #[cfg(feature = "Win32_Media_Audio")]
    pub unsafe fn GetAttributes(&self, pdwattributes: *mut u32, pformat: ::core::option::Option<*mut super::Audio::WAVEFORMATEX>) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetAttributes)(::windows::core::Interface::as_raw(self), pdwattributes, ::core::mem::transmute(pformat.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    pub unsafe fn GetName(&self, pwszname: &mut [u16]) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetName)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pwszname.as_ptr()), pwszname.len() as _).ok()
    }
    pub unsafe fn GetDate(&self) -> ::windows::core::Result<WMDMDATETIME> {
        let mut result__ = ::windows::core::zeroed::<WMDMDATETIME>();
        (::windows::core::Interface::vtable(self).GetDate)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetSize(&self, pdwsizelow: *mut u32, pdwsizehigh: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetSize)(::windows::core::Interface::as_raw(self), pdwsizelow, pdwsizehigh).ok()
    }
    pub unsafe fn GetRights(&self, pprights: *mut *mut WMDMRIGHTS, pnrightscount: *mut u32, abmac: *mut u8) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetRights)(::windows::core::Interface::as_raw(self), pprights, pnrightscount, abmac).ok()
    }
    #[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
    #[cfg(feature = "Win32_Media_Audio")]
    pub unsafe fn CreateStorage<P0>(&self, dwattributes: u32, pformat: ::core::option::Option<*const super::Audio::WAVEFORMATEX>, pwszname: P0) -> ::windows::core::Result<IMDSPStorage>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        let mut result__ = ::windows::core::zeroed::<IMDSPStorage>();
        (::windows::core::Interface::vtable(self).CreateStorage)(::windows::core::Interface::as_raw(self), dwattributes, ::core::mem::transmute(pformat.unwrap_or(::std::ptr::null())), pwszname.into_param().abi(), &mut result__).from_abi(result__)
    }
    pub unsafe fn EnumStorage(&self) -> ::windows::core::Result<IMDSPEnumStorage> {
        let mut result__ = ::windows::core::zeroed::<IMDSPEnumStorage>();
        (::windows::core::Interface::vtable(self).EnumStorage)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SendOpaqueCommand(&self, pcommand: *mut OPAQUECOMMAND) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SendOpaqueCommand)(::windows::core::Interface::as_raw(self), pcommand).ok()
    }
}
::windows::imp::interface_hierarchy!(IMDSPStorage, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IMDSPStorage {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IMDSPStorage {}
impl ::core::fmt::Debug for IMDSPStorage {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMDSPStorage").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IMDSPStorage {
    type Vtable = IMDSPStorage_Vtbl;
}
impl ::core::clone::Clone for IMDSPStorage {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IMDSPStorage {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1dcb3a16_33ed_11d3_8470_00c04f79dbc0);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMDSPStorage_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_Media_Audio")]
    pub SetAttributes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwattributes: u32, pformat: *const super::Audio::WAVEFORMATEX) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Media_Audio"))]
    SetAttributes: usize,
    pub GetStorageGlobals: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppstorageglobals: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Media_Audio")]
    pub GetAttributes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdwattributes: *mut u32, pformat: *mut super::Audio::WAVEFORMATEX) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Media_Audio"))]
    GetAttributes: usize,
    pub GetName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwszname: ::windows::core::PWSTR, nmaxchars: u32) -> ::windows::core::HRESULT,
    pub GetDate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdatetimeutc: *mut WMDMDATETIME) -> ::windows::core::HRESULT,
    pub GetSize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdwsizelow: *mut u32, pdwsizehigh: *mut u32) -> ::windows::core::HRESULT,
    pub GetRights: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pprights: *mut *mut WMDMRIGHTS, pnrightscount: *mut u32, abmac: *mut u8) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Media_Audio")]
    pub CreateStorage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwattributes: u32, pformat: *const super::Audio::WAVEFORMATEX, pwszname: ::windows::core::PCWSTR, ppnewstorage: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Media_Audio"))]
    CreateStorage: usize,
    pub EnumStorage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppenumstorage: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SendOpaqueCommand: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcommand: *mut OPAQUECOMMAND) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
#[repr(transparent)]
pub struct IMDSPStorage2(::windows::core::IUnknown);
impl IMDSPStorage2 {
    #[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
    #[cfg(feature = "Win32_Media_Audio")]
    pub unsafe fn SetAttributes(&self, dwattributes: u32, pformat: ::core::option::Option<*const super::Audio::WAVEFORMATEX>) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.SetAttributes)(::windows::core::Interface::as_raw(self), dwattributes, ::core::mem::transmute(pformat.unwrap_or(::std::ptr::null()))).ok()
    }
    pub unsafe fn GetStorageGlobals(&self) -> ::windows::core::Result<IMDSPStorageGlobals> {
        let mut result__ = ::windows::core::zeroed::<IMDSPStorageGlobals>();
        (::windows::core::Interface::vtable(self).base__.GetStorageGlobals)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
    #[cfg(feature = "Win32_Media_Audio")]
    pub unsafe fn GetAttributes(&self, pdwattributes: *mut u32, pformat: ::core::option::Option<*mut super::Audio::WAVEFORMATEX>) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.GetAttributes)(::windows::core::Interface::as_raw(self), pdwattributes, ::core::mem::transmute(pformat.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    pub unsafe fn GetName(&self, pwszname: &mut [u16]) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.GetName)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pwszname.as_ptr()), pwszname.len() as _).ok()
    }
    pub unsafe fn GetDate(&self) -> ::windows::core::Result<WMDMDATETIME> {
        let mut result__ = ::windows::core::zeroed::<WMDMDATETIME>();
        (::windows::core::Interface::vtable(self).base__.GetDate)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetSize(&self, pdwsizelow: *mut u32, pdwsizehigh: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.GetSize)(::windows::core::Interface::as_raw(self), pdwsizelow, pdwsizehigh).ok()
    }
    pub unsafe fn GetRights(&self, pprights: *mut *mut WMDMRIGHTS, pnrightscount: *mut u32, abmac: *mut u8) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.GetRights)(::windows::core::Interface::as_raw(self), pprights, pnrightscount, abmac).ok()
    }
    #[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
    #[cfg(feature = "Win32_Media_Audio")]
    pub unsafe fn CreateStorage<P0>(&self, dwattributes: u32, pformat: ::core::option::Option<*const super::Audio::WAVEFORMATEX>, pwszname: P0) -> ::windows::core::Result<IMDSPStorage>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        let mut result__ = ::windows::core::zeroed::<IMDSPStorage>();
        (::windows::core::Interface::vtable(self).base__.CreateStorage)(::windows::core::Interface::as_raw(self), dwattributes, ::core::mem::transmute(pformat.unwrap_or(::std::ptr::null())), pwszname.into_param().abi(), &mut result__).from_abi(result__)
    }
    pub unsafe fn EnumStorage(&self) -> ::windows::core::Result<IMDSPEnumStorage> {
        let mut result__ = ::windows::core::zeroed::<IMDSPEnumStorage>();
        (::windows::core::Interface::vtable(self).base__.EnumStorage)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SendOpaqueCommand(&self, pcommand: *mut OPAQUECOMMAND) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.SendOpaqueCommand)(::windows::core::Interface::as_raw(self), pcommand).ok()
    }
    pub unsafe fn GetStorage<P0>(&self, pszstoragename: P0) -> ::windows::core::Result<IMDSPStorage>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        let mut result__ = ::windows::core::zeroed::<IMDSPStorage>();
        (::windows::core::Interface::vtable(self).GetStorage)(::windows::core::Interface::as_raw(self), pszstoragename.into_param().abi(), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`, `\"Win32_Media_Audio\"`, `\"Win32_Media_MediaFoundation\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_Media_Audio", feature = "Win32_Media_MediaFoundation"))]
    pub unsafe fn CreateStorage2<P0>(&self, dwattributes: u32, dwattributesex: u32, paudioformat: ::core::option::Option<*const super::Audio::WAVEFORMATEX>, pvideoformat: ::core::option::Option<*const super::MediaFoundation::VIDEOINFOHEADER>, pwszname: P0, qwfilesize: u64) -> ::windows::core::Result<IMDSPStorage>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        let mut result__ = ::windows::core::zeroed::<IMDSPStorage>();
        (::windows::core::Interface::vtable(self).CreateStorage2)(::windows::core::Interface::as_raw(self), dwattributes, dwattributesex, ::core::mem::transmute(paudioformat.unwrap_or(::std::ptr::null())), ::core::mem::transmute(pvideoformat.unwrap_or(::std::ptr::null())), pwszname.into_param().abi(), qwfilesize, &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`, `\"Win32_Media_Audio\"`, `\"Win32_Media_MediaFoundation\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_Media_Audio", feature = "Win32_Media_MediaFoundation"))]
    pub unsafe fn SetAttributes2(&self, dwattributes: u32, dwattributesex: u32, paudioformat: ::core::option::Option<*const super::Audio::WAVEFORMATEX>, pvideoformat: ::core::option::Option<*const super::MediaFoundation::VIDEOINFOHEADER>) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetAttributes2)(::windows::core::Interface::as_raw(self), dwattributes, dwattributesex, ::core::mem::transmute(paudioformat.unwrap_or(::std::ptr::null())), ::core::mem::transmute(pvideoformat.unwrap_or(::std::ptr::null()))).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`, `\"Win32_Media_Audio\"`, `\"Win32_Media_MediaFoundation\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_Media_Audio", feature = "Win32_Media_MediaFoundation"))]
    pub unsafe fn GetAttributes2(&self, pdwattributes: *mut u32, pdwattributesex: *mut u32, paudioformat: ::core::option::Option<*mut super::Audio::WAVEFORMATEX>, pvideoformat: ::core::option::Option<*mut super::MediaFoundation::VIDEOINFOHEADER>) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetAttributes2)(::windows::core::Interface::as_raw(self), pdwattributes, pdwattributesex, ::core::mem::transmute(paudioformat.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(pvideoformat.unwrap_or(::std::ptr::null_mut()))).ok()
    }
}
::windows::imp::interface_hierarchy!(IMDSPStorage2, ::windows::core::IUnknown, IMDSPStorage);
impl ::core::cmp::PartialEq for IMDSPStorage2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IMDSPStorage2 {}
impl ::core::fmt::Debug for IMDSPStorage2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMDSPStorage2").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IMDSPStorage2 {
    type Vtable = IMDSPStorage2_Vtbl;
}
impl ::core::clone::Clone for IMDSPStorage2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IMDSPStorage2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0a5e07a5_6454_4451_9c36_1c6ae7e2b1d6);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMDSPStorage2_Vtbl {
    pub base__: IMDSPStorage_Vtbl,
    pub GetStorage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszstoragename: ::windows::core::PCWSTR, ppstorage: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_Media_Audio", feature = "Win32_Media_MediaFoundation"))]
    pub CreateStorage2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwattributes: u32, dwattributesex: u32, paudioformat: *const super::Audio::WAVEFORMATEX, pvideoformat: *const super::MediaFoundation::VIDEOINFOHEADER, pwszname: ::windows::core::PCWSTR, qwfilesize: u64, ppnewstorage: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_Media_Audio", feature = "Win32_Media_MediaFoundation")))]
    CreateStorage2: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_Media_Audio", feature = "Win32_Media_MediaFoundation"))]
    pub SetAttributes2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwattributes: u32, dwattributesex: u32, paudioformat: *const super::Audio::WAVEFORMATEX, pvideoformat: *const super::MediaFoundation::VIDEOINFOHEADER) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_Media_Audio", feature = "Win32_Media_MediaFoundation")))]
    SetAttributes2: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_Media_Audio", feature = "Win32_Media_MediaFoundation"))]
    pub GetAttributes2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdwattributes: *mut u32, pdwattributesex: *mut u32, paudioformat: *mut super::Audio::WAVEFORMATEX, pvideoformat: *mut super::MediaFoundation::VIDEOINFOHEADER) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_Media_Audio", feature = "Win32_Media_MediaFoundation")))]
    GetAttributes2: usize,
}
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
#[repr(transparent)]
pub struct IMDSPStorage3(::windows::core::IUnknown);
impl IMDSPStorage3 {
    #[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
    #[cfg(feature = "Win32_Media_Audio")]
    pub unsafe fn SetAttributes(&self, dwattributes: u32, pformat: ::core::option::Option<*const super::Audio::WAVEFORMATEX>) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.SetAttributes)(::windows::core::Interface::as_raw(self), dwattributes, ::core::mem::transmute(pformat.unwrap_or(::std::ptr::null()))).ok()
    }
    pub unsafe fn GetStorageGlobals(&self) -> ::windows::core::Result<IMDSPStorageGlobals> {
        let mut result__ = ::windows::core::zeroed::<IMDSPStorageGlobals>();
        (::windows::core::Interface::vtable(self).base__.base__.GetStorageGlobals)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
    #[cfg(feature = "Win32_Media_Audio")]
    pub unsafe fn GetAttributes(&self, pdwattributes: *mut u32, pformat: ::core::option::Option<*mut super::Audio::WAVEFORMATEX>) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.GetAttributes)(::windows::core::Interface::as_raw(self), pdwattributes, ::core::mem::transmute(pformat.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    pub unsafe fn GetName(&self, pwszname: &mut [u16]) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.GetName)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pwszname.as_ptr()), pwszname.len() as _).ok()
    }
    pub unsafe fn GetDate(&self) -> ::windows::core::Result<WMDMDATETIME> {
        let mut result__ = ::windows::core::zeroed::<WMDMDATETIME>();
        (::windows::core::Interface::vtable(self).base__.base__.GetDate)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetSize(&self, pdwsizelow: *mut u32, pdwsizehigh: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.GetSize)(::windows::core::Interface::as_raw(self), pdwsizelow, pdwsizehigh).ok()
    }
    pub unsafe fn GetRights(&self, pprights: *mut *mut WMDMRIGHTS, pnrightscount: *mut u32, abmac: *mut u8) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.GetRights)(::windows::core::Interface::as_raw(self), pprights, pnrightscount, abmac).ok()
    }
    #[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
    #[cfg(feature = "Win32_Media_Audio")]
    pub unsafe fn CreateStorage<P0>(&self, dwattributes: u32, pformat: ::core::option::Option<*const super::Audio::WAVEFORMATEX>, pwszname: P0) -> ::windows::core::Result<IMDSPStorage>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        let mut result__ = ::windows::core::zeroed::<IMDSPStorage>();
        (::windows::core::Interface::vtable(self).base__.base__.CreateStorage)(::windows::core::Interface::as_raw(self), dwattributes, ::core::mem::transmute(pformat.unwrap_or(::std::ptr::null())), pwszname.into_param().abi(), &mut result__).from_abi(result__)
    }
    pub unsafe fn EnumStorage(&self) -> ::windows::core::Result<IMDSPEnumStorage> {
        let mut result__ = ::windows::core::zeroed::<IMDSPEnumStorage>();
        (::windows::core::Interface::vtable(self).base__.base__.EnumStorage)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SendOpaqueCommand(&self, pcommand: *mut OPAQUECOMMAND) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.SendOpaqueCommand)(::windows::core::Interface::as_raw(self), pcommand).ok()
    }
    pub unsafe fn GetStorage<P0>(&self, pszstoragename: P0) -> ::windows::core::Result<IMDSPStorage>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        let mut result__ = ::windows::core::zeroed::<IMDSPStorage>();
        (::windows::core::Interface::vtable(self).base__.GetStorage)(::windows::core::Interface::as_raw(self), pszstoragename.into_param().abi(), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`, `\"Win32_Media_Audio\"`, `\"Win32_Media_MediaFoundation\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_Media_Audio", feature = "Win32_Media_MediaFoundation"))]
    pub unsafe fn CreateStorage2<P0>(&self, dwattributes: u32, dwattributesex: u32, paudioformat: ::core::option::Option<*const super::Audio::WAVEFORMATEX>, pvideoformat: ::core::option::Option<*const super::MediaFoundation::VIDEOINFOHEADER>, pwszname: P0, qwfilesize: u64) -> ::windows::core::Result<IMDSPStorage>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        let mut result__ = ::windows::core::zeroed::<IMDSPStorage>();
        (::windows::core::Interface::vtable(self).base__.CreateStorage2)(::windows::core::Interface::as_raw(self), dwattributes, dwattributesex, ::core::mem::transmute(paudioformat.unwrap_or(::std::ptr::null())), ::core::mem::transmute(pvideoformat.unwrap_or(::std::ptr::null())), pwszname.into_param().abi(), qwfilesize, &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`, `\"Win32_Media_Audio\"`, `\"Win32_Media_MediaFoundation\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_Media_Audio", feature = "Win32_Media_MediaFoundation"))]
    pub unsafe fn SetAttributes2(&self, dwattributes: u32, dwattributesex: u32, paudioformat: ::core::option::Option<*const super::Audio::WAVEFORMATEX>, pvideoformat: ::core::option::Option<*const super::MediaFoundation::VIDEOINFOHEADER>) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.SetAttributes2)(::windows::core::Interface::as_raw(self), dwattributes, dwattributesex, ::core::mem::transmute(paudioformat.unwrap_or(::std::ptr::null())), ::core::mem::transmute(pvideoformat.unwrap_or(::std::ptr::null()))).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`, `\"Win32_Media_Audio\"`, `\"Win32_Media_MediaFoundation\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_Media_Audio", feature = "Win32_Media_MediaFoundation"))]
    pub unsafe fn GetAttributes2(&self, pdwattributes: *mut u32, pdwattributesex: *mut u32, paudioformat: ::core::option::Option<*mut super::Audio::WAVEFORMATEX>, pvideoformat: ::core::option::Option<*mut super::MediaFoundation::VIDEOINFOHEADER>) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.GetAttributes2)(::windows::core::Interface::as_raw(self), pdwattributes, pdwattributesex, ::core::mem::transmute(paudioformat.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(pvideoformat.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    pub unsafe fn GetMetadata<P0>(&self, pmetadata: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<IWMDMMetaData>,
    {
        (::windows::core::Interface::vtable(self).GetMetadata)(::windows::core::Interface::as_raw(self), pmetadata.into_param().abi()).ok()
    }
    pub unsafe fn SetMetadata<P0>(&self, pmetadata: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<IWMDMMetaData>,
    {
        (::windows::core::Interface::vtable(self).SetMetadata)(::windows::core::Interface::as_raw(self), pmetadata.into_param().abi()).ok()
    }
}
::windows::imp::interface_hierarchy!(IMDSPStorage3, ::windows::core::IUnknown, IMDSPStorage, IMDSPStorage2);
impl ::core::cmp::PartialEq for IMDSPStorage3 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IMDSPStorage3 {}
impl ::core::fmt::Debug for IMDSPStorage3 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMDSPStorage3").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IMDSPStorage3 {
    type Vtable = IMDSPStorage3_Vtbl;
}
impl ::core::clone::Clone for IMDSPStorage3 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IMDSPStorage3 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6c669867_97ed_4a67_9706_1c5529d2a414);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMDSPStorage3_Vtbl {
    pub base__: IMDSPStorage2_Vtbl,
    pub GetMetadata: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pmetadata: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetMetadata: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pmetadata: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
#[repr(transparent)]
pub struct IMDSPStorage4(::windows::core::IUnknown);
impl IMDSPStorage4 {
    #[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
    #[cfg(feature = "Win32_Media_Audio")]
    pub unsafe fn SetAttributes(&self, dwattributes: u32, pformat: ::core::option::Option<*const super::Audio::WAVEFORMATEX>) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.SetAttributes)(::windows::core::Interface::as_raw(self), dwattributes, ::core::mem::transmute(pformat.unwrap_or(::std::ptr::null()))).ok()
    }
    pub unsafe fn GetStorageGlobals(&self) -> ::windows::core::Result<IMDSPStorageGlobals> {
        let mut result__ = ::windows::core::zeroed::<IMDSPStorageGlobals>();
        (::windows::core::Interface::vtable(self).base__.base__.base__.GetStorageGlobals)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
    #[cfg(feature = "Win32_Media_Audio")]
    pub unsafe fn GetAttributes(&self, pdwattributes: *mut u32, pformat: ::core::option::Option<*mut super::Audio::WAVEFORMATEX>) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.GetAttributes)(::windows::core::Interface::as_raw(self), pdwattributes, ::core::mem::transmute(pformat.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    pub unsafe fn GetName(&self, pwszname: &mut [u16]) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.GetName)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pwszname.as_ptr()), pwszname.len() as _).ok()
    }
    pub unsafe fn GetDate(&self) -> ::windows::core::Result<WMDMDATETIME> {
        let mut result__ = ::windows::core::zeroed::<WMDMDATETIME>();
        (::windows::core::Interface::vtable(self).base__.base__.base__.GetDate)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetSize(&self, pdwsizelow: *mut u32, pdwsizehigh: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.GetSize)(::windows::core::Interface::as_raw(self), pdwsizelow, pdwsizehigh).ok()
    }
    pub unsafe fn GetRights(&self, pprights: *mut *mut WMDMRIGHTS, pnrightscount: *mut u32, abmac: *mut u8) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.GetRights)(::windows::core::Interface::as_raw(self), pprights, pnrightscount, abmac).ok()
    }
    #[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
    #[cfg(feature = "Win32_Media_Audio")]
    pub unsafe fn CreateStorage<P0>(&self, dwattributes: u32, pformat: ::core::option::Option<*const super::Audio::WAVEFORMATEX>, pwszname: P0) -> ::windows::core::Result<IMDSPStorage>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        let mut result__ = ::windows::core::zeroed::<IMDSPStorage>();
        (::windows::core::Interface::vtable(self).base__.base__.base__.CreateStorage)(::windows::core::Interface::as_raw(self), dwattributes, ::core::mem::transmute(pformat.unwrap_or(::std::ptr::null())), pwszname.into_param().abi(), &mut result__).from_abi(result__)
    }
    pub unsafe fn EnumStorage(&self) -> ::windows::core::Result<IMDSPEnumStorage> {
        let mut result__ = ::windows::core::zeroed::<IMDSPEnumStorage>();
        (::windows::core::Interface::vtable(self).base__.base__.base__.EnumStorage)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SendOpaqueCommand(&self, pcommand: *mut OPAQUECOMMAND) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.SendOpaqueCommand)(::windows::core::Interface::as_raw(self), pcommand).ok()
    }
    pub unsafe fn GetStorage<P0>(&self, pszstoragename: P0) -> ::windows::core::Result<IMDSPStorage>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        let mut result__ = ::windows::core::zeroed::<IMDSPStorage>();
        (::windows::core::Interface::vtable(self).base__.base__.GetStorage)(::windows::core::Interface::as_raw(self), pszstoragename.into_param().abi(), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`, `\"Win32_Media_Audio\"`, `\"Win32_Media_MediaFoundation\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_Media_Audio", feature = "Win32_Media_MediaFoundation"))]
    pub unsafe fn CreateStorage2<P0>(&self, dwattributes: u32, dwattributesex: u32, paudioformat: ::core::option::Option<*const super::Audio::WAVEFORMATEX>, pvideoformat: ::core::option::Option<*const super::MediaFoundation::VIDEOINFOHEADER>, pwszname: P0, qwfilesize: u64) -> ::windows::core::Result<IMDSPStorage>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        let mut result__ = ::windows::core::zeroed::<IMDSPStorage>();
        (::windows::core::Interface::vtable(self).base__.base__.CreateStorage2)(::windows::core::Interface::as_raw(self), dwattributes, dwattributesex, ::core::mem::transmute(paudioformat.unwrap_or(::std::ptr::null())), ::core::mem::transmute(pvideoformat.unwrap_or(::std::ptr::null())), pwszname.into_param().abi(), qwfilesize, &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`, `\"Win32_Media_Audio\"`, `\"Win32_Media_MediaFoundation\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_Media_Audio", feature = "Win32_Media_MediaFoundation"))]
    pub unsafe fn SetAttributes2(&self, dwattributes: u32, dwattributesex: u32, paudioformat: ::core::option::Option<*const super::Audio::WAVEFORMATEX>, pvideoformat: ::core::option::Option<*const super::MediaFoundation::VIDEOINFOHEADER>) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.SetAttributes2)(::windows::core::Interface::as_raw(self), dwattributes, dwattributesex, ::core::mem::transmute(paudioformat.unwrap_or(::std::ptr::null())), ::core::mem::transmute(pvideoformat.unwrap_or(::std::ptr::null()))).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`, `\"Win32_Media_Audio\"`, `\"Win32_Media_MediaFoundation\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_Media_Audio", feature = "Win32_Media_MediaFoundation"))]
    pub unsafe fn GetAttributes2(&self, pdwattributes: *mut u32, pdwattributesex: *mut u32, paudioformat: ::core::option::Option<*mut super::Audio::WAVEFORMATEX>, pvideoformat: ::core::option::Option<*mut super::MediaFoundation::VIDEOINFOHEADER>) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.GetAttributes2)(::windows::core::Interface::as_raw(self), pdwattributes, pdwattributesex, ::core::mem::transmute(paudioformat.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(pvideoformat.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    pub unsafe fn GetMetadata<P0>(&self, pmetadata: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<IWMDMMetaData>,
    {
        (::windows::core::Interface::vtable(self).base__.GetMetadata)(::windows::core::Interface::as_raw(self), pmetadata.into_param().abi()).ok()
    }
    pub unsafe fn SetMetadata<P0>(&self, pmetadata: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<IWMDMMetaData>,
    {
        (::windows::core::Interface::vtable(self).base__.SetMetadata)(::windows::core::Interface::as_raw(self), pmetadata.into_param().abi()).ok()
    }
    pub unsafe fn SetReferences(&self, ppispstorage: ::core::option::Option<&[::core::option::Option<IMDSPStorage>]>) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetReferences)(::windows::core::Interface::as_raw(self), ppispstorage.as_deref().map_or(0, |slice| slice.len() as _), ::core::mem::transmute(ppispstorage.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr()))).ok()
    }
    pub unsafe fn GetReferences(&self, pdwrefs: *mut u32, pppispstorage: *mut *mut ::core::option::Option<IMDSPStorage>) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetReferences)(::windows::core::Interface::as_raw(self), pdwrefs, pppispstorage).ok()
    }
    pub unsafe fn CreateStorageWithMetadata<P0, P1>(&self, dwattributes: u32, pwszname: P0, pmetadata: P1, qwfilesize: u64) -> ::windows::core::Result<IMDSPStorage>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
        P1: ::windows::core::IntoParam<IWMDMMetaData>,
    {
        let mut result__ = ::windows::core::zeroed::<IMDSPStorage>();
        (::windows::core::Interface::vtable(self).CreateStorageWithMetadata)(::windows::core::Interface::as_raw(self), dwattributes, pwszname.into_param().abi(), pmetadata.into_param().abi(), qwfilesize, &mut result__).from_abi(result__)
    }
    pub unsafe fn GetSpecifiedMetadata<P0>(&self, ppwszpropnames: &[::windows::core::PCWSTR], pmetadata: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<IWMDMMetaData>,
    {
        (::windows::core::Interface::vtable(self).GetSpecifiedMetadata)(::windows::core::Interface::as_raw(self), ppwszpropnames.len() as _, ::core::mem::transmute(ppwszpropnames.as_ptr()), pmetadata.into_param().abi()).ok()
    }
    pub unsafe fn FindStorage<P0>(&self, findscope: WMDM_FIND_SCOPE, pwszuniqueid: P0) -> ::windows::core::Result<IMDSPStorage>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        let mut result__ = ::windows::core::zeroed::<IMDSPStorage>();
        (::windows::core::Interface::vtable(self).FindStorage)(::windows::core::Interface::as_raw(self), findscope, pwszuniqueid.into_param().abi(), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetParent(&self) -> ::windows::core::Result<IMDSPStorage> {
        let mut result__ = ::windows::core::zeroed::<IMDSPStorage>();
        (::windows::core::Interface::vtable(self).GetParent)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
::windows::imp::interface_hierarchy!(IMDSPStorage4, ::windows::core::IUnknown, IMDSPStorage, IMDSPStorage2, IMDSPStorage3);
impl ::core::cmp::PartialEq for IMDSPStorage4 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IMDSPStorage4 {}
impl ::core::fmt::Debug for IMDSPStorage4 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMDSPStorage4").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IMDSPStorage4 {
    type Vtable = IMDSPStorage4_Vtbl;
}
impl ::core::clone::Clone for IMDSPStorage4 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IMDSPStorage4 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3133b2c4_515c_481b_b1ce_39327ecb4f74);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMDSPStorage4_Vtbl {
    pub base__: IMDSPStorage3_Vtbl,
    pub SetReferences: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwrefs: u32, ppispstorage: *const *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetReferences: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdwrefs: *mut u32, pppispstorage: *mut *mut ::core::option::Option<IMDSPStorage>) -> ::windows::core::HRESULT,
    pub CreateStorageWithMetadata: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwattributes: u32, pwszname: ::windows::core::PCWSTR, pmetadata: *mut ::core::ffi::c_void, qwfilesize: u64, ppnewstorage: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetSpecifiedMetadata: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cproperties: u32, ppwszpropnames: *const ::windows::core::PCWSTR, pmetadata: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub FindStorage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, findscope: WMDM_FIND_SCOPE, pwszuniqueid: ::windows::core::PCWSTR, ppstorage: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetParent: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppstorage: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
#[repr(transparent)]
pub struct IMDSPStorageGlobals(::windows::core::IUnknown);
impl IMDSPStorageGlobals {
    pub unsafe fn GetCapabilities(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::windows::core::zeroed::<u32>();
        (::windows::core::Interface::vtable(self).GetCapabilities)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetSerialNumber(&self, pserialnum: *mut WMDMID, abmac: *mut u8) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetSerialNumber)(::windows::core::Interface::as_raw(self), pserialnum, abmac).ok()
    }
    pub unsafe fn GetTotalSize(&self, pdwtotalsizelow: *mut u32, pdwtotalsizehigh: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetTotalSize)(::windows::core::Interface::as_raw(self), pdwtotalsizelow, pdwtotalsizehigh).ok()
    }
    pub unsafe fn GetTotalFree(&self, pdwfreelow: *mut u32, pdwfreehigh: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetTotalFree)(::windows::core::Interface::as_raw(self), pdwfreelow, pdwfreehigh).ok()
    }
    pub unsafe fn GetTotalBad(&self, pdwbadlow: *mut u32, pdwbadhigh: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetTotalBad)(::windows::core::Interface::as_raw(self), pdwbadlow, pdwbadhigh).ok()
    }
    pub unsafe fn GetStatus(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::windows::core::zeroed::<u32>();
        (::windows::core::Interface::vtable(self).GetStatus)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn Initialize<P0>(&self, fumode: u32, pprogress: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<IWMDMProgress>,
    {
        (::windows::core::Interface::vtable(self).Initialize)(::windows::core::Interface::as_raw(self), fumode, pprogress.into_param().abi()).ok()
    }
    pub unsafe fn GetDevice(&self) -> ::windows::core::Result<IMDSPDevice> {
        let mut result__ = ::windows::core::zeroed::<IMDSPDevice>();
        (::windows::core::Interface::vtable(self).GetDevice)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetRootStorage(&self) -> ::windows::core::Result<IMDSPStorage> {
        let mut result__ = ::windows::core::zeroed::<IMDSPStorage>();
        (::windows::core::Interface::vtable(self).GetRootStorage)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
::windows::imp::interface_hierarchy!(IMDSPStorageGlobals, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IMDSPStorageGlobals {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IMDSPStorageGlobals {}
impl ::core::fmt::Debug for IMDSPStorageGlobals {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMDSPStorageGlobals").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IMDSPStorageGlobals {
    type Vtable = IMDSPStorageGlobals_Vtbl;
}
impl ::core::clone::Clone for IMDSPStorageGlobals {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IMDSPStorageGlobals {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1dcb3a17_33ed_11d3_8470_00c04f79dbc0);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMDSPStorageGlobals_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub GetCapabilities: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdwcapabilities: *mut u32) -> ::windows::core::HRESULT,
    pub GetSerialNumber: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pserialnum: *mut WMDMID, abmac: *mut u8) -> ::windows::core::HRESULT,
    pub GetTotalSize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdwtotalsizelow: *mut u32, pdwtotalsizehigh: *mut u32) -> ::windows::core::HRESULT,
    pub GetTotalFree: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdwfreelow: *mut u32, pdwfreehigh: *mut u32) -> ::windows::core::HRESULT,
    pub GetTotalBad: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdwbadlow: *mut u32, pdwbadhigh: *mut u32) -> ::windows::core::HRESULT,
    pub GetStatus: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdwstatus: *mut u32) -> ::windows::core::HRESULT,
    pub Initialize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, fumode: u32, pprogress: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetDevice: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppdevice: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetRootStorage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pproot: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
#[repr(transparent)]
pub struct IMDServiceProvider(::windows::core::IUnknown);
impl IMDServiceProvider {
    pub unsafe fn GetDeviceCount(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::windows::core::zeroed::<u32>();
        (::windows::core::Interface::vtable(self).GetDeviceCount)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn EnumDevices(&self) -> ::windows::core::Result<IMDSPEnumDevice> {
        let mut result__ = ::windows::core::zeroed::<IMDSPEnumDevice>();
        (::windows::core::Interface::vtable(self).EnumDevices)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
::windows::imp::interface_hierarchy!(IMDServiceProvider, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IMDServiceProvider {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IMDServiceProvider {}
impl ::core::fmt::Debug for IMDServiceProvider {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMDServiceProvider").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IMDServiceProvider {
    type Vtable = IMDServiceProvider_Vtbl;
}
impl ::core::clone::Clone for IMDServiceProvider {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IMDServiceProvider {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1dcb3a10_33ed_11d3_8470_00c04f79dbc0);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMDServiceProvider_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub GetDeviceCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdwcount: *mut u32) -> ::windows::core::HRESULT,
    pub EnumDevices: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppenumdevice: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
#[repr(transparent)]
pub struct IMDServiceProvider2(::windows::core::IUnknown);
impl IMDServiceProvider2 {
    pub unsafe fn GetDeviceCount(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::windows::core::zeroed::<u32>();
        (::windows::core::Interface::vtable(self).base__.GetDeviceCount)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn EnumDevices(&self) -> ::windows::core::Result<IMDSPEnumDevice> {
        let mut result__ = ::windows::core::zeroed::<IMDSPEnumDevice>();
        (::windows::core::Interface::vtable(self).base__.EnumDevices)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn CreateDevice<P0>(&self, pwszdevicepath: P0, pdwcount: *mut u32, pppdevicearray: *mut *mut ::core::option::Option<IMDSPDevice>) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).CreateDevice)(::windows::core::Interface::as_raw(self), pwszdevicepath.into_param().abi(), pdwcount, pppdevicearray).ok()
    }
}
::windows::imp::interface_hierarchy!(IMDServiceProvider2, ::windows::core::IUnknown, IMDServiceProvider);
impl ::core::cmp::PartialEq for IMDServiceProvider2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IMDServiceProvider2 {}
impl ::core::fmt::Debug for IMDServiceProvider2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMDServiceProvider2").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IMDServiceProvider2 {
    type Vtable = IMDServiceProvider2_Vtbl;
}
impl ::core::clone::Clone for IMDServiceProvider2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IMDServiceProvider2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb2fa24b7_cda3_4694_9862_413ae1a34819);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMDServiceProvider2_Vtbl {
    pub base__: IMDServiceProvider_Vtbl,
    pub CreateDevice: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwszdevicepath: ::windows::core::PCWSTR, pdwcount: *mut u32, pppdevicearray: *mut *mut ::core::option::Option<IMDSPDevice>) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
#[repr(transparent)]
pub struct IMDServiceProvider3(::windows::core::IUnknown);
impl IMDServiceProvider3 {
    pub unsafe fn GetDeviceCount(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::windows::core::zeroed::<u32>();
        (::windows::core::Interface::vtable(self).base__.base__.GetDeviceCount)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn EnumDevices(&self) -> ::windows::core::Result<IMDSPEnumDevice> {
        let mut result__ = ::windows::core::zeroed::<IMDSPEnumDevice>();
        (::windows::core::Interface::vtable(self).base__.base__.EnumDevices)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn CreateDevice<P0>(&self, pwszdevicepath: P0, pdwcount: *mut u32, pppdevicearray: *mut *mut ::core::option::Option<IMDSPDevice>) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).base__.CreateDevice)(::windows::core::Interface::as_raw(self), pwszdevicepath.into_param().abi(), pdwcount, pppdevicearray).ok()
    }
    pub unsafe fn SetDeviceEnumPreference(&self, dwenumpref: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetDeviceEnumPreference)(::windows::core::Interface::as_raw(self), dwenumpref).ok()
    }
}
::windows::imp::interface_hierarchy!(IMDServiceProvider3, ::windows::core::IUnknown, IMDServiceProvider, IMDServiceProvider2);
impl ::core::cmp::PartialEq for IMDServiceProvider3 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IMDServiceProvider3 {}
impl ::core::fmt::Debug for IMDServiceProvider3 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMDServiceProvider3").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IMDServiceProvider3 {
    type Vtable = IMDServiceProvider3_Vtbl;
}
impl ::core::clone::Clone for IMDServiceProvider3 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IMDServiceProvider3 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4ed13ef3_a971_4d19_9f51_0e1826b2da57);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMDServiceProvider3_Vtbl {
    pub base__: IMDServiceProvider2_Vtbl,
    pub SetDeviceEnumPreference: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwenumpref: u32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
#[repr(transparent)]
pub struct ISCPSecureAuthenticate(::windows::core::IUnknown);
impl ISCPSecureAuthenticate {
    pub unsafe fn GetSecureQuery(&self) -> ::windows::core::Result<ISCPSecureQuery> {
        let mut result__ = ::windows::core::zeroed::<ISCPSecureQuery>();
        (::windows::core::Interface::vtable(self).GetSecureQuery)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
::windows::imp::interface_hierarchy!(ISCPSecureAuthenticate, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for ISCPSecureAuthenticate {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ISCPSecureAuthenticate {}
impl ::core::fmt::Debug for ISCPSecureAuthenticate {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISCPSecureAuthenticate").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for ISCPSecureAuthenticate {
    type Vtable = ISCPSecureAuthenticate_Vtbl;
}
impl ::core::clone::Clone for ISCPSecureAuthenticate {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for ISCPSecureAuthenticate {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1dcb3a0f_33ed_11d3_8470_00c04f79dbc0);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISCPSecureAuthenticate_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub GetSecureQuery: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppsecurequery: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
#[repr(transparent)]
pub struct ISCPSecureAuthenticate2(::windows::core::IUnknown);
impl ISCPSecureAuthenticate2 {
    pub unsafe fn GetSecureQuery(&self) -> ::windows::core::Result<ISCPSecureQuery> {
        let mut result__ = ::windows::core::zeroed::<ISCPSecureQuery>();
        (::windows::core::Interface::vtable(self).base__.GetSecureQuery)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetSCPSession(&self) -> ::windows::core::Result<ISCPSession> {
        let mut result__ = ::windows::core::zeroed::<ISCPSession>();
        (::windows::core::Interface::vtable(self).GetSCPSession)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
::windows::imp::interface_hierarchy!(ISCPSecureAuthenticate2, ::windows::core::IUnknown, ISCPSecureAuthenticate);
impl ::core::cmp::PartialEq for ISCPSecureAuthenticate2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ISCPSecureAuthenticate2 {}
impl ::core::fmt::Debug for ISCPSecureAuthenticate2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISCPSecureAuthenticate2").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for ISCPSecureAuthenticate2 {
    type Vtable = ISCPSecureAuthenticate2_Vtbl;
}
impl ::core::clone::Clone for ISCPSecureAuthenticate2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for ISCPSecureAuthenticate2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb580cfae_1672_47e2_acaa_44bbecbcae5b);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISCPSecureAuthenticate2_Vtbl {
    pub base__: ISCPSecureAuthenticate_Vtbl,
    pub GetSCPSession: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppscpsession: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
#[repr(transparent)]
pub struct ISCPSecureExchange(::windows::core::IUnknown);
impl ISCPSecureExchange {
    pub unsafe fn TransferContainerData(&self, pdata: &[u8], pfureadyflags: *mut u32, abmac: *mut u8) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).TransferContainerData)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pdata.as_ptr()), pdata.len() as _, pfureadyflags, abmac).ok()
    }
    pub unsafe fn ObjectData(&self, pdata: *mut u8, pdwsize: *mut u32, abmac: *mut u8) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).ObjectData)(::windows::core::Interface::as_raw(self), pdata, pdwsize, abmac).ok()
    }
    pub unsafe fn TransferComplete(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).TransferComplete)(::windows::core::Interface::as_raw(self)).ok()
    }
}
::windows::imp::interface_hierarchy!(ISCPSecureExchange, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for ISCPSecureExchange {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ISCPSecureExchange {}
impl ::core::fmt::Debug for ISCPSecureExchange {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISCPSecureExchange").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for ISCPSecureExchange {
    type Vtable = ISCPSecureExchange_Vtbl;
}
impl ::core::clone::Clone for ISCPSecureExchange {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for ISCPSecureExchange {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1dcb3a0e_33ed_11d3_8470_00c04f79dbc0);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISCPSecureExchange_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub TransferContainerData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdata: *const u8, dwsize: u32, pfureadyflags: *mut u32, abmac: *mut u8) -> ::windows::core::HRESULT,
    pub ObjectData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdata: *mut u8, pdwsize: *mut u32, abmac: *mut u8) -> ::windows::core::HRESULT,
    pub TransferComplete: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
#[repr(transparent)]
pub struct ISCPSecureExchange2(::windows::core::IUnknown);
impl ISCPSecureExchange2 {
    pub unsafe fn TransferContainerData(&self, pdata: &[u8], pfureadyflags: *mut u32, abmac: *mut u8) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.TransferContainerData)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pdata.as_ptr()), pdata.len() as _, pfureadyflags, abmac).ok()
    }
    pub unsafe fn ObjectData(&self, pdata: *mut u8, pdwsize: *mut u32, abmac: *mut u8) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.ObjectData)(::windows::core::Interface::as_raw(self), pdata, pdwsize, abmac).ok()
    }
    pub unsafe fn TransferComplete(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.TransferComplete)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn TransferContainerData2<P0>(&self, pdata: &[u8], pprogresscallback: P0, pfureadyflags: *mut u32, abmac: *mut u8) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<IWMDMProgress3>,
    {
        (::windows::core::Interface::vtable(self).TransferContainerData2)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pdata.as_ptr()), pdata.len() as _, pprogresscallback.into_param().abi(), pfureadyflags, abmac).ok()
    }
}
::windows::imp::interface_hierarchy!(ISCPSecureExchange2, ::windows::core::IUnknown, ISCPSecureExchange);
impl ::core::cmp::PartialEq for ISCPSecureExchange2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ISCPSecureExchange2 {}
impl ::core::fmt::Debug for ISCPSecureExchange2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISCPSecureExchange2").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for ISCPSecureExchange2 {
    type Vtable = ISCPSecureExchange2_Vtbl;
}
impl ::core::clone::Clone for ISCPSecureExchange2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for ISCPSecureExchange2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6c62fc7b_2690_483f_9d44_0a20cb35577c);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISCPSecureExchange2_Vtbl {
    pub base__: ISCPSecureExchange_Vtbl,
    pub TransferContainerData2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdata: *const u8, dwsize: u32, pprogresscallback: *mut ::core::ffi::c_void, pfureadyflags: *mut u32, abmac: *mut u8) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
#[repr(transparent)]
pub struct ISCPSecureExchange3(::windows::core::IUnknown);
impl ISCPSecureExchange3 {
    pub unsafe fn TransferContainerData(&self, pdata: &[u8], pfureadyflags: *mut u32, abmac: *mut u8) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.TransferContainerData)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pdata.as_ptr()), pdata.len() as _, pfureadyflags, abmac).ok()
    }
    pub unsafe fn ObjectData(&self, pdata: *mut u8, pdwsize: *mut u32, abmac: *mut u8) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.ObjectData)(::windows::core::Interface::as_raw(self), pdata, pdwsize, abmac).ok()
    }
    pub unsafe fn TransferComplete(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.TransferComplete)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn TransferContainerData2<P0>(&self, pdata: &[u8], pprogresscallback: P0, pfureadyflags: *mut u32, abmac: *mut u8) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<IWMDMProgress3>,
    {
        (::windows::core::Interface::vtable(self).base__.TransferContainerData2)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pdata.as_ptr()), pdata.len() as _, pprogresscallback.into_param().abi(), pfureadyflags, abmac).ok()
    }
    pub unsafe fn TransferContainerDataOnClearChannel<P0, P1>(&self, pdevice: P0, pdata: &[u8], pprogresscallback: P1) -> ::windows::core::Result<u32>
    where
        P0: ::windows::core::IntoParam<IMDSPDevice>,
        P1: ::windows::core::IntoParam<IWMDMProgress3>,
    {
        let mut result__ = ::windows::core::zeroed::<u32>();
        (::windows::core::Interface::vtable(self).TransferContainerDataOnClearChannel)(::windows::core::Interface::as_raw(self), pdevice.into_param().abi(), ::core::mem::transmute(pdata.as_ptr()), pdata.len() as _, pprogresscallback.into_param().abi(), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetObjectDataOnClearChannel<P0>(&self, pdevice: P0, pdata: *mut u8, pdwsize: *mut u32) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<IMDSPDevice>,
    {
        (::windows::core::Interface::vtable(self).GetObjectDataOnClearChannel)(::windows::core::Interface::as_raw(self), pdevice.into_param().abi(), pdata, pdwsize).ok()
    }
    pub unsafe fn TransferCompleteForDevice<P0>(&self, pdevice: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<IMDSPDevice>,
    {
        (::windows::core::Interface::vtable(self).TransferCompleteForDevice)(::windows::core::Interface::as_raw(self), pdevice.into_param().abi()).ok()
    }
}
::windows::imp::interface_hierarchy!(ISCPSecureExchange3, ::windows::core::IUnknown, ISCPSecureExchange, ISCPSecureExchange2);
impl ::core::cmp::PartialEq for ISCPSecureExchange3 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ISCPSecureExchange3 {}
impl ::core::fmt::Debug for ISCPSecureExchange3 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISCPSecureExchange3").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for ISCPSecureExchange3 {
    type Vtable = ISCPSecureExchange3_Vtbl;
}
impl ::core::clone::Clone for ISCPSecureExchange3 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for ISCPSecureExchange3 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xab4e77e4_8908_4b17_bd2a_b1dbe6dd69e1);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISCPSecureExchange3_Vtbl {
    pub base__: ISCPSecureExchange2_Vtbl,
    pub TransferContainerDataOnClearChannel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdevice: *mut ::core::ffi::c_void, pdata: *const u8, dwsize: u32, pprogresscallback: *mut ::core::ffi::c_void, pfureadyflags: *mut u32) -> ::windows::core::HRESULT,
    pub GetObjectDataOnClearChannel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdevice: *mut ::core::ffi::c_void, pdata: *mut u8, pdwsize: *mut u32) -> ::windows::core::HRESULT,
    pub TransferCompleteForDevice: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdevice: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
#[repr(transparent)]
pub struct ISCPSecureQuery(::windows::core::IUnknown);
impl ISCPSecureQuery {
    pub unsafe fn GetDataDemands(&self, pfuflags: *mut u32, pdwminrightsdata: *mut u32, pdwminexaminedata: *mut u32, pdwmindecidedata: *mut u32, abmac: *mut u8) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetDataDemands)(::windows::core::Interface::as_raw(self), pfuflags, pdwminrightsdata, pdwminexaminedata, pdwmindecidedata, abmac).ok()
    }
    pub unsafe fn ExamineData<P0>(&self, fuflags: u32, pwszextension: P0, pdata: &[u8], abmac: *mut u8) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).ExamineData)(::windows::core::Interface::as_raw(self), fuflags, pwszextension.into_param().abi(), ::core::mem::transmute(pdata.as_ptr()), pdata.len() as _, abmac).ok()
    }
    pub unsafe fn MakeDecision<P0>(&self, fuflags: u32, pdata: &[u8], dwappsec: u32, pbspsessionkey: &[u8], pstorageglobals: P0, ppexchange: *mut ::core::option::Option<ISCPSecureExchange>, abmac: *mut u8) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<IMDSPStorageGlobals>,
    {
        (::windows::core::Interface::vtable(self).MakeDecision)(::windows::core::Interface::as_raw(self), fuflags, ::core::mem::transmute(pdata.as_ptr()), pdata.len() as _, dwappsec, ::core::mem::transmute(pbspsessionkey.as_ptr()), pbspsessionkey.len() as _, pstorageglobals.into_param().abi(), ::core::mem::transmute(ppexchange), abmac).ok()
    }
    pub unsafe fn GetRights<P0>(&self, pdata: &[u8], pbspsessionkey: &[u8], pstgglobals: P0, pprights: *mut *mut WMDMRIGHTS, pnrightscount: *mut u32, abmac: *mut u8) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<IMDSPStorageGlobals>,
    {
        (::windows::core::Interface::vtable(self).GetRights)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pdata.as_ptr()), pdata.len() as _, ::core::mem::transmute(pbspsessionkey.as_ptr()), pbspsessionkey.len() as _, pstgglobals.into_param().abi(), pprights, pnrightscount, abmac).ok()
    }
}
::windows::imp::interface_hierarchy!(ISCPSecureQuery, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for ISCPSecureQuery {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ISCPSecureQuery {}
impl ::core::fmt::Debug for ISCPSecureQuery {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISCPSecureQuery").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for ISCPSecureQuery {
    type Vtable = ISCPSecureQuery_Vtbl;
}
impl ::core::clone::Clone for ISCPSecureQuery {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for ISCPSecureQuery {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1dcb3a0d_33ed_11d3_8470_00c04f79dbc0);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISCPSecureQuery_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub GetDataDemands: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pfuflags: *mut u32, pdwminrightsdata: *mut u32, pdwminexaminedata: *mut u32, pdwmindecidedata: *mut u32, abmac: *mut u8) -> ::windows::core::HRESULT,
    pub ExamineData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, fuflags: u32, pwszextension: ::windows::core::PCWSTR, pdata: *const u8, dwsize: u32, abmac: *mut u8) -> ::windows::core::HRESULT,
    pub MakeDecision: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, fuflags: u32, pdata: *const u8, dwsize: u32, dwappsec: u32, pbspsessionkey: *const u8, dwsessionkeylen: u32, pstorageglobals: *mut ::core::ffi::c_void, ppexchange: *mut *mut ::core::ffi::c_void, abmac: *mut u8) -> ::windows::core::HRESULT,
    pub GetRights: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdata: *const u8, dwsize: u32, pbspsessionkey: *const u8, dwsessionkeylen: u32, pstgglobals: *mut ::core::ffi::c_void, pprights: *mut *mut WMDMRIGHTS, pnrightscount: *mut u32, abmac: *mut u8) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
#[repr(transparent)]
pub struct ISCPSecureQuery2(::windows::core::IUnknown);
impl ISCPSecureQuery2 {
    pub unsafe fn GetDataDemands(&self, pfuflags: *mut u32, pdwminrightsdata: *mut u32, pdwminexaminedata: *mut u32, pdwmindecidedata: *mut u32, abmac: *mut u8) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.GetDataDemands)(::windows::core::Interface::as_raw(self), pfuflags, pdwminrightsdata, pdwminexaminedata, pdwmindecidedata, abmac).ok()
    }
    pub unsafe fn ExamineData<P0>(&self, fuflags: u32, pwszextension: P0, pdata: &[u8], abmac: *mut u8) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).base__.ExamineData)(::windows::core::Interface::as_raw(self), fuflags, pwszextension.into_param().abi(), ::core::mem::transmute(pdata.as_ptr()), pdata.len() as _, abmac).ok()
    }
    pub unsafe fn MakeDecision<P0>(&self, fuflags: u32, pdata: &[u8], dwappsec: u32, pbspsessionkey: &[u8], pstorageglobals: P0, ppexchange: *mut ::core::option::Option<ISCPSecureExchange>, abmac: *mut u8) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<IMDSPStorageGlobals>,
    {
        (::windows::core::Interface::vtable(self).base__.MakeDecision)(::windows::core::Interface::as_raw(self), fuflags, ::core::mem::transmute(pdata.as_ptr()), pdata.len() as _, dwappsec, ::core::mem::transmute(pbspsessionkey.as_ptr()), pbspsessionkey.len() as _, pstorageglobals.into_param().abi(), ::core::mem::transmute(ppexchange), abmac).ok()
    }
    pub unsafe fn GetRights<P0>(&self, pdata: &[u8], pbspsessionkey: &[u8], pstgglobals: P0, pprights: *mut *mut WMDMRIGHTS, pnrightscount: *mut u32, abmac: *mut u8) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<IMDSPStorageGlobals>,
    {
        (::windows::core::Interface::vtable(self).base__.GetRights)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pdata.as_ptr()), pdata.len() as _, ::core::mem::transmute(pbspsessionkey.as_ptr()), pbspsessionkey.len() as _, pstgglobals.into_param().abi(), pprights, pnrightscount, abmac).ok()
    }
    pub unsafe fn MakeDecision2<P0, P1>(&self, fuflags: u32, pdata: &[u8], dwappsec: u32, pbspsessionkey: &[u8], pstorageglobals: P0, pappcertapp: &[u8], pappcertsp: &[u8], pszrevocationurl: *mut ::windows::core::PWSTR, pdwrevocationurllen: *mut u32, pdwrevocationbitflag: *mut u32, pqwfilesize: ::core::option::Option<*mut u64>, punknown: P1, ppexchange: *mut ::core::option::Option<ISCPSecureExchange>, abmac: *mut u8) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<IMDSPStorageGlobals>,
        P1: ::windows::core::IntoParam<::windows::core::IUnknown>,
    {
        (::windows::core::Interface::vtable(self).MakeDecision2)(
            ::windows::core::Interface::as_raw(self),
            fuflags,
            ::core::mem::transmute(pdata.as_ptr()),
            pdata.len() as _,
            dwappsec,
            ::core::mem::transmute(pbspsessionkey.as_ptr()),
            pbspsessionkey.len() as _,
            pstorageglobals.into_param().abi(),
            ::core::mem::transmute(pappcertapp.as_ptr()),
            pappcertapp.len() as _,
            ::core::mem::transmute(pappcertsp.as_ptr()),
            pappcertsp.len() as _,
            pszrevocationurl,
            pdwrevocationurllen,
            pdwrevocationbitflag,
            ::core::mem::transmute(pqwfilesize.unwrap_or(::std::ptr::null_mut())),
            punknown.into_param().abi(),
            ::core::mem::transmute(ppexchange),
            abmac,
        )
        .ok()
    }
}
::windows::imp::interface_hierarchy!(ISCPSecureQuery2, ::windows::core::IUnknown, ISCPSecureQuery);
impl ::core::cmp::PartialEq for ISCPSecureQuery2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ISCPSecureQuery2 {}
impl ::core::fmt::Debug for ISCPSecureQuery2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISCPSecureQuery2").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for ISCPSecureQuery2 {
    type Vtable = ISCPSecureQuery2_Vtbl;
}
impl ::core::clone::Clone for ISCPSecureQuery2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for ISCPSecureQuery2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xebe17e25_4fd7_4632_af46_6d93d4fcc72e);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISCPSecureQuery2_Vtbl {
    pub base__: ISCPSecureQuery_Vtbl,
    pub MakeDecision2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, fuflags: u32, pdata: *const u8, dwsize: u32, dwappsec: u32, pbspsessionkey: *const u8, dwsessionkeylen: u32, pstorageglobals: *mut ::core::ffi::c_void, pappcertapp: *const u8, dwappcertapplen: u32, pappcertsp: *const u8, dwappcertsplen: u32, pszrevocationurl: *mut ::windows::core::PWSTR, pdwrevocationurllen: *mut u32, pdwrevocationbitflag: *mut u32, pqwfilesize: *mut u64, punknown: *mut ::core::ffi::c_void, ppexchange: *mut *mut ::core::ffi::c_void, abmac: *mut u8) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
#[repr(transparent)]
pub struct ISCPSecureQuery3(::windows::core::IUnknown);
impl ISCPSecureQuery3 {
    pub unsafe fn GetDataDemands(&self, pfuflags: *mut u32, pdwminrightsdata: *mut u32, pdwminexaminedata: *mut u32, pdwmindecidedata: *mut u32, abmac: *mut u8) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.GetDataDemands)(::windows::core::Interface::as_raw(self), pfuflags, pdwminrightsdata, pdwminexaminedata, pdwmindecidedata, abmac).ok()
    }
    pub unsafe fn ExamineData<P0>(&self, fuflags: u32, pwszextension: P0, pdata: &[u8], abmac: *mut u8) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).base__.base__.ExamineData)(::windows::core::Interface::as_raw(self), fuflags, pwszextension.into_param().abi(), ::core::mem::transmute(pdata.as_ptr()), pdata.len() as _, abmac).ok()
    }
    pub unsafe fn MakeDecision<P0>(&self, fuflags: u32, pdata: &[u8], dwappsec: u32, pbspsessionkey: &[u8], pstorageglobals: P0, ppexchange: *mut ::core::option::Option<ISCPSecureExchange>, abmac: *mut u8) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<IMDSPStorageGlobals>,
    {
        (::windows::core::Interface::vtable(self).base__.base__.MakeDecision)(::windows::core::Interface::as_raw(self), fuflags, ::core::mem::transmute(pdata.as_ptr()), pdata.len() as _, dwappsec, ::core::mem::transmute(pbspsessionkey.as_ptr()), pbspsessionkey.len() as _, pstorageglobals.into_param().abi(), ::core::mem::transmute(ppexchange), abmac).ok()
    }
    pub unsafe fn GetRights<P0>(&self, pdata: &[u8], pbspsessionkey: &[u8], pstgglobals: P0, pprights: *mut *mut WMDMRIGHTS, pnrightscount: *mut u32, abmac: *mut u8) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<IMDSPStorageGlobals>,
    {
        (::windows::core::Interface::vtable(self).base__.base__.GetRights)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pdata.as_ptr()), pdata.len() as _, ::core::mem::transmute(pbspsessionkey.as_ptr()), pbspsessionkey.len() as _, pstgglobals.into_param().abi(), pprights, pnrightscount, abmac).ok()
    }
    pub unsafe fn MakeDecision2<P0, P1>(&self, fuflags: u32, pdata: &[u8], dwappsec: u32, pbspsessionkey: &[u8], pstorageglobals: P0, pappcertapp: &[u8], pappcertsp: &[u8], pszrevocationurl: *mut ::windows::core::PWSTR, pdwrevocationurllen: *mut u32, pdwrevocationbitflag: *mut u32, pqwfilesize: ::core::option::Option<*mut u64>, punknown: P1, ppexchange: *mut ::core::option::Option<ISCPSecureExchange>, abmac: *mut u8) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<IMDSPStorageGlobals>,
        P1: ::windows::core::IntoParam<::windows::core::IUnknown>,
    {
        (::windows::core::Interface::vtable(self).base__.MakeDecision2)(
            ::windows::core::Interface::as_raw(self),
            fuflags,
            ::core::mem::transmute(pdata.as_ptr()),
            pdata.len() as _,
            dwappsec,
            ::core::mem::transmute(pbspsessionkey.as_ptr()),
            pbspsessionkey.len() as _,
            pstorageglobals.into_param().abi(),
            ::core::mem::transmute(pappcertapp.as_ptr()),
            pappcertapp.len() as _,
            ::core::mem::transmute(pappcertsp.as_ptr()),
            pappcertsp.len() as _,
            pszrevocationurl,
            pdwrevocationurllen,
            pdwrevocationbitflag,
            ::core::mem::transmute(pqwfilesize.unwrap_or(::std::ptr::null_mut())),
            punknown.into_param().abi(),
            ::core::mem::transmute(ppexchange),
            abmac,
        )
        .ok()
    }
    pub unsafe fn GetRightsOnClearChannel<P0, P1>(&self, pdata: &[u8], pbspsessionkey: &[u8], pstgglobals: P0, pprogresscallback: P1, pprights: *mut *mut WMDMRIGHTS, pnrightscount: *mut u32) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<IMDSPStorageGlobals>,
        P1: ::windows::core::IntoParam<IWMDMProgress3>,
    {
        (::windows::core::Interface::vtable(self).GetRightsOnClearChannel)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pdata.as_ptr()), pdata.len() as _, ::core::mem::transmute(pbspsessionkey.as_ptr()), pbspsessionkey.len() as _, pstgglobals.into_param().abi(), pprogresscallback.into_param().abi(), pprights, pnrightscount).ok()
    }
    pub unsafe fn MakeDecisionOnClearChannel<P0, P1, P2>(&self, fuflags: u32, pdata: &[u8], dwappsec: u32, pbspsessionkey: &[u8], pstorageglobals: P0, pprogresscallback: P1, pappcertapp: &[u8], pappcertsp: &[u8], pszrevocationurl: *mut ::windows::core::PWSTR, pdwrevocationurllen: *mut u32, pdwrevocationbitflag: *mut u32, pqwfilesize: ::core::option::Option<*mut u64>, punknown: P2, ppexchange: *mut ::core::option::Option<ISCPSecureExchange>) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<IMDSPStorageGlobals>,
        P1: ::windows::core::IntoParam<IWMDMProgress3>,
        P2: ::windows::core::IntoParam<::windows::core::IUnknown>,
    {
        (::windows::core::Interface::vtable(self).MakeDecisionOnClearChannel)(
            ::windows::core::Interface::as_raw(self),
            fuflags,
            ::core::mem::transmute(pdata.as_ptr()),
            pdata.len() as _,
            dwappsec,
            ::core::mem::transmute(pbspsessionkey.as_ptr()),
            pbspsessionkey.len() as _,
            pstorageglobals.into_param().abi(),
            pprogresscallback.into_param().abi(),
            ::core::mem::transmute(pappcertapp.as_ptr()),
            pappcertapp.len() as _,
            ::core::mem::transmute(pappcertsp.as_ptr()),
            pappcertsp.len() as _,
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
::windows::imp::interface_hierarchy!(ISCPSecureQuery3, ::windows::core::IUnknown, ISCPSecureQuery, ISCPSecureQuery2);
impl ::core::cmp::PartialEq for ISCPSecureQuery3 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ISCPSecureQuery3 {}
impl ::core::fmt::Debug for ISCPSecureQuery3 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISCPSecureQuery3").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for ISCPSecureQuery3 {
    type Vtable = ISCPSecureQuery3_Vtbl;
}
impl ::core::clone::Clone for ISCPSecureQuery3 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for ISCPSecureQuery3 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb7edd1a2_4dab_484b_b3c5_ad39b8b4c0b1);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISCPSecureQuery3_Vtbl {
    pub base__: ISCPSecureQuery2_Vtbl,
    pub GetRightsOnClearChannel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdata: *const u8, dwsize: u32, pbspsessionkey: *const u8, dwsessionkeylen: u32, pstgglobals: *mut ::core::ffi::c_void, pprogresscallback: *mut ::core::ffi::c_void, pprights: *mut *mut WMDMRIGHTS, pnrightscount: *mut u32) -> ::windows::core::HRESULT,
    pub MakeDecisionOnClearChannel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, fuflags: u32, pdata: *const u8, dwsize: u32, dwappsec: u32, pbspsessionkey: *const u8, dwsessionkeylen: u32, pstorageglobals: *mut ::core::ffi::c_void, pprogresscallback: *mut ::core::ffi::c_void, pappcertapp: *const u8, dwappcertapplen: u32, pappcertsp: *const u8, dwappcertsplen: u32, pszrevocationurl: *mut ::windows::core::PWSTR, pdwrevocationurllen: *mut u32, pdwrevocationbitflag: *mut u32, pqwfilesize: *mut u64, punknown: *mut ::core::ffi::c_void, ppexchange: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
#[repr(transparent)]
pub struct ISCPSession(::windows::core::IUnknown);
impl ISCPSession {
    pub unsafe fn BeginSession<P0>(&self, pidevice: P0, pctx: &[u8]) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<IMDSPDevice>,
    {
        (::windows::core::Interface::vtable(self).BeginSession)(::windows::core::Interface::as_raw(self), pidevice.into_param().abi(), ::core::mem::transmute(pctx.as_ptr()), pctx.len() as _).ok()
    }
    pub unsafe fn EndSession(&self, pctx: &[u8]) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).EndSession)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pctx.as_ptr()), pctx.len() as _).ok()
    }
    pub unsafe fn GetSecureQuery(&self) -> ::windows::core::Result<ISCPSecureQuery> {
        let mut result__ = ::windows::core::zeroed::<ISCPSecureQuery>();
        (::windows::core::Interface::vtable(self).GetSecureQuery)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
::windows::imp::interface_hierarchy!(ISCPSession, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for ISCPSession {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ISCPSession {}
impl ::core::fmt::Debug for ISCPSession {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISCPSession").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for ISCPSession {
    type Vtable = ISCPSession_Vtbl;
}
impl ::core::clone::Clone for ISCPSession {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for ISCPSession {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x88a3e6ed_eee4_4619_bbb3_fd4fb62715d1);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISCPSession_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub BeginSession: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pidevice: *mut ::core::ffi::c_void, pctx: *const u8, dwsizectx: u32) -> ::windows::core::HRESULT,
    pub EndSession: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pctx: *const u8, dwsizectx: u32) -> ::windows::core::HRESULT,
    pub GetSecureQuery: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppsecurequery: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
#[repr(transparent)]
pub struct IWMDMDevice(::windows::core::IUnknown);
impl IWMDMDevice {
    pub unsafe fn GetName(&self, pwszname: &mut [u16]) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetName)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pwszname.as_ptr()), pwszname.len() as _).ok()
    }
    pub unsafe fn GetManufacturer(&self, pwszname: &mut [u16]) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetManufacturer)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pwszname.as_ptr()), pwszname.len() as _).ok()
    }
    pub unsafe fn GetVersion(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::windows::core::zeroed::<u32>();
        (::windows::core::Interface::vtable(self).GetVersion)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetType(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::windows::core::zeroed::<u32>();
        (::windows::core::Interface::vtable(self).GetType)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetSerialNumber(&self, pserialnumber: *mut WMDMID, abmac: *mut u8) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetSerialNumber)(::windows::core::Interface::as_raw(self), pserialnumber, abmac).ok()
    }
    pub unsafe fn GetPowerSource(&self, pdwpowersource: *mut u32, pdwpercentremaining: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetPowerSource)(::windows::core::Interface::as_raw(self), pdwpowersource, pdwpercentremaining).ok()
    }
    pub unsafe fn GetStatus(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::windows::core::zeroed::<u32>();
        (::windows::core::Interface::vtable(self).GetStatus)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetDeviceIcon(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::windows::core::zeroed::<u32>();
        (::windows::core::Interface::vtable(self).GetDeviceIcon)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn EnumStorage(&self) -> ::windows::core::Result<IWMDMEnumStorage> {
        let mut result__ = ::windows::core::zeroed::<IWMDMEnumStorage>();
        (::windows::core::Interface::vtable(self).EnumStorage)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
    #[cfg(feature = "Win32_Media_Audio")]
    pub unsafe fn GetFormatSupport(&self, ppformatex: *mut *mut super::Audio::WAVEFORMATEX, pnformatcount: *mut u32, pppwszmimetype: *mut *mut ::windows::core::PWSTR, pnmimetypecount: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetFormatSupport)(::windows::core::Interface::as_raw(self), ppformatex, pnformatcount, pppwszmimetype, pnmimetypecount).ok()
    }
    pub unsafe fn SendOpaqueCommand(&self, pcommand: *mut OPAQUECOMMAND) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SendOpaqueCommand)(::windows::core::Interface::as_raw(self), pcommand).ok()
    }
}
::windows::imp::interface_hierarchy!(IWMDMDevice, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IWMDMDevice {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWMDMDevice {}
impl ::core::fmt::Debug for IWMDMDevice {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWMDMDevice").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IWMDMDevice {
    type Vtable = IWMDMDevice_Vtbl;
}
impl ::core::clone::Clone for IWMDMDevice {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IWMDMDevice {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1dcb3a02_33ed_11d3_8470_00c04f79dbc0);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMDMDevice_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub GetName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwszname: ::windows::core::PWSTR, nmaxchars: u32) -> ::windows::core::HRESULT,
    pub GetManufacturer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwszname: ::windows::core::PWSTR, nmaxchars: u32) -> ::windows::core::HRESULT,
    pub GetVersion: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdwversion: *mut u32) -> ::windows::core::HRESULT,
    pub GetType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdwtype: *mut u32) -> ::windows::core::HRESULT,
    pub GetSerialNumber: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pserialnumber: *mut WMDMID, abmac: *mut u8) -> ::windows::core::HRESULT,
    pub GetPowerSource: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdwpowersource: *mut u32, pdwpercentremaining: *mut u32) -> ::windows::core::HRESULT,
    pub GetStatus: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdwstatus: *mut u32) -> ::windows::core::HRESULT,
    pub GetDeviceIcon: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hicon: *mut u32) -> ::windows::core::HRESULT,
    pub EnumStorage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppenumstorage: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Media_Audio")]
    pub GetFormatSupport: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppformatex: *mut *mut super::Audio::WAVEFORMATEX, pnformatcount: *mut u32, pppwszmimetype: *mut *mut ::windows::core::PWSTR, pnmimetypecount: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Media_Audio"))]
    GetFormatSupport: usize,
    pub SendOpaqueCommand: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcommand: *mut OPAQUECOMMAND) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
#[repr(transparent)]
pub struct IWMDMDevice2(::windows::core::IUnknown);
impl IWMDMDevice2 {
    pub unsafe fn GetName(&self, pwszname: &mut [u16]) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.GetName)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pwszname.as_ptr()), pwszname.len() as _).ok()
    }
    pub unsafe fn GetManufacturer(&self, pwszname: &mut [u16]) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.GetManufacturer)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pwszname.as_ptr()), pwszname.len() as _).ok()
    }
    pub unsafe fn GetVersion(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::windows::core::zeroed::<u32>();
        (::windows::core::Interface::vtable(self).base__.GetVersion)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetType(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::windows::core::zeroed::<u32>();
        (::windows::core::Interface::vtable(self).base__.GetType)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetSerialNumber(&self, pserialnumber: *mut WMDMID, abmac: *mut u8) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.GetSerialNumber)(::windows::core::Interface::as_raw(self), pserialnumber, abmac).ok()
    }
    pub unsafe fn GetPowerSource(&self, pdwpowersource: *mut u32, pdwpercentremaining: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.GetPowerSource)(::windows::core::Interface::as_raw(self), pdwpowersource, pdwpercentremaining).ok()
    }
    pub unsafe fn GetStatus(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::windows::core::zeroed::<u32>();
        (::windows::core::Interface::vtable(self).base__.GetStatus)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetDeviceIcon(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::windows::core::zeroed::<u32>();
        (::windows::core::Interface::vtable(self).base__.GetDeviceIcon)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn EnumStorage(&self) -> ::windows::core::Result<IWMDMEnumStorage> {
        let mut result__ = ::windows::core::zeroed::<IWMDMEnumStorage>();
        (::windows::core::Interface::vtable(self).base__.EnumStorage)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
    #[cfg(feature = "Win32_Media_Audio")]
    pub unsafe fn GetFormatSupport(&self, ppformatex: *mut *mut super::Audio::WAVEFORMATEX, pnformatcount: *mut u32, pppwszmimetype: *mut *mut ::windows::core::PWSTR, pnmimetypecount: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.GetFormatSupport)(::windows::core::Interface::as_raw(self), ppformatex, pnformatcount, pppwszmimetype, pnmimetypecount).ok()
    }
    pub unsafe fn SendOpaqueCommand(&self, pcommand: *mut OPAQUECOMMAND) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.SendOpaqueCommand)(::windows::core::Interface::as_raw(self), pcommand).ok()
    }
    pub unsafe fn GetStorage<P0>(&self, pszstoragename: P0) -> ::windows::core::Result<IWMDMStorage>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        let mut result__ = ::windows::core::zeroed::<IWMDMStorage>();
        (::windows::core::Interface::vtable(self).GetStorage)(::windows::core::Interface::as_raw(self), pszstoragename.into_param().abi(), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`, `\"Win32_Media_Audio\"`, `\"Win32_Media_MediaFoundation\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_Media_Audio", feature = "Win32_Media_MediaFoundation"))]
    pub unsafe fn GetFormatSupport2(&self, dwflags: u32, ppaudioformatex: *mut *mut super::Audio::WAVEFORMATEX, pnaudioformatcount: *mut u32, ppvideoformatex: *mut *mut super::MediaFoundation::VIDEOINFOHEADER, pnvideoformatcount: *mut u32, ppfiletype: *mut *mut WMFILECAPABILITIES, pnfiletypecount: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetFormatSupport2)(::windows::core::Interface::as_raw(self), dwflags, ppaudioformatex, pnaudioformatcount, ppvideoformatex, pnvideoformatcount, ppfiletype, pnfiletypecount).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Ole\"`*"]
    #[cfg(feature = "Win32_System_Ole")]
    pub unsafe fn GetSpecifyPropertyPages(&self, ppspecifyproppages: *mut ::core::option::Option<super::super::System::Ole::ISpecifyPropertyPages>, pppunknowns: *mut *mut ::core::option::Option<::windows::core::IUnknown>, pcunks: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetSpecifyPropertyPages)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(ppspecifyproppages), pppunknowns, pcunks).ok()
    }
    pub unsafe fn GetCanonicalName(&self, pwszpnpname: &mut [u16]) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetCanonicalName)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pwszpnpname.as_ptr()), pwszpnpname.len() as _).ok()
    }
}
::windows::imp::interface_hierarchy!(IWMDMDevice2, ::windows::core::IUnknown, IWMDMDevice);
impl ::core::cmp::PartialEq for IWMDMDevice2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWMDMDevice2 {}
impl ::core::fmt::Debug for IWMDMDevice2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWMDMDevice2").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IWMDMDevice2 {
    type Vtable = IWMDMDevice2_Vtbl;
}
impl ::core::clone::Clone for IWMDMDevice2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IWMDMDevice2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe34f3d37_9d67_4fc1_9252_62d28b2f8b55);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMDMDevice2_Vtbl {
    pub base__: IWMDMDevice_Vtbl,
    pub GetStorage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszstoragename: ::windows::core::PCWSTR, ppstorage: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_Media_Audio", feature = "Win32_Media_MediaFoundation"))]
    pub GetFormatSupport2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwflags: u32, ppaudioformatex: *mut *mut super::Audio::WAVEFORMATEX, pnaudioformatcount: *mut u32, ppvideoformatex: *mut *mut super::MediaFoundation::VIDEOINFOHEADER, pnvideoformatcount: *mut u32, ppfiletype: *mut *mut WMFILECAPABILITIES, pnfiletypecount: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_Media_Audio", feature = "Win32_Media_MediaFoundation")))]
    GetFormatSupport2: usize,
    #[cfg(feature = "Win32_System_Ole")]
    pub GetSpecifyPropertyPages: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppspecifyproppages: *mut *mut ::core::ffi::c_void, pppunknowns: *mut *mut ::core::option::Option<::windows::core::IUnknown>, pcunks: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Ole"))]
    GetSpecifyPropertyPages: usize,
    pub GetCanonicalName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwszpnpname: ::windows::core::PWSTR, nmaxchars: u32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
#[repr(transparent)]
pub struct IWMDMDevice3(::windows::core::IUnknown);
impl IWMDMDevice3 {
    pub unsafe fn GetName(&self, pwszname: &mut [u16]) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.GetName)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pwszname.as_ptr()), pwszname.len() as _).ok()
    }
    pub unsafe fn GetManufacturer(&self, pwszname: &mut [u16]) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.GetManufacturer)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pwszname.as_ptr()), pwszname.len() as _).ok()
    }
    pub unsafe fn GetVersion(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::windows::core::zeroed::<u32>();
        (::windows::core::Interface::vtable(self).base__.base__.GetVersion)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetType(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::windows::core::zeroed::<u32>();
        (::windows::core::Interface::vtable(self).base__.base__.GetType)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetSerialNumber(&self, pserialnumber: *mut WMDMID, abmac: *mut u8) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.GetSerialNumber)(::windows::core::Interface::as_raw(self), pserialnumber, abmac).ok()
    }
    pub unsafe fn GetPowerSource(&self, pdwpowersource: *mut u32, pdwpercentremaining: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.GetPowerSource)(::windows::core::Interface::as_raw(self), pdwpowersource, pdwpercentremaining).ok()
    }
    pub unsafe fn GetStatus(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::windows::core::zeroed::<u32>();
        (::windows::core::Interface::vtable(self).base__.base__.GetStatus)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetDeviceIcon(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::windows::core::zeroed::<u32>();
        (::windows::core::Interface::vtable(self).base__.base__.GetDeviceIcon)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn EnumStorage(&self) -> ::windows::core::Result<IWMDMEnumStorage> {
        let mut result__ = ::windows::core::zeroed::<IWMDMEnumStorage>();
        (::windows::core::Interface::vtable(self).base__.base__.EnumStorage)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
    #[cfg(feature = "Win32_Media_Audio")]
    pub unsafe fn GetFormatSupport(&self, ppformatex: *mut *mut super::Audio::WAVEFORMATEX, pnformatcount: *mut u32, pppwszmimetype: *mut *mut ::windows::core::PWSTR, pnmimetypecount: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.GetFormatSupport)(::windows::core::Interface::as_raw(self), ppformatex, pnformatcount, pppwszmimetype, pnmimetypecount).ok()
    }
    pub unsafe fn SendOpaqueCommand(&self, pcommand: *mut OPAQUECOMMAND) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.SendOpaqueCommand)(::windows::core::Interface::as_raw(self), pcommand).ok()
    }
    pub unsafe fn GetStorage<P0>(&self, pszstoragename: P0) -> ::windows::core::Result<IWMDMStorage>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        let mut result__ = ::windows::core::zeroed::<IWMDMStorage>();
        (::windows::core::Interface::vtable(self).base__.GetStorage)(::windows::core::Interface::as_raw(self), pszstoragename.into_param().abi(), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`, `\"Win32_Media_Audio\"`, `\"Win32_Media_MediaFoundation\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_Media_Audio", feature = "Win32_Media_MediaFoundation"))]
    pub unsafe fn GetFormatSupport2(&self, dwflags: u32, ppaudioformatex: *mut *mut super::Audio::WAVEFORMATEX, pnaudioformatcount: *mut u32, ppvideoformatex: *mut *mut super::MediaFoundation::VIDEOINFOHEADER, pnvideoformatcount: *mut u32, ppfiletype: *mut *mut WMFILECAPABILITIES, pnfiletypecount: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.GetFormatSupport2)(::windows::core::Interface::as_raw(self), dwflags, ppaudioformatex, pnaudioformatcount, ppvideoformatex, pnvideoformatcount, ppfiletype, pnfiletypecount).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Ole\"`*"]
    #[cfg(feature = "Win32_System_Ole")]
    pub unsafe fn GetSpecifyPropertyPages(&self, ppspecifyproppages: *mut ::core::option::Option<super::super::System::Ole::ISpecifyPropertyPages>, pppunknowns: *mut *mut ::core::option::Option<::windows::core::IUnknown>, pcunks: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.GetSpecifyPropertyPages)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(ppspecifyproppages), pppunknowns, pcunks).ok()
    }
    pub unsafe fn GetCanonicalName(&self, pwszpnpname: &mut [u16]) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.GetCanonicalName)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pwszpnpname.as_ptr()), pwszpnpname.len() as _).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com_StructuredStorage\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
    pub unsafe fn GetProperty<P0>(&self, pwszpropname: P0) -> ::windows::core::Result<super::super::System::Com::StructuredStorage::PROPVARIANT>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        let mut result__ = ::windows::core::zeroed::<super::super::System::Com::StructuredStorage::PROPVARIANT>();
        (::windows::core::Interface::vtable(self).GetProperty)(::windows::core::Interface::as_raw(self), pwszpropname.into_param().abi(), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com_StructuredStorage\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
    pub unsafe fn SetProperty<P0>(&self, pwszpropname: P0, pvalue: *const super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).SetProperty)(::windows::core::Interface::as_raw(self), pwszpropname.into_param().abi(), pvalue).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com_StructuredStorage\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
    pub unsafe fn GetFormatCapability(&self, format: WMDM_FORMATCODE) -> ::windows::core::Result<WMDM_FORMAT_CAPABILITY> {
        let mut result__ = ::windows::core::zeroed::<WMDM_FORMAT_CAPABILITY>();
        (::windows::core::Interface::vtable(self).GetFormatCapability)(::windows::core::Interface::as_raw(self), format, &mut result__).from_abi(result__)
    }
    pub unsafe fn DeviceIoControl(&self, dwiocontrolcode: u32, lpinbuffer: &[u8], lpoutbuffer: *mut u8, pnoutbuffersize: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).DeviceIoControl)(::windows::core::Interface::as_raw(self), dwiocontrolcode, ::core::mem::transmute(lpinbuffer.as_ptr()), lpinbuffer.len() as _, lpoutbuffer, pnoutbuffersize).ok()
    }
    pub unsafe fn FindStorage<P0>(&self, findscope: WMDM_FIND_SCOPE, pwszuniqueid: P0) -> ::windows::core::Result<IWMDMStorage>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        let mut result__ = ::windows::core::zeroed::<IWMDMStorage>();
        (::windows::core::Interface::vtable(self).FindStorage)(::windows::core::Interface::as_raw(self), findscope, pwszuniqueid.into_param().abi(), &mut result__).from_abi(result__)
    }
}
::windows::imp::interface_hierarchy!(IWMDMDevice3, ::windows::core::IUnknown, IWMDMDevice, IWMDMDevice2);
impl ::core::cmp::PartialEq for IWMDMDevice3 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWMDMDevice3 {}
impl ::core::fmt::Debug for IWMDMDevice3 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWMDMDevice3").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IWMDMDevice3 {
    type Vtable = IWMDMDevice3_Vtbl;
}
impl ::core::clone::Clone for IWMDMDevice3 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IWMDMDevice3 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6c03e4fe_05db_4dda_9e3c_06233a6d5d65);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMDMDevice3_Vtbl {
    pub base__: IWMDMDevice2_Vtbl,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
    pub GetProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwszpropname: ::windows::core::PCWSTR, pvalue: *mut super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage")))]
    GetProperty: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
    pub SetProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwszpropname: ::windows::core::PCWSTR, pvalue: *const super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage")))]
    SetProperty: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
    pub GetFormatCapability: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, format: WMDM_FORMATCODE, pformatsupport: *mut WMDM_FORMAT_CAPABILITY) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage")))]
    GetFormatCapability: usize,
    pub DeviceIoControl: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwiocontrolcode: u32, lpinbuffer: *const u8, ninbuffersize: u32, lpoutbuffer: *mut u8, pnoutbuffersize: *mut u32) -> ::windows::core::HRESULT,
    pub FindStorage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, findscope: WMDM_FIND_SCOPE, pwszuniqueid: ::windows::core::PCWSTR, ppstorage: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
#[repr(transparent)]
pub struct IWMDMDeviceControl(::windows::core::IUnknown);
impl IWMDMDeviceControl {
    pub unsafe fn GetStatus(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::windows::core::zeroed::<u32>();
        (::windows::core::Interface::vtable(self).GetStatus)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetCapabilities(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::windows::core::zeroed::<u32>();
        (::windows::core::Interface::vtable(self).GetCapabilities)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn Play(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Play)(::windows::core::Interface::as_raw(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
    #[cfg(feature = "Win32_Media_Audio")]
    pub unsafe fn Record(&self, pformat: *const super::Audio::WAVEFORMATEX) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Record)(::windows::core::Interface::as_raw(self), pformat).ok()
    }
    pub unsafe fn Pause(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Pause)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Resume(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Resume)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Stop(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Stop)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Seek(&self, fumode: u32, noffset: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Seek)(::windows::core::Interface::as_raw(self), fumode, noffset).ok()
    }
}
::windows::imp::interface_hierarchy!(IWMDMDeviceControl, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IWMDMDeviceControl {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWMDMDeviceControl {}
impl ::core::fmt::Debug for IWMDMDeviceControl {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWMDMDeviceControl").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IWMDMDeviceControl {
    type Vtable = IWMDMDeviceControl_Vtbl;
}
impl ::core::clone::Clone for IWMDMDeviceControl {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IWMDMDeviceControl {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1dcb3a04_33ed_11d3_8470_00c04f79dbc0);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMDMDeviceControl_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub GetStatus: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdwstatus: *mut u32) -> ::windows::core::HRESULT,
    pub GetCapabilities: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdwcapabilitiesmask: *mut u32) -> ::windows::core::HRESULT,
    pub Play: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Media_Audio")]
    pub Record: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pformat: *const super::Audio::WAVEFORMATEX) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Media_Audio"))]
    Record: usize,
    pub Pause: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Resume: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Stop: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Seek: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, fumode: u32, noffset: i32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
#[repr(transparent)]
pub struct IWMDMDeviceSession(::windows::core::IUnknown);
impl IWMDMDeviceSession {
    pub unsafe fn BeginSession(&self, r#type: WMDM_SESSION_TYPE, pctx: ::core::option::Option<&[u8]>) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).BeginSession)(::windows::core::Interface::as_raw(self), r#type, ::core::mem::transmute(pctx.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())), pctx.as_deref().map_or(0, |slice| slice.len() as _)).ok()
    }
    pub unsafe fn EndSession(&self, r#type: WMDM_SESSION_TYPE, pctx: ::core::option::Option<&[u8]>) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).EndSession)(::windows::core::Interface::as_raw(self), r#type, ::core::mem::transmute(pctx.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())), pctx.as_deref().map_or(0, |slice| slice.len() as _)).ok()
    }
}
::windows::imp::interface_hierarchy!(IWMDMDeviceSession, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IWMDMDeviceSession {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWMDMDeviceSession {}
impl ::core::fmt::Debug for IWMDMDeviceSession {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWMDMDeviceSession").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IWMDMDeviceSession {
    type Vtable = IWMDMDeviceSession_Vtbl;
}
impl ::core::clone::Clone for IWMDMDeviceSession {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IWMDMDeviceSession {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x82af0a65_9d96_412c_83e5_3c43e4b06cc7);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMDMDeviceSession_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub BeginSession: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, r#type: WMDM_SESSION_TYPE, pctx: *const u8, dwsizectx: u32) -> ::windows::core::HRESULT,
    pub EndSession: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, r#type: WMDM_SESSION_TYPE, pctx: *const u8, dwsizectx: u32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
#[repr(transparent)]
pub struct IWMDMEnumDevice(::windows::core::IUnknown);
impl IWMDMEnumDevice {
    pub unsafe fn Next(&self, celt: u32, ppdevice: *mut ::core::option::Option<IWMDMDevice>, pceltfetched: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Next)(::windows::core::Interface::as_raw(self), celt, ::core::mem::transmute(ppdevice), pceltfetched).ok()
    }
    pub unsafe fn Skip(&self, celt: u32) -> ::windows::core::Result<u32> {
        let mut result__ = ::windows::core::zeroed::<u32>();
        (::windows::core::Interface::vtable(self).Skip)(::windows::core::Interface::as_raw(self), celt, &mut result__).from_abi(result__)
    }
    pub unsafe fn Reset(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Reset)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Clone(&self) -> ::windows::core::Result<IWMDMEnumDevice> {
        let mut result__ = ::windows::core::zeroed::<IWMDMEnumDevice>();
        (::windows::core::Interface::vtable(self).Clone)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
::windows::imp::interface_hierarchy!(IWMDMEnumDevice, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IWMDMEnumDevice {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWMDMEnumDevice {}
impl ::core::fmt::Debug for IWMDMEnumDevice {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWMDMEnumDevice").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IWMDMEnumDevice {
    type Vtable = IWMDMEnumDevice_Vtbl;
}
impl ::core::clone::Clone for IWMDMEnumDevice {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IWMDMEnumDevice {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1dcb3a01_33ed_11d3_8470_00c04f79dbc0);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMDMEnumDevice_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub Next: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, celt: u32, ppdevice: *mut *mut ::core::ffi::c_void, pceltfetched: *mut u32) -> ::windows::core::HRESULT,
    pub Skip: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, celt: u32, pceltfetched: *mut u32) -> ::windows::core::HRESULT,
    pub Reset: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Clone: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppenumdevice: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
#[repr(transparent)]
pub struct IWMDMEnumStorage(::windows::core::IUnknown);
impl IWMDMEnumStorage {
    pub unsafe fn Next(&self, celt: u32, ppstorage: *mut ::core::option::Option<IWMDMStorage>, pceltfetched: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Next)(::windows::core::Interface::as_raw(self), celt, ::core::mem::transmute(ppstorage), pceltfetched).ok()
    }
    pub unsafe fn Skip(&self, celt: u32) -> ::windows::core::Result<u32> {
        let mut result__ = ::windows::core::zeroed::<u32>();
        (::windows::core::Interface::vtable(self).Skip)(::windows::core::Interface::as_raw(self), celt, &mut result__).from_abi(result__)
    }
    pub unsafe fn Reset(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Reset)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Clone(&self) -> ::windows::core::Result<IWMDMEnumStorage> {
        let mut result__ = ::windows::core::zeroed::<IWMDMEnumStorage>();
        (::windows::core::Interface::vtable(self).Clone)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
::windows::imp::interface_hierarchy!(IWMDMEnumStorage, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IWMDMEnumStorage {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWMDMEnumStorage {}
impl ::core::fmt::Debug for IWMDMEnumStorage {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWMDMEnumStorage").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IWMDMEnumStorage {
    type Vtable = IWMDMEnumStorage_Vtbl;
}
impl ::core::clone::Clone for IWMDMEnumStorage {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IWMDMEnumStorage {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1dcb3a05_33ed_11d3_8470_00c04f79dbc0);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMDMEnumStorage_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub Next: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, celt: u32, ppstorage: *mut *mut ::core::ffi::c_void, pceltfetched: *mut u32) -> ::windows::core::HRESULT,
    pub Skip: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, celt: u32, pceltfetched: *mut u32) -> ::windows::core::HRESULT,
    pub Reset: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Clone: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppenumstorage: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
#[repr(transparent)]
pub struct IWMDMLogger(::windows::core::IUnknown);
impl IWMDMLogger {
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsEnabled(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::windows::core::zeroed::<super::super::Foundation::BOOL>();
        (::windows::core::Interface::vtable(self).IsEnabled)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Enable<P0>(&self, fenable: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<super::super::Foundation::BOOL>,
    {
        (::windows::core::Interface::vtable(self).Enable)(::windows::core::Interface::as_raw(self), fenable.into_param().abi()).ok()
    }
    pub unsafe fn GetLogFileName(&self, pszfilename: ::windows::core::PSTR, nmaxchars: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetLogFileName)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pszfilename), nmaxchars).ok()
    }
    pub unsafe fn SetLogFileName<P0>(&self, pszfilename: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCSTR>,
    {
        (::windows::core::Interface::vtable(self).SetLogFileName)(::windows::core::Interface::as_raw(self), pszfilename.into_param().abi()).ok()
    }
    pub unsafe fn LogString<P0, P1>(&self, dwflags: u32, pszsrcname: P0, pszlog: P1) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCSTR>,
        P1: ::windows::core::IntoParam<::windows::core::PCSTR>,
    {
        (::windows::core::Interface::vtable(self).LogString)(::windows::core::Interface::as_raw(self), dwflags, pszsrcname.into_param().abi(), pszlog.into_param().abi()).ok()
    }
    pub unsafe fn LogDword<P0, P1>(&self, dwflags: u32, pszsrcname: P0, pszlogformat: P1, dwlog: u32) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCSTR>,
        P1: ::windows::core::IntoParam<::windows::core::PCSTR>,
    {
        (::windows::core::Interface::vtable(self).LogDword)(::windows::core::Interface::as_raw(self), dwflags, pszsrcname.into_param().abi(), pszlogformat.into_param().abi(), dwlog).ok()
    }
    pub unsafe fn Reset(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Reset)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn GetSizeParams(&self, pdwmaxsize: *mut u32, pdwshrinktosize: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetSizeParams)(::windows::core::Interface::as_raw(self), pdwmaxsize, pdwshrinktosize).ok()
    }
    pub unsafe fn SetSizeParams(&self, dwmaxsize: u32, dwshrinktosize: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetSizeParams)(::windows::core::Interface::as_raw(self), dwmaxsize, dwshrinktosize).ok()
    }
}
::windows::imp::interface_hierarchy!(IWMDMLogger, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IWMDMLogger {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWMDMLogger {}
impl ::core::fmt::Debug for IWMDMLogger {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWMDMLogger").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IWMDMLogger {
    type Vtable = IWMDMLogger_Vtbl;
}
impl ::core::clone::Clone for IWMDMLogger {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IWMDMLogger {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x110a3200_5a79_11d3_8d78_444553540000);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMDMLogger_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub IsEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pfenabled: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    IsEnabled: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub Enable: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, fenable: super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Enable: usize,
    pub GetLogFileName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszfilename: ::windows::core::PSTR, nmaxchars: u32) -> ::windows::core::HRESULT,
    pub SetLogFileName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszfilename: ::windows::core::PCSTR) -> ::windows::core::HRESULT,
    pub LogString: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwflags: u32, pszsrcname: ::windows::core::PCSTR, pszlog: ::windows::core::PCSTR) -> ::windows::core::HRESULT,
    pub LogDword: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwflags: u32, pszsrcname: ::windows::core::PCSTR, pszlogformat: ::windows::core::PCSTR, dwlog: u32) -> ::windows::core::HRESULT,
    pub Reset: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetSizeParams: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdwmaxsize: *mut u32, pdwshrinktosize: *mut u32) -> ::windows::core::HRESULT,
    pub SetSizeParams: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwmaxsize: u32, dwshrinktosize: u32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
#[repr(transparent)]
pub struct IWMDMMetaData(::windows::core::IUnknown);
impl IWMDMMetaData {
    pub unsafe fn AddItem<P0>(&self, r#type: WMDM_TAG_DATATYPE, pwsztagname: P0, pvalue: ::core::option::Option<&[u8]>) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).AddItem)(::windows::core::Interface::as_raw(self), r#type, pwsztagname.into_param().abi(), ::core::mem::transmute(pvalue.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())), pvalue.as_deref().map_or(0, |slice| slice.len() as _)).ok()
    }
    pub unsafe fn QueryByName<P0>(&self, pwsztagname: P0, ptype: *mut WMDM_TAG_DATATYPE, pvalue: *mut *mut u8, pcblength: *mut u32) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).QueryByName)(::windows::core::Interface::as_raw(self), pwsztagname.into_param().abi(), ptype, pvalue, pcblength).ok()
    }
    pub unsafe fn QueryByIndex(&self, iindex: u32, ppwszname: *mut *mut u16, ptype: *mut WMDM_TAG_DATATYPE, ppvalue: *mut *mut u8, pcblength: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).QueryByIndex)(::windows::core::Interface::as_raw(self), iindex, ppwszname, ptype, ppvalue, pcblength).ok()
    }
    pub unsafe fn GetItemCount(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::windows::core::zeroed::<u32>();
        (::windows::core::Interface::vtable(self).GetItemCount)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
::windows::imp::interface_hierarchy!(IWMDMMetaData, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IWMDMMetaData {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWMDMMetaData {}
impl ::core::fmt::Debug for IWMDMMetaData {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWMDMMetaData").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IWMDMMetaData {
    type Vtable = IWMDMMetaData_Vtbl;
}
impl ::core::clone::Clone for IWMDMMetaData {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IWMDMMetaData {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xec3b0663_0951_460a_9a80_0dceed3c043c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMDMMetaData_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub AddItem: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, r#type: WMDM_TAG_DATATYPE, pwsztagname: ::windows::core::PCWSTR, pvalue: *const u8, ilength: u32) -> ::windows::core::HRESULT,
    pub QueryByName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwsztagname: ::windows::core::PCWSTR, ptype: *mut WMDM_TAG_DATATYPE, pvalue: *mut *mut u8, pcblength: *mut u32) -> ::windows::core::HRESULT,
    pub QueryByIndex: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iindex: u32, ppwszname: *mut *mut u16, ptype: *mut WMDM_TAG_DATATYPE, ppvalue: *mut *mut u8, pcblength: *mut u32) -> ::windows::core::HRESULT,
    pub GetItemCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, icount: *mut u32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
#[repr(transparent)]
pub struct IWMDMNotification(::windows::core::IUnknown);
impl IWMDMNotification {
    pub unsafe fn WMDMMessage<P0>(&self, dwmessagetype: u32, pwszcanonicalname: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).WMDMMessage)(::windows::core::Interface::as_raw(self), dwmessagetype, pwszcanonicalname.into_param().abi()).ok()
    }
}
::windows::imp::interface_hierarchy!(IWMDMNotification, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IWMDMNotification {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWMDMNotification {}
impl ::core::fmt::Debug for IWMDMNotification {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWMDMNotification").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IWMDMNotification {
    type Vtable = IWMDMNotification_Vtbl;
}
impl ::core::clone::Clone for IWMDMNotification {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IWMDMNotification {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3f5e95c0_0f43_4ed4_93d2_c89a45d59b81);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMDMNotification_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub WMDMMessage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwmessagetype: u32, pwszcanonicalname: ::windows::core::PCWSTR) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
#[repr(transparent)]
pub struct IWMDMObjectInfo(::windows::core::IUnknown);
impl IWMDMObjectInfo {
    pub unsafe fn GetPlayLength(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::windows::core::zeroed::<u32>();
        (::windows::core::Interface::vtable(self).GetPlayLength)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetPlayLength(&self, dwlength: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetPlayLength)(::windows::core::Interface::as_raw(self), dwlength).ok()
    }
    pub unsafe fn GetPlayOffset(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::windows::core::zeroed::<u32>();
        (::windows::core::Interface::vtable(self).GetPlayOffset)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetPlayOffset(&self, dwoffset: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetPlayOffset)(::windows::core::Interface::as_raw(self), dwoffset).ok()
    }
    pub unsafe fn GetTotalLength(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::windows::core::zeroed::<u32>();
        (::windows::core::Interface::vtable(self).GetTotalLength)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetLastPlayPosition(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::windows::core::zeroed::<u32>();
        (::windows::core::Interface::vtable(self).GetLastPlayPosition)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetLongestPlayPosition(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::windows::core::zeroed::<u32>();
        (::windows::core::Interface::vtable(self).GetLongestPlayPosition)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
::windows::imp::interface_hierarchy!(IWMDMObjectInfo, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IWMDMObjectInfo {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWMDMObjectInfo {}
impl ::core::fmt::Debug for IWMDMObjectInfo {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWMDMObjectInfo").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IWMDMObjectInfo {
    type Vtable = IWMDMObjectInfo_Vtbl;
}
impl ::core::clone::Clone for IWMDMObjectInfo {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IWMDMObjectInfo {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1dcb3a09_33ed_11d3_8470_00c04f79dbc0);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMDMObjectInfo_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub GetPlayLength: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdwlength: *mut u32) -> ::windows::core::HRESULT,
    pub SetPlayLength: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwlength: u32) -> ::windows::core::HRESULT,
    pub GetPlayOffset: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdwoffset: *mut u32) -> ::windows::core::HRESULT,
    pub SetPlayOffset: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwoffset: u32) -> ::windows::core::HRESULT,
    pub GetTotalLength: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdwlength: *mut u32) -> ::windows::core::HRESULT,
    pub GetLastPlayPosition: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdwlastpos: *mut u32) -> ::windows::core::HRESULT,
    pub GetLongestPlayPosition: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdwlongestpos: *mut u32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
#[repr(transparent)]
pub struct IWMDMOperation(::windows::core::IUnknown);
impl IWMDMOperation {
    pub unsafe fn BeginRead(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).BeginRead)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn BeginWrite(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).BeginWrite)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn GetObjectName(&self, pwszname: &mut [u16]) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetObjectName)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pwszname.as_ptr()), pwszname.len() as _).ok()
    }
    pub unsafe fn SetObjectName(&self, pwszname: &[u16]) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetObjectName)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pwszname.as_ptr()), pwszname.len() as _).ok()
    }
    #[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
    #[cfg(feature = "Win32_Media_Audio")]
    pub unsafe fn GetObjectAttributes(&self, pdwattributes: *mut u32, pformat: ::core::option::Option<*mut super::Audio::WAVEFORMATEX>) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetObjectAttributes)(::windows::core::Interface::as_raw(self), pdwattributes, ::core::mem::transmute(pformat.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    #[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
    #[cfg(feature = "Win32_Media_Audio")]
    pub unsafe fn SetObjectAttributes(&self, dwattributes: u32, pformat: ::core::option::Option<*const super::Audio::WAVEFORMATEX>) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetObjectAttributes)(::windows::core::Interface::as_raw(self), dwattributes, ::core::mem::transmute(pformat.unwrap_or(::std::ptr::null()))).ok()
    }
    pub unsafe fn GetObjectTotalSize(&self, pdwsize: *mut u32, pdwsizehigh: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetObjectTotalSize)(::windows::core::Interface::as_raw(self), pdwsize, pdwsizehigh).ok()
    }
    pub unsafe fn SetObjectTotalSize(&self, dwsize: u32, dwsizehigh: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetObjectTotalSize)(::windows::core::Interface::as_raw(self), dwsize, dwsizehigh).ok()
    }
    pub unsafe fn TransferObjectData(&self, pdata: *mut u8, pdwsize: *mut u32, abmac: *mut u8) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).TransferObjectData)(::windows::core::Interface::as_raw(self), pdata, pdwsize, abmac).ok()
    }
    pub unsafe fn End<P0>(&self, phcompletioncode: *const ::windows::core::HRESULT, pnewobject: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::IUnknown>,
    {
        (::windows::core::Interface::vtable(self).End)(::windows::core::Interface::as_raw(self), phcompletioncode, pnewobject.into_param().abi()).ok()
    }
}
::windows::imp::interface_hierarchy!(IWMDMOperation, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IWMDMOperation {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWMDMOperation {}
impl ::core::fmt::Debug for IWMDMOperation {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWMDMOperation").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IWMDMOperation {
    type Vtable = IWMDMOperation_Vtbl;
}
impl ::core::clone::Clone for IWMDMOperation {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IWMDMOperation {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1dcb3a0b_33ed_11d3_8470_00c04f79dbc0);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMDMOperation_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub BeginRead: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub BeginWrite: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetObjectName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwszname: ::windows::core::PWSTR, nmaxchars: u32) -> ::windows::core::HRESULT,
    pub SetObjectName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwszname: ::windows::core::PCWSTR, nmaxchars: u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Media_Audio")]
    pub GetObjectAttributes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdwattributes: *mut u32, pformat: *mut super::Audio::WAVEFORMATEX) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Media_Audio"))]
    GetObjectAttributes: usize,
    #[cfg(feature = "Win32_Media_Audio")]
    pub SetObjectAttributes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwattributes: u32, pformat: *const super::Audio::WAVEFORMATEX) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Media_Audio"))]
    SetObjectAttributes: usize,
    pub GetObjectTotalSize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdwsize: *mut u32, pdwsizehigh: *mut u32) -> ::windows::core::HRESULT,
    pub SetObjectTotalSize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwsize: u32, dwsizehigh: u32) -> ::windows::core::HRESULT,
    pub TransferObjectData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdata: *mut u8, pdwsize: *mut u32, abmac: *mut u8) -> ::windows::core::HRESULT,
    pub End: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, phcompletioncode: *const ::windows::core::HRESULT, pnewobject: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
#[repr(transparent)]
pub struct IWMDMOperation2(::windows::core::IUnknown);
impl IWMDMOperation2 {
    pub unsafe fn BeginRead(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.BeginRead)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn BeginWrite(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.BeginWrite)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn GetObjectName(&self, pwszname: &mut [u16]) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.GetObjectName)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pwszname.as_ptr()), pwszname.len() as _).ok()
    }
    pub unsafe fn SetObjectName(&self, pwszname: &[u16]) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.SetObjectName)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pwszname.as_ptr()), pwszname.len() as _).ok()
    }
    #[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
    #[cfg(feature = "Win32_Media_Audio")]
    pub unsafe fn GetObjectAttributes(&self, pdwattributes: *mut u32, pformat: ::core::option::Option<*mut super::Audio::WAVEFORMATEX>) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.GetObjectAttributes)(::windows::core::Interface::as_raw(self), pdwattributes, ::core::mem::transmute(pformat.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    #[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
    #[cfg(feature = "Win32_Media_Audio")]
    pub unsafe fn SetObjectAttributes(&self, dwattributes: u32, pformat: ::core::option::Option<*const super::Audio::WAVEFORMATEX>) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.SetObjectAttributes)(::windows::core::Interface::as_raw(self), dwattributes, ::core::mem::transmute(pformat.unwrap_or(::std::ptr::null()))).ok()
    }
    pub unsafe fn GetObjectTotalSize(&self, pdwsize: *mut u32, pdwsizehigh: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.GetObjectTotalSize)(::windows::core::Interface::as_raw(self), pdwsize, pdwsizehigh).ok()
    }
    pub unsafe fn SetObjectTotalSize(&self, dwsize: u32, dwsizehigh: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.SetObjectTotalSize)(::windows::core::Interface::as_raw(self), dwsize, dwsizehigh).ok()
    }
    pub unsafe fn TransferObjectData(&self, pdata: *mut u8, pdwsize: *mut u32, abmac: *mut u8) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.TransferObjectData)(::windows::core::Interface::as_raw(self), pdata, pdwsize, abmac).ok()
    }
    pub unsafe fn End<P0>(&self, phcompletioncode: *const ::windows::core::HRESULT, pnewobject: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::IUnknown>,
    {
        (::windows::core::Interface::vtable(self).base__.End)(::windows::core::Interface::as_raw(self), phcompletioncode, pnewobject.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`, `\"Win32_Media_Audio\"`, `\"Win32_Media_MediaFoundation\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_Media_Audio", feature = "Win32_Media_MediaFoundation"))]
    pub unsafe fn SetObjectAttributes2(&self, dwattributes: u32, dwattributesex: u32, pformat: ::core::option::Option<*const super::Audio::WAVEFORMATEX>, pvideoformat: ::core::option::Option<*const super::MediaFoundation::VIDEOINFOHEADER>) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetObjectAttributes2)(::windows::core::Interface::as_raw(self), dwattributes, dwattributesex, ::core::mem::transmute(pformat.unwrap_or(::std::ptr::null())), ::core::mem::transmute(pvideoformat.unwrap_or(::std::ptr::null()))).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`, `\"Win32_Media_Audio\"`, `\"Win32_Media_MediaFoundation\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_Media_Audio", feature = "Win32_Media_MediaFoundation"))]
    pub unsafe fn GetObjectAttributes2(&self, pdwattributes: *mut u32, pdwattributesex: *mut u32, paudioformat: ::core::option::Option<*mut super::Audio::WAVEFORMATEX>, pvideoformat: ::core::option::Option<*mut super::MediaFoundation::VIDEOINFOHEADER>) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetObjectAttributes2)(::windows::core::Interface::as_raw(self), pdwattributes, pdwattributesex, ::core::mem::transmute(paudioformat.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(pvideoformat.unwrap_or(::std::ptr::null_mut()))).ok()
    }
}
::windows::imp::interface_hierarchy!(IWMDMOperation2, ::windows::core::IUnknown, IWMDMOperation);
impl ::core::cmp::PartialEq for IWMDMOperation2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWMDMOperation2 {}
impl ::core::fmt::Debug for IWMDMOperation2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWMDMOperation2").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IWMDMOperation2 {
    type Vtable = IWMDMOperation2_Vtbl;
}
impl ::core::clone::Clone for IWMDMOperation2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IWMDMOperation2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x33445b48_7df7_425c_ad8f_0fc6d82f9f75);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMDMOperation2_Vtbl {
    pub base__: IWMDMOperation_Vtbl,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_Media_Audio", feature = "Win32_Media_MediaFoundation"))]
    pub SetObjectAttributes2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwattributes: u32, dwattributesex: u32, pformat: *const super::Audio::WAVEFORMATEX, pvideoformat: *const super::MediaFoundation::VIDEOINFOHEADER) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_Media_Audio", feature = "Win32_Media_MediaFoundation")))]
    SetObjectAttributes2: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_Media_Audio", feature = "Win32_Media_MediaFoundation"))]
    pub GetObjectAttributes2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdwattributes: *mut u32, pdwattributesex: *mut u32, paudioformat: *mut super::Audio::WAVEFORMATEX, pvideoformat: *mut super::MediaFoundation::VIDEOINFOHEADER) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_Media_Audio", feature = "Win32_Media_MediaFoundation")))]
    GetObjectAttributes2: usize,
}
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
#[repr(transparent)]
pub struct IWMDMOperation3(::windows::core::IUnknown);
impl IWMDMOperation3 {
    pub unsafe fn BeginRead(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.BeginRead)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn BeginWrite(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.BeginWrite)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn GetObjectName(&self, pwszname: &mut [u16]) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.GetObjectName)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pwszname.as_ptr()), pwszname.len() as _).ok()
    }
    pub unsafe fn SetObjectName(&self, pwszname: &[u16]) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.SetObjectName)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pwszname.as_ptr()), pwszname.len() as _).ok()
    }
    #[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
    #[cfg(feature = "Win32_Media_Audio")]
    pub unsafe fn GetObjectAttributes(&self, pdwattributes: *mut u32, pformat: ::core::option::Option<*mut super::Audio::WAVEFORMATEX>) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.GetObjectAttributes)(::windows::core::Interface::as_raw(self), pdwattributes, ::core::mem::transmute(pformat.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    #[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
    #[cfg(feature = "Win32_Media_Audio")]
    pub unsafe fn SetObjectAttributes(&self, dwattributes: u32, pformat: ::core::option::Option<*const super::Audio::WAVEFORMATEX>) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.SetObjectAttributes)(::windows::core::Interface::as_raw(self), dwattributes, ::core::mem::transmute(pformat.unwrap_or(::std::ptr::null()))).ok()
    }
    pub unsafe fn GetObjectTotalSize(&self, pdwsize: *mut u32, pdwsizehigh: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.GetObjectTotalSize)(::windows::core::Interface::as_raw(self), pdwsize, pdwsizehigh).ok()
    }
    pub unsafe fn SetObjectTotalSize(&self, dwsize: u32, dwsizehigh: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.SetObjectTotalSize)(::windows::core::Interface::as_raw(self), dwsize, dwsizehigh).ok()
    }
    pub unsafe fn TransferObjectData(&self, pdata: *mut u8, pdwsize: *mut u32, abmac: *mut u8) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.TransferObjectData)(::windows::core::Interface::as_raw(self), pdata, pdwsize, abmac).ok()
    }
    pub unsafe fn End<P0>(&self, phcompletioncode: *const ::windows::core::HRESULT, pnewobject: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::IUnknown>,
    {
        (::windows::core::Interface::vtable(self).base__.End)(::windows::core::Interface::as_raw(self), phcompletioncode, pnewobject.into_param().abi()).ok()
    }
    pub unsafe fn TransferObjectDataOnClearChannel(&self, pdata: *mut u8, pdwsize: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).TransferObjectDataOnClearChannel)(::windows::core::Interface::as_raw(self), pdata, pdwsize).ok()
    }
}
::windows::imp::interface_hierarchy!(IWMDMOperation3, ::windows::core::IUnknown, IWMDMOperation);
impl ::core::cmp::PartialEq for IWMDMOperation3 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWMDMOperation3 {}
impl ::core::fmt::Debug for IWMDMOperation3 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWMDMOperation3").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IWMDMOperation3 {
    type Vtable = IWMDMOperation3_Vtbl;
}
impl ::core::clone::Clone for IWMDMOperation3 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IWMDMOperation3 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd1f9b46a_9ca8_46d8_9d0f_1ec9bae54919);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMDMOperation3_Vtbl {
    pub base__: IWMDMOperation_Vtbl,
    pub TransferObjectDataOnClearChannel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdata: *mut u8, pdwsize: *mut u32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
#[repr(transparent)]
pub struct IWMDMProgress(::windows::core::IUnknown);
impl IWMDMProgress {
    pub unsafe fn Begin(&self, dwestimatedticks: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Begin)(::windows::core::Interface::as_raw(self), dwestimatedticks).ok()
    }
    pub unsafe fn Progress(&self, dwtranspiredticks: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Progress)(::windows::core::Interface::as_raw(self), dwtranspiredticks).ok()
    }
    pub unsafe fn End(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).End)(::windows::core::Interface::as_raw(self)).ok()
    }
}
::windows::imp::interface_hierarchy!(IWMDMProgress, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IWMDMProgress {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWMDMProgress {}
impl ::core::fmt::Debug for IWMDMProgress {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWMDMProgress").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IWMDMProgress {
    type Vtable = IWMDMProgress_Vtbl;
}
impl ::core::clone::Clone for IWMDMProgress {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IWMDMProgress {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1dcb3a0c_33ed_11d3_8470_00c04f79dbc0);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMDMProgress_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub Begin: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwestimatedticks: u32) -> ::windows::core::HRESULT,
    pub Progress: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwtranspiredticks: u32) -> ::windows::core::HRESULT,
    pub End: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
#[repr(transparent)]
pub struct IWMDMProgress2(::windows::core::IUnknown);
impl IWMDMProgress2 {
    pub unsafe fn Begin(&self, dwestimatedticks: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.Begin)(::windows::core::Interface::as_raw(self), dwestimatedticks).ok()
    }
    pub unsafe fn Progress(&self, dwtranspiredticks: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.Progress)(::windows::core::Interface::as_raw(self), dwtranspiredticks).ok()
    }
    pub unsafe fn End(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.End)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn End2(&self, hrcompletioncode: ::windows::core::HRESULT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).End2)(::windows::core::Interface::as_raw(self), hrcompletioncode).ok()
    }
}
::windows::imp::interface_hierarchy!(IWMDMProgress2, ::windows::core::IUnknown, IWMDMProgress);
impl ::core::cmp::PartialEq for IWMDMProgress2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWMDMProgress2 {}
impl ::core::fmt::Debug for IWMDMProgress2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWMDMProgress2").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IWMDMProgress2 {
    type Vtable = IWMDMProgress2_Vtbl;
}
impl ::core::clone::Clone for IWMDMProgress2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IWMDMProgress2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3a43f550_b383_4e92_b04a_e6bbc660fefc);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMDMProgress2_Vtbl {
    pub base__: IWMDMProgress_Vtbl,
    pub End2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hrcompletioncode: ::windows::core::HRESULT) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
#[repr(transparent)]
pub struct IWMDMProgress3(::windows::core::IUnknown);
impl IWMDMProgress3 {
    pub unsafe fn Begin(&self, dwestimatedticks: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.Begin)(::windows::core::Interface::as_raw(self), dwestimatedticks).ok()
    }
    pub unsafe fn Progress(&self, dwtranspiredticks: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.Progress)(::windows::core::Interface::as_raw(self), dwtranspiredticks).ok()
    }
    pub unsafe fn End(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.End)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn End2(&self, hrcompletioncode: ::windows::core::HRESULT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.End2)(::windows::core::Interface::as_raw(self), hrcompletioncode).ok()
    }
    pub unsafe fn Begin3(&self, eventid: ::windows::core::GUID, dwestimatedticks: u32, pcontext: ::core::option::Option<*mut OPAQUECOMMAND>) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Begin3)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(eventid), dwestimatedticks, ::core::mem::transmute(pcontext.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    pub unsafe fn Progress3(&self, eventid: ::windows::core::GUID, dwtranspiredticks: u32, pcontext: ::core::option::Option<*mut OPAQUECOMMAND>) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Progress3)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(eventid), dwtranspiredticks, ::core::mem::transmute(pcontext.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    pub unsafe fn End3(&self, eventid: ::windows::core::GUID, hrcompletioncode: ::windows::core::HRESULT, pcontext: ::core::option::Option<*mut OPAQUECOMMAND>) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).End3)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(eventid), hrcompletioncode, ::core::mem::transmute(pcontext.unwrap_or(::std::ptr::null_mut()))).ok()
    }
}
::windows::imp::interface_hierarchy!(IWMDMProgress3, ::windows::core::IUnknown, IWMDMProgress, IWMDMProgress2);
impl ::core::cmp::PartialEq for IWMDMProgress3 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWMDMProgress3 {}
impl ::core::fmt::Debug for IWMDMProgress3 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWMDMProgress3").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IWMDMProgress3 {
    type Vtable = IWMDMProgress3_Vtbl;
}
impl ::core::clone::Clone for IWMDMProgress3 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IWMDMProgress3 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x21de01cb_3bb4_4929_b21a_17af3f80f658);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMDMProgress3_Vtbl {
    pub base__: IWMDMProgress2_Vtbl,
    pub Begin3: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, eventid: ::windows::core::GUID, dwestimatedticks: u32, pcontext: *mut OPAQUECOMMAND) -> ::windows::core::HRESULT,
    pub Progress3: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, eventid: ::windows::core::GUID, dwtranspiredticks: u32, pcontext: *mut OPAQUECOMMAND) -> ::windows::core::HRESULT,
    pub End3: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, eventid: ::windows::core::GUID, hrcompletioncode: ::windows::core::HRESULT, pcontext: *mut OPAQUECOMMAND) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
#[repr(transparent)]
pub struct IWMDMRevoked(::windows::core::IUnknown);
impl IWMDMRevoked {
    pub unsafe fn GetRevocationURL(&self, ppwszrevocationurl: *mut ::windows::core::PWSTR, pdwbufferlen: *mut u32, pdwrevokedbitflag: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetRevocationURL)(::windows::core::Interface::as_raw(self), ppwszrevocationurl, pdwbufferlen, pdwrevokedbitflag).ok()
    }
}
::windows::imp::interface_hierarchy!(IWMDMRevoked, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IWMDMRevoked {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWMDMRevoked {}
impl ::core::fmt::Debug for IWMDMRevoked {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWMDMRevoked").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IWMDMRevoked {
    type Vtable = IWMDMRevoked_Vtbl;
}
impl ::core::clone::Clone for IWMDMRevoked {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IWMDMRevoked {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xebeccedb_88ee_4e55_b6a4_8d9f07d696aa);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMDMRevoked_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub GetRevocationURL: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppwszrevocationurl: *mut ::windows::core::PWSTR, pdwbufferlen: *mut u32, pdwrevokedbitflag: *mut u32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
#[repr(transparent)]
pub struct IWMDMStorage(::windows::core::IUnknown);
impl IWMDMStorage {
    #[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
    #[cfg(feature = "Win32_Media_Audio")]
    pub unsafe fn SetAttributes(&self, dwattributes: u32, pformat: ::core::option::Option<*const super::Audio::WAVEFORMATEX>) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetAttributes)(::windows::core::Interface::as_raw(self), dwattributes, ::core::mem::transmute(pformat.unwrap_or(::std::ptr::null()))).ok()
    }
    pub unsafe fn GetStorageGlobals(&self) -> ::windows::core::Result<IWMDMStorageGlobals> {
        let mut result__ = ::windows::core::zeroed::<IWMDMStorageGlobals>();
        (::windows::core::Interface::vtable(self).GetStorageGlobals)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
    #[cfg(feature = "Win32_Media_Audio")]
    pub unsafe fn GetAttributes(&self, pdwattributes: *mut u32, pformat: ::core::option::Option<*mut super::Audio::WAVEFORMATEX>) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetAttributes)(::windows::core::Interface::as_raw(self), pdwattributes, ::core::mem::transmute(pformat.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    pub unsafe fn GetName(&self, pwszname: &mut [u16]) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetName)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pwszname.as_ptr()), pwszname.len() as _).ok()
    }
    pub unsafe fn GetDate(&self) -> ::windows::core::Result<WMDMDATETIME> {
        let mut result__ = ::windows::core::zeroed::<WMDMDATETIME>();
        (::windows::core::Interface::vtable(self).GetDate)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetSize(&self, pdwsizelow: *mut u32, pdwsizehigh: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetSize)(::windows::core::Interface::as_raw(self), pdwsizelow, pdwsizehigh).ok()
    }
    pub unsafe fn GetRights(&self, pprights: *mut *mut WMDMRIGHTS, pnrightscount: *mut u32, abmac: *mut u8) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetRights)(::windows::core::Interface::as_raw(self), pprights, pnrightscount, abmac).ok()
    }
    pub unsafe fn EnumStorage(&self) -> ::windows::core::Result<IWMDMEnumStorage> {
        let mut result__ = ::windows::core::zeroed::<IWMDMEnumStorage>();
        (::windows::core::Interface::vtable(self).EnumStorage)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SendOpaqueCommand(&self, pcommand: *mut OPAQUECOMMAND) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SendOpaqueCommand)(::windows::core::Interface::as_raw(self), pcommand).ok()
    }
}
::windows::imp::interface_hierarchy!(IWMDMStorage, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IWMDMStorage {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWMDMStorage {}
impl ::core::fmt::Debug for IWMDMStorage {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWMDMStorage").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IWMDMStorage {
    type Vtable = IWMDMStorage_Vtbl;
}
impl ::core::clone::Clone for IWMDMStorage {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IWMDMStorage {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1dcb3a06_33ed_11d3_8470_00c04f79dbc0);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMDMStorage_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_Media_Audio")]
    pub SetAttributes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwattributes: u32, pformat: *const super::Audio::WAVEFORMATEX) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Media_Audio"))]
    SetAttributes: usize,
    pub GetStorageGlobals: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppstorageglobals: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Media_Audio")]
    pub GetAttributes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdwattributes: *mut u32, pformat: *mut super::Audio::WAVEFORMATEX) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Media_Audio"))]
    GetAttributes: usize,
    pub GetName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwszname: ::windows::core::PWSTR, nmaxchars: u32) -> ::windows::core::HRESULT,
    pub GetDate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdatetimeutc: *mut WMDMDATETIME) -> ::windows::core::HRESULT,
    pub GetSize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdwsizelow: *mut u32, pdwsizehigh: *mut u32) -> ::windows::core::HRESULT,
    pub GetRights: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pprights: *mut *mut WMDMRIGHTS, pnrightscount: *mut u32, abmac: *mut u8) -> ::windows::core::HRESULT,
    pub EnumStorage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, penumstorage: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SendOpaqueCommand: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcommand: *mut OPAQUECOMMAND) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
#[repr(transparent)]
pub struct IWMDMStorage2(::windows::core::IUnknown);
impl IWMDMStorage2 {
    #[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
    #[cfg(feature = "Win32_Media_Audio")]
    pub unsafe fn SetAttributes(&self, dwattributes: u32, pformat: ::core::option::Option<*const super::Audio::WAVEFORMATEX>) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.SetAttributes)(::windows::core::Interface::as_raw(self), dwattributes, ::core::mem::transmute(pformat.unwrap_or(::std::ptr::null()))).ok()
    }
    pub unsafe fn GetStorageGlobals(&self) -> ::windows::core::Result<IWMDMStorageGlobals> {
        let mut result__ = ::windows::core::zeroed::<IWMDMStorageGlobals>();
        (::windows::core::Interface::vtable(self).base__.GetStorageGlobals)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
    #[cfg(feature = "Win32_Media_Audio")]
    pub unsafe fn GetAttributes(&self, pdwattributes: *mut u32, pformat: ::core::option::Option<*mut super::Audio::WAVEFORMATEX>) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.GetAttributes)(::windows::core::Interface::as_raw(self), pdwattributes, ::core::mem::transmute(pformat.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    pub unsafe fn GetName(&self, pwszname: &mut [u16]) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.GetName)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pwszname.as_ptr()), pwszname.len() as _).ok()
    }
    pub unsafe fn GetDate(&self) -> ::windows::core::Result<WMDMDATETIME> {
        let mut result__ = ::windows::core::zeroed::<WMDMDATETIME>();
        (::windows::core::Interface::vtable(self).base__.GetDate)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetSize(&self, pdwsizelow: *mut u32, pdwsizehigh: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.GetSize)(::windows::core::Interface::as_raw(self), pdwsizelow, pdwsizehigh).ok()
    }
    pub unsafe fn GetRights(&self, pprights: *mut *mut WMDMRIGHTS, pnrightscount: *mut u32, abmac: *mut u8) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.GetRights)(::windows::core::Interface::as_raw(self), pprights, pnrightscount, abmac).ok()
    }
    pub unsafe fn EnumStorage(&self) -> ::windows::core::Result<IWMDMEnumStorage> {
        let mut result__ = ::windows::core::zeroed::<IWMDMEnumStorage>();
        (::windows::core::Interface::vtable(self).base__.EnumStorage)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SendOpaqueCommand(&self, pcommand: *mut OPAQUECOMMAND) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.SendOpaqueCommand)(::windows::core::Interface::as_raw(self), pcommand).ok()
    }
    pub unsafe fn GetStorage<P0>(&self, pszstoragename: P0) -> ::windows::core::Result<IWMDMStorage>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        let mut result__ = ::windows::core::zeroed::<IWMDMStorage>();
        (::windows::core::Interface::vtable(self).GetStorage)(::windows::core::Interface::as_raw(self), pszstoragename.into_param().abi(), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`, `\"Win32_Media_Audio\"`, `\"Win32_Media_MediaFoundation\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_Media_Audio", feature = "Win32_Media_MediaFoundation"))]
    pub unsafe fn SetAttributes2(&self, dwattributes: u32, dwattributesex: u32, pformat: ::core::option::Option<*const super::Audio::WAVEFORMATEX>, pvideoformat: ::core::option::Option<*const super::MediaFoundation::VIDEOINFOHEADER>) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetAttributes2)(::windows::core::Interface::as_raw(self), dwattributes, dwattributesex, ::core::mem::transmute(pformat.unwrap_or(::std::ptr::null())), ::core::mem::transmute(pvideoformat.unwrap_or(::std::ptr::null()))).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`, `\"Win32_Media_Audio\"`, `\"Win32_Media_MediaFoundation\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_Media_Audio", feature = "Win32_Media_MediaFoundation"))]
    pub unsafe fn GetAttributes2(&self, pdwattributes: *mut u32, pdwattributesex: *mut u32, paudioformat: ::core::option::Option<*mut super::Audio::WAVEFORMATEX>, pvideoformat: ::core::option::Option<*mut super::MediaFoundation::VIDEOINFOHEADER>) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetAttributes2)(::windows::core::Interface::as_raw(self), pdwattributes, pdwattributesex, ::core::mem::transmute(paudioformat.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(pvideoformat.unwrap_or(::std::ptr::null_mut()))).ok()
    }
}
::windows::imp::interface_hierarchy!(IWMDMStorage2, ::windows::core::IUnknown, IWMDMStorage);
impl ::core::cmp::PartialEq for IWMDMStorage2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWMDMStorage2 {}
impl ::core::fmt::Debug for IWMDMStorage2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWMDMStorage2").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IWMDMStorage2 {
    type Vtable = IWMDMStorage2_Vtbl;
}
impl ::core::clone::Clone for IWMDMStorage2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IWMDMStorage2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1ed5a144_5cd5_4683_9eff_72cbdb2d9533);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMDMStorage2_Vtbl {
    pub base__: IWMDMStorage_Vtbl,
    pub GetStorage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszstoragename: ::windows::core::PCWSTR, ppstorage: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_Media_Audio", feature = "Win32_Media_MediaFoundation"))]
    pub SetAttributes2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwattributes: u32, dwattributesex: u32, pformat: *const super::Audio::WAVEFORMATEX, pvideoformat: *const super::MediaFoundation::VIDEOINFOHEADER) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_Media_Audio", feature = "Win32_Media_MediaFoundation")))]
    SetAttributes2: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_Media_Audio", feature = "Win32_Media_MediaFoundation"))]
    pub GetAttributes2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdwattributes: *mut u32, pdwattributesex: *mut u32, paudioformat: *mut super::Audio::WAVEFORMATEX, pvideoformat: *mut super::MediaFoundation::VIDEOINFOHEADER) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_Media_Audio", feature = "Win32_Media_MediaFoundation")))]
    GetAttributes2: usize,
}
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
#[repr(transparent)]
pub struct IWMDMStorage3(::windows::core::IUnknown);
impl IWMDMStorage3 {
    #[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
    #[cfg(feature = "Win32_Media_Audio")]
    pub unsafe fn SetAttributes(&self, dwattributes: u32, pformat: ::core::option::Option<*const super::Audio::WAVEFORMATEX>) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.SetAttributes)(::windows::core::Interface::as_raw(self), dwattributes, ::core::mem::transmute(pformat.unwrap_or(::std::ptr::null()))).ok()
    }
    pub unsafe fn GetStorageGlobals(&self) -> ::windows::core::Result<IWMDMStorageGlobals> {
        let mut result__ = ::windows::core::zeroed::<IWMDMStorageGlobals>();
        (::windows::core::Interface::vtable(self).base__.base__.GetStorageGlobals)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
    #[cfg(feature = "Win32_Media_Audio")]
    pub unsafe fn GetAttributes(&self, pdwattributes: *mut u32, pformat: ::core::option::Option<*mut super::Audio::WAVEFORMATEX>) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.GetAttributes)(::windows::core::Interface::as_raw(self), pdwattributes, ::core::mem::transmute(pformat.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    pub unsafe fn GetName(&self, pwszname: &mut [u16]) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.GetName)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pwszname.as_ptr()), pwszname.len() as _).ok()
    }
    pub unsafe fn GetDate(&self) -> ::windows::core::Result<WMDMDATETIME> {
        let mut result__ = ::windows::core::zeroed::<WMDMDATETIME>();
        (::windows::core::Interface::vtable(self).base__.base__.GetDate)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetSize(&self, pdwsizelow: *mut u32, pdwsizehigh: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.GetSize)(::windows::core::Interface::as_raw(self), pdwsizelow, pdwsizehigh).ok()
    }
    pub unsafe fn GetRights(&self, pprights: *mut *mut WMDMRIGHTS, pnrightscount: *mut u32, abmac: *mut u8) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.GetRights)(::windows::core::Interface::as_raw(self), pprights, pnrightscount, abmac).ok()
    }
    pub unsafe fn EnumStorage(&self) -> ::windows::core::Result<IWMDMEnumStorage> {
        let mut result__ = ::windows::core::zeroed::<IWMDMEnumStorage>();
        (::windows::core::Interface::vtable(self).base__.base__.EnumStorage)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SendOpaqueCommand(&self, pcommand: *mut OPAQUECOMMAND) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.SendOpaqueCommand)(::windows::core::Interface::as_raw(self), pcommand).ok()
    }
    pub unsafe fn GetStorage<P0>(&self, pszstoragename: P0) -> ::windows::core::Result<IWMDMStorage>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        let mut result__ = ::windows::core::zeroed::<IWMDMStorage>();
        (::windows::core::Interface::vtable(self).base__.GetStorage)(::windows::core::Interface::as_raw(self), pszstoragename.into_param().abi(), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`, `\"Win32_Media_Audio\"`, `\"Win32_Media_MediaFoundation\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_Media_Audio", feature = "Win32_Media_MediaFoundation"))]
    pub unsafe fn SetAttributes2(&self, dwattributes: u32, dwattributesex: u32, pformat: ::core::option::Option<*const super::Audio::WAVEFORMATEX>, pvideoformat: ::core::option::Option<*const super::MediaFoundation::VIDEOINFOHEADER>) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.SetAttributes2)(::windows::core::Interface::as_raw(self), dwattributes, dwattributesex, ::core::mem::transmute(pformat.unwrap_or(::std::ptr::null())), ::core::mem::transmute(pvideoformat.unwrap_or(::std::ptr::null()))).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`, `\"Win32_Media_Audio\"`, `\"Win32_Media_MediaFoundation\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_Media_Audio", feature = "Win32_Media_MediaFoundation"))]
    pub unsafe fn GetAttributes2(&self, pdwattributes: *mut u32, pdwattributesex: *mut u32, paudioformat: ::core::option::Option<*mut super::Audio::WAVEFORMATEX>, pvideoformat: ::core::option::Option<*mut super::MediaFoundation::VIDEOINFOHEADER>) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.GetAttributes2)(::windows::core::Interface::as_raw(self), pdwattributes, pdwattributesex, ::core::mem::transmute(paudioformat.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(pvideoformat.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    pub unsafe fn GetMetadata(&self) -> ::windows::core::Result<IWMDMMetaData> {
        let mut result__ = ::windows::core::zeroed::<IWMDMMetaData>();
        (::windows::core::Interface::vtable(self).GetMetadata)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetMetadata<P0>(&self, pmetadata: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<IWMDMMetaData>,
    {
        (::windows::core::Interface::vtable(self).SetMetadata)(::windows::core::Interface::as_raw(self), pmetadata.into_param().abi()).ok()
    }
    pub unsafe fn CreateEmptyMetadataObject(&self) -> ::windows::core::Result<IWMDMMetaData> {
        let mut result__ = ::windows::core::zeroed::<IWMDMMetaData>();
        (::windows::core::Interface::vtable(self).CreateEmptyMetadataObject)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetEnumPreference(&self, pmode: *mut WMDM_STORAGE_ENUM_MODE, pviews: ::core::option::Option<&[WMDMMetadataView]>) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetEnumPreference)(::windows::core::Interface::as_raw(self), pmode, pviews.as_deref().map_or(0, |slice| slice.len() as _), ::core::mem::transmute(pviews.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr()))).ok()
    }
}
::windows::imp::interface_hierarchy!(IWMDMStorage3, ::windows::core::IUnknown, IWMDMStorage, IWMDMStorage2);
impl ::core::cmp::PartialEq for IWMDMStorage3 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWMDMStorage3 {}
impl ::core::fmt::Debug for IWMDMStorage3 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWMDMStorage3").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IWMDMStorage3 {
    type Vtable = IWMDMStorage3_Vtbl;
}
impl ::core::clone::Clone for IWMDMStorage3 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IWMDMStorage3 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x97717eea_926a_464e_96a4_247b0216026e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMDMStorage3_Vtbl {
    pub base__: IWMDMStorage2_Vtbl,
    pub GetMetadata: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppmetadata: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetMetadata: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pmetadata: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub CreateEmptyMetadataObject: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppmetadata: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetEnumPreference: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pmode: *mut WMDM_STORAGE_ENUM_MODE, nviews: u32, pviews: *const WMDMMetadataView) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
#[repr(transparent)]
pub struct IWMDMStorage4(::windows::core::IUnknown);
impl IWMDMStorage4 {
    #[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
    #[cfg(feature = "Win32_Media_Audio")]
    pub unsafe fn SetAttributes(&self, dwattributes: u32, pformat: ::core::option::Option<*const super::Audio::WAVEFORMATEX>) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.SetAttributes)(::windows::core::Interface::as_raw(self), dwattributes, ::core::mem::transmute(pformat.unwrap_or(::std::ptr::null()))).ok()
    }
    pub unsafe fn GetStorageGlobals(&self) -> ::windows::core::Result<IWMDMStorageGlobals> {
        let mut result__ = ::windows::core::zeroed::<IWMDMStorageGlobals>();
        (::windows::core::Interface::vtable(self).base__.base__.base__.GetStorageGlobals)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
    #[cfg(feature = "Win32_Media_Audio")]
    pub unsafe fn GetAttributes(&self, pdwattributes: *mut u32, pformat: ::core::option::Option<*mut super::Audio::WAVEFORMATEX>) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.GetAttributes)(::windows::core::Interface::as_raw(self), pdwattributes, ::core::mem::transmute(pformat.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    pub unsafe fn GetName(&self, pwszname: &mut [u16]) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.GetName)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pwszname.as_ptr()), pwszname.len() as _).ok()
    }
    pub unsafe fn GetDate(&self) -> ::windows::core::Result<WMDMDATETIME> {
        let mut result__ = ::windows::core::zeroed::<WMDMDATETIME>();
        (::windows::core::Interface::vtable(self).base__.base__.base__.GetDate)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetSize(&self, pdwsizelow: *mut u32, pdwsizehigh: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.GetSize)(::windows::core::Interface::as_raw(self), pdwsizelow, pdwsizehigh).ok()
    }
    pub unsafe fn GetRights(&self, pprights: *mut *mut WMDMRIGHTS, pnrightscount: *mut u32, abmac: *mut u8) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.GetRights)(::windows::core::Interface::as_raw(self), pprights, pnrightscount, abmac).ok()
    }
    pub unsafe fn EnumStorage(&self) -> ::windows::core::Result<IWMDMEnumStorage> {
        let mut result__ = ::windows::core::zeroed::<IWMDMEnumStorage>();
        (::windows::core::Interface::vtable(self).base__.base__.base__.EnumStorage)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SendOpaqueCommand(&self, pcommand: *mut OPAQUECOMMAND) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.SendOpaqueCommand)(::windows::core::Interface::as_raw(self), pcommand).ok()
    }
    pub unsafe fn GetStorage<P0>(&self, pszstoragename: P0) -> ::windows::core::Result<IWMDMStorage>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        let mut result__ = ::windows::core::zeroed::<IWMDMStorage>();
        (::windows::core::Interface::vtable(self).base__.base__.GetStorage)(::windows::core::Interface::as_raw(self), pszstoragename.into_param().abi(), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`, `\"Win32_Media_Audio\"`, `\"Win32_Media_MediaFoundation\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_Media_Audio", feature = "Win32_Media_MediaFoundation"))]
    pub unsafe fn SetAttributes2(&self, dwattributes: u32, dwattributesex: u32, pformat: ::core::option::Option<*const super::Audio::WAVEFORMATEX>, pvideoformat: ::core::option::Option<*const super::MediaFoundation::VIDEOINFOHEADER>) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.SetAttributes2)(::windows::core::Interface::as_raw(self), dwattributes, dwattributesex, ::core::mem::transmute(pformat.unwrap_or(::std::ptr::null())), ::core::mem::transmute(pvideoformat.unwrap_or(::std::ptr::null()))).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`, `\"Win32_Media_Audio\"`, `\"Win32_Media_MediaFoundation\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_Media_Audio", feature = "Win32_Media_MediaFoundation"))]
    pub unsafe fn GetAttributes2(&self, pdwattributes: *mut u32, pdwattributesex: *mut u32, paudioformat: ::core::option::Option<*mut super::Audio::WAVEFORMATEX>, pvideoformat: ::core::option::Option<*mut super::MediaFoundation::VIDEOINFOHEADER>) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.GetAttributes2)(::windows::core::Interface::as_raw(self), pdwattributes, pdwattributesex, ::core::mem::transmute(paudioformat.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(pvideoformat.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    pub unsafe fn GetMetadata(&self) -> ::windows::core::Result<IWMDMMetaData> {
        let mut result__ = ::windows::core::zeroed::<IWMDMMetaData>();
        (::windows::core::Interface::vtable(self).base__.GetMetadata)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetMetadata<P0>(&self, pmetadata: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<IWMDMMetaData>,
    {
        (::windows::core::Interface::vtable(self).base__.SetMetadata)(::windows::core::Interface::as_raw(self), pmetadata.into_param().abi()).ok()
    }
    pub unsafe fn CreateEmptyMetadataObject(&self) -> ::windows::core::Result<IWMDMMetaData> {
        let mut result__ = ::windows::core::zeroed::<IWMDMMetaData>();
        (::windows::core::Interface::vtable(self).base__.CreateEmptyMetadataObject)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetEnumPreference(&self, pmode: *mut WMDM_STORAGE_ENUM_MODE, pviews: ::core::option::Option<&[WMDMMetadataView]>) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.SetEnumPreference)(::windows::core::Interface::as_raw(self), pmode, pviews.as_deref().map_or(0, |slice| slice.len() as _), ::core::mem::transmute(pviews.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr()))).ok()
    }
    pub unsafe fn SetReferences(&self, ppiwmdmstorage: ::core::option::Option<&[::core::option::Option<IWMDMStorage>]>) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetReferences)(::windows::core::Interface::as_raw(self), ppiwmdmstorage.as_deref().map_or(0, |slice| slice.len() as _), ::core::mem::transmute(ppiwmdmstorage.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr()))).ok()
    }
    pub unsafe fn GetReferences(&self, pdwrefs: *mut u32, pppiwmdmstorage: *mut *mut ::core::option::Option<IWMDMStorage>) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetReferences)(::windows::core::Interface::as_raw(self), pdwrefs, pppiwmdmstorage).ok()
    }
    pub unsafe fn GetRightsWithProgress<P0>(&self, piprogresscallback: P0, pprights: *mut *mut WMDMRIGHTS, pnrightscount: *mut u32) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<IWMDMProgress3>,
    {
        (::windows::core::Interface::vtable(self).GetRightsWithProgress)(::windows::core::Interface::as_raw(self), piprogresscallback.into_param().abi(), pprights, pnrightscount).ok()
    }
    pub unsafe fn GetSpecifiedMetadata(&self, ppwszpropnames: &[::windows::core::PCWSTR]) -> ::windows::core::Result<IWMDMMetaData> {
        let mut result__ = ::windows::core::zeroed::<IWMDMMetaData>();
        (::windows::core::Interface::vtable(self).GetSpecifiedMetadata)(::windows::core::Interface::as_raw(self), ppwszpropnames.len() as _, ::core::mem::transmute(ppwszpropnames.as_ptr()), &mut result__).from_abi(result__)
    }
    pub unsafe fn FindStorage<P0>(&self, findscope: WMDM_FIND_SCOPE, pwszuniqueid: P0) -> ::windows::core::Result<IWMDMStorage>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        let mut result__ = ::windows::core::zeroed::<IWMDMStorage>();
        (::windows::core::Interface::vtable(self).FindStorage)(::windows::core::Interface::as_raw(self), findscope, pwszuniqueid.into_param().abi(), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetParent(&self) -> ::windows::core::Result<IWMDMStorage> {
        let mut result__ = ::windows::core::zeroed::<IWMDMStorage>();
        (::windows::core::Interface::vtable(self).GetParent)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
::windows::imp::interface_hierarchy!(IWMDMStorage4, ::windows::core::IUnknown, IWMDMStorage, IWMDMStorage2, IWMDMStorage3);
impl ::core::cmp::PartialEq for IWMDMStorage4 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWMDMStorage4 {}
impl ::core::fmt::Debug for IWMDMStorage4 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWMDMStorage4").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IWMDMStorage4 {
    type Vtable = IWMDMStorage4_Vtbl;
}
impl ::core::clone::Clone for IWMDMStorage4 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IWMDMStorage4 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc225bac5_a03a_40b8_9a23_91cf478c64a6);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMDMStorage4_Vtbl {
    pub base__: IWMDMStorage3_Vtbl,
    pub SetReferences: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwrefs: u32, ppiwmdmstorage: *const *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetReferences: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdwrefs: *mut u32, pppiwmdmstorage: *mut *mut ::core::option::Option<IWMDMStorage>) -> ::windows::core::HRESULT,
    pub GetRightsWithProgress: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, piprogresscallback: *mut ::core::ffi::c_void, pprights: *mut *mut WMDMRIGHTS, pnrightscount: *mut u32) -> ::windows::core::HRESULT,
    pub GetSpecifiedMetadata: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cproperties: u32, ppwszpropnames: *const ::windows::core::PCWSTR, ppmetadata: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub FindStorage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, findscope: WMDM_FIND_SCOPE, pwszuniqueid: ::windows::core::PCWSTR, ppstorage: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetParent: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppstorage: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
#[repr(transparent)]
pub struct IWMDMStorageControl(::windows::core::IUnknown);
impl IWMDMStorageControl {
    pub unsafe fn Insert<P0, P1, P2>(&self, fumode: u32, pwszfile: P0, poperation: P1, pprogress: P2) -> ::windows::core::Result<IWMDMStorage>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
        P1: ::windows::core::IntoParam<IWMDMOperation>,
        P2: ::windows::core::IntoParam<IWMDMProgress>,
    {
        let mut result__ = ::windows::core::zeroed::<IWMDMStorage>();
        (::windows::core::Interface::vtable(self).Insert)(::windows::core::Interface::as_raw(self), fumode, pwszfile.into_param().abi(), poperation.into_param().abi(), pprogress.into_param().abi(), &mut result__).from_abi(result__)
    }
    pub unsafe fn Delete<P0>(&self, fumode: u32, pprogress: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<IWMDMProgress>,
    {
        (::windows::core::Interface::vtable(self).Delete)(::windows::core::Interface::as_raw(self), fumode, pprogress.into_param().abi()).ok()
    }
    pub unsafe fn Rename<P0, P1>(&self, fumode: u32, pwsznewname: P0, pprogress: P1) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
        P1: ::windows::core::IntoParam<IWMDMProgress>,
    {
        (::windows::core::Interface::vtable(self).Rename)(::windows::core::Interface::as_raw(self), fumode, pwsznewname.into_param().abi(), pprogress.into_param().abi()).ok()
    }
    pub unsafe fn Read<P0, P1, P2>(&self, fumode: u32, pwszfile: P0, pprogress: P1, poperation: P2) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
        P1: ::windows::core::IntoParam<IWMDMProgress>,
        P2: ::windows::core::IntoParam<IWMDMOperation>,
    {
        (::windows::core::Interface::vtable(self).Read)(::windows::core::Interface::as_raw(self), fumode, pwszfile.into_param().abi(), pprogress.into_param().abi(), poperation.into_param().abi()).ok()
    }
    pub unsafe fn Move<P0, P1>(&self, fumode: u32, ptargetobject: P0, pprogress: P1) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<IWMDMStorage>,
        P1: ::windows::core::IntoParam<IWMDMProgress>,
    {
        (::windows::core::Interface::vtable(self).Move)(::windows::core::Interface::as_raw(self), fumode, ptargetobject.into_param().abi(), pprogress.into_param().abi()).ok()
    }
}
::windows::imp::interface_hierarchy!(IWMDMStorageControl, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IWMDMStorageControl {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWMDMStorageControl {}
impl ::core::fmt::Debug for IWMDMStorageControl {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWMDMStorageControl").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IWMDMStorageControl {
    type Vtable = IWMDMStorageControl_Vtbl;
}
impl ::core::clone::Clone for IWMDMStorageControl {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IWMDMStorageControl {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1dcb3a08_33ed_11d3_8470_00c04f79dbc0);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMDMStorageControl_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub Insert: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, fumode: u32, pwszfile: ::windows::core::PCWSTR, poperation: *mut ::core::ffi::c_void, pprogress: *mut ::core::ffi::c_void, ppnewobject: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Delete: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, fumode: u32, pprogress: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Rename: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, fumode: u32, pwsznewname: ::windows::core::PCWSTR, pprogress: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Read: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, fumode: u32, pwszfile: ::windows::core::PCWSTR, pprogress: *mut ::core::ffi::c_void, poperation: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Move: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, fumode: u32, ptargetobject: *mut ::core::ffi::c_void, pprogress: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
#[repr(transparent)]
pub struct IWMDMStorageControl2(::windows::core::IUnknown);
impl IWMDMStorageControl2 {
    pub unsafe fn Insert<P0, P1, P2>(&self, fumode: u32, pwszfile: P0, poperation: P1, pprogress: P2) -> ::windows::core::Result<IWMDMStorage>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
        P1: ::windows::core::IntoParam<IWMDMOperation>,
        P2: ::windows::core::IntoParam<IWMDMProgress>,
    {
        let mut result__ = ::windows::core::zeroed::<IWMDMStorage>();
        (::windows::core::Interface::vtable(self).base__.Insert)(::windows::core::Interface::as_raw(self), fumode, pwszfile.into_param().abi(), poperation.into_param().abi(), pprogress.into_param().abi(), &mut result__).from_abi(result__)
    }
    pub unsafe fn Delete<P0>(&self, fumode: u32, pprogress: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<IWMDMProgress>,
    {
        (::windows::core::Interface::vtable(self).base__.Delete)(::windows::core::Interface::as_raw(self), fumode, pprogress.into_param().abi()).ok()
    }
    pub unsafe fn Rename<P0, P1>(&self, fumode: u32, pwsznewname: P0, pprogress: P1) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
        P1: ::windows::core::IntoParam<IWMDMProgress>,
    {
        (::windows::core::Interface::vtable(self).base__.Rename)(::windows::core::Interface::as_raw(self), fumode, pwsznewname.into_param().abi(), pprogress.into_param().abi()).ok()
    }
    pub unsafe fn Read<P0, P1, P2>(&self, fumode: u32, pwszfile: P0, pprogress: P1, poperation: P2) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
        P1: ::windows::core::IntoParam<IWMDMProgress>,
        P2: ::windows::core::IntoParam<IWMDMOperation>,
    {
        (::windows::core::Interface::vtable(self).base__.Read)(::windows::core::Interface::as_raw(self), fumode, pwszfile.into_param().abi(), pprogress.into_param().abi(), poperation.into_param().abi()).ok()
    }
    pub unsafe fn Move<P0, P1>(&self, fumode: u32, ptargetobject: P0, pprogress: P1) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<IWMDMStorage>,
        P1: ::windows::core::IntoParam<IWMDMProgress>,
    {
        (::windows::core::Interface::vtable(self).base__.Move)(::windows::core::Interface::as_raw(self), fumode, ptargetobject.into_param().abi(), pprogress.into_param().abi()).ok()
    }
    pub unsafe fn Insert2<P0, P1, P2, P3, P4>(&self, fumode: u32, pwszfilesource: P0, pwszfiledest: P1, poperation: P2, pprogress: P3, punknown: P4, ppnewobject: ::core::option::Option<*mut ::core::option::Option<IWMDMStorage>>) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
        P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
        P2: ::windows::core::IntoParam<IWMDMOperation>,
        P3: ::windows::core::IntoParam<IWMDMProgress>,
        P4: ::windows::core::IntoParam<::windows::core::IUnknown>,
    {
        (::windows::core::Interface::vtable(self).Insert2)(::windows::core::Interface::as_raw(self), fumode, pwszfilesource.into_param().abi(), pwszfiledest.into_param().abi(), poperation.into_param().abi(), pprogress.into_param().abi(), punknown.into_param().abi(), ::core::mem::transmute(ppnewobject.unwrap_or(::std::ptr::null_mut()))).ok()
    }
}
::windows::imp::interface_hierarchy!(IWMDMStorageControl2, ::windows::core::IUnknown, IWMDMStorageControl);
impl ::core::cmp::PartialEq for IWMDMStorageControl2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWMDMStorageControl2 {}
impl ::core::fmt::Debug for IWMDMStorageControl2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWMDMStorageControl2").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IWMDMStorageControl2 {
    type Vtable = IWMDMStorageControl2_Vtbl;
}
impl ::core::clone::Clone for IWMDMStorageControl2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IWMDMStorageControl2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x972c2e88_bd6c_4125_8e09_84f837e637b6);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMDMStorageControl2_Vtbl {
    pub base__: IWMDMStorageControl_Vtbl,
    pub Insert2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, fumode: u32, pwszfilesource: ::windows::core::PCWSTR, pwszfiledest: ::windows::core::PCWSTR, poperation: *mut ::core::ffi::c_void, pprogress: *mut ::core::ffi::c_void, punknown: *mut ::core::ffi::c_void, ppnewobject: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
#[repr(transparent)]
pub struct IWMDMStorageControl3(::windows::core::IUnknown);
impl IWMDMStorageControl3 {
    pub unsafe fn Insert<P0, P1, P2>(&self, fumode: u32, pwszfile: P0, poperation: P1, pprogress: P2) -> ::windows::core::Result<IWMDMStorage>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
        P1: ::windows::core::IntoParam<IWMDMOperation>,
        P2: ::windows::core::IntoParam<IWMDMProgress>,
    {
        let mut result__ = ::windows::core::zeroed::<IWMDMStorage>();
        (::windows::core::Interface::vtable(self).base__.base__.Insert)(::windows::core::Interface::as_raw(self), fumode, pwszfile.into_param().abi(), poperation.into_param().abi(), pprogress.into_param().abi(), &mut result__).from_abi(result__)
    }
    pub unsafe fn Delete<P0>(&self, fumode: u32, pprogress: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<IWMDMProgress>,
    {
        (::windows::core::Interface::vtable(self).base__.base__.Delete)(::windows::core::Interface::as_raw(self), fumode, pprogress.into_param().abi()).ok()
    }
    pub unsafe fn Rename<P0, P1>(&self, fumode: u32, pwsznewname: P0, pprogress: P1) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
        P1: ::windows::core::IntoParam<IWMDMProgress>,
    {
        (::windows::core::Interface::vtable(self).base__.base__.Rename)(::windows::core::Interface::as_raw(self), fumode, pwsznewname.into_param().abi(), pprogress.into_param().abi()).ok()
    }
    pub unsafe fn Read<P0, P1, P2>(&self, fumode: u32, pwszfile: P0, pprogress: P1, poperation: P2) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
        P1: ::windows::core::IntoParam<IWMDMProgress>,
        P2: ::windows::core::IntoParam<IWMDMOperation>,
    {
        (::windows::core::Interface::vtable(self).base__.base__.Read)(::windows::core::Interface::as_raw(self), fumode, pwszfile.into_param().abi(), pprogress.into_param().abi(), poperation.into_param().abi()).ok()
    }
    pub unsafe fn Move<P0, P1>(&self, fumode: u32, ptargetobject: P0, pprogress: P1) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<IWMDMStorage>,
        P1: ::windows::core::IntoParam<IWMDMProgress>,
    {
        (::windows::core::Interface::vtable(self).base__.base__.Move)(::windows::core::Interface::as_raw(self), fumode, ptargetobject.into_param().abi(), pprogress.into_param().abi()).ok()
    }
    pub unsafe fn Insert2<P0, P1, P2, P3, P4>(&self, fumode: u32, pwszfilesource: P0, pwszfiledest: P1, poperation: P2, pprogress: P3, punknown: P4, ppnewobject: ::core::option::Option<*mut ::core::option::Option<IWMDMStorage>>) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
        P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
        P2: ::windows::core::IntoParam<IWMDMOperation>,
        P3: ::windows::core::IntoParam<IWMDMProgress>,
        P4: ::windows::core::IntoParam<::windows::core::IUnknown>,
    {
        (::windows::core::Interface::vtable(self).base__.Insert2)(::windows::core::Interface::as_raw(self), fumode, pwszfilesource.into_param().abi(), pwszfiledest.into_param().abi(), poperation.into_param().abi(), pprogress.into_param().abi(), punknown.into_param().abi(), ::core::mem::transmute(ppnewobject.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    pub unsafe fn Insert3<P0, P1, P2, P3, P4, P5>(&self, fumode: u32, futype: u32, pwszfilesource: P0, pwszfiledest: P1, poperation: P2, pprogress: P3, pmetadata: P4, punknown: P5, ppnewobject: ::core::option::Option<*mut ::core::option::Option<IWMDMStorage>>) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
        P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
        P2: ::windows::core::IntoParam<IWMDMOperation>,
        P3: ::windows::core::IntoParam<IWMDMProgress>,
        P4: ::windows::core::IntoParam<IWMDMMetaData>,
        P5: ::windows::core::IntoParam<::windows::core::IUnknown>,
    {
        (::windows::core::Interface::vtable(self).Insert3)(::windows::core::Interface::as_raw(self), fumode, futype, pwszfilesource.into_param().abi(), pwszfiledest.into_param().abi(), poperation.into_param().abi(), pprogress.into_param().abi(), pmetadata.into_param().abi(), punknown.into_param().abi(), ::core::mem::transmute(ppnewobject.unwrap_or(::std::ptr::null_mut()))).ok()
    }
}
::windows::imp::interface_hierarchy!(IWMDMStorageControl3, ::windows::core::IUnknown, IWMDMStorageControl, IWMDMStorageControl2);
impl ::core::cmp::PartialEq for IWMDMStorageControl3 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWMDMStorageControl3 {}
impl ::core::fmt::Debug for IWMDMStorageControl3 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWMDMStorageControl3").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IWMDMStorageControl3 {
    type Vtable = IWMDMStorageControl3_Vtbl;
}
impl ::core::clone::Clone for IWMDMStorageControl3 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IWMDMStorageControl3 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb3266365_d4f3_4696_8d53_bd27ec60993a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMDMStorageControl3_Vtbl {
    pub base__: IWMDMStorageControl2_Vtbl,
    pub Insert3: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, fumode: u32, futype: u32, pwszfilesource: ::windows::core::PCWSTR, pwszfiledest: ::windows::core::PCWSTR, poperation: *mut ::core::ffi::c_void, pprogress: *mut ::core::ffi::c_void, pmetadata: *mut ::core::ffi::c_void, punknown: *mut ::core::ffi::c_void, ppnewobject: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
#[repr(transparent)]
pub struct IWMDMStorageGlobals(::windows::core::IUnknown);
impl IWMDMStorageGlobals {
    pub unsafe fn GetCapabilities(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::windows::core::zeroed::<u32>();
        (::windows::core::Interface::vtable(self).GetCapabilities)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetSerialNumber(&self, pserialnum: *mut WMDMID, abmac: *mut u8) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetSerialNumber)(::windows::core::Interface::as_raw(self), pserialnum, abmac).ok()
    }
    pub unsafe fn GetTotalSize(&self, pdwtotalsizelow: *mut u32, pdwtotalsizehigh: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetTotalSize)(::windows::core::Interface::as_raw(self), pdwtotalsizelow, pdwtotalsizehigh).ok()
    }
    pub unsafe fn GetTotalFree(&self, pdwfreelow: *mut u32, pdwfreehigh: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetTotalFree)(::windows::core::Interface::as_raw(self), pdwfreelow, pdwfreehigh).ok()
    }
    pub unsafe fn GetTotalBad(&self, pdwbadlow: *mut u32, pdwbadhigh: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetTotalBad)(::windows::core::Interface::as_raw(self), pdwbadlow, pdwbadhigh).ok()
    }
    pub unsafe fn GetStatus(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::windows::core::zeroed::<u32>();
        (::windows::core::Interface::vtable(self).GetStatus)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn Initialize<P0>(&self, fumode: u32, pprogress: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<IWMDMProgress>,
    {
        (::windows::core::Interface::vtable(self).Initialize)(::windows::core::Interface::as_raw(self), fumode, pprogress.into_param().abi()).ok()
    }
}
::windows::imp::interface_hierarchy!(IWMDMStorageGlobals, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IWMDMStorageGlobals {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWMDMStorageGlobals {}
impl ::core::fmt::Debug for IWMDMStorageGlobals {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWMDMStorageGlobals").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IWMDMStorageGlobals {
    type Vtable = IWMDMStorageGlobals_Vtbl;
}
impl ::core::clone::Clone for IWMDMStorageGlobals {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IWMDMStorageGlobals {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1dcb3a07_33ed_11d3_8470_00c04f79dbc0);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMDMStorageGlobals_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub GetCapabilities: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdwcapabilities: *mut u32) -> ::windows::core::HRESULT,
    pub GetSerialNumber: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pserialnum: *mut WMDMID, abmac: *mut u8) -> ::windows::core::HRESULT,
    pub GetTotalSize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdwtotalsizelow: *mut u32, pdwtotalsizehigh: *mut u32) -> ::windows::core::HRESULT,
    pub GetTotalFree: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdwfreelow: *mut u32, pdwfreehigh: *mut u32) -> ::windows::core::HRESULT,
    pub GetTotalBad: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdwbadlow: *mut u32, pdwbadhigh: *mut u32) -> ::windows::core::HRESULT,
    pub GetStatus: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdwstatus: *mut u32) -> ::windows::core::HRESULT,
    pub Initialize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, fumode: u32, pprogress: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
#[repr(transparent)]
pub struct IWMDeviceManager(::windows::core::IUnknown);
impl IWMDeviceManager {
    pub unsafe fn GetRevision(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::windows::core::zeroed::<u32>();
        (::windows::core::Interface::vtable(self).GetRevision)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetDeviceCount(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::windows::core::zeroed::<u32>();
        (::windows::core::Interface::vtable(self).GetDeviceCount)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn EnumDevices(&self) -> ::windows::core::Result<IWMDMEnumDevice> {
        let mut result__ = ::windows::core::zeroed::<IWMDMEnumDevice>();
        (::windows::core::Interface::vtable(self).EnumDevices)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
::windows::imp::interface_hierarchy!(IWMDeviceManager, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IWMDeviceManager {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWMDeviceManager {}
impl ::core::fmt::Debug for IWMDeviceManager {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWMDeviceManager").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IWMDeviceManager {
    type Vtable = IWMDeviceManager_Vtbl;
}
impl ::core::clone::Clone for IWMDeviceManager {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IWMDeviceManager {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1dcb3a00_33ed_11d3_8470_00c04f79dbc0);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMDeviceManager_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub GetRevision: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdwrevision: *mut u32) -> ::windows::core::HRESULT,
    pub GetDeviceCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdwcount: *mut u32) -> ::windows::core::HRESULT,
    pub EnumDevices: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppenumdevice: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
#[repr(transparent)]
pub struct IWMDeviceManager2(::windows::core::IUnknown);
impl IWMDeviceManager2 {
    pub unsafe fn GetRevision(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::windows::core::zeroed::<u32>();
        (::windows::core::Interface::vtable(self).base__.GetRevision)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetDeviceCount(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::windows::core::zeroed::<u32>();
        (::windows::core::Interface::vtable(self).base__.GetDeviceCount)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn EnumDevices(&self) -> ::windows::core::Result<IWMDMEnumDevice> {
        let mut result__ = ::windows::core::zeroed::<IWMDMEnumDevice>();
        (::windows::core::Interface::vtable(self).base__.EnumDevices)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetDeviceFromCanonicalName<P0>(&self, pwszcanonicalname: P0) -> ::windows::core::Result<IWMDMDevice>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        let mut result__ = ::windows::core::zeroed::<IWMDMDevice>();
        (::windows::core::Interface::vtable(self).GetDeviceFromCanonicalName)(::windows::core::Interface::as_raw(self), pwszcanonicalname.into_param().abi(), &mut result__).from_abi(result__)
    }
    pub unsafe fn EnumDevices2(&self) -> ::windows::core::Result<IWMDMEnumDevice> {
        let mut result__ = ::windows::core::zeroed::<IWMDMEnumDevice>();
        (::windows::core::Interface::vtable(self).EnumDevices2)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn Reinitialize(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Reinitialize)(::windows::core::Interface::as_raw(self)).ok()
    }
}
::windows::imp::interface_hierarchy!(IWMDeviceManager2, ::windows::core::IUnknown, IWMDeviceManager);
impl ::core::cmp::PartialEq for IWMDeviceManager2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWMDeviceManager2 {}
impl ::core::fmt::Debug for IWMDeviceManager2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWMDeviceManager2").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IWMDeviceManager2 {
    type Vtable = IWMDeviceManager2_Vtbl;
}
impl ::core::clone::Clone for IWMDeviceManager2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IWMDeviceManager2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x923e5249_8731_4c5b_9b1c_b8b60b6e46af);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMDeviceManager2_Vtbl {
    pub base__: IWMDeviceManager_Vtbl,
    pub GetDeviceFromCanonicalName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwszcanonicalname: ::windows::core::PCWSTR, ppdevice: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub EnumDevices2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppenumdevice: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Reinitialize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
#[repr(transparent)]
pub struct IWMDeviceManager3(::windows::core::IUnknown);
impl IWMDeviceManager3 {
    pub unsafe fn GetRevision(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::windows::core::zeroed::<u32>();
        (::windows::core::Interface::vtable(self).base__.base__.GetRevision)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetDeviceCount(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::windows::core::zeroed::<u32>();
        (::windows::core::Interface::vtable(self).base__.base__.GetDeviceCount)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn EnumDevices(&self) -> ::windows::core::Result<IWMDMEnumDevice> {
        let mut result__ = ::windows::core::zeroed::<IWMDMEnumDevice>();
        (::windows::core::Interface::vtable(self).base__.base__.EnumDevices)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetDeviceFromCanonicalName<P0>(&self, pwszcanonicalname: P0) -> ::windows::core::Result<IWMDMDevice>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        let mut result__ = ::windows::core::zeroed::<IWMDMDevice>();
        (::windows::core::Interface::vtable(self).base__.GetDeviceFromCanonicalName)(::windows::core::Interface::as_raw(self), pwszcanonicalname.into_param().abi(), &mut result__).from_abi(result__)
    }
    pub unsafe fn EnumDevices2(&self) -> ::windows::core::Result<IWMDMEnumDevice> {
        let mut result__ = ::windows::core::zeroed::<IWMDMEnumDevice>();
        (::windows::core::Interface::vtable(self).base__.EnumDevices2)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn Reinitialize(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.Reinitialize)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn SetDeviceEnumPreference(&self, dwenumpref: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetDeviceEnumPreference)(::windows::core::Interface::as_raw(self), dwenumpref).ok()
    }
}
::windows::imp::interface_hierarchy!(IWMDeviceManager3, ::windows::core::IUnknown, IWMDeviceManager, IWMDeviceManager2);
impl ::core::cmp::PartialEq for IWMDeviceManager3 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWMDeviceManager3 {}
impl ::core::fmt::Debug for IWMDeviceManager3 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWMDeviceManager3").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IWMDeviceManager3 {
    type Vtable = IWMDeviceManager3_Vtbl;
}
impl ::core::clone::Clone for IWMDeviceManager3 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IWMDeviceManager3 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xaf185c41_100d_46ed_be2e_9ce8c44594ef);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMDeviceManager3_Vtbl {
    pub base__: IWMDeviceManager2_Vtbl,
    pub SetDeviceEnumPreference: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwenumpref: u32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const ALLOW_OUTOFBAND_NOTIFICATION: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const DO_NOT_VIRTUALIZE_STORAGES_AS_DEVICES: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const EVENT_WMDM_CONTENT_TRANSFER: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x339c9bf4_bcfe_4ed8_94df_eaf8c26ab61b);
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const IOCTL_MTP_CUSTOM_COMMAND: u32 = 827348045u32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const MDSP_READ: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const MDSP_SEEK_BOF: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const MDSP_SEEK_CUR: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const MDSP_SEEK_EOF: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const MDSP_WRITE: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const MTP_COMMAND_MAX_PARAMS: u32 = 5u32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const MTP_NEXTPHASE_NO_DATA: u32 = 3u32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const MTP_NEXTPHASE_READ_DATA: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const MTP_NEXTPHASE_WRITE_DATA: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const MTP_RESPONSE_MAX_PARAMS: u32 = 5u32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const MTP_RESPONSE_OK: u16 = 8193u16;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const MediaDevMgr: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x25baad81_3560_11d3_8471_00c04f79dbc0);
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const MediaDevMgrClassFactory: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x50040c1d_bdbf_4924_b873_f14d6c5bfd66);
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const RSA_KEY_LEN: u32 = 64u32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const SAC_CERT_V1: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const SAC_CERT_X509: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const SAC_MAC_LEN: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const SAC_PROTOCOL_V1: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const SAC_PROTOCOL_WMDM: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const SAC_SESSION_KEYLEN: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const SCP_EVENTID_ACQSECURECLOCK: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x86248cc9_4a59_43e2_9146_48a7f3f4140c);
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const SCP_EVENTID_DRMINFO: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x213dd287_41d2_432b_9e3f_3b4f7b3581dd);
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const SCP_EVENTID_NEEDTOINDIV: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x87a507c7_b469_4386_b976_d5d1ce538a6f);
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const SCP_PARAMID_DRMVERSION: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x41d0155d_7cc7_4217_ada9_005074624da4);
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDMDevice: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x807b3cdf_357a_11d3_8471_00c04f79dbc0);
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDMDeviceEnum: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x430e35af_3971_11d3_8474_00c04f79dbc0);
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDMID_LENGTH: u32 = 128u32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDMLogger: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x110a3202_5a79_11d3_8d78_444553540000);
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDMStorage: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x807b3ce0_357a_11d3_8471_00c04f79dbc0);
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDMStorageEnum: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xeb401a3b_3af7_11d3_8474_00c04f79dbc0);
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDMStorageGlobal: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x807b3ce1_357a_11d3_8471_00c04f79dbc0);
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_APP_REVOKED: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_CONTENT_FILE: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_CONTENT_FOLDER: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_CONTENT_OPERATIONINTERFACE: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_DEVICECAP_CANPAUSE: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_DEVICECAP_CANPLAY: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_DEVICECAP_CANRECORD: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_DEVICECAP_CANRESUME: u32 = 32u32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_DEVICECAP_CANSEEK: u32 = 128u32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_DEVICECAP_CANSTOP: u32 = 64u32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_DEVICECAP_CANSTREAMPLAY: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_DEVICECAP_CANSTREAMRECORD: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_DEVICECAP_HASSECURECLOCK: u32 = 256u32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_DEVICE_PROTOCOL_MSC: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa4d2c26c_a881_44bb_bd5d_1f703c71f7a9);
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_DEVICE_PROTOCOL_MTP: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x979e54e5_0afc_4604_8d93_dc798a4bcf45);
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_DEVICE_PROTOCOL_RAPI: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2a11ed91_8c8f_41e4_82d1_8386e003561c);
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_DEVICE_TYPE_DECODE: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_DEVICE_TYPE_ENCODE: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_DEVICE_TYPE_FILELISTRESYNC: u32 = 512u32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_DEVICE_TYPE_NONREENTRANT: u32 = 256u32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_DEVICE_TYPE_NONSDMI: u32 = 128u32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_DEVICE_TYPE_PLAYBACK: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_DEVICE_TYPE_RECORD: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_DEVICE_TYPE_SDMI: u32 = 64u32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_DEVICE_TYPE_STORAGE: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_DEVICE_TYPE_VIEW_PREF_METADATAVIEW: u32 = 1024u32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_DEVICE_TYPE_VIRTUAL: u32 = 32u32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_E_BUFFERTOOSMALL: i32 = -2147201016i32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_E_BUSY: i32 = -2147201024i32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_E_CALL_OUT_OF_SEQUENCE: i32 = -2147201017i32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_E_CANTOPEN_PMSN_SERVICE_PIPE: i32 = -2147201005i32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_E_INCORRECT_APPSEC: i32 = -2147201008i32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_E_INCORRECT_RIGHTS: i32 = -2147201007i32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_E_INTERFACEDEAD: i32 = -2147201023i32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_E_INVALIDTYPE: i32 = -2147201022i32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_E_LICENSE_EXPIRED: i32 = -2147201006i32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_E_LICENSE_NOTEXIST: i32 = -2147201009i32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_E_MAC_CHECK_FAILED: i32 = -2147201014i32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_E_MOREDATA: i32 = -2147201015i32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_E_NORIGHTS: i32 = -2147201018i32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_E_NOTCERTIFIED: i32 = -2147201019i32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_E_NOTSUPPORTED: i32 = -2147201020i32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_E_PROCESSFAILED: i32 = -2147201021i32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_E_REVOKED: i32 = -2147201010i32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_E_SDMI_NOMORECOPIES: i32 = -2147201011i32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_E_SDMI_TRIGGER: i32 = -2147201012i32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_E_TOO_MANY_SESSIONS: i32 = -2147201005i32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_E_USER_CANCELLED: i32 = -2147201013i32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_FILE_ATTR_AUDIO: u32 = 4096u32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_FILE_ATTR_AUDIOBOOK: u32 = 2097152u32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_FILE_ATTR_CANDELETE: u32 = 32768u32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_FILE_ATTR_CANMOVE: u32 = 65536u32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_FILE_ATTR_CANPLAY: u32 = 16384u32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_FILE_ATTR_CANREAD: u32 = 262144u32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_FILE_ATTR_CANRENAME: u32 = 131072u32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_FILE_ATTR_DATA: u32 = 8192u32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_FILE_ATTR_FILE: u32 = 32u32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_FILE_ATTR_FOLDER: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_FILE_ATTR_HIDDEN: u32 = 4194304u32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_FILE_ATTR_LINK: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_FILE_ATTR_MUSIC: u32 = 524288u32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_FILE_ATTR_READONLY: u32 = 16777216u32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_FILE_ATTR_SYSTEM: u32 = 8388608u32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_FILE_ATTR_VIDEO: u32 = 64u32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_FILE_CREATE_OVERWRITE: u32 = 1048576u32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_GET_FORMAT_SUPPORT_AUDIO: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_GET_FORMAT_SUPPORT_FILE: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_GET_FORMAT_SUPPORT_VIDEO: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_LOG_NOTIMESTAMP: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_LOG_SEV_ERROR: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_LOG_SEV_INFO: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_LOG_SEV_WARN: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_MAC_LENGTH: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_MODE_BLOCK: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_MODE_PROGRESS: u32 = 64u32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_MODE_QUERY: u32 = 32u32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_MODE_RECURSIVE: u32 = 4096u32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_MODE_THREAD: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_MODE_TRANSFER_PROTECTED: u32 = 128u32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_MODE_TRANSFER_UNPROTECTED: u32 = 256u32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_POWER_CAP_BATTERY: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_POWER_CAP_EXTERNAL: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_POWER_IS_BATTERY: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_POWER_IS_EXTERNAL: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_POWER_PERCENT_AVAILABLE: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_RIGHTS_COPY_TO_CD: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_RIGHTS_COPY_TO_NON_SDMI_DEVICE: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_RIGHTS_COPY_TO_SDMI_DEVICE: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_RIGHTS_EXPIRATIONDATE: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_RIGHTS_FREESERIALIDS: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_RIGHTS_GROUPID: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_RIGHTS_NAMEDSERIALIDS: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_RIGHTS_PLAYBACKCOUNT: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_RIGHTS_PLAY_ON_PC: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_SCP_DECIDE_DATA: i32 = 8i32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_SCP_DRMINFO_NOT_DRMPROTECTED: i32 = 0i32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_SCP_DRMINFO_V1HEADER: i32 = 1i32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_SCP_DRMINFO_V2HEADER: i32 = 2i32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_SCP_EXAMINE_DATA: i32 = 2i32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_SCP_EXAMINE_EXTENSION: i32 = 1i32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_SCP_NO_MORE_CHANGES: i32 = 64i32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_SCP_PROTECTED_OUTPUT: i32 = 16i32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_SCP_REVOKED: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_SCP_RIGHTS_DATA: i32 = 64i32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_SCP_TRANSFER_OBJECTDATA: i32 = 32i32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_SCP_UNPROTECTED_OUTPUT: i32 = 32i32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_SEEK_BEGIN: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_SEEK_CURRENT: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_SEEK_END: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_SEEK_REMOTECONTROL: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_SEEK_STREAMINGAUDIO: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_SERVICE_PROVIDER_VENDOR_MICROSOFT: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7de8686d_78ee_43ea_a496_c625ac91cc5d);
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_SP_REVOKED: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_STATUS_BUSY: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_STATUS_DEVICECONTROL_PAUSED: u32 = 32u32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_STATUS_DEVICECONTROL_PLAYING: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_STATUS_DEVICECONTROL_RECORDING: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_STATUS_DEVICECONTROL_REMOTE: u32 = 64u32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_STATUS_DEVICECONTROL_STREAM: u32 = 128u32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_STATUS_DEVICE_NOTPRESENT: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_STATUS_READY: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_STATUS_STORAGECONTROL_APPENDING: u32 = 32768u32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_STATUS_STORAGECONTROL_DELETING: u32 = 16384u32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_STATUS_STORAGECONTROL_INSERTING: u32 = 8192u32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_STATUS_STORAGECONTROL_MOVING: u32 = 65536u32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_STATUS_STORAGECONTROL_READING: u32 = 131072u32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_STATUS_STORAGE_BROKEN: u32 = 1024u32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_STATUS_STORAGE_INITIALIZING: u32 = 512u32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_STATUS_STORAGE_NOTPRESENT: u32 = 256u32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_STATUS_STORAGE_NOTSUPPORTED: u32 = 2048u32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_STATUS_STORAGE_UNFORMATTED: u32 = 4096u32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_STORAGECAP_FILELIMITEXISTS: u32 = 32u32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_STORAGECAP_FILESINFOLDERS: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_STORAGECAP_FILESINROOT: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_STORAGECAP_FOLDERLIMITEXISTS: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_STORAGECAP_FOLDERSINFOLDERS: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_STORAGECAP_FOLDERSINROOT: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_STORAGECAP_NOT_INITIALIZABLE: u32 = 64u32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_STORAGECONTROL_INSERTAFTER: u32 = 1024u32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_STORAGECONTROL_INSERTBEFORE: u32 = 512u32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_STORAGECONTROL_INSERTINTO: u32 = 2048u32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_STORAGE_ATTR_CANEDITMETADATA: u32 = 128u32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_STORAGE_ATTR_FILESYSTEM: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_STORAGE_ATTR_FOLDERS: u32 = 256u32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_STORAGE_ATTR_HAS_FILES: u32 = 67108864u32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_STORAGE_ATTR_HAS_FOLDERS: u32 = 33554432u32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_STORAGE_ATTR_NONREMOVABLE: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_STORAGE_ATTR_REMOVABLE: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_STORAGE_ATTR_VIRTUAL: u32 = 536870912u32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_STORAGE_CONTAINS_DEFAULT: u32 = 268435456u32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_STORAGE_IS_DEFAULT: u32 = 134217728u32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_S_NOT_ALL_PROPERTIES_APPLIED: i32 = 282625i32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_S_NOT_ALL_PROPERTIES_RETRIEVED: i32 = 282626i32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_WMDM_REVOKED: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const g_wszAudioWAVECodec: ::windows::core::PCWSTR = ::windows::w!("WMDM/AudioWAVECodec");
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const g_wszVideoFourCCCodec: ::windows::core::PCWSTR = ::windows::w!("WMDM/VideoFourCCCodec");
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const g_wszWMDMAlbumArt: ::windows::core::PCWSTR = ::windows::w!("WMDM/AlbumArt");
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const g_wszWMDMAlbumArtist: ::windows::core::PCWSTR = ::windows::w!("WMDM/AlbumArtist");
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const g_wszWMDMAlbumCoverData: ::windows::core::PCWSTR = ::windows::w!("WMDM/AlbumCoverData");
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const g_wszWMDMAlbumCoverDuration: ::windows::core::PCWSTR = ::windows::w!("WMDM/AlbumCoverDuration");
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const g_wszWMDMAlbumCoverFormat: ::windows::core::PCWSTR = ::windows::w!("WMDM/AlbumCoverFormat");
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const g_wszWMDMAlbumCoverHeight: ::windows::core::PCWSTR = ::windows::w!("WMDM/AlbumCoverHeight");
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const g_wszWMDMAlbumCoverSize: ::windows::core::PCWSTR = ::windows::w!("WMDM/AlbumCoverSize");
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const g_wszWMDMAlbumCoverWidth: ::windows::core::PCWSTR = ::windows::w!("WMDM/AlbumCoverWidth");
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const g_wszWMDMAlbumTitle: ::windows::core::PCWSTR = ::windows::w!("WMDM/AlbumTitle");
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const g_wszWMDMAudioBitDepth: ::windows::core::PCWSTR = ::windows::w!("WMDM/AudioBitDepth");
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const g_wszWMDMAuthor: ::windows::core::PCWSTR = ::windows::w!("WMDM/Author");
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const g_wszWMDMAuthorDate: ::windows::core::PCWSTR = ::windows::w!("WMDM/AuthorDate");
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const g_wszWMDMBitRateType: ::windows::core::PCWSTR = ::windows::w!("WMDM/BitRateType");
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const g_wszWMDMBitrate: ::windows::core::PCWSTR = ::windows::w!("WMDM/Bitrate");
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const g_wszWMDMBlockAlignment: ::windows::core::PCWSTR = ::windows::w!("WMDM/BlockAlignment");
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const g_wszWMDMBufferSize: ::windows::core::PCWSTR = ::windows::w!("WMDM/BufferSize");
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const g_wszWMDMBuyNow: ::windows::core::PCWSTR = ::windows::w!("WMDM/BuyNow");
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const g_wszWMDMByteBookmark: ::windows::core::PCWSTR = ::windows::w!("WMDM/ByteBookmark");
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const g_wszWMDMCategory: ::windows::core::PCWSTR = ::windows::w!("WMDM/Category");
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const g_wszWMDMCodec: ::windows::core::PCWSTR = ::windows::w!("WMDM/Codec");
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const g_wszWMDMCollectionID: ::windows::core::PCWSTR = ::windows::w!("WMDM/CollectionID");
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const g_wszWMDMComposer: ::windows::core::PCWSTR = ::windows::w!("WMDM/Composer");
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const g_wszWMDMDRMId: ::windows::core::PCWSTR = ::windows::w!("WMDM/DRMId");
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const g_wszWMDMDataLength: ::windows::core::PCWSTR = ::windows::w!("WMDM/DataLength");
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const g_wszWMDMDataOffset: ::windows::core::PCWSTR = ::windows::w!("WMDM/DataOffset");
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const g_wszWMDMDataUnits: ::windows::core::PCWSTR = ::windows::w!("WMDM/DataUnits");
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const g_wszWMDMDescription: ::windows::core::PCWSTR = ::windows::w!("WMDM/Description");
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const g_wszWMDMDestinationURL: ::windows::core::PCWSTR = ::windows::w!("WMDM/DestinationURL");
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const g_wszWMDMDeviceFirmwareVersion: ::windows::core::PCWSTR = ::windows::w!("WMDM/DeviceFirmwareVersion");
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const g_wszWMDMDeviceFriendlyName: ::windows::core::PCWSTR = ::windows::w!("WMDM/DeviceFriendlyName");
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const g_wszWMDMDeviceModelName: ::windows::core::PCWSTR = ::windows::w!("WMDM/DeviceModelName");
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const g_wszWMDMDevicePlayCount: ::windows::core::PCWSTR = ::windows::w!("WMDM/DevicePlayCount");
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const g_wszWMDMDeviceProtocol: ::windows::core::PCWSTR = ::windows::w!("WMDM/DeviceProtocol");
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const g_wszWMDMDeviceRevocationInfo: ::windows::core::PCWSTR = ::windows::w!("WMDM/DeviceRevocationInfo");
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const g_wszWMDMDeviceServiceProviderVendor: ::windows::core::PCWSTR = ::windows::w!("WMDM/DeviceServiceProviderVendor");
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const g_wszWMDMDeviceVendorExtension: ::windows::core::PCWSTR = ::windows::w!("WMDM/DeviceVendorExtension");
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const g_wszWMDMDuration: ::windows::core::PCWSTR = ::windows::w!("WMDM/Duration");
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const g_wszWMDMEditor: ::windows::core::PCWSTR = ::windows::w!("WMDM/Editor");
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const g_wszWMDMEncodingProfile: ::windows::core::PCWSTR = ::windows::w!("WMDM/EncodingProfile");
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const g_wszWMDMFileAttributes: ::windows::core::PCWSTR = ::windows::w!("WMDM/FileAttributes");
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const g_wszWMDMFileCreationDate: ::windows::core::PCWSTR = ::windows::w!("WMDM/FileCreationDate");
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const g_wszWMDMFileName: ::windows::core::PCWSTR = ::windows::w!("WMDM/FileName");
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const g_wszWMDMFileSize: ::windows::core::PCWSTR = ::windows::w!("WMDM/FileSize");
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const g_wszWMDMFormatCode: ::windows::core::PCWSTR = ::windows::w!("WMDM/FormatCode");
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const g_wszWMDMFormatsSupported: ::windows::core::PCWSTR = ::windows::w!("WMDM/FormatsSupported");
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const g_wszWMDMFormatsSupportedAreOrdered: ::windows::core::PCWSTR = ::windows::w!("WMDM/FormatsSupportedAreOrdered");
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const g_wszWMDMFrameRate: ::windows::core::PCWSTR = ::windows::w!("WMDM/FrameRate");
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const g_wszWMDMGenre: ::windows::core::PCWSTR = ::windows::w!("WMDM/Genre");
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const g_wszWMDMHeight: ::windows::core::PCWSTR = ::windows::w!("WMDM/Height");
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const g_wszWMDMIsProtected: ::windows::core::PCWSTR = ::windows::w!("WMDM/IsProtected");
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const g_wszWMDMIsRepeat: ::windows::core::PCWSTR = ::windows::w!("WMDM/IsRepeat");
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const g_wszWMDMKeyFrameDistance: ::windows::core::PCWSTR = ::windows::w!("WMDM/KeyFrameDistance");
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const g_wszWMDMLastModifiedDate: ::windows::core::PCWSTR = ::windows::w!("WMDM/LastModifiedDate");
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const g_wszWMDMMediaClassSecondaryID: ::windows::core::PCWSTR = ::windows::w!("WMDM/MediaClassSecondaryID");
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const g_wszWMDMMediaCredits: ::windows::core::PCWSTR = ::windows::w!("WMDM/MediaCredits");
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const g_wszWMDMMediaGuid: ::windows::core::PCWSTR = ::windows::w!("WMDM/MediaGuid");
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const g_wszWMDMMediaOriginalBroadcastDateTime: ::windows::core::PCWSTR = ::windows::w!("WMDM/MediaOriginalBroadcastDateTime");
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const g_wszWMDMMediaOriginalChannel: ::windows::core::PCWSTR = ::windows::w!("WMDM/MediaOriginalChannel");
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const g_wszWMDMMediaStationName: ::windows::core::PCWSTR = ::windows::w!("WMDM/MediaStationName");
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const g_wszWMDMMetaGenre: ::windows::core::PCWSTR = ::windows::w!("WMDM/MetaGenre");
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const g_wszWMDMNonConsumable: ::windows::core::PCWSTR = ::windows::w!("WMDM/NonConsumable");
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const g_wszWMDMNumChannels: ::windows::core::PCWSTR = ::windows::w!("WMDM/NumChannels");
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const g_wszWMDMObjectBookmark: ::windows::core::PCWSTR = ::windows::w!("WMDM/ObjectBookmark");
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const g_wszWMDMOwner: ::windows::core::PCWSTR = ::windows::w!("WMDM/Owner");
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const g_wszWMDMParentalRating: ::windows::core::PCWSTR = ::windows::w!("WMDM/ParentalRating");
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const g_wszWMDMPersistentUniqueID: ::windows::core::PCWSTR = ::windows::w!("WMDM/PersistentUniqueID");
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const g_wszWMDMPlayCount: ::windows::core::PCWSTR = ::windows::w!("WMDM/PlayCount");
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const g_wszWMDMProviderCopyright: ::windows::core::PCWSTR = ::windows::w!("WMDM/ProviderCopyright");
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const g_wszWMDMQualitySetting: ::windows::core::PCWSTR = ::windows::w!("WMDM/QualitySetting");
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const g_wszWMDMSampleRate: ::windows::core::PCWSTR = ::windows::w!("WMDM/SampleRate");
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const g_wszWMDMScanType: ::windows::core::PCWSTR = ::windows::w!("WMDM/ScanType");
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const g_wszWMDMSourceURL: ::windows::core::PCWSTR = ::windows::w!("WMDM/SourceURL");
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const g_wszWMDMSubTitle: ::windows::core::PCWSTR = ::windows::w!("WMDM/SubTitle");
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const g_wszWMDMSubTitleDescription: ::windows::core::PCWSTR = ::windows::w!("WMDM/SubTitleDescription");
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const g_wszWMDMSupportedDeviceProperties: ::windows::core::PCWSTR = ::windows::w!("WMDM/SupportedDeviceProperties");
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const g_wszWMDMSyncID: ::windows::core::PCWSTR = ::windows::w!("WMDM/SyncID");
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const g_wszWMDMSyncRelationshipID: ::windows::core::PCWSTR = ::windows::w!("WMDM/SyncRelationshipID");
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const g_wszWMDMSyncTime: ::windows::core::PCWSTR = ::windows::w!("WMDM/SyncTime");
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const g_wszWMDMTimeBookmark: ::windows::core::PCWSTR = ::windows::w!("WMDM/TimeBookmark");
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const g_wszWMDMTimeToLive: ::windows::core::PCWSTR = ::windows::w!("WMDM/TimeToLive");
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const g_wszWMDMTitle: ::windows::core::PCWSTR = ::windows::w!("WMDM/Title");
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const g_wszWMDMTotalBitrate: ::windows::core::PCWSTR = ::windows::w!("WMDM/TotalBitrate");
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const g_wszWMDMTrack: ::windows::core::PCWSTR = ::windows::w!("WMDM/Track");
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const g_wszWMDMTrackMood: ::windows::core::PCWSTR = ::windows::w!("WMDM/TrackMood");
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const g_wszWMDMUserEffectiveRating: ::windows::core::PCWSTR = ::windows::w!("WMDM/UserEffectiveRating");
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const g_wszWMDMUserLastPlayTime: ::windows::core::PCWSTR = ::windows::w!("WMDM/UserLastPlayTime");
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const g_wszWMDMUserRating: ::windows::core::PCWSTR = ::windows::w!("WMDM/UserRating");
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const g_wszWMDMUserRatingOnDevice: ::windows::core::PCWSTR = ::windows::w!("WMDM/UserRatingOnDevice");
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const g_wszWMDMVideoBitrate: ::windows::core::PCWSTR = ::windows::w!("WMDM/VideoBitrate");
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const g_wszWMDMWebmaster: ::windows::core::PCWSTR = ::windows::w!("WMDM/Webmaster");
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const g_wszWMDMWidth: ::windows::core::PCWSTR = ::windows::w!("WMDM/Width");
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const g_wszWMDMYear: ::windows::core::PCWSTR = ::windows::w!("WMDM/Year");
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const g_wszWMDMediaClassPrimaryID: ::windows::core::PCWSTR = ::windows::w!("WMDM/MediaClassPrimaryID");
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const g_wszWPDPassthroughPropertyValues: ::windows::core::PCWSTR = ::windows::w!("WPD/PassthroughPropertyValues");
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct WMDMMessage(pub i32);
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_MSG_DEVICE_ARRIVAL: WMDMMessage = WMDMMessage(0i32);
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_MSG_DEVICE_REMOVAL: WMDMMessage = WMDMMessage(1i32);
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_MSG_MEDIA_ARRIVAL: WMDMMessage = WMDMMessage(2i32);
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_MSG_MEDIA_REMOVAL: WMDMMessage = WMDMMessage(3i32);
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
impl ::windows::core::TypeKind for WMDMMessage {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for WMDMMessage {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WMDMMessage").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct WMDM_ENUM_PROP_VALID_VALUES_FORM(pub i32);
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_ENUM_PROP_VALID_VALUES_ANY: WMDM_ENUM_PROP_VALID_VALUES_FORM = WMDM_ENUM_PROP_VALID_VALUES_FORM(0i32);
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_ENUM_PROP_VALID_VALUES_RANGE: WMDM_ENUM_PROP_VALID_VALUES_FORM = WMDM_ENUM_PROP_VALID_VALUES_FORM(1i32);
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_ENUM_PROP_VALID_VALUES_ENUM: WMDM_ENUM_PROP_VALID_VALUES_FORM = WMDM_ENUM_PROP_VALID_VALUES_FORM(2i32);
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
impl ::windows::core::TypeKind for WMDM_ENUM_PROP_VALID_VALUES_FORM {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for WMDM_ENUM_PROP_VALID_VALUES_FORM {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WMDM_ENUM_PROP_VALID_VALUES_FORM").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct WMDM_FIND_SCOPE(pub i32);
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_FIND_SCOPE_GLOBAL: WMDM_FIND_SCOPE = WMDM_FIND_SCOPE(0i32);
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_FIND_SCOPE_IMMEDIATE_CHILDREN: WMDM_FIND_SCOPE = WMDM_FIND_SCOPE(1i32);
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
impl ::windows::core::TypeKind for WMDM_FIND_SCOPE {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for WMDM_FIND_SCOPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WMDM_FIND_SCOPE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct WMDM_FORMATCODE(pub i32);
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_FORMATCODE_NOTUSED: WMDM_FORMATCODE = WMDM_FORMATCODE(0i32);
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_FORMATCODE_ALLIMAGES: WMDM_FORMATCODE = WMDM_FORMATCODE(-1i32);
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_FORMATCODE_UNDEFINED: WMDM_FORMATCODE = WMDM_FORMATCODE(12288i32);
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_FORMATCODE_ASSOCIATION: WMDM_FORMATCODE = WMDM_FORMATCODE(12289i32);
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_FORMATCODE_SCRIPT: WMDM_FORMATCODE = WMDM_FORMATCODE(12290i32);
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_FORMATCODE_EXECUTABLE: WMDM_FORMATCODE = WMDM_FORMATCODE(12291i32);
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_FORMATCODE_TEXT: WMDM_FORMATCODE = WMDM_FORMATCODE(12292i32);
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_FORMATCODE_HTML: WMDM_FORMATCODE = WMDM_FORMATCODE(12293i32);
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_FORMATCODE_DPOF: WMDM_FORMATCODE = WMDM_FORMATCODE(12294i32);
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_FORMATCODE_AIFF: WMDM_FORMATCODE = WMDM_FORMATCODE(12295i32);
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_FORMATCODE_WAVE: WMDM_FORMATCODE = WMDM_FORMATCODE(12296i32);
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_FORMATCODE_MP3: WMDM_FORMATCODE = WMDM_FORMATCODE(12297i32);
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_FORMATCODE_AVI: WMDM_FORMATCODE = WMDM_FORMATCODE(12298i32);
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_FORMATCODE_MPEG: WMDM_FORMATCODE = WMDM_FORMATCODE(12299i32);
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_FORMATCODE_ASF: WMDM_FORMATCODE = WMDM_FORMATCODE(12300i32);
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_FORMATCODE_RESERVED_FIRST: WMDM_FORMATCODE = WMDM_FORMATCODE(12301i32);
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_FORMATCODE_RESERVED_LAST: WMDM_FORMATCODE = WMDM_FORMATCODE(14335i32);
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_FORMATCODE_IMAGE_UNDEFINED: WMDM_FORMATCODE = WMDM_FORMATCODE(14336i32);
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_FORMATCODE_IMAGE_EXIF: WMDM_FORMATCODE = WMDM_FORMATCODE(14337i32);
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_FORMATCODE_IMAGE_TIFFEP: WMDM_FORMATCODE = WMDM_FORMATCODE(14338i32);
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_FORMATCODE_IMAGE_FLASHPIX: WMDM_FORMATCODE = WMDM_FORMATCODE(14339i32);
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_FORMATCODE_IMAGE_BMP: WMDM_FORMATCODE = WMDM_FORMATCODE(14340i32);
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_FORMATCODE_IMAGE_CIFF: WMDM_FORMATCODE = WMDM_FORMATCODE(14341i32);
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_FORMATCODE_IMAGE_GIF: WMDM_FORMATCODE = WMDM_FORMATCODE(14343i32);
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_FORMATCODE_IMAGE_JFIF: WMDM_FORMATCODE = WMDM_FORMATCODE(14344i32);
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_FORMATCODE_IMAGE_PCD: WMDM_FORMATCODE = WMDM_FORMATCODE(14345i32);
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_FORMATCODE_IMAGE_PICT: WMDM_FORMATCODE = WMDM_FORMATCODE(14346i32);
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_FORMATCODE_IMAGE_PNG: WMDM_FORMATCODE = WMDM_FORMATCODE(14347i32);
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_FORMATCODE_IMAGE_TIFF: WMDM_FORMATCODE = WMDM_FORMATCODE(14349i32);
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_FORMATCODE_IMAGE_TIFFIT: WMDM_FORMATCODE = WMDM_FORMATCODE(14350i32);
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_FORMATCODE_IMAGE_JP2: WMDM_FORMATCODE = WMDM_FORMATCODE(14351i32);
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_FORMATCODE_IMAGE_JPX: WMDM_FORMATCODE = WMDM_FORMATCODE(14352i32);
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_FORMATCODE_IMAGE_RESERVED_FIRST: WMDM_FORMATCODE = WMDM_FORMATCODE(14353i32);
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_FORMATCODE_IMAGE_RESERVED_LAST: WMDM_FORMATCODE = WMDM_FORMATCODE(16383i32);
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_FORMATCODE_UNDEFINEDFIRMWARE: WMDM_FORMATCODE = WMDM_FORMATCODE(47106i32);
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_FORMATCODE_WBMP: WMDM_FORMATCODE = WMDM_FORMATCODE(47107i32);
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_FORMATCODE_JPEGXR: WMDM_FORMATCODE = WMDM_FORMATCODE(47108i32);
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_FORMATCODE_WINDOWSIMAGEFORMAT: WMDM_FORMATCODE = WMDM_FORMATCODE(47233i32);
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_FORMATCODE_UNDEFINEDAUDIO: WMDM_FORMATCODE = WMDM_FORMATCODE(47360i32);
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_FORMATCODE_WMA: WMDM_FORMATCODE = WMDM_FORMATCODE(47361i32);
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_FORMATCODE_OGG: WMDM_FORMATCODE = WMDM_FORMATCODE(47362i32);
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_FORMATCODE_AAC: WMDM_FORMATCODE = WMDM_FORMATCODE(47363i32);
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_FORMATCODE_AUDIBLE: WMDM_FORMATCODE = WMDM_FORMATCODE(47364i32);
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_FORMATCODE_FLAC: WMDM_FORMATCODE = WMDM_FORMATCODE(47366i32);
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_FORMATCODE_QCELP: WMDM_FORMATCODE = WMDM_FORMATCODE(47367i32);
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_FORMATCODE_AMR: WMDM_FORMATCODE = WMDM_FORMATCODE(47368i32);
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_FORMATCODE_UNDEFINEDVIDEO: WMDM_FORMATCODE = WMDM_FORMATCODE(47488i32);
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_FORMATCODE_WMV: WMDM_FORMATCODE = WMDM_FORMATCODE(47489i32);
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_FORMATCODE_MP4: WMDM_FORMATCODE = WMDM_FORMATCODE(47490i32);
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_FORMATCODE_MP2: WMDM_FORMATCODE = WMDM_FORMATCODE(47491i32);
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_FORMATCODE_3GP: WMDM_FORMATCODE = WMDM_FORMATCODE(47492i32);
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_FORMATCODE_3G2: WMDM_FORMATCODE = WMDM_FORMATCODE(47493i32);
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_FORMATCODE_AVCHD: WMDM_FORMATCODE = WMDM_FORMATCODE(47494i32);
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_FORMATCODE_ATSCTS: WMDM_FORMATCODE = WMDM_FORMATCODE(47495i32);
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_FORMATCODE_DVBTS: WMDM_FORMATCODE = WMDM_FORMATCODE(47496i32);
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_FORMATCODE_MKV: WMDM_FORMATCODE = WMDM_FORMATCODE(47497i32);
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_FORMATCODE_MKA: WMDM_FORMATCODE = WMDM_FORMATCODE(47498i32);
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_FORMATCODE_MK3D: WMDM_FORMATCODE = WMDM_FORMATCODE(47499i32);
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_FORMATCODE_UNDEFINEDCOLLECTION: WMDM_FORMATCODE = WMDM_FORMATCODE(47616i32);
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_FORMATCODE_ABSTRACTMULTIMEDIAALBUM: WMDM_FORMATCODE = WMDM_FORMATCODE(47617i32);
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_FORMATCODE_ABSTRACTIMAGEALBUM: WMDM_FORMATCODE = WMDM_FORMATCODE(47618i32);
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_FORMATCODE_ABSTRACTAUDIOALBUM: WMDM_FORMATCODE = WMDM_FORMATCODE(47619i32);
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_FORMATCODE_ABSTRACTVIDEOALBUM: WMDM_FORMATCODE = WMDM_FORMATCODE(47620i32);
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_FORMATCODE_ABSTRACTAUDIOVIDEOPLAYLIST: WMDM_FORMATCODE = WMDM_FORMATCODE(47621i32);
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_FORMATCODE_ABSTRACTCONTACTGROUP: WMDM_FORMATCODE = WMDM_FORMATCODE(47622i32);
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_FORMATCODE_ABSTRACTMESSAGEFOLDER: WMDM_FORMATCODE = WMDM_FORMATCODE(47623i32);
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_FORMATCODE_ABSTRACTCHAPTEREDPRODUCTION: WMDM_FORMATCODE = WMDM_FORMATCODE(47624i32);
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_FORMATCODE_MEDIA_CAST: WMDM_FORMATCODE = WMDM_FORMATCODE(47627i32);
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_FORMATCODE_WPLPLAYLIST: WMDM_FORMATCODE = WMDM_FORMATCODE(47632i32);
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_FORMATCODE_M3UPLAYLIST: WMDM_FORMATCODE = WMDM_FORMATCODE(47633i32);
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_FORMATCODE_MPLPLAYLIST: WMDM_FORMATCODE = WMDM_FORMATCODE(47634i32);
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_FORMATCODE_ASXPLAYLIST: WMDM_FORMATCODE = WMDM_FORMATCODE(47635i32);
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_FORMATCODE_PLSPLAYLIST: WMDM_FORMATCODE = WMDM_FORMATCODE(47636i32);
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_FORMATCODE_UNDEFINEDDOCUMENT: WMDM_FORMATCODE = WMDM_FORMATCODE(47744i32);
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_FORMATCODE_ABSTRACTDOCUMENT: WMDM_FORMATCODE = WMDM_FORMATCODE(47745i32);
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_FORMATCODE_XMLDOCUMENT: WMDM_FORMATCODE = WMDM_FORMATCODE(47746i32);
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_FORMATCODE_MICROSOFTWORDDOCUMENT: WMDM_FORMATCODE = WMDM_FORMATCODE(47747i32);
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_FORMATCODE_MHTCOMPILEDHTMLDOCUMENT: WMDM_FORMATCODE = WMDM_FORMATCODE(47748i32);
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_FORMATCODE_MICROSOFTEXCELSPREADSHEET: WMDM_FORMATCODE = WMDM_FORMATCODE(47749i32);
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_FORMATCODE_MICROSOFTPOWERPOINTDOCUMENT: WMDM_FORMATCODE = WMDM_FORMATCODE(47750i32);
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_FORMATCODE_UNDEFINEDMESSAGE: WMDM_FORMATCODE = WMDM_FORMATCODE(47872i32);
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_FORMATCODE_ABSTRACTMESSAGE: WMDM_FORMATCODE = WMDM_FORMATCODE(47873i32);
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_FORMATCODE_UNDEFINEDCONTACT: WMDM_FORMATCODE = WMDM_FORMATCODE(48000i32);
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_FORMATCODE_ABSTRACTCONTACT: WMDM_FORMATCODE = WMDM_FORMATCODE(48001i32);
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_FORMATCODE_VCARD2: WMDM_FORMATCODE = WMDM_FORMATCODE(48002i32);
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_FORMATCODE_VCARD3: WMDM_FORMATCODE = WMDM_FORMATCODE(48003i32);
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_FORMATCODE_UNDEFINEDCALENDARITEM: WMDM_FORMATCODE = WMDM_FORMATCODE(48640i32);
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_FORMATCODE_ABSTRACTCALENDARITEM: WMDM_FORMATCODE = WMDM_FORMATCODE(48641i32);
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_FORMATCODE_VCALENDAR1: WMDM_FORMATCODE = WMDM_FORMATCODE(48642i32);
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_FORMATCODE_VCALENDAR2: WMDM_FORMATCODE = WMDM_FORMATCODE(48643i32);
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_FORMATCODE_UNDEFINEDWINDOWSEXECUTABLE: WMDM_FORMATCODE = WMDM_FORMATCODE(48768i32);
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_FORMATCODE_M4A: WMDM_FORMATCODE = WMDM_FORMATCODE(1297101889i32);
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_FORMATCODE_3GPA: WMDM_FORMATCODE = WMDM_FORMATCODE(860311617i32);
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_FORMATCODE_3G2A: WMDM_FORMATCODE = WMDM_FORMATCODE(860303937i32);
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_FORMATCODE_SECTION: WMDM_FORMATCODE = WMDM_FORMATCODE(48770i32);
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
impl ::windows::core::TypeKind for WMDM_FORMATCODE {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for WMDM_FORMATCODE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WMDM_FORMATCODE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct WMDM_SESSION_TYPE(pub i32);
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_SESSION_NONE: WMDM_SESSION_TYPE = WMDM_SESSION_TYPE(0i32);
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_SESSION_TRANSFER_TO_DEVICE: WMDM_SESSION_TYPE = WMDM_SESSION_TYPE(1i32);
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_SESSION_TRANSFER_FROM_DEVICE: WMDM_SESSION_TYPE = WMDM_SESSION_TYPE(16i32);
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_SESSION_DELETE: WMDM_SESSION_TYPE = WMDM_SESSION_TYPE(256i32);
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_SESSION_CUSTOM: WMDM_SESSION_TYPE = WMDM_SESSION_TYPE(4096i32);
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
impl ::windows::core::TypeKind for WMDM_SESSION_TYPE {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for WMDM_SESSION_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WMDM_SESSION_TYPE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct WMDM_STORAGE_ENUM_MODE(pub i32);
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const ENUM_MODE_RAW: WMDM_STORAGE_ENUM_MODE = WMDM_STORAGE_ENUM_MODE(0i32);
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const ENUM_MODE_USE_DEVICE_PREF: WMDM_STORAGE_ENUM_MODE = WMDM_STORAGE_ENUM_MODE(1i32);
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const ENUM_MODE_METADATA_VIEWS: WMDM_STORAGE_ENUM_MODE = WMDM_STORAGE_ENUM_MODE(2i32);
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
impl ::windows::core::TypeKind for WMDM_STORAGE_ENUM_MODE {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for WMDM_STORAGE_ENUM_MODE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WMDM_STORAGE_ENUM_MODE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct WMDM_TAG_DATATYPE(pub i32);
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_TYPE_DWORD: WMDM_TAG_DATATYPE = WMDM_TAG_DATATYPE(0i32);
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_TYPE_STRING: WMDM_TAG_DATATYPE = WMDM_TAG_DATATYPE(1i32);
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_TYPE_BINARY: WMDM_TAG_DATATYPE = WMDM_TAG_DATATYPE(2i32);
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_TYPE_BOOL: WMDM_TAG_DATATYPE = WMDM_TAG_DATATYPE(3i32);
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_TYPE_QWORD: WMDM_TAG_DATATYPE = WMDM_TAG_DATATYPE(4i32);
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_TYPE_WORD: WMDM_TAG_DATATYPE = WMDM_TAG_DATATYPE(5i32);
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_TYPE_GUID: WMDM_TAG_DATATYPE = WMDM_TAG_DATATYPE(6i32);
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_TYPE_DATE: WMDM_TAG_DATATYPE = WMDM_TAG_DATATYPE(7i32);
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
impl ::windows::core::TypeKind for WMDM_TAG_DATATYPE {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for WMDM_TAG_DATATYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WMDM_TAG_DATATYPE").field(&self.0).finish()
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`, `\"Win32_Foundation\"`*"]
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
impl ::windows::core::TypeKind for MACINFO {
    type TypeKind = ::windows::core::CopyType;
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
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
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
impl ::windows::core::TypeKind for MTP_COMMAND_DATA_IN {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::default::Default for MTP_COMMAND_DATA_IN {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
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
impl ::windows::core::TypeKind for MTP_COMMAND_DATA_OUT {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::default::Default for MTP_COMMAND_DATA_OUT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub struct OPAQUECOMMAND {
    pub guidCommand: ::windows::core::GUID,
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
impl ::windows::core::TypeKind for OPAQUECOMMAND {
    type TypeKind = ::windows::core::CopyType;
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
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
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
impl ::windows::core::TypeKind for WMDMDATETIME {
    type TypeKind = ::windows::core::CopyType;
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
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
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
impl ::windows::core::TypeKind for WMDMDetermineMaxPropStringLen {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::default::Default for WMDMDetermineMaxPropStringLen {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
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
impl ::windows::core::TypeKind for WMDMID {
    type TypeKind = ::windows::core::CopyType;
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
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub struct WMDMMetadataView {
    pub pwszViewName: ::windows::core::PWSTR,
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
impl ::windows::core::TypeKind for WMDMMetadataView {
    type TypeKind = ::windows::core::CopyType;
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
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
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
impl ::windows::core::TypeKind for WMDMRIGHTS {
    type TypeKind = ::windows::core::CopyType;
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
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com_StructuredStorage\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
pub struct WMDM_FORMAT_CAPABILITY {
    pub nPropConfig: u32,
    pub pConfigs: *mut WMDM_PROP_CONFIG,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
impl ::core::marker::Copy for WMDM_FORMAT_CAPABILITY {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
impl ::core::clone::Clone for WMDM_FORMAT_CAPABILITY {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
impl ::core::fmt::Debug for WMDM_FORMAT_CAPABILITY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WMDM_FORMAT_CAPABILITY").field("nPropConfig", &self.nPropConfig).field("pConfigs", &self.pConfigs).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
impl ::windows::core::TypeKind for WMDM_FORMAT_CAPABILITY {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
impl ::core::cmp::PartialEq for WMDM_FORMAT_CAPABILITY {
    fn eq(&self, other: &Self) -> bool {
        self.nPropConfig == other.nPropConfig && self.pConfigs == other.pConfigs
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
impl ::core::cmp::Eq for WMDM_FORMAT_CAPABILITY {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
impl ::core::default::Default for WMDM_FORMAT_CAPABILITY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com_StructuredStorage\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
pub struct WMDM_PROP_CONFIG {
    pub nPreference: u32,
    pub nPropDesc: u32,
    pub pPropDesc: *mut WMDM_PROP_DESC,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
impl ::core::marker::Copy for WMDM_PROP_CONFIG {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
impl ::core::clone::Clone for WMDM_PROP_CONFIG {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
impl ::core::fmt::Debug for WMDM_PROP_CONFIG {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WMDM_PROP_CONFIG").field("nPreference", &self.nPreference).field("nPropDesc", &self.nPropDesc).field("pPropDesc", &self.pPropDesc).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
impl ::windows::core::TypeKind for WMDM_PROP_CONFIG {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
impl ::core::cmp::PartialEq for WMDM_PROP_CONFIG {
    fn eq(&self, other: &Self) -> bool {
        self.nPreference == other.nPreference && self.nPropDesc == other.nPropDesc && self.pPropDesc == other.pPropDesc
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
impl ::core::cmp::Eq for WMDM_PROP_CONFIG {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
impl ::core::default::Default for WMDM_PROP_CONFIG {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com_StructuredStorage\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
pub struct WMDM_PROP_DESC {
    pub pwszPropName: ::windows::core::PWSTR,
    pub ValidValuesForm: WMDM_ENUM_PROP_VALID_VALUES_FORM,
    pub ValidValues: WMDM_PROP_DESC_0,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
impl ::core::clone::Clone for WMDM_PROP_DESC {
    fn clone(&self) -> Self {
        unsafe { ::core::mem::transmute_copy(self) }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
impl ::windows::core::TypeKind for WMDM_PROP_DESC {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
impl ::core::default::Default for WMDM_PROP_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com_StructuredStorage\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
pub union WMDM_PROP_DESC_0 {
    pub ValidValuesRange: ::std::mem::ManuallyDrop<WMDM_PROP_VALUES_RANGE>,
    pub EnumeratedValidValues: WMDM_PROP_VALUES_ENUM,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
impl ::core::clone::Clone for WMDM_PROP_DESC_0 {
    fn clone(&self) -> Self {
        unsafe { ::core::mem::transmute_copy(self) }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
impl ::windows::core::TypeKind for WMDM_PROP_DESC_0 {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
impl ::core::default::Default for WMDM_PROP_DESC_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com_StructuredStorage\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
pub struct WMDM_PROP_VALUES_ENUM {
    pub cEnumValues: u32,
    pub pValues: *mut super::super::System::Com::StructuredStorage::PROPVARIANT,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
impl ::core::marker::Copy for WMDM_PROP_VALUES_ENUM {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
impl ::core::clone::Clone for WMDM_PROP_VALUES_ENUM {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
impl ::core::fmt::Debug for WMDM_PROP_VALUES_ENUM {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WMDM_PROP_VALUES_ENUM").field("cEnumValues", &self.cEnumValues).field("pValues", &self.pValues).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
impl ::windows::core::TypeKind for WMDM_PROP_VALUES_ENUM {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
impl ::core::cmp::PartialEq for WMDM_PROP_VALUES_ENUM {
    fn eq(&self, other: &Self) -> bool {
        self.cEnumValues == other.cEnumValues && self.pValues == other.pValues
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
impl ::core::cmp::Eq for WMDM_PROP_VALUES_ENUM {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
impl ::core::default::Default for WMDM_PROP_VALUES_ENUM {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com_StructuredStorage\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
pub struct WMDM_PROP_VALUES_RANGE {
    pub rangeMin: super::super::System::Com::StructuredStorage::PROPVARIANT,
    pub rangeMax: super::super::System::Com::StructuredStorage::PROPVARIANT,
    pub rangeStep: super::super::System::Com::StructuredStorage::PROPVARIANT,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
impl ::core::clone::Clone for WMDM_PROP_VALUES_RANGE {
    fn clone(&self) -> Self {
        unsafe { ::core::mem::transmute_copy(self) }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
impl ::windows::core::TypeKind for WMDM_PROP_VALUES_RANGE {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
impl ::core::default::Default for WMDM_PROP_VALUES_RANGE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub struct WMFILECAPABILITIES {
    pub pwszMimeType: ::windows::core::PWSTR,
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
impl ::windows::core::TypeKind for WMFILECAPABILITIES {
    type TypeKind = ::windows::core::CopyType;
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
