#![allow(unused_variables, non_upper_case_globals, non_snake_case, unused_unsafe, non_camel_case_types, dead_code, clippy::all)]
pub const DMOCATEGORY_ACOUSTIC_ECHO_CANCEL: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xbf963d80_c559_11d0_8a2b_00a0c9255ac1);
pub const DMOCATEGORY_AGC: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe88c9ba0_c557_11d0_8a2b_00a0c9255ac1);
pub const DMOCATEGORY_AUDIO_CAPTURE_EFFECT: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf665aaba_3e09_4920_aa5f_219811148f09);
pub const DMOCATEGORY_AUDIO_DECODER: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x57f2db8b_e6bb_4513_9d43_dcd2a6593125);
pub const DMOCATEGORY_AUDIO_EFFECT: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf3602b3f_0592_48df_a4cd_674721e7ebeb);
pub const DMOCATEGORY_AUDIO_ENCODER: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x33d9a761_90c8_11d0_bd43_00a0c911ce86);
pub const DMOCATEGORY_AUDIO_NOISE_SUPPRESS: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe07f903f_62fd_4e60_8cdd_dea7236665b5);
pub const DMOCATEGORY_VIDEO_DECODER: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4a69b442_28be_4991_969c_b500adf5d8a8);
pub const DMOCATEGORY_VIDEO_EFFECT: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd990ee14_776c_4723_be46_3da2f56f10b9);
pub const DMOCATEGORY_VIDEO_ENCODER: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x33d9a760_90c8_11d0_bd43_00a0c911ce86);
#[doc = "*Required features: `Win32_Media_DxMediaObjects`*"]
#[inline]
pub unsafe fn DMOEnum(guidcategory: *const ::windows::core::GUID, dwflags: u32, cintypes: u32, pintypes: *const DMO_PARTIAL_MEDIATYPE, couttypes: u32, pouttypes: *const DMO_PARTIAL_MEDIATYPE) -> ::windows::core::Result<IEnumDMO> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DMOEnum(guidcategory: *const ::windows::core::GUID, dwflags: u32, cintypes: u32, pintypes: *const DMO_PARTIAL_MEDIATYPE, couttypes: u32, pouttypes: *const DMO_PARTIAL_MEDIATYPE, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT;
        }
        let mut result__: <IEnumDMO as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        DMOEnum(::core::mem::transmute(guidcategory), ::core::mem::transmute(dwflags), ::core::mem::transmute(cintypes), ::core::mem::transmute(pintypes), ::core::mem::transmute(couttypes), ::core::mem::transmute(pouttypes), &mut result__).from_abi::<IEnumDMO>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Media_DxMediaObjects`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DMOGetName(clsiddmo: *const ::windows::core::GUID, szname: super::super::Foundation::PWSTR) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DMOGetName(clsiddmo: *const ::windows::core::GUID, szname: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT;
        }
        DMOGetName(::core::mem::transmute(clsiddmo), ::core::mem::transmute(szname)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Media_DxMediaObjects`*"]
