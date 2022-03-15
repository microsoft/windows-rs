#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const ALLOW_OUTOFBAND_NOTIFICATION: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const DO_NOT_VIRTUALIZE_STORAGES_AS_DEVICES: u32 = 1u32;
pub const EVENT_WMDM_CONTENT_TRANSFER: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x339c9bf4_bcfe_4ed8_94df_eaf8c26ab61b);
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
#[repr(transparent)]
pub struct IComponentAuthenticate(::windows::core::IUnknown);
impl IComponentAuthenticate {
    #[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
    pub unsafe fn SACAuth(&self, dwprotocolid: u32, dwpass: u32, pbdatain: &[u8], ppbdataout: *mut *mut u8, pdwdataoutlen: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SACAuth)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwprotocolid), ::core::mem::transmute(dwpass), ::core::mem::transmute(::windows::core::as_ptr_or_null(pbdatain)), pbdatain.len() as _, ::core::mem::transmute(ppbdataout), ::core::mem::transmute(pdwdataoutlen)).ok()
    }
    #[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
    pub unsafe fn SACGetProtocols(&self, ppdwprotocols: *mut *mut u32, pdwprotocolcount: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SACGetProtocols)(::core::mem::transmute_copy(self), ::core::mem::transmute(ppdwprotocols), ::core::mem::transmute(pdwprotocolcount)).ok()
    }
}
impl ::core::convert::From<IComponentAuthenticate> for ::windows::core::IUnknown {
    fn from(value: IComponentAuthenticate) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IComponentAuthenticate> for ::windows::core::IUnknown {
    fn from(value: &IComponentAuthenticate) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IComponentAuthenticate {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IComponentAuthenticate {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IComponentAuthenticate {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
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
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa9889c00_6d2b_11d3_8496_00c04f79dbc0);
}
#[repr(C)]
#[doc(hidden)]
pub struct IComponentAuthenticate_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    pub SACAuth: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwprotocolid: u32, dwpass: u32, pbdatain: *const u8, dwdatainlen: u32, ppbdataout: *mut *mut u8, pdwdataoutlen: *mut u32) -> ::windows::core::HRESULT,
    pub SACGetProtocols: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppdwprotocols: *mut *mut u32, pdwprotocolcount: *mut u32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
#[repr(transparent)]
pub struct IMDSPDevice(::windows::core::IUnknown);
impl IMDSPDevice {
    #[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
    pub unsafe fn GetName(&self, pwszname: &mut [u16]) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetName)(::core::mem::transmute_copy(self), ::core::mem::transmute(::windows::core::as_mut_ptr_or_null(pwszname)), pwszname.len() as _).ok()
    }
    #[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
    pub unsafe fn GetManufacturer(&self, pwszname: &mut [u16]) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetManufacturer)(::core::mem::transmute_copy(self), ::core::mem::transmute(::windows::core::as_mut_ptr_or_null(pwszname)), pwszname.len() as _).ok()
    }
    #[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
    pub unsafe fn GetVersion(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetVersion)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
    pub unsafe fn GetType(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetType)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
    pub unsafe fn GetSerialNumber(&self, pserialnumber: *mut WMDMID, abmac: *mut u8) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetSerialNumber)(::core::mem::transmute_copy(self), ::core::mem::transmute(pserialnumber), ::core::mem::transmute(abmac)).ok()
    }
    #[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
    pub unsafe fn GetPowerSource(&self, pdwpowersource: *mut u32, pdwpercentremaining: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetPowerSource)(::core::mem::transmute_copy(self), ::core::mem::transmute(pdwpowersource), ::core::mem::transmute(pdwpercentremaining)).ok()
    }
    #[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
    pub unsafe fn GetStatus(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetStatus)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
    pub unsafe fn GetDeviceIcon(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetDeviceIcon)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
    pub unsafe fn EnumStorage(&self) -> ::windows::core::Result<IMDSPEnumStorage> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).EnumStorage)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IMDSPEnumStorage>(result__)
    }
    #[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
    pub unsafe fn GetFormatSupport(&self, pformatex: *mut *mut _WAVEFORMATEX, pnformatcount: *mut u32, pppwszmimetype: *mut *mut ::windows::core::PWSTR, pnmimetypecount: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetFormatSupport)(::core::mem::transmute_copy(self), ::core::mem::transmute(pformatex), ::core::mem::transmute(pnformatcount), ::core::mem::transmute(pppwszmimetype), ::core::mem::transmute(pnmimetypecount)).ok()
    }
    #[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
    pub unsafe fn SendOpaqueCommand(&self, pcommand: *mut OPAQUECOMMAND) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SendOpaqueCommand)(::core::mem::transmute_copy(self), ::core::mem::transmute(pcommand)).ok()
    }
}
impl ::core::convert::From<IMDSPDevice> for ::windows::core::IUnknown {
    fn from(value: IMDSPDevice) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IMDSPDevice> for ::windows::core::IUnknown {
    fn from(value: &IMDSPDevice) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IMDSPDevice {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IMDSPDevice {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IMDSPDevice {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
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
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1dcb3a12_33ed_11d3_8470_00c04f79dbc0);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMDSPDevice_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    pub GetName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwszname: ::windows::core::PWSTR, nmaxchars: u32) -> ::windows::core::HRESULT,
    pub GetManufacturer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwszname: ::windows::core::PWSTR, nmaxchars: u32) -> ::windows::core::HRESULT,
    pub GetVersion: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdwversion: *mut u32) -> ::windows::core::HRESULT,
    pub GetType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdwtype: *mut u32) -> ::windows::core::HRESULT,
    pub GetSerialNumber: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pserialnumber: *mut WMDMID, abmac: *mut u8) -> ::windows::core::HRESULT,
    pub GetPowerSource: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdwpowersource: *mut u32, pdwpercentremaining: *mut u32) -> ::windows::core::HRESULT,
    pub GetStatus: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdwstatus: *mut u32) -> ::windows::core::HRESULT,
    pub GetDeviceIcon: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hicon: *mut u32) -> ::windows::core::HRESULT,
    pub EnumStorage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppenumstorage: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub GetFormatSupport: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pformatex: *mut *mut _WAVEFORMATEX, pnformatcount: *mut u32, pppwszmimetype: *mut *mut ::windows::core::PWSTR, pnmimetypecount: *mut u32) -> ::windows::core::HRESULT,
    pub SendOpaqueCommand: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcommand: *mut OPAQUECOMMAND) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
#[repr(transparent)]
pub struct IMDSPDevice2(::windows::core::IUnknown);
impl IMDSPDevice2 {
    #[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
    pub unsafe fn GetName(&self, pwszname: &mut [u16]) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.GetName)(::core::mem::transmute_copy(self), ::core::mem::transmute(::windows::core::as_mut_ptr_or_null(pwszname)), pwszname.len() as _).ok()
    }
    #[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
    pub unsafe fn GetManufacturer(&self, pwszname: &mut [u16]) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.GetManufacturer)(::core::mem::transmute_copy(self), ::core::mem::transmute(::windows::core::as_mut_ptr_or_null(pwszname)), pwszname.len() as _).ok()
    }
    #[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
    pub unsafe fn GetVersion(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.GetVersion)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
    pub unsafe fn GetType(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.GetType)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
    pub unsafe fn GetSerialNumber(&self, pserialnumber: *mut WMDMID, abmac: *mut u8) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.GetSerialNumber)(::core::mem::transmute_copy(self), ::core::mem::transmute(pserialnumber), ::core::mem::transmute(abmac)).ok()
    }
    #[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
    pub unsafe fn GetPowerSource(&self, pdwpowersource: *mut u32, pdwpercentremaining: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.GetPowerSource)(::core::mem::transmute_copy(self), ::core::mem::transmute(pdwpowersource), ::core::mem::transmute(pdwpercentremaining)).ok()
    }
    #[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
    pub unsafe fn GetStatus(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.GetStatus)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
    pub unsafe fn GetDeviceIcon(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.GetDeviceIcon)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
    pub unsafe fn EnumStorage(&self) -> ::windows::core::Result<IMDSPEnumStorage> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.EnumStorage)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IMDSPEnumStorage>(result__)
    }
    #[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
    pub unsafe fn GetFormatSupport(&self, pformatex: *mut *mut _WAVEFORMATEX, pnformatcount: *mut u32, pppwszmimetype: *mut *mut ::windows::core::PWSTR, pnmimetypecount: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.GetFormatSupport)(::core::mem::transmute_copy(self), ::core::mem::transmute(pformatex), ::core::mem::transmute(pnformatcount), ::core::mem::transmute(pppwszmimetype), ::core::mem::transmute(pnmimetypecount)).ok()
    }
    #[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
    pub unsafe fn SendOpaqueCommand(&self, pcommand: *mut OPAQUECOMMAND) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.SendOpaqueCommand)(::core::mem::transmute_copy(self), ::core::mem::transmute(pcommand)).ok()
    }
    #[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
    pub unsafe fn GetStorage<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(&self, pszstoragename: Param0) -> ::windows::core::Result<IMDSPStorage> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetStorage)(::core::mem::transmute_copy(self), pszstoragename.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<IMDSPStorage>(result__)
    }
    #[doc = "*Required features: `\"Win32_Media_DeviceManager\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetFormatSupport2(&self, dwflags: u32, ppaudioformatex: *mut *mut _WAVEFORMATEX, pnaudioformatcount: *mut u32, ppvideoformatex: *mut *mut _VIDEOINFOHEADER, pnvideoformatcount: *mut u32, ppfiletype: *mut *mut WMFILECAPABILITIES, pnfiletypecount: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetFormatSupport2)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwflags), ::core::mem::transmute(ppaudioformatex), ::core::mem::transmute(pnaudioformatcount), ::core::mem::transmute(ppvideoformatex), ::core::mem::transmute(pnvideoformatcount), ::core::mem::transmute(ppfiletype), ::core::mem::transmute(pnfiletypecount)).ok()
    }
    #[doc = "*Required features: `\"Win32_Media_DeviceManager\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(feature = "Win32_System_Ole")]
    pub unsafe fn GetSpecifyPropertyPages(&self, ppspecifyproppages: *mut ::core::option::Option<super::super::System::Ole::ISpecifyPropertyPages>, pppunknowns: *mut *mut ::core::option::Option<::windows::core::IUnknown>, pcunks: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetSpecifyPropertyPages)(::core::mem::transmute_copy(self), ::core::mem::transmute(ppspecifyproppages), ::core::mem::transmute(pppunknowns), ::core::mem::transmute(pcunks)).ok()
    }
    #[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
    pub unsafe fn GetCanonicalName(&self, pwszpnpname: &mut [u16]) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetCanonicalName)(::core::mem::transmute_copy(self), ::core::mem::transmute(::windows::core::as_mut_ptr_or_null(pwszpnpname)), pwszpnpname.len() as _).ok()
    }
}
impl ::core::convert::From<IMDSPDevice2> for ::windows::core::IUnknown {
    fn from(value: IMDSPDevice2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IMDSPDevice2> for ::windows::core::IUnknown {
    fn from(value: &IMDSPDevice2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IMDSPDevice2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IMDSPDevice2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IMDSPDevice2> for IMDSPDevice {
    fn from(value: IMDSPDevice2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IMDSPDevice2> for IMDSPDevice {
    fn from(value: &IMDSPDevice2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IMDSPDevice> for IMDSPDevice2 {
    fn into_param(self) -> ::windows::core::Param<'a, IMDSPDevice> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IMDSPDevice> for &'a IMDSPDevice2 {
    fn into_param(self) -> ::windows::core::Param<'a, IMDSPDevice> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IMDSPDevice2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
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
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x420d16ad_c97d_4e00_82aa_00e9f4335ddd);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMDSPDevice2_Vtbl {
    pub base: IMDSPDevice_Vtbl,
    pub GetStorage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszstoragename: ::windows::core::PCWSTR, ppstorage: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub GetFormatSupport2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwflags: u32, ppaudioformatex: *mut *mut _WAVEFORMATEX, pnaudioformatcount: *mut u32, ppvideoformatex: *mut *mut _VIDEOINFOHEADER, pnvideoformatcount: *mut u32, ppfiletype: *mut *mut WMFILECAPABILITIES, pnfiletypecount: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetFormatSupport2: usize,
    #[cfg(feature = "Win32_System_Ole")]
    pub GetSpecifyPropertyPages: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppspecifyproppages: *mut ::windows::core::RawPtr, pppunknowns: *mut *mut *mut ::core::ffi::c_void, pcunks: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Ole"))]
    GetSpecifyPropertyPages: usize,
    pub GetCanonicalName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwszpnpname: ::windows::core::PWSTR, nmaxchars: u32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
#[repr(transparent)]
pub struct IMDSPDevice3(::windows::core::IUnknown);
impl IMDSPDevice3 {
    #[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
    pub unsafe fn GetName(&self, pwszname: &mut [u16]) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.base.GetName)(::core::mem::transmute_copy(self), ::core::mem::transmute(::windows::core::as_mut_ptr_or_null(pwszname)), pwszname.len() as _).ok()
    }
    #[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
    pub unsafe fn GetManufacturer(&self, pwszname: &mut [u16]) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.base.GetManufacturer)(::core::mem::transmute_copy(self), ::core::mem::transmute(::windows::core::as_mut_ptr_or_null(pwszname)), pwszname.len() as _).ok()
    }
    #[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
    pub unsafe fn GetVersion(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.base.GetVersion)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
    pub unsafe fn GetType(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.base.GetType)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
    pub unsafe fn GetSerialNumber(&self, pserialnumber: *mut WMDMID, abmac: *mut u8) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.base.GetSerialNumber)(::core::mem::transmute_copy(self), ::core::mem::transmute(pserialnumber), ::core::mem::transmute(abmac)).ok()
    }
    #[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
    pub unsafe fn GetPowerSource(&self, pdwpowersource: *mut u32, pdwpercentremaining: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.base.GetPowerSource)(::core::mem::transmute_copy(self), ::core::mem::transmute(pdwpowersource), ::core::mem::transmute(pdwpercentremaining)).ok()
    }
    #[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
    pub unsafe fn GetStatus(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.base.GetStatus)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
    pub unsafe fn GetDeviceIcon(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.base.GetDeviceIcon)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
    pub unsafe fn EnumStorage(&self) -> ::windows::core::Result<IMDSPEnumStorage> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.base.EnumStorage)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IMDSPEnumStorage>(result__)
    }
    #[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
    pub unsafe fn GetFormatSupport(&self, pformatex: *mut *mut _WAVEFORMATEX, pnformatcount: *mut u32, pppwszmimetype: *mut *mut ::windows::core::PWSTR, pnmimetypecount: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.base.GetFormatSupport)(::core::mem::transmute_copy(self), ::core::mem::transmute(pformatex), ::core::mem::transmute(pnformatcount), ::core::mem::transmute(pppwszmimetype), ::core::mem::transmute(pnmimetypecount)).ok()
    }
    #[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
    pub unsafe fn SendOpaqueCommand(&self, pcommand: *mut OPAQUECOMMAND) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.base.SendOpaqueCommand)(::core::mem::transmute_copy(self), ::core::mem::transmute(pcommand)).ok()
    }
    #[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
    pub unsafe fn GetStorage<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(&self, pszstoragename: Param0) -> ::windows::core::Result<IMDSPStorage> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.GetStorage)(::core::mem::transmute_copy(self), pszstoragename.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<IMDSPStorage>(result__)
    }
    #[doc = "*Required features: `\"Win32_Media_DeviceManager\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetFormatSupport2(&self, dwflags: u32, ppaudioformatex: *mut *mut _WAVEFORMATEX, pnaudioformatcount: *mut u32, ppvideoformatex: *mut *mut _VIDEOINFOHEADER, pnvideoformatcount: *mut u32, ppfiletype: *mut *mut WMFILECAPABILITIES, pnfiletypecount: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.GetFormatSupport2)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwflags), ::core::mem::transmute(ppaudioformatex), ::core::mem::transmute(pnaudioformatcount), ::core::mem::transmute(ppvideoformatex), ::core::mem::transmute(pnvideoformatcount), ::core::mem::transmute(ppfiletype), ::core::mem::transmute(pnfiletypecount)).ok()
    }
    #[doc = "*Required features: `\"Win32_Media_DeviceManager\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(feature = "Win32_System_Ole")]
    pub unsafe fn GetSpecifyPropertyPages(&self, ppspecifyproppages: *mut ::core::option::Option<super::super::System::Ole::ISpecifyPropertyPages>, pppunknowns: *mut *mut ::core::option::Option<::windows::core::IUnknown>, pcunks: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.GetSpecifyPropertyPages)(::core::mem::transmute_copy(self), ::core::mem::transmute(ppspecifyproppages), ::core::mem::transmute(pppunknowns), ::core::mem::transmute(pcunks)).ok()
    }
    #[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
    pub unsafe fn GetCanonicalName(&self, pwszpnpname: &mut [u16]) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.GetCanonicalName)(::core::mem::transmute_copy(self), ::core::mem::transmute(::windows::core::as_mut_ptr_or_null(pwszpnpname)), pwszpnpname.len() as _).ok()
    }
    #[doc = "*Required features: `\"Win32_Media_DeviceManager\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com_StructuredStorage\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
    pub unsafe fn GetProperty<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(&self, pwszpropname: Param0) -> ::windows::core::Result<super::super::System::Com::StructuredStorage::PROPVARIANT> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::System::Com::StructuredStorage::PROPVARIANT> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetProperty)(::core::mem::transmute_copy(self), pwszpropname.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<super::super::System::Com::StructuredStorage::PROPVARIANT>(result__)
    }
    #[doc = "*Required features: `\"Win32_Media_DeviceManager\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com_StructuredStorage\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
    pub unsafe fn SetProperty<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(&self, pwszpropname: Param0, pvalue: *const super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetProperty)(::core::mem::transmute_copy(self), pwszpropname.into_param().abi(), ::core::mem::transmute(pvalue)).ok()
    }
    #[doc = "*Required features: `\"Win32_Media_DeviceManager\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com_StructuredStorage\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
    pub unsafe fn GetFormatCapability(&self, format: WMDM_FORMATCODE) -> ::windows::core::Result<WMDM_FORMAT_CAPABILITY> {
        let mut result__: WMDM_FORMAT_CAPABILITY = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetFormatCapability)(::core::mem::transmute_copy(self), ::core::mem::transmute(format), ::core::mem::transmute(&mut result__)).from_abi::<WMDM_FORMAT_CAPABILITY>(result__)
    }
    #[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
    pub unsafe fn DeviceIoControl(&self, dwiocontrolcode: u32, lpinbuffer: &[u8], lpoutbuffer: *mut u8, pnoutbuffersize: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).DeviceIoControl)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwiocontrolcode), ::core::mem::transmute(::windows::core::as_ptr_or_null(lpinbuffer)), lpinbuffer.len() as _, ::core::mem::transmute(lpoutbuffer), ::core::mem::transmute(pnoutbuffersize)).ok()
    }
    #[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
    pub unsafe fn FindStorage<'a, Param1: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(&self, findscope: WMDM_FIND_SCOPE, pwszuniqueid: Param1) -> ::windows::core::Result<IMDSPStorage> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).FindStorage)(::core::mem::transmute_copy(self), ::core::mem::transmute(findscope), pwszuniqueid.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<IMDSPStorage>(result__)
    }
}
impl ::core::convert::From<IMDSPDevice3> for ::windows::core::IUnknown {
    fn from(value: IMDSPDevice3) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IMDSPDevice3> for ::windows::core::IUnknown {
    fn from(value: &IMDSPDevice3) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IMDSPDevice3 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IMDSPDevice3 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IMDSPDevice3> for IMDSPDevice {
    fn from(value: IMDSPDevice3) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IMDSPDevice3> for IMDSPDevice {
    fn from(value: &IMDSPDevice3) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IMDSPDevice> for IMDSPDevice3 {
    fn into_param(self) -> ::windows::core::Param<'a, IMDSPDevice> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IMDSPDevice> for &'a IMDSPDevice3 {
    fn into_param(self) -> ::windows::core::Param<'a, IMDSPDevice> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IMDSPDevice3> for IMDSPDevice2 {
    fn from(value: IMDSPDevice3) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IMDSPDevice3> for IMDSPDevice2 {
    fn from(value: &IMDSPDevice3) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IMDSPDevice2> for IMDSPDevice3 {
    fn into_param(self) -> ::windows::core::Param<'a, IMDSPDevice2> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IMDSPDevice2> for &'a IMDSPDevice3 {
    fn into_param(self) -> ::windows::core::Param<'a, IMDSPDevice2> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IMDSPDevice3 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
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
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1a839845_fc55_487c_976f_ee38ac0e8c4e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMDSPDevice3_Vtbl {
    pub base: IMDSPDevice2_Vtbl,
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
    pub FindStorage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, findscope: WMDM_FIND_SCOPE, pwszuniqueid: ::windows::core::PCWSTR, ppstorage: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
#[repr(transparent)]
pub struct IMDSPDeviceControl(::windows::core::IUnknown);
impl IMDSPDeviceControl {
    #[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
    pub unsafe fn GetDCStatus(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetDCStatus)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
    pub unsafe fn GetCapabilities(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetCapabilities)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
    pub unsafe fn Play(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Play)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
    pub unsafe fn Record(&self, pformat: *const _WAVEFORMATEX) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Record)(::core::mem::transmute_copy(self), ::core::mem::transmute(pformat)).ok()
    }
    #[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
    pub unsafe fn Pause(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Pause)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
    pub unsafe fn Resume(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Resume)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
    pub unsafe fn Stop(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Stop)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
    pub unsafe fn Seek(&self, fumode: u32, noffset: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Seek)(::core::mem::transmute_copy(self), ::core::mem::transmute(fumode), ::core::mem::transmute(noffset)).ok()
    }
}
impl ::core::convert::From<IMDSPDeviceControl> for ::windows::core::IUnknown {
    fn from(value: IMDSPDeviceControl) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IMDSPDeviceControl> for ::windows::core::IUnknown {
    fn from(value: &IMDSPDeviceControl) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IMDSPDeviceControl {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IMDSPDeviceControl {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IMDSPDeviceControl {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
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
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1dcb3a14_33ed_11d3_8470_00c04f79dbc0);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMDSPDeviceControl_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    pub GetDCStatus: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdwstatus: *mut u32) -> ::windows::core::HRESULT,
    pub GetCapabilities: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdwcapabilitiesmask: *mut u32) -> ::windows::core::HRESULT,
    pub Play: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Record: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pformat: *const _WAVEFORMATEX) -> ::windows::core::HRESULT,
    pub Pause: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Resume: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Stop: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Seek: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, fumode: u32, noffset: i32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
#[repr(transparent)]
pub struct IMDSPDirectTransfer(::windows::core::IUnknown);
impl IMDSPDirectTransfer {
    #[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
    pub unsafe fn TransferToDevice<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>, Param1: ::windows::core::IntoParam<'a, IWMDMOperation>, Param3: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>, Param4: ::windows::core::IntoParam<'a, IWMDMMetaData>, Param5: ::windows::core::IntoParam<'a, IWMDMProgress>>(&self, pwszsourcefilepath: Param0, psourceoperation: Param1, fuflags: u32, pwszdestinationname: Param3, psourcemetadata: Param4, ptransferprogress: Param5) -> ::windows::core::Result<IMDSPStorage> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).TransferToDevice)(::core::mem::transmute_copy(self), pwszsourcefilepath.into_param().abi(), psourceoperation.into_param().abi(), ::core::mem::transmute(fuflags), pwszdestinationname.into_param().abi(), psourcemetadata.into_param().abi(), ptransferprogress.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<IMDSPStorage>(result__)
    }
}
impl ::core::convert::From<IMDSPDirectTransfer> for ::windows::core::IUnknown {
    fn from(value: IMDSPDirectTransfer) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IMDSPDirectTransfer> for ::windows::core::IUnknown {
    fn from(value: &IMDSPDirectTransfer) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IMDSPDirectTransfer {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IMDSPDirectTransfer {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IMDSPDirectTransfer {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
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
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc2fe57a8_9304_478c_9ee4_47e397b912d7);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMDSPDirectTransfer_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    pub TransferToDevice: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwszsourcefilepath: ::windows::core::PCWSTR, psourceoperation: ::windows::core::RawPtr, fuflags: u32, pwszdestinationname: ::windows::core::PCWSTR, psourcemetadata: ::windows::core::RawPtr, ptransferprogress: ::windows::core::RawPtr, ppnewobject: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
#[repr(transparent)]
pub struct IMDSPEnumDevice(::windows::core::IUnknown);
impl IMDSPEnumDevice {
    #[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
    pub unsafe fn Next(&self, celt: u32, ppdevice: *mut ::core::option::Option<IMDSPDevice>, pceltfetched: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Next)(::core::mem::transmute_copy(self), ::core::mem::transmute(celt), ::core::mem::transmute(ppdevice), ::core::mem::transmute(pceltfetched)).ok()
    }
    #[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
    pub unsafe fn Skip(&self, celt: u32) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).Skip)(::core::mem::transmute_copy(self), ::core::mem::transmute(celt), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
    pub unsafe fn Reset(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Reset)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
    pub unsafe fn Clone(&self) -> ::windows::core::Result<IMDSPEnumDevice> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).Clone)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IMDSPEnumDevice>(result__)
    }
}
impl ::core::convert::From<IMDSPEnumDevice> for ::windows::core::IUnknown {
    fn from(value: IMDSPEnumDevice) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IMDSPEnumDevice> for ::windows::core::IUnknown {
    fn from(value: &IMDSPEnumDevice) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IMDSPEnumDevice {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IMDSPEnumDevice {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IMDSPEnumDevice {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
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
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1dcb3a11_33ed_11d3_8470_00c04f79dbc0);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMDSPEnumDevice_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    pub Next: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, celt: u32, ppdevice: *mut ::windows::core::RawPtr, pceltfetched: *mut u32) -> ::windows::core::HRESULT,
    pub Skip: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, celt: u32, pceltfetched: *mut u32) -> ::windows::core::HRESULT,
    pub Reset: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Clone: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppenumdevice: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
#[repr(transparent)]
pub struct IMDSPEnumStorage(::windows::core::IUnknown);
impl IMDSPEnumStorage {
    #[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
    pub unsafe fn Next(&self, celt: u32, ppstorage: *mut ::core::option::Option<IMDSPStorage>, pceltfetched: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Next)(::core::mem::transmute_copy(self), ::core::mem::transmute(celt), ::core::mem::transmute(ppstorage), ::core::mem::transmute(pceltfetched)).ok()
    }
    #[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
    pub unsafe fn Skip(&self, celt: u32) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).Skip)(::core::mem::transmute_copy(self), ::core::mem::transmute(celt), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
    pub unsafe fn Reset(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Reset)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
    pub unsafe fn Clone(&self) -> ::windows::core::Result<IMDSPEnumStorage> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).Clone)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IMDSPEnumStorage>(result__)
    }
}
impl ::core::convert::From<IMDSPEnumStorage> for ::windows::core::IUnknown {
    fn from(value: IMDSPEnumStorage) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IMDSPEnumStorage> for ::windows::core::IUnknown {
    fn from(value: &IMDSPEnumStorage) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IMDSPEnumStorage {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IMDSPEnumStorage {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IMDSPEnumStorage {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
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
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1dcb3a15_33ed_11d3_8470_00c04f79dbc0);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMDSPEnumStorage_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    pub Next: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, celt: u32, ppstorage: *mut ::windows::core::RawPtr, pceltfetched: *mut u32) -> ::windows::core::HRESULT,
    pub Skip: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, celt: u32, pceltfetched: *mut u32) -> ::windows::core::HRESULT,
    pub Reset: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Clone: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppenumstorage: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
#[repr(transparent)]
pub struct IMDSPObject(::windows::core::IUnknown);
impl IMDSPObject {
    #[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
    pub unsafe fn Open(&self, fumode: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Open)(::core::mem::transmute_copy(self), ::core::mem::transmute(fumode)).ok()
    }
    #[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
    pub unsafe fn Read(&self, pdata: *mut u8, pdwsize: *mut u32, abmac: *mut u8) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Read)(::core::mem::transmute_copy(self), ::core::mem::transmute(pdata), ::core::mem::transmute(pdwsize), ::core::mem::transmute(abmac)).ok()
    }
    #[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
    pub unsafe fn Write(&self, pdata: *const u8, pdwsize: *mut u32, abmac: *mut u8) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Write)(::core::mem::transmute_copy(self), ::core::mem::transmute(pdata), ::core::mem::transmute(pdwsize), ::core::mem::transmute(abmac)).ok()
    }
    #[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
    pub unsafe fn Delete<'a, Param1: ::windows::core::IntoParam<'a, IWMDMProgress>>(&self, fumode: u32, pprogress: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Delete)(::core::mem::transmute_copy(self), ::core::mem::transmute(fumode), pprogress.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
    pub unsafe fn Seek(&self, fuflags: u32, dwoffset: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Seek)(::core::mem::transmute_copy(self), ::core::mem::transmute(fuflags), ::core::mem::transmute(dwoffset)).ok()
    }
    #[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
    pub unsafe fn Rename<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>, Param1: ::windows::core::IntoParam<'a, IWMDMProgress>>(&self, pwsznewname: Param0, pprogress: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Rename)(::core::mem::transmute_copy(self), pwsznewname.into_param().abi(), pprogress.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
    pub unsafe fn Move<'a, Param1: ::windows::core::IntoParam<'a, IWMDMProgress>, Param2: ::windows::core::IntoParam<'a, IMDSPStorage>>(&self, fumode: u32, pprogress: Param1, ptarget: Param2) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Move)(::core::mem::transmute_copy(self), ::core::mem::transmute(fumode), pprogress.into_param().abi(), ptarget.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
    pub unsafe fn Close(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Close)(::core::mem::transmute_copy(self)).ok()
    }
}
impl ::core::convert::From<IMDSPObject> for ::windows::core::IUnknown {
    fn from(value: IMDSPObject) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IMDSPObject> for ::windows::core::IUnknown {
    fn from(value: &IMDSPObject) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IMDSPObject {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IMDSPObject {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IMDSPObject {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
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
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1dcb3a18_33ed_11d3_8470_00c04f79dbc0);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMDSPObject_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    pub Open: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, fumode: u32) -> ::windows::core::HRESULT,
    pub Read: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdata: *mut u8, pdwsize: *mut u32, abmac: *mut u8) -> ::windows::core::HRESULT,
    pub Write: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdata: *const u8, pdwsize: *mut u32, abmac: *mut u8) -> ::windows::core::HRESULT,
    pub Delete: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, fumode: u32, pprogress: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub Seek: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, fuflags: u32, dwoffset: u32) -> ::windows::core::HRESULT,
    pub Rename: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwsznewname: ::windows::core::PCWSTR, pprogress: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub Move: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, fumode: u32, pprogress: ::windows::core::RawPtr, ptarget: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub Close: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
