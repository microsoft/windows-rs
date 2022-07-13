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
#[doc = "*Required features: `\"Win32_Media_DxMediaObjects\"`*"]
#[inline]
pub unsafe fn DMOEnum(guidcategory: *const ::windows::core::GUID, dwflags: u32, cintypes: u32, pintypes: *const DMO_PARTIAL_MEDIATYPE, couttypes: u32, pouttypes: *const DMO_PARTIAL_MEDIATYPE) -> ::windows::core::Result<IEnumDMO> {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn DMOEnum(guidcategory: *const ::windows::core::GUID, dwflags: u32, cintypes: u32, pintypes: *const DMO_PARTIAL_MEDIATYPE, couttypes: u32, pouttypes: *const DMO_PARTIAL_MEDIATYPE, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT;
    }
    let mut result__ = ::core::mem::MaybeUninit::zeroed();
    DMOEnum(::core::mem::transmute(guidcategory), dwflags, cintypes, ::core::mem::transmute(pintypes), couttypes, ::core::mem::transmute(pouttypes), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IEnumDMO>(result__)
}
#[doc = "*Required features: `\"Win32_Media_DxMediaObjects\"`*"]
#[inline]
pub unsafe fn DMOGetName(clsiddmo: *const ::windows::core::GUID, szname: &mut [u16; 80]) -> ::windows::core::Result<()> {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn DMOGetName(clsiddmo: *const ::windows::core::GUID, szname: ::windows::core::PWSTR) -> ::windows::core::HRESULT;
    }
    DMOGetName(::core::mem::transmute(clsiddmo), ::core::mem::transmute(::windows::core::as_mut_ptr_or_null(szname))).ok()
}
#[doc = "*Required features: `\"Win32_Media_DxMediaObjects\"`*"]
#[inline]
pub unsafe fn DMOGetTypes(clsiddmo: *const ::windows::core::GUID, ulinputtypesrequested: u32, pulinputtypessupplied: *mut u32, pinputtypes: *mut DMO_PARTIAL_MEDIATYPE, uloutputtypesrequested: u32, puloutputtypessupplied: *mut u32, poutputtypes: *mut DMO_PARTIAL_MEDIATYPE) -> ::windows::core::Result<()> {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn DMOGetTypes(clsiddmo: *const ::windows::core::GUID, ulinputtypesrequested: u32, pulinputtypessupplied: *mut u32, pinputtypes: *mut DMO_PARTIAL_MEDIATYPE, uloutputtypesrequested: u32, puloutputtypessupplied: *mut u32, poutputtypes: *mut DMO_PARTIAL_MEDIATYPE) -> ::windows::core::HRESULT;
    }
    DMOGetTypes(::core::mem::transmute(clsiddmo), ulinputtypesrequested, ::core::mem::transmute(pulinputtypessupplied), ::core::mem::transmute(pinputtypes), uloutputtypesrequested, ::core::mem::transmute(puloutputtypessupplied), ::core::mem::transmute(poutputtypes)).ok()
}
#[doc = "*Required features: `\"Win32_Media_DxMediaObjects\"`*"]
#[inline]
pub unsafe fn DMORegister<'a, P0>(szname: P0, clsiddmo: *const ::windows::core::GUID, guidcategory: *const ::windows::core::GUID, dwflags: u32, cintypes: u32, pintypes: *const DMO_PARTIAL_MEDIATYPE, couttypes: u32, pouttypes: *const DMO_PARTIAL_MEDIATYPE) -> ::windows::core::Result<()>
where
    P0: ::std::convert::Into<::windows::core::PCWSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn DMORegister(szname: ::windows::core::PCWSTR, clsiddmo: *const ::windows::core::GUID, guidcategory: *const ::windows::core::GUID, dwflags: u32, cintypes: u32, pintypes: *const DMO_PARTIAL_MEDIATYPE, couttypes: u32, pouttypes: *const DMO_PARTIAL_MEDIATYPE) -> ::windows::core::HRESULT;
    }
    DMORegister(szname.into(), ::core::mem::transmute(clsiddmo), ::core::mem::transmute(guidcategory), dwflags, cintypes, ::core::mem::transmute(pintypes), couttypes, ::core::mem::transmute(pouttypes)).ok()
}
#[doc = "*Required features: `\"Win32_Media_DxMediaObjects\"`*"]
#[inline]
pub unsafe fn DMOUnregister(clsiddmo: *const ::windows::core::GUID, guidcategory: *const ::windows::core::GUID) -> ::windows::core::Result<()> {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn DMOUnregister(clsiddmo: *const ::windows::core::GUID, guidcategory: *const ::windows::core::GUID) -> ::windows::core::HRESULT;
    }
    DMOUnregister(::core::mem::transmute(clsiddmo), ::core::mem::transmute(guidcategory)).ok()
}
#[doc = "*Required features: `\"Win32_Media_DxMediaObjects\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct DMO_ENUM_FLAGS(pub i32);
#[doc = "*Required features: `\"Win32_Media_DxMediaObjects\"`*"]
pub const DMO_ENUMF_INCLUDE_KEYED: DMO_ENUM_FLAGS = DMO_ENUM_FLAGS(1i32);
impl ::core::marker::Copy for DMO_ENUM_FLAGS {}
impl ::core::clone::Clone for DMO_ENUM_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DMO_ENUM_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for DMO_ENUM_FLAGS {
    type Abi = Self;
}
impl ::core::fmt::Debug for DMO_ENUM_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DMO_ENUM_FLAGS").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Media_DxMediaObjects\"`*"]
pub const DMO_E_INVALIDSTREAMINDEX: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147220991i32);
#[doc = "*Required features: `\"Win32_Media_DxMediaObjects\"`*"]
pub const DMO_E_INVALIDTYPE: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147220990i32);
#[doc = "*Required features: `\"Win32_Media_DxMediaObjects\"`*"]
pub const DMO_E_NOTACCEPTING: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147220988i32);
#[doc = "*Required features: `\"Win32_Media_DxMediaObjects\"`*"]
pub const DMO_E_NO_MORE_ITEMS: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147220986i32);
#[doc = "*Required features: `\"Win32_Media_DxMediaObjects\"`*"]
pub const DMO_E_TYPE_NOT_ACCEPTED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147220987i32);
#[doc = "*Required features: `\"Win32_Media_DxMediaObjects\"`*"]
pub const DMO_E_TYPE_NOT_SET: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147220989i32);
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_DxMediaObjects\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
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
impl ::core::clone::Clone for DMO_MEDIA_TYPE {
    fn clone(&self) -> Self {
        Self {
            majortype: self.majortype,
            subtype: self.subtype,
            bFixedSizeSamples: self.bFixedSizeSamples,
            bTemporalCompression: self.bTemporalCompression,
            lSampleSize: self.lSampleSize,
            formattype: self.formattype,
            pUnk: self.pUnk.clone(),
            cbFormat: self.cbFormat,
            pbFormat: self.pbFormat,
        }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for DMO_MEDIA_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DMO_MEDIA_TYPE").field("majortype", &self.majortype).field("subtype", &self.subtype).field("bFixedSizeSamples", &self.bFixedSizeSamples).field("bTemporalCompression", &self.bTemporalCompression).field("lSampleSize", &self.lSampleSize).field("formattype", &self.formattype).field("pUnk", &self.pUnk).field("cbFormat", &self.cbFormat).field("pbFormat", &self.pbFormat).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for DMO_MEDIA_TYPE {
    type Abi = ::core::mem::ManuallyDrop<Self>;
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
impl ::core::default::Default for DMO_MEDIA_TYPE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_DxMediaObjects\"`*"]
pub struct DMO_OUTPUT_DATA_BUFFER {
    pub pBuffer: ::core::option::Option<IMediaBuffer>,
    pub dwStatus: u32,
    pub rtTimestamp: i64,
    pub rtTimelength: i64,
}
impl ::core::clone::Clone for DMO_OUTPUT_DATA_BUFFER {
    fn clone(&self) -> Self {
        Self { pBuffer: self.pBuffer.clone(), dwStatus: self.dwStatus, rtTimestamp: self.rtTimestamp, rtTimelength: self.rtTimelength }
    }
}
impl ::core::fmt::Debug for DMO_OUTPUT_DATA_BUFFER {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DMO_OUTPUT_DATA_BUFFER").field("pBuffer", &self.pBuffer).field("dwStatus", &self.dwStatus).field("rtTimestamp", &self.rtTimestamp).field("rtTimelength", &self.rtTimelength).finish()
    }
}
unsafe impl ::windows::core::Abi for DMO_OUTPUT_DATA_BUFFER {
    type Abi = ::core::mem::ManuallyDrop<Self>;
}
impl ::core::cmp::PartialEq for DMO_OUTPUT_DATA_BUFFER {
    fn eq(&self, other: &Self) -> bool {
        self.pBuffer == other.pBuffer && self.dwStatus == other.dwStatus && self.rtTimestamp == other.rtTimestamp && self.rtTimelength == other.rtTimelength
    }
}
impl ::core::cmp::Eq for DMO_OUTPUT_DATA_BUFFER {}
impl ::core::default::Default for DMO_OUTPUT_DATA_BUFFER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_DxMediaObjects\"`*"]
pub struct DMO_PARTIAL_MEDIATYPE {
    pub r#type: ::windows::core::GUID,
    pub subtype: ::windows::core::GUID,
}
impl ::core::marker::Copy for DMO_PARTIAL_MEDIATYPE {}
impl ::core::clone::Clone for DMO_PARTIAL_MEDIATYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DMO_PARTIAL_MEDIATYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DMO_PARTIAL_MEDIATYPE").field("type", &self.r#type).field("subtype", &self.subtype).finish()
    }
}
unsafe impl ::windows::core::Abi for DMO_PARTIAL_MEDIATYPE {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for DMO_PARTIAL_MEDIATYPE {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DMO_PARTIAL_MEDIATYPE>()) == 0 }
    }
}
impl ::core::cmp::Eq for DMO_PARTIAL_MEDIATYPE {}
impl ::core::default::Default for DMO_PARTIAL_MEDIATYPE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_Media_DxMediaObjects\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct DMO_REGISTER_FLAGS(pub i32);
#[doc = "*Required features: `\"Win32_Media_DxMediaObjects\"`*"]
pub const DMO_REGISTERF_IS_KEYED: DMO_REGISTER_FLAGS = DMO_REGISTER_FLAGS(1i32);
impl ::core::marker::Copy for DMO_REGISTER_FLAGS {}
impl ::core::clone::Clone for DMO_REGISTER_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DMO_REGISTER_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for DMO_REGISTER_FLAGS {
    type Abi = Self;
}
impl ::core::fmt::Debug for DMO_REGISTER_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DMO_REGISTER_FLAGS").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Media_DxMediaObjects\"`*"]
#[repr(transparent)]
pub struct IDMOQualityControl(::windows::core::IUnknown);
impl IDMOQualityControl {
    pub unsafe fn SetNow(&self, rtnow: i64) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetNow)(::windows::core::Interface::as_raw(self), rtnow).ok()
    }
    pub unsafe fn SetStatus(&self, dwflags: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetStatus)(::windows::core::Interface::as_raw(self), dwflags).ok()
    }
    pub unsafe fn GetStatus(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).GetStatus)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u32>(result__)
    }
}
impl ::core::convert::From<IDMOQualityControl> for ::windows::core::IUnknown {
    fn from(value: IDMOQualityControl) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IDMOQualityControl> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IDMOQualityControl) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDMOQualityControl> for ::windows::core::IUnknown {
    fn from(value: &IDMOQualityControl) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::clone::Clone for IDMOQualityControl {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IDMOQualityControl {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDMOQualityControl {}
impl ::core::fmt::Debug for IDMOQualityControl {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDMOQualityControl").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IDMOQualityControl {
    type Vtable = IDMOQualityControl_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x65abea96_cf36_453f_af8a_705e98f16260);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDMOQualityControl_Vtbl {
    pub base__: ::windows::core::IUnknownVtbl,
    pub SetNow: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, rtnow: i64) -> ::windows::core::HRESULT,
    pub SetStatus: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwflags: u32) -> ::windows::core::HRESULT,
    pub GetStatus: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdwflags: *mut u32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Media_DxMediaObjects\"`*"]
#[repr(transparent)]
pub struct IDMOVideoOutputOptimizations(::windows::core::IUnknown);
impl IDMOVideoOutputOptimizations {
    pub unsafe fn QueryOperationModePreferences(&self, uloutputstreamindex: u32) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).QueryOperationModePreferences)(::windows::core::Interface::as_raw(self), uloutputstreamindex, ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u32>(result__)
    }
    pub unsafe fn SetOperationMode(&self, uloutputstreamindex: u32, dwenabledfeatures: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetOperationMode)(::windows::core::Interface::as_raw(self), uloutputstreamindex, dwenabledfeatures).ok()
    }
    pub unsafe fn GetCurrentOperationMode(&self, uloutputstreamindex: u32) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).GetCurrentOperationMode)(::windows::core::Interface::as_raw(self), uloutputstreamindex, ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u32>(result__)
    }
    pub unsafe fn GetCurrentSampleRequirements(&self, uloutputstreamindex: u32) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).GetCurrentSampleRequirements)(::windows::core::Interface::as_raw(self), uloutputstreamindex, ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u32>(result__)
    }
}
impl ::core::convert::From<IDMOVideoOutputOptimizations> for ::windows::core::IUnknown {
    fn from(value: IDMOVideoOutputOptimizations) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IDMOVideoOutputOptimizations> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IDMOVideoOutputOptimizations) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDMOVideoOutputOptimizations> for ::windows::core::IUnknown {
    fn from(value: &IDMOVideoOutputOptimizations) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::clone::Clone for IDMOVideoOutputOptimizations {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IDMOVideoOutputOptimizations {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDMOVideoOutputOptimizations {}
impl ::core::fmt::Debug for IDMOVideoOutputOptimizations {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDMOVideoOutputOptimizations").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IDMOVideoOutputOptimizations {
    type Vtable = IDMOVideoOutputOptimizations_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xbe8f4f4e_5b16_4d29_b350_7f6b5d9298ac);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDMOVideoOutputOptimizations_Vtbl {
    pub base__: ::windows::core::IUnknownVtbl,
    pub QueryOperationModePreferences: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, uloutputstreamindex: u32, pdwrequestedcapabilities: *mut u32) -> ::windows::core::HRESULT,
    pub SetOperationMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, uloutputstreamindex: u32, dwenabledfeatures: u32) -> ::windows::core::HRESULT,
    pub GetCurrentOperationMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, uloutputstreamindex: u32, pdwenabledfeatures: *mut u32) -> ::windows::core::HRESULT,
    pub GetCurrentSampleRequirements: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, uloutputstreamindex: u32, pdwrequestedfeatures: *mut u32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Media_DxMediaObjects\"`*"]
