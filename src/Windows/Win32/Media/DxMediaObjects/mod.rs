#![allow(unused_variables, non_upper_case_globals, non_snake_case, unused_unsafe, non_camel_case_types, dead_code, clippy::all)]
pub const DMOCATEGORY_ACOUSTIC_ECHO_CANCEL: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3214294400, 50521, 4560, [138, 43, 0, 160, 201, 37, 90, 193]);
pub const DMOCATEGORY_AGC: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3901528992, 50519, 4560, [138, 43, 0, 160, 201, 37, 90, 193]);
pub const DMOCATEGORY_AUDIO_CAPTURE_EFFECT: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(4133857978, 15881, 18720, [170, 95, 33, 152, 17, 20, 143, 9]);
pub const DMOCATEGORY_AUDIO_DECODER: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1475533707, 59067, 17683, [157, 67, 220, 210, 166, 89, 49, 37]);
pub const DMOCATEGORY_AUDIO_EFFECT: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(4083166015, 1426, 18655, [164, 205, 103, 71, 33, 231, 235, 235]);
pub const DMOCATEGORY_AUDIO_ENCODER: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(869902177, 37064, 4560, [189, 67, 0, 160, 201, 17, 206, 134]);
pub const DMOCATEGORY_AUDIO_NOISE_SUPPRESS: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3766456383, 25341, 20064, [140, 221, 222, 167, 35, 102, 101, 181]);
pub const DMOCATEGORY_VIDEO_DECODER: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1248441410, 10430, 18833, [150, 156, 181, 0, 173, 245, 216, 168]);
pub const DMOCATEGORY_VIDEO_EFFECT: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3650154004, 30572, 18211, [190, 70, 61, 162, 245, 111, 16, 185]);
pub const DMOCATEGORY_VIDEO_ENCODER: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(869902176, 37064, 4560, [189, 67, 0, 160, 201, 17, 206, 134]);
#[doc = "*Required features: `Win32_Media_DxMediaObjects`*"]
#[inline]
pub unsafe fn DMOEnum(guidcategory: *const ::windows::runtime::GUID, dwflags: u32, cintypes: u32, pintypes: *const DMO_PARTIAL_MEDIATYPE, couttypes: u32, pouttypes: *const DMO_PARTIAL_MEDIATYPE) -> ::windows::runtime::Result<IEnumDMO> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DMOEnum(guidcategory: *const ::windows::runtime::GUID, dwflags: u32, cintypes: u32, pintypes: *const DMO_PARTIAL_MEDIATYPE, couttypes: u32, pouttypes: *const DMO_PARTIAL_MEDIATYPE, ppenum: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <IEnumDMO as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        DMOEnum(::core::mem::transmute(guidcategory), ::core::mem::transmute(dwflags), ::core::mem::transmute(cintypes), ::core::mem::transmute(pintypes), ::core::mem::transmute(couttypes), ::core::mem::transmute(pouttypes), &mut result__).from_abi::<IEnumDMO>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Media_DxMediaObjects`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DMOGetName(clsiddmo: *const ::windows::runtime::GUID, szname: super::super::Foundation::PWSTR) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DMOGetName(clsiddmo: *const ::windows::runtime::GUID, szname: super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT;
        }
        DMOGetName(::core::mem::transmute(clsiddmo), ::core::mem::transmute(szname)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Media_DxMediaObjects`*"]
#[inline]
pub unsafe fn DMOGetTypes(clsiddmo: *const ::windows::runtime::GUID, ulinputtypesrequested: u32, pulinputtypessupplied: *mut u32, pinputtypes: *mut DMO_PARTIAL_MEDIATYPE, uloutputtypesrequested: u32, puloutputtypessupplied: *mut u32, poutputtypes: *mut DMO_PARTIAL_MEDIATYPE) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DMOGetTypes(clsiddmo: *const ::windows::runtime::GUID, ulinputtypesrequested: u32, pulinputtypessupplied: *mut u32, pinputtypes: *mut DMO_PARTIAL_MEDIATYPE, uloutputtypesrequested: u32, puloutputtypessupplied: *mut u32, poutputtypes: *mut DMO_PARTIAL_MEDIATYPE) -> ::windows::runtime::HRESULT;
        }
        DMOGetTypes(::core::mem::transmute(clsiddmo), ::core::mem::transmute(ulinputtypesrequested), ::core::mem::transmute(pulinputtypessupplied), ::core::mem::transmute(pinputtypes), ::core::mem::transmute(uloutputtypesrequested), ::core::mem::transmute(puloutputtypessupplied), ::core::mem::transmute(poutputtypes)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Media_DxMediaObjects`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DMORegister<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(szname: Param0, clsiddmo: *const ::windows::runtime::GUID, guidcategory: *const ::windows::runtime::GUID, dwflags: u32, cintypes: u32, pintypes: *const DMO_PARTIAL_MEDIATYPE, couttypes: u32, pouttypes: *const DMO_PARTIAL_MEDIATYPE) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DMORegister(szname: super::super::Foundation::PWSTR, clsiddmo: *const ::windows::runtime::GUID, guidcategory: *const ::windows::runtime::GUID, dwflags: u32, cintypes: u32, pintypes: *const DMO_PARTIAL_MEDIATYPE, couttypes: u32, pouttypes: *const DMO_PARTIAL_MEDIATYPE) -> ::windows::runtime::HRESULT;
        }
        DMORegister(szname.into_param().abi(), ::core::mem::transmute(clsiddmo), ::core::mem::transmute(guidcategory), ::core::mem::transmute(dwflags), ::core::mem::transmute(cintypes), ::core::mem::transmute(pintypes), ::core::mem::transmute(couttypes), ::core::mem::transmute(pouttypes)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Media_DxMediaObjects`*"]
#[inline]
pub unsafe fn DMOUnregister(clsiddmo: *const ::windows::runtime::GUID, guidcategory: *const ::windows::runtime::GUID) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DMOUnregister(clsiddmo: *const ::windows::runtime::GUID, guidcategory: *const ::windows::runtime::GUID) -> ::windows::runtime::HRESULT;
        }
        DMOUnregister(::core::mem::transmute(clsiddmo), ::core::mem::transmute(guidcategory)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Media_DxMediaObjects`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct DMO_ENUM_FLAGS(pub i32);
pub const DMO_ENUMF_INCLUDE_KEYED: DMO_ENUM_FLAGS = DMO_ENUM_FLAGS(1i32);
impl ::core::convert::From<i32> for DMO_ENUM_FLAGS {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for DMO_ENUM_FLAGS {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Media_DxMediaObjects`*"]
pub const DMO_E_INVALIDSTREAMINDEX: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147220991i32 as _);
#[doc = "*Required features: `Win32_Media_DxMediaObjects`*"]
pub const DMO_E_INVALIDTYPE: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147220990i32 as _);
#[doc = "*Required features: `Win32_Media_DxMediaObjects`*"]
pub const DMO_E_NOTACCEPTING: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147220988i32 as _);
#[doc = "*Required features: `Win32_Media_DxMediaObjects`*"]
pub const DMO_E_NO_MORE_ITEMS: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147220986i32 as _);
#[doc = "*Required features: `Win32_Media_DxMediaObjects`*"]
pub const DMO_E_TYPE_NOT_ACCEPTED: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147220987i32 as _);
#[doc = "*Required features: `Win32_Media_DxMediaObjects`*"]
pub const DMO_E_TYPE_NOT_SET: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147220989i32 as _);
#[derive(:: core :: clone :: Clone)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_Media_DxMediaObjects`, `Win32_Foundation`*"]
pub struct DMO_MEDIA_TYPE {
    pub majortype: ::windows::runtime::GUID,
    pub subtype: ::windows::runtime::GUID,
    pub bFixedSizeSamples: super::super::Foundation::BOOL,
    pub bTemporalCompression: super::super::Foundation::BOOL,
    pub lSampleSize: u32,
    pub formattype: ::windows::runtime::GUID,
    pub pUnk: ::core::option::Option<::windows::runtime::IUnknown>,
    pub cbFormat: u32,
    pub pbFormat: *mut u8,
}
#[cfg(feature = "Win32_Foundation")]
impl DMO_MEDIA_TYPE {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DMO_MEDIA_TYPE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for DMO_MEDIA_TYPE {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("DMO_MEDIA_TYPE")
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
impl ::core::cmp::PartialEq for DMO_MEDIA_TYPE {
    fn eq(&self, other: &Self) -> bool {
        self.majortype == other.majortype && self.subtype == other.subtype && self.bFixedSizeSamples == other.bFixedSizeSamples && self.bTemporalCompression == other.bTemporalCompression && self.lSampleSize == other.lSampleSize && self.formattype == other.formattype && self.pUnk == other.pUnk && self.cbFormat == other.cbFormat && self.pbFormat == other.pbFormat
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DMO_MEDIA_TYPE {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for DMO_MEDIA_TYPE {
    type Abi = ::core::mem::ManuallyDrop<Self>;
}
#[derive(:: core :: clone :: Clone)]
#[repr(C)]
#[doc = "*Required features: `Win32_Media_DxMediaObjects`*"]
pub struct DMO_OUTPUT_DATA_BUFFER {
    pub pBuffer: ::core::option::Option<IMediaBuffer>,
    pub dwStatus: u32,
    pub rtTimestamp: i64,
    pub rtTimelength: i64,
}
impl DMO_OUTPUT_DATA_BUFFER {}
impl ::core::default::Default for DMO_OUTPUT_DATA_BUFFER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for DMO_OUTPUT_DATA_BUFFER {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("DMO_OUTPUT_DATA_BUFFER").field("pBuffer", &self.pBuffer).field("dwStatus", &self.dwStatus).field("rtTimestamp", &self.rtTimestamp).field("rtTimelength", &self.rtTimelength).finish()
    }
}
impl ::core::cmp::PartialEq for DMO_OUTPUT_DATA_BUFFER {
    fn eq(&self, other: &Self) -> bool {
        self.pBuffer == other.pBuffer && self.dwStatus == other.dwStatus && self.rtTimestamp == other.rtTimestamp && self.rtTimelength == other.rtTimelength
    }
}
impl ::core::cmp::Eq for DMO_OUTPUT_DATA_BUFFER {}
unsafe impl ::windows::runtime::Abi for DMO_OUTPUT_DATA_BUFFER {
    type Abi = ::core::mem::ManuallyDrop<Self>;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Media_DxMediaObjects`*"]
pub struct DMO_PARTIAL_MEDIATYPE {
    pub r#type: ::windows::runtime::GUID,
    pub subtype: ::windows::runtime::GUID,
}
impl DMO_PARTIAL_MEDIATYPE {}
impl ::core::default::Default for DMO_PARTIAL_MEDIATYPE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for DMO_PARTIAL_MEDIATYPE {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("DMO_PARTIAL_MEDIATYPE").field("r#type", &self.r#type).field("subtype", &self.subtype).finish()
    }
}
impl ::core::cmp::PartialEq for DMO_PARTIAL_MEDIATYPE {
    fn eq(&self, other: &Self) -> bool {
        self.r#type == other.r#type && self.subtype == other.subtype
    }
}
impl ::core::cmp::Eq for DMO_PARTIAL_MEDIATYPE {}
unsafe impl ::windows::runtime::Abi for DMO_PARTIAL_MEDIATYPE {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Media_DxMediaObjects`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct DMO_REGISTER_FLAGS(pub i32);
pub const DMO_REGISTERF_IS_KEYED: DMO_REGISTER_FLAGS = DMO_REGISTER_FLAGS(1i32);
impl ::core::convert::From<i32> for DMO_REGISTER_FLAGS {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for DMO_REGISTER_FLAGS {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Media_DxMediaObjects`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IDMOQualityControl(pub ::windows::runtime::IUnknown);
impl IDMOQualityControl {
    #[doc = "*Required features: `Win32_Media_DxMediaObjects`*"]
    pub unsafe fn SetNow(&self, rtnow: i64) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(rtnow)).ok()
    }
    #[doc = "*Required features: `Win32_Media_DxMediaObjects`*"]
    pub unsafe fn SetStatus(&self, dwflags: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwflags)).ok()
    }
    #[doc = "*Required features: `Win32_Media_DxMediaObjects`*"]
    pub unsafe fn GetStatus(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).5)(::core::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IDMOQualityControl {
    type Vtable = IDMOQualityControl_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1705765526, 53046, 17727, [175, 138, 112, 94, 152, 241, 98, 96]);
}
impl ::core::convert::From<IDMOQualityControl> for ::windows::runtime::IUnknown {
    fn from(value: IDMOQualityControl) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IDMOQualityControl> for ::windows::runtime::IUnknown {
    fn from(value: &IDMOQualityControl) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IDMOQualityControl {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IDMOQualityControl {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDMOQualityControl_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, rtnow: i64) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwflags: u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pdwflags: *mut u32) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_Media_DxMediaObjects`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IDMOVideoOutputOptimizations(pub ::windows::runtime::IUnknown);
impl IDMOVideoOutputOptimizations {
    #[doc = "*Required features: `Win32_Media_DxMediaObjects`*"]
    pub unsafe fn QueryOperationModePreferences(&self, uloutputstreamindex: u32) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(uloutputstreamindex), &mut result__).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `Win32_Media_DxMediaObjects`*"]
    pub unsafe fn SetOperationMode(&self, uloutputstreamindex: u32, dwenabledfeatures: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(uloutputstreamindex), ::core::mem::transmute(dwenabledfeatures)).ok()
    }
    #[doc = "*Required features: `Win32_Media_DxMediaObjects`*"]
    pub unsafe fn GetCurrentOperationMode(&self, uloutputstreamindex: u32) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(uloutputstreamindex), &mut result__).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `Win32_Media_DxMediaObjects`*"]
    pub unsafe fn GetCurrentSampleRequirements(&self, uloutputstreamindex: u32) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(uloutputstreamindex), &mut result__).from_abi::<u32>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IDMOVideoOutputOptimizations {
    type Vtable = IDMOVideoOutputOptimizations_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3197062990, 23318, 19753, [179, 80, 127, 107, 93, 146, 152, 172]);
}
impl ::core::convert::From<IDMOVideoOutputOptimizations> for ::windows::runtime::IUnknown {
    fn from(value: IDMOVideoOutputOptimizations) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IDMOVideoOutputOptimizations> for ::windows::runtime::IUnknown {
    fn from(value: &IDMOVideoOutputOptimizations) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IDMOVideoOutputOptimizations {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IDMOVideoOutputOptimizations {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDMOVideoOutputOptimizations_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, uloutputstreamindex: u32, pdwrequestedcapabilities: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, uloutputstreamindex: u32, dwenabledfeatures: u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, uloutputstreamindex: u32, pdwenabledfeatures: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, uloutputstreamindex: u32, pdwrequestedfeatures: *mut u32) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_Media_DxMediaObjects`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IEnumDMO(pub ::windows::runtime::IUnknown);
impl IEnumDMO {
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_DxMediaObjects`, `Win32_Foundation`*"]
    pub unsafe fn Next(&self, citemstofetch: u32, pclsid: *mut ::windows::runtime::GUID, names: *mut super::super::Foundation::PWSTR, pcitemsfetched: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(citemstofetch), ::core::mem::transmute(pclsid), ::core::mem::transmute(names), ::core::mem::transmute(pcitemsfetched)).ok()
    }
    #[doc = "*Required features: `Win32_Media_DxMediaObjects`*"]
    pub unsafe fn Skip(&self, citemstoskip: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(citemstoskip)).ok()
    }
    #[doc = "*Required features: `Win32_Media_DxMediaObjects`*"]
    pub unsafe fn Reset(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `Win32_Media_DxMediaObjects`*"]
    pub unsafe fn Clone(&self) -> ::windows::runtime::Result<IEnumDMO> {
        let mut result__: <IEnumDMO as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).6)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IEnumDMO>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IEnumDMO {
    type Vtable = IEnumDMO_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(742185354, 11258, 19027, [156, 39, 82, 73, 186, 100, 186, 15]);
}
impl ::core::convert::From<IEnumDMO> for ::windows::runtime::IUnknown {
    fn from(value: IEnumDMO) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IEnumDMO> for ::windows::runtime::IUnknown {
    fn from(value: &IEnumDMO) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IEnumDMO {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IEnumDMO {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IEnumDMO_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, citemstofetch: u32, pclsid: *mut ::windows::runtime::GUID, names: *mut super::super::Foundation::PWSTR, pcitemsfetched: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, citemstoskip: u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppenum: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_Media_DxMediaObjects`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IMediaBuffer(pub ::windows::runtime::IUnknown);
impl IMediaBuffer {
    #[doc = "*Required features: `Win32_Media_DxMediaObjects`*"]
    pub unsafe fn SetLength(&self, cblength: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(cblength)).ok()
    }
    #[doc = "*Required features: `Win32_Media_DxMediaObjects`*"]
    pub unsafe fn GetMaxLength(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `Win32_Media_DxMediaObjects`*"]
    pub unsafe fn GetBufferAndLength(&self, ppbuffer: *mut *mut u8, pcblength: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(ppbuffer), ::core::mem::transmute(pcblength)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IMediaBuffer {
    type Vtable = IMediaBuffer_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1508899001, 37772, 18982, [130, 242, 149, 203, 132, 205, 200, 55]);
}
impl ::core::convert::From<IMediaBuffer> for ::windows::runtime::IUnknown {
    fn from(value: IMediaBuffer) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IMediaBuffer> for ::windows::runtime::IUnknown {
    fn from(value: &IMediaBuffer) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IMediaBuffer {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IMediaBuffer {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaBuffer_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, cblength: u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pcbmaxlength: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppbuffer: *mut *mut u8, pcblength: *mut u32) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_Media_DxMediaObjects`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IMediaObject(pub ::windows::runtime::IUnknown);
impl IMediaObject {
    #[doc = "*Required features: `Win32_Media_DxMediaObjects`*"]
    pub unsafe fn GetStreamCount(&self, pcinputstreams: *mut u32, pcoutputstreams: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(pcinputstreams), ::core::mem::transmute(pcoutputstreams)).ok()
    }
    #[doc = "*Required features: `Win32_Media_DxMediaObjects`*"]
    pub unsafe fn GetInputStreamInfo(&self, dwinputstreamindex: u32) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwinputstreamindex), &mut result__).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `Win32_Media_DxMediaObjects`*"]
    pub unsafe fn GetOutputStreamInfo(&self, dwoutputstreamindex: u32) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwoutputstreamindex), &mut result__).from_abi::<u32>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_DxMediaObjects`, `Win32_Foundation`*"]
    pub unsafe fn GetInputType(&self, dwinputstreamindex: u32, dwtypeindex: u32) -> ::windows::runtime::Result<DMO_MEDIA_TYPE> {
        let mut result__: <DMO_MEDIA_TYPE as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwinputstreamindex), ::core::mem::transmute(dwtypeindex), &mut result__).from_abi::<DMO_MEDIA_TYPE>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_DxMediaObjects`, `Win32_Foundation`*"]
    pub unsafe fn GetOutputType(&self, dwoutputstreamindex: u32, dwtypeindex: u32) -> ::windows::runtime::Result<DMO_MEDIA_TYPE> {
        let mut result__: <DMO_MEDIA_TYPE as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwoutputstreamindex), ::core::mem::transmute(dwtypeindex), &mut result__).from_abi::<DMO_MEDIA_TYPE>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_DxMediaObjects`, `Win32_Foundation`*"]
    pub unsafe fn SetInputType(&self, dwinputstreamindex: u32, pmt: *const DMO_MEDIA_TYPE, dwflags: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).8)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwinputstreamindex), ::core::mem::transmute(pmt), ::core::mem::transmute(dwflags)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_DxMediaObjects`, `Win32_Foundation`*"]
    pub unsafe fn SetOutputType(&self, dwoutputstreamindex: u32, pmt: *const DMO_MEDIA_TYPE, dwflags: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).9)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwoutputstreamindex), ::core::mem::transmute(pmt), ::core::mem::transmute(dwflags)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_DxMediaObjects`, `Win32_Foundation`*"]
    pub unsafe fn GetInputCurrentType(&self, dwinputstreamindex: u32) -> ::windows::runtime::Result<DMO_MEDIA_TYPE> {
        let mut result__: <DMO_MEDIA_TYPE as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).10)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwinputstreamindex), &mut result__).from_abi::<DMO_MEDIA_TYPE>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_DxMediaObjects`, `Win32_Foundation`*"]
    pub unsafe fn GetOutputCurrentType(&self, dwoutputstreamindex: u32) -> ::windows::runtime::Result<DMO_MEDIA_TYPE> {
        let mut result__: <DMO_MEDIA_TYPE as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).11)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwoutputstreamindex), &mut result__).from_abi::<DMO_MEDIA_TYPE>(result__)
    }
    #[doc = "*Required features: `Win32_Media_DxMediaObjects`*"]
    pub unsafe fn GetInputSizeInfo(&self, dwinputstreamindex: u32, pcbsize: *mut u32, pcbmaxlookahead: *mut u32, pcbalignment: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).12)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwinputstreamindex), ::core::mem::transmute(pcbsize), ::core::mem::transmute(pcbmaxlookahead), ::core::mem::transmute(pcbalignment)).ok()
    }
    #[doc = "*Required features: `Win32_Media_DxMediaObjects`*"]
    pub unsafe fn GetOutputSizeInfo(&self, dwoutputstreamindex: u32, pcbsize: *mut u32, pcbalignment: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).13)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwoutputstreamindex), ::core::mem::transmute(pcbsize), ::core::mem::transmute(pcbalignment)).ok()
    }
    #[doc = "*Required features: `Win32_Media_DxMediaObjects`*"]
    pub unsafe fn GetInputMaxLatency(&self, dwinputstreamindex: u32) -> ::windows::runtime::Result<i64> {
        let mut result__: <i64 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).14)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwinputstreamindex), &mut result__).from_abi::<i64>(result__)
    }
    #[doc = "*Required features: `Win32_Media_DxMediaObjects`*"]
    pub unsafe fn SetInputMaxLatency(&self, dwinputstreamindex: u32, rtmaxlatency: i64) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).15)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwinputstreamindex), ::core::mem::transmute(rtmaxlatency)).ok()
    }
    #[doc = "*Required features: `Win32_Media_DxMediaObjects`*"]
    pub unsafe fn Flush(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).16)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `Win32_Media_DxMediaObjects`*"]
    pub unsafe fn Discontinuity(&self, dwinputstreamindex: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).17)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwinputstreamindex)).ok()
    }
    #[doc = "*Required features: `Win32_Media_DxMediaObjects`*"]
    pub unsafe fn AllocateStreamingResources(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).18)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `Win32_Media_DxMediaObjects`*"]
    pub unsafe fn FreeStreamingResources(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).19)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `Win32_Media_DxMediaObjects`*"]
    pub unsafe fn GetInputStatus(&self, dwinputstreamindex: u32) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).20)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwinputstreamindex), &mut result__).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `Win32_Media_DxMediaObjects`*"]
    pub unsafe fn ProcessInput<'a, Param1: ::windows::runtime::IntoParam<'a, IMediaBuffer>>(&self, dwinputstreamindex: u32, pbuffer: Param1, dwflags: u32, rttimestamp: i64, rttimelength: i64) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).21)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwinputstreamindex), pbuffer.into_param().abi(), ::core::mem::transmute(dwflags), ::core::mem::transmute(rttimestamp), ::core::mem::transmute(rttimelength)).ok()
    }
    #[doc = "*Required features: `Win32_Media_DxMediaObjects`*"]
    pub unsafe fn ProcessOutput(&self, dwflags: u32, coutputbuffercount: u32, poutputbuffers: *mut DMO_OUTPUT_DATA_BUFFER, pdwstatus: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).22)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwflags), ::core::mem::transmute(coutputbuffercount), ::core::mem::transmute(poutputbuffers), ::core::mem::transmute(pdwstatus)).ok()
    }
    #[doc = "*Required features: `Win32_Media_DxMediaObjects`*"]
    pub unsafe fn Lock(&self, block: i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).23)(::core::mem::transmute_copy(self), ::core::mem::transmute(block)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IMediaObject {
    type Vtable = IMediaObject_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3635220312, 21652, 16642, [151, 197, 236, 121, 142, 89, 188, 244]);
}
impl ::core::convert::From<IMediaObject> for ::windows::runtime::IUnknown {
    fn from(value: IMediaObject) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IMediaObject> for ::windows::runtime::IUnknown {
    fn from(value: &IMediaObject) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IMediaObject {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IMediaObject {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaObject_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pcinputstreams: *mut u32, pcoutputstreams: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwinputstreamindex: u32, pdwflags: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwoutputstreamindex: u32, pdwflags: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwinputstreamindex: u32, dwtypeindex: u32, pmt: *mut ::core::mem::ManuallyDrop<DMO_MEDIA_TYPE>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwoutputstreamindex: u32, dwtypeindex: u32, pmt: *mut ::core::mem::ManuallyDrop<DMO_MEDIA_TYPE>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwinputstreamindex: u32, pmt: *const ::core::mem::ManuallyDrop<DMO_MEDIA_TYPE>, dwflags: u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwoutputstreamindex: u32, pmt: *const ::core::mem::ManuallyDrop<DMO_MEDIA_TYPE>, dwflags: u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwinputstreamindex: u32, pmt: *mut ::core::mem::ManuallyDrop<DMO_MEDIA_TYPE>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwoutputstreamindex: u32, pmt: *mut ::core::mem::ManuallyDrop<DMO_MEDIA_TYPE>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwinputstreamindex: u32, pcbsize: *mut u32, pcbmaxlookahead: *mut u32, pcbalignment: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwoutputstreamindex: u32, pcbsize: *mut u32, pcbalignment: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwinputstreamindex: u32, prtmaxlatency: *mut i64) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwinputstreamindex: u32, rtmaxlatency: i64) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwinputstreamindex: u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwinputstreamindex: u32, dwflags: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwinputstreamindex: u32, pbuffer: ::windows::runtime::RawPtr, dwflags: u32, rttimestamp: i64, rttimelength: i64) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwflags: u32, coutputbuffercount: u32, poutputbuffers: *mut ::core::mem::ManuallyDrop<DMO_OUTPUT_DATA_BUFFER>, pdwstatus: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, block: i32) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_Media_DxMediaObjects`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IMediaObjectInPlace(pub ::windows::runtime::IUnknown);
impl IMediaObjectInPlace {
    #[doc = "*Required features: `Win32_Media_DxMediaObjects`*"]
    pub unsafe fn Process(&self, ulsize: u32, pdata: *mut u8, reftimestart: i64, dwflags: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(ulsize), ::core::mem::transmute(pdata), ::core::mem::transmute(reftimestart), ::core::mem::transmute(dwflags)).ok()
    }
    #[doc = "*Required features: `Win32_Media_DxMediaObjects`*"]
    pub unsafe fn Clone(&self) -> ::windows::runtime::Result<IMediaObjectInPlace> {
        let mut result__: <IMediaObjectInPlace as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IMediaObjectInPlace>(result__)
    }
    #[doc = "*Required features: `Win32_Media_DxMediaObjects`*"]
    pub unsafe fn GetLatency(&self) -> ::windows::runtime::Result<i64> {
        let mut result__: <i64 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).5)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i64>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IMediaObjectInPlace {
    type Vtable = IMediaObjectInPlace_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1696307920, 4039, 19113, [149, 56, 216, 153, 49, 1, 7, 65]);
}
impl ::core::convert::From<IMediaObjectInPlace> for ::windows::runtime::IUnknown {
    fn from(value: IMediaObjectInPlace) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IMediaObjectInPlace> for ::windows::runtime::IUnknown {
    fn from(value: &IMediaObjectInPlace) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IMediaObjectInPlace {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IMediaObjectInPlace {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaObjectInPlace_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ulsize: u32, pdata: *mut u8, reftimestart: i64, dwflags: u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppmediaobject: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, platencytime: *mut i64) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_Media_DxMediaObjects`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn MoCopyMediaType(pmtdest: *mut DMO_MEDIA_TYPE, pmtsrc: *const DMO_MEDIA_TYPE) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn MoCopyMediaType(pmtdest: *mut ::core::mem::ManuallyDrop<DMO_MEDIA_TYPE>, pmtsrc: *const ::core::mem::ManuallyDrop<DMO_MEDIA_TYPE>) -> ::windows::runtime::HRESULT;
        }
        MoCopyMediaType(::core::mem::transmute(pmtdest), ::core::mem::transmute(pmtsrc)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Media_DxMediaObjects`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn MoCreateMediaType(ppmt: *mut *mut DMO_MEDIA_TYPE, cbformat: u32) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn MoCreateMediaType(ppmt: *mut *mut DMO_MEDIA_TYPE, cbformat: u32) -> ::windows::runtime::HRESULT;
        }
        MoCreateMediaType(::core::mem::transmute(ppmt), ::core::mem::transmute(cbformat)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Media_DxMediaObjects`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn MoDeleteMediaType(pmt: *mut DMO_MEDIA_TYPE) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn MoDeleteMediaType(pmt: *mut ::core::mem::ManuallyDrop<DMO_MEDIA_TYPE>) -> ::windows::runtime::HRESULT;
        }
        MoDeleteMediaType(::core::mem::transmute(pmt)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Media_DxMediaObjects`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn MoDuplicateMediaType(ppmtdest: *mut *mut DMO_MEDIA_TYPE, pmtsrc: *const DMO_MEDIA_TYPE) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn MoDuplicateMediaType(ppmtdest: *mut *mut DMO_MEDIA_TYPE, pmtsrc: *const ::core::mem::ManuallyDrop<DMO_MEDIA_TYPE>) -> ::windows::runtime::HRESULT;
        }
        MoDuplicateMediaType(::core::mem::transmute(ppmtdest), ::core::mem::transmute(pmtsrc)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Media_DxMediaObjects`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn MoFreeMediaType(pmt: *mut DMO_MEDIA_TYPE) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn MoFreeMediaType(pmt: *mut ::core::mem::ManuallyDrop<DMO_MEDIA_TYPE>) -> ::windows::runtime::HRESULT;
        }
        MoFreeMediaType(::core::mem::transmute(pmt)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Media_DxMediaObjects`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn MoInitMediaType(pmt: *mut DMO_MEDIA_TYPE, cbformat: u32) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn MoInitMediaType(pmt: *mut ::core::mem::ManuallyDrop<DMO_MEDIA_TYPE>, cbformat: u32) -> ::windows::runtime::HRESULT;
        }
        MoInitMediaType(::core::mem::transmute(pmt), ::core::mem::transmute(cbformat)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Media_DxMediaObjects`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct _DMO_INPLACE_PROCESS_FLAGS(pub i32);
pub const DMO_INPLACE_NORMAL: _DMO_INPLACE_PROCESS_FLAGS = _DMO_INPLACE_PROCESS_FLAGS(0i32);
pub const DMO_INPLACE_ZERO: _DMO_INPLACE_PROCESS_FLAGS = _DMO_INPLACE_PROCESS_FLAGS(1i32);
impl ::core::convert::From<i32> for _DMO_INPLACE_PROCESS_FLAGS {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for _DMO_INPLACE_PROCESS_FLAGS {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Media_DxMediaObjects`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct _DMO_INPUT_DATA_BUFFER_FLAGS(pub i32);
pub const DMO_INPUT_DATA_BUFFERF_SYNCPOINT: _DMO_INPUT_DATA_BUFFER_FLAGS = _DMO_INPUT_DATA_BUFFER_FLAGS(1i32);
pub const DMO_INPUT_DATA_BUFFERF_TIME: _DMO_INPUT_DATA_BUFFER_FLAGS = _DMO_INPUT_DATA_BUFFER_FLAGS(2i32);
pub const DMO_INPUT_DATA_BUFFERF_TIMELENGTH: _DMO_INPUT_DATA_BUFFER_FLAGS = _DMO_INPUT_DATA_BUFFER_FLAGS(4i32);
pub const DMO_INPUT_DATA_BUFFERF_DISCONTINUITY: _DMO_INPUT_DATA_BUFFER_FLAGS = _DMO_INPUT_DATA_BUFFER_FLAGS(8i32);
impl ::core::convert::From<i32> for _DMO_INPUT_DATA_BUFFER_FLAGS {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for _DMO_INPUT_DATA_BUFFER_FLAGS {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Media_DxMediaObjects`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct _DMO_INPUT_STATUS_FLAGS(pub i32);
pub const DMO_INPUT_STATUSF_ACCEPT_DATA: _DMO_INPUT_STATUS_FLAGS = _DMO_INPUT_STATUS_FLAGS(1i32);
impl ::core::convert::From<i32> for _DMO_INPUT_STATUS_FLAGS {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for _DMO_INPUT_STATUS_FLAGS {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Media_DxMediaObjects`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct _DMO_INPUT_STREAM_INFO_FLAGS(pub i32);
pub const DMO_INPUT_STREAMF_WHOLE_SAMPLES: _DMO_INPUT_STREAM_INFO_FLAGS = _DMO_INPUT_STREAM_INFO_FLAGS(1i32);
pub const DMO_INPUT_STREAMF_SINGLE_SAMPLE_PER_BUFFER: _DMO_INPUT_STREAM_INFO_FLAGS = _DMO_INPUT_STREAM_INFO_FLAGS(2i32);
pub const DMO_INPUT_STREAMF_FIXED_SAMPLE_SIZE: _DMO_INPUT_STREAM_INFO_FLAGS = _DMO_INPUT_STREAM_INFO_FLAGS(4i32);
pub const DMO_INPUT_STREAMF_HOLDS_BUFFERS: _DMO_INPUT_STREAM_INFO_FLAGS = _DMO_INPUT_STREAM_INFO_FLAGS(8i32);
impl ::core::convert::From<i32> for _DMO_INPUT_STREAM_INFO_FLAGS {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for _DMO_INPUT_STREAM_INFO_FLAGS {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Media_DxMediaObjects`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct _DMO_OUTPUT_DATA_BUFFER_FLAGS(pub i32);
pub const DMO_OUTPUT_DATA_BUFFERF_SYNCPOINT: _DMO_OUTPUT_DATA_BUFFER_FLAGS = _DMO_OUTPUT_DATA_BUFFER_FLAGS(1i32);
pub const DMO_OUTPUT_DATA_BUFFERF_TIME: _DMO_OUTPUT_DATA_BUFFER_FLAGS = _DMO_OUTPUT_DATA_BUFFER_FLAGS(2i32);
pub const DMO_OUTPUT_DATA_BUFFERF_TIMELENGTH: _DMO_OUTPUT_DATA_BUFFER_FLAGS = _DMO_OUTPUT_DATA_BUFFER_FLAGS(4i32);
pub const DMO_OUTPUT_DATA_BUFFERF_DISCONTINUITY: _DMO_OUTPUT_DATA_BUFFER_FLAGS = _DMO_OUTPUT_DATA_BUFFER_FLAGS(8i32);
pub const DMO_OUTPUT_DATA_BUFFERF_INCOMPLETE: _DMO_OUTPUT_DATA_BUFFER_FLAGS = _DMO_OUTPUT_DATA_BUFFER_FLAGS(16777216i32);
impl ::core::convert::From<i32> for _DMO_OUTPUT_DATA_BUFFER_FLAGS {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for _DMO_OUTPUT_DATA_BUFFER_FLAGS {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Media_DxMediaObjects`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct _DMO_OUTPUT_STREAM_INFO_FLAGS(pub i32);
pub const DMO_OUTPUT_STREAMF_WHOLE_SAMPLES: _DMO_OUTPUT_STREAM_INFO_FLAGS = _DMO_OUTPUT_STREAM_INFO_FLAGS(1i32);
pub const DMO_OUTPUT_STREAMF_SINGLE_SAMPLE_PER_BUFFER: _DMO_OUTPUT_STREAM_INFO_FLAGS = _DMO_OUTPUT_STREAM_INFO_FLAGS(2i32);
pub const DMO_OUTPUT_STREAMF_FIXED_SAMPLE_SIZE: _DMO_OUTPUT_STREAM_INFO_FLAGS = _DMO_OUTPUT_STREAM_INFO_FLAGS(4i32);
pub const DMO_OUTPUT_STREAMF_DISCARDABLE: _DMO_OUTPUT_STREAM_INFO_FLAGS = _DMO_OUTPUT_STREAM_INFO_FLAGS(8i32);
pub const DMO_OUTPUT_STREAMF_OPTIONAL: _DMO_OUTPUT_STREAM_INFO_FLAGS = _DMO_OUTPUT_STREAM_INFO_FLAGS(16i32);
impl ::core::convert::From<i32> for _DMO_OUTPUT_STREAM_INFO_FLAGS {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for _DMO_OUTPUT_STREAM_INFO_FLAGS {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Media_DxMediaObjects`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct _DMO_PROCESS_OUTPUT_FLAGS(pub i32);
pub const DMO_PROCESS_OUTPUT_DISCARD_WHEN_NO_BUFFER: _DMO_PROCESS_OUTPUT_FLAGS = _DMO_PROCESS_OUTPUT_FLAGS(1i32);
impl ::core::convert::From<i32> for _DMO_PROCESS_OUTPUT_FLAGS {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for _DMO_PROCESS_OUTPUT_FLAGS {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Media_DxMediaObjects`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct _DMO_QUALITY_STATUS_FLAGS(pub i32);
pub const DMO_QUALITY_STATUS_ENABLED: _DMO_QUALITY_STATUS_FLAGS = _DMO_QUALITY_STATUS_FLAGS(1i32);
impl ::core::convert::From<i32> for _DMO_QUALITY_STATUS_FLAGS {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for _DMO_QUALITY_STATUS_FLAGS {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Media_DxMediaObjects`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct _DMO_SET_TYPE_FLAGS(pub i32);
pub const DMO_SET_TYPEF_TEST_ONLY: _DMO_SET_TYPE_FLAGS = _DMO_SET_TYPE_FLAGS(1i32);
pub const DMO_SET_TYPEF_CLEAR: _DMO_SET_TYPE_FLAGS = _DMO_SET_TYPE_FLAGS(2i32);
impl ::core::convert::From<i32> for _DMO_SET_TYPE_FLAGS {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for _DMO_SET_TYPE_FLAGS {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Media_DxMediaObjects`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct _DMO_VIDEO_OUTPUT_STREAM_FLAGS(pub i32);
pub const DMO_VOSF_NEEDS_PREVIOUS_SAMPLE: _DMO_VIDEO_OUTPUT_STREAM_FLAGS = _DMO_VIDEO_OUTPUT_STREAM_FLAGS(1i32);
impl ::core::convert::From<i32> for _DMO_VIDEO_OUTPUT_STREAM_FLAGS {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for _DMO_VIDEO_OUTPUT_STREAM_FLAGS {
    type Abi = Self;
}