#[repr(transparent)]
pub struct IMDSPObject2(::windows::core::IUnknown);
impl IMDSPObject2 {
    #[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
    pub unsafe fn Open(&self, fumode: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.Open)(::core::mem::transmute_copy(self), ::core::mem::transmute(fumode)).ok()
    }
    #[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
    pub unsafe fn Read(&self, pdata: *mut u8, pdwsize: *mut u32, abmac: *mut u8) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.Read)(::core::mem::transmute_copy(self), ::core::mem::transmute(pdata), ::core::mem::transmute(pdwsize), ::core::mem::transmute(abmac)).ok()
    }
    #[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
    pub unsafe fn Write(&self, pdata: *const u8, pdwsize: *mut u32, abmac: *mut u8) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.Write)(::core::mem::transmute_copy(self), ::core::mem::transmute(pdata), ::core::mem::transmute(pdwsize), ::core::mem::transmute(abmac)).ok()
    }
    #[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
    pub unsafe fn Delete<'a, Param1: ::windows::core::IntoParam<'a, IWMDMProgress>>(&self, fumode: u32, pprogress: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.Delete)(::core::mem::transmute_copy(self), ::core::mem::transmute(fumode), pprogress.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
    pub unsafe fn Seek(&self, fuflags: u32, dwoffset: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.Seek)(::core::mem::transmute_copy(self), ::core::mem::transmute(fuflags), ::core::mem::transmute(dwoffset)).ok()
    }
    #[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
    pub unsafe fn Rename<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>, Param1: ::windows::core::IntoParam<'a, IWMDMProgress>>(&self, pwsznewname: Param0, pprogress: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.Rename)(::core::mem::transmute_copy(self), pwsznewname.into_param().abi(), pprogress.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
    pub unsafe fn Move<'a, Param1: ::windows::core::IntoParam<'a, IWMDMProgress>, Param2: ::windows::core::IntoParam<'a, IMDSPStorage>>(&self, fumode: u32, pprogress: Param1, ptarget: Param2) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.Move)(::core::mem::transmute_copy(self), ::core::mem::transmute(fumode), pprogress.into_param().abi(), ptarget.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
    pub unsafe fn Close(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.Close)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
    pub unsafe fn ReadOnClearChannel(&self, pdata: *mut u8, pdwsize: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).ReadOnClearChannel)(::core::mem::transmute_copy(self), ::core::mem::transmute(pdata), ::core::mem::transmute(pdwsize)).ok()
    }
    #[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
    pub unsafe fn WriteOnClearChannel(&self, pdata: *const u8, pdwsize: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).WriteOnClearChannel)(::core::mem::transmute_copy(self), ::core::mem::transmute(pdata), ::core::mem::transmute(pdwsize)).ok()
    }
}
impl ::core::convert::From<IMDSPObject2> for ::windows::core::IUnknown {
    fn from(value: IMDSPObject2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IMDSPObject2> for ::windows::core::IUnknown {
    fn from(value: &IMDSPObject2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IMDSPObject2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IMDSPObject2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IMDSPObject2> for IMDSPObject {
    fn from(value: IMDSPObject2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IMDSPObject2> for IMDSPObject {
    fn from(value: &IMDSPObject2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IMDSPObject> for IMDSPObject2 {
    fn into_param(self) -> ::windows::core::Param<'a, IMDSPObject> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IMDSPObject> for &'a IMDSPObject2 {
    fn into_param(self) -> ::windows::core::Param<'a, IMDSPObject> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IMDSPObject2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
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
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3f34cd3e_5907_4341_9af9_97f4187c3aa5);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMDSPObject2_Vtbl {
    pub base: IMDSPObject_Vtbl,
    pub ReadOnClearChannel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdata: *mut u8, pdwsize: *mut u32) -> ::windows::core::HRESULT,
    pub WriteOnClearChannel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdata: *const u8, pdwsize: *mut u32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
#[repr(transparent)]
pub struct IMDSPObjectInfo(::windows::core::IUnknown);
impl IMDSPObjectInfo {
    #[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
    pub unsafe fn GetPlayLength(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetPlayLength)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
    pub unsafe fn SetPlayLength(&self, dwlength: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetPlayLength)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwlength)).ok()
    }
    #[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
    pub unsafe fn GetPlayOffset(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetPlayOffset)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
    pub unsafe fn SetPlayOffset(&self, dwoffset: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetPlayOffset)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwoffset)).ok()
    }
    #[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
    pub unsafe fn GetTotalLength(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetTotalLength)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
    pub unsafe fn GetLastPlayPosition(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetLastPlayPosition)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
    pub unsafe fn GetLongestPlayPosition(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetLongestPlayPosition)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
}
impl ::core::convert::From<IMDSPObjectInfo> for ::windows::core::IUnknown {
    fn from(value: IMDSPObjectInfo) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IMDSPObjectInfo> for ::windows::core::IUnknown {
    fn from(value: &IMDSPObjectInfo) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IMDSPObjectInfo {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IMDSPObjectInfo {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IMDSPObjectInfo {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
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
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1dcb3a19_33ed_11d3_8470_00c04f79dbc0);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMDSPObjectInfo_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
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
    #[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
    pub unsafe fn GetRevocationURL(&self, ppwszrevocationurl: *mut ::windows::core::PWSTR, pdwbufferlen: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetRevocationURL)(::core::mem::transmute_copy(self), ::core::mem::transmute(ppwszrevocationurl), ::core::mem::transmute(pdwbufferlen)).ok()
    }
}
impl ::core::convert::From<IMDSPRevoked> for ::windows::core::IUnknown {
    fn from(value: IMDSPRevoked) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IMDSPRevoked> for ::windows::core::IUnknown {
    fn from(value: &IMDSPRevoked) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IMDSPRevoked {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IMDSPRevoked {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IMDSPRevoked {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
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
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa4e8f2d4_3f31_464d_b53d_4fc335998184);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMDSPRevoked_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    pub GetRevocationURL: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppwszrevocationurl: *mut ::windows::core::PWSTR, pdwbufferlen: *mut u32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
#[repr(transparent)]
pub struct IMDSPStorage(::windows::core::IUnknown);
impl IMDSPStorage {
    #[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
    pub unsafe fn SetAttributes(&self, dwattributes: u32, pformat: *const _WAVEFORMATEX) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetAttributes)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwattributes), ::core::mem::transmute(pformat)).ok()
    }
    #[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
    pub unsafe fn GetStorageGlobals(&self) -> ::windows::core::Result<IMDSPStorageGlobals> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetStorageGlobals)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IMDSPStorageGlobals>(result__)
    }
    #[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
    pub unsafe fn GetAttributes(&self, pdwattributes: *mut u32, pformat: *mut _WAVEFORMATEX) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetAttributes)(::core::mem::transmute_copy(self), ::core::mem::transmute(pdwattributes), ::core::mem::transmute(pformat)).ok()
    }
    #[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
    pub unsafe fn GetName(&self, pwszname: &mut [u16]) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetName)(::core::mem::transmute_copy(self), ::core::mem::transmute(::windows::core::as_mut_ptr_or_null(pwszname)), pwszname.len() as _).ok()
    }
    #[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
    pub unsafe fn GetDate(&self) -> ::windows::core::Result<WMDMDATETIME> {
        let mut result__: WMDMDATETIME = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetDate)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<WMDMDATETIME>(result__)
    }
    #[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
    pub unsafe fn GetSize(&self, pdwsizelow: *mut u32, pdwsizehigh: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetSize)(::core::mem::transmute_copy(self), ::core::mem::transmute(pdwsizelow), ::core::mem::transmute(pdwsizehigh)).ok()
    }
    #[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
    pub unsafe fn GetRights(&self, pprights: *mut *mut WMDMRIGHTS, pnrightscount: *mut u32, abmac: *mut u8) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetRights)(::core::mem::transmute_copy(self), ::core::mem::transmute(pprights), ::core::mem::transmute(pnrightscount), ::core::mem::transmute(abmac)).ok()
    }
    #[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
    pub unsafe fn CreateStorage<'a, Param2: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(&self, dwattributes: u32, pformat: *const _WAVEFORMATEX, pwszname: Param2) -> ::windows::core::Result<IMDSPStorage> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).CreateStorage)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwattributes), ::core::mem::transmute(pformat), pwszname.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<IMDSPStorage>(result__)
    }
    #[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
    pub unsafe fn EnumStorage(&self) -> ::windows::core::Result<IMDSPEnumStorage> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).EnumStorage)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IMDSPEnumStorage>(result__)
    }
    #[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
    pub unsafe fn SendOpaqueCommand(&self, pcommand: *mut OPAQUECOMMAND) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SendOpaqueCommand)(::core::mem::transmute_copy(self), ::core::mem::transmute(pcommand)).ok()
    }
}
impl ::core::convert::From<IMDSPStorage> for ::windows::core::IUnknown {
    fn from(value: IMDSPStorage) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IMDSPStorage> for ::windows::core::IUnknown {
    fn from(value: &IMDSPStorage) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IMDSPStorage {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IMDSPStorage {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IMDSPStorage {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
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
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1dcb3a16_33ed_11d3_8470_00c04f79dbc0);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMDSPStorage_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    pub SetAttributes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwattributes: u32, pformat: *const _WAVEFORMATEX) -> ::windows::core::HRESULT,
    pub GetStorageGlobals: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppstorageglobals: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub GetAttributes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdwattributes: *mut u32, pformat: *mut _WAVEFORMATEX) -> ::windows::core::HRESULT,
    pub GetName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwszname: ::windows::core::PWSTR, nmaxchars: u32) -> ::windows::core::HRESULT,
    pub GetDate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdatetimeutc: *mut WMDMDATETIME) -> ::windows::core::HRESULT,
    pub GetSize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdwsizelow: *mut u32, pdwsizehigh: *mut u32) -> ::windows::core::HRESULT,
    pub GetRights: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pprights: *mut *mut WMDMRIGHTS, pnrightscount: *mut u32, abmac: *mut u8) -> ::windows::core::HRESULT,
    pub CreateStorage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwattributes: u32, pformat: *const _WAVEFORMATEX, pwszname: ::windows::core::PCWSTR, ppnewstorage: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub EnumStorage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppenumstorage: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub SendOpaqueCommand: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcommand: *mut OPAQUECOMMAND) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