#[repr(transparent)]
pub struct IEnumDMO(::windows::core::IUnknown);
impl IEnumDMO {
    pub unsafe fn Next(&self, citemstofetch: u32, pclsid: *mut ::windows::core::GUID, names: *mut ::windows::core::PWSTR, pcitemsfetched: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Next)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(citemstofetch), ::core::mem::transmute(pclsid), ::core::mem::transmute(names), ::core::mem::transmute(pcitemsfetched)).ok()
    }
    pub unsafe fn Skip(&self, citemstoskip: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Skip)(::windows::core::Interface::as_raw(self), citemstoskip).ok()
    }
    pub unsafe fn Reset(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Reset)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Clone(&self) -> ::windows::core::Result<IEnumDMO> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).Clone)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IEnumDMO>(result__)
    }
}
impl ::core::convert::From<IEnumDMO> for ::windows::core::IUnknown {
    fn from(value: IEnumDMO) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IEnumDMO> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IEnumDMO) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IEnumDMO> for ::windows::core::IUnknown {
    fn from(value: &IEnumDMO) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::clone::Clone for IEnumDMO {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IEnumDMO {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IEnumDMO {}
impl ::core::fmt::Debug for IEnumDMO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IEnumDMO").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IEnumDMO {
    type Vtable = IEnumDMO_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2c3cd98a_2bfa_4a53_9c27_5249ba64ba0f);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEnumDMO_Vtbl {
    pub base__: ::windows::core::IUnknownVtbl,
    pub Next: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, citemstofetch: u32, pclsid: *mut ::windows::core::GUID, names: *mut ::windows::core::PWSTR, pcitemsfetched: *mut u32) -> ::windows::core::HRESULT,
    pub Skip: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, citemstoskip: u32) -> ::windows::core::HRESULT,
    pub Reset: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Clone: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Media_DxMediaObjects\"`*"]
