#![allow(unused_variables, non_upper_case_globals, non_snake_case, unused_unsafe, non_camel_case_types, dead_code, clippy::all)]
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
pub struct AM_WMT_EVENT_DATA {
    pub hrStatus: ::windows::runtime::HRESULT,
    pub pData: *mut ::std::ffi::c_void,
}
impl AM_WMT_EVENT_DATA {}
impl ::std::default::Default for AM_WMT_EVENT_DATA {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for AM_WMT_EVENT_DATA {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("AM_WMT_EVENT_DATA").field("hrStatus", &self.hrStatus).field("pData", &self.pData).finish()
    }
}
impl ::std::cmp::PartialEq for AM_WMT_EVENT_DATA {
    fn eq(&self, other: &Self) -> bool {
        self.hrStatus == other.hrStatus && self.pData == other.pData
    }
}
impl ::std::cmp::Eq for AM_WMT_EVENT_DATA {}
unsafe impl ::windows::runtime::Abi for AM_WMT_EVENT_DATA {
    type Abi = Self;
    type DefaultType = Self;
}
pub const CLSID_ClientNetManager: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3440550862, 40002, 4562, [190, 237, 0, 96, 8, 47, 32, 84]);
pub const CLSID_WMBandwidthSharing_Exclusive: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2942329002, 20887, 4562, [182, 175, 0, 192, 79, 217, 8, 233]);
pub const CLSID_WMBandwidthSharing_Partial: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2942329003, 20887, 4562, [182, 175, 0, 192, 79, 217, 8, 233]);
pub const CLSID_WMMUTEX_Bitrate: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3605146113, 13786, 4561, [144, 52, 0, 160, 201, 3, 73, 190]);
pub const CLSID_WMMUTEX_Language: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3605146112, 13786, 4561, [144, 52, 0, 160, 201, 3, 73, 190]);
pub const CLSID_WMMUTEX_Presentation: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3605146114, 13786, 4561, [144, 52, 0, 160, 201, 3, 73, 190]);
pub const CLSID_WMMUTEX_Unknown: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3605146115, 13786, 4561, [144, 52, 0, 160, 201, 3, 73, 190]);
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
pub struct DRM_COPY_OPL {
    pub wMinimumCopyLevel: u16,
    pub oplIdIncludes: DRM_OPL_OUTPUT_IDS,
    pub oplIdExcludes: DRM_OPL_OUTPUT_IDS,
}
impl DRM_COPY_OPL {}
impl ::std::default::Default for DRM_COPY_OPL {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for DRM_COPY_OPL {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("DRM_COPY_OPL").field("wMinimumCopyLevel", &self.wMinimumCopyLevel).field("oplIdIncludes", &self.oplIdIncludes).field("oplIdExcludes", &self.oplIdExcludes).finish()
    }
}
impl ::std::cmp::PartialEq for DRM_COPY_OPL {
    fn eq(&self, other: &Self) -> bool {
        self.wMinimumCopyLevel == other.wMinimumCopyLevel && self.oplIdIncludes == other.oplIdIncludes && self.oplIdExcludes == other.oplIdExcludes
    }
}
impl ::std::cmp::Eq for DRM_COPY_OPL {}
unsafe impl ::windows::runtime::Abi for DRM_COPY_OPL {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
pub struct DRM_MINIMUM_OUTPUT_PROTECTION_LEVELS {
    pub wCompressedDigitalVideo: u16,
    pub wUncompressedDigitalVideo: u16,
    pub wAnalogVideo: u16,
    pub wCompressedDigitalAudio: u16,
    pub wUncompressedDigitalAudio: u16,
}
impl DRM_MINIMUM_OUTPUT_PROTECTION_LEVELS {}
impl ::std::default::Default for DRM_MINIMUM_OUTPUT_PROTECTION_LEVELS {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for DRM_MINIMUM_OUTPUT_PROTECTION_LEVELS {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("DRM_MINIMUM_OUTPUT_PROTECTION_LEVELS")
            .field("wCompressedDigitalVideo", &self.wCompressedDigitalVideo)
            .field("wUncompressedDigitalVideo", &self.wUncompressedDigitalVideo)
            .field("wAnalogVideo", &self.wAnalogVideo)
            .field("wCompressedDigitalAudio", &self.wCompressedDigitalAudio)
            .field("wUncompressedDigitalAudio", &self.wUncompressedDigitalAudio)
            .finish()
    }
}
impl ::std::cmp::PartialEq for DRM_MINIMUM_OUTPUT_PROTECTION_LEVELS {
    fn eq(&self, other: &Self) -> bool {
        self.wCompressedDigitalVideo == other.wCompressedDigitalVideo && self.wUncompressedDigitalVideo == other.wUncompressedDigitalVideo && self.wAnalogVideo == other.wAnalogVideo && self.wCompressedDigitalAudio == other.wCompressedDigitalAudio && self.wUncompressedDigitalAudio == other.wUncompressedDigitalAudio
    }
}
impl ::std::cmp::Eq for DRM_MINIMUM_OUTPUT_PROTECTION_LEVELS {}
unsafe impl ::windows::runtime::Abi for DRM_MINIMUM_OUTPUT_PROTECTION_LEVELS {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
pub struct DRM_OPL_OUTPUT_IDS {
    pub cIds: u16,
    pub rgIds: *mut ::windows::runtime::GUID,
}
impl DRM_OPL_OUTPUT_IDS {}
impl ::std::default::Default for DRM_OPL_OUTPUT_IDS {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for DRM_OPL_OUTPUT_IDS {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("DRM_OPL_OUTPUT_IDS").field("cIds", &self.cIds).field("rgIds", &self.rgIds).finish()
    }
}
impl ::std::cmp::PartialEq for DRM_OPL_OUTPUT_IDS {
    fn eq(&self, other: &Self) -> bool {
        self.cIds == other.cIds && self.rgIds == other.rgIds
    }
}
impl ::std::cmp::Eq for DRM_OPL_OUTPUT_IDS {}
unsafe impl ::windows::runtime::Abi for DRM_OPL_OUTPUT_IDS {
    type Abi = Self;
    type DefaultType = Self;
}
#[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
pub const DRM_OPL_TYPES: u32 = 1u32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
pub struct DRM_OUTPUT_PROTECTION {
    pub guidId: ::windows::runtime::GUID,
    pub bConfigData: u8,
}
impl DRM_OUTPUT_PROTECTION {}
impl ::std::default::Default for DRM_OUTPUT_PROTECTION {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for DRM_OUTPUT_PROTECTION {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("DRM_OUTPUT_PROTECTION").field("guidId", &self.guidId).field("bConfigData", &self.bConfigData).finish()
    }
}
impl ::std::cmp::PartialEq for DRM_OUTPUT_PROTECTION {
    fn eq(&self, other: &Self) -> bool {
        self.guidId == other.guidId && self.bConfigData == other.bConfigData
    }
}
impl ::std::cmp::Eq for DRM_OUTPUT_PROTECTION {}
unsafe impl ::windows::runtime::Abi for DRM_OUTPUT_PROTECTION {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
pub struct DRM_PLAY_OPL {
    pub minOPL: DRM_MINIMUM_OUTPUT_PROTECTION_LEVELS,
    pub oplIdReserved: DRM_OPL_OUTPUT_IDS,
    pub vopi: DRM_VIDEO_OUTPUT_PROTECTION_IDS,
}
impl DRM_PLAY_OPL {}
impl ::std::default::Default for DRM_PLAY_OPL {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for DRM_PLAY_OPL {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("DRM_PLAY_OPL").field("minOPL", &self.minOPL).field("oplIdReserved", &self.oplIdReserved).field("vopi", &self.vopi).finish()
    }
}
impl ::std::cmp::PartialEq for DRM_PLAY_OPL {
    fn eq(&self, other: &Self) -> bool {
        self.minOPL == other.minOPL && self.oplIdReserved == other.oplIdReserved && self.vopi == other.vopi
    }
}
impl ::std::cmp::Eq for DRM_PLAY_OPL {}
unsafe impl ::windows::runtime::Abi for DRM_PLAY_OPL {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
pub struct DRM_VAL16 {
    pub val: [u8; 16],
}
impl DRM_VAL16 {}
impl ::std::default::Default for DRM_VAL16 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for DRM_VAL16 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("DRM_VAL16").field("val", &self.val).finish()
    }
}
impl ::std::cmp::PartialEq for DRM_VAL16 {
    fn eq(&self, other: &Self) -> bool {
        self.val == other.val
    }
}
impl ::std::cmp::Eq for DRM_VAL16 {}
unsafe impl ::windows::runtime::Abi for DRM_VAL16 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
pub struct DRM_VIDEO_OUTPUT_PROTECTION_IDS {
    pub cEntries: u16,
    pub rgVop: *mut DRM_OUTPUT_PROTECTION,
}
impl DRM_VIDEO_OUTPUT_PROTECTION_IDS {}
impl ::std::default::Default for DRM_VIDEO_OUTPUT_PROTECTION_IDS {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for DRM_VIDEO_OUTPUT_PROTECTION_IDS {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("DRM_VIDEO_OUTPUT_PROTECTION_IDS").field("cEntries", &self.cEntries).field("rgVop", &self.rgVop).finish()
    }
}
impl ::std::cmp::PartialEq for DRM_VIDEO_OUTPUT_PROTECTION_IDS {
    fn eq(&self, other: &Self) -> bool {
        self.cEntries == other.cEntries && self.rgVop == other.rgVop
    }
}
impl ::std::cmp::Eq for DRM_VIDEO_OUTPUT_PROTECTION_IDS {}
unsafe impl ::windows::runtime::Abi for DRM_VIDEO_OUTPUT_PROTECTION_IDS {
    type Abi = Self;
    type DefaultType = Self;
}
#[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct IAMWMBufferPass(::windows::runtime::IUnknown);
impl IAMWMBufferPass {
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
    pub unsafe fn SetNotify<'a, Param0: ::windows::runtime::IntoParam<'a, IAMWMBufferPassCallback>>(&self, pcallback: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), pcallback.into_param().abi()).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IAMWMBufferPass {
    type Vtable = IAMWMBufferPass_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1842878167, 59200, 16675, [158, 36, 36, 68, 65, 38, 68, 216]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAMWMBufferPass_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pcallback: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct IAMWMBufferPassCallback(::windows::runtime::IUnknown);
impl IAMWMBufferPassCallback {
    #[cfg(feature = "Win32_Graphics_DirectShow")]
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`, `Win32_Graphics_DirectShow`*"]
    pub unsafe fn Notify<'a, Param0: ::windows::runtime::IntoParam<'a, INSSBuffer3>, Param1: ::windows::runtime::IntoParam<'a, super::super::Graphics::DirectShow::IPin>>(&self, pnssbuffer3: Param0, ppin: Param1, prtstart: *const i64, prtend: *const i64) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), pnssbuffer3.into_param().abi(), ppin.into_param().abi(), ::std::mem::transmute(prtstart), ::std::mem::transmute(prtend)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IAMWMBufferPassCallback {
    type Vtable = IAMWMBufferPassCallback_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2992341874, 53970, 17586, [134, 83, 27, 141, 174, 51, 36, 137]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAMWMBufferPassCallback_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_Graphics_DirectShow")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pnssbuffer3: ::windows::runtime::RawPtr, ppin: ::windows::runtime::RawPtr, prtstart: *const i64, prtend: *const i64) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_DirectShow"))] usize,
);
#[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct INSNetSourceCreator(::windows::runtime::IUnknown);
impl INSNetSourceCreator {
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
    pub unsafe fn Initialize(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`, `Win32_Foundation`*"]
    pub unsafe fn CreateNetSource<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>, Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>, Param3: ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>, Param4: ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>>(&self, pszstreamname: Param0, pmonitor: Param1, pdata: *const u8, pusercontext: Param3, pcallback: Param4, qwcontext: u64) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), pszstreamname.into_param().abi(), pmonitor.into_param().abi(), ::std::mem::transmute(pdata), pusercontext.into_param().abi(), pcallback.into_param().abi(), ::std::mem::transmute(qwcontext)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`, `Win32_Foundation`*"]
    pub unsafe fn GetNetSourceProperties<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, pszstreamname: Param0) -> ::windows::runtime::Result<::windows::runtime::IUnknown> {
        let mut result__: <::windows::runtime::IUnknown as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self), pszstreamname.into_param().abi(), &mut result__).from_abi::<::windows::runtime::IUnknown>(result__)
    }
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
    pub unsafe fn GetNetSourceSharedNamespace(&self) -> ::windows::runtime::Result<::windows::runtime::IUnknown> {
        let mut result__: <::windows::runtime::IUnknown as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).6)(::std::mem::transmute_copy(self), &mut result__).from_abi::<::windows::runtime::IUnknown>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole_Automation`*"]
    pub unsafe fn GetNetSourceAdminInterface<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, pszstreamname: Param0) -> ::windows::runtime::Result<super::super::System::Com::VARIANT> {
        let mut result__: <super::super::System::Com::VARIANT as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self), pszstreamname.into_param().abi(), &mut result__).from_abi::<super::super::System::Com::VARIANT>(result__)
    }
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
    pub unsafe fn GetNumProtocolsSupported(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).8)(::std::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`, `Win32_Foundation`*"]
    pub unsafe fn GetProtocolName(&self, dwprotocolnum: u32, pwszprotocolname: super::super::Foundation::PWSTR, pcchprotocolname: *mut u16) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).9)(::std::mem::transmute_copy(self), ::std::mem::transmute(dwprotocolnum), ::std::mem::transmute(pwszprotocolname), ::std::mem::transmute(pcchprotocolname)).ok()
    }
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
    pub unsafe fn Shutdown(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).10)(::std::mem::transmute_copy(self)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for INSNetSourceCreator {
    type Vtable = INSNetSourceCreator_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(202260608, 36993, 4562, [190, 236, 0, 96, 8, 47, 32, 84]);
}
#[repr(C)]
#[doc(hidden)]
pub struct INSNetSourceCreator_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pszstreamname: super::super::Foundation::PWSTR, pmonitor: ::windows::runtime::RawPtr, pdata: *const u8, pusercontext: ::windows::runtime::RawPtr, pcallback: ::windows::runtime::RawPtr, qwcontext: u64) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pszstreamname: super::super::Foundation::PWSTR, pppropertiesnode: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppsharednamespace: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pszstreamname: super::super::Foundation::PWSTR, pval: *mut ::std::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation")))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pcprotocols: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwprotocolnum: u32, pwszprotocolname: super::super::Foundation::PWSTR, pcchprotocolname: *mut u16) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct INSSBuffer(::windows::runtime::IUnknown);
impl INSSBuffer {
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
    pub unsafe fn GetLength(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
    pub unsafe fn SetLength(&self, dwlength: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), ::std::mem::transmute(dwlength)).ok()
    }
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
    pub unsafe fn GetMaxLength(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
    pub unsafe fn GetBuffer(&self) -> ::windows::runtime::Result<*mut u8> {
        let mut result__: <*mut u8 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).6)(::std::mem::transmute_copy(self), &mut result__).from_abi::<*mut u8>(result__)
    }
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
    pub unsafe fn GetBufferAndLength(&self, ppdwbuffer: *mut *mut u8, pdwlength: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self), ::std::mem::transmute(ppdwbuffer), ::std::mem::transmute(pdwlength)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for INSSBuffer {
    type Vtable = INSSBuffer_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3788322084, 983, 4562, [158, 237, 0, 96, 151, 210, 215, 207]);
}
#[repr(C)]
#[doc(hidden)]
pub struct INSSBuffer_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pdwlength: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwlength: u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pdwlength: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppdwbuffer: *mut *mut u8) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppdwbuffer: *mut *mut u8, pdwlength: *mut u32) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct INSSBuffer2(::windows::runtime::IUnknown);
impl INSSBuffer2 {
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
    pub unsafe fn GetLength(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
    pub unsafe fn SetLength(&self, dwlength: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), ::std::mem::transmute(dwlength)).ok()
    }
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
    pub unsafe fn GetMaxLength(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
    pub unsafe fn GetBuffer(&self) -> ::windows::runtime::Result<*mut u8> {
        let mut result__: <*mut u8 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).6)(::std::mem::transmute_copy(self), &mut result__).from_abi::<*mut u8>(result__)
    }
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
    pub unsafe fn GetBufferAndLength(&self, ppdwbuffer: *mut *mut u8, pdwlength: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self), ::std::mem::transmute(ppdwbuffer), ::std::mem::transmute(pdwlength)).ok()
    }
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
    pub unsafe fn GetSampleProperties(&self, cbproperties: u32) -> ::windows::runtime::Result<u8> {
        let mut result__: <u8 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).8)(::std::mem::transmute_copy(self), ::std::mem::transmute(cbproperties), &mut result__).from_abi::<u8>(result__)
    }
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
    pub unsafe fn SetSampleProperties(&self, cbproperties: u32, pbproperties: *const u8) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).9)(::std::mem::transmute_copy(self), ::std::mem::transmute(cbproperties), ::std::mem::transmute(pbproperties)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for INSSBuffer2 {
    type Vtable = INSSBuffer2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1330808467, 4149, 17406, [180, 40, 117, 117, 97, 173, 58, 104]);
}
impl ::std::convert::From<INSSBuffer2> for INSSBuffer {
    fn from(value: INSSBuffer2) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&INSSBuffer2> for INSSBuffer {
    fn from(value: &INSSBuffer2) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, INSSBuffer> for INSSBuffer2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, INSSBuffer> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<INSSBuffer>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, INSSBuffer> for &INSSBuffer2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, INSSBuffer> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<INSSBuffer>::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct INSSBuffer2_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pdwlength: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwlength: u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pdwlength: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppdwbuffer: *mut *mut u8) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppdwbuffer: *mut *mut u8, pdwlength: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, cbproperties: u32, pbproperties: *mut u8) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, cbproperties: u32, pbproperties: *const u8) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct INSSBuffer3(::windows::runtime::IUnknown);
impl INSSBuffer3 {
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
    pub unsafe fn GetLength(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
    pub unsafe fn SetLength(&self, dwlength: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), ::std::mem::transmute(dwlength)).ok()
    }
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
    pub unsafe fn GetMaxLength(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
    pub unsafe fn GetBuffer(&self) -> ::windows::runtime::Result<*mut u8> {
        let mut result__: <*mut u8 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).6)(::std::mem::transmute_copy(self), &mut result__).from_abi::<*mut u8>(result__)
    }
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
    pub unsafe fn GetBufferAndLength(&self, ppdwbuffer: *mut *mut u8, pdwlength: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self), ::std::mem::transmute(ppdwbuffer), ::std::mem::transmute(pdwlength)).ok()
    }
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
    pub unsafe fn GetSampleProperties(&self, cbproperties: u32) -> ::windows::runtime::Result<u8> {
        let mut result__: <u8 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).8)(::std::mem::transmute_copy(self), ::std::mem::transmute(cbproperties), &mut result__).from_abi::<u8>(result__)
    }
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
    pub unsafe fn SetSampleProperties(&self, cbproperties: u32, pbproperties: *const u8) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).9)(::std::mem::transmute_copy(self), ::std::mem::transmute(cbproperties), ::std::mem::transmute(pbproperties)).ok()
    }
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
    pub unsafe fn SetProperty<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::GUID>>(&self, guidbufferproperty: Param0, pvbufferproperty: *const ::std::ffi::c_void, dwbufferpropertysize: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).10)(::std::mem::transmute_copy(self), guidbufferproperty.into_param().abi(), ::std::mem::transmute(pvbufferproperty), ::std::mem::transmute(dwbufferpropertysize)).ok()
    }
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
    pub unsafe fn GetProperty<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::GUID>>(&self, guidbufferproperty: Param0, pvbufferproperty: *mut ::std::ffi::c_void, pdwbufferpropertysize: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).11)(::std::mem::transmute_copy(self), guidbufferproperty.into_param().abi(), ::std::mem::transmute(pvbufferproperty), ::std::mem::transmute(pdwbufferpropertysize)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for INSSBuffer3 {
    type Vtable = INSSBuffer3_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3363629743, 30142, 19396, [132, 235, 172, 39, 152, 80, 118, 114]);
}
impl ::std::convert::From<INSSBuffer3> for INSSBuffer2 {
    fn from(value: INSSBuffer3) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&INSSBuffer3> for INSSBuffer2 {
    fn from(value: &INSSBuffer3) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, INSSBuffer2> for INSSBuffer3 {
    fn into_param(self) -> ::windows::runtime::Param<'a, INSSBuffer2> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<INSSBuffer2>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, INSSBuffer2> for &INSSBuffer3 {
    fn into_param(self) -> ::windows::runtime::Param<'a, INSSBuffer2> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<INSSBuffer2>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<INSSBuffer3> for INSSBuffer {
    fn from(value: INSSBuffer3) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&INSSBuffer3> for INSSBuffer {
    fn from(value: &INSSBuffer3) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, INSSBuffer> for INSSBuffer3 {
    fn into_param(self) -> ::windows::runtime::Param<'a, INSSBuffer> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<INSSBuffer>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, INSSBuffer> for &INSSBuffer3 {
    fn into_param(self) -> ::windows::runtime::Param<'a, INSSBuffer> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<INSSBuffer>::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct INSSBuffer3_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pdwlength: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwlength: u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pdwlength: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppdwbuffer: *mut *mut u8) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppdwbuffer: *mut *mut u8, pdwlength: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, cbproperties: u32, pbproperties: *mut u8) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, cbproperties: u32, pbproperties: *const u8) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, guidbufferproperty: ::windows::runtime::GUID, pvbufferproperty: *const ::std::ffi::c_void, dwbufferpropertysize: u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, guidbufferproperty: ::windows::runtime::GUID, pvbufferproperty: *mut ::std::ffi::c_void, pdwbufferpropertysize: *mut u32) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct INSSBuffer4(::windows::runtime::IUnknown);
impl INSSBuffer4 {
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
    pub unsafe fn GetLength(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
    pub unsafe fn SetLength(&self, dwlength: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), ::std::mem::transmute(dwlength)).ok()
    }
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
    pub unsafe fn GetMaxLength(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
    pub unsafe fn GetBuffer(&self) -> ::windows::runtime::Result<*mut u8> {
        let mut result__: <*mut u8 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).6)(::std::mem::transmute_copy(self), &mut result__).from_abi::<*mut u8>(result__)
    }
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
    pub unsafe fn GetBufferAndLength(&self, ppdwbuffer: *mut *mut u8, pdwlength: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self), ::std::mem::transmute(ppdwbuffer), ::std::mem::transmute(pdwlength)).ok()
    }
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
    pub unsafe fn GetSampleProperties(&self, cbproperties: u32) -> ::windows::runtime::Result<u8> {
        let mut result__: <u8 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).8)(::std::mem::transmute_copy(self), ::std::mem::transmute(cbproperties), &mut result__).from_abi::<u8>(result__)
    }
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
    pub unsafe fn SetSampleProperties(&self, cbproperties: u32, pbproperties: *const u8) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).9)(::std::mem::transmute_copy(self), ::std::mem::transmute(cbproperties), ::std::mem::transmute(pbproperties)).ok()
    }
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
    pub unsafe fn SetProperty<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::GUID>>(&self, guidbufferproperty: Param0, pvbufferproperty: *const ::std::ffi::c_void, dwbufferpropertysize: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).10)(::std::mem::transmute_copy(self), guidbufferproperty.into_param().abi(), ::std::mem::transmute(pvbufferproperty), ::std::mem::transmute(dwbufferpropertysize)).ok()
    }
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
    pub unsafe fn GetProperty<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::GUID>>(&self, guidbufferproperty: Param0, pvbufferproperty: *mut ::std::ffi::c_void, pdwbufferpropertysize: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).11)(::std::mem::transmute_copy(self), guidbufferproperty.into_param().abi(), ::std::mem::transmute(pvbufferproperty), ::std::mem::transmute(pdwbufferpropertysize)).ok()
    }
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
    pub unsafe fn GetPropertyCount(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).12)(::std::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
    pub unsafe fn GetPropertyByIndex(&self, dwbufferpropertyindex: u32, pguidbufferproperty: *mut ::windows::runtime::GUID, pvbufferproperty: *mut ::std::ffi::c_void, pdwbufferpropertysize: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).13)(::std::mem::transmute_copy(self), ::std::mem::transmute(dwbufferpropertyindex), ::std::mem::transmute(pguidbufferproperty), ::std::mem::transmute(pvbufferproperty), ::std::mem::transmute(pdwbufferpropertysize)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for INSSBuffer4 {
    type Vtable = INSSBuffer4_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3065576794, 13026, 18900, [169, 16, 194, 108, 200, 84, 101, 237]);
}
impl ::std::convert::From<INSSBuffer4> for INSSBuffer3 {
    fn from(value: INSSBuffer4) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&INSSBuffer4> for INSSBuffer3 {
    fn from(value: &INSSBuffer4) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, INSSBuffer3> for INSSBuffer4 {
    fn into_param(self) -> ::windows::runtime::Param<'a, INSSBuffer3> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<INSSBuffer3>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, INSSBuffer3> for &INSSBuffer4 {
    fn into_param(self) -> ::windows::runtime::Param<'a, INSSBuffer3> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<INSSBuffer3>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<INSSBuffer4> for INSSBuffer2 {
    fn from(value: INSSBuffer4) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&INSSBuffer4> for INSSBuffer2 {
    fn from(value: &INSSBuffer4) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, INSSBuffer2> for INSSBuffer4 {
    fn into_param(self) -> ::windows::runtime::Param<'a, INSSBuffer2> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<INSSBuffer2>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, INSSBuffer2> for &INSSBuffer4 {
    fn into_param(self) -> ::windows::runtime::Param<'a, INSSBuffer2> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<INSSBuffer2>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<INSSBuffer4> for INSSBuffer {
    fn from(value: INSSBuffer4) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&INSSBuffer4> for INSSBuffer {
    fn from(value: &INSSBuffer4) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, INSSBuffer> for INSSBuffer4 {
    fn into_param(self) -> ::windows::runtime::Param<'a, INSSBuffer> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<INSSBuffer>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, INSSBuffer> for &INSSBuffer4 {
    fn into_param(self) -> ::windows::runtime::Param<'a, INSSBuffer> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<INSSBuffer>::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct INSSBuffer4_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pdwlength: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwlength: u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pdwlength: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppdwbuffer: *mut *mut u8) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppdwbuffer: *mut *mut u8, pdwlength: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, cbproperties: u32, pbproperties: *mut u8) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, cbproperties: u32, pbproperties: *const u8) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, guidbufferproperty: ::windows::runtime::GUID, pvbufferproperty: *const ::std::ffi::c_void, dwbufferpropertysize: u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, guidbufferproperty: ::windows::runtime::GUID, pvbufferproperty: *mut ::std::ffi::c_void, pdwbufferpropertysize: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pcbufferproperties: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwbufferpropertyindex: u32, pguidbufferproperty: *mut ::windows::runtime::GUID, pvbufferproperty: *mut ::std::ffi::c_void, pdwbufferpropertysize: *mut u32) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct IWMAddressAccess(::windows::runtime::IUnknown);
impl IWMAddressAccess {
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
    pub unsafe fn GetAccessEntryCount(&self, aetype: WM_AETYPE) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), ::std::mem::transmute(aetype), &mut result__).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
    pub unsafe fn GetAccessEntry(&self, aetype: WM_AETYPE, dwentrynum: u32) -> ::windows::runtime::Result<WM_ADDRESS_ACCESSENTRY> {
        let mut result__: <WM_ADDRESS_ACCESSENTRY as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), ::std::mem::transmute(aetype), ::std::mem::transmute(dwentrynum), &mut result__).from_abi::<WM_ADDRESS_ACCESSENTRY>(result__)
    }
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
    pub unsafe fn AddAccessEntry(&self, aetype: WM_AETYPE, paddraccessentry: *const WM_ADDRESS_ACCESSENTRY) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self), ::std::mem::transmute(aetype), ::std::mem::transmute(paddraccessentry)).ok()
    }
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
    pub unsafe fn RemoveAccessEntry(&self, aetype: WM_AETYPE, dwentrynum: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::std::mem::transmute_copy(self), ::std::mem::transmute(aetype), ::std::mem::transmute(dwentrynum)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IWMAddressAccess {
    type Vtable = IWMAddressAccess_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3141297033, 5683, 20114, [175, 20, 159, 49, 115, 186, 57, 208]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMAddressAccess_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, aetype: WM_AETYPE, pcentries: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, aetype: WM_AETYPE, dwentrynum: u32, paddraccessentry: *mut WM_ADDRESS_ACCESSENTRY) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, aetype: WM_AETYPE, paddraccessentry: *const WM_ADDRESS_ACCESSENTRY) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, aetype: WM_AETYPE, dwentrynum: u32) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct IWMAddressAccess2(::windows::runtime::IUnknown);
impl IWMAddressAccess2 {
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
    pub unsafe fn GetAccessEntryCount(&self, aetype: WM_AETYPE) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), ::std::mem::transmute(aetype), &mut result__).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
    pub unsafe fn GetAccessEntry(&self, aetype: WM_AETYPE, dwentrynum: u32) -> ::windows::runtime::Result<WM_ADDRESS_ACCESSENTRY> {
        let mut result__: <WM_ADDRESS_ACCESSENTRY as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), ::std::mem::transmute(aetype), ::std::mem::transmute(dwentrynum), &mut result__).from_abi::<WM_ADDRESS_ACCESSENTRY>(result__)
    }
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
    pub unsafe fn AddAccessEntry(&self, aetype: WM_AETYPE, paddraccessentry: *const WM_ADDRESS_ACCESSENTRY) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self), ::std::mem::transmute(aetype), ::std::mem::transmute(paddraccessentry)).ok()
    }
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
    pub unsafe fn RemoveAccessEntry(&self, aetype: WM_AETYPE, dwentrynum: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::std::mem::transmute_copy(self), ::std::mem::transmute(aetype), ::std::mem::transmute(dwentrynum)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`, `Win32_Foundation`*"]
    pub unsafe fn GetAccessEntryEx(&self, aetype: WM_AETYPE, dwentrynum: u32, pbstraddress: *mut super::super::Foundation::BSTR, pbstrmask: *mut super::super::Foundation::BSTR) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self), ::std::mem::transmute(aetype), ::std::mem::transmute(dwentrynum), ::std::mem::transmute(pbstraddress), ::std::mem::transmute(pbstrmask)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`, `Win32_Foundation`*"]
    pub unsafe fn AddAccessEntryEx<'a, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>, Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, aetype: WM_AETYPE, bstraddress: Param1, bstrmask: Param2) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).8)(::std::mem::transmute_copy(self), ::std::mem::transmute(aetype), bstraddress.into_param().abi(), bstrmask.into_param().abi()).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IWMAddressAccess2 {
    type Vtable = IWMAddressAccess2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1705525186, 16024, 19789, [129, 181, 42, 116, 40, 134, 179, 61]);
}
impl ::std::convert::From<IWMAddressAccess2> for IWMAddressAccess {
    fn from(value: IWMAddressAccess2) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IWMAddressAccess2> for IWMAddressAccess {
    fn from(value: &IWMAddressAccess2) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IWMAddressAccess> for IWMAddressAccess2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IWMAddressAccess> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IWMAddressAccess>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IWMAddressAccess> for &IWMAddressAccess2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IWMAddressAccess> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IWMAddressAccess>::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMAddressAccess2_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, aetype: WM_AETYPE, pcentries: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, aetype: WM_AETYPE, dwentrynum: u32, paddraccessentry: *mut WM_ADDRESS_ACCESSENTRY) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, aetype: WM_AETYPE, paddraccessentry: *const WM_ADDRESS_ACCESSENTRY) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, aetype: WM_AETYPE, dwentrynum: u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, aetype: WM_AETYPE, dwentrynum: u32, pbstraddress: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>, pbstrmask: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, aetype: WM_AETYPE, bstraddress: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrmask: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct IWMAuthorizer(::windows::runtime::IUnknown);
impl IWMAuthorizer {
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
    pub unsafe fn GetCertCount(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
    pub unsafe fn GetCert(&self, dwindex: u32) -> ::windows::runtime::Result<*mut u8> {
        let mut result__: <*mut u8 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), ::std::mem::transmute(dwindex), &mut result__).from_abi::<*mut u8>(result__)
    }
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
    pub unsafe fn GetSharedData(&self, dwcertindex: u32, pbshareddata: *const u8, pbcert: *const u8) -> ::windows::runtime::Result<*mut u8> {
        let mut result__: <*mut u8 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self), ::std::mem::transmute(dwcertindex), ::std::mem::transmute(pbshareddata), ::std::mem::transmute(pbcert), &mut result__).from_abi::<*mut u8>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IWMAuthorizer {
    type Vtable = IWMAuthorizer_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3652615478, 43437, 20148, [186, 239, 219, 40, 78, 245, 80, 76]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMAuthorizer_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pccerts: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwindex: u32, ppbcertdata: *mut *mut u8) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwcertindex: u32, pbshareddata: *const u8, pbcert: *const u8, ppbshareddata: *mut *mut u8) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct IWMBackupRestoreProps(::windows::runtime::IUnknown);
impl IWMBackupRestoreProps {
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
    pub unsafe fn GetPropCount(&self) -> ::windows::runtime::Result<u16> {
        let mut result__: <u16 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), &mut result__).from_abi::<u16>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`, `Win32_Foundation`*"]
    pub unsafe fn GetPropByIndex(&self, windex: u16, pwszname: super::super::Foundation::PWSTR, pcchnamelen: *mut u16, ptype: *mut WMT_ATTR_DATATYPE, pvalue: *mut u8, pcblength: *mut u16) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), ::std::mem::transmute(windex), ::std::mem::transmute(pwszname), ::std::mem::transmute(pcchnamelen), ::std::mem::transmute(ptype), ::std::mem::transmute(pvalue), ::std::mem::transmute(pcblength)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`, `Win32_Foundation`*"]
    pub unsafe fn GetPropByName<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, pszname: Param0, ptype: *mut WMT_ATTR_DATATYPE, pvalue: *mut u8, pcblength: *mut u16) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self), pszname.into_param().abi(), ::std::mem::transmute(ptype), ::std::mem::transmute(pvalue), ::std::mem::transmute(pcblength)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`, `Win32_Foundation`*"]
    pub unsafe fn SetProp<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, pszname: Param0, r#type: WMT_ATTR_DATATYPE, pvalue: *const u8, cblength: u16) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::std::mem::transmute_copy(self), pszname.into_param().abi(), ::std::mem::transmute(r#type), ::std::mem::transmute(pvalue), ::std::mem::transmute(cblength)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`, `Win32_Foundation`*"]
    pub unsafe fn RemoveProp<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, pcwszname: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self), pcwszname.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
    pub unsafe fn RemoveAllProps(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).8)(::std::mem::transmute_copy(self)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IWMBackupRestoreProps {
    type Vtable = IWMBackupRestoreProps_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1015942566, 39279, 20467, [161, 175, 72, 56, 249, 55, 126, 46]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMBackupRestoreProps_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pcprops: *mut u16) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, windex: u16, pwszname: super::super::Foundation::PWSTR, pcchnamelen: *mut u16, ptype: *mut WMT_ATTR_DATATYPE, pvalue: *mut u8, pcblength: *mut u16) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pszname: super::super::Foundation::PWSTR, ptype: *mut WMT_ATTR_DATATYPE, pvalue: *mut u8, pcblength: *mut u16) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pszname: super::super::Foundation::PWSTR, r#type: WMT_ATTR_DATATYPE, pvalue: *const u8, cblength: u16) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pcwszname: super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct IWMBandwidthSharing(::windows::runtime::IUnknown);
impl IWMBandwidthSharing {
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
    pub unsafe fn GetStreams(&self, pwstreamnumarray: *mut u16, pcstreams: *mut u16) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), ::std::mem::transmute(pwstreamnumarray), ::std::mem::transmute(pcstreams)).ok()
    }
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
    pub unsafe fn AddStream(&self, wstreamnum: u16) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), ::std::mem::transmute(wstreamnum)).ok()
    }
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
    pub unsafe fn RemoveStream(&self, wstreamnum: u16) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self), ::std::mem::transmute(wstreamnum)).ok()
    }
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
    pub unsafe fn GetType(&self) -> ::windows::runtime::Result<::windows::runtime::GUID> {
        let mut result__: <::windows::runtime::GUID as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).6)(::std::mem::transmute_copy(self), &mut result__).from_abi::<::windows::runtime::GUID>(result__)
    }
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
    pub unsafe fn SetType(&self, guidtype: *const ::windows::runtime::GUID) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self), ::std::mem::transmute(guidtype)).ok()
    }
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
    pub unsafe fn GetBandwidth(&self, pdwbitrate: *mut u32, pmsbufferwindow: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).8)(::std::mem::transmute_copy(self), ::std::mem::transmute(pdwbitrate), ::std::mem::transmute(pmsbufferwindow)).ok()
    }
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
    pub unsafe fn SetBandwidth(&self, dwbitrate: u32, msbufferwindow: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).9)(::std::mem::transmute_copy(self), ::std::mem::transmute(dwbitrate), ::std::mem::transmute(msbufferwindow)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IWMBandwidthSharing {
    type Vtable = IWMBandwidthSharing_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2909358833, 63705, 17144, [188, 71, 112, 49, 27, 12, 79, 158]);
}
impl ::std::convert::From<IWMBandwidthSharing> for IWMStreamList {
    fn from(value: IWMBandwidthSharing) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IWMBandwidthSharing> for IWMStreamList {
    fn from(value: &IWMBandwidthSharing) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IWMStreamList> for IWMBandwidthSharing {
    fn into_param(self) -> ::windows::runtime::Param<'a, IWMStreamList> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IWMStreamList>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IWMStreamList> for &IWMBandwidthSharing {
    fn into_param(self) -> ::windows::runtime::Param<'a, IWMStreamList> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IWMStreamList>::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMBandwidthSharing_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pwstreamnumarray: *mut u16, pcstreams: *mut u16) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, wstreamnum: u16) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, wstreamnum: u16) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pguidtype: *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, guidtype: *const ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pdwbitrate: *mut u32, pmsbufferwindow: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwbitrate: u32, msbufferwindow: u32) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct IWMClientConnections(::windows::runtime::IUnknown);
impl IWMClientConnections {
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
    pub unsafe fn GetClientCount(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
    pub unsafe fn GetClientProperties(&self, dwclientnum: u32) -> ::windows::runtime::Result<WM_CLIENT_PROPERTIES> {
        let mut result__: <WM_CLIENT_PROPERTIES as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), ::std::mem::transmute(dwclientnum), &mut result__).from_abi::<WM_CLIENT_PROPERTIES>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IWMClientConnections {
    type Vtable = IWMClientConnections_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1942380560, 41625, 16863, [177, 240, 204, 240, 59, 9, 193, 198]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMClientConnections_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pcclients: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwclientnum: u32, pclientproperties: *mut WM_CLIENT_PROPERTIES) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct IWMClientConnections2(::windows::runtime::IUnknown);
impl IWMClientConnections2 {
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
    pub unsafe fn GetClientCount(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
    pub unsafe fn GetClientProperties(&self, dwclientnum: u32) -> ::windows::runtime::Result<WM_CLIENT_PROPERTIES> {
        let mut result__: <WM_CLIENT_PROPERTIES as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), ::std::mem::transmute(dwclientnum), &mut result__).from_abi::<WM_CLIENT_PROPERTIES>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`, `Win32_Foundation`*"]
    pub unsafe fn GetClientInfo(&self, dwclientnum: u32, pwsznetworkaddress: super::super::Foundation::PWSTR, pcchnetworkaddress: *mut u32, pwszport: super::super::Foundation::PWSTR, pcchport: *mut u32, pwszdnsname: super::super::Foundation::PWSTR, pcchdnsname: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self), ::std::mem::transmute(dwclientnum), ::std::mem::transmute(pwsznetworkaddress), ::std::mem::transmute(pcchnetworkaddress), ::std::mem::transmute(pwszport), ::std::mem::transmute(pcchport), ::std::mem::transmute(pwszdnsname), ::std::mem::transmute(pcchdnsname)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IWMClientConnections2 {
    type Vtable = IWMClientConnections2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1083266846, 18177, 17811, [187, 61, 213, 245, 240, 199, 66, 70]);
}
impl ::std::convert::From<IWMClientConnections2> for IWMClientConnections {
    fn from(value: IWMClientConnections2) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IWMClientConnections2> for IWMClientConnections {
    fn from(value: &IWMClientConnections2) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IWMClientConnections> for IWMClientConnections2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IWMClientConnections> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IWMClientConnections>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IWMClientConnections> for &IWMClientConnections2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IWMClientConnections> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IWMClientConnections>::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMClientConnections2_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pcclients: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwclientnum: u32, pclientproperties: *mut WM_CLIENT_PROPERTIES) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwclientnum: u32, pwsznetworkaddress: super::super::Foundation::PWSTR, pcchnetworkaddress: *mut u32, pwszport: super::super::Foundation::PWSTR, pcchport: *mut u32, pwszdnsname: super::super::Foundation::PWSTR, pcchdnsname: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct IWMCodecAMVideoAccelerator(::windows::runtime::IUnknown);
impl IWMCodecAMVideoAccelerator {
    #[cfg(feature = "Win32_Graphics_DirectShow")]
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`, `Win32_Graphics_DirectShow`*"]
    pub unsafe fn SetAcceleratorInterface<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Graphics::DirectShow::IAMVideoAccelerator>>(&self, piamva: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), piamva.into_param().abi()).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_DirectShow"))]
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`, `Win32_Foundation`, `Win32_Graphics_DirectShow`*"]
    pub unsafe fn NegotiateConnection(&self, pmediatype: *const super::super::Graphics::DirectShow::AM_MEDIA_TYPE) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), ::std::mem::transmute(pmediatype)).ok()
    }
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
    pub unsafe fn SetPlayerNotify<'a, Param0: ::windows::runtime::IntoParam<'a, IWMPlayerTimestampHook>>(&self, phook: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self), phook.into_param().abi()).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IWMCodecAMVideoAccelerator {
    type Vtable = IWMCodecAMVideoAccelerator_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3650019921, 13536, 18989, [147, 18, 155, 76, 120, 141, 159, 161]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMCodecAMVideoAccelerator_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_Graphics_DirectShow")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, piamva: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_DirectShow"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_DirectShow"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pmediatype: *const ::std::mem::ManuallyDrop<super::super::Graphics::DirectShow::AM_MEDIA_TYPE>) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Graphics_DirectShow")))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, phook: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct IWMCodecInfo(::windows::runtime::IUnknown);
impl IWMCodecInfo {
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
    pub unsafe fn GetCodecInfoCount(&self, guidtype: *const ::windows::runtime::GUID) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), ::std::mem::transmute(guidtype), &mut result__).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
    pub unsafe fn GetCodecFormatCount(&self, guidtype: *const ::windows::runtime::GUID, dwcodecindex: u32) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), ::std::mem::transmute(guidtype), ::std::mem::transmute(dwcodecindex), &mut result__).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
    pub unsafe fn GetCodecFormat(&self, guidtype: *const ::windows::runtime::GUID, dwcodecindex: u32, dwformatindex: u32) -> ::windows::runtime::Result<IWMStreamConfig> {
        let mut result__: <IWMStreamConfig as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self), ::std::mem::transmute(guidtype), ::std::mem::transmute(dwcodecindex), ::std::mem::transmute(dwformatindex), &mut result__).from_abi::<IWMStreamConfig>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IWMCodecInfo {
    type Vtable = IWMCodecInfo_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2842752030, 13534, 19096, [179, 186, 228, 179, 202, 117, 40, 240]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMCodecInfo_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, guidtype: *const ::windows::runtime::GUID, pccodecs: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, guidtype: *const ::windows::runtime::GUID, dwcodecindex: u32, pcformat: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, guidtype: *const ::windows::runtime::GUID, dwcodecindex: u32, dwformatindex: u32, ppistreamconfig: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct IWMCodecInfo2(::windows::runtime::IUnknown);
impl IWMCodecInfo2 {
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
    pub unsafe fn GetCodecInfoCount(&self, guidtype: *const ::windows::runtime::GUID) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), ::std::mem::transmute(guidtype), &mut result__).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
    pub unsafe fn GetCodecFormatCount(&self, guidtype: *const ::windows::runtime::GUID, dwcodecindex: u32) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), ::std::mem::transmute(guidtype), ::std::mem::transmute(dwcodecindex), &mut result__).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
    pub unsafe fn GetCodecFormat(&self, guidtype: *const ::windows::runtime::GUID, dwcodecindex: u32, dwformatindex: u32) -> ::windows::runtime::Result<IWMStreamConfig> {
        let mut result__: <IWMStreamConfig as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self), ::std::mem::transmute(guidtype), ::std::mem::transmute(dwcodecindex), ::std::mem::transmute(dwformatindex), &mut result__).from_abi::<IWMStreamConfig>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`, `Win32_Foundation`*"]
    pub unsafe fn GetCodecName(&self, guidtype: *const ::windows::runtime::GUID, dwcodecindex: u32, wszname: super::super::Foundation::PWSTR, pcchname: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::std::mem::transmute_copy(self), ::std::mem::transmute(guidtype), ::std::mem::transmute(dwcodecindex), ::std::mem::transmute(wszname), ::std::mem::transmute(pcchname)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`, `Win32_Foundation`*"]
    pub unsafe fn GetCodecFormatDesc(&self, guidtype: *const ::windows::runtime::GUID, dwcodecindex: u32, dwformatindex: u32, ppistreamconfig: *mut ::std::option::Option<IWMStreamConfig>, wszdesc: super::super::Foundation::PWSTR, pcchdesc: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self), ::std::mem::transmute(guidtype), ::std::mem::transmute(dwcodecindex), ::std::mem::transmute(dwformatindex), ::std::mem::transmute(ppistreamconfig), ::std::mem::transmute(wszdesc), ::std::mem::transmute(pcchdesc)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IWMCodecInfo2 {
    type Vtable = IWMCodecInfo2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2858803827, 46726, 16470, [145, 236, 221, 118, 141, 77, 247, 16]);
}
impl ::std::convert::From<IWMCodecInfo2> for IWMCodecInfo {
    fn from(value: IWMCodecInfo2) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IWMCodecInfo2> for IWMCodecInfo {
    fn from(value: &IWMCodecInfo2) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IWMCodecInfo> for IWMCodecInfo2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IWMCodecInfo> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IWMCodecInfo>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IWMCodecInfo> for &IWMCodecInfo2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IWMCodecInfo> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IWMCodecInfo>::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMCodecInfo2_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, guidtype: *const ::windows::runtime::GUID, pccodecs: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, guidtype: *const ::windows::runtime::GUID, dwcodecindex: u32, pcformat: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, guidtype: *const ::windows::runtime::GUID, dwcodecindex: u32, dwformatindex: u32, ppistreamconfig: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, guidtype: *const ::windows::runtime::GUID, dwcodecindex: u32, wszname: super::super::Foundation::PWSTR, pcchname: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, guidtype: *const ::windows::runtime::GUID, dwcodecindex: u32, dwformatindex: u32, ppistreamconfig: *mut ::windows::runtime::RawPtr, wszdesc: super::super::Foundation::PWSTR, pcchdesc: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct IWMCodecInfo3(::windows::runtime::IUnknown);
impl IWMCodecInfo3 {
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
    pub unsafe fn GetCodecInfoCount(&self, guidtype: *const ::windows::runtime::GUID) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), ::std::mem::transmute(guidtype), &mut result__).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
    pub unsafe fn GetCodecFormatCount(&self, guidtype: *const ::windows::runtime::GUID, dwcodecindex: u32) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), ::std::mem::transmute(guidtype), ::std::mem::transmute(dwcodecindex), &mut result__).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
    pub unsafe fn GetCodecFormat(&self, guidtype: *const ::windows::runtime::GUID, dwcodecindex: u32, dwformatindex: u32) -> ::windows::runtime::Result<IWMStreamConfig> {
        let mut result__: <IWMStreamConfig as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self), ::std::mem::transmute(guidtype), ::std::mem::transmute(dwcodecindex), ::std::mem::transmute(dwformatindex), &mut result__).from_abi::<IWMStreamConfig>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`, `Win32_Foundation`*"]
    pub unsafe fn GetCodecName(&self, guidtype: *const ::windows::runtime::GUID, dwcodecindex: u32, wszname: super::super::Foundation::PWSTR, pcchname: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::std::mem::transmute_copy(self), ::std::mem::transmute(guidtype), ::std::mem::transmute(dwcodecindex), ::std::mem::transmute(wszname), ::std::mem::transmute(pcchname)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`, `Win32_Foundation`*"]
    pub unsafe fn GetCodecFormatDesc(&self, guidtype: *const ::windows::runtime::GUID, dwcodecindex: u32, dwformatindex: u32, ppistreamconfig: *mut ::std::option::Option<IWMStreamConfig>, wszdesc: super::super::Foundation::PWSTR, pcchdesc: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self), ::std::mem::transmute(guidtype), ::std::mem::transmute(dwcodecindex), ::std::mem::transmute(dwformatindex), ::std::mem::transmute(ppistreamconfig), ::std::mem::transmute(wszdesc), ::std::mem::transmute(pcchdesc)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`, `Win32_Foundation`*"]
    pub unsafe fn GetCodecFormatProp<'a, Param3: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, guidtype: *const ::windows::runtime::GUID, dwcodecindex: u32, dwformatindex: u32, pszname: Param3, ptype: *mut WMT_ATTR_DATATYPE, pvalue: *mut u8, pdwsize: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).8)(::std::mem::transmute_copy(self), ::std::mem::transmute(guidtype), ::std::mem::transmute(dwcodecindex), ::std::mem::transmute(dwformatindex), pszname.into_param().abi(), ::std::mem::transmute(ptype), ::std::mem::transmute(pvalue), ::std::mem::transmute(pdwsize)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`, `Win32_Foundation`*"]
    pub unsafe fn GetCodecProp<'a, Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, guidtype: *const ::windows::runtime::GUID, dwcodecindex: u32, pszname: Param2, ptype: *mut WMT_ATTR_DATATYPE, pvalue: *mut u8, pdwsize: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).9)(::std::mem::transmute_copy(self), ::std::mem::transmute(guidtype), ::std::mem::transmute(dwcodecindex), pszname.into_param().abi(), ::std::mem::transmute(ptype), ::std::mem::transmute(pvalue), ::std::mem::transmute(pdwsize)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`, `Win32_Foundation`*"]
    pub unsafe fn SetCodecEnumerationSetting<'a, Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, guidtype: *const ::windows::runtime::GUID, dwcodecindex: u32, pszname: Param2, r#type: WMT_ATTR_DATATYPE, pvalue: *const u8, dwsize: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).10)(::std::mem::transmute_copy(self), ::std::mem::transmute(guidtype), ::std::mem::transmute(dwcodecindex), pszname.into_param().abi(), ::std::mem::transmute(r#type), ::std::mem::transmute(pvalue), ::std::mem::transmute(dwsize)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`, `Win32_Foundation`*"]
    pub unsafe fn GetCodecEnumerationSetting<'a, Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, guidtype: *const ::windows::runtime::GUID, dwcodecindex: u32, pszname: Param2, ptype: *mut WMT_ATTR_DATATYPE, pvalue: *mut u8, pdwsize: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).11)(::std::mem::transmute_copy(self), ::std::mem::transmute(guidtype), ::std::mem::transmute(dwcodecindex), pszname.into_param().abi(), ::std::mem::transmute(ptype), ::std::mem::transmute(pvalue), ::std::mem::transmute(pdwsize)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IWMCodecInfo3 {
    type Vtable = IWMCodecInfo3_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2119300231, 19859, 20376, [138, 180, 39, 208, 86, 90, 220, 81]);
}
impl ::std::convert::From<IWMCodecInfo3> for IWMCodecInfo2 {
    fn from(value: IWMCodecInfo3) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IWMCodecInfo3> for IWMCodecInfo2 {
    fn from(value: &IWMCodecInfo3) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IWMCodecInfo2> for IWMCodecInfo3 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IWMCodecInfo2> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IWMCodecInfo2>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IWMCodecInfo2> for &IWMCodecInfo3 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IWMCodecInfo2> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IWMCodecInfo2>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<IWMCodecInfo3> for IWMCodecInfo {
    fn from(value: IWMCodecInfo3) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IWMCodecInfo3> for IWMCodecInfo {
    fn from(value: &IWMCodecInfo3) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IWMCodecInfo> for IWMCodecInfo3 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IWMCodecInfo> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IWMCodecInfo>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IWMCodecInfo> for &IWMCodecInfo3 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IWMCodecInfo> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IWMCodecInfo>::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMCodecInfo3_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, guidtype: *const ::windows::runtime::GUID, pccodecs: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, guidtype: *const ::windows::runtime::GUID, dwcodecindex: u32, pcformat: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, guidtype: *const ::windows::runtime::GUID, dwcodecindex: u32, dwformatindex: u32, ppistreamconfig: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, guidtype: *const ::windows::runtime::GUID, dwcodecindex: u32, wszname: super::super::Foundation::PWSTR, pcchname: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, guidtype: *const ::windows::runtime::GUID, dwcodecindex: u32, dwformatindex: u32, ppistreamconfig: *mut ::windows::runtime::RawPtr, wszdesc: super::super::Foundation::PWSTR, pcchdesc: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, guidtype: *const ::windows::runtime::GUID, dwcodecindex: u32, dwformatindex: u32, pszname: super::super::Foundation::PWSTR, ptype: *mut WMT_ATTR_DATATYPE, pvalue: *mut u8, pdwsize: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, guidtype: *const ::windows::runtime::GUID, dwcodecindex: u32, pszname: super::super::Foundation::PWSTR, ptype: *mut WMT_ATTR_DATATYPE, pvalue: *mut u8, pdwsize: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, guidtype: *const ::windows::runtime::GUID, dwcodecindex: u32, pszname: super::super::Foundation::PWSTR, r#type: WMT_ATTR_DATATYPE, pvalue: *const u8, dwsize: u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, guidtype: *const ::windows::runtime::GUID, dwcodecindex: u32, pszname: super::super::Foundation::PWSTR, ptype: *mut WMT_ATTR_DATATYPE, pvalue: *mut u8, pdwsize: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct IWMCodecVideoAccelerator(::windows::runtime::IUnknown);
impl IWMCodecVideoAccelerator {
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_DirectShow"))]
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`, `Win32_Foundation`, `Win32_Graphics_DirectShow`*"]
    pub unsafe fn NegotiateConnection<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Graphics::DirectShow::IAMVideoAccelerator>>(&self, piamva: Param0, pmediatype: *const super::super::Graphics::DirectShow::AM_MEDIA_TYPE) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), piamva.into_param().abi(), ::std::mem::transmute(pmediatype)).ok()
    }
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
    pub unsafe fn SetPlayerNotify<'a, Param0: ::windows::runtime::IntoParam<'a, IWMPlayerTimestampHook>>(&self, phook: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), phook.into_param().abi()).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IWMCodecVideoAccelerator {
    type Vtable = IWMCodecVideoAccelerator_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2567324080, 29599, 20116, [168, 8, 152, 136, 218, 143, 117, 175]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMCodecVideoAccelerator_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_DirectShow"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, piamva: ::windows::runtime::RawPtr, pmediatype: *const ::std::mem::ManuallyDrop<super::super::Graphics::DirectShow::AM_MEDIA_TYPE>) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Graphics_DirectShow")))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, phook: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct IWMCredentialCallback(::windows::runtime::IUnknown);
impl IWMCredentialCallback {
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`, `Win32_Foundation`*"]
    pub unsafe fn AcquireCredentials<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, pwszrealm: Param0, pwszsite: Param1, pwszuser: super::super::Foundation::PWSTR, cchuser: u32, pwszpassword: super::super::Foundation::PWSTR, cchpassword: u32, hrstatus: ::windows::runtime::HRESULT, pdwflags: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), pwszrealm.into_param().abi(), pwszsite.into_param().abi(), ::std::mem::transmute(pwszuser), ::std::mem::transmute(cchuser), ::std::mem::transmute(pwszpassword), ::std::mem::transmute(cchpassword), ::std::mem::transmute(hrstatus), ::std::mem::transmute(pdwflags)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IWMCredentialCallback {
    type Vtable = IWMCredentialCallback_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(875433655, 58961, 17676, [151, 91, 42, 206, 44, 144, 196, 142]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMCredentialCallback_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pwszrealm: super::super::Foundation::PWSTR, pwszsite: super::super::Foundation::PWSTR, pwszuser: super::super::Foundation::PWSTR, cchuser: u32, pwszpassword: super::super::Foundation::PWSTR, cchpassword: u32, hrstatus: ::windows::runtime::HRESULT, pdwflags: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct IWMDRMEditor(::windows::runtime::IUnknown);
impl IWMDRMEditor {
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`, `Win32_Foundation`*"]
    pub unsafe fn GetDRMProperty<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, pwstrname: Param0, pdwtype: *mut WMT_ATTR_DATATYPE, pvalue: *mut u8, pcblength: *mut u16) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), pwstrname.into_param().abi(), ::std::mem::transmute(pdwtype), ::std::mem::transmute(pvalue), ::std::mem::transmute(pcblength)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IWMDRMEditor {
    type Vtable = IWMDRMEditor_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(4279439036, 42691, 17062, [180, 1, 195, 56, 44, 62, 8, 179]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMDRMEditor_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pwstrname: super::super::Foundation::PWSTR, pdwtype: *mut WMT_ATTR_DATATYPE, pvalue: *mut u8, pcblength: *mut u16) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct IWMDRMMessageParser(::windows::runtime::IUnknown);
impl IWMDRMMessageParser {
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
    pub unsafe fn ParseRegistrationReqMsg(&self, pbregistrationreqmsg: *const u8, cbregistrationreqmsg: u32, ppdevicecert: *mut ::std::option::Option<INSSBuffer>, pdeviceserialnumber: *mut DRM_VAL16) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), ::std::mem::transmute(pbregistrationreqmsg), ::std::mem::transmute(cbregistrationreqmsg), ::std::mem::transmute(ppdevicecert), ::std::mem::transmute(pdeviceserialnumber)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`, `Win32_Foundation`*"]
    pub unsafe fn ParseLicenseRequestMsg(&self, pblicenserequestmsg: *const u8, cblicenserequestmsg: u32, ppdevicecert: *mut ::std::option::Option<INSSBuffer>, pdeviceserialnumber: *mut DRM_VAL16, pbstraction: *mut super::super::Foundation::BSTR) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), ::std::mem::transmute(pblicenserequestmsg), ::std::mem::transmute(cblicenserequestmsg), ::std::mem::transmute(ppdevicecert), ::std::mem::transmute(pdeviceserialnumber), ::std::mem::transmute(pbstraction)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IWMDRMMessageParser {
    type Vtable = IWMDRMMessageParser_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2805596274, 9632, 19609, [180, 165, 237, 232, 16, 26, 108, 57]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMDRMMessageParser_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbregistrationreqmsg: *const u8, cbregistrationreqmsg: u32, ppdevicecert: *mut ::windows::runtime::RawPtr, pdeviceserialnumber: *mut DRM_VAL16) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pblicenserequestmsg: *const u8, cblicenserequestmsg: u32, ppdevicecert: *mut ::windows::runtime::RawPtr, pdeviceserialnumber: *mut DRM_VAL16, pbstraction: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct IWMDRMReader(::windows::runtime::IUnknown);
impl IWMDRMReader {
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
    pub unsafe fn AcquireLicense(&self, dwflags: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), ::std::mem::transmute(dwflags)).ok()
    }
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
    pub unsafe fn CancelLicenseAcquisition(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
    pub unsafe fn Individualize(&self, dwflags: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self), ::std::mem::transmute(dwflags)).ok()
    }
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
    pub unsafe fn CancelIndividualization(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::std::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
    pub unsafe fn MonitorLicenseAcquisition(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
    pub unsafe fn CancelMonitorLicenseAcquisition(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).8)(::std::mem::transmute_copy(self)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`, `Win32_Foundation`*"]
    pub unsafe fn SetDRMProperty<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, pwstrname: Param0, dwtype: WMT_ATTR_DATATYPE, pvalue: *const u8, cblength: u16) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).9)(::std::mem::transmute_copy(self), pwstrname.into_param().abi(), ::std::mem::transmute(dwtype), ::std::mem::transmute(pvalue), ::std::mem::transmute(cblength)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`, `Win32_Foundation`*"]
    pub unsafe fn GetDRMProperty<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, pwstrname: Param0, pdwtype: *mut WMT_ATTR_DATATYPE, pvalue: *mut u8, pcblength: *mut u16) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).10)(::std::mem::transmute_copy(self), pwstrname.into_param().abi(), ::std::mem::transmute(pdwtype), ::std::mem::transmute(pvalue), ::std::mem::transmute(pcblength)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IWMDRMReader {
    type Vtable = IWMDRMReader_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3531765056, 16103, 17196, [177, 76, 220, 23, 240, 133, 211, 179]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMDRMReader_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwflags: u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwflags: u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pwstrname: super::super::Foundation::PWSTR, dwtype: WMT_ATTR_DATATYPE, pvalue: *const u8, cblength: u16) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pwstrname: super::super::Foundation::PWSTR, pdwtype: *mut WMT_ATTR_DATATYPE, pvalue: *mut u8, pcblength: *mut u16) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct IWMDRMReader2(::windows::runtime::IUnknown);
impl IWMDRMReader2 {
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
    pub unsafe fn AcquireLicense(&self, dwflags: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), ::std::mem::transmute(dwflags)).ok()
    }
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
    pub unsafe fn CancelLicenseAcquisition(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
    pub unsafe fn Individualize(&self, dwflags: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self), ::std::mem::transmute(dwflags)).ok()
    }
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
    pub unsafe fn CancelIndividualization(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::std::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
    pub unsafe fn MonitorLicenseAcquisition(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
    pub unsafe fn CancelMonitorLicenseAcquisition(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).8)(::std::mem::transmute_copy(self)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`, `Win32_Foundation`*"]
    pub unsafe fn SetDRMProperty<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, pwstrname: Param0, dwtype: WMT_ATTR_DATATYPE, pvalue: *const u8, cblength: u16) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).9)(::std::mem::transmute_copy(self), pwstrname.into_param().abi(), ::std::mem::transmute(dwtype), ::std::mem::transmute(pvalue), ::std::mem::transmute(cblength)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`, `Win32_Foundation`*"]
    pub unsafe fn GetDRMProperty<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, pwstrname: Param0, pdwtype: *mut WMT_ATTR_DATATYPE, pvalue: *mut u8, pcblength: *mut u16) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).10)(::std::mem::transmute_copy(self), pwstrname.into_param().abi(), ::std::mem::transmute(pdwtype), ::std::mem::transmute(pvalue), ::std::mem::transmute(pcblength)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`, `Win32_Foundation`*"]
    pub unsafe fn SetEvaluateOutputLevelLicenses<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>>(&self, fevaluate: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).11)(::std::mem::transmute_copy(self), fevaluate.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
    pub unsafe fn GetPlayOutputLevels(&self, pplayopl: *mut DRM_PLAY_OPL, pcblength: *mut u32, pdwminappcompliancelevel: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).12)(::std::mem::transmute_copy(self), ::std::mem::transmute(pplayopl), ::std::mem::transmute(pcblength), ::std::mem::transmute(pdwminappcompliancelevel)).ok()
    }
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
    pub unsafe fn GetCopyOutputLevels(&self, pcopyopl: *mut DRM_COPY_OPL, pcblength: *mut u32, pdwminappcompliancelevel: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).13)(::std::mem::transmute_copy(self), ::std::mem::transmute(pcopyopl), ::std::mem::transmute(pcblength), ::std::mem::transmute(pdwminappcompliancelevel)).ok()
    }
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
    pub unsafe fn TryNextLicense(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).14)(::std::mem::transmute_copy(self)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IWMDRMReader2 {
    type Vtable = IWMDRMReader2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3204348533, 40733, 16501, [185, 217, 163, 195, 123, 218, 73, 160]);
}
impl ::std::convert::From<IWMDRMReader2> for IWMDRMReader {
    fn from(value: IWMDRMReader2) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IWMDRMReader2> for IWMDRMReader {
    fn from(value: &IWMDRMReader2) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IWMDRMReader> for IWMDRMReader2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IWMDRMReader> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IWMDRMReader>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IWMDRMReader> for &IWMDRMReader2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IWMDRMReader> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IWMDRMReader>::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMDRMReader2_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwflags: u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwflags: u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pwstrname: super::super::Foundation::PWSTR, dwtype: WMT_ATTR_DATATYPE, pvalue: *const u8, cblength: u16) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pwstrname: super::super::Foundation::PWSTR, pdwtype: *mut WMT_ATTR_DATATYPE, pvalue: *mut u8, pcblength: *mut u16) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, fevaluate: super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pplayopl: *mut DRM_PLAY_OPL, pcblength: *mut u32, pdwminappcompliancelevel: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pcopyopl: *mut DRM_COPY_OPL, pcblength: *mut u32, pdwminappcompliancelevel: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct IWMDRMReader3(::windows::runtime::IUnknown);
impl IWMDRMReader3 {
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
    pub unsafe fn AcquireLicense(&self, dwflags: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), ::std::mem::transmute(dwflags)).ok()
    }
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
    pub unsafe fn CancelLicenseAcquisition(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
    pub unsafe fn Individualize(&self, dwflags: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self), ::std::mem::transmute(dwflags)).ok()
    }
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
    pub unsafe fn CancelIndividualization(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::std::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
    pub unsafe fn MonitorLicenseAcquisition(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
    pub unsafe fn CancelMonitorLicenseAcquisition(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).8)(::std::mem::transmute_copy(self)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`, `Win32_Foundation`*"]
    pub unsafe fn SetDRMProperty<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, pwstrname: Param0, dwtype: WMT_ATTR_DATATYPE, pvalue: *const u8, cblength: u16) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).9)(::std::mem::transmute_copy(self), pwstrname.into_param().abi(), ::std::mem::transmute(dwtype), ::std::mem::transmute(pvalue), ::std::mem::transmute(cblength)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`, `Win32_Foundation`*"]
    pub unsafe fn GetDRMProperty<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, pwstrname: Param0, pdwtype: *mut WMT_ATTR_DATATYPE, pvalue: *mut u8, pcblength: *mut u16) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).10)(::std::mem::transmute_copy(self), pwstrname.into_param().abi(), ::std::mem::transmute(pdwtype), ::std::mem::transmute(pvalue), ::std::mem::transmute(pcblength)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`, `Win32_Foundation`*"]
    pub unsafe fn SetEvaluateOutputLevelLicenses<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>>(&self, fevaluate: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).11)(::std::mem::transmute_copy(self), fevaluate.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
    pub unsafe fn GetPlayOutputLevels(&self, pplayopl: *mut DRM_PLAY_OPL, pcblength: *mut u32, pdwminappcompliancelevel: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).12)(::std::mem::transmute_copy(self), ::std::mem::transmute(pplayopl), ::std::mem::transmute(pcblength), ::std::mem::transmute(pdwminappcompliancelevel)).ok()
    }
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
    pub unsafe fn GetCopyOutputLevels(&self, pcopyopl: *mut DRM_COPY_OPL, pcblength: *mut u32, pdwminappcompliancelevel: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).13)(::std::mem::transmute_copy(self), ::std::mem::transmute(pcopyopl), ::std::mem::transmute(pcblength), ::std::mem::transmute(pdwminappcompliancelevel)).ok()
    }
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
    pub unsafe fn TryNextLicense(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).14)(::std::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
    pub unsafe fn GetInclusionList(&self, ppguids: *mut *mut ::windows::runtime::GUID, pcguids: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).15)(::std::mem::transmute_copy(self), ::std::mem::transmute(ppguids), ::std::mem::transmute(pcguids)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IWMDRMReader3 {
    type Vtable = IWMDRMReader3_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3766907614, 61927, 20468, [160, 163, 252, 75, 8, 228, 202, 248]);
}
impl ::std::convert::From<IWMDRMReader3> for IWMDRMReader2 {
    fn from(value: IWMDRMReader3) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IWMDRMReader3> for IWMDRMReader2 {
    fn from(value: &IWMDRMReader3) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IWMDRMReader2> for IWMDRMReader3 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IWMDRMReader2> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IWMDRMReader2>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IWMDRMReader2> for &IWMDRMReader3 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IWMDRMReader2> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IWMDRMReader2>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<IWMDRMReader3> for IWMDRMReader {
    fn from(value: IWMDRMReader3) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IWMDRMReader3> for IWMDRMReader {
    fn from(value: &IWMDRMReader3) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IWMDRMReader> for IWMDRMReader3 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IWMDRMReader> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IWMDRMReader>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IWMDRMReader> for &IWMDRMReader3 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IWMDRMReader> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IWMDRMReader>::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMDRMReader3_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwflags: u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwflags: u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pwstrname: super::super::Foundation::PWSTR, dwtype: WMT_ATTR_DATATYPE, pvalue: *const u8, cblength: u16) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pwstrname: super::super::Foundation::PWSTR, pdwtype: *mut WMT_ATTR_DATATYPE, pvalue: *mut u8, pcblength: *mut u16) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, fevaluate: super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pplayopl: *mut DRM_PLAY_OPL, pcblength: *mut u32, pdwminappcompliancelevel: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pcopyopl: *mut DRM_COPY_OPL, pcblength: *mut u32, pdwminappcompliancelevel: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppguids: *mut *mut ::windows::runtime::GUID, pcguids: *mut u32) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct IWMDRMTranscryptionManager(::windows::runtime::IUnknown);
impl IWMDRMTranscryptionManager {
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
    pub unsafe fn CreateTranscryptor(&self) -> ::windows::runtime::Result<IWMDRMTranscryptor> {
        let mut result__: <IWMDRMTranscryptor as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), &mut result__).from_abi::<IWMDRMTranscryptor>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IWMDRMTranscryptionManager {
    type Vtable = IWMDRMTranscryptionManager_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2980612018, 42224, 16506, [176, 46, 239, 189, 35, 187, 236, 223]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMDRMTranscryptionManager_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pptranscryptor: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct IWMDRMTranscryptor(::windows::runtime::IUnknown);
impl IWMDRMTranscryptor {
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`, `Win32_Foundation`*"]
    pub unsafe fn Initialize<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>, Param4: ::windows::runtime::IntoParam<'a, IWMStatusCallback>>(&self, bstrfilename: Param0, pblicenserequestmsg: *mut u8, cblicenserequestmsg: u32, pplicenseresponsemsg: *mut ::std::option::Option<INSSBuffer>, pcallback: Param4, pvcontext: *const ::std::ffi::c_void) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), bstrfilename.into_param().abi(), ::std::mem::transmute(pblicenserequestmsg), ::std::mem::transmute(cblicenserequestmsg), ::std::mem::transmute(pplicenseresponsemsg), pcallback.into_param().abi(), ::std::mem::transmute(pvcontext)).ok()
    }
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
    pub unsafe fn Seek(&self, hnstime: u64) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), ::std::mem::transmute(hnstime)).ok()
    }
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
    pub unsafe fn Read(&self, pbdata: *const u8, pcbdata: *const u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self), ::std::mem::transmute(pbdata), ::std::mem::transmute(pcbdata)).ok()
    }
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
    pub unsafe fn Close(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::std::mem::transmute_copy(self)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IWMDRMTranscryptor {
    type Vtable = IWMDRMTranscryptor_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1761974352, 28271, 19378, [128, 111, 113, 134, 61, 223, 196, 113]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMDRMTranscryptor_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, bstrfilename: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>, pblicenserequestmsg: *mut u8, cblicenserequestmsg: u32, pplicenseresponsemsg: *mut ::windows::runtime::RawPtr, pcallback: ::windows::runtime::RawPtr, pvcontext: *const ::std::ffi::c_void) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, hnstime: u64) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbdata: *const u8, pcbdata: *const u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct IWMDRMTranscryptor2(::windows::runtime::IUnknown);
impl IWMDRMTranscryptor2 {
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`, `Win32_Foundation`*"]
    pub unsafe fn Initialize<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>, Param4: ::windows::runtime::IntoParam<'a, IWMStatusCallback>>(&self, bstrfilename: Param0, pblicenserequestmsg: *mut u8, cblicenserequestmsg: u32, pplicenseresponsemsg: *mut ::std::option::Option<INSSBuffer>, pcallback: Param4, pvcontext: *const ::std::ffi::c_void) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), bstrfilename.into_param().abi(), ::std::mem::transmute(pblicenserequestmsg), ::std::mem::transmute(cblicenserequestmsg), ::std::mem::transmute(pplicenseresponsemsg), pcallback.into_param().abi(), ::std::mem::transmute(pvcontext)).ok()
    }
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
    pub unsafe fn Seek(&self, hnstime: u64) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), ::std::mem::transmute(hnstime)).ok()
    }
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
    pub unsafe fn Read(&self, pbdata: *const u8, pcbdata: *const u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self), ::std::mem::transmute(pbdata), ::std::mem::transmute(pcbdata)).ok()
    }
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
    pub unsafe fn Close(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::std::mem::transmute_copy(self)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`, `Win32_Foundation`*"]
    pub unsafe fn SeekEx<'a, Param3: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>>(&self, cnsstarttime: u64, cnsduration: u64, flrate: f32, fincludefileheader: Param3) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self), ::std::mem::transmute(cnsstarttime), ::std::mem::transmute(cnsduration), ::std::mem::transmute(flrate), fincludefileheader.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`, `Win32_Foundation`*"]
    pub unsafe fn ZeroAdjustTimestamps<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>>(&self, fenable: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).8)(::std::mem::transmute_copy(self), fenable.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
    pub unsafe fn GetSeekStartTime(&self) -> ::windows::runtime::Result<u64> {
        let mut result__: <u64 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).9)(::std::mem::transmute_copy(self), &mut result__).from_abi::<u64>(result__)
    }
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
    pub unsafe fn GetDuration(&self) -> ::windows::runtime::Result<u64> {
        let mut result__: <u64 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).10)(::std::mem::transmute_copy(self), &mut result__).from_abi::<u64>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IWMDRMTranscryptor2 {
    type Vtable = IWMDRMTranscryptor2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3772400543, 54065, 18794, [190, 206, 24, 229, 186, 197, 221, 35]);
}
impl ::std::convert::From<IWMDRMTranscryptor2> for IWMDRMTranscryptor {
    fn from(value: IWMDRMTranscryptor2) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IWMDRMTranscryptor2> for IWMDRMTranscryptor {
    fn from(value: &IWMDRMTranscryptor2) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IWMDRMTranscryptor> for IWMDRMTranscryptor2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IWMDRMTranscryptor> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IWMDRMTranscryptor>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IWMDRMTranscryptor> for &IWMDRMTranscryptor2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IWMDRMTranscryptor> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IWMDRMTranscryptor>::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMDRMTranscryptor2_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, bstrfilename: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>, pblicenserequestmsg: *mut u8, cblicenserequestmsg: u32, pplicenseresponsemsg: *mut ::windows::runtime::RawPtr, pcallback: ::windows::runtime::RawPtr, pvcontext: *const ::std::ffi::c_void) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, hnstime: u64) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbdata: *const u8, pcbdata: *const u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, cnsstarttime: u64, cnsduration: u64, flrate: f32, fincludefileheader: super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, fenable: super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pcnstime: *mut u64) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pcnsduration: *mut u64) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct IWMDRMWriter(::windows::runtime::IUnknown);
impl IWMDRMWriter {
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`, `Win32_Foundation`*"]
    pub unsafe fn GenerateKeySeed(&self, pwszkeyseed: super::super::Foundation::PWSTR, pcwchlength: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), ::std::mem::transmute(pwszkeyseed), ::std::mem::transmute(pcwchlength)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`, `Win32_Foundation`*"]
    pub unsafe fn GenerateKeyID(&self, pwszkeyid: super::super::Foundation::PWSTR, pcwchlength: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), ::std::mem::transmute(pwszkeyid), ::std::mem::transmute(pcwchlength)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`, `Win32_Foundation`*"]
    pub unsafe fn GenerateSigningKeyPair(&self, pwszprivkey: super::super::Foundation::PWSTR, pcwchprivkeylength: *mut u32, pwszpubkey: super::super::Foundation::PWSTR, pcwchpubkeylength: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self), ::std::mem::transmute(pwszprivkey), ::std::mem::transmute(pcwchprivkeylength), ::std::mem::transmute(pwszpubkey), ::std::mem::transmute(pcwchpubkeylength)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`, `Win32_Foundation`*"]
    pub unsafe fn SetDRMAttribute<'a, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, wstreamnum: u16, pszname: Param1, r#type: WMT_ATTR_DATATYPE, pvalue: *const u8, cblength: u16) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::std::mem::transmute_copy(self), ::std::mem::transmute(wstreamnum), pszname.into_param().abi(), ::std::mem::transmute(r#type), ::std::mem::transmute(pvalue), ::std::mem::transmute(cblength)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IWMDRMWriter {
    type Vtable = IWMDRMWriter_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3605683664, 4768, 17396, [144, 171, 163, 253, 69, 30, 106, 7]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMDRMWriter_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pwszkeyseed: super::super::Foundation::PWSTR, pcwchlength: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pwszkeyid: super::super::Foundation::PWSTR, pcwchlength: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pwszprivkey: super::super::Foundation::PWSTR, pcwchprivkeylength: *mut u32, pwszpubkey: super::super::Foundation::PWSTR, pcwchpubkeylength: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, wstreamnum: u16, pszname: super::super::Foundation::PWSTR, r#type: WMT_ATTR_DATATYPE, pvalue: *const u8, cblength: u16) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct IWMDRMWriter2(::windows::runtime::IUnknown);
impl IWMDRMWriter2 {
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`, `Win32_Foundation`*"]
    pub unsafe fn GenerateKeySeed(&self, pwszkeyseed: super::super::Foundation::PWSTR, pcwchlength: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), ::std::mem::transmute(pwszkeyseed), ::std::mem::transmute(pcwchlength)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`, `Win32_Foundation`*"]
    pub unsafe fn GenerateKeyID(&self, pwszkeyid: super::super::Foundation::PWSTR, pcwchlength: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), ::std::mem::transmute(pwszkeyid), ::std::mem::transmute(pcwchlength)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`, `Win32_Foundation`*"]
    pub unsafe fn GenerateSigningKeyPair(&self, pwszprivkey: super::super::Foundation::PWSTR, pcwchprivkeylength: *mut u32, pwszpubkey: super::super::Foundation::PWSTR, pcwchpubkeylength: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self), ::std::mem::transmute(pwszprivkey), ::std::mem::transmute(pcwchprivkeylength), ::std::mem::transmute(pwszpubkey), ::std::mem::transmute(pcwchpubkeylength)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`, `Win32_Foundation`*"]
    pub unsafe fn SetDRMAttribute<'a, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, wstreamnum: u16, pszname: Param1, r#type: WMT_ATTR_DATATYPE, pvalue: *const u8, cblength: u16) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::std::mem::transmute_copy(self), ::std::mem::transmute(wstreamnum), pszname.into_param().abi(), ::std::mem::transmute(r#type), ::std::mem::transmute(pvalue), ::std::mem::transmute(cblength)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`, `Win32_Foundation`*"]
    pub unsafe fn SetWMDRMNetEncryption<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>>(&self, fsamplesencrypted: Param0, pbkeyid: *const u8, cbkeyid: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self), fsamplesencrypted.into_param().abi(), ::std::mem::transmute(pbkeyid), ::std::mem::transmute(cbkeyid)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IWMDRMWriter2 {
    type Vtable = IWMDRMWriter2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(955153044, 16610, 19984, [170, 63, 51, 253, 50, 16, 237, 91]);
}
impl ::std::convert::From<IWMDRMWriter2> for IWMDRMWriter {
    fn from(value: IWMDRMWriter2) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IWMDRMWriter2> for IWMDRMWriter {
    fn from(value: &IWMDRMWriter2) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IWMDRMWriter> for IWMDRMWriter2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IWMDRMWriter> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IWMDRMWriter>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IWMDRMWriter> for &IWMDRMWriter2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IWMDRMWriter> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IWMDRMWriter>::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMDRMWriter2_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pwszkeyseed: super::super::Foundation::PWSTR, pcwchlength: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pwszkeyid: super::super::Foundation::PWSTR, pcwchlength: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pwszprivkey: super::super::Foundation::PWSTR, pcwchprivkeylength: *mut u32, pwszpubkey: super::super::Foundation::PWSTR, pcwchpubkeylength: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, wstreamnum: u16, pszname: super::super::Foundation::PWSTR, r#type: WMT_ATTR_DATATYPE, pvalue: *const u8, cblength: u16) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, fsamplesencrypted: super::super::Foundation::BOOL, pbkeyid: *const u8, cbkeyid: u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct IWMDRMWriter3(::windows::runtime::IUnknown);
impl IWMDRMWriter3 {
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`, `Win32_Foundation`*"]
    pub unsafe fn GenerateKeySeed(&self, pwszkeyseed: super::super::Foundation::PWSTR, pcwchlength: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), ::std::mem::transmute(pwszkeyseed), ::std::mem::transmute(pcwchlength)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`, `Win32_Foundation`*"]
    pub unsafe fn GenerateKeyID(&self, pwszkeyid: super::super::Foundation::PWSTR, pcwchlength: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), ::std::mem::transmute(pwszkeyid), ::std::mem::transmute(pcwchlength)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`, `Win32_Foundation`*"]
    pub unsafe fn GenerateSigningKeyPair(&self, pwszprivkey: super::super::Foundation::PWSTR, pcwchprivkeylength: *mut u32, pwszpubkey: super::super::Foundation::PWSTR, pcwchpubkeylength: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self), ::std::mem::transmute(pwszprivkey), ::std::mem::transmute(pcwchprivkeylength), ::std::mem::transmute(pwszpubkey), ::std::mem::transmute(pcwchpubkeylength)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`, `Win32_Foundation`*"]
    pub unsafe fn SetDRMAttribute<'a, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, wstreamnum: u16, pszname: Param1, r#type: WMT_ATTR_DATATYPE, pvalue: *const u8, cblength: u16) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::std::mem::transmute_copy(self), ::std::mem::transmute(wstreamnum), pszname.into_param().abi(), ::std::mem::transmute(r#type), ::std::mem::transmute(pvalue), ::std::mem::transmute(cblength)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`, `Win32_Foundation`*"]
    pub unsafe fn SetWMDRMNetEncryption<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>>(&self, fsamplesencrypted: Param0, pbkeyid: *const u8, cbkeyid: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self), fsamplesencrypted.into_param().abi(), ::std::mem::transmute(pbkeyid), ::std::mem::transmute(cbkeyid)).ok()
    }
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
    pub unsafe fn SetProtectStreamSamples(&self, pimportinitstruct: *const WMDRM_IMPORT_INIT_STRUCT) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).8)(::std::mem::transmute_copy(self), ::std::mem::transmute(pimportinitstruct)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IWMDRMWriter3 {
    type Vtable = IWMDRMWriter3_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2803384450, 42154, 19934, [172, 156, 231, 93, 189, 17, 23, 206]);
}
impl ::std::convert::From<IWMDRMWriter3> for IWMDRMWriter2 {
    fn from(value: IWMDRMWriter3) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IWMDRMWriter3> for IWMDRMWriter2 {
    fn from(value: &IWMDRMWriter3) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IWMDRMWriter2> for IWMDRMWriter3 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IWMDRMWriter2> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IWMDRMWriter2>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IWMDRMWriter2> for &IWMDRMWriter3 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IWMDRMWriter2> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IWMDRMWriter2>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<IWMDRMWriter3> for IWMDRMWriter {
    fn from(value: IWMDRMWriter3) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IWMDRMWriter3> for IWMDRMWriter {
    fn from(value: &IWMDRMWriter3) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IWMDRMWriter> for IWMDRMWriter3 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IWMDRMWriter> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IWMDRMWriter>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IWMDRMWriter> for &IWMDRMWriter3 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IWMDRMWriter> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IWMDRMWriter>::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMDRMWriter3_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pwszkeyseed: super::super::Foundation::PWSTR, pcwchlength: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pwszkeyid: super::super::Foundation::PWSTR, pcwchlength: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pwszprivkey: super::super::Foundation::PWSTR, pcwchprivkeylength: *mut u32, pwszpubkey: super::super::Foundation::PWSTR, pcwchpubkeylength: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, wstreamnum: u16, pszname: super::super::Foundation::PWSTR, r#type: WMT_ATTR_DATATYPE, pvalue: *const u8, cblength: u16) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, fsamplesencrypted: super::super::Foundation::BOOL, pbkeyid: *const u8, cbkeyid: u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pimportinitstruct: *const WMDRM_IMPORT_INIT_STRUCT) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct IWMDeviceRegistration(::windows::runtime::IUnknown);
impl IWMDeviceRegistration {
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
    pub unsafe fn RegisterDevice<'a, Param3: ::windows::runtime::IntoParam<'a, DRM_VAL16>>(&self, dwregistertype: u32, pbcertificate: *const u8, cbcertificate: u32, serialnumber: Param3) -> ::windows::runtime::Result<IWMRegisteredDevice> {
        let mut result__: <IWMRegisteredDevice as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), ::std::mem::transmute(dwregistertype), ::std::mem::transmute(pbcertificate), ::std::mem::transmute(cbcertificate), serialnumber.into_param().abi(), &mut result__).from_abi::<IWMRegisteredDevice>(result__)
    }
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
    pub unsafe fn UnregisterDevice<'a, Param3: ::windows::runtime::IntoParam<'a, DRM_VAL16>>(&self, dwregistertype: u32, pbcertificate: *const u8, cbcertificate: u32, serialnumber: Param3) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), ::std::mem::transmute(dwregistertype), ::std::mem::transmute(pbcertificate), ::std::mem::transmute(cbcertificate), serialnumber.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
    pub unsafe fn GetRegistrationStats(&self, dwregistertype: u32) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self), ::std::mem::transmute(dwregistertype), &mut result__).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
    pub unsafe fn GetFirstRegisteredDevice(&self, dwregistertype: u32) -> ::windows::runtime::Result<IWMRegisteredDevice> {
        let mut result__: <IWMRegisteredDevice as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).6)(::std::mem::transmute_copy(self), ::std::mem::transmute(dwregistertype), &mut result__).from_abi::<IWMRegisteredDevice>(result__)
    }
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
    pub unsafe fn GetNextRegisteredDevice(&self) -> ::windows::runtime::Result<IWMRegisteredDevice> {
        let mut result__: <IWMRegisteredDevice as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self), &mut result__).from_abi::<IWMRegisteredDevice>(result__)
    }
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
    pub unsafe fn GetRegisteredDeviceByID<'a, Param3: ::windows::runtime::IntoParam<'a, DRM_VAL16>>(&self, dwregistertype: u32, pbcertificate: *const u8, cbcertificate: u32, serialnumber: Param3) -> ::windows::runtime::Result<IWMRegisteredDevice> {
        let mut result__: <IWMRegisteredDevice as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).8)(::std::mem::transmute_copy(self), ::std::mem::transmute(dwregistertype), ::std::mem::transmute(pbcertificate), ::std::mem::transmute(cbcertificate), serialnumber.into_param().abi(), &mut result__).from_abi::<IWMRegisteredDevice>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IWMDeviceRegistration {
    type Vtable = IWMDeviceRegistration_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(4129365763, 36129, 20116, [147, 230, 133, 16, 128, 95, 45, 153]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMDeviceRegistration_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwregistertype: u32, pbcertificate: *const u8, cbcertificate: u32, serialnumber: DRM_VAL16, ppdevice: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwregistertype: u32, pbcertificate: *const u8, cbcertificate: u32, serialnumber: DRM_VAL16) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwregistertype: u32, pcregistereddevices: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwregistertype: u32, ppdevice: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppdevice: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwregistertype: u32, pbcertificate: *const u8, cbcertificate: u32, serialnumber: DRM_VAL16, ppdevice: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct IWMGetSecureChannel(::windows::runtime::IUnknown);
impl IWMGetSecureChannel {
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
    pub unsafe fn GetPeerSecureChannelInterface(&self) -> ::windows::runtime::Result<IWMSecureChannel> {
        let mut result__: <IWMSecureChannel as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), &mut result__).from_abi::<IWMSecureChannel>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IWMGetSecureChannel {
    type Vtable = IWMGetSecureChannel_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2495350168, 50130, 4563, [190, 223, 0, 192, 79, 97, 41, 134]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMGetSecureChannel_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pppeer: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct IWMHeaderInfo(::windows::runtime::IUnknown);
impl IWMHeaderInfo {
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
    pub unsafe fn GetAttributeCount(&self, wstreamnum: u16) -> ::windows::runtime::Result<u16> {
        let mut result__: <u16 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), ::std::mem::transmute(wstreamnum), &mut result__).from_abi::<u16>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`, `Win32_Foundation`*"]
    pub unsafe fn GetAttributeByIndex(&self, windex: u16, pwstreamnum: *mut u16, pwszname: super::super::Foundation::PWSTR, pcchnamelen: *mut u16, ptype: *mut WMT_ATTR_DATATYPE, pvalue: *mut u8, pcblength: *mut u16) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), ::std::mem::transmute(windex), ::std::mem::transmute(pwstreamnum), ::std::mem::transmute(pwszname), ::std::mem::transmute(pcchnamelen), ::std::mem::transmute(ptype), ::std::mem::transmute(pvalue), ::std::mem::transmute(pcblength)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`, `Win32_Foundation`*"]
    pub unsafe fn GetAttributeByName<'a, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, pwstreamnum: *mut u16, pszname: Param1, ptype: *mut WMT_ATTR_DATATYPE, pvalue: *mut u8, pcblength: *mut u16) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self), ::std::mem::transmute(pwstreamnum), pszname.into_param().abi(), ::std::mem::transmute(ptype), ::std::mem::transmute(pvalue), ::std::mem::transmute(pcblength)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`, `Win32_Foundation`*"]
    pub unsafe fn SetAttribute<'a, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, wstreamnum: u16, pszname: Param1, r#type: WMT_ATTR_DATATYPE, pvalue: *const u8, cblength: u16) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::std::mem::transmute_copy(self), ::std::mem::transmute(wstreamnum), pszname.into_param().abi(), ::std::mem::transmute(r#type), ::std::mem::transmute(pvalue), ::std::mem::transmute(cblength)).ok()
    }
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
    pub unsafe fn GetMarkerCount(&self) -> ::windows::runtime::Result<u16> {
        let mut result__: <u16 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self), &mut result__).from_abi::<u16>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`, `Win32_Foundation`*"]
    pub unsafe fn GetMarker(&self, windex: u16, pwszmarkername: super::super::Foundation::PWSTR, pcchmarkernamelen: *mut u16, pcnsmarkertime: *mut u64) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).8)(::std::mem::transmute_copy(self), ::std::mem::transmute(windex), ::std::mem::transmute(pwszmarkername), ::std::mem::transmute(pcchmarkernamelen), ::std::mem::transmute(pcnsmarkertime)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`, `Win32_Foundation`*"]
    pub unsafe fn AddMarker<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, pwszmarkername: Param0, cnsmarkertime: u64) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).9)(::std::mem::transmute_copy(self), pwszmarkername.into_param().abi(), ::std::mem::transmute(cnsmarkertime)).ok()
    }
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
    pub unsafe fn RemoveMarker(&self, windex: u16) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).10)(::std::mem::transmute_copy(self), ::std::mem::transmute(windex)).ok()
    }
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
    pub unsafe fn GetScriptCount(&self) -> ::windows::runtime::Result<u16> {
        let mut result__: <u16 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).11)(::std::mem::transmute_copy(self), &mut result__).from_abi::<u16>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`, `Win32_Foundation`*"]
    pub unsafe fn GetScript(&self, windex: u16, pwsztype: super::super::Foundation::PWSTR, pcchtypelen: *mut u16, pwszcommand: super::super::Foundation::PWSTR, pcchcommandlen: *mut u16, pcnsscripttime: *mut u64) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).12)(::std::mem::transmute_copy(self), ::std::mem::transmute(windex), ::std::mem::transmute(pwsztype), ::std::mem::transmute(pcchtypelen), ::std::mem::transmute(pwszcommand), ::std::mem::transmute(pcchcommandlen), ::std::mem::transmute(pcnsscripttime)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`, `Win32_Foundation`*"]
    pub unsafe fn AddScript<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, pwsztype: Param0, pwszcommand: Param1, cnsscripttime: u64) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).13)(::std::mem::transmute_copy(self), pwsztype.into_param().abi(), pwszcommand.into_param().abi(), ::std::mem::transmute(cnsscripttime)).ok()
    }
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
    pub unsafe fn RemoveScript(&self, windex: u16) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).14)(::std::mem::transmute_copy(self), ::std::mem::transmute(windex)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IWMHeaderInfo {
    type Vtable = IWMHeaderInfo_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2520804314, 11051, 4563, [179, 107, 0, 192, 79, 97, 8, 255]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMHeaderInfo_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, wstreamnum: u16, pcattributes: *mut u16) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, windex: u16, pwstreamnum: *mut u16, pwszname: super::super::Foundation::PWSTR, pcchnamelen: *mut u16, ptype: *mut WMT_ATTR_DATATYPE, pvalue: *mut u8, pcblength: *mut u16) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pwstreamnum: *mut u16, pszname: super::super::Foundation::PWSTR, ptype: *mut WMT_ATTR_DATATYPE, pvalue: *mut u8, pcblength: *mut u16) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, wstreamnum: u16, pszname: super::super::Foundation::PWSTR, r#type: WMT_ATTR_DATATYPE, pvalue: *const u8, cblength: u16) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pcmarkers: *mut u16) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, windex: u16, pwszmarkername: super::super::Foundation::PWSTR, pcchmarkernamelen: *mut u16, pcnsmarkertime: *mut u64) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pwszmarkername: super::super::Foundation::PWSTR, cnsmarkertime: u64) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, windex: u16) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pcscripts: *mut u16) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, windex: u16, pwsztype: super::super::Foundation::PWSTR, pcchtypelen: *mut u16, pwszcommand: super::super::Foundation::PWSTR, pcchcommandlen: *mut u16, pcnsscripttime: *mut u64) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pwsztype: super::super::Foundation::PWSTR, pwszcommand: super::super::Foundation::PWSTR, cnsscripttime: u64) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, windex: u16) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct IWMHeaderInfo2(::windows::runtime::IUnknown);
impl IWMHeaderInfo2 {
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
    pub unsafe fn GetAttributeCount(&self, wstreamnum: u16) -> ::windows::runtime::Result<u16> {
        let mut result__: <u16 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), ::std::mem::transmute(wstreamnum), &mut result__).from_abi::<u16>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`, `Win32_Foundation`*"]
    pub unsafe fn GetAttributeByIndex(&self, windex: u16, pwstreamnum: *mut u16, pwszname: super::super::Foundation::PWSTR, pcchnamelen: *mut u16, ptype: *mut WMT_ATTR_DATATYPE, pvalue: *mut u8, pcblength: *mut u16) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), ::std::mem::transmute(windex), ::std::mem::transmute(pwstreamnum), ::std::mem::transmute(pwszname), ::std::mem::transmute(pcchnamelen), ::std::mem::transmute(ptype), ::std::mem::transmute(pvalue), ::std::mem::transmute(pcblength)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`, `Win32_Foundation`*"]
    pub unsafe fn GetAttributeByName<'a, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, pwstreamnum: *mut u16, pszname: Param1, ptype: *mut WMT_ATTR_DATATYPE, pvalue: *mut u8, pcblength: *mut u16) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self), ::std::mem::transmute(pwstreamnum), pszname.into_param().abi(), ::std::mem::transmute(ptype), ::std::mem::transmute(pvalue), ::std::mem::transmute(pcblength)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`, `Win32_Foundation`*"]
    pub unsafe fn SetAttribute<'a, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, wstreamnum: u16, pszname: Param1, r#type: WMT_ATTR_DATATYPE, pvalue: *const u8, cblength: u16) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::std::mem::transmute_copy(self), ::std::mem::transmute(wstreamnum), pszname.into_param().abi(), ::std::mem::transmute(r#type), ::std::mem::transmute(pvalue), ::std::mem::transmute(cblength)).ok()
    }
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
    pub unsafe fn GetMarkerCount(&self) -> ::windows::runtime::Result<u16> {
        let mut result__: <u16 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self), &mut result__).from_abi::<u16>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`, `Win32_Foundation`*"]
    pub unsafe fn GetMarker(&self, windex: u16, pwszmarkername: super::super::Foundation::PWSTR, pcchmarkernamelen: *mut u16, pcnsmarkertime: *mut u64) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).8)(::std::mem::transmute_copy(self), ::std::mem::transmute(windex), ::std::mem::transmute(pwszmarkername), ::std::mem::transmute(pcchmarkernamelen), ::std::mem::transmute(pcnsmarkertime)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`, `Win32_Foundation`*"]
    pub unsafe fn AddMarker<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, pwszmarkername: Param0, cnsmarkertime: u64) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).9)(::std::mem::transmute_copy(self), pwszmarkername.into_param().abi(), ::std::mem::transmute(cnsmarkertime)).ok()
    }
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
    pub unsafe fn RemoveMarker(&self, windex: u16) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).10)(::std::mem::transmute_copy(self), ::std::mem::transmute(windex)).ok()
    }
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
    pub unsafe fn GetScriptCount(&self) -> ::windows::runtime::Result<u16> {
        let mut result__: <u16 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).11)(::std::mem::transmute_copy(self), &mut result__).from_abi::<u16>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`, `Win32_Foundation`*"]
    pub unsafe fn GetScript(&self, windex: u16, pwsztype: super::super::Foundation::PWSTR, pcchtypelen: *mut u16, pwszcommand: super::super::Foundation::PWSTR, pcchcommandlen: *mut u16, pcnsscripttime: *mut u64) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).12)(::std::mem::transmute_copy(self), ::std::mem::transmute(windex), ::std::mem::transmute(pwsztype), ::std::mem::transmute(pcchtypelen), ::std::mem::transmute(pwszcommand), ::std::mem::transmute(pcchcommandlen), ::std::mem::transmute(pcnsscripttime)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`, `Win32_Foundation`*"]
    pub unsafe fn AddScript<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, pwsztype: Param0, pwszcommand: Param1, cnsscripttime: u64) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).13)(::std::mem::transmute_copy(self), pwsztype.into_param().abi(), pwszcommand.into_param().abi(), ::std::mem::transmute(cnsscripttime)).ok()
    }
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
    pub unsafe fn RemoveScript(&self, windex: u16) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).14)(::std::mem::transmute_copy(self), ::std::mem::transmute(windex)).ok()
    }
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
    pub unsafe fn GetCodecInfoCount(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).15)(::std::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`, `Win32_Foundation`*"]
    pub unsafe fn GetCodecInfo(&self, windex: u32, pcchname: *mut u16, pwszname: super::super::Foundation::PWSTR, pcchdescription: *mut u16, pwszdescription: super::super::Foundation::PWSTR, pcodectype: *mut WMT_CODEC_INFO_TYPE, pcbcodecinfo: *mut u16, pbcodecinfo: *mut u8) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).16)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(windex),
            ::std::mem::transmute(pcchname),
            ::std::mem::transmute(pwszname),
            ::std::mem::transmute(pcchdescription),
            ::std::mem::transmute(pwszdescription),
            ::std::mem::transmute(pcodectype),
            ::std::mem::transmute(pcbcodecinfo),
            ::std::mem::transmute(pbcodecinfo),
        )
        .ok()
    }
}
unsafe impl ::windows::runtime::Interface for IWMHeaderInfo2 {
    type Vtable = IWMHeaderInfo2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(365926273, 17742, 18478, [179, 147, 133, 250, 228, 135, 168, 16]);
}
impl ::std::convert::From<IWMHeaderInfo2> for IWMHeaderInfo {
    fn from(value: IWMHeaderInfo2) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IWMHeaderInfo2> for IWMHeaderInfo {
    fn from(value: &IWMHeaderInfo2) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IWMHeaderInfo> for IWMHeaderInfo2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IWMHeaderInfo> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IWMHeaderInfo>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IWMHeaderInfo> for &IWMHeaderInfo2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IWMHeaderInfo> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IWMHeaderInfo>::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMHeaderInfo2_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, wstreamnum: u16, pcattributes: *mut u16) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, windex: u16, pwstreamnum: *mut u16, pwszname: super::super::Foundation::PWSTR, pcchnamelen: *mut u16, ptype: *mut WMT_ATTR_DATATYPE, pvalue: *mut u8, pcblength: *mut u16) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pwstreamnum: *mut u16, pszname: super::super::Foundation::PWSTR, ptype: *mut WMT_ATTR_DATATYPE, pvalue: *mut u8, pcblength: *mut u16) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, wstreamnum: u16, pszname: super::super::Foundation::PWSTR, r#type: WMT_ATTR_DATATYPE, pvalue: *const u8, cblength: u16) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pcmarkers: *mut u16) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, windex: u16, pwszmarkername: super::super::Foundation::PWSTR, pcchmarkernamelen: *mut u16, pcnsmarkertime: *mut u64) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pwszmarkername: super::super::Foundation::PWSTR, cnsmarkertime: u64) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, windex: u16) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pcscripts: *mut u16) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, windex: u16, pwsztype: super::super::Foundation::PWSTR, pcchtypelen: *mut u16, pwszcommand: super::super::Foundation::PWSTR, pcchcommandlen: *mut u16, pcnsscripttime: *mut u64) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pwsztype: super::super::Foundation::PWSTR, pwszcommand: super::super::Foundation::PWSTR, cnsscripttime: u64) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, windex: u16) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pccodecinfos: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, windex: u32, pcchname: *mut u16, pwszname: super::super::Foundation::PWSTR, pcchdescription: *mut u16, pwszdescription: super::super::Foundation::PWSTR, pcodectype: *mut WMT_CODEC_INFO_TYPE, pcbcodecinfo: *mut u16, pbcodecinfo: *mut u8) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct IWMHeaderInfo3(::windows::runtime::IUnknown);
impl IWMHeaderInfo3 {
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
    pub unsafe fn GetAttributeCount(&self, wstreamnum: u16) -> ::windows::runtime::Result<u16> {
        let mut result__: <u16 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), ::std::mem::transmute(wstreamnum), &mut result__).from_abi::<u16>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`, `Win32_Foundation`*"]
    pub unsafe fn GetAttributeByIndex(&self, windex: u16, pwstreamnum: *mut u16, pwszname: super::super::Foundation::PWSTR, pcchnamelen: *mut u16, ptype: *mut WMT_ATTR_DATATYPE, pvalue: *mut u8, pcblength: *mut u16) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), ::std::mem::transmute(windex), ::std::mem::transmute(pwstreamnum), ::std::mem::transmute(pwszname), ::std::mem::transmute(pcchnamelen), ::std::mem::transmute(ptype), ::std::mem::transmute(pvalue), ::std::mem::transmute(pcblength)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`, `Win32_Foundation`*"]
    pub unsafe fn GetAttributeByName<'a, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, pwstreamnum: *mut u16, pszname: Param1, ptype: *mut WMT_ATTR_DATATYPE, pvalue: *mut u8, pcblength: *mut u16) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self), ::std::mem::transmute(pwstreamnum), pszname.into_param().abi(), ::std::mem::transmute(ptype), ::std::mem::transmute(pvalue), ::std::mem::transmute(pcblength)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`, `Win32_Foundation`*"]
    pub unsafe fn SetAttribute<'a, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, wstreamnum: u16, pszname: Param1, r#type: WMT_ATTR_DATATYPE, pvalue: *const u8, cblength: u16) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::std::mem::transmute_copy(self), ::std::mem::transmute(wstreamnum), pszname.into_param().abi(), ::std::mem::transmute(r#type), ::std::mem::transmute(pvalue), ::std::mem::transmute(cblength)).ok()
    }
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
    pub unsafe fn GetMarkerCount(&self) -> ::windows::runtime::Result<u16> {
        let mut result__: <u16 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self), &mut result__).from_abi::<u16>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`, `Win32_Foundation`*"]
    pub unsafe fn GetMarker(&self, windex: u16, pwszmarkername: super::super::Foundation::PWSTR, pcchmarkernamelen: *mut u16, pcnsmarkertime: *mut u64) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).8)(::std::mem::transmute_copy(self), ::std::mem::transmute(windex), ::std::mem::transmute(pwszmarkername), ::std::mem::transmute(pcchmarkernamelen), ::std::mem::transmute(pcnsmarkertime)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`, `Win32_Foundation`*"]
    pub unsafe fn AddMarker<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, pwszmarkername: Param0, cnsmarkertime: u64) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).9)(::std::mem::transmute_copy(self), pwszmarkername.into_param().abi(), ::std::mem::transmute(cnsmarkertime)).ok()
    }
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
    pub unsafe fn RemoveMarker(&self, windex: u16) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).10)(::std::mem::transmute_copy(self), ::std::mem::transmute(windex)).ok()
    }
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
    pub unsafe fn GetScriptCount(&self) -> ::windows::runtime::Result<u16> {
        let mut result__: <u16 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).11)(::std::mem::transmute_copy(self), &mut result__).from_abi::<u16>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`, `Win32_Foundation`*"]
    pub unsafe fn GetScript(&self, windex: u16, pwsztype: super::super::Foundation::PWSTR, pcchtypelen: *mut u16, pwszcommand: super::super::Foundation::PWSTR, pcchcommandlen: *mut u16, pcnsscripttime: *mut u64) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).12)(::std::mem::transmute_copy(self), ::std::mem::transmute(windex), ::std::mem::transmute(pwsztype), ::std::mem::transmute(pcchtypelen), ::std::mem::transmute(pwszcommand), ::std::mem::transmute(pcchcommandlen), ::std::mem::transmute(pcnsscripttime)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`, `Win32_Foundation`*"]
    pub unsafe fn AddScript<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, pwsztype: Param0, pwszcommand: Param1, cnsscripttime: u64) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).13)(::std::mem::transmute_copy(self), pwsztype.into_param().abi(), pwszcommand.into_param().abi(), ::std::mem::transmute(cnsscripttime)).ok()
    }
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
    pub unsafe fn RemoveScript(&self, windex: u16) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).14)(::std::mem::transmute_copy(self), ::std::mem::transmute(windex)).ok()
    }
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
    pub unsafe fn GetCodecInfoCount(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).15)(::std::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`, `Win32_Foundation`*"]
    pub unsafe fn GetCodecInfo(&self, windex: u32, pcchname: *mut u16, pwszname: super::super::Foundation::PWSTR, pcchdescription: *mut u16, pwszdescription: super::super::Foundation::PWSTR, pcodectype: *mut WMT_CODEC_INFO_TYPE, pcbcodecinfo: *mut u16, pbcodecinfo: *mut u8) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).16)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(windex),
            ::std::mem::transmute(pcchname),
            ::std::mem::transmute(pwszname),
            ::std::mem::transmute(pcchdescription),
            ::std::mem::transmute(pwszdescription),
            ::std::mem::transmute(pcodectype),
            ::std::mem::transmute(pcbcodecinfo),
            ::std::mem::transmute(pbcodecinfo),
        )
        .ok()
    }
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
    pub unsafe fn GetAttributeCountEx(&self, wstreamnum: u16) -> ::windows::runtime::Result<u16> {
        let mut result__: <u16 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).17)(::std::mem::transmute_copy(self), ::std::mem::transmute(wstreamnum), &mut result__).from_abi::<u16>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`, `Win32_Foundation`*"]
    pub unsafe fn GetAttributeIndices<'a, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, wstreamnum: u16, pwszname: Param1, pwlangindex: *const u16, pwindices: *mut u16, pwcount: *mut u16) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).18)(::std::mem::transmute_copy(self), ::std::mem::transmute(wstreamnum), pwszname.into_param().abi(), ::std::mem::transmute(pwlangindex), ::std::mem::transmute(pwindices), ::std::mem::transmute(pwcount)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`, `Win32_Foundation`*"]
    pub unsafe fn GetAttributeByIndexEx(&self, wstreamnum: u16, windex: u16, pwszname: super::super::Foundation::PWSTR, pwnamelen: *mut u16, ptype: *mut WMT_ATTR_DATATYPE, pwlangindex: *mut u16, pvalue: *mut u8, pdwdatalength: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).19)(::std::mem::transmute_copy(self), ::std::mem::transmute(wstreamnum), ::std::mem::transmute(windex), ::std::mem::transmute(pwszname), ::std::mem::transmute(pwnamelen), ::std::mem::transmute(ptype), ::std::mem::transmute(pwlangindex), ::std::mem::transmute(pvalue), ::std::mem::transmute(pdwdatalength)).ok()
    }
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
    pub unsafe fn ModifyAttribute(&self, wstreamnum: u16, windex: u16, r#type: WMT_ATTR_DATATYPE, wlangindex: u16, pvalue: *const u8, dwlength: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).20)(::std::mem::transmute_copy(self), ::std::mem::transmute(wstreamnum), ::std::mem::transmute(windex), ::std::mem::transmute(r#type), ::std::mem::transmute(wlangindex), ::std::mem::transmute(pvalue), ::std::mem::transmute(dwlength)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`, `Win32_Foundation`*"]
    pub unsafe fn AddAttribute<'a, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, wstreamnum: u16, pszname: Param1, pwindex: *mut u16, r#type: WMT_ATTR_DATATYPE, wlangindex: u16, pvalue: *const u8, dwlength: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).21)(::std::mem::transmute_copy(self), ::std::mem::transmute(wstreamnum), pszname.into_param().abi(), ::std::mem::transmute(pwindex), ::std::mem::transmute(r#type), ::std::mem::transmute(wlangindex), ::std::mem::transmute(pvalue), ::std::mem::transmute(dwlength)).ok()
    }
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
    pub unsafe fn DeleteAttribute(&self, wstreamnum: u16, windex: u16) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).22)(::std::mem::transmute_copy(self), ::std::mem::transmute(wstreamnum), ::std::mem::transmute(windex)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`, `Win32_Foundation`*"]
    pub unsafe fn AddCodecInfo<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, pwszname: Param0, pwszdescription: Param1, codectype: WMT_CODEC_INFO_TYPE, cbcodecinfo: u16, pbcodecinfo: *const u8) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).23)(::std::mem::transmute_copy(self), pwszname.into_param().abi(), pwszdescription.into_param().abi(), ::std::mem::transmute(codectype), ::std::mem::transmute(cbcodecinfo), ::std::mem::transmute(pbcodecinfo)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IWMHeaderInfo3 {
    type Vtable = IWMHeaderInfo3_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(365717731, 10188, 20173, [178, 34, 63, 93, 2, 216, 11, 213]);
}
impl ::std::convert::From<IWMHeaderInfo3> for IWMHeaderInfo2 {
    fn from(value: IWMHeaderInfo3) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IWMHeaderInfo3> for IWMHeaderInfo2 {
    fn from(value: &IWMHeaderInfo3) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IWMHeaderInfo2> for IWMHeaderInfo3 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IWMHeaderInfo2> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IWMHeaderInfo2>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IWMHeaderInfo2> for &IWMHeaderInfo3 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IWMHeaderInfo2> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IWMHeaderInfo2>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<IWMHeaderInfo3> for IWMHeaderInfo {
    fn from(value: IWMHeaderInfo3) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IWMHeaderInfo3> for IWMHeaderInfo {
    fn from(value: &IWMHeaderInfo3) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IWMHeaderInfo> for IWMHeaderInfo3 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IWMHeaderInfo> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IWMHeaderInfo>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IWMHeaderInfo> for &IWMHeaderInfo3 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IWMHeaderInfo> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IWMHeaderInfo>::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMHeaderInfo3_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, wstreamnum: u16, pcattributes: *mut u16) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, windex: u16, pwstreamnum: *mut u16, pwszname: super::super::Foundation::PWSTR, pcchnamelen: *mut u16, ptype: *mut WMT_ATTR_DATATYPE, pvalue: *mut u8, pcblength: *mut u16) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pwstreamnum: *mut u16, pszname: super::super::Foundation::PWSTR, ptype: *mut WMT_ATTR_DATATYPE, pvalue: *mut u8, pcblength: *mut u16) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, wstreamnum: u16, pszname: super::super::Foundation::PWSTR, r#type: WMT_ATTR_DATATYPE, pvalue: *const u8, cblength: u16) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pcmarkers: *mut u16) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, windex: u16, pwszmarkername: super::super::Foundation::PWSTR, pcchmarkernamelen: *mut u16, pcnsmarkertime: *mut u64) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pwszmarkername: super::super::Foundation::PWSTR, cnsmarkertime: u64) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, windex: u16) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pcscripts: *mut u16) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, windex: u16, pwsztype: super::super::Foundation::PWSTR, pcchtypelen: *mut u16, pwszcommand: super::super::Foundation::PWSTR, pcchcommandlen: *mut u16, pcnsscripttime: *mut u64) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pwsztype: super::super::Foundation::PWSTR, pwszcommand: super::super::Foundation::PWSTR, cnsscripttime: u64) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, windex: u16) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pccodecinfos: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, windex: u32, pcchname: *mut u16, pwszname: super::super::Foundation::PWSTR, pcchdescription: *mut u16, pwszdescription: super::super::Foundation::PWSTR, pcodectype: *mut WMT_CODEC_INFO_TYPE, pcbcodecinfo: *mut u16, pbcodecinfo: *mut u8) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, wstreamnum: u16, pcattributes: *mut u16) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, wstreamnum: u16, pwszname: super::super::Foundation::PWSTR, pwlangindex: *const u16, pwindices: *mut u16, pwcount: *mut u16) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, wstreamnum: u16, windex: u16, pwszname: super::super::Foundation::PWSTR, pwnamelen: *mut u16, ptype: *mut WMT_ATTR_DATATYPE, pwlangindex: *mut u16, pvalue: *mut u8, pdwdatalength: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, wstreamnum: u16, windex: u16, r#type: WMT_ATTR_DATATYPE, wlangindex: u16, pvalue: *const u8, dwlength: u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, wstreamnum: u16, pszname: super::super::Foundation::PWSTR, pwindex: *mut u16, r#type: WMT_ATTR_DATATYPE, wlangindex: u16, pvalue: *const u8, dwlength: u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, wstreamnum: u16, windex: u16) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pwszname: super::super::Foundation::PWSTR, pwszdescription: super::super::Foundation::PWSTR, codectype: WMT_CODEC_INFO_TYPE, cbcodecinfo: u16, pbcodecinfo: *const u8) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct IWMIStreamProps(::windows::runtime::IUnknown);
impl IWMIStreamProps {
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`, `Win32_Foundation`*"]
    pub unsafe fn GetProperty<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, pszname: Param0, ptype: *mut WMT_ATTR_DATATYPE, pvalue: *mut u8, pdwsize: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), pszname.into_param().abi(), ::std::mem::transmute(ptype), ::std::mem::transmute(pvalue), ::std::mem::transmute(pdwsize)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IWMIStreamProps {
    type Vtable = IWMIStreamProps_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1746328275, 11083, 19598, [129, 73, 135, 76, 52, 131, 167, 83]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMIStreamProps_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pszname: super::super::Foundation::PWSTR, ptype: *mut WMT_ATTR_DATATYPE, pvalue: *mut u8, pdwsize: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct IWMImageInfo(::windows::runtime::IUnknown);
impl IWMImageInfo {
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
    pub unsafe fn GetImageCount(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`, `Win32_Foundation`*"]
    pub unsafe fn GetImage(&self, windex: u32, pcchmimetype: *mut u16, pwszmimetype: super::super::Foundation::PWSTR, pcchdescription: *mut u16, pwszdescription: super::super::Foundation::PWSTR, pimagetype: *mut u16, pcbimagedata: *mut u32, pbimagedata: *mut u8) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(windex),
            ::std::mem::transmute(pcchmimetype),
            ::std::mem::transmute(pwszmimetype),
            ::std::mem::transmute(pcchdescription),
            ::std::mem::transmute(pwszdescription),
            ::std::mem::transmute(pimagetype),
            ::std::mem::transmute(pcbimagedata),
            ::std::mem::transmute(pbimagedata),
        )
        .ok()
    }
}
unsafe impl ::windows::runtime::Interface for IWMImageInfo {
    type Vtable = IWMImageInfo_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2668274614, 29287, 19849, [136, 242, 186, 145, 90, 165, 196, 198]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMImageInfo_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pcimages: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, windex: u32, pcchmimetype: *mut u16, pwszmimetype: super::super::Foundation::PWSTR, pcchdescription: *mut u16, pwszdescription: super::super::Foundation::PWSTR, pimagetype: *mut u16, pcbimagedata: *mut u32, pbimagedata: *mut u8) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct IWMIndexer(::windows::runtime::IUnknown);
impl IWMIndexer {
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`, `Win32_Foundation`*"]
    pub unsafe fn StartIndexing<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>, Param1: ::windows::runtime::IntoParam<'a, IWMStatusCallback>>(&self, pwszurl: Param0, pcallback: Param1, pvcontext: *const ::std::ffi::c_void) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), pwszurl.into_param().abi(), pcallback.into_param().abi(), ::std::mem::transmute(pvcontext)).ok()
    }
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
    pub unsafe fn Cancel(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IWMIndexer {
    type Vtable = IWMIndexer_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1836899441, 39048, 4563, [142, 220, 0, 192, 79, 97, 9, 207]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMIndexer_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pwszurl: super::super::Foundation::PWSTR, pcallback: ::windows::runtime::RawPtr, pvcontext: *const ::std::ffi::c_void) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct IWMIndexer2(::windows::runtime::IUnknown);
impl IWMIndexer2 {
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`, `Win32_Foundation`*"]
    pub unsafe fn StartIndexing<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>, Param1: ::windows::runtime::IntoParam<'a, IWMStatusCallback>>(&self, pwszurl: Param0, pcallback: Param1, pvcontext: *const ::std::ffi::c_void) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), pwszurl.into_param().abi(), pcallback.into_param().abi(), ::std::mem::transmute(pvcontext)).ok()
    }
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
    pub unsafe fn Cancel(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
    pub unsafe fn Configure(&self, wstreamnum: u16, nindexertype: WMT_INDEXER_TYPE, pvinterval: *const ::std::ffi::c_void, pvindextype: *const ::std::ffi::c_void) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self), ::std::mem::transmute(wstreamnum), ::std::mem::transmute(nindexertype), ::std::mem::transmute(pvinterval), ::std::mem::transmute(pvindextype)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IWMIndexer2 {
    type Vtable = IWMIndexer2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3071221314, 25173, 19952, [166, 185, 2, 178, 18, 217, 226, 187]);
}
impl ::std::convert::From<IWMIndexer2> for IWMIndexer {
    fn from(value: IWMIndexer2) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IWMIndexer2> for IWMIndexer {
    fn from(value: &IWMIndexer2) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IWMIndexer> for IWMIndexer2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IWMIndexer> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IWMIndexer>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IWMIndexer> for &IWMIndexer2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IWMIndexer> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IWMIndexer>::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMIndexer2_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pwszurl: super::super::Foundation::PWSTR, pcallback: ::windows::runtime::RawPtr, pvcontext: *const ::std::ffi::c_void) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, wstreamnum: u16, nindexertype: WMT_INDEXER_TYPE, pvinterval: *const ::std::ffi::c_void, pvindextype: *const ::std::ffi::c_void) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct IWMInputMediaProps(::windows::runtime::IUnknown);
impl IWMInputMediaProps {
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
    pub unsafe fn GetType(&self) -> ::windows::runtime::Result<::windows::runtime::GUID> {
        let mut result__: <::windows::runtime::GUID as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), &mut result__).from_abi::<::windows::runtime::GUID>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`, `Win32_Foundation`*"]
    pub unsafe fn GetMediaType(&self, ptype: *mut WM_MEDIA_TYPE, pcbtype: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), ::std::mem::transmute(ptype), ::std::mem::transmute(pcbtype)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`, `Win32_Foundation`*"]
    pub unsafe fn SetMediaType(&self, ptype: *const WM_MEDIA_TYPE) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self), ::std::mem::transmute(ptype)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`, `Win32_Foundation`*"]
    pub unsafe fn GetConnectionName(&self, pwszname: super::super::Foundation::PWSTR, pcchname: *mut u16) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::std::mem::transmute_copy(self), ::std::mem::transmute(pwszname), ::std::mem::transmute(pcchname)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`, `Win32_Foundation`*"]
    pub unsafe fn GetGroupName(&self, pwszname: super::super::Foundation::PWSTR, pcchname: *mut u16) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self), ::std::mem::transmute(pwszname), ::std::mem::transmute(pcchname)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IWMInputMediaProps {
    type Vtable = IWMInputMediaProps_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2520804309, 11051, 4563, [179, 107, 0, 192, 79, 97, 8, 255]);
}
impl ::std::convert::From<IWMInputMediaProps> for IWMMediaProps {
    fn from(value: IWMInputMediaProps) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IWMInputMediaProps> for IWMMediaProps {
    fn from(value: &IWMInputMediaProps) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IWMMediaProps> for IWMInputMediaProps {
    fn into_param(self) -> ::windows::runtime::Param<'a, IWMMediaProps> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IWMMediaProps>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IWMMediaProps> for &IWMInputMediaProps {
    fn into_param(self) -> ::windows::runtime::Param<'a, IWMMediaProps> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IWMMediaProps>::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMInputMediaProps_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pguidtype: *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ptype: *mut ::std::mem::ManuallyDrop<WM_MEDIA_TYPE>, pcbtype: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ptype: *const ::std::mem::ManuallyDrop<WM_MEDIA_TYPE>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pwszname: super::super::Foundation::PWSTR, pcchname: *mut u16) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pwszname: super::super::Foundation::PWSTR, pcchname: *mut u16) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct IWMLanguageList(::windows::runtime::IUnknown);
impl IWMLanguageList {
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
    pub unsafe fn GetLanguageCount(&self) -> ::windows::runtime::Result<u16> {
        let mut result__: <u16 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), &mut result__).from_abi::<u16>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`, `Win32_Foundation`*"]
    pub unsafe fn GetLanguageDetails(&self, windex: u16, pwszlanguagestring: super::super::Foundation::PWSTR, pcchlanguagestringlength: *mut u16) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), ::std::mem::transmute(windex), ::std::mem::transmute(pwszlanguagestring), ::std::mem::transmute(pcchlanguagestringlength)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`, `Win32_Foundation`*"]
    pub unsafe fn AddLanguageByRFC1766String<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, pwszlanguagestring: Param0) -> ::windows::runtime::Result<u16> {
        let mut result__: <u16 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self), pwszlanguagestring.into_param().abi(), &mut result__).from_abi::<u16>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IWMLanguageList {
    type Vtable = IWMLanguageList_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3748151040, 11593, 19854, [146, 183, 251, 25, 246, 160, 220, 87]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMLanguageList_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pwcount: *mut u16) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, windex: u16, pwszlanguagestring: super::super::Foundation::PWSTR, pcchlanguagestringlength: *mut u16) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pwszlanguagestring: super::super::Foundation::PWSTR, pwindex: *mut u16) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct IWMLicenseBackup(::windows::runtime::IUnknown);
impl IWMLicenseBackup {
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
    pub unsafe fn BackupLicenses<'a, Param1: ::windows::runtime::IntoParam<'a, IWMStatusCallback>>(&self, dwflags: u32, pcallback: Param1) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), ::std::mem::transmute(dwflags), pcallback.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
    pub unsafe fn CancelLicenseBackup(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IWMLicenseBackup {
    type Vtable = IWMLicenseBackup_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(98938015, 16310, 17672, [187, 67, 164, 6, 123, 161, 235, 232]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMLicenseBackup_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwflags: u32, pcallback: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct IWMLicenseRestore(::windows::runtime::IUnknown);
impl IWMLicenseRestore {
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
    pub unsafe fn RestoreLicenses<'a, Param1: ::windows::runtime::IntoParam<'a, IWMStatusCallback>>(&self, dwflags: u32, pcallback: Param1) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), ::std::mem::transmute(dwflags), pcallback.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
    pub unsafe fn CancelLicenseRestore(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IWMLicenseRestore {
    type Vtable = IWMLicenseRestore_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3339412276, 41518, 20219, [162, 69, 21, 230, 90, 0, 74, 19]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMLicenseRestore_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwflags: u32, pcallback: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct IWMLicenseRevocationAgent(::windows::runtime::IUnknown);
impl IWMLicenseRevocationAgent {
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
    pub unsafe fn GetLRBChallenge(&self, pmachineid: *const u8, dwmachineidlength: u32, pchallenge: *const u8, dwchallengelength: u32, pchallengeoutput: *mut u8, pdwchallengeoutputlength: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), ::std::mem::transmute(pmachineid), ::std::mem::transmute(dwmachineidlength), ::std::mem::transmute(pchallenge), ::std::mem::transmute(dwchallengelength), ::std::mem::transmute(pchallengeoutput), ::std::mem::transmute(pdwchallengeoutputlength)).ok()
    }
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
    pub unsafe fn ProcessLRB(&self, psignedlrb: *const u8, dwsignedlrblength: u32, psignedack: *mut u8, pdwsignedacklength: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), ::std::mem::transmute(psignedlrb), ::std::mem::transmute(dwsignedlrblength), ::std::mem::transmute(psignedack), ::std::mem::transmute(pdwsignedacklength)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IWMLicenseRevocationAgent {
    type Vtable = IWMLicenseRevocationAgent_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1768420041, 20006, 19287, [136, 148, 121, 152, 128, 247, 172, 123]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMLicenseRevocationAgent_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pmachineid: *const u8, dwmachineidlength: u32, pchallenge: *const u8, dwchallengelength: u32, pchallengeoutput: *mut u8, pdwchallengeoutputlength: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, psignedlrb: *const u8, dwsignedlrblength: u32, psignedack: *mut u8, pdwsignedacklength: *mut u32) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct IWMMediaProps(::windows::runtime::IUnknown);
impl IWMMediaProps {
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
    pub unsafe fn GetType(&self) -> ::windows::runtime::Result<::windows::runtime::GUID> {
        let mut result__: <::windows::runtime::GUID as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), &mut result__).from_abi::<::windows::runtime::GUID>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`, `Win32_Foundation`*"]
    pub unsafe fn GetMediaType(&self, ptype: *mut WM_MEDIA_TYPE, pcbtype: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), ::std::mem::transmute(ptype), ::std::mem::transmute(pcbtype)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`, `Win32_Foundation`*"]
    pub unsafe fn SetMediaType(&self, ptype: *const WM_MEDIA_TYPE) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self), ::std::mem::transmute(ptype)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IWMMediaProps {
    type Vtable = IWMMediaProps_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2520804302, 11051, 4563, [179, 107, 0, 192, 79, 97, 8, 255]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMMediaProps_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pguidtype: *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ptype: *mut ::std::mem::ManuallyDrop<WM_MEDIA_TYPE>, pcbtype: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ptype: *const ::std::mem::ManuallyDrop<WM_MEDIA_TYPE>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct IWMMetadataEditor(::windows::runtime::IUnknown);
impl IWMMetadataEditor {
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`, `Win32_Foundation`*"]
    pub unsafe fn Open<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, pwszfilename: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), pwszfilename.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
    pub unsafe fn Close(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
    pub unsafe fn Flush(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IWMMetadataEditor {
    type Vtable = IWMMetadataEditor_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2520804313, 11051, 4563, [179, 107, 0, 192, 79, 97, 8, 255]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMMetadataEditor_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pwszfilename: super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct IWMMetadataEditor2(::windows::runtime::IUnknown);
impl IWMMetadataEditor2 {
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`, `Win32_Foundation`*"]
    pub unsafe fn Open<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, pwszfilename: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), pwszfilename.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
    pub unsafe fn Close(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
    pub unsafe fn Flush(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`, `Win32_Foundation`*"]
    pub unsafe fn OpenEx<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, pwszfilename: Param0, dwdesiredaccess: u32, dwsharemode: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::std::mem::transmute_copy(self), pwszfilename.into_param().abi(), ::std::mem::transmute(dwdesiredaccess), ::std::mem::transmute(dwsharemode)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IWMMetadataEditor2 {
    type Vtable = IWMMetadataEditor2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(540868579, 11800, 20447, [181, 157, 110, 113, 83, 5, 52, 207]);
}
impl ::std::convert::From<IWMMetadataEditor2> for IWMMetadataEditor {
    fn from(value: IWMMetadataEditor2) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IWMMetadataEditor2> for IWMMetadataEditor {
    fn from(value: &IWMMetadataEditor2) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IWMMetadataEditor> for IWMMetadataEditor2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IWMMetadataEditor> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IWMMetadataEditor>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IWMMetadataEditor> for &IWMMetadataEditor2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IWMMetadataEditor> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IWMMetadataEditor>::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMMetadataEditor2_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pwszfilename: super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pwszfilename: super::super::Foundation::PWSTR, dwdesiredaccess: u32, dwsharemode: u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct IWMMutualExclusion(::windows::runtime::IUnknown);
impl IWMMutualExclusion {
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
    pub unsafe fn GetStreams(&self, pwstreamnumarray: *mut u16, pcstreams: *mut u16) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), ::std::mem::transmute(pwstreamnumarray), ::std::mem::transmute(pcstreams)).ok()
    }
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
    pub unsafe fn AddStream(&self, wstreamnum: u16) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), ::std::mem::transmute(wstreamnum)).ok()
    }
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
    pub unsafe fn RemoveStream(&self, wstreamnum: u16) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self), ::std::mem::transmute(wstreamnum)).ok()
    }
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
    pub unsafe fn GetType(&self) -> ::windows::runtime::Result<::windows::runtime::GUID> {
        let mut result__: <::windows::runtime::GUID as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).6)(::std::mem::transmute_copy(self), &mut result__).from_abi::<::windows::runtime::GUID>(result__)
    }
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
    pub unsafe fn SetType(&self, guidtype: *const ::windows::runtime::GUID) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self), ::std::mem::transmute(guidtype)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IWMMutualExclusion {
    type Vtable = IWMMutualExclusion_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2520804318, 11051, 4563, [179, 107, 0, 192, 79, 97, 8, 255]);
}
impl ::std::convert::From<IWMMutualExclusion> for IWMStreamList {
    fn from(value: IWMMutualExclusion) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IWMMutualExclusion> for IWMStreamList {
    fn from(value: &IWMMutualExclusion) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IWMStreamList> for IWMMutualExclusion {
    fn into_param(self) -> ::windows::runtime::Param<'a, IWMStreamList> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IWMStreamList>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IWMStreamList> for &IWMMutualExclusion {
    fn into_param(self) -> ::windows::runtime::Param<'a, IWMStreamList> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IWMStreamList>::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMMutualExclusion_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pwstreamnumarray: *mut u16, pcstreams: *mut u16) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, wstreamnum: u16) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, wstreamnum: u16) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pguidtype: *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, guidtype: *const ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct IWMMutualExclusion2(::windows::runtime::IUnknown);
impl IWMMutualExclusion2 {
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
    pub unsafe fn GetStreams(&self, pwstreamnumarray: *mut u16, pcstreams: *mut u16) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), ::std::mem::transmute(pwstreamnumarray), ::std::mem::transmute(pcstreams)).ok()
    }
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
    pub unsafe fn AddStream(&self, wstreamnum: u16) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), ::std::mem::transmute(wstreamnum)).ok()
    }
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
    pub unsafe fn RemoveStream(&self, wstreamnum: u16) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self), ::std::mem::transmute(wstreamnum)).ok()
    }
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
    pub unsafe fn GetType(&self) -> ::windows::runtime::Result<::windows::runtime::GUID> {
        let mut result__: <::windows::runtime::GUID as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).6)(::std::mem::transmute_copy(self), &mut result__).from_abi::<::windows::runtime::GUID>(result__)
    }
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
    pub unsafe fn SetType(&self, guidtype: *const ::windows::runtime::GUID) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self), ::std::mem::transmute(guidtype)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`, `Win32_Foundation`*"]
    pub unsafe fn GetName(&self, pwszname: super::super::Foundation::PWSTR, pcchname: *mut u16) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).8)(::std::mem::transmute_copy(self), ::std::mem::transmute(pwszname), ::std::mem::transmute(pcchname)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`, `Win32_Foundation`*"]
    pub unsafe fn SetName<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, pwszname: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).9)(::std::mem::transmute_copy(self), pwszname.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
    pub unsafe fn GetRecordCount(&self) -> ::windows::runtime::Result<u16> {
        let mut result__: <u16 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).10)(::std::mem::transmute_copy(self), &mut result__).from_abi::<u16>(result__)
    }
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
    pub unsafe fn AddRecord(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).11)(::std::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
    pub unsafe fn RemoveRecord(&self, wrecordnumber: u16) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).12)(::std::mem::transmute_copy(self), ::std::mem::transmute(wrecordnumber)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`, `Win32_Foundation`*"]
    pub unsafe fn GetRecordName(&self, wrecordnumber: u16, pwszrecordname: super::super::Foundation::PWSTR, pcchrecordname: *mut u16) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).13)(::std::mem::transmute_copy(self), ::std::mem::transmute(wrecordnumber), ::std::mem::transmute(pwszrecordname), ::std::mem::transmute(pcchrecordname)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`, `Win32_Foundation`*"]
    pub unsafe fn SetRecordName<'a, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, wrecordnumber: u16, pwszrecordname: Param1) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).14)(::std::mem::transmute_copy(self), ::std::mem::transmute(wrecordnumber), pwszrecordname.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
    pub unsafe fn GetStreamsForRecord(&self, wrecordnumber: u16, pwstreamnumarray: *mut u16, pcstreams: *mut u16) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).15)(::std::mem::transmute_copy(self), ::std::mem::transmute(wrecordnumber), ::std::mem::transmute(pwstreamnumarray), ::std::mem::transmute(pcstreams)).ok()
    }
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
    pub unsafe fn AddStreamForRecord(&self, wrecordnumber: u16, wstreamnumber: u16) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).16)(::std::mem::transmute_copy(self), ::std::mem::transmute(wrecordnumber), ::std::mem::transmute(wstreamnumber)).ok()
    }
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
    pub unsafe fn RemoveStreamForRecord(&self, wrecordnumber: u16, wstreamnumber: u16) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).17)(::std::mem::transmute_copy(self), ::std::mem::transmute(wrecordnumber), ::std::mem::transmute(wstreamnumber)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IWMMutualExclusion2 {
    type Vtable = IWMMutualExclusion2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(50509181, 35281, 19362, [133, 201, 22, 111, 44, 83, 235, 145]);
}
impl ::std::convert::From<IWMMutualExclusion2> for IWMMutualExclusion {
    fn from(value: IWMMutualExclusion2) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IWMMutualExclusion2> for IWMMutualExclusion {
    fn from(value: &IWMMutualExclusion2) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IWMMutualExclusion> for IWMMutualExclusion2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IWMMutualExclusion> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IWMMutualExclusion>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IWMMutualExclusion> for &IWMMutualExclusion2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IWMMutualExclusion> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IWMMutualExclusion>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<IWMMutualExclusion2> for IWMStreamList {
    fn from(value: IWMMutualExclusion2) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IWMMutualExclusion2> for IWMStreamList {
    fn from(value: &IWMMutualExclusion2) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IWMStreamList> for IWMMutualExclusion2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IWMStreamList> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IWMStreamList>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IWMStreamList> for &IWMMutualExclusion2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IWMStreamList> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IWMStreamList>::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMMutualExclusion2_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pwstreamnumarray: *mut u16, pcstreams: *mut u16) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, wstreamnum: u16) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, wstreamnum: u16) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pguidtype: *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, guidtype: *const ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pwszname: super::super::Foundation::PWSTR, pcchname: *mut u16) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pwszname: super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pwrecordcount: *mut u16) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, wrecordnumber: u16) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, wrecordnumber: u16, pwszrecordname: super::super::Foundation::PWSTR, pcchrecordname: *mut u16) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, wrecordnumber: u16, pwszrecordname: super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, wrecordnumber: u16, pwstreamnumarray: *mut u16, pcstreams: *mut u16) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, wrecordnumber: u16, wstreamnumber: u16) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, wrecordnumber: u16, wstreamnumber: u16) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct IWMOutputMediaProps(::windows::runtime::IUnknown);
impl IWMOutputMediaProps {
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
    pub unsafe fn GetType(&self) -> ::windows::runtime::Result<::windows::runtime::GUID> {
        let mut result__: <::windows::runtime::GUID as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), &mut result__).from_abi::<::windows::runtime::GUID>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`, `Win32_Foundation`*"]
    pub unsafe fn GetMediaType(&self, ptype: *mut WM_MEDIA_TYPE, pcbtype: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), ::std::mem::transmute(ptype), ::std::mem::transmute(pcbtype)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`, `Win32_Foundation`*"]
    pub unsafe fn SetMediaType(&self, ptype: *const WM_MEDIA_TYPE) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self), ::std::mem::transmute(ptype)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`, `Win32_Foundation`*"]
    pub unsafe fn GetStreamGroupName(&self, pwszname: super::super::Foundation::PWSTR, pcchname: *mut u16) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::std::mem::transmute_copy(self), ::std::mem::transmute(pwszname), ::std::mem::transmute(pcchname)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`, `Win32_Foundation`*"]
    pub unsafe fn GetConnectionName(&self, pwszname: super::super::Foundation::PWSTR, pcchname: *mut u16) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self), ::std::mem::transmute(pwszname), ::std::mem::transmute(pcchname)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IWMOutputMediaProps {
    type Vtable = IWMOutputMediaProps_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2520804311, 11051, 4563, [179, 107, 0, 192, 79, 97, 8, 255]);
}
impl ::std::convert::From<IWMOutputMediaProps> for IWMMediaProps {
    fn from(value: IWMOutputMediaProps) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IWMOutputMediaProps> for IWMMediaProps {
    fn from(value: &IWMOutputMediaProps) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IWMMediaProps> for IWMOutputMediaProps {
    fn into_param(self) -> ::windows::runtime::Param<'a, IWMMediaProps> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IWMMediaProps>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IWMMediaProps> for &IWMOutputMediaProps {
    fn into_param(self) -> ::windows::runtime::Param<'a, IWMMediaProps> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IWMMediaProps>::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMOutputMediaProps_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pguidtype: *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ptype: *mut ::std::mem::ManuallyDrop<WM_MEDIA_TYPE>, pcbtype: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ptype: *const ::std::mem::ManuallyDrop<WM_MEDIA_TYPE>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pwszname: super::super::Foundation::PWSTR, pcchname: *mut u16) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pwszname: super::super::Foundation::PWSTR, pcchname: *mut u16) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct IWMPacketSize(::windows::runtime::IUnknown);
impl IWMPacketSize {
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
    pub unsafe fn GetMaxPacketSize(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
    pub unsafe fn SetMaxPacketSize(&self, dwmaxpacketsize: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), ::std::mem::transmute(dwmaxpacketsize)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IWMPacketSize {
    type Vtable = IWMPacketSize_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3455817643, 6287, 16563, [182, 67, 91, 121, 3, 151, 92, 89]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMPacketSize_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pdwmaxpacketsize: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwmaxpacketsize: u32) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct IWMPacketSize2(::windows::runtime::IUnknown);
impl IWMPacketSize2 {
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
    pub unsafe fn GetMaxPacketSize(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
    pub unsafe fn SetMaxPacketSize(&self, dwmaxpacketsize: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), ::std::mem::transmute(dwmaxpacketsize)).ok()
    }
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
    pub unsafe fn GetMinPacketSize(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
    pub unsafe fn SetMinPacketSize(&self, dwminpacketsize: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::std::mem::transmute_copy(self), ::std::mem::transmute(dwminpacketsize)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IWMPacketSize2 {
    type Vtable = IWMPacketSize2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2348559262, 46662, 16947, [168, 119, 28, 106, 7, 150, 105, 220]);
}
impl ::std::convert::From<IWMPacketSize2> for IWMPacketSize {
    fn from(value: IWMPacketSize2) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IWMPacketSize2> for IWMPacketSize {
    fn from(value: &IWMPacketSize2) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IWMPacketSize> for IWMPacketSize2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IWMPacketSize> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IWMPacketSize>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IWMPacketSize> for &IWMPacketSize2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IWMPacketSize> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IWMPacketSize>::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMPacketSize2_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pdwmaxpacketsize: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwmaxpacketsize: u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pdwminpacketsize: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwminpacketsize: u32) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct IWMPlayerHook(::windows::runtime::IUnknown);
impl IWMPlayerHook {
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
    pub unsafe fn PreDecode(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IWMPlayerHook {
    type Vtable = IWMPlayerHook_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3854027418, 3868, 20326, [144, 2, 116, 236, 80, 216, 179, 4]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMPlayerHook_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct IWMPlayerTimestampHook(::windows::runtime::IUnknown);
impl IWMPlayerTimestampHook {
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
    pub unsafe fn MapTimestamp(&self, rtin: i64) -> ::windows::runtime::Result<i64> {
        let mut result__: <i64 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), ::std::mem::transmute(rtin), &mut result__).from_abi::<i64>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IWMPlayerTimestampHook {
    type Vtable = IWMPlayerTimestampHook_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(676859354, 55694, 18640, [183, 174, 105, 228, 115, 160, 40, 37]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMPlayerTimestampHook_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, rtin: i64, prtout: *mut i64) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct IWMProfile(::windows::runtime::IUnknown);
impl IWMProfile {
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
    pub unsafe fn GetVersion(&self) -> ::windows::runtime::Result<WMT_VERSION> {
        let mut result__: <WMT_VERSION as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), &mut result__).from_abi::<WMT_VERSION>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`, `Win32_Foundation`*"]
    pub unsafe fn GetName(&self, pwszname: super::super::Foundation::PWSTR, pcchname: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), ::std::mem::transmute(pwszname), ::std::mem::transmute(pcchname)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`, `Win32_Foundation`*"]
    pub unsafe fn SetName<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, pwszname: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self), pwszname.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`, `Win32_Foundation`*"]
    pub unsafe fn GetDescription(&self, pwszdescription: super::super::Foundation::PWSTR, pcchdescription: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::std::mem::transmute_copy(self), ::std::mem::transmute(pwszdescription), ::std::mem::transmute(pcchdescription)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`, `Win32_Foundation`*"]
    pub unsafe fn SetDescription<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, pwszdescription: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self), pwszdescription.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
    pub unsafe fn GetStreamCount(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).8)(::std::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
    pub unsafe fn GetStream(&self, dwstreamindex: u32) -> ::windows::runtime::Result<IWMStreamConfig> {
        let mut result__: <IWMStreamConfig as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).9)(::std::mem::transmute_copy(self), ::std::mem::transmute(dwstreamindex), &mut result__).from_abi::<IWMStreamConfig>(result__)
    }
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
    pub unsafe fn GetStreamByNumber(&self, wstreamnum: u16) -> ::windows::runtime::Result<IWMStreamConfig> {
        let mut result__: <IWMStreamConfig as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).10)(::std::mem::transmute_copy(self), ::std::mem::transmute(wstreamnum), &mut result__).from_abi::<IWMStreamConfig>(result__)
    }
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
    pub unsafe fn RemoveStream<'a, Param0: ::windows::runtime::IntoParam<'a, IWMStreamConfig>>(&self, pconfig: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).11)(::std::mem::transmute_copy(self), pconfig.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
    pub unsafe fn RemoveStreamByNumber(&self, wstreamnum: u16) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).12)(::std::mem::transmute_copy(self), ::std::mem::transmute(wstreamnum)).ok()
    }
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
    pub unsafe fn AddStream<'a, Param0: ::windows::runtime::IntoParam<'a, IWMStreamConfig>>(&self, pconfig: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).13)(::std::mem::transmute_copy(self), pconfig.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
    pub unsafe fn ReconfigStream<'a, Param0: ::windows::runtime::IntoParam<'a, IWMStreamConfig>>(&self, pconfig: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).14)(::std::mem::transmute_copy(self), pconfig.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
    pub unsafe fn CreateNewStream(&self, guidstreamtype: *const ::windows::runtime::GUID) -> ::windows::runtime::Result<IWMStreamConfig> {
        let mut result__: <IWMStreamConfig as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).15)(::std::mem::transmute_copy(self), ::std::mem::transmute(guidstreamtype), &mut result__).from_abi::<IWMStreamConfig>(result__)
    }
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
    pub unsafe fn GetMutualExclusionCount(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).16)(::std::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
    pub unsafe fn GetMutualExclusion(&self, dwmeindex: u32) -> ::windows::runtime::Result<IWMMutualExclusion> {
        let mut result__: <IWMMutualExclusion as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).17)(::std::mem::transmute_copy(self), ::std::mem::transmute(dwmeindex), &mut result__).from_abi::<IWMMutualExclusion>(result__)
    }
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
    pub unsafe fn RemoveMutualExclusion<'a, Param0: ::windows::runtime::IntoParam<'a, IWMMutualExclusion>>(&self, pme: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).18)(::std::mem::transmute_copy(self), pme.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
    pub unsafe fn AddMutualExclusion<'a, Param0: ::windows::runtime::IntoParam<'a, IWMMutualExclusion>>(&self, pme: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).19)(::std::mem::transmute_copy(self), pme.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
    pub unsafe fn CreateNewMutualExclusion(&self) -> ::windows::runtime::Result<IWMMutualExclusion> {
        let mut result__: <IWMMutualExclusion as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).20)(::std::mem::transmute_copy(self), &mut result__).from_abi::<IWMMutualExclusion>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IWMProfile {
    type Vtable = IWMProfile_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2520804315, 11051, 4563, [179, 107, 0, 192, 79, 97, 8, 255]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMProfile_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pdwversion: *mut WMT_VERSION) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pwszname: super::super::Foundation::PWSTR, pcchname: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pwszname: super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pwszdescription: super::super::Foundation::PWSTR, pcchdescription: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pwszdescription: super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pcstreams: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwstreamindex: u32, ppconfig: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, wstreamnum: u16, ppconfig: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pconfig: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, wstreamnum: u16) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pconfig: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pconfig: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, guidstreamtype: *const ::windows::runtime::GUID, ppconfig: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pcme: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwmeindex: u32, ppme: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pme: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pme: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppme: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct IWMProfile2(::windows::runtime::IUnknown);
impl IWMProfile2 {
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
    pub unsafe fn GetVersion(&self) -> ::windows::runtime::Result<WMT_VERSION> {
        let mut result__: <WMT_VERSION as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), &mut result__).from_abi::<WMT_VERSION>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`, `Win32_Foundation`*"]
    pub unsafe fn GetName(&self, pwszname: super::super::Foundation::PWSTR, pcchname: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), ::std::mem::transmute(pwszname), ::std::mem::transmute(pcchname)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`, `Win32_Foundation`*"]
    pub unsafe fn SetName<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, pwszname: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self), pwszname.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`, `Win32_Foundation`*"]
    pub unsafe fn GetDescription(&self, pwszdescription: super::super::Foundation::PWSTR, pcchdescription: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::std::mem::transmute_copy(self), ::std::mem::transmute(pwszdescription), ::std::mem::transmute(pcchdescription)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`, `Win32_Foundation`*"]
    pub unsafe fn SetDescription<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, pwszdescription: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self), pwszdescription.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
    pub unsafe fn GetStreamCount(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).8)(::std::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
    pub unsafe fn GetStream(&self, dwstreamindex: u32) -> ::windows::runtime::Result<IWMStreamConfig> {
        let mut result__: <IWMStreamConfig as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).9)(::std::mem::transmute_copy(self), ::std::mem::transmute(dwstreamindex), &mut result__).from_abi::<IWMStreamConfig>(result__)
    }
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
    pub unsafe fn GetStreamByNumber(&self, wstreamnum: u16) -> ::windows::runtime::Result<IWMStreamConfig> {
        let mut result__: <IWMStreamConfig as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).10)(::std::mem::transmute_copy(self), ::std::mem::transmute(wstreamnum), &mut result__).from_abi::<IWMStreamConfig>(result__)
    }
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
    pub unsafe fn RemoveStream<'a, Param0: ::windows::runtime::IntoParam<'a, IWMStreamConfig>>(&self, pconfig: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).11)(::std::mem::transmute_copy(self), pconfig.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
    pub unsafe fn RemoveStreamByNumber(&self, wstreamnum: u16) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).12)(::std::mem::transmute_copy(self), ::std::mem::transmute(wstreamnum)).ok()
    }
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
    pub unsafe fn AddStream<'a, Param0: ::windows::runtime::IntoParam<'a, IWMStreamConfig>>(&self, pconfig: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).13)(::std::mem::transmute_copy(self), pconfig.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
    pub unsafe fn ReconfigStream<'a, Param0: ::windows::runtime::IntoParam<'a, IWMStreamConfig>>(&self, pconfig: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).14)(::std::mem::transmute_copy(self), pconfig.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
    pub unsafe fn CreateNewStream(&self, guidstreamtype: *const ::windows::runtime::GUID) -> ::windows::runtime::Result<IWMStreamConfig> {
        let mut result__: <IWMStreamConfig as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).15)(::std::mem::transmute_copy(self), ::std::mem::transmute(guidstreamtype), &mut result__).from_abi::<IWMStreamConfig>(result__)
    }
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
    pub unsafe fn GetMutualExclusionCount(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).16)(::std::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
    pub unsafe fn GetMutualExclusion(&self, dwmeindex: u32) -> ::windows::runtime::Result<IWMMutualExclusion> {
        let mut result__: <IWMMutualExclusion as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).17)(::std::mem::transmute_copy(self), ::std::mem::transmute(dwmeindex), &mut result__).from_abi::<IWMMutualExclusion>(result__)
    }
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
    pub unsafe fn RemoveMutualExclusion<'a, Param0: ::windows::runtime::IntoParam<'a, IWMMutualExclusion>>(&self, pme: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).18)(::std::mem::transmute_copy(self), pme.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
    pub unsafe fn AddMutualExclusion<'a, Param0: ::windows::runtime::IntoParam<'a, IWMMutualExclusion>>(&self, pme: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).19)(::std::mem::transmute_copy(self), pme.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
    pub unsafe fn CreateNewMutualExclusion(&self) -> ::windows::runtime::Result<IWMMutualExclusion> {
        let mut result__: <IWMMutualExclusion as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).20)(::std::mem::transmute_copy(self), &mut result__).from_abi::<IWMMutualExclusion>(result__)
    }
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
    pub unsafe fn GetProfileID(&self) -> ::windows::runtime::Result<::windows::runtime::GUID> {
        let mut result__: <::windows::runtime::GUID as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).21)(::std::mem::transmute_copy(self), &mut result__).from_abi::<::windows::runtime::GUID>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IWMProfile2 {
    type Vtable = IWMProfile2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(132590899, 55630, 19431, [136, 67, 96, 174, 95, 247, 229, 245]);
}
impl ::std::convert::From<IWMProfile2> for IWMProfile {
    fn from(value: IWMProfile2) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IWMProfile2> for IWMProfile {
    fn from(value: &IWMProfile2) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IWMProfile> for IWMProfile2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IWMProfile> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IWMProfile>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IWMProfile> for &IWMProfile2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IWMProfile> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IWMProfile>::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMProfile2_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pdwversion: *mut WMT_VERSION) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pwszname: super::super::Foundation::PWSTR, pcchname: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pwszname: super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pwszdescription: super::super::Foundation::PWSTR, pcchdescription: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pwszdescription: super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pcstreams: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwstreamindex: u32, ppconfig: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, wstreamnum: u16, ppconfig: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pconfig: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, wstreamnum: u16) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pconfig: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pconfig: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, guidstreamtype: *const ::windows::runtime::GUID, ppconfig: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pcme: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwmeindex: u32, ppme: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pme: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pme: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppme: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pguidid: *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct IWMProfile3(::windows::runtime::IUnknown);
impl IWMProfile3 {
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
    pub unsafe fn GetVersion(&self) -> ::windows::runtime::Result<WMT_VERSION> {
        let mut result__: <WMT_VERSION as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), &mut result__).from_abi::<WMT_VERSION>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`, `Win32_Foundation`*"]
    pub unsafe fn GetName(&self, pwszname: super::super::Foundation::PWSTR, pcchname: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), ::std::mem::transmute(pwszname), ::std::mem::transmute(pcchname)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`, `Win32_Foundation`*"]
    pub unsafe fn SetName<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, pwszname: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self), pwszname.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`, `Win32_Foundation`*"]
    pub unsafe fn GetDescription(&self, pwszdescription: super::super::Foundation::PWSTR, pcchdescription: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::std::mem::transmute_copy(self), ::std::mem::transmute(pwszdescription), ::std::mem::transmute(pcchdescription)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`, `Win32_Foundation`*"]
    pub unsafe fn SetDescription<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, pwszdescription: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self), pwszdescription.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
    pub unsafe fn GetStreamCount(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).8)(::std::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
    pub unsafe fn GetStream(&self, dwstreamindex: u32) -> ::windows::runtime::Result<IWMStreamConfig> {
        let mut result__: <IWMStreamConfig as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).9)(::std::mem::transmute_copy(self), ::std::mem::transmute(dwstreamindex), &mut result__).from_abi::<IWMStreamConfig>(result__)
    }
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
    pub unsafe fn GetStreamByNumber(&self, wstreamnum: u16) -> ::windows::runtime::Result<IWMStreamConfig> {
        let mut result__: <IWMStreamConfig as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).10)(::std::mem::transmute_copy(self), ::std::mem::transmute(wstreamnum), &mut result__).from_abi::<IWMStreamConfig>(result__)
    }
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
    pub unsafe fn RemoveStream<'a, Param0: ::windows::runtime::IntoParam<'a, IWMStreamConfig>>(&self, pconfig: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).11)(::std::mem::transmute_copy(self), pconfig.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
    pub unsafe fn RemoveStreamByNumber(&self, wstreamnum: u16) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).12)(::std::mem::transmute_copy(self), ::std::mem::transmute(wstreamnum)).ok()
    }
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
    pub unsafe fn AddStream<'a, Param0: ::windows::runtime::IntoParam<'a, IWMStreamConfig>>(&self, pconfig: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).13)(::std::mem::transmute_copy(self), pconfig.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
    pub unsafe fn ReconfigStream<'a, Param0: ::windows::runtime::IntoParam<'a, IWMStreamConfig>>(&self, pconfig: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).14)(::std::mem::transmute_copy(self), pconfig.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
    pub unsafe fn CreateNewStream(&self, guidstreamtype: *const ::windows::runtime::GUID) -> ::windows::runtime::Result<IWMStreamConfig> {
        let mut result__: <IWMStreamConfig as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).15)(::std::mem::transmute_copy(self), ::std::mem::transmute(guidstreamtype), &mut result__).from_abi::<IWMStreamConfig>(result__)
    }
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
    pub unsafe fn GetMutualExclusionCount(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).16)(::std::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
    pub unsafe fn GetMutualExclusion(&self, dwmeindex: u32) -> ::windows::runtime::Result<IWMMutualExclusion> {
        let mut result__: <IWMMutualExclusion as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).17)(::std::mem::transmute_copy(self), ::std::mem::transmute(dwmeindex), &mut result__).from_abi::<IWMMutualExclusion>(result__)
    }
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
    pub unsafe fn RemoveMutualExclusion<'a, Param0: ::windows::runtime::IntoParam<'a, IWMMutualExclusion>>(&self, pme: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).18)(::std::mem::transmute_copy(self), pme.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
    pub unsafe fn AddMutualExclusion<'a, Param0: ::windows::runtime::IntoParam<'a, IWMMutualExclusion>>(&self, pme: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).19)(::std::mem::transmute_copy(self), pme.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
    pub unsafe fn CreateNewMutualExclusion(&self) -> ::windows::runtime::Result<IWMMutualExclusion> {
        let mut result__: <IWMMutualExclusion as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).20)(::std::mem::transmute_copy(self), &mut result__).from_abi::<IWMMutualExclusion>(result__)
    }
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
    pub unsafe fn GetProfileID(&self) -> ::windows::runtime::Result<::windows::runtime::GUID> {
        let mut result__: <::windows::runtime::GUID as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).21)(::std::mem::transmute_copy(self), &mut result__).from_abi::<::windows::runtime::GUID>(result__)
    }
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
    pub unsafe fn GetStorageFormat(&self) -> ::windows::runtime::Result<WMT_STORAGE_FORMAT> {
        let mut result__: <WMT_STORAGE_FORMAT as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).22)(::std::mem::transmute_copy(self), &mut result__).from_abi::<WMT_STORAGE_FORMAT>(result__)
    }
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
    pub unsafe fn SetStorageFormat(&self, nstorageformat: WMT_STORAGE_FORMAT) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).23)(::std::mem::transmute_copy(self), ::std::mem::transmute(nstorageformat)).ok()
    }
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
    pub unsafe fn GetBandwidthSharingCount(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).24)(::std::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
    pub unsafe fn GetBandwidthSharing(&self, dwbsindex: u32) -> ::windows::runtime::Result<IWMBandwidthSharing> {
        let mut result__: <IWMBandwidthSharing as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).25)(::std::mem::transmute_copy(self), ::std::mem::transmute(dwbsindex), &mut result__).from_abi::<IWMBandwidthSharing>(result__)
    }
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
    pub unsafe fn RemoveBandwidthSharing<'a, Param0: ::windows::runtime::IntoParam<'a, IWMBandwidthSharing>>(&self, pbs: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).26)(::std::mem::transmute_copy(self), pbs.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
    pub unsafe fn AddBandwidthSharing<'a, Param0: ::windows::runtime::IntoParam<'a, IWMBandwidthSharing>>(&self, pbs: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).27)(::std::mem::transmute_copy(self), pbs.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
    pub unsafe fn CreateNewBandwidthSharing(&self) -> ::windows::runtime::Result<IWMBandwidthSharing> {
        let mut result__: <IWMBandwidthSharing as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).28)(::std::mem::transmute_copy(self), &mut result__).from_abi::<IWMBandwidthSharing>(result__)
    }
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
    pub unsafe fn GetStreamPrioritization(&self) -> ::windows::runtime::Result<IWMStreamPrioritization> {
        let mut result__: <IWMStreamPrioritization as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).29)(::std::mem::transmute_copy(self), &mut result__).from_abi::<IWMStreamPrioritization>(result__)
    }
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
    pub unsafe fn SetStreamPrioritization<'a, Param0: ::windows::runtime::IntoParam<'a, IWMStreamPrioritization>>(&self, psp: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).30)(::std::mem::transmute_copy(self), psp.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
    pub unsafe fn RemoveStreamPrioritization(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).31)(::std::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
    pub unsafe fn CreateNewStreamPrioritization(&self) -> ::windows::runtime::Result<IWMStreamPrioritization> {
        let mut result__: <IWMStreamPrioritization as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).32)(::std::mem::transmute_copy(self), &mut result__).from_abi::<IWMStreamPrioritization>(result__)
    }
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
    pub unsafe fn GetExpectedPacketCount(&self, msduration: u64) -> ::windows::runtime::Result<u64> {
        let mut result__: <u64 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).33)(::std::mem::transmute_copy(self), ::std::mem::transmute(msduration), &mut result__).from_abi::<u64>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IWMProfile3 {
    type Vtable = IWMProfile3_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(15701708, 42081, 17734, [139, 205, 201, 162, 143, 14, 6, 245]);
}
impl ::std::convert::From<IWMProfile3> for IWMProfile2 {
    fn from(value: IWMProfile3) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IWMProfile3> for IWMProfile2 {
    fn from(value: &IWMProfile3) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IWMProfile2> for IWMProfile3 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IWMProfile2> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IWMProfile2>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IWMProfile2> for &IWMProfile3 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IWMProfile2> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IWMProfile2>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<IWMProfile3> for IWMProfile {
    fn from(value: IWMProfile3) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IWMProfile3> for IWMProfile {
    fn from(value: &IWMProfile3) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IWMProfile> for IWMProfile3 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IWMProfile> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IWMProfile>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IWMProfile> for &IWMProfile3 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IWMProfile> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IWMProfile>::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMProfile3_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pdwversion: *mut WMT_VERSION) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pwszname: super::super::Foundation::PWSTR, pcchname: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pwszname: super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pwszdescription: super::super::Foundation::PWSTR, pcchdescription: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pwszdescription: super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pcstreams: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwstreamindex: u32, ppconfig: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, wstreamnum: u16, ppconfig: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pconfig: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, wstreamnum: u16) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pconfig: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pconfig: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, guidstreamtype: *const ::windows::runtime::GUID, ppconfig: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pcme: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwmeindex: u32, ppme: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pme: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pme: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppme: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pguidid: *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pnstorageformat: *mut WMT_STORAGE_FORMAT) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, nstorageformat: WMT_STORAGE_FORMAT) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pcbs: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwbsindex: u32, ppbs: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbs: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbs: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppbs: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppsp: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, psp: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppsp: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, msduration: u64, pcpackets: *mut u64) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct IWMProfileManager(::windows::runtime::IUnknown);
impl IWMProfileManager {
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
    pub unsafe fn CreateEmptyProfile(&self, dwversion: WMT_VERSION) -> ::windows::runtime::Result<IWMProfile> {
        let mut result__: <IWMProfile as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), ::std::mem::transmute(dwversion), &mut result__).from_abi::<IWMProfile>(result__)
    }
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
    pub unsafe fn LoadProfileByID(&self, guidprofile: *const ::windows::runtime::GUID) -> ::windows::runtime::Result<IWMProfile> {
        let mut result__: <IWMProfile as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), ::std::mem::transmute(guidprofile), &mut result__).from_abi::<IWMProfile>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`, `Win32_Foundation`*"]
    pub unsafe fn LoadProfileByData<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, pwszprofile: Param0) -> ::windows::runtime::Result<IWMProfile> {
        let mut result__: <IWMProfile as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self), pwszprofile.into_param().abi(), &mut result__).from_abi::<IWMProfile>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`, `Win32_Foundation`*"]
    pub unsafe fn SaveProfile<'a, Param0: ::windows::runtime::IntoParam<'a, IWMProfile>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, piwmprofile: Param0, pwszprofile: Param1, pdwlength: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::std::mem::transmute_copy(self), piwmprofile.into_param().abi(), pwszprofile.into_param().abi(), ::std::mem::transmute(pdwlength)).ok()
    }
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
    pub unsafe fn GetSystemProfileCount(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
    pub unsafe fn LoadSystemProfile(&self, dwprofileindex: u32) -> ::windows::runtime::Result<IWMProfile> {
        let mut result__: <IWMProfile as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).8)(::std::mem::transmute_copy(self), ::std::mem::transmute(dwprofileindex), &mut result__).from_abi::<IWMProfile>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IWMProfileManager {
    type Vtable = IWMProfileManager_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3513154034, 27808, 18221, [141, 49, 47, 93, 85, 174, 225, 85]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMProfileManager_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwversion: WMT_VERSION, ppprofile: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, guidprofile: *const ::windows::runtime::GUID, ppprofile: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pwszprofile: super::super::Foundation::PWSTR, ppprofile: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, piwmprofile: ::windows::runtime::RawPtr, pwszprofile: super::super::Foundation::PWSTR, pdwlength: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pcprofiles: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwprofileindex: u32, ppprofile: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct IWMProfileManager2(::windows::runtime::IUnknown);
impl IWMProfileManager2 {
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
    pub unsafe fn CreateEmptyProfile(&self, dwversion: WMT_VERSION) -> ::windows::runtime::Result<IWMProfile> {
        let mut result__: <IWMProfile as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), ::std::mem::transmute(dwversion), &mut result__).from_abi::<IWMProfile>(result__)
    }
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
    pub unsafe fn LoadProfileByID(&self, guidprofile: *const ::windows::runtime::GUID) -> ::windows::runtime::Result<IWMProfile> {
        let mut result__: <IWMProfile as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), ::std::mem::transmute(guidprofile), &mut result__).from_abi::<IWMProfile>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`, `Win32_Foundation`*"]
    pub unsafe fn LoadProfileByData<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, pwszprofile: Param0) -> ::windows::runtime::Result<IWMProfile> {
        let mut result__: <IWMProfile as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self), pwszprofile.into_param().abi(), &mut result__).from_abi::<IWMProfile>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`, `Win32_Foundation`*"]
    pub unsafe fn SaveProfile<'a, Param0: ::windows::runtime::IntoParam<'a, IWMProfile>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, piwmprofile: Param0, pwszprofile: Param1, pdwlength: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::std::mem::transmute_copy(self), piwmprofile.into_param().abi(), pwszprofile.into_param().abi(), ::std::mem::transmute(pdwlength)).ok()
    }
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
    pub unsafe fn GetSystemProfileCount(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
    pub unsafe fn LoadSystemProfile(&self, dwprofileindex: u32) -> ::windows::runtime::Result<IWMProfile> {
        let mut result__: <IWMProfile as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).8)(::std::mem::transmute_copy(self), ::std::mem::transmute(dwprofileindex), &mut result__).from_abi::<IWMProfile>(result__)
    }
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
    pub unsafe fn GetSystemProfileVersion(&self, pdwversion: *mut WMT_VERSION) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).9)(::std::mem::transmute_copy(self), ::std::mem::transmute(pdwversion)).ok()
    }
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
    pub unsafe fn SetSystemProfileVersion(&self, dwversion: WMT_VERSION) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).10)(::std::mem::transmute_copy(self), ::std::mem::transmute(dwversion)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IWMProfileManager2 {
    type Vtable = IWMProfileManager2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2056408657, 29633, 18765, [128, 25, 35, 211, 126, 217, 184, 154]);
}
impl ::std::convert::From<IWMProfileManager2> for IWMProfileManager {
    fn from(value: IWMProfileManager2) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IWMProfileManager2> for IWMProfileManager {
    fn from(value: &IWMProfileManager2) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IWMProfileManager> for IWMProfileManager2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IWMProfileManager> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IWMProfileManager>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IWMProfileManager> for &IWMProfileManager2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IWMProfileManager> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IWMProfileManager>::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMProfileManager2_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwversion: WMT_VERSION, ppprofile: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, guidprofile: *const ::windows::runtime::GUID, ppprofile: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pwszprofile: super::super::Foundation::PWSTR, ppprofile: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, piwmprofile: ::windows::runtime::RawPtr, pwszprofile: super::super::Foundation::PWSTR, pdwlength: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pcprofiles: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwprofileindex: u32, ppprofile: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pdwversion: *mut WMT_VERSION) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwversion: WMT_VERSION) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct IWMProfileManagerLanguage(::windows::runtime::IUnknown);
impl IWMProfileManagerLanguage {
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
    pub unsafe fn GetUserLanguageID(&self, wlangid: *mut u16) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), ::std::mem::transmute(wlangid)).ok()
    }
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
    pub unsafe fn SetUserLanguageID(&self, wlangid: u16) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), ::std::mem::transmute(wlangid)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IWMProfileManagerLanguage {
    type Vtable = IWMProfileManagerLanguage_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3125660792, 32480, 19128, [178, 122, 219, 206, 139, 197, 20, 84]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMProfileManagerLanguage_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, wlangid: *mut u16) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, wlangid: u16) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct IWMPropertyVault(::windows::runtime::IUnknown);
impl IWMPropertyVault {
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
    pub unsafe fn GetPropertyCount(&self, pdwcount: *const u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), ::std::mem::transmute(pdwcount)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`, `Win32_Foundation`*"]
    pub unsafe fn GetPropertyByName<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, pszname: Param0, ptype: *mut WMT_ATTR_DATATYPE, pvalue: *mut u8, pdwsize: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), pszname.into_param().abi(), ::std::mem::transmute(ptype), ::std::mem::transmute(pvalue), ::std::mem::transmute(pdwsize)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`, `Win32_Foundation`*"]
    pub unsafe fn SetProperty<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, pszname: Param0, ptype: WMT_ATTR_DATATYPE, pvalue: *const u8, dwsize: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self), pszname.into_param().abi(), ::std::mem::transmute(ptype), ::std::mem::transmute(pvalue), ::std::mem::transmute(dwsize)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`, `Win32_Foundation`*"]
    pub unsafe fn GetPropertyByIndex(&self, dwindex: u32, pszname: super::super::Foundation::PWSTR, pdwnamelen: *mut u32, ptype: *mut WMT_ATTR_DATATYPE, pvalue: *mut u8, pdwsize: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::std::mem::transmute_copy(self), ::std::mem::transmute(dwindex), ::std::mem::transmute(pszname), ::std::mem::transmute(pdwnamelen), ::std::mem::transmute(ptype), ::std::mem::transmute(pvalue), ::std::mem::transmute(pdwsize)).ok()
    }
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
    pub unsafe fn CopyPropertiesFrom<'a, Param0: ::windows::runtime::IntoParam<'a, IWMPropertyVault>>(&self, piwmpropertyvault: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self), piwmpropertyvault.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
    pub unsafe fn Clear(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).8)(::std::mem::transmute_copy(self)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IWMPropertyVault {
    type Vtable = IWMPropertyVault_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1922652793, 20624, 17060, [156, 140, 217, 208, 182, 211, 75, 229]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMPropertyVault_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pdwcount: *const u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pszname: super::super::Foundation::PWSTR, ptype: *mut WMT_ATTR_DATATYPE, pvalue: *mut u8, pdwsize: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pszname: super::super::Foundation::PWSTR, ptype: WMT_ATTR_DATATYPE, pvalue: *const u8, dwsize: u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwindex: u32, pszname: super::super::Foundation::PWSTR, pdwnamelen: *mut u32, ptype: *mut WMT_ATTR_DATATYPE, pvalue: *mut u8, pdwsize: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, piwmpropertyvault: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct IWMProximityDetection(::windows::runtime::IUnknown);
impl IWMProximityDetection {
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
    pub unsafe fn StartDetection<'a, Param6: ::windows::runtime::IntoParam<'a, IWMStatusCallback>>(&self, pbregistrationmsg: *const u8, cbregistrationmsg: u32, pblocaladdress: *const u8, cblocaladdress: u32, dwextraportsallowed: u32, ppregistrationresponsemsg: *mut ::std::option::Option<INSSBuffer>, pcallback: Param6, pvcontext: *const ::std::ffi::c_void) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(pbregistrationmsg),
            ::std::mem::transmute(cbregistrationmsg),
            ::std::mem::transmute(pblocaladdress),
            ::std::mem::transmute(cblocaladdress),
            ::std::mem::transmute(dwextraportsallowed),
            ::std::mem::transmute(ppregistrationresponsemsg),
            pcallback.into_param().abi(),
            ::std::mem::transmute(pvcontext),
        )
        .ok()
    }
}
unsafe impl ::windows::runtime::Interface for IWMProximityDetection {
    type Vtable = IWMProximityDetection_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1788860654, 46673, 19440, [184, 73, 125, 78, 206, 121, 162, 177]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMProximityDetection_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbregistrationmsg: *const u8, cbregistrationmsg: u32, pblocaladdress: *const u8, cblocaladdress: u32, dwextraportsallowed: u32, ppregistrationresponsemsg: *mut ::windows::runtime::RawPtr, pcallback: ::windows::runtime::RawPtr, pvcontext: *const ::std::ffi::c_void) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct IWMReader(::windows::runtime::IUnknown);
impl IWMReader {
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`, `Win32_Foundation`*"]
    pub unsafe fn Open<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>, Param1: ::windows::runtime::IntoParam<'a, IWMReaderCallback>>(&self, pwszurl: Param0, pcallback: Param1, pvcontext: *const ::std::ffi::c_void) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), pwszurl.into_param().abi(), pcallback.into_param().abi(), ::std::mem::transmute(pvcontext)).ok()
    }
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
    pub unsafe fn Close(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
    pub unsafe fn GetOutputCount(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
    pub unsafe fn GetOutputProps(&self, dwoutputnum: u32) -> ::windows::runtime::Result<IWMOutputMediaProps> {
        let mut result__: <IWMOutputMediaProps as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).6)(::std::mem::transmute_copy(self), ::std::mem::transmute(dwoutputnum), &mut result__).from_abi::<IWMOutputMediaProps>(result__)
    }
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
    pub unsafe fn SetOutputProps<'a, Param1: ::windows::runtime::IntoParam<'a, IWMOutputMediaProps>>(&self, dwoutputnum: u32, poutput: Param1) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self), ::std::mem::transmute(dwoutputnum), poutput.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
    pub unsafe fn GetOutputFormatCount(&self, dwoutputnumber: u32) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).8)(::std::mem::transmute_copy(self), ::std::mem::transmute(dwoutputnumber), &mut result__).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
    pub unsafe fn GetOutputFormat(&self, dwoutputnumber: u32, dwformatnumber: u32) -> ::windows::runtime::Result<IWMOutputMediaProps> {
        let mut result__: <IWMOutputMediaProps as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).9)(::std::mem::transmute_copy(self), ::std::mem::transmute(dwoutputnumber), ::std::mem::transmute(dwformatnumber), &mut result__).from_abi::<IWMOutputMediaProps>(result__)
    }
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
    pub unsafe fn Start(&self, cnsstart: u64, cnsduration: u64, frate: f32, pvcontext: *const ::std::ffi::c_void) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).10)(::std::mem::transmute_copy(self), ::std::mem::transmute(cnsstart), ::std::mem::transmute(cnsduration), ::std::mem::transmute(frate), ::std::mem::transmute(pvcontext)).ok()
    }
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
    pub unsafe fn Stop(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).11)(::std::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
    pub unsafe fn Pause(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).12)(::std::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
    pub unsafe fn Resume(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).13)(::std::mem::transmute_copy(self)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IWMReader {
    type Vtable = IWMReader_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2520804310, 11051, 4563, [179, 107, 0, 192, 79, 97, 8, 255]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMReader_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pwszurl: super::super::Foundation::PWSTR, pcallback: ::windows::runtime::RawPtr, pvcontext: *const ::std::ffi::c_void) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pcoutputs: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwoutputnum: u32, ppoutput: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwoutputnum: u32, poutput: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwoutputnumber: u32, pcformats: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwoutputnumber: u32, dwformatnumber: u32, ppprops: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, cnsstart: u64, cnsduration: u64, frate: f32, pvcontext: *const ::std::ffi::c_void) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct IWMReaderAccelerator(::windows::runtime::IUnknown);
impl IWMReaderAccelerator {
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
    pub unsafe fn GetCodecInterface(&self, dwoutputnum: u32, riid: *const ::windows::runtime::GUID, ppvcodecinterface: *mut *mut ::std::ffi::c_void) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), ::std::mem::transmute(dwoutputnum), ::std::mem::transmute(riid), ::std::mem::transmute(ppvcodecinterface)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`, `Win32_Foundation`*"]
    pub unsafe fn Notify(&self, dwoutputnum: u32, psubtype: *const WM_MEDIA_TYPE) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), ::std::mem::transmute(dwoutputnum), ::std::mem::transmute(psubtype)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IWMReaderAccelerator {
    type Vtable = IWMReaderAccelerator_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3185331464, 37965, 19794, [166, 18, 70, 195, 253, 160, 125, 212]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMReaderAccelerator_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwoutputnum: u32, riid: *const ::windows::runtime::GUID, ppvcodecinterface: *mut *mut ::std::ffi::c_void) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwoutputnum: u32, psubtype: *const ::std::mem::ManuallyDrop<WM_MEDIA_TYPE>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct IWMReaderAdvanced(::windows::runtime::IUnknown);
impl IWMReaderAdvanced {
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`, `Win32_Foundation`*"]
    pub unsafe fn SetUserProvidedClock<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>>(&self, fuserclock: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), fuserclock.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`, `Win32_Foundation`*"]
    pub unsafe fn GetUserProvidedClock(&self) -> ::windows::runtime::Result<super::super::Foundation::BOOL> {
        let mut result__: <super::super::Foundation::BOOL as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
    pub unsafe fn DeliverTime(&self, cnstime: u64) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self), ::std::mem::transmute(cnstime)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`, `Win32_Foundation`*"]
    pub unsafe fn SetManualStreamSelection<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>>(&self, fselection: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::std::mem::transmute_copy(self), fselection.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`, `Win32_Foundation`*"]
    pub unsafe fn GetManualStreamSelection(&self) -> ::windows::runtime::Result<super::super::Foundation::BOOL> {
        let mut result__: <super::super::Foundation::BOOL as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
    pub unsafe fn SetStreamsSelected(&self, cstreamcount: u16, pwstreamnumbers: *const u16, pselections: *const WMT_STREAM_SELECTION) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).8)(::std::mem::transmute_copy(self), ::std::mem::transmute(cstreamcount), ::std::mem::transmute(pwstreamnumbers), ::std::mem::transmute(pselections)).ok()
    }
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
    pub unsafe fn GetStreamSelected(&self, wstreamnum: u16) -> ::windows::runtime::Result<WMT_STREAM_SELECTION> {
        let mut result__: <WMT_STREAM_SELECTION as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).9)(::std::mem::transmute_copy(self), ::std::mem::transmute(wstreamnum), &mut result__).from_abi::<WMT_STREAM_SELECTION>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`, `Win32_Foundation`*"]
    pub unsafe fn SetReceiveSelectionCallbacks<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>>(&self, fgetcallbacks: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).10)(::std::mem::transmute_copy(self), fgetcallbacks.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`, `Win32_Foundation`*"]
    pub unsafe fn GetReceiveSelectionCallbacks(&self) -> ::windows::runtime::Result<super::super::Foundation::BOOL> {
        let mut result__: <super::super::Foundation::BOOL as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).11)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`, `Win32_Foundation`*"]
    pub unsafe fn SetReceiveStreamSamples<'a, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>>(&self, wstreamnum: u16, freceivestreamsamples: Param1) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).12)(::std::mem::transmute_copy(self), ::std::mem::transmute(wstreamnum), freceivestreamsamples.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`, `Win32_Foundation`*"]
    pub unsafe fn GetReceiveStreamSamples(&self, wstreamnum: u16) -> ::windows::runtime::Result<super::super::Foundation::BOOL> {
        let mut result__: <super::super::Foundation::BOOL as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).13)(::std::mem::transmute_copy(self), ::std::mem::transmute(wstreamnum), &mut result__).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`, `Win32_Foundation`*"]
    pub unsafe fn SetAllocateForOutput<'a, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>>(&self, dwoutputnum: u32, fallocate: Param1) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).14)(::std::mem::transmute_copy(self), ::std::mem::transmute(dwoutputnum), fallocate.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`, `Win32_Foundation`*"]
    pub unsafe fn GetAllocateForOutput(&self, dwoutputnum: u32) -> ::windows::runtime::Result<super::super::Foundation::BOOL> {
        let mut result__: <super::super::Foundation::BOOL as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).15)(::std::mem::transmute_copy(self), ::std::mem::transmute(dwoutputnum), &mut result__).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`, `Win32_Foundation`*"]
    pub unsafe fn SetAllocateForStream<'a, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>>(&self, wstreamnum: u16, fallocate: Param1) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).16)(::std::mem::transmute_copy(self), ::std::mem::transmute(wstreamnum), fallocate.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`, `Win32_Foundation`*"]
    pub unsafe fn GetAllocateForStream(&self, dwsreamnum: u16) -> ::windows::runtime::Result<super::super::Foundation::BOOL> {
        let mut result__: <super::super::Foundation::BOOL as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).17)(::std::mem::transmute_copy(self), ::std::mem::transmute(dwsreamnum), &mut result__).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
    pub unsafe fn GetStatistics(&self, pstatistics: *mut WM_READER_STATISTICS) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).18)(::std::mem::transmute_copy(self), ::std::mem::transmute(pstatistics)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`, `Win32_Foundation`*"]
    pub unsafe fn SetClientInfo(&self, pclientinfo: *const WM_READER_CLIENTINFO) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).19)(::std::mem::transmute_copy(self), ::std::mem::transmute(pclientinfo)).ok()
    }
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
    pub unsafe fn GetMaxOutputSampleSize(&self, dwoutput: u32) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).20)(::std::mem::transmute_copy(self), ::std::mem::transmute(dwoutput), &mut result__).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
    pub unsafe fn GetMaxStreamSampleSize(&self, wstream: u16) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).21)(::std::mem::transmute_copy(self), ::std::mem::transmute(wstream), &mut result__).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
    pub unsafe fn NotifyLateDelivery(&self, cnslateness: u64) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).22)(::std::mem::transmute_copy(self), ::std::mem::transmute(cnslateness)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IWMReaderAdvanced {
    type Vtable = IWMReaderAdvanced_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2520804330, 11051, 4563, [179, 107, 0, 192, 79, 97, 8, 255]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMReaderAdvanced_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, fuserclock: super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pfuserclock: *mut super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, cnstime: u64) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, fselection: super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pfselection: *mut super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, cstreamcount: u16, pwstreamnumbers: *const u16, pselections: *const WMT_STREAM_SELECTION) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, wstreamnum: u16, pselection: *mut WMT_STREAM_SELECTION) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, fgetcallbacks: super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pfgetcallbacks: *mut super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, wstreamnum: u16, freceivestreamsamples: super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, wstreamnum: u16, pfreceivestreamsamples: *mut super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwoutputnum: u32, fallocate: super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwoutputnum: u32, pfallocate: *mut super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, wstreamnum: u16, fallocate: super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwsreamnum: u16, pfallocate: *mut super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pstatistics: *mut WM_READER_STATISTICS) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pclientinfo: *const WM_READER_CLIENTINFO) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwoutput: u32, pcbmax: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, wstream: u16, pcbmax: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, cnslateness: u64) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct IWMReaderAdvanced2(::windows::runtime::IUnknown);
impl IWMReaderAdvanced2 {
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`, `Win32_Foundation`*"]
    pub unsafe fn SetUserProvidedClock<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>>(&self, fuserclock: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), fuserclock.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`, `Win32_Foundation`*"]
    pub unsafe fn GetUserProvidedClock(&self) -> ::windows::runtime::Result<super::super::Foundation::BOOL> {
        let mut result__: <super::super::Foundation::BOOL as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
    pub unsafe fn DeliverTime(&self, cnstime: u64) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self), ::std::mem::transmute(cnstime)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`, `Win32_Foundation`*"]
    pub unsafe fn SetManualStreamSelection<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>>(&self, fselection: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::std::mem::transmute_copy(self), fselection.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`, `Win32_Foundation`*"]
    pub unsafe fn GetManualStreamSelection(&self) -> ::windows::runtime::Result<super::super::Foundation::BOOL> {
        let mut result__: <super::super::Foundation::BOOL as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
    pub unsafe fn SetStreamsSelected(&self, cstreamcount: u16, pwstreamnumbers: *const u16, pselections: *const WMT_STREAM_SELECTION) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).8)(::std::mem::transmute_copy(self), ::std::mem::transmute(cstreamcount), ::std::mem::transmute(pwstreamnumbers), ::std::mem::transmute(pselections)).ok()
    }
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
    pub unsafe fn GetStreamSelected(&self, wstreamnum: u16) -> ::windows::runtime::Result<WMT_STREAM_SELECTION> {
        let mut result__: <WMT_STREAM_SELECTION as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).9)(::std::mem::transmute_copy(self), ::std::mem::transmute(wstreamnum), &mut result__).from_abi::<WMT_STREAM_SELECTION>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`, `Win32_Foundation`*"]
    pub unsafe fn SetReceiveSelectionCallbacks<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>>(&self, fgetcallbacks: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).10)(::std::mem::transmute_copy(self), fgetcallbacks.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`, `Win32_Foundation`*"]
    pub unsafe fn GetReceiveSelectionCallbacks(&self) -> ::windows::runtime::Result<super::super::Foundation::BOOL> {
        let mut result__: <super::super::Foundation::BOOL as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).11)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`, `Win32_Foundation`*"]
    pub unsafe fn SetReceiveStreamSamples<'a, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>>(&self, wstreamnum: u16, freceivestreamsamples: Param1) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).12)(::std::mem::transmute_copy(self), ::std::mem::transmute(wstreamnum), freceivestreamsamples.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`, `Win32_Foundation`*"]
    pub unsafe fn GetReceiveStreamSamples(&self, wstreamnum: u16) -> ::windows::runtime::Result<super::super::Foundation::BOOL> {
        let mut result__: <super::super::Foundation::BOOL as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).13)(::std::mem::transmute_copy(self), ::std::mem::transmute(wstreamnum), &mut result__).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`, `Win32_Foundation`*"]
    pub unsafe fn SetAllocateForOutput<'a, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>>(&self, dwoutputnum: u32, fallocate: Param1) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).14)(::std::mem::transmute_copy(self), ::std::mem::transmute(dwoutputnum), fallocate.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`, `Win32_Foundation`*"]
    pub unsafe fn GetAllocateForOutput(&self, dwoutputnum: u32) -> ::windows::runtime::Result<super::super::Foundation::BOOL> {
        let mut result__: <super::super::Foundation::BOOL as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).15)(::std::mem::transmute_copy(self), ::std::mem::transmute(dwoutputnum), &mut result__).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`, `Win32_Foundation`*"]
    pub unsafe fn SetAllocateForStream<'a, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>>(&self, wstreamnum: u16, fallocate: Param1) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).16)(::std::mem::transmute_copy(self), ::std::mem::transmute(wstreamnum), fallocate.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`, `Win32_Foundation`*"]
    pub unsafe fn GetAllocateForStream(&self, dwsreamnum: u16) -> ::windows::runtime::Result<super::super::Foundation::BOOL> {
        let mut result__: <super::super::Foundation::BOOL as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).17)(::std::mem::transmute_copy(self), ::std::mem::transmute(dwsreamnum), &mut result__).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
    pub unsafe fn GetStatistics(&self, pstatistics: *mut WM_READER_STATISTICS) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).18)(::std::mem::transmute_copy(self), ::std::mem::transmute(pstatistics)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`, `Win32_Foundation`*"]
    pub unsafe fn SetClientInfo(&self, pclientinfo: *const WM_READER_CLIENTINFO) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).19)(::std::mem::transmute_copy(self), ::std::mem::transmute(pclientinfo)).ok()
    }
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
    pub unsafe fn GetMaxOutputSampleSize(&self, dwoutput: u32) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).20)(::std::mem::transmute_copy(self), ::std::mem::transmute(dwoutput), &mut result__).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
    pub unsafe fn GetMaxStreamSampleSize(&self, wstream: u16) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).21)(::std::mem::transmute_copy(self), ::std::mem::transmute(wstream), &mut result__).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
    pub unsafe fn NotifyLateDelivery(&self, cnslateness: u64) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).22)(::std::mem::transmute_copy(self), ::std::mem::transmute(cnslateness)).ok()
    }
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
    pub unsafe fn SetPlayMode(&self, mode: WMT_PLAY_MODE) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).23)(::std::mem::transmute_copy(self), ::std::mem::transmute(mode)).ok()
    }
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
    pub unsafe fn GetPlayMode(&self) -> ::windows::runtime::Result<WMT_PLAY_MODE> {
        let mut result__: <WMT_PLAY_MODE as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).24)(::std::mem::transmute_copy(self), &mut result__).from_abi::<WMT_PLAY_MODE>(result__)
    }
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
    pub unsafe fn GetBufferProgress(&self, pdwpercent: *mut u32, pcnsbuffering: *mut u64) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).25)(::std::mem::transmute_copy(self), ::std::mem::transmute(pdwpercent), ::std::mem::transmute(pcnsbuffering)).ok()
    }
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
    pub unsafe fn GetDownloadProgress(&self, pdwpercent: *mut u32, pqwbytesdownloaded: *mut u64, pcnsdownload: *mut u64) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).26)(::std::mem::transmute_copy(self), ::std::mem::transmute(pdwpercent), ::std::mem::transmute(pqwbytesdownloaded), ::std::mem::transmute(pcnsdownload)).ok()
    }
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
    pub unsafe fn GetSaveAsProgress(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).27)(::std::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`, `Win32_Foundation`*"]
    pub unsafe fn SaveFileAs<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, pwszfilename: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).28)(::std::mem::transmute_copy(self), pwszfilename.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`, `Win32_Foundation`*"]
    pub unsafe fn GetProtocolName(&self, pwszprotocol: super::super::Foundation::PWSTR, pcchprotocol: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).29)(::std::mem::transmute_copy(self), ::std::mem::transmute(pwszprotocol), ::std::mem::transmute(pcchprotocol)).ok()
    }
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
    pub unsafe fn StartAtMarker(&self, wmarkerindex: u16, cnsduration: u64, frate: f32, pvcontext: *const ::std::ffi::c_void) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).30)(::std::mem::transmute_copy(self), ::std::mem::transmute(wmarkerindex), ::std::mem::transmute(cnsduration), ::std::mem::transmute(frate), ::std::mem::transmute(pvcontext)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`, `Win32_Foundation`*"]
    pub unsafe fn GetOutputSetting<'a, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, dwoutputnum: u32, pszname: Param1, ptype: *mut WMT_ATTR_DATATYPE, pvalue: *mut u8, pcblength: *mut u16) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).31)(::std::mem::transmute_copy(self), ::std::mem::transmute(dwoutputnum), pszname.into_param().abi(), ::std::mem::transmute(ptype), ::std::mem::transmute(pvalue), ::std::mem::transmute(pcblength)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`, `Win32_Foundation`*"]
    pub unsafe fn SetOutputSetting<'a, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, dwoutputnum: u32, pszname: Param1, r#type: WMT_ATTR_DATATYPE, pvalue: *const u8, cblength: u16) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).32)(::std::mem::transmute_copy(self), ::std::mem::transmute(dwoutputnum), pszname.into_param().abi(), ::std::mem::transmute(r#type), ::std::mem::transmute(pvalue), ::std::mem::transmute(cblength)).ok()
    }
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
    pub unsafe fn Preroll(&self, cnsstart: u64, cnsduration: u64, frate: f32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).33)(::std::mem::transmute_copy(self), ::std::mem::transmute(cnsstart), ::std::mem::transmute(cnsduration), ::std::mem::transmute(frate)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`, `Win32_Foundation`*"]
    pub unsafe fn SetLogClientID<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>>(&self, flogclientid: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).34)(::std::mem::transmute_copy(self), flogclientid.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`, `Win32_Foundation`*"]
    pub unsafe fn GetLogClientID(&self) -> ::windows::runtime::Result<super::super::Foundation::BOOL> {
        let mut result__: <super::super::Foundation::BOOL as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).35)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
    pub unsafe fn StopBuffering(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).36)(::std::mem::transmute_copy(self)).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`, `Win32_System_Com`*"]
    pub unsafe fn OpenStream<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::System::Com::IStream>, Param1: ::windows::runtime::IntoParam<'a, IWMReaderCallback>>(&self, pstream: Param0, pcallback: Param1, pvcontext: *const ::std::ffi::c_void) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).37)(::std::mem::transmute_copy(self), pstream.into_param().abi(), pcallback.into_param().abi(), ::std::mem::transmute(pvcontext)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IWMReaderAdvanced2 {
    type Vtable = IWMReaderAdvanced2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2920589637, 47372, 19725, [145, 39, 128, 214, 101, 247, 215, 62]);
}
impl ::std::convert::From<IWMReaderAdvanced2> for IWMReaderAdvanced {
    fn from(value: IWMReaderAdvanced2) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IWMReaderAdvanced2> for IWMReaderAdvanced {
    fn from(value: &IWMReaderAdvanced2) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IWMReaderAdvanced> for IWMReaderAdvanced2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IWMReaderAdvanced> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IWMReaderAdvanced>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IWMReaderAdvanced> for &IWMReaderAdvanced2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IWMReaderAdvanced> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IWMReaderAdvanced>::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMReaderAdvanced2_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, fuserclock: super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pfuserclock: *mut super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, cnstime: u64) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, fselection: super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pfselection: *mut super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, cstreamcount: u16, pwstreamnumbers: *const u16, pselections: *const WMT_STREAM_SELECTION) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, wstreamnum: u16, pselection: *mut WMT_STREAM_SELECTION) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, fgetcallbacks: super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pfgetcallbacks: *mut super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, wstreamnum: u16, freceivestreamsamples: super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, wstreamnum: u16, pfreceivestreamsamples: *mut super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwoutputnum: u32, fallocate: super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwoutputnum: u32, pfallocate: *mut super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, wstreamnum: u16, fallocate: super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwsreamnum: u16, pfallocate: *mut super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pstatistics: *mut WM_READER_STATISTICS) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pclientinfo: *const WM_READER_CLIENTINFO) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwoutput: u32, pcbmax: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, wstream: u16, pcbmax: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, cnslateness: u64) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, mode: WMT_PLAY_MODE) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pmode: *mut WMT_PLAY_MODE) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pdwpercent: *mut u32, pcnsbuffering: *mut u64) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pdwpercent: *mut u32, pqwbytesdownloaded: *mut u64, pcnsdownload: *mut u64) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pdwpercent: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pwszfilename: super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pwszprotocol: super::super::Foundation::PWSTR, pcchprotocol: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, wmarkerindex: u16, cnsduration: u64, frate: f32, pvcontext: *const ::std::ffi::c_void) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwoutputnum: u32, pszname: super::super::Foundation::PWSTR, ptype: *mut WMT_ATTR_DATATYPE, pvalue: *mut u8, pcblength: *mut u16) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwoutputnum: u32, pszname: super::super::Foundation::PWSTR, r#type: WMT_ATTR_DATATYPE, pvalue: *const u8, cblength: u16) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, cnsstart: u64, cnsduration: u64, frate: f32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, flogclientid: super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pflogclientid: *mut super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pstream: ::windows::runtime::RawPtr, pcallback: ::windows::runtime::RawPtr, pvcontext: *const ::std::ffi::c_void) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
);
#[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct IWMReaderAdvanced3(::windows::runtime::IUnknown);
impl IWMReaderAdvanced3 {
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`, `Win32_Foundation`*"]
    pub unsafe fn SetUserProvidedClock<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>>(&self, fuserclock: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), fuserclock.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`, `Win32_Foundation`*"]
    pub unsafe fn GetUserProvidedClock(&self) -> ::windows::runtime::Result<super::super::Foundation::BOOL> {
        let mut result__: <super::super::Foundation::BOOL as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
    pub unsafe fn DeliverTime(&self, cnstime: u64) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self), ::std::mem::transmute(cnstime)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`, `Win32_Foundation`*"]
    pub unsafe fn SetManualStreamSelection<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>>(&self, fselection: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::std::mem::transmute_copy(self), fselection.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`, `Win32_Foundation`*"]
    pub unsafe fn GetManualStreamSelection(&self) -> ::windows::runtime::Result<super::super::Foundation::BOOL> {
        let mut result__: <super::super::Foundation::BOOL as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
    pub unsafe fn SetStreamsSelected(&self, cstreamcount: u16, pwstreamnumbers: *const u16, pselections: *const WMT_STREAM_SELECTION) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).8)(::std::mem::transmute_copy(self), ::std::mem::transmute(cstreamcount), ::std::mem::transmute(pwstreamnumbers), ::std::mem::transmute(pselections)).ok()
    }
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
    pub unsafe fn GetStreamSelected(&self, wstreamnum: u16) -> ::windows::runtime::Result<WMT_STREAM_SELECTION> {
        let mut result__: <WMT_STREAM_SELECTION as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).9)(::std::mem::transmute_copy(self), ::std::mem::transmute(wstreamnum), &mut result__).from_abi::<WMT_STREAM_SELECTION>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`, `Win32_Foundation`*"]
    pub unsafe fn SetReceiveSelectionCallbacks<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>>(&self, fgetcallbacks: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).10)(::std::mem::transmute_copy(self), fgetcallbacks.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`, `Win32_Foundation`*"]
    pub unsafe fn GetReceiveSelectionCallbacks(&self) -> ::windows::runtime::Result<super::super::Foundation::BOOL> {
        let mut result__: <super::super::Foundation::BOOL as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).11)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`, `Win32_Foundation`*"]
    pub unsafe fn SetReceiveStreamSamples<'a, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>>(&self, wstreamnum: u16, freceivestreamsamples: Param1) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).12)(::std::mem::transmute_copy(self), ::std::mem::transmute(wstreamnum), freceivestreamsamples.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`, `Win32_Foundation`*"]
    pub unsafe fn GetReceiveStreamSamples(&self, wstreamnum: u16) -> ::windows::runtime::Result<super::super::Foundation::BOOL> {
        let mut result__: <super::super::Foundation::BOOL as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).13)(::std::mem::transmute_copy(self), ::std::mem::transmute(wstreamnum), &mut result__).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`, `Win32_Foundation`*"]
    pub unsafe fn SetAllocateForOutput<'a, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>>(&self, dwoutputnum: u32, fallocate: Param1) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).14)(::std::mem::transmute_copy(self), ::std::mem::transmute(dwoutputnum), fallocate.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`, `Win32_Foundation`*"]
    pub unsafe fn GetAllocateForOutput(&self, dwoutputnum: u32) -> ::windows::runtime::Result<super::super::Foundation::BOOL> {
        let mut result__: <super::super::Foundation::BOOL as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).15)(::std::mem::transmute_copy(self), ::std::mem::transmute(dwoutputnum), &mut result__).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`, `Win32_Foundation`*"]
    pub unsafe fn SetAllocateForStream<'a, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>>(&self, wstreamnum: u16, fallocate: Param1) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).16)(::std::mem::transmute_copy(self), ::std::mem::transmute(wstreamnum), fallocate.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`, `Win32_Foundation`*"]
    pub unsafe fn GetAllocateForStream(&self, dwsreamnum: u16) -> ::windows::runtime::Result<super::super::Foundation::BOOL> {
        let mut result__: <super::super::Foundation::BOOL as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).17)(::std::mem::transmute_copy(self), ::std::mem::transmute(dwsreamnum), &mut result__).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
    pub unsafe fn GetStatistics(&self, pstatistics: *mut WM_READER_STATISTICS) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).18)(::std::mem::transmute_copy(self), ::std::mem::transmute(pstatistics)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`, `Win32_Foundation`*"]
    pub unsafe fn SetClientInfo(&self, pclientinfo: *const WM_READER_CLIENTINFO) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).19)(::std::mem::transmute_copy(self), ::std::mem::transmute(pclientinfo)).ok()
    }
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
    pub unsafe fn GetMaxOutputSampleSize(&self, dwoutput: u32) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).20)(::std::mem::transmute_copy(self), ::std::mem::transmute(dwoutput), &mut result__).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
    pub unsafe fn GetMaxStreamSampleSize(&self, wstream: u16) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).21)(::std::mem::transmute_copy(self), ::std::mem::transmute(wstream), &mut result__).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
    pub unsafe fn NotifyLateDelivery(&self, cnslateness: u64) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).22)(::std::mem::transmute_copy(self), ::std::mem::transmute(cnslateness)).ok()
    }
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
    pub unsafe fn SetPlayMode(&self, mode: WMT_PLAY_MODE) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).23)(::std::mem::transmute_copy(self), ::std::mem::transmute(mode)).ok()
    }
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
    pub unsafe fn GetPlayMode(&self) -> ::windows::runtime::Result<WMT_PLAY_MODE> {
        let mut result__: <WMT_PLAY_MODE as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).24)(::std::mem::transmute_copy(self), &mut result__).from_abi::<WMT_PLAY_MODE>(result__)
    }
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
    pub unsafe fn GetBufferProgress(&self, pdwpercent: *mut u32, pcnsbuffering: *mut u64) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).25)(::std::mem::transmute_copy(self), ::std::mem::transmute(pdwpercent), ::std::mem::transmute(pcnsbuffering)).ok()
    }
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
    pub unsafe fn GetDownloadProgress(&self, pdwpercent: *mut u32, pqwbytesdownloaded: *mut u64, pcnsdownload: *mut u64) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).26)(::std::mem::transmute_copy(self), ::std::mem::transmute(pdwpercent), ::std::mem::transmute(pqwbytesdownloaded), ::std::mem::transmute(pcnsdownload)).ok()
    }
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
    pub unsafe fn GetSaveAsProgress(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).27)(::std::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`, `Win32_Foundation`*"]
    pub unsafe fn SaveFileAs<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, pwszfilename: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).28)(::std::mem::transmute_copy(self), pwszfilename.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`, `Win32_Foundation`*"]
    pub unsafe fn GetProtocolName(&self, pwszprotocol: super::super::Foundation::PWSTR, pcchprotocol: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).29)(::std::mem::transmute_copy(self), ::std::mem::transmute(pwszprotocol), ::std::mem::transmute(pcchprotocol)).ok()
    }
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
    pub unsafe fn StartAtMarker(&self, wmarkerindex: u16, cnsduration: u64, frate: f32, pvcontext: *const ::std::ffi::c_void) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).30)(::std::mem::transmute_copy(self), ::std::mem::transmute(wmarkerindex), ::std::mem::transmute(cnsduration), ::std::mem::transmute(frate), ::std::mem::transmute(pvcontext)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`, `Win32_Foundation`*"]
    pub unsafe fn GetOutputSetting<'a, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, dwoutputnum: u32, pszname: Param1, ptype: *mut WMT_ATTR_DATATYPE, pvalue: *mut u8, pcblength: *mut u16) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).31)(::std::mem::transmute_copy(self), ::std::mem::transmute(dwoutputnum), pszname.into_param().abi(), ::std::mem::transmute(ptype), ::std::mem::transmute(pvalue), ::std::mem::transmute(pcblength)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`, `Win32_Foundation`*"]
    pub unsafe fn SetOutputSetting<'a, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, dwoutputnum: u32, pszname: Param1, r#type: WMT_ATTR_DATATYPE, pvalue: *const u8, cblength: u16) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).32)(::std::mem::transmute_copy(self), ::std::mem::transmute(dwoutputnum), pszname.into_param().abi(), ::std::mem::transmute(r#type), ::std::mem::transmute(pvalue), ::std::mem::transmute(cblength)).ok()
    }
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
    pub unsafe fn Preroll(&self, cnsstart: u64, cnsduration: u64, frate: f32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).33)(::std::mem::transmute_copy(self), ::std::mem::transmute(cnsstart), ::std::mem::transmute(cnsduration), ::std::mem::transmute(frate)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`, `Win32_Foundation`*"]
    pub unsafe fn SetLogClientID<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>>(&self, flogclientid: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).34)(::std::mem::transmute_copy(self), flogclientid.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`, `Win32_Foundation`*"]
    pub unsafe fn GetLogClientID(&self) -> ::windows::runtime::Result<super::super::Foundation::BOOL> {
        let mut result__: <super::super::Foundation::BOOL as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).35)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
    pub unsafe fn StopBuffering(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).36)(::std::mem::transmute_copy(self)).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`, `Win32_System_Com`*"]
    pub unsafe fn OpenStream<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::System::Com::IStream>, Param1: ::windows::runtime::IntoParam<'a, IWMReaderCallback>>(&self, pstream: Param0, pcallback: Param1, pvcontext: *const ::std::ffi::c_void) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).37)(::std::mem::transmute_copy(self), pstream.into_param().abi(), pcallback.into_param().abi(), ::std::mem::transmute(pvcontext)).ok()
    }
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
    pub unsafe fn StopNetStreaming(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).38)(::std::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
    pub unsafe fn StartAtPosition(&self, wstreamnum: u16, pvoffsetstart: *const ::std::ffi::c_void, pvduration: *const ::std::ffi::c_void, dwoffsetformat: WMT_OFFSET_FORMAT, frate: f32, pvcontext: *const ::std::ffi::c_void) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).39)(::std::mem::transmute_copy(self), ::std::mem::transmute(wstreamnum), ::std::mem::transmute(pvoffsetstart), ::std::mem::transmute(pvduration), ::std::mem::transmute(dwoffsetformat), ::std::mem::transmute(frate), ::std::mem::transmute(pvcontext)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IWMReaderAdvanced3 {
    type Vtable = IWMReaderAdvanced3_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1572890443, 61515, 19022, [159, 42, 177, 175, 222, 44, 129, 0]);
}
impl ::std::convert::From<IWMReaderAdvanced3> for IWMReaderAdvanced2 {
    fn from(value: IWMReaderAdvanced3) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IWMReaderAdvanced3> for IWMReaderAdvanced2 {
    fn from(value: &IWMReaderAdvanced3) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IWMReaderAdvanced2> for IWMReaderAdvanced3 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IWMReaderAdvanced2> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IWMReaderAdvanced2>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IWMReaderAdvanced2> for &IWMReaderAdvanced3 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IWMReaderAdvanced2> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IWMReaderAdvanced2>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<IWMReaderAdvanced3> for IWMReaderAdvanced {
    fn from(value: IWMReaderAdvanced3) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IWMReaderAdvanced3> for IWMReaderAdvanced {
    fn from(value: &IWMReaderAdvanced3) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IWMReaderAdvanced> for IWMReaderAdvanced3 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IWMReaderAdvanced> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IWMReaderAdvanced>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IWMReaderAdvanced> for &IWMReaderAdvanced3 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IWMReaderAdvanced> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IWMReaderAdvanced>::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMReaderAdvanced3_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, fuserclock: super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pfuserclock: *mut super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, cnstime: u64) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, fselection: super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pfselection: *mut super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, cstreamcount: u16, pwstreamnumbers: *const u16, pselections: *const WMT_STREAM_SELECTION) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, wstreamnum: u16, pselection: *mut WMT_STREAM_SELECTION) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, fgetcallbacks: super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pfgetcallbacks: *mut super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, wstreamnum: u16, freceivestreamsamples: super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, wstreamnum: u16, pfreceivestreamsamples: *mut super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwoutputnum: u32, fallocate: super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwoutputnum: u32, pfallocate: *mut super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, wstreamnum: u16, fallocate: super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwsreamnum: u16, pfallocate: *mut super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pstatistics: *mut WM_READER_STATISTICS) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pclientinfo: *const WM_READER_CLIENTINFO) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwoutput: u32, pcbmax: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, wstream: u16, pcbmax: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, cnslateness: u64) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, mode: WMT_PLAY_MODE) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pmode: *mut WMT_PLAY_MODE) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pdwpercent: *mut u32, pcnsbuffering: *mut u64) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pdwpercent: *mut u32, pqwbytesdownloaded: *mut u64, pcnsdownload: *mut u64) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pdwpercent: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pwszfilename: super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pwszprotocol: super::super::Foundation::PWSTR, pcchprotocol: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, wmarkerindex: u16, cnsduration: u64, frate: f32, pvcontext: *const ::std::ffi::c_void) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwoutputnum: u32, pszname: super::super::Foundation::PWSTR, ptype: *mut WMT_ATTR_DATATYPE, pvalue: *mut u8, pcblength: *mut u16) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwoutputnum: u32, pszname: super::super::Foundation::PWSTR, r#type: WMT_ATTR_DATATYPE, pvalue: *const u8, cblength: u16) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, cnsstart: u64, cnsduration: u64, frate: f32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, flogclientid: super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pflogclientid: *mut super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pstream: ::windows::runtime::RawPtr, pcallback: ::windows::runtime::RawPtr, pvcontext: *const ::std::ffi::c_void) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, wstreamnum: u16, pvoffsetstart: *const ::std::ffi::c_void, pvduration: *const ::std::ffi::c_void, dwoffsetformat: WMT_OFFSET_FORMAT, frate: f32, pvcontext: *const ::std::ffi::c_void) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct IWMReaderAdvanced4(::windows::runtime::IUnknown);
impl IWMReaderAdvanced4 {
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`, `Win32_Foundation`*"]
    pub unsafe fn SetUserProvidedClock<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>>(&self, fuserclock: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), fuserclock.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`, `Win32_Foundation`*"]
    pub unsafe fn GetUserProvidedClock(&self) -> ::windows::runtime::Result<super::super::Foundation::BOOL> {
        let mut result__: <super::super::Foundation::BOOL as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
    pub unsafe fn DeliverTime(&self, cnstime: u64) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self), ::std::mem::transmute(cnstime)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`, `Win32_Foundation`*"]
    pub unsafe fn SetManualStreamSelection<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>>(&self, fselection: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::std::mem::transmute_copy(self), fselection.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`, `Win32_Foundation`*"]
    pub unsafe fn GetManualStreamSelection(&self) -> ::windows::runtime::Result<super::super::Foundation::BOOL> {
        let mut result__: <super::super::Foundation::BOOL as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
    pub unsafe fn SetStreamsSelected(&self, cstreamcount: u16, pwstreamnumbers: *const u16, pselections: *const WMT_STREAM_SELECTION) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).8)(::std::mem::transmute_copy(self), ::std::mem::transmute(cstreamcount), ::std::mem::transmute(pwstreamnumbers), ::std::mem::transmute(pselections)).ok()
    }
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
    pub unsafe fn GetStreamSelected(&self, wstreamnum: u16) -> ::windows::runtime::Result<WMT_STREAM_SELECTION> {
        let mut result__: <WMT_STREAM_SELECTION as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).9)(::std::mem::transmute_copy(self), ::std::mem::transmute(wstreamnum), &mut result__).from_abi::<WMT_STREAM_SELECTION>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`, `Win32_Foundation`*"]
    pub unsafe fn SetReceiveSelectionCallbacks<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>>(&self, fgetcallbacks: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).10)(::std::mem::transmute_copy(self), fgetcallbacks.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`, `Win32_Foundation`*"]
    pub unsafe fn GetReceiveSelectionCallbacks(&self) -> ::windows::runtime::Result<super::super::Foundation::BOOL> {
        let mut result__: <super::super::Foundation::BOOL as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).11)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`, `Win32_Foundation`*"]
    pub unsafe fn SetReceiveStreamSamples<'a, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>>(&self, wstreamnum: u16, freceivestreamsamples: Param1) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).12)(::std::mem::transmute_copy(self), ::std::mem::transmute(wstreamnum), freceivestreamsamples.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`, `Win32_Foundation`*"]
    pub unsafe fn GetReceiveStreamSamples(&self, wstreamnum: u16) -> ::windows::runtime::Result<super::super::Foundation::BOOL> {
        let mut result__: <super::super::Foundation::BOOL as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).13)(::std::mem::transmute_copy(self), ::std::mem::transmute(wstreamnum), &mut result__).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`, `Win32_Foundation`*"]
    pub unsafe fn SetAllocateForOutput<'a, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>>(&self, dwoutputnum: u32, fallocate: Param1) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).14)(::std::mem::transmute_copy(self), ::std::mem::transmute(dwoutputnum), fallocate.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`, `Win32_Foundation`*"]
    pub unsafe fn GetAllocateForOutput(&self, dwoutputnum: u32) -> ::windows::runtime::Result<super::super::Foundation::BOOL> {
        let mut result__: <super::super::Foundation::BOOL as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).15)(::std::mem::transmute_copy(self), ::std::mem::transmute(dwoutputnum), &mut result__).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`, `Win32_Foundation`*"]
    pub unsafe fn SetAllocateForStream<'a, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>>(&self, wstreamnum: u16, fallocate: Param1) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).16)(::std::mem::transmute_copy(self), ::std::mem::transmute(wstreamnum), fallocate.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`, `Win32_Foundation`*"]
    pub unsafe fn GetAllocateForStream(&self, dwsreamnum: u16) -> ::windows::runtime::Result<super::super::Foundation::BOOL> {
        let mut result__: <super::super::Foundation::BOOL as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).17)(::std::mem::transmute_copy(self), ::std::mem::transmute(dwsreamnum), &mut result__).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
    pub unsafe fn GetStatistics(&self, pstatistics: *mut WM_READER_STATISTICS) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).18)(::std::mem::transmute_copy(self), ::std::mem::transmute(pstatistics)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`, `Win32_Foundation`*"]
    pub unsafe fn SetClientInfo(&self, pclientinfo: *const WM_READER_CLIENTINFO) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).19)(::std::mem::transmute_copy(self), ::std::mem::transmute(pclientinfo)).ok()
    }
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
    pub unsafe fn GetMaxOutputSampleSize(&self, dwoutput: u32) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).20)(::std::mem::transmute_copy(self), ::std::mem::transmute(dwoutput), &mut result__).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
    pub unsafe fn GetMaxStreamSampleSize(&self, wstream: u16) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).21)(::std::mem::transmute_copy(self), ::std::mem::transmute(wstream), &mut result__).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
    pub unsafe fn NotifyLateDelivery(&self, cnslateness: u64) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).22)(::std::mem::transmute_copy(self), ::std::mem::transmute(cnslateness)).ok()
    }
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
    pub unsafe fn SetPlayMode(&self, mode: WMT_PLAY_MODE) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).23)(::std::mem::transmute_copy(self), ::std::mem::transmute(mode)).ok()
    }
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
    pub unsafe fn GetPlayMode(&self) -> ::windows::runtime::Result<WMT_PLAY_MODE> {
        let mut result__: <WMT_PLAY_MODE as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).24)(::std::mem::transmute_copy(self), &mut result__).from_abi::<WMT_PLAY_MODE>(result__)
    }
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
    pub unsafe fn GetBufferProgress(&self, pdwpercent: *mut u32, pcnsbuffering: *mut u64) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).25)(::std::mem::transmute_copy(self), ::std::mem::transmute(pdwpercent), ::std::mem::transmute(pcnsbuffering)).ok()
    }
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
    pub unsafe fn GetDownloadProgress(&self, pdwpercent: *mut u32, pqwbytesdownloaded: *mut u64, pcnsdownload: *mut u64) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).26)(::std::mem::transmute_copy(self), ::std::mem::transmute(pdwpercent), ::std::mem::transmute(pqwbytesdownloaded), ::std::mem::transmute(pcnsdownload)).ok()
    }
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
    pub unsafe fn GetSaveAsProgress(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).27)(::std::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`, `Win32_Foundation`*"]
    pub unsafe fn SaveFileAs<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, pwszfilename: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).28)(::std::mem::transmute_copy(self), pwszfilename.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`, `Win32_Foundation`*"]
    pub unsafe fn GetProtocolName(&self, pwszprotocol: super::super::Foundation::PWSTR, pcchprotocol: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).29)(::std::mem::transmute_copy(self), ::std::mem::transmute(pwszprotocol), ::std::mem::transmute(pcchprotocol)).ok()
    }
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
    pub unsafe fn StartAtMarker(&self, wmarkerindex: u16, cnsduration: u64, frate: f32, pvcontext: *const ::std::ffi::c_void) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).30)(::std::mem::transmute_copy(self), ::std::mem::transmute(wmarkerindex), ::std::mem::transmute(cnsduration), ::std::mem::transmute(frate), ::std::mem::transmute(pvcontext)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`, `Win32_Foundation`*"]
    pub unsafe fn GetOutputSetting<'a, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, dwoutputnum: u32, pszname: Param1, ptype: *mut WMT_ATTR_DATATYPE, pvalue: *mut u8, pcblength: *mut u16) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).31)(::std::mem::transmute_copy(self), ::std::mem::transmute(dwoutputnum), pszname.into_param().abi(), ::std::mem::transmute(ptype), ::std::mem::transmute(pvalue), ::std::mem::transmute(pcblength)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`, `Win32_Foundation`*"]
    pub unsafe fn SetOutputSetting<'a, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, dwoutputnum: u32, pszname: Param1, r#type: WMT_ATTR_DATATYPE, pvalue: *const u8, cblength: u16) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).32)(::std::mem::transmute_copy(self), ::std::mem::transmute(dwoutputnum), pszname.into_param().abi(), ::std::mem::transmute(r#type), ::std::mem::transmute(pvalue), ::std::mem::transmute(cblength)).ok()
    }
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
    pub unsafe fn Preroll(&self, cnsstart: u64, cnsduration: u64, frate: f32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).33)(::std::mem::transmute_copy(self), ::std::mem::transmute(cnsstart), ::std::mem::transmute(cnsduration), ::std::mem::transmute(frate)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`, `Win32_Foundation`*"]
    pub unsafe fn SetLogClientID<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>>(&self, flogclientid: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).34)(::std::mem::transmute_copy(self), flogclientid.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`, `Win32_Foundation`*"]
    pub unsafe fn GetLogClientID(&self) -> ::windows::runtime::Result<super::super::Foundation::BOOL> {
        let mut result__: <super::super::Foundation::BOOL as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).35)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
    pub unsafe fn StopBuffering(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).36)(::std::mem::transmute_copy(self)).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`, `Win32_System_Com`*"]
    pub unsafe fn OpenStream<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::System::Com::IStream>, Param1: ::windows::runtime::IntoParam<'a, IWMReaderCallback>>(&self, pstream: Param0, pcallback: Param1, pvcontext: *const ::std::ffi::c_void) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).37)(::std::mem::transmute_copy(self), pstream.into_param().abi(), pcallback.into_param().abi(), ::std::mem::transmute(pvcontext)).ok()
    }
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
    pub unsafe fn StopNetStreaming(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).38)(::std::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
    pub unsafe fn StartAtPosition(&self, wstreamnum: u16, pvoffsetstart: *const ::std::ffi::c_void, pvduration: *const ::std::ffi::c_void, dwoffsetformat: WMT_OFFSET_FORMAT, frate: f32, pvcontext: *const ::std::ffi::c_void) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).39)(::std::mem::transmute_copy(self), ::std::mem::transmute(wstreamnum), ::std::mem::transmute(pvoffsetstart), ::std::mem::transmute(pvduration), ::std::mem::transmute(dwoffsetformat), ::std::mem::transmute(frate), ::std::mem::transmute(pvcontext)).ok()
    }
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
    pub unsafe fn GetLanguageCount(&self, dwoutputnum: u32) -> ::windows::runtime::Result<u16> {
        let mut result__: <u16 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).40)(::std::mem::transmute_copy(self), ::std::mem::transmute(dwoutputnum), &mut result__).from_abi::<u16>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`, `Win32_Foundation`*"]
    pub unsafe fn GetLanguage(&self, dwoutputnum: u32, wlanguage: u16, pwszlanguagestring: super::super::Foundation::PWSTR, pcchlanguagestringlength: *mut u16) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).41)(::std::mem::transmute_copy(self), ::std::mem::transmute(dwoutputnum), ::std::mem::transmute(wlanguage), ::std::mem::transmute(pwszlanguagestring), ::std::mem::transmute(pcchlanguagestringlength)).ok()
    }
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
    pub unsafe fn GetMaxSpeedFactor(&self) -> ::windows::runtime::Result<f64> {
        let mut result__: <f64 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).42)(::std::mem::transmute_copy(self), &mut result__).from_abi::<f64>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`, `Win32_Foundation`*"]
    pub unsafe fn IsUsingFastCache(&self) -> ::windows::runtime::Result<super::super::Foundation::BOOL> {
        let mut result__: <super::super::Foundation::BOOL as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).43)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`, `Win32_Foundation`*"]
    pub unsafe fn AddLogParam<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>, Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, wsznamespace: Param0, wszname: Param1, wszvalue: Param2) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).44)(::std::mem::transmute_copy(self), wsznamespace.into_param().abi(), wszname.into_param().abi(), wszvalue.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
    pub unsafe fn SendLogParams(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).45)(::std::mem::transmute_copy(self)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`, `Win32_Foundation`*"]
    pub unsafe fn CanSaveFileAs(&self) -> ::windows::runtime::Result<super::super::Foundation::BOOL> {
        let mut result__: <super::super::Foundation::BOOL as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).46)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
    pub unsafe fn CancelSaveFileAs(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).47)(::std::mem::transmute_copy(self)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`, `Win32_Foundation`*"]
    pub unsafe fn GetURL(&self, pwszurl: super::super::Foundation::PWSTR, pcchurl: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).48)(::std::mem::transmute_copy(self), ::std::mem::transmute(pwszurl), ::std::mem::transmute(pcchurl)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IWMReaderAdvanced4 {
    type Vtable = IWMReaderAdvanced4_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2488956578, 4782, 19784, [189, 60, 205, 29, 144, 57, 155, 133]);
}
impl ::std::convert::From<IWMReaderAdvanced4> for IWMReaderAdvanced3 {
    fn from(value: IWMReaderAdvanced4) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IWMReaderAdvanced4> for IWMReaderAdvanced3 {
    fn from(value: &IWMReaderAdvanced4) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IWMReaderAdvanced3> for IWMReaderAdvanced4 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IWMReaderAdvanced3> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IWMReaderAdvanced3>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IWMReaderAdvanced3> for &IWMReaderAdvanced4 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IWMReaderAdvanced3> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IWMReaderAdvanced3>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<IWMReaderAdvanced4> for IWMReaderAdvanced2 {
    fn from(value: IWMReaderAdvanced4) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IWMReaderAdvanced4> for IWMReaderAdvanced2 {
    fn from(value: &IWMReaderAdvanced4) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IWMReaderAdvanced2> for IWMReaderAdvanced4 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IWMReaderAdvanced2> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IWMReaderAdvanced2>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IWMReaderAdvanced2> for &IWMReaderAdvanced4 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IWMReaderAdvanced2> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IWMReaderAdvanced2>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<IWMReaderAdvanced4> for IWMReaderAdvanced {
    fn from(value: IWMReaderAdvanced4) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IWMReaderAdvanced4> for IWMReaderAdvanced {
    fn from(value: &IWMReaderAdvanced4) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IWMReaderAdvanced> for IWMReaderAdvanced4 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IWMReaderAdvanced> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IWMReaderAdvanced>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IWMReaderAdvanced> for &IWMReaderAdvanced4 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IWMReaderAdvanced> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IWMReaderAdvanced>::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMReaderAdvanced4_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, fuserclock: super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pfuserclock: *mut super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, cnstime: u64) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, fselection: super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pfselection: *mut super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, cstreamcount: u16, pwstreamnumbers: *const u16, pselections: *const WMT_STREAM_SELECTION) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, wstreamnum: u16, pselection: *mut WMT_STREAM_SELECTION) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, fgetcallbacks: super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pfgetcallbacks: *mut super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, wstreamnum: u16, freceivestreamsamples: super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, wstreamnum: u16, pfreceivestreamsamples: *mut super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwoutputnum: u32, fallocate: super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwoutputnum: u32, pfallocate: *mut super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, wstreamnum: u16, fallocate: super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwsreamnum: u16, pfallocate: *mut super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pstatistics: *mut WM_READER_STATISTICS) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pclientinfo: *const WM_READER_CLIENTINFO) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwoutput: u32, pcbmax: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, wstream: u16, pcbmax: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, cnslateness: u64) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, mode: WMT_PLAY_MODE) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pmode: *mut WMT_PLAY_MODE) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pdwpercent: *mut u32, pcnsbuffering: *mut u64) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pdwpercent: *mut u32, pqwbytesdownloaded: *mut u64, pcnsdownload: *mut u64) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pdwpercent: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pwszfilename: super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pwszprotocol: super::super::Foundation::PWSTR, pcchprotocol: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, wmarkerindex: u16, cnsduration: u64, frate: f32, pvcontext: *const ::std::ffi::c_void) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwoutputnum: u32, pszname: super::super::Foundation::PWSTR, ptype: *mut WMT_ATTR_DATATYPE, pvalue: *mut u8, pcblength: *mut u16) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwoutputnum: u32, pszname: super::super::Foundation::PWSTR, r#type: WMT_ATTR_DATATYPE, pvalue: *const u8, cblength: u16) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, cnsstart: u64, cnsduration: u64, frate: f32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, flogclientid: super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pflogclientid: *mut super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pstream: ::windows::runtime::RawPtr, pcallback: ::windows::runtime::RawPtr, pvcontext: *const ::std::ffi::c_void) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, wstreamnum: u16, pvoffsetstart: *const ::std::ffi::c_void, pvduration: *const ::std::ffi::c_void, dwoffsetformat: WMT_OFFSET_FORMAT, frate: f32, pvcontext: *const ::std::ffi::c_void) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwoutputnum: u32, pwlanguagecount: *mut u16) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwoutputnum: u32, wlanguage: u16, pwszlanguagestring: super::super::Foundation::PWSTR, pcchlanguagestringlength: *mut u16) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pdblfactor: *mut f64) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pfusingfastcache: *mut super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, wsznamespace: super::super::Foundation::PWSTR, wszname: super::super::Foundation::PWSTR, wszvalue: super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pfcansave: *mut super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pwszurl: super::super::Foundation::PWSTR, pcchurl: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct IWMReaderAdvanced5(::windows::runtime::IUnknown);
impl IWMReaderAdvanced5 {
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`, `Win32_Foundation`*"]
    pub unsafe fn SetUserProvidedClock<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>>(&self, fuserclock: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), fuserclock.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`, `Win32_Foundation`*"]
    pub unsafe fn GetUserProvidedClock(&self) -> ::windows::runtime::Result<super::super::Foundation::BOOL> {
        let mut result__: <super::super::Foundation::BOOL as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
    pub unsafe fn DeliverTime(&self, cnstime: u64) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self), ::std::mem::transmute(cnstime)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`, `Win32_Foundation`*"]
    pub unsafe fn SetManualStreamSelection<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>>(&self, fselection: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::std::mem::transmute_copy(self), fselection.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`, `Win32_Foundation`*"]
    pub unsafe fn GetManualStreamSelection(&self) -> ::windows::runtime::Result<super::super::Foundation::BOOL> {
        let mut result__: <super::super::Foundation::BOOL as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
    pub unsafe fn SetStreamsSelected(&self, cstreamcount: u16, pwstreamnumbers: *const u16, pselections: *const WMT_STREAM_SELECTION) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).8)(::std::mem::transmute_copy(self), ::std::mem::transmute(cstreamcount), ::std::mem::transmute(pwstreamnumbers), ::std::mem::transmute(pselections)).ok()
    }
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
    pub unsafe fn GetStreamSelected(&self, wstreamnum: u16) -> ::windows::runtime::Result<WMT_STREAM_SELECTION> {
        let mut result__: <WMT_STREAM_SELECTION as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).9)(::std::mem::transmute_copy(self), ::std::mem::transmute(wstreamnum), &mut result__).from_abi::<WMT_STREAM_SELECTION>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`, `Win32_Foundation`*"]
    pub unsafe fn SetReceiveSelectionCallbacks<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>>(&self, fgetcallbacks: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).10)(::std::mem::transmute_copy(self), fgetcallbacks.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`, `Win32_Foundation`*"]
    pub unsafe fn GetReceiveSelectionCallbacks(&self) -> ::windows::runtime::Result<super::super::Foundation::BOOL> {
        let mut result__: <super::super::Foundation::BOOL as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).11)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`, `Win32_Foundation`*"]
    pub unsafe fn SetReceiveStreamSamples<'a, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>>(&self, wstreamnum: u16, freceivestreamsamples: Param1) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).12)(::std::mem::transmute_copy(self), ::std::mem::transmute(wstreamnum), freceivestreamsamples.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`, `Win32_Foundation`*"]
    pub unsafe fn GetReceiveStreamSamples(&self, wstreamnum: u16) -> ::windows::runtime::Result<super::super::Foundation::BOOL> {
        let mut result__: <super::super::Foundation::BOOL as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).13)(::std::mem::transmute_copy(self), ::std::mem::transmute(wstreamnum), &mut result__).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`, `Win32_Foundation`*"]
    pub unsafe fn SetAllocateForOutput<'a, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>>(&self, dwoutputnum: u32, fallocate: Param1) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).14)(::std::mem::transmute_copy(self), ::std::mem::transmute(dwoutputnum), fallocate.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`, `Win32_Foundation`*"]
    pub unsafe fn GetAllocateForOutput(&self, dwoutputnum: u32) -> ::windows::runtime::Result<super::super::Foundation::BOOL> {
        let mut result__: <super::super::Foundation::BOOL as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).15)(::std::mem::transmute_copy(self), ::std::mem::transmute(dwoutputnum), &mut result__).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`, `Win32_Foundation`*"]
    pub unsafe fn SetAllocateForStream<'a, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>>(&self, wstreamnum: u16, fallocate: Param1) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).16)(::std::mem::transmute_copy(self), ::std::mem::transmute(wstreamnum), fallocate.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`, `Win32_Foundation`*"]
    pub unsafe fn GetAllocateForStream(&self, dwsreamnum: u16) -> ::windows::runtime::Result<super::super::Foundation::BOOL> {
        let mut result__: <super::super::Foundation::BOOL as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).17)(::std::mem::transmute_copy(self), ::std::mem::transmute(dwsreamnum), &mut result__).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
    pub unsafe fn GetStatistics(&self, pstatistics: *mut WM_READER_STATISTICS) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).18)(::std::mem::transmute_copy(self), ::std::mem::transmute(pstatistics)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`, `Win32_Foundation`*"]
    pub unsafe fn SetClientInfo(&self, pclientinfo: *const WM_READER_CLIENTINFO) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).19)(::std::mem::transmute_copy(self), ::std::mem::transmute(pclientinfo)).ok()
    }
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
    pub unsafe fn GetMaxOutputSampleSize(&self, dwoutput: u32) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).20)(::std::mem::transmute_copy(self), ::std::mem::transmute(dwoutput), &mut result__).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
    pub unsafe fn GetMaxStreamSampleSize(&self, wstream: u16) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).21)(::std::mem::transmute_copy(self), ::std::mem::transmute(wstream), &mut result__).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
    pub unsafe fn NotifyLateDelivery(&self, cnslateness: u64) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).22)(::std::mem::transmute_copy(self), ::std::mem::transmute(cnslateness)).ok()
    }
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
    pub unsafe fn SetPlayMode(&self, mode: WMT_PLAY_MODE) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).23)(::std::mem::transmute_copy(self), ::std::mem::transmute(mode)).ok()
    }
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
    pub unsafe fn GetPlayMode(&self) -> ::windows::runtime::Result<WMT_PLAY_MODE> {
        let mut result__: <WMT_PLAY_MODE as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).24)(::std::mem::transmute_copy(self), &mut result__).from_abi::<WMT_PLAY_MODE>(result__)
    }
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
    pub unsafe fn GetBufferProgress(&self, pdwpercent: *mut u32, pcnsbuffering: *mut u64) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).25)(::std::mem::transmute_copy(self), ::std::mem::transmute(pdwpercent), ::std::mem::transmute(pcnsbuffering)).ok()
    }
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
    pub unsafe fn GetDownloadProgress(&self, pdwpercent: *mut u32, pqwbytesdownloaded: *mut u64, pcnsdownload: *mut u64) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).26)(::std::mem::transmute_copy(self), ::std::mem::transmute(pdwpercent), ::std::mem::transmute(pqwbytesdownloaded), ::std::mem::transmute(pcnsdownload)).ok()
    }
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
    pub unsafe fn GetSaveAsProgress(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).27)(::std::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`, `Win32_Foundation`*"]
    pub unsafe fn SaveFileAs<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, pwszfilename: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).28)(::std::mem::transmute_copy(self), pwszfilename.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`, `Win32_Foundation`*"]
    pub unsafe fn GetProtocolName(&self, pwszprotocol: super::super::Foundation::PWSTR, pcchprotocol: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).29)(::std::mem::transmute_copy(self), ::std::mem::transmute(pwszprotocol), ::std::mem::transmute(pcchprotocol)).ok()
    }
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
    pub unsafe fn StartAtMarker(&self, wmarkerindex: u16, cnsduration: u64, frate: f32, pvcontext: *const ::std::ffi::c_void) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).30)(::std::mem::transmute_copy(self), ::std::mem::transmute(wmarkerindex), ::std::mem::transmute(cnsduration), ::std::mem::transmute(frate), ::std::mem::transmute(pvcontext)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`, `Win32_Foundation`*"]
    pub unsafe fn GetOutputSetting<'a, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, dwoutputnum: u32, pszname: Param1, ptype: *mut WMT_ATTR_DATATYPE, pvalue: *mut u8, pcblength: *mut u16) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).31)(::std::mem::transmute_copy(self), ::std::mem::transmute(dwoutputnum), pszname.into_param().abi(), ::std::mem::transmute(ptype), ::std::mem::transmute(pvalue), ::std::mem::transmute(pcblength)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`, `Win32_Foundation`*"]
    pub unsafe fn SetOutputSetting<'a, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, dwoutputnum: u32, pszname: Param1, r#type: WMT_ATTR_DATATYPE, pvalue: *const u8, cblength: u16) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).32)(::std::mem::transmute_copy(self), ::std::mem::transmute(dwoutputnum), pszname.into_param().abi(), ::std::mem::transmute(r#type), ::std::mem::transmute(pvalue), ::std::mem::transmute(cblength)).ok()
    }
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
    pub unsafe fn Preroll(&self, cnsstart: u64, cnsduration: u64, frate: f32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).33)(::std::mem::transmute_copy(self), ::std::mem::transmute(cnsstart), ::std::mem::transmute(cnsduration), ::std::mem::transmute(frate)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`, `Win32_Foundation`*"]
    pub unsafe fn SetLogClientID<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>>(&self, flogclientid: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).34)(::std::mem::transmute_copy(self), flogclientid.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`, `Win32_Foundation`*"]
    pub unsafe fn GetLogClientID(&self) -> ::windows::runtime::Result<super::super::Foundation::BOOL> {
        let mut result__: <super::super::Foundation::BOOL as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).35)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
    pub unsafe fn StopBuffering(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).36)(::std::mem::transmute_copy(self)).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`, `Win32_System_Com`*"]
    pub unsafe fn OpenStream<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::System::Com::IStream>, Param1: ::windows::runtime::IntoParam<'a, IWMReaderCallback>>(&self, pstream: Param0, pcallback: Param1, pvcontext: *const ::std::ffi::c_void) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).37)(::std::mem::transmute_copy(self), pstream.into_param().abi(), pcallback.into_param().abi(), ::std::mem::transmute(pvcontext)).ok()
    }
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
    pub unsafe fn StopNetStreaming(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).38)(::std::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
    pub unsafe fn StartAtPosition(&self, wstreamnum: u16, pvoffsetstart: *const ::std::ffi::c_void, pvduration: *const ::std::ffi::c_void, dwoffsetformat: WMT_OFFSET_FORMAT, frate: f32, pvcontext: *const ::std::ffi::c_void) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).39)(::std::mem::transmute_copy(self), ::std::mem::transmute(wstreamnum), ::std::mem::transmute(pvoffsetstart), ::std::mem::transmute(pvduration), ::std::mem::transmute(dwoffsetformat), ::std::mem::transmute(frate), ::std::mem::transmute(pvcontext)).ok()
    }
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
    pub unsafe fn GetLanguageCount(&self, dwoutputnum: u32) -> ::windows::runtime::Result<u16> {
        let mut result__: <u16 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).40)(::std::mem::transmute_copy(self), ::std::mem::transmute(dwoutputnum), &mut result__).from_abi::<u16>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`, `Win32_Foundation`*"]
    pub unsafe fn GetLanguage(&self, dwoutputnum: u32, wlanguage: u16, pwszlanguagestring: super::super::Foundation::PWSTR, pcchlanguagestringlength: *mut u16) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).41)(::std::mem::transmute_copy(self), ::std::mem::transmute(dwoutputnum), ::std::mem::transmute(wlanguage), ::std::mem::transmute(pwszlanguagestring), ::std::mem::transmute(pcchlanguagestringlength)).ok()
    }
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
    pub unsafe fn GetMaxSpeedFactor(&self) -> ::windows::runtime::Result<f64> {
        let mut result__: <f64 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).42)(::std::mem::transmute_copy(self), &mut result__).from_abi::<f64>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`, `Win32_Foundation`*"]
    pub unsafe fn IsUsingFastCache(&self) -> ::windows::runtime::Result<super::super::Foundation::BOOL> {
        let mut result__: <super::super::Foundation::BOOL as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).43)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`, `Win32_Foundation`*"]
    pub unsafe fn AddLogParam<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>, Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, wsznamespace: Param0, wszname: Param1, wszvalue: Param2) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).44)(::std::mem::transmute_copy(self), wsznamespace.into_param().abi(), wszname.into_param().abi(), wszvalue.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
    pub unsafe fn SendLogParams(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).45)(::std::mem::transmute_copy(self)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`, `Win32_Foundation`*"]
    pub unsafe fn CanSaveFileAs(&self) -> ::windows::runtime::Result<super::super::Foundation::BOOL> {
        let mut result__: <super::super::Foundation::BOOL as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).46)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
    pub unsafe fn CancelSaveFileAs(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).47)(::std::mem::transmute_copy(self)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`, `Win32_Foundation`*"]
    pub unsafe fn GetURL(&self, pwszurl: super::super::Foundation::PWSTR, pcchurl: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).48)(::std::mem::transmute_copy(self), ::std::mem::transmute(pwszurl), ::std::mem::transmute(pcchurl)).ok()
    }
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
    pub unsafe fn SetPlayerHook<'a, Param1: ::windows::runtime::IntoParam<'a, IWMPlayerHook>>(&self, dwoutputnum: u32, phook: Param1) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).49)(::std::mem::transmute_copy(self), ::std::mem::transmute(dwoutputnum), phook.into_param().abi()).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IWMReaderAdvanced5 {
    type Vtable = IWMReaderAdvanced5_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(616844720, 21969, 18862, [165, 204, 241, 56, 21, 227, 99, 99]);
}
impl ::std::convert::From<IWMReaderAdvanced5> for IWMReaderAdvanced4 {
    fn from(value: IWMReaderAdvanced5) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IWMReaderAdvanced5> for IWMReaderAdvanced4 {
    fn from(value: &IWMReaderAdvanced5) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IWMReaderAdvanced4> for IWMReaderAdvanced5 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IWMReaderAdvanced4> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IWMReaderAdvanced4>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IWMReaderAdvanced4> for &IWMReaderAdvanced5 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IWMReaderAdvanced4> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IWMReaderAdvanced4>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<IWMReaderAdvanced5> for IWMReaderAdvanced3 {
    fn from(value: IWMReaderAdvanced5) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IWMReaderAdvanced5> for IWMReaderAdvanced3 {
    fn from(value: &IWMReaderAdvanced5) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IWMReaderAdvanced3> for IWMReaderAdvanced5 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IWMReaderAdvanced3> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IWMReaderAdvanced3>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IWMReaderAdvanced3> for &IWMReaderAdvanced5 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IWMReaderAdvanced3> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IWMReaderAdvanced3>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<IWMReaderAdvanced5> for IWMReaderAdvanced2 {
    fn from(value: IWMReaderAdvanced5) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IWMReaderAdvanced5> for IWMReaderAdvanced2 {
    fn from(value: &IWMReaderAdvanced5) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IWMReaderAdvanced2> for IWMReaderAdvanced5 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IWMReaderAdvanced2> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IWMReaderAdvanced2>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IWMReaderAdvanced2> for &IWMReaderAdvanced5 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IWMReaderAdvanced2> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IWMReaderAdvanced2>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<IWMReaderAdvanced5> for IWMReaderAdvanced {
    fn from(value: IWMReaderAdvanced5) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IWMReaderAdvanced5> for IWMReaderAdvanced {
    fn from(value: &IWMReaderAdvanced5) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IWMReaderAdvanced> for IWMReaderAdvanced5 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IWMReaderAdvanced> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IWMReaderAdvanced>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IWMReaderAdvanced> for &IWMReaderAdvanced5 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IWMReaderAdvanced> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IWMReaderAdvanced>::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMReaderAdvanced5_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, fuserclock: super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pfuserclock: *mut super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, cnstime: u64) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, fselection: super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pfselection: *mut super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, cstreamcount: u16, pwstreamnumbers: *const u16, pselections: *const WMT_STREAM_SELECTION) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, wstreamnum: u16, pselection: *mut WMT_STREAM_SELECTION) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, fgetcallbacks: super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pfgetcallbacks: *mut super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, wstreamnum: u16, freceivestreamsamples: super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, wstreamnum: u16, pfreceivestreamsamples: *mut super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwoutputnum: u32, fallocate: super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwoutputnum: u32, pfallocate: *mut super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, wstreamnum: u16, fallocate: super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwsreamnum: u16, pfallocate: *mut super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pstatistics: *mut WM_READER_STATISTICS) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pclientinfo: *const WM_READER_CLIENTINFO) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwoutput: u32, pcbmax: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, wstream: u16, pcbmax: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, cnslateness: u64) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, mode: WMT_PLAY_MODE) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pmode: *mut WMT_PLAY_MODE) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pdwpercent: *mut u32, pcnsbuffering: *mut u64) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pdwpercent: *mut u32, pqwbytesdownloaded: *mut u64, pcnsdownload: *mut u64) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pdwpercent: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pwszfilename: super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pwszprotocol: super::super::Foundation::PWSTR, pcchprotocol: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, wmarkerindex: u16, cnsduration: u64, frate: f32, pvcontext: *const ::std::ffi::c_void) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwoutputnum: u32, pszname: super::super::Foundation::PWSTR, ptype: *mut WMT_ATTR_DATATYPE, pvalue: *mut u8, pcblength: *mut u16) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwoutputnum: u32, pszname: super::super::Foundation::PWSTR, r#type: WMT_ATTR_DATATYPE, pvalue: *const u8, cblength: u16) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, cnsstart: u64, cnsduration: u64, frate: f32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, flogclientid: super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pflogclientid: *mut super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pstream: ::windows::runtime::RawPtr, pcallback: ::windows::runtime::RawPtr, pvcontext: *const ::std::ffi::c_void) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, wstreamnum: u16, pvoffsetstart: *const ::std::ffi::c_void, pvduration: *const ::std::ffi::c_void, dwoffsetformat: WMT_OFFSET_FORMAT, frate: f32, pvcontext: *const ::std::ffi::c_void) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwoutputnum: u32, pwlanguagecount: *mut u16) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwoutputnum: u32, wlanguage: u16, pwszlanguagestring: super::super::Foundation::PWSTR, pcchlanguagestringlength: *mut u16) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pdblfactor: *mut f64) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pfusingfastcache: *mut super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, wsznamespace: super::super::Foundation::PWSTR, wszname: super::super::Foundation::PWSTR, wszvalue: super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pfcansave: *mut super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pwszurl: super::super::Foundation::PWSTR, pcchurl: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwoutputnum: u32, phook: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct IWMReaderAdvanced6(::windows::runtime::IUnknown);
impl IWMReaderAdvanced6 {
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`, `Win32_Foundation`*"]
    pub unsafe fn SetUserProvidedClock<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>>(&self, fuserclock: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), fuserclock.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`, `Win32_Foundation`*"]
    pub unsafe fn GetUserProvidedClock(&self) -> ::windows::runtime::Result<super::super::Foundation::BOOL> {
        let mut result__: <super::super::Foundation::BOOL as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
    pub unsafe fn DeliverTime(&self, cnstime: u64) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self), ::std::mem::transmute(cnstime)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`, `Win32_Foundation`*"]
    pub unsafe fn SetManualStreamSelection<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>>(&self, fselection: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::std::mem::transmute_copy(self), fselection.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`, `Win32_Foundation`*"]
    pub unsafe fn GetManualStreamSelection(&self) -> ::windows::runtime::Result<super::super::Foundation::BOOL> {
        let mut result__: <super::super::Foundation::BOOL as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
    pub unsafe fn SetStreamsSelected(&self, cstreamcount: u16, pwstreamnumbers: *const u16, pselections: *const WMT_STREAM_SELECTION) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).8)(::std::mem::transmute_copy(self), ::std::mem::transmute(cstreamcount), ::std::mem::transmute(pwstreamnumbers), ::std::mem::transmute(pselections)).ok()
    }
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
    pub unsafe fn GetStreamSelected(&self, wstreamnum: u16) -> ::windows::runtime::Result<WMT_STREAM_SELECTION> {
        let mut result__: <WMT_STREAM_SELECTION as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).9)(::std::mem::transmute_copy(self), ::std::mem::transmute(wstreamnum), &mut result__).from_abi::<WMT_STREAM_SELECTION>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`, `Win32_Foundation`*"]
    pub unsafe fn SetReceiveSelectionCallbacks<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>>(&self, fgetcallbacks: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).10)(::std::mem::transmute_copy(self), fgetcallbacks.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`, `Win32_Foundation`*"]
    pub unsafe fn GetReceiveSelectionCallbacks(&self) -> ::windows::runtime::Result<super::super::Foundation::BOOL> {
        let mut result__: <super::super::Foundation::BOOL as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).11)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`, `Win32_Foundation`*"]
    pub unsafe fn SetReceiveStreamSamples<'a, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>>(&self, wstreamnum: u16, freceivestreamsamples: Param1) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).12)(::std::mem::transmute_copy(self), ::std::mem::transmute(wstreamnum), freceivestreamsamples.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`, `Win32_Foundation`*"]
    pub unsafe fn GetReceiveStreamSamples(&self, wstreamnum: u16) -> ::windows::runtime::Result<super::super::Foundation::BOOL> {
        let mut result__: <super::super::Foundation::BOOL as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).13)(::std::mem::transmute_copy(self), ::std::mem::transmute(wstreamnum), &mut result__).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`, `Win32_Foundation`*"]
    pub unsafe fn SetAllocateForOutput<'a, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>>(&self, dwoutputnum: u32, fallocate: Param1) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).14)(::std::mem::transmute_copy(self), ::std::mem::transmute(dwoutputnum), fallocate.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`, `Win32_Foundation`*"]
    pub unsafe fn GetAllocateForOutput(&self, dwoutputnum: u32) -> ::windows::runtime::Result<super::super::Foundation::BOOL> {
        let mut result__: <super::super::Foundation::BOOL as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).15)(::std::mem::transmute_copy(self), ::std::mem::transmute(dwoutputnum), &mut result__).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`, `Win32_Foundation`*"]
    pub unsafe fn SetAllocateForStream<'a, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>>(&self, wstreamnum: u16, fallocate: Param1) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).16)(::std::mem::transmute_copy(self), ::std::mem::transmute(wstreamnum), fallocate.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`, `Win32_Foundation`*"]
    pub unsafe fn GetAllocateForStream(&self, dwsreamnum: u16) -> ::windows::runtime::Result<super::super::Foundation::BOOL> {
        let mut result__: <super::super::Foundation::BOOL as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).17)(::std::mem::transmute_copy(self), ::std::mem::transmute(dwsreamnum), &mut result__).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
    pub unsafe fn GetStatistics(&self, pstatistics: *mut WM_READER_STATISTICS) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).18)(::std::mem::transmute_copy(self), ::std::mem::transmute(pstatistics)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`, `Win32_Foundation`*"]
    pub unsafe fn SetClientInfo(&self, pclientinfo: *const WM_READER_CLIENTINFO) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).19)(::std::mem::transmute_copy(self), ::std::mem::transmute(pclientinfo)).ok()
    }
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
    pub unsafe fn GetMaxOutputSampleSize(&self, dwoutput: u32) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).20)(::std::mem::transmute_copy(self), ::std::mem::transmute(dwoutput), &mut result__).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
    pub unsafe fn GetMaxStreamSampleSize(&self, wstream: u16) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).21)(::std::mem::transmute_copy(self), ::std::mem::transmute(wstream), &mut result__).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
    pub unsafe fn NotifyLateDelivery(&self, cnslateness: u64) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).22)(::std::mem::transmute_copy(self), ::std::mem::transmute(cnslateness)).ok()
    }
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
    pub unsafe fn SetPlayMode(&self, mode: WMT_PLAY_MODE) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).23)(::std::mem::transmute_copy(self), ::std::mem::transmute(mode)).ok()
    }
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
    pub unsafe fn GetPlayMode(&self) -> ::windows::runtime::Result<WMT_PLAY_MODE> {
        let mut result__: <WMT_PLAY_MODE as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).24)(::std::mem::transmute_copy(self), &mut result__).from_abi::<WMT_PLAY_MODE>(result__)
    }
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
    pub unsafe fn GetBufferProgress(&self, pdwpercent: *mut u32, pcnsbuffering: *mut u64) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).25)(::std::mem::transmute_copy(self), ::std::mem::transmute(pdwpercent), ::std::mem::transmute(pcnsbuffering)).ok()
    }
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
    pub unsafe fn GetDownloadProgress(&self, pdwpercent: *mut u32, pqwbytesdownloaded: *mut u64, pcnsdownload: *mut u64) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).26)(::std::mem::transmute_copy(self), ::std::mem::transmute(pdwpercent), ::std::mem::transmute(pqwbytesdownloaded), ::std::mem::transmute(pcnsdownload)).ok()
    }
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
    pub unsafe fn GetSaveAsProgress(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).27)(::std::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`, `Win32_Foundation`*"]
    pub unsafe fn SaveFileAs<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, pwszfilename: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).28)(::std::mem::transmute_copy(self), pwszfilename.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`, `Win32_Foundation`*"]
    pub unsafe fn GetProtocolName(&self, pwszprotocol: super::super::Foundation::PWSTR, pcchprotocol: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).29)(::std::mem::transmute_copy(self), ::std::mem::transmute(pwszprotocol), ::std::mem::transmute(pcchprotocol)).ok()
    }
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
    pub unsafe fn StartAtMarker(&self, wmarkerindex: u16, cnsduration: u64, frate: f32, pvcontext: *const ::std::ffi::c_void) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).30)(::std::mem::transmute_copy(self), ::std::mem::transmute(wmarkerindex), ::std::mem::transmute(cnsduration), ::std::mem::transmute(frate), ::std::mem::transmute(pvcontext)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`, `Win32_Foundation`*"]
    pub unsafe fn GetOutputSetting<'a, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, dwoutputnum: u32, pszname: Param1, ptype: *mut WMT_ATTR_DATATYPE, pvalue: *mut u8, pcblength: *mut u16) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).31)(::std::mem::transmute_copy(self), ::std::mem::transmute(dwoutputnum), pszname.into_param().abi(), ::std::mem::transmute(ptype), ::std::mem::transmute(pvalue), ::std::mem::transmute(pcblength)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`, `Win32_Foundation`*"]
    pub unsafe fn SetOutputSetting<'a, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, dwoutputnum: u32, pszname: Param1, r#type: WMT_ATTR_DATATYPE, pvalue: *const u8, cblength: u16) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).32)(::std::mem::transmute_copy(self), ::std::mem::transmute(dwoutputnum), pszname.into_param().abi(), ::std::mem::transmute(r#type), ::std::mem::transmute(pvalue), ::std::mem::transmute(cblength)).ok()
    }
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
    pub unsafe fn Preroll(&self, cnsstart: u64, cnsduration: u64, frate: f32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).33)(::std::mem::transmute_copy(self), ::std::mem::transmute(cnsstart), ::std::mem::transmute(cnsduration), ::std::mem::transmute(frate)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`, `Win32_Foundation`*"]
    pub unsafe fn SetLogClientID<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>>(&self, flogclientid: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).34)(::std::mem::transmute_copy(self), flogclientid.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`, `Win32_Foundation`*"]
    pub unsafe fn GetLogClientID(&self) -> ::windows::runtime::Result<super::super::Foundation::BOOL> {
        let mut result__: <super::super::Foundation::BOOL as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).35)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
    pub unsafe fn StopBuffering(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).36)(::std::mem::transmute_copy(self)).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`, `Win32_System_Com`*"]
    pub unsafe fn OpenStream<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::System::Com::IStream>, Param1: ::windows::runtime::IntoParam<'a, IWMReaderCallback>>(&self, pstream: Param0, pcallback: Param1, pvcontext: *const ::std::ffi::c_void) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).37)(::std::mem::transmute_copy(self), pstream.into_param().abi(), pcallback.into_param().abi(), ::std::mem::transmute(pvcontext)).ok()
    }
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
    pub unsafe fn StopNetStreaming(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).38)(::std::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
    pub unsafe fn StartAtPosition(&self, wstreamnum: u16, pvoffsetstart: *const ::std::ffi::c_void, pvduration: *const ::std::ffi::c_void, dwoffsetformat: WMT_OFFSET_FORMAT, frate: f32, pvcontext: *const ::std::ffi::c_void) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).39)(::std::mem::transmute_copy(self), ::std::mem::transmute(wstreamnum), ::std::mem::transmute(pvoffsetstart), ::std::mem::transmute(pvduration), ::std::mem::transmute(dwoffsetformat), ::std::mem::transmute(frate), ::std::mem::transmute(pvcontext)).ok()
    }
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
    pub unsafe fn GetLanguageCount(&self, dwoutputnum: u32) -> ::windows::runtime::Result<u16> {
        let mut result__: <u16 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).40)(::std::mem::transmute_copy(self), ::std::mem::transmute(dwoutputnum), &mut result__).from_abi::<u16>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`, `Win32_Foundation`*"]
    pub unsafe fn GetLanguage(&self, dwoutputnum: u32, wlanguage: u16, pwszlanguagestring: super::super::Foundation::PWSTR, pcchlanguagestringlength: *mut u16) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).41)(::std::mem::transmute_copy(self), ::std::mem::transmute(dwoutputnum), ::std::mem::transmute(wlanguage), ::std::mem::transmute(pwszlanguagestring), ::std::mem::transmute(pcchlanguagestringlength)).ok()
    }
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
    pub unsafe fn GetMaxSpeedFactor(&self) -> ::windows::runtime::Result<f64> {
        let mut result__: <f64 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).42)(::std::mem::transmute_copy(self), &mut result__).from_abi::<f64>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`, `Win32_Foundation`*"]
    pub unsafe fn IsUsingFastCache(&self) -> ::windows::runtime::Result<super::super::Foundation::BOOL> {
        let mut result__: <super::super::Foundation::BOOL as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).43)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`, `Win32_Foundation`*"]
    pub unsafe fn AddLogParam<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>, Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, wsznamespace: Param0, wszname: Param1, wszvalue: Param2) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).44)(::std::mem::transmute_copy(self), wsznamespace.into_param().abi(), wszname.into_param().abi(), wszvalue.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
    pub unsafe fn SendLogParams(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).45)(::std::mem::transmute_copy(self)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`, `Win32_Foundation`*"]
    pub unsafe fn CanSaveFileAs(&self) -> ::windows::runtime::Result<super::super::Foundation::BOOL> {
        let mut result__: <super::super::Foundation::BOOL as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).46)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
    pub unsafe fn CancelSaveFileAs(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).47)(::std::mem::transmute_copy(self)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`, `Win32_Foundation`*"]
    pub unsafe fn GetURL(&self, pwszurl: super::super::Foundation::PWSTR, pcchurl: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).48)(::std::mem::transmute_copy(self), ::std::mem::transmute(pwszurl), ::std::mem::transmute(pcchurl)).ok()
    }
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
    pub unsafe fn SetPlayerHook<'a, Param1: ::windows::runtime::IntoParam<'a, IWMPlayerHook>>(&self, dwoutputnum: u32, phook: Param1) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).49)(::std::mem::transmute_copy(self), ::std::mem::transmute(dwoutputnum), phook.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
    pub unsafe fn SetProtectStreamSamples(&self, pbcertificate: *const u8, cbcertificate: u32, dwcertificatetype: u32, dwflags: u32, pbinitializationvector: *mut u8, pcbinitializationvector: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).50)(::std::mem::transmute_copy(self), ::std::mem::transmute(pbcertificate), ::std::mem::transmute(cbcertificate), ::std::mem::transmute(dwcertificatetype), ::std::mem::transmute(dwflags), ::std::mem::transmute(pbinitializationvector), ::std::mem::transmute(pcbinitializationvector)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IWMReaderAdvanced6 {
    type Vtable = IWMReaderAdvanced6_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(413329400, 17039, 19149, [138, 0, 230, 70, 57, 188, 147, 222]);
}
impl ::std::convert::From<IWMReaderAdvanced6> for IWMReaderAdvanced5 {
    fn from(value: IWMReaderAdvanced6) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IWMReaderAdvanced6> for IWMReaderAdvanced5 {
    fn from(value: &IWMReaderAdvanced6) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IWMReaderAdvanced5> for IWMReaderAdvanced6 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IWMReaderAdvanced5> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IWMReaderAdvanced5>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IWMReaderAdvanced5> for &IWMReaderAdvanced6 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IWMReaderAdvanced5> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IWMReaderAdvanced5>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<IWMReaderAdvanced6> for IWMReaderAdvanced4 {
    fn from(value: IWMReaderAdvanced6) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IWMReaderAdvanced6> for IWMReaderAdvanced4 {
    fn from(value: &IWMReaderAdvanced6) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IWMReaderAdvanced4> for IWMReaderAdvanced6 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IWMReaderAdvanced4> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IWMReaderAdvanced4>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IWMReaderAdvanced4> for &IWMReaderAdvanced6 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IWMReaderAdvanced4> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IWMReaderAdvanced4>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<IWMReaderAdvanced6> for IWMReaderAdvanced3 {
    fn from(value: IWMReaderAdvanced6) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IWMReaderAdvanced6> for IWMReaderAdvanced3 {
    fn from(value: &IWMReaderAdvanced6) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IWMReaderAdvanced3> for IWMReaderAdvanced6 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IWMReaderAdvanced3> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IWMReaderAdvanced3>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IWMReaderAdvanced3> for &IWMReaderAdvanced6 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IWMReaderAdvanced3> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IWMReaderAdvanced3>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<IWMReaderAdvanced6> for IWMReaderAdvanced2 {
    fn from(value: IWMReaderAdvanced6) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IWMReaderAdvanced6> for IWMReaderAdvanced2 {
    fn from(value: &IWMReaderAdvanced6) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IWMReaderAdvanced2> for IWMReaderAdvanced6 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IWMReaderAdvanced2> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IWMReaderAdvanced2>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IWMReaderAdvanced2> for &IWMReaderAdvanced6 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IWMReaderAdvanced2> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IWMReaderAdvanced2>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<IWMReaderAdvanced6> for IWMReaderAdvanced {
    fn from(value: IWMReaderAdvanced6) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IWMReaderAdvanced6> for IWMReaderAdvanced {
    fn from(value: &IWMReaderAdvanced6) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IWMReaderAdvanced> for IWMReaderAdvanced6 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IWMReaderAdvanced> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IWMReaderAdvanced>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IWMReaderAdvanced> for &IWMReaderAdvanced6 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IWMReaderAdvanced> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IWMReaderAdvanced>::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMReaderAdvanced6_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, fuserclock: super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pfuserclock: *mut super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, cnstime: u64) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, fselection: super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pfselection: *mut super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, cstreamcount: u16, pwstreamnumbers: *const u16, pselections: *const WMT_STREAM_SELECTION) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, wstreamnum: u16, pselection: *mut WMT_STREAM_SELECTION) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, fgetcallbacks: super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pfgetcallbacks: *mut super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, wstreamnum: u16, freceivestreamsamples: super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, wstreamnum: u16, pfreceivestreamsamples: *mut super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwoutputnum: u32, fallocate: super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwoutputnum: u32, pfallocate: *mut super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, wstreamnum: u16, fallocate: super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwsreamnum: u16, pfallocate: *mut super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pstatistics: *mut WM_READER_STATISTICS) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pclientinfo: *const WM_READER_CLIENTINFO) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwoutput: u32, pcbmax: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, wstream: u16, pcbmax: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, cnslateness: u64) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, mode: WMT_PLAY_MODE) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pmode: *mut WMT_PLAY_MODE) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pdwpercent: *mut u32, pcnsbuffering: *mut u64) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pdwpercent: *mut u32, pqwbytesdownloaded: *mut u64, pcnsdownload: *mut u64) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pdwpercent: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pwszfilename: super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pwszprotocol: super::super::Foundation::PWSTR, pcchprotocol: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, wmarkerindex: u16, cnsduration: u64, frate: f32, pvcontext: *const ::std::ffi::c_void) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwoutputnum: u32, pszname: super::super::Foundation::PWSTR, ptype: *mut WMT_ATTR_DATATYPE, pvalue: *mut u8, pcblength: *mut u16) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwoutputnum: u32, pszname: super::super::Foundation::PWSTR, r#type: WMT_ATTR_DATATYPE, pvalue: *const u8, cblength: u16) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, cnsstart: u64, cnsduration: u64, frate: f32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, flogclientid: super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pflogclientid: *mut super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pstream: ::windows::runtime::RawPtr, pcallback: ::windows::runtime::RawPtr, pvcontext: *const ::std::ffi::c_void) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, wstreamnum: u16, pvoffsetstart: *const ::std::ffi::c_void, pvduration: *const ::std::ffi::c_void, dwoffsetformat: WMT_OFFSET_FORMAT, frate: f32, pvcontext: *const ::std::ffi::c_void) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwoutputnum: u32, pwlanguagecount: *mut u16) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwoutputnum: u32, wlanguage: u16, pwszlanguagestring: super::super::Foundation::PWSTR, pcchlanguagestringlength: *mut u16) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pdblfactor: *mut f64) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pfusingfastcache: *mut super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, wsznamespace: super::super::Foundation::PWSTR, wszname: super::super::Foundation::PWSTR, wszvalue: super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pfcansave: *mut super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pwszurl: super::super::Foundation::PWSTR, pcchurl: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwoutputnum: u32, phook: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbcertificate: *const u8, cbcertificate: u32, dwcertificatetype: u32, dwflags: u32, pbinitializationvector: *mut u8, pcbinitializationvector: *mut u32) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct IWMReaderAllocatorEx(::windows::runtime::IUnknown);
impl IWMReaderAllocatorEx {
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
    pub unsafe fn AllocateForStreamEx(&self, wstreamnum: u16, cbbuffer: u32, ppbuffer: *mut ::std::option::Option<INSSBuffer>, dwflags: u32, cnssampletime: u64, cnssampleduration: u64, pvcontext: *const ::std::ffi::c_void) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), ::std::mem::transmute(wstreamnum), ::std::mem::transmute(cbbuffer), ::std::mem::transmute(ppbuffer), ::std::mem::transmute(dwflags), ::std::mem::transmute(cnssampletime), ::std::mem::transmute(cnssampleduration), ::std::mem::transmute(pvcontext)).ok()
    }
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
    pub unsafe fn AllocateForOutputEx(&self, dwoutputnum: u32, cbbuffer: u32, ppbuffer: *mut ::std::option::Option<INSSBuffer>, dwflags: u32, cnssampletime: u64, cnssampleduration: u64, pvcontext: *const ::std::ffi::c_void) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), ::std::mem::transmute(dwoutputnum), ::std::mem::transmute(cbbuffer), ::std::mem::transmute(ppbuffer), ::std::mem::transmute(dwflags), ::std::mem::transmute(cnssampletime), ::std::mem::transmute(cnssampleduration), ::std::mem::transmute(pvcontext)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IWMReaderAllocatorEx {
    type Vtable = IWMReaderAllocatorEx_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2675322791, 41518, 17037, [147, 201, 172, 130, 243, 170, 254, 90]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMReaderAllocatorEx_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, wstreamnum: u16, cbbuffer: u32, ppbuffer: *mut ::windows::runtime::RawPtr, dwflags: u32, cnssampletime: u64, cnssampleduration: u64, pvcontext: *const ::std::ffi::c_void) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwoutputnum: u32, cbbuffer: u32, ppbuffer: *mut ::windows::runtime::RawPtr, dwflags: u32, cnssampletime: u64, cnssampleduration: u64, pvcontext: *const ::std::ffi::c_void) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct IWMReaderCallback(::windows::runtime::IUnknown);
impl IWMReaderCallback {
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
    pub unsafe fn OnStatus(&self, status: WMT_STATUS, hr: ::windows::runtime::HRESULT, dwtype: WMT_ATTR_DATATYPE, pvalue: *const u8, pvcontext: *const ::std::ffi::c_void) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), ::std::mem::transmute(status), ::std::mem::transmute(hr), ::std::mem::transmute(dwtype), ::std::mem::transmute(pvalue), ::std::mem::transmute(pvcontext)).ok()
    }
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
    pub unsafe fn OnSample<'a, Param4: ::windows::runtime::IntoParam<'a, INSSBuffer>>(&self, dwoutputnum: u32, cnssampletime: u64, cnssampleduration: u64, dwflags: u32, psample: Param4, pvcontext: *const ::std::ffi::c_void) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), ::std::mem::transmute(dwoutputnum), ::std::mem::transmute(cnssampletime), ::std::mem::transmute(cnssampleduration), ::std::mem::transmute(dwflags), psample.into_param().abi(), ::std::mem::transmute(pvcontext)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IWMReaderCallback {
    type Vtable = IWMReaderCallback_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2520804312, 11051, 4563, [179, 107, 0, 192, 79, 97, 8, 255]);
}
impl ::std::convert::From<IWMReaderCallback> for IWMStatusCallback {
    fn from(value: IWMReaderCallback) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IWMReaderCallback> for IWMStatusCallback {
    fn from(value: &IWMReaderCallback) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IWMStatusCallback> for IWMReaderCallback {
    fn into_param(self) -> ::windows::runtime::Param<'a, IWMStatusCallback> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IWMStatusCallback>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IWMStatusCallback> for &IWMReaderCallback {
    fn into_param(self) -> ::windows::runtime::Param<'a, IWMStatusCallback> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IWMStatusCallback>::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMReaderCallback_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, status: WMT_STATUS, hr: ::windows::runtime::HRESULT, dwtype: WMT_ATTR_DATATYPE, pvalue: *const u8, pvcontext: *const ::std::ffi::c_void) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwoutputnum: u32, cnssampletime: u64, cnssampleduration: u64, dwflags: u32, psample: ::windows::runtime::RawPtr, pvcontext: *const ::std::ffi::c_void) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct IWMReaderCallbackAdvanced(::windows::runtime::IUnknown);
impl IWMReaderCallbackAdvanced {
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
    pub unsafe fn OnStreamSample<'a, Param4: ::windows::runtime::IntoParam<'a, INSSBuffer>>(&self, wstreamnum: u16, cnssampletime: u64, cnssampleduration: u64, dwflags: u32, psample: Param4, pvcontext: *const ::std::ffi::c_void) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), ::std::mem::transmute(wstreamnum), ::std::mem::transmute(cnssampletime), ::std::mem::transmute(cnssampleduration), ::std::mem::transmute(dwflags), psample.into_param().abi(), ::std::mem::transmute(pvcontext)).ok()
    }
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
    pub unsafe fn OnTime(&self, cnscurrenttime: u64, pvcontext: *const ::std::ffi::c_void) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), ::std::mem::transmute(cnscurrenttime), ::std::mem::transmute(pvcontext)).ok()
    }
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
    pub unsafe fn OnStreamSelection(&self, wstreamcount: u16, pstreamnumbers: *const u16, pselections: *const WMT_STREAM_SELECTION, pvcontext: *const ::std::ffi::c_void) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self), ::std::mem::transmute(wstreamcount), ::std::mem::transmute(pstreamnumbers), ::std::mem::transmute(pselections), ::std::mem::transmute(pvcontext)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`, `Win32_Foundation`*"]
    pub unsafe fn OnOutputPropsChanged(&self, dwoutputnum: u32, pmediatype: *const WM_MEDIA_TYPE, pvcontext: *const ::std::ffi::c_void) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::std::mem::transmute_copy(self), ::std::mem::transmute(dwoutputnum), ::std::mem::transmute(pmediatype), ::std::mem::transmute(pvcontext)).ok()
    }
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
    pub unsafe fn AllocateForStream(&self, wstreamnum: u16, cbbuffer: u32, ppbuffer: *mut ::std::option::Option<INSSBuffer>, pvcontext: *const ::std::ffi::c_void) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self), ::std::mem::transmute(wstreamnum), ::std::mem::transmute(cbbuffer), ::std::mem::transmute(ppbuffer), ::std::mem::transmute(pvcontext)).ok()
    }
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
    pub unsafe fn AllocateForOutput(&self, dwoutputnum: u32, cbbuffer: u32, ppbuffer: *mut ::std::option::Option<INSSBuffer>, pvcontext: *const ::std::ffi::c_void) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).8)(::std::mem::transmute_copy(self), ::std::mem::transmute(dwoutputnum), ::std::mem::transmute(cbbuffer), ::std::mem::transmute(ppbuffer), ::std::mem::transmute(pvcontext)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IWMReaderCallbackAdvanced {
    type Vtable = IWMReaderCallbackAdvanced_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2520804331, 11051, 4563, [179, 107, 0, 192, 79, 97, 8, 255]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMReaderCallbackAdvanced_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, wstreamnum: u16, cnssampletime: u64, cnssampleduration: u64, dwflags: u32, psample: ::windows::runtime::RawPtr, pvcontext: *const ::std::ffi::c_void) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, cnscurrenttime: u64, pvcontext: *const ::std::ffi::c_void) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, wstreamcount: u16, pstreamnumbers: *const u16, pselections: *const WMT_STREAM_SELECTION, pvcontext: *const ::std::ffi::c_void) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwoutputnum: u32, pmediatype: *const ::std::mem::ManuallyDrop<WM_MEDIA_TYPE>, pvcontext: *const ::std::ffi::c_void) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, wstreamnum: u16, cbbuffer: u32, ppbuffer: *mut ::windows::runtime::RawPtr, pvcontext: *const ::std::ffi::c_void) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwoutputnum: u32, cbbuffer: u32, ppbuffer: *mut ::windows::runtime::RawPtr, pvcontext: *const ::std::ffi::c_void) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct IWMReaderNetworkConfig(::windows::runtime::IUnknown);
impl IWMReaderNetworkConfig {
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
    pub unsafe fn GetBufferingTime(&self) -> ::windows::runtime::Result<u64> {
        let mut result__: <u64 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), &mut result__).from_abi::<u64>(result__)
    }
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
    pub unsafe fn SetBufferingTime(&self, cnsbufferingtime: u64) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), ::std::mem::transmute(cnsbufferingtime)).ok()
    }
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
    pub unsafe fn GetUDPPortRanges(&self, prangearray: *mut WM_PORT_NUMBER_RANGE, pcranges: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self), ::std::mem::transmute(prangearray), ::std::mem::transmute(pcranges)).ok()
    }
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
    pub unsafe fn SetUDPPortRanges(&self, prangearray: *const WM_PORT_NUMBER_RANGE, cranges: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::std::mem::transmute_copy(self), ::std::mem::transmute(prangearray), ::std::mem::transmute(cranges)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`, `Win32_Foundation`*"]
    pub unsafe fn GetProxySettings<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, pwszprotocol: Param0) -> ::windows::runtime::Result<WMT_PROXY_SETTINGS> {
        let mut result__: <WMT_PROXY_SETTINGS as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self), pwszprotocol.into_param().abi(), &mut result__).from_abi::<WMT_PROXY_SETTINGS>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`, `Win32_Foundation`*"]
    pub unsafe fn SetProxySettings<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, pwszprotocol: Param0, proxysetting: WMT_PROXY_SETTINGS) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).8)(::std::mem::transmute_copy(self), pwszprotocol.into_param().abi(), ::std::mem::transmute(proxysetting)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`, `Win32_Foundation`*"]
    pub unsafe fn GetProxyHostName<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, pwszprotocol: Param0, pwszhostname: super::super::Foundation::PWSTR, pcchhostname: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).9)(::std::mem::transmute_copy(self), pwszprotocol.into_param().abi(), ::std::mem::transmute(pwszhostname), ::std::mem::transmute(pcchhostname)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`, `Win32_Foundation`*"]
    pub unsafe fn SetProxyHostName<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, pwszprotocol: Param0, pwszhostname: Param1) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).10)(::std::mem::transmute_copy(self), pwszprotocol.into_param().abi(), pwszhostname.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`, `Win32_Foundation`*"]
    pub unsafe fn GetProxyPort<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, pwszprotocol: Param0) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).11)(::std::mem::transmute_copy(self), pwszprotocol.into_param().abi(), &mut result__).from_abi::<u32>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`, `Win32_Foundation`*"]
    pub unsafe fn SetProxyPort<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, pwszprotocol: Param0, dwport: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).12)(::std::mem::transmute_copy(self), pwszprotocol.into_param().abi(), ::std::mem::transmute(dwport)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`, `Win32_Foundation`*"]
    pub unsafe fn GetProxyExceptionList<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, pwszprotocol: Param0, pwszexceptionlist: super::super::Foundation::PWSTR, pcchexceptionlist: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).13)(::std::mem::transmute_copy(self), pwszprotocol.into_param().abi(), ::std::mem::transmute(pwszexceptionlist), ::std::mem::transmute(pcchexceptionlist)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`, `Win32_Foundation`*"]
    pub unsafe fn SetProxyExceptionList<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, pwszprotocol: Param0, pwszexceptionlist: Param1) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).14)(::std::mem::transmute_copy(self), pwszprotocol.into_param().abi(), pwszexceptionlist.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`, `Win32_Foundation`*"]
    pub unsafe fn GetProxyBypassForLocal<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, pwszprotocol: Param0) -> ::windows::runtime::Result<super::super::Foundation::BOOL> {
        let mut result__: <super::super::Foundation::BOOL as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).15)(::std::mem::transmute_copy(self), pwszprotocol.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`, `Win32_Foundation`*"]
    pub unsafe fn SetProxyBypassForLocal<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>>(&self, pwszprotocol: Param0, fbypassforlocal: Param1) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).16)(::std::mem::transmute_copy(self), pwszprotocol.into_param().abi(), fbypassforlocal.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`, `Win32_Foundation`*"]
    pub unsafe fn GetForceRerunAutoProxyDetection(&self) -> ::windows::runtime::Result<super::super::Foundation::BOOL> {
        let mut result__: <super::super::Foundation::BOOL as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).17)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`, `Win32_Foundation`*"]
    pub unsafe fn SetForceRerunAutoProxyDetection<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>>(&self, fforcererundetection: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).18)(::std::mem::transmute_copy(self), fforcererundetection.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`, `Win32_Foundation`*"]
    pub unsafe fn GetEnableMulticast(&self) -> ::windows::runtime::Result<super::super::Foundation::BOOL> {
        let mut result__: <super::super::Foundation::BOOL as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).19)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`, `Win32_Foundation`*"]
    pub unsafe fn SetEnableMulticast<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>>(&self, fenablemulticast: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).20)(::std::mem::transmute_copy(self), fenablemulticast.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`, `Win32_Foundation`*"]
    pub unsafe fn GetEnableHTTP(&self) -> ::windows::runtime::Result<super::super::Foundation::BOOL> {
        let mut result__: <super::super::Foundation::BOOL as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).21)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`, `Win32_Foundation`*"]
    pub unsafe fn SetEnableHTTP<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>>(&self, fenablehttp: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).22)(::std::mem::transmute_copy(self), fenablehttp.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`, `Win32_Foundation`*"]
    pub unsafe fn GetEnableUDP(&self) -> ::windows::runtime::Result<super::super::Foundation::BOOL> {
        let mut result__: <super::super::Foundation::BOOL as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).23)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`, `Win32_Foundation`*"]
    pub unsafe fn SetEnableUDP<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>>(&self, fenableudp: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).24)(::std::mem::transmute_copy(self), fenableudp.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`, `Win32_Foundation`*"]
    pub unsafe fn GetEnableTCP(&self) -> ::windows::runtime::Result<super::super::Foundation::BOOL> {
        let mut result__: <super::super::Foundation::BOOL as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).25)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`, `Win32_Foundation`*"]
    pub unsafe fn SetEnableTCP<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>>(&self, fenabletcp: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).26)(::std::mem::transmute_copy(self), fenabletcp.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
    pub unsafe fn ResetProtocolRollover(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).27)(::std::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
    pub unsafe fn GetConnectionBandwidth(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).28)(::std::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
    pub unsafe fn SetConnectionBandwidth(&self, dwconnectionbandwidth: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).29)(::std::mem::transmute_copy(self), ::std::mem::transmute(dwconnectionbandwidth)).ok()
    }
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
    pub unsafe fn GetNumProtocolsSupported(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).30)(::std::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`, `Win32_Foundation`*"]
    pub unsafe fn GetSupportedProtocolName(&self, dwprotocolnum: u32, pwszprotocolname: super::super::Foundation::PWSTR, pcchprotocolname: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).31)(::std::mem::transmute_copy(self), ::std::mem::transmute(dwprotocolnum), ::std::mem::transmute(pwszprotocolname), ::std::mem::transmute(pcchprotocolname)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`, `Win32_Foundation`*"]
    pub unsafe fn AddLoggingUrl<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, pwszurl: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).32)(::std::mem::transmute_copy(self), pwszurl.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`, `Win32_Foundation`*"]
    pub unsafe fn GetLoggingUrl(&self, dwindex: u32, pwszurl: super::super::Foundation::PWSTR, pcchurl: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).33)(::std::mem::transmute_copy(self), ::std::mem::transmute(dwindex), ::std::mem::transmute(pwszurl), ::std::mem::transmute(pcchurl)).ok()
    }
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
    pub unsafe fn GetLoggingUrlCount(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).34)(::std::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
    pub unsafe fn ResetLoggingUrlList(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).35)(::std::mem::transmute_copy(self)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IWMReaderNetworkConfig {
    type Vtable = IWMReaderNetworkConfig_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2520804332, 11051, 4563, [179, 107, 0, 192, 79, 97, 8, 255]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMReaderNetworkConfig_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pcnsbufferingtime: *mut u64) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, cnsbufferingtime: u64) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, prangearray: *mut WM_PORT_NUMBER_RANGE, pcranges: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, prangearray: *const WM_PORT_NUMBER_RANGE, cranges: u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pwszprotocol: super::super::Foundation::PWSTR, pproxysetting: *mut WMT_PROXY_SETTINGS) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pwszprotocol: super::super::Foundation::PWSTR, proxysetting: WMT_PROXY_SETTINGS) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pwszprotocol: super::super::Foundation::PWSTR, pwszhostname: super::super::Foundation::PWSTR, pcchhostname: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pwszprotocol: super::super::Foundation::PWSTR, pwszhostname: super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pwszprotocol: super::super::Foundation::PWSTR, pdwport: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pwszprotocol: super::super::Foundation::PWSTR, dwport: u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pwszprotocol: super::super::Foundation::PWSTR, pwszexceptionlist: super::super::Foundation::PWSTR, pcchexceptionlist: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pwszprotocol: super::super::Foundation::PWSTR, pwszexceptionlist: super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pwszprotocol: super::super::Foundation::PWSTR, pfbypassforlocal: *mut super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pwszprotocol: super::super::Foundation::PWSTR, fbypassforlocal: super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pfforcererundetection: *mut super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, fforcererundetection: super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pfenablemulticast: *mut super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, fenablemulticast: super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pfenablehttp: *mut super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, fenablehttp: super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pfenableudp: *mut super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, fenableudp: super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pfenabletcp: *mut super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, fenabletcp: super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pdwconnectionbandwidth: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwconnectionbandwidth: u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pcprotocols: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwprotocolnum: u32, pwszprotocolname: super::super::Foundation::PWSTR, pcchprotocolname: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pwszurl: super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwindex: u32, pwszurl: super::super::Foundation::PWSTR, pcchurl: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pdwurlcount: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct IWMReaderNetworkConfig2(::windows::runtime::IUnknown);
impl IWMReaderNetworkConfig2 {
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
    pub unsafe fn GetBufferingTime(&self) -> ::windows::runtime::Result<u64> {
        let mut result__: <u64 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), &mut result__).from_abi::<u64>(result__)
    }
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
    pub unsafe fn SetBufferingTime(&self, cnsbufferingtime: u64) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), ::std::mem::transmute(cnsbufferingtime)).ok()
    }
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
    pub unsafe fn GetUDPPortRanges(&self, prangearray: *mut WM_PORT_NUMBER_RANGE, pcranges: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self), ::std::mem::transmute(prangearray), ::std::mem::transmute(pcranges)).ok()
    }
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
    pub unsafe fn SetUDPPortRanges(&self, prangearray: *const WM_PORT_NUMBER_RANGE, cranges: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::std::mem::transmute_copy(self), ::std::mem::transmute(prangearray), ::std::mem::transmute(cranges)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`, `Win32_Foundation`*"]
    pub unsafe fn GetProxySettings<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, pwszprotocol: Param0) -> ::windows::runtime::Result<WMT_PROXY_SETTINGS> {
        let mut result__: <WMT_PROXY_SETTINGS as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self), pwszprotocol.into_param().abi(), &mut result__).from_abi::<WMT_PROXY_SETTINGS>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`, `Win32_Foundation`*"]
    pub unsafe fn SetProxySettings<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, pwszprotocol: Param0, proxysetting: WMT_PROXY_SETTINGS) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).8)(::std::mem::transmute_copy(self), pwszprotocol.into_param().abi(), ::std::mem::transmute(proxysetting)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`, `Win32_Foundation`*"]
    pub unsafe fn GetProxyHostName<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, pwszprotocol: Param0, pwszhostname: super::super::Foundation::PWSTR, pcchhostname: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).9)(::std::mem::transmute_copy(self), pwszprotocol.into_param().abi(), ::std::mem::transmute(pwszhostname), ::std::mem::transmute(pcchhostname)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`, `Win32_Foundation`*"]
    pub unsafe fn SetProxyHostName<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, pwszprotocol: Param0, pwszhostname: Param1) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).10)(::std::mem::transmute_copy(self), pwszprotocol.into_param().abi(), pwszhostname.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`, `Win32_Foundation`*"]
    pub unsafe fn GetProxyPort<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, pwszprotocol: Param0) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).11)(::std::mem::transmute_copy(self), pwszprotocol.into_param().abi(), &mut result__).from_abi::<u32>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`, `Win32_Foundation`*"]
    pub unsafe fn SetProxyPort<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, pwszprotocol: Param0, dwport: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).12)(::std::mem::transmute_copy(self), pwszprotocol.into_param().abi(), ::std::mem::transmute(dwport)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`, `Win32_Foundation`*"]
    pub unsafe fn GetProxyExceptionList<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, pwszprotocol: Param0, pwszexceptionlist: super::super::Foundation::PWSTR, pcchexceptionlist: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).13)(::std::mem::transmute_copy(self), pwszprotocol.into_param().abi(), ::std::mem::transmute(pwszexceptionlist), ::std::mem::transmute(pcchexceptionlist)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`, `Win32_Foundation`*"]
    pub unsafe fn SetProxyExceptionList<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, pwszprotocol: Param0, pwszexceptionlist: Param1) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).14)(::std::mem::transmute_copy(self), pwszprotocol.into_param().abi(), pwszexceptionlist.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`, `Win32_Foundation`*"]
    pub unsafe fn GetProxyBypassForLocal<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, pwszprotocol: Param0) -> ::windows::runtime::Result<super::super::Foundation::BOOL> {
        let mut result__: <super::super::Foundation::BOOL as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).15)(::std::mem::transmute_copy(self), pwszprotocol.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`, `Win32_Foundation`*"]
    pub unsafe fn SetProxyBypassForLocal<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>>(&self, pwszprotocol: Param0, fbypassforlocal: Param1) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).16)(::std::mem::transmute_copy(self), pwszprotocol.into_param().abi(), fbypassforlocal.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`, `Win32_Foundation`*"]
    pub unsafe fn GetForceRerunAutoProxyDetection(&self) -> ::windows::runtime::Result<super::super::Foundation::BOOL> {
        let mut result__: <super::super::Foundation::BOOL as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).17)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`, `Win32_Foundation`*"]
    pub unsafe fn SetForceRerunAutoProxyDetection<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>>(&self, fforcererundetection: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).18)(::std::mem::transmute_copy(self), fforcererundetection.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`, `Win32_Foundation`*"]
    pub unsafe fn GetEnableMulticast(&self) -> ::windows::runtime::Result<super::super::Foundation::BOOL> {
        let mut result__: <super::super::Foundation::BOOL as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).19)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`, `Win32_Foundation`*"]
    pub unsafe fn SetEnableMulticast<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>>(&self, fenablemulticast: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).20)(::std::mem::transmute_copy(self), fenablemulticast.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`, `Win32_Foundation`*"]
    pub unsafe fn GetEnableHTTP(&self) -> ::windows::runtime::Result<super::super::Foundation::BOOL> {
        let mut result__: <super::super::Foundation::BOOL as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).21)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`, `Win32_Foundation`*"]
    pub unsafe fn SetEnableHTTP<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>>(&self, fenablehttp: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).22)(::std::mem::transmute_copy(self), fenablehttp.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`, `Win32_Foundation`*"]
    pub unsafe fn GetEnableUDP(&self) -> ::windows::runtime::Result<super::super::Foundation::BOOL> {
        let mut result__: <super::super::Foundation::BOOL as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).23)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`, `Win32_Foundation`*"]
    pub unsafe fn SetEnableUDP<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>>(&self, fenableudp: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).24)(::std::mem::transmute_copy(self), fenableudp.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`, `Win32_Foundation`*"]
    pub unsafe fn GetEnableTCP(&self) -> ::windows::runtime::Result<super::super::Foundation::BOOL> {
        let mut result__: <super::super::Foundation::BOOL as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).25)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`, `Win32_Foundation`*"]
    pub unsafe fn SetEnableTCP<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>>(&self, fenabletcp: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).26)(::std::mem::transmute_copy(self), fenabletcp.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
    pub unsafe fn ResetProtocolRollover(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).27)(::std::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
    pub unsafe fn GetConnectionBandwidth(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).28)(::std::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
    pub unsafe fn SetConnectionBandwidth(&self, dwconnectionbandwidth: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).29)(::std::mem::transmute_copy(self), ::std::mem::transmute(dwconnectionbandwidth)).ok()
    }
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
    pub unsafe fn GetNumProtocolsSupported(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).30)(::std::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`, `Win32_Foundation`*"]
    pub unsafe fn GetSupportedProtocolName(&self, dwprotocolnum: u32, pwszprotocolname: super::super::Foundation::PWSTR, pcchprotocolname: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).31)(::std::mem::transmute_copy(self), ::std::mem::transmute(dwprotocolnum), ::std::mem::transmute(pwszprotocolname), ::std::mem::transmute(pcchprotocolname)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`, `Win32_Foundation`*"]
    pub unsafe fn AddLoggingUrl<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, pwszurl: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).32)(::std::mem::transmute_copy(self), pwszurl.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`, `Win32_Foundation`*"]
    pub unsafe fn GetLoggingUrl(&self, dwindex: u32, pwszurl: super::super::Foundation::PWSTR, pcchurl: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).33)(::std::mem::transmute_copy(self), ::std::mem::transmute(dwindex), ::std::mem::transmute(pwszurl), ::std::mem::transmute(pcchurl)).ok()
    }
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
    pub unsafe fn GetLoggingUrlCount(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).34)(::std::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
    pub unsafe fn ResetLoggingUrlList(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).35)(::std::mem::transmute_copy(self)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`, `Win32_Foundation`*"]
    pub unsafe fn GetEnableContentCaching(&self) -> ::windows::runtime::Result<super::super::Foundation::BOOL> {
        let mut result__: <super::super::Foundation::BOOL as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).36)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`, `Win32_Foundation`*"]
    pub unsafe fn SetEnableContentCaching<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>>(&self, fenablecontentcaching: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).37)(::std::mem::transmute_copy(self), fenablecontentcaching.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`, `Win32_Foundation`*"]
    pub unsafe fn GetEnableFastCache(&self) -> ::windows::runtime::Result<super::super::Foundation::BOOL> {
        let mut result__: <super::super::Foundation::BOOL as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).38)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`, `Win32_Foundation`*"]
    pub unsafe fn SetEnableFastCache<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>>(&self, fenablefastcache: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).39)(::std::mem::transmute_copy(self), fenablefastcache.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
    pub unsafe fn GetAcceleratedStreamingDuration(&self) -> ::windows::runtime::Result<u64> {
        let mut result__: <u64 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).40)(::std::mem::transmute_copy(self), &mut result__).from_abi::<u64>(result__)
    }
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
    pub unsafe fn SetAcceleratedStreamingDuration(&self, cnsaccelduration: u64) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).41)(::std::mem::transmute_copy(self), ::std::mem::transmute(cnsaccelduration)).ok()
    }
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
    pub unsafe fn GetAutoReconnectLimit(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).42)(::std::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
    pub unsafe fn SetAutoReconnectLimit(&self, dwautoreconnectlimit: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).43)(::std::mem::transmute_copy(self), ::std::mem::transmute(dwautoreconnectlimit)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`, `Win32_Foundation`*"]
    pub unsafe fn GetEnableResends(&self) -> ::windows::runtime::Result<super::super::Foundation::BOOL> {
        let mut result__: <super::super::Foundation::BOOL as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).44)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`, `Win32_Foundation`*"]
    pub unsafe fn SetEnableResends<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>>(&self, fenableresends: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).45)(::std::mem::transmute_copy(self), fenableresends.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`, `Win32_Foundation`*"]
    pub unsafe fn GetEnableThinning(&self) -> ::windows::runtime::Result<super::super::Foundation::BOOL> {
        let mut result__: <super::super::Foundation::BOOL as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).46)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`, `Win32_Foundation`*"]
    pub unsafe fn SetEnableThinning<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>>(&self, fenablethinning: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).47)(::std::mem::transmute_copy(self), fenablethinning.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
    pub unsafe fn GetMaxNetPacketSize(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).48)(::std::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IWMReaderNetworkConfig2 {
    type Vtable = IWMReaderNetworkConfig2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3648628819, 1067, 16464, [131, 135, 201, 57, 219, 34, 1, 63]);
}
impl ::std::convert::From<IWMReaderNetworkConfig2> for IWMReaderNetworkConfig {
    fn from(value: IWMReaderNetworkConfig2) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IWMReaderNetworkConfig2> for IWMReaderNetworkConfig {
    fn from(value: &IWMReaderNetworkConfig2) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IWMReaderNetworkConfig> for IWMReaderNetworkConfig2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IWMReaderNetworkConfig> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IWMReaderNetworkConfig>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IWMReaderNetworkConfig> for &IWMReaderNetworkConfig2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IWMReaderNetworkConfig> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IWMReaderNetworkConfig>::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMReaderNetworkConfig2_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pcnsbufferingtime: *mut u64) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, cnsbufferingtime: u64) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, prangearray: *mut WM_PORT_NUMBER_RANGE, pcranges: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, prangearray: *const WM_PORT_NUMBER_RANGE, cranges: u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pwszprotocol: super::super::Foundation::PWSTR, pproxysetting: *mut WMT_PROXY_SETTINGS) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pwszprotocol: super::super::Foundation::PWSTR, proxysetting: WMT_PROXY_SETTINGS) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pwszprotocol: super::super::Foundation::PWSTR, pwszhostname: super::super::Foundation::PWSTR, pcchhostname: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pwszprotocol: super::super::Foundation::PWSTR, pwszhostname: super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pwszprotocol: super::super::Foundation::PWSTR, pdwport: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pwszprotocol: super::super::Foundation::PWSTR, dwport: u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pwszprotocol: super::super::Foundation::PWSTR, pwszexceptionlist: super::super::Foundation::PWSTR, pcchexceptionlist: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pwszprotocol: super::super::Foundation::PWSTR, pwszexceptionlist: super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pwszprotocol: super::super::Foundation::PWSTR, pfbypassforlocal: *mut super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pwszprotocol: super::super::Foundation::PWSTR, fbypassforlocal: super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pfforcererundetection: *mut super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, fforcererundetection: super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pfenablemulticast: *mut super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, fenablemulticast: super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pfenablehttp: *mut super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, fenablehttp: super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pfenableudp: *mut super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, fenableudp: super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pfenabletcp: *mut super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, fenabletcp: super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pdwconnectionbandwidth: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwconnectionbandwidth: u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pcprotocols: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwprotocolnum: u32, pwszprotocolname: super::super::Foundation::PWSTR, pcchprotocolname: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pwszurl: super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwindex: u32, pwszurl: super::super::Foundation::PWSTR, pcchurl: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pdwurlcount: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pfenablecontentcaching: *mut super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, fenablecontentcaching: super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pfenablefastcache: *mut super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, fenablefastcache: super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pcnsaccelduration: *mut u64) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, cnsaccelduration: u64) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pdwautoreconnectlimit: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwautoreconnectlimit: u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pfenableresends: *mut super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, fenableresends: super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pfenablethinning: *mut super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, fenablethinning: super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pdwmaxnetpacketsize: *mut u32) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct IWMReaderPlaylistBurn(::windows::runtime::IUnknown);
impl IWMReaderPlaylistBurn {
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`, `Win32_Foundation`*"]
    pub unsafe fn InitPlaylistBurn<'a, Param2: ::windows::runtime::IntoParam<'a, IWMStatusCallback>>(&self, cfiles: u32, ppwszfilenames: *const super::super::Foundation::PWSTR, pcallback: Param2, pvcontext: *const ::std::ffi::c_void) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), ::std::mem::transmute(cfiles), ::std::mem::transmute(ppwszfilenames), pcallback.into_param().abi(), ::std::mem::transmute(pvcontext)).ok()
    }
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
    pub unsafe fn GetInitResults(&self, cfiles: u32) -> ::windows::runtime::Result<::windows::runtime::HRESULT> {
        let mut result__: <::windows::runtime::HRESULT as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), ::std::mem::transmute(cfiles), &mut result__).from_abi::<::windows::runtime::HRESULT>(result__)
    }
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
    pub unsafe fn Cancel(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
    pub unsafe fn EndPlaylistBurn(&self, hrburnresult: ::windows::runtime::HRESULT) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::std::mem::transmute_copy(self), ::std::mem::transmute(hrburnresult)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IWMReaderPlaylistBurn {
    type Vtable = IWMReaderPlaylistBurn_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(4069262080, 39850, 17527, [168, 70, 23, 68, 217, 203, 245, 51]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMReaderPlaylistBurn_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, cfiles: u32, ppwszfilenames: *const super::super::Foundation::PWSTR, pcallback: ::windows::runtime::RawPtr, pvcontext: *const ::std::ffi::c_void) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, cfiles: u32, phrstati: *mut ::windows::runtime::HRESULT) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, hrburnresult: ::windows::runtime::HRESULT) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct IWMReaderStreamClock(::windows::runtime::IUnknown);
impl IWMReaderStreamClock {
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
    pub unsafe fn GetTime(&self, pcnsnow: *const u64) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), ::std::mem::transmute(pcnsnow)).ok()
    }
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
    pub unsafe fn SetTimer(&self, cnswhen: u64, pvparam: *const ::std::ffi::c_void) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), ::std::mem::transmute(cnswhen), ::std::mem::transmute(pvparam), &mut result__).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
    pub unsafe fn KillTimer(&self, dwtimerid: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self), ::std::mem::transmute(dwtimerid)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IWMReaderStreamClock {
    type Vtable = IWMReaderStreamClock_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2520804333, 11051, 4563, [179, 107, 0, 192, 79, 97, 8, 255]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMReaderStreamClock_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pcnsnow: *const u64) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, cnswhen: u64, pvparam: *const ::std::ffi::c_void, pdwtimerid: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwtimerid: u32) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct IWMReaderTimecode(::windows::runtime::IUnknown);
impl IWMReaderTimecode {
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
    pub unsafe fn GetTimecodeRangeCount(&self, wstreamnum: u16) -> ::windows::runtime::Result<u16> {
        let mut result__: <u16 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), ::std::mem::transmute(wstreamnum), &mut result__).from_abi::<u16>(result__)
    }
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
    pub unsafe fn GetTimecodeRangeBounds(&self, wstreamnum: u16, wrangenum: u16, pstarttimecode: *mut u32, pendtimecode: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), ::std::mem::transmute(wstreamnum), ::std::mem::transmute(wrangenum), ::std::mem::transmute(pstarttimecode), ::std::mem::transmute(pendtimecode)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IWMReaderTimecode {
    type Vtable = IWMReaderTimecode_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(4083802864, 57473, 20454, [132, 80, 184, 16, 178, 244, 16, 209]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMReaderTimecode_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, wstreamnum: u16, pwrangecount: *mut u16) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, wstreamnum: u16, wrangenum: u16, pstarttimecode: *mut u32, pendtimecode: *mut u32) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct IWMReaderTypeNegotiation(::windows::runtime::IUnknown);
impl IWMReaderTypeNegotiation {
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
    pub unsafe fn TryOutputProps<'a, Param1: ::windows::runtime::IntoParam<'a, IWMOutputMediaProps>>(&self, dwoutputnum: u32, poutput: Param1) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), ::std::mem::transmute(dwoutputnum), poutput.into_param().abi()).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IWMReaderTypeNegotiation {
    type Vtable = IWMReaderTypeNegotiation_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(4257109394, 33185, 16874, [147, 189, 115, 92, 173, 26, 220, 5]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMReaderTypeNegotiation_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwoutputnum: u32, poutput: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct IWMRegisterCallback(::windows::runtime::IUnknown);
impl IWMRegisterCallback {
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
    pub unsafe fn Advise<'a, Param0: ::windows::runtime::IntoParam<'a, IWMStatusCallback>>(&self, pcallback: Param0, pvcontext: *const ::std::ffi::c_void) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), pcallback.into_param().abi(), ::std::mem::transmute(pvcontext)).ok()
    }
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
    pub unsafe fn Unadvise<'a, Param0: ::windows::runtime::IntoParam<'a, IWMStatusCallback>>(&self, pcallback: Param0, pvcontext: *const ::std::ffi::c_void) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), pcallback.into_param().abi(), ::std::mem::transmute(pvcontext)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IWMRegisterCallback {
    type Vtable = IWMRegisterCallback_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3477807001, 19938, 20041, [163, 99, 37, 39, 64, 217, 155, 193]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMRegisterCallback_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pcallback: ::windows::runtime::RawPtr, pvcontext: *const ::std::ffi::c_void) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pcallback: ::windows::runtime::RawPtr, pvcontext: *const ::std::ffi::c_void) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct IWMRegisteredDevice(::windows::runtime::IUnknown);
impl IWMRegisteredDevice {
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
    pub unsafe fn GetDeviceSerialNumber(&self) -> ::windows::runtime::Result<DRM_VAL16> {
        let mut result__: <DRM_VAL16 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), &mut result__).from_abi::<DRM_VAL16>(result__)
    }
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
    pub unsafe fn GetDeviceCertificate(&self) -> ::windows::runtime::Result<INSSBuffer> {
        let mut result__: <INSSBuffer as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), &mut result__).from_abi::<INSSBuffer>(result__)
    }
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
    pub unsafe fn GetDeviceType(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
    pub unsafe fn GetAttributeCount(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).6)(::std::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`, `Win32_Foundation`*"]
    pub unsafe fn GetAttributeByIndex(&self, dwindex: u32, pbstrname: *mut super::super::Foundation::BSTR, pbstrvalue: *mut super::super::Foundation::BSTR) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self), ::std::mem::transmute(dwindex), ::std::mem::transmute(pbstrname), ::std::mem::transmute(pbstrvalue)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`, `Win32_Foundation`*"]
    pub unsafe fn GetAttributeByName<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstrname: Param0) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).8)(::std::mem::transmute_copy(self), bstrname.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`, `Win32_Foundation`*"]
    pub unsafe fn SetAttributeByName<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstrname: Param0, bstrvalue: Param1) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).9)(::std::mem::transmute_copy(self), bstrname.into_param().abi(), bstrvalue.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`, `Win32_Foundation`*"]
    pub unsafe fn Approve<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>>(&self, fapprove: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).10)(::std::mem::transmute_copy(self), fapprove.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`, `Win32_Foundation`*"]
    pub unsafe fn IsValid(&self) -> ::windows::runtime::Result<super::super::Foundation::BOOL> {
        let mut result__: <super::super::Foundation::BOOL as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).11)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`, `Win32_Foundation`*"]
    pub unsafe fn IsApproved(&self) -> ::windows::runtime::Result<super::super::Foundation::BOOL> {
        let mut result__: <super::super::Foundation::BOOL as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).12)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`, `Win32_Foundation`*"]
    pub unsafe fn IsWmdrmCompliant(&self) -> ::windows::runtime::Result<super::super::Foundation::BOOL> {
        let mut result__: <super::super::Foundation::BOOL as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).13)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`, `Win32_Foundation`*"]
    pub unsafe fn IsOpened(&self) -> ::windows::runtime::Result<super::super::Foundation::BOOL> {
        let mut result__: <super::super::Foundation::BOOL as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).14)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
    pub unsafe fn Open(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).15)(::std::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
    pub unsafe fn Close(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).16)(::std::mem::transmute_copy(self)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IWMRegisteredDevice {
    type Vtable = IWMRegisteredDevice_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2756721644, 21768, 16712, [151, 172, 191, 167, 87, 96, 167, 13]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMRegisteredDevice_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pserialnumber: *mut DRM_VAL16) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppcertificate: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pdwtype: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pcattributes: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwindex: u32, pbstrname: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>, pbstrvalue: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, bstrname: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>, pbstrvalue: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, bstrname: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrvalue: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, fapprove: super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pfvalid: *mut super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pfapproved: *mut super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pfcompliant: *mut super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pfopened: *mut super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct IWMSBufferAllocator(::windows::runtime::IUnknown);
impl IWMSBufferAllocator {
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
    pub unsafe fn AllocateBuffer(&self, dwmaxbuffersize: u32) -> ::windows::runtime::Result<INSSBuffer> {
        let mut result__: <INSSBuffer as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), ::std::mem::transmute(dwmaxbuffersize), &mut result__).from_abi::<INSSBuffer>(result__)
    }
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
    pub unsafe fn AllocatePageSizeBuffer(&self, dwmaxbuffersize: u32) -> ::windows::runtime::Result<INSSBuffer> {
        let mut result__: <INSSBuffer as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), ::std::mem::transmute(dwmaxbuffersize), &mut result__).from_abi::<INSSBuffer>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IWMSBufferAllocator {
    type Vtable = IWMSBufferAllocator_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1628454052, 8243, 4562, [158, 241, 0, 96, 151, 210, 215, 207]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMSBufferAllocator_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwmaxbuffersize: u32, ppbuffer: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwmaxbuffersize: u32, ppbuffer: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct IWMSInternalAdminNetSource(::windows::runtime::IUnknown);
impl IWMSInternalAdminNetSource {
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`, `Win32_Foundation`*"]
    pub unsafe fn Initialize<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>, Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>, Param2: ::windows::runtime::IntoParam<'a, INSNetSourceCreator>, Param3: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>>(&self, psharednamespace: Param0, pnamespacenode: Param1, pnetsourcecreator: Param2, fembeddedinserver: Param3) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), psharednamespace.into_param().abi(), pnamespacenode.into_param().abi(), pnetsourcecreator.into_param().abi(), fembeddedinserver.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
    pub unsafe fn GetNetSourceCreator(&self) -> ::windows::runtime::Result<INSNetSourceCreator> {
        let mut result__: <INSNetSourceCreator as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), &mut result__).from_abi::<INSNetSourceCreator>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`, `Win32_Foundation`*"]
    pub unsafe fn SetCredentials<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>, Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>, Param3: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>, Param4: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>>(
        &self,
        bstrrealm: Param0,
        bstrname: Param1,
        bstrpassword: Param2,
        fpersist: Param3,
        fconfirmedgood: Param4,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self), bstrrealm.into_param().abi(), bstrname.into_param().abi(), bstrpassword.into_param().abi(), fpersist.into_param().abi(), fconfirmedgood.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`, `Win32_Foundation`*"]
    pub unsafe fn GetCredentials<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstrrealm: Param0, pbstrname: *mut super::super::Foundation::BSTR, pbstrpassword: *mut super::super::Foundation::BSTR, pfconfirmedgood: *mut super::super::Foundation::BOOL) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::std::mem::transmute_copy(self), bstrrealm.into_param().abi(), ::std::mem::transmute(pbstrname), ::std::mem::transmute(pbstrpassword), ::std::mem::transmute(pfconfirmedgood)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`, `Win32_Foundation`*"]
    pub unsafe fn DeleteCredentials<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstrrealm: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self), bstrrealm.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
    pub unsafe fn GetCredentialFlags(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).8)(::std::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
    pub unsafe fn SetCredentialFlags(&self, dwflags: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).9)(::std::mem::transmute_copy(self), ::std::mem::transmute(dwflags)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`, `Win32_Foundation`*"]
    pub unsafe fn FindProxyForURL<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstrprotocol: Param0, bstrhost: Param1, pfproxyenabled: *mut super::super::Foundation::BOOL, pbstrproxyserver: *mut super::super::Foundation::BSTR, pdwproxyport: *mut u32, pdwproxycontext: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).10)(::std::mem::transmute_copy(self), bstrprotocol.into_param().abi(), bstrhost.into_param().abi(), ::std::mem::transmute(pfproxyenabled), ::std::mem::transmute(pbstrproxyserver), ::std::mem::transmute(pdwproxyport), ::std::mem::transmute(pdwproxycontext)).ok()
    }
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
    pub unsafe fn RegisterProxyFailure(&self, hrparam: ::windows::runtime::HRESULT, dwproxycontext: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).11)(::std::mem::transmute_copy(self), ::std::mem::transmute(hrparam), ::std::mem::transmute(dwproxycontext)).ok()
    }
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
    pub unsafe fn ShutdownProxyContext(&self, dwproxycontext: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).12)(::std::mem::transmute_copy(self), ::std::mem::transmute(dwproxycontext)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`, `Win32_Foundation`*"]
    pub unsafe fn IsUsingIE(&self, dwproxycontext: u32) -> ::windows::runtime::Result<super::super::Foundation::BOOL> {
        let mut result__: <super::super::Foundation::BOOL as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).13)(::std::mem::transmute_copy(self), ::std::mem::transmute(dwproxycontext), &mut result__).from_abi::<super::super::Foundation::BOOL>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IWMSInternalAdminNetSource {
    type Vtable = IWMSInternalAdminNetSource_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2343714399, 53543, 19195, [141, 2, 174, 91, 102, 213, 76, 120]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMSInternalAdminNetSource_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, psharednamespace: ::windows::runtime::RawPtr, pnamespacenode: ::windows::runtime::RawPtr, pnetsourcecreator: ::windows::runtime::RawPtr, fembeddedinserver: super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppnetsourcecreator: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, bstrrealm: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrname: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrpassword: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>, fpersist: super::super::Foundation::BOOL, fconfirmedgood: super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, bstrrealm: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>, pbstrname: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>, pbstrpassword: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>, pfconfirmedgood: *mut super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, bstrrealm: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, lpdwflags: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwflags: u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, bstrprotocol: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrhost: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>, pfproxyenabled: *mut super::super::Foundation::BOOL, pbstrproxyserver: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>, pdwproxyport: *mut u32, pdwproxycontext: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, hrparam: ::windows::runtime::HRESULT, dwproxycontext: u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwproxycontext: u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwproxycontext: u32, pfisusingie: *mut super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct IWMSInternalAdminNetSource2(::windows::runtime::IUnknown);
impl IWMSInternalAdminNetSource2 {
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`, `Win32_Foundation`*"]
    pub unsafe fn SetCredentialsEx<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>,
        Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>,
        Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>,
        Param3: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>,
        Param4: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>,
        Param5: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>,
        Param6: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>,
    >(
        &self,
        bstrrealm: Param0,
        bstrurl: Param1,
        fproxy: Param2,
        bstrname: Param3,
        bstrpassword: Param4,
        fpersist: Param5,
        fconfirmedgood: Param6,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), bstrrealm.into_param().abi(), bstrurl.into_param().abi(), fproxy.into_param().abi(), bstrname.into_param().abi(), bstrpassword.into_param().abi(), fpersist.into_param().abi(), fconfirmedgood.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`, `Win32_Foundation`*"]
    pub unsafe fn GetCredentialsEx<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>, Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>>(
        &self,
        bstrrealm: Param0,
        bstrurl: Param1,
        fproxy: Param2,
        pdwurlpolicy: *mut NETSOURCE_URLCREDPOLICY_SETTINGS,
        pbstrname: *mut super::super::Foundation::BSTR,
        pbstrpassword: *mut super::super::Foundation::BSTR,
        pfconfirmedgood: *mut super::super::Foundation::BOOL,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), bstrrealm.into_param().abi(), bstrurl.into_param().abi(), fproxy.into_param().abi(), ::std::mem::transmute(pdwurlpolicy), ::std::mem::transmute(pbstrname), ::std::mem::transmute(pbstrpassword), ::std::mem::transmute(pfconfirmedgood)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`, `Win32_Foundation`*"]
    pub unsafe fn DeleteCredentialsEx<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>, Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>>(&self, bstrrealm: Param0, bstrurl: Param1, fproxy: Param2) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self), bstrrealm.into_param().abi(), bstrurl.into_param().abi(), fproxy.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`, `Win32_Foundation`*"]
    pub unsafe fn FindProxyForURLEx<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>, Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(
        &self,
        bstrprotocol: Param0,
        bstrhost: Param1,
        bstrurl: Param2,
        pfproxyenabled: *mut super::super::Foundation::BOOL,
        pbstrproxyserver: *mut super::super::Foundation::BSTR,
        pdwproxyport: *mut u32,
        pdwproxycontext: *mut u32,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::std::mem::transmute_copy(self), bstrprotocol.into_param().abi(), bstrhost.into_param().abi(), bstrurl.into_param().abi(), ::std::mem::transmute(pfproxyenabled), ::std::mem::transmute(pbstrproxyserver), ::std::mem::transmute(pdwproxyport), ::std::mem::transmute(pdwproxycontext)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IWMSInternalAdminNetSource2 {
    type Vtable = IWMSInternalAdminNetSource2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3880605891, 53111, 19281, [175, 23, 116, 70, 135, 196, 62, 174]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMSInternalAdminNetSource2_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        bstrrealm: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>,
        bstrurl: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>,
        fproxy: super::super::Foundation::BOOL,
        bstrname: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>,
        bstrpassword: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>,
        fpersist: super::super::Foundation::BOOL,
        fconfirmedgood: super::super::Foundation::BOOL,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        bstrrealm: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>,
        bstrurl: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>,
        fproxy: super::super::Foundation::BOOL,
        pdwurlpolicy: *mut NETSOURCE_URLCREDPOLICY_SETTINGS,
        pbstrname: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>,
        pbstrpassword: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>,
        pfconfirmedgood: *mut super::super::Foundation::BOOL,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, bstrrealm: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrurl: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>, fproxy: super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(this: ::windows::runtime::RawPtr, bstrprotocol: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrhost: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrurl: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>, pfproxyenabled: *mut super::super::Foundation::BOOL, pbstrproxyserver: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>, pdwproxyport: *mut u32, pdwproxycontext: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct IWMSInternalAdminNetSource3(::windows::runtime::IUnknown);
impl IWMSInternalAdminNetSource3 {
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`, `Win32_Foundation`*"]
    pub unsafe fn SetCredentialsEx<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>,
        Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>,
        Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>,
        Param3: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>,
        Param4: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>,
        Param5: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>,
        Param6: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>,
    >(
        &self,
        bstrrealm: Param0,
        bstrurl: Param1,
        fproxy: Param2,
        bstrname: Param3,
        bstrpassword: Param4,
        fpersist: Param5,
        fconfirmedgood: Param6,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), bstrrealm.into_param().abi(), bstrurl.into_param().abi(), fproxy.into_param().abi(), bstrname.into_param().abi(), bstrpassword.into_param().abi(), fpersist.into_param().abi(), fconfirmedgood.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`, `Win32_Foundation`*"]
    pub unsafe fn GetCredentialsEx<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>, Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>>(
        &self,
        bstrrealm: Param0,
        bstrurl: Param1,
        fproxy: Param2,
        pdwurlpolicy: *mut NETSOURCE_URLCREDPOLICY_SETTINGS,
        pbstrname: *mut super::super::Foundation::BSTR,
        pbstrpassword: *mut super::super::Foundation::BSTR,
        pfconfirmedgood: *mut super::super::Foundation::BOOL,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), bstrrealm.into_param().abi(), bstrurl.into_param().abi(), fproxy.into_param().abi(), ::std::mem::transmute(pdwurlpolicy), ::std::mem::transmute(pbstrname), ::std::mem::transmute(pbstrpassword), ::std::mem::transmute(pfconfirmedgood)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`, `Win32_Foundation`*"]
    pub unsafe fn DeleteCredentialsEx<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>, Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>>(&self, bstrrealm: Param0, bstrurl: Param1, fproxy: Param2) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self), bstrrealm.into_param().abi(), bstrurl.into_param().abi(), fproxy.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`, `Win32_Foundation`*"]
    pub unsafe fn FindProxyForURLEx<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>, Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(
        &self,
        bstrprotocol: Param0,
        bstrhost: Param1,
        bstrurl: Param2,
        pfproxyenabled: *mut super::super::Foundation::BOOL,
        pbstrproxyserver: *mut super::super::Foundation::BSTR,
        pdwproxyport: *mut u32,
        pdwproxycontext: *mut u32,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::std::mem::transmute_copy(self), bstrprotocol.into_param().abi(), bstrhost.into_param().abi(), bstrurl.into_param().abi(), ::std::mem::transmute(pfproxyenabled), ::std::mem::transmute(pbstrproxyserver), ::std::mem::transmute(pdwproxyport), ::std::mem::transmute(pdwproxycontext)).ok()
    }
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
    pub unsafe fn GetNetSourceCreator2(&self) -> ::windows::runtime::Result<::windows::runtime::IUnknown> {
        let mut result__: <::windows::runtime::IUnknown as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self), &mut result__).from_abi::<::windows::runtime::IUnknown>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`, `Win32_Foundation`*"]
    pub unsafe fn FindProxyForURLEx2<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>, Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(
        &self,
        bstrprotocol: Param0,
        bstrhost: Param1,
        bstrurl: Param2,
        pfproxyenabled: *mut super::super::Foundation::BOOL,
        pbstrproxyserver: *mut super::super::Foundation::BSTR,
        pdwproxyport: *mut u32,
        pqwproxycontext: *mut u64,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).8)(::std::mem::transmute_copy(self), bstrprotocol.into_param().abi(), bstrhost.into_param().abi(), bstrurl.into_param().abi(), ::std::mem::transmute(pfproxyenabled), ::std::mem::transmute(pbstrproxyserver), ::std::mem::transmute(pdwproxyport), ::std::mem::transmute(pqwproxycontext)).ok()
    }
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
    pub unsafe fn RegisterProxyFailure2(&self, hrparam: ::windows::runtime::HRESULT, qwproxycontext: u64) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).9)(::std::mem::transmute_copy(self), ::std::mem::transmute(hrparam), ::std::mem::transmute(qwproxycontext)).ok()
    }
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
    pub unsafe fn ShutdownProxyContext2(&self, qwproxycontext: u64) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).10)(::std::mem::transmute_copy(self), ::std::mem::transmute(qwproxycontext)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`, `Win32_Foundation`*"]
    pub unsafe fn IsUsingIE2(&self, qwproxycontext: u64) -> ::windows::runtime::Result<super::super::Foundation::BOOL> {
        let mut result__: <super::super::Foundation::BOOL as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).11)(::std::mem::transmute_copy(self), ::std::mem::transmute(qwproxycontext), &mut result__).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`, `Win32_Foundation`*"]
    pub unsafe fn SetCredentialsEx2<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>,
        Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>,
        Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>,
        Param3: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>,
        Param4: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>,
        Param5: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>,
        Param6: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>,
        Param7: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>,
    >(
        &self,
        bstrrealm: Param0,
        bstrurl: Param1,
        fproxy: Param2,
        bstrname: Param3,
        bstrpassword: Param4,
        fpersist: Param5,
        fconfirmedgood: Param6,
        fcleartextauthentication: Param7,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).12)(::std::mem::transmute_copy(self), bstrrealm.into_param().abi(), bstrurl.into_param().abi(), fproxy.into_param().abi(), bstrname.into_param().abi(), bstrpassword.into_param().abi(), fpersist.into_param().abi(), fconfirmedgood.into_param().abi(), fcleartextauthentication.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`, `Win32_Foundation`*"]
    pub unsafe fn GetCredentialsEx2<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>, Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>, Param3: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>>(
        &self,
        bstrrealm: Param0,
        bstrurl: Param1,
        fproxy: Param2,
        fcleartextauthentication: Param3,
        pdwurlpolicy: *mut NETSOURCE_URLCREDPOLICY_SETTINGS,
        pbstrname: *mut super::super::Foundation::BSTR,
        pbstrpassword: *mut super::super::Foundation::BSTR,
        pfconfirmedgood: *mut super::super::Foundation::BOOL,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).13)(
            ::std::mem::transmute_copy(self),
            bstrrealm.into_param().abi(),
            bstrurl.into_param().abi(),
            fproxy.into_param().abi(),
            fcleartextauthentication.into_param().abi(),
            ::std::mem::transmute(pdwurlpolicy),
            ::std::mem::transmute(pbstrname),
            ::std::mem::transmute(pbstrpassword),
            ::std::mem::transmute(pfconfirmedgood),
        )
        .ok()
    }
}
unsafe impl ::windows::runtime::Interface for IWMSInternalAdminNetSource3 {
    type Vtable = IWMSInternalAdminNetSource3_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1801703566, 17808, 17583, [158, 179, 87, 255, 30, 115, 191, 128]);
}
impl ::std::convert::From<IWMSInternalAdminNetSource3> for IWMSInternalAdminNetSource2 {
    fn from(value: IWMSInternalAdminNetSource3) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IWMSInternalAdminNetSource3> for IWMSInternalAdminNetSource2 {
    fn from(value: &IWMSInternalAdminNetSource3) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IWMSInternalAdminNetSource2> for IWMSInternalAdminNetSource3 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IWMSInternalAdminNetSource2> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IWMSInternalAdminNetSource2>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IWMSInternalAdminNetSource2> for &IWMSInternalAdminNetSource3 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IWMSInternalAdminNetSource2> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IWMSInternalAdminNetSource2>::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMSInternalAdminNetSource3_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        bstrrealm: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>,
        bstrurl: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>,
        fproxy: super::super::Foundation::BOOL,
        bstrname: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>,
        bstrpassword: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>,
        fpersist: super::super::Foundation::BOOL,
        fconfirmedgood: super::super::Foundation::BOOL,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        bstrrealm: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>,
        bstrurl: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>,
        fproxy: super::super::Foundation::BOOL,
        pdwurlpolicy: *mut NETSOURCE_URLCREDPOLICY_SETTINGS,
        pbstrname: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>,
        pbstrpassword: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>,
        pfconfirmedgood: *mut super::super::Foundation::BOOL,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, bstrrealm: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrurl: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>, fproxy: super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(this: ::windows::runtime::RawPtr, bstrprotocol: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrhost: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrurl: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>, pfproxyenabled: *mut super::super::Foundation::BOOL, pbstrproxyserver: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>, pdwproxyport: *mut u32, pdwproxycontext: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppnetsourcecreator: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(this: ::windows::runtime::RawPtr, bstrprotocol: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrhost: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrurl: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>, pfproxyenabled: *mut super::super::Foundation::BOOL, pbstrproxyserver: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>, pdwproxyport: *mut u32, pqwproxycontext: *mut u64) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, hrparam: ::windows::runtime::HRESULT, qwproxycontext: u64) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, qwproxycontext: u64) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, qwproxycontext: u64, pfisusingie: *mut super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        bstrrealm: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>,
        bstrurl: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>,
        fproxy: super::super::Foundation::BOOL,
        bstrname: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>,
        bstrpassword: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>,
        fpersist: super::super::Foundation::BOOL,
        fconfirmedgood: super::super::Foundation::BOOL,
        fcleartextauthentication: super::super::Foundation::BOOL,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        bstrrealm: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>,
        bstrurl: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>,
        fproxy: super::super::Foundation::BOOL,
        fcleartextauthentication: super::super::Foundation::BOOL,
        pdwurlpolicy: *mut NETSOURCE_URLCREDPOLICY_SETTINGS,
        pbstrname: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>,
        pbstrpassword: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>,
        pfconfirmedgood: *mut super::super::Foundation::BOOL,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct IWMSecureChannel(::windows::runtime::IUnknown);
impl IWMSecureChannel {
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
    pub unsafe fn GetCertCount(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
    pub unsafe fn GetCert(&self, dwindex: u32) -> ::windows::runtime::Result<*mut u8> {
        let mut result__: <*mut u8 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), ::std::mem::transmute(dwindex), &mut result__).from_abi::<*mut u8>(result__)
    }
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
    pub unsafe fn GetSharedData(&self, dwcertindex: u32, pbshareddata: *const u8, pbcert: *const u8) -> ::windows::runtime::Result<*mut u8> {
        let mut result__: <*mut u8 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self), ::std::mem::transmute(dwcertindex), ::std::mem::transmute(pbshareddata), ::std::mem::transmute(pbcert), &mut result__).from_abi::<*mut u8>(result__)
    }
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
    pub unsafe fn WMSC_AddCertificate<'a, Param0: ::windows::runtime::IntoParam<'a, IWMAuthorizer>>(&self, pcert: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::std::mem::transmute_copy(self), pcert.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
    pub unsafe fn WMSC_AddSignature(&self, pbcertsig: *const u8, cbcertsig: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self), ::std::mem::transmute(pbcertsig), ::std::mem::transmute(cbcertsig)).ok()
    }
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
    pub unsafe fn WMSC_Connect<'a, Param0: ::windows::runtime::IntoParam<'a, IWMSecureChannel>>(&self, potherside: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).8)(::std::mem::transmute_copy(self), potherside.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`, `Win32_Foundation`*"]
    pub unsafe fn WMSC_IsConnected(&self) -> ::windows::runtime::Result<super::super::Foundation::BOOL> {
        let mut result__: <super::super::Foundation::BOOL as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).9)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
    pub unsafe fn WMSC_Disconnect(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).10)(::std::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
    pub unsafe fn WMSC_GetValidCertificate(&self, ppbcertificate: *mut *mut u8, pdwsignature: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).11)(::std::mem::transmute_copy(self), ::std::mem::transmute(ppbcertificate), ::std::mem::transmute(pdwsignature)).ok()
    }
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
    pub unsafe fn WMSC_Encrypt(&self, pbdata: *const u8, cbdata: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).12)(::std::mem::transmute_copy(self), ::std::mem::transmute(pbdata), ::std::mem::transmute(cbdata)).ok()
    }
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
    pub unsafe fn WMSC_Decrypt(&self, pbdata: *const u8, cbdata: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).13)(::std::mem::transmute_copy(self), ::std::mem::transmute(pbdata), ::std::mem::transmute(cbdata)).ok()
    }
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
    pub unsafe fn WMSC_Lock(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).14)(::std::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
    pub unsafe fn WMSC_Unlock(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).15)(::std::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
    pub unsafe fn WMSC_SetSharedData(&self, dwcertindex: u32, pbshareddata: *const u8) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).16)(::std::mem::transmute_copy(self), ::std::mem::transmute(dwcertindex), ::std::mem::transmute(pbshareddata)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IWMSecureChannel {
    type Vtable = IWMSecureChannel_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(656431498, 53490, 16777, [189, 16, 145, 196, 110, 240, 147, 111]);
}
impl ::std::convert::From<IWMSecureChannel> for IWMAuthorizer {
    fn from(value: IWMSecureChannel) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IWMSecureChannel> for IWMAuthorizer {
    fn from(value: &IWMSecureChannel) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IWMAuthorizer> for IWMSecureChannel {
    fn into_param(self) -> ::windows::runtime::Param<'a, IWMAuthorizer> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IWMAuthorizer>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IWMAuthorizer> for &IWMSecureChannel {
    fn into_param(self) -> ::windows::runtime::Param<'a, IWMAuthorizer> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IWMAuthorizer>::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMSecureChannel_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pccerts: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwindex: u32, ppbcertdata: *mut *mut u8) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwcertindex: u32, pbshareddata: *const u8, pbcert: *const u8, ppbshareddata: *mut *mut u8) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pcert: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbcertsig: *const u8, cbcertsig: u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, potherside: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pfisconnected: *mut super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppbcertificate: *mut *mut u8, pdwsignature: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbdata: *const u8, cbdata: u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbdata: *const u8, cbdata: u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwcertindex: u32, pbshareddata: *const u8) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct IWMStatusCallback(::windows::runtime::IUnknown);
impl IWMStatusCallback {
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
    pub unsafe fn OnStatus(&self, status: WMT_STATUS, hr: ::windows::runtime::HRESULT, dwtype: WMT_ATTR_DATATYPE, pvalue: *const u8, pvcontext: *const ::std::ffi::c_void) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), ::std::mem::transmute(status), ::std::mem::transmute(hr), ::std::mem::transmute(dwtype), ::std::mem::transmute(pvalue), ::std::mem::transmute(pvcontext)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IWMStatusCallback {
    type Vtable = IWMStatusCallback_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1836899440, 39048, 4563, [142, 220, 0, 192, 79, 97, 9, 207]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMStatusCallback_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, status: WMT_STATUS, hr: ::windows::runtime::HRESULT, dwtype: WMT_ATTR_DATATYPE, pvalue: *const u8, pvcontext: *const ::std::ffi::c_void) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct IWMStreamConfig(::windows::runtime::IUnknown);
impl IWMStreamConfig {
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
    pub unsafe fn GetStreamType(&self) -> ::windows::runtime::Result<::windows::runtime::GUID> {
        let mut result__: <::windows::runtime::GUID as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), &mut result__).from_abi::<::windows::runtime::GUID>(result__)
    }
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
    pub unsafe fn GetStreamNumber(&self) -> ::windows::runtime::Result<u16> {
        let mut result__: <u16 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), &mut result__).from_abi::<u16>(result__)
    }
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
    pub unsafe fn SetStreamNumber(&self, wstreamnum: u16) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self), ::std::mem::transmute(wstreamnum)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`, `Win32_Foundation`*"]
    pub unsafe fn GetStreamName(&self, pwszstreamname: super::super::Foundation::PWSTR, pcchstreamname: *mut u16) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::std::mem::transmute_copy(self), ::std::mem::transmute(pwszstreamname), ::std::mem::transmute(pcchstreamname)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`, `Win32_Foundation`*"]
    pub unsafe fn SetStreamName<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, pwszstreamname: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self), pwszstreamname.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`, `Win32_Foundation`*"]
    pub unsafe fn GetConnectionName(&self, pwszinputname: super::super::Foundation::PWSTR, pcchinputname: *mut u16) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).8)(::std::mem::transmute_copy(self), ::std::mem::transmute(pwszinputname), ::std::mem::transmute(pcchinputname)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`, `Win32_Foundation`*"]
    pub unsafe fn SetConnectionName<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, pwszinputname: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).9)(::std::mem::transmute_copy(self), pwszinputname.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
    pub unsafe fn GetBitrate(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).10)(::std::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
    pub unsafe fn SetBitrate(&self, pdwbitrate: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).11)(::std::mem::transmute_copy(self), ::std::mem::transmute(pdwbitrate)).ok()
    }
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
    pub unsafe fn GetBufferWindow(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).12)(::std::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
    pub unsafe fn SetBufferWindow(&self, msbufferwindow: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).13)(::std::mem::transmute_copy(self), ::std::mem::transmute(msbufferwindow)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IWMStreamConfig {
    type Vtable = IWMStreamConfig_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2520804316, 11051, 4563, [179, 107, 0, 192, 79, 97, 8, 255]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMStreamConfig_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pguidstreamtype: *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pwstreamnum: *mut u16) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, wstreamnum: u16) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pwszstreamname: super::super::Foundation::PWSTR, pcchstreamname: *mut u16) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pwszstreamname: super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pwszinputname: super::super::Foundation::PWSTR, pcchinputname: *mut u16) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pwszinputname: super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pdwbitrate: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pdwbitrate: u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pmsbufferwindow: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, msbufferwindow: u32) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct IWMStreamConfig2(::windows::runtime::IUnknown);
impl IWMStreamConfig2 {
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
    pub unsafe fn GetStreamType(&self) -> ::windows::runtime::Result<::windows::runtime::GUID> {
        let mut result__: <::windows::runtime::GUID as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), &mut result__).from_abi::<::windows::runtime::GUID>(result__)
    }
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
    pub unsafe fn GetStreamNumber(&self) -> ::windows::runtime::Result<u16> {
        let mut result__: <u16 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), &mut result__).from_abi::<u16>(result__)
    }
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
    pub unsafe fn SetStreamNumber(&self, wstreamnum: u16) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self), ::std::mem::transmute(wstreamnum)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`, `Win32_Foundation`*"]
    pub unsafe fn GetStreamName(&self, pwszstreamname: super::super::Foundation::PWSTR, pcchstreamname: *mut u16) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::std::mem::transmute_copy(self), ::std::mem::transmute(pwszstreamname), ::std::mem::transmute(pcchstreamname)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`, `Win32_Foundation`*"]
    pub unsafe fn SetStreamName<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, pwszstreamname: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self), pwszstreamname.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`, `Win32_Foundation`*"]
    pub unsafe fn GetConnectionName(&self, pwszinputname: super::super::Foundation::PWSTR, pcchinputname: *mut u16) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).8)(::std::mem::transmute_copy(self), ::std::mem::transmute(pwszinputname), ::std::mem::transmute(pcchinputname)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`, `Win32_Foundation`*"]
    pub unsafe fn SetConnectionName<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, pwszinputname: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).9)(::std::mem::transmute_copy(self), pwszinputname.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
    pub unsafe fn GetBitrate(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).10)(::std::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
    pub unsafe fn SetBitrate(&self, pdwbitrate: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).11)(::std::mem::transmute_copy(self), ::std::mem::transmute(pdwbitrate)).ok()
    }
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
    pub unsafe fn GetBufferWindow(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).12)(::std::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
    pub unsafe fn SetBufferWindow(&self, msbufferwindow: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).13)(::std::mem::transmute_copy(self), ::std::mem::transmute(msbufferwindow)).ok()
    }
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
    pub unsafe fn GetTransportType(&self) -> ::windows::runtime::Result<WMT_TRANSPORT_TYPE> {
        let mut result__: <WMT_TRANSPORT_TYPE as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).14)(::std::mem::transmute_copy(self), &mut result__).from_abi::<WMT_TRANSPORT_TYPE>(result__)
    }
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
    pub unsafe fn SetTransportType(&self, ntransporttype: WMT_TRANSPORT_TYPE) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).15)(::std::mem::transmute_copy(self), ::std::mem::transmute(ntransporttype)).ok()
    }
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
    pub unsafe fn AddDataUnitExtension<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::GUID>>(&self, guidextensionsystemid: Param0, cbextensiondatasize: u16, pbextensionsysteminfo: *const u8, cbextensionsysteminfo: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).16)(::std::mem::transmute_copy(self), guidextensionsystemid.into_param().abi(), ::std::mem::transmute(cbextensiondatasize), ::std::mem::transmute(pbextensionsysteminfo), ::std::mem::transmute(cbextensionsysteminfo)).ok()
    }
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
    pub unsafe fn GetDataUnitExtensionCount(&self) -> ::windows::runtime::Result<u16> {
        let mut result__: <u16 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).17)(::std::mem::transmute_copy(self), &mut result__).from_abi::<u16>(result__)
    }
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
    pub unsafe fn GetDataUnitExtension(&self, wdataunitextensionnumber: u16, pguidextensionsystemid: *mut ::windows::runtime::GUID, pcbextensiondatasize: *mut u16, pbextensionsysteminfo: *mut u8, pcbextensionsysteminfo: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).18)(::std::mem::transmute_copy(self), ::std::mem::transmute(wdataunitextensionnumber), ::std::mem::transmute(pguidextensionsystemid), ::std::mem::transmute(pcbextensiondatasize), ::std::mem::transmute(pbextensionsysteminfo), ::std::mem::transmute(pcbextensionsysteminfo)).ok()
    }
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
    pub unsafe fn RemoveAllDataUnitExtensions(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).19)(::std::mem::transmute_copy(self)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IWMStreamConfig2 {
    type Vtable = IWMStreamConfig2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1988679883, 64525, 17341, [148, 89, 90, 141, 236, 32, 12, 250]);
}
impl ::std::convert::From<IWMStreamConfig2> for IWMStreamConfig {
    fn from(value: IWMStreamConfig2) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IWMStreamConfig2> for IWMStreamConfig {
    fn from(value: &IWMStreamConfig2) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IWMStreamConfig> for IWMStreamConfig2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IWMStreamConfig> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IWMStreamConfig>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IWMStreamConfig> for &IWMStreamConfig2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IWMStreamConfig> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IWMStreamConfig>::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMStreamConfig2_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pguidstreamtype: *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pwstreamnum: *mut u16) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, wstreamnum: u16) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pwszstreamname: super::super::Foundation::PWSTR, pcchstreamname: *mut u16) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pwszstreamname: super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pwszinputname: super::super::Foundation::PWSTR, pcchinputname: *mut u16) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pwszinputname: super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pdwbitrate: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pdwbitrate: u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pmsbufferwindow: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, msbufferwindow: u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pntransporttype: *mut WMT_TRANSPORT_TYPE) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ntransporttype: WMT_TRANSPORT_TYPE) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, guidextensionsystemid: ::windows::runtime::GUID, cbextensiondatasize: u16, pbextensionsysteminfo: *const u8, cbextensionsysteminfo: u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pcdataunitextensions: *mut u16) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, wdataunitextensionnumber: u16, pguidextensionsystemid: *mut ::windows::runtime::GUID, pcbextensiondatasize: *mut u16, pbextensionsysteminfo: *mut u8, pcbextensionsysteminfo: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct IWMStreamConfig3(::windows::runtime::IUnknown);
impl IWMStreamConfig3 {
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
    pub unsafe fn GetStreamType(&self) -> ::windows::runtime::Result<::windows::runtime::GUID> {
        let mut result__: <::windows::runtime::GUID as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), &mut result__).from_abi::<::windows::runtime::GUID>(result__)
    }
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
    pub unsafe fn GetStreamNumber(&self) -> ::windows::runtime::Result<u16> {
        let mut result__: <u16 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), &mut result__).from_abi::<u16>(result__)
    }
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
    pub unsafe fn SetStreamNumber(&self, wstreamnum: u16) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self), ::std::mem::transmute(wstreamnum)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`, `Win32_Foundation`*"]
    pub unsafe fn GetStreamName(&self, pwszstreamname: super::super::Foundation::PWSTR, pcchstreamname: *mut u16) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::std::mem::transmute_copy(self), ::std::mem::transmute(pwszstreamname), ::std::mem::transmute(pcchstreamname)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`, `Win32_Foundation`*"]
    pub unsafe fn SetStreamName<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, pwszstreamname: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self), pwszstreamname.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`, `Win32_Foundation`*"]
    pub unsafe fn GetConnectionName(&self, pwszinputname: super::super::Foundation::PWSTR, pcchinputname: *mut u16) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).8)(::std::mem::transmute_copy(self), ::std::mem::transmute(pwszinputname), ::std::mem::transmute(pcchinputname)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`, `Win32_Foundation`*"]
    pub unsafe fn SetConnectionName<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, pwszinputname: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).9)(::std::mem::transmute_copy(self), pwszinputname.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
    pub unsafe fn GetBitrate(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).10)(::std::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
    pub unsafe fn SetBitrate(&self, pdwbitrate: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).11)(::std::mem::transmute_copy(self), ::std::mem::transmute(pdwbitrate)).ok()
    }
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
    pub unsafe fn GetBufferWindow(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).12)(::std::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
    pub unsafe fn SetBufferWindow(&self, msbufferwindow: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).13)(::std::mem::transmute_copy(self), ::std::mem::transmute(msbufferwindow)).ok()
    }
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
    pub unsafe fn GetTransportType(&self) -> ::windows::runtime::Result<WMT_TRANSPORT_TYPE> {
        let mut result__: <WMT_TRANSPORT_TYPE as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).14)(::std::mem::transmute_copy(self), &mut result__).from_abi::<WMT_TRANSPORT_TYPE>(result__)
    }
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
    pub unsafe fn SetTransportType(&self, ntransporttype: WMT_TRANSPORT_TYPE) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).15)(::std::mem::transmute_copy(self), ::std::mem::transmute(ntransporttype)).ok()
    }
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
    pub unsafe fn AddDataUnitExtension<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::GUID>>(&self, guidextensionsystemid: Param0, cbextensiondatasize: u16, pbextensionsysteminfo: *const u8, cbextensionsysteminfo: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).16)(::std::mem::transmute_copy(self), guidextensionsystemid.into_param().abi(), ::std::mem::transmute(cbextensiondatasize), ::std::mem::transmute(pbextensionsysteminfo), ::std::mem::transmute(cbextensionsysteminfo)).ok()
    }
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
    pub unsafe fn GetDataUnitExtensionCount(&self) -> ::windows::runtime::Result<u16> {
        let mut result__: <u16 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).17)(::std::mem::transmute_copy(self), &mut result__).from_abi::<u16>(result__)
    }
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
    pub unsafe fn GetDataUnitExtension(&self, wdataunitextensionnumber: u16, pguidextensionsystemid: *mut ::windows::runtime::GUID, pcbextensiondatasize: *mut u16, pbextensionsysteminfo: *mut u8, pcbextensionsysteminfo: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).18)(::std::mem::transmute_copy(self), ::std::mem::transmute(wdataunitextensionnumber), ::std::mem::transmute(pguidextensionsystemid), ::std::mem::transmute(pcbextensiondatasize), ::std::mem::transmute(pbextensionsysteminfo), ::std::mem::transmute(pcbextensionsysteminfo)).ok()
    }
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
    pub unsafe fn RemoveAllDataUnitExtensions(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).19)(::std::mem::transmute_copy(self)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`, `Win32_Foundation`*"]
    pub unsafe fn GetLanguage(&self, pwszlanguagestring: super::super::Foundation::PWSTR, pcchlanguagestringlength: *mut u16) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).20)(::std::mem::transmute_copy(self), ::std::mem::transmute(pwszlanguagestring), ::std::mem::transmute(pcchlanguagestringlength)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`, `Win32_Foundation`*"]
    pub unsafe fn SetLanguage<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, pwszlanguagestring: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).21)(::std::mem::transmute_copy(self), pwszlanguagestring.into_param().abi()).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IWMStreamConfig3 {
    type Vtable = IWMStreamConfig3_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3407233284, 15017, 17831, [154, 201, 77, 174, 225, 49, 214, 225]);
}
impl ::std::convert::From<IWMStreamConfig3> for IWMStreamConfig2 {
    fn from(value: IWMStreamConfig3) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IWMStreamConfig3> for IWMStreamConfig2 {
    fn from(value: &IWMStreamConfig3) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IWMStreamConfig2> for IWMStreamConfig3 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IWMStreamConfig2> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IWMStreamConfig2>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IWMStreamConfig2> for &IWMStreamConfig3 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IWMStreamConfig2> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IWMStreamConfig2>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<IWMStreamConfig3> for IWMStreamConfig {
    fn from(value: IWMStreamConfig3) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IWMStreamConfig3> for IWMStreamConfig {
    fn from(value: &IWMStreamConfig3) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IWMStreamConfig> for IWMStreamConfig3 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IWMStreamConfig> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IWMStreamConfig>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IWMStreamConfig> for &IWMStreamConfig3 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IWMStreamConfig> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IWMStreamConfig>::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMStreamConfig3_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pguidstreamtype: *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pwstreamnum: *mut u16) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, wstreamnum: u16) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pwszstreamname: super::super::Foundation::PWSTR, pcchstreamname: *mut u16) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pwszstreamname: super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pwszinputname: super::super::Foundation::PWSTR, pcchinputname: *mut u16) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pwszinputname: super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pdwbitrate: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pdwbitrate: u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pmsbufferwindow: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, msbufferwindow: u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pntransporttype: *mut WMT_TRANSPORT_TYPE) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ntransporttype: WMT_TRANSPORT_TYPE) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, guidextensionsystemid: ::windows::runtime::GUID, cbextensiondatasize: u16, pbextensionsysteminfo: *const u8, cbextensionsysteminfo: u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pcdataunitextensions: *mut u16) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, wdataunitextensionnumber: u16, pguidextensionsystemid: *mut ::windows::runtime::GUID, pcbextensiondatasize: *mut u16, pbextensionsysteminfo: *mut u8, pcbextensionsysteminfo: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pwszlanguagestring: super::super::Foundation::PWSTR, pcchlanguagestringlength: *mut u16) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pwszlanguagestring: super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct IWMStreamList(::windows::runtime::IUnknown);
impl IWMStreamList {
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
    pub unsafe fn GetStreams(&self, pwstreamnumarray: *mut u16, pcstreams: *mut u16) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), ::std::mem::transmute(pwstreamnumarray), ::std::mem::transmute(pcstreams)).ok()
    }
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
    pub unsafe fn AddStream(&self, wstreamnum: u16) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), ::std::mem::transmute(wstreamnum)).ok()
    }
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
    pub unsafe fn RemoveStream(&self, wstreamnum: u16) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self), ::std::mem::transmute(wstreamnum)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IWMStreamList {
    type Vtable = IWMStreamList_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2520804317, 11051, 4563, [179, 107, 0, 192, 79, 97, 8, 255]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMStreamList_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pwstreamnumarray: *mut u16, pcstreams: *mut u16) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, wstreamnum: u16) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, wstreamnum: u16) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct IWMStreamPrioritization(::windows::runtime::IUnknown);
impl IWMStreamPrioritization {
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`, `Win32_Foundation`*"]
    pub unsafe fn GetPriorityRecords(&self, precordarray: *mut WM_STREAM_PRIORITY_RECORD, pcrecords: *mut u16) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), ::std::mem::transmute(precordarray), ::std::mem::transmute(pcrecords)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`, `Win32_Foundation`*"]
    pub unsafe fn SetPriorityRecords(&self, precordarray: *const WM_STREAM_PRIORITY_RECORD, crecords: u16) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), ::std::mem::transmute(precordarray), ::std::mem::transmute(crecords)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IWMStreamPrioritization {
    type Vtable = IWMStreamPrioritization_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2350669968, 63912, 18248, [142, 195, 221, 17, 8, 186, 30, 119]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMStreamPrioritization_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, precordarray: *mut WM_STREAM_PRIORITY_RECORD, pcrecords: *mut u16) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, precordarray: *const WM_STREAM_PRIORITY_RECORD, crecords: u16) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct IWMSyncReader(::windows::runtime::IUnknown);
impl IWMSyncReader {
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`, `Win32_Foundation`*"]
    pub unsafe fn Open<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, pwszfilename: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), pwszfilename.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
    pub unsafe fn Close(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
    pub unsafe fn SetRange(&self, cnsstarttime: u64, cnsduration: i64) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self), ::std::mem::transmute(cnsstarttime), ::std::mem::transmute(cnsduration)).ok()
    }
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
    pub unsafe fn SetRangeByFrame(&self, wstreamnum: u16, qwframenumber: u64, cframestoread: i64) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::std::mem::transmute_copy(self), ::std::mem::transmute(wstreamnum), ::std::mem::transmute(qwframenumber), ::std::mem::transmute(cframestoread)).ok()
    }
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
    pub unsafe fn GetNextSample(&self, wstreamnum: u16, ppsample: *mut ::std::option::Option<INSSBuffer>, pcnssampletime: *mut u64, pcnsduration: *mut u64, pdwflags: *mut u32, pdwoutputnum: *mut u32, pwstreamnum: *mut u16) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self), ::std::mem::transmute(wstreamnum), ::std::mem::transmute(ppsample), ::std::mem::transmute(pcnssampletime), ::std::mem::transmute(pcnsduration), ::std::mem::transmute(pdwflags), ::std::mem::transmute(pdwoutputnum), ::std::mem::transmute(pwstreamnum)).ok()
    }
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
    pub unsafe fn SetStreamsSelected(&self, cstreamcount: u16, pwstreamnumbers: *const u16, pselections: *const WMT_STREAM_SELECTION) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).8)(::std::mem::transmute_copy(self), ::std::mem::transmute(cstreamcount), ::std::mem::transmute(pwstreamnumbers), ::std::mem::transmute(pselections)).ok()
    }
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
    pub unsafe fn GetStreamSelected(&self, wstreamnum: u16) -> ::windows::runtime::Result<WMT_STREAM_SELECTION> {
        let mut result__: <WMT_STREAM_SELECTION as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).9)(::std::mem::transmute_copy(self), ::std::mem::transmute(wstreamnum), &mut result__).from_abi::<WMT_STREAM_SELECTION>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`, `Win32_Foundation`*"]
    pub unsafe fn SetReadStreamSamples<'a, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>>(&self, wstreamnum: u16, fcompressed: Param1) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).10)(::std::mem::transmute_copy(self), ::std::mem::transmute(wstreamnum), fcompressed.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`, `Win32_Foundation`*"]
    pub unsafe fn GetReadStreamSamples(&self, wstreamnum: u16) -> ::windows::runtime::Result<super::super::Foundation::BOOL> {
        let mut result__: <super::super::Foundation::BOOL as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).11)(::std::mem::transmute_copy(self), ::std::mem::transmute(wstreamnum), &mut result__).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`, `Win32_Foundation`*"]
    pub unsafe fn GetOutputSetting<'a, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, dwoutputnum: u32, pszname: Param1, ptype: *mut WMT_ATTR_DATATYPE, pvalue: *mut u8, pcblength: *mut u16) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).12)(::std::mem::transmute_copy(self), ::std::mem::transmute(dwoutputnum), pszname.into_param().abi(), ::std::mem::transmute(ptype), ::std::mem::transmute(pvalue), ::std::mem::transmute(pcblength)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`, `Win32_Foundation`*"]
    pub unsafe fn SetOutputSetting<'a, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, dwoutputnum: u32, pszname: Param1, r#type: WMT_ATTR_DATATYPE, pvalue: *const u8, cblength: u16) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).13)(::std::mem::transmute_copy(self), ::std::mem::transmute(dwoutputnum), pszname.into_param().abi(), ::std::mem::transmute(r#type), ::std::mem::transmute(pvalue), ::std::mem::transmute(cblength)).ok()
    }
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
    pub unsafe fn GetOutputCount(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).14)(::std::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
    pub unsafe fn GetOutputProps(&self, dwoutputnum: u32) -> ::windows::runtime::Result<IWMOutputMediaProps> {
        let mut result__: <IWMOutputMediaProps as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).15)(::std::mem::transmute_copy(self), ::std::mem::transmute(dwoutputnum), &mut result__).from_abi::<IWMOutputMediaProps>(result__)
    }
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
    pub unsafe fn SetOutputProps<'a, Param1: ::windows::runtime::IntoParam<'a, IWMOutputMediaProps>>(&self, dwoutputnum: u32, poutput: Param1) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).16)(::std::mem::transmute_copy(self), ::std::mem::transmute(dwoutputnum), poutput.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
    pub unsafe fn GetOutputFormatCount(&self, dwoutputnum: u32) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).17)(::std::mem::transmute_copy(self), ::std::mem::transmute(dwoutputnum), &mut result__).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
    pub unsafe fn GetOutputFormat(&self, dwoutputnum: u32, dwformatnum: u32) -> ::windows::runtime::Result<IWMOutputMediaProps> {
        let mut result__: <IWMOutputMediaProps as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).18)(::std::mem::transmute_copy(self), ::std::mem::transmute(dwoutputnum), ::std::mem::transmute(dwformatnum), &mut result__).from_abi::<IWMOutputMediaProps>(result__)
    }
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
    pub unsafe fn GetOutputNumberForStream(&self, wstreamnum: u16) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).19)(::std::mem::transmute_copy(self), ::std::mem::transmute(wstreamnum), &mut result__).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
    pub unsafe fn GetStreamNumberForOutput(&self, dwoutputnum: u32) -> ::windows::runtime::Result<u16> {
        let mut result__: <u16 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).20)(::std::mem::transmute_copy(self), ::std::mem::transmute(dwoutputnum), &mut result__).from_abi::<u16>(result__)
    }
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
    pub unsafe fn GetMaxOutputSampleSize(&self, dwoutput: u32) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).21)(::std::mem::transmute_copy(self), ::std::mem::transmute(dwoutput), &mut result__).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
    pub unsafe fn GetMaxStreamSampleSize(&self, wstream: u16) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).22)(::std::mem::transmute_copy(self), ::std::mem::transmute(wstream), &mut result__).from_abi::<u32>(result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`, `Win32_System_Com`*"]
    pub unsafe fn OpenStream<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::System::Com::IStream>>(&self, pstream: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).23)(::std::mem::transmute_copy(self), pstream.into_param().abi()).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IWMSyncReader {
    type Vtable = IWMSyncReader_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2476208417, 30469, 19913, [176, 73, 152, 182, 152, 24, 132, 20]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMSyncReader_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pwszfilename: super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, cnsstarttime: u64, cnsduration: i64) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, wstreamnum: u16, qwframenumber: u64, cframestoread: i64) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, wstreamnum: u16, ppsample: *mut ::windows::runtime::RawPtr, pcnssampletime: *mut u64, pcnsduration: *mut u64, pdwflags: *mut u32, pdwoutputnum: *mut u32, pwstreamnum: *mut u16) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, cstreamcount: u16, pwstreamnumbers: *const u16, pselections: *const WMT_STREAM_SELECTION) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, wstreamnum: u16, pselection: *mut WMT_STREAM_SELECTION) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, wstreamnum: u16, fcompressed: super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, wstreamnum: u16, pfcompressed: *mut super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwoutputnum: u32, pszname: super::super::Foundation::PWSTR, ptype: *mut WMT_ATTR_DATATYPE, pvalue: *mut u8, pcblength: *mut u16) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwoutputnum: u32, pszname: super::super::Foundation::PWSTR, r#type: WMT_ATTR_DATATYPE, pvalue: *const u8, cblength: u16) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pcoutputs: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwoutputnum: u32, ppoutput: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwoutputnum: u32, poutput: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwoutputnum: u32, pcformats: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwoutputnum: u32, dwformatnum: u32, ppprops: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, wstreamnum: u16, pdwoutputnum: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwoutputnum: u32, pwstreamnum: *mut u16) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwoutput: u32, pcbmax: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, wstream: u16, pcbmax: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pstream: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
);
#[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct IWMSyncReader2(::windows::runtime::IUnknown);
impl IWMSyncReader2 {
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`, `Win32_Foundation`*"]
    pub unsafe fn Open<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, pwszfilename: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), pwszfilename.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
    pub unsafe fn Close(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
    pub unsafe fn SetRange(&self, cnsstarttime: u64, cnsduration: i64) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self), ::std::mem::transmute(cnsstarttime), ::std::mem::transmute(cnsduration)).ok()
    }
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
    pub unsafe fn SetRangeByFrame(&self, wstreamnum: u16, qwframenumber: u64, cframestoread: i64) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::std::mem::transmute_copy(self), ::std::mem::transmute(wstreamnum), ::std::mem::transmute(qwframenumber), ::std::mem::transmute(cframestoread)).ok()
    }
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
    pub unsafe fn GetNextSample(&self, wstreamnum: u16, ppsample: *mut ::std::option::Option<INSSBuffer>, pcnssampletime: *mut u64, pcnsduration: *mut u64, pdwflags: *mut u32, pdwoutputnum: *mut u32, pwstreamnum: *mut u16) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self), ::std::mem::transmute(wstreamnum), ::std::mem::transmute(ppsample), ::std::mem::transmute(pcnssampletime), ::std::mem::transmute(pcnsduration), ::std::mem::transmute(pdwflags), ::std::mem::transmute(pdwoutputnum), ::std::mem::transmute(pwstreamnum)).ok()
    }
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
    pub unsafe fn SetStreamsSelected(&self, cstreamcount: u16, pwstreamnumbers: *const u16, pselections: *const WMT_STREAM_SELECTION) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).8)(::std::mem::transmute_copy(self), ::std::mem::transmute(cstreamcount), ::std::mem::transmute(pwstreamnumbers), ::std::mem::transmute(pselections)).ok()
    }
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
    pub unsafe fn GetStreamSelected(&self, wstreamnum: u16) -> ::windows::runtime::Result<WMT_STREAM_SELECTION> {
        let mut result__: <WMT_STREAM_SELECTION as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).9)(::std::mem::transmute_copy(self), ::std::mem::transmute(wstreamnum), &mut result__).from_abi::<WMT_STREAM_SELECTION>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`, `Win32_Foundation`*"]
    pub unsafe fn SetReadStreamSamples<'a, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>>(&self, wstreamnum: u16, fcompressed: Param1) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).10)(::std::mem::transmute_copy(self), ::std::mem::transmute(wstreamnum), fcompressed.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`, `Win32_Foundation`*"]
    pub unsafe fn GetReadStreamSamples(&self, wstreamnum: u16) -> ::windows::runtime::Result<super::super::Foundation::BOOL> {
        let mut result__: <super::super::Foundation::BOOL as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).11)(::std::mem::transmute_copy(self), ::std::mem::transmute(wstreamnum), &mut result__).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`, `Win32_Foundation`*"]
    pub unsafe fn GetOutputSetting<'a, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, dwoutputnum: u32, pszname: Param1, ptype: *mut WMT_ATTR_DATATYPE, pvalue: *mut u8, pcblength: *mut u16) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).12)(::std::mem::transmute_copy(self), ::std::mem::transmute(dwoutputnum), pszname.into_param().abi(), ::std::mem::transmute(ptype), ::std::mem::transmute(pvalue), ::std::mem::transmute(pcblength)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`, `Win32_Foundation`*"]
    pub unsafe fn SetOutputSetting<'a, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, dwoutputnum: u32, pszname: Param1, r#type: WMT_ATTR_DATATYPE, pvalue: *const u8, cblength: u16) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).13)(::std::mem::transmute_copy(self), ::std::mem::transmute(dwoutputnum), pszname.into_param().abi(), ::std::mem::transmute(r#type), ::std::mem::transmute(pvalue), ::std::mem::transmute(cblength)).ok()
    }
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
    pub unsafe fn GetOutputCount(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).14)(::std::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
    pub unsafe fn GetOutputProps(&self, dwoutputnum: u32) -> ::windows::runtime::Result<IWMOutputMediaProps> {
        let mut result__: <IWMOutputMediaProps as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).15)(::std::mem::transmute_copy(self), ::std::mem::transmute(dwoutputnum), &mut result__).from_abi::<IWMOutputMediaProps>(result__)
    }
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
    pub unsafe fn SetOutputProps<'a, Param1: ::windows::runtime::IntoParam<'a, IWMOutputMediaProps>>(&self, dwoutputnum: u32, poutput: Param1) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).16)(::std::mem::transmute_copy(self), ::std::mem::transmute(dwoutputnum), poutput.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
    pub unsafe fn GetOutputFormatCount(&self, dwoutputnum: u32) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).17)(::std::mem::transmute_copy(self), ::std::mem::transmute(dwoutputnum), &mut result__).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
    pub unsafe fn GetOutputFormat(&self, dwoutputnum: u32, dwformatnum: u32) -> ::windows::runtime::Result<IWMOutputMediaProps> {
        let mut result__: <IWMOutputMediaProps as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).18)(::std::mem::transmute_copy(self), ::std::mem::transmute(dwoutputnum), ::std::mem::transmute(dwformatnum), &mut result__).from_abi::<IWMOutputMediaProps>(result__)
    }
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
    pub unsafe fn GetOutputNumberForStream(&self, wstreamnum: u16) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).19)(::std::mem::transmute_copy(self), ::std::mem::transmute(wstreamnum), &mut result__).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
    pub unsafe fn GetStreamNumberForOutput(&self, dwoutputnum: u32) -> ::windows::runtime::Result<u16> {
        let mut result__: <u16 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).20)(::std::mem::transmute_copy(self), ::std::mem::transmute(dwoutputnum), &mut result__).from_abi::<u16>(result__)
    }
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
    pub unsafe fn GetMaxOutputSampleSize(&self, dwoutput: u32) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).21)(::std::mem::transmute_copy(self), ::std::mem::transmute(dwoutput), &mut result__).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
    pub unsafe fn GetMaxStreamSampleSize(&self, wstream: u16) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).22)(::std::mem::transmute_copy(self), ::std::mem::transmute(wstream), &mut result__).from_abi::<u32>(result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`, `Win32_System_Com`*"]
    pub unsafe fn OpenStream<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::System::Com::IStream>>(&self, pstream: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).23)(::std::mem::transmute_copy(self), pstream.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
    pub unsafe fn SetRangeByTimecode(&self, wstreamnum: u16, pstart: *const WMT_TIMECODE_EXTENSION_DATA, pend: *const WMT_TIMECODE_EXTENSION_DATA) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).24)(::std::mem::transmute_copy(self), ::std::mem::transmute(wstreamnum), ::std::mem::transmute(pstart), ::std::mem::transmute(pend)).ok()
    }
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
    pub unsafe fn SetRangeByFrameEx(&self, wstreamnum: u16, qwframenumber: u64, cframestoread: i64) -> ::windows::runtime::Result<u64> {
        let mut result__: <u64 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).25)(::std::mem::transmute_copy(self), ::std::mem::transmute(wstreamnum), ::std::mem::transmute(qwframenumber), ::std::mem::transmute(cframestoread), &mut result__).from_abi::<u64>(result__)
    }
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
    pub unsafe fn SetAllocateForOutput<'a, Param1: ::windows::runtime::IntoParam<'a, IWMReaderAllocatorEx>>(&self, dwoutputnum: u32, pallocator: Param1) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).26)(::std::mem::transmute_copy(self), ::std::mem::transmute(dwoutputnum), pallocator.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
    pub unsafe fn GetAllocateForOutput(&self, dwoutputnum: u32) -> ::windows::runtime::Result<IWMReaderAllocatorEx> {
        let mut result__: <IWMReaderAllocatorEx as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).27)(::std::mem::transmute_copy(self), ::std::mem::transmute(dwoutputnum), &mut result__).from_abi::<IWMReaderAllocatorEx>(result__)
    }
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
    pub unsafe fn SetAllocateForStream<'a, Param1: ::windows::runtime::IntoParam<'a, IWMReaderAllocatorEx>>(&self, wstreamnum: u16, pallocator: Param1) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).28)(::std::mem::transmute_copy(self), ::std::mem::transmute(wstreamnum), pallocator.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
    pub unsafe fn GetAllocateForStream(&self, dwsreamnum: u16) -> ::windows::runtime::Result<IWMReaderAllocatorEx> {
        let mut result__: <IWMReaderAllocatorEx as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).29)(::std::mem::transmute_copy(self), ::std::mem::transmute(dwsreamnum), &mut result__).from_abi::<IWMReaderAllocatorEx>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IWMSyncReader2 {
    type Vtable = IWMSyncReader2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(4209851681, 7019, 19191, [140, 182, 62, 24, 155, 188, 24, 123]);
}
impl ::std::convert::From<IWMSyncReader2> for IWMSyncReader {
    fn from(value: IWMSyncReader2) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IWMSyncReader2> for IWMSyncReader {
    fn from(value: &IWMSyncReader2) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IWMSyncReader> for IWMSyncReader2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IWMSyncReader> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IWMSyncReader>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IWMSyncReader> for &IWMSyncReader2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IWMSyncReader> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IWMSyncReader>::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMSyncReader2_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pwszfilename: super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, cnsstarttime: u64, cnsduration: i64) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, wstreamnum: u16, qwframenumber: u64, cframestoread: i64) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, wstreamnum: u16, ppsample: *mut ::windows::runtime::RawPtr, pcnssampletime: *mut u64, pcnsduration: *mut u64, pdwflags: *mut u32, pdwoutputnum: *mut u32, pwstreamnum: *mut u16) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, cstreamcount: u16, pwstreamnumbers: *const u16, pselections: *const WMT_STREAM_SELECTION) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, wstreamnum: u16, pselection: *mut WMT_STREAM_SELECTION) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, wstreamnum: u16, fcompressed: super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, wstreamnum: u16, pfcompressed: *mut super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwoutputnum: u32, pszname: super::super::Foundation::PWSTR, ptype: *mut WMT_ATTR_DATATYPE, pvalue: *mut u8, pcblength: *mut u16) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwoutputnum: u32, pszname: super::super::Foundation::PWSTR, r#type: WMT_ATTR_DATATYPE, pvalue: *const u8, cblength: u16) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pcoutputs: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwoutputnum: u32, ppoutput: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwoutputnum: u32, poutput: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwoutputnum: u32, pcformats: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwoutputnum: u32, dwformatnum: u32, ppprops: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, wstreamnum: u16, pdwoutputnum: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwoutputnum: u32, pwstreamnum: *mut u16) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwoutput: u32, pcbmax: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, wstream: u16, pcbmax: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pstream: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, wstreamnum: u16, pstart: *const WMT_TIMECODE_EXTENSION_DATA, pend: *const WMT_TIMECODE_EXTENSION_DATA) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, wstreamnum: u16, qwframenumber: u64, cframestoread: i64, pcnsstarttime: *mut u64) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwoutputnum: u32, pallocator: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwoutputnum: u32, ppallocator: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, wstreamnum: u16, pallocator: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwsreamnum: u16, ppallocator: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct IWMVideoMediaProps(::windows::runtime::IUnknown);
impl IWMVideoMediaProps {
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
    pub unsafe fn GetType(&self) -> ::windows::runtime::Result<::windows::runtime::GUID> {
        let mut result__: <::windows::runtime::GUID as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), &mut result__).from_abi::<::windows::runtime::GUID>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`, `Win32_Foundation`*"]
    pub unsafe fn GetMediaType(&self, ptype: *mut WM_MEDIA_TYPE, pcbtype: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), ::std::mem::transmute(ptype), ::std::mem::transmute(pcbtype)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`, `Win32_Foundation`*"]
    pub unsafe fn SetMediaType(&self, ptype: *const WM_MEDIA_TYPE) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self), ::std::mem::transmute(ptype)).ok()
    }
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
    pub unsafe fn GetMaxKeyFrameSpacing(&self) -> ::windows::runtime::Result<i64> {
        let mut result__: <i64 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).6)(::std::mem::transmute_copy(self), &mut result__).from_abi::<i64>(result__)
    }
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
    pub unsafe fn SetMaxKeyFrameSpacing(&self, lltime: i64) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self), ::std::mem::transmute(lltime)).ok()
    }
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
    pub unsafe fn GetQuality(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).8)(::std::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
    pub unsafe fn SetQuality(&self, dwquality: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).9)(::std::mem::transmute_copy(self), ::std::mem::transmute(dwquality)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IWMVideoMediaProps {
    type Vtable = IWMVideoMediaProps_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2520804303, 11051, 4563, [179, 107, 0, 192, 79, 97, 8, 255]);
}
impl ::std::convert::From<IWMVideoMediaProps> for IWMMediaProps {
    fn from(value: IWMVideoMediaProps) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IWMVideoMediaProps> for IWMMediaProps {
    fn from(value: &IWMVideoMediaProps) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IWMMediaProps> for IWMVideoMediaProps {
    fn into_param(self) -> ::windows::runtime::Param<'a, IWMMediaProps> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IWMMediaProps>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IWMMediaProps> for &IWMVideoMediaProps {
    fn into_param(self) -> ::windows::runtime::Param<'a, IWMMediaProps> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IWMMediaProps>::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMVideoMediaProps_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pguidtype: *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ptype: *mut ::std::mem::ManuallyDrop<WM_MEDIA_TYPE>, pcbtype: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ptype: *const ::std::mem::ManuallyDrop<WM_MEDIA_TYPE>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, plltime: *mut i64) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, lltime: i64) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pdwquality: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwquality: u32) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct IWMWatermarkInfo(::windows::runtime::IUnknown);
impl IWMWatermarkInfo {
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
    pub unsafe fn GetWatermarkEntryCount(&self, wmettype: WMT_WATERMARK_ENTRY_TYPE) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), ::std::mem::transmute(wmettype), &mut result__).from_abi::<u32>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`, `Win32_Foundation`*"]
    pub unsafe fn GetWatermarkEntry(&self, wmettype: WMT_WATERMARK_ENTRY_TYPE, dwentrynum: u32) -> ::windows::runtime::Result<WMT_WATERMARK_ENTRY> {
        let mut result__: <WMT_WATERMARK_ENTRY as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), ::std::mem::transmute(wmettype), ::std::mem::transmute(dwentrynum), &mut result__).from_abi::<WMT_WATERMARK_ENTRY>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IWMWatermarkInfo {
    type Vtable = IWMWatermarkInfo_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1867083874, 62178, 17956, [142, 167, 157, 212, 13, 129, 252, 141]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMWatermarkInfo_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, wmettype: WMT_WATERMARK_ENTRY_TYPE, pdwcount: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, wmettype: WMT_WATERMARK_ENTRY_TYPE, dwentrynum: u32, pentry: *mut WMT_WATERMARK_ENTRY) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct IWMWriter(::windows::runtime::IUnknown);
impl IWMWriter {
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
    pub unsafe fn SetProfileByID(&self, guidprofile: *const ::windows::runtime::GUID) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), ::std::mem::transmute(guidprofile)).ok()
    }
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
    pub unsafe fn SetProfile<'a, Param0: ::windows::runtime::IntoParam<'a, IWMProfile>>(&self, pprofile: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), pprofile.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`, `Win32_Foundation`*"]
    pub unsafe fn SetOutputFilename<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, pwszfilename: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self), pwszfilename.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
    pub unsafe fn GetInputCount(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).6)(::std::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
    pub unsafe fn GetInputProps(&self, dwinputnum: u32) -> ::windows::runtime::Result<IWMInputMediaProps> {
        let mut result__: <IWMInputMediaProps as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self), ::std::mem::transmute(dwinputnum), &mut result__).from_abi::<IWMInputMediaProps>(result__)
    }
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
    pub unsafe fn SetInputProps<'a, Param1: ::windows::runtime::IntoParam<'a, IWMInputMediaProps>>(&self, dwinputnum: u32, pinput: Param1) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).8)(::std::mem::transmute_copy(self), ::std::mem::transmute(dwinputnum), pinput.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
    pub unsafe fn GetInputFormatCount(&self, dwinputnumber: u32) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).9)(::std::mem::transmute_copy(self), ::std::mem::transmute(dwinputnumber), &mut result__).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
    pub unsafe fn GetInputFormat(&self, dwinputnumber: u32, dwformatnumber: u32) -> ::windows::runtime::Result<IWMInputMediaProps> {
        let mut result__: <IWMInputMediaProps as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).10)(::std::mem::transmute_copy(self), ::std::mem::transmute(dwinputnumber), ::std::mem::transmute(dwformatnumber), &mut result__).from_abi::<IWMInputMediaProps>(result__)
    }
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
    pub unsafe fn BeginWriting(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).11)(::std::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
    pub unsafe fn EndWriting(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).12)(::std::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
    pub unsafe fn AllocateSample(&self, dwsamplesize: u32) -> ::windows::runtime::Result<INSSBuffer> {
        let mut result__: <INSSBuffer as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).13)(::std::mem::transmute_copy(self), ::std::mem::transmute(dwsamplesize), &mut result__).from_abi::<INSSBuffer>(result__)
    }
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
    pub unsafe fn WriteSample<'a, Param3: ::windows::runtime::IntoParam<'a, INSSBuffer>>(&self, dwinputnum: u32, cnssampletime: u64, dwflags: u32, psample: Param3) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).14)(::std::mem::transmute_copy(self), ::std::mem::transmute(dwinputnum), ::std::mem::transmute(cnssampletime), ::std::mem::transmute(dwflags), psample.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
    pub unsafe fn Flush(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).15)(::std::mem::transmute_copy(self)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IWMWriter {
    type Vtable = IWMWriter_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2520804308, 11051, 4563, [179, 107, 0, 192, 79, 97, 8, 255]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMWriter_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, guidprofile: *const ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pprofile: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pwszfilename: super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pcinputs: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwinputnum: u32, ppinput: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwinputnum: u32, pinput: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwinputnumber: u32, pcformats: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwinputnumber: u32, dwformatnumber: u32, pprops: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwsamplesize: u32, ppsample: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwinputnum: u32, cnssampletime: u64, dwflags: u32, psample: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct IWMWriterAdvanced(::windows::runtime::IUnknown);
impl IWMWriterAdvanced {
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
    pub unsafe fn GetSinkCount(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
    pub unsafe fn GetSink(&self, dwsinknum: u32) -> ::windows::runtime::Result<IWMWriterSink> {
        let mut result__: <IWMWriterSink as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), ::std::mem::transmute(dwsinknum), &mut result__).from_abi::<IWMWriterSink>(result__)
    }
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
    pub unsafe fn AddSink<'a, Param0: ::windows::runtime::IntoParam<'a, IWMWriterSink>>(&self, psink: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self), psink.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
    pub unsafe fn RemoveSink<'a, Param0: ::windows::runtime::IntoParam<'a, IWMWriterSink>>(&self, psink: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::std::mem::transmute_copy(self), psink.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
    pub unsafe fn WriteStreamSample<'a, Param5: ::windows::runtime::IntoParam<'a, INSSBuffer>>(&self, wstreamnum: u16, cnssampletime: u64, mssamplesendtime: u32, cnssampleduration: u64, dwflags: u32, psample: Param5) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self), ::std::mem::transmute(wstreamnum), ::std::mem::transmute(cnssampletime), ::std::mem::transmute(mssamplesendtime), ::std::mem::transmute(cnssampleduration), ::std::mem::transmute(dwflags), psample.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`, `Win32_Foundation`*"]
    pub unsafe fn SetLiveSource<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>>(&self, fislivesource: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).8)(::std::mem::transmute_copy(self), fislivesource.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`, `Win32_Foundation`*"]
    pub unsafe fn IsRealTime(&self) -> ::windows::runtime::Result<super::super::Foundation::BOOL> {
        let mut result__: <super::super::Foundation::BOOL as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).9)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
    pub unsafe fn GetWriterTime(&self) -> ::windows::runtime::Result<u64> {
        let mut result__: <u64 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).10)(::std::mem::transmute_copy(self), &mut result__).from_abi::<u64>(result__)
    }
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
    pub unsafe fn GetStatistics(&self, wstreamnum: u16) -> ::windows::runtime::Result<WM_WRITER_STATISTICS> {
        let mut result__: <WM_WRITER_STATISTICS as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).11)(::std::mem::transmute_copy(self), ::std::mem::transmute(wstreamnum), &mut result__).from_abi::<WM_WRITER_STATISTICS>(result__)
    }
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
    pub unsafe fn SetSyncTolerance(&self, mswindow: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).12)(::std::mem::transmute_copy(self), ::std::mem::transmute(mswindow)).ok()
    }
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
    pub unsafe fn GetSyncTolerance(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).13)(::std::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IWMWriterAdvanced {
    type Vtable = IWMWriterAdvanced_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2520804323, 11051, 4563, [179, 107, 0, 192, 79, 97, 8, 255]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMWriterAdvanced_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pcsinks: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwsinknum: u32, ppsink: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, psink: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, psink: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, wstreamnum: u16, cnssampletime: u64, mssamplesendtime: u32, cnssampleduration: u64, dwflags: u32, psample: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, fislivesource: super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pfrealtime: *mut super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pcnscurrenttime: *mut u64) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, wstreamnum: u16, pstats: *mut WM_WRITER_STATISTICS) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, mswindow: u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pmswindow: *mut u32) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct IWMWriterAdvanced2(::windows::runtime::IUnknown);
impl IWMWriterAdvanced2 {
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
    pub unsafe fn GetSinkCount(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
    pub unsafe fn GetSink(&self, dwsinknum: u32) -> ::windows::runtime::Result<IWMWriterSink> {
        let mut result__: <IWMWriterSink as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), ::std::mem::transmute(dwsinknum), &mut result__).from_abi::<IWMWriterSink>(result__)
    }
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
    pub unsafe fn AddSink<'a, Param0: ::windows::runtime::IntoParam<'a, IWMWriterSink>>(&self, psink: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self), psink.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
    pub unsafe fn RemoveSink<'a, Param0: ::windows::runtime::IntoParam<'a, IWMWriterSink>>(&self, psink: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::std::mem::transmute_copy(self), psink.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
    pub unsafe fn WriteStreamSample<'a, Param5: ::windows::runtime::IntoParam<'a, INSSBuffer>>(&self, wstreamnum: u16, cnssampletime: u64, mssamplesendtime: u32, cnssampleduration: u64, dwflags: u32, psample: Param5) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self), ::std::mem::transmute(wstreamnum), ::std::mem::transmute(cnssampletime), ::std::mem::transmute(mssamplesendtime), ::std::mem::transmute(cnssampleduration), ::std::mem::transmute(dwflags), psample.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`, `Win32_Foundation`*"]
    pub unsafe fn SetLiveSource<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>>(&self, fislivesource: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).8)(::std::mem::transmute_copy(self), fislivesource.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`, `Win32_Foundation`*"]
    pub unsafe fn IsRealTime(&self) -> ::windows::runtime::Result<super::super::Foundation::BOOL> {
        let mut result__: <super::super::Foundation::BOOL as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).9)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
    pub unsafe fn GetWriterTime(&self) -> ::windows::runtime::Result<u64> {
        let mut result__: <u64 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).10)(::std::mem::transmute_copy(self), &mut result__).from_abi::<u64>(result__)
    }
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
    pub unsafe fn GetStatistics(&self, wstreamnum: u16) -> ::windows::runtime::Result<WM_WRITER_STATISTICS> {
        let mut result__: <WM_WRITER_STATISTICS as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).11)(::std::mem::transmute_copy(self), ::std::mem::transmute(wstreamnum), &mut result__).from_abi::<WM_WRITER_STATISTICS>(result__)
    }
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
    pub unsafe fn SetSyncTolerance(&self, mswindow: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).12)(::std::mem::transmute_copy(self), ::std::mem::transmute(mswindow)).ok()
    }
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
    pub unsafe fn GetSyncTolerance(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).13)(::std::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`, `Win32_Foundation`*"]
    pub unsafe fn GetInputSetting<'a, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, dwinputnum: u32, pszname: Param1, ptype: *mut WMT_ATTR_DATATYPE, pvalue: *mut u8, pcblength: *mut u16) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).14)(::std::mem::transmute_copy(self), ::std::mem::transmute(dwinputnum), pszname.into_param().abi(), ::std::mem::transmute(ptype), ::std::mem::transmute(pvalue), ::std::mem::transmute(pcblength)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`, `Win32_Foundation`*"]
    pub unsafe fn SetInputSetting<'a, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, dwinputnum: u32, pszname: Param1, r#type: WMT_ATTR_DATATYPE, pvalue: *const u8, cblength: u16) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).15)(::std::mem::transmute_copy(self), ::std::mem::transmute(dwinputnum), pszname.into_param().abi(), ::std::mem::transmute(r#type), ::std::mem::transmute(pvalue), ::std::mem::transmute(cblength)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IWMWriterAdvanced2 {
    type Vtable = IWMWriterAdvanced2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2519581164, 49222, 19896, [156, 199, 38, 206, 174, 80, 8, 23]);
}
impl ::std::convert::From<IWMWriterAdvanced2> for IWMWriterAdvanced {
    fn from(value: IWMWriterAdvanced2) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IWMWriterAdvanced2> for IWMWriterAdvanced {
    fn from(value: &IWMWriterAdvanced2) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IWMWriterAdvanced> for IWMWriterAdvanced2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IWMWriterAdvanced> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IWMWriterAdvanced>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IWMWriterAdvanced> for &IWMWriterAdvanced2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IWMWriterAdvanced> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IWMWriterAdvanced>::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMWriterAdvanced2_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pcsinks: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwsinknum: u32, ppsink: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, psink: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, psink: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, wstreamnum: u16, cnssampletime: u64, mssamplesendtime: u32, cnssampleduration: u64, dwflags: u32, psample: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, fislivesource: super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pfrealtime: *mut super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pcnscurrenttime: *mut u64) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, wstreamnum: u16, pstats: *mut WM_WRITER_STATISTICS) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, mswindow: u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pmswindow: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwinputnum: u32, pszname: super::super::Foundation::PWSTR, ptype: *mut WMT_ATTR_DATATYPE, pvalue: *mut u8, pcblength: *mut u16) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwinputnum: u32, pszname: super::super::Foundation::PWSTR, r#type: WMT_ATTR_DATATYPE, pvalue: *const u8, cblength: u16) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct IWMWriterAdvanced3(::windows::runtime::IUnknown);
impl IWMWriterAdvanced3 {
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
    pub unsafe fn GetSinkCount(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
    pub unsafe fn GetSink(&self, dwsinknum: u32) -> ::windows::runtime::Result<IWMWriterSink> {
        let mut result__: <IWMWriterSink as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), ::std::mem::transmute(dwsinknum), &mut result__).from_abi::<IWMWriterSink>(result__)
    }
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
    pub unsafe fn AddSink<'a, Param0: ::windows::runtime::IntoParam<'a, IWMWriterSink>>(&self, psink: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self), psink.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
    pub unsafe fn RemoveSink<'a, Param0: ::windows::runtime::IntoParam<'a, IWMWriterSink>>(&self, psink: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::std::mem::transmute_copy(self), psink.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
    pub unsafe fn WriteStreamSample<'a, Param5: ::windows::runtime::IntoParam<'a, INSSBuffer>>(&self, wstreamnum: u16, cnssampletime: u64, mssamplesendtime: u32, cnssampleduration: u64, dwflags: u32, psample: Param5) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self), ::std::mem::transmute(wstreamnum), ::std::mem::transmute(cnssampletime), ::std::mem::transmute(mssamplesendtime), ::std::mem::transmute(cnssampleduration), ::std::mem::transmute(dwflags), psample.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`, `Win32_Foundation`*"]
    pub unsafe fn SetLiveSource<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>>(&self, fislivesource: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).8)(::std::mem::transmute_copy(self), fislivesource.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`, `Win32_Foundation`*"]
    pub unsafe fn IsRealTime(&self) -> ::windows::runtime::Result<super::super::Foundation::BOOL> {
        let mut result__: <super::super::Foundation::BOOL as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).9)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
    pub unsafe fn GetWriterTime(&self) -> ::windows::runtime::Result<u64> {
        let mut result__: <u64 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).10)(::std::mem::transmute_copy(self), &mut result__).from_abi::<u64>(result__)
    }
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
    pub unsafe fn GetStatistics(&self, wstreamnum: u16) -> ::windows::runtime::Result<WM_WRITER_STATISTICS> {
        let mut result__: <WM_WRITER_STATISTICS as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).11)(::std::mem::transmute_copy(self), ::std::mem::transmute(wstreamnum), &mut result__).from_abi::<WM_WRITER_STATISTICS>(result__)
    }
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
    pub unsafe fn SetSyncTolerance(&self, mswindow: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).12)(::std::mem::transmute_copy(self), ::std::mem::transmute(mswindow)).ok()
    }
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
    pub unsafe fn GetSyncTolerance(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).13)(::std::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`, `Win32_Foundation`*"]
    pub unsafe fn GetInputSetting<'a, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, dwinputnum: u32, pszname: Param1, ptype: *mut WMT_ATTR_DATATYPE, pvalue: *mut u8, pcblength: *mut u16) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).14)(::std::mem::transmute_copy(self), ::std::mem::transmute(dwinputnum), pszname.into_param().abi(), ::std::mem::transmute(ptype), ::std::mem::transmute(pvalue), ::std::mem::transmute(pcblength)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`, `Win32_Foundation`*"]
    pub unsafe fn SetInputSetting<'a, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, dwinputnum: u32, pszname: Param1, r#type: WMT_ATTR_DATATYPE, pvalue: *const u8, cblength: u16) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).15)(::std::mem::transmute_copy(self), ::std::mem::transmute(dwinputnum), pszname.into_param().abi(), ::std::mem::transmute(r#type), ::std::mem::transmute(pvalue), ::std::mem::transmute(cblength)).ok()
    }
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
    pub unsafe fn GetStatisticsEx(&self, wstreamnum: u16) -> ::windows::runtime::Result<WM_WRITER_STATISTICS_EX> {
        let mut result__: <WM_WRITER_STATISTICS_EX as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).16)(::std::mem::transmute_copy(self), ::std::mem::transmute(wstreamnum), &mut result__).from_abi::<WM_WRITER_STATISTICS_EX>(result__)
    }
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
    pub unsafe fn SetNonBlocking(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).17)(::std::mem::transmute_copy(self)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IWMWriterAdvanced3 {
    type Vtable = IWMWriterAdvanced3_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(752240941, 31799, 20086, [157, 59, 89, 38, 17, 131, 162, 46]);
}
impl ::std::convert::From<IWMWriterAdvanced3> for IWMWriterAdvanced2 {
    fn from(value: IWMWriterAdvanced3) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IWMWriterAdvanced3> for IWMWriterAdvanced2 {
    fn from(value: &IWMWriterAdvanced3) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IWMWriterAdvanced2> for IWMWriterAdvanced3 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IWMWriterAdvanced2> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IWMWriterAdvanced2>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IWMWriterAdvanced2> for &IWMWriterAdvanced3 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IWMWriterAdvanced2> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IWMWriterAdvanced2>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<IWMWriterAdvanced3> for IWMWriterAdvanced {
    fn from(value: IWMWriterAdvanced3) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IWMWriterAdvanced3> for IWMWriterAdvanced {
    fn from(value: &IWMWriterAdvanced3) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IWMWriterAdvanced> for IWMWriterAdvanced3 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IWMWriterAdvanced> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IWMWriterAdvanced>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IWMWriterAdvanced> for &IWMWriterAdvanced3 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IWMWriterAdvanced> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IWMWriterAdvanced>::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMWriterAdvanced3_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pcsinks: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwsinknum: u32, ppsink: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, psink: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, psink: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, wstreamnum: u16, cnssampletime: u64, mssamplesendtime: u32, cnssampleduration: u64, dwflags: u32, psample: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, fislivesource: super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pfrealtime: *mut super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pcnscurrenttime: *mut u64) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, wstreamnum: u16, pstats: *mut WM_WRITER_STATISTICS) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, mswindow: u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pmswindow: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwinputnum: u32, pszname: super::super::Foundation::PWSTR, ptype: *mut WMT_ATTR_DATATYPE, pvalue: *mut u8, pcblength: *mut u16) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwinputnum: u32, pszname: super::super::Foundation::PWSTR, r#type: WMT_ATTR_DATATYPE, pvalue: *const u8, cblength: u16) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, wstreamnum: u16, pstats: *mut WM_WRITER_STATISTICS_EX) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct IWMWriterFileSink(::windows::runtime::IUnknown);
impl IWMWriterFileSink {
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
    pub unsafe fn OnHeader<'a, Param0: ::windows::runtime::IntoParam<'a, INSSBuffer>>(&self, pheader: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), pheader.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`, `Win32_Foundation`*"]
    pub unsafe fn IsRealTime(&self) -> ::windows::runtime::Result<super::super::Foundation::BOOL> {
        let mut result__: <super::super::Foundation::BOOL as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
    pub unsafe fn AllocateDataUnit(&self, cbdataunit: u32) -> ::windows::runtime::Result<INSSBuffer> {
        let mut result__: <INSSBuffer as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self), ::std::mem::transmute(cbdataunit), &mut result__).from_abi::<INSSBuffer>(result__)
    }
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
    pub unsafe fn OnDataUnit<'a, Param0: ::windows::runtime::IntoParam<'a, INSSBuffer>>(&self, pdataunit: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::std::mem::transmute_copy(self), pdataunit.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
    pub unsafe fn OnEndWriting(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`, `Win32_Foundation`*"]
    pub unsafe fn Open<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, pwszfilename: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).8)(::std::mem::transmute_copy(self), pwszfilename.into_param().abi()).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IWMWriterFileSink {
    type Vtable = IWMWriterFileSink_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2520804325, 11051, 4563, [179, 107, 0, 192, 79, 97, 8, 255]);
}
impl ::std::convert::From<IWMWriterFileSink> for IWMWriterSink {
    fn from(value: IWMWriterFileSink) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IWMWriterFileSink> for IWMWriterSink {
    fn from(value: &IWMWriterFileSink) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IWMWriterSink> for IWMWriterFileSink {
    fn into_param(self) -> ::windows::runtime::Param<'a, IWMWriterSink> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IWMWriterSink>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IWMWriterSink> for &IWMWriterFileSink {
    fn into_param(self) -> ::windows::runtime::Param<'a, IWMWriterSink> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IWMWriterSink>::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMWriterFileSink_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pheader: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pfrealtime: *mut super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, cbdataunit: u32, ppdataunit: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pdataunit: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pwszfilename: super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct IWMWriterFileSink2(::windows::runtime::IUnknown);
impl IWMWriterFileSink2 {
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
    pub unsafe fn OnHeader<'a, Param0: ::windows::runtime::IntoParam<'a, INSSBuffer>>(&self, pheader: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), pheader.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`, `Win32_Foundation`*"]
    pub unsafe fn IsRealTime(&self) -> ::windows::runtime::Result<super::super::Foundation::BOOL> {
        let mut result__: <super::super::Foundation::BOOL as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
    pub unsafe fn AllocateDataUnit(&self, cbdataunit: u32) -> ::windows::runtime::Result<INSSBuffer> {
        let mut result__: <INSSBuffer as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self), ::std::mem::transmute(cbdataunit), &mut result__).from_abi::<INSSBuffer>(result__)
    }
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
    pub unsafe fn OnDataUnit<'a, Param0: ::windows::runtime::IntoParam<'a, INSSBuffer>>(&self, pdataunit: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::std::mem::transmute_copy(self), pdataunit.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
    pub unsafe fn OnEndWriting(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`, `Win32_Foundation`*"]
    pub unsafe fn Open<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, pwszfilename: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).8)(::std::mem::transmute_copy(self), pwszfilename.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
    pub unsafe fn Start(&self, cnsstarttime: u64) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).9)(::std::mem::transmute_copy(self), ::std::mem::transmute(cnsstarttime)).ok()
    }
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
    pub unsafe fn Stop(&self, cnsstoptime: u64) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).10)(::std::mem::transmute_copy(self), ::std::mem::transmute(cnsstoptime)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`, `Win32_Foundation`*"]
    pub unsafe fn IsStopped(&self) -> ::windows::runtime::Result<super::super::Foundation::BOOL> {
        let mut result__: <super::super::Foundation::BOOL as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).11)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
    pub unsafe fn GetFileDuration(&self) -> ::windows::runtime::Result<u64> {
        let mut result__: <u64 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).12)(::std::mem::transmute_copy(self), &mut result__).from_abi::<u64>(result__)
    }
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
    pub unsafe fn GetFileSize(&self) -> ::windows::runtime::Result<u64> {
        let mut result__: <u64 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).13)(::std::mem::transmute_copy(self), &mut result__).from_abi::<u64>(result__)
    }
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
    pub unsafe fn Close(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).14)(::std::mem::transmute_copy(self)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`, `Win32_Foundation`*"]
    pub unsafe fn IsClosed(&self) -> ::windows::runtime::Result<super::super::Foundation::BOOL> {
        let mut result__: <super::super::Foundation::BOOL as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).15)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BOOL>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IWMWriterFileSink2 {
    type Vtable = IWMWriterFileSink2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(338176935, 19183, 16901, [140, 229, 194, 41, 3, 90, 5, 188]);
}
impl ::std::convert::From<IWMWriterFileSink2> for IWMWriterFileSink {
    fn from(value: IWMWriterFileSink2) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IWMWriterFileSink2> for IWMWriterFileSink {
    fn from(value: &IWMWriterFileSink2) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IWMWriterFileSink> for IWMWriterFileSink2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IWMWriterFileSink> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IWMWriterFileSink>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IWMWriterFileSink> for &IWMWriterFileSink2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IWMWriterFileSink> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IWMWriterFileSink>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<IWMWriterFileSink2> for IWMWriterSink {
    fn from(value: IWMWriterFileSink2) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IWMWriterFileSink2> for IWMWriterSink {
    fn from(value: &IWMWriterFileSink2) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IWMWriterSink> for IWMWriterFileSink2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IWMWriterSink> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IWMWriterSink>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IWMWriterSink> for &IWMWriterFileSink2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IWMWriterSink> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IWMWriterSink>::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMWriterFileSink2_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pheader: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pfrealtime: *mut super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, cbdataunit: u32, ppdataunit: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pdataunit: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pwszfilename: super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, cnsstarttime: u64) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, cnsstoptime: u64) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pfstopped: *mut super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pcnsduration: *mut u64) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pcbfile: *mut u64) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pfclosed: *mut super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct IWMWriterFileSink3(::windows::runtime::IUnknown);
impl IWMWriterFileSink3 {
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
    pub unsafe fn OnHeader<'a, Param0: ::windows::runtime::IntoParam<'a, INSSBuffer>>(&self, pheader: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), pheader.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`, `Win32_Foundation`*"]
    pub unsafe fn IsRealTime(&self) -> ::windows::runtime::Result<super::super::Foundation::BOOL> {
        let mut result__: <super::super::Foundation::BOOL as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
    pub unsafe fn AllocateDataUnit(&self, cbdataunit: u32) -> ::windows::runtime::Result<INSSBuffer> {
        let mut result__: <INSSBuffer as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self), ::std::mem::transmute(cbdataunit), &mut result__).from_abi::<INSSBuffer>(result__)
    }
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
    pub unsafe fn OnDataUnit<'a, Param0: ::windows::runtime::IntoParam<'a, INSSBuffer>>(&self, pdataunit: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::std::mem::transmute_copy(self), pdataunit.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
    pub unsafe fn OnEndWriting(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`, `Win32_Foundation`*"]
    pub unsafe fn Open<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, pwszfilename: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).8)(::std::mem::transmute_copy(self), pwszfilename.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
    pub unsafe fn Start(&self, cnsstarttime: u64) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).9)(::std::mem::transmute_copy(self), ::std::mem::transmute(cnsstarttime)).ok()
    }
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
    pub unsafe fn Stop(&self, cnsstoptime: u64) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).10)(::std::mem::transmute_copy(self), ::std::mem::transmute(cnsstoptime)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`, `Win32_Foundation`*"]
    pub unsafe fn IsStopped(&self) -> ::windows::runtime::Result<super::super::Foundation::BOOL> {
        let mut result__: <super::super::Foundation::BOOL as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).11)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
    pub unsafe fn GetFileDuration(&self) -> ::windows::runtime::Result<u64> {
        let mut result__: <u64 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).12)(::std::mem::transmute_copy(self), &mut result__).from_abi::<u64>(result__)
    }
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
    pub unsafe fn GetFileSize(&self) -> ::windows::runtime::Result<u64> {
        let mut result__: <u64 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).13)(::std::mem::transmute_copy(self), &mut result__).from_abi::<u64>(result__)
    }
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
    pub unsafe fn Close(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).14)(::std::mem::transmute_copy(self)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`, `Win32_Foundation`*"]
    pub unsafe fn IsClosed(&self) -> ::windows::runtime::Result<super::super::Foundation::BOOL> {
        let mut result__: <super::super::Foundation::BOOL as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).15)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`, `Win32_Foundation`*"]
    pub unsafe fn SetAutoIndexing<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>>(&self, fdoautoindexing: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).16)(::std::mem::transmute_copy(self), fdoautoindexing.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`, `Win32_Foundation`*"]
    pub unsafe fn GetAutoIndexing(&self) -> ::windows::runtime::Result<super::super::Foundation::BOOL> {
        let mut result__: <super::super::Foundation::BOOL as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).17)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`, `Win32_Foundation`*"]
    pub unsafe fn SetControlStream<'a, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>>(&self, wstreamnumber: u16, fshouldcontrolstartandstop: Param1) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).18)(::std::mem::transmute_copy(self), ::std::mem::transmute(wstreamnumber), fshouldcontrolstartandstop.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
    pub unsafe fn GetMode(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).19)(::std::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
    pub unsafe fn OnDataUnitEx(&self, pfilesinkdataunit: *const WMT_FILESINK_DATA_UNIT) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).20)(::std::mem::transmute_copy(self), ::std::mem::transmute(pfilesinkdataunit)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`, `Win32_Foundation`*"]
    pub unsafe fn SetUnbufferedIO<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>>(&self, funbufferedio: Param0, frestrictmemusage: Param1) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).21)(::std::mem::transmute_copy(self), funbufferedio.into_param().abi(), frestrictmemusage.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`, `Win32_Foundation`*"]
    pub unsafe fn GetUnbufferedIO(&self) -> ::windows::runtime::Result<super::super::Foundation::BOOL> {
        let mut result__: <super::super::Foundation::BOOL as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).22)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
    pub unsafe fn CompleteOperations(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).23)(::std::mem::transmute_copy(self)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IWMWriterFileSink3 {
    type Vtable = IWMWriterFileSink3_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1072320491, 10565, 18343, [161, 221, 197, 58, 143, 196, 196, 92]);
}
impl ::std::convert::From<IWMWriterFileSink3> for IWMWriterFileSink2 {
    fn from(value: IWMWriterFileSink3) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IWMWriterFileSink3> for IWMWriterFileSink2 {
    fn from(value: &IWMWriterFileSink3) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IWMWriterFileSink2> for IWMWriterFileSink3 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IWMWriterFileSink2> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IWMWriterFileSink2>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IWMWriterFileSink2> for &IWMWriterFileSink3 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IWMWriterFileSink2> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IWMWriterFileSink2>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<IWMWriterFileSink3> for IWMWriterFileSink {
    fn from(value: IWMWriterFileSink3) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IWMWriterFileSink3> for IWMWriterFileSink {
    fn from(value: &IWMWriterFileSink3) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IWMWriterFileSink> for IWMWriterFileSink3 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IWMWriterFileSink> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IWMWriterFileSink>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IWMWriterFileSink> for &IWMWriterFileSink3 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IWMWriterFileSink> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IWMWriterFileSink>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<IWMWriterFileSink3> for IWMWriterSink {
    fn from(value: IWMWriterFileSink3) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IWMWriterFileSink3> for IWMWriterSink {
    fn from(value: &IWMWriterFileSink3) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IWMWriterSink> for IWMWriterFileSink3 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IWMWriterSink> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IWMWriterSink>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IWMWriterSink> for &IWMWriterFileSink3 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IWMWriterSink> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IWMWriterSink>::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMWriterFileSink3_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pheader: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pfrealtime: *mut super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, cbdataunit: u32, ppdataunit: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pdataunit: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pwszfilename: super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, cnsstarttime: u64) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, cnsstoptime: u64) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pfstopped: *mut super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pcnsduration: *mut u64) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pcbfile: *mut u64) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pfclosed: *mut super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, fdoautoindexing: super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pfautoindexing: *mut super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, wstreamnumber: u16, fshouldcontrolstartandstop: super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pdwfilesinkmode: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pfilesinkdataunit: *const ::std::mem::ManuallyDrop<WMT_FILESINK_DATA_UNIT>) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, funbufferedio: super::super::Foundation::BOOL, frestrictmemusage: super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pfunbufferedio: *mut super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct IWMWriterNetworkSink(::windows::runtime::IUnknown);
impl IWMWriterNetworkSink {
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
    pub unsafe fn OnHeader<'a, Param0: ::windows::runtime::IntoParam<'a, INSSBuffer>>(&self, pheader: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), pheader.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`, `Win32_Foundation`*"]
    pub unsafe fn IsRealTime(&self) -> ::windows::runtime::Result<super::super::Foundation::BOOL> {
        let mut result__: <super::super::Foundation::BOOL as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
    pub unsafe fn AllocateDataUnit(&self, cbdataunit: u32) -> ::windows::runtime::Result<INSSBuffer> {
        let mut result__: <INSSBuffer as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self), ::std::mem::transmute(cbdataunit), &mut result__).from_abi::<INSSBuffer>(result__)
    }
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
    pub unsafe fn OnDataUnit<'a, Param0: ::windows::runtime::IntoParam<'a, INSSBuffer>>(&self, pdataunit: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::std::mem::transmute_copy(self), pdataunit.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
    pub unsafe fn OnEndWriting(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
    pub unsafe fn SetMaximumClients(&self, dwmaxclients: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).8)(::std::mem::transmute_copy(self), ::std::mem::transmute(dwmaxclients)).ok()
    }
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
    pub unsafe fn GetMaximumClients(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).9)(::std::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
    pub unsafe fn SetNetworkProtocol(&self, protocol: WMT_NET_PROTOCOL) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).10)(::std::mem::transmute_copy(self), ::std::mem::transmute(protocol)).ok()
    }
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
    pub unsafe fn GetNetworkProtocol(&self) -> ::windows::runtime::Result<WMT_NET_PROTOCOL> {
        let mut result__: <WMT_NET_PROTOCOL as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).11)(::std::mem::transmute_copy(self), &mut result__).from_abi::<WMT_NET_PROTOCOL>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`, `Win32_Foundation`*"]
    pub unsafe fn GetHostURL(&self, pwszurl: super::super::Foundation::PWSTR, pcchurl: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).12)(::std::mem::transmute_copy(self), ::std::mem::transmute(pwszurl), ::std::mem::transmute(pcchurl)).ok()
    }
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
    pub unsafe fn Open(&self, pdwportnum: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).13)(::std::mem::transmute_copy(self), ::std::mem::transmute(pdwportnum)).ok()
    }
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
    pub unsafe fn Disconnect(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).14)(::std::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
    pub unsafe fn Close(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).15)(::std::mem::transmute_copy(self)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IWMWriterNetworkSink {
    type Vtable = IWMWriterNetworkSink_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2520804327, 11051, 4563, [179, 107, 0, 192, 79, 97, 8, 255]);
}
impl ::std::convert::From<IWMWriterNetworkSink> for IWMWriterSink {
    fn from(value: IWMWriterNetworkSink) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IWMWriterNetworkSink> for IWMWriterSink {
    fn from(value: &IWMWriterNetworkSink) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IWMWriterSink> for IWMWriterNetworkSink {
    fn into_param(self) -> ::windows::runtime::Param<'a, IWMWriterSink> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IWMWriterSink>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IWMWriterSink> for &IWMWriterNetworkSink {
    fn into_param(self) -> ::windows::runtime::Param<'a, IWMWriterSink> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IWMWriterSink>::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMWriterNetworkSink_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pheader: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pfrealtime: *mut super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, cbdataunit: u32, ppdataunit: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pdataunit: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwmaxclients: u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pdwmaxclients: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, protocol: WMT_NET_PROTOCOL) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pprotocol: *mut WMT_NET_PROTOCOL) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pwszurl: super::super::Foundation::PWSTR, pcchurl: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pdwportnum: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct IWMWriterPostView(::windows::runtime::IUnknown);
impl IWMWriterPostView {
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
    pub unsafe fn SetPostViewCallback<'a, Param0: ::windows::runtime::IntoParam<'a, IWMWriterPostViewCallback>>(&self, pcallback: Param0, pvcontext: *mut ::std::ffi::c_void) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), pcallback.into_param().abi(), ::std::mem::transmute(pvcontext)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`, `Win32_Foundation`*"]
    pub unsafe fn SetReceivePostViewSamples<'a, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>>(&self, wstreamnum: u16, freceivepostviewsamples: Param1) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), ::std::mem::transmute(wstreamnum), freceivepostviewsamples.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`, `Win32_Foundation`*"]
    pub unsafe fn GetReceivePostViewSamples(&self, wstreamnum: u16) -> ::windows::runtime::Result<super::super::Foundation::BOOL> {
        let mut result__: <super::super::Foundation::BOOL as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self), ::std::mem::transmute(wstreamnum), &mut result__).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
    pub unsafe fn GetPostViewProps(&self, wstreamnumber: u16) -> ::windows::runtime::Result<IWMMediaProps> {
        let mut result__: <IWMMediaProps as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).6)(::std::mem::transmute_copy(self), ::std::mem::transmute(wstreamnumber), &mut result__).from_abi::<IWMMediaProps>(result__)
    }
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
    pub unsafe fn SetPostViewProps<'a, Param1: ::windows::runtime::IntoParam<'a, IWMMediaProps>>(&self, wstreamnumber: u16, poutput: Param1) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self), ::std::mem::transmute(wstreamnumber), poutput.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
    pub unsafe fn GetPostViewFormatCount(&self, wstreamnumber: u16) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).8)(::std::mem::transmute_copy(self), ::std::mem::transmute(wstreamnumber), &mut result__).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
    pub unsafe fn GetPostViewFormat(&self, wstreamnumber: u16, dwformatnumber: u32) -> ::windows::runtime::Result<IWMMediaProps> {
        let mut result__: <IWMMediaProps as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).9)(::std::mem::transmute_copy(self), ::std::mem::transmute(wstreamnumber), ::std::mem::transmute(dwformatnumber), &mut result__).from_abi::<IWMMediaProps>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`, `Win32_Foundation`*"]
    pub unsafe fn SetAllocateForPostView<'a, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>>(&self, wstreamnumber: u16, fallocate: Param1) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).10)(::std::mem::transmute_copy(self), ::std::mem::transmute(wstreamnumber), fallocate.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`, `Win32_Foundation`*"]
    pub unsafe fn GetAllocateForPostView(&self, wstreamnumber: u16) -> ::windows::runtime::Result<super::super::Foundation::BOOL> {
        let mut result__: <super::super::Foundation::BOOL as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).11)(::std::mem::transmute_copy(self), ::std::mem::transmute(wstreamnumber), &mut result__).from_abi::<super::super::Foundation::BOOL>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IWMWriterPostView {
    type Vtable = IWMWriterPostView_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2179075300, 30191, 18714, [128, 4, 252, 83, 196, 91, 220, 62]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMWriterPostView_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pcallback: ::windows::runtime::RawPtr, pvcontext: *mut ::std::ffi::c_void) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, wstreamnum: u16, freceivepostviewsamples: super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, wstreamnum: u16, pfreceivepostviewsamples: *mut super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, wstreamnumber: u16, ppoutput: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, wstreamnumber: u16, poutput: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, wstreamnumber: u16, pcformats: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, wstreamnumber: u16, dwformatnumber: u32, ppprops: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, wstreamnumber: u16, fallocate: super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, wstreamnumber: u16, pfallocate: *mut super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct IWMWriterPostViewCallback(::windows::runtime::IUnknown);
impl IWMWriterPostViewCallback {
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
    pub unsafe fn OnStatus(&self, status: WMT_STATUS, hr: ::windows::runtime::HRESULT, dwtype: WMT_ATTR_DATATYPE, pvalue: *const u8, pvcontext: *const ::std::ffi::c_void) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), ::std::mem::transmute(status), ::std::mem::transmute(hr), ::std::mem::transmute(dwtype), ::std::mem::transmute(pvalue), ::std::mem::transmute(pvcontext)).ok()
    }
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
    pub unsafe fn OnPostViewSample<'a, Param4: ::windows::runtime::IntoParam<'a, INSSBuffer>>(&self, wstreamnumber: u16, cnssampletime: u64, cnssampleduration: u64, dwflags: u32, psample: Param4, pvcontext: *const ::std::ffi::c_void) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), ::std::mem::transmute(wstreamnumber), ::std::mem::transmute(cnssampletime), ::std::mem::transmute(cnssampleduration), ::std::mem::transmute(dwflags), psample.into_param().abi(), ::std::mem::transmute(pvcontext)).ok()
    }
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
    pub unsafe fn AllocateForPostView(&self, wstreamnum: u16, cbbuffer: u32, ppbuffer: *mut ::std::option::Option<INSSBuffer>, pvcontext: *const ::std::ffi::c_void) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self), ::std::mem::transmute(wstreamnum), ::std::mem::transmute(cbbuffer), ::std::mem::transmute(ppbuffer), ::std::mem::transmute(pvcontext)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IWMWriterPostViewCallback {
    type Vtable = IWMWriterPostViewCallback_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3654702237, 41363, 20260, [179, 8, 3, 18, 61, 155, 127, 141]);
}
impl ::std::convert::From<IWMWriterPostViewCallback> for IWMStatusCallback {
    fn from(value: IWMWriterPostViewCallback) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IWMWriterPostViewCallback> for IWMStatusCallback {
    fn from(value: &IWMWriterPostViewCallback) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IWMStatusCallback> for IWMWriterPostViewCallback {
    fn into_param(self) -> ::windows::runtime::Param<'a, IWMStatusCallback> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IWMStatusCallback>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IWMStatusCallback> for &IWMWriterPostViewCallback {
    fn into_param(self) -> ::windows::runtime::Param<'a, IWMStatusCallback> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IWMStatusCallback>::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMWriterPostViewCallback_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, status: WMT_STATUS, hr: ::windows::runtime::HRESULT, dwtype: WMT_ATTR_DATATYPE, pvalue: *const u8, pvcontext: *const ::std::ffi::c_void) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, wstreamnumber: u16, cnssampletime: u64, cnssampleduration: u64, dwflags: u32, psample: ::windows::runtime::RawPtr, pvcontext: *const ::std::ffi::c_void) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, wstreamnum: u16, cbbuffer: u32, ppbuffer: *mut ::windows::runtime::RawPtr, pvcontext: *const ::std::ffi::c_void) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct IWMWriterPreprocess(::windows::runtime::IUnknown);
impl IWMWriterPreprocess {
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
    pub unsafe fn GetMaxPreprocessingPasses(&self, dwinputnum: u32, dwflags: u32) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), ::std::mem::transmute(dwinputnum), ::std::mem::transmute(dwflags), &mut result__).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
    pub unsafe fn SetNumPreprocessingPasses(&self, dwinputnum: u32, dwflags: u32, dwnumpasses: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), ::std::mem::transmute(dwinputnum), ::std::mem::transmute(dwflags), ::std::mem::transmute(dwnumpasses)).ok()
    }
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
    pub unsafe fn BeginPreprocessingPass(&self, dwinputnum: u32, dwflags: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self), ::std::mem::transmute(dwinputnum), ::std::mem::transmute(dwflags)).ok()
    }
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
    pub unsafe fn PreprocessSample<'a, Param3: ::windows::runtime::IntoParam<'a, INSSBuffer>>(&self, dwinputnum: u32, cnssampletime: u64, dwflags: u32, psample: Param3) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::std::mem::transmute_copy(self), ::std::mem::transmute(dwinputnum), ::std::mem::transmute(cnssampletime), ::std::mem::transmute(dwflags), psample.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
    pub unsafe fn EndPreprocessingPass(&self, dwinputnum: u32, dwflags: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self), ::std::mem::transmute(dwinputnum), ::std::mem::transmute(dwflags)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IWMWriterPreprocess {
    type Vtable = IWMWriterPreprocess_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(4233405061, 14532, 17845, [170, 35, 133, 185, 247, 203, 66, 75]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMWriterPreprocess_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwinputnum: u32, dwflags: u32, pdwmaxnumpasses: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwinputnum: u32, dwflags: u32, dwnumpasses: u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwinputnum: u32, dwflags: u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwinputnum: u32, cnssampletime: u64, dwflags: u32, psample: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwinputnum: u32, dwflags: u32) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct IWMWriterPushSink(::windows::runtime::IUnknown);
impl IWMWriterPushSink {
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
    pub unsafe fn OnHeader<'a, Param0: ::windows::runtime::IntoParam<'a, INSSBuffer>>(&self, pheader: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), pheader.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`, `Win32_Foundation`*"]
    pub unsafe fn IsRealTime(&self) -> ::windows::runtime::Result<super::super::Foundation::BOOL> {
        let mut result__: <super::super::Foundation::BOOL as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
    pub unsafe fn AllocateDataUnit(&self, cbdataunit: u32) -> ::windows::runtime::Result<INSSBuffer> {
        let mut result__: <INSSBuffer as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self), ::std::mem::transmute(cbdataunit), &mut result__).from_abi::<INSSBuffer>(result__)
    }
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
    pub unsafe fn OnDataUnit<'a, Param0: ::windows::runtime::IntoParam<'a, INSSBuffer>>(&self, pdataunit: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::std::mem::transmute_copy(self), pdataunit.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
    pub unsafe fn OnEndWriting(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`, `Win32_Foundation`*"]
    pub unsafe fn Connect<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>, Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>>(&self, pwszurl: Param0, pwsztemplateurl: Param1, fautodestroy: Param2) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).8)(::std::mem::transmute_copy(self), pwszurl.into_param().abi(), pwsztemplateurl.into_param().abi(), fautodestroy.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
    pub unsafe fn Disconnect(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).9)(::std::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
    pub unsafe fn EndSession(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).10)(::std::mem::transmute_copy(self)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IWMWriterPushSink {
    type Vtable = IWMWriterPushSink_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3692095141, 1836, 18045, [191, 87, 99, 48, 169, 221, 225, 42]);
}
impl ::std::convert::From<IWMWriterPushSink> for IWMWriterSink {
    fn from(value: IWMWriterPushSink) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IWMWriterPushSink> for IWMWriterSink {
    fn from(value: &IWMWriterPushSink) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IWMWriterSink> for IWMWriterPushSink {
    fn into_param(self) -> ::windows::runtime::Param<'a, IWMWriterSink> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IWMWriterSink>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IWMWriterSink> for &IWMWriterPushSink {
    fn into_param(self) -> ::windows::runtime::Param<'a, IWMWriterSink> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IWMWriterSink>::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMWriterPushSink_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pheader: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pfrealtime: *mut super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, cbdataunit: u32, ppdataunit: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pdataunit: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pwszurl: super::super::Foundation::PWSTR, pwsztemplateurl: super::super::Foundation::PWSTR, fautodestroy: super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct IWMWriterSink(::windows::runtime::IUnknown);
impl IWMWriterSink {
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
    pub unsafe fn OnHeader<'a, Param0: ::windows::runtime::IntoParam<'a, INSSBuffer>>(&self, pheader: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), pheader.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`, `Win32_Foundation`*"]
    pub unsafe fn IsRealTime(&self) -> ::windows::runtime::Result<super::super::Foundation::BOOL> {
        let mut result__: <super::super::Foundation::BOOL as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
    pub unsafe fn AllocateDataUnit(&self, cbdataunit: u32) -> ::windows::runtime::Result<INSSBuffer> {
        let mut result__: <INSSBuffer as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self), ::std::mem::transmute(cbdataunit), &mut result__).from_abi::<INSSBuffer>(result__)
    }
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
    pub unsafe fn OnDataUnit<'a, Param0: ::windows::runtime::IntoParam<'a, INSSBuffer>>(&self, pdataunit: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::std::mem::transmute_copy(self), pdataunit.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
    pub unsafe fn OnEndWriting(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IWMWriterSink {
    type Vtable = IWMWriterSink_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2520804324, 11051, 4563, [179, 107, 0, 192, 79, 97, 8, 255]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMWriterSink_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pheader: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pfrealtime: *mut super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, cbdataunit: u32, ppdataunit: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pdataunit: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct NETSOURCE_URLCREDPOLICY_SETTINGS(pub i32);
pub const NETSOURCE_URLCREDPOLICY_SETTING_SILENTLOGONOK: NETSOURCE_URLCREDPOLICY_SETTINGS = NETSOURCE_URLCREDPOLICY_SETTINGS(0i32);
pub const NETSOURCE_URLCREDPOLICY_SETTING_MUSTPROMPTUSER: NETSOURCE_URLCREDPOLICY_SETTINGS = NETSOURCE_URLCREDPOLICY_SETTINGS(1i32);
pub const NETSOURCE_URLCREDPOLICY_SETTING_ANONYMOUSONLY: NETSOURCE_URLCREDPOLICY_SETTINGS = NETSOURCE_URLCREDPOLICY_SETTINGS(2i32);
impl ::std::convert::From<i32> for NETSOURCE_URLCREDPOLICY_SETTINGS {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for NETSOURCE_URLCREDPOLICY_SETTINGS {
    type Abi = Self;
    type DefaultType = Self;
}
#[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct WEBSTREAM_SAMPLE_TYPE(pub i32);
pub const WEBSTREAM_SAMPLE_TYPE_FILE: WEBSTREAM_SAMPLE_TYPE = WEBSTREAM_SAMPLE_TYPE(1i32);
pub const WEBSTREAM_SAMPLE_TYPE_RENDER: WEBSTREAM_SAMPLE_TYPE = WEBSTREAM_SAMPLE_TYPE(2i32);
impl ::std::convert::From<i32> for WEBSTREAM_SAMPLE_TYPE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for WEBSTREAM_SAMPLE_TYPE {
    type Abi = Self;
    type DefaultType = Self;
}
#[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
#[inline]
pub unsafe fn WMCreateBackupRestorer<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>>(pcallback: Param0) -> ::windows::runtime::Result<IWMLicenseBackup> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WMCreateBackupRestorer(pcallback: ::windows::runtime::RawPtr, ppbackup: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <IWMLicenseBackup as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        WMCreateBackupRestorer(pcallback.into_param().abi(), &mut result__).from_abi::<IWMLicenseBackup>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
#[inline]
pub unsafe fn WMCreateEditor() -> ::windows::runtime::Result<IWMMetadataEditor> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WMCreateEditor(ppeditor: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <IWMMetadataEditor as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        WMCreateEditor(&mut result__).from_abi::<IWMMetadataEditor>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
#[inline]
pub unsafe fn WMCreateIndexer() -> ::windows::runtime::Result<IWMIndexer> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WMCreateIndexer(ppindexer: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <IWMIndexer as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        WMCreateIndexer(&mut result__).from_abi::<IWMIndexer>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
#[inline]
pub unsafe fn WMCreateProfileManager() -> ::windows::runtime::Result<IWMProfileManager> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WMCreateProfileManager(ppprofilemanager: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <IWMProfileManager as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        WMCreateProfileManager(&mut result__).from_abi::<IWMProfileManager>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
#[inline]
pub unsafe fn WMCreateReader<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>>(punkcert: Param0, dwrights: u32) -> ::windows::runtime::Result<IWMReader> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WMCreateReader(punkcert: ::windows::runtime::RawPtr, dwrights: u32, ppreader: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <IWMReader as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        WMCreateReader(punkcert.into_param().abi(), ::std::mem::transmute(dwrights), &mut result__).from_abi::<IWMReader>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
#[inline]
pub unsafe fn WMCreateSyncReader<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>>(punkcert: Param0, dwrights: u32) -> ::windows::runtime::Result<IWMSyncReader> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WMCreateSyncReader(punkcert: ::windows::runtime::RawPtr, dwrights: u32, ppsyncreader: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <IWMSyncReader as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        WMCreateSyncReader(punkcert.into_param().abi(), ::std::mem::transmute(dwrights), &mut result__).from_abi::<IWMSyncReader>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
#[inline]
pub unsafe fn WMCreateWriter<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>>(punkcert: Param0) -> ::windows::runtime::Result<IWMWriter> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WMCreateWriter(punkcert: ::windows::runtime::RawPtr, ppwriter: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <IWMWriter as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        WMCreateWriter(punkcert.into_param().abi(), &mut result__).from_abi::<IWMWriter>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
#[inline]
pub unsafe fn WMCreateWriterFileSink() -> ::windows::runtime::Result<IWMWriterFileSink> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WMCreateWriterFileSink(ppsink: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <IWMWriterFileSink as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        WMCreateWriterFileSink(&mut result__).from_abi::<IWMWriterFileSink>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
#[inline]
pub unsafe fn WMCreateWriterNetworkSink() -> ::windows::runtime::Result<IWMWriterNetworkSink> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WMCreateWriterNetworkSink(ppsink: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <IWMWriterNetworkSink as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        WMCreateWriterNetworkSink(&mut result__).from_abi::<IWMWriterNetworkSink>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
#[inline]
pub unsafe fn WMCreateWriterPushSink() -> ::windows::runtime::Result<IWMWriterPushSink> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WMCreateWriterPushSink(ppsink: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <IWMWriterPushSink as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        WMCreateWriterPushSink(&mut result__).from_abi::<IWMWriterPushSink>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
pub struct WMDRM_IMPORT_INIT_STRUCT {
    pub dwVersion: u32,
    pub cbEncryptedSessionKeyMessage: u32,
    pub pbEncryptedSessionKeyMessage: *mut u8,
    pub cbEncryptedKeyMessage: u32,
    pub pbEncryptedKeyMessage: *mut u8,
}
impl WMDRM_IMPORT_INIT_STRUCT {}
impl ::std::default::Default for WMDRM_IMPORT_INIT_STRUCT {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for WMDRM_IMPORT_INIT_STRUCT {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("WMDRM_IMPORT_INIT_STRUCT")
            .field("dwVersion", &self.dwVersion)
            .field("cbEncryptedSessionKeyMessage", &self.cbEncryptedSessionKeyMessage)
            .field("pbEncryptedSessionKeyMessage", &self.pbEncryptedSessionKeyMessage)
            .field("cbEncryptedKeyMessage", &self.cbEncryptedKeyMessage)
            .field("pbEncryptedKeyMessage", &self.pbEncryptedKeyMessage)
            .finish()
    }
}
impl ::std::cmp::PartialEq for WMDRM_IMPORT_INIT_STRUCT {
    fn eq(&self, other: &Self) -> bool {
        self.dwVersion == other.dwVersion && self.cbEncryptedSessionKeyMessage == other.cbEncryptedSessionKeyMessage && self.pbEncryptedSessionKeyMessage == other.pbEncryptedSessionKeyMessage && self.cbEncryptedKeyMessage == other.cbEncryptedKeyMessage && self.pbEncryptedKeyMessage == other.pbEncryptedKeyMessage
    }
}
impl ::std::cmp::Eq for WMDRM_IMPORT_INIT_STRUCT {}
unsafe impl ::windows::runtime::Abi for WMDRM_IMPORT_INIT_STRUCT {
    type Abi = Self;
    type DefaultType = Self;
}
#[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
pub const WMDRM_IMPORT_INIT_STRUCT_DEFINED: u32 = 1u32;
pub const WMFORMAT_MPEG2Video: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3765272803, 56134, 4559, [180, 209, 0, 128, 95, 108, 187, 234]);
pub const WMFORMAT_Script: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1552224498, 57022, 19623, [187, 165, 240, 122, 16, 79, 141, 255]);
pub const WMFORMAT_VideoInfo: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(89694080, 50006, 4558, [191, 1, 0, 170, 0, 85, 89, 90]);
pub const WMFORMAT_WaveFormatEx: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(89694081, 50006, 4558, [191, 1, 0, 170, 0, 85, 89, 90]);
pub const WMFORMAT_WebStream: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3659426579, 33625, 16464, [179, 152, 56, 142, 150, 91, 240, 12]);
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_Media_WindowsMediaFormat`, `Win32_Foundation`*"]
#[inline]
pub unsafe fn WMIsContentProtected<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(pwszfilename: Param0, pfisprotected: *mut super::super::Foundation::BOOL) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WMIsContentProtected(pwszfilename: super::super::Foundation::PWSTR, pfisprotected: *mut super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT;
        }
        WMIsContentProtected(pwszfilename.into_param().abi(), ::std::mem::transmute(pfisprotected)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub const WMMEDIASUBTYPE_ACELPnet: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(304, 0, 16, [128, 0, 0, 170, 0, 56, 155, 113]);
pub const WMMEDIASUBTYPE_Base: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(0, 0, 16, [128, 0, 0, 170, 0, 56, 155, 113]);
pub const WMMEDIASUBTYPE_DRM: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(9, 0, 16, [128, 0, 0, 170, 0, 56, 155, 113]);
pub const WMMEDIASUBTYPE_I420: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(808596553, 0, 16, [128, 0, 0, 170, 0, 56, 155, 113]);
pub const WMMEDIASUBTYPE_IYUV: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1448433993, 0, 16, [128, 0, 0, 170, 0, 56, 155, 113]);
pub const WMMEDIASUBTYPE_M4S2: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(844313677, 0, 16, [128, 0, 0, 170, 0, 56, 155, 113]);
pub const WMMEDIASUBTYPE_MP3: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(85, 0, 16, [128, 0, 0, 170, 0, 56, 155, 113]);
pub const WMMEDIASUBTYPE_MP43: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(859066445, 0, 16, [128, 0, 0, 170, 0, 56, 155, 113]);
pub const WMMEDIASUBTYPE_MP4S: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1395937357, 0, 16, [128, 0, 0, 170, 0, 56, 155, 113]);
pub const WMMEDIASUBTYPE_MPEG2_VIDEO: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3765272614, 56134, 4559, [180, 209, 0, 128, 95, 108, 187, 234]);
pub const WMMEDIASUBTYPE_MSS1: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(827544397, 0, 16, [128, 0, 0, 170, 0, 56, 155, 113]);
pub const WMMEDIASUBTYPE_MSS2: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(844321613, 0, 16, [128, 0, 0, 170, 0, 56, 155, 113]);
pub const WMMEDIASUBTYPE_P422: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(842150992, 0, 16, [128, 0, 0, 170, 0, 56, 155, 113]);
pub const WMMEDIASUBTYPE_PCM: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1, 0, 16, [128, 0, 0, 170, 0, 56, 155, 113]);
pub const WMMEDIASUBTYPE_RGB1: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3828804472, 21071, 4558, [159, 83, 0, 32, 175, 11, 167, 112]);
pub const WMMEDIASUBTYPE_RGB24: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3828804477, 21071, 4558, [159, 83, 0, 32, 175, 11, 167, 112]);
pub const WMMEDIASUBTYPE_RGB32: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3828804478, 21071, 4558, [159, 83, 0, 32, 175, 11, 167, 112]);
pub const WMMEDIASUBTYPE_RGB4: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3828804473, 21071, 4558, [159, 83, 0, 32, 175, 11, 167, 112]);
pub const WMMEDIASUBTYPE_RGB555: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3828804476, 21071, 4558, [159, 83, 0, 32, 175, 11, 167, 112]);
pub const WMMEDIASUBTYPE_RGB565: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3828804475, 21071, 4558, [159, 83, 0, 32, 175, 11, 167, 112]);
pub const WMMEDIASUBTYPE_RGB8: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3828804474, 21071, 4558, [159, 83, 0, 32, 175, 11, 167, 112]);
pub const WMMEDIASUBTYPE_UYVY: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1498831189, 0, 16, [128, 0, 0, 170, 0, 56, 155, 113]);
pub const WMMEDIASUBTYPE_VIDEOIMAGE: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(491406834, 58870, 19268, [131, 136, 240, 174, 92, 14, 12, 55]);
pub const WMMEDIASUBTYPE_WMAudioV2: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(353, 0, 16, [128, 0, 0, 170, 0, 56, 155, 113]);
pub const WMMEDIASUBTYPE_WMAudioV7: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(353, 0, 16, [128, 0, 0, 170, 0, 56, 155, 113]);
pub const WMMEDIASUBTYPE_WMAudioV8: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(353, 0, 16, [128, 0, 0, 170, 0, 56, 155, 113]);
pub const WMMEDIASUBTYPE_WMAudioV9: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(354, 0, 16, [128, 0, 0, 170, 0, 56, 155, 113]);
pub const WMMEDIASUBTYPE_WMAudio_Lossless: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(355, 0, 16, [128, 0, 0, 170, 0, 56, 155, 113]);
pub const WMMEDIASUBTYPE_WMSP1: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(10, 0, 16, [128, 0, 0, 170, 0, 56, 155, 113]);
pub const WMMEDIASUBTYPE_WMSP2: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(11, 0, 16, [128, 0, 0, 170, 0, 56, 155, 113]);
pub const WMMEDIASUBTYPE_WMV1: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(827739479, 0, 16, [128, 0, 0, 170, 0, 56, 155, 113]);
pub const WMMEDIASUBTYPE_WMV2: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(844516695, 0, 16, [128, 0, 0, 170, 0, 56, 155, 113]);
pub const WMMEDIASUBTYPE_WMV3: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(861293911, 0, 16, [128, 0, 0, 170, 0, 56, 155, 113]);
pub const WMMEDIASUBTYPE_WMVA: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1096174935, 0, 16, [128, 0, 0, 170, 0, 56, 155, 113]);
pub const WMMEDIASUBTYPE_WMVP: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1347833175, 0, 16, [128, 0, 0, 170, 0, 56, 155, 113]);
pub const WMMEDIASUBTYPE_WVC1: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(826496599, 0, 16, [128, 0, 0, 170, 0, 56, 155, 113]);
pub const WMMEDIASUBTYPE_WVP2: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(844125783, 0, 16, [128, 0, 0, 170, 0, 56, 155, 113]);
pub const WMMEDIASUBTYPE_WebStream: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2002933716, 50727, 16843, [143, 129, 122, 199, 255, 28, 64, 204]);
pub const WMMEDIASUBTYPE_YUY2: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(844715353, 0, 16, [128, 0, 0, 170, 0, 56, 155, 113]);
pub const WMMEDIASUBTYPE_YV12: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(842094169, 0, 16, [128, 0, 0, 170, 0, 56, 155, 113]);
pub const WMMEDIASUBTYPE_YVU9: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(961893977, 0, 16, [128, 0, 0, 170, 0, 56, 155, 113]);
pub const WMMEDIASUBTYPE_YVYU: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1431918169, 0, 16, [128, 0, 0, 170, 0, 56, 155, 113]);
pub const WMMEDIATYPE_Audio: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1935963489, 0, 16, [128, 0, 0, 170, 0, 56, 155, 113]);
pub const WMMEDIATYPE_FileTransfer: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3655628153, 37646, 17447, [173, 252, 173, 128, 242, 144, 228, 112]);
pub const WMMEDIATYPE_Image: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(883232728, 35493, 17286, [129, 254, 160, 239, 224, 72, 142, 49]);
pub const WMMEDIATYPE_Script: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1935895908, 0, 16, [128, 0, 0, 170, 0, 56, 155, 113]);
pub const WMMEDIATYPE_Text: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2612666023, 23218, 18473, [186, 87, 9, 64, 32, 155, 207, 62]);
pub const WMMEDIATYPE_Video: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1935960438, 0, 16, [128, 0, 0, 170, 0, 56, 155, 113]);
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
#[doc = "*Required features: `Win32_Media_WindowsMediaFormat`, `Win32_Foundation`, `Win32_Graphics_Gdi`*"]
pub struct WMMPEG2VIDEOINFO {
    pub hdr: WMVIDEOINFOHEADER2,
    pub dwStartTimeCode: u32,
    pub cbSequenceHeader: u32,
    pub dwProfile: u32,
    pub dwLevel: u32,
    pub dwFlags: u32,
    pub dwSequenceHeader: [u32; 1],
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl WMMPEG2VIDEOINFO {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::std::default::Default for WMMPEG2VIDEOINFO {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::std::fmt::Debug for WMMPEG2VIDEOINFO {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("WMMPEG2VIDEOINFO")
            .field("hdr", &self.hdr)
            .field("dwStartTimeCode", &self.dwStartTimeCode)
            .field("cbSequenceHeader", &self.cbSequenceHeader)
            .field("dwProfile", &self.dwProfile)
            .field("dwLevel", &self.dwLevel)
            .field("dwFlags", &self.dwFlags)
            .field("dwSequenceHeader", &self.dwSequenceHeader)
            .finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::std::cmp::PartialEq for WMMPEG2VIDEOINFO {
    fn eq(&self, other: &Self) -> bool {
        self.hdr == other.hdr && self.dwStartTimeCode == other.dwStartTimeCode && self.cbSequenceHeader == other.cbSequenceHeader && self.dwProfile == other.dwProfile && self.dwLevel == other.dwLevel && self.dwFlags == other.dwFlags && self.dwSequenceHeader == other.dwSequenceHeader
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::std::cmp::Eq for WMMPEG2VIDEOINFO {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
unsafe impl ::windows::runtime::Abi for WMMPEG2VIDEOINFO {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
pub struct WMSCRIPTFORMAT {
    pub scriptType: ::windows::runtime::GUID,
}
impl WMSCRIPTFORMAT {}
impl ::std::default::Default for WMSCRIPTFORMAT {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for WMSCRIPTFORMAT {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("WMSCRIPTFORMAT").field("scriptType", &self.scriptType).finish()
    }
}
impl ::std::cmp::PartialEq for WMSCRIPTFORMAT {
    fn eq(&self, other: &Self) -> bool {
        self.scriptType == other.scriptType
    }
}
impl ::std::cmp::Eq for WMSCRIPTFORMAT {}
unsafe impl ::windows::runtime::Abi for WMSCRIPTFORMAT {
    type Abi = Self;
    type DefaultType = Self;
}
pub const WMSCRIPTTYPE_TwoStrings: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2196998768, 49823, 4561, [151, 173, 0, 160, 201, 94, 168, 80]);
#[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct WMT_ATTR_DATATYPE(pub i32);
pub const WMT_TYPE_DWORD: WMT_ATTR_DATATYPE = WMT_ATTR_DATATYPE(0i32);
pub const WMT_TYPE_STRING: WMT_ATTR_DATATYPE = WMT_ATTR_DATATYPE(1i32);
pub const WMT_TYPE_BINARY: WMT_ATTR_DATATYPE = WMT_ATTR_DATATYPE(2i32);
pub const WMT_TYPE_BOOL: WMT_ATTR_DATATYPE = WMT_ATTR_DATATYPE(3i32);
pub const WMT_TYPE_QWORD: WMT_ATTR_DATATYPE = WMT_ATTR_DATATYPE(4i32);
pub const WMT_TYPE_WORD: WMT_ATTR_DATATYPE = WMT_ATTR_DATATYPE(5i32);
pub const WMT_TYPE_GUID: WMT_ATTR_DATATYPE = WMT_ATTR_DATATYPE(6i32);
impl ::std::convert::From<i32> for WMT_ATTR_DATATYPE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for WMT_ATTR_DATATYPE {
    type Abi = Self;
    type DefaultType = Self;
}
#[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct WMT_ATTR_IMAGETYPE(pub i32);
pub const WMT_IMAGETYPE_BITMAP: WMT_ATTR_IMAGETYPE = WMT_ATTR_IMAGETYPE(1i32);
pub const WMT_IMAGETYPE_JPEG: WMT_ATTR_IMAGETYPE = WMT_ATTR_IMAGETYPE(2i32);
pub const WMT_IMAGETYPE_GIF: WMT_ATTR_IMAGETYPE = WMT_ATTR_IMAGETYPE(3i32);
impl ::std::convert::From<i32> for WMT_ATTR_IMAGETYPE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for WMT_ATTR_IMAGETYPE {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone)]
#[repr(C)]
#[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
pub struct WMT_BUFFER_SEGMENT {
    pub pBuffer: ::std::option::Option<INSSBuffer>,
    pub cbOffset: u32,
    pub cbLength: u32,
}
impl WMT_BUFFER_SEGMENT {}
impl ::std::default::Default for WMT_BUFFER_SEGMENT {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for WMT_BUFFER_SEGMENT {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("WMT_BUFFER_SEGMENT").field("pBuffer", &self.pBuffer).field("cbOffset", &self.cbOffset).field("cbLength", &self.cbLength).finish()
    }
}
impl ::std::cmp::PartialEq for WMT_BUFFER_SEGMENT {
    fn eq(&self, other: &Self) -> bool {
        self.pBuffer == other.pBuffer && self.cbOffset == other.cbOffset && self.cbLength == other.cbLength
    }
}
impl ::std::cmp::Eq for WMT_BUFFER_SEGMENT {}
unsafe impl ::windows::runtime::Abi for WMT_BUFFER_SEGMENT {
    type Abi = ::std::mem::ManuallyDrop<Self>;
    type DefaultType = Self;
}
#[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct WMT_CODEC_INFO_TYPE(pub i32);
pub const WMT_CODECINFO_AUDIO: WMT_CODEC_INFO_TYPE = WMT_CODEC_INFO_TYPE(0i32);
pub const WMT_CODECINFO_VIDEO: WMT_CODEC_INFO_TYPE = WMT_CODEC_INFO_TYPE(1i32);
pub const WMT_CODECINFO_UNKNOWN: WMT_CODEC_INFO_TYPE = WMT_CODEC_INFO_TYPE(-1i32);
impl ::std::convert::From<i32> for WMT_CODEC_INFO_TYPE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for WMT_CODEC_INFO_TYPE {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
pub struct WMT_COLORSPACEINFO_EXTENSION_DATA {
    pub ucColorPrimaries: u8,
    pub ucColorTransferChar: u8,
    pub ucColorMatrixCoef: u8,
}
impl WMT_COLORSPACEINFO_EXTENSION_DATA {}
impl ::std::default::Default for WMT_COLORSPACEINFO_EXTENSION_DATA {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for WMT_COLORSPACEINFO_EXTENSION_DATA {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("WMT_COLORSPACEINFO_EXTENSION_DATA").field("ucColorPrimaries", &self.ucColorPrimaries).field("ucColorTransferChar", &self.ucColorTransferChar).field("ucColorMatrixCoef", &self.ucColorMatrixCoef).finish()
    }
}
impl ::std::cmp::PartialEq for WMT_COLORSPACEINFO_EXTENSION_DATA {
    fn eq(&self, other: &Self) -> bool {
        self.ucColorPrimaries == other.ucColorPrimaries && self.ucColorTransferChar == other.ucColorTransferChar && self.ucColorMatrixCoef == other.ucColorMatrixCoef
    }
}
impl ::std::cmp::Eq for WMT_COLORSPACEINFO_EXTENSION_DATA {}
unsafe impl ::windows::runtime::Abi for WMT_COLORSPACEINFO_EXTENSION_DATA {
    type Abi = Self;
    type DefaultType = Self;
}
#[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct WMT_CREDENTIAL_FLAGS(pub i32);
pub const WMT_CREDENTIAL_SAVE: WMT_CREDENTIAL_FLAGS = WMT_CREDENTIAL_FLAGS(1i32);
pub const WMT_CREDENTIAL_DONT_CACHE: WMT_CREDENTIAL_FLAGS = WMT_CREDENTIAL_FLAGS(2i32);
pub const WMT_CREDENTIAL_CLEAR_TEXT: WMT_CREDENTIAL_FLAGS = WMT_CREDENTIAL_FLAGS(4i32);
pub const WMT_CREDENTIAL_PROXY: WMT_CREDENTIAL_FLAGS = WMT_CREDENTIAL_FLAGS(8i32);
pub const WMT_CREDENTIAL_ENCRYPT: WMT_CREDENTIAL_FLAGS = WMT_CREDENTIAL_FLAGS(16i32);
impl ::std::convert::From<i32> for WMT_CREDENTIAL_FLAGS {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for WMT_CREDENTIAL_FLAGS {
    type Abi = Self;
    type DefaultType = Self;
}
pub const WMT_DMOCATEGORY_AUDIO_WATERMARK: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1696734298, 64117, 19257, [181, 12, 6, 195, 54, 182, 163, 239]);
pub const WMT_DMOCATEGORY_VIDEO_WATERMARK: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(410831138, 36604, 17412, [157, 175, 99, 244, 131, 13, 241, 188]);
#[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct WMT_DRMLA_TRUST(pub i32);
pub const WMT_DRMLA_UNTRUSTED: WMT_DRMLA_TRUST = WMT_DRMLA_TRUST(0i32);
pub const WMT_DRMLA_TRUSTED: WMT_DRMLA_TRUST = WMT_DRMLA_TRUST(1i32);
pub const WMT_DRMLA_TAMPERED: WMT_DRMLA_TRUST = WMT_DRMLA_TRUST(2i32);
impl ::std::convert::From<i32> for WMT_DRMLA_TRUST {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for WMT_DRMLA_TRUST {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone)]
#[repr(C)]
#[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
pub struct WMT_FILESINK_DATA_UNIT {
    pub packetHeaderBuffer: WMT_BUFFER_SEGMENT,
    pub cPayloads: u32,
    pub pPayloadHeaderBuffers: *mut WMT_BUFFER_SEGMENT,
    pub cPayloadDataFragments: u32,
    pub pPayloadDataFragments: *mut WMT_PAYLOAD_FRAGMENT,
}
impl WMT_FILESINK_DATA_UNIT {}
impl ::std::default::Default for WMT_FILESINK_DATA_UNIT {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for WMT_FILESINK_DATA_UNIT {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("WMT_FILESINK_DATA_UNIT")
            .field("packetHeaderBuffer", &self.packetHeaderBuffer)
            .field("cPayloads", &self.cPayloads)
            .field("pPayloadHeaderBuffers", &self.pPayloadHeaderBuffers)
            .field("cPayloadDataFragments", &self.cPayloadDataFragments)
            .field("pPayloadDataFragments", &self.pPayloadDataFragments)
            .finish()
    }
}
impl ::std::cmp::PartialEq for WMT_FILESINK_DATA_UNIT {
    fn eq(&self, other: &Self) -> bool {
        self.packetHeaderBuffer == other.packetHeaderBuffer && self.cPayloads == other.cPayloads && self.pPayloadHeaderBuffers == other.pPayloadHeaderBuffers && self.cPayloadDataFragments == other.cPayloadDataFragments && self.pPayloadDataFragments == other.pPayloadDataFragments
    }
}
impl ::std::cmp::Eq for WMT_FILESINK_DATA_UNIT {}
unsafe impl ::windows::runtime::Abi for WMT_FILESINK_DATA_UNIT {
    type Abi = ::std::mem::ManuallyDrop<Self>;
    type DefaultType = Self;
}
#[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct WMT_FILESINK_MODE(pub i32);
pub const WMT_FM_SINGLE_BUFFERS: WMT_FILESINK_MODE = WMT_FILESINK_MODE(1i32);
pub const WMT_FM_FILESINK_DATA_UNITS: WMT_FILESINK_MODE = WMT_FILESINK_MODE(2i32);
pub const WMT_FM_FILESINK_UNBUFFERED: WMT_FILESINK_MODE = WMT_FILESINK_MODE(4i32);
impl ::std::convert::From<i32> for WMT_FILESINK_MODE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for WMT_FILESINK_MODE {
    type Abi = Self;
    type DefaultType = Self;
}
#[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct WMT_IMAGE_TYPE(pub i32);
pub const WMT_IT_NONE: WMT_IMAGE_TYPE = WMT_IMAGE_TYPE(0i32);
pub const WMT_IT_BITMAP: WMT_IMAGE_TYPE = WMT_IMAGE_TYPE(1i32);
pub const WMT_IT_JPEG: WMT_IMAGE_TYPE = WMT_IMAGE_TYPE(2i32);
pub const WMT_IT_GIF: WMT_IMAGE_TYPE = WMT_IMAGE_TYPE(3i32);
impl ::std::convert::From<i32> for WMT_IMAGE_TYPE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for WMT_IMAGE_TYPE {
    type Abi = Self;
    type DefaultType = Self;
}
#[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct WMT_INDEXER_TYPE(pub i32);
pub const WMT_IT_PRESENTATION_TIME: WMT_INDEXER_TYPE = WMT_INDEXER_TYPE(0i32);
pub const WMT_IT_FRAME_NUMBERS: WMT_INDEXER_TYPE = WMT_INDEXER_TYPE(1i32);
pub const WMT_IT_TIMECODE: WMT_INDEXER_TYPE = WMT_INDEXER_TYPE(2i32);
impl ::std::convert::From<i32> for WMT_INDEXER_TYPE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for WMT_INDEXER_TYPE {
    type Abi = Self;
    type DefaultType = Self;
}
#[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct WMT_INDEX_TYPE(pub i32);
pub const WMT_IT_NEAREST_DATA_UNIT: WMT_INDEX_TYPE = WMT_INDEX_TYPE(1i32);
pub const WMT_IT_NEAREST_OBJECT: WMT_INDEX_TYPE = WMT_INDEX_TYPE(2i32);
pub const WMT_IT_NEAREST_CLEAN_POINT: WMT_INDEX_TYPE = WMT_INDEX_TYPE(3i32);
impl ::std::convert::From<i32> for WMT_INDEX_TYPE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for WMT_INDEX_TYPE {
    type Abi = Self;
    type DefaultType = Self;
}
#[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct WMT_MUSICSPEECH_CLASS_MODE(pub i32);
pub const WMT_MS_CLASS_MUSIC: WMT_MUSICSPEECH_CLASS_MODE = WMT_MUSICSPEECH_CLASS_MODE(0i32);
pub const WMT_MS_CLASS_SPEECH: WMT_MUSICSPEECH_CLASS_MODE = WMT_MUSICSPEECH_CLASS_MODE(1i32);
pub const WMT_MS_CLASS_MIXED: WMT_MUSICSPEECH_CLASS_MODE = WMT_MUSICSPEECH_CLASS_MODE(2i32);
impl ::std::convert::From<i32> for WMT_MUSICSPEECH_CLASS_MODE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for WMT_MUSICSPEECH_CLASS_MODE {
    type Abi = Self;
    type DefaultType = Self;
}
#[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct WMT_NET_PROTOCOL(pub i32);
pub const WMT_PROTOCOL_HTTP: WMT_NET_PROTOCOL = WMT_NET_PROTOCOL(0i32);
impl ::std::convert::From<i32> for WMT_NET_PROTOCOL {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for WMT_NET_PROTOCOL {
    type Abi = Self;
    type DefaultType = Self;
}
#[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct WMT_OFFSET_FORMAT(pub i32);
pub const WMT_OFFSET_FORMAT_100NS: WMT_OFFSET_FORMAT = WMT_OFFSET_FORMAT(0i32);
pub const WMT_OFFSET_FORMAT_FRAME_NUMBERS: WMT_OFFSET_FORMAT = WMT_OFFSET_FORMAT(1i32);
pub const WMT_OFFSET_FORMAT_PLAYLIST_OFFSET: WMT_OFFSET_FORMAT = WMT_OFFSET_FORMAT(2i32);
pub const WMT_OFFSET_FORMAT_TIMECODE: WMT_OFFSET_FORMAT = WMT_OFFSET_FORMAT(3i32);
pub const WMT_OFFSET_FORMAT_100NS_APPROXIMATE: WMT_OFFSET_FORMAT = WMT_OFFSET_FORMAT(4i32);
impl ::std::convert::From<i32> for WMT_OFFSET_FORMAT {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for WMT_OFFSET_FORMAT {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone)]
#[repr(C)]
#[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
pub struct WMT_PAYLOAD_FRAGMENT {
    pub dwPayloadIndex: u32,
    pub segmentData: WMT_BUFFER_SEGMENT,
}
impl WMT_PAYLOAD_FRAGMENT {}
impl ::std::default::Default for WMT_PAYLOAD_FRAGMENT {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for WMT_PAYLOAD_FRAGMENT {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("WMT_PAYLOAD_FRAGMENT").field("dwPayloadIndex", &self.dwPayloadIndex).field("segmentData", &self.segmentData).finish()
    }
}
impl ::std::cmp::PartialEq for WMT_PAYLOAD_FRAGMENT {
    fn eq(&self, other: &Self) -> bool {
        self.dwPayloadIndex == other.dwPayloadIndex && self.segmentData == other.segmentData
    }
}
impl ::std::cmp::Eq for WMT_PAYLOAD_FRAGMENT {}
unsafe impl ::windows::runtime::Abi for WMT_PAYLOAD_FRAGMENT {
    type Abi = ::std::mem::ManuallyDrop<Self>;
    type DefaultType = Self;
}
#[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct WMT_PLAY_MODE(pub i32);
pub const WMT_PLAY_MODE_AUTOSELECT: WMT_PLAY_MODE = WMT_PLAY_MODE(0i32);
pub const WMT_PLAY_MODE_LOCAL: WMT_PLAY_MODE = WMT_PLAY_MODE(1i32);
pub const WMT_PLAY_MODE_DOWNLOAD: WMT_PLAY_MODE = WMT_PLAY_MODE(2i32);
pub const WMT_PLAY_MODE_STREAMING: WMT_PLAY_MODE = WMT_PLAY_MODE(3i32);
impl ::std::convert::From<i32> for WMT_PLAY_MODE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for WMT_PLAY_MODE {
    type Abi = Self;
    type DefaultType = Self;
}
#[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct WMT_PROXY_SETTINGS(pub i32);
pub const WMT_PROXY_SETTING_NONE: WMT_PROXY_SETTINGS = WMT_PROXY_SETTINGS(0i32);
pub const WMT_PROXY_SETTING_MANUAL: WMT_PROXY_SETTINGS = WMT_PROXY_SETTINGS(1i32);
pub const WMT_PROXY_SETTING_AUTO: WMT_PROXY_SETTINGS = WMT_PROXY_SETTINGS(2i32);
pub const WMT_PROXY_SETTING_BROWSER: WMT_PROXY_SETTINGS = WMT_PROXY_SETTINGS(3i32);
pub const WMT_PROXY_SETTING_MAX: WMT_PROXY_SETTINGS = WMT_PROXY_SETTINGS(4i32);
impl ::std::convert::From<i32> for WMT_PROXY_SETTINGS {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for WMT_PROXY_SETTINGS {
    type Abi = Self;
    type DefaultType = Self;
}
#[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct WMT_RIGHTS(pub i32);
pub const WMT_RIGHT_PLAYBACK: WMT_RIGHTS = WMT_RIGHTS(1i32);
pub const WMT_RIGHT_COPY_TO_NON_SDMI_DEVICE: WMT_RIGHTS = WMT_RIGHTS(2i32);
pub const WMT_RIGHT_COPY_TO_CD: WMT_RIGHTS = WMT_RIGHTS(8i32);
pub const WMT_RIGHT_COPY_TO_SDMI_DEVICE: WMT_RIGHTS = WMT_RIGHTS(16i32);
pub const WMT_RIGHT_ONE_TIME: WMT_RIGHTS = WMT_RIGHTS(32i32);
pub const WMT_RIGHT_SAVE_STREAM_PROTECTED: WMT_RIGHTS = WMT_RIGHTS(64i32);
pub const WMT_RIGHT_COPY: WMT_RIGHTS = WMT_RIGHTS(128i32);
pub const WMT_RIGHT_COLLABORATIVE_PLAY: WMT_RIGHTS = WMT_RIGHTS(256i32);
pub const WMT_RIGHT_SDMI_TRIGGER: WMT_RIGHTS = WMT_RIGHTS(65536i32);
pub const WMT_RIGHT_SDMI_NOMORECOPIES: WMT_RIGHTS = WMT_RIGHTS(131072i32);
impl ::std::convert::From<i32> for WMT_RIGHTS {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for WMT_RIGHTS {
    type Abi = Self;
    type DefaultType = Self;
}
#[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct WMT_STATUS(pub i32);
pub const WMT_ERROR: WMT_STATUS = WMT_STATUS(0i32);
pub const WMT_OPENED: WMT_STATUS = WMT_STATUS(1i32);
pub const WMT_BUFFERING_START: WMT_STATUS = WMT_STATUS(2i32);
pub const WMT_BUFFERING_STOP: WMT_STATUS = WMT_STATUS(3i32);
pub const WMT_EOF: WMT_STATUS = WMT_STATUS(4i32);
pub const WMT_END_OF_FILE: WMT_STATUS = WMT_STATUS(4i32);
pub const WMT_END_OF_SEGMENT: WMT_STATUS = WMT_STATUS(5i32);
pub const WMT_END_OF_STREAMING: WMT_STATUS = WMT_STATUS(6i32);
pub const WMT_LOCATING: WMT_STATUS = WMT_STATUS(7i32);
pub const WMT_CONNECTING: WMT_STATUS = WMT_STATUS(8i32);
pub const WMT_NO_RIGHTS: WMT_STATUS = WMT_STATUS(9i32);
pub const WMT_MISSING_CODEC: WMT_STATUS = WMT_STATUS(10i32);
pub const WMT_STARTED: WMT_STATUS = WMT_STATUS(11i32);
pub const WMT_STOPPED: WMT_STATUS = WMT_STATUS(12i32);
pub const WMT_CLOSED: WMT_STATUS = WMT_STATUS(13i32);
pub const WMT_STRIDING: WMT_STATUS = WMT_STATUS(14i32);
pub const WMT_TIMER: WMT_STATUS = WMT_STATUS(15i32);
pub const WMT_INDEX_PROGRESS: WMT_STATUS = WMT_STATUS(16i32);
pub const WMT_SAVEAS_START: WMT_STATUS = WMT_STATUS(17i32);
pub const WMT_SAVEAS_STOP: WMT_STATUS = WMT_STATUS(18i32);
pub const WMT_NEW_SOURCEFLAGS: WMT_STATUS = WMT_STATUS(19i32);
pub const WMT_NEW_METADATA: WMT_STATUS = WMT_STATUS(20i32);
pub const WMT_BACKUPRESTORE_BEGIN: WMT_STATUS = WMT_STATUS(21i32);
pub const WMT_SOURCE_SWITCH: WMT_STATUS = WMT_STATUS(22i32);
pub const WMT_ACQUIRE_LICENSE: WMT_STATUS = WMT_STATUS(23i32);
pub const WMT_INDIVIDUALIZE: WMT_STATUS = WMT_STATUS(24i32);
pub const WMT_NEEDS_INDIVIDUALIZATION: WMT_STATUS = WMT_STATUS(25i32);
pub const WMT_NO_RIGHTS_EX: WMT_STATUS = WMT_STATUS(26i32);
pub const WMT_BACKUPRESTORE_END: WMT_STATUS = WMT_STATUS(27i32);
pub const WMT_BACKUPRESTORE_CONNECTING: WMT_STATUS = WMT_STATUS(28i32);
pub const WMT_BACKUPRESTORE_DISCONNECTING: WMT_STATUS = WMT_STATUS(29i32);
pub const WMT_ERROR_WITHURL: WMT_STATUS = WMT_STATUS(30i32);
pub const WMT_RESTRICTED_LICENSE: WMT_STATUS = WMT_STATUS(31i32);
pub const WMT_CLIENT_CONNECT: WMT_STATUS = WMT_STATUS(32i32);
pub const WMT_CLIENT_DISCONNECT: WMT_STATUS = WMT_STATUS(33i32);
pub const WMT_NATIVE_OUTPUT_PROPS_CHANGED: WMT_STATUS = WMT_STATUS(34i32);
pub const WMT_RECONNECT_START: WMT_STATUS = WMT_STATUS(35i32);
pub const WMT_RECONNECT_END: WMT_STATUS = WMT_STATUS(36i32);
pub const WMT_CLIENT_CONNECT_EX: WMT_STATUS = WMT_STATUS(37i32);
pub const WMT_CLIENT_DISCONNECT_EX: WMT_STATUS = WMT_STATUS(38i32);
pub const WMT_SET_FEC_SPAN: WMT_STATUS = WMT_STATUS(39i32);
pub const WMT_PREROLL_READY: WMT_STATUS = WMT_STATUS(40i32);
pub const WMT_PREROLL_COMPLETE: WMT_STATUS = WMT_STATUS(41i32);
pub const WMT_CLIENT_PROPERTIES: WMT_STATUS = WMT_STATUS(42i32);
pub const WMT_LICENSEURL_SIGNATURE_STATE: WMT_STATUS = WMT_STATUS(43i32);
pub const WMT_INIT_PLAYLIST_BURN: WMT_STATUS = WMT_STATUS(44i32);
pub const WMT_TRANSCRYPTOR_INIT: WMT_STATUS = WMT_STATUS(45i32);
pub const WMT_TRANSCRYPTOR_SEEKED: WMT_STATUS = WMT_STATUS(46i32);
pub const WMT_TRANSCRYPTOR_READ: WMT_STATUS = WMT_STATUS(47i32);
pub const WMT_TRANSCRYPTOR_CLOSED: WMT_STATUS = WMT_STATUS(48i32);
pub const WMT_PROXIMITY_RESULT: WMT_STATUS = WMT_STATUS(49i32);
pub const WMT_PROXIMITY_COMPLETED: WMT_STATUS = WMT_STATUS(50i32);
pub const WMT_CONTENT_ENABLER: WMT_STATUS = WMT_STATUS(51i32);
impl ::std::convert::From<i32> for WMT_STATUS {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for WMT_STATUS {
    type Abi = Self;
    type DefaultType = Self;
}
#[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct WMT_STORAGE_FORMAT(pub i32);
pub const WMT_Storage_Format_MP3: WMT_STORAGE_FORMAT = WMT_STORAGE_FORMAT(0i32);
pub const WMT_Storage_Format_V1: WMT_STORAGE_FORMAT = WMT_STORAGE_FORMAT(1i32);
impl ::std::convert::From<i32> for WMT_STORAGE_FORMAT {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for WMT_STORAGE_FORMAT {
    type Abi = Self;
    type DefaultType = Self;
}
#[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct WMT_STREAM_SELECTION(pub i32);
pub const WMT_OFF: WMT_STREAM_SELECTION = WMT_STREAM_SELECTION(0i32);
pub const WMT_CLEANPOINT_ONLY: WMT_STREAM_SELECTION = WMT_STREAM_SELECTION(1i32);
pub const WMT_ON: WMT_STREAM_SELECTION = WMT_STREAM_SELECTION(2i32);
impl ::std::convert::From<i32> for WMT_STREAM_SELECTION {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for WMT_STREAM_SELECTION {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C, packed(2))]
#[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
pub struct WMT_TIMECODE_EXTENSION_DATA {
    pub wRange: u16,
    pub dwTimecode: u32,
    pub dwUserbits: u32,
    pub dwAmFlags: u32,
}
impl WMT_TIMECODE_EXTENSION_DATA {}
impl ::std::default::Default for WMT_TIMECODE_EXTENSION_DATA {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::cmp::PartialEq for WMT_TIMECODE_EXTENSION_DATA {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::std::cmp::Eq for WMT_TIMECODE_EXTENSION_DATA {}
unsafe impl ::windows::runtime::Abi for WMT_TIMECODE_EXTENSION_DATA {
    type Abi = Self;
    type DefaultType = Self;
}
#[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct WMT_TIMECODE_FRAMERATE(pub i32);
pub const WMT_TIMECODE_FRAMERATE_30: WMT_TIMECODE_FRAMERATE = WMT_TIMECODE_FRAMERATE(0i32);
pub const WMT_TIMECODE_FRAMERATE_30DROP: WMT_TIMECODE_FRAMERATE = WMT_TIMECODE_FRAMERATE(1i32);
pub const WMT_TIMECODE_FRAMERATE_25: WMT_TIMECODE_FRAMERATE = WMT_TIMECODE_FRAMERATE(2i32);
pub const WMT_TIMECODE_FRAMERATE_24: WMT_TIMECODE_FRAMERATE = WMT_TIMECODE_FRAMERATE(3i32);
impl ::std::convert::From<i32> for WMT_TIMECODE_FRAMERATE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for WMT_TIMECODE_FRAMERATE {
    type Abi = Self;
    type DefaultType = Self;
}
#[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct WMT_TRANSPORT_TYPE(pub i32);
pub const WMT_Transport_Type_Unreliable: WMT_TRANSPORT_TYPE = WMT_TRANSPORT_TYPE(0i32);
pub const WMT_Transport_Type_Reliable: WMT_TRANSPORT_TYPE = WMT_TRANSPORT_TYPE(1i32);
impl ::std::convert::From<i32> for WMT_TRANSPORT_TYPE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for WMT_TRANSPORT_TYPE {
    type Abi = Self;
    type DefaultType = Self;
}
#[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct WMT_VERSION(pub i32);
pub const WMT_VER_4_0: WMT_VERSION = WMT_VERSION(262144i32);
pub const WMT_VER_7_0: WMT_VERSION = WMT_VERSION(458752i32);
pub const WMT_VER_8_0: WMT_VERSION = WMT_VERSION(524288i32);
pub const WMT_VER_9_0: WMT_VERSION = WMT_VERSION(589824i32);
impl ::std::convert::From<i32> for WMT_VERSION {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for WMT_VERSION {
    type Abi = Self;
    type DefaultType = Self;
}
#[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
pub const WMT_VIDEOIMAGE_INTEGER_DENOMINATOR: i32 = 65536i32;
#[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
pub const WMT_VIDEOIMAGE_MAGIC_NUMBER: u32 = 491406834u32;
#[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
pub const WMT_VIDEOIMAGE_MAGIC_NUMBER_2: u32 = 491406835u32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
pub struct WMT_VIDEOIMAGE_SAMPLE {
    pub dwMagic: u32,
    pub cbStruct: u32,
    pub dwControlFlags: u32,
    pub dwInputFlagsCur: u32,
    pub lCurMotionXtoX: i32,
    pub lCurMotionYtoX: i32,
    pub lCurMotionXoffset: i32,
    pub lCurMotionXtoY: i32,
    pub lCurMotionYtoY: i32,
    pub lCurMotionYoffset: i32,
    pub lCurBlendCoef1: i32,
    pub lCurBlendCoef2: i32,
    pub dwInputFlagsPrev: u32,
    pub lPrevMotionXtoX: i32,
    pub lPrevMotionYtoX: i32,
    pub lPrevMotionXoffset: i32,
    pub lPrevMotionXtoY: i32,
    pub lPrevMotionYtoY: i32,
    pub lPrevMotionYoffset: i32,
    pub lPrevBlendCoef1: i32,
    pub lPrevBlendCoef2: i32,
}
impl WMT_VIDEOIMAGE_SAMPLE {}
impl ::std::default::Default for WMT_VIDEOIMAGE_SAMPLE {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for WMT_VIDEOIMAGE_SAMPLE {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("WMT_VIDEOIMAGE_SAMPLE")
            .field("dwMagic", &self.dwMagic)
            .field("cbStruct", &self.cbStruct)
            .field("dwControlFlags", &self.dwControlFlags)
            .field("dwInputFlagsCur", &self.dwInputFlagsCur)
            .field("lCurMotionXtoX", &self.lCurMotionXtoX)
            .field("lCurMotionYtoX", &self.lCurMotionYtoX)
            .field("lCurMotionXoffset", &self.lCurMotionXoffset)
            .field("lCurMotionXtoY", &self.lCurMotionXtoY)
            .field("lCurMotionYtoY", &self.lCurMotionYtoY)
            .field("lCurMotionYoffset", &self.lCurMotionYoffset)
            .field("lCurBlendCoef1", &self.lCurBlendCoef1)
            .field("lCurBlendCoef2", &self.lCurBlendCoef2)
            .field("dwInputFlagsPrev", &self.dwInputFlagsPrev)
            .field("lPrevMotionXtoX", &self.lPrevMotionXtoX)
            .field("lPrevMotionYtoX", &self.lPrevMotionYtoX)
            .field("lPrevMotionXoffset", &self.lPrevMotionXoffset)
            .field("lPrevMotionXtoY", &self.lPrevMotionXtoY)
            .field("lPrevMotionYtoY", &self.lPrevMotionYtoY)
            .field("lPrevMotionYoffset", &self.lPrevMotionYoffset)
            .field("lPrevBlendCoef1", &self.lPrevBlendCoef1)
            .field("lPrevBlendCoef2", &self.lPrevBlendCoef2)
            .finish()
    }
}
impl ::std::cmp::PartialEq for WMT_VIDEOIMAGE_SAMPLE {
    fn eq(&self, other: &Self) -> bool {
        self.dwMagic == other.dwMagic
            && self.cbStruct == other.cbStruct
            && self.dwControlFlags == other.dwControlFlags
            && self.dwInputFlagsCur == other.dwInputFlagsCur
            && self.lCurMotionXtoX == other.lCurMotionXtoX
            && self.lCurMotionYtoX == other.lCurMotionYtoX
            && self.lCurMotionXoffset == other.lCurMotionXoffset
            && self.lCurMotionXtoY == other.lCurMotionXtoY
            && self.lCurMotionYtoY == other.lCurMotionYtoY
            && self.lCurMotionYoffset == other.lCurMotionYoffset
            && self.lCurBlendCoef1 == other.lCurBlendCoef1
            && self.lCurBlendCoef2 == other.lCurBlendCoef2
            && self.dwInputFlagsPrev == other.dwInputFlagsPrev
            && self.lPrevMotionXtoX == other.lPrevMotionXtoX
            && self.lPrevMotionYtoX == other.lPrevMotionYtoX
            && self.lPrevMotionXoffset == other.lPrevMotionXoffset
            && self.lPrevMotionXtoY == other.lPrevMotionXtoY
            && self.lPrevMotionYtoY == other.lPrevMotionYtoY
            && self.lPrevMotionYoffset == other.lPrevMotionYoffset
            && self.lPrevBlendCoef1 == other.lPrevBlendCoef1
            && self.lPrevBlendCoef2 == other.lPrevBlendCoef2
    }
}
impl ::std::cmp::Eq for WMT_VIDEOIMAGE_SAMPLE {}
unsafe impl ::windows::runtime::Abi for WMT_VIDEOIMAGE_SAMPLE {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_Media_WindowsMediaFormat`, `Win32_Foundation`*"]
pub struct WMT_VIDEOIMAGE_SAMPLE2 {
    pub dwMagic: u32,
    pub dwStructSize: u32,
    pub dwControlFlags: u32,
    pub dwViewportWidth: u32,
    pub dwViewportHeight: u32,
    pub dwCurrImageWidth: u32,
    pub dwCurrImageHeight: u32,
    pub fCurrRegionX0: f32,
    pub fCurrRegionY0: f32,
    pub fCurrRegionWidth: f32,
    pub fCurrRegionHeight: f32,
    pub fCurrBlendCoef: f32,
    pub dwPrevImageWidth: u32,
    pub dwPrevImageHeight: u32,
    pub fPrevRegionX0: f32,
    pub fPrevRegionY0: f32,
    pub fPrevRegionWidth: f32,
    pub fPrevRegionHeight: f32,
    pub fPrevBlendCoef: f32,
    pub dwEffectType: u32,
    pub dwNumEffectParas: u32,
    pub fEffectPara0: f32,
    pub fEffectPara1: f32,
    pub fEffectPara2: f32,
    pub fEffectPara3: f32,
    pub fEffectPara4: f32,
    pub bKeepPrevImage: super::super::Foundation::BOOL,
}
#[cfg(feature = "Win32_Foundation")]
impl WMT_VIDEOIMAGE_SAMPLE2 {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for WMT_VIDEOIMAGE_SAMPLE2 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for WMT_VIDEOIMAGE_SAMPLE2 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("WMT_VIDEOIMAGE_SAMPLE2")
            .field("dwMagic", &self.dwMagic)
            .field("dwStructSize", &self.dwStructSize)
            .field("dwControlFlags", &self.dwControlFlags)
            .field("dwViewportWidth", &self.dwViewportWidth)
            .field("dwViewportHeight", &self.dwViewportHeight)
            .field("dwCurrImageWidth", &self.dwCurrImageWidth)
            .field("dwCurrImageHeight", &self.dwCurrImageHeight)
            .field("fCurrRegionX0", &self.fCurrRegionX0)
            .field("fCurrRegionY0", &self.fCurrRegionY0)
            .field("fCurrRegionWidth", &self.fCurrRegionWidth)
            .field("fCurrRegionHeight", &self.fCurrRegionHeight)
            .field("fCurrBlendCoef", &self.fCurrBlendCoef)
            .field("dwPrevImageWidth", &self.dwPrevImageWidth)
            .field("dwPrevImageHeight", &self.dwPrevImageHeight)
            .field("fPrevRegionX0", &self.fPrevRegionX0)
            .field("fPrevRegionY0", &self.fPrevRegionY0)
            .field("fPrevRegionWidth", &self.fPrevRegionWidth)
            .field("fPrevRegionHeight", &self.fPrevRegionHeight)
            .field("fPrevBlendCoef", &self.fPrevBlendCoef)
            .field("dwEffectType", &self.dwEffectType)
            .field("dwNumEffectParas", &self.dwNumEffectParas)
            .field("fEffectPara0", &self.fEffectPara0)
            .field("fEffectPara1", &self.fEffectPara1)
            .field("fEffectPara2", &self.fEffectPara2)
            .field("fEffectPara3", &self.fEffectPara3)
            .field("fEffectPara4", &self.fEffectPara4)
            .field("bKeepPrevImage", &self.bKeepPrevImage)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for WMT_VIDEOIMAGE_SAMPLE2 {
    fn eq(&self, other: &Self) -> bool {
        self.dwMagic == other.dwMagic
            && self.dwStructSize == other.dwStructSize
            && self.dwControlFlags == other.dwControlFlags
            && self.dwViewportWidth == other.dwViewportWidth
            && self.dwViewportHeight == other.dwViewportHeight
            && self.dwCurrImageWidth == other.dwCurrImageWidth
            && self.dwCurrImageHeight == other.dwCurrImageHeight
            && self.fCurrRegionX0 == other.fCurrRegionX0
            && self.fCurrRegionY0 == other.fCurrRegionY0
            && self.fCurrRegionWidth == other.fCurrRegionWidth
            && self.fCurrRegionHeight == other.fCurrRegionHeight
            && self.fCurrBlendCoef == other.fCurrBlendCoef
            && self.dwPrevImageWidth == other.dwPrevImageWidth
            && self.dwPrevImageHeight == other.dwPrevImageHeight
            && self.fPrevRegionX0 == other.fPrevRegionX0
            && self.fPrevRegionY0 == other.fPrevRegionY0
            && self.fPrevRegionWidth == other.fPrevRegionWidth
            && self.fPrevRegionHeight == other.fPrevRegionHeight
            && self.fPrevBlendCoef == other.fPrevBlendCoef
            && self.dwEffectType == other.dwEffectType
            && self.dwNumEffectParas == other.dwNumEffectParas
            && self.fEffectPara0 == other.fEffectPara0
            && self.fEffectPara1 == other.fEffectPara1
            && self.fEffectPara2 == other.fEffectPara2
            && self.fEffectPara3 == other.fEffectPara3
            && self.fEffectPara4 == other.fEffectPara4
            && self.bKeepPrevImage == other.bKeepPrevImage
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for WMT_VIDEOIMAGE_SAMPLE2 {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for WMT_VIDEOIMAGE_SAMPLE2 {
    type Abi = Self;
    type DefaultType = Self;
}
#[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
pub const WMT_VIDEOIMAGE_SAMPLE_ADV_BLENDING: u32 = 8u32;
#[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
pub const WMT_VIDEOIMAGE_SAMPLE_BLENDING: u32 = 4u32;
#[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
pub const WMT_VIDEOIMAGE_SAMPLE_INPUT_FRAME: u32 = 1u32;
#[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
pub const WMT_VIDEOIMAGE_SAMPLE_MOTION: u32 = 1u32;
#[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
pub const WMT_VIDEOIMAGE_SAMPLE_OUTPUT_FRAME: u32 = 2u32;
#[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
pub const WMT_VIDEOIMAGE_SAMPLE_ROTATION: u32 = 2u32;
#[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
pub const WMT_VIDEOIMAGE_SAMPLE_USES_CURRENT_INPUT_FRAME: u32 = 4u32;
#[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
pub const WMT_VIDEOIMAGE_SAMPLE_USES_PREVIOUS_INPUT_FRAME: u32 = 8u32;
#[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
pub const WMT_VIDEOIMAGE_TRANSITION_BOW_TIE: u32 = 11u32;
#[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
pub const WMT_VIDEOIMAGE_TRANSITION_CIRCLE: u32 = 12u32;
#[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
pub const WMT_VIDEOIMAGE_TRANSITION_CROSS_FADE: u32 = 13u32;
#[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
pub const WMT_VIDEOIMAGE_TRANSITION_DIAGONAL: u32 = 14u32;
#[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
pub const WMT_VIDEOIMAGE_TRANSITION_DIAMOND: u32 = 15u32;
#[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
pub const WMT_VIDEOIMAGE_TRANSITION_FADE_TO_COLOR: u32 = 16u32;
#[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
pub const WMT_VIDEOIMAGE_TRANSITION_FILLED_V: u32 = 17u32;
#[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
pub const WMT_VIDEOIMAGE_TRANSITION_FLIP: u32 = 18u32;
#[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
pub const WMT_VIDEOIMAGE_TRANSITION_INSET: u32 = 19u32;
#[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
pub const WMT_VIDEOIMAGE_TRANSITION_IRIS: u32 = 20u32;
#[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
pub const WMT_VIDEOIMAGE_TRANSITION_PAGE_ROLL: u32 = 21u32;
#[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
pub const WMT_VIDEOIMAGE_TRANSITION_RECTANGLE: u32 = 23u32;
#[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
pub const WMT_VIDEOIMAGE_TRANSITION_REVEAL: u32 = 24u32;
#[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
pub const WMT_VIDEOIMAGE_TRANSITION_SLIDE: u32 = 27u32;
#[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
pub const WMT_VIDEOIMAGE_TRANSITION_SPLIT: u32 = 29u32;
#[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
pub const WMT_VIDEOIMAGE_TRANSITION_STAR: u32 = 30u32;
#[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
pub const WMT_VIDEOIMAGE_TRANSITION_WHEEL: u32 = 31u32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_Media_WindowsMediaFormat`, `Win32_Foundation`*"]
pub struct WMT_WATERMARK_ENTRY {
    pub wmetType: WMT_WATERMARK_ENTRY_TYPE,
    pub clsid: ::windows::runtime::GUID,
    pub cbDisplayName: u32,
    pub pwszDisplayName: super::super::Foundation::PWSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl WMT_WATERMARK_ENTRY {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for WMT_WATERMARK_ENTRY {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for WMT_WATERMARK_ENTRY {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("WMT_WATERMARK_ENTRY").field("wmetType", &self.wmetType).field("clsid", &self.clsid).field("cbDisplayName", &self.cbDisplayName).field("pwszDisplayName", &self.pwszDisplayName).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for WMT_WATERMARK_ENTRY {
    fn eq(&self, other: &Self) -> bool {
        self.wmetType == other.wmetType && self.clsid == other.clsid && self.cbDisplayName == other.cbDisplayName && self.pwszDisplayName == other.pwszDisplayName
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for WMT_WATERMARK_ENTRY {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for WMT_WATERMARK_ENTRY {
    type Abi = Self;
    type DefaultType = Self;
}
#[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct WMT_WATERMARK_ENTRY_TYPE(pub i32);
pub const WMT_WMETYPE_AUDIO: WMT_WATERMARK_ENTRY_TYPE = WMT_WATERMARK_ENTRY_TYPE(1i32);
pub const WMT_WMETYPE_VIDEO: WMT_WATERMARK_ENTRY_TYPE = WMT_WATERMARK_ENTRY_TYPE(2i32);
impl ::std::convert::From<i32> for WMT_WATERMARK_ENTRY_TYPE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for WMT_WATERMARK_ENTRY_TYPE {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
pub struct WMT_WEBSTREAM_FORMAT {
    pub cbSize: u16,
    pub cbSampleHeaderFixedData: u16,
    pub wVersion: u16,
    pub wReserved: u16,
}
impl WMT_WEBSTREAM_FORMAT {}
impl ::std::default::Default for WMT_WEBSTREAM_FORMAT {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for WMT_WEBSTREAM_FORMAT {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("WMT_WEBSTREAM_FORMAT").field("cbSize", &self.cbSize).field("cbSampleHeaderFixedData", &self.cbSampleHeaderFixedData).field("wVersion", &self.wVersion).field("wReserved", &self.wReserved).finish()
    }
}
impl ::std::cmp::PartialEq for WMT_WEBSTREAM_FORMAT {
    fn eq(&self, other: &Self) -> bool {
        self.cbSize == other.cbSize && self.cbSampleHeaderFixedData == other.cbSampleHeaderFixedData && self.wVersion == other.wVersion && self.wReserved == other.wReserved
    }
}
impl ::std::cmp::Eq for WMT_WEBSTREAM_FORMAT {}
unsafe impl ::windows::runtime::Abi for WMT_WEBSTREAM_FORMAT {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
pub struct WMT_WEBSTREAM_SAMPLE_HEADER {
    pub cbLength: u16,
    pub wPart: u16,
    pub cTotalParts: u16,
    pub wSampleType: u16,
    pub wszURL: [u16; 1],
}
impl WMT_WEBSTREAM_SAMPLE_HEADER {}
impl ::std::default::Default for WMT_WEBSTREAM_SAMPLE_HEADER {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for WMT_WEBSTREAM_SAMPLE_HEADER {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("WMT_WEBSTREAM_SAMPLE_HEADER").field("cbLength", &self.cbLength).field("wPart", &self.wPart).field("cTotalParts", &self.cTotalParts).field("wSampleType", &self.wSampleType).field("wszURL", &self.wszURL).finish()
    }
}
impl ::std::cmp::PartialEq for WMT_WEBSTREAM_SAMPLE_HEADER {
    fn eq(&self, other: &Self) -> bool {
        self.cbLength == other.cbLength && self.wPart == other.wPart && self.cTotalParts == other.cTotalParts && self.wSampleType == other.wSampleType && self.wszURL == other.wszURL
    }
}
impl ::std::cmp::Eq for WMT_WEBSTREAM_SAMPLE_HEADER {}
unsafe impl ::windows::runtime::Abi for WMT_WEBSTREAM_SAMPLE_HEADER {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
#[doc = "*Required features: `Win32_Media_WindowsMediaFormat`, `Win32_Foundation`, `Win32_Graphics_Gdi`*"]
pub struct WMVIDEOINFOHEADER {
    pub rcSource: super::super::Foundation::RECT,
    pub rcTarget: super::super::Foundation::RECT,
    pub dwBitRate: u32,
    pub dwBitErrorRate: u32,
    pub AvgTimePerFrame: i64,
    pub bmiHeader: super::super::Graphics::Gdi::BITMAPINFOHEADER,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl WMVIDEOINFOHEADER {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::std::default::Default for WMVIDEOINFOHEADER {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::std::fmt::Debug for WMVIDEOINFOHEADER {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("WMVIDEOINFOHEADER").field("rcSource", &self.rcSource).field("rcTarget", &self.rcTarget).field("dwBitRate", &self.dwBitRate).field("dwBitErrorRate", &self.dwBitErrorRate).field("AvgTimePerFrame", &self.AvgTimePerFrame).field("bmiHeader", &self.bmiHeader).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::std::cmp::PartialEq for WMVIDEOINFOHEADER {
    fn eq(&self, other: &Self) -> bool {
        self.rcSource == other.rcSource && self.rcTarget == other.rcTarget && self.dwBitRate == other.dwBitRate && self.dwBitErrorRate == other.dwBitErrorRate && self.AvgTimePerFrame == other.AvgTimePerFrame && self.bmiHeader == other.bmiHeader
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::std::cmp::Eq for WMVIDEOINFOHEADER {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
unsafe impl ::windows::runtime::Abi for WMVIDEOINFOHEADER {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
#[doc = "*Required features: `Win32_Media_WindowsMediaFormat`, `Win32_Foundation`, `Win32_Graphics_Gdi`*"]
pub struct WMVIDEOINFOHEADER2 {
    pub rcSource: super::super::Foundation::RECT,
    pub rcTarget: super::super::Foundation::RECT,
    pub dwBitRate: u32,
    pub dwBitErrorRate: u32,
    pub AvgTimePerFrame: i64,
    pub dwInterlaceFlags: u32,
    pub dwCopyProtectFlags: u32,
    pub dwPictAspectRatioX: u32,
    pub dwPictAspectRatioY: u32,
    pub dwReserved1: u32,
    pub dwReserved2: u32,
    pub bmiHeader: super::super::Graphics::Gdi::BITMAPINFOHEADER,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl WMVIDEOINFOHEADER2 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::std::default::Default for WMVIDEOINFOHEADER2 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::std::fmt::Debug for WMVIDEOINFOHEADER2 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("WMVIDEOINFOHEADER2")
            .field("rcSource", &self.rcSource)
            .field("rcTarget", &self.rcTarget)
            .field("dwBitRate", &self.dwBitRate)
            .field("dwBitErrorRate", &self.dwBitErrorRate)
            .field("AvgTimePerFrame", &self.AvgTimePerFrame)
            .field("dwInterlaceFlags", &self.dwInterlaceFlags)
            .field("dwCopyProtectFlags", &self.dwCopyProtectFlags)
            .field("dwPictAspectRatioX", &self.dwPictAspectRatioX)
            .field("dwPictAspectRatioY", &self.dwPictAspectRatioY)
            .field("dwReserved1", &self.dwReserved1)
            .field("dwReserved2", &self.dwReserved2)
            .field("bmiHeader", &self.bmiHeader)
            .finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::std::cmp::PartialEq for WMVIDEOINFOHEADER2 {
    fn eq(&self, other: &Self) -> bool {
        self.rcSource == other.rcSource
            && self.rcTarget == other.rcTarget
            && self.dwBitRate == other.dwBitRate
            && self.dwBitErrorRate == other.dwBitErrorRate
            && self.AvgTimePerFrame == other.AvgTimePerFrame
            && self.dwInterlaceFlags == other.dwInterlaceFlags
            && self.dwCopyProtectFlags == other.dwCopyProtectFlags
            && self.dwPictAspectRatioX == other.dwPictAspectRatioX
            && self.dwPictAspectRatioY == other.dwPictAspectRatioY
            && self.dwReserved1 == other.dwReserved1
            && self.dwReserved2 == other.dwReserved2
            && self.bmiHeader == other.bmiHeader
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::std::cmp::Eq for WMVIDEOINFOHEADER2 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
unsafe impl ::windows::runtime::Abi for WMVIDEOINFOHEADER2 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
pub struct WM_ADDRESS_ACCESSENTRY {
    pub dwIPAddress: u32,
    pub dwMask: u32,
}
impl WM_ADDRESS_ACCESSENTRY {}
impl ::std::default::Default for WM_ADDRESS_ACCESSENTRY {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for WM_ADDRESS_ACCESSENTRY {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("WM_ADDRESS_ACCESSENTRY").field("dwIPAddress", &self.dwIPAddress).field("dwMask", &self.dwMask).finish()
    }
}
impl ::std::cmp::PartialEq for WM_ADDRESS_ACCESSENTRY {
    fn eq(&self, other: &Self) -> bool {
        self.dwIPAddress == other.dwIPAddress && self.dwMask == other.dwMask
    }
}
impl ::std::cmp::Eq for WM_ADDRESS_ACCESSENTRY {}
unsafe impl ::windows::runtime::Abi for WM_ADDRESS_ACCESSENTRY {
    type Abi = Self;
    type DefaultType = Self;
}
#[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct WM_AETYPE(pub i32);
pub const WM_AETYPE_INCLUDE: WM_AETYPE = WM_AETYPE(105i32);
pub const WM_AETYPE_EXCLUDE: WM_AETYPE = WM_AETYPE(101i32);
impl ::std::convert::From<i32> for WM_AETYPE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for WM_AETYPE {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
pub struct WM_CLIENT_PROPERTIES {
    pub dwIPAddress: u32,
    pub dwPort: u32,
}
impl WM_CLIENT_PROPERTIES {}
impl ::std::default::Default for WM_CLIENT_PROPERTIES {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for WM_CLIENT_PROPERTIES {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("WM_CLIENT_PROPERTIES").field("dwIPAddress", &self.dwIPAddress).field("dwPort", &self.dwPort).finish()
    }
}
impl ::std::cmp::PartialEq for WM_CLIENT_PROPERTIES {
    fn eq(&self, other: &Self) -> bool {
        self.dwIPAddress == other.dwIPAddress && self.dwPort == other.dwPort
    }
}
impl ::std::cmp::Eq for WM_CLIENT_PROPERTIES {}
unsafe impl ::windows::runtime::Abi for WM_CLIENT_PROPERTIES {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_Media_WindowsMediaFormat`, `Win32_Foundation`*"]
pub struct WM_CLIENT_PROPERTIES_EX {
    pub cbSize: u32,
    pub pwszIPAddress: super::super::Foundation::PWSTR,
    pub pwszPort: super::super::Foundation::PWSTR,
    pub pwszDNSName: super::super::Foundation::PWSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl WM_CLIENT_PROPERTIES_EX {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for WM_CLIENT_PROPERTIES_EX {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for WM_CLIENT_PROPERTIES_EX {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("WM_CLIENT_PROPERTIES_EX").field("cbSize", &self.cbSize).field("pwszIPAddress", &self.pwszIPAddress).field("pwszPort", &self.pwszPort).field("pwszDNSName", &self.pwszDNSName).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for WM_CLIENT_PROPERTIES_EX {
    fn eq(&self, other: &Self) -> bool {
        self.cbSize == other.cbSize && self.pwszIPAddress == other.pwszIPAddress && self.pwszPort == other.pwszPort && self.pwszDNSName == other.pwszDNSName
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for WM_CLIENT_PROPERTIES_EX {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for WM_CLIENT_PROPERTIES_EX {
    type Abi = Self;
    type DefaultType = Self;
}
#[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
pub const WM_CL_INTERLACED420: u32 = 0u32;
#[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
pub const WM_CL_PROGRESSIVE420: u32 = 1u32;
#[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
pub const WM_CT_BOTTOM_FIELD_FIRST: u32 = 32u32;
#[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
pub const WM_CT_INTERLACED: u32 = 128u32;
#[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
pub const WM_CT_REPEAT_FIRST_FIELD: u32 = 16u32;
#[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
pub const WM_CT_TOP_FIELD_FIRST: u32 = 64u32;
#[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct WM_DM_INTERLACED_TYPE(pub i32);
pub const WM_DM_NOTINTERLACED: WM_DM_INTERLACED_TYPE = WM_DM_INTERLACED_TYPE(0i32);
pub const WM_DM_DEINTERLACE_NORMAL: WM_DM_INTERLACED_TYPE = WM_DM_INTERLACED_TYPE(1i32);
pub const WM_DM_DEINTERLACE_HALFSIZE: WM_DM_INTERLACED_TYPE = WM_DM_INTERLACED_TYPE(2i32);
pub const WM_DM_DEINTERLACE_HALFSIZEDOUBLERATE: WM_DM_INTERLACED_TYPE = WM_DM_INTERLACED_TYPE(3i32);
pub const WM_DM_DEINTERLACE_INVERSETELECINE: WM_DM_INTERLACED_TYPE = WM_DM_INTERLACED_TYPE(4i32);
pub const WM_DM_DEINTERLACE_VERTICALHALFSIZEDOUBLERATE: WM_DM_INTERLACED_TYPE = WM_DM_INTERLACED_TYPE(5i32);
impl ::std::convert::From<i32> for WM_DM_INTERLACED_TYPE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for WM_DM_INTERLACED_TYPE {
    type Abi = Self;
    type DefaultType = Self;
}
#[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct WM_DM_IT_FIRST_FRAME_COHERENCY(pub i32);
pub const WM_DM_IT_DISABLE_COHERENT_MODE: WM_DM_IT_FIRST_FRAME_COHERENCY = WM_DM_IT_FIRST_FRAME_COHERENCY(0i32);
pub const WM_DM_IT_FIRST_FRAME_IN_CLIP_IS_AA_TOP: WM_DM_IT_FIRST_FRAME_COHERENCY = WM_DM_IT_FIRST_FRAME_COHERENCY(1i32);
pub const WM_DM_IT_FIRST_FRAME_IN_CLIP_IS_BB_TOP: WM_DM_IT_FIRST_FRAME_COHERENCY = WM_DM_IT_FIRST_FRAME_COHERENCY(2i32);
pub const WM_DM_IT_FIRST_FRAME_IN_CLIP_IS_BC_TOP: WM_DM_IT_FIRST_FRAME_COHERENCY = WM_DM_IT_FIRST_FRAME_COHERENCY(3i32);
pub const WM_DM_IT_FIRST_FRAME_IN_CLIP_IS_CD_TOP: WM_DM_IT_FIRST_FRAME_COHERENCY = WM_DM_IT_FIRST_FRAME_COHERENCY(4i32);
pub const WM_DM_IT_FIRST_FRAME_IN_CLIP_IS_DD_TOP: WM_DM_IT_FIRST_FRAME_COHERENCY = WM_DM_IT_FIRST_FRAME_COHERENCY(5i32);
pub const WM_DM_IT_FIRST_FRAME_IN_CLIP_IS_AA_BOTTOM: WM_DM_IT_FIRST_FRAME_COHERENCY = WM_DM_IT_FIRST_FRAME_COHERENCY(6i32);
pub const WM_DM_IT_FIRST_FRAME_IN_CLIP_IS_BB_BOTTOM: WM_DM_IT_FIRST_FRAME_COHERENCY = WM_DM_IT_FIRST_FRAME_COHERENCY(7i32);
pub const WM_DM_IT_FIRST_FRAME_IN_CLIP_IS_BC_BOTTOM: WM_DM_IT_FIRST_FRAME_COHERENCY = WM_DM_IT_FIRST_FRAME_COHERENCY(8i32);
pub const WM_DM_IT_FIRST_FRAME_IN_CLIP_IS_CD_BOTTOM: WM_DM_IT_FIRST_FRAME_COHERENCY = WM_DM_IT_FIRST_FRAME_COHERENCY(9i32);
pub const WM_DM_IT_FIRST_FRAME_IN_CLIP_IS_DD_BOTTOM: WM_DM_IT_FIRST_FRAME_COHERENCY = WM_DM_IT_FIRST_FRAME_COHERENCY(10i32);
impl ::std::convert::From<i32> for WM_DM_IT_FIRST_FRAME_COHERENCY {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for WM_DM_IT_FIRST_FRAME_COHERENCY {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C, packed(1))]
#[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
pub struct WM_LEAKY_BUCKET_PAIR {
    pub dwBitrate: u32,
    pub msBufferWindow: u32,
}
impl WM_LEAKY_BUCKET_PAIR {}
impl ::std::default::Default for WM_LEAKY_BUCKET_PAIR {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::cmp::PartialEq for WM_LEAKY_BUCKET_PAIR {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::std::cmp::Eq for WM_LEAKY_BUCKET_PAIR {}
unsafe impl ::windows::runtime::Abi for WM_LEAKY_BUCKET_PAIR {
    type Abi = Self;
    type DefaultType = Self;
}
#[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
pub const WM_MAX_STREAMS: u32 = 63u32;
#[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
pub const WM_MAX_VIDEO_STREAMS: u32 = 63u32;
#[derive(:: std :: clone :: Clone)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_Media_WindowsMediaFormat`, `Win32_Foundation`*"]
pub struct WM_MEDIA_TYPE {
    pub majortype: ::windows::runtime::GUID,
    pub subtype: ::windows::runtime::GUID,
    pub bFixedSizeSamples: super::super::Foundation::BOOL,
    pub bTemporalCompression: super::super::Foundation::BOOL,
    pub lSampleSize: u32,
    pub formattype: ::windows::runtime::GUID,
    pub pUnk: ::std::option::Option<::windows::runtime::IUnknown>,
    pub cbFormat: u32,
    pub pbFormat: *mut u8,
}
#[cfg(feature = "Win32_Foundation")]
impl WM_MEDIA_TYPE {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for WM_MEDIA_TYPE {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for WM_MEDIA_TYPE {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("WM_MEDIA_TYPE")
            .field("majortype", &self.majortype)
            .field("subtype", &self.subtype)
            .field("bFixedSizeSamples", &self.bFixedSizeSamples)
            .field("bTemporalCompression", &self.bTemporalCompression)
            .field("lSampleSize", &self.lSampleSize)
            .field("formattype", &self.formattype)
            .field("pUnk", &self.pUnk)
            .field("cbFormat", &self.cbFormat)
            .field("pbFormat", &self.pbFormat)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for WM_MEDIA_TYPE {
    fn eq(&self, other: &Self) -> bool {
        self.majortype == other.majortype && self.subtype == other.subtype && self.bFixedSizeSamples == other.bFixedSizeSamples && self.bTemporalCompression == other.bTemporalCompression && self.lSampleSize == other.lSampleSize && self.formattype == other.formattype && self.pUnk == other.pUnk && self.cbFormat == other.cbFormat && self.pbFormat == other.pbFormat
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for WM_MEDIA_TYPE {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for WM_MEDIA_TYPE {
    type Abi = ::std::mem::ManuallyDrop<Self>;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C, packed(1))]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_Media_WindowsMediaFormat`, `Win32_Foundation`*"]
pub struct WM_PICTURE {
    pub pwszMIMEType: super::super::Foundation::PWSTR,
    pub bPictureType: u8,
    pub pwszDescription: super::super::Foundation::PWSTR,
    pub dwDataLen: u32,
    pub pbData: *mut u8,
}
#[cfg(feature = "Win32_Foundation")]
impl WM_PICTURE {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for WM_PICTURE {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for WM_PICTURE {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for WM_PICTURE {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for WM_PICTURE {
    type Abi = Self;
    type DefaultType = Self;
}
#[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct WM_PLAYBACK_DRC_LEVEL(pub i32);
pub const WM_PLAYBACK_DRC_HIGH: WM_PLAYBACK_DRC_LEVEL = WM_PLAYBACK_DRC_LEVEL(0i32);
pub const WM_PLAYBACK_DRC_MEDIUM: WM_PLAYBACK_DRC_LEVEL = WM_PLAYBACK_DRC_LEVEL(1i32);
pub const WM_PLAYBACK_DRC_LOW: WM_PLAYBACK_DRC_LEVEL = WM_PLAYBACK_DRC_LEVEL(2i32);
impl ::std::convert::From<i32> for WM_PLAYBACK_DRC_LEVEL {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for WM_PLAYBACK_DRC_LEVEL {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
pub struct WM_PORT_NUMBER_RANGE {
    pub wPortBegin: u16,
    pub wPortEnd: u16,
}
impl WM_PORT_NUMBER_RANGE {}
impl ::std::default::Default for WM_PORT_NUMBER_RANGE {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for WM_PORT_NUMBER_RANGE {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("WM_PORT_NUMBER_RANGE").field("wPortBegin", &self.wPortBegin).field("wPortEnd", &self.wPortEnd).finish()
    }
}
impl ::std::cmp::PartialEq for WM_PORT_NUMBER_RANGE {
    fn eq(&self, other: &Self) -> bool {
        self.wPortBegin == other.wPortBegin && self.wPortEnd == other.wPortEnd
    }
}
impl ::std::cmp::Eq for WM_PORT_NUMBER_RANGE {}
unsafe impl ::windows::runtime::Abi for WM_PORT_NUMBER_RANGE {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_Media_WindowsMediaFormat`, `Win32_Foundation`*"]
pub struct WM_READER_CLIENTINFO {
    pub cbSize: u32,
    pub wszLang: super::super::Foundation::PWSTR,
    pub wszBrowserUserAgent: super::super::Foundation::PWSTR,
    pub wszBrowserWebPage: super::super::Foundation::PWSTR,
    pub qwReserved: u64,
    pub pReserved: *mut super::super::Foundation::LPARAM,
    pub wszHostExe: super::super::Foundation::PWSTR,
    pub qwHostVersion: u64,
    pub wszPlayerUserAgent: super::super::Foundation::PWSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl WM_READER_CLIENTINFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for WM_READER_CLIENTINFO {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for WM_READER_CLIENTINFO {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("WM_READER_CLIENTINFO")
            .field("cbSize", &self.cbSize)
            .field("wszLang", &self.wszLang)
            .field("wszBrowserUserAgent", &self.wszBrowserUserAgent)
            .field("wszBrowserWebPage", &self.wszBrowserWebPage)
            .field("qwReserved", &self.qwReserved)
            .field("pReserved", &self.pReserved)
            .field("wszHostExe", &self.wszHostExe)
            .field("qwHostVersion", &self.qwHostVersion)
            .field("wszPlayerUserAgent", &self.wszPlayerUserAgent)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for WM_READER_CLIENTINFO {
    fn eq(&self, other: &Self) -> bool {
        self.cbSize == other.cbSize && self.wszLang == other.wszLang && self.wszBrowserUserAgent == other.wszBrowserUserAgent && self.wszBrowserWebPage == other.wszBrowserWebPage && self.qwReserved == other.qwReserved && self.pReserved == other.pReserved && self.wszHostExe == other.wszHostExe && self.qwHostVersion == other.qwHostVersion && self.wszPlayerUserAgent == other.wszPlayerUserAgent
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for WM_READER_CLIENTINFO {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for WM_READER_CLIENTINFO {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
pub struct WM_READER_STATISTICS {
    pub cbSize: u32,
    pub dwBandwidth: u32,
    pub cPacketsReceived: u32,
    pub cPacketsRecovered: u32,
    pub cPacketsLost: u32,
    pub wQuality: u16,
}
impl WM_READER_STATISTICS {}
impl ::std::default::Default for WM_READER_STATISTICS {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for WM_READER_STATISTICS {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("WM_READER_STATISTICS").field("cbSize", &self.cbSize).field("dwBandwidth", &self.dwBandwidth).field("cPacketsReceived", &self.cPacketsReceived).field("cPacketsRecovered", &self.cPacketsRecovered).field("cPacketsLost", &self.cPacketsLost).field("wQuality", &self.wQuality).finish()
    }
}
impl ::std::cmp::PartialEq for WM_READER_STATISTICS {
    fn eq(&self, other: &Self) -> bool {
        self.cbSize == other.cbSize && self.dwBandwidth == other.dwBandwidth && self.cPacketsReceived == other.cPacketsReceived && self.cPacketsRecovered == other.cPacketsRecovered && self.cPacketsLost == other.cPacketsLost && self.wQuality == other.wQuality
    }
}
impl ::std::cmp::Eq for WM_READER_STATISTICS {}
unsafe impl ::windows::runtime::Abi for WM_READER_STATISTICS {
    type Abi = Self;
    type DefaultType = Self;
}
#[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct WM_SFEX_TYPE(pub i32);
pub const WM_SFEX_NOTASYNCPOINT: WM_SFEX_TYPE = WM_SFEX_TYPE(2i32);
pub const WM_SFEX_DATALOSS: WM_SFEX_TYPE = WM_SFEX_TYPE(4i32);
impl ::std::convert::From<i32> for WM_SFEX_TYPE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for WM_SFEX_TYPE {
    type Abi = Self;
    type DefaultType = Self;
}
#[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct WM_SF_TYPE(pub i32);
pub const WM_SF_CLEANPOINT: WM_SF_TYPE = WM_SF_TYPE(1i32);
pub const WM_SF_DISCONTINUITY: WM_SF_TYPE = WM_SF_TYPE(2i32);
pub const WM_SF_DATALOSS: WM_SF_TYPE = WM_SF_TYPE(4i32);
impl ::std::convert::From<i32> for WM_SF_TYPE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for WM_SF_TYPE {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C, packed(2))]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_Media_WindowsMediaFormat`, `Win32_Foundation`*"]
pub struct WM_STREAM_PRIORITY_RECORD {
    pub wStreamNumber: u16,
    pub fMandatory: super::super::Foundation::BOOL,
}
#[cfg(feature = "Win32_Foundation")]
impl WM_STREAM_PRIORITY_RECORD {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for WM_STREAM_PRIORITY_RECORD {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for WM_STREAM_PRIORITY_RECORD {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for WM_STREAM_PRIORITY_RECORD {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for WM_STREAM_PRIORITY_RECORD {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C, packed(1))]
#[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
pub struct WM_STREAM_TYPE_INFO {
    pub guidMajorType: ::windows::runtime::GUID,
    pub cbFormat: u32,
}
impl WM_STREAM_TYPE_INFO {}
impl ::std::default::Default for WM_STREAM_TYPE_INFO {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::cmp::PartialEq for WM_STREAM_TYPE_INFO {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::std::cmp::Eq for WM_STREAM_TYPE_INFO {}
unsafe impl ::windows::runtime::Abi for WM_STREAM_TYPE_INFO {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C, packed(1))]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_Media_WindowsMediaFormat`, `Win32_Foundation`*"]
pub struct WM_SYNCHRONISED_LYRICS {
    pub bTimeStampFormat: u8,
    pub bContentType: u8,
    pub pwszContentDescriptor: super::super::Foundation::PWSTR,
    pub dwLyricsLen: u32,
    pub pbLyrics: *mut u8,
}
#[cfg(feature = "Win32_Foundation")]
impl WM_SYNCHRONISED_LYRICS {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for WM_SYNCHRONISED_LYRICS {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for WM_SYNCHRONISED_LYRICS {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for WM_SYNCHRONISED_LYRICS {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for WM_SYNCHRONISED_LYRICS {
    type Abi = Self;
    type DefaultType = Self;
}
pub const WM_SampleExtensionGUID_ChromaLocation: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1281019040, 37494, 19244, [158, 76, 160, 237, 239, 221, 33, 126]);
pub const WM_SampleExtensionGUID_ColorSpaceInfo: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(4154120790, 12523, 20267, [159, 122, 242, 75, 19, 154, 17, 87]);
pub const WM_SampleExtensionGUID_ContentType: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3583040544, 1980, 17260, [156, 247, 243, 187, 251, 241, 164, 220]);
pub const WM_SampleExtensionGUID_FileName: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3781553166, 6637, 17879, [180, 167, 37, 203, 209, 226, 142, 155]);
pub const WM_SampleExtensionGUID_OutputCleanPoint: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(4146740335, 28340, 20156, [177, 146, 9, 173, 151, 89, 232, 40]);
pub const WM_SampleExtensionGUID_PixelAspectRatio: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(455009620, 63978, 19400, [130, 26, 55, 107, 116, 228, 196, 184]);
pub const WM_SampleExtensionGUID_SampleDuration: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3334313040, 34431, 18695, [131, 163, 199, 121, 33, 183, 51, 173]);
pub const WM_SampleExtensionGUID_SampleProtectionSalt: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1409539822, 47598, 17295, [170, 131, 56, 4, 153, 126, 86, 157]);
pub const WM_SampleExtensionGUID_Timecode: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(966104556, 34407, 20013, [143, 219, 152, 129, 76, 231, 108, 30]);
pub const WM_SampleExtensionGUID_UserDataInfo: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1932244218, 30910, 17737, [153, 189, 2, 219, 26, 85, 183, 168]);
#[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
pub const WM_SampleExtension_ChromaLocation_Size: u32 = 1u32;
#[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
pub const WM_SampleExtension_ColorSpaceInfo_Size: u32 = 3u32;
#[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
pub const WM_SampleExtension_ContentType_Size: u32 = 1u32;
#[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
pub const WM_SampleExtension_PixelAspectRatio_Size: u32 = 2u32;
#[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
pub const WM_SampleExtension_SampleDuration_Size: u32 = 2u32;
#[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
pub const WM_SampleExtension_Timecode_Size: u32 = 14u32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C, packed(1))]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_Media_WindowsMediaFormat`, `Win32_Foundation`*"]
pub struct WM_USER_TEXT {
    pub pwszDescription: super::super::Foundation::PWSTR,
    pub pwszText: super::super::Foundation::PWSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl WM_USER_TEXT {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for WM_USER_TEXT {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for WM_USER_TEXT {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for WM_USER_TEXT {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for WM_USER_TEXT {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C, packed(1))]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_Media_WindowsMediaFormat`, `Win32_Foundation`*"]
pub struct WM_USER_WEB_URL {
    pub pwszDescription: super::super::Foundation::PWSTR,
    pub pwszURL: super::super::Foundation::PWSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl WM_USER_WEB_URL {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for WM_USER_WEB_URL {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for WM_USER_WEB_URL {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for WM_USER_WEB_URL {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for WM_USER_WEB_URL {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
pub struct WM_WRITER_STATISTICS {
    pub qwSampleCount: u64,
    pub qwByteCount: u64,
    pub qwDroppedSampleCount: u64,
    pub qwDroppedByteCount: u64,
    pub dwCurrentBitrate: u32,
    pub dwAverageBitrate: u32,
    pub dwExpectedBitrate: u32,
    pub dwCurrentSampleRate: u32,
    pub dwAverageSampleRate: u32,
    pub dwExpectedSampleRate: u32,
}
impl WM_WRITER_STATISTICS {}
impl ::std::default::Default for WM_WRITER_STATISTICS {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for WM_WRITER_STATISTICS {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("WM_WRITER_STATISTICS")
            .field("qwSampleCount", &self.qwSampleCount)
            .field("qwByteCount", &self.qwByteCount)
            .field("qwDroppedSampleCount", &self.qwDroppedSampleCount)
            .field("qwDroppedByteCount", &self.qwDroppedByteCount)
            .field("dwCurrentBitrate", &self.dwCurrentBitrate)
            .field("dwAverageBitrate", &self.dwAverageBitrate)
            .field("dwExpectedBitrate", &self.dwExpectedBitrate)
            .field("dwCurrentSampleRate", &self.dwCurrentSampleRate)
            .field("dwAverageSampleRate", &self.dwAverageSampleRate)
            .field("dwExpectedSampleRate", &self.dwExpectedSampleRate)
            .finish()
    }
}
impl ::std::cmp::PartialEq for WM_WRITER_STATISTICS {
    fn eq(&self, other: &Self) -> bool {
        self.qwSampleCount == other.qwSampleCount
            && self.qwByteCount == other.qwByteCount
            && self.qwDroppedSampleCount == other.qwDroppedSampleCount
            && self.qwDroppedByteCount == other.qwDroppedByteCount
            && self.dwCurrentBitrate == other.dwCurrentBitrate
            && self.dwAverageBitrate == other.dwAverageBitrate
            && self.dwExpectedBitrate == other.dwExpectedBitrate
            && self.dwCurrentSampleRate == other.dwCurrentSampleRate
            && self.dwAverageSampleRate == other.dwAverageSampleRate
            && self.dwExpectedSampleRate == other.dwExpectedSampleRate
    }
}
impl ::std::cmp::Eq for WM_WRITER_STATISTICS {}
unsafe impl ::windows::runtime::Abi for WM_WRITER_STATISTICS {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
pub struct WM_WRITER_STATISTICS_EX {
    pub dwBitratePlusOverhead: u32,
    pub dwCurrentSampleDropRateInQueue: u32,
    pub dwCurrentSampleDropRateInCodec: u32,
    pub dwCurrentSampleDropRateInMultiplexer: u32,
    pub dwTotalSampleDropsInQueue: u32,
    pub dwTotalSampleDropsInCodec: u32,
    pub dwTotalSampleDropsInMultiplexer: u32,
}
impl WM_WRITER_STATISTICS_EX {}
impl ::std::default::Default for WM_WRITER_STATISTICS_EX {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for WM_WRITER_STATISTICS_EX {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("WM_WRITER_STATISTICS_EX")
            .field("dwBitratePlusOverhead", &self.dwBitratePlusOverhead)
            .field("dwCurrentSampleDropRateInQueue", &self.dwCurrentSampleDropRateInQueue)
            .field("dwCurrentSampleDropRateInCodec", &self.dwCurrentSampleDropRateInCodec)
            .field("dwCurrentSampleDropRateInMultiplexer", &self.dwCurrentSampleDropRateInMultiplexer)
            .field("dwTotalSampleDropsInQueue", &self.dwTotalSampleDropsInQueue)
            .field("dwTotalSampleDropsInCodec", &self.dwTotalSampleDropsInCodec)
            .field("dwTotalSampleDropsInMultiplexer", &self.dwTotalSampleDropsInMultiplexer)
            .finish()
    }
}
impl ::std::cmp::PartialEq for WM_WRITER_STATISTICS_EX {
    fn eq(&self, other: &Self) -> bool {
        self.dwBitratePlusOverhead == other.dwBitratePlusOverhead
            && self.dwCurrentSampleDropRateInQueue == other.dwCurrentSampleDropRateInQueue
            && self.dwCurrentSampleDropRateInCodec == other.dwCurrentSampleDropRateInCodec
            && self.dwCurrentSampleDropRateInMultiplexer == other.dwCurrentSampleDropRateInMultiplexer
            && self.dwTotalSampleDropsInQueue == other.dwTotalSampleDropsInQueue
            && self.dwTotalSampleDropsInCodec == other.dwTotalSampleDropsInCodec
            && self.dwTotalSampleDropsInMultiplexer == other.dwTotalSampleDropsInMultiplexer
    }
}
impl ::std::cmp::Eq for WM_WRITER_STATISTICS_EX {}
unsafe impl ::windows::runtime::Abi for WM_WRITER_STATISTICS_EX {
    type Abi = Self;
    type DefaultType = Self;
}
#[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct _AM_ASFWRITERCONFIG_PARAM(pub i32);
pub const AM_CONFIGASFWRITER_PARAM_AUTOINDEX: _AM_ASFWRITERCONFIG_PARAM = _AM_ASFWRITERCONFIG_PARAM(1i32);
pub const AM_CONFIGASFWRITER_PARAM_MULTIPASS: _AM_ASFWRITERCONFIG_PARAM = _AM_ASFWRITERCONFIG_PARAM(2i32);
pub const AM_CONFIGASFWRITER_PARAM_DONTCOMPRESS: _AM_ASFWRITERCONFIG_PARAM = _AM_ASFWRITERCONFIG_PARAM(3i32);
impl ::std::convert::From<i32> for _AM_ASFWRITERCONFIG_PARAM {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for _AM_ASFWRITERCONFIG_PARAM {
    type Abi = Self;
    type DefaultType = Self;
}
#[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
pub const g_dwWMContentAttributes: u32 = 5u32;
#[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
pub const g_dwWMNSCAttributes: u32 = 5u32;
#[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
pub const g_dwWMSpecialAttributes: u32 = 20u32;
#[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
pub const g_wszASFLeakyBucketPairs: &'static str = "ASFLeakyBucketPairs";
#[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
pub const g_wszAllowInterlacedOutput: &'static str = "AllowInterlacedOutput";
#[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
pub const g_wszAverageLevel: &'static str = "AverageLevel";
#[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
pub const g_wszBufferAverage: &'static str = "Buffer Average";
#[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
pub const g_wszComplexity: &'static str = "_COMPLEXITYEX";
#[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
pub const g_wszComplexityLive: &'static str = "_COMPLEXITYEXLIVE";
#[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
pub const g_wszComplexityMax: &'static str = "_COMPLEXITYEXMAX";
#[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
pub const g_wszComplexityOffline: &'static str = "_COMPLEXITYEXOFFLINE";
#[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
pub const g_wszDecoderComplexityRequested: &'static str = "_DECODERCOMPLEXITYPROFILE";
#[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
pub const g_wszDedicatedDeliveryThread: &'static str = "DedicatedDeliveryThread";
#[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
pub const g_wszDeinterlaceMode: &'static str = "DeinterlaceMode";
#[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
pub const g_wszDeliverOnReceive: &'static str = "DeliverOnReceive";
#[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
pub const g_wszDeviceConformanceTemplate: &'static str = "DeviceConformanceTemplate";
#[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
pub const g_wszDynamicRangeControl: &'static str = "DynamicRangeControl";
#[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
pub const g_wszEDL: &'static str = "_EDL";
#[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
pub const g_wszEarlyDataDelivery: &'static str = "EarlyDataDelivery";
#[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
pub const g_wszEnableDiscreteOutput: &'static str = "EnableDiscreteOutput";
#[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
pub const g_wszEnableFrameInterpolation: &'static str = "EnableFrameInterpolation";
#[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
pub const g_wszEnableWMAProSPDIFOutput: &'static str = "EnableWMAProSPDIFOutput";
#[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
pub const g_wszFailSeekOnError: &'static str = "FailSeekOnError";
#[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
pub const g_wszFixedFrameRate: &'static str = "FixedFrameRate";
#[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
pub const g_wszFold6To2Channels3: &'static str = "Fold6To2Channels3";
#[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
pub const g_wszFoldToChannelsTemplate: &'static str = "Fold%luTo%luChannels%lu";
#[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
pub const g_wszInitialPatternForInverseTelecine: &'static str = "InitialPatternForInverseTelecine";
#[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
pub const g_wszInterlacedCoding: &'static str = "InterlacedCoding";
#[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
pub const g_wszIsVBRSupported: &'static str = "_ISVBRSUPPORTED";
#[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
pub const g_wszJPEGCompressionQuality: &'static str = "JPEGCompressionQuality";
#[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
pub const g_wszJustInTimeDecode: &'static str = "JustInTimeDecode";
#[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
pub const g_wszMixedClassMode: &'static str = "MixedClassMode";
#[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
pub const g_wszMusicClassMode: &'static str = "MusicClassMode";
#[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
pub const g_wszMusicSpeechClassMode: &'static str = "MusicSpeechClassMode";
#[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
pub const g_wszNeedsPreviousSample: &'static str = "NeedsPreviousSample";
#[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
pub const g_wszNumPasses: &'static str = "_PASSESUSED";
#[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
pub const g_wszOriginalSourceFormatTag: &'static str = "_SOURCEFORMATTAG";
#[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
pub const g_wszOriginalWaveFormat: &'static str = "_ORIGINALWAVEFORMAT";
#[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
pub const g_wszPeakValue: &'static str = "PeakValue";
#[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
pub const g_wszPermitSeeksBeyondEndOfStream: &'static str = "PermitSeeksBeyondEndOfStream";
#[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
pub const g_wszReloadIndexOnSeek: &'static str = "ReloadIndexOnSeek";
#[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
pub const g_wszScrambledAudio: &'static str = "ScrambledAudio";
#[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
pub const g_wszSingleOutputBuffer: &'static str = "SingleOutputBuffer";
#[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
pub const g_wszSoftwareScaling: &'static str = "SoftwareScaling";
#[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
pub const g_wszSourceBufferTime: &'static str = "SourceBufferTime";
#[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
pub const g_wszSourceMaxBytesAtOnce: &'static str = "SourceMaxBytesAtOnce";
#[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
pub const g_wszSpeakerConfig: &'static str = "SpeakerConfig";
#[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
pub const g_wszSpeechCaps: &'static str = "SpeechFormatCap";
#[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
pub const g_wszSpeechClassMode: &'static str = "SpeechClassMode";
#[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
pub const g_wszStreamLanguage: &'static str = "StreamLanguage";
#[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
pub const g_wszStreamNumIndexObjects: &'static str = "StreamNumIndexObjects";
#[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
pub const g_wszUsePacketAtSeekPoint: &'static str = "UsePacketAtSeekPoint";
#[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
pub const g_wszVBRBitrateMax: &'static str = "_RMAX";
#[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
pub const g_wszVBRBufferWindowMax: &'static str = "_BMAX";
#[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
pub const g_wszVBREnabled: &'static str = "_VBRENABLED";
#[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
pub const g_wszVBRPeak: &'static str = "VBR Peak";
#[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
pub const g_wszVBRQuality: &'static str = "_VBRQUALITY";
#[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
pub const g_wszVideoSampleDurations: &'static str = "VideoSampleDurations";
#[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
pub const g_wszWMADID: &'static str = "WM/ADID";
#[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
pub const g_wszWMASFPacketCount: &'static str = "WM/ASFPacketCount";
#[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
pub const g_wszWMASFSecurityObjectsSize: &'static str = "WM/ASFSecurityObjectsSize";
#[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
pub const g_wszWMAlbumArtist: &'static str = "WM/AlbumArtist";
#[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
pub const g_wszWMAlbumArtistSort: &'static str = "WM/AlbumArtistSort";
#[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
pub const g_wszWMAlbumCoverURL: &'static str = "WM/AlbumCoverURL";
#[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
pub const g_wszWMAlbumTitle: &'static str = "WM/AlbumTitle";
#[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
pub const g_wszWMAlbumTitleSort: &'static str = "WM/AlbumTitleSort";
#[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
pub const g_wszWMAspectRatioX: &'static str = "AspectRatioX";
#[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
pub const g_wszWMAspectRatioY: &'static str = "AspectRatioY";
#[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
pub const g_wszWMAudioFileURL: &'static str = "WM/AudioFileURL";
#[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
pub const g_wszWMAudioSourceURL: &'static str = "WM/AudioSourceURL";
#[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
pub const g_wszWMAuthor: &'static str = "Author";
#[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
pub const g_wszWMAuthorSort: &'static str = "AuthorSort";
#[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
pub const g_wszWMAuthorURL: &'static str = "WM/AuthorURL";
#[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
pub const g_wszWMBannerImageData: &'static str = "BannerImageData";
#[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
pub const g_wszWMBannerImageType: &'static str = "BannerImageType";
#[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
pub const g_wszWMBannerImageURL: &'static str = "BannerImageURL";
#[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
pub const g_wszWMBeatsPerMinute: &'static str = "WM/BeatsPerMinute";
#[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
pub const g_wszWMBitrate: &'static str = "Bitrate";
#[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
pub const g_wszWMBroadcast: &'static str = "Broadcast";
#[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
pub const g_wszWMCategory: &'static str = "WM/Category";
#[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
pub const g_wszWMCodec: &'static str = "WM/Codec";
#[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
pub const g_wszWMComposer: &'static str = "WM/Composer";
#[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
pub const g_wszWMComposerSort: &'static str = "WM/ComposerSort";
#[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
pub const g_wszWMConductor: &'static str = "WM/Conductor";
#[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
pub const g_wszWMContainerFormat: &'static str = "WM/ContainerFormat";
#[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
pub const g_wszWMContentDistributor: &'static str = "WM/ContentDistributor";
#[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
pub const g_wszWMContentGroupDescription: &'static str = "WM/ContentGroupDescription";
#[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
pub const g_wszWMCopyright: &'static str = "Copyright";
#[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
pub const g_wszWMCopyrightURL: &'static str = "CopyrightURL";
#[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
pub const g_wszWMCurrentBitrate: &'static str = "CurrentBitrate";
#[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
pub const g_wszWMDRM: &'static str = "WM/DRM";
#[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
pub const g_wszWMDRM_ContentID: &'static str = "DRM_ContentID";
#[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
pub const g_wszWMDRM_Flags: &'static str = "DRM_Flags";
#[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
pub const g_wszWMDRM_HeaderSignPrivKey: &'static str = "DRM_HeaderSignPrivKey";
#[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
pub const g_wszWMDRM_IndividualizedVersion: &'static str = "DRM_IndividualizedVersion";
#[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
pub const g_wszWMDRM_KeyID: &'static str = "DRM_KeyID";
#[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
pub const g_wszWMDRM_KeySeed: &'static str = "DRM_KeySeed";
#[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
pub const g_wszWMDRM_LASignatureCert: &'static str = "DRM_LASignatureCert";
#[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
pub const g_wszWMDRM_LASignatureLicSrvCert: &'static str = "DRM_LASignatureLicSrvCert";
#[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
pub const g_wszWMDRM_LASignaturePrivKey: &'static str = "DRM_LASignaturePrivKey";
#[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
pub const g_wszWMDRM_LASignatureRootCert: &'static str = "DRM_LASignatureRootCert";
#[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
pub const g_wszWMDRM_Level: &'static str = "DRM_Level";
#[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
pub const g_wszWMDRM_LicenseAcqURL: &'static str = "DRM_LicenseAcqURL";
#[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
pub const g_wszWMDRM_SourceID: &'static str = "DRM_SourceID";
#[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
pub const g_wszWMDRM_V1LicenseAcqURL: &'static str = "DRM_V1LicenseAcqURL";
#[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
pub const g_wszWMDVDID: &'static str = "WM/DVDID";
#[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
pub const g_wszWMDescription: &'static str = "Description";
#[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
pub const g_wszWMDirector: &'static str = "WM/Director";
#[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
pub const g_wszWMDuration: &'static str = "Duration";
#[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
pub const g_wszWMEncodedBy: &'static str = "WM/EncodedBy";
#[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
pub const g_wszWMEncodingSettings: &'static str = "WM/EncodingSettings";
#[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
pub const g_wszWMEncodingTime: &'static str = "WM/EncodingTime";
#[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
pub const g_wszWMEpisodeNumber: &'static str = "WM/EpisodeNumber";
#[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
pub const g_wszWMFileSize: &'static str = "FileSize";
#[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
pub const g_wszWMGenre: &'static str = "WM/Genre";
#[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
pub const g_wszWMGenreID: &'static str = "WM/GenreID";
#[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
pub const g_wszWMHasArbitraryDataStream: &'static str = "HasArbitraryDataStream";
#[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
pub const g_wszWMHasAttachedImages: &'static str = "HasAttachedImages";
#[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
pub const g_wszWMHasAudio: &'static str = "HasAudio";
#[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
pub const g_wszWMHasFileTransferStream: &'static str = "HasFileTransferStream";
#[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
pub const g_wszWMHasImage: &'static str = "HasImage";
#[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
pub const g_wszWMHasScript: &'static str = "HasScript";
#[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
pub const g_wszWMHasVideo: &'static str = "HasVideo";
#[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
pub const g_wszWMISAN: &'static str = "WM/ISAN";
#[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
pub const g_wszWMISRC: &'static str = "WM/ISRC";
#[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
pub const g_wszWMInitialKey: &'static str = "WM/InitialKey";
#[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
pub const g_wszWMIsCompilation: &'static str = "WM/IsCompilation";
#[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
pub const g_wszWMIsVBR: &'static str = "IsVBR";
#[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
pub const g_wszWMLanguage: &'static str = "WM/Language";
#[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
pub const g_wszWMLyrics: &'static str = "WM/Lyrics";
#[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
pub const g_wszWMLyrics_Synchronised: &'static str = "WM/Lyrics_Synchronised";
#[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
pub const g_wszWMMCDI: &'static str = "WM/MCDI";
#[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
pub const g_wszWMMediaClassPrimaryID: &'static str = "WM/MediaClassPrimaryID";
#[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
pub const g_wszWMMediaClassSecondaryID: &'static str = "WM/MediaClassSecondaryID";
#[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
pub const g_wszWMMediaCredits: &'static str = "WM/MediaCredits";
#[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
pub const g_wszWMMediaIsDelay: &'static str = "WM/MediaIsDelay";
#[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
pub const g_wszWMMediaIsFinale: &'static str = "WM/MediaIsFinale";
#[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
pub const g_wszWMMediaIsLive: &'static str = "WM/MediaIsLive";
#[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
pub const g_wszWMMediaIsPremiere: &'static str = "WM/MediaIsPremiere";
#[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
pub const g_wszWMMediaIsRepeat: &'static str = "WM/MediaIsRepeat";
#[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
pub const g_wszWMMediaIsSAP: &'static str = "WM/MediaIsSAP";
#[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
pub const g_wszWMMediaIsStereo: &'static str = "WM/MediaIsStereo";
#[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
pub const g_wszWMMediaIsSubtitled: &'static str = "WM/MediaIsSubtitled";
#[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
pub const g_wszWMMediaIsTape: &'static str = "WM/MediaIsTape";
#[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
pub const g_wszWMMediaNetworkAffiliation: &'static str = "WM/MediaNetworkAffiliation";
#[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
pub const g_wszWMMediaOriginalBroadcastDateTime: &'static str = "WM/MediaOriginalBroadcastDateTime";
#[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
pub const g_wszWMMediaOriginalChannel: &'static str = "WM/MediaOriginalChannel";
#[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
pub const g_wszWMMediaStationCallSign: &'static str = "WM/MediaStationCallSign";
#[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
pub const g_wszWMMediaStationName: &'static str = "WM/MediaStationName";
#[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
pub const g_wszWMModifiedBy: &'static str = "WM/ModifiedBy";
#[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
pub const g_wszWMMood: &'static str = "WM/Mood";
#[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
pub const g_wszWMNSCAddress: &'static str = "NSC_Address";
#[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
pub const g_wszWMNSCDescription: &'static str = "NSC_Description";
#[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
pub const g_wszWMNSCEmail: &'static str = "NSC_Email";
#[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
pub const g_wszWMNSCName: &'static str = "NSC_Name";
#[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
pub const g_wszWMNSCPhone: &'static str = "NSC_Phone";
#[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
pub const g_wszWMNumberOfFrames: &'static str = "NumberOfFrames";
#[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
pub const g_wszWMOptimalBitrate: &'static str = "OptimalBitrate";
#[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
pub const g_wszWMOriginalAlbumTitle: &'static str = "WM/OriginalAlbumTitle";
#[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
pub const g_wszWMOriginalArtist: &'static str = "WM/OriginalArtist";
#[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
pub const g_wszWMOriginalFilename: &'static str = "WM/OriginalFilename";
#[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
pub const g_wszWMOriginalLyricist: &'static str = "WM/OriginalLyricist";
#[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
pub const g_wszWMOriginalReleaseTime: &'static str = "WM/OriginalReleaseTime";
#[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
pub const g_wszWMOriginalReleaseYear: &'static str = "WM/OriginalReleaseYear";
#[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
pub const g_wszWMParentalRating: &'static str = "WM/ParentalRating";
#[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
pub const g_wszWMParentalRatingReason: &'static str = "WM/ParentalRatingReason";
#[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
pub const g_wszWMPartOfSet: &'static str = "WM/PartOfSet";
#[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
pub const g_wszWMPeakBitrate: &'static str = "WM/PeakBitrate";
#[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
pub const g_wszWMPeriod: &'static str = "WM/Period";
#[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
pub const g_wszWMPicture: &'static str = "WM/Picture";
#[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
pub const g_wszWMPlaylistDelay: &'static str = "WM/PlaylistDelay";
#[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
pub const g_wszWMProducer: &'static str = "WM/Producer";
#[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
pub const g_wszWMPromotionURL: &'static str = "WM/PromotionURL";
#[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
pub const g_wszWMProtected: &'static str = "Is_Protected";
#[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
pub const g_wszWMProtectionType: &'static str = "WM/ProtectionType";
#[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
pub const g_wszWMProvider: &'static str = "WM/Provider";
#[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
pub const g_wszWMProviderCopyright: &'static str = "WM/ProviderCopyright";
#[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
pub const g_wszWMProviderRating: &'static str = "WM/ProviderRating";
#[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
pub const g_wszWMProviderStyle: &'static str = "WM/ProviderStyle";
#[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
pub const g_wszWMPublisher: &'static str = "WM/Publisher";
#[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
pub const g_wszWMRadioStationName: &'static str = "WM/RadioStationName";
#[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
pub const g_wszWMRadioStationOwner: &'static str = "WM/RadioStationOwner";
#[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
pub const g_wszWMRating: &'static str = "Rating";
#[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
pub const g_wszWMSeasonNumber: &'static str = "WM/SeasonNumber";
#[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
pub const g_wszWMSeekable: &'static str = "Seekable";
#[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
pub const g_wszWMSharedUserRating: &'static str = "WM/SharedUserRating";
#[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
pub const g_wszWMSignature_Name: &'static str = "Signature_Name";
#[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
pub const g_wszWMSkipBackward: &'static str = "Can_Skip_Backward";
#[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
pub const g_wszWMSkipForward: &'static str = "Can_Skip_Forward";
#[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
pub const g_wszWMStreamTypeInfo: &'static str = "WM/StreamTypeInfo";
#[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
pub const g_wszWMStridable: &'static str = "Stridable";
#[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
pub const g_wszWMSubTitle: &'static str = "WM/SubTitle";
#[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
pub const g_wszWMSubTitleDescription: &'static str = "WM/SubTitleDescription";
#[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
pub const g_wszWMSubscriptionContentID: &'static str = "WM/SubscriptionContentID";
#[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
pub const g_wszWMText: &'static str = "WM/Text";
#[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
pub const g_wszWMTitle: &'static str = "Title";
#[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
pub const g_wszWMTitleSort: &'static str = "TitleSort";
#[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
pub const g_wszWMToolName: &'static str = "WM/ToolName";
#[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
pub const g_wszWMToolVersion: &'static str = "WM/ToolVersion";
#[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
pub const g_wszWMTrack: &'static str = "WM/Track";
#[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
pub const g_wszWMTrackNumber: &'static str = "WM/TrackNumber";
#[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
pub const g_wszWMTrusted: &'static str = "Is_Trusted";
#[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
pub const g_wszWMUniqueFileIdentifier: &'static str = "WM/UniqueFileIdentifier";
#[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
pub const g_wszWMUse_Advanced_DRM: &'static str = "Use_Advanced_DRM";
#[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
pub const g_wszWMUse_DRM: &'static str = "Use_DRM";
#[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
pub const g_wszWMUserWebURL: &'static str = "WM/UserWebURL";
#[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
pub const g_wszWMVideoClosedCaptioning: &'static str = "WM/VideoClosedCaptioning";
#[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
pub const g_wszWMVideoFrameRate: &'static str = "WM/VideoFrameRate";
#[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
pub const g_wszWMVideoHeight: &'static str = "WM/VideoHeight";
#[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
pub const g_wszWMVideoWidth: &'static str = "WM/VideoWidth";
#[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
pub const g_wszWMWMADRCAverageReference: &'static str = "WM/WMADRCAverageReference";
#[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
pub const g_wszWMWMADRCAverageTarget: &'static str = "WM/WMADRCAverageTarget";
#[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
pub const g_wszWMWMADRCPeakReference: &'static str = "WM/WMADRCPeakReference";
#[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
pub const g_wszWMWMADRCPeakTarget: &'static str = "WM/WMADRCPeakTarget";
#[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
pub const g_wszWMWMCPDistributor: &'static str = "WM/WMCPDistributor";
#[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
pub const g_wszWMWMCPDistributorID: &'static str = "WM/WMCPDistributorID";
#[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
pub const g_wszWMWMCollectionGroupID: &'static str = "WM/WMCollectionGroupID";
#[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
pub const g_wszWMWMCollectionID: &'static str = "WM/WMCollectionID";
#[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
pub const g_wszWMWMContentID: &'static str = "WM/WMContentID";
#[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
pub const g_wszWMWMShadowFileSourceDRMType: &'static str = "WM/WMShadowFileSourceDRMType";
#[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
pub const g_wszWMWMShadowFileSourceFileType: &'static str = "WM/WMShadowFileSourceFileType";
#[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
pub const g_wszWMWriter: &'static str = "WM/Writer";
#[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
pub const g_wszWMYear: &'static str = "WM/Year";
#[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
pub const g_wszWatermarkCLSID: &'static str = "WatermarkCLSID";
#[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
pub const g_wszWatermarkConfig: &'static str = "WatermarkConfig";