#[repr(transparent)]
pub struct IMDSPStorage2(::windows::core::IUnknown);
impl IMDSPStorage2 {
    #[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
    pub unsafe fn SetAttributes(&self, dwattributes: u32, pformat: *const _WAVEFORMATEX) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.SetAttributes)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwattributes), ::core::mem::transmute(pformat)).ok()
    }
    #[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
    pub unsafe fn GetStorageGlobals(&self) -> ::windows::core::Result<IMDSPStorageGlobals> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.GetStorageGlobals)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IMDSPStorageGlobals>(result__)
    }
    #[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
    pub unsafe fn GetAttributes(&self, pdwattributes: *mut u32, pformat: *mut _WAVEFORMATEX) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.GetAttributes)(::core::mem::transmute_copy(self), ::core::mem::transmute(pdwattributes), ::core::mem::transmute(pformat)).ok()
    }
    #[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
    pub unsafe fn GetName(&self, pwszname: &mut [u16]) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.GetName)(::core::mem::transmute_copy(self), ::core::mem::transmute(::windows::core::as_mut_ptr_or_null(pwszname)), pwszname.len() as _).ok()
    }
    #[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
    pub unsafe fn GetDate(&self) -> ::windows::core::Result<WMDMDATETIME> {
        let mut result__: WMDMDATETIME = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.GetDate)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<WMDMDATETIME>(result__)
    }
    #[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
    pub unsafe fn GetSize(&self, pdwsizelow: *mut u32, pdwsizehigh: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.GetSize)(::core::mem::transmute_copy(self), ::core::mem::transmute(pdwsizelow), ::core::mem::transmute(pdwsizehigh)).ok()
    }
    #[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
    pub unsafe fn GetRights(&self, pprights: *mut *mut WMDMRIGHTS, pnrightscount: *mut u32, abmac: *mut u8) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.GetRights)(::core::mem::transmute_copy(self), ::core::mem::transmute(pprights), ::core::mem::transmute(pnrightscount), ::core::mem::transmute(abmac)).ok()
    }
    #[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
    pub unsafe fn CreateStorage<'a, Param2: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(&self, dwattributes: u32, pformat: *const _WAVEFORMATEX, pwszname: Param2) -> ::windows::core::Result<IMDSPStorage> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.CreateStorage)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwattributes), ::core::mem::transmute(pformat), pwszname.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<IMDSPStorage>(result__)
    }
    #[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
    pub unsafe fn EnumStorage(&self) -> ::windows::core::Result<IMDSPEnumStorage> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.EnumStorage)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IMDSPEnumStorage>(result__)
    }
    #[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
    pub unsafe fn SendOpaqueCommand(&self, pcommand: *mut OPAQUECOMMAND) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.SendOpaqueCommand)(::core::mem::transmute_copy(self), ::core::mem::transmute(pcommand)).ok()
    }
    #[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
    pub unsafe fn GetStorage<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(&self, pszstoragename: Param0) -> ::windows::core::Result<IMDSPStorage> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetStorage)(::core::mem::transmute_copy(self), pszstoragename.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<IMDSPStorage>(result__)
    }
    #[doc = "*Required features: `\"Win32_Media_DeviceManager\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CreateStorage2<'a, Param4: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(&self, dwattributes: u32, dwattributesex: u32, paudioformat: *const _WAVEFORMATEX, pvideoformat: *const _VIDEOINFOHEADER, pwszname: Param4, qwfilesize: u64) -> ::windows::core::Result<IMDSPStorage> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).CreateStorage2)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwattributes), ::core::mem::transmute(dwattributesex), ::core::mem::transmute(paudioformat), ::core::mem::transmute(pvideoformat), pwszname.into_param().abi(), ::core::mem::transmute(qwfilesize), ::core::mem::transmute(&mut result__)).from_abi::<IMDSPStorage>(result__)
    }
    #[doc = "*Required features: `\"Win32_Media_DeviceManager\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetAttributes2(&self, dwattributes: u32, dwattributesex: u32, paudioformat: *const _WAVEFORMATEX, pvideoformat: *const _VIDEOINFOHEADER) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetAttributes2)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwattributes), ::core::mem::transmute(dwattributesex), ::core::mem::transmute(paudioformat), ::core::mem::transmute(pvideoformat)).ok()
    }
    #[doc = "*Required features: `\"Win32_Media_DeviceManager\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetAttributes2(&self, pdwattributes: *mut u32, pdwattributesex: *mut u32, paudioformat: *mut _WAVEFORMATEX, pvideoformat: *mut _VIDEOINFOHEADER) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetAttributes2)(::core::mem::transmute_copy(self), ::core::mem::transmute(pdwattributes), ::core::mem::transmute(pdwattributesex), ::core::mem::transmute(paudioformat), ::core::mem::transmute(pvideoformat)).ok()
    }
}
impl ::core::convert::From<IMDSPStorage2> for ::windows::core::IUnknown {
    fn from(value: IMDSPStorage2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IMDSPStorage2> for ::windows::core::IUnknown {
    fn from(value: &IMDSPStorage2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IMDSPStorage2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IMDSPStorage2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IMDSPStorage2> for IMDSPStorage {
    fn from(value: IMDSPStorage2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IMDSPStorage2> for IMDSPStorage {
    fn from(value: &IMDSPStorage2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IMDSPStorage> for IMDSPStorage2 {
    fn into_param(self) -> ::windows::core::Param<'a, IMDSPStorage> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IMDSPStorage> for &'a IMDSPStorage2 {
    fn into_param(self) -> ::windows::core::Param<'a, IMDSPStorage> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IMDSPStorage2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
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
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0a5e07a5_6454_4451_9c36_1c6ae7e2b1d6);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMDSPStorage2_Vtbl {
    pub base: IMDSPStorage_Vtbl,
    pub GetStorage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszstoragename: ::windows::core::PCWSTR, ppstorage: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub CreateStorage2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwattributes: u32, dwattributesex: u32, paudioformat: *const _WAVEFORMATEX, pvideoformat: *const _VIDEOINFOHEADER, pwszname: ::windows::core::PCWSTR, qwfilesize: u64, ppnewstorage: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    CreateStorage2: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetAttributes2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwattributes: u32, dwattributesex: u32, paudioformat: *const _WAVEFORMATEX, pvideoformat: *const _VIDEOINFOHEADER) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetAttributes2: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub GetAttributes2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdwattributes: *mut u32, pdwattributesex: *mut u32, paudioformat: *mut _WAVEFORMATEX, pvideoformat: *mut _VIDEOINFOHEADER) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetAttributes2: usize,
}
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
#[repr(transparent)]
pub struct IMDSPStorage3(::windows::core::IUnknown);
impl IMDSPStorage3 {
    #[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
    pub unsafe fn SetAttributes(&self, dwattributes: u32, pformat: *const _WAVEFORMATEX) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.base.SetAttributes)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwattributes), ::core::mem::transmute(pformat)).ok()
    }
    #[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
    pub unsafe fn GetStorageGlobals(&self) -> ::windows::core::Result<IMDSPStorageGlobals> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.base.GetStorageGlobals)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IMDSPStorageGlobals>(result__)
    }
    #[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
    pub unsafe fn GetAttributes(&self, pdwattributes: *mut u32, pformat: *mut _WAVEFORMATEX) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.base.GetAttributes)(::core::mem::transmute_copy(self), ::core::mem::transmute(pdwattributes), ::core::mem::transmute(pformat)).ok()
    }
    #[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
    pub unsafe fn GetName(&self, pwszname: &mut [u16]) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.base.GetName)(::core::mem::transmute_copy(self), ::core::mem::transmute(::windows::core::as_mut_ptr_or_null(pwszname)), pwszname.len() as _).ok()
    }
    #[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
    pub unsafe fn GetDate(&self) -> ::windows::core::Result<WMDMDATETIME> {
        let mut result__: WMDMDATETIME = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.base.GetDate)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<WMDMDATETIME>(result__)
    }
    #[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
    pub unsafe fn GetSize(&self, pdwsizelow: *mut u32, pdwsizehigh: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.base.GetSize)(::core::mem::transmute_copy(self), ::core::mem::transmute(pdwsizelow), ::core::mem::transmute(pdwsizehigh)).ok()
    }
    #[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
    pub unsafe fn GetRights(&self, pprights: *mut *mut WMDMRIGHTS, pnrightscount: *mut u32, abmac: *mut u8) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.base.GetRights)(::core::mem::transmute_copy(self), ::core::mem::transmute(pprights), ::core::mem::transmute(pnrightscount), ::core::mem::transmute(abmac)).ok()
    }
    #[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
    pub unsafe fn CreateStorage<'a, Param2: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(&self, dwattributes: u32, pformat: *const _WAVEFORMATEX, pwszname: Param2) -> ::windows::core::Result<IMDSPStorage> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.base.CreateStorage)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwattributes), ::core::mem::transmute(pformat), pwszname.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<IMDSPStorage>(result__)
    }
    #[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
    pub unsafe fn EnumStorage(&self) -> ::windows::core::Result<IMDSPEnumStorage> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.base.EnumStorage)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IMDSPEnumStorage>(result__)
    }
    #[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
    pub unsafe fn SendOpaqueCommand(&self, pcommand: *mut OPAQUECOMMAND) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.base.SendOpaqueCommand)(::core::mem::transmute_copy(self), ::core::mem::transmute(pcommand)).ok()
    }
    #[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
    pub unsafe fn GetStorage<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(&self, pszstoragename: Param0) -> ::windows::core::Result<IMDSPStorage> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.GetStorage)(::core::mem::transmute_copy(self), pszstoragename.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<IMDSPStorage>(result__)
    }
    #[doc = "*Required features: `\"Win32_Media_DeviceManager\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CreateStorage2<'a, Param4: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(&self, dwattributes: u32, dwattributesex: u32, paudioformat: *const _WAVEFORMATEX, pvideoformat: *const _VIDEOINFOHEADER, pwszname: Param4, qwfilesize: u64) -> ::windows::core::Result<IMDSPStorage> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.CreateStorage2)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwattributes), ::core::mem::transmute(dwattributesex), ::core::mem::transmute(paudioformat), ::core::mem::transmute(pvideoformat), pwszname.into_param().abi(), ::core::mem::transmute(qwfilesize), ::core::mem::transmute(&mut result__)).from_abi::<IMDSPStorage>(result__)
    }
    #[doc = "*Required features: `\"Win32_Media_DeviceManager\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetAttributes2(&self, dwattributes: u32, dwattributesex: u32, paudioformat: *const _WAVEFORMATEX, pvideoformat: *const _VIDEOINFOHEADER) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.SetAttributes2)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwattributes), ::core::mem::transmute(dwattributesex), ::core::mem::transmute(paudioformat), ::core::mem::transmute(pvideoformat)).ok()
    }
    #[doc = "*Required features: `\"Win32_Media_DeviceManager\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetAttributes2(&self, pdwattributes: *mut u32, pdwattributesex: *mut u32, paudioformat: *mut _WAVEFORMATEX, pvideoformat: *mut _VIDEOINFOHEADER) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.GetAttributes2)(::core::mem::transmute_copy(self), ::core::mem::transmute(pdwattributes), ::core::mem::transmute(pdwattributesex), ::core::mem::transmute(paudioformat), ::core::mem::transmute(pvideoformat)).ok()
    }
    #[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
    pub unsafe fn GetMetadata<'a, Param0: ::windows::core::IntoParam<'a, IWMDMMetaData>>(&self, pmetadata: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetMetadata)(::core::mem::transmute_copy(self), pmetadata.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
    pub unsafe fn SetMetadata<'a, Param0: ::windows::core::IntoParam<'a, IWMDMMetaData>>(&self, pmetadata: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetMetadata)(::core::mem::transmute_copy(self), pmetadata.into_param().abi()).ok()
    }
}
impl ::core::convert::From<IMDSPStorage3> for ::windows::core::IUnknown {
    fn from(value: IMDSPStorage3) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IMDSPStorage3> for ::windows::core::IUnknown {
    fn from(value: &IMDSPStorage3) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IMDSPStorage3 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IMDSPStorage3 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IMDSPStorage3> for IMDSPStorage {
    fn from(value: IMDSPStorage3) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IMDSPStorage3> for IMDSPStorage {
    fn from(value: &IMDSPStorage3) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IMDSPStorage> for IMDSPStorage3 {
    fn into_param(self) -> ::windows::core::Param<'a, IMDSPStorage> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IMDSPStorage> for &'a IMDSPStorage3 {
    fn into_param(self) -> ::windows::core::Param<'a, IMDSPStorage> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IMDSPStorage3> for IMDSPStorage2 {
    fn from(value: IMDSPStorage3) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IMDSPStorage3> for IMDSPStorage2 {
    fn from(value: &IMDSPStorage3) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IMDSPStorage2> for IMDSPStorage3 {
    fn into_param(self) -> ::windows::core::Param<'a, IMDSPStorage2> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IMDSPStorage2> for &'a IMDSPStorage3 {
    fn into_param(self) -> ::windows::core::Param<'a, IMDSPStorage2> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IMDSPStorage3 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
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
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6c669867_97ed_4a67_9706_1c5529d2a414);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMDSPStorage3_Vtbl {
    pub base: IMDSPStorage2_Vtbl,
    pub GetMetadata: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pmetadata: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub SetMetadata: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pmetadata: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
#[repr(transparent)]
pub struct IMDSPStorage4(::windows::core::IUnknown);
impl IMDSPStorage4 {
    #[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
    pub unsafe fn SetAttributes(&self, dwattributes: u32, pformat: *const _WAVEFORMATEX) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.base.base.SetAttributes)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwattributes), ::core::mem::transmute(pformat)).ok()
    }
    #[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
    pub unsafe fn GetStorageGlobals(&self) -> ::windows::core::Result<IMDSPStorageGlobals> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.base.base.GetStorageGlobals)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IMDSPStorageGlobals>(result__)
    }
    #[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
    pub unsafe fn GetAttributes(&self, pdwattributes: *mut u32, pformat: *mut _WAVEFORMATEX) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.base.base.GetAttributes)(::core::mem::transmute_copy(self), ::core::mem::transmute(pdwattributes), ::core::mem::transmute(pformat)).ok()
    }
    #[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
    pub unsafe fn GetName(&self, pwszname: &mut [u16]) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.base.base.GetName)(::core::mem::transmute_copy(self), ::core::mem::transmute(::windows::core::as_mut_ptr_or_null(pwszname)), pwszname.len() as _).ok()
    }
    #[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
    pub unsafe fn GetDate(&self) -> ::windows::core::Result<WMDMDATETIME> {
        let mut result__: WMDMDATETIME = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.base.base.GetDate)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<WMDMDATETIME>(result__)
    }
    #[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
    pub unsafe fn GetSize(&self, pdwsizelow: *mut u32, pdwsizehigh: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.base.base.GetSize)(::core::mem::transmute_copy(self), ::core::mem::transmute(pdwsizelow), ::core::mem::transmute(pdwsizehigh)).ok()
    }
    #[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
    pub unsafe fn GetRights(&self, pprights: *mut *mut WMDMRIGHTS, pnrightscount: *mut u32, abmac: *mut u8) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.base.base.GetRights)(::core::mem::transmute_copy(self), ::core::mem::transmute(pprights), ::core::mem::transmute(pnrightscount), ::core::mem::transmute(abmac)).ok()
    }
    #[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
    pub unsafe fn CreateStorage<'a, Param2: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(&self, dwattributes: u32, pformat: *const _WAVEFORMATEX, pwszname: Param2) -> ::windows::core::Result<IMDSPStorage> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.base.base.CreateStorage)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwattributes), ::core::mem::transmute(pformat), pwszname.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<IMDSPStorage>(result__)
    }
    #[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
    pub unsafe fn EnumStorage(&self) -> ::windows::core::Result<IMDSPEnumStorage> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.base.base.EnumStorage)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IMDSPEnumStorage>(result__)
    }
    #[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
    pub unsafe fn SendOpaqueCommand(&self, pcommand: *mut OPAQUECOMMAND) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.base.base.SendOpaqueCommand)(::core::mem::transmute_copy(self), ::core::mem::transmute(pcommand)).ok()
    }
    #[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
    pub unsafe fn GetStorage<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(&self, pszstoragename: Param0) -> ::windows::core::Result<IMDSPStorage> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.base.GetStorage)(::core::mem::transmute_copy(self), pszstoragename.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<IMDSPStorage>(result__)
    }
    #[doc = "*Required features: `\"Win32_Media_DeviceManager\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CreateStorage2<'a, Param4: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(&self, dwattributes: u32, dwattributesex: u32, paudioformat: *const _WAVEFORMATEX, pvideoformat: *const _VIDEOINFOHEADER, pwszname: Param4, qwfilesize: u64) -> ::windows::core::Result<IMDSPStorage> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.base.CreateStorage2)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwattributes), ::core::mem::transmute(dwattributesex), ::core::mem::transmute(paudioformat), ::core::mem::transmute(pvideoformat), pwszname.into_param().abi(), ::core::mem::transmute(qwfilesize), ::core::mem::transmute(&mut result__)).from_abi::<IMDSPStorage>(result__)
    }
    #[doc = "*Required features: `\"Win32_Media_DeviceManager\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetAttributes2(&self, dwattributes: u32, dwattributesex: u32, paudioformat: *const _WAVEFORMATEX, pvideoformat: *const _VIDEOINFOHEADER) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.base.SetAttributes2)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwattributes), ::core::mem::transmute(dwattributesex), ::core::mem::transmute(paudioformat), ::core::mem::transmute(pvideoformat)).ok()
    }
    #[doc = "*Required features: `\"Win32_Media_DeviceManager\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetAttributes2(&self, pdwattributes: *mut u32, pdwattributesex: *mut u32, paudioformat: *mut _WAVEFORMATEX, pvideoformat: *mut _VIDEOINFOHEADER) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.base.GetAttributes2)(::core::mem::transmute_copy(self), ::core::mem::transmute(pdwattributes), ::core::mem::transmute(pdwattributesex), ::core::mem::transmute(paudioformat), ::core::mem::transmute(pvideoformat)).ok()
    }
    #[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
    pub unsafe fn GetMetadata<'a, Param0: ::windows::core::IntoParam<'a, IWMDMMetaData>>(&self, pmetadata: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.GetMetadata)(::core::mem::transmute_copy(self), pmetadata.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
    pub unsafe fn SetMetadata<'a, Param0: ::windows::core::IntoParam<'a, IWMDMMetaData>>(&self, pmetadata: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.SetMetadata)(::core::mem::transmute_copy(self), pmetadata.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
    pub unsafe fn SetReferences(&self, ppispstorage: &[::core::option::Option<IMDSPStorage>]) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetReferences)(::core::mem::transmute_copy(self), ppispstorage.len() as _, ::core::mem::transmute(::windows::core::as_ptr_or_null(ppispstorage))).ok()
    }
    #[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
    pub unsafe fn GetReferences(&self, pdwrefs: *mut u32, pppispstorage: *mut *mut ::core::option::Option<IMDSPStorage>) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetReferences)(::core::mem::transmute_copy(self), ::core::mem::transmute(pdwrefs), ::core::mem::transmute(pppispstorage)).ok()
    }
    #[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
    pub unsafe fn CreateStorageWithMetadata<'a, Param1: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>, Param2: ::windows::core::IntoParam<'a, IWMDMMetaData>>(&self, dwattributes: u32, pwszname: Param1, pmetadata: Param2, qwfilesize: u64) -> ::windows::core::Result<IMDSPStorage> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).CreateStorageWithMetadata)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwattributes), pwszname.into_param().abi(), pmetadata.into_param().abi(), ::core::mem::transmute(qwfilesize), ::core::mem::transmute(&mut result__)).from_abi::<IMDSPStorage>(result__)
    }
    #[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
    pub unsafe fn GetSpecifiedMetadata<'a, Param2: ::windows::core::IntoParam<'a, IWMDMMetaData>>(&self, ppwszpropnames: &[::windows::core::PWSTR], pmetadata: Param2) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetSpecifiedMetadata)(::core::mem::transmute_copy(self), ppwszpropnames.len() as _, ::core::mem::transmute(::windows::core::as_ptr_or_null(ppwszpropnames)), pmetadata.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
    pub unsafe fn FindStorage<'a, Param1: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(&self, findscope: WMDM_FIND_SCOPE, pwszuniqueid: Param1) -> ::windows::core::Result<IMDSPStorage> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).FindStorage)(::core::mem::transmute_copy(self), ::core::mem::transmute(findscope), pwszuniqueid.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<IMDSPStorage>(result__)
    }
    #[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
    pub unsafe fn GetParent(&self) -> ::windows::core::Result<IMDSPStorage> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetParent)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IMDSPStorage>(result__)
    }
}
impl ::core::convert::From<IMDSPStorage4> for ::windows::core::IUnknown {
    fn from(value: IMDSPStorage4) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IMDSPStorage4> for ::windows::core::IUnknown {
    fn from(value: &IMDSPStorage4) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IMDSPStorage4 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IMDSPStorage4 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IMDSPStorage4> for IMDSPStorage {
    fn from(value: IMDSPStorage4) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IMDSPStorage4> for IMDSPStorage {
    fn from(value: &IMDSPStorage4) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IMDSPStorage> for IMDSPStorage4 {
    fn into_param(self) -> ::windows::core::Param<'a, IMDSPStorage> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IMDSPStorage> for &'a IMDSPStorage4 {
    fn into_param(self) -> ::windows::core::Param<'a, IMDSPStorage> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IMDSPStorage4> for IMDSPStorage2 {
    fn from(value: IMDSPStorage4) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IMDSPStorage4> for IMDSPStorage2 {
    fn from(value: &IMDSPStorage4) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IMDSPStorage2> for IMDSPStorage4 {
    fn into_param(self) -> ::windows::core::Param<'a, IMDSPStorage2> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IMDSPStorage2> for &'a IMDSPStorage4 {
    fn into_param(self) -> ::windows::core::Param<'a, IMDSPStorage2> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IMDSPStorage4> for IMDSPStorage3 {
    fn from(value: IMDSPStorage4) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IMDSPStorage4> for IMDSPStorage3 {
    fn from(value: &IMDSPStorage4) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IMDSPStorage3> for IMDSPStorage4 {
    fn into_param(self) -> ::windows::core::Param<'a, IMDSPStorage3> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IMDSPStorage3> for &'a IMDSPStorage4 {
    fn into_param(self) -> ::windows::core::Param<'a, IMDSPStorage3> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IMDSPStorage4 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
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
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3133b2c4_515c_481b_b1ce_39327ecb4f74);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMDSPStorage4_Vtbl {
    pub base: IMDSPStorage3_Vtbl,
    pub SetReferences: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwrefs: u32, ppispstorage: *const ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub GetReferences: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdwrefs: *mut u32, pppispstorage: *mut *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub CreateStorageWithMetadata: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwattributes: u32, pwszname: ::windows::core::PCWSTR, pmetadata: ::windows::core::RawPtr, qwfilesize: u64, ppnewstorage: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub GetSpecifiedMetadata: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cproperties: u32, ppwszpropnames: *const ::windows::core::PWSTR, pmetadata: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub FindStorage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, findscope: WMDM_FIND_SCOPE, pwszuniqueid: ::windows::core::PCWSTR, ppstorage: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub GetParent: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppstorage: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
#[repr(transparent)]
pub struct IMDSPStorageGlobals(::windows::core::IUnknown);
impl IMDSPStorageGlobals {
    #[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
    pub unsafe fn GetCapabilities(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetCapabilities)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
    pub unsafe fn GetSerialNumber(&self, pserialnum: *mut WMDMID, abmac: *mut u8) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetSerialNumber)(::core::mem::transmute_copy(self), ::core::mem::transmute(pserialnum), ::core::mem::transmute(abmac)).ok()
    }
    #[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
    pub unsafe fn GetTotalSize(&self, pdwtotalsizelow: *mut u32, pdwtotalsizehigh: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetTotalSize)(::core::mem::transmute_copy(self), ::core::mem::transmute(pdwtotalsizelow), ::core::mem::transmute(pdwtotalsizehigh)).ok()
    }
    #[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
    pub unsafe fn GetTotalFree(&self, pdwfreelow: *mut u32, pdwfreehigh: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetTotalFree)(::core::mem::transmute_copy(self), ::core::mem::transmute(pdwfreelow), ::core::mem::transmute(pdwfreehigh)).ok()
    }
    #[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
    pub unsafe fn GetTotalBad(&self, pdwbadlow: *mut u32, pdwbadhigh: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetTotalBad)(::core::mem::transmute_copy(self), ::core::mem::transmute(pdwbadlow), ::core::mem::transmute(pdwbadhigh)).ok()
    }
    #[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
    pub unsafe fn GetStatus(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetStatus)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
    pub unsafe fn Initialize<'a, Param1: ::windows::core::IntoParam<'a, IWMDMProgress>>(&self, fumode: u32, pprogress: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Initialize)(::core::mem::transmute_copy(self), ::core::mem::transmute(fumode), pprogress.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
    pub unsafe fn GetDevice(&self) -> ::windows::core::Result<IMDSPDevice> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetDevice)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IMDSPDevice>(result__)
    }
    #[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
    pub unsafe fn GetRootStorage(&self) -> ::windows::core::Result<IMDSPStorage> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetRootStorage)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IMDSPStorage>(result__)
    }
}
impl ::core::convert::From<IMDSPStorageGlobals> for ::windows::core::IUnknown {
    fn from(value: IMDSPStorageGlobals) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IMDSPStorageGlobals> for ::windows::core::IUnknown {
    fn from(value: &IMDSPStorageGlobals) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IMDSPStorageGlobals {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IMDSPStorageGlobals {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IMDSPStorageGlobals {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
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
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1dcb3a17_33ed_11d3_8470_00c04f79dbc0);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMDSPStorageGlobals_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    pub GetCapabilities: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdwcapabilities: *mut u32) -> ::windows::core::HRESULT,
    pub GetSerialNumber: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pserialnum: *mut WMDMID, abmac: *mut u8) -> ::windows::core::HRESULT,
    pub GetTotalSize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdwtotalsizelow: *mut u32, pdwtotalsizehigh: *mut u32) -> ::windows::core::HRESULT,
    pub GetTotalFree: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdwfreelow: *mut u32, pdwfreehigh: *mut u32) -> ::windows::core::HRESULT,
    pub GetTotalBad: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdwbadlow: *mut u32, pdwbadhigh: *mut u32) -> ::windows::core::HRESULT,
    pub GetStatus: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdwstatus: *mut u32) -> ::windows::core::HRESULT,
    pub Initialize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, fumode: u32, pprogress: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub GetDevice: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppdevice: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub GetRootStorage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pproot: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
#[repr(transparent)]
pub struct IMDServiceProvider(::windows::core::IUnknown);
impl IMDServiceProvider {
    #[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
    pub unsafe fn GetDeviceCount(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetDeviceCount)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
    pub unsafe fn EnumDevices(&self) -> ::windows::core::Result<IMDSPEnumDevice> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).EnumDevices)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IMDSPEnumDevice>(result__)
    }
}
impl ::core::convert::From<IMDServiceProvider> for ::windows::core::IUnknown {
    fn from(value: IMDServiceProvider) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IMDServiceProvider> for ::windows::core::IUnknown {
    fn from(value: &IMDServiceProvider) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IMDServiceProvider {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IMDServiceProvider {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IMDServiceProvider {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
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
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1dcb3a10_33ed_11d3_8470_00c04f79dbc0);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMDServiceProvider_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    pub GetDeviceCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdwcount: *mut u32) -> ::windows::core::HRESULT,
    pub EnumDevices: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppenumdevice: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
#[repr(transparent)]
pub struct IMDServiceProvider2(::windows::core::IUnknown);
impl IMDServiceProvider2 {
    #[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
    pub unsafe fn GetDeviceCount(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.GetDeviceCount)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
    pub unsafe fn EnumDevices(&self) -> ::windows::core::Result<IMDSPEnumDevice> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.EnumDevices)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IMDSPEnumDevice>(result__)
    }
    #[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
    pub unsafe fn CreateDevice<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(&self, pwszdevicepath: Param0, pdwcount: *mut u32, pppdevicearray: *mut *mut ::core::option::Option<IMDSPDevice>) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).CreateDevice)(::core::mem::transmute_copy(self), pwszdevicepath.into_param().abi(), ::core::mem::transmute(pdwcount), ::core::mem::transmute(pppdevicearray)).ok()
    }
}
impl ::core::convert::From<IMDServiceProvider2> for ::windows::core::IUnknown {
    fn from(value: IMDServiceProvider2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IMDServiceProvider2> for ::windows::core::IUnknown {
    fn from(value: &IMDServiceProvider2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IMDServiceProvider2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IMDServiceProvider2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IMDServiceProvider2> for IMDServiceProvider {
    fn from(value: IMDServiceProvider2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IMDServiceProvider2> for IMDServiceProvider {
    fn from(value: &IMDServiceProvider2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IMDServiceProvider> for IMDServiceProvider2 {
    fn into_param(self) -> ::windows::core::Param<'a, IMDServiceProvider> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IMDServiceProvider> for &'a IMDServiceProvider2 {
    fn into_param(self) -> ::windows::core::Param<'a, IMDServiceProvider> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IMDServiceProvider2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
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
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb2fa24b7_cda3_4694_9862_413ae1a34819);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMDServiceProvider2_Vtbl {
    pub base: IMDServiceProvider_Vtbl,
    pub CreateDevice: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwszdevicepath: ::windows::core::PCWSTR, pdwcount: *mut u32, pppdevicearray: *mut *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
#[repr(transparent)]
pub struct IMDServiceProvider3(::windows::core::IUnknown);
impl IMDServiceProvider3 {
    #[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
    pub unsafe fn GetDeviceCount(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.base.GetDeviceCount)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
    pub unsafe fn EnumDevices(&self) -> ::windows::core::Result<IMDSPEnumDevice> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.base.EnumDevices)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IMDSPEnumDevice>(result__)
    }
    #[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
    pub unsafe fn CreateDevice<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(&self, pwszdevicepath: Param0, pdwcount: *mut u32, pppdevicearray: *mut *mut ::core::option::Option<IMDSPDevice>) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.CreateDevice)(::core::mem::transmute_copy(self), pwszdevicepath.into_param().abi(), ::core::mem::transmute(pdwcount), ::core::mem::transmute(pppdevicearray)).ok()
    }
    #[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
    pub unsafe fn SetDeviceEnumPreference(&self, dwenumpref: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetDeviceEnumPreference)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwenumpref)).ok()
    }
}
impl ::core::convert::From<IMDServiceProvider3> for ::windows::core::IUnknown {
    fn from(value: IMDServiceProvider3) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IMDServiceProvider3> for ::windows::core::IUnknown {
    fn from(value: &IMDServiceProvider3) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IMDServiceProvider3 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IMDServiceProvider3 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IMDServiceProvider3> for IMDServiceProvider {
    fn from(value: IMDServiceProvider3) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IMDServiceProvider3> for IMDServiceProvider {
    fn from(value: &IMDServiceProvider3) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IMDServiceProvider> for IMDServiceProvider3 {
    fn into_param(self) -> ::windows::core::Param<'a, IMDServiceProvider> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IMDServiceProvider> for &'a IMDServiceProvider3 {
    fn into_param(self) -> ::windows::core::Param<'a, IMDServiceProvider> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IMDServiceProvider3> for IMDServiceProvider2 {
    fn from(value: IMDServiceProvider3) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IMDServiceProvider3> for IMDServiceProvider2 {
    fn from(value: &IMDServiceProvider3) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IMDServiceProvider2> for IMDServiceProvider3 {
    fn into_param(self) -> ::windows::core::Param<'a, IMDServiceProvider2> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IMDServiceProvider2> for &'a IMDServiceProvider3 {
    fn into_param(self) -> ::windows::core::Param<'a, IMDServiceProvider2> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IMDServiceProvider3 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
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
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4ed13ef3_a971_4d19_9f51_0e1826b2da57);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMDServiceProvider3_Vtbl {
    pub base: IMDServiceProvider2_Vtbl,
    pub SetDeviceEnumPreference: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwenumpref: u32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const IOCTL_MTP_CUSTOM_COMMAND: u32 = 827348045u32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
#[repr(transparent)]
pub struct ISCPSecureAuthenticate(::windows::core::IUnknown);
impl ISCPSecureAuthenticate {
    #[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
    pub unsafe fn GetSecureQuery(&self) -> ::windows::core::Result<ISCPSecureQuery> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetSecureQuery)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<ISCPSecureQuery>(result__)
    }
}
impl ::core::convert::From<ISCPSecureAuthenticate> for ::windows::core::IUnknown {
    fn from(value: ISCPSecureAuthenticate) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ISCPSecureAuthenticate> for ::windows::core::IUnknown {
    fn from(value: &ISCPSecureAuthenticate) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ISCPSecureAuthenticate {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a ISCPSecureAuthenticate {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for ISCPSecureAuthenticate {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
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
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1dcb3a0f_33ed_11d3_8470_00c04f79dbc0);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISCPSecureAuthenticate_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    pub GetSecureQuery: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppsecurequery: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
#[repr(transparent)]
pub struct ISCPSecureAuthenticate2(::windows::core::IUnknown);
impl ISCPSecureAuthenticate2 {
    #[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
    pub unsafe fn GetSecureQuery(&self) -> ::windows::core::Result<ISCPSecureQuery> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.GetSecureQuery)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<ISCPSecureQuery>(result__)
    }
    #[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
    pub unsafe fn GetSCPSession(&self) -> ::windows::core::Result<ISCPSession> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetSCPSession)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<ISCPSession>(result__)
    }
}
impl ::core::convert::From<ISCPSecureAuthenticate2> for ::windows::core::IUnknown {
    fn from(value: ISCPSecureAuthenticate2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ISCPSecureAuthenticate2> for ::windows::core::IUnknown {
    fn from(value: &ISCPSecureAuthenticate2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ISCPSecureAuthenticate2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a ISCPSecureAuthenticate2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ISCPSecureAuthenticate2> for ISCPSecureAuthenticate {
    fn from(value: ISCPSecureAuthenticate2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ISCPSecureAuthenticate2> for ISCPSecureAuthenticate {
    fn from(value: &ISCPSecureAuthenticate2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ISCPSecureAuthenticate> for ISCPSecureAuthenticate2 {
    fn into_param(self) -> ::windows::core::Param<'a, ISCPSecureAuthenticate> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ISCPSecureAuthenticate> for &'a ISCPSecureAuthenticate2 {
    fn into_param(self) -> ::windows::core::Param<'a, ISCPSecureAuthenticate> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for ISCPSecureAuthenticate2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
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
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb580cfae_1672_47e2_acaa_44bbecbcae5b);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISCPSecureAuthenticate2_Vtbl {
    pub base: ISCPSecureAuthenticate_Vtbl,
    pub GetSCPSession: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppscpsession: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
#[repr(transparent)]
pub struct ISCPSecureExchange(::windows::core::IUnknown);
impl ISCPSecureExchange {
    #[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
    pub unsafe fn TransferContainerData(&self, pdata: &[u8], pfureadyflags: *mut u32, abmac: *mut u8) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).TransferContainerData)(::core::mem::transmute_copy(self), ::core::mem::transmute(::windows::core::as_ptr_or_null(pdata)), pdata.len() as _, ::core::mem::transmute(pfureadyflags), ::core::mem::transmute(abmac)).ok()
    }
    #[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
    pub unsafe fn ObjectData(&self, pdata: *mut u8, pdwsize: *mut u32, abmac: *mut u8) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).ObjectData)(::core::mem::transmute_copy(self), ::core::mem::transmute(pdata), ::core::mem::transmute(pdwsize), ::core::mem::transmute(abmac)).ok()
    }
    #[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
    pub unsafe fn TransferComplete(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).TransferComplete)(::core::mem::transmute_copy(self)).ok()
    }
}
impl ::core::convert::From<ISCPSecureExchange> for ::windows::core::IUnknown {
    fn from(value: ISCPSecureExchange) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ISCPSecureExchange> for ::windows::core::IUnknown {
    fn from(value: &ISCPSecureExchange) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ISCPSecureExchange {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a ISCPSecureExchange {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for ISCPSecureExchange {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
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
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1dcb3a0e_33ed_11d3_8470_00c04f79dbc0);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISCPSecureExchange_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    pub TransferContainerData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdata: *const u8, dwsize: u32, pfureadyflags: *mut u32, abmac: *mut u8) -> ::windows::core::HRESULT,
    pub ObjectData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdata: *mut u8, pdwsize: *mut u32, abmac: *mut u8) -> ::windows::core::HRESULT,
    pub TransferComplete: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
#[repr(transparent)]
pub struct ISCPSecureExchange2(::windows::core::IUnknown);
impl ISCPSecureExchange2 {
    #[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
    pub unsafe fn TransferContainerData(&self, pdata: &[u8], pfureadyflags: *mut u32, abmac: *mut u8) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.TransferContainerData)(::core::mem::transmute_copy(self), ::core::mem::transmute(::windows::core::as_ptr_or_null(pdata)), pdata.len() as _, ::core::mem::transmute(pfureadyflags), ::core::mem::transmute(abmac)).ok()
    }
    #[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
    pub unsafe fn ObjectData(&self, pdata: *mut u8, pdwsize: *mut u32, abmac: *mut u8) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.ObjectData)(::core::mem::transmute_copy(self), ::core::mem::transmute(pdata), ::core::mem::transmute(pdwsize), ::core::mem::transmute(abmac)).ok()
    }
    #[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
    pub unsafe fn TransferComplete(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.TransferComplete)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
    pub unsafe fn TransferContainerData2<'a, Param2: ::windows::core::IntoParam<'a, IWMDMProgress3>>(&self, pdata: &[u8], pprogresscallback: Param2, pfureadyflags: *mut u32, abmac: *mut u8) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).TransferContainerData2)(::core::mem::transmute_copy(self), ::core::mem::transmute(::windows::core::as_ptr_or_null(pdata)), pdata.len() as _, pprogresscallback.into_param().abi(), ::core::mem::transmute(pfureadyflags), ::core::mem::transmute(abmac)).ok()
    }
}
impl ::core::convert::From<ISCPSecureExchange2> for ::windows::core::IUnknown {
    fn from(value: ISCPSecureExchange2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ISCPSecureExchange2> for ::windows::core::IUnknown {
    fn from(value: &ISCPSecureExchange2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ISCPSecureExchange2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a ISCPSecureExchange2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ISCPSecureExchange2> for ISCPSecureExchange {
    fn from(value: ISCPSecureExchange2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ISCPSecureExchange2> for ISCPSecureExchange {
    fn from(value: &ISCPSecureExchange2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ISCPSecureExchange> for ISCPSecureExchange2 {
    fn into_param(self) -> ::windows::core::Param<'a, ISCPSecureExchange> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ISCPSecureExchange> for &'a ISCPSecureExchange2 {
    fn into_param(self) -> ::windows::core::Param<'a, ISCPSecureExchange> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for ISCPSecureExchange2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
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
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6c62fc7b_2690_483f_9d44_0a20cb35577c);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISCPSecureExchange2_Vtbl {
    pub base: ISCPSecureExchange_Vtbl,
    pub TransferContainerData2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdata: *const u8, dwsize: u32, pprogresscallback: ::windows::core::RawPtr, pfureadyflags: *mut u32, abmac: *mut u8) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
#[repr(transparent)]
pub struct ISCPSecureExchange3(::windows::core::IUnknown);
impl ISCPSecureExchange3 {
    #[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
    pub unsafe fn TransferContainerData(&self, pdata: &[u8], pfureadyflags: *mut u32, abmac: *mut u8) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.base.TransferContainerData)(::core::mem::transmute_copy(self), ::core::mem::transmute(::windows::core::as_ptr_or_null(pdata)), pdata.len() as _, ::core::mem::transmute(pfureadyflags), ::core::mem::transmute(abmac)).ok()
    }
    #[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
    pub unsafe fn ObjectData(&self, pdata: *mut u8, pdwsize: *mut u32, abmac: *mut u8) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.base.ObjectData)(::core::mem::transmute_copy(self), ::core::mem::transmute(pdata), ::core::mem::transmute(pdwsize), ::core::mem::transmute(abmac)).ok()
    }
    #[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
    pub unsafe fn TransferComplete(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.base.TransferComplete)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
    pub unsafe fn TransferContainerData2<'a, Param2: ::windows::core::IntoParam<'a, IWMDMProgress3>>(&self, pdata: &[u8], pprogresscallback: Param2, pfureadyflags: *mut u32, abmac: *mut u8) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.TransferContainerData2)(::core::mem::transmute_copy(self), ::core::mem::transmute(::windows::core::as_ptr_or_null(pdata)), pdata.len() as _, pprogresscallback.into_param().abi(), ::core::mem::transmute(pfureadyflags), ::core::mem::transmute(abmac)).ok()
    }
    #[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
    pub unsafe fn TransferContainerDataOnClearChannel<'a, Param0: ::windows::core::IntoParam<'a, IMDSPDevice>, Param3: ::windows::core::IntoParam<'a, IWMDMProgress3>>(&self, pdevice: Param0, pdata: &[u8], pprogresscallback: Param3) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).TransferContainerDataOnClearChannel)(::core::mem::transmute_copy(self), pdevice.into_param().abi(), ::core::mem::transmute(::windows::core::as_ptr_or_null(pdata)), pdata.len() as _, pprogresscallback.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
    pub unsafe fn GetObjectDataOnClearChannel<'a, Param0: ::windows::core::IntoParam<'a, IMDSPDevice>>(&self, pdevice: Param0, pdata: *mut u8, pdwsize: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetObjectDataOnClearChannel)(::core::mem::transmute_copy(self), pdevice.into_param().abi(), ::core::mem::transmute(pdata), ::core::mem::transmute(pdwsize)).ok()
    }
    #[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
    pub unsafe fn TransferCompleteForDevice<'a, Param0: ::windows::core::IntoParam<'a, IMDSPDevice>>(&self, pdevice: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).TransferCompleteForDevice)(::core::mem::transmute_copy(self), pdevice.into_param().abi()).ok()
    }
}
impl ::core::convert::From<ISCPSecureExchange3> for ::windows::core::IUnknown {
    fn from(value: ISCPSecureExchange3) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ISCPSecureExchange3> for ::windows::core::IUnknown {
    fn from(value: &ISCPSecureExchange3) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ISCPSecureExchange3 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a ISCPSecureExchange3 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ISCPSecureExchange3> for ISCPSecureExchange {
    fn from(value: ISCPSecureExchange3) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ISCPSecureExchange3> for ISCPSecureExchange {
    fn from(value: &ISCPSecureExchange3) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ISCPSecureExchange> for ISCPSecureExchange3 {
    fn into_param(self) -> ::windows::core::Param<'a, ISCPSecureExchange> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ISCPSecureExchange> for &'a ISCPSecureExchange3 {
    fn into_param(self) -> ::windows::core::Param<'a, ISCPSecureExchange> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ISCPSecureExchange3> for ISCPSecureExchange2 {
    fn from(value: ISCPSecureExchange3) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ISCPSecureExchange3> for ISCPSecureExchange2 {
    fn from(value: &ISCPSecureExchange3) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ISCPSecureExchange2> for ISCPSecureExchange3 {
    fn into_param(self) -> ::windows::core::Param<'a, ISCPSecureExchange2> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ISCPSecureExchange2> for &'a ISCPSecureExchange3 {
    fn into_param(self) -> ::windows::core::Param<'a, ISCPSecureExchange2> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for ISCPSecureExchange3 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
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
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xab4e77e4_8908_4b17_bd2a_b1dbe6dd69e1);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISCPSecureExchange3_Vtbl {
    pub base: ISCPSecureExchange2_Vtbl,
    pub TransferContainerDataOnClearChannel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdevice: ::windows::core::RawPtr, pdata: *const u8, dwsize: u32, pprogresscallback: ::windows::core::RawPtr, pfureadyflags: *mut u32) -> ::windows::core::HRESULT,
    pub GetObjectDataOnClearChannel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdevice: ::windows::core::RawPtr, pdata: *mut u8, pdwsize: *mut u32) -> ::windows::core::HRESULT,
    pub TransferCompleteForDevice: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdevice: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
#[repr(transparent)]
pub struct ISCPSecureQuery(::windows::core::IUnknown);
impl ISCPSecureQuery {
    #[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
    pub unsafe fn GetDataDemands(&self, pfuflags: *mut u32, pdwminrightsdata: *mut u32, pdwminexaminedata: *mut u32, pdwmindecidedata: *mut u32, abmac: *mut u8) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetDataDemands)(::core::mem::transmute_copy(self), ::core::mem::transmute(pfuflags), ::core::mem::transmute(pdwminrightsdata), ::core::mem::transmute(pdwminexaminedata), ::core::mem::transmute(pdwmindecidedata), ::core::mem::transmute(abmac)).ok()
    }
    #[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
    pub unsafe fn ExamineData<'a, Param1: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(&self, fuflags: u32, pwszextension: Param1, pdata: &[u8], abmac: *mut u8) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).ExamineData)(::core::mem::transmute_copy(self), ::core::mem::transmute(fuflags), pwszextension.into_param().abi(), ::core::mem::transmute(::windows::core::as_ptr_or_null(pdata)), pdata.len() as _, ::core::mem::transmute(abmac)).ok()
    }
    #[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
    pub unsafe fn MakeDecision<'a, Param6: ::windows::core::IntoParam<'a, IMDSPStorageGlobals>>(&self, fuflags: u32, pdata: &[u8], dwappsec: u32, pbspsessionkey: &[u8], pstorageglobals: Param6, ppexchange: *mut ::core::option::Option<ISCPSecureExchange>, abmac: *mut u8) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).MakeDecision)(::core::mem::transmute_copy(self), ::core::mem::transmute(fuflags), ::core::mem::transmute(::windows::core::as_ptr_or_null(pdata)), pdata.len() as _, ::core::mem::transmute(dwappsec), ::core::mem::transmute(::windows::core::as_ptr_or_null(pbspsessionkey)), pbspsessionkey.len() as _, pstorageglobals.into_param().abi(), ::core::mem::transmute(ppexchange), ::core::mem::transmute(abmac)).ok()
    }
    #[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
    pub unsafe fn GetRights<'a, Param4: ::windows::core::IntoParam<'a, IMDSPStorageGlobals>>(&self, pdata: &[u8], pbspsessionkey: &[u8], pstgglobals: Param4, pprights: *mut *mut WMDMRIGHTS, pnrightscount: *mut u32, abmac: *mut u8) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetRights)(::core::mem::transmute_copy(self), ::core::mem::transmute(::windows::core::as_ptr_or_null(pdata)), pdata.len() as _, ::core::mem::transmute(::windows::core::as_ptr_or_null(pbspsessionkey)), pbspsessionkey.len() as _, pstgglobals.into_param().abi(), ::core::mem::transmute(pprights), ::core::mem::transmute(pnrightscount), ::core::mem::transmute(abmac)).ok()
    }
}
impl ::core::convert::From<ISCPSecureQuery> for ::windows::core::IUnknown {
    fn from(value: ISCPSecureQuery) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ISCPSecureQuery> for ::windows::core::IUnknown {
    fn from(value: &ISCPSecureQuery) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ISCPSecureQuery {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a ISCPSecureQuery {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for ISCPSecureQuery {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
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
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1dcb3a0d_33ed_11d3_8470_00c04f79dbc0);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISCPSecureQuery_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    pub GetDataDemands: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pfuflags: *mut u32, pdwminrightsdata: *mut u32, pdwminexaminedata: *mut u32, pdwmindecidedata: *mut u32, abmac: *mut u8) -> ::windows::core::HRESULT,
    pub ExamineData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, fuflags: u32, pwszextension: ::windows::core::PCWSTR, pdata: *const u8, dwsize: u32, abmac: *mut u8) -> ::windows::core::HRESULT,
    pub MakeDecision: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, fuflags: u32, pdata: *const u8, dwsize: u32, dwappsec: u32, pbspsessionkey: *const u8, dwsessionkeylen: u32, pstorageglobals: ::windows::core::RawPtr, ppexchange: *mut ::windows::core::RawPtr, abmac: *mut u8) -> ::windows::core::HRESULT,
    pub GetRights: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdata: *const u8, dwsize: u32, pbspsessionkey: *const u8, dwsessionkeylen: u32, pstgglobals: ::windows::core::RawPtr, pprights: *mut *mut WMDMRIGHTS, pnrightscount: *mut u32, abmac: *mut u8) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
#[repr(transparent)]
pub struct ISCPSecureQuery2(::windows::core::IUnknown);
impl ISCPSecureQuery2 {
    #[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
    pub unsafe fn GetDataDemands(&self, pfuflags: *mut u32, pdwminrightsdata: *mut u32, pdwminexaminedata: *mut u32, pdwmindecidedata: *mut u32, abmac: *mut u8) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.GetDataDemands)(::core::mem::transmute_copy(self), ::core::mem::transmute(pfuflags), ::core::mem::transmute(pdwminrightsdata), ::core::mem::transmute(pdwminexaminedata), ::core::mem::transmute(pdwmindecidedata), ::core::mem::transmute(abmac)).ok()
    }
    #[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
    pub unsafe fn ExamineData<'a, Param1: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(&self, fuflags: u32, pwszextension: Param1, pdata: &[u8], abmac: *mut u8) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.ExamineData)(::core::mem::transmute_copy(self), ::core::mem::transmute(fuflags), pwszextension.into_param().abi(), ::core::mem::transmute(::windows::core::as_ptr_or_null(pdata)), pdata.len() as _, ::core::mem::transmute(abmac)).ok()
    }
    #[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
    pub unsafe fn MakeDecision<'a, Param6: ::windows::core::IntoParam<'a, IMDSPStorageGlobals>>(&self, fuflags: u32, pdata: &[u8], dwappsec: u32, pbspsessionkey: &[u8], pstorageglobals: Param6, ppexchange: *mut ::core::option::Option<ISCPSecureExchange>, abmac: *mut u8) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.MakeDecision)(::core::mem::transmute_copy(self), ::core::mem::transmute(fuflags), ::core::mem::transmute(::windows::core::as_ptr_or_null(pdata)), pdata.len() as _, ::core::mem::transmute(dwappsec), ::core::mem::transmute(::windows::core::as_ptr_or_null(pbspsessionkey)), pbspsessionkey.len() as _, pstorageglobals.into_param().abi(), ::core::mem::transmute(ppexchange), ::core::mem::transmute(abmac)).ok()
    }
    #[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
    pub unsafe fn GetRights<'a, Param4: ::windows::core::IntoParam<'a, IMDSPStorageGlobals>>(&self, pdata: &[u8], pbspsessionkey: &[u8], pstgglobals: Param4, pprights: *mut *mut WMDMRIGHTS, pnrightscount: *mut u32, abmac: *mut u8) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.GetRights)(::core::mem::transmute_copy(self), ::core::mem::transmute(::windows::core::as_ptr_or_null(pdata)), pdata.len() as _, ::core::mem::transmute(::windows::core::as_ptr_or_null(pbspsessionkey)), pbspsessionkey.len() as _, pstgglobals.into_param().abi(), ::core::mem::transmute(pprights), ::core::mem::transmute(pnrightscount), ::core::mem::transmute(abmac)).ok()
    }
    #[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
    pub unsafe fn MakeDecision2<'a, Param6: ::windows::core::IntoParam<'a, IMDSPStorageGlobals>, Param15: ::windows::core::IntoParam<'a, ::windows::core::IUnknown>>(&self, fuflags: u32, pdata: &[u8], dwappsec: u32, pbspsessionkey: &[u8], pstorageglobals: Param6, pappcertapp: &[u8], pappcertsp: &[u8], pszrevocationurl: *mut ::windows::core::PWSTR, pdwrevocationurllen: *mut u32, pdwrevocationbitflag: *mut u32, pqwfilesize: *mut u64, punknown: Param15, ppexchange: *mut ::core::option::Option<ISCPSecureExchange>, abmac: *mut u8) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).MakeDecision2)(
            ::core::mem::transmute_copy(self),
            ::core::mem::transmute(fuflags),
            ::core::mem::transmute(::windows::core::as_ptr_or_null(pdata)),
            pdata.len() as _,
            ::core::mem::transmute(dwappsec),
            ::core::mem::transmute(::windows::core::as_ptr_or_null(pbspsessionkey)),
            pbspsessionkey.len() as _,
            pstorageglobals.into_param().abi(),
            ::core::mem::transmute(::windows::core::as_ptr_or_null(pappcertapp)),
            pappcertapp.len() as _,
            ::core::mem::transmute(::windows::core::as_ptr_or_null(pappcertsp)),
            pappcertsp.len() as _,
            ::core::mem::transmute(pszrevocationurl),
            ::core::mem::transmute(pdwrevocationurllen),
            ::core::mem::transmute(pdwrevocationbitflag),
            ::core::mem::transmute(pqwfilesize),
            punknown.into_param().abi(),
            ::core::mem::transmute(ppexchange),
            ::core::mem::transmute(abmac),
        )
        .ok()
    }
}
impl ::core::convert::From<ISCPSecureQuery2> for ::windows::core::IUnknown {
    fn from(value: ISCPSecureQuery2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ISCPSecureQuery2> for ::windows::core::IUnknown {
    fn from(value: &ISCPSecureQuery2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ISCPSecureQuery2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a ISCPSecureQuery2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ISCPSecureQuery2> for ISCPSecureQuery {
    fn from(value: ISCPSecureQuery2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ISCPSecureQuery2> for ISCPSecureQuery {
    fn from(value: &ISCPSecureQuery2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ISCPSecureQuery> for ISCPSecureQuery2 {
    fn into_param(self) -> ::windows::core::Param<'a, ISCPSecureQuery> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ISCPSecureQuery> for &'a ISCPSecureQuery2 {
    fn into_param(self) -> ::windows::core::Param<'a, ISCPSecureQuery> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for ISCPSecureQuery2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
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
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xebe17e25_4fd7_4632_af46_6d93d4fcc72e);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISCPSecureQuery2_Vtbl {
    pub base: ISCPSecureQuery_Vtbl,
    pub MakeDecision2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, fuflags: u32, pdata: *const u8, dwsize: u32, dwappsec: u32, pbspsessionkey: *const u8, dwsessionkeylen: u32, pstorageglobals: ::windows::core::RawPtr, pappcertapp: *const u8, dwappcertapplen: u32, pappcertsp: *const u8, dwappcertsplen: u32, pszrevocationurl: *mut ::windows::core::PWSTR, pdwrevocationurllen: *mut u32, pdwrevocationbitflag: *mut u32, pqwfilesize: *mut u64, punknown: *mut ::core::ffi::c_void, ppexchange: *mut ::windows::core::RawPtr, abmac: *mut u8) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
#[repr(transparent)]
pub struct ISCPSecureQuery3(::windows::core::IUnknown);
impl ISCPSecureQuery3 {
    #[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
    pub unsafe fn GetDataDemands(&self, pfuflags: *mut u32, pdwminrightsdata: *mut u32, pdwminexaminedata: *mut u32, pdwmindecidedata: *mut u32, abmac: *mut u8) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.base.GetDataDemands)(::core::mem::transmute_copy(self), ::core::mem::transmute(pfuflags), ::core::mem::transmute(pdwminrightsdata), ::core::mem::transmute(pdwminexaminedata), ::core::mem::transmute(pdwmindecidedata), ::core::mem::transmute(abmac)).ok()
    }
    #[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
    pub unsafe fn ExamineData<'a, Param1: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(&self, fuflags: u32, pwszextension: Param1, pdata: &[u8], abmac: *mut u8) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.base.ExamineData)(::core::mem::transmute_copy(self), ::core::mem::transmute(fuflags), pwszextension.into_param().abi(), ::core::mem::transmute(::windows::core::as_ptr_or_null(pdata)), pdata.len() as _, ::core::mem::transmute(abmac)).ok()
    }
    #[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
    pub unsafe fn MakeDecision<'a, Param6: ::windows::core::IntoParam<'a, IMDSPStorageGlobals>>(&self, fuflags: u32, pdata: &[u8], dwappsec: u32, pbspsessionkey: &[u8], pstorageglobals: Param6, ppexchange: *mut ::core::option::Option<ISCPSecureExchange>, abmac: *mut u8) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.base.MakeDecision)(::core::mem::transmute_copy(self), ::core::mem::transmute(fuflags), ::core::mem::transmute(::windows::core::as_ptr_or_null(pdata)), pdata.len() as _, ::core::mem::transmute(dwappsec), ::core::mem::transmute(::windows::core::as_ptr_or_null(pbspsessionkey)), pbspsessionkey.len() as _, pstorageglobals.into_param().abi(), ::core::mem::transmute(ppexchange), ::core::mem::transmute(abmac)).ok()
    }
    #[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
    pub unsafe fn GetRights<'a, Param4: ::windows::core::IntoParam<'a, IMDSPStorageGlobals>>(&self, pdata: &[u8], pbspsessionkey: &[u8], pstgglobals: Param4, pprights: *mut *mut WMDMRIGHTS, pnrightscount: *mut u32, abmac: *mut u8) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.base.GetRights)(::core::mem::transmute_copy(self), ::core::mem::transmute(::windows::core::as_ptr_or_null(pdata)), pdata.len() as _, ::core::mem::transmute(::windows::core::as_ptr_or_null(pbspsessionkey)), pbspsessionkey.len() as _, pstgglobals.into_param().abi(), ::core::mem::transmute(pprights), ::core::mem::transmute(pnrightscount), ::core::mem::transmute(abmac)).ok()
    }
    #[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
    pub unsafe fn MakeDecision2<'a, Param6: ::windows::core::IntoParam<'a, IMDSPStorageGlobals>, Param15: ::windows::core::IntoParam<'a, ::windows::core::IUnknown>>(&self, fuflags: u32, pdata: &[u8], dwappsec: u32, pbspsessionkey: &[u8], pstorageglobals: Param6, pappcertapp: &[u8], pappcertsp: &[u8], pszrevocationurl: *mut ::windows::core::PWSTR, pdwrevocationurllen: *mut u32, pdwrevocationbitflag: *mut u32, pqwfilesize: *mut u64, punknown: Param15, ppexchange: *mut ::core::option::Option<ISCPSecureExchange>, abmac: *mut u8) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.MakeDecision2)(
            ::core::mem::transmute_copy(self),
            ::core::mem::transmute(fuflags),
            ::core::mem::transmute(::windows::core::as_ptr_or_null(pdata)),
            pdata.len() as _,
            ::core::mem::transmute(dwappsec),
            ::core::mem::transmute(::windows::core::as_ptr_or_null(pbspsessionkey)),
            pbspsessionkey.len() as _,
            pstorageglobals.into_param().abi(),
            ::core::mem::transmute(::windows::core::as_ptr_or_null(pappcertapp)),
            pappcertapp.len() as _,
            ::core::mem::transmute(::windows::core::as_ptr_or_null(pappcertsp)),
            pappcertsp.len() as _,
            ::core::mem::transmute(pszrevocationurl),
            ::core::mem::transmute(pdwrevocationurllen),
            ::core::mem::transmute(pdwrevocationbitflag),
            ::core::mem::transmute(pqwfilesize),
            punknown.into_param().abi(),
            ::core::mem::transmute(ppexchange),
            ::core::mem::transmute(abmac),
        )
        .ok()
    }
    #[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
    pub unsafe fn GetRightsOnClearChannel<'a, Param4: ::windows::core::IntoParam<'a, IMDSPStorageGlobals>, Param5: ::windows::core::IntoParam<'a, IWMDMProgress3>>(&self, pdata: &[u8], pbspsessionkey: &[u8], pstgglobals: Param4, pprogresscallback: Param5, pprights: *mut *mut WMDMRIGHTS, pnrightscount: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetRightsOnClearChannel)(::core::mem::transmute_copy(self), ::core::mem::transmute(::windows::core::as_ptr_or_null(pdata)), pdata.len() as _, ::core::mem::transmute(::windows::core::as_ptr_or_null(pbspsessionkey)), pbspsessionkey.len() as _, pstgglobals.into_param().abi(), pprogresscallback.into_param().abi(), ::core::mem::transmute(pprights), ::core::mem::transmute(pnrightscount)).ok()
    }
    #[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
    pub unsafe fn MakeDecisionOnClearChannel<'a, Param6: ::windows::core::IntoParam<'a, IMDSPStorageGlobals>, Param7: ::windows::core::IntoParam<'a, IWMDMProgress3>, Param16: ::windows::core::IntoParam<'a, ::windows::core::IUnknown>>(&self, fuflags: u32, pdata: &[u8], dwappsec: u32, pbspsessionkey: &[u8], pstorageglobals: Param6, pprogresscallback: Param7, pappcertapp: &[u8], pappcertsp: &[u8], pszrevocationurl: *mut ::windows::core::PWSTR, pdwrevocationurllen: *mut u32, pdwrevocationbitflag: *mut u32, pqwfilesize: *mut u64, punknown: Param16, ppexchange: *mut ::core::option::Option<ISCPSecureExchange>) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).MakeDecisionOnClearChannel)(
            ::core::mem::transmute_copy(self),
            ::core::mem::transmute(fuflags),
            ::core::mem::transmute(::windows::core::as_ptr_or_null(pdata)),
            pdata.len() as _,
            ::core::mem::transmute(dwappsec),
            ::core::mem::transmute(::windows::core::as_ptr_or_null(pbspsessionkey)),
            pbspsessionkey.len() as _,
            pstorageglobals.into_param().abi(),
            pprogresscallback.into_param().abi(),
            ::core::mem::transmute(::windows::core::as_ptr_or_null(pappcertapp)),
            pappcertapp.len() as _,
            ::core::mem::transmute(::windows::core::as_ptr_or_null(pappcertsp)),
            pappcertsp.len() as _,
            ::core::mem::transmute(pszrevocationurl),
            ::core::mem::transmute(pdwrevocationurllen),
            ::core::mem::transmute(pdwrevocationbitflag),
            ::core::mem::transmute(pqwfilesize),
            punknown.into_param().abi(),
            ::core::mem::transmute(ppexchange),
        )
        .ok()
    }
}
impl ::core::convert::From<ISCPSecureQuery3> for ::windows::core::IUnknown {
    fn from(value: ISCPSecureQuery3) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ISCPSecureQuery3> for ::windows::core::IUnknown {
    fn from(value: &ISCPSecureQuery3) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ISCPSecureQuery3 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a ISCPSecureQuery3 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ISCPSecureQuery3> for ISCPSecureQuery {
    fn from(value: ISCPSecureQuery3) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ISCPSecureQuery3> for ISCPSecureQuery {
    fn from(value: &ISCPSecureQuery3) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ISCPSecureQuery> for ISCPSecureQuery3 {
    fn into_param(self) -> ::windows::core::Param<'a, ISCPSecureQuery> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ISCPSecureQuery> for &'a ISCPSecureQuery3 {
    fn into_param(self) -> ::windows::core::Param<'a, ISCPSecureQuery> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ISCPSecureQuery3> for ISCPSecureQuery2 {
    fn from(value: ISCPSecureQuery3) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ISCPSecureQuery3> for ISCPSecureQuery2 {
    fn from(value: &ISCPSecureQuery3) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ISCPSecureQuery2> for ISCPSecureQuery3 {
    fn into_param(self) -> ::windows::core::Param<'a, ISCPSecureQuery2> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ISCPSecureQuery2> for &'a ISCPSecureQuery3 {
    fn into_param(self) -> ::windows::core::Param<'a, ISCPSecureQuery2> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for ISCPSecureQuery3 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
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
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb7edd1a2_4dab_484b_b3c5_ad39b8b4c0b1);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISCPSecureQuery3_Vtbl {
    pub base: ISCPSecureQuery2_Vtbl,
    pub GetRightsOnClearChannel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdata: *const u8, dwsize: u32, pbspsessionkey: *const u8, dwsessionkeylen: u32, pstgglobals: ::windows::core::RawPtr, pprogresscallback: ::windows::core::RawPtr, pprights: *mut *mut WMDMRIGHTS, pnrightscount: *mut u32) -> ::windows::core::HRESULT,
    pub MakeDecisionOnClearChannel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, fuflags: u32, pdata: *const u8, dwsize: u32, dwappsec: u32, pbspsessionkey: *const u8, dwsessionkeylen: u32, pstorageglobals: ::windows::core::RawPtr, pprogresscallback: ::windows::core::RawPtr, pappcertapp: *const u8, dwappcertapplen: u32, pappcertsp: *const u8, dwappcertsplen: u32, pszrevocationurl: *mut ::windows::core::PWSTR, pdwrevocationurllen: *mut u32, pdwrevocationbitflag: *mut u32, pqwfilesize: *mut u64, punknown: *mut ::core::ffi::c_void, ppexchange: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
#[repr(transparent)]
pub struct ISCPSession(::windows::core::IUnknown);
impl ISCPSession {
    #[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
    pub unsafe fn BeginSession<'a, Param0: ::windows::core::IntoParam<'a, IMDSPDevice>>(&self, pidevice: Param0, pctx: &[u8]) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).BeginSession)(::core::mem::transmute_copy(self), pidevice.into_param().abi(), ::core::mem::transmute(::windows::core::as_ptr_or_null(pctx)), pctx.len() as _).ok()
    }
    #[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
    pub unsafe fn EndSession(&self, pctx: &[u8]) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).EndSession)(::core::mem::transmute_copy(self), ::core::mem::transmute(::windows::core::as_ptr_or_null(pctx)), pctx.len() as _).ok()
    }
    #[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
    pub unsafe fn GetSecureQuery(&self) -> ::windows::core::Result<ISCPSecureQuery> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetSecureQuery)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<ISCPSecureQuery>(result__)
    }
}
impl ::core::convert::From<ISCPSession> for ::windows::core::IUnknown {
    fn from(value: ISCPSession) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ISCPSession> for ::windows::core::IUnknown {
    fn from(value: &ISCPSession) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ISCPSession {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a ISCPSession {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for ISCPSession {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
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
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x88a3e6ed_eee4_4619_bbb3_fd4fb62715d1);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISCPSession_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    pub BeginSession: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pidevice: ::windows::core::RawPtr, pctx: *const u8, dwsizectx: u32) -> ::windows::core::HRESULT,
    pub EndSession: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pctx: *const u8, dwsizectx: u32) -> ::windows::core::HRESULT,
    pub GetSecureQuery: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppsecurequery: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
#[repr(transparent)]
pub struct IWMDMDevice(::windows::core::IUnknown);
impl IWMDMDevice {
    #[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
    pub unsafe fn GetName(&self, pwszname: &mut [u16]) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetName)(::core::mem::transmute_copy(self), ::core::mem::transmute(::windows::core::as_mut_ptr_or_null(pwszname)), pwszname.len() as _).ok()
    }
    #[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
    pub unsafe fn GetManufacturer(&self, pwszname: &mut [u16]) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetManufacturer)(::core::mem::transmute_copy(self), ::core::mem::transmute(::windows::core::as_mut_ptr_or_null(pwszname)), pwszname.len() as _).ok()
    }
    #[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
    pub unsafe fn GetVersion(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetVersion)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
    pub unsafe fn GetType(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetType)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
    pub unsafe fn GetSerialNumber(&self, pserialnumber: *mut WMDMID, abmac: *mut u8) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetSerialNumber)(::core::mem::transmute_copy(self), ::core::mem::transmute(pserialnumber), ::core::mem::transmute(abmac)).ok()
    }
    #[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
    pub unsafe fn GetPowerSource(&self, pdwpowersource: *mut u32, pdwpercentremaining: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetPowerSource)(::core::mem::transmute_copy(self), ::core::mem::transmute(pdwpowersource), ::core::mem::transmute(pdwpercentremaining)).ok()
    }
    #[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
    pub unsafe fn GetStatus(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetStatus)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
    pub unsafe fn GetDeviceIcon(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetDeviceIcon)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
    pub unsafe fn EnumStorage(&self) -> ::windows::core::Result<IWMDMEnumStorage> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).EnumStorage)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IWMDMEnumStorage>(result__)
    }
    #[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
    pub unsafe fn GetFormatSupport(&self, ppformatex: *mut *mut _WAVEFORMATEX, pnformatcount: *mut u32, pppwszmimetype: *mut *mut ::windows::core::PWSTR, pnmimetypecount: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetFormatSupport)(::core::mem::transmute_copy(self), ::core::mem::transmute(ppformatex), ::core::mem::transmute(pnformatcount), ::core::mem::transmute(pppwszmimetype), ::core::mem::transmute(pnmimetypecount)).ok()
    }
    #[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
    pub unsafe fn SendOpaqueCommand(&self, pcommand: *mut OPAQUECOMMAND) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SendOpaqueCommand)(::core::mem::transmute_copy(self), ::core::mem::transmute(pcommand)).ok()
    }
}
impl ::core::convert::From<IWMDMDevice> for ::windows::core::IUnknown {
    fn from(value: IWMDMDevice) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWMDMDevice> for ::windows::core::IUnknown {
    fn from(value: &IWMDMDevice) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IWMDMDevice {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IWMDMDevice {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IWMDMDevice {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
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
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1dcb3a02_33ed_11d3_8470_00c04f79dbc0);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMDMDevice_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    pub GetName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwszname: ::windows::core::PWSTR, nmaxchars: u32) -> ::windows::core::HRESULT,
    pub GetManufacturer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwszname: ::windows::core::PWSTR, nmaxchars: u32) -> ::windows::core::HRESULT,
    pub GetVersion: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdwversion: *mut u32) -> ::windows::core::HRESULT,
    pub GetType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdwtype: *mut u32) -> ::windows::core::HRESULT,
    pub GetSerialNumber: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pserialnumber: *mut WMDMID, abmac: *mut u8) -> ::windows::core::HRESULT,
    pub GetPowerSource: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdwpowersource: *mut u32, pdwpercentremaining: *mut u32) -> ::windows::core::HRESULT,
    pub GetStatus: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdwstatus: *mut u32) -> ::windows::core::HRESULT,
    pub GetDeviceIcon: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hicon: *mut u32) -> ::windows::core::HRESULT,
    pub EnumStorage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppenumstorage: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub GetFormatSupport: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppformatex: *mut *mut _WAVEFORMATEX, pnformatcount: *mut u32, pppwszmimetype: *mut *mut ::windows::core::PWSTR, pnmimetypecount: *mut u32) -> ::windows::core::HRESULT,
    pub SendOpaqueCommand: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcommand: *mut OPAQUECOMMAND) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
