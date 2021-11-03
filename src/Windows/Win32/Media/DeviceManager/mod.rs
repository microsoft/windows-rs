#![allow(unused_variables, non_upper_case_globals, non_snake_case, unused_unsafe, non_camel_case_types, dead_code, clippy::all)]
#[doc = "*Required features: `Win32_Media_DeviceManager`*"]
pub const ALLOW_OUTOFBAND_NOTIFICATION: u32 = 2u32;
#[doc = "*Required features: `Win32_Media_DeviceManager`*"]
pub const DO_NOT_VIRTUALIZE_STORAGES_AS_DEVICES: u32 = 1u32;
pub const EVENT_WMDM_CONTENT_TRANSFER: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(865901556, 48382, 20184, [148, 223, 234, 248, 194, 106, 182, 27]);
#[doc = "*Required features: `Win32_Media_DeviceManager`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IComponentAuthenticate(pub ::windows::runtime::IUnknown);
impl IComponentAuthenticate {
    #[doc = "*Required features: `Win32_Media_DeviceManager`*"]
    pub unsafe fn SACAuth(&self, dwprotocolid: u32, dwpass: u32, pbdatain: *const u8, dwdatainlen: u32, ppbdataout: *mut *mut u8, pdwdataoutlen: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), ::std::mem::transmute(dwprotocolid), ::std::mem::transmute(dwpass), ::std::mem::transmute(pbdatain), ::std::mem::transmute(dwdatainlen), ::std::mem::transmute(ppbdataout), ::std::mem::transmute(pdwdataoutlen)).ok()
    }
    #[doc = "*Required features: `Win32_Media_DeviceManager`*"]
    pub unsafe fn SACGetProtocols(&self, ppdwprotocols: *mut *mut u32, pdwprotocolcount: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), ::std::mem::transmute(ppdwprotocols), ::std::mem::transmute(pdwprotocolcount)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IComponentAuthenticate {
    type Vtable = IComponentAuthenticate_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2844302336, 27947, 4563, [132, 150, 0, 192, 79, 121, 219, 192]);
}
impl ::std::convert::From<IComponentAuthenticate> for ::windows::runtime::IUnknown {
    fn from(value: IComponentAuthenticate) -> Self {
        value.0
    }
}
impl ::std::convert::From<&IComponentAuthenticate> for ::windows::runtime::IUnknown {
    fn from(value: &IComponentAuthenticate) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IComponentAuthenticate {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IComponentAuthenticate {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IComponentAuthenticate_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwprotocolid: u32, dwpass: u32, pbdatain: *const u8, dwdatainlen: u32, ppbdataout: *mut *mut u8, pdwdataoutlen: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppdwprotocols: *mut *mut u32, pdwprotocolcount: *mut u32) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_Media_DeviceManager`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IMDSPDevice(pub ::windows::runtime::IUnknown);
impl IMDSPDevice {
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_DeviceManager`, `Win32_Foundation`*"]
    pub unsafe fn GetName(&self, pwszname: super::super::Foundation::PWSTR, nmaxchars: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), ::std::mem::transmute(pwszname), ::std::mem::transmute(nmaxchars)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_DeviceManager`, `Win32_Foundation`*"]
    pub unsafe fn GetManufacturer(&self, pwszname: super::super::Foundation::PWSTR, nmaxchars: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), ::std::mem::transmute(pwszname), ::std::mem::transmute(nmaxchars)).ok()
    }
    #[doc = "*Required features: `Win32_Media_DeviceManager`*"]
    pub unsafe fn GetVersion(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `Win32_Media_DeviceManager`*"]
    pub unsafe fn GetType(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).6)(::std::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `Win32_Media_DeviceManager`*"]
    pub unsafe fn GetSerialNumber(&self, pserialnumber: *mut WMDMID, abmac: *mut u8) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self), ::std::mem::transmute(pserialnumber), ::std::mem::transmute(abmac)).ok()
    }
    #[doc = "*Required features: `Win32_Media_DeviceManager`*"]
    pub unsafe fn GetPowerSource(&self, pdwpowersource: *mut u32, pdwpercentremaining: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).8)(::std::mem::transmute_copy(self), ::std::mem::transmute(pdwpowersource), ::std::mem::transmute(pdwpercentremaining)).ok()
    }
    #[doc = "*Required features: `Win32_Media_DeviceManager`*"]
    pub unsafe fn GetStatus(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).9)(::std::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `Win32_Media_DeviceManager`*"]
    pub unsafe fn GetDeviceIcon(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).10)(::std::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `Win32_Media_DeviceManager`*"]
    pub unsafe fn EnumStorage(&self) -> ::windows::runtime::Result<IMDSPEnumStorage> {
        let mut result__: <IMDSPEnumStorage as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).11)(::std::mem::transmute_copy(self), &mut result__).from_abi::<IMDSPEnumStorage>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_DeviceManager`, `Win32_Foundation`*"]
    pub unsafe fn GetFormatSupport(&self, pformatex: *mut *mut _WAVEFORMATEX, pnformatcount: *mut u32, pppwszmimetype: *mut *mut super::super::Foundation::PWSTR, pnmimetypecount: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).12)(::std::mem::transmute_copy(self), ::std::mem::transmute(pformatex), ::std::mem::transmute(pnformatcount), ::std::mem::transmute(pppwszmimetype), ::std::mem::transmute(pnmimetypecount)).ok()
    }
    #[doc = "*Required features: `Win32_Media_DeviceManager`*"]
    pub unsafe fn SendOpaqueCommand(&self, pcommand: *mut OPAQUECOMMAND) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).13)(::std::mem::transmute_copy(self), ::std::mem::transmute(pcommand)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IMDSPDevice {
    type Vtable = IMDSPDevice_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(499857938, 13293, 4563, [132, 112, 0, 192, 79, 121, 219, 192]);
}
impl ::std::convert::From<IMDSPDevice> for ::windows::runtime::IUnknown {
    fn from(value: IMDSPDevice) -> Self {
        value.0
    }
}
impl ::std::convert::From<&IMDSPDevice> for ::windows::runtime::IUnknown {
    fn from(value: &IMDSPDevice) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IMDSPDevice {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IMDSPDevice {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMDSPDevice_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pwszname: super::super::Foundation::PWSTR, nmaxchars: u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pwszname: super::super::Foundation::PWSTR, nmaxchars: u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pdwversion: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pdwtype: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pserialnumber: *mut WMDMID, abmac: *mut u8) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pdwpowersource: *mut u32, pdwpercentremaining: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pdwstatus: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, hicon: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppenumstorage: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pformatex: *mut *mut _WAVEFORMATEX, pnformatcount: *mut u32, pppwszmimetype: *mut *mut super::super::Foundation::PWSTR, pnmimetypecount: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pcommand: *mut OPAQUECOMMAND) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_Media_DeviceManager`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IMDSPDevice2(pub ::windows::runtime::IUnknown);
impl IMDSPDevice2 {
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_DeviceManager`, `Win32_Foundation`*"]
    pub unsafe fn GetName(&self, pwszname: super::super::Foundation::PWSTR, nmaxchars: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), ::std::mem::transmute(pwszname), ::std::mem::transmute(nmaxchars)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_DeviceManager`, `Win32_Foundation`*"]
    pub unsafe fn GetManufacturer(&self, pwszname: super::super::Foundation::PWSTR, nmaxchars: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), ::std::mem::transmute(pwszname), ::std::mem::transmute(nmaxchars)).ok()
    }
    #[doc = "*Required features: `Win32_Media_DeviceManager`*"]
    pub unsafe fn GetVersion(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `Win32_Media_DeviceManager`*"]
    pub unsafe fn GetType(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).6)(::std::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `Win32_Media_DeviceManager`*"]
    pub unsafe fn GetSerialNumber(&self, pserialnumber: *mut WMDMID, abmac: *mut u8) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self), ::std::mem::transmute(pserialnumber), ::std::mem::transmute(abmac)).ok()
    }
    #[doc = "*Required features: `Win32_Media_DeviceManager`*"]
    pub unsafe fn GetPowerSource(&self, pdwpowersource: *mut u32, pdwpercentremaining: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).8)(::std::mem::transmute_copy(self), ::std::mem::transmute(pdwpowersource), ::std::mem::transmute(pdwpercentremaining)).ok()
    }
    #[doc = "*Required features: `Win32_Media_DeviceManager`*"]
    pub unsafe fn GetStatus(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).9)(::std::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `Win32_Media_DeviceManager`*"]
    pub unsafe fn GetDeviceIcon(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).10)(::std::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `Win32_Media_DeviceManager`*"]
    pub unsafe fn EnumStorage(&self) -> ::windows::runtime::Result<IMDSPEnumStorage> {
        let mut result__: <IMDSPEnumStorage as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).11)(::std::mem::transmute_copy(self), &mut result__).from_abi::<IMDSPEnumStorage>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_DeviceManager`, `Win32_Foundation`*"]
    pub unsafe fn GetFormatSupport(&self, pformatex: *mut *mut _WAVEFORMATEX, pnformatcount: *mut u32, pppwszmimetype: *mut *mut super::super::Foundation::PWSTR, pnmimetypecount: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).12)(::std::mem::transmute_copy(self), ::std::mem::transmute(pformatex), ::std::mem::transmute(pnformatcount), ::std::mem::transmute(pppwszmimetype), ::std::mem::transmute(pnmimetypecount)).ok()
    }
    #[doc = "*Required features: `Win32_Media_DeviceManager`*"]
    pub unsafe fn SendOpaqueCommand(&self, pcommand: *mut OPAQUECOMMAND) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).13)(::std::mem::transmute_copy(self), ::std::mem::transmute(pcommand)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_DeviceManager`, `Win32_Foundation`*"]
    pub unsafe fn GetStorage<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, pszstoragename: Param0) -> ::windows::runtime::Result<IMDSPStorage> {
        let mut result__: <IMDSPStorage as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).14)(::std::mem::transmute_copy(self), pszstoragename.into_param().abi(), &mut result__).from_abi::<IMDSPStorage>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_DeviceManager`, `Win32_Foundation`*"]
    pub unsafe fn GetFormatSupport2(&self, dwflags: u32, ppaudioformatex: *mut *mut _WAVEFORMATEX, pnaudioformatcount: *mut u32, ppvideoformatex: *mut *mut _VIDEOINFOHEADER, pnvideoformatcount: *mut u32, ppfiletype: *mut *mut WMFILECAPABILITIES, pnfiletypecount: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).15)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(dwflags),
            ::std::mem::transmute(ppaudioformatex),
            ::std::mem::transmute(pnaudioformatcount),
            ::std::mem::transmute(ppvideoformatex),
            ::std::mem::transmute(pnvideoformatcount),
            ::std::mem::transmute(ppfiletype),
            ::std::mem::transmute(pnfiletypecount),
        )
        .ok()
    }
    #[cfg(feature = "Win32_System_Ole")]
    #[doc = "*Required features: `Win32_Media_DeviceManager`, `Win32_System_Ole`*"]
    pub unsafe fn GetSpecifyPropertyPages(&self, ppspecifyproppages: *mut ::std::option::Option<super::super::System::Ole::ISpecifyPropertyPages>, pppunknowns: *mut *mut ::std::option::Option<::windows::runtime::IUnknown>, pcunks: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).16)(::std::mem::transmute_copy(self), ::std::mem::transmute(ppspecifyproppages), ::std::mem::transmute(pppunknowns), ::std::mem::transmute(pcunks)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_DeviceManager`, `Win32_Foundation`*"]
    pub unsafe fn GetCanonicalName(&self, pwszpnpname: super::super::Foundation::PWSTR, nmaxchars: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).17)(::std::mem::transmute_copy(self), ::std::mem::transmute(pwszpnpname), ::std::mem::transmute(nmaxchars)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IMDSPDevice2 {
    type Vtable = IMDSPDevice2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1108154029, 51581, 19968, [130, 170, 0, 233, 244, 51, 93, 221]);
}
impl ::std::convert::From<IMDSPDevice2> for ::windows::runtime::IUnknown {
    fn from(value: IMDSPDevice2) -> Self {
        value.0
    }
}
impl ::std::convert::From<&IMDSPDevice2> for ::windows::runtime::IUnknown {
    fn from(value: &IMDSPDevice2) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IMDSPDevice2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IMDSPDevice2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::std::convert::From<IMDSPDevice2> for IMDSPDevice {
    fn from(value: IMDSPDevice2) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IMDSPDevice2> for IMDSPDevice {
    fn from(value: &IMDSPDevice2) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IMDSPDevice> for IMDSPDevice2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IMDSPDevice> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IMDSPDevice>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IMDSPDevice> for &IMDSPDevice2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IMDSPDevice> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IMDSPDevice>::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMDSPDevice2_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pwszname: super::super::Foundation::PWSTR, nmaxchars: u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pwszname: super::super::Foundation::PWSTR, nmaxchars: u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pdwversion: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pdwtype: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pserialnumber: *mut WMDMID, abmac: *mut u8) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pdwpowersource: *mut u32, pdwpercentremaining: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pdwstatus: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, hicon: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppenumstorage: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pformatex: *mut *mut _WAVEFORMATEX, pnformatcount: *mut u32, pppwszmimetype: *mut *mut super::super::Foundation::PWSTR, pnmimetypecount: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pcommand: *mut OPAQUECOMMAND) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pszstoragename: super::super::Foundation::PWSTR, ppstorage: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwflags: u32, ppaudioformatex: *mut *mut _WAVEFORMATEX, pnaudioformatcount: *mut u32, ppvideoformatex: *mut *mut _VIDEOINFOHEADER, pnvideoformatcount: *mut u32, ppfiletype: *mut *mut WMFILECAPABILITIES, pnfiletypecount: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_System_Ole")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppspecifyproppages: *mut ::windows::runtime::RawPtr, pppunknowns: *mut *mut ::windows::runtime::RawPtr, pcunks: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Ole"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pwszpnpname: super::super::Foundation::PWSTR, nmaxchars: u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[doc = "*Required features: `Win32_Media_DeviceManager`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IMDSPDevice3(pub ::windows::runtime::IUnknown);
impl IMDSPDevice3 {
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_DeviceManager`, `Win32_Foundation`*"]
    pub unsafe fn GetName(&self, pwszname: super::super::Foundation::PWSTR, nmaxchars: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), ::std::mem::transmute(pwszname), ::std::mem::transmute(nmaxchars)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_DeviceManager`, `Win32_Foundation`*"]
    pub unsafe fn GetManufacturer(&self, pwszname: super::super::Foundation::PWSTR, nmaxchars: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), ::std::mem::transmute(pwszname), ::std::mem::transmute(nmaxchars)).ok()
    }
    #[doc = "*Required features: `Win32_Media_DeviceManager`*"]
    pub unsafe fn GetVersion(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `Win32_Media_DeviceManager`*"]
    pub unsafe fn GetType(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).6)(::std::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `Win32_Media_DeviceManager`*"]
    pub unsafe fn GetSerialNumber(&self, pserialnumber: *mut WMDMID, abmac: *mut u8) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self), ::std::mem::transmute(pserialnumber), ::std::mem::transmute(abmac)).ok()
    }
    #[doc = "*Required features: `Win32_Media_DeviceManager`*"]
    pub unsafe fn GetPowerSource(&self, pdwpowersource: *mut u32, pdwpercentremaining: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).8)(::std::mem::transmute_copy(self), ::std::mem::transmute(pdwpowersource), ::std::mem::transmute(pdwpercentremaining)).ok()
    }
    #[doc = "*Required features: `Win32_Media_DeviceManager`*"]
    pub unsafe fn GetStatus(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).9)(::std::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `Win32_Media_DeviceManager`*"]
    pub unsafe fn GetDeviceIcon(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).10)(::std::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `Win32_Media_DeviceManager`*"]
    pub unsafe fn EnumStorage(&self) -> ::windows::runtime::Result<IMDSPEnumStorage> {
        let mut result__: <IMDSPEnumStorage as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).11)(::std::mem::transmute_copy(self), &mut result__).from_abi::<IMDSPEnumStorage>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_DeviceManager`, `Win32_Foundation`*"]
    pub unsafe fn GetFormatSupport(&self, pformatex: *mut *mut _WAVEFORMATEX, pnformatcount: *mut u32, pppwszmimetype: *mut *mut super::super::Foundation::PWSTR, pnmimetypecount: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).12)(::std::mem::transmute_copy(self), ::std::mem::transmute(pformatex), ::std::mem::transmute(pnformatcount), ::std::mem::transmute(pppwszmimetype), ::std::mem::transmute(pnmimetypecount)).ok()
    }
    #[doc = "*Required features: `Win32_Media_DeviceManager`*"]
    pub unsafe fn SendOpaqueCommand(&self, pcommand: *mut OPAQUECOMMAND) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).13)(::std::mem::transmute_copy(self), ::std::mem::transmute(pcommand)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_DeviceManager`, `Win32_Foundation`*"]
    pub unsafe fn GetStorage<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, pszstoragename: Param0) -> ::windows::runtime::Result<IMDSPStorage> {
        let mut result__: <IMDSPStorage as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).14)(::std::mem::transmute_copy(self), pszstoragename.into_param().abi(), &mut result__).from_abi::<IMDSPStorage>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_DeviceManager`, `Win32_Foundation`*"]
    pub unsafe fn GetFormatSupport2(&self, dwflags: u32, ppaudioformatex: *mut *mut _WAVEFORMATEX, pnaudioformatcount: *mut u32, ppvideoformatex: *mut *mut _VIDEOINFOHEADER, pnvideoformatcount: *mut u32, ppfiletype: *mut *mut WMFILECAPABILITIES, pnfiletypecount: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).15)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(dwflags),
            ::std::mem::transmute(ppaudioformatex),
            ::std::mem::transmute(pnaudioformatcount),
            ::std::mem::transmute(ppvideoformatex),
            ::std::mem::transmute(pnvideoformatcount),
            ::std::mem::transmute(ppfiletype),
            ::std::mem::transmute(pnfiletypecount),
        )
        .ok()
    }
    #[cfg(feature = "Win32_System_Ole")]
    #[doc = "*Required features: `Win32_Media_DeviceManager`, `Win32_System_Ole`*"]
    pub unsafe fn GetSpecifyPropertyPages(&self, ppspecifyproppages: *mut ::std::option::Option<super::super::System::Ole::ISpecifyPropertyPages>, pppunknowns: *mut *mut ::std::option::Option<::windows::runtime::IUnknown>, pcunks: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).16)(::std::mem::transmute_copy(self), ::std::mem::transmute(ppspecifyproppages), ::std::mem::transmute(pppunknowns), ::std::mem::transmute(pcunks)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_DeviceManager`, `Win32_Foundation`*"]
    pub unsafe fn GetCanonicalName(&self, pwszpnpname: super::super::Foundation::PWSTR, nmaxchars: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).17)(::std::mem::transmute_copy(self), ::std::mem::transmute(pwszpnpname), ::std::mem::transmute(nmaxchars)).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Ole_Automation"))]
    #[doc = "*Required features: `Win32_Media_DeviceManager`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Com_StructuredStorage`, `Win32_System_Ole_Automation`*"]
    pub unsafe fn GetProperty<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, pwszpropname: Param0) -> ::windows::runtime::Result<super::super::System::Com::StructuredStorage::PROPVARIANT> {
        let mut result__: <super::super::System::Com::StructuredStorage::PROPVARIANT as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).18)(::std::mem::transmute_copy(self), pwszpropname.into_param().abi(), &mut result__).from_abi::<super::super::System::Com::StructuredStorage::PROPVARIANT>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Ole_Automation"))]
    #[doc = "*Required features: `Win32_Media_DeviceManager`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Com_StructuredStorage`, `Win32_System_Ole_Automation`*"]
    pub unsafe fn SetProperty<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, pwszpropname: Param0, pvalue: *const super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).19)(::std::mem::transmute_copy(self), pwszpropname.into_param().abi(), ::std::mem::transmute(pvalue)).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Ole_Automation"))]
    #[doc = "*Required features: `Win32_Media_DeviceManager`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Com_StructuredStorage`, `Win32_System_Ole_Automation`*"]
    pub unsafe fn GetFormatCapability(&self, format: WMDM_FORMATCODE) -> ::windows::runtime::Result<WMDM_FORMAT_CAPABILITY> {
        let mut result__: <WMDM_FORMAT_CAPABILITY as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).20)(::std::mem::transmute_copy(self), ::std::mem::transmute(format), &mut result__).from_abi::<WMDM_FORMAT_CAPABILITY>(result__)
    }
    #[doc = "*Required features: `Win32_Media_DeviceManager`*"]
    pub unsafe fn DeviceIoControl(&self, dwiocontrolcode: u32, lpinbuffer: *const u8, ninbuffersize: u32, lpoutbuffer: *mut u8, pnoutbuffersize: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).21)(::std::mem::transmute_copy(self), ::std::mem::transmute(dwiocontrolcode), ::std::mem::transmute(lpinbuffer), ::std::mem::transmute(ninbuffersize), ::std::mem::transmute(lpoutbuffer), ::std::mem::transmute(pnoutbuffersize)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_DeviceManager`, `Win32_Foundation`*"]
    pub unsafe fn FindStorage<'a, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, findscope: WMDM_FIND_SCOPE, pwszuniqueid: Param1) -> ::windows::runtime::Result<IMDSPStorage> {
        let mut result__: <IMDSPStorage as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).22)(::std::mem::transmute_copy(self), ::std::mem::transmute(findscope), pwszuniqueid.into_param().abi(), &mut result__).from_abi::<IMDSPStorage>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IMDSPDevice3 {
    type Vtable = IMDSPDevice3_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(444831813, 64597, 18556, [151, 111, 238, 56, 172, 14, 140, 78]);
}
impl ::std::convert::From<IMDSPDevice3> for ::windows::runtime::IUnknown {
    fn from(value: IMDSPDevice3) -> Self {
        value.0
    }
}
impl ::std::convert::From<&IMDSPDevice3> for ::windows::runtime::IUnknown {
    fn from(value: &IMDSPDevice3) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IMDSPDevice3 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IMDSPDevice3 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::std::convert::From<IMDSPDevice3> for IMDSPDevice2 {
    fn from(value: IMDSPDevice3) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IMDSPDevice3> for IMDSPDevice2 {
    fn from(value: &IMDSPDevice3) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IMDSPDevice2> for IMDSPDevice3 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IMDSPDevice2> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IMDSPDevice2>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IMDSPDevice2> for &IMDSPDevice3 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IMDSPDevice2> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IMDSPDevice2>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<IMDSPDevice3> for IMDSPDevice {
    fn from(value: IMDSPDevice3) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IMDSPDevice3> for IMDSPDevice {
    fn from(value: &IMDSPDevice3) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IMDSPDevice> for IMDSPDevice3 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IMDSPDevice> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IMDSPDevice>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IMDSPDevice> for &IMDSPDevice3 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IMDSPDevice> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IMDSPDevice>::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMDSPDevice3_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pwszname: super::super::Foundation::PWSTR, nmaxchars: u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pwszname: super::super::Foundation::PWSTR, nmaxchars: u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pdwversion: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pdwtype: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pserialnumber: *mut WMDMID, abmac: *mut u8) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pdwpowersource: *mut u32, pdwpercentremaining: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pdwstatus: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, hicon: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppenumstorage: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pformatex: *mut *mut _WAVEFORMATEX, pnformatcount: *mut u32, pppwszmimetype: *mut *mut super::super::Foundation::PWSTR, pnmimetypecount: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pcommand: *mut OPAQUECOMMAND) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pszstoragename: super::super::Foundation::PWSTR, ppstorage: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwflags: u32, ppaudioformatex: *mut *mut _WAVEFORMATEX, pnaudioformatcount: *mut u32, ppvideoformatex: *mut *mut _VIDEOINFOHEADER, pnvideoformatcount: *mut u32, ppfiletype: *mut *mut WMFILECAPABILITIES, pnfiletypecount: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_System_Ole")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppspecifyproppages: *mut ::windows::runtime::RawPtr, pppunknowns: *mut *mut ::windows::runtime::RawPtr, pcunks: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Ole"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pwszpnpname: super::super::Foundation::PWSTR, nmaxchars: u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Ole_Automation"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pwszpropname: super::super::Foundation::PWSTR, pvalue: *mut ::std::mem::ManuallyDrop<super::super::System::Com::StructuredStorage::PROPVARIANT>) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Ole_Automation")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Ole_Automation"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pwszpropname: super::super::Foundation::PWSTR, pvalue: *const ::std::mem::ManuallyDrop<super::super::System::Com::StructuredStorage::PROPVARIANT>) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Ole_Automation")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Ole_Automation"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, format: WMDM_FORMATCODE, pformatsupport: *mut WMDM_FORMAT_CAPABILITY) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Ole_Automation")))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwiocontrolcode: u32, lpinbuffer: *const u8, ninbuffersize: u32, lpoutbuffer: *mut u8, pnoutbuffersize: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, findscope: WMDM_FIND_SCOPE, pwszuniqueid: super::super::Foundation::PWSTR, ppstorage: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[doc = "*Required features: `Win32_Media_DeviceManager`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IMDSPDeviceControl(pub ::windows::runtime::IUnknown);
impl IMDSPDeviceControl {
    #[doc = "*Required features: `Win32_Media_DeviceManager`*"]
    pub unsafe fn GetDCStatus(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `Win32_Media_DeviceManager`*"]
    pub unsafe fn GetCapabilities(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `Win32_Media_DeviceManager`*"]
    pub unsafe fn Play(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `Win32_Media_DeviceManager`*"]
    pub unsafe fn Record(&self, pformat: *const _WAVEFORMATEX) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::std::mem::transmute_copy(self), ::std::mem::transmute(pformat)).ok()
    }
    #[doc = "*Required features: `Win32_Media_DeviceManager`*"]
    pub unsafe fn Pause(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `Win32_Media_DeviceManager`*"]
    pub unsafe fn Resume(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).8)(::std::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `Win32_Media_DeviceManager`*"]
    pub unsafe fn Stop(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).9)(::std::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `Win32_Media_DeviceManager`*"]
    pub unsafe fn Seek(&self, fumode: u32, noffset: i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).10)(::std::mem::transmute_copy(self), ::std::mem::transmute(fumode), ::std::mem::transmute(noffset)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IMDSPDeviceControl {
    type Vtable = IMDSPDeviceControl_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(499857940, 13293, 4563, [132, 112, 0, 192, 79, 121, 219, 192]);
}
impl ::std::convert::From<IMDSPDeviceControl> for ::windows::runtime::IUnknown {
    fn from(value: IMDSPDeviceControl) -> Self {
        value.0
    }
}
impl ::std::convert::From<&IMDSPDeviceControl> for ::windows::runtime::IUnknown {
    fn from(value: &IMDSPDeviceControl) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IMDSPDeviceControl {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IMDSPDeviceControl {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMDSPDeviceControl_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pdwstatus: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pdwcapabilitiesmask: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pformat: *const _WAVEFORMATEX) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, fumode: u32, noffset: i32) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_Media_DeviceManager`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IMDSPDirectTransfer(pub ::windows::runtime::IUnknown);
impl IMDSPDirectTransfer {
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_DeviceManager`, `Win32_Foundation`*"]
    pub unsafe fn TransferToDevice<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>, Param1: ::windows::runtime::IntoParam<'a, IWMDMOperation>, Param3: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>, Param4: ::windows::runtime::IntoParam<'a, IWMDMMetaData>, Param5: ::windows::runtime::IntoParam<'a, IWMDMProgress>>(
        &self,
        pwszsourcefilepath: Param0,
        psourceoperation: Param1,
        fuflags: u32,
        pwszdestinationname: Param3,
        psourcemetadata: Param4,
        ptransferprogress: Param5,
    ) -> ::windows::runtime::Result<IMDSPStorage> {
        let mut result__: <IMDSPStorage as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), pwszsourcefilepath.into_param().abi(), psourceoperation.into_param().abi(), ::std::mem::transmute(fuflags), pwszdestinationname.into_param().abi(), psourcemetadata.into_param().abi(), ptransferprogress.into_param().abi(), &mut result__).from_abi::<IMDSPStorage>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IMDSPDirectTransfer {
    type Vtable = IMDSPDirectTransfer_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3271448488, 37636, 18316, [158, 228, 71, 227, 151, 185, 18, 215]);
}
impl ::std::convert::From<IMDSPDirectTransfer> for ::windows::runtime::IUnknown {
    fn from(value: IMDSPDirectTransfer) -> Self {
        value.0
    }
}
impl ::std::convert::From<&IMDSPDirectTransfer> for ::windows::runtime::IUnknown {
    fn from(value: &IMDSPDirectTransfer) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IMDSPDirectTransfer {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IMDSPDirectTransfer {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMDSPDirectTransfer_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pwszsourcefilepath: super::super::Foundation::PWSTR, psourceoperation: ::windows::runtime::RawPtr, fuflags: u32, pwszdestinationname: super::super::Foundation::PWSTR, psourcemetadata: ::windows::runtime::RawPtr, ptransferprogress: ::windows::runtime::RawPtr, ppnewobject: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[doc = "*Required features: `Win32_Media_DeviceManager`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IMDSPEnumDevice(pub ::windows::runtime::IUnknown);
impl IMDSPEnumDevice {
    #[doc = "*Required features: `Win32_Media_DeviceManager`*"]
    pub unsafe fn Next(&self, celt: u32, ppdevice: *mut ::std::option::Option<IMDSPDevice>, pceltfetched: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), ::std::mem::transmute(celt), ::std::mem::transmute(ppdevice), ::std::mem::transmute(pceltfetched)).ok()
    }
    #[doc = "*Required features: `Win32_Media_DeviceManager`*"]
    pub unsafe fn Skip(&self, celt: u32) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), ::std::mem::transmute(celt), &mut result__).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `Win32_Media_DeviceManager`*"]
    pub unsafe fn Reset(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `Win32_Media_DeviceManager`*"]
    pub unsafe fn Clone(&self) -> ::windows::runtime::Result<IMDSPEnumDevice> {
        let mut result__: <IMDSPEnumDevice as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).6)(::std::mem::transmute_copy(self), &mut result__).from_abi::<IMDSPEnumDevice>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IMDSPEnumDevice {
    type Vtable = IMDSPEnumDevice_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(499857937, 13293, 4563, [132, 112, 0, 192, 79, 121, 219, 192]);
}
impl ::std::convert::From<IMDSPEnumDevice> for ::windows::runtime::IUnknown {
    fn from(value: IMDSPEnumDevice) -> Self {
        value.0
    }
}
impl ::std::convert::From<&IMDSPEnumDevice> for ::windows::runtime::IUnknown {
    fn from(value: &IMDSPEnumDevice) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IMDSPEnumDevice {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IMDSPEnumDevice {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMDSPEnumDevice_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, celt: u32, ppdevice: *mut ::windows::runtime::RawPtr, pceltfetched: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, celt: u32, pceltfetched: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppenumdevice: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_Media_DeviceManager`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IMDSPEnumStorage(pub ::windows::runtime::IUnknown);
impl IMDSPEnumStorage {
    #[doc = "*Required features: `Win32_Media_DeviceManager`*"]
    pub unsafe fn Next(&self, celt: u32, ppstorage: *mut ::std::option::Option<IMDSPStorage>, pceltfetched: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), ::std::mem::transmute(celt), ::std::mem::transmute(ppstorage), ::std::mem::transmute(pceltfetched)).ok()
    }
    #[doc = "*Required features: `Win32_Media_DeviceManager`*"]
    pub unsafe fn Skip(&self, celt: u32) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), ::std::mem::transmute(celt), &mut result__).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `Win32_Media_DeviceManager`*"]
    pub unsafe fn Reset(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `Win32_Media_DeviceManager`*"]
    pub unsafe fn Clone(&self) -> ::windows::runtime::Result<IMDSPEnumStorage> {
        let mut result__: <IMDSPEnumStorage as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).6)(::std::mem::transmute_copy(self), &mut result__).from_abi::<IMDSPEnumStorage>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IMDSPEnumStorage {
    type Vtable = IMDSPEnumStorage_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(499857941, 13293, 4563, [132, 112, 0, 192, 79, 121, 219, 192]);
}
impl ::std::convert::From<IMDSPEnumStorage> for ::windows::runtime::IUnknown {
    fn from(value: IMDSPEnumStorage) -> Self {
        value.0
    }
}
impl ::std::convert::From<&IMDSPEnumStorage> for ::windows::runtime::IUnknown {
    fn from(value: &IMDSPEnumStorage) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IMDSPEnumStorage {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IMDSPEnumStorage {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMDSPEnumStorage_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, celt: u32, ppstorage: *mut ::windows::runtime::RawPtr, pceltfetched: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, celt: u32, pceltfetched: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppenumstorage: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_Media_DeviceManager`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IMDSPObject(pub ::windows::runtime::IUnknown);
impl IMDSPObject {
    #[doc = "*Required features: `Win32_Media_DeviceManager`*"]
    pub unsafe fn Open(&self, fumode: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), ::std::mem::transmute(fumode)).ok()
    }
    #[doc = "*Required features: `Win32_Media_DeviceManager`*"]
    pub unsafe fn Read(&self, pdata: *mut u8, pdwsize: *mut u32, abmac: *mut u8) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), ::std::mem::transmute(pdata), ::std::mem::transmute(pdwsize), ::std::mem::transmute(abmac)).ok()
    }
    #[doc = "*Required features: `Win32_Media_DeviceManager`*"]
    pub unsafe fn Write(&self, pdata: *const u8, pdwsize: *mut u32, abmac: *mut u8) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self), ::std::mem::transmute(pdata), ::std::mem::transmute(pdwsize), ::std::mem::transmute(abmac)).ok()
    }
    #[doc = "*Required features: `Win32_Media_DeviceManager`*"]
    pub unsafe fn Delete<'a, Param1: ::windows::runtime::IntoParam<'a, IWMDMProgress>>(&self, fumode: u32, pprogress: Param1) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::std::mem::transmute_copy(self), ::std::mem::transmute(fumode), pprogress.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_Media_DeviceManager`*"]
    pub unsafe fn Seek(&self, fuflags: u32, dwoffset: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self), ::std::mem::transmute(fuflags), ::std::mem::transmute(dwoffset)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_DeviceManager`, `Win32_Foundation`*"]
    pub unsafe fn Rename<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>, Param1: ::windows::runtime::IntoParam<'a, IWMDMProgress>>(&self, pwsznewname: Param0, pprogress: Param1) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).8)(::std::mem::transmute_copy(self), pwsznewname.into_param().abi(), pprogress.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_Media_DeviceManager`*"]
    pub unsafe fn Move<'a, Param1: ::windows::runtime::IntoParam<'a, IWMDMProgress>, Param2: ::windows::runtime::IntoParam<'a, IMDSPStorage>>(&self, fumode: u32, pprogress: Param1, ptarget: Param2) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).9)(::std::mem::transmute_copy(self), ::std::mem::transmute(fumode), pprogress.into_param().abi(), ptarget.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_Media_DeviceManager`*"]
    pub unsafe fn Close(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).10)(::std::mem::transmute_copy(self)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IMDSPObject {
    type Vtable = IMDSPObject_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(499857944, 13293, 4563, [132, 112, 0, 192, 79, 121, 219, 192]);
}
impl ::std::convert::From<IMDSPObject> for ::windows::runtime::IUnknown {
    fn from(value: IMDSPObject) -> Self {
        value.0
    }
}
impl ::std::convert::From<&IMDSPObject> for ::windows::runtime::IUnknown {
    fn from(value: &IMDSPObject) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IMDSPObject {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IMDSPObject {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMDSPObject_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, fumode: u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pdata: *mut u8, pdwsize: *mut u32, abmac: *mut u8) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pdata: *const u8, pdwsize: *mut u32, abmac: *mut u8) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, fumode: u32, pprogress: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, fuflags: u32, dwoffset: u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pwsznewname: super::super::Foundation::PWSTR, pprogress: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, fumode: u32, pprogress: ::windows::runtime::RawPtr, ptarget: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_Media_DeviceManager`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IMDSPObject2(pub ::windows::runtime::IUnknown);
impl IMDSPObject2 {
    #[doc = "*Required features: `Win32_Media_DeviceManager`*"]
    pub unsafe fn Open(&self, fumode: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), ::std::mem::transmute(fumode)).ok()
    }
    #[doc = "*Required features: `Win32_Media_DeviceManager`*"]
    pub unsafe fn Read(&self, pdata: *mut u8, pdwsize: *mut u32, abmac: *mut u8) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), ::std::mem::transmute(pdata), ::std::mem::transmute(pdwsize), ::std::mem::transmute(abmac)).ok()
    }
    #[doc = "*Required features: `Win32_Media_DeviceManager`*"]
    pub unsafe fn Write(&self, pdata: *const u8, pdwsize: *mut u32, abmac: *mut u8) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self), ::std::mem::transmute(pdata), ::std::mem::transmute(pdwsize), ::std::mem::transmute(abmac)).ok()
    }
    #[doc = "*Required features: `Win32_Media_DeviceManager`*"]
    pub unsafe fn Delete<'a, Param1: ::windows::runtime::IntoParam<'a, IWMDMProgress>>(&self, fumode: u32, pprogress: Param1) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::std::mem::transmute_copy(self), ::std::mem::transmute(fumode), pprogress.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_Media_DeviceManager`*"]
    pub unsafe fn Seek(&self, fuflags: u32, dwoffset: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self), ::std::mem::transmute(fuflags), ::std::mem::transmute(dwoffset)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_DeviceManager`, `Win32_Foundation`*"]
    pub unsafe fn Rename<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>, Param1: ::windows::runtime::IntoParam<'a, IWMDMProgress>>(&self, pwsznewname: Param0, pprogress: Param1) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).8)(::std::mem::transmute_copy(self), pwsznewname.into_param().abi(), pprogress.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_Media_DeviceManager`*"]
    pub unsafe fn Move<'a, Param1: ::windows::runtime::IntoParam<'a, IWMDMProgress>, Param2: ::windows::runtime::IntoParam<'a, IMDSPStorage>>(&self, fumode: u32, pprogress: Param1, ptarget: Param2) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).9)(::std::mem::transmute_copy(self), ::std::mem::transmute(fumode), pprogress.into_param().abi(), ptarget.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_Media_DeviceManager`*"]
    pub unsafe fn Close(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).10)(::std::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `Win32_Media_DeviceManager`*"]
    pub unsafe fn ReadOnClearChannel(&self, pdata: *mut u8, pdwsize: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).11)(::std::mem::transmute_copy(self), ::std::mem::transmute(pdata), ::std::mem::transmute(pdwsize)).ok()
    }
    #[doc = "*Required features: `Win32_Media_DeviceManager`*"]
    pub unsafe fn WriteOnClearChannel(&self, pdata: *const u8, pdwsize: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).12)(::std::mem::transmute_copy(self), ::std::mem::transmute(pdata), ::std::mem::transmute(pdwsize)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IMDSPObject2 {
    type Vtable = IMDSPObject2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1060425022, 22791, 17217, [154, 249, 151, 244, 24, 124, 58, 165]);
}
impl ::std::convert::From<IMDSPObject2> for ::windows::runtime::IUnknown {
    fn from(value: IMDSPObject2) -> Self {
        value.0
    }
}
impl ::std::convert::From<&IMDSPObject2> for ::windows::runtime::IUnknown {
    fn from(value: &IMDSPObject2) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IMDSPObject2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IMDSPObject2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::std::convert::From<IMDSPObject2> for IMDSPObject {
    fn from(value: IMDSPObject2) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IMDSPObject2> for IMDSPObject {
    fn from(value: &IMDSPObject2) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IMDSPObject> for IMDSPObject2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IMDSPObject> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IMDSPObject>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IMDSPObject> for &IMDSPObject2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IMDSPObject> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IMDSPObject>::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMDSPObject2_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, fumode: u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pdata: *mut u8, pdwsize: *mut u32, abmac: *mut u8) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pdata: *const u8, pdwsize: *mut u32, abmac: *mut u8) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, fumode: u32, pprogress: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, fuflags: u32, dwoffset: u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pwsznewname: super::super::Foundation::PWSTR, pprogress: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, fumode: u32, pprogress: ::windows::runtime::RawPtr, ptarget: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pdata: *mut u8, pdwsize: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pdata: *const u8, pdwsize: *mut u32) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_Media_DeviceManager`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IMDSPObjectInfo(pub ::windows::runtime::IUnknown);
impl IMDSPObjectInfo {
    #[doc = "*Required features: `Win32_Media_DeviceManager`*"]
    pub unsafe fn GetPlayLength(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `Win32_Media_DeviceManager`*"]
    pub unsafe fn SetPlayLength(&self, dwlength: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), ::std::mem::transmute(dwlength)).ok()
    }
    #[doc = "*Required features: `Win32_Media_DeviceManager`*"]
    pub unsafe fn GetPlayOffset(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `Win32_Media_DeviceManager`*"]
    pub unsafe fn SetPlayOffset(&self, dwoffset: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::std::mem::transmute_copy(self), ::std::mem::transmute(dwoffset)).ok()
    }
    #[doc = "*Required features: `Win32_Media_DeviceManager`*"]
    pub unsafe fn GetTotalLength(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `Win32_Media_DeviceManager`*"]
    pub unsafe fn GetLastPlayPosition(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).8)(::std::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `Win32_Media_DeviceManager`*"]
    pub unsafe fn GetLongestPlayPosition(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).9)(::std::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IMDSPObjectInfo {
    type Vtable = IMDSPObjectInfo_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(499857945, 13293, 4563, [132, 112, 0, 192, 79, 121, 219, 192]);
}
impl ::std::convert::From<IMDSPObjectInfo> for ::windows::runtime::IUnknown {
    fn from(value: IMDSPObjectInfo) -> Self {
        value.0
    }
}
impl ::std::convert::From<&IMDSPObjectInfo> for ::windows::runtime::IUnknown {
    fn from(value: &IMDSPObjectInfo) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IMDSPObjectInfo {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IMDSPObjectInfo {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMDSPObjectInfo_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pdwlength: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwlength: u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pdwoffset: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwoffset: u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pdwlength: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pdwlastpos: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pdwlongestpos: *mut u32) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_Media_DeviceManager`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IMDSPRevoked(pub ::windows::runtime::IUnknown);
impl IMDSPRevoked {
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_DeviceManager`, `Win32_Foundation`*"]
    pub unsafe fn GetRevocationURL(&self, ppwszrevocationurl: *mut super::super::Foundation::PWSTR, pdwbufferlen: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), ::std::mem::transmute(ppwszrevocationurl), ::std::mem::transmute(pdwbufferlen)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IMDSPRevoked {
    type Vtable = IMDSPRevoked_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2766729940, 16177, 17997, [181, 61, 79, 195, 53, 153, 129, 132]);
}
impl ::std::convert::From<IMDSPRevoked> for ::windows::runtime::IUnknown {
    fn from(value: IMDSPRevoked) -> Self {
        value.0
    }
}
impl ::std::convert::From<&IMDSPRevoked> for ::windows::runtime::IUnknown {
    fn from(value: &IMDSPRevoked) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IMDSPRevoked {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IMDSPRevoked {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMDSPRevoked_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppwszrevocationurl: *mut super::super::Foundation::PWSTR, pdwbufferlen: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[doc = "*Required features: `Win32_Media_DeviceManager`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IMDSPStorage(pub ::windows::runtime::IUnknown);
impl IMDSPStorage {
    #[doc = "*Required features: `Win32_Media_DeviceManager`*"]
    pub unsafe fn SetAttributes(&self, dwattributes: u32, pformat: *const _WAVEFORMATEX) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), ::std::mem::transmute(dwattributes), ::std::mem::transmute(pformat)).ok()
    }
    #[doc = "*Required features: `Win32_Media_DeviceManager`*"]
    pub unsafe fn GetStorageGlobals(&self) -> ::windows::runtime::Result<IMDSPStorageGlobals> {
        let mut result__: <IMDSPStorageGlobals as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), &mut result__).from_abi::<IMDSPStorageGlobals>(result__)
    }
    #[doc = "*Required features: `Win32_Media_DeviceManager`*"]
    pub unsafe fn GetAttributes(&self, pdwattributes: *mut u32, pformat: *mut _WAVEFORMATEX) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self), ::std::mem::transmute(pdwattributes), ::std::mem::transmute(pformat)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_DeviceManager`, `Win32_Foundation`*"]
    pub unsafe fn GetName(&self, pwszname: super::super::Foundation::PWSTR, nmaxchars: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::std::mem::transmute_copy(self), ::std::mem::transmute(pwszname), ::std::mem::transmute(nmaxchars)).ok()
    }
    #[doc = "*Required features: `Win32_Media_DeviceManager`*"]
    pub unsafe fn GetDate(&self) -> ::windows::runtime::Result<WMDMDATETIME> {
        let mut result__: <WMDMDATETIME as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self), &mut result__).from_abi::<WMDMDATETIME>(result__)
    }
    #[doc = "*Required features: `Win32_Media_DeviceManager`*"]
    pub unsafe fn GetSize(&self, pdwsizelow: *mut u32, pdwsizehigh: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).8)(::std::mem::transmute_copy(self), ::std::mem::transmute(pdwsizelow), ::std::mem::transmute(pdwsizehigh)).ok()
    }
    #[doc = "*Required features: `Win32_Media_DeviceManager`*"]
    pub unsafe fn GetRights(&self, pprights: *mut *mut WMDMRIGHTS, pnrightscount: *mut u32, abmac: *mut u8) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).9)(::std::mem::transmute_copy(self), ::std::mem::transmute(pprights), ::std::mem::transmute(pnrightscount), ::std::mem::transmute(abmac)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_DeviceManager`, `Win32_Foundation`*"]
    pub unsafe fn CreateStorage<'a, Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, dwattributes: u32, pformat: *const _WAVEFORMATEX, pwszname: Param2) -> ::windows::runtime::Result<IMDSPStorage> {
        let mut result__: <IMDSPStorage as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).10)(::std::mem::transmute_copy(self), ::std::mem::transmute(dwattributes), ::std::mem::transmute(pformat), pwszname.into_param().abi(), &mut result__).from_abi::<IMDSPStorage>(result__)
    }
    #[doc = "*Required features: `Win32_Media_DeviceManager`*"]
    pub unsafe fn EnumStorage(&self) -> ::windows::runtime::Result<IMDSPEnumStorage> {
        let mut result__: <IMDSPEnumStorage as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).11)(::std::mem::transmute_copy(self), &mut result__).from_abi::<IMDSPEnumStorage>(result__)
    }
    #[doc = "*Required features: `Win32_Media_DeviceManager`*"]
    pub unsafe fn SendOpaqueCommand(&self, pcommand: *mut OPAQUECOMMAND) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).12)(::std::mem::transmute_copy(self), ::std::mem::transmute(pcommand)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IMDSPStorage {
    type Vtable = IMDSPStorage_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(499857942, 13293, 4563, [132, 112, 0, 192, 79, 121, 219, 192]);
}
impl ::std::convert::From<IMDSPStorage> for ::windows::runtime::IUnknown {
    fn from(value: IMDSPStorage) -> Self {
        value.0
    }
}
impl ::std::convert::From<&IMDSPStorage> for ::windows::runtime::IUnknown {
    fn from(value: &IMDSPStorage) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IMDSPStorage {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IMDSPStorage {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMDSPStorage_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwattributes: u32, pformat: *const _WAVEFORMATEX) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppstorageglobals: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pdwattributes: *mut u32, pformat: *mut _WAVEFORMATEX) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pwszname: super::super::Foundation::PWSTR, nmaxchars: u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pdatetimeutc: *mut WMDMDATETIME) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pdwsizelow: *mut u32, pdwsizehigh: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pprights: *mut *mut WMDMRIGHTS, pnrightscount: *mut u32, abmac: *mut u8) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwattributes: u32, pformat: *const _WAVEFORMATEX, pwszname: super::super::Foundation::PWSTR, ppnewstorage: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppenumstorage: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pcommand: *mut OPAQUECOMMAND) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_Media_DeviceManager`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IMDSPStorage2(pub ::windows::runtime::IUnknown);
impl IMDSPStorage2 {
    #[doc = "*Required features: `Win32_Media_DeviceManager`*"]
    pub unsafe fn SetAttributes(&self, dwattributes: u32, pformat: *const _WAVEFORMATEX) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), ::std::mem::transmute(dwattributes), ::std::mem::transmute(pformat)).ok()
    }
    #[doc = "*Required features: `Win32_Media_DeviceManager`*"]
    pub unsafe fn GetStorageGlobals(&self) -> ::windows::runtime::Result<IMDSPStorageGlobals> {
        let mut result__: <IMDSPStorageGlobals as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), &mut result__).from_abi::<IMDSPStorageGlobals>(result__)
    }
    #[doc = "*Required features: `Win32_Media_DeviceManager`*"]
    pub unsafe fn GetAttributes(&self, pdwattributes: *mut u32, pformat: *mut _WAVEFORMATEX) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self), ::std::mem::transmute(pdwattributes), ::std::mem::transmute(pformat)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_DeviceManager`, `Win32_Foundation`*"]
    pub unsafe fn GetName(&self, pwszname: super::super::Foundation::PWSTR, nmaxchars: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::std::mem::transmute_copy(self), ::std::mem::transmute(pwszname), ::std::mem::transmute(nmaxchars)).ok()
    }
    #[doc = "*Required features: `Win32_Media_DeviceManager`*"]
    pub unsafe fn GetDate(&self) -> ::windows::runtime::Result<WMDMDATETIME> {
        let mut result__: <WMDMDATETIME as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self), &mut result__).from_abi::<WMDMDATETIME>(result__)
    }
    #[doc = "*Required features: `Win32_Media_DeviceManager`*"]
    pub unsafe fn GetSize(&self, pdwsizelow: *mut u32, pdwsizehigh: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).8)(::std::mem::transmute_copy(self), ::std::mem::transmute(pdwsizelow), ::std::mem::transmute(pdwsizehigh)).ok()
    }
    #[doc = "*Required features: `Win32_Media_DeviceManager`*"]
    pub unsafe fn GetRights(&self, pprights: *mut *mut WMDMRIGHTS, pnrightscount: *mut u32, abmac: *mut u8) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).9)(::std::mem::transmute_copy(self), ::std::mem::transmute(pprights), ::std::mem::transmute(pnrightscount), ::std::mem::transmute(abmac)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_DeviceManager`, `Win32_Foundation`*"]
    pub unsafe fn CreateStorage<'a, Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, dwattributes: u32, pformat: *const _WAVEFORMATEX, pwszname: Param2) -> ::windows::runtime::Result<IMDSPStorage> {
        let mut result__: <IMDSPStorage as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).10)(::std::mem::transmute_copy(self), ::std::mem::transmute(dwattributes), ::std::mem::transmute(pformat), pwszname.into_param().abi(), &mut result__).from_abi::<IMDSPStorage>(result__)
    }
    #[doc = "*Required features: `Win32_Media_DeviceManager`*"]
    pub unsafe fn EnumStorage(&self) -> ::windows::runtime::Result<IMDSPEnumStorage> {
        let mut result__: <IMDSPEnumStorage as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).11)(::std::mem::transmute_copy(self), &mut result__).from_abi::<IMDSPEnumStorage>(result__)
    }
    #[doc = "*Required features: `Win32_Media_DeviceManager`*"]
    pub unsafe fn SendOpaqueCommand(&self, pcommand: *mut OPAQUECOMMAND) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).12)(::std::mem::transmute_copy(self), ::std::mem::transmute(pcommand)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_DeviceManager`, `Win32_Foundation`*"]
    pub unsafe fn GetStorage<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, pszstoragename: Param0) -> ::windows::runtime::Result<IMDSPStorage> {
        let mut result__: <IMDSPStorage as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).13)(::std::mem::transmute_copy(self), pszstoragename.into_param().abi(), &mut result__).from_abi::<IMDSPStorage>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_DeviceManager`, `Win32_Foundation`*"]
    pub unsafe fn CreateStorage2<'a, Param4: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, dwattributes: u32, dwattributesex: u32, paudioformat: *const _WAVEFORMATEX, pvideoformat: *const _VIDEOINFOHEADER, pwszname: Param4, qwfilesize: u64) -> ::windows::runtime::Result<IMDSPStorage> {
        let mut result__: <IMDSPStorage as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).14)(::std::mem::transmute_copy(self), ::std::mem::transmute(dwattributes), ::std::mem::transmute(dwattributesex), ::std::mem::transmute(paudioformat), ::std::mem::transmute(pvideoformat), pwszname.into_param().abi(), ::std::mem::transmute(qwfilesize), &mut result__).from_abi::<IMDSPStorage>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_DeviceManager`, `Win32_Foundation`*"]
    pub unsafe fn SetAttributes2(&self, dwattributes: u32, dwattributesex: u32, paudioformat: *const _WAVEFORMATEX, pvideoformat: *const _VIDEOINFOHEADER) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).15)(::std::mem::transmute_copy(self), ::std::mem::transmute(dwattributes), ::std::mem::transmute(dwattributesex), ::std::mem::transmute(paudioformat), ::std::mem::transmute(pvideoformat)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_DeviceManager`, `Win32_Foundation`*"]
    pub unsafe fn GetAttributes2(&self, pdwattributes: *mut u32, pdwattributesex: *mut u32, paudioformat: *mut _WAVEFORMATEX, pvideoformat: *mut _VIDEOINFOHEADER) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).16)(::std::mem::transmute_copy(self), ::std::mem::transmute(pdwattributes), ::std::mem::transmute(pdwattributesex), ::std::mem::transmute(paudioformat), ::std::mem::transmute(pvideoformat)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IMDSPStorage2 {
    type Vtable = IMDSPStorage2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(173934501, 25684, 17489, [156, 54, 28, 106, 231, 226, 177, 214]);
}
impl ::std::convert::From<IMDSPStorage2> for ::windows::runtime::IUnknown {
    fn from(value: IMDSPStorage2) -> Self {
        value.0
    }
}
impl ::std::convert::From<&IMDSPStorage2> for ::windows::runtime::IUnknown {
    fn from(value: &IMDSPStorage2) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IMDSPStorage2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IMDSPStorage2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::std::convert::From<IMDSPStorage2> for IMDSPStorage {
    fn from(value: IMDSPStorage2) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IMDSPStorage2> for IMDSPStorage {
    fn from(value: &IMDSPStorage2) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IMDSPStorage> for IMDSPStorage2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IMDSPStorage> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IMDSPStorage>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IMDSPStorage> for &IMDSPStorage2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IMDSPStorage> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IMDSPStorage>::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMDSPStorage2_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwattributes: u32, pformat: *const _WAVEFORMATEX) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppstorageglobals: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pdwattributes: *mut u32, pformat: *mut _WAVEFORMATEX) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pwszname: super::super::Foundation::PWSTR, nmaxchars: u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pdatetimeutc: *mut WMDMDATETIME) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pdwsizelow: *mut u32, pdwsizehigh: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pprights: *mut *mut WMDMRIGHTS, pnrightscount: *mut u32, abmac: *mut u8) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwattributes: u32, pformat: *const _WAVEFORMATEX, pwszname: super::super::Foundation::PWSTR, ppnewstorage: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppenumstorage: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pcommand: *mut OPAQUECOMMAND) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pszstoragename: super::super::Foundation::PWSTR, ppstorage: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwattributes: u32, dwattributesex: u32, paudioformat: *const _WAVEFORMATEX, pvideoformat: *const _VIDEOINFOHEADER, pwszname: super::super::Foundation::PWSTR, qwfilesize: u64, ppnewstorage: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwattributes: u32, dwattributesex: u32, paudioformat: *const _WAVEFORMATEX, pvideoformat: *const _VIDEOINFOHEADER) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pdwattributes: *mut u32, pdwattributesex: *mut u32, paudioformat: *mut _WAVEFORMATEX, pvideoformat: *mut _VIDEOINFOHEADER) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[doc = "*Required features: `Win32_Media_DeviceManager`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IMDSPStorage3(pub ::windows::runtime::IUnknown);
impl IMDSPStorage3 {
    #[doc = "*Required features: `Win32_Media_DeviceManager`*"]
    pub unsafe fn SetAttributes(&self, dwattributes: u32, pformat: *const _WAVEFORMATEX) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), ::std::mem::transmute(dwattributes), ::std::mem::transmute(pformat)).ok()
    }
    #[doc = "*Required features: `Win32_Media_DeviceManager`*"]
    pub unsafe fn GetStorageGlobals(&self) -> ::windows::runtime::Result<IMDSPStorageGlobals> {
        let mut result__: <IMDSPStorageGlobals as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), &mut result__).from_abi::<IMDSPStorageGlobals>(result__)
    }
    #[doc = "*Required features: `Win32_Media_DeviceManager`*"]
    pub unsafe fn GetAttributes(&self, pdwattributes: *mut u32, pformat: *mut _WAVEFORMATEX) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self), ::std::mem::transmute(pdwattributes), ::std::mem::transmute(pformat)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_DeviceManager`, `Win32_Foundation`*"]
    pub unsafe fn GetName(&self, pwszname: super::super::Foundation::PWSTR, nmaxchars: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::std::mem::transmute_copy(self), ::std::mem::transmute(pwszname), ::std::mem::transmute(nmaxchars)).ok()
    }
    #[doc = "*Required features: `Win32_Media_DeviceManager`*"]
    pub unsafe fn GetDate(&self) -> ::windows::runtime::Result<WMDMDATETIME> {
        let mut result__: <WMDMDATETIME as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self), &mut result__).from_abi::<WMDMDATETIME>(result__)
    }
    #[doc = "*Required features: `Win32_Media_DeviceManager`*"]
    pub unsafe fn GetSize(&self, pdwsizelow: *mut u32, pdwsizehigh: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).8)(::std::mem::transmute_copy(self), ::std::mem::transmute(pdwsizelow), ::std::mem::transmute(pdwsizehigh)).ok()
    }
    #[doc = "*Required features: `Win32_Media_DeviceManager`*"]
    pub unsafe fn GetRights(&self, pprights: *mut *mut WMDMRIGHTS, pnrightscount: *mut u32, abmac: *mut u8) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).9)(::std::mem::transmute_copy(self), ::std::mem::transmute(pprights), ::std::mem::transmute(pnrightscount), ::std::mem::transmute(abmac)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_DeviceManager`, `Win32_Foundation`*"]
    pub unsafe fn CreateStorage<'a, Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, dwattributes: u32, pformat: *const _WAVEFORMATEX, pwszname: Param2) -> ::windows::runtime::Result<IMDSPStorage> {
        let mut result__: <IMDSPStorage as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).10)(::std::mem::transmute_copy(self), ::std::mem::transmute(dwattributes), ::std::mem::transmute(pformat), pwszname.into_param().abi(), &mut result__).from_abi::<IMDSPStorage>(result__)
    }
    #[doc = "*Required features: `Win32_Media_DeviceManager`*"]
    pub unsafe fn EnumStorage(&self) -> ::windows::runtime::Result<IMDSPEnumStorage> {
        let mut result__: <IMDSPEnumStorage as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).11)(::std::mem::transmute_copy(self), &mut result__).from_abi::<IMDSPEnumStorage>(result__)
    }
    #[doc = "*Required features: `Win32_Media_DeviceManager`*"]
    pub unsafe fn SendOpaqueCommand(&self, pcommand: *mut OPAQUECOMMAND) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).12)(::std::mem::transmute_copy(self), ::std::mem::transmute(pcommand)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_DeviceManager`, `Win32_Foundation`*"]
    pub unsafe fn GetStorage<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, pszstoragename: Param0) -> ::windows::runtime::Result<IMDSPStorage> {
        let mut result__: <IMDSPStorage as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).13)(::std::mem::transmute_copy(self), pszstoragename.into_param().abi(), &mut result__).from_abi::<IMDSPStorage>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_DeviceManager`, `Win32_Foundation`*"]
    pub unsafe fn CreateStorage2<'a, Param4: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, dwattributes: u32, dwattributesex: u32, paudioformat: *const _WAVEFORMATEX, pvideoformat: *const _VIDEOINFOHEADER, pwszname: Param4, qwfilesize: u64) -> ::windows::runtime::Result<IMDSPStorage> {
        let mut result__: <IMDSPStorage as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).14)(::std::mem::transmute_copy(self), ::std::mem::transmute(dwattributes), ::std::mem::transmute(dwattributesex), ::std::mem::transmute(paudioformat), ::std::mem::transmute(pvideoformat), pwszname.into_param().abi(), ::std::mem::transmute(qwfilesize), &mut result__).from_abi::<IMDSPStorage>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_DeviceManager`, `Win32_Foundation`*"]
    pub unsafe fn SetAttributes2(&self, dwattributes: u32, dwattributesex: u32, paudioformat: *const _WAVEFORMATEX, pvideoformat: *const _VIDEOINFOHEADER) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).15)(::std::mem::transmute_copy(self), ::std::mem::transmute(dwattributes), ::std::mem::transmute(dwattributesex), ::std::mem::transmute(paudioformat), ::std::mem::transmute(pvideoformat)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_DeviceManager`, `Win32_Foundation`*"]
    pub unsafe fn GetAttributes2(&self, pdwattributes: *mut u32, pdwattributesex: *mut u32, paudioformat: *mut _WAVEFORMATEX, pvideoformat: *mut _VIDEOINFOHEADER) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).16)(::std::mem::transmute_copy(self), ::std::mem::transmute(pdwattributes), ::std::mem::transmute(pdwattributesex), ::std::mem::transmute(paudioformat), ::std::mem::transmute(pvideoformat)).ok()
    }
    #[doc = "*Required features: `Win32_Media_DeviceManager`*"]
    pub unsafe fn GetMetadata<'a, Param0: ::windows::runtime::IntoParam<'a, IWMDMMetaData>>(&self, pmetadata: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).17)(::std::mem::transmute_copy(self), pmetadata.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_Media_DeviceManager`*"]
    pub unsafe fn SetMetadata<'a, Param0: ::windows::runtime::IntoParam<'a, IWMDMMetaData>>(&self, pmetadata: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).18)(::std::mem::transmute_copy(self), pmetadata.into_param().abi()).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IMDSPStorage3 {
    type Vtable = IMDSPStorage3_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1818663015, 38893, 19047, [151, 6, 28, 85, 41, 210, 164, 20]);
}
impl ::std::convert::From<IMDSPStorage3> for ::windows::runtime::IUnknown {
    fn from(value: IMDSPStorage3) -> Self {
        value.0
    }
}
impl ::std::convert::From<&IMDSPStorage3> for ::windows::runtime::IUnknown {
    fn from(value: &IMDSPStorage3) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IMDSPStorage3 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IMDSPStorage3 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::std::convert::From<IMDSPStorage3> for IMDSPStorage2 {
    fn from(value: IMDSPStorage3) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IMDSPStorage3> for IMDSPStorage2 {
    fn from(value: &IMDSPStorage3) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IMDSPStorage2> for IMDSPStorage3 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IMDSPStorage2> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IMDSPStorage2>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IMDSPStorage2> for &IMDSPStorage3 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IMDSPStorage2> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IMDSPStorage2>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<IMDSPStorage3> for IMDSPStorage {
    fn from(value: IMDSPStorage3) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IMDSPStorage3> for IMDSPStorage {
    fn from(value: &IMDSPStorage3) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IMDSPStorage> for IMDSPStorage3 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IMDSPStorage> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IMDSPStorage>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IMDSPStorage> for &IMDSPStorage3 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IMDSPStorage> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IMDSPStorage>::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMDSPStorage3_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwattributes: u32, pformat: *const _WAVEFORMATEX) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppstorageglobals: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pdwattributes: *mut u32, pformat: *mut _WAVEFORMATEX) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pwszname: super::super::Foundation::PWSTR, nmaxchars: u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pdatetimeutc: *mut WMDMDATETIME) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pdwsizelow: *mut u32, pdwsizehigh: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pprights: *mut *mut WMDMRIGHTS, pnrightscount: *mut u32, abmac: *mut u8) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwattributes: u32, pformat: *const _WAVEFORMATEX, pwszname: super::super::Foundation::PWSTR, ppnewstorage: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppenumstorage: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pcommand: *mut OPAQUECOMMAND) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pszstoragename: super::super::Foundation::PWSTR, ppstorage: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwattributes: u32, dwattributesex: u32, paudioformat: *const _WAVEFORMATEX, pvideoformat: *const _VIDEOINFOHEADER, pwszname: super::super::Foundation::PWSTR, qwfilesize: u64, ppnewstorage: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwattributes: u32, dwattributesex: u32, paudioformat: *const _WAVEFORMATEX, pvideoformat: *const _VIDEOINFOHEADER) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pdwattributes: *mut u32, pdwattributesex: *mut u32, paudioformat: *mut _WAVEFORMATEX, pvideoformat: *mut _VIDEOINFOHEADER) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pmetadata: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pmetadata: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_Media_DeviceManager`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IMDSPStorage4(pub ::windows::runtime::IUnknown);
impl IMDSPStorage4 {
    #[doc = "*Required features: `Win32_Media_DeviceManager`*"]
    pub unsafe fn SetAttributes(&self, dwattributes: u32, pformat: *const _WAVEFORMATEX) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), ::std::mem::transmute(dwattributes), ::std::mem::transmute(pformat)).ok()
    }
    #[doc = "*Required features: `Win32_Media_DeviceManager`*"]
    pub unsafe fn GetStorageGlobals(&self) -> ::windows::runtime::Result<IMDSPStorageGlobals> {
        let mut result__: <IMDSPStorageGlobals as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), &mut result__).from_abi::<IMDSPStorageGlobals>(result__)
    }
    #[doc = "*Required features: `Win32_Media_DeviceManager`*"]
    pub unsafe fn GetAttributes(&self, pdwattributes: *mut u32, pformat: *mut _WAVEFORMATEX) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self), ::std::mem::transmute(pdwattributes), ::std::mem::transmute(pformat)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_DeviceManager`, `Win32_Foundation`*"]
    pub unsafe fn GetName(&self, pwszname: super::super::Foundation::PWSTR, nmaxchars: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::std::mem::transmute_copy(self), ::std::mem::transmute(pwszname), ::std::mem::transmute(nmaxchars)).ok()
    }
    #[doc = "*Required features: `Win32_Media_DeviceManager`*"]
    pub unsafe fn GetDate(&self) -> ::windows::runtime::Result<WMDMDATETIME> {
        let mut result__: <WMDMDATETIME as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self), &mut result__).from_abi::<WMDMDATETIME>(result__)
    }
    #[doc = "*Required features: `Win32_Media_DeviceManager`*"]
    pub unsafe fn GetSize(&self, pdwsizelow: *mut u32, pdwsizehigh: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).8)(::std::mem::transmute_copy(self), ::std::mem::transmute(pdwsizelow), ::std::mem::transmute(pdwsizehigh)).ok()
    }
    #[doc = "*Required features: `Win32_Media_DeviceManager`*"]
    pub unsafe fn GetRights(&self, pprights: *mut *mut WMDMRIGHTS, pnrightscount: *mut u32, abmac: *mut u8) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).9)(::std::mem::transmute_copy(self), ::std::mem::transmute(pprights), ::std::mem::transmute(pnrightscount), ::std::mem::transmute(abmac)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_DeviceManager`, `Win32_Foundation`*"]
    pub unsafe fn CreateStorage<'a, Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, dwattributes: u32, pformat: *const _WAVEFORMATEX, pwszname: Param2) -> ::windows::runtime::Result<IMDSPStorage> {
        let mut result__: <IMDSPStorage as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).10)(::std::mem::transmute_copy(self), ::std::mem::transmute(dwattributes), ::std::mem::transmute(pformat), pwszname.into_param().abi(), &mut result__).from_abi::<IMDSPStorage>(result__)
    }
    #[doc = "*Required features: `Win32_Media_DeviceManager`*"]
    pub unsafe fn EnumStorage(&self) -> ::windows::runtime::Result<IMDSPEnumStorage> {
        let mut result__: <IMDSPEnumStorage as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).11)(::std::mem::transmute_copy(self), &mut result__).from_abi::<IMDSPEnumStorage>(result__)
    }
    #[doc = "*Required features: `Win32_Media_DeviceManager`*"]
    pub unsafe fn SendOpaqueCommand(&self, pcommand: *mut OPAQUECOMMAND) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).12)(::std::mem::transmute_copy(self), ::std::mem::transmute(pcommand)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_DeviceManager`, `Win32_Foundation`*"]
    pub unsafe fn GetStorage<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, pszstoragename: Param0) -> ::windows::runtime::Result<IMDSPStorage> {
        let mut result__: <IMDSPStorage as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).13)(::std::mem::transmute_copy(self), pszstoragename.into_param().abi(), &mut result__).from_abi::<IMDSPStorage>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_DeviceManager`, `Win32_Foundation`*"]
    pub unsafe fn CreateStorage2<'a, Param4: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, dwattributes: u32, dwattributesex: u32, paudioformat: *const _WAVEFORMATEX, pvideoformat: *const _VIDEOINFOHEADER, pwszname: Param4, qwfilesize: u64) -> ::windows::runtime::Result<IMDSPStorage> {
        let mut result__: <IMDSPStorage as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).14)(::std::mem::transmute_copy(self), ::std::mem::transmute(dwattributes), ::std::mem::transmute(dwattributesex), ::std::mem::transmute(paudioformat), ::std::mem::transmute(pvideoformat), pwszname.into_param().abi(), ::std::mem::transmute(qwfilesize), &mut result__).from_abi::<IMDSPStorage>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_DeviceManager`, `Win32_Foundation`*"]
    pub unsafe fn SetAttributes2(&self, dwattributes: u32, dwattributesex: u32, paudioformat: *const _WAVEFORMATEX, pvideoformat: *const _VIDEOINFOHEADER) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).15)(::std::mem::transmute_copy(self), ::std::mem::transmute(dwattributes), ::std::mem::transmute(dwattributesex), ::std::mem::transmute(paudioformat), ::std::mem::transmute(pvideoformat)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_DeviceManager`, `Win32_Foundation`*"]
    pub unsafe fn GetAttributes2(&self, pdwattributes: *mut u32, pdwattributesex: *mut u32, paudioformat: *mut _WAVEFORMATEX, pvideoformat: *mut _VIDEOINFOHEADER) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).16)(::std::mem::transmute_copy(self), ::std::mem::transmute(pdwattributes), ::std::mem::transmute(pdwattributesex), ::std::mem::transmute(paudioformat), ::std::mem::transmute(pvideoformat)).ok()
    }
    #[doc = "*Required features: `Win32_Media_DeviceManager`*"]
    pub unsafe fn GetMetadata<'a, Param0: ::windows::runtime::IntoParam<'a, IWMDMMetaData>>(&self, pmetadata: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).17)(::std::mem::transmute_copy(self), pmetadata.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_Media_DeviceManager`*"]
    pub unsafe fn SetMetadata<'a, Param0: ::windows::runtime::IntoParam<'a, IWMDMMetaData>>(&self, pmetadata: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).18)(::std::mem::transmute_copy(self), pmetadata.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_Media_DeviceManager`*"]
    pub unsafe fn SetReferences(&self, dwrefs: u32, ppispstorage: *const ::std::option::Option<IMDSPStorage>) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).19)(::std::mem::transmute_copy(self), ::std::mem::transmute(dwrefs), ::std::mem::transmute(ppispstorage)).ok()
    }
    #[doc = "*Required features: `Win32_Media_DeviceManager`*"]
    pub unsafe fn GetReferences(&self, pdwrefs: *mut u32, pppispstorage: *mut *mut ::std::option::Option<IMDSPStorage>) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).20)(::std::mem::transmute_copy(self), ::std::mem::transmute(pdwrefs), ::std::mem::transmute(pppispstorage)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_DeviceManager`, `Win32_Foundation`*"]
    pub unsafe fn CreateStorageWithMetadata<'a, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>, Param2: ::windows::runtime::IntoParam<'a, IWMDMMetaData>>(&self, dwattributes: u32, pwszname: Param1, pmetadata: Param2, qwfilesize: u64) -> ::windows::runtime::Result<IMDSPStorage> {
        let mut result__: <IMDSPStorage as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).21)(::std::mem::transmute_copy(self), ::std::mem::transmute(dwattributes), pwszname.into_param().abi(), pmetadata.into_param().abi(), ::std::mem::transmute(qwfilesize), &mut result__).from_abi::<IMDSPStorage>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_DeviceManager`, `Win32_Foundation`*"]
    pub unsafe fn GetSpecifiedMetadata<'a, Param2: ::windows::runtime::IntoParam<'a, IWMDMMetaData>>(&self, cproperties: u32, ppwszpropnames: *const super::super::Foundation::PWSTR, pmetadata: Param2) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).22)(::std::mem::transmute_copy(self), ::std::mem::transmute(cproperties), ::std::mem::transmute(ppwszpropnames), pmetadata.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_DeviceManager`, `Win32_Foundation`*"]
    pub unsafe fn FindStorage<'a, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, findscope: WMDM_FIND_SCOPE, pwszuniqueid: Param1) -> ::windows::runtime::Result<IMDSPStorage> {
        let mut result__: <IMDSPStorage as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).23)(::std::mem::transmute_copy(self), ::std::mem::transmute(findscope), pwszuniqueid.into_param().abi(), &mut result__).from_abi::<IMDSPStorage>(result__)
    }
    #[doc = "*Required features: `Win32_Media_DeviceManager`*"]
    pub unsafe fn GetParent(&self) -> ::windows::runtime::Result<IMDSPStorage> {
        let mut result__: <IMDSPStorage as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).24)(::std::mem::transmute_copy(self), &mut result__).from_abi::<IMDSPStorage>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IMDSPStorage4 {
    type Vtable = IMDSPStorage4_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(825471684, 20828, 18459, [177, 206, 57, 50, 126, 203, 79, 116]);
}
impl ::std::convert::From<IMDSPStorage4> for ::windows::runtime::IUnknown {
    fn from(value: IMDSPStorage4) -> Self {
        value.0
    }
}
impl ::std::convert::From<&IMDSPStorage4> for ::windows::runtime::IUnknown {
    fn from(value: &IMDSPStorage4) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IMDSPStorage4 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IMDSPStorage4 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::std::convert::From<IMDSPStorage4> for IMDSPStorage3 {
    fn from(value: IMDSPStorage4) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IMDSPStorage4> for IMDSPStorage3 {
    fn from(value: &IMDSPStorage4) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IMDSPStorage3> for IMDSPStorage4 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IMDSPStorage3> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IMDSPStorage3>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IMDSPStorage3> for &IMDSPStorage4 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IMDSPStorage3> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IMDSPStorage3>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<IMDSPStorage4> for IMDSPStorage2 {
    fn from(value: IMDSPStorage4) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IMDSPStorage4> for IMDSPStorage2 {
    fn from(value: &IMDSPStorage4) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IMDSPStorage2> for IMDSPStorage4 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IMDSPStorage2> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IMDSPStorage2>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IMDSPStorage2> for &IMDSPStorage4 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IMDSPStorage2> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IMDSPStorage2>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<IMDSPStorage4> for IMDSPStorage {
    fn from(value: IMDSPStorage4) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IMDSPStorage4> for IMDSPStorage {
    fn from(value: &IMDSPStorage4) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IMDSPStorage> for IMDSPStorage4 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IMDSPStorage> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IMDSPStorage>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IMDSPStorage> for &IMDSPStorage4 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IMDSPStorage> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IMDSPStorage>::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMDSPStorage4_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwattributes: u32, pformat: *const _WAVEFORMATEX) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppstorageglobals: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pdwattributes: *mut u32, pformat: *mut _WAVEFORMATEX) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pwszname: super::super::Foundation::PWSTR, nmaxchars: u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pdatetimeutc: *mut WMDMDATETIME) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pdwsizelow: *mut u32, pdwsizehigh: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pprights: *mut *mut WMDMRIGHTS, pnrightscount: *mut u32, abmac: *mut u8) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwattributes: u32, pformat: *const _WAVEFORMATEX, pwszname: super::super::Foundation::PWSTR, ppnewstorage: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppenumstorage: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pcommand: *mut OPAQUECOMMAND) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pszstoragename: super::super::Foundation::PWSTR, ppstorage: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwattributes: u32, dwattributesex: u32, paudioformat: *const _WAVEFORMATEX, pvideoformat: *const _VIDEOINFOHEADER, pwszname: super::super::Foundation::PWSTR, qwfilesize: u64, ppnewstorage: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwattributes: u32, dwattributesex: u32, paudioformat: *const _WAVEFORMATEX, pvideoformat: *const _VIDEOINFOHEADER) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pdwattributes: *mut u32, pdwattributesex: *mut u32, paudioformat: *mut _WAVEFORMATEX, pvideoformat: *mut _VIDEOINFOHEADER) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pmetadata: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pmetadata: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwrefs: u32, ppispstorage: *const ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pdwrefs: *mut u32, pppispstorage: *mut *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwattributes: u32, pwszname: super::super::Foundation::PWSTR, pmetadata: ::windows::runtime::RawPtr, qwfilesize: u64, ppnewstorage: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, cproperties: u32, ppwszpropnames: *const super::super::Foundation::PWSTR, pmetadata: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, findscope: WMDM_FIND_SCOPE, pwszuniqueid: super::super::Foundation::PWSTR, ppstorage: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppstorage: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_Media_DeviceManager`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IMDSPStorageGlobals(pub ::windows::runtime::IUnknown);
impl IMDSPStorageGlobals {
    #[doc = "*Required features: `Win32_Media_DeviceManager`*"]
    pub unsafe fn GetCapabilities(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `Win32_Media_DeviceManager`*"]
    pub unsafe fn GetSerialNumber(&self, pserialnum: *mut WMDMID, abmac: *mut u8) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), ::std::mem::transmute(pserialnum), ::std::mem::transmute(abmac)).ok()
    }
    #[doc = "*Required features: `Win32_Media_DeviceManager`*"]
    pub unsafe fn GetTotalSize(&self, pdwtotalsizelow: *mut u32, pdwtotalsizehigh: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self), ::std::mem::transmute(pdwtotalsizelow), ::std::mem::transmute(pdwtotalsizehigh)).ok()
    }
    #[doc = "*Required features: `Win32_Media_DeviceManager`*"]
    pub unsafe fn GetTotalFree(&self, pdwfreelow: *mut u32, pdwfreehigh: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::std::mem::transmute_copy(self), ::std::mem::transmute(pdwfreelow), ::std::mem::transmute(pdwfreehigh)).ok()
    }
    #[doc = "*Required features: `Win32_Media_DeviceManager`*"]
    pub unsafe fn GetTotalBad(&self, pdwbadlow: *mut u32, pdwbadhigh: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self), ::std::mem::transmute(pdwbadlow), ::std::mem::transmute(pdwbadhigh)).ok()
    }
    #[doc = "*Required features: `Win32_Media_DeviceManager`*"]
    pub unsafe fn GetStatus(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).8)(::std::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `Win32_Media_DeviceManager`*"]
    pub unsafe fn Initialize<'a, Param1: ::windows::runtime::IntoParam<'a, IWMDMProgress>>(&self, fumode: u32, pprogress: Param1) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).9)(::std::mem::transmute_copy(self), ::std::mem::transmute(fumode), pprogress.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_Media_DeviceManager`*"]
    pub unsafe fn GetDevice(&self) -> ::windows::runtime::Result<IMDSPDevice> {
        let mut result__: <IMDSPDevice as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).10)(::std::mem::transmute_copy(self), &mut result__).from_abi::<IMDSPDevice>(result__)
    }
    #[doc = "*Required features: `Win32_Media_DeviceManager`*"]
    pub unsafe fn GetRootStorage(&self) -> ::windows::runtime::Result<IMDSPStorage> {
        let mut result__: <IMDSPStorage as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).11)(::std::mem::transmute_copy(self), &mut result__).from_abi::<IMDSPStorage>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IMDSPStorageGlobals {
    type Vtable = IMDSPStorageGlobals_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(499857943, 13293, 4563, [132, 112, 0, 192, 79, 121, 219, 192]);
}
impl ::std::convert::From<IMDSPStorageGlobals> for ::windows::runtime::IUnknown {
    fn from(value: IMDSPStorageGlobals) -> Self {
        value.0
    }
}
impl ::std::convert::From<&IMDSPStorageGlobals> for ::windows::runtime::IUnknown {
    fn from(value: &IMDSPStorageGlobals) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IMDSPStorageGlobals {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IMDSPStorageGlobals {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMDSPStorageGlobals_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pdwcapabilities: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pserialnum: *mut WMDMID, abmac: *mut u8) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pdwtotalsizelow: *mut u32, pdwtotalsizehigh: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pdwfreelow: *mut u32, pdwfreehigh: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pdwbadlow: *mut u32, pdwbadhigh: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pdwstatus: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, fumode: u32, pprogress: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppdevice: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pproot: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_Media_DeviceManager`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IMDServiceProvider(pub ::windows::runtime::IUnknown);
impl IMDServiceProvider {
    #[doc = "*Required features: `Win32_Media_DeviceManager`*"]
    pub unsafe fn GetDeviceCount(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `Win32_Media_DeviceManager`*"]
    pub unsafe fn EnumDevices(&self) -> ::windows::runtime::Result<IMDSPEnumDevice> {
        let mut result__: <IMDSPEnumDevice as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), &mut result__).from_abi::<IMDSPEnumDevice>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IMDServiceProvider {
    type Vtable = IMDServiceProvider_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(499857936, 13293, 4563, [132, 112, 0, 192, 79, 121, 219, 192]);
}
impl ::std::convert::From<IMDServiceProvider> for ::windows::runtime::IUnknown {
    fn from(value: IMDServiceProvider) -> Self {
        value.0
    }
}
impl ::std::convert::From<&IMDServiceProvider> for ::windows::runtime::IUnknown {
    fn from(value: &IMDServiceProvider) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IMDServiceProvider {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IMDServiceProvider {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMDServiceProvider_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pdwcount: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppenumdevice: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_Media_DeviceManager`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IMDServiceProvider2(pub ::windows::runtime::IUnknown);
impl IMDServiceProvider2 {
    #[doc = "*Required features: `Win32_Media_DeviceManager`*"]
    pub unsafe fn GetDeviceCount(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `Win32_Media_DeviceManager`*"]
    pub unsafe fn EnumDevices(&self) -> ::windows::runtime::Result<IMDSPEnumDevice> {
        let mut result__: <IMDSPEnumDevice as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), &mut result__).from_abi::<IMDSPEnumDevice>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_DeviceManager`, `Win32_Foundation`*"]
    pub unsafe fn CreateDevice<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, pwszdevicepath: Param0, pdwcount: *mut u32, pppdevicearray: *mut *mut ::std::option::Option<IMDSPDevice>) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self), pwszdevicepath.into_param().abi(), ::std::mem::transmute(pdwcount), ::std::mem::transmute(pppdevicearray)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IMDServiceProvider2 {
    type Vtable = IMDServiceProvider2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3002737847, 52643, 18068, [152, 98, 65, 58, 225, 163, 72, 25]);
}
impl ::std::convert::From<IMDServiceProvider2> for ::windows::runtime::IUnknown {
    fn from(value: IMDServiceProvider2) -> Self {
        value.0
    }
}
impl ::std::convert::From<&IMDServiceProvider2> for ::windows::runtime::IUnknown {
    fn from(value: &IMDServiceProvider2) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IMDServiceProvider2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IMDServiceProvider2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::std::convert::From<IMDServiceProvider2> for IMDServiceProvider {
    fn from(value: IMDServiceProvider2) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IMDServiceProvider2> for IMDServiceProvider {
    fn from(value: &IMDServiceProvider2) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IMDServiceProvider> for IMDServiceProvider2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IMDServiceProvider> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IMDServiceProvider>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IMDServiceProvider> for &IMDServiceProvider2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IMDServiceProvider> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IMDServiceProvider>::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMDServiceProvider2_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pdwcount: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppenumdevice: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pwszdevicepath: super::super::Foundation::PWSTR, pdwcount: *mut u32, pppdevicearray: *mut *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[doc = "*Required features: `Win32_Media_DeviceManager`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IMDServiceProvider3(pub ::windows::runtime::IUnknown);
impl IMDServiceProvider3 {
    #[doc = "*Required features: `Win32_Media_DeviceManager`*"]
    pub unsafe fn GetDeviceCount(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `Win32_Media_DeviceManager`*"]
    pub unsafe fn EnumDevices(&self) -> ::windows::runtime::Result<IMDSPEnumDevice> {
        let mut result__: <IMDSPEnumDevice as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), &mut result__).from_abi::<IMDSPEnumDevice>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_DeviceManager`, `Win32_Foundation`*"]
    pub unsafe fn CreateDevice<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, pwszdevicepath: Param0, pdwcount: *mut u32, pppdevicearray: *mut *mut ::std::option::Option<IMDSPDevice>) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self), pwszdevicepath.into_param().abi(), ::std::mem::transmute(pdwcount), ::std::mem::transmute(pppdevicearray)).ok()
    }
    #[doc = "*Required features: `Win32_Media_DeviceManager`*"]
    pub unsafe fn SetDeviceEnumPreference(&self, dwenumpref: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::std::mem::transmute_copy(self), ::std::mem::transmute(dwenumpref)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IMDServiceProvider3 {
    type Vtable = IMDServiceProvider3_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1322335987, 43377, 19737, [159, 81, 14, 24, 38, 178, 218, 87]);
}
impl ::std::convert::From<IMDServiceProvider3> for ::windows::runtime::IUnknown {
    fn from(value: IMDServiceProvider3) -> Self {
        value.0
    }
}
impl ::std::convert::From<&IMDServiceProvider3> for ::windows::runtime::IUnknown {
    fn from(value: &IMDServiceProvider3) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IMDServiceProvider3 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IMDServiceProvider3 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::std::convert::From<IMDServiceProvider3> for IMDServiceProvider2 {
    fn from(value: IMDServiceProvider3) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IMDServiceProvider3> for IMDServiceProvider2 {
    fn from(value: &IMDServiceProvider3) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IMDServiceProvider2> for IMDServiceProvider3 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IMDServiceProvider2> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IMDServiceProvider2>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IMDServiceProvider2> for &IMDServiceProvider3 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IMDServiceProvider2> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IMDServiceProvider2>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<IMDServiceProvider3> for IMDServiceProvider {
    fn from(value: IMDServiceProvider3) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IMDServiceProvider3> for IMDServiceProvider {
    fn from(value: &IMDServiceProvider3) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IMDServiceProvider> for IMDServiceProvider3 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IMDServiceProvider> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IMDServiceProvider>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IMDServiceProvider> for &IMDServiceProvider3 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IMDServiceProvider> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IMDServiceProvider>::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMDServiceProvider3_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pdwcount: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppenumdevice: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pwszdevicepath: super::super::Foundation::PWSTR, pdwcount: *mut u32, pppdevicearray: *mut *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwenumpref: u32) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_Media_DeviceManager`*"]
pub const IOCTL_MTP_CUSTOM_COMMAND: u32 = 827348045u32;
#[doc = "*Required features: `Win32_Media_DeviceManager`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct ISCPSecureAuthenticate(pub ::windows::runtime::IUnknown);
impl ISCPSecureAuthenticate {
    #[doc = "*Required features: `Win32_Media_DeviceManager`*"]
    pub unsafe fn GetSecureQuery(&self) -> ::windows::runtime::Result<ISCPSecureQuery> {
        let mut result__: <ISCPSecureQuery as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), &mut result__).from_abi::<ISCPSecureQuery>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for ISCPSecureAuthenticate {
    type Vtable = ISCPSecureAuthenticate_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(499857935, 13293, 4563, [132, 112, 0, 192, 79, 121, 219, 192]);
}
impl ::std::convert::From<ISCPSecureAuthenticate> for ::windows::runtime::IUnknown {
    fn from(value: ISCPSecureAuthenticate) -> Self {
        value.0
    }
}
impl ::std::convert::From<&ISCPSecureAuthenticate> for ::windows::runtime::IUnknown {
    fn from(value: &ISCPSecureAuthenticate) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ISCPSecureAuthenticate {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a ISCPSecureAuthenticate {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ISCPSecureAuthenticate_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppsecurequery: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_Media_DeviceManager`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct ISCPSecureAuthenticate2(pub ::windows::runtime::IUnknown);
impl ISCPSecureAuthenticate2 {
    #[doc = "*Required features: `Win32_Media_DeviceManager`*"]
    pub unsafe fn GetSecureQuery(&self) -> ::windows::runtime::Result<ISCPSecureQuery> {
        let mut result__: <ISCPSecureQuery as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), &mut result__).from_abi::<ISCPSecureQuery>(result__)
    }
    #[doc = "*Required features: `Win32_Media_DeviceManager`*"]
    pub unsafe fn GetSCPSession(&self) -> ::windows::runtime::Result<ISCPSession> {
        let mut result__: <ISCPSession as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), &mut result__).from_abi::<ISCPSession>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for ISCPSecureAuthenticate2 {
    type Vtable = ISCPSecureAuthenticate2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3045117870, 5746, 18402, [172, 170, 68, 187, 236, 188, 174, 91]);
}
impl ::std::convert::From<ISCPSecureAuthenticate2> for ::windows::runtime::IUnknown {
    fn from(value: ISCPSecureAuthenticate2) -> Self {
        value.0
    }
}
impl ::std::convert::From<&ISCPSecureAuthenticate2> for ::windows::runtime::IUnknown {
    fn from(value: &ISCPSecureAuthenticate2) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ISCPSecureAuthenticate2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a ISCPSecureAuthenticate2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::std::convert::From<ISCPSecureAuthenticate2> for ISCPSecureAuthenticate {
    fn from(value: ISCPSecureAuthenticate2) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&ISCPSecureAuthenticate2> for ISCPSecureAuthenticate {
    fn from(value: &ISCPSecureAuthenticate2) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ISCPSecureAuthenticate> for ISCPSecureAuthenticate2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ISCPSecureAuthenticate> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<ISCPSecureAuthenticate>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ISCPSecureAuthenticate> for &ISCPSecureAuthenticate2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ISCPSecureAuthenticate> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<ISCPSecureAuthenticate>::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ISCPSecureAuthenticate2_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppsecurequery: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppscpsession: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_Media_DeviceManager`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct ISCPSecureExchange(pub ::windows::runtime::IUnknown);
impl ISCPSecureExchange {
    #[doc = "*Required features: `Win32_Media_DeviceManager`*"]
    pub unsafe fn TransferContainerData(&self, pdata: *const u8, dwsize: u32, pfureadyflags: *mut u32, abmac: *mut u8) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), ::std::mem::transmute(pdata), ::std::mem::transmute(dwsize), ::std::mem::transmute(pfureadyflags), ::std::mem::transmute(abmac)).ok()
    }
    #[doc = "*Required features: `Win32_Media_DeviceManager`*"]
    pub unsafe fn ObjectData(&self, pdata: *mut u8, pdwsize: *mut u32, abmac: *mut u8) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), ::std::mem::transmute(pdata), ::std::mem::transmute(pdwsize), ::std::mem::transmute(abmac)).ok()
    }
    #[doc = "*Required features: `Win32_Media_DeviceManager`*"]
    pub unsafe fn TransferComplete(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for ISCPSecureExchange {
    type Vtable = ISCPSecureExchange_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(499857934, 13293, 4563, [132, 112, 0, 192, 79, 121, 219, 192]);
}
impl ::std::convert::From<ISCPSecureExchange> for ::windows::runtime::IUnknown {
    fn from(value: ISCPSecureExchange) -> Self {
        value.0
    }
}
impl ::std::convert::From<&ISCPSecureExchange> for ::windows::runtime::IUnknown {
    fn from(value: &ISCPSecureExchange) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ISCPSecureExchange {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a ISCPSecureExchange {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ISCPSecureExchange_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pdata: *const u8, dwsize: u32, pfureadyflags: *mut u32, abmac: *mut u8) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pdata: *mut u8, pdwsize: *mut u32, abmac: *mut u8) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_Media_DeviceManager`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct ISCPSecureExchange2(pub ::windows::runtime::IUnknown);
impl ISCPSecureExchange2 {
    #[doc = "*Required features: `Win32_Media_DeviceManager`*"]
    pub unsafe fn TransferContainerData(&self, pdata: *const u8, dwsize: u32, pfureadyflags: *mut u32, abmac: *mut u8) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), ::std::mem::transmute(pdata), ::std::mem::transmute(dwsize), ::std::mem::transmute(pfureadyflags), ::std::mem::transmute(abmac)).ok()
    }
    #[doc = "*Required features: `Win32_Media_DeviceManager`*"]
    pub unsafe fn ObjectData(&self, pdata: *mut u8, pdwsize: *mut u32, abmac: *mut u8) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), ::std::mem::transmute(pdata), ::std::mem::transmute(pdwsize), ::std::mem::transmute(abmac)).ok()
    }
    #[doc = "*Required features: `Win32_Media_DeviceManager`*"]
    pub unsafe fn TransferComplete(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `Win32_Media_DeviceManager`*"]
    pub unsafe fn TransferContainerData2<'a, Param2: ::windows::runtime::IntoParam<'a, IWMDMProgress3>>(&self, pdata: *const u8, dwsize: u32, pprogresscallback: Param2, pfureadyflags: *mut u32, abmac: *mut u8) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::std::mem::transmute_copy(self), ::std::mem::transmute(pdata), ::std::mem::transmute(dwsize), pprogresscallback.into_param().abi(), ::std::mem::transmute(pfureadyflags), ::std::mem::transmute(abmac)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for ISCPSecureExchange2 {
    type Vtable = ISCPSecureExchange2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1818426491, 9872, 18495, [157, 68, 10, 32, 203, 53, 87, 124]);
}
impl ::std::convert::From<ISCPSecureExchange2> for ::windows::runtime::IUnknown {
    fn from(value: ISCPSecureExchange2) -> Self {
        value.0
    }
}
impl ::std::convert::From<&ISCPSecureExchange2> for ::windows::runtime::IUnknown {
    fn from(value: &ISCPSecureExchange2) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ISCPSecureExchange2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a ISCPSecureExchange2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::std::convert::From<ISCPSecureExchange2> for ISCPSecureExchange {
    fn from(value: ISCPSecureExchange2) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&ISCPSecureExchange2> for ISCPSecureExchange {
    fn from(value: &ISCPSecureExchange2) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ISCPSecureExchange> for ISCPSecureExchange2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ISCPSecureExchange> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<ISCPSecureExchange>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ISCPSecureExchange> for &ISCPSecureExchange2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ISCPSecureExchange> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<ISCPSecureExchange>::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ISCPSecureExchange2_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pdata: *const u8, dwsize: u32, pfureadyflags: *mut u32, abmac: *mut u8) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pdata: *mut u8, pdwsize: *mut u32, abmac: *mut u8) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pdata: *const u8, dwsize: u32, pprogresscallback: ::windows::runtime::RawPtr, pfureadyflags: *mut u32, abmac: *mut u8) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_Media_DeviceManager`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct ISCPSecureExchange3(pub ::windows::runtime::IUnknown);
impl ISCPSecureExchange3 {
    #[doc = "*Required features: `Win32_Media_DeviceManager`*"]
    pub unsafe fn TransferContainerData(&self, pdata: *const u8, dwsize: u32, pfureadyflags: *mut u32, abmac: *mut u8) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), ::std::mem::transmute(pdata), ::std::mem::transmute(dwsize), ::std::mem::transmute(pfureadyflags), ::std::mem::transmute(abmac)).ok()
    }
    #[doc = "*Required features: `Win32_Media_DeviceManager`*"]
    pub unsafe fn ObjectData(&self, pdata: *mut u8, pdwsize: *mut u32, abmac: *mut u8) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), ::std::mem::transmute(pdata), ::std::mem::transmute(pdwsize), ::std::mem::transmute(abmac)).ok()
    }
    #[doc = "*Required features: `Win32_Media_DeviceManager`*"]
    pub unsafe fn TransferComplete(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `Win32_Media_DeviceManager`*"]
    pub unsafe fn TransferContainerData2<'a, Param2: ::windows::runtime::IntoParam<'a, IWMDMProgress3>>(&self, pdata: *const u8, dwsize: u32, pprogresscallback: Param2, pfureadyflags: *mut u32, abmac: *mut u8) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::std::mem::transmute_copy(self), ::std::mem::transmute(pdata), ::std::mem::transmute(dwsize), pprogresscallback.into_param().abi(), ::std::mem::transmute(pfureadyflags), ::std::mem::transmute(abmac)).ok()
    }
    #[doc = "*Required features: `Win32_Media_DeviceManager`*"]
    pub unsafe fn TransferContainerDataOnClearChannel<'a, Param0: ::windows::runtime::IntoParam<'a, IMDSPDevice>, Param3: ::windows::runtime::IntoParam<'a, IWMDMProgress3>>(&self, pdevice: Param0, pdata: *const u8, dwsize: u32, pprogresscallback: Param3) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self), pdevice.into_param().abi(), ::std::mem::transmute(pdata), ::std::mem::transmute(dwsize), pprogresscallback.into_param().abi(), &mut result__).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `Win32_Media_DeviceManager`*"]
    pub unsafe fn GetObjectDataOnClearChannel<'a, Param0: ::windows::runtime::IntoParam<'a, IMDSPDevice>>(&self, pdevice: Param0, pdata: *mut u8, pdwsize: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).8)(::std::mem::transmute_copy(self), pdevice.into_param().abi(), ::std::mem::transmute(pdata), ::std::mem::transmute(pdwsize)).ok()
    }
    #[doc = "*Required features: `Win32_Media_DeviceManager`*"]
    pub unsafe fn TransferCompleteForDevice<'a, Param0: ::windows::runtime::IntoParam<'a, IMDSPDevice>>(&self, pdevice: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).9)(::std::mem::transmute_copy(self), pdevice.into_param().abi()).ok()
    }
}
unsafe impl ::windows::runtime::Interface for ISCPSecureExchange3 {
    type Vtable = ISCPSecureExchange3_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2874046436, 35080, 19223, [189, 42, 177, 219, 230, 221, 105, 225]);
}
impl ::std::convert::From<ISCPSecureExchange3> for ::windows::runtime::IUnknown {
    fn from(value: ISCPSecureExchange3) -> Self {
        value.0
    }
}
impl ::std::convert::From<&ISCPSecureExchange3> for ::windows::runtime::IUnknown {
    fn from(value: &ISCPSecureExchange3) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ISCPSecureExchange3 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a ISCPSecureExchange3 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::std::convert::From<ISCPSecureExchange3> for ISCPSecureExchange2 {
    fn from(value: ISCPSecureExchange3) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&ISCPSecureExchange3> for ISCPSecureExchange2 {
    fn from(value: &ISCPSecureExchange3) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ISCPSecureExchange2> for ISCPSecureExchange3 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ISCPSecureExchange2> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<ISCPSecureExchange2>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ISCPSecureExchange2> for &ISCPSecureExchange3 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ISCPSecureExchange2> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<ISCPSecureExchange2>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<ISCPSecureExchange3> for ISCPSecureExchange {
    fn from(value: ISCPSecureExchange3) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&ISCPSecureExchange3> for ISCPSecureExchange {
    fn from(value: &ISCPSecureExchange3) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ISCPSecureExchange> for ISCPSecureExchange3 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ISCPSecureExchange> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<ISCPSecureExchange>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ISCPSecureExchange> for &ISCPSecureExchange3 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ISCPSecureExchange> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<ISCPSecureExchange>::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ISCPSecureExchange3_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pdata: *const u8, dwsize: u32, pfureadyflags: *mut u32, abmac: *mut u8) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pdata: *mut u8, pdwsize: *mut u32, abmac: *mut u8) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pdata: *const u8, dwsize: u32, pprogresscallback: ::windows::runtime::RawPtr, pfureadyflags: *mut u32, abmac: *mut u8) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pdevice: ::windows::runtime::RawPtr, pdata: *const u8, dwsize: u32, pprogresscallback: ::windows::runtime::RawPtr, pfureadyflags: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pdevice: ::windows::runtime::RawPtr, pdata: *mut u8, pdwsize: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pdevice: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_Media_DeviceManager`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct ISCPSecureQuery(pub ::windows::runtime::IUnknown);
impl ISCPSecureQuery {
    #[doc = "*Required features: `Win32_Media_DeviceManager`*"]
    pub unsafe fn GetDataDemands(&self, pfuflags: *mut u32, pdwminrightsdata: *mut u32, pdwminexaminedata: *mut u32, pdwmindecidedata: *mut u32, abmac: *mut u8) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), ::std::mem::transmute(pfuflags), ::std::mem::transmute(pdwminrightsdata), ::std::mem::transmute(pdwminexaminedata), ::std::mem::transmute(pdwmindecidedata), ::std::mem::transmute(abmac)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_DeviceManager`, `Win32_Foundation`*"]
    pub unsafe fn ExamineData<'a, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, fuflags: u32, pwszextension: Param1, pdata: *const u8, dwsize: u32, abmac: *mut u8) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), ::std::mem::transmute(fuflags), pwszextension.into_param().abi(), ::std::mem::transmute(pdata), ::std::mem::transmute(dwsize), ::std::mem::transmute(abmac)).ok()
    }
    #[doc = "*Required features: `Win32_Media_DeviceManager`*"]
    pub unsafe fn MakeDecision<'a, Param6: ::windows::runtime::IntoParam<'a, IMDSPStorageGlobals>>(&self, fuflags: u32, pdata: *const u8, dwsize: u32, dwappsec: u32, pbspsessionkey: *const u8, dwsessionkeylen: u32, pstorageglobals: Param6, ppexchange: *mut ::std::option::Option<ISCPSecureExchange>, abmac: *mut u8) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(fuflags),
            ::std::mem::transmute(pdata),
            ::std::mem::transmute(dwsize),
            ::std::mem::transmute(dwappsec),
            ::std::mem::transmute(pbspsessionkey),
            ::std::mem::transmute(dwsessionkeylen),
            pstorageglobals.into_param().abi(),
            ::std::mem::transmute(ppexchange),
            ::std::mem::transmute(abmac),
        )
        .ok()
    }
    #[doc = "*Required features: `Win32_Media_DeviceManager`*"]
    pub unsafe fn GetRights<'a, Param4: ::windows::runtime::IntoParam<'a, IMDSPStorageGlobals>>(&self, pdata: *const u8, dwsize: u32, pbspsessionkey: *const u8, dwsessionkeylen: u32, pstgglobals: Param4, pprights: *mut *mut WMDMRIGHTS, pnrightscount: *mut u32, abmac: *mut u8) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(pdata),
            ::std::mem::transmute(dwsize),
            ::std::mem::transmute(pbspsessionkey),
            ::std::mem::transmute(dwsessionkeylen),
            pstgglobals.into_param().abi(),
            ::std::mem::transmute(pprights),
            ::std::mem::transmute(pnrightscount),
            ::std::mem::transmute(abmac),
        )
        .ok()
    }
}
unsafe impl ::windows::runtime::Interface for ISCPSecureQuery {
    type Vtable = ISCPSecureQuery_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(499857933, 13293, 4563, [132, 112, 0, 192, 79, 121, 219, 192]);
}
impl ::std::convert::From<ISCPSecureQuery> for ::windows::runtime::IUnknown {
    fn from(value: ISCPSecureQuery) -> Self {
        value.0
    }
}
impl ::std::convert::From<&ISCPSecureQuery> for ::windows::runtime::IUnknown {
    fn from(value: &ISCPSecureQuery) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ISCPSecureQuery {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a ISCPSecureQuery {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ISCPSecureQuery_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pfuflags: *mut u32, pdwminrightsdata: *mut u32, pdwminexaminedata: *mut u32, pdwmindecidedata: *mut u32, abmac: *mut u8) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, fuflags: u32, pwszextension: super::super::Foundation::PWSTR, pdata: *const u8, dwsize: u32, abmac: *mut u8) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, fuflags: u32, pdata: *const u8, dwsize: u32, dwappsec: u32, pbspsessionkey: *const u8, dwsessionkeylen: u32, pstorageglobals: ::windows::runtime::RawPtr, ppexchange: *mut ::windows::runtime::RawPtr, abmac: *mut u8) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pdata: *const u8, dwsize: u32, pbspsessionkey: *const u8, dwsessionkeylen: u32, pstgglobals: ::windows::runtime::RawPtr, pprights: *mut *mut WMDMRIGHTS, pnrightscount: *mut u32, abmac: *mut u8) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_Media_DeviceManager`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct ISCPSecureQuery2(pub ::windows::runtime::IUnknown);
impl ISCPSecureQuery2 {
    #[doc = "*Required features: `Win32_Media_DeviceManager`*"]
    pub unsafe fn GetDataDemands(&self, pfuflags: *mut u32, pdwminrightsdata: *mut u32, pdwminexaminedata: *mut u32, pdwmindecidedata: *mut u32, abmac: *mut u8) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), ::std::mem::transmute(pfuflags), ::std::mem::transmute(pdwminrightsdata), ::std::mem::transmute(pdwminexaminedata), ::std::mem::transmute(pdwmindecidedata), ::std::mem::transmute(abmac)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_DeviceManager`, `Win32_Foundation`*"]
    pub unsafe fn ExamineData<'a, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, fuflags: u32, pwszextension: Param1, pdata: *const u8, dwsize: u32, abmac: *mut u8) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), ::std::mem::transmute(fuflags), pwszextension.into_param().abi(), ::std::mem::transmute(pdata), ::std::mem::transmute(dwsize), ::std::mem::transmute(abmac)).ok()
    }
    #[doc = "*Required features: `Win32_Media_DeviceManager`*"]
    pub unsafe fn MakeDecision<'a, Param6: ::windows::runtime::IntoParam<'a, IMDSPStorageGlobals>>(&self, fuflags: u32, pdata: *const u8, dwsize: u32, dwappsec: u32, pbspsessionkey: *const u8, dwsessionkeylen: u32, pstorageglobals: Param6, ppexchange: *mut ::std::option::Option<ISCPSecureExchange>, abmac: *mut u8) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(fuflags),
            ::std::mem::transmute(pdata),
            ::std::mem::transmute(dwsize),
            ::std::mem::transmute(dwappsec),
            ::std::mem::transmute(pbspsessionkey),
            ::std::mem::transmute(dwsessionkeylen),
            pstorageglobals.into_param().abi(),
            ::std::mem::transmute(ppexchange),
            ::std::mem::transmute(abmac),
        )
        .ok()
    }
    #[doc = "*Required features: `Win32_Media_DeviceManager`*"]
    pub unsafe fn GetRights<'a, Param4: ::windows::runtime::IntoParam<'a, IMDSPStorageGlobals>>(&self, pdata: *const u8, dwsize: u32, pbspsessionkey: *const u8, dwsessionkeylen: u32, pstgglobals: Param4, pprights: *mut *mut WMDMRIGHTS, pnrightscount: *mut u32, abmac: *mut u8) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(pdata),
            ::std::mem::transmute(dwsize),
            ::std::mem::transmute(pbspsessionkey),
            ::std::mem::transmute(dwsessionkeylen),
            pstgglobals.into_param().abi(),
            ::std::mem::transmute(pprights),
            ::std::mem::transmute(pnrightscount),
            ::std::mem::transmute(abmac),
        )
        .ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_DeviceManager`, `Win32_Foundation`*"]
    pub unsafe fn MakeDecision2<'a, Param6: ::windows::runtime::IntoParam<'a, IMDSPStorageGlobals>, Param15: ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>>(
        &self,
        fuflags: u32,
        pdata: *const u8,
        dwsize: u32,
        dwappsec: u32,
        pbspsessionkey: *const u8,
        dwsessionkeylen: u32,
        pstorageglobals: Param6,
        pappcertapp: *const u8,
        dwappcertapplen: u32,
        pappcertsp: *const u8,
        dwappcertsplen: u32,
        pszrevocationurl: *mut super::super::Foundation::PWSTR,
        pdwrevocationurllen: *mut u32,
        pdwrevocationbitflag: *mut u32,
        pqwfilesize: *mut u64,
        punknown: Param15,
        ppexchange: *mut ::std::option::Option<ISCPSecureExchange>,
        abmac: *mut u8,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).7)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(fuflags),
            ::std::mem::transmute(pdata),
            ::std::mem::transmute(dwsize),
            ::std::mem::transmute(dwappsec),
            ::std::mem::transmute(pbspsessionkey),
            ::std::mem::transmute(dwsessionkeylen),
            pstorageglobals.into_param().abi(),
            ::std::mem::transmute(pappcertapp),
            ::std::mem::transmute(dwappcertapplen),
            ::std::mem::transmute(pappcertsp),
            ::std::mem::transmute(dwappcertsplen),
            ::std::mem::transmute(pszrevocationurl),
            ::std::mem::transmute(pdwrevocationurllen),
            ::std::mem::transmute(pdwrevocationbitflag),
            ::std::mem::transmute(pqwfilesize),
            punknown.into_param().abi(),
            ::std::mem::transmute(ppexchange),
            ::std::mem::transmute(abmac),
        )
        .ok()
    }
}
unsafe impl ::windows::runtime::Interface for ISCPSecureQuery2 {
    type Vtable = ISCPSecureQuery2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3957423653, 20439, 17970, [175, 70, 109, 147, 212, 252, 199, 46]);
}
impl ::std::convert::From<ISCPSecureQuery2> for ::windows::runtime::IUnknown {
    fn from(value: ISCPSecureQuery2) -> Self {
        value.0
    }
}
impl ::std::convert::From<&ISCPSecureQuery2> for ::windows::runtime::IUnknown {
    fn from(value: &ISCPSecureQuery2) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ISCPSecureQuery2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a ISCPSecureQuery2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::std::convert::From<ISCPSecureQuery2> for ISCPSecureQuery {
    fn from(value: ISCPSecureQuery2) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&ISCPSecureQuery2> for ISCPSecureQuery {
    fn from(value: &ISCPSecureQuery2) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ISCPSecureQuery> for ISCPSecureQuery2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ISCPSecureQuery> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<ISCPSecureQuery>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ISCPSecureQuery> for &ISCPSecureQuery2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ISCPSecureQuery> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<ISCPSecureQuery>::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ISCPSecureQuery2_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pfuflags: *mut u32, pdwminrightsdata: *mut u32, pdwminexaminedata: *mut u32, pdwmindecidedata: *mut u32, abmac: *mut u8) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, fuflags: u32, pwszextension: super::super::Foundation::PWSTR, pdata: *const u8, dwsize: u32, abmac: *mut u8) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, fuflags: u32, pdata: *const u8, dwsize: u32, dwappsec: u32, pbspsessionkey: *const u8, dwsessionkeylen: u32, pstorageglobals: ::windows::runtime::RawPtr, ppexchange: *mut ::windows::runtime::RawPtr, abmac: *mut u8) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pdata: *const u8, dwsize: u32, pbspsessionkey: *const u8, dwsessionkeylen: u32, pstgglobals: ::windows::runtime::RawPtr, pprights: *mut *mut WMDMRIGHTS, pnrightscount: *mut u32, abmac: *mut u8) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        fuflags: u32,
        pdata: *const u8,
        dwsize: u32,
        dwappsec: u32,
        pbspsessionkey: *const u8,
        dwsessionkeylen: u32,
        pstorageglobals: ::windows::runtime::RawPtr,
        pappcertapp: *const u8,
        dwappcertapplen: u32,
        pappcertsp: *const u8,
        dwappcertsplen: u32,
        pszrevocationurl: *mut super::super::Foundation::PWSTR,
        pdwrevocationurllen: *mut u32,
        pdwrevocationbitflag: *mut u32,
        pqwfilesize: *mut u64,
        punknown: ::windows::runtime::RawPtr,
        ppexchange: *mut ::windows::runtime::RawPtr,
        abmac: *mut u8,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[doc = "*Required features: `Win32_Media_DeviceManager`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct ISCPSecureQuery3(pub ::windows::runtime::IUnknown);
impl ISCPSecureQuery3 {
    #[doc = "*Required features: `Win32_Media_DeviceManager`*"]
    pub unsafe fn GetDataDemands(&self, pfuflags: *mut u32, pdwminrightsdata: *mut u32, pdwminexaminedata: *mut u32, pdwmindecidedata: *mut u32, abmac: *mut u8) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), ::std::mem::transmute(pfuflags), ::std::mem::transmute(pdwminrightsdata), ::std::mem::transmute(pdwminexaminedata), ::std::mem::transmute(pdwmindecidedata), ::std::mem::transmute(abmac)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_DeviceManager`, `Win32_Foundation`*"]
    pub unsafe fn ExamineData<'a, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, fuflags: u32, pwszextension: Param1, pdata: *const u8, dwsize: u32, abmac: *mut u8) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), ::std::mem::transmute(fuflags), pwszextension.into_param().abi(), ::std::mem::transmute(pdata), ::std::mem::transmute(dwsize), ::std::mem::transmute(abmac)).ok()
    }
    #[doc = "*Required features: `Win32_Media_DeviceManager`*"]
    pub unsafe fn MakeDecision<'a, Param6: ::windows::runtime::IntoParam<'a, IMDSPStorageGlobals>>(&self, fuflags: u32, pdata: *const u8, dwsize: u32, dwappsec: u32, pbspsessionkey: *const u8, dwsessionkeylen: u32, pstorageglobals: Param6, ppexchange: *mut ::std::option::Option<ISCPSecureExchange>, abmac: *mut u8) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(fuflags),
            ::std::mem::transmute(pdata),
            ::std::mem::transmute(dwsize),
            ::std::mem::transmute(dwappsec),
            ::std::mem::transmute(pbspsessionkey),
            ::std::mem::transmute(dwsessionkeylen),
            pstorageglobals.into_param().abi(),
            ::std::mem::transmute(ppexchange),
            ::std::mem::transmute(abmac),
        )
        .ok()
    }
    #[doc = "*Required features: `Win32_Media_DeviceManager`*"]
    pub unsafe fn GetRights<'a, Param4: ::windows::runtime::IntoParam<'a, IMDSPStorageGlobals>>(&self, pdata: *const u8, dwsize: u32, pbspsessionkey: *const u8, dwsessionkeylen: u32, pstgglobals: Param4, pprights: *mut *mut WMDMRIGHTS, pnrightscount: *mut u32, abmac: *mut u8) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(pdata),
            ::std::mem::transmute(dwsize),
            ::std::mem::transmute(pbspsessionkey),
            ::std::mem::transmute(dwsessionkeylen),
            pstgglobals.into_param().abi(),
            ::std::mem::transmute(pprights),
            ::std::mem::transmute(pnrightscount),
            ::std::mem::transmute(abmac),
        )
        .ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_DeviceManager`, `Win32_Foundation`*"]
    pub unsafe fn MakeDecision2<'a, Param6: ::windows::runtime::IntoParam<'a, IMDSPStorageGlobals>, Param15: ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>>(
        &self,
        fuflags: u32,
        pdata: *const u8,
        dwsize: u32,
        dwappsec: u32,
        pbspsessionkey: *const u8,
        dwsessionkeylen: u32,
        pstorageglobals: Param6,
        pappcertapp: *const u8,
        dwappcertapplen: u32,
        pappcertsp: *const u8,
        dwappcertsplen: u32,
        pszrevocationurl: *mut super::super::Foundation::PWSTR,
        pdwrevocationurllen: *mut u32,
        pdwrevocationbitflag: *mut u32,
        pqwfilesize: *mut u64,
        punknown: Param15,
        ppexchange: *mut ::std::option::Option<ISCPSecureExchange>,
        abmac: *mut u8,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).7)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(fuflags),
            ::std::mem::transmute(pdata),
            ::std::mem::transmute(dwsize),
            ::std::mem::transmute(dwappsec),
            ::std::mem::transmute(pbspsessionkey),
            ::std::mem::transmute(dwsessionkeylen),
            pstorageglobals.into_param().abi(),
            ::std::mem::transmute(pappcertapp),
            ::std::mem::transmute(dwappcertapplen),
            ::std::mem::transmute(pappcertsp),
            ::std::mem::transmute(dwappcertsplen),
            ::std::mem::transmute(pszrevocationurl),
            ::std::mem::transmute(pdwrevocationurllen),
            ::std::mem::transmute(pdwrevocationbitflag),
            ::std::mem::transmute(pqwfilesize),
            punknown.into_param().abi(),
            ::std::mem::transmute(ppexchange),
            ::std::mem::transmute(abmac),
        )
        .ok()
    }
    #[doc = "*Required features: `Win32_Media_DeviceManager`*"]
    pub unsafe fn GetRightsOnClearChannel<'a, Param4: ::windows::runtime::IntoParam<'a, IMDSPStorageGlobals>, Param5: ::windows::runtime::IntoParam<'a, IWMDMProgress3>>(&self, pdata: *const u8, dwsize: u32, pbspsessionkey: *const u8, dwsessionkeylen: u32, pstgglobals: Param4, pprogresscallback: Param5, pprights: *mut *mut WMDMRIGHTS, pnrightscount: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).8)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(pdata),
            ::std::mem::transmute(dwsize),
            ::std::mem::transmute(pbspsessionkey),
            ::std::mem::transmute(dwsessionkeylen),
            pstgglobals.into_param().abi(),
            pprogresscallback.into_param().abi(),
            ::std::mem::transmute(pprights),
            ::std::mem::transmute(pnrightscount),
        )
        .ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_DeviceManager`, `Win32_Foundation`*"]
    pub unsafe fn MakeDecisionOnClearChannel<'a, Param6: ::windows::runtime::IntoParam<'a, IMDSPStorageGlobals>, Param7: ::windows::runtime::IntoParam<'a, IWMDMProgress3>, Param16: ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>>(
        &self,
        fuflags: u32,
        pdata: *const u8,
        dwsize: u32,
        dwappsec: u32,
        pbspsessionkey: *const u8,
        dwsessionkeylen: u32,
        pstorageglobals: Param6,
        pprogresscallback: Param7,
        pappcertapp: *const u8,
        dwappcertapplen: u32,
        pappcertsp: *const u8,
        dwappcertsplen: u32,
        pszrevocationurl: *mut super::super::Foundation::PWSTR,
        pdwrevocationurllen: *mut u32,
        pdwrevocationbitflag: *mut u32,
        pqwfilesize: *mut u64,
        punknown: Param16,
        ppexchange: *mut ::std::option::Option<ISCPSecureExchange>,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).9)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(fuflags),
            ::std::mem::transmute(pdata),
            ::std::mem::transmute(dwsize),
            ::std::mem::transmute(dwappsec),
            ::std::mem::transmute(pbspsessionkey),
            ::std::mem::transmute(dwsessionkeylen),
            pstorageglobals.into_param().abi(),
            pprogresscallback.into_param().abi(),
            ::std::mem::transmute(pappcertapp),
            ::std::mem::transmute(dwappcertapplen),
            ::std::mem::transmute(pappcertsp),
            ::std::mem::transmute(dwappcertsplen),
            ::std::mem::transmute(pszrevocationurl),
            ::std::mem::transmute(pdwrevocationurllen),
            ::std::mem::transmute(pdwrevocationbitflag),
            ::std::mem::transmute(pqwfilesize),
            punknown.into_param().abi(),
            ::std::mem::transmute(ppexchange),
        )
        .ok()
    }
}
unsafe impl ::windows::runtime::Interface for ISCPSecureQuery3 {
    type Vtable = ISCPSecureQuery3_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3085816226, 19883, 18507, [179, 197, 173, 57, 184, 180, 192, 177]);
}
impl ::std::convert::From<ISCPSecureQuery3> for ::windows::runtime::IUnknown {
    fn from(value: ISCPSecureQuery3) -> Self {
        value.0
    }
}
impl ::std::convert::From<&ISCPSecureQuery3> for ::windows::runtime::IUnknown {
    fn from(value: &ISCPSecureQuery3) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ISCPSecureQuery3 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a ISCPSecureQuery3 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::std::convert::From<ISCPSecureQuery3> for ISCPSecureQuery2 {
    fn from(value: ISCPSecureQuery3) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&ISCPSecureQuery3> for ISCPSecureQuery2 {
    fn from(value: &ISCPSecureQuery3) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ISCPSecureQuery2> for ISCPSecureQuery3 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ISCPSecureQuery2> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<ISCPSecureQuery2>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ISCPSecureQuery2> for &ISCPSecureQuery3 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ISCPSecureQuery2> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<ISCPSecureQuery2>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<ISCPSecureQuery3> for ISCPSecureQuery {
    fn from(value: ISCPSecureQuery3) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&ISCPSecureQuery3> for ISCPSecureQuery {
    fn from(value: &ISCPSecureQuery3) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ISCPSecureQuery> for ISCPSecureQuery3 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ISCPSecureQuery> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<ISCPSecureQuery>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ISCPSecureQuery> for &ISCPSecureQuery3 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ISCPSecureQuery> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<ISCPSecureQuery>::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ISCPSecureQuery3_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pfuflags: *mut u32, pdwminrightsdata: *mut u32, pdwminexaminedata: *mut u32, pdwmindecidedata: *mut u32, abmac: *mut u8) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, fuflags: u32, pwszextension: super::super::Foundation::PWSTR, pdata: *const u8, dwsize: u32, abmac: *mut u8) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, fuflags: u32, pdata: *const u8, dwsize: u32, dwappsec: u32, pbspsessionkey: *const u8, dwsessionkeylen: u32, pstorageglobals: ::windows::runtime::RawPtr, ppexchange: *mut ::windows::runtime::RawPtr, abmac: *mut u8) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pdata: *const u8, dwsize: u32, pbspsessionkey: *const u8, dwsessionkeylen: u32, pstgglobals: ::windows::runtime::RawPtr, pprights: *mut *mut WMDMRIGHTS, pnrightscount: *mut u32, abmac: *mut u8) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        fuflags: u32,
        pdata: *const u8,
        dwsize: u32,
        dwappsec: u32,
        pbspsessionkey: *const u8,
        dwsessionkeylen: u32,
        pstorageglobals: ::windows::runtime::RawPtr,
        pappcertapp: *const u8,
        dwappcertapplen: u32,
        pappcertsp: *const u8,
        dwappcertsplen: u32,
        pszrevocationurl: *mut super::super::Foundation::PWSTR,
        pdwrevocationurllen: *mut u32,
        pdwrevocationbitflag: *mut u32,
        pqwfilesize: *mut u64,
        punknown: ::windows::runtime::RawPtr,
        ppexchange: *mut ::windows::runtime::RawPtr,
        abmac: *mut u8,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pdata: *const u8, dwsize: u32, pbspsessionkey: *const u8, dwsessionkeylen: u32, pstgglobals: ::windows::runtime::RawPtr, pprogresscallback: ::windows::runtime::RawPtr, pprights: *mut *mut WMDMRIGHTS, pnrightscount: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        fuflags: u32,
        pdata: *const u8,
        dwsize: u32,
        dwappsec: u32,
        pbspsessionkey: *const u8,
        dwsessionkeylen: u32,
        pstorageglobals: ::windows::runtime::RawPtr,
        pprogresscallback: ::windows::runtime::RawPtr,
        pappcertapp: *const u8,
        dwappcertapplen: u32,
        pappcertsp: *const u8,
        dwappcertsplen: u32,
        pszrevocationurl: *mut super::super::Foundation::PWSTR,
        pdwrevocationurllen: *mut u32,
        pdwrevocationbitflag: *mut u32,
        pqwfilesize: *mut u64,
        punknown: ::windows::runtime::RawPtr,
        ppexchange: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[doc = "*Required features: `Win32_Media_DeviceManager`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct ISCPSession(pub ::windows::runtime::IUnknown);
impl ISCPSession {
    #[doc = "*Required features: `Win32_Media_DeviceManager`*"]
    pub unsafe fn BeginSession<'a, Param0: ::windows::runtime::IntoParam<'a, IMDSPDevice>>(&self, pidevice: Param0, pctx: *const u8, dwsizectx: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), pidevice.into_param().abi(), ::std::mem::transmute(pctx), ::std::mem::transmute(dwsizectx)).ok()
    }
    #[doc = "*Required features: `Win32_Media_DeviceManager`*"]
    pub unsafe fn EndSession(&self, pctx: *const u8, dwsizectx: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), ::std::mem::transmute(pctx), ::std::mem::transmute(dwsizectx)).ok()
    }
    #[doc = "*Required features: `Win32_Media_DeviceManager`*"]
    pub unsafe fn GetSecureQuery(&self) -> ::windows::runtime::Result<ISCPSecureQuery> {
        let mut result__: <ISCPSecureQuery as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self), &mut result__).from_abi::<ISCPSecureQuery>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for ISCPSession {
    type Vtable = ISCPSession_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2292442861, 61156, 17945, [187, 179, 253, 79, 182, 39, 21, 209]);
}
impl ::std::convert::From<ISCPSession> for ::windows::runtime::IUnknown {
    fn from(value: ISCPSession) -> Self {
        value.0
    }
}
impl ::std::convert::From<&ISCPSession> for ::windows::runtime::IUnknown {
    fn from(value: &ISCPSession) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ISCPSession {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a ISCPSession {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ISCPSession_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pidevice: ::windows::runtime::RawPtr, pctx: *const u8, dwsizectx: u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pctx: *const u8, dwsizectx: u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppsecurequery: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_Media_DeviceManager`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IWMDMDevice(pub ::windows::runtime::IUnknown);
impl IWMDMDevice {
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_DeviceManager`, `Win32_Foundation`*"]
    pub unsafe fn GetName(&self, pwszname: super::super::Foundation::PWSTR, nmaxchars: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), ::std::mem::transmute(pwszname), ::std::mem::transmute(nmaxchars)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_DeviceManager`, `Win32_Foundation`*"]
    pub unsafe fn GetManufacturer(&self, pwszname: super::super::Foundation::PWSTR, nmaxchars: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), ::std::mem::transmute(pwszname), ::std::mem::transmute(nmaxchars)).ok()
    }
    #[doc = "*Required features: `Win32_Media_DeviceManager`*"]
    pub unsafe fn GetVersion(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `Win32_Media_DeviceManager`*"]
    pub unsafe fn GetType(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).6)(::std::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `Win32_Media_DeviceManager`*"]
    pub unsafe fn GetSerialNumber(&self, pserialnumber: *mut WMDMID, abmac: *mut u8) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self), ::std::mem::transmute(pserialnumber), ::std::mem::transmute(abmac)).ok()
    }
    #[doc = "*Required features: `Win32_Media_DeviceManager`*"]
    pub unsafe fn GetPowerSource(&self, pdwpowersource: *mut u32, pdwpercentremaining: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).8)(::std::mem::transmute_copy(self), ::std::mem::transmute(pdwpowersource), ::std::mem::transmute(pdwpercentremaining)).ok()
    }
    #[doc = "*Required features: `Win32_Media_DeviceManager`*"]
    pub unsafe fn GetStatus(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).9)(::std::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `Win32_Media_DeviceManager`*"]
    pub unsafe fn GetDeviceIcon(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).10)(::std::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `Win32_Media_DeviceManager`*"]
    pub unsafe fn EnumStorage(&self) -> ::windows::runtime::Result<IWMDMEnumStorage> {
        let mut result__: <IWMDMEnumStorage as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).11)(::std::mem::transmute_copy(self), &mut result__).from_abi::<IWMDMEnumStorage>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_DeviceManager`, `Win32_Foundation`*"]
    pub unsafe fn GetFormatSupport(&self, ppformatex: *mut *mut _WAVEFORMATEX, pnformatcount: *mut u32, pppwszmimetype: *mut *mut super::super::Foundation::PWSTR, pnmimetypecount: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).12)(::std::mem::transmute_copy(self), ::std::mem::transmute(ppformatex), ::std::mem::transmute(pnformatcount), ::std::mem::transmute(pppwszmimetype), ::std::mem::transmute(pnmimetypecount)).ok()
    }
    #[doc = "*Required features: `Win32_Media_DeviceManager`*"]
    pub unsafe fn SendOpaqueCommand(&self, pcommand: *mut OPAQUECOMMAND) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).13)(::std::mem::transmute_copy(self), ::std::mem::transmute(pcommand)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IWMDMDevice {
    type Vtable = IWMDMDevice_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(499857922, 13293, 4563, [132, 112, 0, 192, 79, 121, 219, 192]);
}
impl ::std::convert::From<IWMDMDevice> for ::windows::runtime::IUnknown {
    fn from(value: IWMDMDevice) -> Self {
        value.0
    }
}
impl ::std::convert::From<&IWMDMDevice> for ::windows::runtime::IUnknown {
    fn from(value: &IWMDMDevice) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IWMDMDevice {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IWMDMDevice {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMDMDevice_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pwszname: super::super::Foundation::PWSTR, nmaxchars: u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pwszname: super::super::Foundation::PWSTR, nmaxchars: u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pdwversion: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pdwtype: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pserialnumber: *mut WMDMID, abmac: *mut u8) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pdwpowersource: *mut u32, pdwpercentremaining: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pdwstatus: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, hicon: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppenumstorage: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppformatex: *mut *mut _WAVEFORMATEX, pnformatcount: *mut u32, pppwszmimetype: *mut *mut super::super::Foundation::PWSTR, pnmimetypecount: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pcommand: *mut OPAQUECOMMAND) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_Media_DeviceManager`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IWMDMDevice2(pub ::windows::runtime::IUnknown);
impl IWMDMDevice2 {
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_DeviceManager`, `Win32_Foundation`*"]
    pub unsafe fn GetName(&self, pwszname: super::super::Foundation::PWSTR, nmaxchars: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), ::std::mem::transmute(pwszname), ::std::mem::transmute(nmaxchars)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_DeviceManager`, `Win32_Foundation`*"]
    pub unsafe fn GetManufacturer(&self, pwszname: super::super::Foundation::PWSTR, nmaxchars: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), ::std::mem::transmute(pwszname), ::std::mem::transmute(nmaxchars)).ok()
    }
    #[doc = "*Required features: `Win32_Media_DeviceManager`*"]
    pub unsafe fn GetVersion(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `Win32_Media_DeviceManager`*"]
    pub unsafe fn GetType(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).6)(::std::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `Win32_Media_DeviceManager`*"]
    pub unsafe fn GetSerialNumber(&self, pserialnumber: *mut WMDMID, abmac: *mut u8) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self), ::std::mem::transmute(pserialnumber), ::std::mem::transmute(abmac)).ok()
    }
    #[doc = "*Required features: `Win32_Media_DeviceManager`*"]
    pub unsafe fn GetPowerSource(&self, pdwpowersource: *mut u32, pdwpercentremaining: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).8)(::std::mem::transmute_copy(self), ::std::mem::transmute(pdwpowersource), ::std::mem::transmute(pdwpercentremaining)).ok()
    }
    #[doc = "*Required features: `Win32_Media_DeviceManager`*"]
    pub unsafe fn GetStatus(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).9)(::std::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `Win32_Media_DeviceManager`*"]
    pub unsafe fn GetDeviceIcon(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).10)(::std::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `Win32_Media_DeviceManager`*"]
    pub unsafe fn EnumStorage(&self) -> ::windows::runtime::Result<IWMDMEnumStorage> {
        let mut result__: <IWMDMEnumStorage as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).11)(::std::mem::transmute_copy(self), &mut result__).from_abi::<IWMDMEnumStorage>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_DeviceManager`, `Win32_Foundation`*"]
    pub unsafe fn GetFormatSupport(&self, ppformatex: *mut *mut _WAVEFORMATEX, pnformatcount: *mut u32, pppwszmimetype: *mut *mut super::super::Foundation::PWSTR, pnmimetypecount: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).12)(::std::mem::transmute_copy(self), ::std::mem::transmute(ppformatex), ::std::mem::transmute(pnformatcount), ::std::mem::transmute(pppwszmimetype), ::std::mem::transmute(pnmimetypecount)).ok()
    }
    #[doc = "*Required features: `Win32_Media_DeviceManager`*"]
    pub unsafe fn SendOpaqueCommand(&self, pcommand: *mut OPAQUECOMMAND) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).13)(::std::mem::transmute_copy(self), ::std::mem::transmute(pcommand)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_DeviceManager`, `Win32_Foundation`*"]
    pub unsafe fn GetStorage<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, pszstoragename: Param0) -> ::windows::runtime::Result<IWMDMStorage> {
        let mut result__: <IWMDMStorage as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).14)(::std::mem::transmute_copy(self), pszstoragename.into_param().abi(), &mut result__).from_abi::<IWMDMStorage>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_DeviceManager`, `Win32_Foundation`*"]
    pub unsafe fn GetFormatSupport2(&self, dwflags: u32, ppaudioformatex: *mut *mut _WAVEFORMATEX, pnaudioformatcount: *mut u32, ppvideoformatex: *mut *mut _VIDEOINFOHEADER, pnvideoformatcount: *mut u32, ppfiletype: *mut *mut WMFILECAPABILITIES, pnfiletypecount: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).15)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(dwflags),
            ::std::mem::transmute(ppaudioformatex),
            ::std::mem::transmute(pnaudioformatcount),
            ::std::mem::transmute(ppvideoformatex),
            ::std::mem::transmute(pnvideoformatcount),
            ::std::mem::transmute(ppfiletype),
            ::std::mem::transmute(pnfiletypecount),
        )
        .ok()
    }
    #[cfg(feature = "Win32_System_Ole")]
    #[doc = "*Required features: `Win32_Media_DeviceManager`, `Win32_System_Ole`*"]
    pub unsafe fn GetSpecifyPropertyPages(&self, ppspecifyproppages: *mut ::std::option::Option<super::super::System::Ole::ISpecifyPropertyPages>, pppunknowns: *mut *mut ::std::option::Option<::windows::runtime::IUnknown>, pcunks: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).16)(::std::mem::transmute_copy(self), ::std::mem::transmute(ppspecifyproppages), ::std::mem::transmute(pppunknowns), ::std::mem::transmute(pcunks)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_DeviceManager`, `Win32_Foundation`*"]
    pub unsafe fn GetCanonicalName(&self, pwszpnpname: super::super::Foundation::PWSTR, nmaxchars: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).17)(::std::mem::transmute_copy(self), ::std::mem::transmute(pwszpnpname), ::std::mem::transmute(nmaxchars)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IWMDMDevice2 {
    type Vtable = IWMDMDevice2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3813621047, 40295, 20417, [146, 82, 98, 210, 139, 47, 139, 85]);
}
impl ::std::convert::From<IWMDMDevice2> for ::windows::runtime::IUnknown {
    fn from(value: IWMDMDevice2) -> Self {
        value.0
    }
}
impl ::std::convert::From<&IWMDMDevice2> for ::windows::runtime::IUnknown {
    fn from(value: &IWMDMDevice2) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IWMDMDevice2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IWMDMDevice2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::std::convert::From<IWMDMDevice2> for IWMDMDevice {
    fn from(value: IWMDMDevice2) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IWMDMDevice2> for IWMDMDevice {
    fn from(value: &IWMDMDevice2) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IWMDMDevice> for IWMDMDevice2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IWMDMDevice> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IWMDMDevice>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IWMDMDevice> for &IWMDMDevice2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IWMDMDevice> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IWMDMDevice>::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMDMDevice2_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pwszname: super::super::Foundation::PWSTR, nmaxchars: u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pwszname: super::super::Foundation::PWSTR, nmaxchars: u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pdwversion: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pdwtype: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pserialnumber: *mut WMDMID, abmac: *mut u8) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pdwpowersource: *mut u32, pdwpercentremaining: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pdwstatus: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, hicon: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppenumstorage: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppformatex: *mut *mut _WAVEFORMATEX, pnformatcount: *mut u32, pppwszmimetype: *mut *mut super::super::Foundation::PWSTR, pnmimetypecount: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pcommand: *mut OPAQUECOMMAND) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pszstoragename: super::super::Foundation::PWSTR, ppstorage: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwflags: u32, ppaudioformatex: *mut *mut _WAVEFORMATEX, pnaudioformatcount: *mut u32, ppvideoformatex: *mut *mut _VIDEOINFOHEADER, pnvideoformatcount: *mut u32, ppfiletype: *mut *mut WMFILECAPABILITIES, pnfiletypecount: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_System_Ole")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppspecifyproppages: *mut ::windows::runtime::RawPtr, pppunknowns: *mut *mut ::windows::runtime::RawPtr, pcunks: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Ole"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pwszpnpname: super::super::Foundation::PWSTR, nmaxchars: u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[doc = "*Required features: `Win32_Media_DeviceManager`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IWMDMDevice3(pub ::windows::runtime::IUnknown);
impl IWMDMDevice3 {
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_DeviceManager`, `Win32_Foundation`*"]
    pub unsafe fn GetName(&self, pwszname: super::super::Foundation::PWSTR, nmaxchars: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), ::std::mem::transmute(pwszname), ::std::mem::transmute(nmaxchars)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_DeviceManager`, `Win32_Foundation`*"]
    pub unsafe fn GetManufacturer(&self, pwszname: super::super::Foundation::PWSTR, nmaxchars: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), ::std::mem::transmute(pwszname), ::std::mem::transmute(nmaxchars)).ok()
    }
    #[doc = "*Required features: `Win32_Media_DeviceManager`*"]
    pub unsafe fn GetVersion(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `Win32_Media_DeviceManager`*"]
    pub unsafe fn GetType(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).6)(::std::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `Win32_Media_DeviceManager`*"]
    pub unsafe fn GetSerialNumber(&self, pserialnumber: *mut WMDMID, abmac: *mut u8) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self), ::std::mem::transmute(pserialnumber), ::std::mem::transmute(abmac)).ok()
    }
    #[doc = "*Required features: `Win32_Media_DeviceManager`*"]
    pub unsafe fn GetPowerSource(&self, pdwpowersource: *mut u32, pdwpercentremaining: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).8)(::std::mem::transmute_copy(self), ::std::mem::transmute(pdwpowersource), ::std::mem::transmute(pdwpercentremaining)).ok()
    }
    #[doc = "*Required features: `Win32_Media_DeviceManager`*"]
    pub unsafe fn GetStatus(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).9)(::std::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `Win32_Media_DeviceManager`*"]
    pub unsafe fn GetDeviceIcon(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).10)(::std::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `Win32_Media_DeviceManager`*"]
    pub unsafe fn EnumStorage(&self) -> ::windows::runtime::Result<IWMDMEnumStorage> {
        let mut result__: <IWMDMEnumStorage as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).11)(::std::mem::transmute_copy(self), &mut result__).from_abi::<IWMDMEnumStorage>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_DeviceManager`, `Win32_Foundation`*"]
    pub unsafe fn GetFormatSupport(&self, ppformatex: *mut *mut _WAVEFORMATEX, pnformatcount: *mut u32, pppwszmimetype: *mut *mut super::super::Foundation::PWSTR, pnmimetypecount: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).12)(::std::mem::transmute_copy(self), ::std::mem::transmute(ppformatex), ::std::mem::transmute(pnformatcount), ::std::mem::transmute(pppwszmimetype), ::std::mem::transmute(pnmimetypecount)).ok()
    }
    #[doc = "*Required features: `Win32_Media_DeviceManager`*"]
    pub unsafe fn SendOpaqueCommand(&self, pcommand: *mut OPAQUECOMMAND) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).13)(::std::mem::transmute_copy(self), ::std::mem::transmute(pcommand)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_DeviceManager`, `Win32_Foundation`*"]
    pub unsafe fn GetStorage<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, pszstoragename: Param0) -> ::windows::runtime::Result<IWMDMStorage> {
        let mut result__: <IWMDMStorage as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).14)(::std::mem::transmute_copy(self), pszstoragename.into_param().abi(), &mut result__).from_abi::<IWMDMStorage>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_DeviceManager`, `Win32_Foundation`*"]
    pub unsafe fn GetFormatSupport2(&self, dwflags: u32, ppaudioformatex: *mut *mut _WAVEFORMATEX, pnaudioformatcount: *mut u32, ppvideoformatex: *mut *mut _VIDEOINFOHEADER, pnvideoformatcount: *mut u32, ppfiletype: *mut *mut WMFILECAPABILITIES, pnfiletypecount: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).15)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(dwflags),
            ::std::mem::transmute(ppaudioformatex),
            ::std::mem::transmute(pnaudioformatcount),
            ::std::mem::transmute(ppvideoformatex),
            ::std::mem::transmute(pnvideoformatcount),
            ::std::mem::transmute(ppfiletype),
            ::std::mem::transmute(pnfiletypecount),
        )
        .ok()
    }
    #[cfg(feature = "Win32_System_Ole")]
    #[doc = "*Required features: `Win32_Media_DeviceManager`, `Win32_System_Ole`*"]
    pub unsafe fn GetSpecifyPropertyPages(&self, ppspecifyproppages: *mut ::std::option::Option<super::super::System::Ole::ISpecifyPropertyPages>, pppunknowns: *mut *mut ::std::option::Option<::windows::runtime::IUnknown>, pcunks: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).16)(::std::mem::transmute_copy(self), ::std::mem::transmute(ppspecifyproppages), ::std::mem::transmute(pppunknowns), ::std::mem::transmute(pcunks)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_DeviceManager`, `Win32_Foundation`*"]
    pub unsafe fn GetCanonicalName(&self, pwszpnpname: super::super::Foundation::PWSTR, nmaxchars: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).17)(::std::mem::transmute_copy(self), ::std::mem::transmute(pwszpnpname), ::std::mem::transmute(nmaxchars)).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Ole_Automation"))]
    #[doc = "*Required features: `Win32_Media_DeviceManager`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Com_StructuredStorage`, `Win32_System_Ole_Automation`*"]
    pub unsafe fn GetProperty<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, pwszpropname: Param0) -> ::windows::runtime::Result<super::super::System::Com::StructuredStorage::PROPVARIANT> {
        let mut result__: <super::super::System::Com::StructuredStorage::PROPVARIANT as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).18)(::std::mem::transmute_copy(self), pwszpropname.into_param().abi(), &mut result__).from_abi::<super::super::System::Com::StructuredStorage::PROPVARIANT>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Ole_Automation"))]
    #[doc = "*Required features: `Win32_Media_DeviceManager`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Com_StructuredStorage`, `Win32_System_Ole_Automation`*"]
    pub unsafe fn SetProperty<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, pwszpropname: Param0, pvalue: *const super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).19)(::std::mem::transmute_copy(self), pwszpropname.into_param().abi(), ::std::mem::transmute(pvalue)).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Ole_Automation"))]
    #[doc = "*Required features: `Win32_Media_DeviceManager`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Com_StructuredStorage`, `Win32_System_Ole_Automation`*"]
    pub unsafe fn GetFormatCapability(&self, format: WMDM_FORMATCODE) -> ::windows::runtime::Result<WMDM_FORMAT_CAPABILITY> {
        let mut result__: <WMDM_FORMAT_CAPABILITY as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).20)(::std::mem::transmute_copy(self), ::std::mem::transmute(format), &mut result__).from_abi::<WMDM_FORMAT_CAPABILITY>(result__)
    }
    #[doc = "*Required features: `Win32_Media_DeviceManager`*"]
    pub unsafe fn DeviceIoControl(&self, dwiocontrolcode: u32, lpinbuffer: *const u8, ninbuffersize: u32, lpoutbuffer: *mut u8, pnoutbuffersize: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).21)(::std::mem::transmute_copy(self), ::std::mem::transmute(dwiocontrolcode), ::std::mem::transmute(lpinbuffer), ::std::mem::transmute(ninbuffersize), ::std::mem::transmute(lpoutbuffer), ::std::mem::transmute(pnoutbuffersize)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_DeviceManager`, `Win32_Foundation`*"]
    pub unsafe fn FindStorage<'a, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, findscope: WMDM_FIND_SCOPE, pwszuniqueid: Param1) -> ::windows::runtime::Result<IWMDMStorage> {
        let mut result__: <IWMDMStorage as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).22)(::std::mem::transmute_copy(self), ::std::mem::transmute(findscope), pwszuniqueid.into_param().abi(), &mut result__).from_abi::<IWMDMStorage>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IWMDMDevice3 {
    type Vtable = IWMDMDevice3_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1812194558, 1499, 19930, [158, 60, 6, 35, 58, 109, 93, 101]);
}
impl ::std::convert::From<IWMDMDevice3> for ::windows::runtime::IUnknown {
    fn from(value: IWMDMDevice3) -> Self {
        value.0
    }
}
impl ::std::convert::From<&IWMDMDevice3> for ::windows::runtime::IUnknown {
    fn from(value: &IWMDMDevice3) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IWMDMDevice3 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IWMDMDevice3 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::std::convert::From<IWMDMDevice3> for IWMDMDevice2 {
    fn from(value: IWMDMDevice3) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IWMDMDevice3> for IWMDMDevice2 {
    fn from(value: &IWMDMDevice3) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IWMDMDevice2> for IWMDMDevice3 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IWMDMDevice2> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IWMDMDevice2>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IWMDMDevice2> for &IWMDMDevice3 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IWMDMDevice2> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IWMDMDevice2>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<IWMDMDevice3> for IWMDMDevice {
    fn from(value: IWMDMDevice3) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IWMDMDevice3> for IWMDMDevice {
    fn from(value: &IWMDMDevice3) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IWMDMDevice> for IWMDMDevice3 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IWMDMDevice> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IWMDMDevice>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IWMDMDevice> for &IWMDMDevice3 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IWMDMDevice> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IWMDMDevice>::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMDMDevice3_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pwszname: super::super::Foundation::PWSTR, nmaxchars: u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pwszname: super::super::Foundation::PWSTR, nmaxchars: u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pdwversion: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pdwtype: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pserialnumber: *mut WMDMID, abmac: *mut u8) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pdwpowersource: *mut u32, pdwpercentremaining: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pdwstatus: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, hicon: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppenumstorage: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppformatex: *mut *mut _WAVEFORMATEX, pnformatcount: *mut u32, pppwszmimetype: *mut *mut super::super::Foundation::PWSTR, pnmimetypecount: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pcommand: *mut OPAQUECOMMAND) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pszstoragename: super::super::Foundation::PWSTR, ppstorage: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwflags: u32, ppaudioformatex: *mut *mut _WAVEFORMATEX, pnaudioformatcount: *mut u32, ppvideoformatex: *mut *mut _VIDEOINFOHEADER, pnvideoformatcount: *mut u32, ppfiletype: *mut *mut WMFILECAPABILITIES, pnfiletypecount: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_System_Ole")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppspecifyproppages: *mut ::windows::runtime::RawPtr, pppunknowns: *mut *mut ::windows::runtime::RawPtr, pcunks: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Ole"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pwszpnpname: super::super::Foundation::PWSTR, nmaxchars: u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Ole_Automation"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pwszpropname: super::super::Foundation::PWSTR, pvalue: *mut ::std::mem::ManuallyDrop<super::super::System::Com::StructuredStorage::PROPVARIANT>) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Ole_Automation")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Ole_Automation"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pwszpropname: super::super::Foundation::PWSTR, pvalue: *const ::std::mem::ManuallyDrop<super::super::System::Com::StructuredStorage::PROPVARIANT>) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Ole_Automation")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Ole_Automation"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, format: WMDM_FORMATCODE, pformatsupport: *mut WMDM_FORMAT_CAPABILITY) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Ole_Automation")))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwiocontrolcode: u32, lpinbuffer: *const u8, ninbuffersize: u32, lpoutbuffer: *mut u8, pnoutbuffersize: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, findscope: WMDM_FIND_SCOPE, pwszuniqueid: super::super::Foundation::PWSTR, ppstorage: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[doc = "*Required features: `Win32_Media_DeviceManager`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IWMDMDeviceControl(pub ::windows::runtime::IUnknown);
impl IWMDMDeviceControl {
    #[doc = "*Required features: `Win32_Media_DeviceManager`*"]
    pub unsafe fn GetStatus(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `Win32_Media_DeviceManager`*"]
    pub unsafe fn GetCapabilities(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `Win32_Media_DeviceManager`*"]
    pub unsafe fn Play(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `Win32_Media_DeviceManager`*"]
    pub unsafe fn Record(&self, pformat: *const _WAVEFORMATEX) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::std::mem::transmute_copy(self), ::std::mem::transmute(pformat)).ok()
    }
    #[doc = "*Required features: `Win32_Media_DeviceManager`*"]
    pub unsafe fn Pause(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `Win32_Media_DeviceManager`*"]
    pub unsafe fn Resume(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).8)(::std::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `Win32_Media_DeviceManager`*"]
    pub unsafe fn Stop(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).9)(::std::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `Win32_Media_DeviceManager`*"]
    pub unsafe fn Seek(&self, fumode: u32, noffset: i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).10)(::std::mem::transmute_copy(self), ::std::mem::transmute(fumode), ::std::mem::transmute(noffset)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IWMDMDeviceControl {
    type Vtable = IWMDMDeviceControl_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(499857924, 13293, 4563, [132, 112, 0, 192, 79, 121, 219, 192]);
}
impl ::std::convert::From<IWMDMDeviceControl> for ::windows::runtime::IUnknown {
    fn from(value: IWMDMDeviceControl) -> Self {
        value.0
    }
}
impl ::std::convert::From<&IWMDMDeviceControl> for ::windows::runtime::IUnknown {
    fn from(value: &IWMDMDeviceControl) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IWMDMDeviceControl {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IWMDMDeviceControl {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMDMDeviceControl_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pdwstatus: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pdwcapabilitiesmask: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pformat: *const _WAVEFORMATEX) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, fumode: u32, noffset: i32) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_Media_DeviceManager`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IWMDMDeviceSession(pub ::windows::runtime::IUnknown);
impl IWMDMDeviceSession {
    #[doc = "*Required features: `Win32_Media_DeviceManager`*"]
    pub unsafe fn BeginSession(&self, r#type: WMDM_SESSION_TYPE, pctx: *const u8, dwsizectx: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), ::std::mem::transmute(r#type), ::std::mem::transmute(pctx), ::std::mem::transmute(dwsizectx)).ok()
    }
    #[doc = "*Required features: `Win32_Media_DeviceManager`*"]
    pub unsafe fn EndSession(&self, r#type: WMDM_SESSION_TYPE, pctx: *const u8, dwsizectx: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), ::std::mem::transmute(r#type), ::std::mem::transmute(pctx), ::std::mem::transmute(dwsizectx)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IWMDMDeviceSession {
    type Vtable = IWMDMDeviceSession_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2192509541, 40342, 16684, [131, 229, 60, 67, 228, 176, 108, 199]);
}
impl ::std::convert::From<IWMDMDeviceSession> for ::windows::runtime::IUnknown {
    fn from(value: IWMDMDeviceSession) -> Self {
        value.0
    }
}
impl ::std::convert::From<&IWMDMDeviceSession> for ::windows::runtime::IUnknown {
    fn from(value: &IWMDMDeviceSession) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IWMDMDeviceSession {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IWMDMDeviceSession {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMDMDeviceSession_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, r#type: WMDM_SESSION_TYPE, pctx: *const u8, dwsizectx: u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, r#type: WMDM_SESSION_TYPE, pctx: *const u8, dwsizectx: u32) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_Media_DeviceManager`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IWMDMEnumDevice(pub ::windows::runtime::IUnknown);
impl IWMDMEnumDevice {
    #[doc = "*Required features: `Win32_Media_DeviceManager`*"]
    pub unsafe fn Next(&self, celt: u32, ppdevice: *mut ::std::option::Option<IWMDMDevice>, pceltfetched: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), ::std::mem::transmute(celt), ::std::mem::transmute(ppdevice), ::std::mem::transmute(pceltfetched)).ok()
    }
    #[doc = "*Required features: `Win32_Media_DeviceManager`*"]
    pub unsafe fn Skip(&self, celt: u32) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), ::std::mem::transmute(celt), &mut result__).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `Win32_Media_DeviceManager`*"]
    pub unsafe fn Reset(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `Win32_Media_DeviceManager`*"]
    pub unsafe fn Clone(&self) -> ::windows::runtime::Result<IWMDMEnumDevice> {
        let mut result__: <IWMDMEnumDevice as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).6)(::std::mem::transmute_copy(self), &mut result__).from_abi::<IWMDMEnumDevice>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IWMDMEnumDevice {
    type Vtable = IWMDMEnumDevice_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(499857921, 13293, 4563, [132, 112, 0, 192, 79, 121, 219, 192]);
}
impl ::std::convert::From<IWMDMEnumDevice> for ::windows::runtime::IUnknown {
    fn from(value: IWMDMEnumDevice) -> Self {
        value.0
    }
}
impl ::std::convert::From<&IWMDMEnumDevice> for ::windows::runtime::IUnknown {
    fn from(value: &IWMDMEnumDevice) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IWMDMEnumDevice {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IWMDMEnumDevice {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMDMEnumDevice_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, celt: u32, ppdevice: *mut ::windows::runtime::RawPtr, pceltfetched: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, celt: u32, pceltfetched: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppenumdevice: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_Media_DeviceManager`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IWMDMEnumStorage(pub ::windows::runtime::IUnknown);
impl IWMDMEnumStorage {
    #[doc = "*Required features: `Win32_Media_DeviceManager`*"]
    pub unsafe fn Next(&self, celt: u32, ppstorage: *mut ::std::option::Option<IWMDMStorage>, pceltfetched: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), ::std::mem::transmute(celt), ::std::mem::transmute(ppstorage), ::std::mem::transmute(pceltfetched)).ok()
    }
    #[doc = "*Required features: `Win32_Media_DeviceManager`*"]
    pub unsafe fn Skip(&self, celt: u32) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), ::std::mem::transmute(celt), &mut result__).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `Win32_Media_DeviceManager`*"]
    pub unsafe fn Reset(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `Win32_Media_DeviceManager`*"]
    pub unsafe fn Clone(&self) -> ::windows::runtime::Result<IWMDMEnumStorage> {
        let mut result__: <IWMDMEnumStorage as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).6)(::std::mem::transmute_copy(self), &mut result__).from_abi::<IWMDMEnumStorage>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IWMDMEnumStorage {
    type Vtable = IWMDMEnumStorage_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(499857925, 13293, 4563, [132, 112, 0, 192, 79, 121, 219, 192]);
}
impl ::std::convert::From<IWMDMEnumStorage> for ::windows::runtime::IUnknown {
    fn from(value: IWMDMEnumStorage) -> Self {
        value.0
    }
}
impl ::std::convert::From<&IWMDMEnumStorage> for ::windows::runtime::IUnknown {
    fn from(value: &IWMDMEnumStorage) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IWMDMEnumStorage {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IWMDMEnumStorage {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMDMEnumStorage_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, celt: u32, ppstorage: *mut ::windows::runtime::RawPtr, pceltfetched: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, celt: u32, pceltfetched: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppenumstorage: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_Media_DeviceManager`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IWMDMLogger(pub ::windows::runtime::IUnknown);
impl IWMDMLogger {
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_DeviceManager`, `Win32_Foundation`*"]
    pub unsafe fn IsEnabled(&self) -> ::windows::runtime::Result<super::super::Foundation::BOOL> {
        let mut result__: <super::super::Foundation::BOOL as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_DeviceManager`, `Win32_Foundation`*"]
    pub unsafe fn Enable<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>>(&self, fenable: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), fenable.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_DeviceManager`, `Win32_Foundation`*"]
    pub unsafe fn GetLogFileName(&self, pszfilename: super::super::Foundation::PSTR, nmaxchars: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self), ::std::mem::transmute(pszfilename), ::std::mem::transmute(nmaxchars)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_DeviceManager`, `Win32_Foundation`*"]
    pub unsafe fn SetLogFileName<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>>(&self, pszfilename: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::std::mem::transmute_copy(self), pszfilename.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_DeviceManager`, `Win32_Foundation`*"]
    pub unsafe fn LogString<'a, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>, Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>>(&self, dwflags: u32, pszsrcname: Param1, pszlog: Param2) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self), ::std::mem::transmute(dwflags), pszsrcname.into_param().abi(), pszlog.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_DeviceManager`, `Win32_Foundation`*"]
    pub unsafe fn LogDword<'a, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>, Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>>(&self, dwflags: u32, pszsrcname: Param1, pszlogformat: Param2, dwlog: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).8)(::std::mem::transmute_copy(self), ::std::mem::transmute(dwflags), pszsrcname.into_param().abi(), pszlogformat.into_param().abi(), ::std::mem::transmute(dwlog)).ok()
    }
    #[doc = "*Required features: `Win32_Media_DeviceManager`*"]
    pub unsafe fn Reset(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).9)(::std::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `Win32_Media_DeviceManager`*"]
    pub unsafe fn GetSizeParams(&self, pdwmaxsize: *mut u32, pdwshrinktosize: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).10)(::std::mem::transmute_copy(self), ::std::mem::transmute(pdwmaxsize), ::std::mem::transmute(pdwshrinktosize)).ok()
    }
    #[doc = "*Required features: `Win32_Media_DeviceManager`*"]
    pub unsafe fn SetSizeParams(&self, dwmaxsize: u32, dwshrinktosize: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).11)(::std::mem::transmute_copy(self), ::std::mem::transmute(dwmaxsize), ::std::mem::transmute(dwshrinktosize)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IWMDMLogger {
    type Vtable = IWMDMLogger_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(285880832, 23161, 4563, [141, 120, 68, 69, 83, 84, 0, 0]);
}
impl ::std::convert::From<IWMDMLogger> for ::windows::runtime::IUnknown {
    fn from(value: IWMDMLogger) -> Self {
        value.0
    }
}
impl ::std::convert::From<&IWMDMLogger> for ::windows::runtime::IUnknown {
    fn from(value: &IWMDMLogger) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IWMDMLogger {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IWMDMLogger {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMDMLogger_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pfenabled: *mut super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, fenable: super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pszfilename: super::super::Foundation::PSTR, nmaxchars: u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pszfilename: super::super::Foundation::PSTR) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwflags: u32, pszsrcname: super::super::Foundation::PSTR, pszlog: super::super::Foundation::PSTR) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwflags: u32, pszsrcname: super::super::Foundation::PSTR, pszlogformat: super::super::Foundation::PSTR, dwlog: u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pdwmaxsize: *mut u32, pdwshrinktosize: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwmaxsize: u32, dwshrinktosize: u32) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_Media_DeviceManager`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IWMDMMetaData(pub ::windows::runtime::IUnknown);
impl IWMDMMetaData {
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_DeviceManager`, `Win32_Foundation`*"]
    pub unsafe fn AddItem<'a, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, r#type: WMDM_TAG_DATATYPE, pwsztagname: Param1, pvalue: *const u8, ilength: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), ::std::mem::transmute(r#type), pwsztagname.into_param().abi(), ::std::mem::transmute(pvalue), ::std::mem::transmute(ilength)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_DeviceManager`, `Win32_Foundation`*"]
    pub unsafe fn QueryByName<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, pwsztagname: Param0, ptype: *mut WMDM_TAG_DATATYPE, pvalue: *mut *mut u8, pcblength: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), pwsztagname.into_param().abi(), ::std::mem::transmute(ptype), ::std::mem::transmute(pvalue), ::std::mem::transmute(pcblength)).ok()
    }
    #[doc = "*Required features: `Win32_Media_DeviceManager`*"]
    pub unsafe fn QueryByIndex(&self, iindex: u32, ppwszname: *mut *mut u16, ptype: *mut WMDM_TAG_DATATYPE, ppvalue: *mut *mut u8, pcblength: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self), ::std::mem::transmute(iindex), ::std::mem::transmute(ppwszname), ::std::mem::transmute(ptype), ::std::mem::transmute(ppvalue), ::std::mem::transmute(pcblength)).ok()
    }
    #[doc = "*Required features: `Win32_Media_DeviceManager`*"]
    pub unsafe fn GetItemCount(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).6)(::std::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IWMDMMetaData {
    type Vtable = IWMDMMetaData_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3963291235, 2385, 17930, [154, 128, 13, 206, 237, 60, 4, 60]);
}
impl ::std::convert::From<IWMDMMetaData> for ::windows::runtime::IUnknown {
    fn from(value: IWMDMMetaData) -> Self {
        value.0
    }
}
impl ::std::convert::From<&IWMDMMetaData> for ::windows::runtime::IUnknown {
    fn from(value: &IWMDMMetaData) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IWMDMMetaData {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IWMDMMetaData {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMDMMetaData_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, r#type: WMDM_TAG_DATATYPE, pwsztagname: super::super::Foundation::PWSTR, pvalue: *const u8, ilength: u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pwsztagname: super::super::Foundation::PWSTR, ptype: *mut WMDM_TAG_DATATYPE, pvalue: *mut *mut u8, pcblength: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iindex: u32, ppwszname: *mut *mut u16, ptype: *mut WMDM_TAG_DATATYPE, ppvalue: *mut *mut u8, pcblength: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, icount: *mut u32) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_Media_DeviceManager`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IWMDMNotification(pub ::windows::runtime::IUnknown);
impl IWMDMNotification {
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_DeviceManager`, `Win32_Foundation`*"]
    pub unsafe fn WMDMMessage<'a, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, dwmessagetype: u32, pwszcanonicalname: Param1) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), ::std::mem::transmute(dwmessagetype), pwszcanonicalname.into_param().abi()).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IWMDMNotification {
    type Vtable = IWMDMNotification_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1063163328, 3907, 20180, [147, 210, 200, 154, 69, 213, 155, 129]);
}
impl ::std::convert::From<IWMDMNotification> for ::windows::runtime::IUnknown {
    fn from(value: IWMDMNotification) -> Self {
        value.0
    }
}
impl ::std::convert::From<&IWMDMNotification> for ::windows::runtime::IUnknown {
    fn from(value: &IWMDMNotification) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IWMDMNotification {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IWMDMNotification {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMDMNotification_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwmessagetype: u32, pwszcanonicalname: super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[doc = "*Required features: `Win32_Media_DeviceManager`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IWMDMObjectInfo(pub ::windows::runtime::IUnknown);
impl IWMDMObjectInfo {
    #[doc = "*Required features: `Win32_Media_DeviceManager`*"]
    pub unsafe fn GetPlayLength(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `Win32_Media_DeviceManager`*"]
    pub unsafe fn SetPlayLength(&self, dwlength: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), ::std::mem::transmute(dwlength)).ok()
    }
    #[doc = "*Required features: `Win32_Media_DeviceManager`*"]
    pub unsafe fn GetPlayOffset(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `Win32_Media_DeviceManager`*"]
    pub unsafe fn SetPlayOffset(&self, dwoffset: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::std::mem::transmute_copy(self), ::std::mem::transmute(dwoffset)).ok()
    }
    #[doc = "*Required features: `Win32_Media_DeviceManager`*"]
    pub unsafe fn GetTotalLength(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `Win32_Media_DeviceManager`*"]
    pub unsafe fn GetLastPlayPosition(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).8)(::std::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `Win32_Media_DeviceManager`*"]
    pub unsafe fn GetLongestPlayPosition(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).9)(::std::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IWMDMObjectInfo {
    type Vtable = IWMDMObjectInfo_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(499857929, 13293, 4563, [132, 112, 0, 192, 79, 121, 219, 192]);
}
impl ::std::convert::From<IWMDMObjectInfo> for ::windows::runtime::IUnknown {
    fn from(value: IWMDMObjectInfo) -> Self {
        value.0
    }
}
impl ::std::convert::From<&IWMDMObjectInfo> for ::windows::runtime::IUnknown {
    fn from(value: &IWMDMObjectInfo) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IWMDMObjectInfo {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IWMDMObjectInfo {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMDMObjectInfo_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pdwlength: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwlength: u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pdwoffset: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwoffset: u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pdwlength: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pdwlastpos: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pdwlongestpos: *mut u32) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_Media_DeviceManager`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IWMDMOperation(pub ::windows::runtime::IUnknown);
impl IWMDMOperation {
    #[doc = "*Required features: `Win32_Media_DeviceManager`*"]
    pub unsafe fn BeginRead(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `Win32_Media_DeviceManager`*"]
    pub unsafe fn BeginWrite(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_DeviceManager`, `Win32_Foundation`*"]
    pub unsafe fn GetObjectName(&self, pwszname: super::super::Foundation::PWSTR, nmaxchars: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self), ::std::mem::transmute(pwszname), ::std::mem::transmute(nmaxchars)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_DeviceManager`, `Win32_Foundation`*"]
    pub unsafe fn SetObjectName<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, pwszname: Param0, nmaxchars: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::std::mem::transmute_copy(self), pwszname.into_param().abi(), ::std::mem::transmute(nmaxchars)).ok()
    }
    #[doc = "*Required features: `Win32_Media_DeviceManager`*"]
    pub unsafe fn GetObjectAttributes(&self, pdwattributes: *mut u32, pformat: *mut _WAVEFORMATEX) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self), ::std::mem::transmute(pdwattributes), ::std::mem::transmute(pformat)).ok()
    }
    #[doc = "*Required features: `Win32_Media_DeviceManager`*"]
    pub unsafe fn SetObjectAttributes(&self, dwattributes: u32, pformat: *const _WAVEFORMATEX) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).8)(::std::mem::transmute_copy(self), ::std::mem::transmute(dwattributes), ::std::mem::transmute(pformat)).ok()
    }
    #[doc = "*Required features: `Win32_Media_DeviceManager`*"]
    pub unsafe fn GetObjectTotalSize(&self, pdwsize: *mut u32, pdwsizehigh: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).9)(::std::mem::transmute_copy(self), ::std::mem::transmute(pdwsize), ::std::mem::transmute(pdwsizehigh)).ok()
    }
    #[doc = "*Required features: `Win32_Media_DeviceManager`*"]
    pub unsafe fn SetObjectTotalSize(&self, dwsize: u32, dwsizehigh: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).10)(::std::mem::transmute_copy(self), ::std::mem::transmute(dwsize), ::std::mem::transmute(dwsizehigh)).ok()
    }
    #[doc = "*Required features: `Win32_Media_DeviceManager`*"]
    pub unsafe fn TransferObjectData(&self, pdata: *mut u8, pdwsize: *mut u32, abmac: *mut u8) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).11)(::std::mem::transmute_copy(self), ::std::mem::transmute(pdata), ::std::mem::transmute(pdwsize), ::std::mem::transmute(abmac)).ok()
    }
    #[doc = "*Required features: `Win32_Media_DeviceManager`*"]
    pub unsafe fn End<'a, Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>>(&self, phcompletioncode: *const ::windows::runtime::HRESULT, pnewobject: Param1) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).12)(::std::mem::transmute_copy(self), ::std::mem::transmute(phcompletioncode), pnewobject.into_param().abi()).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IWMDMOperation {
    type Vtable = IWMDMOperation_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(499857931, 13293, 4563, [132, 112, 0, 192, 79, 121, 219, 192]);
}
impl ::std::convert::From<IWMDMOperation> for ::windows::runtime::IUnknown {
    fn from(value: IWMDMOperation) -> Self {
        value.0
    }
}
impl ::std::convert::From<&IWMDMOperation> for ::windows::runtime::IUnknown {
    fn from(value: &IWMDMOperation) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IWMDMOperation {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IWMDMOperation {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMDMOperation_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pwszname: super::super::Foundation::PWSTR, nmaxchars: u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pwszname: super::super::Foundation::PWSTR, nmaxchars: u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pdwattributes: *mut u32, pformat: *mut _WAVEFORMATEX) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwattributes: u32, pformat: *const _WAVEFORMATEX) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pdwsize: *mut u32, pdwsizehigh: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwsize: u32, dwsizehigh: u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pdata: *mut u8, pdwsize: *mut u32, abmac: *mut u8) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, phcompletioncode: *const ::windows::runtime::HRESULT, pnewobject: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_Media_DeviceManager`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IWMDMOperation2(pub ::windows::runtime::IUnknown);
impl IWMDMOperation2 {
    #[doc = "*Required features: `Win32_Media_DeviceManager`*"]
    pub unsafe fn BeginRead(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `Win32_Media_DeviceManager`*"]
    pub unsafe fn BeginWrite(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_DeviceManager`, `Win32_Foundation`*"]
    pub unsafe fn GetObjectName(&self, pwszname: super::super::Foundation::PWSTR, nmaxchars: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self), ::std::mem::transmute(pwszname), ::std::mem::transmute(nmaxchars)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_DeviceManager`, `Win32_Foundation`*"]
    pub unsafe fn SetObjectName<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, pwszname: Param0, nmaxchars: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::std::mem::transmute_copy(self), pwszname.into_param().abi(), ::std::mem::transmute(nmaxchars)).ok()
    }
    #[doc = "*Required features: `Win32_Media_DeviceManager`*"]
    pub unsafe fn GetObjectAttributes(&self, pdwattributes: *mut u32, pformat: *mut _WAVEFORMATEX) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self), ::std::mem::transmute(pdwattributes), ::std::mem::transmute(pformat)).ok()
    }
    #[doc = "*Required features: `Win32_Media_DeviceManager`*"]
    pub unsafe fn SetObjectAttributes(&self, dwattributes: u32, pformat: *const _WAVEFORMATEX) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).8)(::std::mem::transmute_copy(self), ::std::mem::transmute(dwattributes), ::std::mem::transmute(pformat)).ok()
    }
    #[doc = "*Required features: `Win32_Media_DeviceManager`*"]
    pub unsafe fn GetObjectTotalSize(&self, pdwsize: *mut u32, pdwsizehigh: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).9)(::std::mem::transmute_copy(self), ::std::mem::transmute(pdwsize), ::std::mem::transmute(pdwsizehigh)).ok()
    }
    #[doc = "*Required features: `Win32_Media_DeviceManager`*"]
    pub unsafe fn SetObjectTotalSize(&self, dwsize: u32, dwsizehigh: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).10)(::std::mem::transmute_copy(self), ::std::mem::transmute(dwsize), ::std::mem::transmute(dwsizehigh)).ok()
    }
    #[doc = "*Required features: `Win32_Media_DeviceManager`*"]
    pub unsafe fn TransferObjectData(&self, pdata: *mut u8, pdwsize: *mut u32, abmac: *mut u8) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).11)(::std::mem::transmute_copy(self), ::std::mem::transmute(pdata), ::std::mem::transmute(pdwsize), ::std::mem::transmute(abmac)).ok()
    }
    #[doc = "*Required features: `Win32_Media_DeviceManager`*"]
    pub unsafe fn End<'a, Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>>(&self, phcompletioncode: *const ::windows::runtime::HRESULT, pnewobject: Param1) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).12)(::std::mem::transmute_copy(self), ::std::mem::transmute(phcompletioncode), pnewobject.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_DeviceManager`, `Win32_Foundation`*"]
    pub unsafe fn SetObjectAttributes2(&self, dwattributes: u32, dwattributesex: u32, pformat: *const _WAVEFORMATEX, pvideoformat: *const _VIDEOINFOHEADER) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).13)(::std::mem::transmute_copy(self), ::std::mem::transmute(dwattributes), ::std::mem::transmute(dwattributesex), ::std::mem::transmute(pformat), ::std::mem::transmute(pvideoformat)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_DeviceManager`, `Win32_Foundation`*"]
    pub unsafe fn GetObjectAttributes2(&self, pdwattributes: *mut u32, pdwattributesex: *mut u32, paudioformat: *mut _WAVEFORMATEX, pvideoformat: *mut _VIDEOINFOHEADER) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).14)(::std::mem::transmute_copy(self), ::std::mem::transmute(pdwattributes), ::std::mem::transmute(pdwattributesex), ::std::mem::transmute(paudioformat), ::std::mem::transmute(pvideoformat)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IWMDMOperation2 {
    type Vtable = IWMDMOperation2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(860117832, 32247, 16988, [173, 143, 15, 198, 216, 47, 159, 117]);
}
impl ::std::convert::From<IWMDMOperation2> for ::windows::runtime::IUnknown {
    fn from(value: IWMDMOperation2) -> Self {
        value.0
    }
}
impl ::std::convert::From<&IWMDMOperation2> for ::windows::runtime::IUnknown {
    fn from(value: &IWMDMOperation2) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IWMDMOperation2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IWMDMOperation2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::std::convert::From<IWMDMOperation2> for IWMDMOperation {
    fn from(value: IWMDMOperation2) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IWMDMOperation2> for IWMDMOperation {
    fn from(value: &IWMDMOperation2) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IWMDMOperation> for IWMDMOperation2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IWMDMOperation> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IWMDMOperation>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IWMDMOperation> for &IWMDMOperation2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IWMDMOperation> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IWMDMOperation>::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMDMOperation2_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pwszname: super::super::Foundation::PWSTR, nmaxchars: u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pwszname: super::super::Foundation::PWSTR, nmaxchars: u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pdwattributes: *mut u32, pformat: *mut _WAVEFORMATEX) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwattributes: u32, pformat: *const _WAVEFORMATEX) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pdwsize: *mut u32, pdwsizehigh: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwsize: u32, dwsizehigh: u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pdata: *mut u8, pdwsize: *mut u32, abmac: *mut u8) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, phcompletioncode: *const ::windows::runtime::HRESULT, pnewobject: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwattributes: u32, dwattributesex: u32, pformat: *const _WAVEFORMATEX, pvideoformat: *const _VIDEOINFOHEADER) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pdwattributes: *mut u32, pdwattributesex: *mut u32, paudioformat: *mut _WAVEFORMATEX, pvideoformat: *mut _VIDEOINFOHEADER) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[doc = "*Required features: `Win32_Media_DeviceManager`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IWMDMOperation3(pub ::windows::runtime::IUnknown);
impl IWMDMOperation3 {
    #[doc = "*Required features: `Win32_Media_DeviceManager`*"]
    pub unsafe fn BeginRead(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `Win32_Media_DeviceManager`*"]
    pub unsafe fn BeginWrite(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_DeviceManager`, `Win32_Foundation`*"]
    pub unsafe fn GetObjectName(&self, pwszname: super::super::Foundation::PWSTR, nmaxchars: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self), ::std::mem::transmute(pwszname), ::std::mem::transmute(nmaxchars)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_DeviceManager`, `Win32_Foundation`*"]
    pub unsafe fn SetObjectName<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, pwszname: Param0, nmaxchars: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::std::mem::transmute_copy(self), pwszname.into_param().abi(), ::std::mem::transmute(nmaxchars)).ok()
    }
    #[doc = "*Required features: `Win32_Media_DeviceManager`*"]
    pub unsafe fn GetObjectAttributes(&self, pdwattributes: *mut u32, pformat: *mut _WAVEFORMATEX) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self), ::std::mem::transmute(pdwattributes), ::std::mem::transmute(pformat)).ok()
    }
    #[doc = "*Required features: `Win32_Media_DeviceManager`*"]
    pub unsafe fn SetObjectAttributes(&self, dwattributes: u32, pformat: *const _WAVEFORMATEX) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).8)(::std::mem::transmute_copy(self), ::std::mem::transmute(dwattributes), ::std::mem::transmute(pformat)).ok()
    }
    #[doc = "*Required features: `Win32_Media_DeviceManager`*"]
    pub unsafe fn GetObjectTotalSize(&self, pdwsize: *mut u32, pdwsizehigh: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).9)(::std::mem::transmute_copy(self), ::std::mem::transmute(pdwsize), ::std::mem::transmute(pdwsizehigh)).ok()
    }
    #[doc = "*Required features: `Win32_Media_DeviceManager`*"]
    pub unsafe fn SetObjectTotalSize(&self, dwsize: u32, dwsizehigh: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).10)(::std::mem::transmute_copy(self), ::std::mem::transmute(dwsize), ::std::mem::transmute(dwsizehigh)).ok()
    }
    #[doc = "*Required features: `Win32_Media_DeviceManager`*"]
    pub unsafe fn TransferObjectData(&self, pdata: *mut u8, pdwsize: *mut u32, abmac: *mut u8) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).11)(::std::mem::transmute_copy(self), ::std::mem::transmute(pdata), ::std::mem::transmute(pdwsize), ::std::mem::transmute(abmac)).ok()
    }
    #[doc = "*Required features: `Win32_Media_DeviceManager`*"]
    pub unsafe fn End<'a, Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>>(&self, phcompletioncode: *const ::windows::runtime::HRESULT, pnewobject: Param1) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).12)(::std::mem::transmute_copy(self), ::std::mem::transmute(phcompletioncode), pnewobject.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_Media_DeviceManager`*"]
    pub unsafe fn TransferObjectDataOnClearChannel(&self, pdata: *mut u8, pdwsize: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).13)(::std::mem::transmute_copy(self), ::std::mem::transmute(pdata), ::std::mem::transmute(pdwsize)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IWMDMOperation3 {
    type Vtable = IWMDMOperation3_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3522802794, 40104, 18136, [157, 15, 30, 201, 186, 229, 73, 25]);
}
impl ::std::convert::From<IWMDMOperation3> for ::windows::runtime::IUnknown {
    fn from(value: IWMDMOperation3) -> Self {
        value.0
    }
}
impl ::std::convert::From<&IWMDMOperation3> for ::windows::runtime::IUnknown {
    fn from(value: &IWMDMOperation3) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IWMDMOperation3 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IWMDMOperation3 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::std::convert::From<IWMDMOperation3> for IWMDMOperation {
    fn from(value: IWMDMOperation3) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IWMDMOperation3> for IWMDMOperation {
    fn from(value: &IWMDMOperation3) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IWMDMOperation> for IWMDMOperation3 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IWMDMOperation> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IWMDMOperation>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IWMDMOperation> for &IWMDMOperation3 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IWMDMOperation> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IWMDMOperation>::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMDMOperation3_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pwszname: super::super::Foundation::PWSTR, nmaxchars: u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pwszname: super::super::Foundation::PWSTR, nmaxchars: u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pdwattributes: *mut u32, pformat: *mut _WAVEFORMATEX) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwattributes: u32, pformat: *const _WAVEFORMATEX) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pdwsize: *mut u32, pdwsizehigh: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwsize: u32, dwsizehigh: u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pdata: *mut u8, pdwsize: *mut u32, abmac: *mut u8) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, phcompletioncode: *const ::windows::runtime::HRESULT, pnewobject: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pdata: *mut u8, pdwsize: *mut u32) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_Media_DeviceManager`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IWMDMProgress(pub ::windows::runtime::IUnknown);
impl IWMDMProgress {
    #[doc = "*Required features: `Win32_Media_DeviceManager`*"]
    pub unsafe fn Begin(&self, dwestimatedticks: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), ::std::mem::transmute(dwestimatedticks)).ok()
    }
    #[doc = "*Required features: `Win32_Media_DeviceManager`*"]
    pub unsafe fn Progress(&self, dwtranspiredticks: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), ::std::mem::transmute(dwtranspiredticks)).ok()
    }
    #[doc = "*Required features: `Win32_Media_DeviceManager`*"]
    pub unsafe fn End(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IWMDMProgress {
    type Vtable = IWMDMProgress_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(499857932, 13293, 4563, [132, 112, 0, 192, 79, 121, 219, 192]);
}
impl ::std::convert::From<IWMDMProgress> for ::windows::runtime::IUnknown {
    fn from(value: IWMDMProgress) -> Self {
        value.0
    }
}
impl ::std::convert::From<&IWMDMProgress> for ::windows::runtime::IUnknown {
    fn from(value: &IWMDMProgress) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IWMDMProgress {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IWMDMProgress {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMDMProgress_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwestimatedticks: u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwtranspiredticks: u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_Media_DeviceManager`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IWMDMProgress2(pub ::windows::runtime::IUnknown);
impl IWMDMProgress2 {
    #[doc = "*Required features: `Win32_Media_DeviceManager`*"]
    pub unsafe fn Begin(&self, dwestimatedticks: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), ::std::mem::transmute(dwestimatedticks)).ok()
    }
    #[doc = "*Required features: `Win32_Media_DeviceManager`*"]
    pub unsafe fn Progress(&self, dwtranspiredticks: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), ::std::mem::transmute(dwtranspiredticks)).ok()
    }
    #[doc = "*Required features: `Win32_Media_DeviceManager`*"]
    pub unsafe fn End(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `Win32_Media_DeviceManager`*"]
    pub unsafe fn End2(&self, hrcompletioncode: ::windows::runtime::HRESULT) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::std::mem::transmute_copy(self), ::std::mem::transmute(hrcompletioncode)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IWMDMProgress2 {
    type Vtable = IWMDMProgress2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(977532240, 45955, 20114, [176, 74, 230, 187, 198, 96, 254, 252]);
}
impl ::std::convert::From<IWMDMProgress2> for ::windows::runtime::IUnknown {
    fn from(value: IWMDMProgress2) -> Self {
        value.0
    }
}
impl ::std::convert::From<&IWMDMProgress2> for ::windows::runtime::IUnknown {
    fn from(value: &IWMDMProgress2) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IWMDMProgress2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IWMDMProgress2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::std::convert::From<IWMDMProgress2> for IWMDMProgress {
    fn from(value: IWMDMProgress2) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IWMDMProgress2> for IWMDMProgress {
    fn from(value: &IWMDMProgress2) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IWMDMProgress> for IWMDMProgress2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IWMDMProgress> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IWMDMProgress>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IWMDMProgress> for &IWMDMProgress2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IWMDMProgress> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IWMDMProgress>::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMDMProgress2_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwestimatedticks: u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwtranspiredticks: u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, hrcompletioncode: ::windows::runtime::HRESULT) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_Media_DeviceManager`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IWMDMProgress3(pub ::windows::runtime::IUnknown);
impl IWMDMProgress3 {
    #[doc = "*Required features: `Win32_Media_DeviceManager`*"]
    pub unsafe fn Begin(&self, dwestimatedticks: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), ::std::mem::transmute(dwestimatedticks)).ok()
    }
    #[doc = "*Required features: `Win32_Media_DeviceManager`*"]
    pub unsafe fn Progress(&self, dwtranspiredticks: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), ::std::mem::transmute(dwtranspiredticks)).ok()
    }
    #[doc = "*Required features: `Win32_Media_DeviceManager`*"]
    pub unsafe fn End(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `Win32_Media_DeviceManager`*"]
    pub unsafe fn End2(&self, hrcompletioncode: ::windows::runtime::HRESULT) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::std::mem::transmute_copy(self), ::std::mem::transmute(hrcompletioncode)).ok()
    }
    #[doc = "*Required features: `Win32_Media_DeviceManager`*"]
    pub unsafe fn Begin3<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::GUID>>(&self, eventid: Param0, dwestimatedticks: u32, pcontext: *mut OPAQUECOMMAND) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self), eventid.into_param().abi(), ::std::mem::transmute(dwestimatedticks), ::std::mem::transmute(pcontext)).ok()
    }
    #[doc = "*Required features: `Win32_Media_DeviceManager`*"]
    pub unsafe fn Progress3<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::GUID>>(&self, eventid: Param0, dwtranspiredticks: u32, pcontext: *mut OPAQUECOMMAND) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).8)(::std::mem::transmute_copy(self), eventid.into_param().abi(), ::std::mem::transmute(dwtranspiredticks), ::std::mem::transmute(pcontext)).ok()
    }
    #[doc = "*Required features: `Win32_Media_DeviceManager`*"]
    pub unsafe fn End3<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::GUID>>(&self, eventid: Param0, hrcompletioncode: ::windows::runtime::HRESULT, pcontext: *mut OPAQUECOMMAND) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).9)(::std::mem::transmute_copy(self), eventid.into_param().abi(), ::std::mem::transmute(hrcompletioncode), ::std::mem::transmute(pcontext)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IWMDMProgress3 {
    type Vtable = IWMDMProgress3_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(568197579, 15284, 18729, [178, 26, 23, 175, 63, 128, 246, 88]);
}
impl ::std::convert::From<IWMDMProgress3> for ::windows::runtime::IUnknown {
    fn from(value: IWMDMProgress3) -> Self {
        value.0
    }
}
impl ::std::convert::From<&IWMDMProgress3> for ::windows::runtime::IUnknown {
    fn from(value: &IWMDMProgress3) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IWMDMProgress3 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IWMDMProgress3 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::std::convert::From<IWMDMProgress3> for IWMDMProgress2 {
    fn from(value: IWMDMProgress3) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IWMDMProgress3> for IWMDMProgress2 {
    fn from(value: &IWMDMProgress3) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IWMDMProgress2> for IWMDMProgress3 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IWMDMProgress2> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IWMDMProgress2>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IWMDMProgress2> for &IWMDMProgress3 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IWMDMProgress2> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IWMDMProgress2>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<IWMDMProgress3> for IWMDMProgress {
    fn from(value: IWMDMProgress3) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IWMDMProgress3> for IWMDMProgress {
    fn from(value: &IWMDMProgress3) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IWMDMProgress> for IWMDMProgress3 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IWMDMProgress> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IWMDMProgress>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IWMDMProgress> for &IWMDMProgress3 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IWMDMProgress> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IWMDMProgress>::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMDMProgress3_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwestimatedticks: u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwtranspiredticks: u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, hrcompletioncode: ::windows::runtime::HRESULT) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, eventid: ::windows::runtime::GUID, dwestimatedticks: u32, pcontext: *mut OPAQUECOMMAND) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, eventid: ::windows::runtime::GUID, dwtranspiredticks: u32, pcontext: *mut OPAQUECOMMAND) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, eventid: ::windows::runtime::GUID, hrcompletioncode: ::windows::runtime::HRESULT, pcontext: *mut OPAQUECOMMAND) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_Media_DeviceManager`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IWMDMRevoked(pub ::windows::runtime::IUnknown);
impl IWMDMRevoked {
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_DeviceManager`, `Win32_Foundation`*"]
    pub unsafe fn GetRevocationURL(&self, ppwszrevocationurl: *mut super::super::Foundation::PWSTR, pdwbufferlen: *mut u32, pdwrevokedbitflag: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), ::std::mem::transmute(ppwszrevocationurl), ::std::mem::transmute(pdwbufferlen), ::std::mem::transmute(pdwrevokedbitflag)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IWMDMRevoked {
    type Vtable = IWMDMRevoked_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3958165211, 35054, 20053, [182, 164, 141, 159, 7, 214, 150, 170]);
}
impl ::std::convert::From<IWMDMRevoked> for ::windows::runtime::IUnknown {
    fn from(value: IWMDMRevoked) -> Self {
        value.0
    }
}
impl ::std::convert::From<&IWMDMRevoked> for ::windows::runtime::IUnknown {
    fn from(value: &IWMDMRevoked) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IWMDMRevoked {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IWMDMRevoked {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMDMRevoked_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppwszrevocationurl: *mut super::super::Foundation::PWSTR, pdwbufferlen: *mut u32, pdwrevokedbitflag: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[doc = "*Required features: `Win32_Media_DeviceManager`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IWMDMStorage(pub ::windows::runtime::IUnknown);
impl IWMDMStorage {
    #[doc = "*Required features: `Win32_Media_DeviceManager`*"]
    pub unsafe fn SetAttributes(&self, dwattributes: u32, pformat: *const _WAVEFORMATEX) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), ::std::mem::transmute(dwattributes), ::std::mem::transmute(pformat)).ok()
    }
    #[doc = "*Required features: `Win32_Media_DeviceManager`*"]
    pub unsafe fn GetStorageGlobals(&self) -> ::windows::runtime::Result<IWMDMStorageGlobals> {
        let mut result__: <IWMDMStorageGlobals as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), &mut result__).from_abi::<IWMDMStorageGlobals>(result__)
    }
    #[doc = "*Required features: `Win32_Media_DeviceManager`*"]
    pub unsafe fn GetAttributes(&self, pdwattributes: *mut u32, pformat: *mut _WAVEFORMATEX) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self), ::std::mem::transmute(pdwattributes), ::std::mem::transmute(pformat)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_DeviceManager`, `Win32_Foundation`*"]
    pub unsafe fn GetName(&self, pwszname: super::super::Foundation::PWSTR, nmaxchars: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::std::mem::transmute_copy(self), ::std::mem::transmute(pwszname), ::std::mem::transmute(nmaxchars)).ok()
    }
    #[doc = "*Required features: `Win32_Media_DeviceManager`*"]
    pub unsafe fn GetDate(&self) -> ::windows::runtime::Result<WMDMDATETIME> {
        let mut result__: <WMDMDATETIME as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self), &mut result__).from_abi::<WMDMDATETIME>(result__)
    }
    #[doc = "*Required features: `Win32_Media_DeviceManager`*"]
    pub unsafe fn GetSize(&self, pdwsizelow: *mut u32, pdwsizehigh: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).8)(::std::mem::transmute_copy(self), ::std::mem::transmute(pdwsizelow), ::std::mem::transmute(pdwsizehigh)).ok()
    }
    #[doc = "*Required features: `Win32_Media_DeviceManager`*"]
    pub unsafe fn GetRights(&self, pprights: *mut *mut WMDMRIGHTS, pnrightscount: *mut u32, abmac: *mut u8) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).9)(::std::mem::transmute_copy(self), ::std::mem::transmute(pprights), ::std::mem::transmute(pnrightscount), ::std::mem::transmute(abmac)).ok()
    }
    #[doc = "*Required features: `Win32_Media_DeviceManager`*"]
    pub unsafe fn EnumStorage(&self) -> ::windows::runtime::Result<IWMDMEnumStorage> {
        let mut result__: <IWMDMEnumStorage as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).10)(::std::mem::transmute_copy(self), &mut result__).from_abi::<IWMDMEnumStorage>(result__)
    }
    #[doc = "*Required features: `Win32_Media_DeviceManager`*"]
    pub unsafe fn SendOpaqueCommand(&self, pcommand: *mut OPAQUECOMMAND) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).11)(::std::mem::transmute_copy(self), ::std::mem::transmute(pcommand)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IWMDMStorage {
    type Vtable = IWMDMStorage_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(499857926, 13293, 4563, [132, 112, 0, 192, 79, 121, 219, 192]);
}
impl ::std::convert::From<IWMDMStorage> for ::windows::runtime::IUnknown {
    fn from(value: IWMDMStorage) -> Self {
        value.0
    }
}
impl ::std::convert::From<&IWMDMStorage> for ::windows::runtime::IUnknown {
    fn from(value: &IWMDMStorage) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IWMDMStorage {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IWMDMStorage {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMDMStorage_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwattributes: u32, pformat: *const _WAVEFORMATEX) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppstorageglobals: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pdwattributes: *mut u32, pformat: *mut _WAVEFORMATEX) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pwszname: super::super::Foundation::PWSTR, nmaxchars: u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pdatetimeutc: *mut WMDMDATETIME) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pdwsizelow: *mut u32, pdwsizehigh: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pprights: *mut *mut WMDMRIGHTS, pnrightscount: *mut u32, abmac: *mut u8) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, penumstorage: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pcommand: *mut OPAQUECOMMAND) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_Media_DeviceManager`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IWMDMStorage2(pub ::windows::runtime::IUnknown);
impl IWMDMStorage2 {
    #[doc = "*Required features: `Win32_Media_DeviceManager`*"]
    pub unsafe fn SetAttributes(&self, dwattributes: u32, pformat: *const _WAVEFORMATEX) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), ::std::mem::transmute(dwattributes), ::std::mem::transmute(pformat)).ok()
    }
    #[doc = "*Required features: `Win32_Media_DeviceManager`*"]
    pub unsafe fn GetStorageGlobals(&self) -> ::windows::runtime::Result<IWMDMStorageGlobals> {
        let mut result__: <IWMDMStorageGlobals as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), &mut result__).from_abi::<IWMDMStorageGlobals>(result__)
    }
    #[doc = "*Required features: `Win32_Media_DeviceManager`*"]
    pub unsafe fn GetAttributes(&self, pdwattributes: *mut u32, pformat: *mut _WAVEFORMATEX) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self), ::std::mem::transmute(pdwattributes), ::std::mem::transmute(pformat)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_DeviceManager`, `Win32_Foundation`*"]
    pub unsafe fn GetName(&self, pwszname: super::super::Foundation::PWSTR, nmaxchars: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::std::mem::transmute_copy(self), ::std::mem::transmute(pwszname), ::std::mem::transmute(nmaxchars)).ok()
    }
    #[doc = "*Required features: `Win32_Media_DeviceManager`*"]
    pub unsafe fn GetDate(&self) -> ::windows::runtime::Result<WMDMDATETIME> {
        let mut result__: <WMDMDATETIME as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self), &mut result__).from_abi::<WMDMDATETIME>(result__)
    }
    #[doc = "*Required features: `Win32_Media_DeviceManager`*"]
    pub unsafe fn GetSize(&self, pdwsizelow: *mut u32, pdwsizehigh: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).8)(::std::mem::transmute_copy(self), ::std::mem::transmute(pdwsizelow), ::std::mem::transmute(pdwsizehigh)).ok()
    }
    #[doc = "*Required features: `Win32_Media_DeviceManager`*"]
    pub unsafe fn GetRights(&self, pprights: *mut *mut WMDMRIGHTS, pnrightscount: *mut u32, abmac: *mut u8) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).9)(::std::mem::transmute_copy(self), ::std::mem::transmute(pprights), ::std::mem::transmute(pnrightscount), ::std::mem::transmute(abmac)).ok()
    }
    #[doc = "*Required features: `Win32_Media_DeviceManager`*"]
    pub unsafe fn EnumStorage(&self) -> ::windows::runtime::Result<IWMDMEnumStorage> {
        let mut result__: <IWMDMEnumStorage as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).10)(::std::mem::transmute_copy(self), &mut result__).from_abi::<IWMDMEnumStorage>(result__)
    }
    #[doc = "*Required features: `Win32_Media_DeviceManager`*"]
    pub unsafe fn SendOpaqueCommand(&self, pcommand: *mut OPAQUECOMMAND) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).11)(::std::mem::transmute_copy(self), ::std::mem::transmute(pcommand)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_DeviceManager`, `Win32_Foundation`*"]
    pub unsafe fn GetStorage<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, pszstoragename: Param0) -> ::windows::runtime::Result<IWMDMStorage> {
        let mut result__: <IWMDMStorage as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).12)(::std::mem::transmute_copy(self), pszstoragename.into_param().abi(), &mut result__).from_abi::<IWMDMStorage>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_DeviceManager`, `Win32_Foundation`*"]
    pub unsafe fn SetAttributes2(&self, dwattributes: u32, dwattributesex: u32, pformat: *const _WAVEFORMATEX, pvideoformat: *const _VIDEOINFOHEADER) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).13)(::std::mem::transmute_copy(self), ::std::mem::transmute(dwattributes), ::std::mem::transmute(dwattributesex), ::std::mem::transmute(pformat), ::std::mem::transmute(pvideoformat)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_DeviceManager`, `Win32_Foundation`*"]
    pub unsafe fn GetAttributes2(&self, pdwattributes: *mut u32, pdwattributesex: *mut u32, paudioformat: *mut _WAVEFORMATEX, pvideoformat: *mut _VIDEOINFOHEADER) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).14)(::std::mem::transmute_copy(self), ::std::mem::transmute(pdwattributes), ::std::mem::transmute(pdwattributesex), ::std::mem::transmute(paudioformat), ::std::mem::transmute(pvideoformat)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IWMDMStorage2 {
    type Vtable = IWMDMStorage2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(517316932, 23765, 18051, [158, 255, 114, 203, 219, 45, 149, 51]);
}
impl ::std::convert::From<IWMDMStorage2> for ::windows::runtime::IUnknown {
    fn from(value: IWMDMStorage2) -> Self {
        value.0
    }
}
impl ::std::convert::From<&IWMDMStorage2> for ::windows::runtime::IUnknown {
    fn from(value: &IWMDMStorage2) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IWMDMStorage2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IWMDMStorage2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::std::convert::From<IWMDMStorage2> for IWMDMStorage {
    fn from(value: IWMDMStorage2) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IWMDMStorage2> for IWMDMStorage {
    fn from(value: &IWMDMStorage2) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IWMDMStorage> for IWMDMStorage2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IWMDMStorage> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IWMDMStorage>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IWMDMStorage> for &IWMDMStorage2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IWMDMStorage> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IWMDMStorage>::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMDMStorage2_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwattributes: u32, pformat: *const _WAVEFORMATEX) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppstorageglobals: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pdwattributes: *mut u32, pformat: *mut _WAVEFORMATEX) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pwszname: super::super::Foundation::PWSTR, nmaxchars: u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pdatetimeutc: *mut WMDMDATETIME) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pdwsizelow: *mut u32, pdwsizehigh: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pprights: *mut *mut WMDMRIGHTS, pnrightscount: *mut u32, abmac: *mut u8) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, penumstorage: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pcommand: *mut OPAQUECOMMAND) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pszstoragename: super::super::Foundation::PWSTR, ppstorage: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwattributes: u32, dwattributesex: u32, pformat: *const _WAVEFORMATEX, pvideoformat: *const _VIDEOINFOHEADER) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pdwattributes: *mut u32, pdwattributesex: *mut u32, paudioformat: *mut _WAVEFORMATEX, pvideoformat: *mut _VIDEOINFOHEADER) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[doc = "*Required features: `Win32_Media_DeviceManager`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IWMDMStorage3(pub ::windows::runtime::IUnknown);
impl IWMDMStorage3 {
    #[doc = "*Required features: `Win32_Media_DeviceManager`*"]
    pub unsafe fn SetAttributes(&self, dwattributes: u32, pformat: *const _WAVEFORMATEX) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), ::std::mem::transmute(dwattributes), ::std::mem::transmute(pformat)).ok()
    }
    #[doc = "*Required features: `Win32_Media_DeviceManager`*"]
    pub unsafe fn GetStorageGlobals(&self) -> ::windows::runtime::Result<IWMDMStorageGlobals> {
        let mut result__: <IWMDMStorageGlobals as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), &mut result__).from_abi::<IWMDMStorageGlobals>(result__)
    }
    #[doc = "*Required features: `Win32_Media_DeviceManager`*"]
    pub unsafe fn GetAttributes(&self, pdwattributes: *mut u32, pformat: *mut _WAVEFORMATEX) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self), ::std::mem::transmute(pdwattributes), ::std::mem::transmute(pformat)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_DeviceManager`, `Win32_Foundation`*"]
    pub unsafe fn GetName(&self, pwszname: super::super::Foundation::PWSTR, nmaxchars: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::std::mem::transmute_copy(self), ::std::mem::transmute(pwszname), ::std::mem::transmute(nmaxchars)).ok()
    }
    #[doc = "*Required features: `Win32_Media_DeviceManager`*"]
    pub unsafe fn GetDate(&self) -> ::windows::runtime::Result<WMDMDATETIME> {
        let mut result__: <WMDMDATETIME as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self), &mut result__).from_abi::<WMDMDATETIME>(result__)
    }
    #[doc = "*Required features: `Win32_Media_DeviceManager`*"]
    pub unsafe fn GetSize(&self, pdwsizelow: *mut u32, pdwsizehigh: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).8)(::std::mem::transmute_copy(self), ::std::mem::transmute(pdwsizelow), ::std::mem::transmute(pdwsizehigh)).ok()
    }
    #[doc = "*Required features: `Win32_Media_DeviceManager`*"]
    pub unsafe fn GetRights(&self, pprights: *mut *mut WMDMRIGHTS, pnrightscount: *mut u32, abmac: *mut u8) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).9)(::std::mem::transmute_copy(self), ::std::mem::transmute(pprights), ::std::mem::transmute(pnrightscount), ::std::mem::transmute(abmac)).ok()
    }
    #[doc = "*Required features: `Win32_Media_DeviceManager`*"]
    pub unsafe fn EnumStorage(&self) -> ::windows::runtime::Result<IWMDMEnumStorage> {
        let mut result__: <IWMDMEnumStorage as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).10)(::std::mem::transmute_copy(self), &mut result__).from_abi::<IWMDMEnumStorage>(result__)
    }
    #[doc = "*Required features: `Win32_Media_DeviceManager`*"]
    pub unsafe fn SendOpaqueCommand(&self, pcommand: *mut OPAQUECOMMAND) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).11)(::std::mem::transmute_copy(self), ::std::mem::transmute(pcommand)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_DeviceManager`, `Win32_Foundation`*"]
    pub unsafe fn GetStorage<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, pszstoragename: Param0) -> ::windows::runtime::Result<IWMDMStorage> {
        let mut result__: <IWMDMStorage as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).12)(::std::mem::transmute_copy(self), pszstoragename.into_param().abi(), &mut result__).from_abi::<IWMDMStorage>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_DeviceManager`, `Win32_Foundation`*"]
    pub unsafe fn SetAttributes2(&self, dwattributes: u32, dwattributesex: u32, pformat: *const _WAVEFORMATEX, pvideoformat: *const _VIDEOINFOHEADER) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).13)(::std::mem::transmute_copy(self), ::std::mem::transmute(dwattributes), ::std::mem::transmute(dwattributesex), ::std::mem::transmute(pformat), ::std::mem::transmute(pvideoformat)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_DeviceManager`, `Win32_Foundation`*"]
    pub unsafe fn GetAttributes2(&self, pdwattributes: *mut u32, pdwattributesex: *mut u32, paudioformat: *mut _WAVEFORMATEX, pvideoformat: *mut _VIDEOINFOHEADER) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).14)(::std::mem::transmute_copy(self), ::std::mem::transmute(pdwattributes), ::std::mem::transmute(pdwattributesex), ::std::mem::transmute(paudioformat), ::std::mem::transmute(pvideoformat)).ok()
    }
    #[doc = "*Required features: `Win32_Media_DeviceManager`*"]
    pub unsafe fn GetMetadata(&self) -> ::windows::runtime::Result<IWMDMMetaData> {
        let mut result__: <IWMDMMetaData as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).15)(::std::mem::transmute_copy(self), &mut result__).from_abi::<IWMDMMetaData>(result__)
    }
    #[doc = "*Required features: `Win32_Media_DeviceManager`*"]
    pub unsafe fn SetMetadata<'a, Param0: ::windows::runtime::IntoParam<'a, IWMDMMetaData>>(&self, pmetadata: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).16)(::std::mem::transmute_copy(self), pmetadata.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_Media_DeviceManager`*"]
    pub unsafe fn CreateEmptyMetadataObject(&self) -> ::windows::runtime::Result<IWMDMMetaData> {
        let mut result__: <IWMDMMetaData as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).17)(::std::mem::transmute_copy(self), &mut result__).from_abi::<IWMDMMetaData>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_DeviceManager`, `Win32_Foundation`*"]
    pub unsafe fn SetEnumPreference(&self, pmode: *mut WMDM_STORAGE_ENUM_MODE, nviews: u32, pviews: *const WMDMMetadataView) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).18)(::std::mem::transmute_copy(self), ::std::mem::transmute(pmode), ::std::mem::transmute(nviews), ::std::mem::transmute(pviews)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IWMDMStorage3 {
    type Vtable = IWMDMStorage3_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2540797674, 37482, 17998, [150, 164, 36, 123, 2, 22, 2, 110]);
}
impl ::std::convert::From<IWMDMStorage3> for ::windows::runtime::IUnknown {
    fn from(value: IWMDMStorage3) -> Self {
        value.0
    }
}
impl ::std::convert::From<&IWMDMStorage3> for ::windows::runtime::IUnknown {
    fn from(value: &IWMDMStorage3) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IWMDMStorage3 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IWMDMStorage3 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::std::convert::From<IWMDMStorage3> for IWMDMStorage2 {
    fn from(value: IWMDMStorage3) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IWMDMStorage3> for IWMDMStorage2 {
    fn from(value: &IWMDMStorage3) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IWMDMStorage2> for IWMDMStorage3 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IWMDMStorage2> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IWMDMStorage2>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IWMDMStorage2> for &IWMDMStorage3 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IWMDMStorage2> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IWMDMStorage2>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<IWMDMStorage3> for IWMDMStorage {
    fn from(value: IWMDMStorage3) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IWMDMStorage3> for IWMDMStorage {
    fn from(value: &IWMDMStorage3) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IWMDMStorage> for IWMDMStorage3 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IWMDMStorage> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IWMDMStorage>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IWMDMStorage> for &IWMDMStorage3 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IWMDMStorage> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IWMDMStorage>::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMDMStorage3_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwattributes: u32, pformat: *const _WAVEFORMATEX) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppstorageglobals: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pdwattributes: *mut u32, pformat: *mut _WAVEFORMATEX) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pwszname: super::super::Foundation::PWSTR, nmaxchars: u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pdatetimeutc: *mut WMDMDATETIME) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pdwsizelow: *mut u32, pdwsizehigh: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pprights: *mut *mut WMDMRIGHTS, pnrightscount: *mut u32, abmac: *mut u8) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, penumstorage: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pcommand: *mut OPAQUECOMMAND) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pszstoragename: super::super::Foundation::PWSTR, ppstorage: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwattributes: u32, dwattributesex: u32, pformat: *const _WAVEFORMATEX, pvideoformat: *const _VIDEOINFOHEADER) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pdwattributes: *mut u32, pdwattributesex: *mut u32, paudioformat: *mut _WAVEFORMATEX, pvideoformat: *mut _VIDEOINFOHEADER) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppmetadata: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pmetadata: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppmetadata: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pmode: *mut WMDM_STORAGE_ENUM_MODE, nviews: u32, pviews: *const WMDMMetadataView) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[doc = "*Required features: `Win32_Media_DeviceManager`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IWMDMStorage4(pub ::windows::runtime::IUnknown);
impl IWMDMStorage4 {
    #[doc = "*Required features: `Win32_Media_DeviceManager`*"]
    pub unsafe fn SetAttributes(&self, dwattributes: u32, pformat: *const _WAVEFORMATEX) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), ::std::mem::transmute(dwattributes), ::std::mem::transmute(pformat)).ok()
    }
    #[doc = "*Required features: `Win32_Media_DeviceManager`*"]
    pub unsafe fn GetStorageGlobals(&self) -> ::windows::runtime::Result<IWMDMStorageGlobals> {
        let mut result__: <IWMDMStorageGlobals as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), &mut result__).from_abi::<IWMDMStorageGlobals>(result__)
    }
    #[doc = "*Required features: `Win32_Media_DeviceManager`*"]
    pub unsafe fn GetAttributes(&self, pdwattributes: *mut u32, pformat: *mut _WAVEFORMATEX) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self), ::std::mem::transmute(pdwattributes), ::std::mem::transmute(pformat)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_DeviceManager`, `Win32_Foundation`*"]
    pub unsafe fn GetName(&self, pwszname: super::super::Foundation::PWSTR, nmaxchars: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::std::mem::transmute_copy(self), ::std::mem::transmute(pwszname), ::std::mem::transmute(nmaxchars)).ok()
    }
    #[doc = "*Required features: `Win32_Media_DeviceManager`*"]
    pub unsafe fn GetDate(&self) -> ::windows::runtime::Result<WMDMDATETIME> {
        let mut result__: <WMDMDATETIME as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self), &mut result__).from_abi::<WMDMDATETIME>(result__)
    }
    #[doc = "*Required features: `Win32_Media_DeviceManager`*"]
    pub unsafe fn GetSize(&self, pdwsizelow: *mut u32, pdwsizehigh: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).8)(::std::mem::transmute_copy(self), ::std::mem::transmute(pdwsizelow), ::std::mem::transmute(pdwsizehigh)).ok()
    }
    #[doc = "*Required features: `Win32_Media_DeviceManager`*"]
    pub unsafe fn GetRights(&self, pprights: *mut *mut WMDMRIGHTS, pnrightscount: *mut u32, abmac: *mut u8) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).9)(::std::mem::transmute_copy(self), ::std::mem::transmute(pprights), ::std::mem::transmute(pnrightscount), ::std::mem::transmute(abmac)).ok()
    }
    #[doc = "*Required features: `Win32_Media_DeviceManager`*"]
    pub unsafe fn EnumStorage(&self) -> ::windows::runtime::Result<IWMDMEnumStorage> {
        let mut result__: <IWMDMEnumStorage as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).10)(::std::mem::transmute_copy(self), &mut result__).from_abi::<IWMDMEnumStorage>(result__)
    }
    #[doc = "*Required features: `Win32_Media_DeviceManager`*"]
    pub unsafe fn SendOpaqueCommand(&self, pcommand: *mut OPAQUECOMMAND) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).11)(::std::mem::transmute_copy(self), ::std::mem::transmute(pcommand)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_DeviceManager`, `Win32_Foundation`*"]
    pub unsafe fn GetStorage<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, pszstoragename: Param0) -> ::windows::runtime::Result<IWMDMStorage> {
        let mut result__: <IWMDMStorage as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).12)(::std::mem::transmute_copy(self), pszstoragename.into_param().abi(), &mut result__).from_abi::<IWMDMStorage>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_DeviceManager`, `Win32_Foundation`*"]
    pub unsafe fn SetAttributes2(&self, dwattributes: u32, dwattributesex: u32, pformat: *const _WAVEFORMATEX, pvideoformat: *const _VIDEOINFOHEADER) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).13)(::std::mem::transmute_copy(self), ::std::mem::transmute(dwattributes), ::std::mem::transmute(dwattributesex), ::std::mem::transmute(pformat), ::std::mem::transmute(pvideoformat)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_DeviceManager`, `Win32_Foundation`*"]
    pub unsafe fn GetAttributes2(&self, pdwattributes: *mut u32, pdwattributesex: *mut u32, paudioformat: *mut _WAVEFORMATEX, pvideoformat: *mut _VIDEOINFOHEADER) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).14)(::std::mem::transmute_copy(self), ::std::mem::transmute(pdwattributes), ::std::mem::transmute(pdwattributesex), ::std::mem::transmute(paudioformat), ::std::mem::transmute(pvideoformat)).ok()
    }
    #[doc = "*Required features: `Win32_Media_DeviceManager`*"]
    pub unsafe fn GetMetadata(&self) -> ::windows::runtime::Result<IWMDMMetaData> {
        let mut result__: <IWMDMMetaData as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).15)(::std::mem::transmute_copy(self), &mut result__).from_abi::<IWMDMMetaData>(result__)
    }
    #[doc = "*Required features: `Win32_Media_DeviceManager`*"]
    pub unsafe fn SetMetadata<'a, Param0: ::windows::runtime::IntoParam<'a, IWMDMMetaData>>(&self, pmetadata: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).16)(::std::mem::transmute_copy(self), pmetadata.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_Media_DeviceManager`*"]
    pub unsafe fn CreateEmptyMetadataObject(&self) -> ::windows::runtime::Result<IWMDMMetaData> {
        let mut result__: <IWMDMMetaData as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).17)(::std::mem::transmute_copy(self), &mut result__).from_abi::<IWMDMMetaData>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_DeviceManager`, `Win32_Foundation`*"]
    pub unsafe fn SetEnumPreference(&self, pmode: *mut WMDM_STORAGE_ENUM_MODE, nviews: u32, pviews: *const WMDMMetadataView) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).18)(::std::mem::transmute_copy(self), ::std::mem::transmute(pmode), ::std::mem::transmute(nviews), ::std::mem::transmute(pviews)).ok()
    }
    #[doc = "*Required features: `Win32_Media_DeviceManager`*"]
    pub unsafe fn SetReferences(&self, dwrefs: u32, ppiwmdmstorage: *const ::std::option::Option<IWMDMStorage>) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).19)(::std::mem::transmute_copy(self), ::std::mem::transmute(dwrefs), ::std::mem::transmute(ppiwmdmstorage)).ok()
    }
    #[doc = "*Required features: `Win32_Media_DeviceManager`*"]
    pub unsafe fn GetReferences(&self, pdwrefs: *mut u32, pppiwmdmstorage: *mut *mut ::std::option::Option<IWMDMStorage>) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).20)(::std::mem::transmute_copy(self), ::std::mem::transmute(pdwrefs), ::std::mem::transmute(pppiwmdmstorage)).ok()
    }
    #[doc = "*Required features: `Win32_Media_DeviceManager`*"]
    pub unsafe fn GetRightsWithProgress<'a, Param0: ::windows::runtime::IntoParam<'a, IWMDMProgress3>>(&self, piprogresscallback: Param0, pprights: *mut *mut WMDMRIGHTS, pnrightscount: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).21)(::std::mem::transmute_copy(self), piprogresscallback.into_param().abi(), ::std::mem::transmute(pprights), ::std::mem::transmute(pnrightscount)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_DeviceManager`, `Win32_Foundation`*"]
    pub unsafe fn GetSpecifiedMetadata(&self, cproperties: u32, ppwszpropnames: *const super::super::Foundation::PWSTR) -> ::windows::runtime::Result<IWMDMMetaData> {
        let mut result__: <IWMDMMetaData as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).22)(::std::mem::transmute_copy(self), ::std::mem::transmute(cproperties), ::std::mem::transmute(ppwszpropnames), &mut result__).from_abi::<IWMDMMetaData>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_DeviceManager`, `Win32_Foundation`*"]
    pub unsafe fn FindStorage<'a, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, findscope: WMDM_FIND_SCOPE, pwszuniqueid: Param1) -> ::windows::runtime::Result<IWMDMStorage> {
        let mut result__: <IWMDMStorage as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).23)(::std::mem::transmute_copy(self), ::std::mem::transmute(findscope), pwszuniqueid.into_param().abi(), &mut result__).from_abi::<IWMDMStorage>(result__)
    }
    #[doc = "*Required features: `Win32_Media_DeviceManager`*"]
    pub unsafe fn GetParent(&self) -> ::windows::runtime::Result<IWMDMStorage> {
        let mut result__: <IWMDMStorage as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).24)(::std::mem::transmute_copy(self), &mut result__).from_abi::<IWMDMStorage>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IWMDMStorage4 {
    type Vtable = IWMDMStorage4_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3257252549, 41018, 16568, [154, 35, 145, 207, 71, 140, 100, 166]);
}
impl ::std::convert::From<IWMDMStorage4> for ::windows::runtime::IUnknown {
    fn from(value: IWMDMStorage4) -> Self {
        value.0
    }
}
impl ::std::convert::From<&IWMDMStorage4> for ::windows::runtime::IUnknown {
    fn from(value: &IWMDMStorage4) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IWMDMStorage4 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IWMDMStorage4 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::std::convert::From<IWMDMStorage4> for IWMDMStorage3 {
    fn from(value: IWMDMStorage4) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IWMDMStorage4> for IWMDMStorage3 {
    fn from(value: &IWMDMStorage4) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IWMDMStorage3> for IWMDMStorage4 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IWMDMStorage3> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IWMDMStorage3>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IWMDMStorage3> for &IWMDMStorage4 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IWMDMStorage3> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IWMDMStorage3>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<IWMDMStorage4> for IWMDMStorage2 {
    fn from(value: IWMDMStorage4) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IWMDMStorage4> for IWMDMStorage2 {
    fn from(value: &IWMDMStorage4) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IWMDMStorage2> for IWMDMStorage4 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IWMDMStorage2> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IWMDMStorage2>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IWMDMStorage2> for &IWMDMStorage4 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IWMDMStorage2> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IWMDMStorage2>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<IWMDMStorage4> for IWMDMStorage {
    fn from(value: IWMDMStorage4) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IWMDMStorage4> for IWMDMStorage {
    fn from(value: &IWMDMStorage4) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IWMDMStorage> for IWMDMStorage4 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IWMDMStorage> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IWMDMStorage>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IWMDMStorage> for &IWMDMStorage4 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IWMDMStorage> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IWMDMStorage>::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMDMStorage4_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwattributes: u32, pformat: *const _WAVEFORMATEX) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppstorageglobals: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pdwattributes: *mut u32, pformat: *mut _WAVEFORMATEX) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pwszname: super::super::Foundation::PWSTR, nmaxchars: u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pdatetimeutc: *mut WMDMDATETIME) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pdwsizelow: *mut u32, pdwsizehigh: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pprights: *mut *mut WMDMRIGHTS, pnrightscount: *mut u32, abmac: *mut u8) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, penumstorage: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pcommand: *mut OPAQUECOMMAND) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pszstoragename: super::super::Foundation::PWSTR, ppstorage: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwattributes: u32, dwattributesex: u32, pformat: *const _WAVEFORMATEX, pvideoformat: *const _VIDEOINFOHEADER) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pdwattributes: *mut u32, pdwattributesex: *mut u32, paudioformat: *mut _WAVEFORMATEX, pvideoformat: *mut _VIDEOINFOHEADER) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppmetadata: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pmetadata: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppmetadata: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pmode: *mut WMDM_STORAGE_ENUM_MODE, nviews: u32, pviews: *const WMDMMetadataView) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwrefs: u32, ppiwmdmstorage: *const ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pdwrefs: *mut u32, pppiwmdmstorage: *mut *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, piprogresscallback: ::windows::runtime::RawPtr, pprights: *mut *mut WMDMRIGHTS, pnrightscount: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, cproperties: u32, ppwszpropnames: *const super::super::Foundation::PWSTR, ppmetadata: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, findscope: WMDM_FIND_SCOPE, pwszuniqueid: super::super::Foundation::PWSTR, ppstorage: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppstorage: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_Media_DeviceManager`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IWMDMStorageControl(pub ::windows::runtime::IUnknown);
impl IWMDMStorageControl {
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_DeviceManager`, `Win32_Foundation`*"]
    pub unsafe fn Insert<'a, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>, Param2: ::windows::runtime::IntoParam<'a, IWMDMOperation>, Param3: ::windows::runtime::IntoParam<'a, IWMDMProgress>>(&self, fumode: u32, pwszfile: Param1, poperation: Param2, pprogress: Param3) -> ::windows::runtime::Result<IWMDMStorage> {
        let mut result__: <IWMDMStorage as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), ::std::mem::transmute(fumode), pwszfile.into_param().abi(), poperation.into_param().abi(), pprogress.into_param().abi(), &mut result__).from_abi::<IWMDMStorage>(result__)
    }
    #[doc = "*Required features: `Win32_Media_DeviceManager`*"]
    pub unsafe fn Delete<'a, Param1: ::windows::runtime::IntoParam<'a, IWMDMProgress>>(&self, fumode: u32, pprogress: Param1) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), ::std::mem::transmute(fumode), pprogress.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_DeviceManager`, `Win32_Foundation`*"]
    pub unsafe fn Rename<'a, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>, Param2: ::windows::runtime::IntoParam<'a, IWMDMProgress>>(&self, fumode: u32, pwsznewname: Param1, pprogress: Param2) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self), ::std::mem::transmute(fumode), pwsznewname.into_param().abi(), pprogress.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_DeviceManager`, `Win32_Foundation`*"]
    pub unsafe fn Read<'a, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>, Param2: ::windows::runtime::IntoParam<'a, IWMDMProgress>, Param3: ::windows::runtime::IntoParam<'a, IWMDMOperation>>(&self, fumode: u32, pwszfile: Param1, pprogress: Param2, poperation: Param3) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::std::mem::transmute_copy(self), ::std::mem::transmute(fumode), pwszfile.into_param().abi(), pprogress.into_param().abi(), poperation.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_Media_DeviceManager`*"]
    pub unsafe fn Move<'a, Param1: ::windows::runtime::IntoParam<'a, IWMDMStorage>, Param2: ::windows::runtime::IntoParam<'a, IWMDMProgress>>(&self, fumode: u32, ptargetobject: Param1, pprogress: Param2) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self), ::std::mem::transmute(fumode), ptargetobject.into_param().abi(), pprogress.into_param().abi()).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IWMDMStorageControl {
    type Vtable = IWMDMStorageControl_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(499857928, 13293, 4563, [132, 112, 0, 192, 79, 121, 219, 192]);
}
impl ::std::convert::From<IWMDMStorageControl> for ::windows::runtime::IUnknown {
    fn from(value: IWMDMStorageControl) -> Self {
        value.0
    }
}
impl ::std::convert::From<&IWMDMStorageControl> for ::windows::runtime::IUnknown {
    fn from(value: &IWMDMStorageControl) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IWMDMStorageControl {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IWMDMStorageControl {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMDMStorageControl_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, fumode: u32, pwszfile: super::super::Foundation::PWSTR, poperation: ::windows::runtime::RawPtr, pprogress: ::windows::runtime::RawPtr, ppnewobject: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, fumode: u32, pprogress: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, fumode: u32, pwsznewname: super::super::Foundation::PWSTR, pprogress: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, fumode: u32, pwszfile: super::super::Foundation::PWSTR, pprogress: ::windows::runtime::RawPtr, poperation: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, fumode: u32, ptargetobject: ::windows::runtime::RawPtr, pprogress: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_Media_DeviceManager`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IWMDMStorageControl2(pub ::windows::runtime::IUnknown);
impl IWMDMStorageControl2 {
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_DeviceManager`, `Win32_Foundation`*"]
    pub unsafe fn Insert<'a, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>, Param2: ::windows::runtime::IntoParam<'a, IWMDMOperation>, Param3: ::windows::runtime::IntoParam<'a, IWMDMProgress>>(&self, fumode: u32, pwszfile: Param1, poperation: Param2, pprogress: Param3) -> ::windows::runtime::Result<IWMDMStorage> {
        let mut result__: <IWMDMStorage as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), ::std::mem::transmute(fumode), pwszfile.into_param().abi(), poperation.into_param().abi(), pprogress.into_param().abi(), &mut result__).from_abi::<IWMDMStorage>(result__)
    }
    #[doc = "*Required features: `Win32_Media_DeviceManager`*"]
    pub unsafe fn Delete<'a, Param1: ::windows::runtime::IntoParam<'a, IWMDMProgress>>(&self, fumode: u32, pprogress: Param1) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), ::std::mem::transmute(fumode), pprogress.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_DeviceManager`, `Win32_Foundation`*"]
    pub unsafe fn Rename<'a, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>, Param2: ::windows::runtime::IntoParam<'a, IWMDMProgress>>(&self, fumode: u32, pwsznewname: Param1, pprogress: Param2) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self), ::std::mem::transmute(fumode), pwsznewname.into_param().abi(), pprogress.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_DeviceManager`, `Win32_Foundation`*"]
    pub unsafe fn Read<'a, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>, Param2: ::windows::runtime::IntoParam<'a, IWMDMProgress>, Param3: ::windows::runtime::IntoParam<'a, IWMDMOperation>>(&self, fumode: u32, pwszfile: Param1, pprogress: Param2, poperation: Param3) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::std::mem::transmute_copy(self), ::std::mem::transmute(fumode), pwszfile.into_param().abi(), pprogress.into_param().abi(), poperation.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_Media_DeviceManager`*"]
    pub unsafe fn Move<'a, Param1: ::windows::runtime::IntoParam<'a, IWMDMStorage>, Param2: ::windows::runtime::IntoParam<'a, IWMDMProgress>>(&self, fumode: u32, ptargetobject: Param1, pprogress: Param2) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self), ::std::mem::transmute(fumode), ptargetobject.into_param().abi(), pprogress.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_DeviceManager`, `Win32_Foundation`*"]
    pub unsafe fn Insert2<'a, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>, Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>, Param3: ::windows::runtime::IntoParam<'a, IWMDMOperation>, Param4: ::windows::runtime::IntoParam<'a, IWMDMProgress>, Param5: ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>>(
        &self,
        fumode: u32,
        pwszfilesource: Param1,
        pwszfiledest: Param2,
        poperation: Param3,
        pprogress: Param4,
        punknown: Param5,
        ppnewobject: *mut ::std::option::Option<IWMDMStorage>,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).8)(::std::mem::transmute_copy(self), ::std::mem::transmute(fumode), pwszfilesource.into_param().abi(), pwszfiledest.into_param().abi(), poperation.into_param().abi(), pprogress.into_param().abi(), punknown.into_param().abi(), ::std::mem::transmute(ppnewobject)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IWMDMStorageControl2 {
    type Vtable = IWMDMStorageControl2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2536255112, 48492, 16677, [142, 9, 132, 248, 55, 230, 55, 182]);
}
impl ::std::convert::From<IWMDMStorageControl2> for ::windows::runtime::IUnknown {
    fn from(value: IWMDMStorageControl2) -> Self {
        value.0
    }
}
impl ::std::convert::From<&IWMDMStorageControl2> for ::windows::runtime::IUnknown {
    fn from(value: &IWMDMStorageControl2) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IWMDMStorageControl2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IWMDMStorageControl2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::std::convert::From<IWMDMStorageControl2> for IWMDMStorageControl {
    fn from(value: IWMDMStorageControl2) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IWMDMStorageControl2> for IWMDMStorageControl {
    fn from(value: &IWMDMStorageControl2) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IWMDMStorageControl> for IWMDMStorageControl2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IWMDMStorageControl> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IWMDMStorageControl>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IWMDMStorageControl> for &IWMDMStorageControl2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IWMDMStorageControl> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IWMDMStorageControl>::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMDMStorageControl2_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, fumode: u32, pwszfile: super::super::Foundation::PWSTR, poperation: ::windows::runtime::RawPtr, pprogress: ::windows::runtime::RawPtr, ppnewobject: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, fumode: u32, pprogress: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, fumode: u32, pwsznewname: super::super::Foundation::PWSTR, pprogress: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, fumode: u32, pwszfile: super::super::Foundation::PWSTR, pprogress: ::windows::runtime::RawPtr, poperation: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, fumode: u32, ptargetobject: ::windows::runtime::RawPtr, pprogress: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, fumode: u32, pwszfilesource: super::super::Foundation::PWSTR, pwszfiledest: super::super::Foundation::PWSTR, poperation: ::windows::runtime::RawPtr, pprogress: ::windows::runtime::RawPtr, punknown: ::windows::runtime::RawPtr, ppnewobject: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[doc = "*Required features: `Win32_Media_DeviceManager`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IWMDMStorageControl3(pub ::windows::runtime::IUnknown);
impl IWMDMStorageControl3 {
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_DeviceManager`, `Win32_Foundation`*"]
    pub unsafe fn Insert<'a, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>, Param2: ::windows::runtime::IntoParam<'a, IWMDMOperation>, Param3: ::windows::runtime::IntoParam<'a, IWMDMProgress>>(&self, fumode: u32, pwszfile: Param1, poperation: Param2, pprogress: Param3) -> ::windows::runtime::Result<IWMDMStorage> {
        let mut result__: <IWMDMStorage as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), ::std::mem::transmute(fumode), pwszfile.into_param().abi(), poperation.into_param().abi(), pprogress.into_param().abi(), &mut result__).from_abi::<IWMDMStorage>(result__)
    }
    #[doc = "*Required features: `Win32_Media_DeviceManager`*"]
    pub unsafe fn Delete<'a, Param1: ::windows::runtime::IntoParam<'a, IWMDMProgress>>(&self, fumode: u32, pprogress: Param1) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), ::std::mem::transmute(fumode), pprogress.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_DeviceManager`, `Win32_Foundation`*"]
    pub unsafe fn Rename<'a, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>, Param2: ::windows::runtime::IntoParam<'a, IWMDMProgress>>(&self, fumode: u32, pwsznewname: Param1, pprogress: Param2) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self), ::std::mem::transmute(fumode), pwsznewname.into_param().abi(), pprogress.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_DeviceManager`, `Win32_Foundation`*"]
    pub unsafe fn Read<'a, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>, Param2: ::windows::runtime::IntoParam<'a, IWMDMProgress>, Param3: ::windows::runtime::IntoParam<'a, IWMDMOperation>>(&self, fumode: u32, pwszfile: Param1, pprogress: Param2, poperation: Param3) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::std::mem::transmute_copy(self), ::std::mem::transmute(fumode), pwszfile.into_param().abi(), pprogress.into_param().abi(), poperation.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_Media_DeviceManager`*"]
    pub unsafe fn Move<'a, Param1: ::windows::runtime::IntoParam<'a, IWMDMStorage>, Param2: ::windows::runtime::IntoParam<'a, IWMDMProgress>>(&self, fumode: u32, ptargetobject: Param1, pprogress: Param2) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self), ::std::mem::transmute(fumode), ptargetobject.into_param().abi(), pprogress.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_DeviceManager`, `Win32_Foundation`*"]
    pub unsafe fn Insert2<'a, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>, Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>, Param3: ::windows::runtime::IntoParam<'a, IWMDMOperation>, Param4: ::windows::runtime::IntoParam<'a, IWMDMProgress>, Param5: ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>>(
        &self,
        fumode: u32,
        pwszfilesource: Param1,
        pwszfiledest: Param2,
        poperation: Param3,
        pprogress: Param4,
        punknown: Param5,
        ppnewobject: *mut ::std::option::Option<IWMDMStorage>,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).8)(::std::mem::transmute_copy(self), ::std::mem::transmute(fumode), pwszfilesource.into_param().abi(), pwszfiledest.into_param().abi(), poperation.into_param().abi(), pprogress.into_param().abi(), punknown.into_param().abi(), ::std::mem::transmute(ppnewobject)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_DeviceManager`, `Win32_Foundation`*"]
    pub unsafe fn Insert3<'a, Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>, Param3: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>, Param4: ::windows::runtime::IntoParam<'a, IWMDMOperation>, Param5: ::windows::runtime::IntoParam<'a, IWMDMProgress>, Param6: ::windows::runtime::IntoParam<'a, IWMDMMetaData>, Param7: ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>>(
        &self,
        fumode: u32,
        futype: u32,
        pwszfilesource: Param2,
        pwszfiledest: Param3,
        poperation: Param4,
        pprogress: Param5,
        pmetadata: Param6,
        punknown: Param7,
        ppnewobject: *mut ::std::option::Option<IWMDMStorage>,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).9)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(fumode),
            ::std::mem::transmute(futype),
            pwszfilesource.into_param().abi(),
            pwszfiledest.into_param().abi(),
            poperation.into_param().abi(),
            pprogress.into_param().abi(),
            pmetadata.into_param().abi(),
            punknown.into_param().abi(),
            ::std::mem::transmute(ppnewobject),
        )
        .ok()
    }
}
unsafe impl ::windows::runtime::Interface for IWMDMStorageControl3 {
    type Vtable = IWMDMStorageControl3_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3005637477, 54515, 18070, [141, 83, 189, 39, 236, 96, 153, 58]);
}
impl ::std::convert::From<IWMDMStorageControl3> for ::windows::runtime::IUnknown {
    fn from(value: IWMDMStorageControl3) -> Self {
        value.0
    }
}
impl ::std::convert::From<&IWMDMStorageControl3> for ::windows::runtime::IUnknown {
    fn from(value: &IWMDMStorageControl3) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IWMDMStorageControl3 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IWMDMStorageControl3 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::std::convert::From<IWMDMStorageControl3> for IWMDMStorageControl2 {
    fn from(value: IWMDMStorageControl3) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IWMDMStorageControl3> for IWMDMStorageControl2 {
    fn from(value: &IWMDMStorageControl3) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IWMDMStorageControl2> for IWMDMStorageControl3 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IWMDMStorageControl2> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IWMDMStorageControl2>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IWMDMStorageControl2> for &IWMDMStorageControl3 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IWMDMStorageControl2> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IWMDMStorageControl2>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<IWMDMStorageControl3> for IWMDMStorageControl {
    fn from(value: IWMDMStorageControl3) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IWMDMStorageControl3> for IWMDMStorageControl {
    fn from(value: &IWMDMStorageControl3) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IWMDMStorageControl> for IWMDMStorageControl3 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IWMDMStorageControl> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IWMDMStorageControl>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IWMDMStorageControl> for &IWMDMStorageControl3 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IWMDMStorageControl> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IWMDMStorageControl>::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMDMStorageControl3_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, fumode: u32, pwszfile: super::super::Foundation::PWSTR, poperation: ::windows::runtime::RawPtr, pprogress: ::windows::runtime::RawPtr, ppnewobject: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, fumode: u32, pprogress: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, fumode: u32, pwsznewname: super::super::Foundation::PWSTR, pprogress: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, fumode: u32, pwszfile: super::super::Foundation::PWSTR, pprogress: ::windows::runtime::RawPtr, poperation: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, fumode: u32, ptargetobject: ::windows::runtime::RawPtr, pprogress: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, fumode: u32, pwszfilesource: super::super::Foundation::PWSTR, pwszfiledest: super::super::Foundation::PWSTR, poperation: ::windows::runtime::RawPtr, pprogress: ::windows::runtime::RawPtr, punknown: ::windows::runtime::RawPtr, ppnewobject: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, fumode: u32, futype: u32, pwszfilesource: super::super::Foundation::PWSTR, pwszfiledest: super::super::Foundation::PWSTR, poperation: ::windows::runtime::RawPtr, pprogress: ::windows::runtime::RawPtr, pmetadata: ::windows::runtime::RawPtr, punknown: ::windows::runtime::RawPtr, ppnewobject: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[doc = "*Required features: `Win32_Media_DeviceManager`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IWMDMStorageGlobals(pub ::windows::runtime::IUnknown);
impl IWMDMStorageGlobals {
    #[doc = "*Required features: `Win32_Media_DeviceManager`*"]
    pub unsafe fn GetCapabilities(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `Win32_Media_DeviceManager`*"]
    pub unsafe fn GetSerialNumber(&self, pserialnum: *mut WMDMID, abmac: *mut u8) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), ::std::mem::transmute(pserialnum), ::std::mem::transmute(abmac)).ok()
    }
    #[doc = "*Required features: `Win32_Media_DeviceManager`*"]
    pub unsafe fn GetTotalSize(&self, pdwtotalsizelow: *mut u32, pdwtotalsizehigh: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self), ::std::mem::transmute(pdwtotalsizelow), ::std::mem::transmute(pdwtotalsizehigh)).ok()
    }
    #[doc = "*Required features: `Win32_Media_DeviceManager`*"]
    pub unsafe fn GetTotalFree(&self, pdwfreelow: *mut u32, pdwfreehigh: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::std::mem::transmute_copy(self), ::std::mem::transmute(pdwfreelow), ::std::mem::transmute(pdwfreehigh)).ok()
    }
    #[doc = "*Required features: `Win32_Media_DeviceManager`*"]
    pub unsafe fn GetTotalBad(&self, pdwbadlow: *mut u32, pdwbadhigh: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self), ::std::mem::transmute(pdwbadlow), ::std::mem::transmute(pdwbadhigh)).ok()
    }
    #[doc = "*Required features: `Win32_Media_DeviceManager`*"]
    pub unsafe fn GetStatus(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).8)(::std::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `Win32_Media_DeviceManager`*"]
    pub unsafe fn Initialize<'a, Param1: ::windows::runtime::IntoParam<'a, IWMDMProgress>>(&self, fumode: u32, pprogress: Param1) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).9)(::std::mem::transmute_copy(self), ::std::mem::transmute(fumode), pprogress.into_param().abi()).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IWMDMStorageGlobals {
    type Vtable = IWMDMStorageGlobals_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(499857927, 13293, 4563, [132, 112, 0, 192, 79, 121, 219, 192]);
}
impl ::std::convert::From<IWMDMStorageGlobals> for ::windows::runtime::IUnknown {
    fn from(value: IWMDMStorageGlobals) -> Self {
        value.0
    }
}
impl ::std::convert::From<&IWMDMStorageGlobals> for ::windows::runtime::IUnknown {
    fn from(value: &IWMDMStorageGlobals) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IWMDMStorageGlobals {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IWMDMStorageGlobals {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMDMStorageGlobals_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pdwcapabilities: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pserialnum: *mut WMDMID, abmac: *mut u8) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pdwtotalsizelow: *mut u32, pdwtotalsizehigh: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pdwfreelow: *mut u32, pdwfreehigh: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pdwbadlow: *mut u32, pdwbadhigh: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pdwstatus: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, fumode: u32, pprogress: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_Media_DeviceManager`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IWMDeviceManager(pub ::windows::runtime::IUnknown);
impl IWMDeviceManager {
    #[doc = "*Required features: `Win32_Media_DeviceManager`*"]
    pub unsafe fn GetRevision(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `Win32_Media_DeviceManager`*"]
    pub unsafe fn GetDeviceCount(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `Win32_Media_DeviceManager`*"]
    pub unsafe fn EnumDevices(&self) -> ::windows::runtime::Result<IWMDMEnumDevice> {
        let mut result__: <IWMDMEnumDevice as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self), &mut result__).from_abi::<IWMDMEnumDevice>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IWMDeviceManager {
    type Vtable = IWMDeviceManager_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(499857920, 13293, 4563, [132, 112, 0, 192, 79, 121, 219, 192]);
}
impl ::std::convert::From<IWMDeviceManager> for ::windows::runtime::IUnknown {
    fn from(value: IWMDeviceManager) -> Self {
        value.0
    }
}
impl ::std::convert::From<&IWMDeviceManager> for ::windows::runtime::IUnknown {
    fn from(value: &IWMDeviceManager) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IWMDeviceManager {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IWMDeviceManager {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMDeviceManager_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pdwrevision: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pdwcount: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppenumdevice: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_Media_DeviceManager`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IWMDeviceManager2(pub ::windows::runtime::IUnknown);
impl IWMDeviceManager2 {
    #[doc = "*Required features: `Win32_Media_DeviceManager`*"]
    pub unsafe fn GetRevision(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `Win32_Media_DeviceManager`*"]
    pub unsafe fn GetDeviceCount(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `Win32_Media_DeviceManager`*"]
    pub unsafe fn EnumDevices(&self) -> ::windows::runtime::Result<IWMDMEnumDevice> {
        let mut result__: <IWMDMEnumDevice as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self), &mut result__).from_abi::<IWMDMEnumDevice>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_DeviceManager`, `Win32_Foundation`*"]
    pub unsafe fn GetDeviceFromCanonicalName<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, pwszcanonicalname: Param0) -> ::windows::runtime::Result<IWMDMDevice> {
        let mut result__: <IWMDMDevice as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).6)(::std::mem::transmute_copy(self), pwszcanonicalname.into_param().abi(), &mut result__).from_abi::<IWMDMDevice>(result__)
    }
    #[doc = "*Required features: `Win32_Media_DeviceManager`*"]
    pub unsafe fn EnumDevices2(&self) -> ::windows::runtime::Result<IWMDMEnumDevice> {
        let mut result__: <IWMDMEnumDevice as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self), &mut result__).from_abi::<IWMDMEnumDevice>(result__)
    }
    #[doc = "*Required features: `Win32_Media_DeviceManager`*"]
    pub unsafe fn Reinitialize(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).8)(::std::mem::transmute_copy(self)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IWMDeviceManager2 {
    type Vtable = IWMDeviceManager2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2453557833, 34609, 19547, [155, 28, 184, 182, 11, 110, 70, 175]);
}
impl ::std::convert::From<IWMDeviceManager2> for ::windows::runtime::IUnknown {
    fn from(value: IWMDeviceManager2) -> Self {
        value.0
    }
}
impl ::std::convert::From<&IWMDeviceManager2> for ::windows::runtime::IUnknown {
    fn from(value: &IWMDeviceManager2) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IWMDeviceManager2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IWMDeviceManager2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::std::convert::From<IWMDeviceManager2> for IWMDeviceManager {
    fn from(value: IWMDeviceManager2) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IWMDeviceManager2> for IWMDeviceManager {
    fn from(value: &IWMDeviceManager2) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IWMDeviceManager> for IWMDeviceManager2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IWMDeviceManager> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IWMDeviceManager>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IWMDeviceManager> for &IWMDeviceManager2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IWMDeviceManager> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IWMDeviceManager>::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMDeviceManager2_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pdwrevision: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pdwcount: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppenumdevice: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pwszcanonicalname: super::super::Foundation::PWSTR, ppdevice: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppenumdevice: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_Media_DeviceManager`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IWMDeviceManager3(pub ::windows::runtime::IUnknown);
impl IWMDeviceManager3 {
    #[doc = "*Required features: `Win32_Media_DeviceManager`*"]
    pub unsafe fn GetRevision(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `Win32_Media_DeviceManager`*"]
    pub unsafe fn GetDeviceCount(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `Win32_Media_DeviceManager`*"]
    pub unsafe fn EnumDevices(&self) -> ::windows::runtime::Result<IWMDMEnumDevice> {
        let mut result__: <IWMDMEnumDevice as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self), &mut result__).from_abi::<IWMDMEnumDevice>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_DeviceManager`, `Win32_Foundation`*"]
    pub unsafe fn GetDeviceFromCanonicalName<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, pwszcanonicalname: Param0) -> ::windows::runtime::Result<IWMDMDevice> {
        let mut result__: <IWMDMDevice as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).6)(::std::mem::transmute_copy(self), pwszcanonicalname.into_param().abi(), &mut result__).from_abi::<IWMDMDevice>(result__)
    }
    #[doc = "*Required features: `Win32_Media_DeviceManager`*"]
    pub unsafe fn EnumDevices2(&self) -> ::windows::runtime::Result<IWMDMEnumDevice> {
        let mut result__: <IWMDMEnumDevice as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self), &mut result__).from_abi::<IWMDMEnumDevice>(result__)
    }
    #[doc = "*Required features: `Win32_Media_DeviceManager`*"]
    pub unsafe fn Reinitialize(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).8)(::std::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `Win32_Media_DeviceManager`*"]
    pub unsafe fn SetDeviceEnumPreference(&self, dwenumpref: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).9)(::std::mem::transmute_copy(self), ::std::mem::transmute(dwenumpref)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IWMDeviceManager3 {
    type Vtable = IWMDeviceManager3_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2937609281, 4109, 18157, [190, 46, 156, 232, 196, 69, 148, 239]);
}
impl ::std::convert::From<IWMDeviceManager3> for ::windows::runtime::IUnknown {
    fn from(value: IWMDeviceManager3) -> Self {
        value.0
    }
}
impl ::std::convert::From<&IWMDeviceManager3> for ::windows::runtime::IUnknown {
    fn from(value: &IWMDeviceManager3) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IWMDeviceManager3 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IWMDeviceManager3 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::std::convert::From<IWMDeviceManager3> for IWMDeviceManager2 {
    fn from(value: IWMDeviceManager3) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IWMDeviceManager3> for IWMDeviceManager2 {
    fn from(value: &IWMDeviceManager3) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IWMDeviceManager2> for IWMDeviceManager3 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IWMDeviceManager2> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IWMDeviceManager2>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IWMDeviceManager2> for &IWMDeviceManager3 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IWMDeviceManager2> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IWMDeviceManager2>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<IWMDeviceManager3> for IWMDeviceManager {
    fn from(value: IWMDeviceManager3) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IWMDeviceManager3> for IWMDeviceManager {
    fn from(value: &IWMDeviceManager3) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IWMDeviceManager> for IWMDeviceManager3 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IWMDeviceManager> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IWMDeviceManager>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IWMDeviceManager> for &IWMDeviceManager3 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IWMDeviceManager> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IWMDeviceManager>::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMDeviceManager3_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pdwrevision: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pdwcount: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppenumdevice: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pwszcanonicalname: super::super::Foundation::PWSTR, ppdevice: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppenumdevice: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwenumpref: u32) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_Media_DeviceManager`*"]
pub const MDSP_READ: u32 = 1u32;
#[doc = "*Required features: `Win32_Media_DeviceManager`*"]
pub const MDSP_SEEK_BOF: u32 = 1u32;
#[doc = "*Required features: `Win32_Media_DeviceManager`*"]
pub const MDSP_SEEK_CUR: u32 = 2u32;
#[doc = "*Required features: `Win32_Media_DeviceManager`*"]
pub const MDSP_SEEK_EOF: u32 = 4u32;
#[doc = "*Required features: `Win32_Media_DeviceManager`*"]
pub const MDSP_WRITE: u32 = 2u32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C, packed(1))]
#[doc = "*Required features: `Win32_Media_DeviceManager`*"]
pub struct MTP_COMMAND_DATA_IN {
    pub OpCode: u16,
    pub NumParams: u32,
    pub Params: [u32; 5],
    pub NextPhase: u32,
    pub CommandWriteDataSize: u32,
    pub CommandWriteData: [u8; 1],
}
impl MTP_COMMAND_DATA_IN {}
impl ::std::default::Default for MTP_COMMAND_DATA_IN {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::cmp::PartialEq for MTP_COMMAND_DATA_IN {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::std::cmp::Eq for MTP_COMMAND_DATA_IN {}
unsafe impl ::windows::runtime::Abi for MTP_COMMAND_DATA_IN {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C, packed(1))]
#[doc = "*Required features: `Win32_Media_DeviceManager`*"]
pub struct MTP_COMMAND_DATA_OUT {
    pub ResponseCode: u16,
    pub NumParams: u32,
    pub Params: [u32; 5],
    pub CommandReadDataSize: u32,
    pub CommandReadData: [u8; 1],
}
impl MTP_COMMAND_DATA_OUT {}
impl ::std::default::Default for MTP_COMMAND_DATA_OUT {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::cmp::PartialEq for MTP_COMMAND_DATA_OUT {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::std::cmp::Eq for MTP_COMMAND_DATA_OUT {}
unsafe impl ::windows::runtime::Abi for MTP_COMMAND_DATA_OUT {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Media_DeviceManager`*"]
pub const MTP_COMMAND_MAX_PARAMS: u32 = 5u32;
#[doc = "*Required features: `Win32_Media_DeviceManager`*"]
pub const MTP_NEXTPHASE_NO_DATA: u32 = 3u32;
#[doc = "*Required features: `Win32_Media_DeviceManager`*"]
pub const MTP_NEXTPHASE_READ_DATA: u32 = 1u32;
#[doc = "*Required features: `Win32_Media_DeviceManager`*"]
pub const MTP_NEXTPHASE_WRITE_DATA: u32 = 2u32;
#[doc = "*Required features: `Win32_Media_DeviceManager`*"]
pub const MTP_RESPONSE_MAX_PARAMS: u32 = 5u32;
#[doc = "*Required features: `Win32_Media_DeviceManager`*"]
pub const MTP_RESPONSE_OK: u16 = 8193u16;
pub const MediaDevMgr: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(632991105, 13664, 4563, [132, 113, 0, 192, 79, 121, 219, 192]);
pub const MediaDevMgrClassFactory: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1342442525, 48575, 18724, [184, 115, 241, 77, 108, 91, 253, 102]);
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Media_DeviceManager`*"]
pub struct OPAQUECOMMAND {
    pub guidCommand: ::windows::runtime::GUID,
    pub dwDataLen: u32,
    pub pData: *mut u8,
    pub abMAC: [u8; 20],
}
impl OPAQUECOMMAND {}
impl ::std::default::Default for OPAQUECOMMAND {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for OPAQUECOMMAND {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("OPAQUECOMMAND").field("guidCommand", &self.guidCommand).field("dwDataLen", &self.dwDataLen).field("pData", &self.pData).field("abMAC", &self.abMAC).finish()
    }
}
impl ::std::cmp::PartialEq for OPAQUECOMMAND {
    fn eq(&self, other: &Self) -> bool {
        self.guidCommand == other.guidCommand && self.dwDataLen == other.dwDataLen && self.pData == other.pData && self.abMAC == other.abMAC
    }
}
impl ::std::cmp::Eq for OPAQUECOMMAND {}
unsafe impl ::windows::runtime::Abi for OPAQUECOMMAND {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Media_DeviceManager`*"]
pub const RSA_KEY_LEN: u32 = 64u32;
#[doc = "*Required features: `Win32_Media_DeviceManager`*"]
pub const SAC_CERT_V1: u32 = 2u32;
#[doc = "*Required features: `Win32_Media_DeviceManager`*"]
pub const SAC_CERT_X509: u32 = 1u32;
#[doc = "*Required features: `Win32_Media_DeviceManager`*"]
pub const SAC_MAC_LEN: u32 = 8u32;
#[doc = "*Required features: `Win32_Media_DeviceManager`*"]
pub const SAC_PROTOCOL_V1: u32 = 2u32;
#[doc = "*Required features: `Win32_Media_DeviceManager`*"]
pub const SAC_PROTOCOL_WMDM: u32 = 1u32;
#[doc = "*Required features: `Win32_Media_DeviceManager`*"]
pub const SAC_SESSION_KEYLEN: u32 = 8u32;
pub const SCP_EVENTID_ACQSECURECLOCK: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2250542281, 19033, 17378, [145, 70, 72, 167, 243, 244, 20, 12]);
pub const SCP_EVENTID_DRMINFO: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(557699719, 16850, 17195, [158, 63, 59, 79, 123, 53, 129, 221]);
pub const SCP_EVENTID_NEEDTOINDIV: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2275739591, 46185, 17286, [185, 118, 213, 209, 206, 83, 138, 111]);
pub const SCP_PARAMID_DRMVERSION: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1104155997, 31943, 16919, [173, 169, 0, 80, 116, 98, 77, 164]);
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Media_DeviceManager`*"]
pub struct WMDMDATETIME {
    pub wYear: u16,
    pub wMonth: u16,
    pub wDay: u16,
    pub wHour: u16,
    pub wMinute: u16,
    pub wSecond: u16,
}
impl WMDMDATETIME {}
impl ::std::default::Default for WMDMDATETIME {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for WMDMDATETIME {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("WMDMDATETIME").field("wYear", &self.wYear).field("wMonth", &self.wMonth).field("wDay", &self.wDay).field("wHour", &self.wHour).field("wMinute", &self.wMinute).field("wSecond", &self.wSecond).finish()
    }
}
impl ::std::cmp::PartialEq for WMDMDATETIME {
    fn eq(&self, other: &Self) -> bool {
        self.wYear == other.wYear && self.wMonth == other.wMonth && self.wDay == other.wDay && self.wHour == other.wHour && self.wMinute == other.wMinute && self.wSecond == other.wSecond
    }
}
impl ::std::cmp::Eq for WMDMDATETIME {}
unsafe impl ::windows::runtime::Abi for WMDMDATETIME {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Media_DeviceManager`*"]
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
impl WMDMDetermineMaxPropStringLen {}
impl ::std::default::Default for WMDMDetermineMaxPropStringLen {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::cmp::PartialEq for WMDMDetermineMaxPropStringLen {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::std::cmp::Eq for WMDMDetermineMaxPropStringLen {}
unsafe impl ::windows::runtime::Abi for WMDMDetermineMaxPropStringLen {
    type Abi = Self;
}
pub const WMDMDevice: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2155560159, 13690, 4563, [132, 113, 0, 192, 79, 121, 219, 192]);
pub const WMDMDeviceEnum: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1125004719, 14705, 4563, [132, 116, 0, 192, 79, 121, 219, 192]);
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Media_DeviceManager`*"]
pub struct WMDMID {
    pub cbSize: u32,
    pub dwVendorID: u32,
    pub pID: [u8; 128],
    pub SerialNumberLength: u32,
}
impl WMDMID {}
impl ::std::default::Default for WMDMID {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for WMDMID {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("WMDMID").field("cbSize", &self.cbSize).field("dwVendorID", &self.dwVendorID).field("pID", &self.pID).field("SerialNumberLength", &self.SerialNumberLength).finish()
    }
}
impl ::std::cmp::PartialEq for WMDMID {
    fn eq(&self, other: &Self) -> bool {
        self.cbSize == other.cbSize && self.dwVendorID == other.dwVendorID && self.pID == other.pID && self.SerialNumberLength == other.SerialNumberLength
    }
}
impl ::std::cmp::Eq for WMDMID {}
unsafe impl ::windows::runtime::Abi for WMDMID {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Media_DeviceManager`*"]
pub const WMDMID_LENGTH: u32 = 128u32;
pub const WMDMLogger: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(285880834, 23161, 4563, [141, 120, 68, 69, 83, 84, 0, 0]);
#[doc = "*Required features: `Win32_Media_DeviceManager`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct WMDMMessage(pub i32);
pub const WMDM_MSG_DEVICE_ARRIVAL: WMDMMessage = WMDMMessage(0i32);
pub const WMDM_MSG_DEVICE_REMOVAL: WMDMMessage = WMDMMessage(1i32);
pub const WMDM_MSG_MEDIA_ARRIVAL: WMDMMessage = WMDMMessage(2i32);
pub const WMDM_MSG_MEDIA_REMOVAL: WMDMMessage = WMDMMessage(3i32);
impl ::std::convert::From<i32> for WMDMMessage {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for WMDMMessage {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_Media_DeviceManager`, `Win32_Foundation`*"]
pub struct WMDMMetadataView {
    pub pwszViewName: super::super::Foundation::PWSTR,
    pub nDepth: u32,
    pub ppwszTags: *mut *mut u16,
}
#[cfg(feature = "Win32_Foundation")]
impl WMDMMetadataView {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for WMDMMetadataView {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for WMDMMetadataView {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("WMDMMetadataView").field("pwszViewName", &self.pwszViewName).field("nDepth", &self.nDepth).field("ppwszTags", &self.ppwszTags).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for WMDMMetadataView {
    fn eq(&self, other: &Self) -> bool {
        self.pwszViewName == other.pwszViewName && self.nDepth == other.nDepth && self.ppwszTags == other.ppwszTags
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for WMDMMetadataView {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for WMDMMetadataView {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Media_DeviceManager`*"]
pub struct WMDMRIGHTS {
    pub cbSize: u32,
    pub dwContentType: u32,
    pub fuFlags: u32,
    pub fuRights: u32,
    pub dwAppSec: u32,
    pub dwPlaybackCount: u32,
    pub ExpirationDate: WMDMDATETIME,
}
impl WMDMRIGHTS {}
impl ::std::default::Default for WMDMRIGHTS {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for WMDMRIGHTS {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("WMDMRIGHTS")
            .field("cbSize", &self.cbSize)
            .field("dwContentType", &self.dwContentType)
            .field("fuFlags", &self.fuFlags)
            .field("fuRights", &self.fuRights)
            .field("dwAppSec", &self.dwAppSec)
            .field("dwPlaybackCount", &self.dwPlaybackCount)
            .field("ExpirationDate", &self.ExpirationDate)
            .finish()
    }
}
impl ::std::cmp::PartialEq for WMDMRIGHTS {
    fn eq(&self, other: &Self) -> bool {
        self.cbSize == other.cbSize && self.dwContentType == other.dwContentType && self.fuFlags == other.fuFlags && self.fuRights == other.fuRights && self.dwAppSec == other.dwAppSec && self.dwPlaybackCount == other.dwPlaybackCount && self.ExpirationDate == other.ExpirationDate
    }
}
impl ::std::cmp::Eq for WMDMRIGHTS {}
unsafe impl ::windows::runtime::Abi for WMDMRIGHTS {
    type Abi = Self;
}
pub const WMDMStorage: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2155560160, 13690, 4563, [132, 113, 0, 192, 79, 121, 219, 192]);
pub const WMDMStorageEnum: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3946846779, 15095, 4563, [132, 116, 0, 192, 79, 121, 219, 192]);
pub const WMDMStorageGlobal: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2155560161, 13690, 4563, [132, 113, 0, 192, 79, 121, 219, 192]);
#[doc = "*Required features: `Win32_Media_DeviceManager`*"]
pub const WMDM_APP_REVOKED: u32 = 2u32;
#[doc = "*Required features: `Win32_Media_DeviceManager`*"]
pub const WMDM_CONTENT_FILE: u32 = 4u32;
#[doc = "*Required features: `Win32_Media_DeviceManager`*"]
pub const WMDM_CONTENT_FOLDER: u32 = 8u32;
#[doc = "*Required features: `Win32_Media_DeviceManager`*"]
pub const WMDM_CONTENT_OPERATIONINTERFACE: u32 = 16u32;
#[doc = "*Required features: `Win32_Media_DeviceManager`*"]
pub const WMDM_DEVICECAP_CANPAUSE: u32 = 16u32;
#[doc = "*Required features: `Win32_Media_DeviceManager`*"]
pub const WMDM_DEVICECAP_CANPLAY: u32 = 1u32;
#[doc = "*Required features: `Win32_Media_DeviceManager`*"]
pub const WMDM_DEVICECAP_CANRECORD: u32 = 4u32;
#[doc = "*Required features: `Win32_Media_DeviceManager`*"]
pub const WMDM_DEVICECAP_CANRESUME: u32 = 32u32;
#[doc = "*Required features: `Win32_Media_DeviceManager`*"]
pub const WMDM_DEVICECAP_CANSEEK: u32 = 128u32;
#[doc = "*Required features: `Win32_Media_DeviceManager`*"]
pub const WMDM_DEVICECAP_CANSTOP: u32 = 64u32;
#[doc = "*Required features: `Win32_Media_DeviceManager`*"]
pub const WMDM_DEVICECAP_CANSTREAMPLAY: u32 = 2u32;
#[doc = "*Required features: `Win32_Media_DeviceManager`*"]
pub const WMDM_DEVICECAP_CANSTREAMRECORD: u32 = 8u32;
#[doc = "*Required features: `Win32_Media_DeviceManager`*"]
pub const WMDM_DEVICECAP_HASSECURECLOCK: u32 = 256u32;
pub const WMDM_DEVICE_PROTOCOL_MSC: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2765275756, 43137, 17595, [189, 93, 31, 112, 60, 113, 247, 169]);
pub const WMDM_DEVICE_PROTOCOL_MTP: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2543736037, 2812, 17924, [141, 147, 220, 121, 138, 75, 207, 69]);
pub const WMDM_DEVICE_PROTOCOL_RAPI: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(705818001, 35983, 16868, [130, 209, 131, 134, 224, 3, 86, 28]);
#[doc = "*Required features: `Win32_Media_DeviceManager`*"]
pub const WMDM_DEVICE_TYPE_DECODE: u32 = 4u32;
#[doc = "*Required features: `Win32_Media_DeviceManager`*"]
pub const WMDM_DEVICE_TYPE_ENCODE: u32 = 8u32;
#[doc = "*Required features: `Win32_Media_DeviceManager`*"]
pub const WMDM_DEVICE_TYPE_FILELISTRESYNC: u32 = 512u32;
#[doc = "*Required features: `Win32_Media_DeviceManager`*"]
pub const WMDM_DEVICE_TYPE_NONREENTRANT: u32 = 256u32;
#[doc = "*Required features: `Win32_Media_DeviceManager`*"]
pub const WMDM_DEVICE_TYPE_NONSDMI: u32 = 128u32;
#[doc = "*Required features: `Win32_Media_DeviceManager`*"]
pub const WMDM_DEVICE_TYPE_PLAYBACK: u32 = 1u32;
#[doc = "*Required features: `Win32_Media_DeviceManager`*"]
pub const WMDM_DEVICE_TYPE_RECORD: u32 = 2u32;
#[doc = "*Required features: `Win32_Media_DeviceManager`*"]
pub const WMDM_DEVICE_TYPE_SDMI: u32 = 64u32;
#[doc = "*Required features: `Win32_Media_DeviceManager`*"]
pub const WMDM_DEVICE_TYPE_STORAGE: u32 = 16u32;
#[doc = "*Required features: `Win32_Media_DeviceManager`*"]
pub const WMDM_DEVICE_TYPE_VIEW_PREF_METADATAVIEW: u32 = 1024u32;
#[doc = "*Required features: `Win32_Media_DeviceManager`*"]
pub const WMDM_DEVICE_TYPE_VIRTUAL: u32 = 32u32;
#[doc = "*Required features: `Win32_Media_DeviceManager`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct WMDM_ENUM_PROP_VALID_VALUES_FORM(pub i32);
pub const WMDM_ENUM_PROP_VALID_VALUES_ANY: WMDM_ENUM_PROP_VALID_VALUES_FORM = WMDM_ENUM_PROP_VALID_VALUES_FORM(0i32);
pub const WMDM_ENUM_PROP_VALID_VALUES_RANGE: WMDM_ENUM_PROP_VALID_VALUES_FORM = WMDM_ENUM_PROP_VALID_VALUES_FORM(1i32);
pub const WMDM_ENUM_PROP_VALID_VALUES_ENUM: WMDM_ENUM_PROP_VALID_VALUES_FORM = WMDM_ENUM_PROP_VALID_VALUES_FORM(2i32);
impl ::std::convert::From<i32> for WMDM_ENUM_PROP_VALID_VALUES_FORM {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for WMDM_ENUM_PROP_VALID_VALUES_FORM {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Media_DeviceManager`*"]
pub const WMDM_E_BUFFERTOOSMALL: i32 = -2147201016i32;
#[doc = "*Required features: `Win32_Media_DeviceManager`*"]
pub const WMDM_E_BUSY: i32 = -2147201024i32;
#[doc = "*Required features: `Win32_Media_DeviceManager`*"]
pub const WMDM_E_CALL_OUT_OF_SEQUENCE: i32 = -2147201017i32;
#[doc = "*Required features: `Win32_Media_DeviceManager`*"]
pub const WMDM_E_CANTOPEN_PMSN_SERVICE_PIPE: i32 = -2147201005i32;
#[doc = "*Required features: `Win32_Media_DeviceManager`*"]
pub const WMDM_E_INCORRECT_APPSEC: i32 = -2147201008i32;
#[doc = "*Required features: `Win32_Media_DeviceManager`*"]
pub const WMDM_E_INCORRECT_RIGHTS: i32 = -2147201007i32;
#[doc = "*Required features: `Win32_Media_DeviceManager`*"]
pub const WMDM_E_INTERFACEDEAD: i32 = -2147201023i32;
#[doc = "*Required features: `Win32_Media_DeviceManager`*"]
pub const WMDM_E_INVALIDTYPE: i32 = -2147201022i32;
#[doc = "*Required features: `Win32_Media_DeviceManager`*"]
pub const WMDM_E_LICENSE_EXPIRED: i32 = -2147201006i32;
#[doc = "*Required features: `Win32_Media_DeviceManager`*"]
pub const WMDM_E_LICENSE_NOTEXIST: i32 = -2147201009i32;
#[doc = "*Required features: `Win32_Media_DeviceManager`*"]
pub const WMDM_E_MAC_CHECK_FAILED: i32 = -2147201014i32;
#[doc = "*Required features: `Win32_Media_DeviceManager`*"]
pub const WMDM_E_MOREDATA: i32 = -2147201015i32;
#[doc = "*Required features: `Win32_Media_DeviceManager`*"]
pub const WMDM_E_NORIGHTS: i32 = -2147201018i32;
#[doc = "*Required features: `Win32_Media_DeviceManager`*"]
pub const WMDM_E_NOTCERTIFIED: i32 = -2147201019i32;
#[doc = "*Required features: `Win32_Media_DeviceManager`*"]
pub const WMDM_E_NOTSUPPORTED: i32 = -2147201020i32;
#[doc = "*Required features: `Win32_Media_DeviceManager`*"]
pub const WMDM_E_PROCESSFAILED: i32 = -2147201021i32;
#[doc = "*Required features: `Win32_Media_DeviceManager`*"]
pub const WMDM_E_REVOKED: i32 = -2147201010i32;
#[doc = "*Required features: `Win32_Media_DeviceManager`*"]
pub const WMDM_E_SDMI_NOMORECOPIES: i32 = -2147201011i32;
#[doc = "*Required features: `Win32_Media_DeviceManager`*"]
pub const WMDM_E_SDMI_TRIGGER: i32 = -2147201012i32;
#[doc = "*Required features: `Win32_Media_DeviceManager`*"]
pub const WMDM_E_TOO_MANY_SESSIONS: i32 = -2147201005i32;
#[doc = "*Required features: `Win32_Media_DeviceManager`*"]
pub const WMDM_E_USER_CANCELLED: i32 = -2147201013i32;
#[doc = "*Required features: `Win32_Media_DeviceManager`*"]
pub const WMDM_FILE_ATTR_AUDIO: u32 = 4096u32;
#[doc = "*Required features: `Win32_Media_DeviceManager`*"]
pub const WMDM_FILE_ATTR_AUDIOBOOK: u32 = 2097152u32;
#[doc = "*Required features: `Win32_Media_DeviceManager`*"]
pub const WMDM_FILE_ATTR_CANDELETE: u32 = 32768u32;
#[doc = "*Required features: `Win32_Media_DeviceManager`*"]
pub const WMDM_FILE_ATTR_CANMOVE: u32 = 65536u32;
#[doc = "*Required features: `Win32_Media_DeviceManager`*"]
pub const WMDM_FILE_ATTR_CANPLAY: u32 = 16384u32;
#[doc = "*Required features: `Win32_Media_DeviceManager`*"]
pub const WMDM_FILE_ATTR_CANREAD: u32 = 262144u32;
#[doc = "*Required features: `Win32_Media_DeviceManager`*"]
pub const WMDM_FILE_ATTR_CANRENAME: u32 = 131072u32;
#[doc = "*Required features: `Win32_Media_DeviceManager`*"]
pub const WMDM_FILE_ATTR_DATA: u32 = 8192u32;
#[doc = "*Required features: `Win32_Media_DeviceManager`*"]
pub const WMDM_FILE_ATTR_FILE: u32 = 32u32;
#[doc = "*Required features: `Win32_Media_DeviceManager`*"]
pub const WMDM_FILE_ATTR_FOLDER: u32 = 8u32;
#[doc = "*Required features: `Win32_Media_DeviceManager`*"]
pub const WMDM_FILE_ATTR_HIDDEN: u32 = 4194304u32;
#[doc = "*Required features: `Win32_Media_DeviceManager`*"]
pub const WMDM_FILE_ATTR_LINK: u32 = 16u32;
#[doc = "*Required features: `Win32_Media_DeviceManager`*"]
pub const WMDM_FILE_ATTR_MUSIC: u32 = 524288u32;
#[doc = "*Required features: `Win32_Media_DeviceManager`*"]
pub const WMDM_FILE_ATTR_READONLY: u32 = 16777216u32;
#[doc = "*Required features: `Win32_Media_DeviceManager`*"]
pub const WMDM_FILE_ATTR_SYSTEM: u32 = 8388608u32;
#[doc = "*Required features: `Win32_Media_DeviceManager`*"]
pub const WMDM_FILE_ATTR_VIDEO: u32 = 64u32;
#[doc = "*Required features: `Win32_Media_DeviceManager`*"]
pub const WMDM_FILE_CREATE_OVERWRITE: u32 = 1048576u32;
#[doc = "*Required features: `Win32_Media_DeviceManager`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct WMDM_FIND_SCOPE(pub i32);
pub const WMDM_FIND_SCOPE_GLOBAL: WMDM_FIND_SCOPE = WMDM_FIND_SCOPE(0i32);
pub const WMDM_FIND_SCOPE_IMMEDIATE_CHILDREN: WMDM_FIND_SCOPE = WMDM_FIND_SCOPE(1i32);
impl ::std::convert::From<i32> for WMDM_FIND_SCOPE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for WMDM_FIND_SCOPE {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Media_DeviceManager`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct WMDM_FORMATCODE(pub i32);
pub const WMDM_FORMATCODE_NOTUSED: WMDM_FORMATCODE = WMDM_FORMATCODE(0i32);
pub const WMDM_FORMATCODE_ALLIMAGES: WMDM_FORMATCODE = WMDM_FORMATCODE(-1i32);
pub const WMDM_FORMATCODE_UNDEFINED: WMDM_FORMATCODE = WMDM_FORMATCODE(12288i32);
pub const WMDM_FORMATCODE_ASSOCIATION: WMDM_FORMATCODE = WMDM_FORMATCODE(12289i32);
pub const WMDM_FORMATCODE_SCRIPT: WMDM_FORMATCODE = WMDM_FORMATCODE(12290i32);
pub const WMDM_FORMATCODE_EXECUTABLE: WMDM_FORMATCODE = WMDM_FORMATCODE(12291i32);
pub const WMDM_FORMATCODE_TEXT: WMDM_FORMATCODE = WMDM_FORMATCODE(12292i32);
pub const WMDM_FORMATCODE_HTML: WMDM_FORMATCODE = WMDM_FORMATCODE(12293i32);
pub const WMDM_FORMATCODE_DPOF: WMDM_FORMATCODE = WMDM_FORMATCODE(12294i32);
pub const WMDM_FORMATCODE_AIFF: WMDM_FORMATCODE = WMDM_FORMATCODE(12295i32);
pub const WMDM_FORMATCODE_WAVE: WMDM_FORMATCODE = WMDM_FORMATCODE(12296i32);
pub const WMDM_FORMATCODE_MP3: WMDM_FORMATCODE = WMDM_FORMATCODE(12297i32);
pub const WMDM_FORMATCODE_AVI: WMDM_FORMATCODE = WMDM_FORMATCODE(12298i32);
pub const WMDM_FORMATCODE_MPEG: WMDM_FORMATCODE = WMDM_FORMATCODE(12299i32);
pub const WMDM_FORMATCODE_ASF: WMDM_FORMATCODE = WMDM_FORMATCODE(12300i32);
pub const WMDM_FORMATCODE_RESERVED_FIRST: WMDM_FORMATCODE = WMDM_FORMATCODE(12301i32);
pub const WMDM_FORMATCODE_RESERVED_LAST: WMDM_FORMATCODE = WMDM_FORMATCODE(14335i32);
pub const WMDM_FORMATCODE_IMAGE_UNDEFINED: WMDM_FORMATCODE = WMDM_FORMATCODE(14336i32);
pub const WMDM_FORMATCODE_IMAGE_EXIF: WMDM_FORMATCODE = WMDM_FORMATCODE(14337i32);
pub const WMDM_FORMATCODE_IMAGE_TIFFEP: WMDM_FORMATCODE = WMDM_FORMATCODE(14338i32);
pub const WMDM_FORMATCODE_IMAGE_FLASHPIX: WMDM_FORMATCODE = WMDM_FORMATCODE(14339i32);
pub const WMDM_FORMATCODE_IMAGE_BMP: WMDM_FORMATCODE = WMDM_FORMATCODE(14340i32);
pub const WMDM_FORMATCODE_IMAGE_CIFF: WMDM_FORMATCODE = WMDM_FORMATCODE(14341i32);
pub const WMDM_FORMATCODE_IMAGE_GIF: WMDM_FORMATCODE = WMDM_FORMATCODE(14343i32);
pub const WMDM_FORMATCODE_IMAGE_JFIF: WMDM_FORMATCODE = WMDM_FORMATCODE(14344i32);
pub const WMDM_FORMATCODE_IMAGE_PCD: WMDM_FORMATCODE = WMDM_FORMATCODE(14345i32);
pub const WMDM_FORMATCODE_IMAGE_PICT: WMDM_FORMATCODE = WMDM_FORMATCODE(14346i32);
pub const WMDM_FORMATCODE_IMAGE_PNG: WMDM_FORMATCODE = WMDM_FORMATCODE(14347i32);
pub const WMDM_FORMATCODE_IMAGE_TIFF: WMDM_FORMATCODE = WMDM_FORMATCODE(14349i32);
pub const WMDM_FORMATCODE_IMAGE_TIFFIT: WMDM_FORMATCODE = WMDM_FORMATCODE(14350i32);
pub const WMDM_FORMATCODE_IMAGE_JP2: WMDM_FORMATCODE = WMDM_FORMATCODE(14351i32);
pub const WMDM_FORMATCODE_IMAGE_JPX: WMDM_FORMATCODE = WMDM_FORMATCODE(14352i32);
pub const WMDM_FORMATCODE_IMAGE_RESERVED_FIRST: WMDM_FORMATCODE = WMDM_FORMATCODE(14353i32);
pub const WMDM_FORMATCODE_IMAGE_RESERVED_LAST: WMDM_FORMATCODE = WMDM_FORMATCODE(16383i32);
pub const WMDM_FORMATCODE_UNDEFINEDFIRMWARE: WMDM_FORMATCODE = WMDM_FORMATCODE(47106i32);
pub const WMDM_FORMATCODE_WBMP: WMDM_FORMATCODE = WMDM_FORMATCODE(47107i32);
pub const WMDM_FORMATCODE_JPEGXR: WMDM_FORMATCODE = WMDM_FORMATCODE(47108i32);
pub const WMDM_FORMATCODE_WINDOWSIMAGEFORMAT: WMDM_FORMATCODE = WMDM_FORMATCODE(47233i32);
pub const WMDM_FORMATCODE_UNDEFINEDAUDIO: WMDM_FORMATCODE = WMDM_FORMATCODE(47360i32);
pub const WMDM_FORMATCODE_WMA: WMDM_FORMATCODE = WMDM_FORMATCODE(47361i32);
pub const WMDM_FORMATCODE_OGG: WMDM_FORMATCODE = WMDM_FORMATCODE(47362i32);
pub const WMDM_FORMATCODE_AAC: WMDM_FORMATCODE = WMDM_FORMATCODE(47363i32);
pub const WMDM_FORMATCODE_AUDIBLE: WMDM_FORMATCODE = WMDM_FORMATCODE(47364i32);
pub const WMDM_FORMATCODE_FLAC: WMDM_FORMATCODE = WMDM_FORMATCODE(47366i32);
pub const WMDM_FORMATCODE_QCELP: WMDM_FORMATCODE = WMDM_FORMATCODE(47367i32);
pub const WMDM_FORMATCODE_AMR: WMDM_FORMATCODE = WMDM_FORMATCODE(47368i32);
pub const WMDM_FORMATCODE_UNDEFINEDVIDEO: WMDM_FORMATCODE = WMDM_FORMATCODE(47488i32);
pub const WMDM_FORMATCODE_WMV: WMDM_FORMATCODE = WMDM_FORMATCODE(47489i32);
pub const WMDM_FORMATCODE_MP4: WMDM_FORMATCODE = WMDM_FORMATCODE(47490i32);
pub const WMDM_FORMATCODE_MP2: WMDM_FORMATCODE = WMDM_FORMATCODE(47491i32);
pub const WMDM_FORMATCODE_3GP: WMDM_FORMATCODE = WMDM_FORMATCODE(47492i32);
pub const WMDM_FORMATCODE_3G2: WMDM_FORMATCODE = WMDM_FORMATCODE(47493i32);
pub const WMDM_FORMATCODE_AVCHD: WMDM_FORMATCODE = WMDM_FORMATCODE(47494i32);
pub const WMDM_FORMATCODE_ATSCTS: WMDM_FORMATCODE = WMDM_FORMATCODE(47495i32);
pub const WMDM_FORMATCODE_DVBTS: WMDM_FORMATCODE = WMDM_FORMATCODE(47496i32);
pub const WMDM_FORMATCODE_MKV: WMDM_FORMATCODE = WMDM_FORMATCODE(47497i32);
pub const WMDM_FORMATCODE_MKA: WMDM_FORMATCODE = WMDM_FORMATCODE(47498i32);
pub const WMDM_FORMATCODE_MK3D: WMDM_FORMATCODE = WMDM_FORMATCODE(47499i32);
pub const WMDM_FORMATCODE_UNDEFINEDCOLLECTION: WMDM_FORMATCODE = WMDM_FORMATCODE(47616i32);
pub const WMDM_FORMATCODE_ABSTRACTMULTIMEDIAALBUM: WMDM_FORMATCODE = WMDM_FORMATCODE(47617i32);
pub const WMDM_FORMATCODE_ABSTRACTIMAGEALBUM: WMDM_FORMATCODE = WMDM_FORMATCODE(47618i32);
pub const WMDM_FORMATCODE_ABSTRACTAUDIOALBUM: WMDM_FORMATCODE = WMDM_FORMATCODE(47619i32);
pub const WMDM_FORMATCODE_ABSTRACTVIDEOALBUM: WMDM_FORMATCODE = WMDM_FORMATCODE(47620i32);
pub const WMDM_FORMATCODE_ABSTRACTAUDIOVIDEOPLAYLIST: WMDM_FORMATCODE = WMDM_FORMATCODE(47621i32);
pub const WMDM_FORMATCODE_ABSTRACTCONTACTGROUP: WMDM_FORMATCODE = WMDM_FORMATCODE(47622i32);
pub const WMDM_FORMATCODE_ABSTRACTMESSAGEFOLDER: WMDM_FORMATCODE = WMDM_FORMATCODE(47623i32);
pub const WMDM_FORMATCODE_ABSTRACTCHAPTEREDPRODUCTION: WMDM_FORMATCODE = WMDM_FORMATCODE(47624i32);
pub const WMDM_FORMATCODE_MEDIA_CAST: WMDM_FORMATCODE = WMDM_FORMATCODE(47627i32);
pub const WMDM_FORMATCODE_WPLPLAYLIST: WMDM_FORMATCODE = WMDM_FORMATCODE(47632i32);
pub const WMDM_FORMATCODE_M3UPLAYLIST: WMDM_FORMATCODE = WMDM_FORMATCODE(47633i32);
pub const WMDM_FORMATCODE_MPLPLAYLIST: WMDM_FORMATCODE = WMDM_FORMATCODE(47634i32);
pub const WMDM_FORMATCODE_ASXPLAYLIST: WMDM_FORMATCODE = WMDM_FORMATCODE(47635i32);
pub const WMDM_FORMATCODE_PLSPLAYLIST: WMDM_FORMATCODE = WMDM_FORMATCODE(47636i32);
pub const WMDM_FORMATCODE_UNDEFINEDDOCUMENT: WMDM_FORMATCODE = WMDM_FORMATCODE(47744i32);
pub const WMDM_FORMATCODE_ABSTRACTDOCUMENT: WMDM_FORMATCODE = WMDM_FORMATCODE(47745i32);
pub const WMDM_FORMATCODE_XMLDOCUMENT: WMDM_FORMATCODE = WMDM_FORMATCODE(47746i32);
pub const WMDM_FORMATCODE_MICROSOFTWORDDOCUMENT: WMDM_FORMATCODE = WMDM_FORMATCODE(47747i32);
pub const WMDM_FORMATCODE_MHTCOMPILEDHTMLDOCUMENT: WMDM_FORMATCODE = WMDM_FORMATCODE(47748i32);
pub const WMDM_FORMATCODE_MICROSOFTEXCELSPREADSHEET: WMDM_FORMATCODE = WMDM_FORMATCODE(47749i32);
pub const WMDM_FORMATCODE_MICROSOFTPOWERPOINTDOCUMENT: WMDM_FORMATCODE = WMDM_FORMATCODE(47750i32);
pub const WMDM_FORMATCODE_UNDEFINEDMESSAGE: WMDM_FORMATCODE = WMDM_FORMATCODE(47872i32);
pub const WMDM_FORMATCODE_ABSTRACTMESSAGE: WMDM_FORMATCODE = WMDM_FORMATCODE(47873i32);
pub const WMDM_FORMATCODE_UNDEFINEDCONTACT: WMDM_FORMATCODE = WMDM_FORMATCODE(48000i32);
pub const WMDM_FORMATCODE_ABSTRACTCONTACT: WMDM_FORMATCODE = WMDM_FORMATCODE(48001i32);
pub const WMDM_FORMATCODE_VCARD2: WMDM_FORMATCODE = WMDM_FORMATCODE(48002i32);
pub const WMDM_FORMATCODE_VCARD3: WMDM_FORMATCODE = WMDM_FORMATCODE(48003i32);
pub const WMDM_FORMATCODE_UNDEFINEDCALENDARITEM: WMDM_FORMATCODE = WMDM_FORMATCODE(48640i32);
pub const WMDM_FORMATCODE_ABSTRACTCALENDARITEM: WMDM_FORMATCODE = WMDM_FORMATCODE(48641i32);
pub const WMDM_FORMATCODE_VCALENDAR1: WMDM_FORMATCODE = WMDM_FORMATCODE(48642i32);
pub const WMDM_FORMATCODE_VCALENDAR2: WMDM_FORMATCODE = WMDM_FORMATCODE(48643i32);
pub const WMDM_FORMATCODE_UNDEFINEDWINDOWSEXECUTABLE: WMDM_FORMATCODE = WMDM_FORMATCODE(48768i32);
pub const WMDM_FORMATCODE_M4A: WMDM_FORMATCODE = WMDM_FORMATCODE(1297101889i32);
pub const WMDM_FORMATCODE_3GPA: WMDM_FORMATCODE = WMDM_FORMATCODE(860311617i32);
pub const WMDM_FORMATCODE_3G2A: WMDM_FORMATCODE = WMDM_FORMATCODE(860303937i32);
pub const WMDM_FORMATCODE_SECTION: WMDM_FORMATCODE = WMDM_FORMATCODE(48770i32);
impl ::std::convert::From<i32> for WMDM_FORMATCODE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for WMDM_FORMATCODE {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Ole_Automation"))]
#[doc = "*Required features: `Win32_Media_DeviceManager`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Com_StructuredStorage`, `Win32_System_Ole_Automation`*"]
pub struct WMDM_FORMAT_CAPABILITY {
    pub nPropConfig: u32,
    pub pConfigs: *mut WMDM_PROP_CONFIG,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Ole_Automation"))]
impl WMDM_FORMAT_CAPABILITY {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Ole_Automation"))]
impl ::std::default::Default for WMDM_FORMAT_CAPABILITY {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Ole_Automation"))]
impl ::std::fmt::Debug for WMDM_FORMAT_CAPABILITY {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("WMDM_FORMAT_CAPABILITY").field("nPropConfig", &self.nPropConfig).field("pConfigs", &self.pConfigs).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Ole_Automation"))]
impl ::std::cmp::PartialEq for WMDM_FORMAT_CAPABILITY {
    fn eq(&self, other: &Self) -> bool {
        self.nPropConfig == other.nPropConfig && self.pConfigs == other.pConfigs
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Ole_Automation"))]
impl ::std::cmp::Eq for WMDM_FORMAT_CAPABILITY {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Ole_Automation"))]
unsafe impl ::windows::runtime::Abi for WMDM_FORMAT_CAPABILITY {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Media_DeviceManager`*"]
pub const WMDM_GET_FORMAT_SUPPORT_AUDIO: u32 = 1u32;
#[doc = "*Required features: `Win32_Media_DeviceManager`*"]
pub const WMDM_GET_FORMAT_SUPPORT_FILE: u32 = 4u32;
#[doc = "*Required features: `Win32_Media_DeviceManager`*"]
pub const WMDM_GET_FORMAT_SUPPORT_VIDEO: u32 = 2u32;
#[doc = "*Required features: `Win32_Media_DeviceManager`*"]
pub const WMDM_LOG_NOTIMESTAMP: u32 = 16u32;
#[doc = "*Required features: `Win32_Media_DeviceManager`*"]
pub const WMDM_LOG_SEV_ERROR: u32 = 4u32;
#[doc = "*Required features: `Win32_Media_DeviceManager`*"]
pub const WMDM_LOG_SEV_INFO: u32 = 1u32;
#[doc = "*Required features: `Win32_Media_DeviceManager`*"]
pub const WMDM_LOG_SEV_WARN: u32 = 2u32;
#[doc = "*Required features: `Win32_Media_DeviceManager`*"]
pub const WMDM_MAC_LENGTH: u32 = 8u32;
#[doc = "*Required features: `Win32_Media_DeviceManager`*"]
pub const WMDM_MODE_BLOCK: u32 = 1u32;
#[doc = "*Required features: `Win32_Media_DeviceManager`*"]
pub const WMDM_MODE_PROGRESS: u32 = 64u32;
#[doc = "*Required features: `Win32_Media_DeviceManager`*"]
pub const WMDM_MODE_QUERY: u32 = 32u32;
#[doc = "*Required features: `Win32_Media_DeviceManager`*"]
pub const WMDM_MODE_RECURSIVE: u32 = 4096u32;
#[doc = "*Required features: `Win32_Media_DeviceManager`*"]
pub const WMDM_MODE_THREAD: u32 = 2u32;
#[doc = "*Required features: `Win32_Media_DeviceManager`*"]
pub const WMDM_MODE_TRANSFER_PROTECTED: u32 = 128u32;
#[doc = "*Required features: `Win32_Media_DeviceManager`*"]
pub const WMDM_MODE_TRANSFER_UNPROTECTED: u32 = 256u32;
#[doc = "*Required features: `Win32_Media_DeviceManager`*"]
pub const WMDM_POWER_CAP_BATTERY: u32 = 1u32;
#[doc = "*Required features: `Win32_Media_DeviceManager`*"]
pub const WMDM_POWER_CAP_EXTERNAL: u32 = 2u32;
#[doc = "*Required features: `Win32_Media_DeviceManager`*"]
pub const WMDM_POWER_IS_BATTERY: u32 = 4u32;
#[doc = "*Required features: `Win32_Media_DeviceManager`*"]
pub const WMDM_POWER_IS_EXTERNAL: u32 = 8u32;
#[doc = "*Required features: `Win32_Media_DeviceManager`*"]
pub const WMDM_POWER_PERCENT_AVAILABLE: u32 = 16u32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Ole_Automation"))]
#[doc = "*Required features: `Win32_Media_DeviceManager`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Com_StructuredStorage`, `Win32_System_Ole_Automation`*"]
pub struct WMDM_PROP_CONFIG {
    pub nPreference: u32,
    pub nPropDesc: u32,
    pub pPropDesc: *mut WMDM_PROP_DESC,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Ole_Automation"))]
impl WMDM_PROP_CONFIG {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Ole_Automation"))]
impl ::std::default::Default for WMDM_PROP_CONFIG {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Ole_Automation"))]
impl ::std::fmt::Debug for WMDM_PROP_CONFIG {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("WMDM_PROP_CONFIG").field("nPreference", &self.nPreference).field("nPropDesc", &self.nPropDesc).field("pPropDesc", &self.pPropDesc).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Ole_Automation"))]
impl ::std::cmp::PartialEq for WMDM_PROP_CONFIG {
    fn eq(&self, other: &Self) -> bool {
        self.nPreference == other.nPreference && self.nPropDesc == other.nPropDesc && self.pPropDesc == other.pPropDesc
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Ole_Automation"))]
impl ::std::cmp::Eq for WMDM_PROP_CONFIG {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Ole_Automation"))]
unsafe impl ::windows::runtime::Abi for WMDM_PROP_CONFIG {
    type Abi = Self;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Ole_Automation"))]
impl ::std::clone::Clone for WMDM_PROP_DESC {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Ole_Automation"))]
#[doc = "*Required features: `Win32_Media_DeviceManager`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Com_StructuredStorage`, `Win32_System_Ole_Automation`*"]
pub struct WMDM_PROP_DESC {
    pub pwszPropName: super::super::Foundation::PWSTR,
    pub ValidValuesForm: WMDM_ENUM_PROP_VALID_VALUES_FORM,
    pub ValidValues: WMDM_PROP_DESC_0,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Ole_Automation"))]
impl WMDM_PROP_DESC {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Ole_Automation"))]
impl ::std::default::Default for WMDM_PROP_DESC {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Ole_Automation"))]
impl ::std::cmp::PartialEq for WMDM_PROP_DESC {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Ole_Automation"))]
impl ::std::cmp::Eq for WMDM_PROP_DESC {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Ole_Automation"))]
unsafe impl ::windows::runtime::Abi for WMDM_PROP_DESC {
    type Abi = ::std::mem::ManuallyDrop<Self>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Ole_Automation"))]
impl ::std::clone::Clone for WMDM_PROP_DESC_0 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Ole_Automation"))]
#[doc = "*Required features: `Win32_Media_DeviceManager`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Com_StructuredStorage`, `Win32_System_Ole_Automation`*"]
pub union WMDM_PROP_DESC_0 {
    pub ValidValuesRange: ::std::mem::ManuallyDrop<WMDM_PROP_VALUES_RANGE>,
    pub EnumeratedValidValues: WMDM_PROP_VALUES_ENUM,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Ole_Automation"))]
impl WMDM_PROP_DESC_0 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Ole_Automation"))]
impl ::std::default::Default for WMDM_PROP_DESC_0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Ole_Automation"))]
impl ::std::cmp::PartialEq for WMDM_PROP_DESC_0 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Ole_Automation"))]
impl ::std::cmp::Eq for WMDM_PROP_DESC_0 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Ole_Automation"))]
unsafe impl ::windows::runtime::Abi for WMDM_PROP_DESC_0 {
    type Abi = ::std::mem::ManuallyDrop<Self>;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Ole_Automation"))]
#[doc = "*Required features: `Win32_Media_DeviceManager`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Com_StructuredStorage`, `Win32_System_Ole_Automation`*"]
pub struct WMDM_PROP_VALUES_ENUM {
    pub cEnumValues: u32,
    pub pValues: *mut super::super::System::Com::StructuredStorage::PROPVARIANT,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Ole_Automation"))]
impl WMDM_PROP_VALUES_ENUM {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Ole_Automation"))]
impl ::std::default::Default for WMDM_PROP_VALUES_ENUM {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Ole_Automation"))]
impl ::std::fmt::Debug for WMDM_PROP_VALUES_ENUM {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("WMDM_PROP_VALUES_ENUM").field("cEnumValues", &self.cEnumValues).field("pValues", &self.pValues).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Ole_Automation"))]
impl ::std::cmp::PartialEq for WMDM_PROP_VALUES_ENUM {
    fn eq(&self, other: &Self) -> bool {
        self.cEnumValues == other.cEnumValues && self.pValues == other.pValues
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Ole_Automation"))]
impl ::std::cmp::Eq for WMDM_PROP_VALUES_ENUM {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Ole_Automation"))]
unsafe impl ::windows::runtime::Abi for WMDM_PROP_VALUES_ENUM {
    type Abi = Self;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Ole_Automation"))]
impl ::std::clone::Clone for WMDM_PROP_VALUES_RANGE {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Ole_Automation"))]
#[doc = "*Required features: `Win32_Media_DeviceManager`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Com_StructuredStorage`, `Win32_System_Ole_Automation`*"]
pub struct WMDM_PROP_VALUES_RANGE {
    pub rangeMin: super::super::System::Com::StructuredStorage::PROPVARIANT,
    pub rangeMax: super::super::System::Com::StructuredStorage::PROPVARIANT,
    pub rangeStep: super::super::System::Com::StructuredStorage::PROPVARIANT,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Ole_Automation"))]
impl WMDM_PROP_VALUES_RANGE {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Ole_Automation"))]
impl ::std::default::Default for WMDM_PROP_VALUES_RANGE {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Ole_Automation"))]
impl ::std::cmp::PartialEq for WMDM_PROP_VALUES_RANGE {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Ole_Automation"))]
impl ::std::cmp::Eq for WMDM_PROP_VALUES_RANGE {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Ole_Automation"))]
unsafe impl ::windows::runtime::Abi for WMDM_PROP_VALUES_RANGE {
    type Abi = ::std::mem::ManuallyDrop<Self>;
}
#[doc = "*Required features: `Win32_Media_DeviceManager`*"]
pub const WMDM_RIGHTS_COPY_TO_CD: u32 = 8u32;
#[doc = "*Required features: `Win32_Media_DeviceManager`*"]
pub const WMDM_RIGHTS_COPY_TO_NON_SDMI_DEVICE: u32 = 2u32;
#[doc = "*Required features: `Win32_Media_DeviceManager`*"]
pub const WMDM_RIGHTS_COPY_TO_SDMI_DEVICE: u32 = 16u32;
#[doc = "*Required features: `Win32_Media_DeviceManager`*"]
pub const WMDM_RIGHTS_EXPIRATIONDATE: u32 = 2u32;
#[doc = "*Required features: `Win32_Media_DeviceManager`*"]
pub const WMDM_RIGHTS_FREESERIALIDS: u32 = 8u32;
#[doc = "*Required features: `Win32_Media_DeviceManager`*"]
pub const WMDM_RIGHTS_GROUPID: u32 = 4u32;
#[doc = "*Required features: `Win32_Media_DeviceManager`*"]
pub const WMDM_RIGHTS_NAMEDSERIALIDS: u32 = 16u32;
#[doc = "*Required features: `Win32_Media_DeviceManager`*"]
pub const WMDM_RIGHTS_PLAYBACKCOUNT: u32 = 1u32;
#[doc = "*Required features: `Win32_Media_DeviceManager`*"]
pub const WMDM_RIGHTS_PLAY_ON_PC: u32 = 1u32;
#[doc = "*Required features: `Win32_Media_DeviceManager`*"]
pub const WMDM_SCP_DECIDE_DATA: i32 = 8i32;
#[doc = "*Required features: `Win32_Media_DeviceManager`*"]
pub const WMDM_SCP_DRMINFO_NOT_DRMPROTECTED: i32 = 0i32;
#[doc = "*Required features: `Win32_Media_DeviceManager`*"]
pub const WMDM_SCP_DRMINFO_V1HEADER: i32 = 1i32;
#[doc = "*Required features: `Win32_Media_DeviceManager`*"]
pub const WMDM_SCP_DRMINFO_V2HEADER: i32 = 2i32;
#[doc = "*Required features: `Win32_Media_DeviceManager`*"]
pub const WMDM_SCP_EXAMINE_DATA: i32 = 2i32;
#[doc = "*Required features: `Win32_Media_DeviceManager`*"]
pub const WMDM_SCP_EXAMINE_EXTENSION: i32 = 1i32;
#[doc = "*Required features: `Win32_Media_DeviceManager`*"]
pub const WMDM_SCP_NO_MORE_CHANGES: i32 = 64i32;
#[doc = "*Required features: `Win32_Media_DeviceManager`*"]
pub const WMDM_SCP_PROTECTED_OUTPUT: i32 = 16i32;
#[doc = "*Required features: `Win32_Media_DeviceManager`*"]
pub const WMDM_SCP_REVOKED: u32 = 8u32;
#[doc = "*Required features: `Win32_Media_DeviceManager`*"]
pub const WMDM_SCP_RIGHTS_DATA: i32 = 64i32;
#[doc = "*Required features: `Win32_Media_DeviceManager`*"]
pub const WMDM_SCP_TRANSFER_OBJECTDATA: i32 = 32i32;
#[doc = "*Required features: `Win32_Media_DeviceManager`*"]
pub const WMDM_SCP_UNPROTECTED_OUTPUT: i32 = 32i32;
#[doc = "*Required features: `Win32_Media_DeviceManager`*"]
pub const WMDM_SEEK_BEGIN: u32 = 1u32;
#[doc = "*Required features: `Win32_Media_DeviceManager`*"]
pub const WMDM_SEEK_CURRENT: u32 = 2u32;
#[doc = "*Required features: `Win32_Media_DeviceManager`*"]
pub const WMDM_SEEK_END: u32 = 8u32;
#[doc = "*Required features: `Win32_Media_DeviceManager`*"]
pub const WMDM_SEEK_REMOTECONTROL: u32 = 1u32;
#[doc = "*Required features: `Win32_Media_DeviceManager`*"]
pub const WMDM_SEEK_STREAMINGAUDIO: u32 = 2u32;
pub const WMDM_SERVICE_PROVIDER_VENDOR_MICROSOFT: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2112383085, 30958, 17386, [164, 150, 198, 37, 172, 145, 204, 93]);
#[doc = "*Required features: `Win32_Media_DeviceManager`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct WMDM_SESSION_TYPE(pub i32);
pub const WMDM_SESSION_NONE: WMDM_SESSION_TYPE = WMDM_SESSION_TYPE(0i32);
pub const WMDM_SESSION_TRANSFER_TO_DEVICE: WMDM_SESSION_TYPE = WMDM_SESSION_TYPE(1i32);
pub const WMDM_SESSION_TRANSFER_FROM_DEVICE: WMDM_SESSION_TYPE = WMDM_SESSION_TYPE(16i32);
pub const WMDM_SESSION_DELETE: WMDM_SESSION_TYPE = WMDM_SESSION_TYPE(256i32);
pub const WMDM_SESSION_CUSTOM: WMDM_SESSION_TYPE = WMDM_SESSION_TYPE(4096i32);
impl ::std::convert::From<i32> for WMDM_SESSION_TYPE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for WMDM_SESSION_TYPE {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Media_DeviceManager`*"]
pub const WMDM_SP_REVOKED: u32 = 4u32;
#[doc = "*Required features: `Win32_Media_DeviceManager`*"]
pub const WMDM_STATUS_BUSY: u32 = 2u32;
#[doc = "*Required features: `Win32_Media_DeviceManager`*"]
pub const WMDM_STATUS_DEVICECONTROL_PAUSED: u32 = 32u32;
#[doc = "*Required features: `Win32_Media_DeviceManager`*"]
pub const WMDM_STATUS_DEVICECONTROL_PLAYING: u32 = 8u32;
#[doc = "*Required features: `Win32_Media_DeviceManager`*"]
pub const WMDM_STATUS_DEVICECONTROL_RECORDING: u32 = 16u32;
#[doc = "*Required features: `Win32_Media_DeviceManager`*"]
pub const WMDM_STATUS_DEVICECONTROL_REMOTE: u32 = 64u32;
#[doc = "*Required features: `Win32_Media_DeviceManager`*"]
pub const WMDM_STATUS_DEVICECONTROL_STREAM: u32 = 128u32;
#[doc = "*Required features: `Win32_Media_DeviceManager`*"]
pub const WMDM_STATUS_DEVICE_NOTPRESENT: u32 = 4u32;
#[doc = "*Required features: `Win32_Media_DeviceManager`*"]
pub const WMDM_STATUS_READY: u32 = 1u32;
#[doc = "*Required features: `Win32_Media_DeviceManager`*"]
pub const WMDM_STATUS_STORAGECONTROL_APPENDING: u32 = 32768u32;
#[doc = "*Required features: `Win32_Media_DeviceManager`*"]
pub const WMDM_STATUS_STORAGECONTROL_DELETING: u32 = 16384u32;
#[doc = "*Required features: `Win32_Media_DeviceManager`*"]
pub const WMDM_STATUS_STORAGECONTROL_INSERTING: u32 = 8192u32;
#[doc = "*Required features: `Win32_Media_DeviceManager`*"]
pub const WMDM_STATUS_STORAGECONTROL_MOVING: u32 = 65536u32;
#[doc = "*Required features: `Win32_Media_DeviceManager`*"]
pub const WMDM_STATUS_STORAGECONTROL_READING: u32 = 131072u32;
#[doc = "*Required features: `Win32_Media_DeviceManager`*"]
pub const WMDM_STATUS_STORAGE_BROKEN: u32 = 1024u32;
#[doc = "*Required features: `Win32_Media_DeviceManager`*"]
pub const WMDM_STATUS_STORAGE_INITIALIZING: u32 = 512u32;
#[doc = "*Required features: `Win32_Media_DeviceManager`*"]
pub const WMDM_STATUS_STORAGE_NOTPRESENT: u32 = 256u32;
#[doc = "*Required features: `Win32_Media_DeviceManager`*"]
pub const WMDM_STATUS_STORAGE_NOTSUPPORTED: u32 = 2048u32;
#[doc = "*Required features: `Win32_Media_DeviceManager`*"]
pub const WMDM_STATUS_STORAGE_UNFORMATTED: u32 = 4096u32;
#[doc = "*Required features: `Win32_Media_DeviceManager`*"]
pub const WMDM_STORAGECAP_FILELIMITEXISTS: u32 = 32u32;
#[doc = "*Required features: `Win32_Media_DeviceManager`*"]
pub const WMDM_STORAGECAP_FILESINFOLDERS: u32 = 8u32;
#[doc = "*Required features: `Win32_Media_DeviceManager`*"]
pub const WMDM_STORAGECAP_FILESINROOT: u32 = 2u32;
#[doc = "*Required features: `Win32_Media_DeviceManager`*"]
pub const WMDM_STORAGECAP_FOLDERLIMITEXISTS: u32 = 16u32;
#[doc = "*Required features: `Win32_Media_DeviceManager`*"]
pub const WMDM_STORAGECAP_FOLDERSINFOLDERS: u32 = 4u32;
#[doc = "*Required features: `Win32_Media_DeviceManager`*"]
pub const WMDM_STORAGECAP_FOLDERSINROOT: u32 = 1u32;
#[doc = "*Required features: `Win32_Media_DeviceManager`*"]
pub const WMDM_STORAGECAP_NOT_INITIALIZABLE: u32 = 64u32;
#[doc = "*Required features: `Win32_Media_DeviceManager`*"]
pub const WMDM_STORAGECONTROL_INSERTAFTER: u32 = 1024u32;
#[doc = "*Required features: `Win32_Media_DeviceManager`*"]
pub const WMDM_STORAGECONTROL_INSERTBEFORE: u32 = 512u32;
#[doc = "*Required features: `Win32_Media_DeviceManager`*"]
pub const WMDM_STORAGECONTROL_INSERTINTO: u32 = 2048u32;
#[doc = "*Required features: `Win32_Media_DeviceManager`*"]
pub const WMDM_STORAGE_ATTR_CANEDITMETADATA: u32 = 128u32;
#[doc = "*Required features: `Win32_Media_DeviceManager`*"]
pub const WMDM_STORAGE_ATTR_FILESYSTEM: u32 = 1u32;
#[doc = "*Required features: `Win32_Media_DeviceManager`*"]
pub const WMDM_STORAGE_ATTR_FOLDERS: u32 = 256u32;
#[doc = "*Required features: `Win32_Media_DeviceManager`*"]
pub const WMDM_STORAGE_ATTR_HAS_FILES: u32 = 67108864u32;
#[doc = "*Required features: `Win32_Media_DeviceManager`*"]
pub const WMDM_STORAGE_ATTR_HAS_FOLDERS: u32 = 33554432u32;
#[doc = "*Required features: `Win32_Media_DeviceManager`*"]
pub const WMDM_STORAGE_ATTR_NONREMOVABLE: u32 = 4u32;
#[doc = "*Required features: `Win32_Media_DeviceManager`*"]
pub const WMDM_STORAGE_ATTR_REMOVABLE: u32 = 2u32;
#[doc = "*Required features: `Win32_Media_DeviceManager`*"]
pub const WMDM_STORAGE_ATTR_VIRTUAL: u32 = 536870912u32;
#[doc = "*Required features: `Win32_Media_DeviceManager`*"]
pub const WMDM_STORAGE_CONTAINS_DEFAULT: u32 = 268435456u32;
#[doc = "*Required features: `Win32_Media_DeviceManager`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct WMDM_STORAGE_ENUM_MODE(pub i32);
pub const ENUM_MODE_RAW: WMDM_STORAGE_ENUM_MODE = WMDM_STORAGE_ENUM_MODE(0i32);
pub const ENUM_MODE_USE_DEVICE_PREF: WMDM_STORAGE_ENUM_MODE = WMDM_STORAGE_ENUM_MODE(1i32);
pub const ENUM_MODE_METADATA_VIEWS: WMDM_STORAGE_ENUM_MODE = WMDM_STORAGE_ENUM_MODE(2i32);
impl ::std::convert::From<i32> for WMDM_STORAGE_ENUM_MODE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for WMDM_STORAGE_ENUM_MODE {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Media_DeviceManager`*"]
pub const WMDM_STORAGE_IS_DEFAULT: u32 = 134217728u32;
#[doc = "*Required features: `Win32_Media_DeviceManager`*"]
pub const WMDM_S_NOT_ALL_PROPERTIES_APPLIED: i32 = 282625i32;
#[doc = "*Required features: `Win32_Media_DeviceManager`*"]
pub const WMDM_S_NOT_ALL_PROPERTIES_RETRIEVED: i32 = 282626i32;
#[doc = "*Required features: `Win32_Media_DeviceManager`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct WMDM_TAG_DATATYPE(pub i32);
pub const WMDM_TYPE_DWORD: WMDM_TAG_DATATYPE = WMDM_TAG_DATATYPE(0i32);
pub const WMDM_TYPE_STRING: WMDM_TAG_DATATYPE = WMDM_TAG_DATATYPE(1i32);
pub const WMDM_TYPE_BINARY: WMDM_TAG_DATATYPE = WMDM_TAG_DATATYPE(2i32);
pub const WMDM_TYPE_BOOL: WMDM_TAG_DATATYPE = WMDM_TAG_DATATYPE(3i32);
pub const WMDM_TYPE_QWORD: WMDM_TAG_DATATYPE = WMDM_TAG_DATATYPE(4i32);
pub const WMDM_TYPE_WORD: WMDM_TAG_DATATYPE = WMDM_TAG_DATATYPE(5i32);
pub const WMDM_TYPE_GUID: WMDM_TAG_DATATYPE = WMDM_TAG_DATATYPE(6i32);
pub const WMDM_TYPE_DATE: WMDM_TAG_DATATYPE = WMDM_TAG_DATATYPE(7i32);
impl ::std::convert::From<i32> for WMDM_TAG_DATATYPE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for WMDM_TAG_DATATYPE {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Media_DeviceManager`*"]
pub const WMDM_WMDM_REVOKED: u32 = 1u32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_Media_DeviceManager`, `Win32_Foundation`*"]
pub struct WMFILECAPABILITIES {
    pub pwszMimeType: super::super::Foundation::PWSTR,
    pub dwReserved: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl WMFILECAPABILITIES {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for WMFILECAPABILITIES {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for WMFILECAPABILITIES {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("WMFILECAPABILITIES").field("pwszMimeType", &self.pwszMimeType).field("dwReserved", &self.dwReserved).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for WMFILECAPABILITIES {
    fn eq(&self, other: &Self) -> bool {
        self.pwszMimeType == other.pwszMimeType && self.dwReserved == other.dwReserved
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for WMFILECAPABILITIES {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for WMFILECAPABILITIES {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Media_DeviceManager`*"]
pub struct _BITMAPINFOHEADER {
    pub biSize: u32,
    pub biWidth: i32,
    pub biHeight: i32,
    pub biPlanes: u16,
    pub biBitCount: u16,
    pub biCompression: u32,
    pub biSizeImage: u32,
    pub biXPelsPerMeter: i32,
    pub biYPelsPerMeter: i32,
    pub biClrUsed: u32,
    pub biClrImportant: u32,
}
impl _BITMAPINFOHEADER {}
impl ::std::default::Default for _BITMAPINFOHEADER {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for _BITMAPINFOHEADER {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("_BITMAPINFOHEADER")
            .field("biSize", &self.biSize)
            .field("biWidth", &self.biWidth)
            .field("biHeight", &self.biHeight)
            .field("biPlanes", &self.biPlanes)
            .field("biBitCount", &self.biBitCount)
            .field("biCompression", &self.biCompression)
            .field("biSizeImage", &self.biSizeImage)
            .field("biXPelsPerMeter", &self.biXPelsPerMeter)
            .field("biYPelsPerMeter", &self.biYPelsPerMeter)
            .field("biClrUsed", &self.biClrUsed)
            .field("biClrImportant", &self.biClrImportant)
            .finish()
    }
}
impl ::std::cmp::PartialEq for _BITMAPINFOHEADER {
    fn eq(&self, other: &Self) -> bool {
        self.biSize == other.biSize && self.biWidth == other.biWidth && self.biHeight == other.biHeight && self.biPlanes == other.biPlanes && self.biBitCount == other.biBitCount && self.biCompression == other.biCompression && self.biSizeImage == other.biSizeImage && self.biXPelsPerMeter == other.biXPelsPerMeter && self.biYPelsPerMeter == other.biYPelsPerMeter && self.biClrUsed == other.biClrUsed && self.biClrImportant == other.biClrImportant
    }
}
impl ::std::cmp::Eq for _BITMAPINFOHEADER {}
unsafe impl ::windows::runtime::Abi for _BITMAPINFOHEADER {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_Media_DeviceManager`, `Win32_Foundation`*"]
pub struct _VIDEOINFOHEADER {
    pub rcSource: super::super::Foundation::RECT,
    pub rcTarget: super::super::Foundation::RECT,
    pub dwBitRate: u32,
    pub dwBitErrorRate: u32,
    pub AvgTimePerFrame: i64,
    pub bmiHeader: _BITMAPINFOHEADER,
}
#[cfg(feature = "Win32_Foundation")]
impl _VIDEOINFOHEADER {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for _VIDEOINFOHEADER {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for _VIDEOINFOHEADER {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("_VIDEOINFOHEADER").field("rcSource", &self.rcSource).field("rcTarget", &self.rcTarget).field("dwBitRate", &self.dwBitRate).field("dwBitErrorRate", &self.dwBitErrorRate).field("AvgTimePerFrame", &self.AvgTimePerFrame).field("bmiHeader", &self.bmiHeader).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for _VIDEOINFOHEADER {
    fn eq(&self, other: &Self) -> bool {
        self.rcSource == other.rcSource && self.rcTarget == other.rcTarget && self.dwBitRate == other.dwBitRate && self.dwBitErrorRate == other.dwBitErrorRate && self.AvgTimePerFrame == other.AvgTimePerFrame && self.bmiHeader == other.bmiHeader
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for _VIDEOINFOHEADER {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for _VIDEOINFOHEADER {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Media_DeviceManager`*"]
pub struct _WAVEFORMATEX {
    pub wFormatTag: u16,
    pub nChannels: u16,
    pub nSamplesPerSec: u32,
    pub nAvgBytesPerSec: u32,
    pub nBlockAlign: u16,
    pub wBitsPerSample: u16,
    pub cbSize: u16,
}
impl _WAVEFORMATEX {}
impl ::std::default::Default for _WAVEFORMATEX {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for _WAVEFORMATEX {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("_WAVEFORMATEX")
            .field("wFormatTag", &self.wFormatTag)
            .field("nChannels", &self.nChannels)
            .field("nSamplesPerSec", &self.nSamplesPerSec)
            .field("nAvgBytesPerSec", &self.nAvgBytesPerSec)
            .field("nBlockAlign", &self.nBlockAlign)
            .field("wBitsPerSample", &self.wBitsPerSample)
            .field("cbSize", &self.cbSize)
            .finish()
    }
}
impl ::std::cmp::PartialEq for _WAVEFORMATEX {
    fn eq(&self, other: &Self) -> bool {
        self.wFormatTag == other.wFormatTag && self.nChannels == other.nChannels && self.nSamplesPerSec == other.nSamplesPerSec && self.nAvgBytesPerSec == other.nAvgBytesPerSec && self.nBlockAlign == other.nBlockAlign && self.wBitsPerSample == other.wBitsPerSample && self.cbSize == other.cbSize
    }
}
impl ::std::cmp::Eq for _WAVEFORMATEX {}
unsafe impl ::windows::runtime::Abi for _WAVEFORMATEX {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_Media_DeviceManager`, `Win32_Foundation`*"]
pub struct __MACINFO {
    pub fUsed: super::super::Foundation::BOOL,
    pub abMacState: [u8; 36],
}
#[cfg(feature = "Win32_Foundation")]
impl __MACINFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for __MACINFO {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for __MACINFO {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("__MACINFO").field("fUsed", &self.fUsed).field("abMacState", &self.abMacState).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for __MACINFO {
    fn eq(&self, other: &Self) -> bool {
        self.fUsed == other.fUsed && self.abMacState == other.abMacState
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for __MACINFO {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for __MACINFO {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Media_DeviceManager`*"]
pub const g_wszAudioWAVECodec: &'static str = "WMDM/AudioWAVECodec";
#[doc = "*Required features: `Win32_Media_DeviceManager`*"]
pub const g_wszVideoFourCCCodec: &'static str = "WMDM/VideoFourCCCodec";
#[doc = "*Required features: `Win32_Media_DeviceManager`*"]
pub const g_wszWMDMAlbumArt: &'static str = "WMDM/AlbumArt";
#[doc = "*Required features: `Win32_Media_DeviceManager`*"]
pub const g_wszWMDMAlbumArtist: &'static str = "WMDM/AlbumArtist";
#[doc = "*Required features: `Win32_Media_DeviceManager`*"]
pub const g_wszWMDMAlbumCoverData: &'static str = "WMDM/AlbumCoverData";
#[doc = "*Required features: `Win32_Media_DeviceManager`*"]
pub const g_wszWMDMAlbumCoverDuration: &'static str = "WMDM/AlbumCoverDuration";
#[doc = "*Required features: `Win32_Media_DeviceManager`*"]
pub const g_wszWMDMAlbumCoverFormat: &'static str = "WMDM/AlbumCoverFormat";
#[doc = "*Required features: `Win32_Media_DeviceManager`*"]
pub const g_wszWMDMAlbumCoverHeight: &'static str = "WMDM/AlbumCoverHeight";
#[doc = "*Required features: `Win32_Media_DeviceManager`*"]
pub const g_wszWMDMAlbumCoverSize: &'static str = "WMDM/AlbumCoverSize";
#[doc = "*Required features: `Win32_Media_DeviceManager`*"]
pub const g_wszWMDMAlbumCoverWidth: &'static str = "WMDM/AlbumCoverWidth";
#[doc = "*Required features: `Win32_Media_DeviceManager`*"]
pub const g_wszWMDMAlbumTitle: &'static str = "WMDM/AlbumTitle";
#[doc = "*Required features: `Win32_Media_DeviceManager`*"]
pub const g_wszWMDMAudioBitDepth: &'static str = "WMDM/AudioBitDepth";
#[doc = "*Required features: `Win32_Media_DeviceManager`*"]
pub const g_wszWMDMAuthor: &'static str = "WMDM/Author";
#[doc = "*Required features: `Win32_Media_DeviceManager`*"]
pub const g_wszWMDMAuthorDate: &'static str = "WMDM/AuthorDate";
#[doc = "*Required features: `Win32_Media_DeviceManager`*"]
pub const g_wszWMDMBitRateType: &'static str = "WMDM/BitRateType";
#[doc = "*Required features: `Win32_Media_DeviceManager`*"]
pub const g_wszWMDMBitrate: &'static str = "WMDM/Bitrate";
#[doc = "*Required features: `Win32_Media_DeviceManager`*"]
pub const g_wszWMDMBlockAlignment: &'static str = "WMDM/BlockAlignment";
#[doc = "*Required features: `Win32_Media_DeviceManager`*"]
pub const g_wszWMDMBufferSize: &'static str = "WMDM/BufferSize";
#[doc = "*Required features: `Win32_Media_DeviceManager`*"]
pub const g_wszWMDMBuyNow: &'static str = "WMDM/BuyNow";
#[doc = "*Required features: `Win32_Media_DeviceManager`*"]
pub const g_wszWMDMByteBookmark: &'static str = "WMDM/ByteBookmark";
#[doc = "*Required features: `Win32_Media_DeviceManager`*"]
pub const g_wszWMDMCategory: &'static str = "WMDM/Category";
#[doc = "*Required features: `Win32_Media_DeviceManager`*"]
pub const g_wszWMDMCodec: &'static str = "WMDM/Codec";
#[doc = "*Required features: `Win32_Media_DeviceManager`*"]
pub const g_wszWMDMCollectionID: &'static str = "WMDM/CollectionID";
#[doc = "*Required features: `Win32_Media_DeviceManager`*"]
pub const g_wszWMDMComposer: &'static str = "WMDM/Composer";
#[doc = "*Required features: `Win32_Media_DeviceManager`*"]
pub const g_wszWMDMDRMId: &'static str = "WMDM/DRMId";
#[doc = "*Required features: `Win32_Media_DeviceManager`*"]
pub const g_wszWMDMDataLength: &'static str = "WMDM/DataLength";
#[doc = "*Required features: `Win32_Media_DeviceManager`*"]
pub const g_wszWMDMDataOffset: &'static str = "WMDM/DataOffset";
#[doc = "*Required features: `Win32_Media_DeviceManager`*"]
pub const g_wszWMDMDataUnits: &'static str = "WMDM/DataUnits";
#[doc = "*Required features: `Win32_Media_DeviceManager`*"]
pub const g_wszWMDMDescription: &'static str = "WMDM/Description";
#[doc = "*Required features: `Win32_Media_DeviceManager`*"]
pub const g_wszWMDMDestinationURL: &'static str = "WMDM/DestinationURL";
#[doc = "*Required features: `Win32_Media_DeviceManager`*"]
pub const g_wszWMDMDeviceFirmwareVersion: &'static str = "WMDM/DeviceFirmwareVersion";
#[doc = "*Required features: `Win32_Media_DeviceManager`*"]
pub const g_wszWMDMDeviceFriendlyName: &'static str = "WMDM/DeviceFriendlyName";
#[doc = "*Required features: `Win32_Media_DeviceManager`*"]
pub const g_wszWMDMDeviceModelName: &'static str = "WMDM/DeviceModelName";
#[doc = "*Required features: `Win32_Media_DeviceManager`*"]
pub const g_wszWMDMDevicePlayCount: &'static str = "WMDM/DevicePlayCount";
#[doc = "*Required features: `Win32_Media_DeviceManager`*"]
pub const g_wszWMDMDeviceProtocol: &'static str = "WMDM/DeviceProtocol";
#[doc = "*Required features: `Win32_Media_DeviceManager`*"]
pub const g_wszWMDMDeviceRevocationInfo: &'static str = "WMDM/DeviceRevocationInfo";
#[doc = "*Required features: `Win32_Media_DeviceManager`*"]
pub const g_wszWMDMDeviceServiceProviderVendor: &'static str = "WMDM/DeviceServiceProviderVendor";
#[doc = "*Required features: `Win32_Media_DeviceManager`*"]
pub const g_wszWMDMDeviceVendorExtension: &'static str = "WMDM/DeviceVendorExtension";
#[doc = "*Required features: `Win32_Media_DeviceManager`*"]
pub const g_wszWMDMDuration: &'static str = "WMDM/Duration";
#[doc = "*Required features: `Win32_Media_DeviceManager`*"]
pub const g_wszWMDMEditor: &'static str = "WMDM/Editor";
#[doc = "*Required features: `Win32_Media_DeviceManager`*"]
pub const g_wszWMDMEncodingProfile: &'static str = "WMDM/EncodingProfile";
#[doc = "*Required features: `Win32_Media_DeviceManager`*"]
pub const g_wszWMDMFileAttributes: &'static str = "WMDM/FileAttributes";
#[doc = "*Required features: `Win32_Media_DeviceManager`*"]
pub const g_wszWMDMFileCreationDate: &'static str = "WMDM/FileCreationDate";
#[doc = "*Required features: `Win32_Media_DeviceManager`*"]
pub const g_wszWMDMFileName: &'static str = "WMDM/FileName";
#[doc = "*Required features: `Win32_Media_DeviceManager`*"]
pub const g_wszWMDMFileSize: &'static str = "WMDM/FileSize";
#[doc = "*Required features: `Win32_Media_DeviceManager`*"]
pub const g_wszWMDMFormatCode: &'static str = "WMDM/FormatCode";
#[doc = "*Required features: `Win32_Media_DeviceManager`*"]
pub const g_wszWMDMFormatsSupported: &'static str = "WMDM/FormatsSupported";
#[doc = "*Required features: `Win32_Media_DeviceManager`*"]
pub const g_wszWMDMFormatsSupportedAreOrdered: &'static str = "WMDM/FormatsSupportedAreOrdered";
#[doc = "*Required features: `Win32_Media_DeviceManager`*"]
pub const g_wszWMDMFrameRate: &'static str = "WMDM/FrameRate";
#[doc = "*Required features: `Win32_Media_DeviceManager`*"]
pub const g_wszWMDMGenre: &'static str = "WMDM/Genre";
#[doc = "*Required features: `Win32_Media_DeviceManager`*"]
pub const g_wszWMDMHeight: &'static str = "WMDM/Height";
#[doc = "*Required features: `Win32_Media_DeviceManager`*"]
pub const g_wszWMDMIsProtected: &'static str = "WMDM/IsProtected";
#[doc = "*Required features: `Win32_Media_DeviceManager`*"]
pub const g_wszWMDMIsRepeat: &'static str = "WMDM/IsRepeat";
#[doc = "*Required features: `Win32_Media_DeviceManager`*"]
pub const g_wszWMDMKeyFrameDistance: &'static str = "WMDM/KeyFrameDistance";
#[doc = "*Required features: `Win32_Media_DeviceManager`*"]
pub const g_wszWMDMLastModifiedDate: &'static str = "WMDM/LastModifiedDate";
#[doc = "*Required features: `Win32_Media_DeviceManager`*"]
pub const g_wszWMDMMediaClassSecondaryID: &'static str = "WMDM/MediaClassSecondaryID";
#[doc = "*Required features: `Win32_Media_DeviceManager`*"]
pub const g_wszWMDMMediaCredits: &'static str = "WMDM/MediaCredits";
#[doc = "*Required features: `Win32_Media_DeviceManager`*"]
pub const g_wszWMDMMediaGuid: &'static str = "WMDM/MediaGuid";
#[doc = "*Required features: `Win32_Media_DeviceManager`*"]
pub const g_wszWMDMMediaOriginalBroadcastDateTime: &'static str = "WMDM/MediaOriginalBroadcastDateTime";
#[doc = "*Required features: `Win32_Media_DeviceManager`*"]
pub const g_wszWMDMMediaOriginalChannel: &'static str = "WMDM/MediaOriginalChannel";
#[doc = "*Required features: `Win32_Media_DeviceManager`*"]
pub const g_wszWMDMMediaStationName: &'static str = "WMDM/MediaStationName";
#[doc = "*Required features: `Win32_Media_DeviceManager`*"]
pub const g_wszWMDMMetaGenre: &'static str = "WMDM/MetaGenre";
#[doc = "*Required features: `Win32_Media_DeviceManager`*"]
pub const g_wszWMDMNonConsumable: &'static str = "WMDM/NonConsumable";
#[doc = "*Required features: `Win32_Media_DeviceManager`*"]
pub const g_wszWMDMNumChannels: &'static str = "WMDM/NumChannels";
#[doc = "*Required features: `Win32_Media_DeviceManager`*"]
pub const g_wszWMDMObjectBookmark: &'static str = "WMDM/ObjectBookmark";
#[doc = "*Required features: `Win32_Media_DeviceManager`*"]
pub const g_wszWMDMOwner: &'static str = "WMDM/Owner";
#[doc = "*Required features: `Win32_Media_DeviceManager`*"]
pub const g_wszWMDMParentalRating: &'static str = "WMDM/ParentalRating";
#[doc = "*Required features: `Win32_Media_DeviceManager`*"]
pub const g_wszWMDMPersistentUniqueID: &'static str = "WMDM/PersistentUniqueID";
#[doc = "*Required features: `Win32_Media_DeviceManager`*"]
pub const g_wszWMDMPlayCount: &'static str = "WMDM/PlayCount";
#[doc = "*Required features: `Win32_Media_DeviceManager`*"]
pub const g_wszWMDMProviderCopyright: &'static str = "WMDM/ProviderCopyright";
#[doc = "*Required features: `Win32_Media_DeviceManager`*"]
pub const g_wszWMDMQualitySetting: &'static str = "WMDM/QualitySetting";
#[doc = "*Required features: `Win32_Media_DeviceManager`*"]
pub const g_wszWMDMSampleRate: &'static str = "WMDM/SampleRate";
#[doc = "*Required features: `Win32_Media_DeviceManager`*"]
pub const g_wszWMDMScanType: &'static str = "WMDM/ScanType";
#[doc = "*Required features: `Win32_Media_DeviceManager`*"]
pub const g_wszWMDMSourceURL: &'static str = "WMDM/SourceURL";
#[doc = "*Required features: `Win32_Media_DeviceManager`*"]
pub const g_wszWMDMSubTitle: &'static str = "WMDM/SubTitle";
#[doc = "*Required features: `Win32_Media_DeviceManager`*"]
pub const g_wszWMDMSubTitleDescription: &'static str = "WMDM/SubTitleDescription";
#[doc = "*Required features: `Win32_Media_DeviceManager`*"]
pub const g_wszWMDMSupportedDeviceProperties: &'static str = "WMDM/SupportedDeviceProperties";
#[doc = "*Required features: `Win32_Media_DeviceManager`*"]
pub const g_wszWMDMSyncID: &'static str = "WMDM/SyncID";
#[doc = "*Required features: `Win32_Media_DeviceManager`*"]
pub const g_wszWMDMSyncRelationshipID: &'static str = "WMDM/SyncRelationshipID";
#[doc = "*Required features: `Win32_Media_DeviceManager`*"]
pub const g_wszWMDMSyncTime: &'static str = "WMDM/SyncTime";
#[doc = "*Required features: `Win32_Media_DeviceManager`*"]
pub const g_wszWMDMTimeBookmark: &'static str = "WMDM/TimeBookmark";
#[doc = "*Required features: `Win32_Media_DeviceManager`*"]
pub const g_wszWMDMTimeToLive: &'static str = "WMDM/TimeToLive";
#[doc = "*Required features: `Win32_Media_DeviceManager`*"]
pub const g_wszWMDMTitle: &'static str = "WMDM/Title";
#[doc = "*Required features: `Win32_Media_DeviceManager`*"]
pub const g_wszWMDMTotalBitrate: &'static str = "WMDM/TotalBitrate";
#[doc = "*Required features: `Win32_Media_DeviceManager`*"]
pub const g_wszWMDMTrack: &'static str = "WMDM/Track";
#[doc = "*Required features: `Win32_Media_DeviceManager`*"]
pub const g_wszWMDMTrackMood: &'static str = "WMDM/TrackMood";
#[doc = "*Required features: `Win32_Media_DeviceManager`*"]
pub const g_wszWMDMUserEffectiveRating: &'static str = "WMDM/UserEffectiveRating";
#[doc = "*Required features: `Win32_Media_DeviceManager`*"]
pub const g_wszWMDMUserLastPlayTime: &'static str = "WMDM/UserLastPlayTime";
#[doc = "*Required features: `Win32_Media_DeviceManager`*"]
pub const g_wszWMDMUserRating: &'static str = "WMDM/UserRating";
#[doc = "*Required features: `Win32_Media_DeviceManager`*"]
pub const g_wszWMDMUserRatingOnDevice: &'static str = "WMDM/UserRatingOnDevice";
#[doc = "*Required features: `Win32_Media_DeviceManager`*"]
pub const g_wszWMDMVideoBitrate: &'static str = "WMDM/VideoBitrate";
#[doc = "*Required features: `Win32_Media_DeviceManager`*"]
pub const g_wszWMDMWebmaster: &'static str = "WMDM/Webmaster";
#[doc = "*Required features: `Win32_Media_DeviceManager`*"]
pub const g_wszWMDMWidth: &'static str = "WMDM/Width";
#[doc = "*Required features: `Win32_Media_DeviceManager`*"]
pub const g_wszWMDMYear: &'static str = "WMDM/Year";
#[doc = "*Required features: `Win32_Media_DeviceManager`*"]
pub const g_wszWMDMediaClassPrimaryID: &'static str = "WMDM/MediaClassPrimaryID";
#[doc = "*Required features: `Win32_Media_DeviceManager`*"]
pub const g_wszWPDPassthroughPropertyValues: &'static str = "WPD/PassthroughPropertyValues";
