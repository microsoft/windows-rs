#![allow(unused_variables, non_upper_case_globals, non_snake_case, unused_unsafe, non_camel_case_types, dead_code, clippy::all)]
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
pub struct FindSimilarFileIndexResults {
    pub m_FileIndex: u32,
    pub m_MatchCount: u32,
}
impl FindSimilarFileIndexResults {}
impl ::core::default::Default for FindSimilarFileIndexResults {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for FindSimilarFileIndexResults {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("FindSimilarFileIndexResults").field("m_FileIndex", &self.m_FileIndex).field("m_MatchCount", &self.m_MatchCount).finish()
    }
}
impl ::core::cmp::PartialEq for FindSimilarFileIndexResults {
    fn eq(&self, other: &Self) -> bool {
        self.m_FileIndex == other.m_FileIndex && self.m_MatchCount == other.m_MatchCount
    }
}
impl ::core::cmp::Eq for FindSimilarFileIndexResults {}
unsafe impl ::windows::core::Abi for FindSimilarFileIndexResults {
    type Abi = Self;
}
pub const FindSimilarResults: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x96236a93_9dbc_11da_9e3f_0011114ae311);
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct GeneratorParametersType(pub i32);
pub const RDCGENTYPE_Unused: GeneratorParametersType = GeneratorParametersType(0i32);
pub const RDCGENTYPE_FilterMax: GeneratorParametersType = GeneratorParametersType(1i32);
impl ::core::convert::From<i32> for GeneratorParametersType {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for GeneratorParametersType {
    type Abi = Self;
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IFindSimilarResults(pub ::windows::core::IUnknown);
impl IFindSimilarResults {
    pub unsafe fn GetSize(&self) -> ::windows::core::Result<u32> {
        let mut result__: <u32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    pub unsafe fn GetNextFileId(&self, numtraitsmatched: *mut u32, similarityfileid: *mut SimilarityFileId) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(numtraitsmatched), ::core::mem::transmute(similarityfileid)).ok()
    }
}
unsafe impl ::windows::core::Interface for IFindSimilarResults {
    type Vtable = IFindSimilarResults_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x96236a81_9dbc_11da_9e3f_0011114ae311);
}
impl ::core::convert::From<IFindSimilarResults> for ::windows::core::IUnknown {
    fn from(value: IFindSimilarResults) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IFindSimilarResults> for ::windows::core::IUnknown {
    fn from(value: &IFindSimilarResults) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IFindSimilarResults {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IFindSimilarResults {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IFindSimilarResults_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, size: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, numtraitsmatched: *mut u32, similarityfileid: *mut SimilarityFileId) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IRdcComparator(pub ::windows::core::IUnknown);
impl IRdcComparator {
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Process<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(&self, endofinput: Param0, endofoutput: *mut super::super::Foundation::BOOL, inputbuffer: *mut RdcBufferPointer, outputbuffer: *mut RdcNeedPointer, rdc_errorcode: *mut RDC_ErrorCode) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), endofinput.into_param().abi(), ::core::mem::transmute(endofoutput), ::core::mem::transmute(inputbuffer), ::core::mem::transmute(outputbuffer), ::core::mem::transmute(rdc_errorcode)).ok()
    }
}
unsafe impl ::windows::core::Interface for IRdcComparator {
    type Vtable = IRdcComparator_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x96236a77_9dbc_11da_9e3f_0011114ae311);
}
impl ::core::convert::From<IRdcComparator> for ::windows::core::IUnknown {
    fn from(value: IRdcComparator) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IRdcComparator> for ::windows::core::IUnknown {
    fn from(value: &IRdcComparator) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IRdcComparator {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IRdcComparator {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IRdcComparator_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, endofinput: super::super::Foundation::BOOL, endofoutput: *mut super::super::Foundation::BOOL, inputbuffer: *mut RdcBufferPointer, outputbuffer: *mut RdcNeedPointer, rdc_errorcode: *mut RDC_ErrorCode) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IRdcFileReader(pub ::windows::core::IUnknown);
impl IRdcFileReader {
    pub unsafe fn GetFileSize(&self) -> ::windows::core::Result<u64> {
        let mut result__: <u64 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), &mut result__).from_abi::<u64>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Read(&self, offsetfilestart: u64, bytestoread: u32, bytesactuallyread: *mut u32, buffer: *mut u8, eof: *mut super::super::Foundation::BOOL) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(offsetfilestart), ::core::mem::transmute(bytestoread), ::core::mem::transmute(bytesactuallyread), ::core::mem::transmute(buffer), ::core::mem::transmute(eof)).ok()
    }
    pub unsafe fn GetFilePosition(&self) -> ::windows::core::Result<u64> {
        let mut result__: <u64 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), &mut result__).from_abi::<u64>(result__)
    }
}
unsafe impl ::windows::core::Interface for IRdcFileReader {
    type Vtable = IRdcFileReader_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x96236a74_9dbc_11da_9e3f_0011114ae311);
}
impl ::core::convert::From<IRdcFileReader> for ::windows::core::IUnknown {
    fn from(value: IRdcFileReader) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IRdcFileReader> for ::windows::core::IUnknown {
    fn from(value: &IRdcFileReader) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IRdcFileReader {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IRdcFileReader {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IRdcFileReader_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, filesize: *mut u64) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, offsetfilestart: u64, bytestoread: u32, bytesactuallyread: *mut u32, buffer: *mut u8, eof: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, offsetfromstart: *mut u64) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IRdcFileWriter(pub ::windows::core::IUnknown);
impl IRdcFileWriter {
    pub unsafe fn GetFileSize(&self) -> ::windows::core::Result<u64> {
        let mut result__: <u64 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), &mut result__).from_abi::<u64>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Read(&self, offsetfilestart: u64, bytestoread: u32, bytesactuallyread: *mut u32, buffer: *mut u8, eof: *mut super::super::Foundation::BOOL) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(offsetfilestart), ::core::mem::transmute(bytestoread), ::core::mem::transmute(bytesactuallyread), ::core::mem::transmute(buffer), ::core::mem::transmute(eof)).ok()
    }
    pub unsafe fn GetFilePosition(&self) -> ::windows::core::Result<u64> {
        let mut result__: <u64 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), &mut result__).from_abi::<u64>(result__)
    }
    pub unsafe fn Write(&self, offsetfilestart: u64, bytestowrite: u32) -> ::windows::core::Result<u8> {
        let mut result__: <u8 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(offsetfilestart), ::core::mem::transmute(bytestowrite), &mut result__).from_abi::<u8>(result__)
    }
    pub unsafe fn Truncate(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn DeleteOnClose(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self)).ok()
    }
}
unsafe impl ::windows::core::Interface for IRdcFileWriter {
    type Vtable = IRdcFileWriter_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x96236a75_9dbc_11da_9e3f_0011114ae311);
}
impl ::core::convert::From<IRdcFileWriter> for ::windows::core::IUnknown {
    fn from(value: IRdcFileWriter) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IRdcFileWriter> for ::windows::core::IUnknown {
    fn from(value: &IRdcFileWriter) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IRdcFileWriter {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IRdcFileWriter {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::From<IRdcFileWriter> for IRdcFileReader {
    fn from(value: IRdcFileWriter) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IRdcFileWriter> for IRdcFileReader {
    fn from(value: &IRdcFileWriter) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IRdcFileReader> for IRdcFileWriter {
    fn into_param(self) -> ::windows::core::Param<'a, IRdcFileReader> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IRdcFileReader> for &IRdcFileWriter {
    fn into_param(self) -> ::windows::core::Param<'a, IRdcFileReader> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IRdcFileWriter_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, filesize: *mut u64) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, offsetfilestart: u64, bytestoread: u32, bytesactuallyread: *mut u32, buffer: *mut u8, eof: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, offsetfromstart: *mut u64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, offsetfilestart: u64, bytestowrite: u32, buffer: *mut u8) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IRdcGenerator(pub ::windows::core::IUnknown);
impl IRdcGenerator {
    pub unsafe fn GetGeneratorParameters(&self, level: u32) -> ::windows::core::Result<IRdcGeneratorParameters> {
        let mut result__: <IRdcGeneratorParameters as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(level), &mut result__).from_abi::<IRdcGeneratorParameters>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Process<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(&self, endofinput: Param0, endofoutput: *mut super::super::Foundation::BOOL, inputbuffer: *mut RdcBufferPointer, depth: u32, outputbuffers: *mut *mut RdcBufferPointer, rdc_errorcode: *mut RDC_ErrorCode) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), endofinput.into_param().abi(), ::core::mem::transmute(endofoutput), ::core::mem::transmute(inputbuffer), ::core::mem::transmute(depth), ::core::mem::transmute(outputbuffers), ::core::mem::transmute(rdc_errorcode)).ok()
    }
}
unsafe impl ::windows::core::Interface for IRdcGenerator {
    type Vtable = IRdcGenerator_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x96236a73_9dbc_11da_9e3f_0011114ae311);
}
impl ::core::convert::From<IRdcGenerator> for ::windows::core::IUnknown {
    fn from(value: IRdcGenerator) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IRdcGenerator> for ::windows::core::IUnknown {
    fn from(value: &IRdcGenerator) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IRdcGenerator {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IRdcGenerator {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IRdcGenerator_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, level: u32, igeneratorparameters: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, endofinput: super::super::Foundation::BOOL, endofoutput: *mut super::super::Foundation::BOOL, inputbuffer: *mut RdcBufferPointer, depth: u32, outputbuffers: *mut *mut RdcBufferPointer, rdc_errorcode: *mut RDC_ErrorCode) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IRdcGeneratorFilterMaxParameters(pub ::windows::core::IUnknown);
impl IRdcGeneratorFilterMaxParameters {
    pub unsafe fn GetHorizonSize(&self) -> ::windows::core::Result<u32> {
        let mut result__: <u32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    pub unsafe fn SetHorizonSize(&self, horizonsize: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(horizonsize)).ok()
    }
    pub unsafe fn GetHashWindowSize(&self) -> ::windows::core::Result<u32> {
        let mut result__: <u32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    pub unsafe fn SetHashWindowSize(&self, hashwindowsize: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(hashwindowsize)).ok()
    }
}
unsafe impl ::windows::core::Interface for IRdcGeneratorFilterMaxParameters {
    type Vtable = IRdcGeneratorFilterMaxParameters_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x96236a72_9dbc_11da_9e3f_0011114ae311);
}
impl ::core::convert::From<IRdcGeneratorFilterMaxParameters> for ::windows::core::IUnknown {
    fn from(value: IRdcGeneratorFilterMaxParameters) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IRdcGeneratorFilterMaxParameters> for ::windows::core::IUnknown {
    fn from(value: &IRdcGeneratorFilterMaxParameters) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IRdcGeneratorFilterMaxParameters {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IRdcGeneratorFilterMaxParameters {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IRdcGeneratorFilterMaxParameters_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, horizonsize: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, horizonsize: u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, hashwindowsize: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, hashwindowsize: u32) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IRdcGeneratorParameters(pub ::windows::core::IUnknown);
impl IRdcGeneratorParameters {
    pub unsafe fn GetGeneratorParametersType(&self) -> ::windows::core::Result<GeneratorParametersType> {
        let mut result__: <GeneratorParametersType as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), &mut result__).from_abi::<GeneratorParametersType>(result__)
    }
    pub unsafe fn GetParametersVersion(&self, currentversion: *mut u32, minimumcompatibleappversion: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(currentversion), ::core::mem::transmute(minimumcompatibleappversion)).ok()
    }
    pub unsafe fn GetSerializeSize(&self) -> ::windows::core::Result<u32> {
        let mut result__: <u32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    pub unsafe fn Serialize(&self, size: u32, parametersblob: *mut u8, byteswritten: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(size), ::core::mem::transmute(parametersblob), ::core::mem::transmute(byteswritten)).ok()
    }
}
unsafe impl ::windows::core::Interface for IRdcGeneratorParameters {
    type Vtable = IRdcGeneratorParameters_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x96236a71_9dbc_11da_9e3f_0011114ae311);
}
impl ::core::convert::From<IRdcGeneratorParameters> for ::windows::core::IUnknown {
    fn from(value: IRdcGeneratorParameters) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IRdcGeneratorParameters> for ::windows::core::IUnknown {
    fn from(value: &IRdcGeneratorParameters) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IRdcGeneratorParameters {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IRdcGeneratorParameters {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IRdcGeneratorParameters_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, parameterstype: *mut GeneratorParametersType) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, currentversion: *mut u32, minimumcompatibleappversion: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, size: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, size: u32, parametersblob: *mut u8, byteswritten: *mut u32) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IRdcLibrary(pub ::windows::core::IUnknown);
impl IRdcLibrary {
    pub unsafe fn ComputeDefaultRecursionDepth(&self, filesize: u64) -> ::windows::core::Result<u32> {
        let mut result__: <u32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(filesize), &mut result__).from_abi::<u32>(result__)
    }
    pub unsafe fn CreateGeneratorParameters(&self, parameterstype: GeneratorParametersType, level: u32) -> ::windows::core::Result<IRdcGeneratorParameters> {
        let mut result__: <IRdcGeneratorParameters as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(parameterstype), ::core::mem::transmute(level), &mut result__).from_abi::<IRdcGeneratorParameters>(result__)
    }
    pub unsafe fn OpenGeneratorParameters(&self, size: u32, parametersblob: *const u8) -> ::windows::core::Result<IRdcGeneratorParameters> {
        let mut result__: <IRdcGeneratorParameters as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(size), ::core::mem::transmute(parametersblob), &mut result__).from_abi::<IRdcGeneratorParameters>(result__)
    }
    pub unsafe fn CreateGenerator(&self, depth: u32, igeneratorparametersarray: *const ::core::option::Option<IRdcGeneratorParameters>) -> ::windows::core::Result<IRdcGenerator> {
        let mut result__: <IRdcGenerator as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(depth), ::core::mem::transmute(igeneratorparametersarray), &mut result__).from_abi::<IRdcGenerator>(result__)
    }
    pub unsafe fn CreateComparator<'a, Param0: ::windows::core::IntoParam<'a, IRdcFileReader>>(&self, iseedsignaturesfile: Param0, comparatorbuffersize: u32) -> ::windows::core::Result<IRdcComparator> {
        let mut result__: <IRdcComparator as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), iseedsignaturesfile.into_param().abi(), ::core::mem::transmute(comparatorbuffersize), &mut result__).from_abi::<IRdcComparator>(result__)
    }
    pub unsafe fn CreateSignatureReader<'a, Param0: ::windows::core::IntoParam<'a, IRdcFileReader>>(&self, ifilereader: Param0) -> ::windows::core::Result<IRdcSignatureReader> {
        let mut result__: <IRdcSignatureReader as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), ifilereader.into_param().abi(), &mut result__).from_abi::<IRdcSignatureReader>(result__)
    }
    pub unsafe fn GetRDCVersion(&self, currentversion: *mut u32, minimumcompatibleappversion: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), ::core::mem::transmute(currentversion), ::core::mem::transmute(minimumcompatibleappversion)).ok()
    }
}
unsafe impl ::windows::core::Interface for IRdcLibrary {
    type Vtable = IRdcLibrary_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x96236a78_9dbc_11da_9e3f_0011114ae311);
}
impl ::core::convert::From<IRdcLibrary> for ::windows::core::IUnknown {
    fn from(value: IRdcLibrary) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IRdcLibrary> for ::windows::core::IUnknown {
    fn from(value: &IRdcLibrary) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IRdcLibrary {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IRdcLibrary {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IRdcLibrary_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, filesize: u64, depth: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, parameterstype: GeneratorParametersType, level: u32, igeneratorparameters: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, size: u32, parametersblob: *const u8, igeneratorparameters: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, depth: u32, igeneratorparametersarray: *const ::windows::core::RawPtr, igenerator: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iseedsignaturesfile: ::windows::core::RawPtr, comparatorbuffersize: u32, icomparator: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, ifilereader: ::windows::core::RawPtr, isignaturereader: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, currentversion: *mut u32, minimumcompatibleappversion: *mut u32) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IRdcSignatureReader(pub ::windows::core::IUnknown);
impl IRdcSignatureReader {
    pub unsafe fn ReadHeader(&self) -> ::windows::core::Result<RDC_ErrorCode> {
        let mut result__: <RDC_ErrorCode as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), &mut result__).from_abi::<RDC_ErrorCode>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ReadSignatures(&self, rdcsignaturepointer: *mut RdcSignaturePointer, endofoutput: *mut super::super::Foundation::BOOL) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(rdcsignaturepointer), ::core::mem::transmute(endofoutput)).ok()
    }
}
unsafe impl ::windows::core::Interface for IRdcSignatureReader {
    type Vtable = IRdcSignatureReader_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x96236a76_9dbc_11da_9e3f_0011114ae311);
}
impl ::core::convert::From<IRdcSignatureReader> for ::windows::core::IUnknown {
    fn from(value: IRdcSignatureReader) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IRdcSignatureReader> for ::windows::core::IUnknown {
    fn from(value: &IRdcSignatureReader) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IRdcSignatureReader {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IRdcSignatureReader {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IRdcSignatureReader_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, rdc_errorcode: *mut RDC_ErrorCode) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, rdcsignaturepointer: *mut RdcSignaturePointer, endofoutput: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IRdcSimilarityGenerator(pub ::windows::core::IUnknown);
impl IRdcSimilarityGenerator {
    pub unsafe fn EnableSimilarity(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn Results(&self) -> ::windows::core::Result<SimilarityData> {
        let mut result__: <SimilarityData as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), &mut result__).from_abi::<SimilarityData>(result__)
    }
}
unsafe impl ::windows::core::Interface for IRdcSimilarityGenerator {
    type Vtable = IRdcSimilarityGenerator_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x96236a80_9dbc_11da_9e3f_0011114ae311);
}
impl ::core::convert::From<IRdcSimilarityGenerator> for ::windows::core::IUnknown {
    fn from(value: IRdcSimilarityGenerator) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IRdcSimilarityGenerator> for ::windows::core::IUnknown {
    fn from(value: &IRdcSimilarityGenerator) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IRdcSimilarityGenerator {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IRdcSimilarityGenerator {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IRdcSimilarityGenerator_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, similaritydata: *mut SimilarityData) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ISimilarity(pub ::windows::core::IUnknown);
impl ISimilarity {
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CreateTable<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(&self, path: Param0, truncate: Param1, securitydescriptor: *const u8, recordsize: u32) -> ::windows::core::Result<RdcCreatedTables> {
        let mut result__: <RdcCreatedTables as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), path.into_param().abi(), truncate.into_param().abi(), ::core::mem::transmute(securitydescriptor), ::core::mem::transmute(recordsize), &mut result__).from_abi::<RdcCreatedTables>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CreateTableIndirect<'a, Param0: ::windows::core::IntoParam<'a, ISimilarityTraitsMapping>, Param1: ::windows::core::IntoParam<'a, IRdcFileWriter>, Param2: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(&self, mapping: Param0, fileidfile: Param1, truncate: Param2, recordsize: u32) -> ::windows::core::Result<RdcCreatedTables> {
        let mut result__: <RdcCreatedTables as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), mapping.into_param().abi(), fileidfile.into_param().abi(), truncate.into_param().abi(), ::core::mem::transmute(recordsize), &mut result__).from_abi::<RdcCreatedTables>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CloseTable<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(&self, isvalid: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), isvalid.into_param().abi()).ok()
    }
    pub unsafe fn Append(&self, similarityfileid: *const SimilarityFileId, similaritydata: *const SimilarityData) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(similarityfileid), ::core::mem::transmute(similaritydata)).ok()
    }
    pub unsafe fn FindSimilarFileId(&self, similaritydata: *const SimilarityData, numberofmatchesrequired: u16, resultssize: u32) -> ::windows::core::Result<IFindSimilarResults> {
        let mut result__: <IFindSimilarResults as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(similaritydata), ::core::mem::transmute(numberofmatchesrequired), ::core::mem::transmute(resultssize), &mut result__).from_abi::<IFindSimilarResults>(result__)
    }
    pub unsafe fn CopyAndSwap<'a, Param0: ::windows::core::IntoParam<'a, ISimilarity>, Param1: ::windows::core::IntoParam<'a, ISimilarityReportProgress>>(&self, newsimilaritytables: Param0, reportprogress: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), newsimilaritytables.into_param().abi(), reportprogress.into_param().abi()).ok()
    }
    pub unsafe fn GetRecordCount(&self) -> ::windows::core::Result<u32> {
        let mut result__: <u32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
}
unsafe impl ::windows::core::Interface for ISimilarity {
    type Vtable = ISimilarity_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x96236a83_9dbc_11da_9e3f_0011114ae311);
}
impl ::core::convert::From<ISimilarity> for ::windows::core::IUnknown {
    fn from(value: ISimilarity) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ISimilarity> for ::windows::core::IUnknown {
    fn from(value: &ISimilarity) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ISimilarity {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a ISimilarity {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ISimilarity_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, path: super::super::Foundation::PWSTR, truncate: super::super::Foundation::BOOL, securitydescriptor: *const u8, recordsize: u32, isnew: *mut RdcCreatedTables) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, mapping: ::windows::core::RawPtr, fileidfile: ::windows::core::RawPtr, truncate: super::super::Foundation::BOOL, recordsize: u32, isnew: *mut RdcCreatedTables) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, isvalid: super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, similarityfileid: *const SimilarityFileId, similaritydata: *const SimilarityData) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, similaritydata: *const SimilarityData, numberofmatchesrequired: u16, resultssize: u32, findsimilarresults: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, newsimilaritytables: ::windows::core::RawPtr, reportprogress: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, recordcount: *mut u32) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ISimilarityFileIdTable(pub ::windows::core::IUnknown);
