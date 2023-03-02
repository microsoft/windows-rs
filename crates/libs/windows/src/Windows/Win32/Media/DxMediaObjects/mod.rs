#[doc = "*Required features: `\"Win32_Media_DxMediaObjects\"`*"]
#[inline]
pub unsafe fn DMOEnum(guidcategory: *const ::windows::core::GUID, dwflags: u32, cintypes: u32, pintypes: *const DMO_PARTIAL_MEDIATYPE, couttypes: u32, pouttypes: *const DMO_PARTIAL_MEDIATYPE) -> ::windows::core::Result<IEnumDMO> {
    ::windows::imp::link ! ( "msdmo.dll""system" fn DMOEnum ( guidcategory : *const :: windows::core::GUID , dwflags : u32 , cintypes : u32 , pintypes : *const DMO_PARTIAL_MEDIATYPE , couttypes : u32 , pouttypes : *const DMO_PARTIAL_MEDIATYPE , ppenum : *mut * mut::core::ffi::c_void ) -> :: windows::core::HRESULT );
    let mut result__ = ::windows::core::zeroed::<IEnumDMO>();
    DMOEnum(guidcategory, dwflags, cintypes, pintypes, couttypes, pouttypes, &mut result__).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_Media_DxMediaObjects\"`*"]
#[inline]
pub unsafe fn DMOGetName(clsiddmo: *const ::windows::core::GUID, szname: &mut [u16; 80]) -> ::windows::core::Result<()> {
    ::windows::imp::link ! ( "msdmo.dll""system" fn DMOGetName ( clsiddmo : *const :: windows::core::GUID , szname : :: windows::core::PWSTR ) -> :: windows::core::HRESULT );
    DMOGetName(clsiddmo, ::core::mem::transmute(szname.as_ptr())).ok()
}
#[doc = "*Required features: `\"Win32_Media_DxMediaObjects\"`*"]
#[inline]
pub unsafe fn DMOGetTypes(clsiddmo: *const ::windows::core::GUID, ulinputtypesrequested: u32, pulinputtypessupplied: *mut u32, pinputtypes: *mut DMO_PARTIAL_MEDIATYPE, uloutputtypesrequested: u32, puloutputtypessupplied: *mut u32, poutputtypes: *mut DMO_PARTIAL_MEDIATYPE) -> ::windows::core::Result<()> {
    ::windows::imp::link ! ( "msdmo.dll""system" fn DMOGetTypes ( clsiddmo : *const :: windows::core::GUID , ulinputtypesrequested : u32 , pulinputtypessupplied : *mut u32 , pinputtypes : *mut DMO_PARTIAL_MEDIATYPE , uloutputtypesrequested : u32 , puloutputtypessupplied : *mut u32 , poutputtypes : *mut DMO_PARTIAL_MEDIATYPE ) -> :: windows::core::HRESULT );
    DMOGetTypes(clsiddmo, ulinputtypesrequested, pulinputtypessupplied, pinputtypes, uloutputtypesrequested, puloutputtypessupplied, poutputtypes).ok()
}
#[doc = "*Required features: `\"Win32_Media_DxMediaObjects\"`*"]
#[inline]
pub unsafe fn DMORegister<P0>(szname: P0, clsiddmo: *const ::windows::core::GUID, guidcategory: *const ::windows::core::GUID, dwflags: u32, cintypes: u32, pintypes: *const DMO_PARTIAL_MEDIATYPE, couttypes: u32, pouttypes: *const DMO_PARTIAL_MEDIATYPE) -> ::windows::core::Result<()>
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows::imp::link ! ( "msdmo.dll""system" fn DMORegister ( szname : :: windows::core::PCWSTR , clsiddmo : *const :: windows::core::GUID , guidcategory : *const :: windows::core::GUID , dwflags : u32 , cintypes : u32 , pintypes : *const DMO_PARTIAL_MEDIATYPE , couttypes : u32 , pouttypes : *const DMO_PARTIAL_MEDIATYPE ) -> :: windows::core::HRESULT );
    DMORegister(szname.into_param().abi(), clsiddmo, guidcategory, dwflags, cintypes, pintypes, couttypes, pouttypes).ok()
}
#[doc = "*Required features: `\"Win32_Media_DxMediaObjects\"`*"]
#[inline]
pub unsafe fn DMOUnregister(clsiddmo: *const ::windows::core::GUID, guidcategory: *const ::windows::core::GUID) -> ::windows::core::Result<()> {
    ::windows::imp::link ! ( "msdmo.dll""system" fn DMOUnregister ( clsiddmo : *const :: windows::core::GUID , guidcategory : *const :: windows::core::GUID ) -> :: windows::core::HRESULT );
    DMOUnregister(clsiddmo, guidcategory).ok()
}
#[doc = "*Required features: `\"Win32_Media_DxMediaObjects\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn MoCopyMediaType(pmtdest: *mut DMO_MEDIA_TYPE, pmtsrc: *const DMO_MEDIA_TYPE) -> ::windows::core::Result<()> {
    ::windows::imp::link ! ( "msdmo.dll""system" fn MoCopyMediaType ( pmtdest : *mut DMO_MEDIA_TYPE , pmtsrc : *const DMO_MEDIA_TYPE ) -> :: windows::core::HRESULT );
    MoCopyMediaType(pmtdest, pmtsrc).ok()
}
#[doc = "*Required features: `\"Win32_Media_DxMediaObjects\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn MoCreateMediaType(ppmt: *mut *mut DMO_MEDIA_TYPE, cbformat: u32) -> ::windows::core::Result<()> {
    ::windows::imp::link ! ( "msdmo.dll""system" fn MoCreateMediaType ( ppmt : *mut *mut DMO_MEDIA_TYPE , cbformat : u32 ) -> :: windows::core::HRESULT );
    MoCreateMediaType(ppmt, cbformat).ok()
}
#[doc = "*Required features: `\"Win32_Media_DxMediaObjects\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn MoDeleteMediaType(pmt: *mut DMO_MEDIA_TYPE) -> ::windows::core::Result<()> {
    ::windows::imp::link ! ( "msdmo.dll""system" fn MoDeleteMediaType ( pmt : *mut DMO_MEDIA_TYPE ) -> :: windows::core::HRESULT );
    MoDeleteMediaType(pmt).ok()
}
#[doc = "*Required features: `\"Win32_Media_DxMediaObjects\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn MoDuplicateMediaType(ppmtdest: *mut *mut DMO_MEDIA_TYPE, pmtsrc: *const DMO_MEDIA_TYPE) -> ::windows::core::Result<()> {
    ::windows::imp::link ! ( "msdmo.dll""system" fn MoDuplicateMediaType ( ppmtdest : *mut *mut DMO_MEDIA_TYPE , pmtsrc : *const DMO_MEDIA_TYPE ) -> :: windows::core::HRESULT );
    MoDuplicateMediaType(ppmtdest, pmtsrc).ok()
}
#[doc = "*Required features: `\"Win32_Media_DxMediaObjects\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn MoFreeMediaType(pmt: *mut DMO_MEDIA_TYPE) -> ::windows::core::Result<()> {
    ::windows::imp::link ! ( "msdmo.dll""system" fn MoFreeMediaType ( pmt : *mut DMO_MEDIA_TYPE ) -> :: windows::core::HRESULT );
    MoFreeMediaType(pmt).ok()
}
#[doc = "*Required features: `\"Win32_Media_DxMediaObjects\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn MoInitMediaType(pmt: *mut DMO_MEDIA_TYPE, cbformat: u32) -> ::windows::core::Result<()> {
    ::windows::imp::link ! ( "msdmo.dll""system" fn MoInitMediaType ( pmt : *mut DMO_MEDIA_TYPE , cbformat : u32 ) -> :: windows::core::HRESULT );
    MoInitMediaType(pmt, cbformat).ok()
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
        let mut result__ = ::windows::core::zeroed::<u32>();
        (::windows::core::Interface::vtable(self).GetStatus)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
