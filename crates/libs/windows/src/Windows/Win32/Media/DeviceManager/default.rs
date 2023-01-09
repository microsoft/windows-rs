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
impl IMDSPDevice2 {
    pub unsafe fn GetName(&self, pwszname: &mut [u16]) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetName)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(pwszname.as_ptr()), pwszname.len() as _).ok()
    }
    pub unsafe fn GetManufacturer(&self, pwszname: &mut [u16]) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetManufacturer)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(pwszname.as_ptr()), pwszname.len() as _).ok()
    }
    pub unsafe fn GetVersion(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetVersion)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetType(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetType)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetSerialNumber(&self, pserialnumber: *mut WMDMID, abmac: *mut u8) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetSerialNumber)(::windows::core::Vtable::as_raw(self), pserialnumber, abmac).ok()
    }
    pub unsafe fn GetPowerSource(&self, pdwpowersource: *mut u32, pdwpercentremaining: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetPowerSource)(::windows::core::Vtable::as_raw(self), pdwpowersource, pdwpercentremaining).ok()
    }
    pub unsafe fn GetStatus(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetStatus)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetDeviceIcon(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetDeviceIcon)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn EnumStorage(&self) -> ::windows::core::Result<IMDSPEnumStorage> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.EnumStorage)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
    #[cfg(feature = "Win32_Media_Audio")]
    pub unsafe fn GetFormatSupport(&self, pformatex: *mut *mut super::Audio::WAVEFORMATEX, pnformatcount: *mut u32, pppwszmimetype: *mut *mut ::windows::core::PWSTR, pnmimetypecount: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetFormatSupport)(::windows::core::Vtable::as_raw(self), pformatex, pnformatcount, pppwszmimetype, pnmimetypecount).ok()
    }
    pub unsafe fn SendOpaqueCommand(&self, pcommand: *mut OPAQUECOMMAND) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SendOpaqueCommand)(::windows::core::Vtable::as_raw(self), pcommand).ok()
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
impl IMDSPDevice3 {
    pub unsafe fn GetName(&self, pwszname: &mut [u16]) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.GetName)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(pwszname.as_ptr()), pwszname.len() as _).ok()
    }
    pub unsafe fn GetManufacturer(&self, pwszname: &mut [u16]) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.GetManufacturer)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(pwszname.as_ptr()), pwszname.len() as _).ok()
    }
    pub unsafe fn GetVersion(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.GetVersion)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetType(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.GetType)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetSerialNumber(&self, pserialnumber: *mut WMDMID, abmac: *mut u8) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.GetSerialNumber)(::windows::core::Vtable::as_raw(self), pserialnumber, abmac).ok()
    }
    pub unsafe fn GetPowerSource(&self, pdwpowersource: *mut u32, pdwpercentremaining: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.GetPowerSource)(::windows::core::Vtable::as_raw(self), pdwpowersource, pdwpercentremaining).ok()
    }
    pub unsafe fn GetStatus(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.GetStatus)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetDeviceIcon(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.GetDeviceIcon)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn EnumStorage(&self) -> ::windows::core::Result<IMDSPEnumStorage> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.EnumStorage)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
    #[cfg(feature = "Win32_Media_Audio")]
    pub unsafe fn GetFormatSupport(&self, pformatex: *mut *mut super::Audio::WAVEFORMATEX, pnformatcount: *mut u32, pppwszmimetype: *mut *mut ::windows::core::PWSTR, pnmimetypecount: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.GetFormatSupport)(::windows::core::Vtable::as_raw(self), pformatex, pnformatcount, pppwszmimetype, pnmimetypecount).ok()
    }
    pub unsafe fn SendOpaqueCommand(&self, pcommand: *mut OPAQUECOMMAND) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.SendOpaqueCommand)(::windows::core::Vtable::as_raw(self), pcommand).ok()
    }
    pub unsafe fn GetStorage<P0>(&self, pszstoragename: P0) -> ::windows::core::Result<IMDSPStorage>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetStorage)(::windows::core::Vtable::as_raw(self), pszstoragename.into().abi(), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`, `\"Win32_Media_Audio\"`, `\"Win32_Media_MediaFoundation\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_Media_Audio", feature = "Win32_Media_MediaFoundation"))]
    pub unsafe fn GetFormatSupport2(&self, dwflags: u32, ppaudioformatex: *mut *mut super::Audio::WAVEFORMATEX, pnaudioformatcount: *mut u32, ppvideoformatex: *mut *mut super::MediaFoundation::VIDEOINFOHEADER, pnvideoformatcount: *mut u32, ppfiletype: *mut *mut WMFILECAPABILITIES, pnfiletypecount: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetFormatSupport2)(::windows::core::Vtable::as_raw(self), dwflags, ppaudioformatex, pnaudioformatcount, ppvideoformatex, pnvideoformatcount, ppfiletype, pnfiletypecount).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Ole\"`*"]
    #[cfg(feature = "Win32_System_Ole")]
    pub unsafe fn GetSpecifyPropertyPages(&self, ppspecifyproppages: *mut ::core::option::Option<super::super::System::Ole::ISpecifyPropertyPages>, pppunknowns: *mut *mut ::core::option::Option<::windows::core::IUnknown>, pcunks: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetSpecifyPropertyPages)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(ppspecifyproppages), pppunknowns, pcunks).ok()
    }
    pub unsafe fn GetCanonicalName(&self, pwszpnpname: &mut [u16]) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetCanonicalName)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(pwszpnpname.as_ptr()), pwszpnpname.len() as _).ok()
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
impl IMDSPObject2 {
    pub unsafe fn Open(&self, fumode: u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.Open)(::windows::core::Vtable::as_raw(self), fumode).ok()
    }
    pub unsafe fn Read(&self, pdata: *mut u8, pdwsize: *mut u32, abmac: *mut u8) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.Read)(::windows::core::Vtable::as_raw(self), pdata, pdwsize, abmac).ok()
    }
    pub unsafe fn Write(&self, pdata: *const u8, pdwsize: *mut u32, abmac: *mut u8) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.Write)(::windows::core::Vtable::as_raw(self), pdata, pdwsize, abmac).ok()
    }
    pub unsafe fn Delete<P0>(&self, fumode: u32, pprogress: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IWMDMProgress>>,
    {
        (::windows::core::Vtable::vtable(self).base__.Delete)(::windows::core::Vtable::as_raw(self), fumode, pprogress.into().abi()).ok()
    }
    pub unsafe fn Seek(&self, fuflags: u32, dwoffset: u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.Seek)(::windows::core::Vtable::as_raw(self), fuflags, dwoffset).ok()
    }
    pub unsafe fn Rename<P0, P1>(&self, pwsznewname: P0, pprogress: P1) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
        P1: ::std::convert::Into<::windows::core::InParam<IWMDMProgress>>,
    {
        (::windows::core::Vtable::vtable(self).base__.Rename)(::windows::core::Vtable::as_raw(self), pwsznewname.into().abi(), pprogress.into().abi()).ok()
    }
    pub unsafe fn Move<P0, P1>(&self, fumode: u32, pprogress: P0, ptarget: P1) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IWMDMProgress>>,
        P1: ::std::convert::Into<::windows::core::InParam<IMDSPStorage>>,
    {
        (::windows::core::Vtable::vtable(self).base__.Move)(::windows::core::Vtable::as_raw(self), fumode, pprogress.into().abi(), ptarget.into().abi()).ok()
    }
    pub unsafe fn Close(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.Close)(::windows::core::Vtable::as_raw(self)).ok()
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
impl IMDSPStorage2 {
    #[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
    #[cfg(feature = "Win32_Media_Audio")]
    pub unsafe fn SetAttributes(&self, dwattributes: u32, pformat: ::core::option::Option<*const super::Audio::WAVEFORMATEX>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetAttributes)(::windows::core::Vtable::as_raw(self), dwattributes, ::core::mem::transmute(pformat.unwrap_or(::std::ptr::null()))).ok()
    }
    pub unsafe fn GetStorageGlobals(&self) -> ::windows::core::Result<IMDSPStorageGlobals> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetStorageGlobals)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
    #[cfg(feature = "Win32_Media_Audio")]
    pub unsafe fn GetAttributes(&self, pdwattributes: *mut u32, pformat: ::core::option::Option<*mut super::Audio::WAVEFORMATEX>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetAttributes)(::windows::core::Vtable::as_raw(self), pdwattributes, ::core::mem::transmute(pformat.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    pub unsafe fn GetName(&self, pwszname: &mut [u16]) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetName)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(pwszname.as_ptr()), pwszname.len() as _).ok()
    }
    pub unsafe fn GetDate(&self) -> ::windows::core::Result<WMDMDATETIME> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetDate)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetSize(&self, pdwsizelow: *mut u32, pdwsizehigh: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetSize)(::windows::core::Vtable::as_raw(self), pdwsizelow, pdwsizehigh).ok()
    }
    pub unsafe fn GetRights(&self, pprights: *mut *mut WMDMRIGHTS, pnrightscount: *mut u32, abmac: *mut u8) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetRights)(::windows::core::Vtable::as_raw(self), pprights, pnrightscount, abmac).ok()
    }
    #[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
    #[cfg(feature = "Win32_Media_Audio")]
    pub unsafe fn CreateStorage<P0>(&self, dwattributes: u32, pformat: ::core::option::Option<*const super::Audio::WAVEFORMATEX>, pwszname: P0) -> ::windows::core::Result<IMDSPStorage>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.CreateStorage)(::windows::core::Vtable::as_raw(self), dwattributes, ::core::mem::transmute(pformat.unwrap_or(::std::ptr::null())), pwszname.into().abi(), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn EnumStorage(&self) -> ::windows::core::Result<IMDSPEnumStorage> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.EnumStorage)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SendOpaqueCommand(&self, pcommand: *mut OPAQUECOMMAND) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SendOpaqueCommand)(::windows::core::Vtable::as_raw(self), pcommand).ok()
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
impl IMDSPStorage3 {
    #[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
    #[cfg(feature = "Win32_Media_Audio")]
    pub unsafe fn SetAttributes(&self, dwattributes: u32, pformat: ::core::option::Option<*const super::Audio::WAVEFORMATEX>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.SetAttributes)(::windows::core::Vtable::as_raw(self), dwattributes, ::core::mem::transmute(pformat.unwrap_or(::std::ptr::null()))).ok()
    }
    pub unsafe fn GetStorageGlobals(&self) -> ::windows::core::Result<IMDSPStorageGlobals> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.GetStorageGlobals)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
    #[cfg(feature = "Win32_Media_Audio")]
    pub unsafe fn GetAttributes(&self, pdwattributes: *mut u32, pformat: ::core::option::Option<*mut super::Audio::WAVEFORMATEX>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.GetAttributes)(::windows::core::Vtable::as_raw(self), pdwattributes, ::core::mem::transmute(pformat.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    pub unsafe fn GetName(&self, pwszname: &mut [u16]) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.GetName)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(pwszname.as_ptr()), pwszname.len() as _).ok()
    }
    pub unsafe fn GetDate(&self) -> ::windows::core::Result<WMDMDATETIME> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.GetDate)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetSize(&self, pdwsizelow: *mut u32, pdwsizehigh: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.GetSize)(::windows::core::Vtable::as_raw(self), pdwsizelow, pdwsizehigh).ok()
    }
    pub unsafe fn GetRights(&self, pprights: *mut *mut WMDMRIGHTS, pnrightscount: *mut u32, abmac: *mut u8) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.GetRights)(::windows::core::Vtable::as_raw(self), pprights, pnrightscount, abmac).ok()
    }
    #[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
    #[cfg(feature = "Win32_Media_Audio")]
    pub unsafe fn CreateStorage<P0>(&self, dwattributes: u32, pformat: ::core::option::Option<*const super::Audio::WAVEFORMATEX>, pwszname: P0) -> ::windows::core::Result<IMDSPStorage>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.CreateStorage)(::windows::core::Vtable::as_raw(self), dwattributes, ::core::mem::transmute(pformat.unwrap_or(::std::ptr::null())), pwszname.into().abi(), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn EnumStorage(&self) -> ::windows::core::Result<IMDSPEnumStorage> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.EnumStorage)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SendOpaqueCommand(&self, pcommand: *mut OPAQUECOMMAND) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.SendOpaqueCommand)(::windows::core::Vtable::as_raw(self), pcommand).ok()
    }
    pub unsafe fn GetStorage<P0>(&self, pszstoragename: P0) -> ::windows::core::Result<IMDSPStorage>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetStorage)(::windows::core::Vtable::as_raw(self), pszstoragename.into().abi(), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`, `\"Win32_Media_Audio\"`, `\"Win32_Media_MediaFoundation\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_Media_Audio", feature = "Win32_Media_MediaFoundation"))]
    pub unsafe fn CreateStorage2<P0>(&self, dwattributes: u32, dwattributesex: u32, paudioformat: ::core::option::Option<*const super::Audio::WAVEFORMATEX>, pvideoformat: ::core::option::Option<*const super::MediaFoundation::VIDEOINFOHEADER>, pwszname: P0, qwfilesize: u64) -> ::windows::core::Result<IMDSPStorage>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.CreateStorage2)(::windows::core::Vtable::as_raw(self), dwattributes, dwattributesex, ::core::mem::transmute(paudioformat.unwrap_or(::std::ptr::null())), ::core::mem::transmute(pvideoformat.unwrap_or(::std::ptr::null())), pwszname.into().abi(), qwfilesize, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`, `\"Win32_Media_Audio\"`, `\"Win32_Media_MediaFoundation\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_Media_Audio", feature = "Win32_Media_MediaFoundation"))]
    pub unsafe fn SetAttributes2(&self, dwattributes: u32, dwattributesex: u32, paudioformat: ::core::option::Option<*const super::Audio::WAVEFORMATEX>, pvideoformat: ::core::option::Option<*const super::MediaFoundation::VIDEOINFOHEADER>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetAttributes2)(::windows::core::Vtable::as_raw(self), dwattributes, dwattributesex, ::core::mem::transmute(paudioformat.unwrap_or(::std::ptr::null())), ::core::mem::transmute(pvideoformat.unwrap_or(::std::ptr::null()))).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`, `\"Win32_Media_Audio\"`, `\"Win32_Media_MediaFoundation\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_Media_Audio", feature = "Win32_Media_MediaFoundation"))]
    pub unsafe fn GetAttributes2(&self, pdwattributes: *mut u32, pdwattributesex: *mut u32, paudioformat: ::core::option::Option<*mut super::Audio::WAVEFORMATEX>, pvideoformat: ::core::option::Option<*mut super::MediaFoundation::VIDEOINFOHEADER>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetAttributes2)(::windows::core::Vtable::as_raw(self), pdwattributes, pdwattributesex, ::core::mem::transmute(paudioformat.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(pvideoformat.unwrap_or(::std::ptr::null_mut()))).ok()
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
impl IMDSPStorage4 {
    #[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
    #[cfg(feature = "Win32_Media_Audio")]
    pub unsafe fn SetAttributes(&self, dwattributes: u32, pformat: ::core::option::Option<*const super::Audio::WAVEFORMATEX>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.SetAttributes)(::windows::core::Vtable::as_raw(self), dwattributes, ::core::mem::transmute(pformat.unwrap_or(::std::ptr::null()))).ok()
    }
    pub unsafe fn GetStorageGlobals(&self) -> ::windows::core::Result<IMDSPStorageGlobals> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.GetStorageGlobals)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
    #[cfg(feature = "Win32_Media_Audio")]
    pub unsafe fn GetAttributes(&self, pdwattributes: *mut u32, pformat: ::core::option::Option<*mut super::Audio::WAVEFORMATEX>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.GetAttributes)(::windows::core::Vtable::as_raw(self), pdwattributes, ::core::mem::transmute(pformat.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    pub unsafe fn GetName(&self, pwszname: &mut [u16]) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.GetName)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(pwszname.as_ptr()), pwszname.len() as _).ok()
    }
    pub unsafe fn GetDate(&self) -> ::windows::core::Result<WMDMDATETIME> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.GetDate)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetSize(&self, pdwsizelow: *mut u32, pdwsizehigh: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.GetSize)(::windows::core::Vtable::as_raw(self), pdwsizelow, pdwsizehigh).ok()
    }
    pub unsafe fn GetRights(&self, pprights: *mut *mut WMDMRIGHTS, pnrightscount: *mut u32, abmac: *mut u8) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.GetRights)(::windows::core::Vtable::as_raw(self), pprights, pnrightscount, abmac).ok()
    }
    #[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
    #[cfg(feature = "Win32_Media_Audio")]
    pub unsafe fn CreateStorage<P0>(&self, dwattributes: u32, pformat: ::core::option::Option<*const super::Audio::WAVEFORMATEX>, pwszname: P0) -> ::windows::core::Result<IMDSPStorage>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.CreateStorage)(::windows::core::Vtable::as_raw(self), dwattributes, ::core::mem::transmute(pformat.unwrap_or(::std::ptr::null())), pwszname.into().abi(), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn EnumStorage(&self) -> ::windows::core::Result<IMDSPEnumStorage> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.EnumStorage)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SendOpaqueCommand(&self, pcommand: *mut OPAQUECOMMAND) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.SendOpaqueCommand)(::windows::core::Vtable::as_raw(self), pcommand).ok()
    }
    pub unsafe fn GetStorage<P0>(&self, pszstoragename: P0) -> ::windows::core::Result<IMDSPStorage>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.GetStorage)(::windows::core::Vtable::as_raw(self), pszstoragename.into().abi(), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`, `\"Win32_Media_Audio\"`, `\"Win32_Media_MediaFoundation\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_Media_Audio", feature = "Win32_Media_MediaFoundation"))]
    pub unsafe fn CreateStorage2<P0>(&self, dwattributes: u32, dwattributesex: u32, paudioformat: ::core::option::Option<*const super::Audio::WAVEFORMATEX>, pvideoformat: ::core::option::Option<*const super::MediaFoundation::VIDEOINFOHEADER>, pwszname: P0, qwfilesize: u64) -> ::windows::core::Result<IMDSPStorage>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.CreateStorage2)(::windows::core::Vtable::as_raw(self), dwattributes, dwattributesex, ::core::mem::transmute(paudioformat.unwrap_or(::std::ptr::null())), ::core::mem::transmute(pvideoformat.unwrap_or(::std::ptr::null())), pwszname.into().abi(), qwfilesize, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`, `\"Win32_Media_Audio\"`, `\"Win32_Media_MediaFoundation\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_Media_Audio", feature = "Win32_Media_MediaFoundation"))]
    pub unsafe fn SetAttributes2(&self, dwattributes: u32, dwattributesex: u32, paudioformat: ::core::option::Option<*const super::Audio::WAVEFORMATEX>, pvideoformat: ::core::option::Option<*const super::MediaFoundation::VIDEOINFOHEADER>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.SetAttributes2)(::windows::core::Vtable::as_raw(self), dwattributes, dwattributesex, ::core::mem::transmute(paudioformat.unwrap_or(::std::ptr::null())), ::core::mem::transmute(pvideoformat.unwrap_or(::std::ptr::null()))).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`, `\"Win32_Media_Audio\"`, `\"Win32_Media_MediaFoundation\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_Media_Audio", feature = "Win32_Media_MediaFoundation"))]
    pub unsafe fn GetAttributes2(&self, pdwattributes: *mut u32, pdwattributesex: *mut u32, paudioformat: ::core::option::Option<*mut super::Audio::WAVEFORMATEX>, pvideoformat: ::core::option::Option<*mut super::MediaFoundation::VIDEOINFOHEADER>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.GetAttributes2)(::windows::core::Vtable::as_raw(self), pdwattributes, pdwattributesex, ::core::mem::transmute(paudioformat.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(pvideoformat.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    pub unsafe fn GetMetadata<P0>(&self, pmetadata: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IWMDMMetaData>>,
    {
        (::windows::core::Vtable::vtable(self).base__.GetMetadata)(::windows::core::Vtable::as_raw(self), pmetadata.into().abi()).ok()
    }
    pub unsafe fn SetMetadata<P0>(&self, pmetadata: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IWMDMMetaData>>,
    {
        (::windows::core::Vtable::vtable(self).base__.SetMetadata)(::windows::core::Vtable::as_raw(self), pmetadata.into().abi()).ok()
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
impl IMDServiceProvider2 {
    pub unsafe fn GetDeviceCount(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetDeviceCount)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn EnumDevices(&self) -> ::windows::core::Result<IMDSPEnumDevice> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.EnumDevices)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
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
impl IMDServiceProvider3 {
    pub unsafe fn GetDeviceCount(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.GetDeviceCount)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn EnumDevices(&self) -> ::windows::core::Result<IMDSPEnumDevice> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.EnumDevices)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CreateDevice<P0>(&self, pwszdevicepath: P0, pdwcount: *mut u32, pppdevicearray: *mut *mut ::core::option::Option<IMDSPDevice>) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).base__.CreateDevice)(::windows::core::Vtable::as_raw(self), pwszdevicepath.into().abi(), pdwcount, pppdevicearray).ok()
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
impl ISCPSecureAuthenticate2 {
    pub unsafe fn GetSecureQuery(&self) -> ::windows::core::Result<ISCPSecureQuery> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetSecureQuery)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
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
impl ISCPSecureExchange2 {
    pub unsafe fn TransferContainerData(&self, pdata: &[u8], pfureadyflags: *mut u32, abmac: *mut u8) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.TransferContainerData)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(pdata.as_ptr()), pdata.len() as _, pfureadyflags, abmac).ok()
    }
    pub unsafe fn ObjectData(&self, pdata: *mut u8, pdwsize: *mut u32, abmac: *mut u8) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.ObjectData)(::windows::core::Vtable::as_raw(self), pdata, pdwsize, abmac).ok()
    }
    pub unsafe fn TransferComplete(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.TransferComplete)(::windows::core::Vtable::as_raw(self)).ok()
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
impl ISCPSecureExchange3 {
    pub unsafe fn TransferContainerData(&self, pdata: &[u8], pfureadyflags: *mut u32, abmac: *mut u8) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.TransferContainerData)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(pdata.as_ptr()), pdata.len() as _, pfureadyflags, abmac).ok()
    }
    pub unsafe fn ObjectData(&self, pdata: *mut u8, pdwsize: *mut u32, abmac: *mut u8) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.ObjectData)(::windows::core::Vtable::as_raw(self), pdata, pdwsize, abmac).ok()
    }
    pub unsafe fn TransferComplete(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.TransferComplete)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn TransferContainerData2<P0>(&self, pdata: &[u8], pprogresscallback: P0, pfureadyflags: *mut u32, abmac: *mut u8) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IWMDMProgress3>>,
    {
        (::windows::core::Vtable::vtable(self).base__.TransferContainerData2)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(pdata.as_ptr()), pdata.len() as _, pprogresscallback.into().abi(), pfureadyflags, abmac).ok()
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
impl ISCPSecureQuery2 {
    pub unsafe fn GetDataDemands(&self, pfuflags: *mut u32, pdwminrightsdata: *mut u32, pdwminexaminedata: *mut u32, pdwmindecidedata: *mut u32, abmac: *mut u8) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetDataDemands)(::windows::core::Vtable::as_raw(self), pfuflags, pdwminrightsdata, pdwminexaminedata, pdwmindecidedata, abmac).ok()
    }
    pub unsafe fn ExamineData<P0>(&self, fuflags: u32, pwszextension: P0, pdata: &[u8], abmac: *mut u8) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).base__.ExamineData)(::windows::core::Vtable::as_raw(self), fuflags, pwszextension.into().abi(), ::core::mem::transmute(pdata.as_ptr()), pdata.len() as _, abmac).ok()
    }
    pub unsafe fn MakeDecision<P0>(&self, fuflags: u32, pdata: &[u8], dwappsec: u32, pbspsessionkey: &[u8], pstorageglobals: P0, ppexchange: *mut ::core::option::Option<ISCPSecureExchange>, abmac: *mut u8) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IMDSPStorageGlobals>>,
    {
        (::windows::core::Vtable::vtable(self).base__.MakeDecision)(::windows::core::Vtable::as_raw(self), fuflags, ::core::mem::transmute(pdata.as_ptr()), pdata.len() as _, dwappsec, ::core::mem::transmute(pbspsessionkey.as_ptr()), pbspsessionkey.len() as _, pstorageglobals.into().abi(), ::core::mem::transmute(ppexchange), abmac).ok()
    }
    pub unsafe fn GetRights<P0>(&self, pdata: &[u8], pbspsessionkey: &[u8], pstgglobals: P0, pprights: *mut *mut WMDMRIGHTS, pnrightscount: *mut u32, abmac: *mut u8) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IMDSPStorageGlobals>>,
    {
        (::windows::core::Vtable::vtable(self).base__.GetRights)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(pdata.as_ptr()), pdata.len() as _, ::core::mem::transmute(pbspsessionkey.as_ptr()), pbspsessionkey.len() as _, pstgglobals.into().abi(), pprights, pnrightscount, abmac).ok()
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
impl ISCPSecureQuery3 {
    pub unsafe fn GetDataDemands(&self, pfuflags: *mut u32, pdwminrightsdata: *mut u32, pdwminexaminedata: *mut u32, pdwmindecidedata: *mut u32, abmac: *mut u8) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.GetDataDemands)(::windows::core::Vtable::as_raw(self), pfuflags, pdwminrightsdata, pdwminexaminedata, pdwmindecidedata, abmac).ok()
    }
    pub unsafe fn ExamineData<P0>(&self, fuflags: u32, pwszextension: P0, pdata: &[u8], abmac: *mut u8) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.ExamineData)(::windows::core::Vtable::as_raw(self), fuflags, pwszextension.into().abi(), ::core::mem::transmute(pdata.as_ptr()), pdata.len() as _, abmac).ok()
    }
    pub unsafe fn MakeDecision<P0>(&self, fuflags: u32, pdata: &[u8], dwappsec: u32, pbspsessionkey: &[u8], pstorageglobals: P0, ppexchange: *mut ::core::option::Option<ISCPSecureExchange>, abmac: *mut u8) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IMDSPStorageGlobals>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.MakeDecision)(::windows::core::Vtable::as_raw(self), fuflags, ::core::mem::transmute(pdata.as_ptr()), pdata.len() as _, dwappsec, ::core::mem::transmute(pbspsessionkey.as_ptr()), pbspsessionkey.len() as _, pstorageglobals.into().abi(), ::core::mem::transmute(ppexchange), abmac).ok()
    }
    pub unsafe fn GetRights<P0>(&self, pdata: &[u8], pbspsessionkey: &[u8], pstgglobals: P0, pprights: *mut *mut WMDMRIGHTS, pnrightscount: *mut u32, abmac: *mut u8) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IMDSPStorageGlobals>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.GetRights)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(pdata.as_ptr()), pdata.len() as _, ::core::mem::transmute(pbspsessionkey.as_ptr()), pbspsessionkey.len() as _, pstgglobals.into().abi(), pprights, pnrightscount, abmac).ok()
    }
    pub unsafe fn MakeDecision2<P0, P1>(&self, fuflags: u32, pdata: &[u8], dwappsec: u32, pbspsessionkey: &[u8], pstorageglobals: P0, pappcertapp: &[u8], pappcertsp: &[u8], pszrevocationurl: *mut ::windows::core::PWSTR, pdwrevocationurllen: *mut u32, pdwrevocationbitflag: *mut u32, pqwfilesize: ::core::option::Option<*mut u64>, punknown: P1, ppexchange: *mut ::core::option::Option<ISCPSecureExchange>, abmac: *mut u8) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IMDSPStorageGlobals>>,
        P1: ::std::convert::Into<::windows::core::InParam<::windows::core::IUnknown>>,
    {
        (::windows::core::Vtable::vtable(self).base__.MakeDecision2)(
            ::windows::core::Vtable::as_raw(self),
            fuflags,
            ::core::mem::transmute(pdata.as_ptr()),
            pdata.len() as _,
            dwappsec,
            ::core::mem::transmute(pbspsessionkey.as_ptr()),
            pbspsessionkey.len() as _,
            pstorageglobals.into().abi(),
            ::core::mem::transmute(pappcertapp.as_ptr()),
            pappcertapp.len() as _,
            ::core::mem::transmute(pappcertsp.as_ptr()),
            pappcertsp.len() as _,
            pszrevocationurl,
            pdwrevocationurllen,
            pdwrevocationbitflag,
            ::core::mem::transmute(pqwfilesize.unwrap_or(::std::ptr::null_mut())),
            punknown.into().abi(),
            ::core::mem::transmute(ppexchange),
            abmac,
        )
        .ok()
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
impl IWMDMDevice2 {
    pub unsafe fn GetName(&self, pwszname: &mut [u16]) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetName)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(pwszname.as_ptr()), pwszname.len() as _).ok()
    }
    pub unsafe fn GetManufacturer(&self, pwszname: &mut [u16]) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetManufacturer)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(pwszname.as_ptr()), pwszname.len() as _).ok()
    }
    pub unsafe fn GetVersion(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetVersion)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetType(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetType)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetSerialNumber(&self, pserialnumber: *mut WMDMID, abmac: *mut u8) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetSerialNumber)(::windows::core::Vtable::as_raw(self), pserialnumber, abmac).ok()
    }
    pub unsafe fn GetPowerSource(&self, pdwpowersource: *mut u32, pdwpercentremaining: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetPowerSource)(::windows::core::Vtable::as_raw(self), pdwpowersource, pdwpercentremaining).ok()
    }
    pub unsafe fn GetStatus(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetStatus)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetDeviceIcon(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetDeviceIcon)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn EnumStorage(&self) -> ::windows::core::Result<IWMDMEnumStorage> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.EnumStorage)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
    #[cfg(feature = "Win32_Media_Audio")]
    pub unsafe fn GetFormatSupport(&self, ppformatex: *mut *mut super::Audio::WAVEFORMATEX, pnformatcount: *mut u32, pppwszmimetype: *mut *mut ::windows::core::PWSTR, pnmimetypecount: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetFormatSupport)(::windows::core::Vtable::as_raw(self), ppformatex, pnformatcount, pppwszmimetype, pnmimetypecount).ok()
    }
    pub unsafe fn SendOpaqueCommand(&self, pcommand: *mut OPAQUECOMMAND) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SendOpaqueCommand)(::windows::core::Vtable::as_raw(self), pcommand).ok()
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
impl IWMDMDevice3 {
    pub unsafe fn GetName(&self, pwszname: &mut [u16]) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.GetName)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(pwszname.as_ptr()), pwszname.len() as _).ok()
    }
    pub unsafe fn GetManufacturer(&self, pwszname: &mut [u16]) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.GetManufacturer)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(pwszname.as_ptr()), pwszname.len() as _).ok()
    }
    pub unsafe fn GetVersion(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.GetVersion)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetType(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.GetType)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetSerialNumber(&self, pserialnumber: *mut WMDMID, abmac: *mut u8) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.GetSerialNumber)(::windows::core::Vtable::as_raw(self), pserialnumber, abmac).ok()
    }
    pub unsafe fn GetPowerSource(&self, pdwpowersource: *mut u32, pdwpercentremaining: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.GetPowerSource)(::windows::core::Vtable::as_raw(self), pdwpowersource, pdwpercentremaining).ok()
    }
    pub unsafe fn GetStatus(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.GetStatus)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetDeviceIcon(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.GetDeviceIcon)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn EnumStorage(&self) -> ::windows::core::Result<IWMDMEnumStorage> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.EnumStorage)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
    #[cfg(feature = "Win32_Media_Audio")]
    pub unsafe fn GetFormatSupport(&self, ppformatex: *mut *mut super::Audio::WAVEFORMATEX, pnformatcount: *mut u32, pppwszmimetype: *mut *mut ::windows::core::PWSTR, pnmimetypecount: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.GetFormatSupport)(::windows::core::Vtable::as_raw(self), ppformatex, pnformatcount, pppwszmimetype, pnmimetypecount).ok()
    }
    pub unsafe fn SendOpaqueCommand(&self, pcommand: *mut OPAQUECOMMAND) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.SendOpaqueCommand)(::windows::core::Vtable::as_raw(self), pcommand).ok()
    }
    pub unsafe fn GetStorage<P0>(&self, pszstoragename: P0) -> ::windows::core::Result<IWMDMStorage>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetStorage)(::windows::core::Vtable::as_raw(self), pszstoragename.into().abi(), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`, `\"Win32_Media_Audio\"`, `\"Win32_Media_MediaFoundation\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_Media_Audio", feature = "Win32_Media_MediaFoundation"))]
    pub unsafe fn GetFormatSupport2(&self, dwflags: u32, ppaudioformatex: *mut *mut super::Audio::WAVEFORMATEX, pnaudioformatcount: *mut u32, ppvideoformatex: *mut *mut super::MediaFoundation::VIDEOINFOHEADER, pnvideoformatcount: *mut u32, ppfiletype: *mut *mut WMFILECAPABILITIES, pnfiletypecount: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetFormatSupport2)(::windows::core::Vtable::as_raw(self), dwflags, ppaudioformatex, pnaudioformatcount, ppvideoformatex, pnvideoformatcount, ppfiletype, pnfiletypecount).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Ole\"`*"]
    #[cfg(feature = "Win32_System_Ole")]
    pub unsafe fn GetSpecifyPropertyPages(&self, ppspecifyproppages: *mut ::core::option::Option<super::super::System::Ole::ISpecifyPropertyPages>, pppunknowns: *mut *mut ::core::option::Option<::windows::core::IUnknown>, pcunks: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetSpecifyPropertyPages)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(ppspecifyproppages), pppunknowns, pcunks).ok()
    }
    pub unsafe fn GetCanonicalName(&self, pwszpnpname: &mut [u16]) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetCanonicalName)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(pwszpnpname.as_ptr()), pwszpnpname.len() as _).ok()
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
impl IWMDMOperation2 {
    pub unsafe fn BeginRead(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.BeginRead)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn BeginWrite(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.BeginWrite)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn GetObjectName(&self, pwszname: &mut [u16]) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetObjectName)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(pwszname.as_ptr()), pwszname.len() as _).ok()
    }
    pub unsafe fn SetObjectName(&self, pwszname: &[u16]) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetObjectName)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(pwszname.as_ptr()), pwszname.len() as _).ok()
    }
    #[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
    #[cfg(feature = "Win32_Media_Audio")]
    pub unsafe fn GetObjectAttributes(&self, pdwattributes: *mut u32, pformat: ::core::option::Option<*mut super::Audio::WAVEFORMATEX>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetObjectAttributes)(::windows::core::Vtable::as_raw(self), pdwattributes, ::core::mem::transmute(pformat.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    #[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
    #[cfg(feature = "Win32_Media_Audio")]
    pub unsafe fn SetObjectAttributes(&self, dwattributes: u32, pformat: ::core::option::Option<*const super::Audio::WAVEFORMATEX>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetObjectAttributes)(::windows::core::Vtable::as_raw(self), dwattributes, ::core::mem::transmute(pformat.unwrap_or(::std::ptr::null()))).ok()
    }
    pub unsafe fn GetObjectTotalSize(&self, pdwsize: *mut u32, pdwsizehigh: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetObjectTotalSize)(::windows::core::Vtable::as_raw(self), pdwsize, pdwsizehigh).ok()
    }
    pub unsafe fn SetObjectTotalSize(&self, dwsize: u32, dwsizehigh: u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetObjectTotalSize)(::windows::core::Vtable::as_raw(self), dwsize, dwsizehigh).ok()
    }
    pub unsafe fn TransferObjectData(&self, pdata: *mut u8, pdwsize: *mut u32, abmac: *mut u8) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.TransferObjectData)(::windows::core::Vtable::as_raw(self), pdata, pdwsize, abmac).ok()
    }
    pub unsafe fn End<P0>(&self, phcompletioncode: *const ::windows::core::HRESULT, pnewobject: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::IUnknown>>,
    {
        (::windows::core::Vtable::vtable(self).base__.End)(::windows::core::Vtable::as_raw(self), phcompletioncode, pnewobject.into().abi()).ok()
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
impl IWMDMOperation3 {
    pub unsafe fn BeginRead(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.BeginRead)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn BeginWrite(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.BeginWrite)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn GetObjectName(&self, pwszname: &mut [u16]) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetObjectName)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(pwszname.as_ptr()), pwszname.len() as _).ok()
    }
    pub unsafe fn SetObjectName(&self, pwszname: &[u16]) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetObjectName)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(pwszname.as_ptr()), pwszname.len() as _).ok()
    }
    #[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
    #[cfg(feature = "Win32_Media_Audio")]
    pub unsafe fn GetObjectAttributes(&self, pdwattributes: *mut u32, pformat: ::core::option::Option<*mut super::Audio::WAVEFORMATEX>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetObjectAttributes)(::windows::core::Vtable::as_raw(self), pdwattributes, ::core::mem::transmute(pformat.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    #[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
    #[cfg(feature = "Win32_Media_Audio")]
    pub unsafe fn SetObjectAttributes(&self, dwattributes: u32, pformat: ::core::option::Option<*const super::Audio::WAVEFORMATEX>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetObjectAttributes)(::windows::core::Vtable::as_raw(self), dwattributes, ::core::mem::transmute(pformat.unwrap_or(::std::ptr::null()))).ok()
    }
    pub unsafe fn GetObjectTotalSize(&self, pdwsize: *mut u32, pdwsizehigh: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetObjectTotalSize)(::windows::core::Vtable::as_raw(self), pdwsize, pdwsizehigh).ok()
    }
    pub unsafe fn SetObjectTotalSize(&self, dwsize: u32, dwsizehigh: u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetObjectTotalSize)(::windows::core::Vtable::as_raw(self), dwsize, dwsizehigh).ok()
    }
    pub unsafe fn TransferObjectData(&self, pdata: *mut u8, pdwsize: *mut u32, abmac: *mut u8) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.TransferObjectData)(::windows::core::Vtable::as_raw(self), pdata, pdwsize, abmac).ok()
    }
    pub unsafe fn End<P0>(&self, phcompletioncode: *const ::windows::core::HRESULT, pnewobject: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::IUnknown>>,
    {
        (::windows::core::Vtable::vtable(self).base__.End)(::windows::core::Vtable::as_raw(self), phcompletioncode, pnewobject.into().abi()).ok()
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
impl IWMDMProgress2 {
    pub unsafe fn Begin(&self, dwestimatedticks: u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.Begin)(::windows::core::Vtable::as_raw(self), dwestimatedticks).ok()
    }
    pub unsafe fn Progress(&self, dwtranspiredticks: u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.Progress)(::windows::core::Vtable::as_raw(self), dwtranspiredticks).ok()
    }
    pub unsafe fn End(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.End)(::windows::core::Vtable::as_raw(self)).ok()
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
impl IWMDMProgress3 {
    pub unsafe fn Begin(&self, dwestimatedticks: u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.Begin)(::windows::core::Vtable::as_raw(self), dwestimatedticks).ok()
    }
    pub unsafe fn Progress(&self, dwtranspiredticks: u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.Progress)(::windows::core::Vtable::as_raw(self), dwtranspiredticks).ok()
    }
    pub unsafe fn End(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.End)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn End2(&self, hrcompletioncode: ::windows::core::HRESULT) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.End2)(::windows::core::Vtable::as_raw(self), hrcompletioncode).ok()
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
impl IWMDMStorage2 {
    #[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
    #[cfg(feature = "Win32_Media_Audio")]
    pub unsafe fn SetAttributes(&self, dwattributes: u32, pformat: ::core::option::Option<*const super::Audio::WAVEFORMATEX>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetAttributes)(::windows::core::Vtable::as_raw(self), dwattributes, ::core::mem::transmute(pformat.unwrap_or(::std::ptr::null()))).ok()
    }
    pub unsafe fn GetStorageGlobals(&self) -> ::windows::core::Result<IWMDMStorageGlobals> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetStorageGlobals)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
    #[cfg(feature = "Win32_Media_Audio")]
    pub unsafe fn GetAttributes(&self, pdwattributes: *mut u32, pformat: ::core::option::Option<*mut super::Audio::WAVEFORMATEX>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetAttributes)(::windows::core::Vtable::as_raw(self), pdwattributes, ::core::mem::transmute(pformat.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    pub unsafe fn GetName(&self, pwszname: &mut [u16]) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetName)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(pwszname.as_ptr()), pwszname.len() as _).ok()
    }
    pub unsafe fn GetDate(&self) -> ::windows::core::Result<WMDMDATETIME> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetDate)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetSize(&self, pdwsizelow: *mut u32, pdwsizehigh: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetSize)(::windows::core::Vtable::as_raw(self), pdwsizelow, pdwsizehigh).ok()
    }
    pub unsafe fn GetRights(&self, pprights: *mut *mut WMDMRIGHTS, pnrightscount: *mut u32, abmac: *mut u8) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetRights)(::windows::core::Vtable::as_raw(self), pprights, pnrightscount, abmac).ok()
    }
    pub unsafe fn EnumStorage(&self) -> ::windows::core::Result<IWMDMEnumStorage> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.EnumStorage)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SendOpaqueCommand(&self, pcommand: *mut OPAQUECOMMAND) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SendOpaqueCommand)(::windows::core::Vtable::as_raw(self), pcommand).ok()
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
impl IWMDMStorage3 {
    #[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
    #[cfg(feature = "Win32_Media_Audio")]
    pub unsafe fn SetAttributes(&self, dwattributes: u32, pformat: ::core::option::Option<*const super::Audio::WAVEFORMATEX>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.SetAttributes)(::windows::core::Vtable::as_raw(self), dwattributes, ::core::mem::transmute(pformat.unwrap_or(::std::ptr::null()))).ok()
    }
    pub unsafe fn GetStorageGlobals(&self) -> ::windows::core::Result<IWMDMStorageGlobals> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.GetStorageGlobals)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
    #[cfg(feature = "Win32_Media_Audio")]
    pub unsafe fn GetAttributes(&self, pdwattributes: *mut u32, pformat: ::core::option::Option<*mut super::Audio::WAVEFORMATEX>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.GetAttributes)(::windows::core::Vtable::as_raw(self), pdwattributes, ::core::mem::transmute(pformat.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    pub unsafe fn GetName(&self, pwszname: &mut [u16]) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.GetName)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(pwszname.as_ptr()), pwszname.len() as _).ok()
    }
    pub unsafe fn GetDate(&self) -> ::windows::core::Result<WMDMDATETIME> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.GetDate)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetSize(&self, pdwsizelow: *mut u32, pdwsizehigh: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.GetSize)(::windows::core::Vtable::as_raw(self), pdwsizelow, pdwsizehigh).ok()
    }
    pub unsafe fn GetRights(&self, pprights: *mut *mut WMDMRIGHTS, pnrightscount: *mut u32, abmac: *mut u8) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.GetRights)(::windows::core::Vtable::as_raw(self), pprights, pnrightscount, abmac).ok()
    }
    pub unsafe fn EnumStorage(&self) -> ::windows::core::Result<IWMDMEnumStorage> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.EnumStorage)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SendOpaqueCommand(&self, pcommand: *mut OPAQUECOMMAND) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.SendOpaqueCommand)(::windows::core::Vtable::as_raw(self), pcommand).ok()
    }
    pub unsafe fn GetStorage<P0>(&self, pszstoragename: P0) -> ::windows::core::Result<IWMDMStorage>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetStorage)(::windows::core::Vtable::as_raw(self), pszstoragename.into().abi(), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`, `\"Win32_Media_Audio\"`, `\"Win32_Media_MediaFoundation\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_Media_Audio", feature = "Win32_Media_MediaFoundation"))]
    pub unsafe fn SetAttributes2(&self, dwattributes: u32, dwattributesex: u32, pformat: ::core::option::Option<*const super::Audio::WAVEFORMATEX>, pvideoformat: ::core::option::Option<*const super::MediaFoundation::VIDEOINFOHEADER>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetAttributes2)(::windows::core::Vtable::as_raw(self), dwattributes, dwattributesex, ::core::mem::transmute(pformat.unwrap_or(::std::ptr::null())), ::core::mem::transmute(pvideoformat.unwrap_or(::std::ptr::null()))).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`, `\"Win32_Media_Audio\"`, `\"Win32_Media_MediaFoundation\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_Media_Audio", feature = "Win32_Media_MediaFoundation"))]
    pub unsafe fn GetAttributes2(&self, pdwattributes: *mut u32, pdwattributesex: *mut u32, paudioformat: ::core::option::Option<*mut super::Audio::WAVEFORMATEX>, pvideoformat: ::core::option::Option<*mut super::MediaFoundation::VIDEOINFOHEADER>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetAttributes2)(::windows::core::Vtable::as_raw(self), pdwattributes, pdwattributesex, ::core::mem::transmute(paudioformat.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(pvideoformat.unwrap_or(::std::ptr::null_mut()))).ok()
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
impl IWMDMStorage4 {
    #[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
    #[cfg(feature = "Win32_Media_Audio")]
    pub unsafe fn SetAttributes(&self, dwattributes: u32, pformat: ::core::option::Option<*const super::Audio::WAVEFORMATEX>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.SetAttributes)(::windows::core::Vtable::as_raw(self), dwattributes, ::core::mem::transmute(pformat.unwrap_or(::std::ptr::null()))).ok()
    }
    pub unsafe fn GetStorageGlobals(&self) -> ::windows::core::Result<IWMDMStorageGlobals> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.GetStorageGlobals)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
    #[cfg(feature = "Win32_Media_Audio")]
    pub unsafe fn GetAttributes(&self, pdwattributes: *mut u32, pformat: ::core::option::Option<*mut super::Audio::WAVEFORMATEX>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.GetAttributes)(::windows::core::Vtable::as_raw(self), pdwattributes, ::core::mem::transmute(pformat.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    pub unsafe fn GetName(&self, pwszname: &mut [u16]) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.GetName)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(pwszname.as_ptr()), pwszname.len() as _).ok()
    }
    pub unsafe fn GetDate(&self) -> ::windows::core::Result<WMDMDATETIME> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.GetDate)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetSize(&self, pdwsizelow: *mut u32, pdwsizehigh: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.GetSize)(::windows::core::Vtable::as_raw(self), pdwsizelow, pdwsizehigh).ok()
    }
    pub unsafe fn GetRights(&self, pprights: *mut *mut WMDMRIGHTS, pnrightscount: *mut u32, abmac: *mut u8) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.GetRights)(::windows::core::Vtable::as_raw(self), pprights, pnrightscount, abmac).ok()
    }
    pub unsafe fn EnumStorage(&self) -> ::windows::core::Result<IWMDMEnumStorage> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.EnumStorage)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SendOpaqueCommand(&self, pcommand: *mut OPAQUECOMMAND) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.SendOpaqueCommand)(::windows::core::Vtable::as_raw(self), pcommand).ok()
    }
    pub unsafe fn GetStorage<P0>(&self, pszstoragename: P0) -> ::windows::core::Result<IWMDMStorage>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.GetStorage)(::windows::core::Vtable::as_raw(self), pszstoragename.into().abi(), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`, `\"Win32_Media_Audio\"`, `\"Win32_Media_MediaFoundation\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_Media_Audio", feature = "Win32_Media_MediaFoundation"))]
    pub unsafe fn SetAttributes2(&self, dwattributes: u32, dwattributesex: u32, pformat: ::core::option::Option<*const super::Audio::WAVEFORMATEX>, pvideoformat: ::core::option::Option<*const super::MediaFoundation::VIDEOINFOHEADER>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.SetAttributes2)(::windows::core::Vtable::as_raw(self), dwattributes, dwattributesex, ::core::mem::transmute(pformat.unwrap_or(::std::ptr::null())), ::core::mem::transmute(pvideoformat.unwrap_or(::std::ptr::null()))).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`, `\"Win32_Media_Audio\"`, `\"Win32_Media_MediaFoundation\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_Media_Audio", feature = "Win32_Media_MediaFoundation"))]
    pub unsafe fn GetAttributes2(&self, pdwattributes: *mut u32, pdwattributesex: *mut u32, paudioformat: ::core::option::Option<*mut super::Audio::WAVEFORMATEX>, pvideoformat: ::core::option::Option<*mut super::MediaFoundation::VIDEOINFOHEADER>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.GetAttributes2)(::windows::core::Vtable::as_raw(self), pdwattributes, pdwattributesex, ::core::mem::transmute(paudioformat.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(pvideoformat.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    pub unsafe fn GetMetadata(&self) -> ::windows::core::Result<IWMDMMetaData> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetMetadata)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetMetadata<P0>(&self, pmetadata: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IWMDMMetaData>>,
    {
        (::windows::core::Vtable::vtable(self).base__.SetMetadata)(::windows::core::Vtable::as_raw(self), pmetadata.into().abi()).ok()
    }
    pub unsafe fn CreateEmptyMetadataObject(&self) -> ::windows::core::Result<IWMDMMetaData> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.CreateEmptyMetadataObject)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetEnumPreference(&self, pmode: *mut WMDM_STORAGE_ENUM_MODE, pviews: ::core::option::Option<&[WMDMMetadataView]>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetEnumPreference)(::windows::core::Vtable::as_raw(self), pmode, pviews.as_deref().map_or(0, |slice| slice.len() as _), ::core::mem::transmute(pviews.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr()))).ok()
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
impl IWMDMStorageControl2 {
    pub unsafe fn Insert<P0, P1, P2>(&self, fumode: u32, pwszfile: P0, poperation: P1, pprogress: P2) -> ::windows::core::Result<IWMDMStorage>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
        P1: ::std::convert::Into<::windows::core::InParam<IWMDMOperation>>,
        P2: ::std::convert::Into<::windows::core::InParam<IWMDMProgress>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.Insert)(::windows::core::Vtable::as_raw(self), fumode, pwszfile.into().abi(), poperation.into().abi(), pprogress.into().abi(), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn Delete<P0>(&self, fumode: u32, pprogress: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IWMDMProgress>>,
    {
        (::windows::core::Vtable::vtable(self).base__.Delete)(::windows::core::Vtable::as_raw(self), fumode, pprogress.into().abi()).ok()
    }
    pub unsafe fn Rename<P0, P1>(&self, fumode: u32, pwsznewname: P0, pprogress: P1) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
        P1: ::std::convert::Into<::windows::core::InParam<IWMDMProgress>>,
    {
        (::windows::core::Vtable::vtable(self).base__.Rename)(::windows::core::Vtable::as_raw(self), fumode, pwsznewname.into().abi(), pprogress.into().abi()).ok()
    }
    pub unsafe fn Read<P0, P1, P2>(&self, fumode: u32, pwszfile: P0, pprogress: P1, poperation: P2) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
        P1: ::std::convert::Into<::windows::core::InParam<IWMDMProgress>>,
        P2: ::std::convert::Into<::windows::core::InParam<IWMDMOperation>>,
    {
        (::windows::core::Vtable::vtable(self).base__.Read)(::windows::core::Vtable::as_raw(self), fumode, pwszfile.into().abi(), pprogress.into().abi(), poperation.into().abi()).ok()
    }
    pub unsafe fn Move<P0, P1>(&self, fumode: u32, ptargetobject: P0, pprogress: P1) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IWMDMStorage>>,
        P1: ::std::convert::Into<::windows::core::InParam<IWMDMProgress>>,
    {
        (::windows::core::Vtable::vtable(self).base__.Move)(::windows::core::Vtable::as_raw(self), fumode, ptargetobject.into().abi(), pprogress.into().abi()).ok()
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
impl IWMDMStorageControl3 {
    pub unsafe fn Insert<P0, P1, P2>(&self, fumode: u32, pwszfile: P0, poperation: P1, pprogress: P2) -> ::windows::core::Result<IWMDMStorage>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
        P1: ::std::convert::Into<::windows::core::InParam<IWMDMOperation>>,
        P2: ::std::convert::Into<::windows::core::InParam<IWMDMProgress>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.Insert)(::windows::core::Vtable::as_raw(self), fumode, pwszfile.into().abi(), poperation.into().abi(), pprogress.into().abi(), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn Delete<P0>(&self, fumode: u32, pprogress: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IWMDMProgress>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.Delete)(::windows::core::Vtable::as_raw(self), fumode, pprogress.into().abi()).ok()
    }
    pub unsafe fn Rename<P0, P1>(&self, fumode: u32, pwsznewname: P0, pprogress: P1) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
        P1: ::std::convert::Into<::windows::core::InParam<IWMDMProgress>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.Rename)(::windows::core::Vtable::as_raw(self), fumode, pwsznewname.into().abi(), pprogress.into().abi()).ok()
    }
    pub unsafe fn Read<P0, P1, P2>(&self, fumode: u32, pwszfile: P0, pprogress: P1, poperation: P2) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
        P1: ::std::convert::Into<::windows::core::InParam<IWMDMProgress>>,
        P2: ::std::convert::Into<::windows::core::InParam<IWMDMOperation>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.Read)(::windows::core::Vtable::as_raw(self), fumode, pwszfile.into().abi(), pprogress.into().abi(), poperation.into().abi()).ok()
    }
    pub unsafe fn Move<P0, P1>(&self, fumode: u32, ptargetobject: P0, pprogress: P1) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IWMDMStorage>>,
        P1: ::std::convert::Into<::windows::core::InParam<IWMDMProgress>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.Move)(::windows::core::Vtable::as_raw(self), fumode, ptargetobject.into().abi(), pprogress.into().abi()).ok()
    }
    pub unsafe fn Insert2<P0, P1, P2, P3, P4>(&self, fumode: u32, pwszfilesource: P0, pwszfiledest: P1, poperation: P2, pprogress: P3, punknown: P4, ppnewobject: ::core::option::Option<*mut ::core::option::Option<IWMDMStorage>>) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
        P1: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
        P2: ::std::convert::Into<::windows::core::InParam<IWMDMOperation>>,
        P3: ::std::convert::Into<::windows::core::InParam<IWMDMProgress>>,
        P4: ::std::convert::Into<::windows::core::InParam<::windows::core::IUnknown>>,
    {
        (::windows::core::Vtable::vtable(self).base__.Insert2)(::windows::core::Vtable::as_raw(self), fumode, pwszfilesource.into().abi(), pwszfiledest.into().abi(), poperation.into().abi(), pprogress.into().abi(), punknown.into().abi(), ::core::mem::transmute(ppnewobject.unwrap_or(::std::ptr::null_mut()))).ok()
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
impl IWMDeviceManager2 {
    pub unsafe fn GetRevision(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetRevision)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetDeviceCount(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetDeviceCount)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn EnumDevices(&self) -> ::windows::core::Result<IWMDMEnumDevice> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.EnumDevices)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
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
impl IWMDeviceManager3 {
    pub unsafe fn GetRevision(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.GetRevision)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetDeviceCount(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.GetDeviceCount)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn EnumDevices(&self) -> ::windows::core::Result<IWMDMEnumDevice> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.EnumDevices)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetDeviceFromCanonicalName<P0>(&self, pwszcanonicalname: P0) -> ::windows::core::Result<IWMDMDevice>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetDeviceFromCanonicalName)(::windows::core::Vtable::as_raw(self), pwszcanonicalname.into().abi(), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn EnumDevices2(&self) -> ::windows::core::Result<IWMDMEnumDevice> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.EnumDevices2)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn Reinitialize(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.Reinitialize)(::windows::core::Vtable::as_raw(self)).ok()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for MACINFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
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
impl ::core::fmt::Debug for MACINFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MACINFO").field("fUsed", &self.fUsed).field("abMacState", &self.abMacState).finish()
    }
}
impl ::core::default::Default for MTP_COMMAND_DATA_IN {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for MTP_COMMAND_DATA_OUT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for OPAQUECOMMAND {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for OPAQUECOMMAND {
    fn eq(&self, other: &Self) -> bool {
        self.guidCommand == other.guidCommand && self.dwDataLen == other.dwDataLen && self.pData == other.pData && self.abMAC == other.abMAC
    }
}
impl ::core::cmp::Eq for OPAQUECOMMAND {}
impl ::core::fmt::Debug for OPAQUECOMMAND {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("OPAQUECOMMAND").field("guidCommand", &self.guidCommand).field("dwDataLen", &self.dwDataLen).field("pData", &self.pData).field("abMAC", &self.abMAC).finish()
    }
}
impl ::core::default::Default for WMDMDATETIME {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for WMDMDATETIME {
    fn eq(&self, other: &Self) -> bool {
        self.wYear == other.wYear && self.wMonth == other.wMonth && self.wDay == other.wDay && self.wHour == other.wHour && self.wMinute == other.wMinute && self.wSecond == other.wSecond
    }
}
impl ::core::cmp::Eq for WMDMDATETIME {}
impl ::core::fmt::Debug for WMDMDATETIME {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WMDMDATETIME").field("wYear", &self.wYear).field("wMonth", &self.wMonth).field("wDay", &self.wDay).field("wHour", &self.wHour).field("wMinute", &self.wMinute).field("wSecond", &self.wSecond).finish()
    }
}
impl ::core::default::Default for WMDMDetermineMaxPropStringLen {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for WMDMID {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for WMDMID {
    fn eq(&self, other: &Self) -> bool {
        self.cbSize == other.cbSize && self.dwVendorID == other.dwVendorID && self.pID == other.pID && self.SerialNumberLength == other.SerialNumberLength
    }
}
impl ::core::cmp::Eq for WMDMID {}
impl ::core::fmt::Debug for WMDMID {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WMDMID").field("cbSize", &self.cbSize).field("dwVendorID", &self.dwVendorID).field("pID", &self.pID).field("SerialNumberLength", &self.SerialNumberLength).finish()
    }
}
impl ::core::default::Default for WMDMMessage {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for WMDMMessage {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WMDMMessage").field(&self.0).finish()
    }
}
impl ::core::default::Default for WMDMMetadataView {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for WMDMMetadataView {
    fn eq(&self, other: &Self) -> bool {
        self.pwszViewName == other.pwszViewName && self.nDepth == other.nDepth && self.ppwszTags == other.ppwszTags
    }
}
impl ::core::cmp::Eq for WMDMMetadataView {}
impl ::core::fmt::Debug for WMDMMetadataView {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WMDMMetadataView").field("pwszViewName", &self.pwszViewName).field("nDepth", &self.nDepth).field("ppwszTags", &self.ppwszTags).finish()
    }
}
impl ::core::default::Default for WMDMRIGHTS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for WMDMRIGHTS {
    fn eq(&self, other: &Self) -> bool {
        self.cbSize == other.cbSize && self.dwContentType == other.dwContentType && self.fuFlags == other.fuFlags && self.fuRights == other.fuRights && self.dwAppSec == other.dwAppSec && self.dwPlaybackCount == other.dwPlaybackCount && self.ExpirationDate == other.ExpirationDate
    }
}
impl ::core::cmp::Eq for WMDMRIGHTS {}
impl ::core::fmt::Debug for WMDMRIGHTS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WMDMRIGHTS").field("cbSize", &self.cbSize).field("dwContentType", &self.dwContentType).field("fuFlags", &self.fuFlags).field("fuRights", &self.fuRights).field("dwAppSec", &self.dwAppSec).field("dwPlaybackCount", &self.dwPlaybackCount).field("ExpirationDate", &self.ExpirationDate).finish()
    }
}
impl ::core::default::Default for WMDM_ENUM_PROP_VALID_VALUES_FORM {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for WMDM_ENUM_PROP_VALID_VALUES_FORM {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WMDM_ENUM_PROP_VALID_VALUES_FORM").field(&self.0).finish()
    }
}
impl ::core::default::Default for WMDM_FIND_SCOPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for WMDM_FIND_SCOPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WMDM_FIND_SCOPE").field(&self.0).finish()
    }
}
impl ::core::default::Default for WMDM_FORMATCODE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for WMDM_FORMATCODE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WMDM_FORMATCODE").field(&self.0).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
impl ::core::default::Default for WMDM_FORMAT_CAPABILITY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
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
impl ::core::fmt::Debug for WMDM_FORMAT_CAPABILITY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WMDM_FORMAT_CAPABILITY").field("nPropConfig", &self.nPropConfig).field("pConfigs", &self.pConfigs).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
impl ::core::default::Default for WMDM_PROP_CONFIG {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
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
impl ::core::fmt::Debug for WMDM_PROP_CONFIG {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WMDM_PROP_CONFIG").field("nPreference", &self.nPreference).field("nPropDesc", &self.nPropDesc).field("pPropDesc", &self.pPropDesc).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
impl ::core::default::Default for WMDM_PROP_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
impl ::core::default::Default for WMDM_PROP_VALUES_ENUM {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
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
impl ::core::fmt::Debug for WMDM_PROP_VALUES_ENUM {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WMDM_PROP_VALUES_ENUM").field("cEnumValues", &self.cEnumValues).field("pValues", &self.pValues).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
impl ::core::default::Default for WMDM_PROP_VALUES_RANGE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for WMDM_SESSION_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for WMDM_SESSION_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WMDM_SESSION_TYPE").field(&self.0).finish()
    }
}
impl ::core::default::Default for WMDM_STORAGE_ENUM_MODE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for WMDM_STORAGE_ENUM_MODE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WMDM_STORAGE_ENUM_MODE").field(&self.0).finish()
    }
}
impl ::core::default::Default for WMDM_TAG_DATATYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for WMDM_TAG_DATATYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WMDM_TAG_DATATYPE").field(&self.0).finish()
    }
}
impl ::core::default::Default for WMFILECAPABILITIES {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for WMFILECAPABILITIES {
    fn eq(&self, other: &Self) -> bool {
        self.pwszMimeType == other.pwszMimeType && self.dwReserved == other.dwReserved
    }
}
impl ::core::cmp::Eq for WMFILECAPABILITIES {}
impl ::core::fmt::Debug for WMFILECAPABILITIES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WMFILECAPABILITIES").field("pwszMimeType", &self.pwszMimeType).field("dwReserved", &self.dwReserved).finish()
    }
}