#[repr(transparent)]
pub struct IWMDMDevice2(::windows::core::IUnknown);
impl IWMDMDevice2 {
    #[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
    pub unsafe fn GetName(&self, pwszname: &mut [u16]) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.GetName)(::core::mem::transmute_copy(self), ::core::mem::transmute(::windows::core::as_mut_ptr_or_null(pwszname)), pwszname.len() as _).ok()
    }
    #[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
    pub unsafe fn GetManufacturer(&self, pwszname: &mut [u16]) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.GetManufacturer)(::core::mem::transmute_copy(self), ::core::mem::transmute(::windows::core::as_mut_ptr_or_null(pwszname)), pwszname.len() as _).ok()
    }
    #[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
    pub unsafe fn GetVersion(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.GetVersion)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
    pub unsafe fn GetType(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.GetType)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
    pub unsafe fn GetSerialNumber(&self, pserialnumber: *mut WMDMID, abmac: *mut u8) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.GetSerialNumber)(::core::mem::transmute_copy(self), ::core::mem::transmute(pserialnumber), ::core::mem::transmute(abmac)).ok()
    }
    #[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
    pub unsafe fn GetPowerSource(&self, pdwpowersource: *mut u32, pdwpercentremaining: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.GetPowerSource)(::core::mem::transmute_copy(self), ::core::mem::transmute(pdwpowersource), ::core::mem::transmute(pdwpercentremaining)).ok()
    }
    #[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
    pub unsafe fn GetStatus(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.GetStatus)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
    pub unsafe fn GetDeviceIcon(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.GetDeviceIcon)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
    pub unsafe fn EnumStorage(&self) -> ::windows::core::Result<IWMDMEnumStorage> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.EnumStorage)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IWMDMEnumStorage>(result__)
    }
    #[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
    pub unsafe fn GetFormatSupport(&self, ppformatex: *mut *mut _WAVEFORMATEX, pnformatcount: *mut u32, pppwszmimetype: *mut *mut ::windows::core::PWSTR, pnmimetypecount: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.GetFormatSupport)(::core::mem::transmute_copy(self), ::core::mem::transmute(ppformatex), ::core::mem::transmute(pnformatcount), ::core::mem::transmute(pppwszmimetype), ::core::mem::transmute(pnmimetypecount)).ok()
    }
    #[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
    pub unsafe fn SendOpaqueCommand(&self, pcommand: *mut OPAQUECOMMAND) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.SendOpaqueCommand)(::core::mem::transmute_copy(self), ::core::mem::transmute(pcommand)).ok()
    }
    #[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
    pub unsafe fn GetStorage<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(&self, pszstoragename: Param0) -> ::windows::core::Result<IWMDMStorage> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetStorage)(::core::mem::transmute_copy(self), pszstoragename.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<IWMDMStorage>(result__)
    }
    #[doc = "*Required features: `\"Win32_Media_DeviceManager\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetFormatSupport2(&self, dwflags: u32, ppaudioformatex: *mut *mut _WAVEFORMATEX, pnaudioformatcount: *mut u32, ppvideoformatex: *mut *mut _VIDEOINFOHEADER, pnvideoformatcount: *mut u32, ppfiletype: *mut *mut WMFILECAPABILITIES, pnfiletypecount: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetFormatSupport2)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwflags), ::core::mem::transmute(ppaudioformatex), ::core::mem::transmute(pnaudioformatcount), ::core::mem::transmute(ppvideoformatex), ::core::mem::transmute(pnvideoformatcount), ::core::mem::transmute(ppfiletype), ::core::mem::transmute(pnfiletypecount)).ok()
    }
    #[doc = "*Required features: `\"Win32_Media_DeviceManager\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(feature = "Win32_System_Ole")]
    pub unsafe fn GetSpecifyPropertyPages(&self, ppspecifyproppages: *mut ::core::option::Option<super::super::System::Ole::ISpecifyPropertyPages>, pppunknowns: *mut *mut ::core::option::Option<::windows::core::IUnknown>, pcunks: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetSpecifyPropertyPages)(::core::mem::transmute_copy(self), ::core::mem::transmute(ppspecifyproppages), ::core::mem::transmute(pppunknowns), ::core::mem::transmute(pcunks)).ok()
    }
    #[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
    pub unsafe fn GetCanonicalName(&self, pwszpnpname: &mut [u16]) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetCanonicalName)(::core::mem::transmute_copy(self), ::core::mem::transmute(::windows::core::as_mut_ptr_or_null(pwszpnpname)), pwszpnpname.len() as _).ok()
    }
}
impl ::core::convert::From<IWMDMDevice2> for ::windows::core::IUnknown {
    fn from(value: IWMDMDevice2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWMDMDevice2> for ::windows::core::IUnknown {
    fn from(value: &IWMDMDevice2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IWMDMDevice2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IWMDMDevice2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IWMDMDevice2> for IWMDMDevice {
    fn from(value: IWMDMDevice2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWMDMDevice2> for IWMDMDevice {
    fn from(value: &IWMDMDevice2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IWMDMDevice> for IWMDMDevice2 {
    fn into_param(self) -> ::windows::core::Param<'a, IWMDMDevice> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IWMDMDevice> for &'a IWMDMDevice2 {
    fn into_param(self) -> ::windows::core::Param<'a, IWMDMDevice> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IWMDMDevice2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
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
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe34f3d37_9d67_4fc1_9252_62d28b2f8b55);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMDMDevice2_Vtbl {
    pub base: IWMDMDevice_Vtbl,
    pub GetStorage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszstoragename: ::windows::core::PCWSTR, ppstorage: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub GetFormatSupport2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwflags: u32, ppaudioformatex: *mut *mut _WAVEFORMATEX, pnaudioformatcount: *mut u32, ppvideoformatex: *mut *mut _VIDEOINFOHEADER, pnvideoformatcount: *mut u32, ppfiletype: *mut *mut WMFILECAPABILITIES, pnfiletypecount: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetFormatSupport2: usize,
    #[cfg(feature = "Win32_System_Ole")]
    pub GetSpecifyPropertyPages: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppspecifyproppages: *mut ::windows::core::RawPtr, pppunknowns: *mut *mut *mut ::core::ffi::c_void, pcunks: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Ole"))]
    GetSpecifyPropertyPages: usize,
    pub GetCanonicalName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwszpnpname: ::windows::core::PWSTR, nmaxchars: u32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
#[repr(transparent)]
pub struct IWMDMDevice3(::windows::core::IUnknown);
impl IWMDMDevice3 {
    #[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
    pub unsafe fn GetName(&self, pwszname: &mut [u16]) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.base.GetName)(::core::mem::transmute_copy(self), ::core::mem::transmute(::windows::core::as_mut_ptr_or_null(pwszname)), pwszname.len() as _).ok()
    }
    #[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
    pub unsafe fn GetManufacturer(&self, pwszname: &mut [u16]) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.base.GetManufacturer)(::core::mem::transmute_copy(self), ::core::mem::transmute(::windows::core::as_mut_ptr_or_null(pwszname)), pwszname.len() as _).ok()
    }
    #[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
    pub unsafe fn GetVersion(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.base.GetVersion)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
    pub unsafe fn GetType(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.base.GetType)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
    pub unsafe fn GetSerialNumber(&self, pserialnumber: *mut WMDMID, abmac: *mut u8) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.base.GetSerialNumber)(::core::mem::transmute_copy(self), ::core::mem::transmute(pserialnumber), ::core::mem::transmute(abmac)).ok()
    }
    #[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
    pub unsafe fn GetPowerSource(&self, pdwpowersource: *mut u32, pdwpercentremaining: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.base.GetPowerSource)(::core::mem::transmute_copy(self), ::core::mem::transmute(pdwpowersource), ::core::mem::transmute(pdwpercentremaining)).ok()
    }
    #[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
    pub unsafe fn GetStatus(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.base.GetStatus)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
    pub unsafe fn GetDeviceIcon(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.base.GetDeviceIcon)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
    pub unsafe fn EnumStorage(&self) -> ::windows::core::Result<IWMDMEnumStorage> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.base.EnumStorage)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IWMDMEnumStorage>(result__)
    }
    #[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
    pub unsafe fn GetFormatSupport(&self, ppformatex: *mut *mut _WAVEFORMATEX, pnformatcount: *mut u32, pppwszmimetype: *mut *mut ::windows::core::PWSTR, pnmimetypecount: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.base.GetFormatSupport)(::core::mem::transmute_copy(self), ::core::mem::transmute(ppformatex), ::core::mem::transmute(pnformatcount), ::core::mem::transmute(pppwszmimetype), ::core::mem::transmute(pnmimetypecount)).ok()
    }
    #[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
    pub unsafe fn SendOpaqueCommand(&self, pcommand: *mut OPAQUECOMMAND) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.base.SendOpaqueCommand)(::core::mem::transmute_copy(self), ::core::mem::transmute(pcommand)).ok()
    }
    #[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
    pub unsafe fn GetStorage<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(&self, pszstoragename: Param0) -> ::windows::core::Result<IWMDMStorage> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.GetStorage)(::core::mem::transmute_copy(self), pszstoragename.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<IWMDMStorage>(result__)
    }
    #[doc = "*Required features: `\"Win32_Media_DeviceManager\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetFormatSupport2(&self, dwflags: u32, ppaudioformatex: *mut *mut _WAVEFORMATEX, pnaudioformatcount: *mut u32, ppvideoformatex: *mut *mut _VIDEOINFOHEADER, pnvideoformatcount: *mut u32, ppfiletype: *mut *mut WMFILECAPABILITIES, pnfiletypecount: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.GetFormatSupport2)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwflags), ::core::mem::transmute(ppaudioformatex), ::core::mem::transmute(pnaudioformatcount), ::core::mem::transmute(ppvideoformatex), ::core::mem::transmute(pnvideoformatcount), ::core::mem::transmute(ppfiletype), ::core::mem::transmute(pnfiletypecount)).ok()
    }
    #[doc = "*Required features: `\"Win32_Media_DeviceManager\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(feature = "Win32_System_Ole")]
    pub unsafe fn GetSpecifyPropertyPages(&self, ppspecifyproppages: *mut ::core::option::Option<super::super::System::Ole::ISpecifyPropertyPages>, pppunknowns: *mut *mut ::core::option::Option<::windows::core::IUnknown>, pcunks: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.GetSpecifyPropertyPages)(::core::mem::transmute_copy(self), ::core::mem::transmute(ppspecifyproppages), ::core::mem::transmute(pppunknowns), ::core::mem::transmute(pcunks)).ok()
    }
    #[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
    pub unsafe fn GetCanonicalName(&self, pwszpnpname: &mut [u16]) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.GetCanonicalName)(::core::mem::transmute_copy(self), ::core::mem::transmute(::windows::core::as_mut_ptr_or_null(pwszpnpname)), pwszpnpname.len() as _).ok()
    }
    #[doc = "*Required features: `\"Win32_Media_DeviceManager\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com_StructuredStorage\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
    pub unsafe fn GetProperty<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(&self, pwszpropname: Param0) -> ::windows::core::Result<super::super::System::Com::StructuredStorage::PROPVARIANT> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::System::Com::StructuredStorage::PROPVARIANT> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetProperty)(::core::mem::transmute_copy(self), pwszpropname.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<super::super::System::Com::StructuredStorage::PROPVARIANT>(result__)
    }
    #[doc = "*Required features: `\"Win32_Media_DeviceManager\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com_StructuredStorage\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
    pub unsafe fn SetProperty<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(&self, pwszpropname: Param0, pvalue: *const super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetProperty)(::core::mem::transmute_copy(self), pwszpropname.into_param().abi(), ::core::mem::transmute(pvalue)).ok()
    }
    #[doc = "*Required features: `\"Win32_Media_DeviceManager\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com_StructuredStorage\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
    pub unsafe fn GetFormatCapability(&self, format: WMDM_FORMATCODE) -> ::windows::core::Result<WMDM_FORMAT_CAPABILITY> {
        let mut result__: WMDM_FORMAT_CAPABILITY = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetFormatCapability)(::core::mem::transmute_copy(self), ::core::mem::transmute(format), ::core::mem::transmute(&mut result__)).from_abi::<WMDM_FORMAT_CAPABILITY>(result__)
    }
    #[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
    pub unsafe fn DeviceIoControl(&self, dwiocontrolcode: u32, lpinbuffer: &[u8], lpoutbuffer: *mut u8, pnoutbuffersize: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).DeviceIoControl)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwiocontrolcode), ::core::mem::transmute(::windows::core::as_ptr_or_null(lpinbuffer)), lpinbuffer.len() as _, ::core::mem::transmute(lpoutbuffer), ::core::mem::transmute(pnoutbuffersize)).ok()
    }
    #[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
    pub unsafe fn FindStorage<'a, Param1: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(&self, findscope: WMDM_FIND_SCOPE, pwszuniqueid: Param1) -> ::windows::core::Result<IWMDMStorage> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).FindStorage)(::core::mem::transmute_copy(self), ::core::mem::transmute(findscope), pwszuniqueid.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<IWMDMStorage>(result__)
    }
}
impl ::core::convert::From<IWMDMDevice3> for ::windows::core::IUnknown {
    fn from(value: IWMDMDevice3) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWMDMDevice3> for ::windows::core::IUnknown {
    fn from(value: &IWMDMDevice3) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IWMDMDevice3 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IWMDMDevice3 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IWMDMDevice3> for IWMDMDevice {
    fn from(value: IWMDMDevice3) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWMDMDevice3> for IWMDMDevice {
    fn from(value: &IWMDMDevice3) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IWMDMDevice> for IWMDMDevice3 {
    fn into_param(self) -> ::windows::core::Param<'a, IWMDMDevice> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IWMDMDevice> for &'a IWMDMDevice3 {
    fn into_param(self) -> ::windows::core::Param<'a, IWMDMDevice> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IWMDMDevice3> for IWMDMDevice2 {
    fn from(value: IWMDMDevice3) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWMDMDevice3> for IWMDMDevice2 {
    fn from(value: &IWMDMDevice3) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IWMDMDevice2> for IWMDMDevice3 {
    fn into_param(self) -> ::windows::core::Param<'a, IWMDMDevice2> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IWMDMDevice2> for &'a IWMDMDevice3 {
    fn into_param(self) -> ::windows::core::Param<'a, IWMDMDevice2> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IWMDMDevice3 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
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
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6c03e4fe_05db_4dda_9e3c_06233a6d5d65);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMDMDevice3_Vtbl {
    pub base: IWMDMDevice2_Vtbl,
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
    pub FindStorage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, findscope: WMDM_FIND_SCOPE, pwszuniqueid: ::windows::core::PCWSTR, ppstorage: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
#[repr(transparent)]
pub struct IWMDMDeviceControl(::windows::core::IUnknown);
impl IWMDMDeviceControl {
    #[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
    pub unsafe fn GetStatus(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetStatus)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
    pub unsafe fn GetCapabilities(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetCapabilities)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
    pub unsafe fn Play(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Play)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
    pub unsafe fn Record(&self, pformat: *const _WAVEFORMATEX) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Record)(::core::mem::transmute_copy(self), ::core::mem::transmute(pformat)).ok()
    }
    #[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
    pub unsafe fn Pause(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Pause)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
    pub unsafe fn Resume(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Resume)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
    pub unsafe fn Stop(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Stop)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
    pub unsafe fn Seek(&self, fumode: u32, noffset: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Seek)(::core::mem::transmute_copy(self), ::core::mem::transmute(fumode), ::core::mem::transmute(noffset)).ok()
    }
}
impl ::core::convert::From<IWMDMDeviceControl> for ::windows::core::IUnknown {
    fn from(value: IWMDMDeviceControl) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWMDMDeviceControl> for ::windows::core::IUnknown {
    fn from(value: &IWMDMDeviceControl) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IWMDMDeviceControl {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IWMDMDeviceControl {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IWMDMDeviceControl {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
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
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1dcb3a04_33ed_11d3_8470_00c04f79dbc0);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMDMDeviceControl_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    pub GetStatus: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdwstatus: *mut u32) -> ::windows::core::HRESULT,
    pub GetCapabilities: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdwcapabilitiesmask: *mut u32) -> ::windows::core::HRESULT,
    pub Play: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Record: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pformat: *const _WAVEFORMATEX) -> ::windows::core::HRESULT,
    pub Pause: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Resume: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Stop: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Seek: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, fumode: u32, noffset: i32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
#[repr(transparent)]
pub struct IWMDMDeviceSession(::windows::core::IUnknown);
impl IWMDMDeviceSession {
    #[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
    pub unsafe fn BeginSession(&self, r#type: WMDM_SESSION_TYPE, pctx: &[u8]) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).BeginSession)(::core::mem::transmute_copy(self), ::core::mem::transmute(r#type), ::core::mem::transmute(::windows::core::as_ptr_or_null(pctx)), pctx.len() as _).ok()
    }
    #[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
    pub unsafe fn EndSession(&self, r#type: WMDM_SESSION_TYPE, pctx: &[u8]) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).EndSession)(::core::mem::transmute_copy(self), ::core::mem::transmute(r#type), ::core::mem::transmute(::windows::core::as_ptr_or_null(pctx)), pctx.len() as _).ok()
    }
}
impl ::core::convert::From<IWMDMDeviceSession> for ::windows::core::IUnknown {
    fn from(value: IWMDMDeviceSession) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWMDMDeviceSession> for ::windows::core::IUnknown {
    fn from(value: &IWMDMDeviceSession) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IWMDMDeviceSession {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IWMDMDeviceSession {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IWMDMDeviceSession {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
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
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x82af0a65_9d96_412c_83e5_3c43e4b06cc7);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMDMDeviceSession_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    pub BeginSession: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, r#type: WMDM_SESSION_TYPE, pctx: *const u8, dwsizectx: u32) -> ::windows::core::HRESULT,
    pub EndSession: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, r#type: WMDM_SESSION_TYPE, pctx: *const u8, dwsizectx: u32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
#[repr(transparent)]
pub struct IWMDMEnumDevice(::windows::core::IUnknown);
impl IWMDMEnumDevice {
    #[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
    pub unsafe fn Next(&self, celt: u32, ppdevice: *mut ::core::option::Option<IWMDMDevice>, pceltfetched: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Next)(::core::mem::transmute_copy(self), ::core::mem::transmute(celt), ::core::mem::transmute(ppdevice), ::core::mem::transmute(pceltfetched)).ok()
    }
    #[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
    pub unsafe fn Skip(&self, celt: u32) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).Skip)(::core::mem::transmute_copy(self), ::core::mem::transmute(celt), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
    pub unsafe fn Reset(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Reset)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
    pub unsafe fn Clone(&self) -> ::windows::core::Result<IWMDMEnumDevice> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).Clone)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IWMDMEnumDevice>(result__)
    }
}
impl ::core::convert::From<IWMDMEnumDevice> for ::windows::core::IUnknown {
    fn from(value: IWMDMEnumDevice) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWMDMEnumDevice> for ::windows::core::IUnknown {
    fn from(value: &IWMDMEnumDevice) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IWMDMEnumDevice {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IWMDMEnumDevice {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IWMDMEnumDevice {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
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
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1dcb3a01_33ed_11d3_8470_00c04f79dbc0);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMDMEnumDevice_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    pub Next: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, celt: u32, ppdevice: *mut ::windows::core::RawPtr, pceltfetched: *mut u32) -> ::windows::core::HRESULT,
    pub Skip: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, celt: u32, pceltfetched: *mut u32) -> ::windows::core::HRESULT,
    pub Reset: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Clone: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppenumdevice: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
#[repr(transparent)]
pub struct IWMDMEnumStorage(::windows::core::IUnknown);
impl IWMDMEnumStorage {
    #[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
    pub unsafe fn Next(&self, celt: u32, ppstorage: *mut ::core::option::Option<IWMDMStorage>, pceltfetched: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Next)(::core::mem::transmute_copy(self), ::core::mem::transmute(celt), ::core::mem::transmute(ppstorage), ::core::mem::transmute(pceltfetched)).ok()
    }
    #[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
    pub unsafe fn Skip(&self, celt: u32) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).Skip)(::core::mem::transmute_copy(self), ::core::mem::transmute(celt), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
    pub unsafe fn Reset(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Reset)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
    pub unsafe fn Clone(&self) -> ::windows::core::Result<IWMDMEnumStorage> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).Clone)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IWMDMEnumStorage>(result__)
    }
}
impl ::core::convert::From<IWMDMEnumStorage> for ::windows::core::IUnknown {
    fn from(value: IWMDMEnumStorage) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWMDMEnumStorage> for ::windows::core::IUnknown {
    fn from(value: &IWMDMEnumStorage) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IWMDMEnumStorage {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IWMDMEnumStorage {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IWMDMEnumStorage {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
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
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1dcb3a05_33ed_11d3_8470_00c04f79dbc0);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMDMEnumStorage_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    pub Next: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, celt: u32, ppstorage: *mut ::windows::core::RawPtr, pceltfetched: *mut u32) -> ::windows::core::HRESULT,
    pub Skip: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, celt: u32, pceltfetched: *mut u32) -> ::windows::core::HRESULT,
    pub Reset: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Clone: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppenumstorage: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
#[repr(transparent)]
pub struct IWMDMLogger(::windows::core::IUnknown);
impl IWMDMLogger {
    #[doc = "*Required features: `\"Win32_Media_DeviceManager\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsEnabled(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__: super::super::Foundation::BOOL = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).IsEnabled)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[doc = "*Required features: `\"Win32_Media_DeviceManager\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Enable<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(&self, fenable: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Enable)(::core::mem::transmute_copy(self), fenable.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
    pub unsafe fn GetLogFileName(&self, pszfilename: ::windows::core::PSTR, nmaxchars: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetLogFileName)(::core::mem::transmute_copy(self), ::core::mem::transmute(pszfilename), ::core::mem::transmute(nmaxchars)).ok()
    }
    #[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
    pub unsafe fn SetLogFileName<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCSTR>>(&self, pszfilename: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetLogFileName)(::core::mem::transmute_copy(self), pszfilename.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
    pub unsafe fn LogString<'a, Param1: ::windows::core::IntoParam<'a, ::windows::core::PCSTR>, Param2: ::windows::core::IntoParam<'a, ::windows::core::PCSTR>>(&self, dwflags: u32, pszsrcname: Param1, pszlog: Param2) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).LogString)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwflags), pszsrcname.into_param().abi(), pszlog.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
    pub unsafe fn LogDword<'a, Param1: ::windows::core::IntoParam<'a, ::windows::core::PCSTR>, Param2: ::windows::core::IntoParam<'a, ::windows::core::PCSTR>>(&self, dwflags: u32, pszsrcname: Param1, pszlogformat: Param2, dwlog: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).LogDword)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwflags), pszsrcname.into_param().abi(), pszlogformat.into_param().abi(), ::core::mem::transmute(dwlog)).ok()
    }
    #[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
    pub unsafe fn Reset(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Reset)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
    pub unsafe fn GetSizeParams(&self, pdwmaxsize: *mut u32, pdwshrinktosize: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetSizeParams)(::core::mem::transmute_copy(self), ::core::mem::transmute(pdwmaxsize), ::core::mem::transmute(pdwshrinktosize)).ok()
    }
    #[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
    pub unsafe fn SetSizeParams(&self, dwmaxsize: u32, dwshrinktosize: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetSizeParams)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwmaxsize), ::core::mem::transmute(dwshrinktosize)).ok()
    }
}
impl ::core::convert::From<IWMDMLogger> for ::windows::core::IUnknown {
    fn from(value: IWMDMLogger) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWMDMLogger> for ::windows::core::IUnknown {
    fn from(value: &IWMDMLogger) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IWMDMLogger {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IWMDMLogger {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IWMDMLogger {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
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
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x110a3200_5a79_11d3_8d78_444553540000);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMDMLogger_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
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
    #[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
    pub unsafe fn AddItem<'a, Param1: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(&self, r#type: WMDM_TAG_DATATYPE, pwsztagname: Param1, pvalue: &[u8]) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).AddItem)(::core::mem::transmute_copy(self), ::core::mem::transmute(r#type), pwsztagname.into_param().abi(), ::core::mem::transmute(::windows::core::as_ptr_or_null(pvalue)), pvalue.len() as _).ok()
    }
    #[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
    pub unsafe fn QueryByName<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(&self, pwsztagname: Param0, ptype: *mut WMDM_TAG_DATATYPE, pvalue: *mut *mut u8, pcblength: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).QueryByName)(::core::mem::transmute_copy(self), pwsztagname.into_param().abi(), ::core::mem::transmute(ptype), ::core::mem::transmute(pvalue), ::core::mem::transmute(pcblength)).ok()
    }
    #[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
    pub unsafe fn QueryByIndex(&self, iindex: u32, ppwszname: *mut *mut u16, ptype: *mut WMDM_TAG_DATATYPE, ppvalue: *mut *mut u8, pcblength: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).QueryByIndex)(::core::mem::transmute_copy(self), ::core::mem::transmute(iindex), ::core::mem::transmute(ppwszname), ::core::mem::transmute(ptype), ::core::mem::transmute(ppvalue), ::core::mem::transmute(pcblength)).ok()
    }
    #[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
    pub unsafe fn GetItemCount(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetItemCount)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
}
impl ::core::convert::From<IWMDMMetaData> for ::windows::core::IUnknown {
    fn from(value: IWMDMMetaData) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWMDMMetaData> for ::windows::core::IUnknown {
    fn from(value: &IWMDMMetaData) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IWMDMMetaData {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IWMDMMetaData {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IWMDMMetaData {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
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
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xec3b0663_0951_460a_9a80_0dceed3c043c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMDMMetaData_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    pub AddItem: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, r#type: WMDM_TAG_DATATYPE, pwsztagname: ::windows::core::PCWSTR, pvalue: *const u8, ilength: u32) -> ::windows::core::HRESULT,
    pub QueryByName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwsztagname: ::windows::core::PCWSTR, ptype: *mut WMDM_TAG_DATATYPE, pvalue: *mut *mut u8, pcblength: *mut u32) -> ::windows::core::HRESULT,
    pub QueryByIndex: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iindex: u32, ppwszname: *mut *mut u16, ptype: *mut WMDM_TAG_DATATYPE, ppvalue: *mut *mut u8, pcblength: *mut u32) -> ::windows::core::HRESULT,
    pub GetItemCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, icount: *mut u32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
#[repr(transparent)]
pub struct IWMDMNotification(::windows::core::IUnknown);
impl IWMDMNotification {
    #[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
    pub unsafe fn WMDMMessage<'a, Param1: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(&self, dwmessagetype: u32, pwszcanonicalname: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).WMDMMessage)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwmessagetype), pwszcanonicalname.into_param().abi()).ok()
    }
}
impl ::core::convert::From<IWMDMNotification> for ::windows::core::IUnknown {
    fn from(value: IWMDMNotification) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWMDMNotification> for ::windows::core::IUnknown {
    fn from(value: &IWMDMNotification) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IWMDMNotification {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IWMDMNotification {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IWMDMNotification {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
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
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3f5e95c0_0f43_4ed4_93d2_c89a45d59b81);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMDMNotification_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    pub WMDMMessage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwmessagetype: u32, pwszcanonicalname: ::windows::core::PCWSTR) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
#[repr(transparent)]
pub struct IWMDMObjectInfo(::windows::core::IUnknown);
impl IWMDMObjectInfo {
    #[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
    pub unsafe fn GetPlayLength(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetPlayLength)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
    pub unsafe fn SetPlayLength(&self, dwlength: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetPlayLength)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwlength)).ok()
    }
    #[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
    pub unsafe fn GetPlayOffset(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetPlayOffset)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
    pub unsafe fn SetPlayOffset(&self, dwoffset: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetPlayOffset)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwoffset)).ok()
    }
    #[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
    pub unsafe fn GetTotalLength(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetTotalLength)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
    pub unsafe fn GetLastPlayPosition(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetLastPlayPosition)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
    pub unsafe fn GetLongestPlayPosition(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetLongestPlayPosition)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
}
impl ::core::convert::From<IWMDMObjectInfo> for ::windows::core::IUnknown {
    fn from(value: IWMDMObjectInfo) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWMDMObjectInfo> for ::windows::core::IUnknown {
    fn from(value: &IWMDMObjectInfo) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IWMDMObjectInfo {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IWMDMObjectInfo {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IWMDMObjectInfo {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
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
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1dcb3a09_33ed_11d3_8470_00c04f79dbc0);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMDMObjectInfo_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
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
    #[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
    pub unsafe fn BeginRead(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).BeginRead)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
    pub unsafe fn BeginWrite(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).BeginWrite)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
    pub unsafe fn GetObjectName(&self, pwszname: &mut [u16]) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetObjectName)(::core::mem::transmute_copy(self), ::core::mem::transmute(::windows::core::as_mut_ptr_or_null(pwszname)), pwszname.len() as _).ok()
    }
    #[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
    pub unsafe fn SetObjectName(&self, pwszname: &[u16]) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetObjectName)(::core::mem::transmute_copy(self), ::core::mem::transmute(::windows::core::as_ptr_or_null(pwszname)), pwszname.len() as _).ok()
    }
    #[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
    pub unsafe fn GetObjectAttributes(&self, pdwattributes: *mut u32, pformat: *mut _WAVEFORMATEX) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetObjectAttributes)(::core::mem::transmute_copy(self), ::core::mem::transmute(pdwattributes), ::core::mem::transmute(pformat)).ok()
    }
    #[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
    pub unsafe fn SetObjectAttributes(&self, dwattributes: u32, pformat: *const _WAVEFORMATEX) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetObjectAttributes)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwattributes), ::core::mem::transmute(pformat)).ok()
    }
    #[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
    pub unsafe fn GetObjectTotalSize(&self, pdwsize: *mut u32, pdwsizehigh: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetObjectTotalSize)(::core::mem::transmute_copy(self), ::core::mem::transmute(pdwsize), ::core::mem::transmute(pdwsizehigh)).ok()
    }
    #[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
    pub unsafe fn SetObjectTotalSize(&self, dwsize: u32, dwsizehigh: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetObjectTotalSize)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwsize), ::core::mem::transmute(dwsizehigh)).ok()
    }
    #[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
    pub unsafe fn TransferObjectData(&self, pdata: *mut u8, pdwsize: *mut u32, abmac: *mut u8) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).TransferObjectData)(::core::mem::transmute_copy(self), ::core::mem::transmute(pdata), ::core::mem::transmute(pdwsize), ::core::mem::transmute(abmac)).ok()
    }
    #[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
    pub unsafe fn End<'a, Param1: ::windows::core::IntoParam<'a, ::windows::core::IUnknown>>(&self, phcompletioncode: *const ::windows::core::HRESULT, pnewobject: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).End)(::core::mem::transmute_copy(self), ::core::mem::transmute(phcompletioncode), pnewobject.into_param().abi()).ok()
    }
}
impl ::core::convert::From<IWMDMOperation> for ::windows::core::IUnknown {
    fn from(value: IWMDMOperation) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWMDMOperation> for ::windows::core::IUnknown {
    fn from(value: &IWMDMOperation) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IWMDMOperation {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IWMDMOperation {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IWMDMOperation {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
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
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1dcb3a0b_33ed_11d3_8470_00c04f79dbc0);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMDMOperation_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    pub BeginRead: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub BeginWrite: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetObjectName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwszname: ::windows::core::PWSTR, nmaxchars: u32) -> ::windows::core::HRESULT,
    pub SetObjectName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwszname: ::windows::core::PCWSTR, nmaxchars: u32) -> ::windows::core::HRESULT,
    pub GetObjectAttributes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdwattributes: *mut u32, pformat: *mut _WAVEFORMATEX) -> ::windows::core::HRESULT,
    pub SetObjectAttributes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwattributes: u32, pformat: *const _WAVEFORMATEX) -> ::windows::core::HRESULT,
    pub GetObjectTotalSize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdwsize: *mut u32, pdwsizehigh: *mut u32) -> ::windows::core::HRESULT,
    pub SetObjectTotalSize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwsize: u32, dwsizehigh: u32) -> ::windows::core::HRESULT,
    pub TransferObjectData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdata: *mut u8, pdwsize: *mut u32, abmac: *mut u8) -> ::windows::core::HRESULT,
    pub End: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, phcompletioncode: *const ::windows::core::HRESULT, pnewobject: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