::windows::imp::interface_hierarchy!(IDMOQualityControl, ::windows::core::IUnknown);
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
}
impl ::core::clone::Clone for IDMOQualityControl {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IDMOQualityControl {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x65abea96_cf36_453f_af8a_705e98f16260);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDMOQualityControl_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub SetNow: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, rtnow: i64) -> ::windows::core::HRESULT,
    pub SetStatus: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwflags: u32) -> ::windows::core::HRESULT,
    pub GetStatus: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdwflags: *mut u32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Media_DxMediaObjects\"`*"]
#[repr(transparent)]
pub struct IDMOVideoOutputOptimizations(::windows::core::IUnknown);
impl IDMOVideoOutputOptimizations {
    pub unsafe fn QueryOperationModePreferences(&self, uloutputstreamindex: u32) -> ::windows::core::Result<u32> {
        let mut result__ = ::windows::core::zeroed::<u32>();
        (::windows::core::Interface::vtable(self).QueryOperationModePreferences)(::windows::core::Interface::as_raw(self), uloutputstreamindex, &mut result__).from_abi(result__)
    }
    pub unsafe fn SetOperationMode(&self, uloutputstreamindex: u32, dwenabledfeatures: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetOperationMode)(::windows::core::Interface::as_raw(self), uloutputstreamindex, dwenabledfeatures).ok()
    }
    pub unsafe fn GetCurrentOperationMode(&self, uloutputstreamindex: u32) -> ::windows::core::Result<u32> {
        let mut result__ = ::windows::core::zeroed::<u32>();
        (::windows::core::Interface::vtable(self).GetCurrentOperationMode)(::windows::core::Interface::as_raw(self), uloutputstreamindex, &mut result__).from_abi(result__)
    }
    pub unsafe fn GetCurrentSampleRequirements(&self, uloutputstreamindex: u32) -> ::windows::core::Result<u32> {
        let mut result__ = ::windows::core::zeroed::<u32>();
        (::windows::core::Interface::vtable(self).GetCurrentSampleRequirements)(::windows::core::Interface::as_raw(self), uloutputstreamindex, &mut result__).from_abi(result__)
    }
}
::windows::imp::interface_hierarchy!(IDMOVideoOutputOptimizations, ::windows::core::IUnknown);
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
}
impl ::core::clone::Clone for IDMOVideoOutputOptimizations {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IDMOVideoOutputOptimizations {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xbe8f4f4e_5b16_4d29_b350_7f6b5d9298ac);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDMOVideoOutputOptimizations_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
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
        (::windows::core::Interface::vtable(self).Next)(::windows::core::Interface::as_raw(self), citemstofetch, pclsid, names, pcitemsfetched).ok()
    }
    pub unsafe fn Skip(&self, citemstoskip: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Skip)(::windows::core::Interface::as_raw(self), citemstoskip).ok()
    }
    pub unsafe fn Reset(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Reset)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Clone(&self) -> ::windows::core::Result<IEnumDMO> {
        let mut result__ = ::windows::core::zeroed::<IEnumDMO>();
        (::windows::core::Interface::vtable(self).Clone)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
::windows::imp::interface_hierarchy!(IEnumDMO, ::windows::core::IUnknown);
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
}
impl ::core::clone::Clone for IEnumDMO {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IEnumDMO {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2c3cd98a_2bfa_4a53_9c27_5249ba64ba0f);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEnumDMO_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
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
        let mut result__ = ::windows::core::zeroed::<u32>();
        (::windows::core::Interface::vtable(self).GetMaxLength)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetBufferAndLength(&self, ppbuffer: ::core::option::Option<*mut *mut u8>, pcblength: ::core::option::Option<*mut u32>) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetBufferAndLength)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(ppbuffer.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(pcblength.unwrap_or(::std::ptr::null_mut()))).ok()
    }
}
::windows::imp::interface_hierarchy!(IMediaBuffer, ::windows::core::IUnknown);
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
}
impl ::core::clone::Clone for IMediaBuffer {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IMediaBuffer {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x59eff8b9_938c_4a26_82f2_95cb84cdc837);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaBuffer_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub SetLength: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cblength: u32) -> ::windows::core::HRESULT,
    pub GetMaxLength: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcbmaxlength: *mut u32) -> ::windows::core::HRESULT,
    pub GetBufferAndLength: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppbuffer: *mut *mut u8, pcblength: *mut u32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Media_DxMediaObjects\"`*"]
