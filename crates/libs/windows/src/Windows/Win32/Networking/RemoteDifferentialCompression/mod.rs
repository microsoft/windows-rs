#[doc = "*Required features: `\"Win32_Networking_RemoteDifferentialCompression\"`*"]
#[repr(transparent)]
pub struct IFindSimilarResults(::windows::core::IUnknown);
impl IFindSimilarResults {
    pub unsafe fn GetSize(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::windows::core::zeroed::<u32>();
        (::windows::core::Interface::vtable(self).GetSize)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetNextFileId(&self, numtraitsmatched: *mut u32, similarityfileid: *mut SimilarityFileId) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetNextFileId)(::windows::core::Interface::as_raw(self), numtraitsmatched, similarityfileid).ok()
    }
}
::windows::imp::interface_hierarchy!(IFindSimilarResults, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IFindSimilarResults {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IFindSimilarResults {}
impl ::core::fmt::Debug for IFindSimilarResults {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IFindSimilarResults").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IFindSimilarResults {
    type Vtable = IFindSimilarResults_Vtbl;
}
impl ::core::clone::Clone for IFindSimilarResults {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IFindSimilarResults {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x96236a81_9dbc_11da_9e3f_0011114ae311);
}
#[repr(C)]
#[doc(hidden)]
pub struct IFindSimilarResults_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub GetSize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, size: *mut u32) -> ::windows::core::HRESULT,
    pub GetNextFileId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, numtraitsmatched: *mut u32, similarityfileid: *mut SimilarityFileId) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Networking_RemoteDifferentialCompression\"`*"]