impl ISimilarityFileIdTable {
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CreateTable<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(&self, path: Param0, truncate: Param1, securitydescriptor: *const u8, recordsize: u32) -> ::windows::core::Result<RdcCreatedTables> {
        let mut result__: <RdcCreatedTables as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), path.into_param().abi(), truncate.into_param().abi(), ::core::mem::transmute(securitydescriptor), ::core::mem::transmute(recordsize), &mut result__).from_abi::<RdcCreatedTables>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CreateTableIndirect<'a, Param0: ::windows::core::IntoParam<'a, IRdcFileWriter>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(&self, fileidfile: Param0, truncate: Param1, recordsize: u32) -> ::windows::core::Result<RdcCreatedTables> {
        let mut result__: <RdcCreatedTables as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), fileidfile.into_param().abi(), truncate.into_param().abi(), ::core::mem::transmute(recordsize), &mut result__).from_abi::<RdcCreatedTables>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CloseTable<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(&self, isvalid: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), isvalid.into_param().abi()).ok()
    }
    pub unsafe fn Append(&self, similarityfileid: *const SimilarityFileId) -> ::windows::core::Result<u32> {
        let mut result__: <u32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(similarityfileid), &mut result__).from_abi::<u32>(result__)
    }
    pub unsafe fn Lookup(&self, similarityfileindex: u32) -> ::windows::core::Result<SimilarityFileId> {
        let mut result__: <SimilarityFileId as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(similarityfileindex), &mut result__).from_abi::<SimilarityFileId>(result__)
    }
    pub unsafe fn Invalidate(&self, similarityfileindex: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), ::core::mem::transmute(similarityfileindex)).ok()
    }
    pub unsafe fn GetRecordCount(&self) -> ::windows::core::Result<u32> {
        let mut result__: <u32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
}
unsafe impl ::windows::core::Interface for ISimilarityFileIdTable {
    type Vtable = ISimilarityFileIdTable_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x96236a7f_9dbc_11da_9e3f_0011114ae311);
}
impl ::core::convert::From<ISimilarityFileIdTable> for ::windows::core::IUnknown {
    fn from(value: ISimilarityFileIdTable) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ISimilarityFileIdTable> for ::windows::core::IUnknown {
    fn from(value: &ISimilarityFileIdTable) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ISimilarityFileIdTable {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a ISimilarityFileIdTable {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ISimilarityFileIdTable_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, path: super::super::Foundation::PWSTR, truncate: super::super::Foundation::BOOL, securitydescriptor: *const u8, recordsize: u32, isnew: *mut RdcCreatedTables) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, fileidfile: ::windows::core::RawPtr, truncate: super::super::Foundation::BOOL, recordsize: u32, isnew: *mut RdcCreatedTables) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, isvalid: super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, similarityfileid: *const SimilarityFileId, similarityfileindex: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, similarityfileindex: u32, similarityfileid: *mut SimilarityFileId) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, similarityfileindex: u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, recordcount: *mut u32) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ISimilarityReportProgress(pub ::windows::core::IUnknown);
