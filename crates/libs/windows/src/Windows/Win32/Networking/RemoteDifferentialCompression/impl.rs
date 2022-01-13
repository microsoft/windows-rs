pub trait IFindSimilarResultsImpl: Sized {
    fn GetSize(&mut self) -> ::windows::core::Result<u32>;
    fn GetNextFileId(&mut self, numtraitsmatched: *mut u32, similarityfileid: *mut SimilarityFileId) -> ::windows::core::Result<()>;
}
impl IFindSimilarResultsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFindSimilarResultsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IFindSimilarResultsVtbl {
        unsafe extern "system" fn GetSize<Impl: IFindSimilarResultsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, size: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetSize() {
                ::core::result::Result::Ok(ok__) => {
                    *size = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetNextFileId<Impl: IFindSimilarResultsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, numtraitsmatched: *mut u32, similarityfileid: *mut SimilarityFileId) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetNextFileId(::core::mem::transmute_copy(&numtraitsmatched), ::core::mem::transmute_copy(&similarityfileid)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            GetSize: GetSize::<Impl, IMPL_OFFSET>,
            GetNextFileId: GetNextFileId::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFindSimilarResults as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IRdcComparatorImpl: Sized {
    fn Process(&mut self, endofinput: super::super::Foundation::BOOL, endofoutput: *mut super::super::Foundation::BOOL, inputbuffer: *mut RdcBufferPointer, outputbuffer: *mut RdcNeedPointer, rdc_errorcode: *mut RDC_ErrorCode) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IRdcComparatorVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRdcComparatorImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IRdcComparatorVtbl {
        unsafe extern "system" fn Process<Impl: IRdcComparatorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, endofinput: super::super::Foundation::BOOL, endofoutput: *mut super::super::Foundation::BOOL, inputbuffer: *mut RdcBufferPointer, outputbuffer: *mut RdcNeedPointer, rdc_errorcode: *mut RDC_ErrorCode) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Process(::core::mem::transmute_copy(&endofinput), ::core::mem::transmute_copy(&endofoutput), ::core::mem::transmute_copy(&inputbuffer), ::core::mem::transmute_copy(&outputbuffer), ::core::mem::transmute_copy(&rdc_errorcode)).into()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), Process: Process::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRdcComparator as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IRdcFileReaderImpl: Sized {
    fn GetFileSize(&mut self) -> ::windows::core::Result<u64>;
    fn Read(&mut self, offsetfilestart: u64, bytestoread: u32, bytesactuallyread: *mut u32, buffer: *mut u8, eof: *mut super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn GetFilePosition(&mut self) -> ::windows::core::Result<u64>;
}
#[cfg(feature = "Win32_Foundation")]
impl IRdcFileReaderVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRdcFileReaderImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IRdcFileReaderVtbl {
        unsafe extern "system" fn GetFileSize<Impl: IRdcFileReaderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, filesize: *mut u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetFileSize() {
                ::core::result::Result::Ok(ok__) => {
                    *filesize = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Read<Impl: IRdcFileReaderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, offsetfilestart: u64, bytestoread: u32, bytesactuallyread: *mut u32, buffer: *mut u8, eof: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Read(::core::mem::transmute_copy(&offsetfilestart), ::core::mem::transmute_copy(&bytestoread), ::core::mem::transmute_copy(&bytesactuallyread), ::core::mem::transmute_copy(&buffer), ::core::mem::transmute_copy(&eof)).into()
        }
        unsafe extern "system" fn GetFilePosition<Impl: IRdcFileReaderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, offsetfromstart: *mut u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetFilePosition() {
                ::core::result::Result::Ok(ok__) => {
                    *offsetfromstart = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            GetFileSize: GetFileSize::<Impl, IMPL_OFFSET>,
            Read: Read::<Impl, IMPL_OFFSET>,
            GetFilePosition: GetFilePosition::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRdcFileReader as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IRdcFileWriterImpl: Sized + IRdcFileReaderImpl {
    fn Write(&mut self, offsetfilestart: u64, bytestowrite: u32) -> ::windows::core::Result<u8>;
    fn Truncate(&mut self) -> ::windows::core::Result<()>;
    fn DeleteOnClose(&mut self) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IRdcFileWriterVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRdcFileWriterImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IRdcFileWriterVtbl {
        unsafe extern "system" fn Write<Impl: IRdcFileWriterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, offsetfilestart: u64, bytestowrite: u32, buffer: *mut u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Write(::core::mem::transmute_copy(&offsetfilestart), ::core::mem::transmute_copy(&bytestowrite)) {
                ::core::result::Result::Ok(ok__) => {
                    *buffer = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Truncate<Impl: IRdcFileWriterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Truncate().into()
        }
        unsafe extern "system" fn DeleteOnClose<Impl: IRdcFileWriterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).DeleteOnClose().into()
        }
        Self {
            base: IRdcFileReaderVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            Write: Write::<Impl, IMPL_OFFSET>,
            Truncate: Truncate::<Impl, IMPL_OFFSET>,
            DeleteOnClose: DeleteOnClose::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRdcFileWriter as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IRdcGeneratorImpl: Sized {
    fn GetGeneratorParameters(&mut self, level: u32) -> ::windows::core::Result<IRdcGeneratorParameters>;
    fn Process(&mut self, endofinput: super::super::Foundation::BOOL, endofoutput: *mut super::super::Foundation::BOOL, inputbuffer: *mut RdcBufferPointer, depth: u32, outputbuffers: *mut *mut RdcBufferPointer, rdc_errorcode: *mut RDC_ErrorCode) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IRdcGeneratorVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRdcGeneratorImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IRdcGeneratorVtbl {
        unsafe extern "system" fn GetGeneratorParameters<Impl: IRdcGeneratorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, level: u32, igeneratorparameters: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetGeneratorParameters(::core::mem::transmute_copy(&level)) {
                ::core::result::Result::Ok(ok__) => {
                    *igeneratorparameters = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Process<Impl: IRdcGeneratorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, endofinput: super::super::Foundation::BOOL, endofoutput: *mut super::super::Foundation::BOOL, inputbuffer: *mut RdcBufferPointer, depth: u32, outputbuffers: *mut *mut RdcBufferPointer, rdc_errorcode: *mut RDC_ErrorCode) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Process(::core::mem::transmute_copy(&endofinput), ::core::mem::transmute_copy(&endofoutput), ::core::mem::transmute_copy(&inputbuffer), ::core::mem::transmute_copy(&depth), ::core::mem::transmute_copy(&outputbuffers), ::core::mem::transmute_copy(&rdc_errorcode)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            GetGeneratorParameters: GetGeneratorParameters::<Impl, IMPL_OFFSET>,
            Process: Process::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRdcGenerator as ::windows::core::Interface>::IID
    }
}
pub trait IRdcGeneratorFilterMaxParametersImpl: Sized {
    fn GetHorizonSize(&mut self) -> ::windows::core::Result<u32>;
    fn SetHorizonSize(&mut self, horizonsize: u32) -> ::windows::core::Result<()>;
    fn GetHashWindowSize(&mut self) -> ::windows::core::Result<u32>;
    fn SetHashWindowSize(&mut self, hashwindowsize: u32) -> ::windows::core::Result<()>;
}
impl IRdcGeneratorFilterMaxParametersVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRdcGeneratorFilterMaxParametersImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IRdcGeneratorFilterMaxParametersVtbl {
        unsafe extern "system" fn GetHorizonSize<Impl: IRdcGeneratorFilterMaxParametersImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, horizonsize: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetHorizonSize() {
                ::core::result::Result::Ok(ok__) => {
                    *horizonsize = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetHorizonSize<Impl: IRdcGeneratorFilterMaxParametersImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, horizonsize: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetHorizonSize(::core::mem::transmute_copy(&horizonsize)).into()
        }
        unsafe extern "system" fn GetHashWindowSize<Impl: IRdcGeneratorFilterMaxParametersImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hashwindowsize: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetHashWindowSize() {
                ::core::result::Result::Ok(ok__) => {
                    *hashwindowsize = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetHashWindowSize<Impl: IRdcGeneratorFilterMaxParametersImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hashwindowsize: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetHashWindowSize(::core::mem::transmute_copy(&hashwindowsize)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            GetHorizonSize: GetHorizonSize::<Impl, IMPL_OFFSET>,
            SetHorizonSize: SetHorizonSize::<Impl, IMPL_OFFSET>,
            GetHashWindowSize: GetHashWindowSize::<Impl, IMPL_OFFSET>,
            SetHashWindowSize: SetHashWindowSize::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRdcGeneratorFilterMaxParameters as ::windows::core::Interface>::IID
    }
}
pub trait IRdcGeneratorParametersImpl: Sized {
    fn GetGeneratorParametersType(&mut self) -> ::windows::core::Result<GeneratorParametersType>;
    fn GetParametersVersion(&mut self, currentversion: *mut u32, minimumcompatibleappversion: *mut u32) -> ::windows::core::Result<()>;
    fn GetSerializeSize(&mut self) -> ::windows::core::Result<u32>;
    fn Serialize(&mut self, size: u32, parametersblob: *mut u8, byteswritten: *mut u32) -> ::windows::core::Result<()>;
}
impl IRdcGeneratorParametersVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRdcGeneratorParametersImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IRdcGeneratorParametersVtbl {
        unsafe extern "system" fn GetGeneratorParametersType<Impl: IRdcGeneratorParametersImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, parameterstype: *mut GeneratorParametersType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetGeneratorParametersType() {
                ::core::result::Result::Ok(ok__) => {
                    *parameterstype = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetParametersVersion<Impl: IRdcGeneratorParametersImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, currentversion: *mut u32, minimumcompatibleappversion: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetParametersVersion(::core::mem::transmute_copy(&currentversion), ::core::mem::transmute_copy(&minimumcompatibleappversion)).into()
        }
        unsafe extern "system" fn GetSerializeSize<Impl: IRdcGeneratorParametersImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, size: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetSerializeSize() {
                ::core::result::Result::Ok(ok__) => {
                    *size = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Serialize<Impl: IRdcGeneratorParametersImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, size: u32, parametersblob: *mut u8, byteswritten: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Serialize(::core::mem::transmute_copy(&size), ::core::mem::transmute_copy(&parametersblob), ::core::mem::transmute_copy(&byteswritten)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            GetGeneratorParametersType: GetGeneratorParametersType::<Impl, IMPL_OFFSET>,
            GetParametersVersion: GetParametersVersion::<Impl, IMPL_OFFSET>,
            GetSerializeSize: GetSerializeSize::<Impl, IMPL_OFFSET>,
            Serialize: Serialize::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRdcGeneratorParameters as ::windows::core::Interface>::IID
    }
}
pub trait IRdcLibraryImpl: Sized {
    fn ComputeDefaultRecursionDepth(&mut self, filesize: u64) -> ::windows::core::Result<u32>;
    fn CreateGeneratorParameters(&mut self, parameterstype: GeneratorParametersType, level: u32) -> ::windows::core::Result<IRdcGeneratorParameters>;
    fn OpenGeneratorParameters(&mut self, size: u32, parametersblob: *const u8) -> ::windows::core::Result<IRdcGeneratorParameters>;
    fn CreateGenerator(&mut self, depth: u32, igeneratorparametersarray: *const ::core::option::Option<IRdcGeneratorParameters>) -> ::windows::core::Result<IRdcGenerator>;
    fn CreateComparator(&mut self, iseedsignaturesfile: ::core::option::Option<IRdcFileReader>, comparatorbuffersize: u32) -> ::windows::core::Result<IRdcComparator>;
    fn CreateSignatureReader(&mut self, ifilereader: ::core::option::Option<IRdcFileReader>) -> ::windows::core::Result<IRdcSignatureReader>;
    fn GetRDCVersion(&mut self, currentversion: *mut u32, minimumcompatibleappversion: *mut u32) -> ::windows::core::Result<()>;
}
impl IRdcLibraryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRdcLibraryImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IRdcLibraryVtbl {
        unsafe extern "system" fn ComputeDefaultRecursionDepth<Impl: IRdcLibraryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, filesize: u64, depth: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ComputeDefaultRecursionDepth(::core::mem::transmute_copy(&filesize)) {
                ::core::result::Result::Ok(ok__) => {
                    *depth = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateGeneratorParameters<Impl: IRdcLibraryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, parameterstype: GeneratorParametersType, level: u32, igeneratorparameters: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateGeneratorParameters(::core::mem::transmute_copy(&parameterstype), ::core::mem::transmute_copy(&level)) {
                ::core::result::Result::Ok(ok__) => {
                    *igeneratorparameters = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OpenGeneratorParameters<Impl: IRdcLibraryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, size: u32, parametersblob: *const u8, igeneratorparameters: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OpenGeneratorParameters(::core::mem::transmute_copy(&size), ::core::mem::transmute_copy(&parametersblob)) {
                ::core::result::Result::Ok(ok__) => {
                    *igeneratorparameters = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateGenerator<Impl: IRdcLibraryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, depth: u32, igeneratorparametersarray: *const ::windows::core::RawPtr, igenerator: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateGenerator(::core::mem::transmute_copy(&depth), ::core::mem::transmute_copy(&igeneratorparametersarray)) {
                ::core::result::Result::Ok(ok__) => {
                    *igenerator = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateComparator<Impl: IRdcLibraryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, iseedsignaturesfile: ::windows::core::RawPtr, comparatorbuffersize: u32, icomparator: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateComparator(::core::mem::transmute(&iseedsignaturesfile), ::core::mem::transmute_copy(&comparatorbuffersize)) {
                ::core::result::Result::Ok(ok__) => {
                    *icomparator = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateSignatureReader<Impl: IRdcLibraryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ifilereader: ::windows::core::RawPtr, isignaturereader: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateSignatureReader(::core::mem::transmute(&ifilereader)) {
                ::core::result::Result::Ok(ok__) => {
                    *isignaturereader = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetRDCVersion<Impl: IRdcLibraryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, currentversion: *mut u32, minimumcompatibleappversion: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetRDCVersion(::core::mem::transmute_copy(&currentversion), ::core::mem::transmute_copy(&minimumcompatibleappversion)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            ComputeDefaultRecursionDepth: ComputeDefaultRecursionDepth::<Impl, IMPL_OFFSET>,
            CreateGeneratorParameters: CreateGeneratorParameters::<Impl, IMPL_OFFSET>,
            OpenGeneratorParameters: OpenGeneratorParameters::<Impl, IMPL_OFFSET>,
            CreateGenerator: CreateGenerator::<Impl, IMPL_OFFSET>,
            CreateComparator: CreateComparator::<Impl, IMPL_OFFSET>,
            CreateSignatureReader: CreateSignatureReader::<Impl, IMPL_OFFSET>,
            GetRDCVersion: GetRDCVersion::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRdcLibrary as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IRdcSignatureReaderImpl: Sized {
    fn ReadHeader(&mut self) -> ::windows::core::Result<RDC_ErrorCode>;
    fn ReadSignatures(&mut self, rdcsignaturepointer: *mut RdcSignaturePointer, endofoutput: *mut super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IRdcSignatureReaderVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRdcSignatureReaderImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IRdcSignatureReaderVtbl {
        unsafe extern "system" fn ReadHeader<Impl: IRdcSignatureReaderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rdc_errorcode: *mut RDC_ErrorCode) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ReadHeader() {
                ::core::result::Result::Ok(ok__) => {
                    *rdc_errorcode = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ReadSignatures<Impl: IRdcSignatureReaderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rdcsignaturepointer: *mut RdcSignaturePointer, endofoutput: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ReadSignatures(::core::mem::transmute_copy(&rdcsignaturepointer), ::core::mem::transmute_copy(&endofoutput)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            ReadHeader: ReadHeader::<Impl, IMPL_OFFSET>,
            ReadSignatures: ReadSignatures::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRdcSignatureReader as ::windows::core::Interface>::IID
    }
}
pub trait IRdcSimilarityGeneratorImpl: Sized {
    fn EnableSimilarity(&mut self) -> ::windows::core::Result<()>;
    fn Results(&mut self) -> ::windows::core::Result<SimilarityData>;
}
impl IRdcSimilarityGeneratorVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRdcSimilarityGeneratorImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IRdcSimilarityGeneratorVtbl {
        unsafe extern "system" fn EnableSimilarity<Impl: IRdcSimilarityGeneratorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).EnableSimilarity().into()
        }
        unsafe extern "system" fn Results<Impl: IRdcSimilarityGeneratorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, similaritydata: *mut SimilarityData) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Results() {
                ::core::result::Result::Ok(ok__) => {
                    *similaritydata = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            EnableSimilarity: EnableSimilarity::<Impl, IMPL_OFFSET>,
            Results: Results::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRdcSimilarityGenerator as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait ISimilarityImpl: Sized {
    fn CreateTable(&mut self, path: super::super::Foundation::PWSTR, truncate: super::super::Foundation::BOOL, securitydescriptor: *const u8, recordsize: u32) -> ::windows::core::Result<RdcCreatedTables>;
    fn CreateTableIndirect(&mut self, mapping: ::core::option::Option<ISimilarityTraitsMapping>, fileidfile: ::core::option::Option<IRdcFileWriter>, truncate: super::super::Foundation::BOOL, recordsize: u32) -> ::windows::core::Result<RdcCreatedTables>;
    fn CloseTable(&mut self, isvalid: super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn Append(&mut self, similarityfileid: *const SimilarityFileId, similaritydata: *const SimilarityData) -> ::windows::core::Result<()>;
    fn FindSimilarFileId(&mut self, similaritydata: *const SimilarityData, numberofmatchesrequired: u16, resultssize: u32) -> ::windows::core::Result<IFindSimilarResults>;
    fn CopyAndSwap(&mut self, newsimilaritytables: ::core::option::Option<ISimilarity>, reportprogress: ::core::option::Option<ISimilarityReportProgress>) -> ::windows::core::Result<()>;
    fn GetRecordCount(&mut self) -> ::windows::core::Result<u32>;
}
#[cfg(feature = "Win32_Foundation")]
impl ISimilarityVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISimilarityImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISimilarityVtbl {
        unsafe extern "system" fn CreateTable<Impl: ISimilarityImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, path: super::super::Foundation::PWSTR, truncate: super::super::Foundation::BOOL, securitydescriptor: *const u8, recordsize: u32, isnew: *mut RdcCreatedTables) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateTable(::core::mem::transmute_copy(&path), ::core::mem::transmute_copy(&truncate), ::core::mem::transmute_copy(&securitydescriptor), ::core::mem::transmute_copy(&recordsize)) {
                ::core::result::Result::Ok(ok__) => {
                    *isnew = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateTableIndirect<Impl: ISimilarityImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, mapping: ::windows::core::RawPtr, fileidfile: ::windows::core::RawPtr, truncate: super::super::Foundation::BOOL, recordsize: u32, isnew: *mut RdcCreatedTables) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateTableIndirect(::core::mem::transmute(&mapping), ::core::mem::transmute(&fileidfile), ::core::mem::transmute_copy(&truncate), ::core::mem::transmute_copy(&recordsize)) {
                ::core::result::Result::Ok(ok__) => {
                    *isnew = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CloseTable<Impl: ISimilarityImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, isvalid: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).CloseTable(::core::mem::transmute_copy(&isvalid)).into()
        }
        unsafe extern "system" fn Append<Impl: ISimilarityImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, similarityfileid: *const SimilarityFileId, similaritydata: *const SimilarityData) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Append(::core::mem::transmute_copy(&similarityfileid), ::core::mem::transmute_copy(&similaritydata)).into()
        }
        unsafe extern "system" fn FindSimilarFileId<Impl: ISimilarityImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, similaritydata: *const SimilarityData, numberofmatchesrequired: u16, resultssize: u32, findsimilarresults: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FindSimilarFileId(::core::mem::transmute_copy(&similaritydata), ::core::mem::transmute_copy(&numberofmatchesrequired), ::core::mem::transmute_copy(&resultssize)) {
                ::core::result::Result::Ok(ok__) => {
                    *findsimilarresults = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CopyAndSwap<Impl: ISimilarityImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newsimilaritytables: ::windows::core::RawPtr, reportprogress: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).CopyAndSwap(::core::mem::transmute(&newsimilaritytables), ::core::mem::transmute(&reportprogress)).into()
        }
        unsafe extern "system" fn GetRecordCount<Impl: ISimilarityImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, recordcount: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetRecordCount() {
                ::core::result::Result::Ok(ok__) => {
                    *recordcount = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            CreateTable: CreateTable::<Impl, IMPL_OFFSET>,
            CreateTableIndirect: CreateTableIndirect::<Impl, IMPL_OFFSET>,
            CloseTable: CloseTable::<Impl, IMPL_OFFSET>,
            Append: Append::<Impl, IMPL_OFFSET>,
            FindSimilarFileId: FindSimilarFileId::<Impl, IMPL_OFFSET>,
            CopyAndSwap: CopyAndSwap::<Impl, IMPL_OFFSET>,
            GetRecordCount: GetRecordCount::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISimilarity as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait ISimilarityFileIdTableImpl: Sized {
    fn CreateTable(&mut self, path: super::super::Foundation::PWSTR, truncate: super::super::Foundation::BOOL, securitydescriptor: *const u8, recordsize: u32) -> ::windows::core::Result<RdcCreatedTables>;
    fn CreateTableIndirect(&mut self, fileidfile: ::core::option::Option<IRdcFileWriter>, truncate: super::super::Foundation::BOOL, recordsize: u32) -> ::windows::core::Result<RdcCreatedTables>;
    fn CloseTable(&mut self, isvalid: super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn Append(&mut self, similarityfileid: *const SimilarityFileId) -> ::windows::core::Result<u32>;
    fn Lookup(&mut self, similarityfileindex: u32) -> ::windows::core::Result<SimilarityFileId>;
    fn Invalidate(&mut self, similarityfileindex: u32) -> ::windows::core::Result<()>;
    fn GetRecordCount(&mut self) -> ::windows::core::Result<u32>;
}
#[cfg(feature = "Win32_Foundation")]
impl ISimilarityFileIdTableVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISimilarityFileIdTableImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISimilarityFileIdTableVtbl {
        unsafe extern "system" fn CreateTable<Impl: ISimilarityFileIdTableImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, path: super::super::Foundation::PWSTR, truncate: super::super::Foundation::BOOL, securitydescriptor: *const u8, recordsize: u32, isnew: *mut RdcCreatedTables) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateTable(::core::mem::transmute_copy(&path), ::core::mem::transmute_copy(&truncate), ::core::mem::transmute_copy(&securitydescriptor), ::core::mem::transmute_copy(&recordsize)) {
                ::core::result::Result::Ok(ok__) => {
                    *isnew = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateTableIndirect<Impl: ISimilarityFileIdTableImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fileidfile: ::windows::core::RawPtr, truncate: super::super::Foundation::BOOL, recordsize: u32, isnew: *mut RdcCreatedTables) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateTableIndirect(::core::mem::transmute(&fileidfile), ::core::mem::transmute_copy(&truncate), ::core::mem::transmute_copy(&recordsize)) {
                ::core::result::Result::Ok(ok__) => {
                    *isnew = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CloseTable<Impl: ISimilarityFileIdTableImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, isvalid: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).CloseTable(::core::mem::transmute_copy(&isvalid)).into()
        }
        unsafe extern "system" fn Append<Impl: ISimilarityFileIdTableImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, similarityfileid: *const SimilarityFileId, similarityfileindex: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Append(::core::mem::transmute_copy(&similarityfileid)) {
                ::core::result::Result::Ok(ok__) => {
                    *similarityfileindex = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Lookup<Impl: ISimilarityFileIdTableImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, similarityfileindex: u32, similarityfileid: *mut SimilarityFileId) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Lookup(::core::mem::transmute_copy(&similarityfileindex)) {
                ::core::result::Result::Ok(ok__) => {
                    *similarityfileid = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Invalidate<Impl: ISimilarityFileIdTableImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, similarityfileindex: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Invalidate(::core::mem::transmute_copy(&similarityfileindex)).into()
        }
        unsafe extern "system" fn GetRecordCount<Impl: ISimilarityFileIdTableImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, recordcount: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetRecordCount() {
                ::core::result::Result::Ok(ok__) => {
                    *recordcount = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            CreateTable: CreateTable::<Impl, IMPL_OFFSET>,
            CreateTableIndirect: CreateTableIndirect::<Impl, IMPL_OFFSET>,
            CloseTable: CloseTable::<Impl, IMPL_OFFSET>,
            Append: Append::<Impl, IMPL_OFFSET>,
            Lookup: Lookup::<Impl, IMPL_OFFSET>,
            Invalidate: Invalidate::<Impl, IMPL_OFFSET>,
            GetRecordCount: GetRecordCount::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISimilarityFileIdTable as ::windows::core::Interface>::IID
    }
}
pub trait ISimilarityReportProgressImpl: Sized {
    fn ReportProgress(&mut self, percentcompleted: u32) -> ::windows::core::Result<()>;
}
impl ISimilarityReportProgressVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISimilarityReportProgressImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISimilarityReportProgressVtbl {
        unsafe extern "system" fn ReportProgress<Impl: ISimilarityReportProgressImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, percentcompleted: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ReportProgress(::core::mem::transmute_copy(&percentcompleted)).into()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), ReportProgress: ReportProgress::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISimilarityReportProgress as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait ISimilarityTableDumpStateImpl: Sized {
    fn GetNextData(&mut self, resultssize: u32, resultsused: *mut u32, eof: *mut super::super::Foundation::BOOL, results: *mut SimilarityDumpData) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ISimilarityTableDumpStateVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISimilarityTableDumpStateImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISimilarityTableDumpStateVtbl {
        unsafe extern "system" fn GetNextData<Impl: ISimilarityTableDumpStateImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, resultssize: u32, resultsused: *mut u32, eof: *mut super::super::Foundation::BOOL, results: *mut SimilarityDumpData) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetNextData(::core::mem::transmute_copy(&resultssize), ::core::mem::transmute_copy(&resultsused), ::core::mem::transmute_copy(&eof), ::core::mem::transmute_copy(&results)).into()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), GetNextData: GetNextData::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISimilarityTableDumpState as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait ISimilarityTraitsMappedViewImpl: Sized {
    fn Flush(&mut self) -> ::windows::core::Result<()>;
    fn Unmap(&mut self) -> ::windows::core::Result<()>;
    fn Get(&mut self, index: u64, dirty: super::super::Foundation::BOOL, numelements: u32) -> ::windows::core::Result<SimilarityMappedViewInfo>;
    fn GetView(&mut self, mappedpagebegin: *mut *mut u8, mappedpageend: *mut *mut u8);
}
#[cfg(feature = "Win32_Foundation")]
impl ISimilarityTraitsMappedViewVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISimilarityTraitsMappedViewImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISimilarityTraitsMappedViewVtbl {
        unsafe extern "system" fn Flush<Impl: ISimilarityTraitsMappedViewImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Flush().into()
        }
        unsafe extern "system" fn Unmap<Impl: ISimilarityTraitsMappedViewImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Unmap().into()
        }
        unsafe extern "system" fn Get<Impl: ISimilarityTraitsMappedViewImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u64, dirty: super::super::Foundation::BOOL, numelements: u32, viewinfo: *mut SimilarityMappedViewInfo) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Get(::core::mem::transmute_copy(&index), ::core::mem::transmute_copy(&dirty), ::core::mem::transmute_copy(&numelements)) {
                ::core::result::Result::Ok(ok__) => {
                    *viewinfo = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetView<Impl: ISimilarityTraitsMappedViewImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, mappedpagebegin: *mut *mut u8, mappedpageend: *mut *mut u8) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetView(::core::mem::transmute_copy(&mappedpagebegin), ::core::mem::transmute_copy(&mappedpageend))
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            Flush: Flush::<Impl, IMPL_OFFSET>,
            Unmap: Unmap::<Impl, IMPL_OFFSET>,
            Get: Get::<Impl, IMPL_OFFSET>,
            GetView: GetView::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISimilarityTraitsMappedView as ::windows::core::Interface>::IID
    }
}
pub trait ISimilarityTraitsMappingImpl: Sized {
    fn CloseMapping(&mut self);
    fn SetFileSize(&mut self, filesize: u64) -> ::windows::core::Result<()>;
    fn GetFileSize(&mut self) -> ::windows::core::Result<u64>;
    fn OpenMapping(&mut self, accessmode: RdcMappingAccessMode, begin: u64, end: u64) -> ::windows::core::Result<u64>;
    fn ResizeMapping(&mut self, accessmode: RdcMappingAccessMode, begin: u64, end: u64) -> ::windows::core::Result<u64>;
    fn GetPageSize(&mut self, pagesize: *mut u32);
    fn CreateView(&mut self, minimummappedpages: u32, accessmode: RdcMappingAccessMode) -> ::windows::core::Result<ISimilarityTraitsMappedView>;
}
impl ISimilarityTraitsMappingVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISimilarityTraitsMappingImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISimilarityTraitsMappingVtbl {
        unsafe extern "system" fn CloseMapping<Impl: ISimilarityTraitsMappingImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).CloseMapping()
        }
        unsafe extern "system" fn SetFileSize<Impl: ISimilarityTraitsMappingImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, filesize: u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetFileSize(::core::mem::transmute_copy(&filesize)).into()
        }
        unsafe extern "system" fn GetFileSize<Impl: ISimilarityTraitsMappingImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, filesize: *mut u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetFileSize() {
                ::core::result::Result::Ok(ok__) => {
                    *filesize = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OpenMapping<Impl: ISimilarityTraitsMappingImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, accessmode: RdcMappingAccessMode, begin: u64, end: u64, actualend: *mut u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OpenMapping(::core::mem::transmute_copy(&accessmode), ::core::mem::transmute_copy(&begin), ::core::mem::transmute_copy(&end)) {
                ::core::result::Result::Ok(ok__) => {
                    *actualend = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ResizeMapping<Impl: ISimilarityTraitsMappingImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, accessmode: RdcMappingAccessMode, begin: u64, end: u64, actualend: *mut u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ResizeMapping(::core::mem::transmute_copy(&accessmode), ::core::mem::transmute_copy(&begin), ::core::mem::transmute_copy(&end)) {
                ::core::result::Result::Ok(ok__) => {
                    *actualend = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPageSize<Impl: ISimilarityTraitsMappingImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pagesize: *mut u32) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetPageSize(::core::mem::transmute_copy(&pagesize))
        }
        unsafe extern "system" fn CreateView<Impl: ISimilarityTraitsMappingImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, minimummappedpages: u32, accessmode: RdcMappingAccessMode, mappedview: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateView(::core::mem::transmute_copy(&minimummappedpages), ::core::mem::transmute_copy(&accessmode)) {
                ::core::result::Result::Ok(ok__) => {
                    *mappedview = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            CloseMapping: CloseMapping::<Impl, IMPL_OFFSET>,
            SetFileSize: SetFileSize::<Impl, IMPL_OFFSET>,
            GetFileSize: GetFileSize::<Impl, IMPL_OFFSET>,
            OpenMapping: OpenMapping::<Impl, IMPL_OFFSET>,
            ResizeMapping: ResizeMapping::<Impl, IMPL_OFFSET>,
            GetPageSize: GetPageSize::<Impl, IMPL_OFFSET>,
            CreateView: CreateView::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISimilarityTraitsMapping as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait ISimilarityTraitsTableImpl: Sized {
    fn CreateTable(&mut self, path: super::super::Foundation::PWSTR, truncate: super::super::Foundation::BOOL, securitydescriptor: *const u8) -> ::windows::core::Result<RdcCreatedTables>;
    fn CreateTableIndirect(&mut self, mapping: ::core::option::Option<ISimilarityTraitsMapping>, truncate: super::super::Foundation::BOOL) -> ::windows::core::Result<RdcCreatedTables>;
    fn CloseTable(&mut self, isvalid: super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn Append(&mut self, data: *const SimilarityData, fileindex: u32) -> ::windows::core::Result<()>;
    fn FindSimilarFileIndex(&mut self, similaritydata: *const SimilarityData, numberofmatchesrequired: u16, findsimilarfileindexresults: *mut FindSimilarFileIndexResults, resultssize: u32, resultsused: *mut u32) -> ::windows::core::Result<()>;
    fn BeginDump(&mut self) -> ::windows::core::Result<ISimilarityTableDumpState>;
    fn GetLastIndex(&mut self) -> ::windows::core::Result<u32>;
}
#[cfg(feature = "Win32_Foundation")]
impl ISimilarityTraitsTableVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISimilarityTraitsTableImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISimilarityTraitsTableVtbl {
        unsafe extern "system" fn CreateTable<Impl: ISimilarityTraitsTableImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, path: super::super::Foundation::PWSTR, truncate: super::super::Foundation::BOOL, securitydescriptor: *const u8, isnew: *mut RdcCreatedTables) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateTable(::core::mem::transmute_copy(&path), ::core::mem::transmute_copy(&truncate), ::core::mem::transmute_copy(&securitydescriptor)) {
                ::core::result::Result::Ok(ok__) => {
                    *isnew = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateTableIndirect<Impl: ISimilarityTraitsTableImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, mapping: ::windows::core::RawPtr, truncate: super::super::Foundation::BOOL, isnew: *mut RdcCreatedTables) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateTableIndirect(::core::mem::transmute(&mapping), ::core::mem::transmute_copy(&truncate)) {
                ::core::result::Result::Ok(ok__) => {
                    *isnew = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CloseTable<Impl: ISimilarityTraitsTableImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, isvalid: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).CloseTable(::core::mem::transmute_copy(&isvalid)).into()
        }
        unsafe extern "system" fn Append<Impl: ISimilarityTraitsTableImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, data: *const SimilarityData, fileindex: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Append(::core::mem::transmute_copy(&data), ::core::mem::transmute_copy(&fileindex)).into()
        }
        unsafe extern "system" fn FindSimilarFileIndex<Impl: ISimilarityTraitsTableImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, similaritydata: *const SimilarityData, numberofmatchesrequired: u16, findsimilarfileindexresults: *mut FindSimilarFileIndexResults, resultssize: u32, resultsused: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).FindSimilarFileIndex(::core::mem::transmute_copy(&similaritydata), ::core::mem::transmute_copy(&numberofmatchesrequired), ::core::mem::transmute_copy(&findsimilarfileindexresults), ::core::mem::transmute_copy(&resultssize), ::core::mem::transmute_copy(&resultsused)).into()
        }
        unsafe extern "system" fn BeginDump<Impl: ISimilarityTraitsTableImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, similaritytabledumpstate: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).BeginDump() {
                ::core::result::Result::Ok(ok__) => {
                    *similaritytabledumpstate = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetLastIndex<Impl: ISimilarityTraitsTableImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fileindex: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetLastIndex() {
                ::core::result::Result::Ok(ok__) => {
                    *fileindex = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            CreateTable: CreateTable::<Impl, IMPL_OFFSET>,
            CreateTableIndirect: CreateTableIndirect::<Impl, IMPL_OFFSET>,
            CloseTable: CloseTable::<Impl, IMPL_OFFSET>,
            Append: Append::<Impl, IMPL_OFFSET>,
            FindSimilarFileIndex: FindSimilarFileIndex::<Impl, IMPL_OFFSET>,
            BeginDump: BeginDump::<Impl, IMPL_OFFSET>,
            GetLastIndex: GetLastIndex::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISimilarityTraitsTable as ::windows::core::Interface>::IID
    }
}
