pub trait IFindSimilarResultsImpl: Sized {
    fn GetSize();
    fn GetNextFileId();
}
impl IFindSimilarResultsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFindSimilarResultsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IFindSimilarResultsVtbl {
        unsafe extern "system" fn GetSize<Impl: IFindSimilarResultsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, size: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetNextFileId<Impl: IFindSimilarResultsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, numtraitsmatched: *mut u32, similarityfileid: *mut SimilarityFileId) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetSize::<Impl, IMPL_OFFSET>, GetNextFileId::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFindSimilarResults as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IRdcComparatorImpl: Sized {
    fn Process();
}
#[cfg(feature = "Win32_Foundation")]
impl IRdcComparatorVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRdcComparatorImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IRdcComparatorVtbl {
        unsafe extern "system" fn Process<Impl: IRdcComparatorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, endofinput: super::super::Foundation::BOOL, endofoutput: *mut super::super::Foundation::BOOL, inputbuffer: *mut RdcBufferPointer, outputbuffer: *mut RdcNeedPointer, rdc_errorcode: *mut RDC_ErrorCode) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, Process::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRdcComparator as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IRdcFileReaderImpl: Sized {
    fn GetFileSize();
    fn Read();
    fn GetFilePosition();
}
#[cfg(feature = "Win32_Foundation")]
impl IRdcFileReaderVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRdcFileReaderImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IRdcFileReaderVtbl {
        unsafe extern "system" fn GetFileSize<Impl: IRdcFileReaderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, filesize: *mut u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Read<Impl: IRdcFileReaderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, offsetfilestart: u64, bytestoread: u32, bytesactuallyread: *mut u32, buffer: *mut u8, eof: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetFilePosition<Impl: IRdcFileReaderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, offsetfromstart: *mut u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetFileSize::<Impl, IMPL_OFFSET>, Read::<Impl, IMPL_OFFSET>, GetFilePosition::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRdcFileReader as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IRdcFileWriterImpl: Sized + IRdcFileReaderImpl {
    fn Write();
    fn Truncate();
    fn DeleteOnClose();
}
#[cfg(feature = "Win32_Foundation")]
impl IRdcFileWriterVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRdcFileWriterImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IRdcFileWriterVtbl {
        unsafe extern "system" fn Write<Impl: IRdcFileWriterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, offsetfilestart: u64, bytestowrite: u32, buffer: *mut u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Truncate<Impl: IRdcFileWriterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn DeleteOnClose<Impl: IRdcFileWriterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetFileSize::<Impl, IMPL_OFFSET>, Read::<Impl, IMPL_OFFSET>, GetFilePosition::<Impl, IMPL_OFFSET>, Write::<Impl, IMPL_OFFSET>, Truncate::<Impl, IMPL_OFFSET>, DeleteOnClose::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRdcFileWriter as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IRdcGeneratorImpl: Sized {
    fn GetGeneratorParameters();
    fn Process();
}
#[cfg(feature = "Win32_Foundation")]
impl IRdcGeneratorVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRdcGeneratorImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IRdcGeneratorVtbl {
        unsafe extern "system" fn GetGeneratorParameters<Impl: IRdcGeneratorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, level: u32, igeneratorparameters: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Process<Impl: IRdcGeneratorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, endofinput: super::super::Foundation::BOOL, endofoutput: *mut super::super::Foundation::BOOL, inputbuffer: *mut RdcBufferPointer, depth: u32, outputbuffers: *mut *mut RdcBufferPointer, rdc_errorcode: *mut RDC_ErrorCode) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetGeneratorParameters::<Impl, IMPL_OFFSET>, Process::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRdcGenerator as ::windows::core::Interface>::IID
    }
}
pub trait IRdcGeneratorFilterMaxParametersImpl: Sized {
    fn GetHorizonSize();
    fn SetHorizonSize();
    fn GetHashWindowSize();
    fn SetHashWindowSize();
}
impl IRdcGeneratorFilterMaxParametersVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRdcGeneratorFilterMaxParametersImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IRdcGeneratorFilterMaxParametersVtbl {
        unsafe extern "system" fn GetHorizonSize<Impl: IRdcGeneratorFilterMaxParametersImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, horizonsize: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetHorizonSize<Impl: IRdcGeneratorFilterMaxParametersImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, horizonsize: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetHashWindowSize<Impl: IRdcGeneratorFilterMaxParametersImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hashwindowsize: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetHashWindowSize<Impl: IRdcGeneratorFilterMaxParametersImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hashwindowsize: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetHorizonSize::<Impl, IMPL_OFFSET>, SetHorizonSize::<Impl, IMPL_OFFSET>, GetHashWindowSize::<Impl, IMPL_OFFSET>, SetHashWindowSize::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRdcGeneratorFilterMaxParameters as ::windows::core::Interface>::IID
    }
}
pub trait IRdcGeneratorParametersImpl: Sized {
    fn GetGeneratorParametersType();
    fn GetParametersVersion();
    fn GetSerializeSize();
    fn Serialize();
}
impl IRdcGeneratorParametersVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRdcGeneratorParametersImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IRdcGeneratorParametersVtbl {
        unsafe extern "system" fn GetGeneratorParametersType<Impl: IRdcGeneratorParametersImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, parameterstype: *mut GeneratorParametersType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetParametersVersion<Impl: IRdcGeneratorParametersImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, currentversion: *mut u32, minimumcompatibleappversion: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetSerializeSize<Impl: IRdcGeneratorParametersImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, size: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Serialize<Impl: IRdcGeneratorParametersImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, size: u32, parametersblob: *mut u8, byteswritten: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetGeneratorParametersType::<Impl, IMPL_OFFSET>, GetParametersVersion::<Impl, IMPL_OFFSET>, GetSerializeSize::<Impl, IMPL_OFFSET>, Serialize::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRdcGeneratorParameters as ::windows::core::Interface>::IID
    }
}
pub trait IRdcLibraryImpl: Sized {
    fn ComputeDefaultRecursionDepth();
    fn CreateGeneratorParameters();
    fn OpenGeneratorParameters();
    fn CreateGenerator();
    fn CreateComparator();
    fn CreateSignatureReader();
    fn GetRDCVersion();
}
impl IRdcLibraryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRdcLibraryImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IRdcLibraryVtbl {
        unsafe extern "system" fn ComputeDefaultRecursionDepth<Impl: IRdcLibraryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, filesize: u64, depth: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CreateGeneratorParameters<Impl: IRdcLibraryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, parameterstype: GeneratorParametersType, level: u32, igeneratorparameters: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn OpenGeneratorParameters<Impl: IRdcLibraryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, size: u32, parametersblob: *const u8, igeneratorparameters: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CreateGenerator<Impl: IRdcLibraryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, depth: u32, igeneratorparametersarray: *const ::windows::core::RawPtr, igenerator: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CreateComparator<Impl: IRdcLibraryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, iseedsignaturesfile: ::windows::core::RawPtr, comparatorbuffersize: u32, icomparator: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CreateSignatureReader<Impl: IRdcLibraryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ifilereader: ::windows::core::RawPtr, isignaturereader: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetRDCVersion<Impl: IRdcLibraryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, currentversion: *mut u32, minimumcompatibleappversion: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ComputeDefaultRecursionDepth::<Impl, IMPL_OFFSET>, CreateGeneratorParameters::<Impl, IMPL_OFFSET>, OpenGeneratorParameters::<Impl, IMPL_OFFSET>, CreateGenerator::<Impl, IMPL_OFFSET>, CreateComparator::<Impl, IMPL_OFFSET>, CreateSignatureReader::<Impl, IMPL_OFFSET>, GetRDCVersion::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRdcLibrary as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IRdcSignatureReaderImpl: Sized {
    fn ReadHeader();
    fn ReadSignatures();
}
#[cfg(feature = "Win32_Foundation")]
impl IRdcSignatureReaderVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRdcSignatureReaderImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IRdcSignatureReaderVtbl {
        unsafe extern "system" fn ReadHeader<Impl: IRdcSignatureReaderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rdc_errorcode: *mut RDC_ErrorCode) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ReadSignatures<Impl: IRdcSignatureReaderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rdcsignaturepointer: *mut RdcSignaturePointer, endofoutput: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ReadHeader::<Impl, IMPL_OFFSET>, ReadSignatures::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRdcSignatureReader as ::windows::core::Interface>::IID
    }
}
pub trait IRdcSimilarityGeneratorImpl: Sized {
    fn EnableSimilarity();
    fn Results();
}
impl IRdcSimilarityGeneratorVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRdcSimilarityGeneratorImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IRdcSimilarityGeneratorVtbl {
        unsafe extern "system" fn EnableSimilarity<Impl: IRdcSimilarityGeneratorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Results<Impl: IRdcSimilarityGeneratorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, similaritydata: *mut SimilarityData) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, EnableSimilarity::<Impl, IMPL_OFFSET>, Results::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRdcSimilarityGenerator as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait ISimilarityImpl: Sized {
    fn CreateTable();
    fn CreateTableIndirect();
    fn CloseTable();
    fn Append();
    fn FindSimilarFileId();
    fn CopyAndSwap();
    fn GetRecordCount();
}
#[cfg(feature = "Win32_Foundation")]
impl ISimilarityVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISimilarityImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISimilarityVtbl {
        unsafe extern "system" fn CreateTable<Impl: ISimilarityImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, path: super::super::Foundation::PWSTR, truncate: super::super::Foundation::BOOL, securitydescriptor: *const u8, recordsize: u32, isnew: *mut RdcCreatedTables) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CreateTableIndirect<Impl: ISimilarityImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, mapping: ::windows::core::RawPtr, fileidfile: ::windows::core::RawPtr, truncate: super::super::Foundation::BOOL, recordsize: u32, isnew: *mut RdcCreatedTables) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CloseTable<Impl: ISimilarityImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, isvalid: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Append<Impl: ISimilarityImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, similarityfileid: *const SimilarityFileId, similaritydata: *const SimilarityData) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn FindSimilarFileId<Impl: ISimilarityImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, similaritydata: *const SimilarityData, numberofmatchesrequired: u16, resultssize: u32, findsimilarresults: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CopyAndSwap<Impl: ISimilarityImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newsimilaritytables: ::windows::core::RawPtr, reportprogress: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetRecordCount<Impl: ISimilarityImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, recordcount: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, CreateTable::<Impl, IMPL_OFFSET>, CreateTableIndirect::<Impl, IMPL_OFFSET>, CloseTable::<Impl, IMPL_OFFSET>, Append::<Impl, IMPL_OFFSET>, FindSimilarFileId::<Impl, IMPL_OFFSET>, CopyAndSwap::<Impl, IMPL_OFFSET>, GetRecordCount::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISimilarity as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait ISimilarityFileIdTableImpl: Sized {
    fn CreateTable();
    fn CreateTableIndirect();
    fn CloseTable();
    fn Append();
    fn Lookup();
    fn Invalidate();
    fn GetRecordCount();
}
#[cfg(feature = "Win32_Foundation")]
impl ISimilarityFileIdTableVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISimilarityFileIdTableImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISimilarityFileIdTableVtbl {
        unsafe extern "system" fn CreateTable<Impl: ISimilarityFileIdTableImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, path: super::super::Foundation::PWSTR, truncate: super::super::Foundation::BOOL, securitydescriptor: *const u8, recordsize: u32, isnew: *mut RdcCreatedTables) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CreateTableIndirect<Impl: ISimilarityFileIdTableImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fileidfile: ::windows::core::RawPtr, truncate: super::super::Foundation::BOOL, recordsize: u32, isnew: *mut RdcCreatedTables) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CloseTable<Impl: ISimilarityFileIdTableImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, isvalid: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Append<Impl: ISimilarityFileIdTableImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, similarityfileid: *const SimilarityFileId, similarityfileindex: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Lookup<Impl: ISimilarityFileIdTableImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, similarityfileindex: u32, similarityfileid: *mut SimilarityFileId) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Invalidate<Impl: ISimilarityFileIdTableImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, similarityfileindex: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetRecordCount<Impl: ISimilarityFileIdTableImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, recordcount: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, CreateTable::<Impl, IMPL_OFFSET>, CreateTableIndirect::<Impl, IMPL_OFFSET>, CloseTable::<Impl, IMPL_OFFSET>, Append::<Impl, IMPL_OFFSET>, Lookup::<Impl, IMPL_OFFSET>, Invalidate::<Impl, IMPL_OFFSET>, GetRecordCount::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISimilarityFileIdTable as ::windows::core::Interface>::IID
    }
}
pub trait ISimilarityReportProgressImpl: Sized {
    fn ReportProgress();
}
impl ISimilarityReportProgressVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISimilarityReportProgressImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISimilarityReportProgressVtbl {
        unsafe extern "system" fn ReportProgress<Impl: ISimilarityReportProgressImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, percentcompleted: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ReportProgress::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISimilarityReportProgress as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait ISimilarityTableDumpStateImpl: Sized {
    fn GetNextData();
}
#[cfg(feature = "Win32_Foundation")]
impl ISimilarityTableDumpStateVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISimilarityTableDumpStateImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISimilarityTableDumpStateVtbl {
        unsafe extern "system" fn GetNextData<Impl: ISimilarityTableDumpStateImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, resultssize: u32, resultsused: *mut u32, eof: *mut super::super::Foundation::BOOL, results: *mut SimilarityDumpData) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetNextData::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISimilarityTableDumpState as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait ISimilarityTraitsMappedViewImpl: Sized {
    fn Flush();
    fn Unmap();
    fn Get();
    fn GetView();
}
#[cfg(feature = "Win32_Foundation")]
impl ISimilarityTraitsMappedViewVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISimilarityTraitsMappedViewImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISimilarityTraitsMappedViewVtbl {
        unsafe extern "system" fn Flush<Impl: ISimilarityTraitsMappedViewImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Unmap<Impl: ISimilarityTraitsMappedViewImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Get<Impl: ISimilarityTraitsMappedViewImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u64, dirty: super::super::Foundation::BOOL, numelements: u32, viewinfo: *mut SimilarityMappedViewInfo) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetView<Impl: ISimilarityTraitsMappedViewImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, mappedpagebegin: *mut *mut u8, mappedpageend: *mut *mut u8) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, Flush::<Impl, IMPL_OFFSET>, Unmap::<Impl, IMPL_OFFSET>, Get::<Impl, IMPL_OFFSET>, GetView::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISimilarityTraitsMappedView as ::windows::core::Interface>::IID
    }
}
pub trait ISimilarityTraitsMappingImpl: Sized {
    fn CloseMapping();
    fn SetFileSize();
    fn GetFileSize();
    fn OpenMapping();
    fn ResizeMapping();
    fn GetPageSize();
    fn CreateView();
}
impl ISimilarityTraitsMappingVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISimilarityTraitsMappingImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISimilarityTraitsMappingVtbl {
        unsafe extern "system" fn CloseMapping<Impl: ISimilarityTraitsMappingImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetFileSize<Impl: ISimilarityTraitsMappingImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, filesize: u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetFileSize<Impl: ISimilarityTraitsMappingImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, filesize: *mut u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn OpenMapping<Impl: ISimilarityTraitsMappingImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, accessmode: RdcMappingAccessMode, begin: u64, end: u64, actualend: *mut u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ResizeMapping<Impl: ISimilarityTraitsMappingImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, accessmode: RdcMappingAccessMode, begin: u64, end: u64, actualend: *mut u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetPageSize<Impl: ISimilarityTraitsMappingImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pagesize: *mut u32) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CreateView<Impl: ISimilarityTraitsMappingImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, minimummappedpages: u32, accessmode: RdcMappingAccessMode, mappedview: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, CloseMapping::<Impl, IMPL_OFFSET>, SetFileSize::<Impl, IMPL_OFFSET>, GetFileSize::<Impl, IMPL_OFFSET>, OpenMapping::<Impl, IMPL_OFFSET>, ResizeMapping::<Impl, IMPL_OFFSET>, GetPageSize::<Impl, IMPL_OFFSET>, CreateView::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISimilarityTraitsMapping as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait ISimilarityTraitsTableImpl: Sized {
    fn CreateTable();
    fn CreateTableIndirect();
    fn CloseTable();
    fn Append();
    fn FindSimilarFileIndex();
    fn BeginDump();
    fn GetLastIndex();
}
#[cfg(feature = "Win32_Foundation")]
impl ISimilarityTraitsTableVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISimilarityTraitsTableImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISimilarityTraitsTableVtbl {
        unsafe extern "system" fn CreateTable<Impl: ISimilarityTraitsTableImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, path: super::super::Foundation::PWSTR, truncate: super::super::Foundation::BOOL, securitydescriptor: *const u8, isnew: *mut RdcCreatedTables) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CreateTableIndirect<Impl: ISimilarityTraitsTableImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, mapping: ::windows::core::RawPtr, truncate: super::super::Foundation::BOOL, isnew: *mut RdcCreatedTables) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CloseTable<Impl: ISimilarityTraitsTableImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, isvalid: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Append<Impl: ISimilarityTraitsTableImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, data: *const SimilarityData, fileindex: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn FindSimilarFileIndex<Impl: ISimilarityTraitsTableImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, similaritydata: *const SimilarityData, numberofmatchesrequired: u16, findsimilarfileindexresults: *mut FindSimilarFileIndexResults, resultssize: u32, resultsused: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn BeginDump<Impl: ISimilarityTraitsTableImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, similaritytabledumpstate: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetLastIndex<Impl: ISimilarityTraitsTableImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fileindex: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, CreateTable::<Impl, IMPL_OFFSET>, CreateTableIndirect::<Impl, IMPL_OFFSET>, CloseTable::<Impl, IMPL_OFFSET>, Append::<Impl, IMPL_OFFSET>, FindSimilarFileIndex::<Impl, IMPL_OFFSET>, BeginDump::<Impl, IMPL_OFFSET>, GetLastIndex::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISimilarityTraitsTable as ::windows::core::Interface>::IID
    }
}