#[inline]
pub unsafe fn DMOGetTypes(clsiddmo: *const ::windows::core::GUID, ulinputtypesrequested: u32, pulinputtypessupplied: *mut u32, pinputtypes: *mut DMO_PARTIAL_MEDIATYPE, uloutputtypesrequested: u32, puloutputtypessupplied: *mut u32, poutputtypes: *mut DMO_PARTIAL_MEDIATYPE) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DMOGetTypes(clsiddmo: *const ::windows::core::GUID, ulinputtypesrequested: u32, pulinputtypessupplied: *mut u32, pinputtypes: *mut DMO_PARTIAL_MEDIATYPE, uloutputtypesrequested: u32, puloutputtypessupplied: *mut u32, poutputtypes: *mut DMO_PARTIAL_MEDIATYPE) -> ::windows::core::HRESULT;
        }
        DMOGetTypes(::core::mem::transmute(clsiddmo), ::core::mem::transmute(ulinputtypesrequested), ::core::mem::transmute(pulinputtypessupplied), ::core::mem::transmute(pinputtypes), ::core::mem::transmute(uloutputtypesrequested), ::core::mem::transmute(puloutputtypessupplied), ::core::mem::transmute(poutputtypes)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Media_DxMediaObjects`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DMORegister<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(szname: Param0, clsiddmo: *const ::windows::core::GUID, guidcategory: *const ::windows::core::GUID, dwflags: u32, cintypes: u32, pintypes: *const DMO_PARTIAL_MEDIATYPE, couttypes: u32, pouttypes: *const DMO_PARTIAL_MEDIATYPE) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DMORegister(szname: super::super::Foundation::PWSTR, clsiddmo: *const ::windows::core::GUID, guidcategory: *const ::windows::core::GUID, dwflags: u32, cintypes: u32, pintypes: *const DMO_PARTIAL_MEDIATYPE, couttypes: u32, pouttypes: *const DMO_PARTIAL_MEDIATYPE) -> ::windows::core::HRESULT;
        }
        DMORegister(szname.into_param().abi(), ::core::mem::transmute(clsiddmo), ::core::mem::transmute(guidcategory), ::core::mem::transmute(dwflags), ::core::mem::transmute(cintypes), ::core::mem::transmute(pintypes), ::core::mem::transmute(couttypes), ::core::mem::transmute(pouttypes)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Media_DxMediaObjects`*"]
#[inline]
pub unsafe fn DMOUnregister(clsiddmo: *const ::windows::core::GUID, guidcategory: *const ::windows::core::GUID) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DMOUnregister(clsiddmo: *const ::windows::core::GUID, guidcategory: *const ::windows::core::GUID) -> ::windows::core::HRESULT;
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
unsafe impl ::windows::core::Abi for DMO_ENUM_FLAGS {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Media_DxMediaObjects`*"]
pub const DMO_E_INVALIDSTREAMINDEX: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147220991i32 as _);
#[doc = "*Required features: `Win32_Media_DxMediaObjects`*"]
pub const DMO_E_INVALIDTYPE: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147220990i32 as _);
#[doc = "*Required features: `Win32_Media_DxMediaObjects`*"]
pub const DMO_E_NOTACCEPTING: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147220988i32 as _);
#[doc = "*Required features: `Win32_Media_DxMediaObjects`*"]
pub const DMO_E_NO_MORE_ITEMS: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147220986i32 as _);
#[doc = "*Required features: `Win32_Media_DxMediaObjects`*"]
pub const DMO_E_TYPE_NOT_ACCEPTED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147220987i32 as _);
#[doc = "*Required features: `Win32_Media_DxMediaObjects`*"]
pub const DMO_E_TYPE_NOT_SET: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147220989i32 as _);
#[derive(:: core :: clone :: Clone)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_Media_DxMediaObjects`, `Win32_Foundation`*"]
pub struct DMO_MEDIA_TYPE {
    pub majortype: ::windows::core::GUID,
    pub subtype: ::windows::core::GUID,
    pub bFixedSizeSamples: super::super::Foundation::BOOL,
    pub bTemporalCompression: super::super::Foundation::BOOL,
    pub lSampleSize: u32,
    pub formattype: ::windows::core::GUID,
    pub pUnk: ::core::option::Option<::windows::core::IUnknown>,
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
unsafe impl ::windows::core::Abi for DMO_MEDIA_TYPE {
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
unsafe impl ::windows::core::Abi for DMO_OUTPUT_DATA_BUFFER {
    type Abi = ::core::mem::ManuallyDrop<Self>;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Media_DxMediaObjects`*"]
pub struct DMO_PARTIAL_MEDIATYPE {
    pub r#type: ::windows::core::GUID,
    pub subtype: ::windows::core::GUID,
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
unsafe impl ::windows::core::Abi for DMO_PARTIAL_MEDIATYPE {
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
unsafe impl ::windows::core::Abi for DMO_REGISTER_FLAGS {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Media_DxMediaObjects`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IDMOQualityControl(pub ::windows::core::IUnknown);
impl IDMOQualityControl {
    #[doc = "*Required features: `Win32_Media_DxMediaObjects`*"]
    pub unsafe fn SetNow(&self, rtnow: i64) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(rtnow)).ok()
    }
    #[doc = "*Required features: `Win32_Media_DxMediaObjects`*"]
    pub unsafe fn SetStatus(&self, dwflags: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwflags)).ok()
    }
    #[doc = "*Required features: `Win32_Media_DxMediaObjects`*"]
    pub unsafe fn GetStatus(&self) -> ::windows::core::Result<u32> {
        let mut result__: <u32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
}
unsafe impl ::windows::core::Interface for IDMOQualityControl {
    type Vtable = IDMOQualityControl_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x65abea96_cf36_453f_af8a_705e98f16260);
}
impl ::core::convert::From<IDMOQualityControl> for ::windows::core::IUnknown {
    fn from(value: IDMOQualityControl) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IDMOQualityControl> for ::windows::core::IUnknown {
    fn from(value: &IDMOQualityControl) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IDMOQualityControl {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IDMOQualityControl {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDMOQualityControl_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, rtnow: i64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, dwflags: u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pdwflags: *mut u32) -> ::windows::core::HRESULT,
);
#[doc = "*Required features: `Win32_Media_DxMediaObjects`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IDMOVideoOutputOptimizations(pub ::windows::core::IUnknown);
impl IDMOVideoOutputOptimizations {
    #[doc = "*Required features: `Win32_Media_DxMediaObjects`*"]
    pub unsafe fn QueryOperationModePreferences(&self, uloutputstreamindex: u32) -> ::windows::core::Result<u32> {
        let mut result__: <u32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(uloutputstreamindex), &mut result__).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `Win32_Media_DxMediaObjects`*"]
    pub unsafe fn SetOperationMode(&self, uloutputstreamindex: u32, dwenabledfeatures: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(uloutputstreamindex), ::core::mem::transmute(dwenabledfeatures)).ok()
    }
    #[doc = "*Required features: `Win32_Media_DxMediaObjects`*"]
    pub unsafe fn GetCurrentOperationMode(&self, uloutputstreamindex: u32) -> ::windows::core::Result<u32> {
        let mut result__: <u32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(uloutputstreamindex), &mut result__).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `Win32_Media_DxMediaObjects`*"]
    pub unsafe fn GetCurrentSampleRequirements(&self, uloutputstreamindex: u32) -> ::windows::core::Result<u32> {
        let mut result__: <u32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(uloutputstreamindex), &mut result__).from_abi::<u32>(result__)
    }
}
unsafe impl ::windows::core::Interface for IDMOVideoOutputOptimizations {
    type Vtable = IDMOVideoOutputOptimizations_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xbe8f4f4e_5b16_4d29_b350_7f6b5d9298ac);
}
impl ::core::convert::From<IDMOVideoOutputOptimizations> for ::windows::core::IUnknown {
    fn from(value: IDMOVideoOutputOptimizations) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IDMOVideoOutputOptimizations> for ::windows::core::IUnknown {
    fn from(value: &IDMOVideoOutputOptimizations) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IDMOVideoOutputOptimizations {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IDMOVideoOutputOptimizations {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDMOVideoOutputOptimizations_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, uloutputstreamindex: u32, pdwrequestedcapabilities: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, uloutputstreamindex: u32, dwenabledfeatures: u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, uloutputstreamindex: u32, pdwenabledfeatures: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, uloutputstreamindex: u32, pdwrequestedfeatures: *mut u32) -> ::windows::core::HRESULT,
);
#[doc = "*Required features: `Win32_Media_DxMediaObjects`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IEnumDMO(pub ::windows::core::IUnknown);
impl IEnumDMO {
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_DxMediaObjects`, `Win32_Foundation`*"]
    pub unsafe fn Next(&self, citemstofetch: u32, pclsid: *mut ::windows::core::GUID, names: *mut super::super::Foundation::PWSTR, pcitemsfetched: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(citemstofetch), ::core::mem::transmute(pclsid), ::core::mem::transmute(names), ::core::mem::transmute(pcitemsfetched)).ok()
    }
    #[doc = "*Required features: `Win32_Media_DxMediaObjects`*"]
    pub unsafe fn Skip(&self, citemstoskip: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(citemstoskip)).ok()
    }
    #[doc = "*Required features: `Win32_Media_DxMediaObjects`*"]
    pub unsafe fn Reset(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `Win32_Media_DxMediaObjects`*"]
    pub unsafe fn Clone(&self) -> ::windows::core::Result<IEnumDMO> {
        let mut result__: <IEnumDMO as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IEnumDMO>(result__)
    }
}
unsafe impl ::windows::core::Interface for IEnumDMO {
    type Vtable = IEnumDMO_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2c3cd98a_2bfa_4a53_9c27_5249ba64ba0f);
}
impl ::core::convert::From<IEnumDMO> for ::windows::core::IUnknown {
    fn from(value: IEnumDMO) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IEnumDMO> for ::windows::core::IUnknown {
    fn from(value: &IEnumDMO) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IEnumDMO {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IEnumDMO {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IEnumDMO_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, citemstofetch: u32, pclsid: *mut ::windows::core::GUID, names: *mut super::super::Foundation::PWSTR, pcitemsfetched: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, citemstoskip: u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[doc = "*Required features: `Win32_Media_DxMediaObjects`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IMediaBuffer(pub ::windows::core::IUnknown);
impl IMediaBuffer {
    #[doc = "*Required features: `Win32_Media_DxMediaObjects`*"]
    pub unsafe fn SetLength(&self, cblength: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(cblength)).ok()
    }
    #[doc = "*Required features: `Win32_Media_DxMediaObjects`*"]
    pub unsafe fn GetMaxLength(&self) -> ::windows::core::Result<u32> {
        let mut result__: <u32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `Win32_Media_DxMediaObjects`*"]
    pub unsafe fn GetBufferAndLength(&self, ppbuffer: *mut *mut u8, pcblength: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(ppbuffer), ::core::mem::transmute(pcblength)).ok()
    }
}
unsafe impl ::windows::core::Interface for IMediaBuffer {
    type Vtable = IMediaBuffer_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x59eff8b9_938c_4a26_82f2_95cb84cdc837);
}
impl ::core::convert::From<IMediaBuffer> for ::windows::core::IUnknown {
    fn from(value: IMediaBuffer) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IMediaBuffer> for ::windows::core::IUnknown {
    fn from(value: &IMediaBuffer) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IMediaBuffer {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IMediaBuffer {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaBuffer_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, cblength: u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pcbmaxlength: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, ppbuffer: *mut *mut u8, pcblength: *mut u32) -> ::windows::core::HRESULT,
);
#[doc = "*Required features: `Win32_Media_DxMediaObjects`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IMediaObject(pub ::windows::core::IUnknown);
impl IMediaObject {
    #[doc = "*Required features: `Win32_Media_DxMediaObjects`*"]
    pub unsafe fn GetStreamCount(&self, pcinputstreams: *mut u32, pcoutputstreams: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(pcinputstreams), ::core::mem::transmute(pcoutputstreams)).ok()
    }
    #[doc = "*Required features: `Win32_Media_DxMediaObjects`*"]
    pub unsafe fn GetInputStreamInfo(&self, dwinputstreamindex: u32) -> ::windows::core::Result<u32> {
        let mut result__: <u32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwinputstreamindex), &mut result__).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `Win32_Media_DxMediaObjects`*"]
    pub unsafe fn GetOutputStreamInfo(&self, dwoutputstreamindex: u32) -> ::windows::core::Result<u32> {
        let mut result__: <u32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwoutputstreamindex), &mut result__).from_abi::<u32>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_DxMediaObjects`, `Win32_Foundation`*"]
    pub unsafe fn GetInputType(&self, dwinputstreamindex: u32, dwtypeindex: u32) -> ::windows::core::Result<DMO_MEDIA_TYPE> {
        let mut result__: <DMO_MEDIA_TYPE as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwinputstreamindex), ::core::mem::transmute(dwtypeindex), &mut result__).from_abi::<DMO_MEDIA_TYPE>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_DxMediaObjects`, `Win32_Foundation`*"]
    pub unsafe fn GetOutputType(&self, dwoutputstreamindex: u32, dwtypeindex: u32) -> ::windows::core::Result<DMO_MEDIA_TYPE> {
        let mut result__: <DMO_MEDIA_TYPE as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwoutputstreamindex), ::core::mem::transmute(dwtypeindex), &mut result__).from_abi::<DMO_MEDIA_TYPE>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_DxMediaObjects`, `Win32_Foundation`*"]
    pub unsafe fn SetInputType(&self, dwinputstreamindex: u32, pmt: *const DMO_MEDIA_TYPE, dwflags: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwinputstreamindex), ::core::mem::transmute(pmt), ::core::mem::transmute(dwflags)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_DxMediaObjects`, `Win32_Foundation`*"]
    pub unsafe fn SetOutputType(&self, dwoutputstreamindex: u32, pmt: *const DMO_MEDIA_TYPE, dwflags: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwoutputstreamindex), ::core::mem::transmute(pmt), ::core::mem::transmute(dwflags)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_DxMediaObjects`, `Win32_Foundation`*"]
    pub unsafe fn GetInputCurrentType(&self, dwinputstreamindex: u32) -> ::windows::core::Result<DMO_MEDIA_TYPE> {
        let mut result__: <DMO_MEDIA_TYPE as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwinputstreamindex), &mut result__).from_abi::<DMO_MEDIA_TYPE>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_DxMediaObjects`, `Win32_Foundation`*"]
    pub unsafe fn GetOutputCurrentType(&self, dwoutputstreamindex: u32) -> ::windows::core::Result<DMO_MEDIA_TYPE> {
        let mut result__: <DMO_MEDIA_TYPE as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).11)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwoutputstreamindex), &mut result__).from_abi::<DMO_MEDIA_TYPE>(result__)
    }
    #[doc = "*Required features: `Win32_Media_DxMediaObjects`*"]
    pub unsafe fn GetInputSizeInfo(&self, dwinputstreamindex: u32, pcbsize: *mut u32, pcbmaxlookahead: *mut u32, pcbalignment: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).12)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwinputstreamindex), ::core::mem::transmute(pcbsize), ::core::mem::transmute(pcbmaxlookahead), ::core::mem::transmute(pcbalignment)).ok()
    }
    #[doc = "*Required features: `Win32_Media_DxMediaObjects`*"]
    pub unsafe fn GetOutputSizeInfo(&self, dwoutputstreamindex: u32, pcbsize: *mut u32, pcbalignment: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).13)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwoutputstreamindex), ::core::mem::transmute(pcbsize), ::core::mem::transmute(pcbalignment)).ok()
    }
    #[doc = "*Required features: `Win32_Media_DxMediaObjects`*"]
    pub unsafe fn GetInputMaxLatency(&self, dwinputstreamindex: u32) -> ::windows::core::Result<i64> {
        let mut result__: <i64 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).14)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwinputstreamindex), &mut result__).from_abi::<i64>(result__)
    }
    #[doc = "*Required features: `Win32_Media_DxMediaObjects`*"]
    pub unsafe fn SetInputMaxLatency(&self, dwinputstreamindex: u32, rtmaxlatency: i64) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).15)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwinputstreamindex), ::core::mem::transmute(rtmaxlatency)).ok()
    }
    #[doc = "*Required features: `Win32_Media_DxMediaObjects`*"]
    pub unsafe fn Flush(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).16)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `Win32_Media_DxMediaObjects`*"]
    pub unsafe fn Discontinuity(&self, dwinputstreamindex: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).17)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwinputstreamindex)).ok()
    }
    #[doc = "*Required features: `Win32_Media_DxMediaObjects`*"]
    pub unsafe fn AllocateStreamingResources(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).18)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `Win32_Media_DxMediaObjects`*"]
    pub unsafe fn FreeStreamingResources(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).19)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `Win32_Media_DxMediaObjects`*"]
    pub unsafe fn GetInputStatus(&self, dwinputstreamindex: u32) -> ::windows::core::Result<u32> {
        let mut result__: <u32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).20)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwinputstreamindex), &mut result__).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `Win32_Media_DxMediaObjects`*"]
    pub unsafe fn ProcessInput<'a, Param1: ::windows::core::IntoParam<'a, IMediaBuffer>>(&self, dwinputstreamindex: u32, pbuffer: Param1, dwflags: u32, rttimestamp: i64, rttimelength: i64) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).21)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwinputstreamindex), pbuffer.into_param().abi(), ::core::mem::transmute(dwflags), ::core::mem::transmute(rttimestamp), ::core::mem::transmute(rttimelength)).ok()
    }
    #[doc = "*Required features: `Win32_Media_DxMediaObjects`*"]
    pub unsafe fn ProcessOutput(&self, dwflags: u32, coutputbuffercount: u32, poutputbuffers: *mut DMO_OUTPUT_DATA_BUFFER, pdwstatus: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).22)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwflags), ::core::mem::transmute(coutputbuffercount), ::core::mem::transmute(poutputbuffers), ::core::mem::transmute(pdwstatus)).ok()
    }
    #[doc = "*Required features: `Win32_Media_DxMediaObjects`*"]
    pub unsafe fn Lock(&self, block: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).23)(::core::mem::transmute_copy(self), ::core::mem::transmute(block)).ok()
    }
}
unsafe impl ::windows::core::Interface for IMediaObject {
    type Vtable = IMediaObject_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd8ad0f58_5494_4102_97c5_ec798e59bcf4);
}
impl ::core::convert::From<IMediaObject> for ::windows::core::IUnknown {
    fn from(value: IMediaObject) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IMediaObject> for ::windows::core::IUnknown {
    fn from(value: &IMediaObject) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IMediaObject {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IMediaObject {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaObject_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pcinputstreams: *mut u32, pcoutputstreams: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, dwinputstreamindex: u32, pdwflags: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, dwoutputstreamindex: u32, pdwflags: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, dwinputstreamindex: u32, dwtypeindex: u32, pmt: *mut ::core::mem::ManuallyDrop<DMO_MEDIA_TYPE>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, dwoutputstreamindex: u32, dwtypeindex: u32, pmt: *mut ::core::mem::ManuallyDrop<DMO_MEDIA_TYPE>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, dwinputstreamindex: u32, pmt: *const ::core::mem::ManuallyDrop<DMO_MEDIA_TYPE>, dwflags: u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, dwoutputstreamindex: u32, pmt: *const ::core::mem::ManuallyDrop<DMO_MEDIA_TYPE>, dwflags: u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, dwinputstreamindex: u32, pmt: *mut ::core::mem::ManuallyDrop<DMO_MEDIA_TYPE>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, dwoutputstreamindex: u32, pmt: *mut ::core::mem::ManuallyDrop<DMO_MEDIA_TYPE>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, dwinputstreamindex: u32, pcbsize: *mut u32, pcbmaxlookahead: *mut u32, pcbalignment: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, dwoutputstreamindex: u32, pcbsize: *mut u32, pcbalignment: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, dwinputstreamindex: u32, prtmaxlatency: *mut i64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, dwinputstreamindex: u32, rtmaxlatency: i64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, dwinputstreamindex: u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, dwinputstreamindex: u32, dwflags: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, dwinputstreamindex: u32, pbuffer: ::windows::core::RawPtr, dwflags: u32, rttimestamp: i64, rttimelength: i64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, dwflags: u32, coutputbuffercount: u32, poutputbuffers: *mut ::core::mem::ManuallyDrop<DMO_OUTPUT_DATA_BUFFER>, pdwstatus: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, block: i32) -> ::windows::core::HRESULT,
);
#[doc = "*Required features: `Win32_Media_DxMediaObjects`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IMediaObjectInPlace(pub ::windows::core::IUnknown);
impl IMediaObjectInPlace {
    #[doc = "*Required features: `Win32_Media_DxMediaObjects`*"]
    pub unsafe fn Process(&self, ulsize: u32, pdata: *mut u8, reftimestart: i64, dwflags: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(ulsize), ::core::mem::transmute(pdata), ::core::mem::transmute(reftimestart), ::core::mem::transmute(dwflags)).ok()
    }
    #[doc = "*Required features: `Win32_Media_DxMediaObjects`*"]
    pub unsafe fn Clone(&self) -> ::windows::core::Result<IMediaObjectInPlace> {
        let mut result__: <IMediaObjectInPlace as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IMediaObjectInPlace>(result__)
    }
    #[doc = "*Required features: `Win32_Media_DxMediaObjects`*"]
    pub unsafe fn GetLatency(&self) -> ::windows::core::Result<i64> {
        let mut result__: <i64 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i64>(result__)
    }
}
unsafe impl ::windows::core::Interface for IMediaObjectInPlace {
    type Vtable = IMediaObjectInPlace_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x651b9ad0_0fc7_4aa9_9538_d89931010741);
}
impl ::core::convert::From<IMediaObjectInPlace> for ::windows::core::IUnknown {
    fn from(value: IMediaObjectInPlace) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IMediaObjectInPlace> for ::windows::core::IUnknown {
    fn from(value: &IMediaObjectInPlace) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IMediaObjectInPlace {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IMediaObjectInPlace {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaObjectInPlace_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, ulsize: u32, pdata: *mut u8, reftimestart: i64, dwflags: u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, ppmediaobject: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, platencytime: *mut i64) -> ::windows::core::HRESULT,
);
#[doc = "*Required features: `Win32_Media_DxMediaObjects`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn MoCopyMediaType(pmtdest: *mut DMO_MEDIA_TYPE, pmtsrc: *const DMO_MEDIA_TYPE) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn MoCopyMediaType(pmtdest: *mut ::core::mem::ManuallyDrop<DMO_MEDIA_TYPE>, pmtsrc: *const ::core::mem::ManuallyDrop<DMO_MEDIA_TYPE>) -> ::windows::core::HRESULT;
        }
        MoCopyMediaType(::core::mem::transmute(pmtdest), ::core::mem::transmute(pmtsrc)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Media_DxMediaObjects`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn MoCreateMediaType(ppmt: *mut *mut DMO_MEDIA_TYPE, cbformat: u32) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn MoCreateMediaType(ppmt: *mut *mut DMO_MEDIA_TYPE, cbformat: u32) -> ::windows::core::HRESULT;
        }
        MoCreateMediaType(::core::mem::transmute(ppmt), ::core::mem::transmute(cbformat)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Media_DxMediaObjects`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn MoDeleteMediaType(pmt: *mut DMO_MEDIA_TYPE) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn MoDeleteMediaType(pmt: *mut ::core::mem::ManuallyDrop<DMO_MEDIA_TYPE>) -> ::windows::core::HRESULT;
        }
        MoDeleteMediaType(::core::mem::transmute(pmt)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Media_DxMediaObjects`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn MoDuplicateMediaType(ppmtdest: *mut *mut DMO_MEDIA_TYPE, pmtsrc: *const DMO_MEDIA_TYPE) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn MoDuplicateMediaType(ppmtdest: *mut *mut DMO_MEDIA_TYPE, pmtsrc: *const ::core::mem::ManuallyDrop<DMO_MEDIA_TYPE>) -> ::windows::core::HRESULT;
        }
        MoDuplicateMediaType(::core::mem::transmute(ppmtdest), ::core::mem::transmute(pmtsrc)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Media_DxMediaObjects`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn MoFreeMediaType(pmt: *mut DMO_MEDIA_TYPE) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn MoFreeMediaType(pmt: *mut ::core::mem::ManuallyDrop<DMO_MEDIA_TYPE>) -> ::windows::core::HRESULT;
        }
        MoFreeMediaType(::core::mem::transmute(pmt)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Media_DxMediaObjects`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn MoInitMediaType(pmt: *mut DMO_MEDIA_TYPE, cbformat: u32) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn MoInitMediaType(pmt: *mut ::core::mem::ManuallyDrop<DMO_MEDIA_TYPE>, cbformat: u32) -> ::windows::core::HRESULT;
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
unsafe impl ::windows::core::Abi for _DMO_INPLACE_PROCESS_FLAGS {
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
unsafe impl ::windows::core::Abi for _DMO_INPUT_DATA_BUFFER_FLAGS {
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
unsafe impl ::windows::core::Abi for _DMO_INPUT_STATUS_FLAGS {
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
unsafe impl ::windows::core::Abi for _DMO_INPUT_STREAM_INFO_FLAGS {
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
unsafe impl ::windows::core::Abi for _DMO_OUTPUT_DATA_BUFFER_FLAGS {
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
unsafe impl ::windows::core::Abi for _DMO_OUTPUT_STREAM_INFO_FLAGS {
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
unsafe impl ::windows::core::Abi for _DMO_PROCESS_OUTPUT_FLAGS {
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
unsafe impl ::windows::core::Abi for _DMO_QUALITY_STATUS_FLAGS {
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
unsafe impl ::windows::core::Abi for _DMO_SET_TYPE_FLAGS {
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
unsafe impl ::windows::core::Abi for _DMO_VIDEO_OUTPUT_STREAM_FLAGS {
    type Abi = Self;
}