#[repr(transparent)]
pub struct IRdcComparator(::windows::core::IUnknown);
impl IRdcComparator {
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Process<P0>(&self, endofinput: P0, endofoutput: *mut super::super::Foundation::BOOL, inputbuffer: *mut RdcBufferPointer, outputbuffer: *mut RdcNeedPointer, rdc_errorcode: *mut RDC_ErrorCode) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<super::super::Foundation::BOOL>,
    {
        (::windows::core::Interface::vtable(self).Process)(::windows::core::Interface::as_raw(self), endofinput.into_param().abi(), endofoutput, inputbuffer, outputbuffer, rdc_errorcode).ok()
    }
}
::windows::imp::interface_hierarchy!(IRdcComparator, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IRdcComparator {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IRdcComparator {}
impl ::core::fmt::Debug for IRdcComparator {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IRdcComparator").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IRdcComparator {
    type Vtable = IRdcComparator_Vtbl;
}
impl ::core::clone::Clone for IRdcComparator {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IRdcComparator {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x96236a77_9dbc_11da_9e3f_0011114ae311);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRdcComparator_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub Process: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, endofinput: super::super::Foundation::BOOL, endofoutput: *mut super::super::Foundation::BOOL, inputbuffer: *mut RdcBufferPointer, outputbuffer: *mut RdcNeedPointer, rdc_errorcode: *mut RDC_ErrorCode) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Process: usize,
}
#[doc = "*Required features: `\"Win32_Networking_RemoteDifferentialCompression\"`*"]
#[repr(transparent)]
pub struct IRdcFileReader(::windows::core::IUnknown);
impl IRdcFileReader {
    pub unsafe fn GetFileSize(&self) -> ::windows::core::Result<u64> {
        let mut result__ = ::windows::core::zeroed::<u64>();
        (::windows::core::Interface::vtable(self).GetFileSize)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Read(&self, offsetfilestart: u64, bytestoread: u32, bytesactuallyread: *mut u32, buffer: *mut u8, eof: *mut super::super::Foundation::BOOL) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Read)(::windows::core::Interface::as_raw(self), offsetfilestart, bytestoread, bytesactuallyread, buffer, eof).ok()
    }
    pub unsafe fn GetFilePosition(&self) -> ::windows::core::Result<u64> {
        let mut result__ = ::windows::core::zeroed::<u64>();
        (::windows::core::Interface::vtable(self).GetFilePosition)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
::windows::imp::interface_hierarchy!(IRdcFileReader, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IRdcFileReader {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IRdcFileReader {}
impl ::core::fmt::Debug for IRdcFileReader {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IRdcFileReader").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IRdcFileReader {
    type Vtable = IRdcFileReader_Vtbl;
}
impl ::core::clone::Clone for IRdcFileReader {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IRdcFileReader {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x96236a74_9dbc_11da_9e3f_0011114ae311);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRdcFileReader_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub GetFileSize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, filesize: *mut u64) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub Read: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, offsetfilestart: u64, bytestoread: u32, bytesactuallyread: *mut u32, buffer: *mut u8, eof: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Read: usize,
    pub GetFilePosition: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, offsetfromstart: *mut u64) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Networking_RemoteDifferentialCompression\"`*"]
#[repr(transparent)]
pub struct IRdcFileWriter(::windows::core::IUnknown);
impl IRdcFileWriter {
    pub unsafe fn GetFileSize(&self) -> ::windows::core::Result<u64> {
        let mut result__ = ::windows::core::zeroed::<u64>();
        (::windows::core::Interface::vtable(self).base__.GetFileSize)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Read(&self, offsetfilestart: u64, bytestoread: u32, bytesactuallyread: *mut u32, buffer: *mut u8, eof: *mut super::super::Foundation::BOOL) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.Read)(::windows::core::Interface::as_raw(self), offsetfilestart, bytestoread, bytesactuallyread, buffer, eof).ok()
    }
    pub unsafe fn GetFilePosition(&self) -> ::windows::core::Result<u64> {
        let mut result__ = ::windows::core::zeroed::<u64>();
        (::windows::core::Interface::vtable(self).base__.GetFilePosition)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn Write(&self, offsetfilestart: u64, bytestowrite: u32) -> ::windows::core::Result<u8> {
        let mut result__ = ::windows::core::zeroed::<u8>();
        (::windows::core::Interface::vtable(self).Write)(::windows::core::Interface::as_raw(self), offsetfilestart, bytestowrite, &mut result__).from_abi(result__)
    }
    pub unsafe fn Truncate(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Truncate)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn DeleteOnClose(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).DeleteOnClose)(::windows::core::Interface::as_raw(self)).ok()
    }
}
::windows::imp::interface_hierarchy!(IRdcFileWriter, ::windows::core::IUnknown, IRdcFileReader);
impl ::core::cmp::PartialEq for IRdcFileWriter {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IRdcFileWriter {}
impl ::core::fmt::Debug for IRdcFileWriter {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IRdcFileWriter").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IRdcFileWriter {
    type Vtable = IRdcFileWriter_Vtbl;
}
impl ::core::clone::Clone for IRdcFileWriter {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IRdcFileWriter {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x96236a75_9dbc_11da_9e3f_0011114ae311);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRdcFileWriter_Vtbl {
    pub base__: IRdcFileReader_Vtbl,
    pub Write: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, offsetfilestart: u64, bytestowrite: u32, buffer: *mut u8) -> ::windows::core::HRESULT,
    pub Truncate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub DeleteOnClose: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Networking_RemoteDifferentialCompression\"`*"]
#[repr(transparent)]
pub struct IRdcGenerator(::windows::core::IUnknown);
impl IRdcGenerator {
    pub unsafe fn GetGeneratorParameters(&self, level: u32) -> ::windows::core::Result<IRdcGeneratorParameters> {
        let mut result__ = ::windows::core::zeroed::<IRdcGeneratorParameters>();
        (::windows::core::Interface::vtable(self).GetGeneratorParameters)(::windows::core::Interface::as_raw(self), level, &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Process<P0>(&self, endofinput: P0, endofoutput: *mut super::super::Foundation::BOOL, inputbuffer: *mut RdcBufferPointer, outputbuffers: &mut [*mut RdcBufferPointer], rdc_errorcode: *mut RDC_ErrorCode) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<super::super::Foundation::BOOL>,
    {
        (::windows::core::Interface::vtable(self).Process)(::windows::core::Interface::as_raw(self), endofinput.into_param().abi(), endofoutput, inputbuffer, outputbuffers.len() as _, ::core::mem::transmute(outputbuffers.as_ptr()), rdc_errorcode).ok()
    }
}
::windows::imp::interface_hierarchy!(IRdcGenerator, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IRdcGenerator {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IRdcGenerator {}
impl ::core::fmt::Debug for IRdcGenerator {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IRdcGenerator").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IRdcGenerator {
    type Vtable = IRdcGenerator_Vtbl;
}
impl ::core::clone::Clone for IRdcGenerator {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IRdcGenerator {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x96236a73_9dbc_11da_9e3f_0011114ae311);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRdcGenerator_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub GetGeneratorParameters: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, level: u32, igeneratorparameters: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub Process: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, endofinput: super::super::Foundation::BOOL, endofoutput: *mut super::super::Foundation::BOOL, inputbuffer: *mut RdcBufferPointer, depth: u32, outputbuffers: *mut *mut RdcBufferPointer, rdc_errorcode: *mut RDC_ErrorCode) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Process: usize,
}
#[doc = "*Required features: `\"Win32_Networking_RemoteDifferentialCompression\"`*"]
#[repr(transparent)]
pub struct IRdcGeneratorFilterMaxParameters(::windows::core::IUnknown);
impl IRdcGeneratorFilterMaxParameters {
    pub unsafe fn GetHorizonSize(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::windows::core::zeroed::<u32>();
        (::windows::core::Interface::vtable(self).GetHorizonSize)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetHorizonSize(&self, horizonsize: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetHorizonSize)(::windows::core::Interface::as_raw(self), horizonsize).ok()
    }
    pub unsafe fn GetHashWindowSize(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::windows::core::zeroed::<u32>();
        (::windows::core::Interface::vtable(self).GetHashWindowSize)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetHashWindowSize(&self, hashwindowsize: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetHashWindowSize)(::windows::core::Interface::as_raw(self), hashwindowsize).ok()
    }
}
::windows::imp::interface_hierarchy!(IRdcGeneratorFilterMaxParameters, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IRdcGeneratorFilterMaxParameters {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IRdcGeneratorFilterMaxParameters {}
impl ::core::fmt::Debug for IRdcGeneratorFilterMaxParameters {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IRdcGeneratorFilterMaxParameters").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IRdcGeneratorFilterMaxParameters {
    type Vtable = IRdcGeneratorFilterMaxParameters_Vtbl;
}
impl ::core::clone::Clone for IRdcGeneratorFilterMaxParameters {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IRdcGeneratorFilterMaxParameters {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x96236a72_9dbc_11da_9e3f_0011114ae311);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRdcGeneratorFilterMaxParameters_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub GetHorizonSize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, horizonsize: *mut u32) -> ::windows::core::HRESULT,
    pub SetHorizonSize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, horizonsize: u32) -> ::windows::core::HRESULT,
    pub GetHashWindowSize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hashwindowsize: *mut u32) -> ::windows::core::HRESULT,
    pub SetHashWindowSize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hashwindowsize: u32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Networking_RemoteDifferentialCompression\"`*"]
#[repr(transparent)]
pub struct IRdcGeneratorParameters(::windows::core::IUnknown);
impl IRdcGeneratorParameters {
    pub unsafe fn GetGeneratorParametersType(&self) -> ::windows::core::Result<GeneratorParametersType> {
        let mut result__ = ::windows::core::zeroed::<GeneratorParametersType>();
        (::windows::core::Interface::vtable(self).GetGeneratorParametersType)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetParametersVersion(&self, currentversion: *mut u32, minimumcompatibleappversion: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetParametersVersion)(::windows::core::Interface::as_raw(self), currentversion, minimumcompatibleappversion).ok()
    }
    pub unsafe fn GetSerializeSize(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::windows::core::zeroed::<u32>();
        (::windows::core::Interface::vtable(self).GetSerializeSize)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn Serialize(&self, size: u32, parametersblob: *mut u8, byteswritten: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Serialize)(::windows::core::Interface::as_raw(self), size, parametersblob, byteswritten).ok()
    }
}
::windows::imp::interface_hierarchy!(IRdcGeneratorParameters, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IRdcGeneratorParameters {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IRdcGeneratorParameters {}
impl ::core::fmt::Debug for IRdcGeneratorParameters {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IRdcGeneratorParameters").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IRdcGeneratorParameters {
    type Vtable = IRdcGeneratorParameters_Vtbl;
}
impl ::core::clone::Clone for IRdcGeneratorParameters {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IRdcGeneratorParameters {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x96236a71_9dbc_11da_9e3f_0011114ae311);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRdcGeneratorParameters_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub GetGeneratorParametersType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, parameterstype: *mut GeneratorParametersType) -> ::windows::core::HRESULT,
    pub GetParametersVersion: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, currentversion: *mut u32, minimumcompatibleappversion: *mut u32) -> ::windows::core::HRESULT,
    pub GetSerializeSize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, size: *mut u32) -> ::windows::core::HRESULT,
    pub Serialize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, size: u32, parametersblob: *mut u8, byteswritten: *mut u32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Networking_RemoteDifferentialCompression\"`*"]
#[repr(transparent)]
pub struct IRdcLibrary(::windows::core::IUnknown);
impl IRdcLibrary {
    pub unsafe fn ComputeDefaultRecursionDepth(&self, filesize: u64) -> ::windows::core::Result<u32> {
        let mut result__ = ::windows::core::zeroed::<u32>();
        (::windows::core::Interface::vtable(self).ComputeDefaultRecursionDepth)(::windows::core::Interface::as_raw(self), filesize, &mut result__).from_abi(result__)
    }
    pub unsafe fn CreateGeneratorParameters(&self, parameterstype: GeneratorParametersType, level: u32) -> ::windows::core::Result<IRdcGeneratorParameters> {
        let mut result__ = ::windows::core::zeroed::<IRdcGeneratorParameters>();
        (::windows::core::Interface::vtable(self).CreateGeneratorParameters)(::windows::core::Interface::as_raw(self), parameterstype, level, &mut result__).from_abi(result__)
    }
    pub unsafe fn OpenGeneratorParameters(&self, size: u32, parametersblob: *const u8) -> ::windows::core::Result<IRdcGeneratorParameters> {
        let mut result__ = ::windows::core::zeroed::<IRdcGeneratorParameters>();
        (::windows::core::Interface::vtable(self).OpenGeneratorParameters)(::windows::core::Interface::as_raw(self), size, parametersblob, &mut result__).from_abi(result__)
    }
    pub unsafe fn CreateGenerator(&self, igeneratorparametersarray: &[::core::option::Option<IRdcGeneratorParameters>]) -> ::windows::core::Result<IRdcGenerator> {
        let mut result__ = ::windows::core::zeroed::<IRdcGenerator>();
        (::windows::core::Interface::vtable(self).CreateGenerator)(::windows::core::Interface::as_raw(self), igeneratorparametersarray.len() as _, ::core::mem::transmute(igeneratorparametersarray.as_ptr()), &mut result__).from_abi(result__)
    }
    pub unsafe fn CreateComparator<P0>(&self, iseedsignaturesfile: P0, comparatorbuffersize: u32) -> ::windows::core::Result<IRdcComparator>
    where
        P0: ::windows::core::IntoParam<IRdcFileReader>,
    {
        let mut result__ = ::windows::core::zeroed::<IRdcComparator>();
        (::windows::core::Interface::vtable(self).CreateComparator)(::windows::core::Interface::as_raw(self), iseedsignaturesfile.into_param().abi(), comparatorbuffersize, &mut result__).from_abi(result__)
    }
    pub unsafe fn CreateSignatureReader<P0>(&self, ifilereader: P0) -> ::windows::core::Result<IRdcSignatureReader>
    where
        P0: ::windows::core::IntoParam<IRdcFileReader>,
    {
        let mut result__ = ::windows::core::zeroed::<IRdcSignatureReader>();
        (::windows::core::Interface::vtable(self).CreateSignatureReader)(::windows::core::Interface::as_raw(self), ifilereader.into_param().abi(), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetRDCVersion(&self, currentversion: *mut u32, minimumcompatibleappversion: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetRDCVersion)(::windows::core::Interface::as_raw(self), currentversion, minimumcompatibleappversion).ok()
    }
}
::windows::imp::interface_hierarchy!(IRdcLibrary, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IRdcLibrary {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IRdcLibrary {}
impl ::core::fmt::Debug for IRdcLibrary {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IRdcLibrary").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IRdcLibrary {
    type Vtable = IRdcLibrary_Vtbl;
}
impl ::core::clone::Clone for IRdcLibrary {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IRdcLibrary {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x96236a78_9dbc_11da_9e3f_0011114ae311);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRdcLibrary_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub ComputeDefaultRecursionDepth: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, filesize: u64, depth: *mut u32) -> ::windows::core::HRESULT,
    pub CreateGeneratorParameters: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, parameterstype: GeneratorParametersType, level: u32, igeneratorparameters: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub OpenGeneratorParameters: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, size: u32, parametersblob: *const u8, igeneratorparameters: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub CreateGenerator: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, depth: u32, igeneratorparametersarray: *const *mut ::core::ffi::c_void, igenerator: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub CreateComparator: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iseedsignaturesfile: *mut ::core::ffi::c_void, comparatorbuffersize: u32, icomparator: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub CreateSignatureReader: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ifilereader: *mut ::core::ffi::c_void, isignaturereader: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetRDCVersion: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, currentversion: *mut u32, minimumcompatibleappversion: *mut u32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Networking_RemoteDifferentialCompression\"`*"]
#[repr(transparent)]
pub struct IRdcSignatureReader(::windows::core::IUnknown);
impl IRdcSignatureReader {
    pub unsafe fn ReadHeader(&self) -> ::windows::core::Result<RDC_ErrorCode> {
        let mut result__ = ::windows::core::zeroed::<RDC_ErrorCode>();
        (::windows::core::Interface::vtable(self).ReadHeader)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ReadSignatures(&self, rdcsignaturepointer: *mut RdcSignaturePointer, endofoutput: *mut super::super::Foundation::BOOL) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).ReadSignatures)(::windows::core::Interface::as_raw(self), rdcsignaturepointer, endofoutput).ok()
    }
}
::windows::imp::interface_hierarchy!(IRdcSignatureReader, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IRdcSignatureReader {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IRdcSignatureReader {}
impl ::core::fmt::Debug for IRdcSignatureReader {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IRdcSignatureReader").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IRdcSignatureReader {
    type Vtable = IRdcSignatureReader_Vtbl;
}
impl ::core::clone::Clone for IRdcSignatureReader {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IRdcSignatureReader {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x96236a76_9dbc_11da_9e3f_0011114ae311);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRdcSignatureReader_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub ReadHeader: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, rdc_errorcode: *mut RDC_ErrorCode) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub ReadSignatures: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, rdcsignaturepointer: *mut RdcSignaturePointer, endofoutput: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    ReadSignatures: usize,
}
#[doc = "*Required features: `\"Win32_Networking_RemoteDifferentialCompression\"`*"]
#[repr(transparent)]
pub struct IRdcSimilarityGenerator(::windows::core::IUnknown);
impl IRdcSimilarityGenerator {
    pub unsafe fn EnableSimilarity(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).EnableSimilarity)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Results(&self) -> ::windows::core::Result<SimilarityData> {
        let mut result__ = ::windows::core::zeroed::<SimilarityData>();
        (::windows::core::Interface::vtable(self).Results)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
::windows::imp::interface_hierarchy!(IRdcSimilarityGenerator, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IRdcSimilarityGenerator {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IRdcSimilarityGenerator {}
impl ::core::fmt::Debug for IRdcSimilarityGenerator {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IRdcSimilarityGenerator").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IRdcSimilarityGenerator {
    type Vtable = IRdcSimilarityGenerator_Vtbl;
}
impl ::core::clone::Clone for IRdcSimilarityGenerator {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IRdcSimilarityGenerator {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x96236a80_9dbc_11da_9e3f_0011114ae311);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRdcSimilarityGenerator_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub EnableSimilarity: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Results: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, similaritydata: *mut SimilarityData) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Networking_RemoteDifferentialCompression\"`*"]
#[repr(transparent)]
pub struct ISimilarity(::windows::core::IUnknown);
impl ISimilarity {
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CreateTable<P0, P1>(&self, path: P0, truncate: P1, securitydescriptor: *const u8, recordsize: u32) -> ::windows::core::Result<RdcCreatedTables>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
        P1: ::windows::core::IntoParam<super::super::Foundation::BOOL>,
    {
        let mut result__ = ::windows::core::zeroed::<RdcCreatedTables>();
        (::windows::core::Interface::vtable(self).CreateTable)(::windows::core::Interface::as_raw(self), path.into_param().abi(), truncate.into_param().abi(), securitydescriptor, recordsize, &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CreateTableIndirect<P0, P1, P2>(&self, mapping: P0, fileidfile: P1, truncate: P2, recordsize: u32) -> ::windows::core::Result<RdcCreatedTables>
    where
        P0: ::windows::core::IntoParam<ISimilarityTraitsMapping>,
        P1: ::windows::core::IntoParam<IRdcFileWriter>,
        P2: ::windows::core::IntoParam<super::super::Foundation::BOOL>,
    {
        let mut result__ = ::windows::core::zeroed::<RdcCreatedTables>();
        (::windows::core::Interface::vtable(self).CreateTableIndirect)(::windows::core::Interface::as_raw(self), mapping.into_param().abi(), fileidfile.into_param().abi(), truncate.into_param().abi(), recordsize, &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CloseTable<P0>(&self, isvalid: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<super::super::Foundation::BOOL>,
    {
        (::windows::core::Interface::vtable(self).CloseTable)(::windows::core::Interface::as_raw(self), isvalid.into_param().abi()).ok()
    }
    pub unsafe fn Append(&self, similarityfileid: *const SimilarityFileId, similaritydata: *const SimilarityData) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Append)(::windows::core::Interface::as_raw(self), similarityfileid, similaritydata).ok()
    }
    pub unsafe fn FindSimilarFileId(&self, similaritydata: *const SimilarityData, numberofmatchesrequired: u16, resultssize: u32) -> ::windows::core::Result<IFindSimilarResults> {
        let mut result__ = ::windows::core::zeroed::<IFindSimilarResults>();
        (::windows::core::Interface::vtable(self).FindSimilarFileId)(::windows::core::Interface::as_raw(self), similaritydata, numberofmatchesrequired, resultssize, &mut result__).from_abi(result__)
    }
    pub unsafe fn CopyAndSwap<P0, P1>(&self, newsimilaritytables: P0, reportprogress: P1) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<ISimilarity>,
        P1: ::windows::core::IntoParam<ISimilarityReportProgress>,
    {
        (::windows::core::Interface::vtable(self).CopyAndSwap)(::windows::core::Interface::as_raw(self), newsimilaritytables.into_param().abi(), reportprogress.into_param().abi()).ok()
    }
    pub unsafe fn GetRecordCount(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::windows::core::zeroed::<u32>();
        (::windows::core::Interface::vtable(self).GetRecordCount)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
::windows::imp::interface_hierarchy!(ISimilarity, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for ISimilarity {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ISimilarity {}
impl ::core::fmt::Debug for ISimilarity {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISimilarity").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for ISimilarity {
    type Vtable = ISimilarity_Vtbl;
}
impl ::core::clone::Clone for ISimilarity {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for ISimilarity {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x96236a83_9dbc_11da_9e3f_0011114ae311);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISimilarity_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub CreateTable: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, path: ::windows::core::PCWSTR, truncate: super::super::Foundation::BOOL, securitydescriptor: *const u8, recordsize: u32, isnew: *mut RdcCreatedTables) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    CreateTable: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub CreateTableIndirect: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, mapping: *mut ::core::ffi::c_void, fileidfile: *mut ::core::ffi::c_void, truncate: super::super::Foundation::BOOL, recordsize: u32, isnew: *mut RdcCreatedTables) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    CreateTableIndirect: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub CloseTable: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, isvalid: super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    CloseTable: usize,
    pub Append: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, similarityfileid: *const SimilarityFileId, similaritydata: *const SimilarityData) -> ::windows::core::HRESULT,
    pub FindSimilarFileId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, similaritydata: *const SimilarityData, numberofmatchesrequired: u16, resultssize: u32, findsimilarresults: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub CopyAndSwap: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, newsimilaritytables: *mut ::core::ffi::c_void, reportprogress: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetRecordCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, recordcount: *mut u32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Networking_RemoteDifferentialCompression\"`*"]
#[repr(transparent)]
pub struct ISimilarityFileIdTable(::windows::core::IUnknown);
impl ISimilarityFileIdTable {
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CreateTable<P0, P1>(&self, path: P0, truncate: P1, securitydescriptor: *const u8, recordsize: u32) -> ::windows::core::Result<RdcCreatedTables>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
        P1: ::windows::core::IntoParam<super::super::Foundation::BOOL>,
    {
        let mut result__ = ::windows::core::zeroed::<RdcCreatedTables>();
        (::windows::core::Interface::vtable(self).CreateTable)(::windows::core::Interface::as_raw(self), path.into_param().abi(), truncate.into_param().abi(), securitydescriptor, recordsize, &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CreateTableIndirect<P0, P1>(&self, fileidfile: P0, truncate: P1, recordsize: u32) -> ::windows::core::Result<RdcCreatedTables>
    where
        P0: ::windows::core::IntoParam<IRdcFileWriter>,
        P1: ::windows::core::IntoParam<super::super::Foundation::BOOL>,
    {
        let mut result__ = ::windows::core::zeroed::<RdcCreatedTables>();
        (::windows::core::Interface::vtable(self).CreateTableIndirect)(::windows::core::Interface::as_raw(self), fileidfile.into_param().abi(), truncate.into_param().abi(), recordsize, &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CloseTable<P0>(&self, isvalid: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<super::super::Foundation::BOOL>,
    {
        (::windows::core::Interface::vtable(self).CloseTable)(::windows::core::Interface::as_raw(self), isvalid.into_param().abi()).ok()
    }
    pub unsafe fn Append(&self, similarityfileid: *const SimilarityFileId) -> ::windows::core::Result<u32> {
        let mut result__ = ::windows::core::zeroed::<u32>();
        (::windows::core::Interface::vtable(self).Append)(::windows::core::Interface::as_raw(self), similarityfileid, &mut result__).from_abi(result__)
    }
    pub unsafe fn Lookup(&self, similarityfileindex: u32, similarityfileid: *mut SimilarityFileId) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Lookup)(::windows::core::Interface::as_raw(self), similarityfileindex, similarityfileid).ok()
    }
    pub unsafe fn Invalidate(&self, similarityfileindex: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Invalidate)(::windows::core::Interface::as_raw(self), similarityfileindex).ok()
    }
    pub unsafe fn GetRecordCount(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::windows::core::zeroed::<u32>();
        (::windows::core::Interface::vtable(self).GetRecordCount)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
::windows::imp::interface_hierarchy!(ISimilarityFileIdTable, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for ISimilarityFileIdTable {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ISimilarityFileIdTable {}
impl ::core::fmt::Debug for ISimilarityFileIdTable {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISimilarityFileIdTable").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for ISimilarityFileIdTable {
    type Vtable = ISimilarityFileIdTable_Vtbl;
}
impl ::core::clone::Clone for ISimilarityFileIdTable {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for ISimilarityFileIdTable {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x96236a7f_9dbc_11da_9e3f_0011114ae311);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISimilarityFileIdTable_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub CreateTable: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, path: ::windows::core::PCWSTR, truncate: super::super::Foundation::BOOL, securitydescriptor: *const u8, recordsize: u32, isnew: *mut RdcCreatedTables) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    CreateTable: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub CreateTableIndirect: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, fileidfile: *mut ::core::ffi::c_void, truncate: super::super::Foundation::BOOL, recordsize: u32, isnew: *mut RdcCreatedTables) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    CreateTableIndirect: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub CloseTable: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, isvalid: super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    CloseTable: usize,
    pub Append: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, similarityfileid: *const SimilarityFileId, similarityfileindex: *mut u32) -> ::windows::core::HRESULT,
    pub Lookup: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, similarityfileindex: u32, similarityfileid: *mut SimilarityFileId) -> ::windows::core::HRESULT,
    pub Invalidate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, similarityfileindex: u32) -> ::windows::core::HRESULT,
    pub GetRecordCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, recordcount: *mut u32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Networking_RemoteDifferentialCompression\"`*"]
#[repr(transparent)]
pub struct ISimilarityReportProgress(::windows::core::IUnknown);
impl ISimilarityReportProgress {
    pub unsafe fn ReportProgress(&self, percentcompleted: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).ReportProgress)(::windows::core::Interface::as_raw(self), percentcompleted).ok()
    }
}
::windows::imp::interface_hierarchy!(ISimilarityReportProgress, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for ISimilarityReportProgress {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ISimilarityReportProgress {}
impl ::core::fmt::Debug for ISimilarityReportProgress {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISimilarityReportProgress").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for ISimilarityReportProgress {
    type Vtable = ISimilarityReportProgress_Vtbl;
}
impl ::core::clone::Clone for ISimilarityReportProgress {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for ISimilarityReportProgress {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x96236a7a_9dbc_11da_9e3f_0011114ae311);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISimilarityReportProgress_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub ReportProgress: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, percentcompleted: u32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Networking_RemoteDifferentialCompression\"`*"]
#[repr(transparent)]
pub struct ISimilarityTableDumpState(::windows::core::IUnknown);
impl ISimilarityTableDumpState {
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetNextData(&self, resultssize: u32, resultsused: *mut u32, eof: *mut super::super::Foundation::BOOL, results: *mut SimilarityDumpData) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetNextData)(::windows::core::Interface::as_raw(self), resultssize, resultsused, eof, results).ok()
    }
}
::windows::imp::interface_hierarchy!(ISimilarityTableDumpState, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for ISimilarityTableDumpState {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ISimilarityTableDumpState {}
impl ::core::fmt::Debug for ISimilarityTableDumpState {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISimilarityTableDumpState").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for ISimilarityTableDumpState {
    type Vtable = ISimilarityTableDumpState_Vtbl;
}
impl ::core::clone::Clone for ISimilarityTableDumpState {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for ISimilarityTableDumpState {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x96236a7b_9dbc_11da_9e3f_0011114ae311);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISimilarityTableDumpState_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub GetNextData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, resultssize: u32, resultsused: *mut u32, eof: *mut super::super::Foundation::BOOL, results: *mut SimilarityDumpData) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetNextData: usize,
}
#[doc = "*Required features: `\"Win32_Networking_RemoteDifferentialCompression\"`*"]
#[repr(transparent)]
pub struct ISimilarityTraitsMappedView(::windows::core::IUnknown);
impl ISimilarityTraitsMappedView {
    pub unsafe fn Flush(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Flush)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Unmap(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Unmap)(::windows::core::Interface::as_raw(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Get<P0>(&self, index: u64, dirty: P0, numelements: u32) -> ::windows::core::Result<SimilarityMappedViewInfo>
    where
        P0: ::windows::core::IntoParam<super::super::Foundation::BOOL>,
    {
        let mut result__ = ::windows::core::zeroed::<SimilarityMappedViewInfo>();
        (::windows::core::Interface::vtable(self).Get)(::windows::core::Interface::as_raw(self), index, dirty.into_param().abi(), numelements, &mut result__).from_abi(result__)
    }
    pub unsafe fn GetView(&self, mappedpagebegin: *mut *mut u8, mappedpageend: *mut *mut u8) {
        (::windows::core::Interface::vtable(self).GetView)(::windows::core::Interface::as_raw(self), mappedpagebegin, mappedpageend)
    }
}
::windows::imp::interface_hierarchy!(ISimilarityTraitsMappedView, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for ISimilarityTraitsMappedView {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ISimilarityTraitsMappedView {}
impl ::core::fmt::Debug for ISimilarityTraitsMappedView {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISimilarityTraitsMappedView").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for ISimilarityTraitsMappedView {
    type Vtable = ISimilarityTraitsMappedView_Vtbl;
}
impl ::core::clone::Clone for ISimilarityTraitsMappedView {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for ISimilarityTraitsMappedView {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x96236a7c_9dbc_11da_9e3f_0011114ae311);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISimilarityTraitsMappedView_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub Flush: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Unmap: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub Get: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: u64, dirty: super::super::Foundation::BOOL, numelements: u32, viewinfo: *mut SimilarityMappedViewInfo) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Get: usize,
    pub GetView: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, mappedpagebegin: *mut *mut u8, mappedpageend: *mut *mut u8),
}
#[doc = "*Required features: `\"Win32_Networking_RemoteDifferentialCompression\"`*"]
#[repr(transparent)]
pub struct ISimilarityTraitsMapping(::windows::core::IUnknown);
impl ISimilarityTraitsMapping {
    pub unsafe fn CloseMapping(&self) {
        (::windows::core::Interface::vtable(self).CloseMapping)(::windows::core::Interface::as_raw(self))
    }
    pub unsafe fn SetFileSize(&self, filesize: u64) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetFileSize)(::windows::core::Interface::as_raw(self), filesize).ok()
    }
    pub unsafe fn GetFileSize(&self) -> ::windows::core::Result<u64> {
        let mut result__ = ::windows::core::zeroed::<u64>();
        (::windows::core::Interface::vtable(self).GetFileSize)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn OpenMapping(&self, accessmode: RdcMappingAccessMode, begin: u64, end: u64) -> ::windows::core::Result<u64> {
        let mut result__ = ::windows::core::zeroed::<u64>();
        (::windows::core::Interface::vtable(self).OpenMapping)(::windows::core::Interface::as_raw(self), accessmode, begin, end, &mut result__).from_abi(result__)
    }
    pub unsafe fn ResizeMapping(&self, accessmode: RdcMappingAccessMode, begin: u64, end: u64) -> ::windows::core::Result<u64> {
        let mut result__ = ::windows::core::zeroed::<u64>();
        (::windows::core::Interface::vtable(self).ResizeMapping)(::windows::core::Interface::as_raw(self), accessmode, begin, end, &mut result__).from_abi(result__)
    }
    pub unsafe fn GetPageSize(&self) -> u32 {
        let mut result__ = ::windows::core::zeroed::<u32>();
        (::windows::core::Interface::vtable(self).GetPageSize)(::windows::core::Interface::as_raw(self), &mut result__);
        ::std::mem::transmute(result__)
    }
    pub unsafe fn CreateView(&self, minimummappedpages: u32, accessmode: RdcMappingAccessMode) -> ::windows::core::Result<ISimilarityTraitsMappedView> {
        let mut result__ = ::windows::core::zeroed::<ISimilarityTraitsMappedView>();
        (::windows::core::Interface::vtable(self).CreateView)(::windows::core::Interface::as_raw(self), minimummappedpages, accessmode, &mut result__).from_abi(result__)
    }
}
::windows::imp::interface_hierarchy!(ISimilarityTraitsMapping, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for ISimilarityTraitsMapping {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ISimilarityTraitsMapping {}
impl ::core::fmt::Debug for ISimilarityTraitsMapping {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISimilarityTraitsMapping").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for ISimilarityTraitsMapping {
    type Vtable = ISimilarityTraitsMapping_Vtbl;
}
impl ::core::clone::Clone for ISimilarityTraitsMapping {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for ISimilarityTraitsMapping {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x96236a7d_9dbc_11da_9e3f_0011114ae311);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISimilarityTraitsMapping_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub CloseMapping: unsafe extern "system" fn(this: *mut ::core::ffi::c_void),
    pub SetFileSize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, filesize: u64) -> ::windows::core::HRESULT,
    pub GetFileSize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, filesize: *mut u64) -> ::windows::core::HRESULT,
    pub OpenMapping: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, accessmode: RdcMappingAccessMode, begin: u64, end: u64, actualend: *mut u64) -> ::windows::core::HRESULT,
    pub ResizeMapping: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, accessmode: RdcMappingAccessMode, begin: u64, end: u64, actualend: *mut u64) -> ::windows::core::HRESULT,
    pub GetPageSize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pagesize: *mut u32),
    pub CreateView: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, minimummappedpages: u32, accessmode: RdcMappingAccessMode, mappedview: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Networking_RemoteDifferentialCompression\"`*"]
#[repr(transparent)]
pub struct ISimilarityTraitsTable(::windows::core::IUnknown);
impl ISimilarityTraitsTable {
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CreateTable<P0, P1>(&self, path: P0, truncate: P1, securitydescriptor: *const u8) -> ::windows::core::Result<RdcCreatedTables>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
        P1: ::windows::core::IntoParam<super::super::Foundation::BOOL>,
    {
        let mut result__ = ::windows::core::zeroed::<RdcCreatedTables>();
        (::windows::core::Interface::vtable(self).CreateTable)(::windows::core::Interface::as_raw(self), path.into_param().abi(), truncate.into_param().abi(), securitydescriptor, &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CreateTableIndirect<P0, P1>(&self, mapping: P0, truncate: P1) -> ::windows::core::Result<RdcCreatedTables>
    where
        P0: ::windows::core::IntoParam<ISimilarityTraitsMapping>,
        P1: ::windows::core::IntoParam<super::super::Foundation::BOOL>,
    {
        let mut result__ = ::windows::core::zeroed::<RdcCreatedTables>();
        (::windows::core::Interface::vtable(self).CreateTableIndirect)(::windows::core::Interface::as_raw(self), mapping.into_param().abi(), truncate.into_param().abi(), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CloseTable<P0>(&self, isvalid: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<super::super::Foundation::BOOL>,
    {
        (::windows::core::Interface::vtable(self).CloseTable)(::windows::core::Interface::as_raw(self), isvalid.into_param().abi()).ok()
    }
    pub unsafe fn Append(&self, data: *const SimilarityData, fileindex: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Append)(::windows::core::Interface::as_raw(self), data, fileindex).ok()
    }
    pub unsafe fn FindSimilarFileIndex(&self, similaritydata: *const SimilarityData, numberofmatchesrequired: u16, findsimilarfileindexresults: *mut FindSimilarFileIndexResults, resultssize: u32, resultsused: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).FindSimilarFileIndex)(::windows::core::Interface::as_raw(self), similaritydata, numberofmatchesrequired, findsimilarfileindexresults, resultssize, resultsused).ok()
    }
    pub unsafe fn BeginDump(&self) -> ::windows::core::Result<ISimilarityTableDumpState> {
        let mut result__ = ::windows::core::zeroed::<ISimilarityTableDumpState>();
        (::windows::core::Interface::vtable(self).BeginDump)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetLastIndex(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::windows::core::zeroed::<u32>();
        (::windows::core::Interface::vtable(self).GetLastIndex)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
::windows::imp::interface_hierarchy!(ISimilarityTraitsTable, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for ISimilarityTraitsTable {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ISimilarityTraitsTable {}
impl ::core::fmt::Debug for ISimilarityTraitsTable {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISimilarityTraitsTable").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for ISimilarityTraitsTable {
    type Vtable = ISimilarityTraitsTable_Vtbl;
}
impl ::core::clone::Clone for ISimilarityTraitsTable {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for ISimilarityTraitsTable {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x96236a7e_9dbc_11da_9e3f_0011114ae311);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISimilarityTraitsTable_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub CreateTable: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, path: ::windows::core::PCWSTR, truncate: super::super::Foundation::BOOL, securitydescriptor: *const u8, isnew: *mut RdcCreatedTables) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    CreateTable: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub CreateTableIndirect: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, mapping: *mut ::core::ffi::c_void, truncate: super::super::Foundation::BOOL, isnew: *mut RdcCreatedTables) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    CreateTableIndirect: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub CloseTable: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, isvalid: super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    CloseTable: usize,
    pub Append: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, data: *const SimilarityData, fileindex: u32) -> ::windows::core::HRESULT,
    pub FindSimilarFileIndex: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, similaritydata: *const SimilarityData, numberofmatchesrequired: u16, findsimilarfileindexresults: *mut FindSimilarFileIndexResults, resultssize: u32, resultsused: *mut u32) -> ::windows::core::HRESULT,
    pub BeginDump: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, similaritytabledumpstate: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetLastIndex: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, fileindex: *mut u32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Networking_RemoteDifferentialCompression\"`*"]
pub const FindSimilarResults: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x96236a93_9dbc_11da_9e3f_0011114ae311);
#[doc = "*Required features: `\"Win32_Networking_RemoteDifferentialCompression\"`*"]
pub const MSRDC_DEFAULT_COMPAREBUFFER: u32 = 3200000u32;
#[doc = "*Required features: `\"Win32_Networking_RemoteDifferentialCompression\"`*"]
pub const MSRDC_DEFAULT_HASHWINDOWSIZE_1: u32 = 48u32;
#[doc = "*Required features: `\"Win32_Networking_RemoteDifferentialCompression\"`*"]
pub const MSRDC_DEFAULT_HASHWINDOWSIZE_N: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Networking_RemoteDifferentialCompression\"`*"]
pub const MSRDC_DEFAULT_HORIZONSIZE_1: u32 = 1024u32;
#[doc = "*Required features: `\"Win32_Networking_RemoteDifferentialCompression\"`*"]
pub const MSRDC_DEFAULT_HORIZONSIZE_N: u32 = 128u32;
#[doc = "*Required features: `\"Win32_Networking_RemoteDifferentialCompression\"`*"]
pub const MSRDC_MAXIMUM_COMPAREBUFFER: u32 = 1073741824u32;
#[doc = "*Required features: `\"Win32_Networking_RemoteDifferentialCompression\"`*"]
pub const MSRDC_MAXIMUM_DEPTH: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Networking_RemoteDifferentialCompression\"`*"]
pub const MSRDC_MAXIMUM_HASHWINDOWSIZE: u32 = 96u32;
#[doc = "*Required features: `\"Win32_Networking_RemoteDifferentialCompression\"`*"]
pub const MSRDC_MAXIMUM_HORIZONSIZE: u32 = 16384u32;
#[doc = "*Required features: `\"Win32_Networking_RemoteDifferentialCompression\"`*"]
pub const MSRDC_MAXIMUM_MATCHESREQUIRED: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Networking_RemoteDifferentialCompression\"`*"]
pub const MSRDC_MAXIMUM_TRAITVALUE: u32 = 63u32;
#[doc = "*Required features: `\"Win32_Networking_RemoteDifferentialCompression\"`*"]
pub const MSRDC_MINIMUM_COMPAREBUFFER: u32 = 100000u32;
#[doc = "*Required features: `\"Win32_Networking_RemoteDifferentialCompression\"`*"]
pub const MSRDC_MINIMUM_COMPATIBLE_APP_VERSION: u32 = 65536u32;
#[doc = "*Required features: `\"Win32_Networking_RemoteDifferentialCompression\"`*"]
pub const MSRDC_MINIMUM_DEPTH: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Networking_RemoteDifferentialCompression\"`*"]
pub const MSRDC_MINIMUM_HASHWINDOWSIZE: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Networking_RemoteDifferentialCompression\"`*"]
pub const MSRDC_MINIMUM_HORIZONSIZE: u32 = 128u32;
#[doc = "*Required features: `\"Win32_Networking_RemoteDifferentialCompression\"`*"]
pub const MSRDC_MINIMUM_INPUTBUFFERSIZE: u32 = 1024u32;
#[doc = "*Required features: `\"Win32_Networking_RemoteDifferentialCompression\"`*"]
pub const MSRDC_MINIMUM_MATCHESREQUIRED: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Networking_RemoteDifferentialCompression\"`*"]
pub const MSRDC_SIGNATURE_HASHSIZE: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Networking_RemoteDifferentialCompression\"`*"]
pub const MSRDC_VERSION: u32 = 65536u32;
#[doc = "*Required features: `\"Win32_Networking_RemoteDifferentialCompression\"`*"]
pub const RDCE_TABLE_CORRUPT: u32 = 2147745794u32;
#[doc = "*Required features: `\"Win32_Networking_RemoteDifferentialCompression\"`*"]
pub const RDCE_TABLE_FULL: u32 = 2147745793u32;
#[doc = "*Required features: `\"Win32_Networking_RemoteDifferentialCompression\"`*"]
pub const RdcComparator: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x96236a8b_9dbc_11da_9e3f_0011114ae311);
#[doc = "*Required features: `\"Win32_Networking_RemoteDifferentialCompression\"`*"]
pub const RdcFileReader: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x96236a89_9dbc_11da_9e3f_0011114ae311);
#[doc = "*Required features: `\"Win32_Networking_RemoteDifferentialCompression\"`*"]
pub const RdcGenerator: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x96236a88_9dbc_11da_9e3f_0011114ae311);
#[doc = "*Required features: `\"Win32_Networking_RemoteDifferentialCompression\"`*"]
pub const RdcGeneratorFilterMaxParameters: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x96236a87_9dbc_11da_9e3f_0011114ae311);
#[doc = "*Required features: `\"Win32_Networking_RemoteDifferentialCompression\"`*"]
pub const RdcGeneratorParameters: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x96236a86_9dbc_11da_9e3f_0011114ae311);
#[doc = "*Required features: `\"Win32_Networking_RemoteDifferentialCompression\"`*"]
pub const RdcLibrary: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x96236a85_9dbc_11da_9e3f_0011114ae311);
#[doc = "*Required features: `\"Win32_Networking_RemoteDifferentialCompression\"`*"]
pub const RdcSignatureReader: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x96236a8a_9dbc_11da_9e3f_0011114ae311);
#[doc = "*Required features: `\"Win32_Networking_RemoteDifferentialCompression\"`*"]
pub const RdcSimilarityGenerator: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x96236a92_9dbc_11da_9e3f_0011114ae311);
#[doc = "*Required features: `\"Win32_Networking_RemoteDifferentialCompression\"`*"]
pub const Similarity: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x96236a91_9dbc_11da_9e3f_0011114ae311);
#[doc = "*Required features: `\"Win32_Networking_RemoteDifferentialCompression\"`*"]
pub const SimilarityFileIdMaxSize: u32 = 32u32;
#[doc = "*Required features: `\"Win32_Networking_RemoteDifferentialCompression\"`*"]
pub const SimilarityFileIdMinSize: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Networking_RemoteDifferentialCompression\"`*"]
pub const SimilarityFileIdTable: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x96236a90_9dbc_11da_9e3f_0011114ae311);
#[doc = "*Required features: `\"Win32_Networking_RemoteDifferentialCompression\"`*"]
pub const SimilarityReportProgress: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x96236a8d_9dbc_11da_9e3f_0011114ae311);
#[doc = "*Required features: `\"Win32_Networking_RemoteDifferentialCompression\"`*"]
pub const SimilarityTableDumpState: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x96236a8e_9dbc_11da_9e3f_0011114ae311);
#[doc = "*Required features: `\"Win32_Networking_RemoteDifferentialCompression\"`*"]
pub const SimilarityTraitsMappedView: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x96236a95_9dbc_11da_9e3f_0011114ae311);
#[doc = "*Required features: `\"Win32_Networking_RemoteDifferentialCompression\"`*"]
pub const SimilarityTraitsMapping: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x96236a94_9dbc_11da_9e3f_0011114ae311);
#[doc = "*Required features: `\"Win32_Networking_RemoteDifferentialCompression\"`*"]
pub const SimilarityTraitsTable: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x96236a8f_9dbc_11da_9e3f_0011114ae311);
#[doc = "*Required features: `\"Win32_Networking_RemoteDifferentialCompression\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct GeneratorParametersType(pub i32);
#[doc = "*Required features: `\"Win32_Networking_RemoteDifferentialCompression\"`*"]
pub const RDCGENTYPE_Unused: GeneratorParametersType = GeneratorParametersType(0i32);
#[doc = "*Required features: `\"Win32_Networking_RemoteDifferentialCompression\"`*"]
pub const RDCGENTYPE_FilterMax: GeneratorParametersType = GeneratorParametersType(1i32);
impl ::core::marker::Copy for GeneratorParametersType {}
impl ::core::clone::Clone for GeneratorParametersType {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for GeneratorParametersType {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for GeneratorParametersType {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for GeneratorParametersType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("GeneratorParametersType").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Networking_RemoteDifferentialCompression\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct RDC_ErrorCode(pub i32);
#[doc = "*Required features: `\"Win32_Networking_RemoteDifferentialCompression\"`*"]
pub const RDC_NoError: RDC_ErrorCode = RDC_ErrorCode(0i32);
#[doc = "*Required features: `\"Win32_Networking_RemoteDifferentialCompression\"`*"]
pub const RDC_HeaderVersionNewer: RDC_ErrorCode = RDC_ErrorCode(1i32);
#[doc = "*Required features: `\"Win32_Networking_RemoteDifferentialCompression\"`*"]
pub const RDC_HeaderVersionOlder: RDC_ErrorCode = RDC_ErrorCode(2i32);
#[doc = "*Required features: `\"Win32_Networking_RemoteDifferentialCompression\"`*"]
pub const RDC_HeaderMissingOrCorrupt: RDC_ErrorCode = RDC_ErrorCode(3i32);
#[doc = "*Required features: `\"Win32_Networking_RemoteDifferentialCompression\"`*"]
pub const RDC_HeaderWrongType: RDC_ErrorCode = RDC_ErrorCode(4i32);
#[doc = "*Required features: `\"Win32_Networking_RemoteDifferentialCompression\"`*"]
pub const RDC_DataMissingOrCorrupt: RDC_ErrorCode = RDC_ErrorCode(5i32);
#[doc = "*Required features: `\"Win32_Networking_RemoteDifferentialCompression\"`*"]
pub const RDC_DataTooManyRecords: RDC_ErrorCode = RDC_ErrorCode(6i32);
#[doc = "*Required features: `\"Win32_Networking_RemoteDifferentialCompression\"`*"]
pub const RDC_FileChecksumMismatch: RDC_ErrorCode = RDC_ErrorCode(7i32);
#[doc = "*Required features: `\"Win32_Networking_RemoteDifferentialCompression\"`*"]
pub const RDC_ApplicationError: RDC_ErrorCode = RDC_ErrorCode(8i32);
#[doc = "*Required features: `\"Win32_Networking_RemoteDifferentialCompression\"`*"]
pub const RDC_Aborted: RDC_ErrorCode = RDC_ErrorCode(9i32);
#[doc = "*Required features: `\"Win32_Networking_RemoteDifferentialCompression\"`*"]
pub const RDC_Win32Error: RDC_ErrorCode = RDC_ErrorCode(10i32);
impl ::core::marker::Copy for RDC_ErrorCode {}
impl ::core::clone::Clone for RDC_ErrorCode {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for RDC_ErrorCode {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for RDC_ErrorCode {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for RDC_ErrorCode {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("RDC_ErrorCode").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Networking_RemoteDifferentialCompression\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct RdcCreatedTables(pub i32);
#[doc = "*Required features: `\"Win32_Networking_RemoteDifferentialCompression\"`*"]
pub const RDCTABLE_InvalidOrUnknown: RdcCreatedTables = RdcCreatedTables(0i32);
#[doc = "*Required features: `\"Win32_Networking_RemoteDifferentialCompression\"`*"]
pub const RDCTABLE_Existing: RdcCreatedTables = RdcCreatedTables(1i32);
#[doc = "*Required features: `\"Win32_Networking_RemoteDifferentialCompression\"`*"]
pub const RDCTABLE_New: RdcCreatedTables = RdcCreatedTables(2i32);
impl ::core::marker::Copy for RdcCreatedTables {}
impl ::core::clone::Clone for RdcCreatedTables {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for RdcCreatedTables {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for RdcCreatedTables {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for RdcCreatedTables {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("RdcCreatedTables").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Networking_RemoteDifferentialCompression\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct RdcMappingAccessMode(pub i32);
#[doc = "*Required features: `\"Win32_Networking_RemoteDifferentialCompression\"`*"]
pub const RDCMAPPING_Undefined: RdcMappingAccessMode = RdcMappingAccessMode(0i32);
#[doc = "*Required features: `\"Win32_Networking_RemoteDifferentialCompression\"`*"]
pub const RDCMAPPING_ReadOnly: RdcMappingAccessMode = RdcMappingAccessMode(1i32);
#[doc = "*Required features: `\"Win32_Networking_RemoteDifferentialCompression\"`*"]
pub const RDCMAPPING_ReadWrite: RdcMappingAccessMode = RdcMappingAccessMode(2i32);
impl ::core::marker::Copy for RdcMappingAccessMode {}
impl ::core::clone::Clone for RdcMappingAccessMode {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for RdcMappingAccessMode {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for RdcMappingAccessMode {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for RdcMappingAccessMode {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("RdcMappingAccessMode").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Networking_RemoteDifferentialCompression\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct RdcNeedType(pub i32);
#[doc = "*Required features: `\"Win32_Networking_RemoteDifferentialCompression\"`*"]
pub const RDCNEED_SOURCE: RdcNeedType = RdcNeedType(0i32);
#[doc = "*Required features: `\"Win32_Networking_RemoteDifferentialCompression\"`*"]
pub const RDCNEED_TARGET: RdcNeedType = RdcNeedType(1i32);
#[doc = "*Required features: `\"Win32_Networking_RemoteDifferentialCompression\"`*"]
pub const RDCNEED_SEED: RdcNeedType = RdcNeedType(2i32);
#[doc = "*Required features: `\"Win32_Networking_RemoteDifferentialCompression\"`*"]
pub const RDCNEED_SEED_MAX: RdcNeedType = RdcNeedType(255i32);
impl ::core::marker::Copy for RdcNeedType {}
impl ::core::clone::Clone for RdcNeedType {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for RdcNeedType {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for RdcNeedType {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for RdcNeedType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("RdcNeedType").field(&self.0).finish()
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_RemoteDifferentialCompression\"`*"]
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
impl ::core::fmt::Debug for FindSimilarFileIndexResults {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("FindSimilarFileIndexResults").field("m_FileIndex", &self.m_FileIndex).field("m_MatchCount", &self.m_MatchCount).finish()
    }
}
impl ::windows::core::TypeKind for FindSimilarFileIndexResults {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for FindSimilarFileIndexResults {
    fn eq(&self, other: &Self) -> bool {
        self.m_FileIndex == other.m_FileIndex && self.m_MatchCount == other.m_MatchCount
    }
}
impl ::core::cmp::Eq for FindSimilarFileIndexResults {}
impl ::core::default::Default for FindSimilarFileIndexResults {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_RemoteDifferentialCompression\"`*"]
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
impl ::core::fmt::Debug for RdcBufferPointer {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("RdcBufferPointer").field("m_Size", &self.m_Size).field("m_Used", &self.m_Used).field("m_Data", &self.m_Data).finish()
    }
}
impl ::windows::core::TypeKind for RdcBufferPointer {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for RdcBufferPointer {
    fn eq(&self, other: &Self) -> bool {
        self.m_Size == other.m_Size && self.m_Used == other.m_Used && self.m_Data == other.m_Data
    }
}
impl ::core::cmp::Eq for RdcBufferPointer {}
impl ::core::default::Default for RdcBufferPointer {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_RemoteDifferentialCompression\"`*"]
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
impl ::core::fmt::Debug for RdcNeed {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("RdcNeed").field("m_BlockType", &self.m_BlockType).field("m_FileOffset", &self.m_FileOffset).field("m_BlockLength", &self.m_BlockLength).finish()
    }
}
impl ::windows::core::TypeKind for RdcNeed {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for RdcNeed {
    fn eq(&self, other: &Self) -> bool {
        self.m_BlockType == other.m_BlockType && self.m_FileOffset == other.m_FileOffset && self.m_BlockLength == other.m_BlockLength
    }
}
impl ::core::cmp::Eq for RdcNeed {}
impl ::core::default::Default for RdcNeed {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_RemoteDifferentialCompression\"`*"]
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
impl ::core::fmt::Debug for RdcNeedPointer {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("RdcNeedPointer").field("m_Size", &self.m_Size).field("m_Used", &self.m_Used).field("m_Data", &self.m_Data).finish()
    }
}
impl ::windows::core::TypeKind for RdcNeedPointer {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for RdcNeedPointer {
    fn eq(&self, other: &Self) -> bool {
        self.m_Size == other.m_Size && self.m_Used == other.m_Used && self.m_Data == other.m_Data
    }
}
impl ::core::cmp::Eq for RdcNeedPointer {}
impl ::core::default::Default for RdcNeedPointer {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_RemoteDifferentialCompression\"`*"]
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
impl ::core::fmt::Debug for RdcSignature {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("RdcSignature").field("m_Signature", &self.m_Signature).field("m_BlockLength", &self.m_BlockLength).finish()
    }
}
impl ::windows::core::TypeKind for RdcSignature {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for RdcSignature {
    fn eq(&self, other: &Self) -> bool {
        self.m_Signature == other.m_Signature && self.m_BlockLength == other.m_BlockLength
    }
}
impl ::core::cmp::Eq for RdcSignature {}
impl ::core::default::Default for RdcSignature {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_RemoteDifferentialCompression\"`*"]
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
impl ::core::fmt::Debug for RdcSignaturePointer {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("RdcSignaturePointer").field("m_Size", &self.m_Size).field("m_Used", &self.m_Used).field("m_Data", &self.m_Data).finish()
    }
}
impl ::windows::core::TypeKind for RdcSignaturePointer {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for RdcSignaturePointer {
    fn eq(&self, other: &Self) -> bool {
        self.m_Size == other.m_Size && self.m_Used == other.m_Used && self.m_Data == other.m_Data
    }
}
impl ::core::cmp::Eq for RdcSignaturePointer {}
impl ::core::default::Default for RdcSignaturePointer {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_RemoteDifferentialCompression\"`*"]
pub struct SimilarityData {
    pub m_Data: [u8; 16],
}
impl ::core::marker::Copy for SimilarityData {}
impl ::core::clone::Clone for SimilarityData {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for SimilarityData {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SimilarityData").field("m_Data", &self.m_Data).finish()
    }
}
impl ::windows::core::TypeKind for SimilarityData {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for SimilarityData {
    fn eq(&self, other: &Self) -> bool {
        self.m_Data == other.m_Data
    }
}
impl ::core::cmp::Eq for SimilarityData {}
impl ::core::default::Default for SimilarityData {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_RemoteDifferentialCompression\"`*"]
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
impl ::core::fmt::Debug for SimilarityDumpData {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SimilarityDumpData").field("m_FileIndex", &self.m_FileIndex).field("m_Data", &self.m_Data).finish()
    }
}
impl ::windows::core::TypeKind for SimilarityDumpData {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for SimilarityDumpData {
    fn eq(&self, other: &Self) -> bool {
        self.m_FileIndex == other.m_FileIndex && self.m_Data == other.m_Data
    }
}
impl ::core::cmp::Eq for SimilarityDumpData {}
impl ::core::default::Default for SimilarityDumpData {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_RemoteDifferentialCompression\"`*"]
pub struct SimilarityFileId {
    pub m_FileId: [u8; 32],
}
impl ::core::marker::Copy for SimilarityFileId {}
impl ::core::clone::Clone for SimilarityFileId {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for SimilarityFileId {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SimilarityFileId").field("m_FileId", &self.m_FileId).finish()
    }
}
impl ::windows::core::TypeKind for SimilarityFileId {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for SimilarityFileId {
    fn eq(&self, other: &Self) -> bool {
        self.m_FileId == other.m_FileId
    }
}
impl ::core::cmp::Eq for SimilarityFileId {}
impl ::core::default::Default for SimilarityFileId {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_RemoteDifferentialCompression\"`*"]
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
impl ::core::fmt::Debug for SimilarityMappedViewInfo {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SimilarityMappedViewInfo").field("m_Data", &self.m_Data).field("m_Length", &self.m_Length).finish()
    }
}
impl ::windows::core::TypeKind for SimilarityMappedViewInfo {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for SimilarityMappedViewInfo {
    fn eq(&self, other: &Self) -> bool {
        self.m_Data == other.m_Data && self.m_Length == other.m_Length
    }
}
impl ::core::cmp::Eq for SimilarityMappedViewInfo {}
impl ::core::default::Default for SimilarityMappedViewInfo {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
