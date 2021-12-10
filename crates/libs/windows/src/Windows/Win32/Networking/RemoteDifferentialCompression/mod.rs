#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
#[repr(C)]
#[doc = "*Required features: 'Win32_Networking_RemoteDifferentialCompression'*"]
pub struct FindSimilarFileIndexResults {
    pub m_FileIndex: u32,
    pub m_MatchCount: u32,
}
impl ::core::marker::Copy for FindSimilarFileIndexResults {}
impl ::core::clone::Clone for FindSimilarFileIndexResults {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for FindSimilarFileIndexResults {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for FindSimilarFileIndexResults {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<FindSimilarFileIndexResults>()) == 0 }
    }
}
impl ::core::cmp::Eq for FindSimilarFileIndexResults {}
impl ::core::default::Default for FindSimilarFileIndexResults {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
pub const FindSimilarResults: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x96236a93_9dbc_11da_9e3f_0011114ae311);
#[doc = "*Required features: 'Win32_Networking_RemoteDifferentialCompression'*"]
pub type GeneratorParametersType = i32;
#[doc = "*Required features: 'Win32_Networking_RemoteDifferentialCompression'*"]
pub const RDCGENTYPE_Unused: GeneratorParametersType = 0i32;
#[doc = "*Required features: 'Win32_Networking_RemoteDifferentialCompression'*"]
pub const RDCGENTYPE_FilterMax: GeneratorParametersType = 1i32;
#[doc = "*Required features: 'Win32_Networking_RemoteDifferentialCompression'*"]
#[repr(transparent)]
pub struct IFindSimilarResults(::windows::core::IUnknown);
impl IFindSimilarResults {
    #[doc = "*Required features: 'Win32_Networking_RemoteDifferentialCompression'*"]
    pub unsafe fn GetSize(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: 'Win32_Networking_RemoteDifferentialCompression'*"]
    pub unsafe fn GetNextFileId(&self, numtraitsmatched: *mut u32, similarityfileid: *mut SimilarityFileId) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(numtraitsmatched), ::core::mem::transmute(similarityfileid)).ok()
    }
}
impl ::core::convert::From<IFindSimilarResults> for ::windows::core::IUnknown {
    fn from(value: IFindSimilarResults) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IFindSimilarResults> for ::windows::core::IUnknown {
    fn from(value: &IFindSimilarResults) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IFindSimilarResults {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IFindSimilarResults {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IFindSimilarResults {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IFindSimilarResults {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IFindSimilarResults {}
unsafe impl ::windows::core::Interface for IFindSimilarResults {
    type Vtable = IFindSimilarResultsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x96236a81_9dbc_11da_9e3f_0011114ae311);
}
#[repr(C)]
#[doc(hidden)]
pub struct IFindSimilarResultsVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, size: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, numtraitsmatched: *mut u32, similarityfileid: *mut SimilarityFileId) -> ::windows::core::HRESULT,
);
#[doc = "*Required features: 'Win32_Networking_RemoteDifferentialCompression'*"]
#[repr(transparent)]
pub struct IRdcComparator(::windows::core::IUnknown);
impl IRdcComparator {
    #[doc = "*Required features: 'Win32_Networking_RemoteDifferentialCompression', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Process<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(&self, endofinput: Param0, endofoutput: *mut super::super::Foundation::BOOL, inputbuffer: *mut RdcBufferPointer, outputbuffer: *mut RdcNeedPointer, rdc_errorcode: *mut RDC_ErrorCode) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), endofinput.into_param().abi(), ::core::mem::transmute(endofoutput), ::core::mem::transmute(inputbuffer), ::core::mem::transmute(outputbuffer), ::core::mem::transmute(rdc_errorcode)).ok()
    }
}
impl ::core::convert::From<IRdcComparator> for ::windows::core::IUnknown {
    fn from(value: IRdcComparator) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IRdcComparator> for ::windows::core::IUnknown {
    fn from(value: &IRdcComparator) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IRdcComparator {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IRdcComparator {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IRdcComparator {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IRdcComparator {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IRdcComparator {}
unsafe impl ::windows::core::Interface for IRdcComparator {
    type Vtable = IRdcComparatorVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x96236a77_9dbc_11da_9e3f_0011114ae311);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRdcComparatorVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, endofinput: super::super::Foundation::BOOL, endofoutput: *mut super::super::Foundation::BOOL, inputbuffer: *mut RdcBufferPointer, outputbuffer: *mut RdcNeedPointer, rdc_errorcode: *mut RDC_ErrorCode) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[doc = "*Required features: 'Win32_Networking_RemoteDifferentialCompression'*"]
#[repr(transparent)]
pub struct IRdcFileReader(::windows::core::IUnknown);
impl IRdcFileReader {
    #[doc = "*Required features: 'Win32_Networking_RemoteDifferentialCompression'*"]
    pub unsafe fn GetFileSize(&self) -> ::windows::core::Result<u64> {
        let mut result__: u64 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u64>(result__)
    }
    #[doc = "*Required features: 'Win32_Networking_RemoteDifferentialCompression', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Read(&self, offsetfilestart: u64, bytestoread: u32, bytesactuallyread: *mut u32, buffer: *mut u8, eof: *mut super::super::Foundation::BOOL) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(offsetfilestart), ::core::mem::transmute(bytestoread), ::core::mem::transmute(bytesactuallyread), ::core::mem::transmute(buffer), ::core::mem::transmute(eof)).ok()
    }
    #[doc = "*Required features: 'Win32_Networking_RemoteDifferentialCompression'*"]
    pub unsafe fn GetFilePosition(&self) -> ::windows::core::Result<u64> {
        let mut result__: u64 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u64>(result__)
    }
}
impl ::core::convert::From<IRdcFileReader> for ::windows::core::IUnknown {
    fn from(value: IRdcFileReader) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IRdcFileReader> for ::windows::core::IUnknown {
    fn from(value: &IRdcFileReader) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IRdcFileReader {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IRdcFileReader {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IRdcFileReader {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IRdcFileReader {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IRdcFileReader {}
unsafe impl ::windows::core::Interface for IRdcFileReader {
    type Vtable = IRdcFileReaderVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x96236a74_9dbc_11da_9e3f_0011114ae311);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRdcFileReaderVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, filesize: *mut u64) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, offsetfilestart: u64, bytestoread: u32, bytesactuallyread: *mut u32, buffer: *mut u8, eof: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, offsetfromstart: *mut u64) -> ::windows::core::HRESULT,
);
#[doc = "*Required features: 'Win32_Networking_RemoteDifferentialCompression'*"]
#[repr(transparent)]
pub struct IRdcFileWriter(::windows::core::IUnknown);
impl IRdcFileWriter {
    #[doc = "*Required features: 'Win32_Networking_RemoteDifferentialCompression'*"]
    pub unsafe fn GetFileSize(&self) -> ::windows::core::Result<u64> {
        let mut result__: u64 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u64>(result__)
    }
    #[doc = "*Required features: 'Win32_Networking_RemoteDifferentialCompression', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Read(&self, offsetfilestart: u64, bytestoread: u32, bytesactuallyread: *mut u32, buffer: *mut u8, eof: *mut super::super::Foundation::BOOL) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(offsetfilestart), ::core::mem::transmute(bytestoread), ::core::mem::transmute(bytesactuallyread), ::core::mem::transmute(buffer), ::core::mem::transmute(eof)).ok()
    }
    #[doc = "*Required features: 'Win32_Networking_RemoteDifferentialCompression'*"]
    pub unsafe fn GetFilePosition(&self) -> ::windows::core::Result<u64> {
        let mut result__: u64 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u64>(result__)
    }
    #[doc = "*Required features: 'Win32_Networking_RemoteDifferentialCompression'*"]
    pub unsafe fn Write(&self, offsetfilestart: u64, bytestowrite: u32) -> ::windows::core::Result<u8> {
        let mut result__: u8 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(offsetfilestart), ::core::mem::transmute(bytestowrite), ::core::mem::transmute(&mut result__)).from_abi::<u8>(result__)
    }
    #[doc = "*Required features: 'Win32_Networking_RemoteDifferentialCompression'*"]
    pub unsafe fn Truncate(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: 'Win32_Networking_RemoteDifferentialCompression'*"]
    pub unsafe fn DeleteOnClose(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self)).ok()
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
impl ::core::convert::From<IRdcFileWriter> for ::windows::core::IUnknown {
    fn from(value: IRdcFileWriter) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IRdcFileWriter> for ::windows::core::IUnknown {
    fn from(value: &IRdcFileWriter) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IRdcFileWriter {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IRdcFileWriter {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IRdcFileWriter {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IRdcFileWriter {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IRdcFileWriter {}
unsafe impl ::windows::core::Interface for IRdcFileWriter {
    type Vtable = IRdcFileWriterVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x96236a75_9dbc_11da_9e3f_0011114ae311);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRdcFileWriterVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, filesize: *mut u64) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, offsetfilestart: u64, bytestoread: u32, bytesactuallyread: *mut u32, buffer: *mut u8, eof: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, offsetfromstart: *mut u64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, offsetfilestart: u64, bytestowrite: u32, buffer: *mut u8) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
);
#[doc = "*Required features: 'Win32_Networking_RemoteDifferentialCompression'*"]
#[repr(transparent)]
pub struct IRdcGenerator(::windows::core::IUnknown);
impl IRdcGenerator {
    #[doc = "*Required features: 'Win32_Networking_RemoteDifferentialCompression'*"]
    pub unsafe fn GetGeneratorParameters(&self, level: u32) -> ::windows::core::Result<IRdcGeneratorParameters> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(level), ::core::mem::transmute(&mut result__)).from_abi::<IRdcGeneratorParameters>(result__)
    }
    #[doc = "*Required features: 'Win32_Networking_RemoteDifferentialCompression', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Process<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(&self, endofinput: Param0, endofoutput: *mut super::super::Foundation::BOOL, inputbuffer: *mut RdcBufferPointer, depth: u32, outputbuffers: *mut *mut RdcBufferPointer, rdc_errorcode: *mut RDC_ErrorCode) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), endofinput.into_param().abi(), ::core::mem::transmute(endofoutput), ::core::mem::transmute(inputbuffer), ::core::mem::transmute(depth), ::core::mem::transmute(outputbuffers), ::core::mem::transmute(rdc_errorcode)).ok()
    }
}
impl ::core::convert::From<IRdcGenerator> for ::windows::core::IUnknown {
    fn from(value: IRdcGenerator) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IRdcGenerator> for ::windows::core::IUnknown {
    fn from(value: &IRdcGenerator) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IRdcGenerator {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IRdcGenerator {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IRdcGenerator {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IRdcGenerator {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IRdcGenerator {}
unsafe impl ::windows::core::Interface for IRdcGenerator {
    type Vtable = IRdcGeneratorVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x96236a73_9dbc_11da_9e3f_0011114ae311);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRdcGeneratorVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, level: u32, igeneratorparameters: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, endofinput: super::super::Foundation::BOOL, endofoutput: *mut super::super::Foundation::BOOL, inputbuffer: *mut RdcBufferPointer, depth: u32, outputbuffers: *mut *mut RdcBufferPointer, rdc_errorcode: *mut RDC_ErrorCode) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[doc = "*Required features: 'Win32_Networking_RemoteDifferentialCompression'*"]
#[repr(transparent)]
pub struct IRdcGeneratorFilterMaxParameters(::windows::core::IUnknown);
impl IRdcGeneratorFilterMaxParameters {
    #[doc = "*Required features: 'Win32_Networking_RemoteDifferentialCompression'*"]
    pub unsafe fn GetHorizonSize(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: 'Win32_Networking_RemoteDifferentialCompression'*"]
    pub unsafe fn SetHorizonSize(&self, horizonsize: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(horizonsize)).ok()
    }
    #[doc = "*Required features: 'Win32_Networking_RemoteDifferentialCompression'*"]
    pub unsafe fn GetHashWindowSize(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: 'Win32_Networking_RemoteDifferentialCompression'*"]
    pub unsafe fn SetHashWindowSize(&self, hashwindowsize: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(hashwindowsize)).ok()
    }
}
impl ::core::convert::From<IRdcGeneratorFilterMaxParameters> for ::windows::core::IUnknown {
    fn from(value: IRdcGeneratorFilterMaxParameters) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IRdcGeneratorFilterMaxParameters> for ::windows::core::IUnknown {
    fn from(value: &IRdcGeneratorFilterMaxParameters) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IRdcGeneratorFilterMaxParameters {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IRdcGeneratorFilterMaxParameters {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IRdcGeneratorFilterMaxParameters {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IRdcGeneratorFilterMaxParameters {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IRdcGeneratorFilterMaxParameters {}
unsafe impl ::windows::core::Interface for IRdcGeneratorFilterMaxParameters {
    type Vtable = IRdcGeneratorFilterMaxParametersVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x96236a72_9dbc_11da_9e3f_0011114ae311);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRdcGeneratorFilterMaxParametersVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, horizonsize: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, horizonsize: u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hashwindowsize: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hashwindowsize: u32) -> ::windows::core::HRESULT,
);
#[doc = "*Required features: 'Win32_Networking_RemoteDifferentialCompression'*"]
#[repr(transparent)]
pub struct IRdcGeneratorParameters(::windows::core::IUnknown);
impl IRdcGeneratorParameters {
    #[doc = "*Required features: 'Win32_Networking_RemoteDifferentialCompression'*"]
    pub unsafe fn GetGeneratorParametersType(&self) -> ::windows::core::Result<GeneratorParametersType> {
        let mut result__: GeneratorParametersType = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<GeneratorParametersType>(result__)
    }
    #[doc = "*Required features: 'Win32_Networking_RemoteDifferentialCompression'*"]
    pub unsafe fn GetParametersVersion(&self, currentversion: *mut u32, minimumcompatibleappversion: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(currentversion), ::core::mem::transmute(minimumcompatibleappversion)).ok()
    }
    #[doc = "*Required features: 'Win32_Networking_RemoteDifferentialCompression'*"]
    pub unsafe fn GetSerializeSize(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: 'Win32_Networking_RemoteDifferentialCompression'*"]
    pub unsafe fn Serialize(&self, size: u32, parametersblob: *mut u8, byteswritten: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(size), ::core::mem::transmute(parametersblob), ::core::mem::transmute(byteswritten)).ok()
    }
}
impl ::core::convert::From<IRdcGeneratorParameters> for ::windows::core::IUnknown {
    fn from(value: IRdcGeneratorParameters) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IRdcGeneratorParameters> for ::windows::core::IUnknown {
    fn from(value: &IRdcGeneratorParameters) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IRdcGeneratorParameters {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IRdcGeneratorParameters {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IRdcGeneratorParameters {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IRdcGeneratorParameters {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IRdcGeneratorParameters {}
unsafe impl ::windows::core::Interface for IRdcGeneratorParameters {
    type Vtable = IRdcGeneratorParametersVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x96236a71_9dbc_11da_9e3f_0011114ae311);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRdcGeneratorParametersVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, parameterstype: *mut GeneratorParametersType) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, currentversion: *mut u32, minimumcompatibleappversion: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, size: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, size: u32, parametersblob: *mut u8, byteswritten: *mut u32) -> ::windows::core::HRESULT,
);
#[doc = "*Required features: 'Win32_Networking_RemoteDifferentialCompression'*"]
#[repr(transparent)]
pub struct IRdcLibrary(::windows::core::IUnknown);
impl IRdcLibrary {
    #[doc = "*Required features: 'Win32_Networking_RemoteDifferentialCompression'*"]
    pub unsafe fn ComputeDefaultRecursionDepth(&self, filesize: u64) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(filesize), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: 'Win32_Networking_RemoteDifferentialCompression'*"]
    pub unsafe fn CreateGeneratorParameters(&self, parameterstype: GeneratorParametersType, level: u32) -> ::windows::core::Result<IRdcGeneratorParameters> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(parameterstype), ::core::mem::transmute(level), ::core::mem::transmute(&mut result__)).from_abi::<IRdcGeneratorParameters>(result__)
    }
    #[doc = "*Required features: 'Win32_Networking_RemoteDifferentialCompression'*"]
    pub unsafe fn OpenGeneratorParameters(&self, size: u32, parametersblob: *const u8) -> ::windows::core::Result<IRdcGeneratorParameters> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(size), ::core::mem::transmute(parametersblob), ::core::mem::transmute(&mut result__)).from_abi::<IRdcGeneratorParameters>(result__)
    }
    #[doc = "*Required features: 'Win32_Networking_RemoteDifferentialCompression'*"]
    pub unsafe fn CreateGenerator(&self, depth: u32, igeneratorparametersarray: *const ::core::option::Option<IRdcGeneratorParameters>) -> ::windows::core::Result<IRdcGenerator> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(depth), ::core::mem::transmute(igeneratorparametersarray), ::core::mem::transmute(&mut result__)).from_abi::<IRdcGenerator>(result__)
    }
    #[doc = "*Required features: 'Win32_Networking_RemoteDifferentialCompression'*"]
    pub unsafe fn CreateComparator<'a, Param0: ::windows::core::IntoParam<'a, IRdcFileReader>>(&self, iseedsignaturesfile: Param0, comparatorbuffersize: u32) -> ::windows::core::Result<IRdcComparator> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), iseedsignaturesfile.into_param().abi(), ::core::mem::transmute(comparatorbuffersize), ::core::mem::transmute(&mut result__)).from_abi::<IRdcComparator>(result__)
    }
    #[doc = "*Required features: 'Win32_Networking_RemoteDifferentialCompression'*"]
    pub unsafe fn CreateSignatureReader<'a, Param0: ::windows::core::IntoParam<'a, IRdcFileReader>>(&self, ifilereader: Param0) -> ::windows::core::Result<IRdcSignatureReader> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), ifilereader.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<IRdcSignatureReader>(result__)
    }
    #[doc = "*Required features: 'Win32_Networking_RemoteDifferentialCompression'*"]
    pub unsafe fn GetRDCVersion(&self, currentversion: *mut u32, minimumcompatibleappversion: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), ::core::mem::transmute(currentversion), ::core::mem::transmute(minimumcompatibleappversion)).ok()
    }
}
impl ::core::convert::From<IRdcLibrary> for ::windows::core::IUnknown {
    fn from(value: IRdcLibrary) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IRdcLibrary> for ::windows::core::IUnknown {
    fn from(value: &IRdcLibrary) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IRdcLibrary {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IRdcLibrary {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IRdcLibrary {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IRdcLibrary {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IRdcLibrary {}
unsafe impl ::windows::core::Interface for IRdcLibrary {
    type Vtable = IRdcLibraryVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x96236a78_9dbc_11da_9e3f_0011114ae311);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRdcLibraryVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, filesize: u64, depth: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, parameterstype: GeneratorParametersType, level: u32, igeneratorparameters: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, size: u32, parametersblob: *const u8, igeneratorparameters: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, depth: u32, igeneratorparametersarray: *const ::windows::core::RawPtr, igenerator: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iseedsignaturesfile: ::windows::core::RawPtr, comparatorbuffersize: u32, icomparator: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ifilereader: ::windows::core::RawPtr, isignaturereader: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, currentversion: *mut u32, minimumcompatibleappversion: *mut u32) -> ::windows::core::HRESULT,
);
#[doc = "*Required features: 'Win32_Networking_RemoteDifferentialCompression'*"]
#[repr(transparent)]
pub struct IRdcSignatureReader(::windows::core::IUnknown);
impl IRdcSignatureReader {
    #[doc = "*Required features: 'Win32_Networking_RemoteDifferentialCompression'*"]
    pub unsafe fn ReadHeader(&self) -> ::windows::core::Result<RDC_ErrorCode> {
        let mut result__: RDC_ErrorCode = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<RDC_ErrorCode>(result__)
    }
    #[doc = "*Required features: 'Win32_Networking_RemoteDifferentialCompression', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ReadSignatures(&self, rdcsignaturepointer: *mut RdcSignaturePointer, endofoutput: *mut super::super::Foundation::BOOL) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(rdcsignaturepointer), ::core::mem::transmute(endofoutput)).ok()
    }
}
impl ::core::convert::From<IRdcSignatureReader> for ::windows::core::IUnknown {
    fn from(value: IRdcSignatureReader) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IRdcSignatureReader> for ::windows::core::IUnknown {
    fn from(value: &IRdcSignatureReader) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IRdcSignatureReader {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IRdcSignatureReader {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IRdcSignatureReader {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IRdcSignatureReader {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IRdcSignatureReader {}
unsafe impl ::windows::core::Interface for IRdcSignatureReader {
    type Vtable = IRdcSignatureReaderVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x96236a76_9dbc_11da_9e3f_0011114ae311);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRdcSignatureReaderVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, rdc_errorcode: *mut RDC_ErrorCode) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, rdcsignaturepointer: *mut RdcSignaturePointer, endofoutput: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[doc = "*Required features: 'Win32_Networking_RemoteDifferentialCompression'*"]
#[repr(transparent)]
pub struct IRdcSimilarityGenerator(::windows::core::IUnknown);
impl IRdcSimilarityGenerator {
    #[doc = "*Required features: 'Win32_Networking_RemoteDifferentialCompression'*"]
    pub unsafe fn EnableSimilarity(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: 'Win32_Networking_RemoteDifferentialCompression'*"]
    pub unsafe fn Results(&self) -> ::windows::core::Result<SimilarityData> {
        let mut result__: SimilarityData = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<SimilarityData>(result__)
    }
}
impl ::core::convert::From<IRdcSimilarityGenerator> for ::windows::core::IUnknown {
    fn from(value: IRdcSimilarityGenerator) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IRdcSimilarityGenerator> for ::windows::core::IUnknown {
    fn from(value: &IRdcSimilarityGenerator) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IRdcSimilarityGenerator {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IRdcSimilarityGenerator {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IRdcSimilarityGenerator {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IRdcSimilarityGenerator {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IRdcSimilarityGenerator {}
unsafe impl ::windows::core::Interface for IRdcSimilarityGenerator {
    type Vtable = IRdcSimilarityGeneratorVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x96236a80_9dbc_11da_9e3f_0011114ae311);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRdcSimilarityGeneratorVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, similaritydata: *mut SimilarityData) -> ::windows::core::HRESULT,
);
#[doc = "*Required features: 'Win32_Networking_RemoteDifferentialCompression'*"]
#[repr(transparent)]
pub struct ISimilarity(::windows::core::IUnknown);
impl ISimilarity {
    #[doc = "*Required features: 'Win32_Networking_RemoteDifferentialCompression', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CreateTable<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(&self, path: Param0, truncate: Param1, securitydescriptor: *const u8, recordsize: u32) -> ::windows::core::Result<RdcCreatedTables> {
        let mut result__: RdcCreatedTables = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), path.into_param().abi(), truncate.into_param().abi(), ::core::mem::transmute(securitydescriptor), ::core::mem::transmute(recordsize), ::core::mem::transmute(&mut result__)).from_abi::<RdcCreatedTables>(result__)
    }
    #[doc = "*Required features: 'Win32_Networking_RemoteDifferentialCompression', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CreateTableIndirect<'a, Param0: ::windows::core::IntoParam<'a, ISimilarityTraitsMapping>, Param1: ::windows::core::IntoParam<'a, IRdcFileWriter>, Param2: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(&self, mapping: Param0, fileidfile: Param1, truncate: Param2, recordsize: u32) -> ::windows::core::Result<RdcCreatedTables> {
        let mut result__: RdcCreatedTables = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), mapping.into_param().abi(), fileidfile.into_param().abi(), truncate.into_param().abi(), ::core::mem::transmute(recordsize), ::core::mem::transmute(&mut result__)).from_abi::<RdcCreatedTables>(result__)
    }
    #[doc = "*Required features: 'Win32_Networking_RemoteDifferentialCompression', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CloseTable<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(&self, isvalid: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), isvalid.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_Networking_RemoteDifferentialCompression'*"]
    pub unsafe fn Append(&self, similarityfileid: *const SimilarityFileId, similaritydata: *const SimilarityData) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(similarityfileid), ::core::mem::transmute(similaritydata)).ok()
    }
    #[doc = "*Required features: 'Win32_Networking_RemoteDifferentialCompression'*"]
    pub unsafe fn FindSimilarFileId(&self, similaritydata: *const SimilarityData, numberofmatchesrequired: u16, resultssize: u32) -> ::windows::core::Result<IFindSimilarResults> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(similaritydata), ::core::mem::transmute(numberofmatchesrequired), ::core::mem::transmute(resultssize), ::core::mem::transmute(&mut result__)).from_abi::<IFindSimilarResults>(result__)
    }
    #[doc = "*Required features: 'Win32_Networking_RemoteDifferentialCompression'*"]
    pub unsafe fn CopyAndSwap<'a, Param0: ::windows::core::IntoParam<'a, ISimilarity>, Param1: ::windows::core::IntoParam<'a, ISimilarityReportProgress>>(&self, newsimilaritytables: Param0, reportprogress: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), newsimilaritytables.into_param().abi(), reportprogress.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_Networking_RemoteDifferentialCompression'*"]
    pub unsafe fn GetRecordCount(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
}
impl ::core::convert::From<ISimilarity> for ::windows::core::IUnknown {
    fn from(value: ISimilarity) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ISimilarity> for ::windows::core::IUnknown {
    fn from(value: &ISimilarity) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ISimilarity {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &ISimilarity {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for ISimilarity {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ISimilarity {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ISimilarity {}
unsafe impl ::windows::core::Interface for ISimilarity {
    type Vtable = ISimilarityVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x96236a83_9dbc_11da_9e3f_0011114ae311);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISimilarityVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, path: super::super::Foundation::PWSTR, truncate: super::super::Foundation::BOOL, securitydescriptor: *const u8, recordsize: u32, isnew: *mut RdcCreatedTables) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, mapping: ::windows::core::RawPtr, fileidfile: ::windows::core::RawPtr, truncate: super::super::Foundation::BOOL, recordsize: u32, isnew: *mut RdcCreatedTables) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, isvalid: super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, similarityfileid: *const SimilarityFileId, similaritydata: *const SimilarityData) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, similaritydata: *const SimilarityData, numberofmatchesrequired: u16, resultssize: u32, findsimilarresults: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, newsimilaritytables: ::windows::core::RawPtr, reportprogress: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, recordcount: *mut u32) -> ::windows::core::HRESULT,
);
#[doc = "*Required features: 'Win32_Networking_RemoteDifferentialCompression'*"]
#[repr(transparent)]
pub struct ISimilarityFileIdTable(::windows::core::IUnknown);
impl ISimilarityFileIdTable {
    #[doc = "*Required features: 'Win32_Networking_RemoteDifferentialCompression', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CreateTable<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(&self, path: Param0, truncate: Param1, securitydescriptor: *const u8, recordsize: u32) -> ::windows::core::Result<RdcCreatedTables> {
        let mut result__: RdcCreatedTables = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), path.into_param().abi(), truncate.into_param().abi(), ::core::mem::transmute(securitydescriptor), ::core::mem::transmute(recordsize), ::core::mem::transmute(&mut result__)).from_abi::<RdcCreatedTables>(result__)
    }
    #[doc = "*Required features: 'Win32_Networking_RemoteDifferentialCompression', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CreateTableIndirect<'a, Param0: ::windows::core::IntoParam<'a, IRdcFileWriter>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(&self, fileidfile: Param0, truncate: Param1, recordsize: u32) -> ::windows::core::Result<RdcCreatedTables> {
        let mut result__: RdcCreatedTables = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), fileidfile.into_param().abi(), truncate.into_param().abi(), ::core::mem::transmute(recordsize), ::core::mem::transmute(&mut result__)).from_abi::<RdcCreatedTables>(result__)
    }
    #[doc = "*Required features: 'Win32_Networking_RemoteDifferentialCompression', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CloseTable<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(&self, isvalid: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), isvalid.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_Networking_RemoteDifferentialCompression'*"]
    pub unsafe fn Append(&self, similarityfileid: *const SimilarityFileId) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(similarityfileid), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: 'Win32_Networking_RemoteDifferentialCompression'*"]
    pub unsafe fn Lookup(&self, similarityfileindex: u32) -> ::windows::core::Result<SimilarityFileId> {
        let mut result__: SimilarityFileId = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(similarityfileindex), ::core::mem::transmute(&mut result__)).from_abi::<SimilarityFileId>(result__)
    }
    #[doc = "*Required features: 'Win32_Networking_RemoteDifferentialCompression'*"]
    pub unsafe fn Invalidate(&self, similarityfileindex: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), ::core::mem::transmute(similarityfileindex)).ok()
    }
    #[doc = "*Required features: 'Win32_Networking_RemoteDifferentialCompression'*"]
    pub unsafe fn GetRecordCount(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
}
impl ::core::convert::From<ISimilarityFileIdTable> for ::windows::core::IUnknown {
    fn from(value: ISimilarityFileIdTable) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ISimilarityFileIdTable> for ::windows::core::IUnknown {
    fn from(value: &ISimilarityFileIdTable) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ISimilarityFileIdTable {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &ISimilarityFileIdTable {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for ISimilarityFileIdTable {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ISimilarityFileIdTable {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ISimilarityFileIdTable {}
unsafe impl ::windows::core::Interface for ISimilarityFileIdTable {
    type Vtable = ISimilarityFileIdTableVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x96236a7f_9dbc_11da_9e3f_0011114ae311);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISimilarityFileIdTableVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, path: super::super::Foundation::PWSTR, truncate: super::super::Foundation::BOOL, securitydescriptor: *const u8, recordsize: u32, isnew: *mut RdcCreatedTables) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, fileidfile: ::windows::core::RawPtr, truncate: super::super::Foundation::BOOL, recordsize: u32, isnew: *mut RdcCreatedTables) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, isvalid: super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, similarityfileid: *const SimilarityFileId, similarityfileindex: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, similarityfileindex: u32, similarityfileid: *mut SimilarityFileId) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, similarityfileindex: u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, recordcount: *mut u32) -> ::windows::core::HRESULT,
);
#[doc = "*Required features: 'Win32_Networking_RemoteDifferentialCompression'*"]
#[repr(transparent)]
pub struct ISimilarityReportProgress(::windows::core::IUnknown);
impl ISimilarityReportProgress {
    #[doc = "*Required features: 'Win32_Networking_RemoteDifferentialCompression'*"]
    pub unsafe fn ReportProgress(&self, percentcompleted: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(percentcompleted)).ok()
    }
}
impl ::core::convert::From<ISimilarityReportProgress> for ::windows::core::IUnknown {
    fn from(value: ISimilarityReportProgress) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ISimilarityReportProgress> for ::windows::core::IUnknown {
    fn from(value: &ISimilarityReportProgress) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ISimilarityReportProgress {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &ISimilarityReportProgress {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for ISimilarityReportProgress {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ISimilarityReportProgress {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ISimilarityReportProgress {}
unsafe impl ::windows::core::Interface for ISimilarityReportProgress {
    type Vtable = ISimilarityReportProgressVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x96236a7a_9dbc_11da_9e3f_0011114ae311);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISimilarityReportProgressVtbl(pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT, pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32, pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32, pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, percentcompleted: u32) -> ::windows::core::HRESULT);
#[doc = "*Required features: 'Win32_Networking_RemoteDifferentialCompression'*"]
#[repr(transparent)]
pub struct ISimilarityTableDumpState(::windows::core::IUnknown);
impl ISimilarityTableDumpState {
    #[doc = "*Required features: 'Win32_Networking_RemoteDifferentialCompression', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetNextData(&self, resultssize: u32, resultsused: *mut u32, eof: *mut super::super::Foundation::BOOL, results: *mut SimilarityDumpData) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(resultssize), ::core::mem::transmute(resultsused), ::core::mem::transmute(eof), ::core::mem::transmute(results)).ok()
    }
}
impl ::core::convert::From<ISimilarityTableDumpState> for ::windows::core::IUnknown {
    fn from(value: ISimilarityTableDumpState) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ISimilarityTableDumpState> for ::windows::core::IUnknown {
    fn from(value: &ISimilarityTableDumpState) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ISimilarityTableDumpState {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &ISimilarityTableDumpState {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for ISimilarityTableDumpState {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ISimilarityTableDumpState {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ISimilarityTableDumpState {}
unsafe impl ::windows::core::Interface for ISimilarityTableDumpState {
    type Vtable = ISimilarityTableDumpStateVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x96236a7b_9dbc_11da_9e3f_0011114ae311);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISimilarityTableDumpStateVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, resultssize: u32, resultsused: *mut u32, eof: *mut super::super::Foundation::BOOL, results: *mut SimilarityDumpData) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[doc = "*Required features: 'Win32_Networking_RemoteDifferentialCompression'*"]
#[repr(transparent)]
pub struct ISimilarityTraitsMappedView(::windows::core::IUnknown);
impl ISimilarityTraitsMappedView {
    #[doc = "*Required features: 'Win32_Networking_RemoteDifferentialCompression'*"]
    pub unsafe fn Flush(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: 'Win32_Networking_RemoteDifferentialCompression'*"]
    pub unsafe fn Unmap(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: 'Win32_Networking_RemoteDifferentialCompression', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Get<'a, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(&self, index: u64, dirty: Param1, numelements: u32) -> ::windows::core::Result<SimilarityMappedViewInfo> {
        let mut result__: SimilarityMappedViewInfo = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(index), dirty.into_param().abi(), ::core::mem::transmute(numelements), ::core::mem::transmute(&mut result__)).from_abi::<SimilarityMappedViewInfo>(result__)
    }
    #[doc = "*Required features: 'Win32_Networking_RemoteDifferentialCompression'*"]
    pub unsafe fn GetView(&self, mappedpagebegin: *mut *mut u8, mappedpageend: *mut *mut u8) {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(mappedpagebegin), ::core::mem::transmute(mappedpageend))
    }
}
impl ::core::convert::From<ISimilarityTraitsMappedView> for ::windows::core::IUnknown {
    fn from(value: ISimilarityTraitsMappedView) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ISimilarityTraitsMappedView> for ::windows::core::IUnknown {
    fn from(value: &ISimilarityTraitsMappedView) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ISimilarityTraitsMappedView {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &ISimilarityTraitsMappedView {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for ISimilarityTraitsMappedView {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ISimilarityTraitsMappedView {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ISimilarityTraitsMappedView {}
unsafe impl ::windows::core::Interface for ISimilarityTraitsMappedView {
    type Vtable = ISimilarityTraitsMappedViewVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x96236a7c_9dbc_11da_9e3f_0011114ae311);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISimilarityTraitsMappedViewVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: u64, dirty: super::super::Foundation::BOOL, numelements: u32, viewinfo: *mut SimilarityMappedViewInfo) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, mappedpagebegin: *mut *mut u8, mappedpageend: *mut *mut u8),
);
#[doc = "*Required features: 'Win32_Networking_RemoteDifferentialCompression'*"]
#[repr(transparent)]
pub struct ISimilarityTraitsMapping(::windows::core::IUnknown);
impl ISimilarityTraitsMapping {
    #[doc = "*Required features: 'Win32_Networking_RemoteDifferentialCompression'*"]
    pub unsafe fn CloseMapping(&self) {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self))
    }
    #[doc = "*Required features: 'Win32_Networking_RemoteDifferentialCompression'*"]
    pub unsafe fn SetFileSize(&self, filesize: u64) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(filesize)).ok()
    }
    #[doc = "*Required features: 'Win32_Networking_RemoteDifferentialCompression'*"]
    pub unsafe fn GetFileSize(&self) -> ::windows::core::Result<u64> {
        let mut result__: u64 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u64>(result__)
    }
    #[doc = "*Required features: 'Win32_Networking_RemoteDifferentialCompression'*"]
    pub unsafe fn OpenMapping(&self, accessmode: RdcMappingAccessMode, begin: u64, end: u64) -> ::windows::core::Result<u64> {
        let mut result__: u64 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(accessmode), ::core::mem::transmute(begin), ::core::mem::transmute(end), ::core::mem::transmute(&mut result__)).from_abi::<u64>(result__)
    }
    #[doc = "*Required features: 'Win32_Networking_RemoteDifferentialCompression'*"]
    pub unsafe fn ResizeMapping(&self, accessmode: RdcMappingAccessMode, begin: u64, end: u64) -> ::windows::core::Result<u64> {
        let mut result__: u64 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(accessmode), ::core::mem::transmute(begin), ::core::mem::transmute(end), ::core::mem::transmute(&mut result__)).from_abi::<u64>(result__)
    }
    #[doc = "*Required features: 'Win32_Networking_RemoteDifferentialCompression'*"]
    pub unsafe fn GetPageSize(&self, pagesize: *mut u32) {
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), ::core::mem::transmute(pagesize))
    }
    #[doc = "*Required features: 'Win32_Networking_RemoteDifferentialCompression'*"]
    pub unsafe fn CreateView(&self, minimummappedpages: u32, accessmode: RdcMappingAccessMode) -> ::windows::core::Result<ISimilarityTraitsMappedView> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), ::core::mem::transmute(minimummappedpages), ::core::mem::transmute(accessmode), ::core::mem::transmute(&mut result__)).from_abi::<ISimilarityTraitsMappedView>(result__)
    }
}
impl ::core::convert::From<ISimilarityTraitsMapping> for ::windows::core::IUnknown {
    fn from(value: ISimilarityTraitsMapping) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ISimilarityTraitsMapping> for ::windows::core::IUnknown {
    fn from(value: &ISimilarityTraitsMapping) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ISimilarityTraitsMapping {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &ISimilarityTraitsMapping {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for ISimilarityTraitsMapping {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ISimilarityTraitsMapping {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ISimilarityTraitsMapping {}
unsafe impl ::windows::core::Interface for ISimilarityTraitsMapping {
    type Vtable = ISimilarityTraitsMappingVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x96236a7d_9dbc_11da_9e3f_0011114ae311);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISimilarityTraitsMappingVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void),
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, filesize: u64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, filesize: *mut u64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, accessmode: RdcMappingAccessMode, begin: u64, end: u64, actualend: *mut u64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, accessmode: RdcMappingAccessMode, begin: u64, end: u64, actualend: *mut u64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pagesize: *mut u32),
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, minimummappedpages: u32, accessmode: RdcMappingAccessMode, mappedview: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[doc = "*Required features: 'Win32_Networking_RemoteDifferentialCompression'*"]
#[repr(transparent)]
pub struct ISimilarityTraitsTable(::windows::core::IUnknown);
impl ISimilarityTraitsTable {
    #[doc = "*Required features: 'Win32_Networking_RemoteDifferentialCompression', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CreateTable<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(&self, path: Param0, truncate: Param1, securitydescriptor: *const u8) -> ::windows::core::Result<RdcCreatedTables> {
        let mut result__: RdcCreatedTables = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), path.into_param().abi(), truncate.into_param().abi(), ::core::mem::transmute(securitydescriptor), ::core::mem::transmute(&mut result__)).from_abi::<RdcCreatedTables>(result__)
    }
    #[doc = "*Required features: 'Win32_Networking_RemoteDifferentialCompression', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CreateTableIndirect<'a, Param0: ::windows::core::IntoParam<'a, ISimilarityTraitsMapping>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(&self, mapping: Param0, truncate: Param1) -> ::windows::core::Result<RdcCreatedTables> {
        let mut result__: RdcCreatedTables = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), mapping.into_param().abi(), truncate.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<RdcCreatedTables>(result__)
    }
    #[doc = "*Required features: 'Win32_Networking_RemoteDifferentialCompression', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CloseTable<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(&self, isvalid: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), isvalid.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_Networking_RemoteDifferentialCompression'*"]
    pub unsafe fn Append(&self, data: *const SimilarityData, fileindex: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(data), ::core::mem::transmute(fileindex)).ok()
    }
    #[doc = "*Required features: 'Win32_Networking_RemoteDifferentialCompression'*"]
    pub unsafe fn FindSimilarFileIndex(&self, similaritydata: *const SimilarityData, numberofmatchesrequired: u16, findsimilarfileindexresults: *mut FindSimilarFileIndexResults, resultssize: u32, resultsused: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(similaritydata), ::core::mem::transmute(numberofmatchesrequired), ::core::mem::transmute(findsimilarfileindexresults), ::core::mem::transmute(resultssize), ::core::mem::transmute(resultsused)).ok()
    }
    #[doc = "*Required features: 'Win32_Networking_RemoteDifferentialCompression'*"]
    pub unsafe fn BeginDump(&self) -> ::windows::core::Result<ISimilarityTableDumpState> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<ISimilarityTableDumpState>(result__)
    }
    #[doc = "*Required features: 'Win32_Networking_RemoteDifferentialCompression'*"]
    pub unsafe fn GetLastIndex(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
}
impl ::core::convert::From<ISimilarityTraitsTable> for ::windows::core::IUnknown {
    fn from(value: ISimilarityTraitsTable) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ISimilarityTraitsTable> for ::windows::core::IUnknown {
    fn from(value: &ISimilarityTraitsTable) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ISimilarityTraitsTable {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &ISimilarityTraitsTable {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for ISimilarityTraitsTable {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ISimilarityTraitsTable {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ISimilarityTraitsTable {}
unsafe impl ::windows::core::Interface for ISimilarityTraitsTable {
    type Vtable = ISimilarityTraitsTableVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x96236a7e_9dbc_11da_9e3f_0011114ae311);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISimilarityTraitsTableVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, path: super::super::Foundation::PWSTR, truncate: super::super::Foundation::BOOL, securitydescriptor: *const u8, isnew: *mut RdcCreatedTables) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, mapping: ::windows::core::RawPtr, truncate: super::super::Foundation::BOOL, isnew: *mut RdcCreatedTables) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, isvalid: super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, data: *const SimilarityData, fileindex: u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, similaritydata: *const SimilarityData, numberofmatchesrequired: u16, findsimilarfileindexresults: *mut FindSimilarFileIndexResults, resultssize: u32, resultsused: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, similaritytabledumpstate: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, fileindex: *mut u32) -> ::windows::core::HRESULT,
);
#[doc = "*Required features: 'Win32_Networking_RemoteDifferentialCompression'*"]
pub const MSRDC_DEFAULT_COMPAREBUFFER: u32 = 3200000u32;
#[doc = "*Required features: 'Win32_Networking_RemoteDifferentialCompression'*"]
pub const MSRDC_DEFAULT_HASHWINDOWSIZE_1: u32 = 48u32;
#[doc = "*Required features: 'Win32_Networking_RemoteDifferentialCompression'*"]
pub const MSRDC_DEFAULT_HASHWINDOWSIZE_N: u32 = 2u32;
#[doc = "*Required features: 'Win32_Networking_RemoteDifferentialCompression'*"]
pub const MSRDC_DEFAULT_HORIZONSIZE_1: u32 = 1024u32;
#[doc = "*Required features: 'Win32_Networking_RemoteDifferentialCompression'*"]
pub const MSRDC_DEFAULT_HORIZONSIZE_N: u32 = 128u32;
#[doc = "*Required features: 'Win32_Networking_RemoteDifferentialCompression'*"]
pub const MSRDC_MAXIMUM_COMPAREBUFFER: u32 = 1073741824u32;
#[doc = "*Required features: 'Win32_Networking_RemoteDifferentialCompression'*"]
pub const MSRDC_MAXIMUM_DEPTH: u32 = 8u32;
#[doc = "*Required features: 'Win32_Networking_RemoteDifferentialCompression'*"]
pub const MSRDC_MAXIMUM_HASHWINDOWSIZE: u32 = 96u32;
#[doc = "*Required features: 'Win32_Networking_RemoteDifferentialCompression'*"]
pub const MSRDC_MAXIMUM_HORIZONSIZE: u32 = 16384u32;
#[doc = "*Required features: 'Win32_Networking_RemoteDifferentialCompression'*"]
pub const MSRDC_MAXIMUM_MATCHESREQUIRED: u32 = 16u32;
#[doc = "*Required features: 'Win32_Networking_RemoteDifferentialCompression'*"]
pub const MSRDC_MAXIMUM_TRAITVALUE: u32 = 63u32;
#[doc = "*Required features: 'Win32_Networking_RemoteDifferentialCompression'*"]
pub const MSRDC_MINIMUM_COMPAREBUFFER: u32 = 100000u32;
#[doc = "*Required features: 'Win32_Networking_RemoteDifferentialCompression'*"]
pub const MSRDC_MINIMUM_COMPATIBLE_APP_VERSION: u32 = 65536u32;
#[doc = "*Required features: 'Win32_Networking_RemoteDifferentialCompression'*"]
pub const MSRDC_MINIMUM_DEPTH: u32 = 1u32;
#[doc = "*Required features: 'Win32_Networking_RemoteDifferentialCompression'*"]
pub const MSRDC_MINIMUM_HASHWINDOWSIZE: u32 = 2u32;
#[doc = "*Required features: 'Win32_Networking_RemoteDifferentialCompression'*"]
pub const MSRDC_MINIMUM_HORIZONSIZE: u32 = 128u32;
#[doc = "*Required features: 'Win32_Networking_RemoteDifferentialCompression'*"]
pub const MSRDC_MINIMUM_INPUTBUFFERSIZE: u32 = 1024u32;
#[doc = "*Required features: 'Win32_Networking_RemoteDifferentialCompression'*"]
pub const MSRDC_MINIMUM_MATCHESREQUIRED: u32 = 1u32;
#[doc = "*Required features: 'Win32_Networking_RemoteDifferentialCompression'*"]
pub const MSRDC_SIGNATURE_HASHSIZE: u32 = 16u32;
#[doc = "*Required features: 'Win32_Networking_RemoteDifferentialCompression'*"]
pub const MSRDC_VERSION: u32 = 65536u32;
#[doc = "*Required features: 'Win32_Networking_RemoteDifferentialCompression'*"]
pub const RDCE_TABLE_CORRUPT: u32 = 2147745794u32;
#[doc = "*Required features: 'Win32_Networking_RemoteDifferentialCompression'*"]
pub const RDCE_TABLE_FULL: u32 = 2147745793u32;
#[doc = "*Required features: 'Win32_Networking_RemoteDifferentialCompression'*"]
pub type RDC_ErrorCode = i32;
#[doc = "*Required features: 'Win32_Networking_RemoteDifferentialCompression'*"]
pub const RDC_NoError: RDC_ErrorCode = 0i32;
#[doc = "*Required features: 'Win32_Networking_RemoteDifferentialCompression'*"]
pub const RDC_HeaderVersionNewer: RDC_ErrorCode = 1i32;
#[doc = "*Required features: 'Win32_Networking_RemoteDifferentialCompression'*"]
pub const RDC_HeaderVersionOlder: RDC_ErrorCode = 2i32;
#[doc = "*Required features: 'Win32_Networking_RemoteDifferentialCompression'*"]
pub const RDC_HeaderMissingOrCorrupt: RDC_ErrorCode = 3i32;
#[doc = "*Required features: 'Win32_Networking_RemoteDifferentialCompression'*"]
pub const RDC_HeaderWrongType: RDC_ErrorCode = 4i32;
#[doc = "*Required features: 'Win32_Networking_RemoteDifferentialCompression'*"]
pub const RDC_DataMissingOrCorrupt: RDC_ErrorCode = 5i32;
#[doc = "*Required features: 'Win32_Networking_RemoteDifferentialCompression'*"]
pub const RDC_DataTooManyRecords: RDC_ErrorCode = 6i32;
#[doc = "*Required features: 'Win32_Networking_RemoteDifferentialCompression'*"]
pub const RDC_FileChecksumMismatch: RDC_ErrorCode = 7i32;
#[doc = "*Required features: 'Win32_Networking_RemoteDifferentialCompression'*"]
pub const RDC_ApplicationError: RDC_ErrorCode = 8i32;
#[doc = "*Required features: 'Win32_Networking_RemoteDifferentialCompression'*"]
pub const RDC_Aborted: RDC_ErrorCode = 9i32;
#[doc = "*Required features: 'Win32_Networking_RemoteDifferentialCompression'*"]
pub const RDC_Win32Error: RDC_ErrorCode = 10i32;
#[repr(C)]
#[doc = "*Required features: 'Win32_Networking_RemoteDifferentialCompression'*"]
pub struct RdcBufferPointer {
    pub m_Size: u32,
    pub m_Used: u32,
    pub m_Data: *mut u8,
}
impl ::core::marker::Copy for RdcBufferPointer {}
impl ::core::clone::Clone for RdcBufferPointer {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for RdcBufferPointer {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for RdcBufferPointer {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<RdcBufferPointer>()) == 0 }
    }
}
impl ::core::cmp::Eq for RdcBufferPointer {}
impl ::core::default::Default for RdcBufferPointer {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
pub const RdcComparator: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x96236a8b_9dbc_11da_9e3f_0011114ae311);
#[doc = "*Required features: 'Win32_Networking_RemoteDifferentialCompression'*"]
pub type RdcCreatedTables = i32;
#[doc = "*Required features: 'Win32_Networking_RemoteDifferentialCompression'*"]
pub const RDCTABLE_InvalidOrUnknown: RdcCreatedTables = 0i32;
#[doc = "*Required features: 'Win32_Networking_RemoteDifferentialCompression'*"]
pub const RDCTABLE_Existing: RdcCreatedTables = 1i32;
#[doc = "*Required features: 'Win32_Networking_RemoteDifferentialCompression'*"]
pub const RDCTABLE_New: RdcCreatedTables = 2i32;
pub const RdcFileReader: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x96236a89_9dbc_11da_9e3f_0011114ae311);
pub const RdcGenerator: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x96236a88_9dbc_11da_9e3f_0011114ae311);
pub const RdcGeneratorFilterMaxParameters: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x96236a87_9dbc_11da_9e3f_0011114ae311);
pub const RdcGeneratorParameters: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x96236a86_9dbc_11da_9e3f_0011114ae311);
pub const RdcLibrary: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x96236a85_9dbc_11da_9e3f_0011114ae311);
#[doc = "*Required features: 'Win32_Networking_RemoteDifferentialCompression'*"]
pub type RdcMappingAccessMode = i32;
#[doc = "*Required features: 'Win32_Networking_RemoteDifferentialCompression'*"]
pub const RDCMAPPING_Undefined: RdcMappingAccessMode = 0i32;
#[doc = "*Required features: 'Win32_Networking_RemoteDifferentialCompression'*"]
pub const RDCMAPPING_ReadOnly: RdcMappingAccessMode = 1i32;
#[doc = "*Required features: 'Win32_Networking_RemoteDifferentialCompression'*"]
pub const RDCMAPPING_ReadWrite: RdcMappingAccessMode = 2i32;
#[repr(C)]
#[doc = "*Required features: 'Win32_Networking_RemoteDifferentialCompression'*"]
pub struct RdcNeed {
    pub m_BlockType: RdcNeedType,
    pub m_FileOffset: u64,
    pub m_BlockLength: u64,
}
impl ::core::marker::Copy for RdcNeed {}
impl ::core::clone::Clone for RdcNeed {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for RdcNeed {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for RdcNeed {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<RdcNeed>()) == 0 }
    }
}
impl ::core::cmp::Eq for RdcNeed {}
impl ::core::default::Default for RdcNeed {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_Networking_RemoteDifferentialCompression'*"]
pub struct RdcNeedPointer {
    pub m_Size: u32,
    pub m_Used: u32,
    pub m_Data: *mut RdcNeed,
}
impl ::core::marker::Copy for RdcNeedPointer {}
impl ::core::clone::Clone for RdcNeedPointer {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for RdcNeedPointer {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for RdcNeedPointer {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<RdcNeedPointer>()) == 0 }
    }
}
impl ::core::cmp::Eq for RdcNeedPointer {}
impl ::core::default::Default for RdcNeedPointer {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: 'Win32_Networking_RemoteDifferentialCompression'*"]
pub type RdcNeedType = i32;
#[doc = "*Required features: 'Win32_Networking_RemoteDifferentialCompression'*"]
pub const RDCNEED_SOURCE: RdcNeedType = 0i32;
#[doc = "*Required features: 'Win32_Networking_RemoteDifferentialCompression'*"]
pub const RDCNEED_TARGET: RdcNeedType = 1i32;
#[doc = "*Required features: 'Win32_Networking_RemoteDifferentialCompression'*"]
pub const RDCNEED_SEED: RdcNeedType = 2i32;
#[doc = "*Required features: 'Win32_Networking_RemoteDifferentialCompression'*"]
pub const RDCNEED_SEED_MAX: RdcNeedType = 255i32;
#[repr(C)]
#[doc = "*Required features: 'Win32_Networking_RemoteDifferentialCompression'*"]
pub struct RdcSignature {
    pub m_Signature: [u8; 16],
    pub m_BlockLength: u16,
}
impl ::core::marker::Copy for RdcSignature {}
impl ::core::clone::Clone for RdcSignature {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for RdcSignature {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for RdcSignature {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<RdcSignature>()) == 0 }
    }
}
impl ::core::cmp::Eq for RdcSignature {}
impl ::core::default::Default for RdcSignature {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_Networking_RemoteDifferentialCompression'*"]
pub struct RdcSignaturePointer {
    pub m_Size: u32,
    pub m_Used: u32,
    pub m_Data: *mut RdcSignature,
}
impl ::core::marker::Copy for RdcSignaturePointer {}
impl ::core::clone::Clone for RdcSignaturePointer {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for RdcSignaturePointer {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for RdcSignaturePointer {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<RdcSignaturePointer>()) == 0 }
    }
}
impl ::core::cmp::Eq for RdcSignaturePointer {}
impl ::core::default::Default for RdcSignaturePointer {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
pub const RdcSignatureReader: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x96236a8a_9dbc_11da_9e3f_0011114ae311);
pub const RdcSimilarityGenerator: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x96236a92_9dbc_11da_9e3f_0011114ae311);
pub const Similarity: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x96236a91_9dbc_11da_9e3f_0011114ae311);
#[repr(C)]
#[doc = "*Required features: 'Win32_Networking_RemoteDifferentialCompression'*"]
pub struct SimilarityData {
    pub m_Data: [u8; 16],
}
impl ::core::marker::Copy for SimilarityData {}
impl ::core::clone::Clone for SimilarityData {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for SimilarityData {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for SimilarityData {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<SimilarityData>()) == 0 }
    }
}
impl ::core::cmp::Eq for SimilarityData {}
impl ::core::default::Default for SimilarityData {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_Networking_RemoteDifferentialCompression'*"]
pub struct SimilarityDumpData {
    pub m_FileIndex: u32,
    pub m_Data: SimilarityData,
}
impl ::core::marker::Copy for SimilarityDumpData {}
impl ::core::clone::Clone for SimilarityDumpData {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for SimilarityDumpData {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for SimilarityDumpData {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<SimilarityDumpData>()) == 0 }
    }
}
impl ::core::cmp::Eq for SimilarityDumpData {}
impl ::core::default::Default for SimilarityDumpData {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_Networking_RemoteDifferentialCompression'*"]
pub struct SimilarityFileId {
    pub m_FileId: [u8; 32],
}
impl ::core::marker::Copy for SimilarityFileId {}
impl ::core::clone::Clone for SimilarityFileId {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for SimilarityFileId {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for SimilarityFileId {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<SimilarityFileId>()) == 0 }
    }
}
impl ::core::cmp::Eq for SimilarityFileId {}
impl ::core::default::Default for SimilarityFileId {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: 'Win32_Networking_RemoteDifferentialCompression'*"]
pub const SimilarityFileIdMaxSize: u32 = 32u32;
#[doc = "*Required features: 'Win32_Networking_RemoteDifferentialCompression'*"]
pub const SimilarityFileIdMinSize: u32 = 4u32;
pub const SimilarityFileIdTable: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x96236a90_9dbc_11da_9e3f_0011114ae311);
#[repr(C)]
#[doc = "*Required features: 'Win32_Networking_RemoteDifferentialCompression'*"]
pub struct SimilarityMappedViewInfo {
    pub m_Data: *mut u8,
    pub m_Length: u32,
}
impl ::core::marker::Copy for SimilarityMappedViewInfo {}
impl ::core::clone::Clone for SimilarityMappedViewInfo {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for SimilarityMappedViewInfo {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for SimilarityMappedViewInfo {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<SimilarityMappedViewInfo>()) == 0 }
    }
}
impl ::core::cmp::Eq for SimilarityMappedViewInfo {}
impl ::core::default::Default for SimilarityMappedViewInfo {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
pub const SimilarityReportProgress: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x96236a8d_9dbc_11da_9e3f_0011114ae311);
pub const SimilarityTableDumpState: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x96236a8e_9dbc_11da_9e3f_0011114ae311);
pub const SimilarityTraitsMappedView: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x96236a95_9dbc_11da_9e3f_0011114ae311);
pub const SimilarityTraitsMapping: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x96236a94_9dbc_11da_9e3f_0011114ae311);
pub const SimilarityTraitsTable: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x96236a8f_9dbc_11da_9e3f_0011114ae311);