#[repr(transparent)]
pub struct IMediaObject(::windows::core::IUnknown);
impl IMediaObject {
    pub unsafe fn GetStreamCount(&self, pcinputstreams: *mut u32, pcoutputstreams: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetStreamCount)(::windows::core::Interface::as_raw(self), pcinputstreams, pcoutputstreams).ok()
    }
    pub unsafe fn GetInputStreamInfo(&self, dwinputstreamindex: u32) -> ::windows::core::Result<u32> {
        let mut result__ = ::windows::core::zeroed::<u32>();
        (::windows::core::Interface::vtable(self).GetInputStreamInfo)(::windows::core::Interface::as_raw(self), dwinputstreamindex, &mut result__).from_abi(result__)
    }
    pub unsafe fn GetOutputStreamInfo(&self, dwoutputstreamindex: u32) -> ::windows::core::Result<u32> {
        let mut result__ = ::windows::core::zeroed::<u32>();
        (::windows::core::Interface::vtable(self).GetOutputStreamInfo)(::windows::core::Interface::as_raw(self), dwoutputstreamindex, &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetInputType(&self, dwinputstreamindex: u32, dwtypeindex: u32, pmt: ::core::option::Option<*mut DMO_MEDIA_TYPE>) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetInputType)(::windows::core::Interface::as_raw(self), dwinputstreamindex, dwtypeindex, ::core::mem::transmute(pmt.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetOutputType(&self, dwoutputstreamindex: u32, dwtypeindex: u32, pmt: ::core::option::Option<*mut DMO_MEDIA_TYPE>) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetOutputType)(::windows::core::Interface::as_raw(self), dwoutputstreamindex, dwtypeindex, ::core::mem::transmute(pmt.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetInputType(&self, dwinputstreamindex: u32, pmt: ::core::option::Option<*const DMO_MEDIA_TYPE>, dwflags: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetInputType)(::windows::core::Interface::as_raw(self), dwinputstreamindex, ::core::mem::transmute(pmt.unwrap_or(::std::ptr::null())), dwflags).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetOutputType(&self, dwoutputstreamindex: u32, pmt: ::core::option::Option<*const DMO_MEDIA_TYPE>, dwflags: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetOutputType)(::windows::core::Interface::as_raw(self), dwoutputstreamindex, ::core::mem::transmute(pmt.unwrap_or(::std::ptr::null())), dwflags).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetInputCurrentType(&self, dwinputstreamindex: u32, pmt: *mut DMO_MEDIA_TYPE) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetInputCurrentType)(::windows::core::Interface::as_raw(self), dwinputstreamindex, pmt).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetOutputCurrentType(&self, dwoutputstreamindex: u32, pmt: *mut DMO_MEDIA_TYPE) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetOutputCurrentType)(::windows::core::Interface::as_raw(self), dwoutputstreamindex, pmt).ok()
    }
    pub unsafe fn GetInputSizeInfo(&self, dwinputstreamindex: u32, pcbsize: *mut u32, pcbmaxlookahead: *mut u32, pcbalignment: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetInputSizeInfo)(::windows::core::Interface::as_raw(self), dwinputstreamindex, pcbsize, pcbmaxlookahead, pcbalignment).ok()
    }
    pub unsafe fn GetOutputSizeInfo(&self, dwoutputstreamindex: u32, pcbsize: *mut u32, pcbalignment: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetOutputSizeInfo)(::windows::core::Interface::as_raw(self), dwoutputstreamindex, pcbsize, pcbalignment).ok()
    }
    pub unsafe fn GetInputMaxLatency(&self, dwinputstreamindex: u32) -> ::windows::core::Result<i64> {
        let mut result__ = ::windows::core::zeroed::<i64>();
        (::windows::core::Interface::vtable(self).GetInputMaxLatency)(::windows::core::Interface::as_raw(self), dwinputstreamindex, &mut result__).from_abi(result__)
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
        let mut result__ = ::windows::core::zeroed::<u32>();
        (::windows::core::Interface::vtable(self).GetInputStatus)(::windows::core::Interface::as_raw(self), dwinputstreamindex, &mut result__).from_abi(result__)
    }
    pub unsafe fn ProcessInput<P0>(&self, dwinputstreamindex: u32, pbuffer: P0, dwflags: u32, rttimestamp: i64, rttimelength: i64) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<IMediaBuffer>,
    {
        (::windows::core::Interface::vtable(self).ProcessInput)(::windows::core::Interface::as_raw(self), dwinputstreamindex, pbuffer.into_param().abi(), dwflags, rttimestamp, rttimelength).ok()
    }
    pub unsafe fn ProcessOutput(&self, dwflags: u32, poutputbuffers: &mut [DMO_OUTPUT_DATA_BUFFER], pdwstatus: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).ProcessOutput)(::windows::core::Interface::as_raw(self), dwflags, poutputbuffers.len() as _, ::core::mem::transmute(poutputbuffers.as_ptr()), pdwstatus).ok()
    }
    pub unsafe fn Lock(&self, block: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Lock)(::windows::core::Interface::as_raw(self), block).ok()
    }
}
::windows::imp::interface_hierarchy!(IMediaObject, ::windows::core::IUnknown);
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
}
impl ::core::clone::Clone for IMediaObject {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IMediaObject {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd8ad0f58_5494_4102_97c5_ec798e59bcf4);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaObject_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
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
    pub unsafe fn Process(&self, pdata: &mut [u8], reftimestart: i64, dwflags: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Process)(::windows::core::Interface::as_raw(self), pdata.len() as _, ::core::mem::transmute(pdata.as_ptr()), reftimestart, dwflags).ok()
    }
    pub unsafe fn Clone(&self) -> ::windows::core::Result<IMediaObjectInPlace> {
        let mut result__ = ::windows::core::zeroed::<IMediaObjectInPlace>();
        (::windows::core::Interface::vtable(self).Clone)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetLatency(&self) -> ::windows::core::Result<i64> {
        let mut result__ = ::windows::core::zeroed::<i64>();
        (::windows::core::Interface::vtable(self).GetLatency)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
::windows::imp::interface_hierarchy!(IMediaObjectInPlace, ::windows::core::IUnknown);
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
}
impl ::core::clone::Clone for IMediaObjectInPlace {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IMediaObjectInPlace {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x651b9ad0_0fc7_4aa9_9538_d89931010741);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaObjectInPlace_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub Process: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ulsize: u32, pdata: *mut u8, reftimestart: i64, dwflags: u32) -> ::windows::core::HRESULT,
    pub Clone: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppmediaobject: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetLatency: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, platencytime: *mut i64) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Media_DxMediaObjects\"`*"]