impl ISimilarityReportProgress {
    pub unsafe fn ReportProgress(&self, percentcompleted: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(percentcompleted)).ok()
    }
}
unsafe impl ::windows::core::Interface for ISimilarityReportProgress {
    type Vtable = ISimilarityReportProgress_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x96236a7a_9dbc_11da_9e3f_0011114ae311);
}
impl ::core::convert::From<ISimilarityReportProgress> for ::windows::core::IUnknown {
    fn from(value: ISimilarityReportProgress) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ISimilarityReportProgress> for ::windows::core::IUnknown {
    fn from(value: &ISimilarityReportProgress) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ISimilarityReportProgress {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a ISimilarityReportProgress {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ISimilarityReportProgress_abi(pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT, pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32, pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32, pub unsafe extern "system" fn(this: ::windows::core::RawPtr, percentcompleted: u32) -> ::windows::core::HRESULT);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ISimilarityTableDumpState(pub ::windows::core::IUnknown);
impl ISimilarityTableDumpState {
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetNextData(&self, resultssize: u32, resultsused: *mut u32, eof: *mut super::super::Foundation::BOOL, results: *mut SimilarityDumpData) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(resultssize), ::core::mem::transmute(resultsused), ::core::mem::transmute(eof), ::core::mem::transmute(results)).ok()
    }
}
unsafe impl ::windows::core::Interface for ISimilarityTableDumpState {
    type Vtable = ISimilarityTableDumpState_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x96236a7b_9dbc_11da_9e3f_0011114ae311);
}
impl ::core::convert::From<ISimilarityTableDumpState> for ::windows::core::IUnknown {
    fn from(value: ISimilarityTableDumpState) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ISimilarityTableDumpState> for ::windows::core::IUnknown {
    fn from(value: &ISimilarityTableDumpState) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ISimilarityTableDumpState {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a ISimilarityTableDumpState {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ISimilarityTableDumpState_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, resultssize: u32, resultsused: *mut u32, eof: *mut super::super::Foundation::BOOL, results: *mut SimilarityDumpData) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ISimilarityTraitsMappedView(pub ::windows::core::IUnknown);
impl ISimilarityTraitsMappedView {
    pub unsafe fn Flush(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn Unmap(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Get<'a, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(&self, index: u64, dirty: Param1, numelements: u32) -> ::windows::core::Result<SimilarityMappedViewInfo> {
        let mut result__: <SimilarityMappedViewInfo as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(index), dirty.into_param().abi(), ::core::mem::transmute(numelements), &mut result__).from_abi::<SimilarityMappedViewInfo>(result__)
    }
    pub unsafe fn GetView(&self, mappedpagebegin: *mut *mut u8, mappedpageend: *mut *mut u8) {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(mappedpagebegin), ::core::mem::transmute(mappedpageend)))
    }
}
unsafe impl ::windows::core::Interface for ISimilarityTraitsMappedView {
    type Vtable = ISimilarityTraitsMappedView_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x96236a7c_9dbc_11da_9e3f_0011114ae311);
}
impl ::core::convert::From<ISimilarityTraitsMappedView> for ::windows::core::IUnknown {
    fn from(value: ISimilarityTraitsMappedView) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ISimilarityTraitsMappedView> for ::windows::core::IUnknown {
    fn from(value: &ISimilarityTraitsMappedView) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ISimilarityTraitsMappedView {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a ISimilarityTraitsMappedView {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ISimilarityTraitsMappedView_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, index: u64, dirty: super::super::Foundation::BOOL, numelements: u32, viewinfo: *mut SimilarityMappedViewInfo) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, mappedpagebegin: *mut *mut u8, mappedpageend: *mut *mut u8),
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ISimilarityTraitsMapping(pub ::windows::core::IUnknown);
impl ISimilarityTraitsMapping {
    pub unsafe fn CloseMapping(&self) {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self)))
    }
    pub unsafe fn SetFileSize(&self, filesize: u64) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(filesize)).ok()
    }
    pub unsafe fn GetFileSize(&self) -> ::windows::core::Result<u64> {
        let mut result__: <u64 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), &mut result__).from_abi::<u64>(result__)
    }
    pub unsafe fn OpenMapping(&self, accessmode: RdcMappingAccessMode, begin: u64, end: u64) -> ::windows::core::Result<u64> {
        let mut result__: <u64 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(accessmode), ::core::mem::transmute(begin), ::core::mem::transmute(end), &mut result__).from_abi::<u64>(result__)
    }
    pub unsafe fn ResizeMapping(&self, accessmode: RdcMappingAccessMode, begin: u64, end: u64) -> ::windows::core::Result<u64> {
        let mut result__: <u64 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(accessmode), ::core::mem::transmute(begin), ::core::mem::transmute(end), &mut result__).from_abi::<u64>(result__)
    }
    pub unsafe fn GetPageSize(&self, pagesize: *mut u32) {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), ::core::mem::transmute(pagesize)))
    }
    pub unsafe fn CreateView(&self, minimummappedpages: u32, accessmode: RdcMappingAccessMode) -> ::windows::core::Result<ISimilarityTraitsMappedView> {
        let mut result__: <ISimilarityTraitsMappedView as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), ::core::mem::transmute(minimummappedpages), ::core::mem::transmute(accessmode), &mut result__).from_abi::<ISimilarityTraitsMappedView>(result__)
    }
}
unsafe impl ::windows::core::Interface for ISimilarityTraitsMapping {
    type Vtable = ISimilarityTraitsMapping_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x96236a7d_9dbc_11da_9e3f_0011114ae311);
}
impl ::core::convert::From<ISimilarityTraitsMapping> for ::windows::core::IUnknown {
    fn from(value: ISimilarityTraitsMapping) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ISimilarityTraitsMapping> for ::windows::core::IUnknown {
    fn from(value: &ISimilarityTraitsMapping) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ISimilarityTraitsMapping {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a ISimilarityTraitsMapping {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ISimilarityTraitsMapping_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr),
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, filesize: u64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, filesize: *mut u64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, accessmode: RdcMappingAccessMode, begin: u64, end: u64, actualend: *mut u64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, accessmode: RdcMappingAccessMode, begin: u64, end: u64, actualend: *mut u64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pagesize: *mut u32),
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, minimummappedpages: u32, accessmode: RdcMappingAccessMode, mappedview: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ISimilarityTraitsTable(pub ::windows::core::IUnknown);
impl ISimilarityTraitsTable {
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CreateTable<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(&self, path: Param0, truncate: Param1, securitydescriptor: *const u8) -> ::windows::core::Result<RdcCreatedTables> {
        let mut result__: <RdcCreatedTables as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), path.into_param().abi(), truncate.into_param().abi(), ::core::mem::transmute(securitydescriptor), &mut result__).from_abi::<RdcCreatedTables>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CreateTableIndirect<'a, Param0: ::windows::core::IntoParam<'a, ISimilarityTraitsMapping>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(&self, mapping: Param0, truncate: Param1) -> ::windows::core::Result<RdcCreatedTables> {
        let mut result__: <RdcCreatedTables as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), mapping.into_param().abi(), truncate.into_param().abi(), &mut result__).from_abi::<RdcCreatedTables>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CloseTable<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(&self, isvalid: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), isvalid.into_param().abi()).ok()
    }
    pub unsafe fn Append(&self, data: *const SimilarityData, fileindex: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(data), ::core::mem::transmute(fileindex)).ok()
    }
    pub unsafe fn FindSimilarFileIndex(&self, similaritydata: *const SimilarityData, numberofmatchesrequired: u16, findsimilarfileindexresults: *mut FindSimilarFileIndexResults, resultssize: u32, resultsused: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(similaritydata), ::core::mem::transmute(numberofmatchesrequired), ::core::mem::transmute(findsimilarfileindexresults), ::core::mem::transmute(resultssize), ::core::mem::transmute(resultsused)).ok()
    }
    pub unsafe fn BeginDump(&self) -> ::windows::core::Result<ISimilarityTableDumpState> {
        let mut result__: <ISimilarityTableDumpState as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), &mut result__).from_abi::<ISimilarityTableDumpState>(result__)
    }
    pub unsafe fn GetLastIndex(&self) -> ::windows::core::Result<u32> {
        let mut result__: <u32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
}
unsafe impl ::windows::core::Interface for ISimilarityTraitsTable {
    type Vtable = ISimilarityTraitsTable_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x96236a7e_9dbc_11da_9e3f_0011114ae311);
}
impl ::core::convert::From<ISimilarityTraitsTable> for ::windows::core::IUnknown {
    fn from(value: ISimilarityTraitsTable) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ISimilarityTraitsTable> for ::windows::core::IUnknown {
    fn from(value: &ISimilarityTraitsTable) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ISimilarityTraitsTable {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a ISimilarityTraitsTable {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ISimilarityTraitsTable_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, path: super::super::Foundation::PWSTR, truncate: super::super::Foundation::BOOL, securitydescriptor: *const u8, isnew: *mut RdcCreatedTables) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, mapping: ::windows::core::RawPtr, truncate: super::super::Foundation::BOOL, isnew: *mut RdcCreatedTables) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, isvalid: super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, data: *const SimilarityData, fileindex: u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, similaritydata: *const SimilarityData, numberofmatchesrequired: u16, findsimilarfileindexresults: *mut FindSimilarFileIndexResults, resultssize: u32, resultsused: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, similaritytabledumpstate: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, fileindex: *mut u32) -> ::windows::core::HRESULT,
);
pub const MSRDC_DEFAULT_COMPAREBUFFER: u32 = 3200000u32;
pub const MSRDC_DEFAULT_HASHWINDOWSIZE_1: u32 = 48u32;
pub const MSRDC_DEFAULT_HASHWINDOWSIZE_N: u32 = 2u32;
pub const MSRDC_DEFAULT_HORIZONSIZE_1: u32 = 1024u32;
pub const MSRDC_DEFAULT_HORIZONSIZE_N: u32 = 128u32;
pub const MSRDC_MAXIMUM_COMPAREBUFFER: u32 = 1073741824u32;
pub const MSRDC_MAXIMUM_DEPTH: u32 = 8u32;
pub const MSRDC_MAXIMUM_HASHWINDOWSIZE: u32 = 96u32;
pub const MSRDC_MAXIMUM_HORIZONSIZE: u32 = 16384u32;
pub const MSRDC_MAXIMUM_MATCHESREQUIRED: u32 = 16u32;
pub const MSRDC_MAXIMUM_TRAITVALUE: u32 = 63u32;
pub const MSRDC_MINIMUM_COMPAREBUFFER: u32 = 100000u32;
pub const MSRDC_MINIMUM_COMPATIBLE_APP_VERSION: u32 = 65536u32;
pub const MSRDC_MINIMUM_DEPTH: u32 = 1u32;
pub const MSRDC_MINIMUM_HASHWINDOWSIZE: u32 = 2u32;
pub const MSRDC_MINIMUM_HORIZONSIZE: u32 = 128u32;
pub const MSRDC_MINIMUM_INPUTBUFFERSIZE: u32 = 1024u32;
pub const MSRDC_MINIMUM_MATCHESREQUIRED: u32 = 1u32;
pub const MSRDC_SIGNATURE_HASHSIZE: u32 = 16u32;
pub const MSRDC_VERSION: u32 = 65536u32;
pub const RDCE_TABLE_CORRUPT: u32 = 2147745794u32;
pub const RDCE_TABLE_FULL: u32 = 2147745793u32;
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct RDC_ErrorCode(pub i32);
pub const RDC_NoError: RDC_ErrorCode = RDC_ErrorCode(0i32);
pub const RDC_HeaderVersionNewer: RDC_ErrorCode = RDC_ErrorCode(1i32);
pub const RDC_HeaderVersionOlder: RDC_ErrorCode = RDC_ErrorCode(2i32);
pub const RDC_HeaderMissingOrCorrupt: RDC_ErrorCode = RDC_ErrorCode(3i32);
pub const RDC_HeaderWrongType: RDC_ErrorCode = RDC_ErrorCode(4i32);
pub const RDC_DataMissingOrCorrupt: RDC_ErrorCode = RDC_ErrorCode(5i32);
pub const RDC_DataTooManyRecords: RDC_ErrorCode = RDC_ErrorCode(6i32);
pub const RDC_FileChecksumMismatch: RDC_ErrorCode = RDC_ErrorCode(7i32);
pub const RDC_ApplicationError: RDC_ErrorCode = RDC_ErrorCode(8i32);
pub const RDC_Aborted: RDC_ErrorCode = RDC_ErrorCode(9i32);
pub const RDC_Win32Error: RDC_ErrorCode = RDC_ErrorCode(10i32);
impl ::core::convert::From<i32> for RDC_ErrorCode {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for RDC_ErrorCode {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
pub struct RdcBufferPointer {
    pub m_Size: u32,
    pub m_Used: u32,
    pub m_Data: *mut u8,
}
impl RdcBufferPointer {}
impl ::core::default::Default for RdcBufferPointer {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for RdcBufferPointer {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("RdcBufferPointer").field("m_Size", &self.m_Size).field("m_Used", &self.m_Used).field("m_Data", &self.m_Data).finish()
    }
}
impl ::core::cmp::PartialEq for RdcBufferPointer {
    fn eq(&self, other: &Self) -> bool {
        self.m_Size == other.m_Size && self.m_Used == other.m_Used && self.m_Data == other.m_Data
    }
}
impl ::core::cmp::Eq for RdcBufferPointer {}
unsafe impl ::windows::core::Abi for RdcBufferPointer {
    type Abi = Self;
}
pub const RdcComparator: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x96236a8b_9dbc_11da_9e3f_0011114ae311);
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct RdcCreatedTables(pub i32);
pub const RDCTABLE_InvalidOrUnknown: RdcCreatedTables = RdcCreatedTables(0i32);
pub const RDCTABLE_Existing: RdcCreatedTables = RdcCreatedTables(1i32);
pub const RDCTABLE_New: RdcCreatedTables = RdcCreatedTables(2i32);
impl ::core::convert::From<i32> for RdcCreatedTables {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for RdcCreatedTables {
    type Abi = Self;
}
pub const RdcFileReader: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x96236a89_9dbc_11da_9e3f_0011114ae311);
pub const RdcGenerator: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x96236a88_9dbc_11da_9e3f_0011114ae311);
pub const RdcGeneratorFilterMaxParameters: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x96236a87_9dbc_11da_9e3f_0011114ae311);
pub const RdcGeneratorParameters: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x96236a86_9dbc_11da_9e3f_0011114ae311);
pub const RdcLibrary: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x96236a85_9dbc_11da_9e3f_0011114ae311);
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct RdcMappingAccessMode(pub i32);
pub const RDCMAPPING_Undefined: RdcMappingAccessMode = RdcMappingAccessMode(0i32);
pub const RDCMAPPING_ReadOnly: RdcMappingAccessMode = RdcMappingAccessMode(1i32);
pub const RDCMAPPING_ReadWrite: RdcMappingAccessMode = RdcMappingAccessMode(2i32);
impl ::core::convert::From<i32> for RdcMappingAccessMode {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for RdcMappingAccessMode {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
pub struct RdcNeed {
    pub m_BlockType: RdcNeedType,
    pub m_FileOffset: u64,
    pub m_BlockLength: u64,
}
impl RdcNeed {}
impl ::core::default::Default for RdcNeed {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for RdcNeed {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("RdcNeed").field("m_BlockType", &self.m_BlockType).field("m_FileOffset", &self.m_FileOffset).field("m_BlockLength", &self.m_BlockLength).finish()
    }
}
impl ::core::cmp::PartialEq for RdcNeed {
    fn eq(&self, other: &Self) -> bool {
        self.m_BlockType == other.m_BlockType && self.m_FileOffset == other.m_FileOffset && self.m_BlockLength == other.m_BlockLength
    }
}
impl ::core::cmp::Eq for RdcNeed {}
unsafe impl ::windows::core::Abi for RdcNeed {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
pub struct RdcNeedPointer {
    pub m_Size: u32,
    pub m_Used: u32,
    pub m_Data: *mut RdcNeed,
}
impl RdcNeedPointer {}
impl ::core::default::Default for RdcNeedPointer {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for RdcNeedPointer {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("RdcNeedPointer").field("m_Size", &self.m_Size).field("m_Used", &self.m_Used).field("m_Data", &self.m_Data).finish()
    }
}
impl ::core::cmp::PartialEq for RdcNeedPointer {
    fn eq(&self, other: &Self) -> bool {
        self.m_Size == other.m_Size && self.m_Used == other.m_Used && self.m_Data == other.m_Data
    }
}
impl ::core::cmp::Eq for RdcNeedPointer {}
unsafe impl ::windows::core::Abi for RdcNeedPointer {
    type Abi = Self;
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct RdcNeedType(pub i32);
pub const RDCNEED_SOURCE: RdcNeedType = RdcNeedType(0i32);
pub const RDCNEED_TARGET: RdcNeedType = RdcNeedType(1i32);
pub const RDCNEED_SEED: RdcNeedType = RdcNeedType(2i32);
pub const RDCNEED_SEED_MAX: RdcNeedType = RdcNeedType(255i32);
impl ::core::convert::From<i32> for RdcNeedType {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for RdcNeedType {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
pub struct RdcSignature {
    pub m_Signature: [u8; 16],
    pub m_BlockLength: u16,
}
impl RdcSignature {}
impl ::core::default::Default for RdcSignature {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for RdcSignature {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("RdcSignature").field("m_Signature", &self.m_Signature).field("m_BlockLength", &self.m_BlockLength).finish()
    }
}
impl ::core::cmp::PartialEq for RdcSignature {
    fn eq(&self, other: &Self) -> bool {
        self.m_Signature == other.m_Signature && self.m_BlockLength == other.m_BlockLength
    }
}
impl ::core::cmp::Eq for RdcSignature {}
unsafe impl ::windows::core::Abi for RdcSignature {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
pub struct RdcSignaturePointer {
    pub m_Size: u32,
    pub m_Used: u32,
    pub m_Data: *mut RdcSignature,
}
impl RdcSignaturePointer {}
impl ::core::default::Default for RdcSignaturePointer {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for RdcSignaturePointer {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("RdcSignaturePointer").field("m_Size", &self.m_Size).field("m_Used", &self.m_Used).field("m_Data", &self.m_Data).finish()
    }
}
impl ::core::cmp::PartialEq for RdcSignaturePointer {
    fn eq(&self, other: &Self) -> bool {
        self.m_Size == other.m_Size && self.m_Used == other.m_Used && self.m_Data == other.m_Data
    }
}
impl ::core::cmp::Eq for RdcSignaturePointer {}
unsafe impl ::windows::core::Abi for RdcSignaturePointer {
    type Abi = Self;
}
pub const RdcSignatureReader: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x96236a8a_9dbc_11da_9e3f_0011114ae311);
pub const RdcSimilarityGenerator: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x96236a92_9dbc_11da_9e3f_0011114ae311);
pub const Similarity: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x96236a91_9dbc_11da_9e3f_0011114ae311);
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
pub struct SimilarityData {
    pub m_Data: [u8; 16],
}
impl SimilarityData {}
impl ::core::default::Default for SimilarityData {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for SimilarityData {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("SimilarityData").field("m_Data", &self.m_Data).finish()
    }
}
impl ::core::cmp::PartialEq for SimilarityData {
    fn eq(&self, other: &Self) -> bool {
        self.m_Data == other.m_Data
    }
}
impl ::core::cmp::Eq for SimilarityData {}
unsafe impl ::windows::core::Abi for SimilarityData {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
pub struct SimilarityDumpData {
    pub m_FileIndex: u32,
    pub m_Data: SimilarityData,
}
impl SimilarityDumpData {}
impl ::core::default::Default for SimilarityDumpData {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for SimilarityDumpData {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("SimilarityDumpData").field("m_FileIndex", &self.m_FileIndex).field("m_Data", &self.m_Data).finish()
    }
}
impl ::core::cmp::PartialEq for SimilarityDumpData {
    fn eq(&self, other: &Self) -> bool {
        self.m_FileIndex == other.m_FileIndex && self.m_Data == other.m_Data
    }
}
impl ::core::cmp::Eq for SimilarityDumpData {}
unsafe impl ::windows::core::Abi for SimilarityDumpData {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
pub struct SimilarityFileId {
    pub m_FileId: [u8; 32],
}
impl SimilarityFileId {}
impl ::core::default::Default for SimilarityFileId {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for SimilarityFileId {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("SimilarityFileId").field("m_FileId", &self.m_FileId).finish()
    }
}
impl ::core::cmp::PartialEq for SimilarityFileId {
    fn eq(&self, other: &Self) -> bool {
        self.m_FileId == other.m_FileId
    }
}
impl ::core::cmp::Eq for SimilarityFileId {}
unsafe impl ::windows::core::Abi for SimilarityFileId {
    type Abi = Self;
}
pub const SimilarityFileIdMaxSize: u32 = 32u32;
pub const SimilarityFileIdMinSize: u32 = 4u32;
pub const SimilarityFileIdTable: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x96236a90_9dbc_11da_9e3f_0011114ae311);
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
pub struct SimilarityMappedViewInfo {
    pub m_Data: *mut u8,
    pub m_Length: u32,
}
impl SimilarityMappedViewInfo {}
impl ::core::default::Default for SimilarityMappedViewInfo {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for SimilarityMappedViewInfo {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("SimilarityMappedViewInfo").field("m_Data", &self.m_Data).field("m_Length", &self.m_Length).finish()
    }
}
impl ::core::cmp::PartialEq for SimilarityMappedViewInfo {
    fn eq(&self, other: &Self) -> bool {
        self.m_Data == other.m_Data && self.m_Length == other.m_Length
    }
}
impl ::core::cmp::Eq for SimilarityMappedViewInfo {}
unsafe impl ::windows::core::Abi for SimilarityMappedViewInfo {
    type Abi = Self;
}
pub const SimilarityReportProgress: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x96236a8d_9dbc_11da_9e3f_0011114ae311);
pub const SimilarityTableDumpState: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x96236a8e_9dbc_11da_9e3f_0011114ae311);
pub const SimilarityTraitsMappedView: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x96236a95_9dbc_11da_9e3f_0011114ae311);
pub const SimilarityTraitsMapping: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x96236a94_9dbc_11da_9e3f_0011114ae311);
pub const SimilarityTraitsTable: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x96236a8f_9dbc_11da_9e3f_0011114ae311);