#[repr(transparent)]
pub struct IMediaBuffer(::windows::core::IUnknown);
impl IMediaBuffer {
    pub unsafe fn SetLength(&self, cblength: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetLength)(::windows::core::Interface::as_raw(self), cblength).ok()
    }
    pub unsafe fn GetMaxLength(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).GetMaxLength)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u32>(result__)
    }
    pub unsafe fn GetBufferAndLength(&self, ppbuffer: *mut *mut u8, pcblength: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetBufferAndLength)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(ppbuffer), ::core::mem::transmute(pcblength)).ok()
    }
}
impl ::core::convert::From<IMediaBuffer> for ::windows::core::IUnknown {
    fn from(value: IMediaBuffer) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IMediaBuffer> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IMediaBuffer) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IMediaBuffer> for ::windows::core::IUnknown {
    fn from(value: &IMediaBuffer) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::clone::Clone for IMediaBuffer {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IMediaBuffer {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IMediaBuffer {}
impl ::core::fmt::Debug for IMediaBuffer {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMediaBuffer").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IMediaBuffer {
    type Vtable = IMediaBuffer_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x59eff8b9_938c_4a26_82f2_95cb84cdc837);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaBuffer_Vtbl {
    pub base__: ::windows::core::IUnknownVtbl,
    pub SetLength: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cblength: u32) -> ::windows::core::HRESULT,
    pub GetMaxLength: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcbmaxlength: *mut u32) -> ::windows::core::HRESULT,
    pub GetBufferAndLength: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppbuffer: *mut *mut u8, pcblength: *mut u32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Media_DxMediaObjects\"`*"]
#[repr(transparent)]
pub struct IMediaObject(::windows::core::IUnknown);
impl IMediaObject {
    pub unsafe fn GetStreamCount(&self, pcinputstreams: *mut u32, pcoutputstreams: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetStreamCount)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pcinputstreams), ::core::mem::transmute(pcoutputstreams)).ok()
    }
    pub unsafe fn GetInputStreamInfo(&self, dwinputstreamindex: u32) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).GetInputStreamInfo)(::windows::core::Interface::as_raw(self), dwinputstreamindex, ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u32>(result__)
    }
    pub unsafe fn GetOutputStreamInfo(&self, dwoutputstreamindex: u32) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).GetOutputStreamInfo)(::windows::core::Interface::as_raw(self), dwoutputstreamindex, ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetInputType(&self, dwinputstreamindex: u32, dwtypeindex: u32) -> ::windows::core::Result<DMO_MEDIA_TYPE> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).GetInputType)(::windows::core::Interface::as_raw(self), dwinputstreamindex, dwtypeindex, ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<DMO_MEDIA_TYPE>(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetOutputType(&self, dwoutputstreamindex: u32, dwtypeindex: u32) -> ::windows::core::Result<DMO_MEDIA_TYPE> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).GetOutputType)(::windows::core::Interface::as_raw(self), dwoutputstreamindex, dwtypeindex, ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<DMO_MEDIA_TYPE>(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetInputType(&self, dwinputstreamindex: u32, pmt: *const DMO_MEDIA_TYPE, dwflags: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetInputType)(::windows::core::Interface::as_raw(self), dwinputstreamindex, ::core::mem::transmute(pmt), dwflags).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetOutputType(&self, dwoutputstreamindex: u32, pmt: *const DMO_MEDIA_TYPE, dwflags: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetOutputType)(::windows::core::Interface::as_raw(self), dwoutputstreamindex, ::core::mem::transmute(pmt), dwflags).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetInputCurrentType(&self, dwinputstreamindex: u32) -> ::windows::core::Result<DMO_MEDIA_TYPE> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).GetInputCurrentType)(::windows::core::Interface::as_raw(self), dwinputstreamindex, ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<DMO_MEDIA_TYPE>(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetOutputCurrentType(&self, dwoutputstreamindex: u32) -> ::windows::core::Result<DMO_MEDIA_TYPE> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).GetOutputCurrentType)(::windows::core::Interface::as_raw(self), dwoutputstreamindex, ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<DMO_MEDIA_TYPE>(result__)
    }
    pub unsafe fn GetInputSizeInfo(&self, dwinputstreamindex: u32, pcbsize: *mut u32, pcbmaxlookahead: *mut u32, pcbalignment: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetInputSizeInfo)(::windows::core::Interface::as_raw(self), dwinputstreamindex, ::core::mem::transmute(pcbsize), ::core::mem::transmute(pcbmaxlookahead), ::core::mem::transmute(pcbalignment)).ok()
    }
    pub unsafe fn GetOutputSizeInfo(&self, dwoutputstreamindex: u32, pcbsize: *mut u32, pcbalignment: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetOutputSizeInfo)(::windows::core::Interface::as_raw(self), dwoutputstreamindex, ::core::mem::transmute(pcbsize), ::core::mem::transmute(pcbalignment)).ok()
    }
    pub unsafe fn GetInputMaxLatency(&self, dwinputstreamindex: u32) -> ::windows::core::Result<i64> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).GetInputMaxLatency)(::windows::core::Interface::as_raw(self), dwinputstreamindex, ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i64>(result__)
    }
    pub unsafe fn SetInputMaxLatency(&self, dwinputstreamindex: u32, rtmaxlatency: i64) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetInputMaxLatency)(::windows::core::Interface::as_raw(self), dwinputstreamindex, rtmaxlatency).ok()
    }
    pub unsafe fn Flush(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Flush)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Discontinuity(&self, dwinputstreamindex: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Discontinuity)(::windows::core::Interface::as_raw(self), dwinputstreamindex).ok()
    }
    pub unsafe fn AllocateStreamingResources(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).AllocateStreamingResources)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn FreeStreamingResources(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).FreeStreamingResources)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn GetInputStatus(&self, dwinputstreamindex: u32) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).GetInputStatus)(::windows::core::Interface::as_raw(self), dwinputstreamindex, ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u32>(result__)
    }
    pub unsafe fn ProcessInput<'a, P0>(&self, dwinputstreamindex: u32, pbuffer: P0, dwflags: u32, rttimestamp: i64, rttimelength: i64) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, IMediaBuffer>>,
    {
        (::windows::core::Interface::vtable(self).ProcessInput)(::windows::core::Interface::as_raw(self), dwinputstreamindex, pbuffer.into().abi(), dwflags, rttimestamp, rttimelength).ok()
    }
    pub unsafe fn ProcessOutput(&self, dwflags: u32, poutputbuffers: &mut [DMO_OUTPUT_DATA_BUFFER], pdwstatus: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).ProcessOutput)(::windows::core::Interface::as_raw(self), dwflags, poutputbuffers.len() as _, ::core::mem::transmute(::windows::core::as_mut_ptr_or_null(poutputbuffers)), ::core::mem::transmute(pdwstatus)).ok()
    }
    pub unsafe fn Lock(&self, block: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Lock)(::windows::core::Interface::as_raw(self), block).ok()
    }
}
impl ::core::convert::From<IMediaObject> for ::windows::core::IUnknown {
    fn from(value: IMediaObject) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IMediaObject> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IMediaObject) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IMediaObject> for ::windows::core::IUnknown {
    fn from(value: &IMediaObject) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::clone::Clone for IMediaObject {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IMediaObject {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IMediaObject {}
impl ::core::fmt::Debug for IMediaObject {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMediaObject").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IMediaObject {
    type Vtable = IMediaObject_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd8ad0f58_5494_4102_97c5_ec798e59bcf4);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaObject_Vtbl {
    pub base__: ::windows::core::IUnknownVtbl,
    pub GetStreamCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcinputstreams: *mut u32, pcoutputstreams: *mut u32) -> ::windows::core::HRESULT,
    pub GetInputStreamInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwinputstreamindex: u32, pdwflags: *mut u32) -> ::windows::core::HRESULT,
    pub GetOutputStreamInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwoutputstreamindex: u32, pdwflags: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub GetInputType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwinputstreamindex: u32, dwtypeindex: u32, pmt: *mut DMO_MEDIA_TYPE) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetInputType: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub GetOutputType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwoutputstreamindex: u32, dwtypeindex: u32, pmt: *mut DMO_MEDIA_TYPE) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetOutputType: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetInputType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwinputstreamindex: u32, pmt: *const DMO_MEDIA_TYPE, dwflags: u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetInputType: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetOutputType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwoutputstreamindex: u32, pmt: *const DMO_MEDIA_TYPE, dwflags: u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetOutputType: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub GetInputCurrentType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwinputstreamindex: u32, pmt: *mut DMO_MEDIA_TYPE) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetInputCurrentType: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub GetOutputCurrentType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwoutputstreamindex: u32, pmt: *mut DMO_MEDIA_TYPE) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetOutputCurrentType: usize,
    pub GetInputSizeInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwinputstreamindex: u32, pcbsize: *mut u32, pcbmaxlookahead: *mut u32, pcbalignment: *mut u32) -> ::windows::core::HRESULT,
    pub GetOutputSizeInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwoutputstreamindex: u32, pcbsize: *mut u32, pcbalignment: *mut u32) -> ::windows::core::HRESULT,
    pub GetInputMaxLatency: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwinputstreamindex: u32, prtmaxlatency: *mut i64) -> ::windows::core::HRESULT,
    pub SetInputMaxLatency: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwinputstreamindex: u32, rtmaxlatency: i64) -> ::windows::core::HRESULT,
    pub Flush: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Discontinuity: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwinputstreamindex: u32) -> ::windows::core::HRESULT,
    pub AllocateStreamingResources: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub FreeStreamingResources: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetInputStatus: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwinputstreamindex: u32, dwflags: *mut u32) -> ::windows::core::HRESULT,
    pub ProcessInput: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwinputstreamindex: u32, pbuffer: *mut ::core::ffi::c_void, dwflags: u32, rttimestamp: i64, rttimelength: i64) -> ::windows::core::HRESULT,
    pub ProcessOutput: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwflags: u32, coutputbuffercount: u32, poutputbuffers: *mut DMO_OUTPUT_DATA_BUFFER, pdwstatus: *mut u32) -> ::windows::core::HRESULT,
    pub Lock: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, block: i32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Media_DxMediaObjects\"`*"]