pub const DMOCATEGORY_ACOUSTIC_ECHO_CANCEL: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xbf963d80_c559_11d0_8a2b_00a0c9255ac1);
#[doc = "*Required features: `\"Win32_Media_DxMediaObjects\"`*"]
pub const DMOCATEGORY_AGC: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe88c9ba0_c557_11d0_8a2b_00a0c9255ac1);
#[doc = "*Required features: `\"Win32_Media_DxMediaObjects\"`*"]
pub const DMOCATEGORY_AUDIO_CAPTURE_EFFECT: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf665aaba_3e09_4920_aa5f_219811148f09);
#[doc = "*Required features: `\"Win32_Media_DxMediaObjects\"`*"]
pub const DMOCATEGORY_AUDIO_DECODER: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x57f2db8b_e6bb_4513_9d43_dcd2a6593125);
#[doc = "*Required features: `\"Win32_Media_DxMediaObjects\"`*"]
pub const DMOCATEGORY_AUDIO_EFFECT: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf3602b3f_0592_48df_a4cd_674721e7ebeb);
#[doc = "*Required features: `\"Win32_Media_DxMediaObjects\"`*"]
pub const DMOCATEGORY_AUDIO_ENCODER: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x33d9a761_90c8_11d0_bd43_00a0c911ce86);
#[doc = "*Required features: `\"Win32_Media_DxMediaObjects\"`*"]
pub const DMOCATEGORY_AUDIO_NOISE_SUPPRESS: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe07f903f_62fd_4e60_8cdd_dea7236665b5);
#[doc = "*Required features: `\"Win32_Media_DxMediaObjects\"`*"]
pub const DMOCATEGORY_VIDEO_DECODER: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4a69b442_28be_4991_969c_b500adf5d8a8);
#[doc = "*Required features: `\"Win32_Media_DxMediaObjects\"`*"]
pub const DMOCATEGORY_VIDEO_EFFECT: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd990ee14_776c_4723_be46_3da2f56f10b9);
#[doc = "*Required features: `\"Win32_Media_DxMediaObjects\"`*"]
pub const DMOCATEGORY_VIDEO_ENCODER: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x33d9a760_90c8_11d0_bd43_00a0c911ce86);
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
impl ::windows::core::TypeKind for DMO_ENUM_FLAGS {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for DMO_ENUM_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DMO_ENUM_FLAGS").field(&self.0).finish()
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
impl ::windows::core::TypeKind for DMO_REGISTER_FLAGS {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for DMO_REGISTER_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DMO_REGISTER_FLAGS").field(&self.0).finish()
    }
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
impl ::windows::core::TypeKind for _DMO_INPLACE_PROCESS_FLAGS {
    type TypeKind = ::windows::core::CopyType;
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
impl ::windows::core::TypeKind for _DMO_INPUT_DATA_BUFFER_FLAGS {
    type TypeKind = ::windows::core::CopyType;
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
impl ::windows::core::TypeKind for _DMO_INPUT_STATUS_FLAGS {
    type TypeKind = ::windows::core::CopyType;
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
impl ::windows::core::TypeKind for _DMO_INPUT_STREAM_INFO_FLAGS {
    type TypeKind = ::windows::core::CopyType;
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
impl ::windows::core::TypeKind for _DMO_OUTPUT_DATA_BUFFER_FLAGS {
    type TypeKind = ::windows::core::CopyType;
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
impl ::windows::core::TypeKind for _DMO_OUTPUT_STREAM_INFO_FLAGS {
    type TypeKind = ::windows::core::CopyType;
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
impl ::windows::core::TypeKind for _DMO_PROCESS_OUTPUT_FLAGS {
    type TypeKind = ::windows::core::CopyType;
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
impl ::windows::core::TypeKind for _DMO_QUALITY_STATUS_FLAGS {
    type TypeKind = ::windows::core::CopyType;
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
impl ::windows::core::TypeKind for _DMO_SET_TYPE_FLAGS {
    type TypeKind = ::windows::core::CopyType;
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
impl ::windows::core::TypeKind for _DMO_VIDEO_OUTPUT_STREAM_FLAGS {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for _DMO_VIDEO_OUTPUT_STREAM_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("_DMO_VIDEO_OUTPUT_STREAM_FLAGS").field(&self.0).finish()
    }
}
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
    pub pUnk: ::std::mem::ManuallyDrop<::core::option::Option<::windows::core::IUnknown>>,
    pub cbFormat: u32,
    pub pbFormat: *mut u8,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DMO_MEDIA_TYPE {
    fn clone(&self) -> Self {
        unsafe { ::core::mem::transmute_copy(self) }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for DMO_MEDIA_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DMO_MEDIA_TYPE").field("majortype", &self.majortype).field("subtype", &self.subtype).field("bFixedSizeSamples", &self.bFixedSizeSamples).field("bTemporalCompression", &self.bTemporalCompression).field("lSampleSize", &self.lSampleSize).field("formattype", &self.formattype).field("pUnk", &self.pUnk).field("cbFormat", &self.cbFormat).field("pbFormat", &self.pbFormat).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for DMO_MEDIA_TYPE {
    type TypeKind = ::windows::core::CopyType;
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
    pub pBuffer: ::std::mem::ManuallyDrop<::core::option::Option<IMediaBuffer>>,
    pub dwStatus: u32,
    pub rtTimestamp: i64,
    pub rtTimelength: i64,
}
impl ::core::clone::Clone for DMO_OUTPUT_DATA_BUFFER {
    fn clone(&self) -> Self {
        unsafe { ::core::mem::transmute_copy(self) }
    }
}
impl ::core::fmt::Debug for DMO_OUTPUT_DATA_BUFFER {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DMO_OUTPUT_DATA_BUFFER").field("pBuffer", &self.pBuffer).field("dwStatus", &self.dwStatus).field("rtTimestamp", &self.rtTimestamp).field("rtTimelength", &self.rtTimelength).finish()
    }
}
impl ::windows::core::TypeKind for DMO_OUTPUT_DATA_BUFFER {
    type TypeKind = ::windows::core::CopyType;
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
impl ::windows::core::TypeKind for DMO_PARTIAL_MEDIATYPE {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for DMO_PARTIAL_MEDIATYPE {
    fn eq(&self, other: &Self) -> bool {
        self.r#type == other.r#type && self.subtype == other.subtype
    }
}
impl ::core::cmp::Eq for DMO_PARTIAL_MEDIATYPE {}
impl ::core::default::Default for DMO_PARTIAL_MEDIATYPE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