#[repr(transparent)]
pub struct IWMDMOperation2(::windows::core::IUnknown);
impl IWMDMOperation2 {
    #[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
    pub unsafe fn BeginRead(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.BeginRead)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
    pub unsafe fn BeginWrite(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.BeginWrite)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
    pub unsafe fn GetObjectName(&self, pwszname: &mut [u16]) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.GetObjectName)(::core::mem::transmute_copy(self), ::core::mem::transmute(::windows::core::as_mut_ptr_or_null(pwszname)), pwszname.len() as _).ok()
    }
    #[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
    pub unsafe fn SetObjectName(&self, pwszname: &[u16]) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.SetObjectName)(::core::mem::transmute_copy(self), ::core::mem::transmute(::windows::core::as_ptr_or_null(pwszname)), pwszname.len() as _).ok()
    }
    #[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
    pub unsafe fn GetObjectAttributes(&self, pdwattributes: *mut u32, pformat: *mut _WAVEFORMATEX) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.GetObjectAttributes)(::core::mem::transmute_copy(self), ::core::mem::transmute(pdwattributes), ::core::mem::transmute(pformat)).ok()
    }
    #[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
    pub unsafe fn SetObjectAttributes(&self, dwattributes: u32, pformat: *const _WAVEFORMATEX) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.SetObjectAttributes)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwattributes), ::core::mem::transmute(pformat)).ok()
    }
    #[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
    pub unsafe fn GetObjectTotalSize(&self, pdwsize: *mut u32, pdwsizehigh: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.GetObjectTotalSize)(::core::mem::transmute_copy(self), ::core::mem::transmute(pdwsize), ::core::mem::transmute(pdwsizehigh)).ok()
    }
    #[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
    pub unsafe fn SetObjectTotalSize(&self, dwsize: u32, dwsizehigh: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.SetObjectTotalSize)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwsize), ::core::mem::transmute(dwsizehigh)).ok()
    }
    #[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
    pub unsafe fn TransferObjectData(&self, pdata: *mut u8, pdwsize: *mut u32, abmac: *mut u8) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.TransferObjectData)(::core::mem::transmute_copy(self), ::core::mem::transmute(pdata), ::core::mem::transmute(pdwsize), ::core::mem::transmute(abmac)).ok()
    }
    #[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
    pub unsafe fn End<'a, Param1: ::windows::core::IntoParam<'a, ::windows::core::IUnknown>>(&self, phcompletioncode: *const ::windows::core::HRESULT, pnewobject: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.End)(::core::mem::transmute_copy(self), ::core::mem::transmute(phcompletioncode), pnewobject.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Media_DeviceManager\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetObjectAttributes2(&self, dwattributes: u32, dwattributesex: u32, pformat: *const _WAVEFORMATEX, pvideoformat: *const _VIDEOINFOHEADER) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetObjectAttributes2)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwattributes), ::core::mem::transmute(dwattributesex), ::core::mem::transmute(pformat), ::core::mem::transmute(pvideoformat)).ok()
    }
    #[doc = "*Required features: `\"Win32_Media_DeviceManager\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetObjectAttributes2(&self, pdwattributes: *mut u32, pdwattributesex: *mut u32, paudioformat: *mut _WAVEFORMATEX, pvideoformat: *mut _VIDEOINFOHEADER) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetObjectAttributes2)(::core::mem::transmute_copy(self), ::core::mem::transmute(pdwattributes), ::core::mem::transmute(pdwattributesex), ::core::mem::transmute(paudioformat), ::core::mem::transmute(pvideoformat)).ok()
    }
}
impl ::core::convert::From<IWMDMOperation2> for ::windows::core::IUnknown {
    fn from(value: IWMDMOperation2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWMDMOperation2> for ::windows::core::IUnknown {
    fn from(value: &IWMDMOperation2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IWMDMOperation2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IWMDMOperation2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IWMDMOperation2> for IWMDMOperation {
    fn from(value: IWMDMOperation2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWMDMOperation2> for IWMDMOperation {
    fn from(value: &IWMDMOperation2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IWMDMOperation> for IWMDMOperation2 {
    fn into_param(self) -> ::windows::core::Param<'a, IWMDMOperation> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IWMDMOperation> for &'a IWMDMOperation2 {
    fn into_param(self) -> ::windows::core::Param<'a, IWMDMOperation> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IWMDMOperation2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
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
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x33445b48_7df7_425c_ad8f_0fc6d82f9f75);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMDMOperation2_Vtbl {
    pub base: IWMDMOperation_Vtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub SetObjectAttributes2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwattributes: u32, dwattributesex: u32, pformat: *const _WAVEFORMATEX, pvideoformat: *const _VIDEOINFOHEADER) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetObjectAttributes2: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub GetObjectAttributes2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdwattributes: *mut u32, pdwattributesex: *mut u32, paudioformat: *mut _WAVEFORMATEX, pvideoformat: *mut _VIDEOINFOHEADER) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetObjectAttributes2: usize,
}
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
#[repr(transparent)]
pub struct IWMDMOperation3(::windows::core::IUnknown);
impl IWMDMOperation3 {
    #[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
    pub unsafe fn BeginRead(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.BeginRead)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
    pub unsafe fn BeginWrite(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.BeginWrite)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
    pub unsafe fn GetObjectName(&self, pwszname: &mut [u16]) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.GetObjectName)(::core::mem::transmute_copy(self), ::core::mem::transmute(::windows::core::as_mut_ptr_or_null(pwszname)), pwszname.len() as _).ok()
    }
    #[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
    pub unsafe fn SetObjectName(&self, pwszname: &[u16]) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.SetObjectName)(::core::mem::transmute_copy(self), ::core::mem::transmute(::windows::core::as_ptr_or_null(pwszname)), pwszname.len() as _).ok()
    }
    #[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
    pub unsafe fn GetObjectAttributes(&self, pdwattributes: *mut u32, pformat: *mut _WAVEFORMATEX) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.GetObjectAttributes)(::core::mem::transmute_copy(self), ::core::mem::transmute(pdwattributes), ::core::mem::transmute(pformat)).ok()
    }
    #[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
    pub unsafe fn SetObjectAttributes(&self, dwattributes: u32, pformat: *const _WAVEFORMATEX) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.SetObjectAttributes)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwattributes), ::core::mem::transmute(pformat)).ok()
    }
    #[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
    pub unsafe fn GetObjectTotalSize(&self, pdwsize: *mut u32, pdwsizehigh: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.GetObjectTotalSize)(::core::mem::transmute_copy(self), ::core::mem::transmute(pdwsize), ::core::mem::transmute(pdwsizehigh)).ok()
    }
    #[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
    pub unsafe fn SetObjectTotalSize(&self, dwsize: u32, dwsizehigh: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.SetObjectTotalSize)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwsize), ::core::mem::transmute(dwsizehigh)).ok()
    }
    #[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
    pub unsafe fn TransferObjectData(&self, pdata: *mut u8, pdwsize: *mut u32, abmac: *mut u8) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.TransferObjectData)(::core::mem::transmute_copy(self), ::core::mem::transmute(pdata), ::core::mem::transmute(pdwsize), ::core::mem::transmute(abmac)).ok()
    }
    #[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
    pub unsafe fn End<'a, Param1: ::windows::core::IntoParam<'a, ::windows::core::IUnknown>>(&self, phcompletioncode: *const ::windows::core::HRESULT, pnewobject: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.End)(::core::mem::transmute_copy(self), ::core::mem::transmute(phcompletioncode), pnewobject.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
    pub unsafe fn TransferObjectDataOnClearChannel(&self, pdata: *mut u8, pdwsize: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).TransferObjectDataOnClearChannel)(::core::mem::transmute_copy(self), ::core::mem::transmute(pdata), ::core::mem::transmute(pdwsize)).ok()
    }
}
impl ::core::convert::From<IWMDMOperation3> for ::windows::core::IUnknown {
    fn from(value: IWMDMOperation3) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWMDMOperation3> for ::windows::core::IUnknown {
    fn from(value: &IWMDMOperation3) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IWMDMOperation3 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IWMDMOperation3 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IWMDMOperation3> for IWMDMOperation {
    fn from(value: IWMDMOperation3) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWMDMOperation3> for IWMDMOperation {
    fn from(value: &IWMDMOperation3) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IWMDMOperation> for IWMDMOperation3 {
    fn into_param(self) -> ::windows::core::Param<'a, IWMDMOperation> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IWMDMOperation> for &'a IWMDMOperation3 {
    fn into_param(self) -> ::windows::core::Param<'a, IWMDMOperation> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IWMDMOperation3 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
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
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd1f9b46a_9ca8_46d8_9d0f_1ec9bae54919);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMDMOperation3_Vtbl {
    pub base: IWMDMOperation_Vtbl,
    pub TransferObjectDataOnClearChannel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdata: *mut u8, pdwsize: *mut u32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
#[repr(transparent)]
pub struct IWMDMProgress(::windows::core::IUnknown);
impl IWMDMProgress {
    #[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
    pub unsafe fn Begin(&self, dwestimatedticks: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Begin)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwestimatedticks)).ok()
    }
    #[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
    pub unsafe fn Progress(&self, dwtranspiredticks: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Progress)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwtranspiredticks)).ok()
    }
    #[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
    pub unsafe fn End(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).End)(::core::mem::transmute_copy(self)).ok()
    }
}
impl ::core::convert::From<IWMDMProgress> for ::windows::core::IUnknown {
    fn from(value: IWMDMProgress) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWMDMProgress> for ::windows::core::IUnknown {
    fn from(value: &IWMDMProgress) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IWMDMProgress {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IWMDMProgress {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IWMDMProgress {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
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
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1dcb3a0c_33ed_11d3_8470_00c04f79dbc0);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMDMProgress_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    pub Begin: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwestimatedticks: u32) -> ::windows::core::HRESULT,
    pub Progress: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwtranspiredticks: u32) -> ::windows::core::HRESULT,
    pub End: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
#[repr(transparent)]
pub struct IWMDMProgress2(::windows::core::IUnknown);
impl IWMDMProgress2 {
    #[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
    pub unsafe fn Begin(&self, dwestimatedticks: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.Begin)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwestimatedticks)).ok()
    }
    #[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
    pub unsafe fn Progress(&self, dwtranspiredticks: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.Progress)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwtranspiredticks)).ok()
    }
    #[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
    pub unsafe fn End(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.End)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
    pub unsafe fn End2(&self, hrcompletioncode: ::windows::core::HRESULT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).End2)(::core::mem::transmute_copy(self), ::core::mem::transmute(hrcompletioncode)).ok()
    }
}
impl ::core::convert::From<IWMDMProgress2> for ::windows::core::IUnknown {
    fn from(value: IWMDMProgress2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWMDMProgress2> for ::windows::core::IUnknown {
    fn from(value: &IWMDMProgress2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IWMDMProgress2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IWMDMProgress2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IWMDMProgress2> for IWMDMProgress {
    fn from(value: IWMDMProgress2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWMDMProgress2> for IWMDMProgress {
    fn from(value: &IWMDMProgress2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IWMDMProgress> for IWMDMProgress2 {
    fn into_param(self) -> ::windows::core::Param<'a, IWMDMProgress> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IWMDMProgress> for &'a IWMDMProgress2 {
    fn into_param(self) -> ::windows::core::Param<'a, IWMDMProgress> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IWMDMProgress2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
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
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3a43f550_b383_4e92_b04a_e6bbc660fefc);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMDMProgress2_Vtbl {
    pub base: IWMDMProgress_Vtbl,
    pub End2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hrcompletioncode: ::windows::core::HRESULT) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
#[repr(transparent)]
pub struct IWMDMProgress3(::windows::core::IUnknown);
impl IWMDMProgress3 {
    #[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
    pub unsafe fn Begin(&self, dwestimatedticks: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.base.Begin)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwestimatedticks)).ok()
    }
    #[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
    pub unsafe fn Progress(&self, dwtranspiredticks: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.base.Progress)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwtranspiredticks)).ok()
    }
    #[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
    pub unsafe fn End(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.base.End)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
    pub unsafe fn End2(&self, hrcompletioncode: ::windows::core::HRESULT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.End2)(::core::mem::transmute_copy(self), ::core::mem::transmute(hrcompletioncode)).ok()
    }
    #[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
    pub unsafe fn Begin3<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::GUID>>(&self, eventid: Param0, dwestimatedticks: u32, pcontext: *mut OPAQUECOMMAND) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Begin3)(::core::mem::transmute_copy(self), eventid.into_param().abi(), ::core::mem::transmute(dwestimatedticks), ::core::mem::transmute(pcontext)).ok()
    }
    #[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
    pub unsafe fn Progress3<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::GUID>>(&self, eventid: Param0, dwtranspiredticks: u32, pcontext: *mut OPAQUECOMMAND) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Progress3)(::core::mem::transmute_copy(self), eventid.into_param().abi(), ::core::mem::transmute(dwtranspiredticks), ::core::mem::transmute(pcontext)).ok()
    }
    #[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
    pub unsafe fn End3<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::GUID>>(&self, eventid: Param0, hrcompletioncode: ::windows::core::HRESULT, pcontext: *mut OPAQUECOMMAND) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).End3)(::core::mem::transmute_copy(self), eventid.into_param().abi(), ::core::mem::transmute(hrcompletioncode), ::core::mem::transmute(pcontext)).ok()
    }
}
impl ::core::convert::From<IWMDMProgress3> for ::windows::core::IUnknown {
    fn from(value: IWMDMProgress3) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWMDMProgress3> for ::windows::core::IUnknown {
    fn from(value: &IWMDMProgress3) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IWMDMProgress3 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IWMDMProgress3 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IWMDMProgress3> for IWMDMProgress {
    fn from(value: IWMDMProgress3) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWMDMProgress3> for IWMDMProgress {
    fn from(value: &IWMDMProgress3) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IWMDMProgress> for IWMDMProgress3 {
    fn into_param(self) -> ::windows::core::Param<'a, IWMDMProgress> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IWMDMProgress> for &'a IWMDMProgress3 {
    fn into_param(self) -> ::windows::core::Param<'a, IWMDMProgress> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IWMDMProgress3> for IWMDMProgress2 {
    fn from(value: IWMDMProgress3) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWMDMProgress3> for IWMDMProgress2 {
    fn from(value: &IWMDMProgress3) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IWMDMProgress2> for IWMDMProgress3 {
    fn into_param(self) -> ::windows::core::Param<'a, IWMDMProgress2> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IWMDMProgress2> for &'a IWMDMProgress3 {
    fn into_param(self) -> ::windows::core::Param<'a, IWMDMProgress2> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IWMDMProgress3 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
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
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x21de01cb_3bb4_4929_b21a_17af3f80f658);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMDMProgress3_Vtbl {
    pub base: IWMDMProgress2_Vtbl,
    pub Begin3: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, eventid: ::windows::core::GUID, dwestimatedticks: u32, pcontext: *mut OPAQUECOMMAND) -> ::windows::core::HRESULT,
    pub Progress3: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, eventid: ::windows::core::GUID, dwtranspiredticks: u32, pcontext: *mut OPAQUECOMMAND) -> ::windows::core::HRESULT,
    pub End3: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, eventid: ::windows::core::GUID, hrcompletioncode: ::windows::core::HRESULT, pcontext: *mut OPAQUECOMMAND) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
#[repr(transparent)]
pub struct IWMDMRevoked(::windows::core::IUnknown);
impl IWMDMRevoked {
    #[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
    pub unsafe fn GetRevocationURL(&self, ppwszrevocationurl: *mut ::windows::core::PWSTR, pdwbufferlen: *mut u32, pdwrevokedbitflag: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetRevocationURL)(::core::mem::transmute_copy(self), ::core::mem::transmute(ppwszrevocationurl), ::core::mem::transmute(pdwbufferlen), ::core::mem::transmute(pdwrevokedbitflag)).ok()
    }
}
impl ::core::convert::From<IWMDMRevoked> for ::windows::core::IUnknown {
    fn from(value: IWMDMRevoked) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWMDMRevoked> for ::windows::core::IUnknown {
    fn from(value: &IWMDMRevoked) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IWMDMRevoked {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IWMDMRevoked {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IWMDMRevoked {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
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
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xebeccedb_88ee_4e55_b6a4_8d9f07d696aa);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMDMRevoked_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    pub GetRevocationURL: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppwszrevocationurl: *mut ::windows::core::PWSTR, pdwbufferlen: *mut u32, pdwrevokedbitflag: *mut u32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
#[repr(transparent)]
pub struct IWMDMStorage(::windows::core::IUnknown);
impl IWMDMStorage {
    #[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
    pub unsafe fn SetAttributes(&self, dwattributes: u32, pformat: *const _WAVEFORMATEX) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetAttributes)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwattributes), ::core::mem::transmute(pformat)).ok()
    }
    #[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
    pub unsafe fn GetStorageGlobals(&self) -> ::windows::core::Result<IWMDMStorageGlobals> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetStorageGlobals)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IWMDMStorageGlobals>(result__)
    }
    #[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
    pub unsafe fn GetAttributes(&self, pdwattributes: *mut u32, pformat: *mut _WAVEFORMATEX) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetAttributes)(::core::mem::transmute_copy(self), ::core::mem::transmute(pdwattributes), ::core::mem::transmute(pformat)).ok()
    }
    #[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
    pub unsafe fn GetName(&self, pwszname: &mut [u16]) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetName)(::core::mem::transmute_copy(self), ::core::mem::transmute(::windows::core::as_mut_ptr_or_null(pwszname)), pwszname.len() as _).ok()
    }
    #[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
    pub unsafe fn GetDate(&self) -> ::windows::core::Result<WMDMDATETIME> {
        let mut result__: WMDMDATETIME = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetDate)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<WMDMDATETIME>(result__)
    }
    #[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
    pub unsafe fn GetSize(&self, pdwsizelow: *mut u32, pdwsizehigh: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetSize)(::core::mem::transmute_copy(self), ::core::mem::transmute(pdwsizelow), ::core::mem::transmute(pdwsizehigh)).ok()
    }
    #[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
    pub unsafe fn GetRights(&self, pprights: *mut *mut WMDMRIGHTS, pnrightscount: *mut u32, abmac: *mut u8) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetRights)(::core::mem::transmute_copy(self), ::core::mem::transmute(pprights), ::core::mem::transmute(pnrightscount), ::core::mem::transmute(abmac)).ok()
    }
    #[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
    pub unsafe fn EnumStorage(&self) -> ::windows::core::Result<IWMDMEnumStorage> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).EnumStorage)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IWMDMEnumStorage>(result__)
    }
    #[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
    pub unsafe fn SendOpaqueCommand(&self, pcommand: *mut OPAQUECOMMAND) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SendOpaqueCommand)(::core::mem::transmute_copy(self), ::core::mem::transmute(pcommand)).ok()
    }
}
impl ::core::convert::From<IWMDMStorage> for ::windows::core::IUnknown {
    fn from(value: IWMDMStorage) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWMDMStorage> for ::windows::core::IUnknown {
    fn from(value: &IWMDMStorage) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IWMDMStorage {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IWMDMStorage {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IWMDMStorage {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
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
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1dcb3a06_33ed_11d3_8470_00c04f79dbc0);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMDMStorage_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    pub SetAttributes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwattributes: u32, pformat: *const _WAVEFORMATEX) -> ::windows::core::HRESULT,
    pub GetStorageGlobals: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppstorageglobals: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub GetAttributes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdwattributes: *mut u32, pformat: *mut _WAVEFORMATEX) -> ::windows::core::HRESULT,
    pub GetName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwszname: ::windows::core::PWSTR, nmaxchars: u32) -> ::windows::core::HRESULT,
    pub GetDate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdatetimeutc: *mut WMDMDATETIME) -> ::windows::core::HRESULT,
    pub GetSize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdwsizelow: *mut u32, pdwsizehigh: *mut u32) -> ::windows::core::HRESULT,
    pub GetRights: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pprights: *mut *mut WMDMRIGHTS, pnrightscount: *mut u32, abmac: *mut u8) -> ::windows::core::HRESULT,
    pub EnumStorage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, penumstorage: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub SendOpaqueCommand: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcommand: *mut OPAQUECOMMAND) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