#[repr(transparent)]
pub struct IMediaObjectInPlace(::windows::core::IUnknown);
impl IMediaObjectInPlace {
    pub unsafe fn Process(&self, ulsize: u32, pdata: *mut u8, reftimestart: i64, dwflags: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Process)(::windows::core::Interface::as_raw(self), ulsize, ::core::mem::transmute(pdata), reftimestart, dwflags).ok()
    }
    pub unsafe fn Clone(&self) -> ::windows::core::Result<IMediaObjectInPlace> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).Clone)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IMediaObjectInPlace>(result__)
    }
    pub unsafe fn GetLatency(&self) -> ::windows::core::Result<i64> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).GetLatency)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i64>(result__)
    }
}
impl ::core::convert::From<IMediaObjectInPlace> for ::windows::core::IUnknown {
    fn from(value: IMediaObjectInPlace) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IMediaObjectInPlace> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IMediaObjectInPlace) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IMediaObjectInPlace> for ::windows::core::IUnknown {
    fn from(value: &IMediaObjectInPlace) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::clone::Clone for IMediaObjectInPlace {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IMediaObjectInPlace {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IMediaObjectInPlace {}
impl ::core::fmt::Debug for IMediaObjectInPlace {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMediaObjectInPlace").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IMediaObjectInPlace {
    type Vtable = IMediaObjectInPlace_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x651b9ad0_0fc7_4aa9_9538_d89931010741);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaObjectInPlace_Vtbl {
    pub base__: ::windows::core::IUnknownVtbl,
    pub Process: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ulsize: u32, pdata: *mut u8, reftimestart: i64, dwflags: u32) -> ::windows::core::HRESULT,
    pub Clone: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppmediaobject: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetLatency: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, platencytime: *mut i64) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Media_DxMediaObjects\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn MoCopyMediaType(pmtdest: *mut DMO_MEDIA_TYPE, pmtsrc: *const DMO_MEDIA_TYPE) -> ::windows::core::Result<()> {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn MoCopyMediaType(pmtdest: *mut DMO_MEDIA_TYPE, pmtsrc: *const DMO_MEDIA_TYPE) -> ::windows::core::HRESULT;
    }
    MoCopyMediaType(::core::mem::transmute(pmtdest), ::core::mem::transmute(pmtsrc)).ok()
}
#[doc = "*Required features: `\"Win32_Media_DxMediaObjects\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn MoCreateMediaType(ppmt: *mut *mut DMO_MEDIA_TYPE, cbformat: u32) -> ::windows::core::Result<()> {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn MoCreateMediaType(ppmt: *mut *mut DMO_MEDIA_TYPE, cbformat: u32) -> ::windows::core::HRESULT;
    }
    MoCreateMediaType(::core::mem::transmute(ppmt), cbformat).ok()
}
#[doc = "*Required features: `\"Win32_Media_DxMediaObjects\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn MoDeleteMediaType(pmt: *mut DMO_MEDIA_TYPE) -> ::windows::core::Result<()> {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn MoDeleteMediaType(pmt: *mut DMO_MEDIA_TYPE) -> ::windows::core::HRESULT;
    }
    MoDeleteMediaType(::core::mem::transmute(pmt)).ok()
}
#[doc = "*Required features: `\"Win32_Media_DxMediaObjects\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn MoDuplicateMediaType(ppmtdest: *mut *mut DMO_MEDIA_TYPE, pmtsrc: *const DMO_MEDIA_TYPE) -> ::windows::core::Result<()> {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn MoDuplicateMediaType(ppmtdest: *mut *mut DMO_MEDIA_TYPE, pmtsrc: *const DMO_MEDIA_TYPE) -> ::windows::core::HRESULT;
    }
    MoDuplicateMediaType(::core::mem::transmute(ppmtdest), ::core::mem::transmute(pmtsrc)).ok()
}
#[doc = "*Required features: `\"Win32_Media_DxMediaObjects\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn MoFreeMediaType(pmt: *mut DMO_MEDIA_TYPE) -> ::windows::core::Result<()> {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn MoFreeMediaType(pmt: *mut DMO_MEDIA_TYPE) -> ::windows::core::HRESULT;
    }
    MoFreeMediaType(::core::mem::transmute(pmt)).ok()
}
#[doc = "*Required features: `\"Win32_Media_DxMediaObjects\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn MoInitMediaType(pmt: *mut DMO_MEDIA_TYPE, cbformat: u32) -> ::windows::core::Result<()> {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn MoInitMediaType(pmt: *mut DMO_MEDIA_TYPE, cbformat: u32) -> ::windows::core::HRESULT;
    }
    MoInitMediaType(::core::mem::transmute(pmt), cbformat).ok()
}
#[doc = "*Required features: `\"Win32_Media_DxMediaObjects\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct _DMO_INPLACE_PROCESS_FLAGS(pub i32);
#[doc = "*Required features: `\"Win32_Media_DxMediaObjects\"`*"]
pub const DMO_INPLACE_NORMAL: _DMO_INPLACE_PROCESS_FLAGS = _DMO_INPLACE_PROCESS_FLAGS(0i32);
#[doc = "*Required features: `\"Win32_Media_DxMediaObjects\"`*"]
pub const DMO_INPLACE_ZERO: _DMO_INPLACE_PROCESS_FLAGS = _DMO_INPLACE_PROCESS_FLAGS(1i32);
impl ::core::marker::Copy for _DMO_INPLACE_PROCESS_FLAGS {}
impl ::core::clone::Clone for _DMO_INPLACE_PROCESS_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for _DMO_INPLACE_PROCESS_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for _DMO_INPLACE_PROCESS_FLAGS {
    type Abi = Self;
}
impl ::core::fmt::Debug for _DMO_INPLACE_PROCESS_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("_DMO_INPLACE_PROCESS_FLAGS").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Media_DxMediaObjects\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct _DMO_INPUT_DATA_BUFFER_FLAGS(pub i32);
#[doc = "*Required features: `\"Win32_Media_DxMediaObjects\"`*"]
pub const DMO_INPUT_DATA_BUFFERF_SYNCPOINT: _DMO_INPUT_DATA_BUFFER_FLAGS = _DMO_INPUT_DATA_BUFFER_FLAGS(1i32);
#[doc = "*Required features: `\"Win32_Media_DxMediaObjects\"`*"]
pub const DMO_INPUT_DATA_BUFFERF_TIME: _DMO_INPUT_DATA_BUFFER_FLAGS = _DMO_INPUT_DATA_BUFFER_FLAGS(2i32);
#[doc = "*Required features: `\"Win32_Media_DxMediaObjects\"`*"]
pub const DMO_INPUT_DATA_BUFFERF_TIMELENGTH: _DMO_INPUT_DATA_BUFFER_FLAGS = _DMO_INPUT_DATA_BUFFER_FLAGS(4i32);
#[doc = "*Required features: `\"Win32_Media_DxMediaObjects\"`*"]
pub const DMO_INPUT_DATA_BUFFERF_DISCONTINUITY: _DMO_INPUT_DATA_BUFFER_FLAGS = _DMO_INPUT_DATA_BUFFER_FLAGS(8i32);
impl ::core::marker::Copy for _DMO_INPUT_DATA_BUFFER_FLAGS {}
impl ::core::clone::Clone for _DMO_INPUT_DATA_BUFFER_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for _DMO_INPUT_DATA_BUFFER_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for _DMO_INPUT_DATA_BUFFER_FLAGS {
    type Abi = Self;
}
impl ::core::fmt::Debug for _DMO_INPUT_DATA_BUFFER_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("_DMO_INPUT_DATA_BUFFER_FLAGS").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Media_DxMediaObjects\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct _DMO_INPUT_STATUS_FLAGS(pub i32);
#[doc = "*Required features: `\"Win32_Media_DxMediaObjects\"`*"]
pub const DMO_INPUT_STATUSF_ACCEPT_DATA: _DMO_INPUT_STATUS_FLAGS = _DMO_INPUT_STATUS_FLAGS(1i32);
impl ::core::marker::Copy for _DMO_INPUT_STATUS_FLAGS {}
impl ::core::clone::Clone for _DMO_INPUT_STATUS_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for _DMO_INPUT_STATUS_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for _DMO_INPUT_STATUS_FLAGS {
    type Abi = Self;
}
impl ::core::fmt::Debug for _DMO_INPUT_STATUS_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("_DMO_INPUT_STATUS_FLAGS").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Media_DxMediaObjects\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct _DMO_INPUT_STREAM_INFO_FLAGS(pub i32);
#[doc = "*Required features: `\"Win32_Media_DxMediaObjects\"`*"]
pub const DMO_INPUT_STREAMF_WHOLE_SAMPLES: _DMO_INPUT_STREAM_INFO_FLAGS = _DMO_INPUT_STREAM_INFO_FLAGS(1i32);
#[doc = "*Required features: `\"Win32_Media_DxMediaObjects\"`*"]
pub const DMO_INPUT_STREAMF_SINGLE_SAMPLE_PER_BUFFER: _DMO_INPUT_STREAM_INFO_FLAGS = _DMO_INPUT_STREAM_INFO_FLAGS(2i32);
#[doc = "*Required features: `\"Win32_Media_DxMediaObjects\"`*"]
pub const DMO_INPUT_STREAMF_FIXED_SAMPLE_SIZE: _DMO_INPUT_STREAM_INFO_FLAGS = _DMO_INPUT_STREAM_INFO_FLAGS(4i32);
#[doc = "*Required features: `\"Win32_Media_DxMediaObjects\"`*"]
pub const DMO_INPUT_STREAMF_HOLDS_BUFFERS: _DMO_INPUT_STREAM_INFO_FLAGS = _DMO_INPUT_STREAM_INFO_FLAGS(8i32);
impl ::core::marker::Copy for _DMO_INPUT_STREAM_INFO_FLAGS {}
impl ::core::clone::Clone for _DMO_INPUT_STREAM_INFO_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for _DMO_INPUT_STREAM_INFO_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for _DMO_INPUT_STREAM_INFO_FLAGS {
    type Abi = Self;
}
impl ::core::fmt::Debug for _DMO_INPUT_STREAM_INFO_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("_DMO_INPUT_STREAM_INFO_FLAGS").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Media_DxMediaObjects\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct _DMO_OUTPUT_DATA_BUFFER_FLAGS(pub i32);
#[doc = "*Required features: `\"Win32_Media_DxMediaObjects\"`*"]
pub const DMO_OUTPUT_DATA_BUFFERF_SYNCPOINT: _DMO_OUTPUT_DATA_BUFFER_FLAGS = _DMO_OUTPUT_DATA_BUFFER_FLAGS(1i32);
#[doc = "*Required features: `\"Win32_Media_DxMediaObjects\"`*"]
pub const DMO_OUTPUT_DATA_BUFFERF_TIME: _DMO_OUTPUT_DATA_BUFFER_FLAGS = _DMO_OUTPUT_DATA_BUFFER_FLAGS(2i32);
#[doc = "*Required features: `\"Win32_Media_DxMediaObjects\"`*"]
pub const DMO_OUTPUT_DATA_BUFFERF_TIMELENGTH: _DMO_OUTPUT_DATA_BUFFER_FLAGS = _DMO_OUTPUT_DATA_BUFFER_FLAGS(4i32);
#[doc = "*Required features: `\"Win32_Media_DxMediaObjects\"`*"]
pub const DMO_OUTPUT_DATA_BUFFERF_DISCONTINUITY: _DMO_OUTPUT_DATA_BUFFER_FLAGS = _DMO_OUTPUT_DATA_BUFFER_FLAGS(8i32);
#[doc = "*Required features: `\"Win32_Media_DxMediaObjects\"`*"]
pub const DMO_OUTPUT_DATA_BUFFERF_INCOMPLETE: _DMO_OUTPUT_DATA_BUFFER_FLAGS = _DMO_OUTPUT_DATA_BUFFER_FLAGS(16777216i32);
impl ::core::marker::Copy for _DMO_OUTPUT_DATA_BUFFER_FLAGS {}
impl ::core::clone::Clone for _DMO_OUTPUT_DATA_BUFFER_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for _DMO_OUTPUT_DATA_BUFFER_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for _DMO_OUTPUT_DATA_BUFFER_FLAGS {
    type Abi = Self;
}
impl ::core::fmt::Debug for _DMO_OUTPUT_DATA_BUFFER_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("_DMO_OUTPUT_DATA_BUFFER_FLAGS").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Media_DxMediaObjects\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct _DMO_OUTPUT_STREAM_INFO_FLAGS(pub i32);
#[doc = "*Required features: `\"Win32_Media_DxMediaObjects\"`*"]
pub const DMO_OUTPUT_STREAMF_WHOLE_SAMPLES: _DMO_OUTPUT_STREAM_INFO_FLAGS = _DMO_OUTPUT_STREAM_INFO_FLAGS(1i32);
#[doc = "*Required features: `\"Win32_Media_DxMediaObjects\"`*"]
pub const DMO_OUTPUT_STREAMF_SINGLE_SAMPLE_PER_BUFFER: _DMO_OUTPUT_STREAM_INFO_FLAGS = _DMO_OUTPUT_STREAM_INFO_FLAGS(2i32);
#[doc = "*Required features: `\"Win32_Media_DxMediaObjects\"`*"]
pub const DMO_OUTPUT_STREAMF_FIXED_SAMPLE_SIZE: _DMO_OUTPUT_STREAM_INFO_FLAGS = _DMO_OUTPUT_STREAM_INFO_FLAGS(4i32);
#[doc = "*Required features: `\"Win32_Media_DxMediaObjects\"`*"]
pub const DMO_OUTPUT_STREAMF_DISCARDABLE: _DMO_OUTPUT_STREAM_INFO_FLAGS = _DMO_OUTPUT_STREAM_INFO_FLAGS(8i32);
#[doc = "*Required features: `\"Win32_Media_DxMediaObjects\"`*"]
pub const DMO_OUTPUT_STREAMF_OPTIONAL: _DMO_OUTPUT_STREAM_INFO_FLAGS = _DMO_OUTPUT_STREAM_INFO_FLAGS(16i32);
impl ::core::marker::Copy for _DMO_OUTPUT_STREAM_INFO_FLAGS {}
impl ::core::clone::Clone for _DMO_OUTPUT_STREAM_INFO_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for _DMO_OUTPUT_STREAM_INFO_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for _DMO_OUTPUT_STREAM_INFO_FLAGS {
    type Abi = Self;
}
impl ::core::fmt::Debug for _DMO_OUTPUT_STREAM_INFO_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("_DMO_OUTPUT_STREAM_INFO_FLAGS").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Media_DxMediaObjects\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct _DMO_PROCESS_OUTPUT_FLAGS(pub i32);
#[doc = "*Required features: `\"Win32_Media_DxMediaObjects\"`*"]
pub const DMO_PROCESS_OUTPUT_DISCARD_WHEN_NO_BUFFER: _DMO_PROCESS_OUTPUT_FLAGS = _DMO_PROCESS_OUTPUT_FLAGS(1i32);
impl ::core::marker::Copy for _DMO_PROCESS_OUTPUT_FLAGS {}
impl ::core::clone::Clone for _DMO_PROCESS_OUTPUT_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for _DMO_PROCESS_OUTPUT_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for _DMO_PROCESS_OUTPUT_FLAGS {
    type Abi = Self;
}
impl ::core::fmt::Debug for _DMO_PROCESS_OUTPUT_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("_DMO_PROCESS_OUTPUT_FLAGS").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Media_DxMediaObjects\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct _DMO_QUALITY_STATUS_FLAGS(pub i32);
#[doc = "*Required features: `\"Win32_Media_DxMediaObjects\"`*"]
pub const DMO_QUALITY_STATUS_ENABLED: _DMO_QUALITY_STATUS_FLAGS = _DMO_QUALITY_STATUS_FLAGS(1i32);
impl ::core::marker::Copy for _DMO_QUALITY_STATUS_FLAGS {}
impl ::core::clone::Clone for _DMO_QUALITY_STATUS_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for _DMO_QUALITY_STATUS_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for _DMO_QUALITY_STATUS_FLAGS {
    type Abi = Self;
}
impl ::core::fmt::Debug for _DMO_QUALITY_STATUS_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("_DMO_QUALITY_STATUS_FLAGS").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Media_DxMediaObjects\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct _DMO_SET_TYPE_FLAGS(pub i32);
#[doc = "*Required features: `\"Win32_Media_DxMediaObjects\"`*"]
pub const DMO_SET_TYPEF_TEST_ONLY: _DMO_SET_TYPE_FLAGS = _DMO_SET_TYPE_FLAGS(1i32);
#[doc = "*Required features: `\"Win32_Media_DxMediaObjects\"`*"]
pub const DMO_SET_TYPEF_CLEAR: _DMO_SET_TYPE_FLAGS = _DMO_SET_TYPE_FLAGS(2i32);
impl ::core::marker::Copy for _DMO_SET_TYPE_FLAGS {}
impl ::core::clone::Clone for _DMO_SET_TYPE_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for _DMO_SET_TYPE_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for _DMO_SET_TYPE_FLAGS {
    type Abi = Self;
}
impl ::core::fmt::Debug for _DMO_SET_TYPE_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("_DMO_SET_TYPE_FLAGS").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Media_DxMediaObjects\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct _DMO_VIDEO_OUTPUT_STREAM_FLAGS(pub i32);
#[doc = "*Required features: `\"Win32_Media_DxMediaObjects\"`*"]
pub const DMO_VOSF_NEEDS_PREVIOUS_SAMPLE: _DMO_VIDEO_OUTPUT_STREAM_FLAGS = _DMO_VIDEO_OUTPUT_STREAM_FLAGS(1i32);
impl ::core::marker::Copy for _DMO_VIDEO_OUTPUT_STREAM_FLAGS {}
impl ::core::clone::Clone for _DMO_VIDEO_OUTPUT_STREAM_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for _DMO_VIDEO_OUTPUT_STREAM_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for _DMO_VIDEO_OUTPUT_STREAM_FLAGS {
    type Abi = Self;
}
impl ::core::fmt::Debug for _DMO_VIDEO_OUTPUT_STREAM_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("_DMO_VIDEO_OUTPUT_STREAM_FLAGS").field(&self.0).finish()
    }
}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