#[repr(transparent)]
pub struct IWMDMStorage2(::windows::core::IUnknown);
impl IWMDMStorage2 {
    #[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
    pub unsafe fn SetAttributes(&self, dwattributes: u32, pformat: *const _WAVEFORMATEX) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.SetAttributes)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwattributes), ::core::mem::transmute(pformat)).ok()
    }
    #[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
    pub unsafe fn GetStorageGlobals(&self) -> ::windows::core::Result<IWMDMStorageGlobals> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.GetStorageGlobals)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IWMDMStorageGlobals>(result__)
    }
    #[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
    pub unsafe fn GetAttributes(&self, pdwattributes: *mut u32, pformat: *mut _WAVEFORMATEX) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.GetAttributes)(::core::mem::transmute_copy(self), ::core::mem::transmute(pdwattributes), ::core::mem::transmute(pformat)).ok()
    }
    #[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
    pub unsafe fn GetName(&self, pwszname: &mut [u16]) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.GetName)(::core::mem::transmute_copy(self), ::core::mem::transmute(::windows::core::as_mut_ptr_or_null(pwszname)), pwszname.len() as _).ok()
    }
    #[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
    pub unsafe fn GetDate(&self) -> ::windows::core::Result<WMDMDATETIME> {
        let mut result__: WMDMDATETIME = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.GetDate)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<WMDMDATETIME>(result__)
    }
    #[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
    pub unsafe fn GetSize(&self, pdwsizelow: *mut u32, pdwsizehigh: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.GetSize)(::core::mem::transmute_copy(self), ::core::mem::transmute(pdwsizelow), ::core::mem::transmute(pdwsizehigh)).ok()
    }
    #[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
    pub unsafe fn GetRights(&self, pprights: *mut *mut WMDMRIGHTS, pnrightscount: *mut u32, abmac: *mut u8) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.GetRights)(::core::mem::transmute_copy(self), ::core::mem::transmute(pprights), ::core::mem::transmute(pnrightscount), ::core::mem::transmute(abmac)).ok()
    }
    #[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
    pub unsafe fn EnumStorage(&self) -> ::windows::core::Result<IWMDMEnumStorage> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.EnumStorage)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IWMDMEnumStorage>(result__)
    }
    #[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
    pub unsafe fn SendOpaqueCommand(&self, pcommand: *mut OPAQUECOMMAND) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.SendOpaqueCommand)(::core::mem::transmute_copy(self), ::core::mem::transmute(pcommand)).ok()
    }
    #[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
    pub unsafe fn GetStorage<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(&self, pszstoragename: Param0) -> ::windows::core::Result<IWMDMStorage> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetStorage)(::core::mem::transmute_copy(self), pszstoragename.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<IWMDMStorage>(result__)
    }
    #[doc = "*Required features: `\"Win32_Media_DeviceManager\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetAttributes2(&self, dwattributes: u32, dwattributesex: u32, pformat: *const _WAVEFORMATEX, pvideoformat: *const _VIDEOINFOHEADER) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetAttributes2)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwattributes), ::core::mem::transmute(dwattributesex), ::core::mem::transmute(pformat), ::core::mem::transmute(pvideoformat)).ok()
    }
    #[doc = "*Required features: `\"Win32_Media_DeviceManager\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetAttributes2(&self, pdwattributes: *mut u32, pdwattributesex: *mut u32, paudioformat: *mut _WAVEFORMATEX, pvideoformat: *mut _VIDEOINFOHEADER) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetAttributes2)(::core::mem::transmute_copy(self), ::core::mem::transmute(pdwattributes), ::core::mem::transmute(pdwattributesex), ::core::mem::transmute(paudioformat), ::core::mem::transmute(pvideoformat)).ok()
    }
}
impl ::core::convert::From<IWMDMStorage2> for ::windows::core::IUnknown {
    fn from(value: IWMDMStorage2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWMDMStorage2> for ::windows::core::IUnknown {
    fn from(value: &IWMDMStorage2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IWMDMStorage2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IWMDMStorage2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IWMDMStorage2> for IWMDMStorage {
    fn from(value: IWMDMStorage2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWMDMStorage2> for IWMDMStorage {
    fn from(value: &IWMDMStorage2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IWMDMStorage> for IWMDMStorage2 {
    fn into_param(self) -> ::windows::core::Param<'a, IWMDMStorage> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IWMDMStorage> for &'a IWMDMStorage2 {
    fn into_param(self) -> ::windows::core::Param<'a, IWMDMStorage> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IWMDMStorage2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
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
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1ed5a144_5cd5_4683_9eff_72cbdb2d9533);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMDMStorage2_Vtbl {
    pub base: IWMDMStorage_Vtbl,
    pub GetStorage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszstoragename: ::windows::core::PCWSTR, ppstorage: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub SetAttributes2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwattributes: u32, dwattributesex: u32, pformat: *const _WAVEFORMATEX, pvideoformat: *const _VIDEOINFOHEADER) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetAttributes2: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub GetAttributes2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdwattributes: *mut u32, pdwattributesex: *mut u32, paudioformat: *mut _WAVEFORMATEX, pvideoformat: *mut _VIDEOINFOHEADER) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetAttributes2: usize,
}
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
#[repr(transparent)]
pub struct IWMDMStorage3(::windows::core::IUnknown);
impl IWMDMStorage3 {
    #[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
    pub unsafe fn SetAttributes(&self, dwattributes: u32, pformat: *const _WAVEFORMATEX) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.base.SetAttributes)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwattributes), ::core::mem::transmute(pformat)).ok()
    }
    #[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
    pub unsafe fn GetStorageGlobals(&self) -> ::windows::core::Result<IWMDMStorageGlobals> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.base.GetStorageGlobals)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IWMDMStorageGlobals>(result__)
    }
    #[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
    pub unsafe fn GetAttributes(&self, pdwattributes: *mut u32, pformat: *mut _WAVEFORMATEX) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.base.GetAttributes)(::core::mem::transmute_copy(self), ::core::mem::transmute(pdwattributes), ::core::mem::transmute(pformat)).ok()
    }
    #[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
    pub unsafe fn GetName(&self, pwszname: &mut [u16]) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.base.GetName)(::core::mem::transmute_copy(self), ::core::mem::transmute(::windows::core::as_mut_ptr_or_null(pwszname)), pwszname.len() as _).ok()
    }
    #[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
    pub unsafe fn GetDate(&self) -> ::windows::core::Result<WMDMDATETIME> {
        let mut result__: WMDMDATETIME = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.base.GetDate)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<WMDMDATETIME>(result__)
    }
    #[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
    pub unsafe fn GetSize(&self, pdwsizelow: *mut u32, pdwsizehigh: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.base.GetSize)(::core::mem::transmute_copy(self), ::core::mem::transmute(pdwsizelow), ::core::mem::transmute(pdwsizehigh)).ok()
    }
    #[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
    pub unsafe fn GetRights(&self, pprights: *mut *mut WMDMRIGHTS, pnrightscount: *mut u32, abmac: *mut u8) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.base.GetRights)(::core::mem::transmute_copy(self), ::core::mem::transmute(pprights), ::core::mem::transmute(pnrightscount), ::core::mem::transmute(abmac)).ok()
    }
    #[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
    pub unsafe fn EnumStorage(&self) -> ::windows::core::Result<IWMDMEnumStorage> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.base.EnumStorage)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IWMDMEnumStorage>(result__)
    }
    #[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
    pub unsafe fn SendOpaqueCommand(&self, pcommand: *mut OPAQUECOMMAND) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.base.SendOpaqueCommand)(::core::mem::transmute_copy(self), ::core::mem::transmute(pcommand)).ok()
    }
    #[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
    pub unsafe fn GetStorage<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(&self, pszstoragename: Param0) -> ::windows::core::Result<IWMDMStorage> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.GetStorage)(::core::mem::transmute_copy(self), pszstoragename.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<IWMDMStorage>(result__)
    }
    #[doc = "*Required features: `\"Win32_Media_DeviceManager\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetAttributes2(&self, dwattributes: u32, dwattributesex: u32, pformat: *const _WAVEFORMATEX, pvideoformat: *const _VIDEOINFOHEADER) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.SetAttributes2)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwattributes), ::core::mem::transmute(dwattributesex), ::core::mem::transmute(pformat), ::core::mem::transmute(pvideoformat)).ok()
    }
    #[doc = "*Required features: `\"Win32_Media_DeviceManager\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetAttributes2(&self, pdwattributes: *mut u32, pdwattributesex: *mut u32, paudioformat: *mut _WAVEFORMATEX, pvideoformat: *mut _VIDEOINFOHEADER) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.GetAttributes2)(::core::mem::transmute_copy(self), ::core::mem::transmute(pdwattributes), ::core::mem::transmute(pdwattributesex), ::core::mem::transmute(paudioformat), ::core::mem::transmute(pvideoformat)).ok()
    }
    #[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
    pub unsafe fn GetMetadata(&self) -> ::windows::core::Result<IWMDMMetaData> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetMetadata)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IWMDMMetaData>(result__)
    }
    #[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
    pub unsafe fn SetMetadata<'a, Param0: ::windows::core::IntoParam<'a, IWMDMMetaData>>(&self, pmetadata: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetMetadata)(::core::mem::transmute_copy(self), pmetadata.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
    pub unsafe fn CreateEmptyMetadataObject(&self) -> ::windows::core::Result<IWMDMMetaData> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).CreateEmptyMetadataObject)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IWMDMMetaData>(result__)
    }
    #[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
    pub unsafe fn SetEnumPreference(&self, pmode: *mut WMDM_STORAGE_ENUM_MODE, pviews: &[WMDMMetadataView]) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetEnumPreference)(::core::mem::transmute_copy(self), ::core::mem::transmute(pmode), pviews.len() as _, ::core::mem::transmute(::windows::core::as_ptr_or_null(pviews))).ok()
    }
}
impl ::core::convert::From<IWMDMStorage3> for ::windows::core::IUnknown {
    fn from(value: IWMDMStorage3) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWMDMStorage3> for ::windows::core::IUnknown {
    fn from(value: &IWMDMStorage3) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IWMDMStorage3 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IWMDMStorage3 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IWMDMStorage3> for IWMDMStorage {
    fn from(value: IWMDMStorage3) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWMDMStorage3> for IWMDMStorage {
    fn from(value: &IWMDMStorage3) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IWMDMStorage> for IWMDMStorage3 {
    fn into_param(self) -> ::windows::core::Param<'a, IWMDMStorage> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IWMDMStorage> for &'a IWMDMStorage3 {
    fn into_param(self) -> ::windows::core::Param<'a, IWMDMStorage> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IWMDMStorage3> for IWMDMStorage2 {
    fn from(value: IWMDMStorage3) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWMDMStorage3> for IWMDMStorage2 {
    fn from(value: &IWMDMStorage3) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IWMDMStorage2> for IWMDMStorage3 {
    fn into_param(self) -> ::windows::core::Param<'a, IWMDMStorage2> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IWMDMStorage2> for &'a IWMDMStorage3 {
    fn into_param(self) -> ::windows::core::Param<'a, IWMDMStorage2> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IWMDMStorage3 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
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
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x97717eea_926a_464e_96a4_247b0216026e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMDMStorage3_Vtbl {
    pub base: IWMDMStorage2_Vtbl,
    pub GetMetadata: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppmetadata: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub SetMetadata: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pmetadata: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub CreateEmptyMetadataObject: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppmetadata: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub SetEnumPreference: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pmode: *mut WMDM_STORAGE_ENUM_MODE, nviews: u32, pviews: *const WMDMMetadataView) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
#[repr(transparent)]
pub struct IWMDMStorage4(::windows::core::IUnknown);
impl IWMDMStorage4 {
    #[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
    pub unsafe fn SetAttributes(&self, dwattributes: u32, pformat: *const _WAVEFORMATEX) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.base.base.SetAttributes)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwattributes), ::core::mem::transmute(pformat)).ok()
    }
    #[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
    pub unsafe fn GetStorageGlobals(&self) -> ::windows::core::Result<IWMDMStorageGlobals> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.base.base.GetStorageGlobals)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IWMDMStorageGlobals>(result__)
    }
    #[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
    pub unsafe fn GetAttributes(&self, pdwattributes: *mut u32, pformat: *mut _WAVEFORMATEX) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.base.base.GetAttributes)(::core::mem::transmute_copy(self), ::core::mem::transmute(pdwattributes), ::core::mem::transmute(pformat)).ok()
    }
    #[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
    pub unsafe fn GetName(&self, pwszname: &mut [u16]) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.base.base.GetName)(::core::mem::transmute_copy(self), ::core::mem::transmute(::windows::core::as_mut_ptr_or_null(pwszname)), pwszname.len() as _).ok()
    }
    #[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
    pub unsafe fn GetDate(&self) -> ::windows::core::Result<WMDMDATETIME> {
        let mut result__: WMDMDATETIME = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.base.base.GetDate)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<WMDMDATETIME>(result__)
    }
    #[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
    pub unsafe fn GetSize(&self, pdwsizelow: *mut u32, pdwsizehigh: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.base.base.GetSize)(::core::mem::transmute_copy(self), ::core::mem::transmute(pdwsizelow), ::core::mem::transmute(pdwsizehigh)).ok()
    }
    #[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
    pub unsafe fn GetRights(&self, pprights: *mut *mut WMDMRIGHTS, pnrightscount: *mut u32, abmac: *mut u8) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.base.base.GetRights)(::core::mem::transmute_copy(self), ::core::mem::transmute(pprights), ::core::mem::transmute(pnrightscount), ::core::mem::transmute(abmac)).ok()
    }
    #[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
    pub unsafe fn EnumStorage(&self) -> ::windows::core::Result<IWMDMEnumStorage> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.base.base.EnumStorage)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IWMDMEnumStorage>(result__)
    }
    #[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
    pub unsafe fn SendOpaqueCommand(&self, pcommand: *mut OPAQUECOMMAND) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.base.base.SendOpaqueCommand)(::core::mem::transmute_copy(self), ::core::mem::transmute(pcommand)).ok()
    }
    #[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
    pub unsafe fn GetStorage<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(&self, pszstoragename: Param0) -> ::windows::core::Result<IWMDMStorage> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.base.GetStorage)(::core::mem::transmute_copy(self), pszstoragename.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<IWMDMStorage>(result__)
    }
    #[doc = "*Required features: `\"Win32_Media_DeviceManager\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetAttributes2(&self, dwattributes: u32, dwattributesex: u32, pformat: *const _WAVEFORMATEX, pvideoformat: *const _VIDEOINFOHEADER) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.base.SetAttributes2)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwattributes), ::core::mem::transmute(dwattributesex), ::core::mem::transmute(pformat), ::core::mem::transmute(pvideoformat)).ok()
    }
    #[doc = "*Required features: `\"Win32_Media_DeviceManager\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetAttributes2(&self, pdwattributes: *mut u32, pdwattributesex: *mut u32, paudioformat: *mut _WAVEFORMATEX, pvideoformat: *mut _VIDEOINFOHEADER) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.base.GetAttributes2)(::core::mem::transmute_copy(self), ::core::mem::transmute(pdwattributes), ::core::mem::transmute(pdwattributesex), ::core::mem::transmute(paudioformat), ::core::mem::transmute(pvideoformat)).ok()
    }
    #[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
    pub unsafe fn GetMetadata(&self) -> ::windows::core::Result<IWMDMMetaData> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.GetMetadata)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IWMDMMetaData>(result__)
    }
    #[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
    pub unsafe fn SetMetadata<'a, Param0: ::windows::core::IntoParam<'a, IWMDMMetaData>>(&self, pmetadata: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.SetMetadata)(::core::mem::transmute_copy(self), pmetadata.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
    pub unsafe fn CreateEmptyMetadataObject(&self) -> ::windows::core::Result<IWMDMMetaData> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.CreateEmptyMetadataObject)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IWMDMMetaData>(result__)
    }
    #[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
    pub unsafe fn SetEnumPreference(&self, pmode: *mut WMDM_STORAGE_ENUM_MODE, pviews: &[WMDMMetadataView]) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.SetEnumPreference)(::core::mem::transmute_copy(self), ::core::mem::transmute(pmode), pviews.len() as _, ::core::mem::transmute(::windows::core::as_ptr_or_null(pviews))).ok()
    }
    #[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
    pub unsafe fn SetReferences(&self, ppiwmdmstorage: &[::core::option::Option<IWMDMStorage>]) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetReferences)(::core::mem::transmute_copy(self), ppiwmdmstorage.len() as _, ::core::mem::transmute(::windows::core::as_ptr_or_null(ppiwmdmstorage))).ok()
    }
    #[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
    pub unsafe fn GetReferences(&self, pdwrefs: *mut u32, pppiwmdmstorage: *mut *mut ::core::option::Option<IWMDMStorage>) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetReferences)(::core::mem::transmute_copy(self), ::core::mem::transmute(pdwrefs), ::core::mem::transmute(pppiwmdmstorage)).ok()
    }
    #[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
    pub unsafe fn GetRightsWithProgress<'a, Param0: ::windows::core::IntoParam<'a, IWMDMProgress3>>(&self, piprogresscallback: Param0, pprights: *mut *mut WMDMRIGHTS, pnrightscount: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetRightsWithProgress)(::core::mem::transmute_copy(self), piprogresscallback.into_param().abi(), ::core::mem::transmute(pprights), ::core::mem::transmute(pnrightscount)).ok()
    }
    #[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
    pub unsafe fn GetSpecifiedMetadata(&self, ppwszpropnames: &[::windows::core::PWSTR]) -> ::windows::core::Result<IWMDMMetaData> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetSpecifiedMetadata)(::core::mem::transmute_copy(self), ppwszpropnames.len() as _, ::core::mem::transmute(::windows::core::as_ptr_or_null(ppwszpropnames)), ::core::mem::transmute(&mut result__)).from_abi::<IWMDMMetaData>(result__)
    }
    #[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
    pub unsafe fn FindStorage<'a, Param1: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(&self, findscope: WMDM_FIND_SCOPE, pwszuniqueid: Param1) -> ::windows::core::Result<IWMDMStorage> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).FindStorage)(::core::mem::transmute_copy(self), ::core::mem::transmute(findscope), pwszuniqueid.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<IWMDMStorage>(result__)
    }
    #[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
    pub unsafe fn GetParent(&self) -> ::windows::core::Result<IWMDMStorage> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetParent)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IWMDMStorage>(result__)
    }
}
impl ::core::convert::From<IWMDMStorage4> for ::windows::core::IUnknown {
    fn from(value: IWMDMStorage4) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWMDMStorage4> for ::windows::core::IUnknown {
    fn from(value: &IWMDMStorage4) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IWMDMStorage4 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IWMDMStorage4 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IWMDMStorage4> for IWMDMStorage {
    fn from(value: IWMDMStorage4) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWMDMStorage4> for IWMDMStorage {
    fn from(value: &IWMDMStorage4) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IWMDMStorage> for IWMDMStorage4 {
    fn into_param(self) -> ::windows::core::Param<'a, IWMDMStorage> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IWMDMStorage> for &'a IWMDMStorage4 {
    fn into_param(self) -> ::windows::core::Param<'a, IWMDMStorage> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IWMDMStorage4> for IWMDMStorage2 {
    fn from(value: IWMDMStorage4) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWMDMStorage4> for IWMDMStorage2 {
    fn from(value: &IWMDMStorage4) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IWMDMStorage2> for IWMDMStorage4 {
    fn into_param(self) -> ::windows::core::Param<'a, IWMDMStorage2> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IWMDMStorage2> for &'a IWMDMStorage4 {
    fn into_param(self) -> ::windows::core::Param<'a, IWMDMStorage2> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IWMDMStorage4> for IWMDMStorage3 {
    fn from(value: IWMDMStorage4) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWMDMStorage4> for IWMDMStorage3 {
    fn from(value: &IWMDMStorage4) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IWMDMStorage3> for IWMDMStorage4 {
    fn into_param(self) -> ::windows::core::Param<'a, IWMDMStorage3> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IWMDMStorage3> for &'a IWMDMStorage4 {
    fn into_param(self) -> ::windows::core::Param<'a, IWMDMStorage3> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IWMDMStorage4 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
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
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc225bac5_a03a_40b8_9a23_91cf478c64a6);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMDMStorage4_Vtbl {
    pub base: IWMDMStorage3_Vtbl,
    pub SetReferences: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwrefs: u32, ppiwmdmstorage: *const ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub GetReferences: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdwrefs: *mut u32, pppiwmdmstorage: *mut *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub GetRightsWithProgress: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, piprogresscallback: ::windows::core::RawPtr, pprights: *mut *mut WMDMRIGHTS, pnrightscount: *mut u32) -> ::windows::core::HRESULT,
    pub GetSpecifiedMetadata: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cproperties: u32, ppwszpropnames: *const ::windows::core::PWSTR, ppmetadata: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub FindStorage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, findscope: WMDM_FIND_SCOPE, pwszuniqueid: ::windows::core::PCWSTR, ppstorage: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub GetParent: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppstorage: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
#[repr(transparent)]
pub struct IWMDMStorageControl(::windows::core::IUnknown);
impl IWMDMStorageControl {
    #[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
    pub unsafe fn Insert<'a, Param1: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>, Param2: ::windows::core::IntoParam<'a, IWMDMOperation>, Param3: ::windows::core::IntoParam<'a, IWMDMProgress>>(&self, fumode: u32, pwszfile: Param1, poperation: Param2, pprogress: Param3) -> ::windows::core::Result<IWMDMStorage> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).Insert)(::core::mem::transmute_copy(self), ::core::mem::transmute(fumode), pwszfile.into_param().abi(), poperation.into_param().abi(), pprogress.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<IWMDMStorage>(result__)
    }
    #[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
    pub unsafe fn Delete<'a, Param1: ::windows::core::IntoParam<'a, IWMDMProgress>>(&self, fumode: u32, pprogress: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Delete)(::core::mem::transmute_copy(self), ::core::mem::transmute(fumode), pprogress.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
    pub unsafe fn Rename<'a, Param1: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>, Param2: ::windows::core::IntoParam<'a, IWMDMProgress>>(&self, fumode: u32, pwsznewname: Param1, pprogress: Param2) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Rename)(::core::mem::transmute_copy(self), ::core::mem::transmute(fumode), pwsznewname.into_param().abi(), pprogress.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
    pub unsafe fn Read<'a, Param1: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>, Param2: ::windows::core::IntoParam<'a, IWMDMProgress>, Param3: ::windows::core::IntoParam<'a, IWMDMOperation>>(&self, fumode: u32, pwszfile: Param1, pprogress: Param2, poperation: Param3) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Read)(::core::mem::transmute_copy(self), ::core::mem::transmute(fumode), pwszfile.into_param().abi(), pprogress.into_param().abi(), poperation.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
    pub unsafe fn Move<'a, Param1: ::windows::core::IntoParam<'a, IWMDMStorage>, Param2: ::windows::core::IntoParam<'a, IWMDMProgress>>(&self, fumode: u32, ptargetobject: Param1, pprogress: Param2) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Move)(::core::mem::transmute_copy(self), ::core::mem::transmute(fumode), ptargetobject.into_param().abi(), pprogress.into_param().abi()).ok()
    }
}
impl ::core::convert::From<IWMDMStorageControl> for ::windows::core::IUnknown {
    fn from(value: IWMDMStorageControl) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWMDMStorageControl> for ::windows::core::IUnknown {
    fn from(value: &IWMDMStorageControl) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IWMDMStorageControl {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IWMDMStorageControl {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IWMDMStorageControl {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
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
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1dcb3a08_33ed_11d3_8470_00c04f79dbc0);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMDMStorageControl_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    pub Insert: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, fumode: u32, pwszfile: ::windows::core::PCWSTR, poperation: ::windows::core::RawPtr, pprogress: ::windows::core::RawPtr, ppnewobject: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub Delete: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, fumode: u32, pprogress: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub Rename: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, fumode: u32, pwsznewname: ::windows::core::PCWSTR, pprogress: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub Read: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, fumode: u32, pwszfile: ::windows::core::PCWSTR, pprogress: ::windows::core::RawPtr, poperation: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub Move: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, fumode: u32, ptargetobject: ::windows::core::RawPtr, pprogress: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
#[repr(transparent)]
pub struct IWMDMStorageControl2(::windows::core::IUnknown);
impl IWMDMStorageControl2 {
    #[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
    pub unsafe fn Insert<'a, Param1: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>, Param2: ::windows::core::IntoParam<'a, IWMDMOperation>, Param3: ::windows::core::IntoParam<'a, IWMDMProgress>>(&self, fumode: u32, pwszfile: Param1, poperation: Param2, pprogress: Param3) -> ::windows::core::Result<IWMDMStorage> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.Insert)(::core::mem::transmute_copy(self), ::core::mem::transmute(fumode), pwszfile.into_param().abi(), poperation.into_param().abi(), pprogress.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<IWMDMStorage>(result__)
    }
    #[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
    pub unsafe fn Delete<'a, Param1: ::windows::core::IntoParam<'a, IWMDMProgress>>(&self, fumode: u32, pprogress: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.Delete)(::core::mem::transmute_copy(self), ::core::mem::transmute(fumode), pprogress.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
    pub unsafe fn Rename<'a, Param1: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>, Param2: ::windows::core::IntoParam<'a, IWMDMProgress>>(&self, fumode: u32, pwsznewname: Param1, pprogress: Param2) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.Rename)(::core::mem::transmute_copy(self), ::core::mem::transmute(fumode), pwsznewname.into_param().abi(), pprogress.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
    pub unsafe fn Read<'a, Param1: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>, Param2: ::windows::core::IntoParam<'a, IWMDMProgress>, Param3: ::windows::core::IntoParam<'a, IWMDMOperation>>(&self, fumode: u32, pwszfile: Param1, pprogress: Param2, poperation: Param3) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.Read)(::core::mem::transmute_copy(self), ::core::mem::transmute(fumode), pwszfile.into_param().abi(), pprogress.into_param().abi(), poperation.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
    pub unsafe fn Move<'a, Param1: ::windows::core::IntoParam<'a, IWMDMStorage>, Param2: ::windows::core::IntoParam<'a, IWMDMProgress>>(&self, fumode: u32, ptargetobject: Param1, pprogress: Param2) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.Move)(::core::mem::transmute_copy(self), ::core::mem::transmute(fumode), ptargetobject.into_param().abi(), pprogress.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
    pub unsafe fn Insert2<'a, Param1: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>, Param2: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>, Param3: ::windows::core::IntoParam<'a, IWMDMOperation>, Param4: ::windows::core::IntoParam<'a, IWMDMProgress>, Param5: ::windows::core::IntoParam<'a, ::windows::core::IUnknown>>(&self, fumode: u32, pwszfilesource: Param1, pwszfiledest: Param2, poperation: Param3, pprogress: Param4, punknown: Param5, ppnewobject: *mut ::core::option::Option<IWMDMStorage>) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Insert2)(::core::mem::transmute_copy(self), ::core::mem::transmute(fumode), pwszfilesource.into_param().abi(), pwszfiledest.into_param().abi(), poperation.into_param().abi(), pprogress.into_param().abi(), punknown.into_param().abi(), ::core::mem::transmute(ppnewobject)).ok()
    }
}
impl ::core::convert::From<IWMDMStorageControl2> for ::windows::core::IUnknown {
    fn from(value: IWMDMStorageControl2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWMDMStorageControl2> for ::windows::core::IUnknown {
    fn from(value: &IWMDMStorageControl2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IWMDMStorageControl2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IWMDMStorageControl2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IWMDMStorageControl2> for IWMDMStorageControl {
    fn from(value: IWMDMStorageControl2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWMDMStorageControl2> for IWMDMStorageControl {
    fn from(value: &IWMDMStorageControl2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IWMDMStorageControl> for IWMDMStorageControl2 {
    fn into_param(self) -> ::windows::core::Param<'a, IWMDMStorageControl> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IWMDMStorageControl> for &'a IWMDMStorageControl2 {
    fn into_param(self) -> ::windows::core::Param<'a, IWMDMStorageControl> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IWMDMStorageControl2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
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
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x972c2e88_bd6c_4125_8e09_84f837e637b6);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMDMStorageControl2_Vtbl {
    pub base: IWMDMStorageControl_Vtbl,
    pub Insert2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, fumode: u32, pwszfilesource: ::windows::core::PCWSTR, pwszfiledest: ::windows::core::PCWSTR, poperation: ::windows::core::RawPtr, pprogress: ::windows::core::RawPtr, punknown: *mut ::core::ffi::c_void, ppnewobject: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
#[repr(transparent)]
pub struct IWMDMStorageControl3(::windows::core::IUnknown);
impl IWMDMStorageControl3 {
    #[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
    pub unsafe fn Insert<'a, Param1: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>, Param2: ::windows::core::IntoParam<'a, IWMDMOperation>, Param3: ::windows::core::IntoParam<'a, IWMDMProgress>>(&self, fumode: u32, pwszfile: Param1, poperation: Param2, pprogress: Param3) -> ::windows::core::Result<IWMDMStorage> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.base.Insert)(::core::mem::transmute_copy(self), ::core::mem::transmute(fumode), pwszfile.into_param().abi(), poperation.into_param().abi(), pprogress.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<IWMDMStorage>(result__)
    }
    #[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
    pub unsafe fn Delete<'a, Param1: ::windows::core::IntoParam<'a, IWMDMProgress>>(&self, fumode: u32, pprogress: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.base.Delete)(::core::mem::transmute_copy(self), ::core::mem::transmute(fumode), pprogress.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
    pub unsafe fn Rename<'a, Param1: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>, Param2: ::windows::core::IntoParam<'a, IWMDMProgress>>(&self, fumode: u32, pwsznewname: Param1, pprogress: Param2) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.base.Rename)(::core::mem::transmute_copy(self), ::core::mem::transmute(fumode), pwsznewname.into_param().abi(), pprogress.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
    pub unsafe fn Read<'a, Param1: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>, Param2: ::windows::core::IntoParam<'a, IWMDMProgress>, Param3: ::windows::core::IntoParam<'a, IWMDMOperation>>(&self, fumode: u32, pwszfile: Param1, pprogress: Param2, poperation: Param3) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.base.Read)(::core::mem::transmute_copy(self), ::core::mem::transmute(fumode), pwszfile.into_param().abi(), pprogress.into_param().abi(), poperation.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
    pub unsafe fn Move<'a, Param1: ::windows::core::IntoParam<'a, IWMDMStorage>, Param2: ::windows::core::IntoParam<'a, IWMDMProgress>>(&self, fumode: u32, ptargetobject: Param1, pprogress: Param2) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.base.Move)(::core::mem::transmute_copy(self), ::core::mem::transmute(fumode), ptargetobject.into_param().abi(), pprogress.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
    pub unsafe fn Insert2<'a, Param1: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>, Param2: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>, Param3: ::windows::core::IntoParam<'a, IWMDMOperation>, Param4: ::windows::core::IntoParam<'a, IWMDMProgress>, Param5: ::windows::core::IntoParam<'a, ::windows::core::IUnknown>>(&self, fumode: u32, pwszfilesource: Param1, pwszfiledest: Param2, poperation: Param3, pprogress: Param4, punknown: Param5, ppnewobject: *mut ::core::option::Option<IWMDMStorage>) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.Insert2)(::core::mem::transmute_copy(self), ::core::mem::transmute(fumode), pwszfilesource.into_param().abi(), pwszfiledest.into_param().abi(), poperation.into_param().abi(), pprogress.into_param().abi(), punknown.into_param().abi(), ::core::mem::transmute(ppnewobject)).ok()
    }
    #[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
    pub unsafe fn Insert3<'a, Param2: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>, Param3: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>, Param4: ::windows::core::IntoParam<'a, IWMDMOperation>, Param5: ::windows::core::IntoParam<'a, IWMDMProgress>, Param6: ::windows::core::IntoParam<'a, IWMDMMetaData>, Param7: ::windows::core::IntoParam<'a, ::windows::core::IUnknown>>(&self, fumode: u32, futype: u32, pwszfilesource: Param2, pwszfiledest: Param3, poperation: Param4, pprogress: Param5, pmetadata: Param6, punknown: Param7, ppnewobject: *mut ::core::option::Option<IWMDMStorage>) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Insert3)(::core::mem::transmute_copy(self), ::core::mem::transmute(fumode), ::core::mem::transmute(futype), pwszfilesource.into_param().abi(), pwszfiledest.into_param().abi(), poperation.into_param().abi(), pprogress.into_param().abi(), pmetadata.into_param().abi(), punknown.into_param().abi(), ::core::mem::transmute(ppnewobject)).ok()
    }
}
impl ::core::convert::From<IWMDMStorageControl3> for ::windows::core::IUnknown {
    fn from(value: IWMDMStorageControl3) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWMDMStorageControl3> for ::windows::core::IUnknown {
    fn from(value: &IWMDMStorageControl3) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IWMDMStorageControl3 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IWMDMStorageControl3 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IWMDMStorageControl3> for IWMDMStorageControl {
    fn from(value: IWMDMStorageControl3) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWMDMStorageControl3> for IWMDMStorageControl {
    fn from(value: &IWMDMStorageControl3) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IWMDMStorageControl> for IWMDMStorageControl3 {
    fn into_param(self) -> ::windows::core::Param<'a, IWMDMStorageControl> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IWMDMStorageControl> for &'a IWMDMStorageControl3 {
    fn into_param(self) -> ::windows::core::Param<'a, IWMDMStorageControl> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IWMDMStorageControl3> for IWMDMStorageControl2 {
    fn from(value: IWMDMStorageControl3) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWMDMStorageControl3> for IWMDMStorageControl2 {
    fn from(value: &IWMDMStorageControl3) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IWMDMStorageControl2> for IWMDMStorageControl3 {
    fn into_param(self) -> ::windows::core::Param<'a, IWMDMStorageControl2> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IWMDMStorageControl2> for &'a IWMDMStorageControl3 {
    fn into_param(self) -> ::windows::core::Param<'a, IWMDMStorageControl2> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IWMDMStorageControl3 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
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
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb3266365_d4f3_4696_8d53_bd27ec60993a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMDMStorageControl3_Vtbl {
    pub base: IWMDMStorageControl2_Vtbl,
    pub Insert3: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, fumode: u32, futype: u32, pwszfilesource: ::windows::core::PCWSTR, pwszfiledest: ::windows::core::PCWSTR, poperation: ::windows::core::RawPtr, pprogress: ::windows::core::RawPtr, pmetadata: ::windows::core::RawPtr, punknown: *mut ::core::ffi::c_void, ppnewobject: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
#[repr(transparent)]
pub struct IWMDMStorageGlobals(::windows::core::IUnknown);
impl IWMDMStorageGlobals {
    #[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
    pub unsafe fn GetCapabilities(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetCapabilities)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
    pub unsafe fn GetSerialNumber(&self, pserialnum: *mut WMDMID, abmac: *mut u8) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetSerialNumber)(::core::mem::transmute_copy(self), ::core::mem::transmute(pserialnum), ::core::mem::transmute(abmac)).ok()
    }
    #[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
    pub unsafe fn GetTotalSize(&self, pdwtotalsizelow: *mut u32, pdwtotalsizehigh: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetTotalSize)(::core::mem::transmute_copy(self), ::core::mem::transmute(pdwtotalsizelow), ::core::mem::transmute(pdwtotalsizehigh)).ok()
    }
    #[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
    pub unsafe fn GetTotalFree(&self, pdwfreelow: *mut u32, pdwfreehigh: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetTotalFree)(::core::mem::transmute_copy(self), ::core::mem::transmute(pdwfreelow), ::core::mem::transmute(pdwfreehigh)).ok()
    }
    #[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
    pub unsafe fn GetTotalBad(&self, pdwbadlow: *mut u32, pdwbadhigh: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetTotalBad)(::core::mem::transmute_copy(self), ::core::mem::transmute(pdwbadlow), ::core::mem::transmute(pdwbadhigh)).ok()
    }
    #[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
    pub unsafe fn GetStatus(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetStatus)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
    pub unsafe fn Initialize<'a, Param1: ::windows::core::IntoParam<'a, IWMDMProgress>>(&self, fumode: u32, pprogress: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Initialize)(::core::mem::transmute_copy(self), ::core::mem::transmute(fumode), pprogress.into_param().abi()).ok()
    }
}
impl ::core::convert::From<IWMDMStorageGlobals> for ::windows::core::IUnknown {
    fn from(value: IWMDMStorageGlobals) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWMDMStorageGlobals> for ::windows::core::IUnknown {
    fn from(value: &IWMDMStorageGlobals) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IWMDMStorageGlobals {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IWMDMStorageGlobals {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IWMDMStorageGlobals {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
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
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1dcb3a07_33ed_11d3_8470_00c04f79dbc0);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMDMStorageGlobals_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    pub GetCapabilities: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdwcapabilities: *mut u32) -> ::windows::core::HRESULT,
    pub GetSerialNumber: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pserialnum: *mut WMDMID, abmac: *mut u8) -> ::windows::core::HRESULT,
    pub GetTotalSize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdwtotalsizelow: *mut u32, pdwtotalsizehigh: *mut u32) -> ::windows::core::HRESULT,
    pub GetTotalFree: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdwfreelow: *mut u32, pdwfreehigh: *mut u32) -> ::windows::core::HRESULT,
    pub GetTotalBad: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdwbadlow: *mut u32, pdwbadhigh: *mut u32) -> ::windows::core::HRESULT,
    pub GetStatus: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdwstatus: *mut u32) -> ::windows::core::HRESULT,
    pub Initialize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, fumode: u32, pprogress: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
#[repr(transparent)]
pub struct IWMDeviceManager(::windows::core::IUnknown);
impl IWMDeviceManager {
    #[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
    pub unsafe fn GetRevision(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetRevision)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
    pub unsafe fn GetDeviceCount(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetDeviceCount)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
    pub unsafe fn EnumDevices(&self) -> ::windows::core::Result<IWMDMEnumDevice> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).EnumDevices)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IWMDMEnumDevice>(result__)
    }
}
impl ::core::convert::From<IWMDeviceManager> for ::windows::core::IUnknown {
    fn from(value: IWMDeviceManager) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWMDeviceManager> for ::windows::core::IUnknown {
    fn from(value: &IWMDeviceManager) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IWMDeviceManager {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IWMDeviceManager {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IWMDeviceManager {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
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
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1dcb3a00_33ed_11d3_8470_00c04f79dbc0);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMDeviceManager_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    pub GetRevision: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdwrevision: *mut u32) -> ::windows::core::HRESULT,
    pub GetDeviceCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdwcount: *mut u32) -> ::windows::core::HRESULT,
    pub EnumDevices: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppenumdevice: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
#[repr(transparent)]
pub struct IWMDeviceManager2(::windows::core::IUnknown);
impl IWMDeviceManager2 {
    #[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
    pub unsafe fn GetRevision(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.GetRevision)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
    pub unsafe fn GetDeviceCount(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.GetDeviceCount)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
    pub unsafe fn EnumDevices(&self) -> ::windows::core::Result<IWMDMEnumDevice> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.EnumDevices)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IWMDMEnumDevice>(result__)
    }
    #[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
    pub unsafe fn GetDeviceFromCanonicalName<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(&self, pwszcanonicalname: Param0) -> ::windows::core::Result<IWMDMDevice> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetDeviceFromCanonicalName)(::core::mem::transmute_copy(self), pwszcanonicalname.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<IWMDMDevice>(result__)
    }
    #[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
    pub unsafe fn EnumDevices2(&self) -> ::windows::core::Result<IWMDMEnumDevice> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).EnumDevices2)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IWMDMEnumDevice>(result__)
    }
    #[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
    pub unsafe fn Reinitialize(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Reinitialize)(::core::mem::transmute_copy(self)).ok()
    }
}
impl ::core::convert::From<IWMDeviceManager2> for ::windows::core::IUnknown {
    fn from(value: IWMDeviceManager2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWMDeviceManager2> for ::windows::core::IUnknown {
    fn from(value: &IWMDeviceManager2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IWMDeviceManager2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IWMDeviceManager2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IWMDeviceManager2> for IWMDeviceManager {
    fn from(value: IWMDeviceManager2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWMDeviceManager2> for IWMDeviceManager {
    fn from(value: &IWMDeviceManager2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IWMDeviceManager> for IWMDeviceManager2 {
    fn into_param(self) -> ::windows::core::Param<'a, IWMDeviceManager> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IWMDeviceManager> for &'a IWMDeviceManager2 {
    fn into_param(self) -> ::windows::core::Param<'a, IWMDeviceManager> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IWMDeviceManager2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
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
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x923e5249_8731_4c5b_9b1c_b8b60b6e46af);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMDeviceManager2_Vtbl {
    pub base: IWMDeviceManager_Vtbl,
    pub GetDeviceFromCanonicalName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwszcanonicalname: ::windows::core::PCWSTR, ppdevice: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub EnumDevices2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppenumdevice: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub Reinitialize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
#[repr(transparent)]
pub struct IWMDeviceManager3(::windows::core::IUnknown);
impl IWMDeviceManager3 {
    #[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
    pub unsafe fn GetRevision(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.base.GetRevision)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
    pub unsafe fn GetDeviceCount(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.base.GetDeviceCount)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
    pub unsafe fn EnumDevices(&self) -> ::windows::core::Result<IWMDMEnumDevice> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.base.EnumDevices)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IWMDMEnumDevice>(result__)
    }
    #[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
    pub unsafe fn GetDeviceFromCanonicalName<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(&self, pwszcanonicalname: Param0) -> ::windows::core::Result<IWMDMDevice> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.GetDeviceFromCanonicalName)(::core::mem::transmute_copy(self), pwszcanonicalname.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<IWMDMDevice>(result__)
    }
    #[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
    pub unsafe fn EnumDevices2(&self) -> ::windows::core::Result<IWMDMEnumDevice> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.EnumDevices2)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IWMDMEnumDevice>(result__)
    }
    #[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
    pub unsafe fn Reinitialize(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.Reinitialize)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
    pub unsafe fn SetDeviceEnumPreference(&self, dwenumpref: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetDeviceEnumPreference)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwenumpref)).ok()
    }
}
impl ::core::convert::From<IWMDeviceManager3> for ::windows::core::IUnknown {
    fn from(value: IWMDeviceManager3) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWMDeviceManager3> for ::windows::core::IUnknown {
    fn from(value: &IWMDeviceManager3) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IWMDeviceManager3 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IWMDeviceManager3 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IWMDeviceManager3> for IWMDeviceManager {
    fn from(value: IWMDeviceManager3) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWMDeviceManager3> for IWMDeviceManager {
    fn from(value: &IWMDeviceManager3) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IWMDeviceManager> for IWMDeviceManager3 {
    fn into_param(self) -> ::windows::core::Param<'a, IWMDeviceManager> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IWMDeviceManager> for &'a IWMDeviceManager3 {
    fn into_param(self) -> ::windows::core::Param<'a, IWMDeviceManager> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IWMDeviceManager3> for IWMDeviceManager2 {
    fn from(value: IWMDeviceManager3) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWMDeviceManager3> for IWMDeviceManager2 {
    fn from(value: &IWMDeviceManager3) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IWMDeviceManager2> for IWMDeviceManager3 {
    fn into_param(self) -> ::windows::core::Param<'a, IWMDeviceManager2> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IWMDeviceManager2> for &'a IWMDeviceManager3 {
    fn into_param(self) -> ::windows::core::Param<'a, IWMDeviceManager2> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IWMDeviceManager3 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
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
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xaf185c41_100d_46ed_be2e_9ce8c44594ef);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMDeviceManager3_Vtbl {
    pub base: IWMDeviceManager2_Vtbl,
    pub SetDeviceEnumPreference: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwenumpref: u32) -> ::windows::core::HRESULT,
}
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
unsafe impl ::windows::core::Abi for MTP_COMMAND_DATA_IN {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for MTP_COMMAND_DATA_IN {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MTP_COMMAND_DATA_IN>()) == 0 }
    }
}
impl ::core::cmp::Eq for MTP_COMMAND_DATA_IN {}
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
unsafe impl ::windows::core::Abi for MTP_COMMAND_DATA_OUT {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for MTP_COMMAND_DATA_OUT {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MTP_COMMAND_DATA_OUT>()) == 0 }
    }
}
impl ::core::cmp::Eq for MTP_COMMAND_DATA_OUT {}
impl ::core::default::Default for MTP_COMMAND_DATA_OUT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
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
pub const MediaDevMgr: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x25baad81_3560_11d3_8471_00c04f79dbc0);
pub const MediaDevMgrClassFactory: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x50040c1d_bdbf_4924_b873_f14d6c5bfd66);
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
unsafe impl ::windows::core::Abi for OPAQUECOMMAND {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for OPAQUECOMMAND {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<OPAQUECOMMAND>()) == 0 }
    }
}
impl ::core::cmp::Eq for OPAQUECOMMAND {}
impl ::core::default::Default for OPAQUECOMMAND {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
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
pub const SCP_EVENTID_ACQSECURECLOCK: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x86248cc9_4a59_43e2_9146_48a7f3f4140c);
pub const SCP_EVENTID_DRMINFO: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x213dd287_41d2_432b_9e3f_3b4f7b3581dd);
pub const SCP_EVENTID_NEEDTOINDIV: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x87a507c7_b469_4386_b976_d5d1ce538a6f);
pub const SCP_PARAMID_DRMVERSION: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x41d0155d_7cc7_4217_ada9_005074624da4);
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
unsafe impl ::windows::core::Abi for WMDMDATETIME {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WMDMDATETIME {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WMDMDATETIME>()) == 0 }
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
unsafe impl ::windows::core::Abi for WMDMDetermineMaxPropStringLen {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WMDMDetermineMaxPropStringLen {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WMDMDetermineMaxPropStringLen>()) == 0 }
    }
}
impl ::core::cmp::Eq for WMDMDetermineMaxPropStringLen {}
impl ::core::default::Default for WMDMDetermineMaxPropStringLen {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
pub const WMDMDevice: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x807b3cdf_357a_11d3_8471_00c04f79dbc0);
pub const WMDMDeviceEnum: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x430e35af_3971_11d3_8474_00c04f79dbc0);
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
unsafe impl ::windows::core::Abi for WMDMID {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WMDMID {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WMDMID>()) == 0 }
    }
}
impl ::core::cmp::Eq for WMDMID {}
impl ::core::default::Default for WMDMID {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDMID_LENGTH: u32 = 128u32;
pub const WMDMLogger: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x110a3202_5a79_11d3_8d78_444553540000);
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
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
unsafe impl ::windows::core::Abi for WMDMMessage {
    type Abi = Self;
}
impl ::core::fmt::Debug for WMDMMessage {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WMDMMessage").field(&self.0).finish()
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
unsafe impl ::windows::core::Abi for WMDMMetadataView {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WMDMMetadataView {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WMDMMetadataView>()) == 0 }
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
unsafe impl ::windows::core::Abi for WMDMRIGHTS {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WMDMRIGHTS {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WMDMRIGHTS>()) == 0 }
    }
}
impl ::core::cmp::Eq for WMDMRIGHTS {}
impl ::core::default::Default for WMDMRIGHTS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
pub const WMDMStorage: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x807b3ce0_357a_11d3_8471_00c04f79dbc0);
pub const WMDMStorageEnum: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xeb401a3b_3af7_11d3_8474_00c04f79dbc0);
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
pub const WMDM_DEVICE_PROTOCOL_MSC: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa4d2c26c_a881_44bb_bd5d_1f703c71f7a9);
pub const WMDM_DEVICE_PROTOCOL_MTP: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x979e54e5_0afc_4604_8d93_dc798a4bcf45);
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
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
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
unsafe impl ::windows::core::Abi for WMDM_ENUM_PROP_VALID_VALUES_FORM {
    type Abi = Self;
}
impl ::core::fmt::Debug for WMDM_ENUM_PROP_VALID_VALUES_FORM {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WMDM_ENUM_PROP_VALID_VALUES_FORM").field(&self.0).finish()
    }
}
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
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
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
unsafe impl ::windows::core::Abi for WMDM_FIND_SCOPE {
    type Abi = Self;
}
impl ::core::fmt::Debug for WMDM_FIND_SCOPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WMDM_FIND_SCOPE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
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
unsafe impl ::windows::core::Abi for WMDM_FORMATCODE {
    type Abi = Self;
}
impl ::core::fmt::Debug for WMDM_FORMATCODE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WMDM_FORMATCODE").field(&self.0).finish()
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
unsafe impl ::windows::core::Abi for WMDM_FORMAT_CAPABILITY {
    type Abi = Self;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
impl ::core::cmp::PartialEq for WMDM_FORMAT_CAPABILITY {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WMDM_FORMAT_CAPABILITY>()) == 0 }
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
unsafe impl ::windows::core::Abi for WMDM_PROP_CONFIG {
    type Abi = Self;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
impl ::core::cmp::PartialEq for WMDM_PROP_CONFIG {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WMDM_PROP_CONFIG>()) == 0 }
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
        Self { pwszPropName: self.pwszPropName, ValidValuesForm: self.ValidValuesForm, ValidValues: self.ValidValues.clone() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
unsafe impl ::windows::core::Abi for WMDM_PROP_DESC {
    type Abi = ::core::mem::ManuallyDrop<Self>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
impl ::core::cmp::PartialEq for WMDM_PROP_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.pwszPropName == other.pwszPropName && self.ValidValuesForm == other.ValidValuesForm && self.ValidValues == other.ValidValues
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
impl ::core::cmp::Eq for WMDM_PROP_DESC {}
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
    pub ValidValuesRange: ::core::mem::ManuallyDrop<WMDM_PROP_VALUES_RANGE>,
    pub EnumeratedValidValues: WMDM_PROP_VALUES_ENUM,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
impl ::core::clone::Clone for WMDM_PROP_DESC_0 {
    fn clone(&self) -> Self {
        unsafe { ::core::mem::transmute_copy(self) }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
unsafe impl ::windows::core::Abi for WMDM_PROP_DESC_0 {
    type Abi = ::core::mem::ManuallyDrop<Self>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
impl ::core::cmp::PartialEq for WMDM_PROP_DESC_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WMDM_PROP_DESC_0>()) == 0 }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
impl ::core::cmp::Eq for WMDM_PROP_DESC_0 {}
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
unsafe impl ::windows::core::Abi for WMDM_PROP_VALUES_ENUM {
    type Abi = Self;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
impl ::core::cmp::PartialEq for WMDM_PROP_VALUES_ENUM {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WMDM_PROP_VALUES_ENUM>()) == 0 }
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
        Self { rangeMin: self.rangeMin.clone(), rangeMax: self.rangeMax.clone(), rangeStep: self.rangeStep.clone() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
unsafe impl ::windows::core::Abi for WMDM_PROP_VALUES_RANGE {
    type Abi = ::core::mem::ManuallyDrop<Self>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
impl ::core::cmp::PartialEq for WMDM_PROP_VALUES_RANGE {
    fn eq(&self, other: &Self) -> bool {
        self.rangeMin == other.rangeMin && self.rangeMax == other.rangeMax && self.rangeStep == other.rangeStep
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
impl ::core::cmp::Eq for WMDM_PROP_VALUES_RANGE {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
impl ::core::default::Default for WMDM_PROP_VALUES_RANGE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
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
pub const WMDM_SERVICE_PROVIDER_VENDOR_MICROSOFT: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7de8686d_78ee_43ea_a496_c625ac91cc5d);
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
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
unsafe impl ::windows::core::Abi for WMDM_SESSION_TYPE {
    type Abi = Self;
}
impl ::core::fmt::Debug for WMDM_SESSION_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WMDM_SESSION_TYPE").field(&self.0).finish()
    }
}
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
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
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
unsafe impl ::windows::core::Abi for WMDM_STORAGE_ENUM_MODE {
    type Abi = Self;
}
impl ::core::fmt::Debug for WMDM_STORAGE_ENUM_MODE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WMDM_STORAGE_ENUM_MODE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_STORAGE_IS_DEFAULT: u32 = 134217728u32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_S_NOT_ALL_PROPERTIES_APPLIED: i32 = 282625i32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_S_NOT_ALL_PROPERTIES_RETRIEVED: i32 = 282626i32;
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
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
unsafe impl ::windows::core::Abi for WMDM_TAG_DATATYPE {
    type Abi = Self;
}
impl ::core::fmt::Debug for WMDM_TAG_DATATYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WMDM_TAG_DATATYPE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const WMDM_WMDM_REVOKED: u32 = 1u32;
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
unsafe impl ::windows::core::Abi for WMFILECAPABILITIES {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WMFILECAPABILITIES {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WMFILECAPABILITIES>()) == 0 }
    }
}
impl ::core::cmp::Eq for WMFILECAPABILITIES {}
impl ::core::default::Default for WMFILECAPABILITIES {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
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
impl ::core::marker::Copy for _BITMAPINFOHEADER {}
impl ::core::clone::Clone for _BITMAPINFOHEADER {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for _BITMAPINFOHEADER {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("_BITMAPINFOHEADER").field("biSize", &self.biSize).field("biWidth", &self.biWidth).field("biHeight", &self.biHeight).field("biPlanes", &self.biPlanes).field("biBitCount", &self.biBitCount).field("biCompression", &self.biCompression).field("biSizeImage", &self.biSizeImage).field("biXPelsPerMeter", &self.biXPelsPerMeter).field("biYPelsPerMeter", &self.biYPelsPerMeter).field("biClrUsed", &self.biClrUsed).field("biClrImportant", &self.biClrImportant).finish()
    }
}
unsafe impl ::windows::core::Abi for _BITMAPINFOHEADER {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for _BITMAPINFOHEADER {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<_BITMAPINFOHEADER>()) == 0 }
    }
}
impl ::core::cmp::Eq for _BITMAPINFOHEADER {}
impl ::core::default::Default for _BITMAPINFOHEADER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct _VIDEOINFOHEADER {
    pub rcSource: super::super::Foundation::RECT,
    pub rcTarget: super::super::Foundation::RECT,
    pub dwBitRate: u32,
    pub dwBitErrorRate: u32,
    pub AvgTimePerFrame: i64,
    pub bmiHeader: _BITMAPINFOHEADER,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for _VIDEOINFOHEADER {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for _VIDEOINFOHEADER {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for _VIDEOINFOHEADER {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("_VIDEOINFOHEADER").field("rcSource", &self.rcSource).field("rcTarget", &self.rcTarget).field("dwBitRate", &self.dwBitRate).field("dwBitErrorRate", &self.dwBitErrorRate).field("AvgTimePerFrame", &self.AvgTimePerFrame).field("bmiHeader", &self.bmiHeader).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for _VIDEOINFOHEADER {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for _VIDEOINFOHEADER {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<_VIDEOINFOHEADER>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for _VIDEOINFOHEADER {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for _VIDEOINFOHEADER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub struct _WAVEFORMATEX {
    pub wFormatTag: u16,
    pub nChannels: u16,
    pub nSamplesPerSec: u32,
    pub nAvgBytesPerSec: u32,
    pub nBlockAlign: u16,
    pub wBitsPerSample: u16,
    pub cbSize: u16,
}
impl ::core::marker::Copy for _WAVEFORMATEX {}
impl ::core::clone::Clone for _WAVEFORMATEX {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for _WAVEFORMATEX {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("_WAVEFORMATEX").field("wFormatTag", &self.wFormatTag).field("nChannels", &self.nChannels).field("nSamplesPerSec", &self.nSamplesPerSec).field("nAvgBytesPerSec", &self.nAvgBytesPerSec).field("nBlockAlign", &self.nBlockAlign).field("wBitsPerSample", &self.wBitsPerSample).field("cbSize", &self.cbSize).finish()
    }
}
unsafe impl ::windows::core::Abi for _WAVEFORMATEX {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for _WAVEFORMATEX {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<_WAVEFORMATEX>()) == 0 }
    }
}
impl ::core::cmp::Eq for _WAVEFORMATEX {}
impl ::core::default::Default for _WAVEFORMATEX {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct __MACINFO {
    pub fUsed: super::super::Foundation::BOOL,
    pub abMacState: [u8; 36],
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for __MACINFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for __MACINFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for __MACINFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("__MACINFO").field("fUsed", &self.fUsed).field("abMacState", &self.abMacState).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for __MACINFO {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for __MACINFO {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<__MACINFO>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for __MACINFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for __MACINFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const g_wszAudioWAVECodec: &'static str = "WMDM/AudioWAVECodec";
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const g_wszVideoFourCCCodec: &'static str = "WMDM/VideoFourCCCodec";
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const g_wszWMDMAlbumArt: &'static str = "WMDM/AlbumArt";
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const g_wszWMDMAlbumArtist: &'static str = "WMDM/AlbumArtist";
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const g_wszWMDMAlbumCoverData: &'static str = "WMDM/AlbumCoverData";
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const g_wszWMDMAlbumCoverDuration: &'static str = "WMDM/AlbumCoverDuration";
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const g_wszWMDMAlbumCoverFormat: &'static str = "WMDM/AlbumCoverFormat";
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const g_wszWMDMAlbumCoverHeight: &'static str = "WMDM/AlbumCoverHeight";
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const g_wszWMDMAlbumCoverSize: &'static str = "WMDM/AlbumCoverSize";
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const g_wszWMDMAlbumCoverWidth: &'static str = "WMDM/AlbumCoverWidth";
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const g_wszWMDMAlbumTitle: &'static str = "WMDM/AlbumTitle";
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const g_wszWMDMAudioBitDepth: &'static str = "WMDM/AudioBitDepth";
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const g_wszWMDMAuthor: &'static str = "WMDM/Author";
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const g_wszWMDMAuthorDate: &'static str = "WMDM/AuthorDate";
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const g_wszWMDMBitRateType: &'static str = "WMDM/BitRateType";
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const g_wszWMDMBitrate: &'static str = "WMDM/Bitrate";
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const g_wszWMDMBlockAlignment: &'static str = "WMDM/BlockAlignment";
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const g_wszWMDMBufferSize: &'static str = "WMDM/BufferSize";
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const g_wszWMDMBuyNow: &'static str = "WMDM/BuyNow";
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const g_wszWMDMByteBookmark: &'static str = "WMDM/ByteBookmark";
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const g_wszWMDMCategory: &'static str = "WMDM/Category";
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const g_wszWMDMCodec: &'static str = "WMDM/Codec";
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const g_wszWMDMCollectionID: &'static str = "WMDM/CollectionID";
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const g_wszWMDMComposer: &'static str = "WMDM/Composer";
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const g_wszWMDMDRMId: &'static str = "WMDM/DRMId";
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const g_wszWMDMDataLength: &'static str = "WMDM/DataLength";
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const g_wszWMDMDataOffset: &'static str = "WMDM/DataOffset";
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const g_wszWMDMDataUnits: &'static str = "WMDM/DataUnits";
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const g_wszWMDMDescription: &'static str = "WMDM/Description";
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const g_wszWMDMDestinationURL: &'static str = "WMDM/DestinationURL";
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const g_wszWMDMDeviceFirmwareVersion: &'static str = "WMDM/DeviceFirmwareVersion";
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const g_wszWMDMDeviceFriendlyName: &'static str = "WMDM/DeviceFriendlyName";
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const g_wszWMDMDeviceModelName: &'static str = "WMDM/DeviceModelName";
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const g_wszWMDMDevicePlayCount: &'static str = "WMDM/DevicePlayCount";
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const g_wszWMDMDeviceProtocol: &'static str = "WMDM/DeviceProtocol";
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const g_wszWMDMDeviceRevocationInfo: &'static str = "WMDM/DeviceRevocationInfo";
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const g_wszWMDMDeviceServiceProviderVendor: &'static str = "WMDM/DeviceServiceProviderVendor";
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const g_wszWMDMDeviceVendorExtension: &'static str = "WMDM/DeviceVendorExtension";
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const g_wszWMDMDuration: &'static str = "WMDM/Duration";
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const g_wszWMDMEditor: &'static str = "WMDM/Editor";
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const g_wszWMDMEncodingProfile: &'static str = "WMDM/EncodingProfile";
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const g_wszWMDMFileAttributes: &'static str = "WMDM/FileAttributes";
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const g_wszWMDMFileCreationDate: &'static str = "WMDM/FileCreationDate";
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const g_wszWMDMFileName: &'static str = "WMDM/FileName";
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const g_wszWMDMFileSize: &'static str = "WMDM/FileSize";
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const g_wszWMDMFormatCode: &'static str = "WMDM/FormatCode";
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const g_wszWMDMFormatsSupported: &'static str = "WMDM/FormatsSupported";
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const g_wszWMDMFormatsSupportedAreOrdered: &'static str = "WMDM/FormatsSupportedAreOrdered";
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const g_wszWMDMFrameRate: &'static str = "WMDM/FrameRate";
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const g_wszWMDMGenre: &'static str = "WMDM/Genre";
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const g_wszWMDMHeight: &'static str = "WMDM/Height";
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const g_wszWMDMIsProtected: &'static str = "WMDM/IsProtected";
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const g_wszWMDMIsRepeat: &'static str = "WMDM/IsRepeat";
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const g_wszWMDMKeyFrameDistance: &'static str = "WMDM/KeyFrameDistance";
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const g_wszWMDMLastModifiedDate: &'static str = "WMDM/LastModifiedDate";
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const g_wszWMDMMediaClassSecondaryID: &'static str = "WMDM/MediaClassSecondaryID";
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const g_wszWMDMMediaCredits: &'static str = "WMDM/MediaCredits";
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const g_wszWMDMMediaGuid: &'static str = "WMDM/MediaGuid";
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const g_wszWMDMMediaOriginalBroadcastDateTime: &'static str = "WMDM/MediaOriginalBroadcastDateTime";
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const g_wszWMDMMediaOriginalChannel: &'static str = "WMDM/MediaOriginalChannel";
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const g_wszWMDMMediaStationName: &'static str = "WMDM/MediaStationName";
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const g_wszWMDMMetaGenre: &'static str = "WMDM/MetaGenre";
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const g_wszWMDMNonConsumable: &'static str = "WMDM/NonConsumable";
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const g_wszWMDMNumChannels: &'static str = "WMDM/NumChannels";
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const g_wszWMDMObjectBookmark: &'static str = "WMDM/ObjectBookmark";
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const g_wszWMDMOwner: &'static str = "WMDM/Owner";
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const g_wszWMDMParentalRating: &'static str = "WMDM/ParentalRating";
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const g_wszWMDMPersistentUniqueID: &'static str = "WMDM/PersistentUniqueID";
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const g_wszWMDMPlayCount: &'static str = "WMDM/PlayCount";
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const g_wszWMDMProviderCopyright: &'static str = "WMDM/ProviderCopyright";
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const g_wszWMDMQualitySetting: &'static str = "WMDM/QualitySetting";
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const g_wszWMDMSampleRate: &'static str = "WMDM/SampleRate";
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const g_wszWMDMScanType: &'static str = "WMDM/ScanType";
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const g_wszWMDMSourceURL: &'static str = "WMDM/SourceURL";
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const g_wszWMDMSubTitle: &'static str = "WMDM/SubTitle";
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const g_wszWMDMSubTitleDescription: &'static str = "WMDM/SubTitleDescription";
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const g_wszWMDMSupportedDeviceProperties: &'static str = "WMDM/SupportedDeviceProperties";
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const g_wszWMDMSyncID: &'static str = "WMDM/SyncID";
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const g_wszWMDMSyncRelationshipID: &'static str = "WMDM/SyncRelationshipID";
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const g_wszWMDMSyncTime: &'static str = "WMDM/SyncTime";
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const g_wszWMDMTimeBookmark: &'static str = "WMDM/TimeBookmark";
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const g_wszWMDMTimeToLive: &'static str = "WMDM/TimeToLive";
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const g_wszWMDMTitle: &'static str = "WMDM/Title";
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const g_wszWMDMTotalBitrate: &'static str = "WMDM/TotalBitrate";
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const g_wszWMDMTrack: &'static str = "WMDM/Track";
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const g_wszWMDMTrackMood: &'static str = "WMDM/TrackMood";
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const g_wszWMDMUserEffectiveRating: &'static str = "WMDM/UserEffectiveRating";
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const g_wszWMDMUserLastPlayTime: &'static str = "WMDM/UserLastPlayTime";
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const g_wszWMDMUserRating: &'static str = "WMDM/UserRating";
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const g_wszWMDMUserRatingOnDevice: &'static str = "WMDM/UserRatingOnDevice";
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const g_wszWMDMVideoBitrate: &'static str = "WMDM/VideoBitrate";
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const g_wszWMDMWebmaster: &'static str = "WMDM/Webmaster";
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const g_wszWMDMWidth: &'static str = "WMDM/Width";
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const g_wszWMDMYear: &'static str = "WMDM/Year";
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const g_wszWMDMediaClassPrimaryID: &'static str = "WMDM/MediaClassPrimaryID";
#[doc = "*Required features: `\"Win32_Media_DeviceManager\"`*"]
pub const g_wszWPDPassthroughPropertyValues: &'static str = "WPD/PassthroughPropertyValues";
#[cfg(feature = "implement")]
::core::include!("impl.rs");
