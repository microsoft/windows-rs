pub trait IFindSimilarResultsImpl: Sized {
    fn GetSize();
    fn GetNextFileId();
}
impl ::windows::core::RuntimeName for IFindSimilarResults {
    const NAME: &'static str = "Windows.Win32.Networking.RemoteDifferentialCompression.IFindSimilarResults";
}
impl IFindSimilarResultsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFindSimilarResultsImpl, const OFFSET: isize>() -> IFindSimilarResultsVtbl {
        unsafe extern "system" fn GetSize<Impl: IFindSimilarResultsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, size: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetSize(::core::mem::transmute_copy(&size)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetNextFileId<Impl: IFindSimilarResultsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, numtraitsmatched: *mut u32, similarityfileid: *mut SimilarityFileId) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetNextFileId(::core::mem::transmute_copy(&numtraitsmatched), ::core::mem::transmute_copy(&similarityfileid)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IFindSimilarResults>, ::windows::core::GetTrustLevel, GetSize::<Impl, OFFSET>, GetNextFileId::<Impl, OFFSET>)
    }
}
pub trait IRdcComparatorImpl: Sized {
    fn Process();
}
impl ::windows::core::RuntimeName for IRdcComparator {
    const NAME: &'static str = "Windows.Win32.Networking.RemoteDifferentialCompression.IRdcComparator";
}
impl IRdcComparatorVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRdcComparatorImpl, const OFFSET: isize>() -> IRdcComparatorVtbl {
        unsafe extern "system" fn Process<Impl: IRdcComparatorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, endofinput: super::super::Foundation::BOOL, endofoutput: *mut super::super::Foundation::BOOL, inputbuffer: *mut RdcBufferPointer, outputbuffer: *mut RdcNeedPointer, rdc_errorcode: *mut RDC_ErrorCode) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Process(
                &*(&endofinput as *const <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType),
                ::core::mem::transmute_copy(&endofoutput),
                &*(&inputbuffer as *const <RdcBufferPointer as ::windows::core::Abi>::Abi as *const <RdcBufferPointer as ::windows::core::DefaultType>::DefaultType),
                &*(&outputbuffer as *const <RdcNeedPointer as ::windows::core::Abi>::Abi as *const <RdcNeedPointer as ::windows::core::DefaultType>::DefaultType),
                ::core::mem::transmute_copy(&rdc_errorcode),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IRdcComparator>, ::windows::core::GetTrustLevel, Process::<Impl, OFFSET>)
    }
}
pub trait IRdcFileReaderImpl: Sized {
    fn GetFileSize();
    fn Read();
    fn GetFilePosition();
}
impl ::windows::core::RuntimeName for IRdcFileReader {
    const NAME: &'static str = "Windows.Win32.Networking.RemoteDifferentialCompression.IRdcFileReader";
}
impl IRdcFileReaderVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRdcFileReaderImpl, const OFFSET: isize>() -> IRdcFileReaderVtbl {
        unsafe extern "system" fn GetFileSize<Impl: IRdcFileReaderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, filesize: *mut u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetFileSize(::core::mem::transmute_copy(&filesize)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Read<Impl: IRdcFileReaderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, offsetfilestart: u64, bytestoread: u32, bytesactuallyread: *mut u32, buffer: *mut u8, eof: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Read(offsetfilestart, bytestoread, ::core::mem::transmute_copy(&bytesactuallyread), ::core::mem::transmute_copy(&buffer), ::core::mem::transmute_copy(&eof)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFilePosition<Impl: IRdcFileReaderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, offsetfromstart: *mut u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetFilePosition(::core::mem::transmute_copy(&offsetfromstart)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IRdcFileReader>, ::windows::core::GetTrustLevel, GetFileSize::<Impl, OFFSET>, Read::<Impl, OFFSET>, GetFilePosition::<Impl, OFFSET>)
    }
}
pub trait IRdcFileWriterImpl: Sized + IRdcFileReaderImpl {
    fn Write();
    fn Truncate();
    fn DeleteOnClose();
}
impl ::windows::core::RuntimeName for IRdcFileWriter {
    const NAME: &'static str = "Windows.Win32.Networking.RemoteDifferentialCompression.IRdcFileWriter";
}
impl IRdcFileWriterVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRdcFileWriterImpl, const OFFSET: isize>() -> IRdcFileWriterVtbl {
        unsafe extern "system" fn Write<Impl: IRdcFileWriterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, offsetfilestart: u64, bytestowrite: u32, buffer: *mut u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Write(offsetfilestart, bytestowrite, ::core::mem::transmute_copy(&buffer)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Truncate<Impl: IRdcFileWriterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Truncate() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DeleteOnClose<Impl: IRdcFileWriterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DeleteOnClose() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IRdcFileWriter>, ::windows::core::GetTrustLevel, Write::<Impl, OFFSET>, Truncate::<Impl, OFFSET>, DeleteOnClose::<Impl, OFFSET>)
    }
}
pub trait IRdcGeneratorImpl: Sized {
    fn GetGeneratorParameters();
    fn Process();
}
impl ::windows::core::RuntimeName for IRdcGenerator {
    const NAME: &'static str = "Windows.Win32.Networking.RemoteDifferentialCompression.IRdcGenerator";
}
impl IRdcGeneratorVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRdcGeneratorImpl, const OFFSET: isize>() -> IRdcGeneratorVtbl {
        unsafe extern "system" fn GetGeneratorParameters<Impl: IRdcGeneratorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, level: u32, igeneratorparameters: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetGeneratorParameters(level, ::core::mem::transmute_copy(&igeneratorparameters)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Process<Impl: IRdcGeneratorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, endofinput: super::super::Foundation::BOOL, endofoutput: *mut super::super::Foundation::BOOL, inputbuffer: *mut RdcBufferPointer, depth: u32, outputbuffers: *mut *mut RdcBufferPointer, rdc_errorcode: *mut RDC_ErrorCode) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Process(&*(&endofinput as *const <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&endofoutput), &*(&inputbuffer as *const <RdcBufferPointer as ::windows::core::Abi>::Abi as *const <RdcBufferPointer as ::windows::core::DefaultType>::DefaultType), depth, ::core::mem::transmute_copy(&outputbuffers), ::core::mem::transmute_copy(&rdc_errorcode)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IRdcGenerator>, ::windows::core::GetTrustLevel, GetGeneratorParameters::<Impl, OFFSET>, Process::<Impl, OFFSET>)
    }
}
pub trait IRdcGeneratorFilterMaxParametersImpl: Sized {
    fn GetHorizonSize();
    fn SetHorizonSize();
    fn GetHashWindowSize();
    fn SetHashWindowSize();
}
impl ::windows::core::RuntimeName for IRdcGeneratorFilterMaxParameters {
    const NAME: &'static str = "Windows.Win32.Networking.RemoteDifferentialCompression.IRdcGeneratorFilterMaxParameters";
}
impl IRdcGeneratorFilterMaxParametersVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRdcGeneratorFilterMaxParametersImpl, const OFFSET: isize>() -> IRdcGeneratorFilterMaxParametersVtbl {
        unsafe extern "system" fn GetHorizonSize<Impl: IRdcGeneratorFilterMaxParametersImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, horizonsize: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetHorizonSize(::core::mem::transmute_copy(&horizonsize)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetHorizonSize<Impl: IRdcGeneratorFilterMaxParametersImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, horizonsize: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetHorizonSize(horizonsize) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetHashWindowSize<Impl: IRdcGeneratorFilterMaxParametersImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hashwindowsize: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetHashWindowSize(::core::mem::transmute_copy(&hashwindowsize)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetHashWindowSize<Impl: IRdcGeneratorFilterMaxParametersImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hashwindowsize: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetHashWindowSize(hashwindowsize) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IRdcGeneratorFilterMaxParameters>, ::windows::core::GetTrustLevel, GetHorizonSize::<Impl, OFFSET>, SetHorizonSize::<Impl, OFFSET>, GetHashWindowSize::<Impl, OFFSET>, SetHashWindowSize::<Impl, OFFSET>)
    }
}
pub trait IRdcGeneratorParametersImpl: Sized {
    fn GetGeneratorParametersType();
    fn GetParametersVersion();
    fn GetSerializeSize();
    fn Serialize();
}
impl ::windows::core::RuntimeName for IRdcGeneratorParameters {
    const NAME: &'static str = "Windows.Win32.Networking.RemoteDifferentialCompression.IRdcGeneratorParameters";
}
impl IRdcGeneratorParametersVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRdcGeneratorParametersImpl, const OFFSET: isize>() -> IRdcGeneratorParametersVtbl {
        unsafe extern "system" fn GetGeneratorParametersType<Impl: IRdcGeneratorParametersImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, parameterstype: *mut GeneratorParametersType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetGeneratorParametersType(::core::mem::transmute_copy(&parameterstype)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetParametersVersion<Impl: IRdcGeneratorParametersImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, currentversion: *mut u32, minimumcompatibleappversion: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetParametersVersion(::core::mem::transmute_copy(&currentversion), ::core::mem::transmute_copy(&minimumcompatibleappversion)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSerializeSize<Impl: IRdcGeneratorParametersImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, size: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetSerializeSize(::core::mem::transmute_copy(&size)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Serialize<Impl: IRdcGeneratorParametersImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, size: u32, parametersblob: *mut u8, byteswritten: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Serialize(size, ::core::mem::transmute_copy(&parametersblob), ::core::mem::transmute_copy(&byteswritten)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IRdcGeneratorParameters>, ::windows::core::GetTrustLevel, GetGeneratorParametersType::<Impl, OFFSET>, GetParametersVersion::<Impl, OFFSET>, GetSerializeSize::<Impl, OFFSET>, Serialize::<Impl, OFFSET>)
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
impl ::windows::core::RuntimeName for IRdcLibrary {
    const NAME: &'static str = "Windows.Win32.Networking.RemoteDifferentialCompression.IRdcLibrary";
}
impl IRdcLibraryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRdcLibraryImpl, const OFFSET: isize>() -> IRdcLibraryVtbl {
        unsafe extern "system" fn ComputeDefaultRecursionDepth<Impl: IRdcLibraryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, filesize: u64, depth: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ComputeDefaultRecursionDepth(filesize, ::core::mem::transmute_copy(&depth)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateGeneratorParameters<Impl: IRdcLibraryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, parameterstype: GeneratorParametersType, level: u32, igeneratorparameters: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateGeneratorParameters(parameterstype, level, ::core::mem::transmute_copy(&igeneratorparameters)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OpenGeneratorParameters<Impl: IRdcLibraryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, size: u32, parametersblob: *const u8, igeneratorparameters: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OpenGeneratorParameters(size, parametersblob, ::core::mem::transmute_copy(&igeneratorparameters)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateGenerator<Impl: IRdcLibraryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, depth: u32, igeneratorparametersarray: *const ::windows::core::RawPtr, igenerator: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateGenerator(depth, &*(&igeneratorparametersarray as *const <IRdcGeneratorParameters as ::windows::core::Abi>::Abi as *const <IRdcGeneratorParameters as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&igenerator)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateComparator<Impl: IRdcLibraryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, iseedsignaturesfile: ::windows::core::RawPtr, comparatorbuffersize: u32, icomparator: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateComparator(&*(&iseedsignaturesfile as *const <IRdcFileReader as ::windows::core::Abi>::Abi as *const <IRdcFileReader as ::windows::core::DefaultType>::DefaultType), comparatorbuffersize, ::core::mem::transmute_copy(&icomparator)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateSignatureReader<Impl: IRdcLibraryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ifilereader: ::windows::core::RawPtr, isignaturereader: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateSignatureReader(&*(&ifilereader as *const <IRdcFileReader as ::windows::core::Abi>::Abi as *const <IRdcFileReader as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&isignaturereader)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetRDCVersion<Impl: IRdcLibraryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, currentversion: *mut u32, minimumcompatibleappversion: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetRDCVersion(::core::mem::transmute_copy(&currentversion), ::core::mem::transmute_copy(&minimumcompatibleappversion)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(
            ::windows::core::QueryInterface::<Identity, OFFSET>,
            ::windows::core::AddRef::<Identity, OFFSET>,
            ::windows::core::Release::<Identity, OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<IRdcLibrary>,
            ::windows::core::GetTrustLevel,
            ComputeDefaultRecursionDepth::<Impl, OFFSET>,
            CreateGeneratorParameters::<Impl, OFFSET>,
            OpenGeneratorParameters::<Impl, OFFSET>,
            CreateGenerator::<Impl, OFFSET>,
            CreateComparator::<Impl, OFFSET>,
            CreateSignatureReader::<Impl, OFFSET>,
            GetRDCVersion::<Impl, OFFSET>,
        )
    }
}
pub trait IRdcSignatureReaderImpl: Sized {
    fn ReadHeader();
    fn ReadSignatures();
}
impl ::windows::core::RuntimeName for IRdcSignatureReader {
    const NAME: &'static str = "Windows.Win32.Networking.RemoteDifferentialCompression.IRdcSignatureReader";
}
impl IRdcSignatureReaderVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRdcSignatureReaderImpl, const OFFSET: isize>() -> IRdcSignatureReaderVtbl {
        unsafe extern "system" fn ReadHeader<Impl: IRdcSignatureReaderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rdc_errorcode: *mut RDC_ErrorCode) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ReadHeader(::core::mem::transmute_copy(&rdc_errorcode)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ReadSignatures<Impl: IRdcSignatureReaderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rdcsignaturepointer: *mut RdcSignaturePointer, endofoutput: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ReadSignatures(&*(&rdcsignaturepointer as *const <RdcSignaturePointer as ::windows::core::Abi>::Abi as *const <RdcSignaturePointer as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&endofoutput)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IRdcSignatureReader>, ::windows::core::GetTrustLevel, ReadHeader::<Impl, OFFSET>, ReadSignatures::<Impl, OFFSET>)
    }
}
pub trait IRdcSimilarityGeneratorImpl: Sized {
    fn EnableSimilarity();
    fn Results();
}
impl ::windows::core::RuntimeName for IRdcSimilarityGenerator {
    const NAME: &'static str = "Windows.Win32.Networking.RemoteDifferentialCompression.IRdcSimilarityGenerator";
}
impl IRdcSimilarityGeneratorVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRdcSimilarityGeneratorImpl, const OFFSET: isize>() -> IRdcSimilarityGeneratorVtbl {
        unsafe extern "system" fn EnableSimilarity<Impl: IRdcSimilarityGeneratorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EnableSimilarity() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Results<Impl: IRdcSimilarityGeneratorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, similaritydata: *mut SimilarityData) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Results(::core::mem::transmute_copy(&similaritydata)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IRdcSimilarityGenerator>, ::windows::core::GetTrustLevel, EnableSimilarity::<Impl, OFFSET>, Results::<Impl, OFFSET>)
    }
}
pub trait ISimilarityImpl: Sized {
    fn CreateTable();
    fn CreateTableIndirect();
    fn CloseTable();
    fn Append();
    fn FindSimilarFileId();
    fn CopyAndSwap();
    fn GetRecordCount();
}
impl ::windows::core::RuntimeName for ISimilarity {
    const NAME: &'static str = "Windows.Win32.Networking.RemoteDifferentialCompression.ISimilarity";
}
impl ISimilarityVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISimilarityImpl, const OFFSET: isize>() -> ISimilarityVtbl {
        unsafe extern "system" fn CreateTable<Impl: ISimilarityImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, path: super::super::Foundation::PWSTR, truncate: super::super::Foundation::BOOL, securitydescriptor: *const u8, recordsize: u32, isnew: *mut RdcCreatedTables) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateTable(&*(&path as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), &*(&truncate as *const <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType), securitydescriptor, recordsize, ::core::mem::transmute_copy(&isnew)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateTableIndirect<Impl: ISimilarityImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, mapping: ::windows::core::RawPtr, fileidfile: ::windows::core::RawPtr, truncate: super::super::Foundation::BOOL, recordsize: u32, isnew: *mut RdcCreatedTables) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateTableIndirect(
                &*(&mapping as *const <ISimilarityTraitsMapping as ::windows::core::Abi>::Abi as *const <ISimilarityTraitsMapping as ::windows::core::DefaultType>::DefaultType),
                &*(&fileidfile as *const <IRdcFileWriter as ::windows::core::Abi>::Abi as *const <IRdcFileWriter as ::windows::core::DefaultType>::DefaultType),
                &*(&truncate as *const <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType),
                recordsize,
                ::core::mem::transmute_copy(&isnew),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CloseTable<Impl: ISimilarityImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, isvalid: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CloseTable(&*(&isvalid as *const <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Append<Impl: ISimilarityImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, similarityfileid: *const SimilarityFileId, similaritydata: *const SimilarityData) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Append(&*(&similarityfileid as *const <SimilarityFileId as ::windows::core::Abi>::Abi as *const <SimilarityFileId as ::windows::core::DefaultType>::DefaultType), &*(&similaritydata as *const <SimilarityData as ::windows::core::Abi>::Abi as *const <SimilarityData as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FindSimilarFileId<Impl: ISimilarityImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, similaritydata: *const SimilarityData, numberofmatchesrequired: u16, resultssize: u32, findsimilarresults: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FindSimilarFileId(&*(&similaritydata as *const <SimilarityData as ::windows::core::Abi>::Abi as *const <SimilarityData as ::windows::core::DefaultType>::DefaultType), numberofmatchesrequired, resultssize, ::core::mem::transmute_copy(&findsimilarresults)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CopyAndSwap<Impl: ISimilarityImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newsimilaritytables: ::windows::core::RawPtr, reportprogress: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CopyAndSwap(&*(&newsimilaritytables as *const <ISimilarity as ::windows::core::Abi>::Abi as *const <ISimilarity as ::windows::core::DefaultType>::DefaultType), &*(&reportprogress as *const <ISimilarityReportProgress as ::windows::core::Abi>::Abi as *const <ISimilarityReportProgress as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetRecordCount<Impl: ISimilarityImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, recordcount: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetRecordCount(::core::mem::transmute_copy(&recordcount)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ISimilarity>, ::windows::core::GetTrustLevel, CreateTable::<Impl, OFFSET>, CreateTableIndirect::<Impl, OFFSET>, CloseTable::<Impl, OFFSET>, Append::<Impl, OFFSET>, FindSimilarFileId::<Impl, OFFSET>, CopyAndSwap::<Impl, OFFSET>, GetRecordCount::<Impl, OFFSET>)
    }
}
pub trait ISimilarityFileIdTableImpl: Sized {
    fn CreateTable();
    fn CreateTableIndirect();
    fn CloseTable();
    fn Append();
    fn Lookup();
    fn Invalidate();
    fn GetRecordCount();
}
impl ::windows::core::RuntimeName for ISimilarityFileIdTable {
    const NAME: &'static str = "Windows.Win32.Networking.RemoteDifferentialCompression.ISimilarityFileIdTable";
}
impl ISimilarityFileIdTableVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISimilarityFileIdTableImpl, const OFFSET: isize>() -> ISimilarityFileIdTableVtbl {
        unsafe extern "system" fn CreateTable<Impl: ISimilarityFileIdTableImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, path: super::super::Foundation::PWSTR, truncate: super::super::Foundation::BOOL, securitydescriptor: *const u8, recordsize: u32, isnew: *mut RdcCreatedTables) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateTable(&*(&path as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), &*(&truncate as *const <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType), securitydescriptor, recordsize, ::core::mem::transmute_copy(&isnew)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateTableIndirect<Impl: ISimilarityFileIdTableImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fileidfile: ::windows::core::RawPtr, truncate: super::super::Foundation::BOOL, recordsize: u32, isnew: *mut RdcCreatedTables) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateTableIndirect(&*(&fileidfile as *const <IRdcFileWriter as ::windows::core::Abi>::Abi as *const <IRdcFileWriter as ::windows::core::DefaultType>::DefaultType), &*(&truncate as *const <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType), recordsize, ::core::mem::transmute_copy(&isnew)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CloseTable<Impl: ISimilarityFileIdTableImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, isvalid: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CloseTable(&*(&isvalid as *const <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Append<Impl: ISimilarityFileIdTableImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, similarityfileid: *const SimilarityFileId, similarityfileindex: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Append(&*(&similarityfileid as *const <SimilarityFileId as ::windows::core::Abi>::Abi as *const <SimilarityFileId as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&similarityfileindex)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Lookup<Impl: ISimilarityFileIdTableImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, similarityfileindex: u32, similarityfileid: *mut SimilarityFileId) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Lookup(similarityfileindex, ::core::mem::transmute_copy(&similarityfileid)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Invalidate<Impl: ISimilarityFileIdTableImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, similarityfileindex: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Invalidate(similarityfileindex) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetRecordCount<Impl: ISimilarityFileIdTableImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, recordcount: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetRecordCount(::core::mem::transmute_copy(&recordcount)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ISimilarityFileIdTable>, ::windows::core::GetTrustLevel, CreateTable::<Impl, OFFSET>, CreateTableIndirect::<Impl, OFFSET>, CloseTable::<Impl, OFFSET>, Append::<Impl, OFFSET>, Lookup::<Impl, OFFSET>, Invalidate::<Impl, OFFSET>, GetRecordCount::<Impl, OFFSET>)
    }
}
pub trait ISimilarityReportProgressImpl: Sized {
    fn ReportProgress();
}
impl ::windows::core::RuntimeName for ISimilarityReportProgress {
    const NAME: &'static str = "Windows.Win32.Networking.RemoteDifferentialCompression.ISimilarityReportProgress";
}
impl ISimilarityReportProgressVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISimilarityReportProgressImpl, const OFFSET: isize>() -> ISimilarityReportProgressVtbl {
        unsafe extern "system" fn ReportProgress<Impl: ISimilarityReportProgressImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, percentcompleted: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ReportProgress(percentcompleted) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ISimilarityReportProgress>, ::windows::core::GetTrustLevel, ReportProgress::<Impl, OFFSET>)
    }
}
pub trait ISimilarityTableDumpStateImpl: Sized {
    fn GetNextData();
}
impl ::windows::core::RuntimeName for ISimilarityTableDumpState {
    const NAME: &'static str = "Windows.Win32.Networking.RemoteDifferentialCompression.ISimilarityTableDumpState";
}
impl ISimilarityTableDumpStateVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISimilarityTableDumpStateImpl, const OFFSET: isize>() -> ISimilarityTableDumpStateVtbl {
        unsafe extern "system" fn GetNextData<Impl: ISimilarityTableDumpStateImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, resultssize: u32, resultsused: *mut u32, eof: *mut super::super::Foundation::BOOL, results: *mut SimilarityDumpData) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetNextData(resultssize, ::core::mem::transmute_copy(&resultsused), ::core::mem::transmute_copy(&eof), &*(&results as *const <SimilarityDumpData as ::windows::core::Abi>::Abi as *const <SimilarityDumpData as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ISimilarityTableDumpState>, ::windows::core::GetTrustLevel, GetNextData::<Impl, OFFSET>)
    }
}
pub trait ISimilarityTraitsMappedViewImpl: Sized {
    fn Flush();
    fn Unmap();
    fn Get();
    fn GetView();
}
impl ::windows::core::RuntimeName for ISimilarityTraitsMappedView {
    const NAME: &'static str = "Windows.Win32.Networking.RemoteDifferentialCompression.ISimilarityTraitsMappedView";
}
impl ISimilarityTraitsMappedViewVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISimilarityTraitsMappedViewImpl, const OFFSET: isize>() -> ISimilarityTraitsMappedViewVtbl {
        unsafe extern "system" fn Flush<Impl: ISimilarityTraitsMappedViewImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Flush() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Unmap<Impl: ISimilarityTraitsMappedViewImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Unmap() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Get<Impl: ISimilarityTraitsMappedViewImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u64, dirty: super::super::Foundation::BOOL, numelements: u32, viewinfo: *mut SimilarityMappedViewInfo) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Get(index, &*(&dirty as *const <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType), numelements, ::core::mem::transmute_copy(&viewinfo)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetView<Impl: ISimilarityTraitsMappedViewImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, mappedpagebegin: *mut *mut u8, mappedpageend: *mut *mut u8) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetView(::core::mem::transmute_copy(&mappedpagebegin), ::core::mem::transmute_copy(&mappedpageend)).into()
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ISimilarityTraitsMappedView>, ::windows::core::GetTrustLevel, Flush::<Impl, OFFSET>, Unmap::<Impl, OFFSET>, Get::<Impl, OFFSET>, GetView::<Impl, OFFSET>)
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
impl ::windows::core::RuntimeName for ISimilarityTraitsMapping {
    const NAME: &'static str = "Windows.Win32.Networking.RemoteDifferentialCompression.ISimilarityTraitsMapping";
}
impl ISimilarityTraitsMappingVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISimilarityTraitsMappingImpl, const OFFSET: isize>() -> ISimilarityTraitsMappingVtbl {
        unsafe extern "system" fn CloseMapping<Impl: ISimilarityTraitsMappingImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).CloseMapping().into()
        }
        unsafe extern "system" fn SetFileSize<Impl: ISimilarityTraitsMappingImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, filesize: u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetFileSize(filesize) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFileSize<Impl: ISimilarityTraitsMappingImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, filesize: *mut u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetFileSize(::core::mem::transmute_copy(&filesize)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OpenMapping<Impl: ISimilarityTraitsMappingImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, accessmode: RdcMappingAccessMode, begin: u64, end: u64, actualend: *mut u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OpenMapping(accessmode, begin, end, ::core::mem::transmute_copy(&actualend)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ResizeMapping<Impl: ISimilarityTraitsMappingImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, accessmode: RdcMappingAccessMode, begin: u64, end: u64, actualend: *mut u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ResizeMapping(accessmode, begin, end, ::core::mem::transmute_copy(&actualend)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPageSize<Impl: ISimilarityTraitsMappingImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pagesize: *mut u32) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetPageSize(::core::mem::transmute_copy(&pagesize)).into()
        }
        unsafe extern "system" fn CreateView<Impl: ISimilarityTraitsMappingImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, minimummappedpages: u32, accessmode: RdcMappingAccessMode, mappedview: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateView(minimummappedpages, accessmode, ::core::mem::transmute_copy(&mappedview)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ISimilarityTraitsMapping>, ::windows::core::GetTrustLevel, CloseMapping::<Impl, OFFSET>, SetFileSize::<Impl, OFFSET>, GetFileSize::<Impl, OFFSET>, OpenMapping::<Impl, OFFSET>, ResizeMapping::<Impl, OFFSET>, GetPageSize::<Impl, OFFSET>, CreateView::<Impl, OFFSET>)
    }
}
pub trait ISimilarityTraitsTableImpl: Sized {
    fn CreateTable();
    fn CreateTableIndirect();
    fn CloseTable();
    fn Append();
    fn FindSimilarFileIndex();
    fn BeginDump();
    fn GetLastIndex();
}
impl ::windows::core::RuntimeName for ISimilarityTraitsTable {
    const NAME: &'static str = "Windows.Win32.Networking.RemoteDifferentialCompression.ISimilarityTraitsTable";
}
impl ISimilarityTraitsTableVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISimilarityTraitsTableImpl, const OFFSET: isize>() -> ISimilarityTraitsTableVtbl {
        unsafe extern "system" fn CreateTable<Impl: ISimilarityTraitsTableImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, path: super::super::Foundation::PWSTR, truncate: super::super::Foundation::BOOL, securitydescriptor: *const u8, isnew: *mut RdcCreatedTables) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateTable(&*(&path as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), &*(&truncate as *const <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType), securitydescriptor, ::core::mem::transmute_copy(&isnew)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateTableIndirect<Impl: ISimilarityTraitsTableImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, mapping: ::windows::core::RawPtr, truncate: super::super::Foundation::BOOL, isnew: *mut RdcCreatedTables) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateTableIndirect(&*(&mapping as *const <ISimilarityTraitsMapping as ::windows::core::Abi>::Abi as *const <ISimilarityTraitsMapping as ::windows::core::DefaultType>::DefaultType), &*(&truncate as *const <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&isnew)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CloseTable<Impl: ISimilarityTraitsTableImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, isvalid: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CloseTable(&*(&isvalid as *const <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Append<Impl: ISimilarityTraitsTableImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, data: *const SimilarityData, fileindex: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Append(&*(&data as *const <SimilarityData as ::windows::core::Abi>::Abi as *const <SimilarityData as ::windows::core::DefaultType>::DefaultType), fileindex) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FindSimilarFileIndex<Impl: ISimilarityTraitsTableImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, similaritydata: *const SimilarityData, numberofmatchesrequired: u16, findsimilarfileindexresults: *mut FindSimilarFileIndexResults, resultssize: u32, resultsused: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FindSimilarFileIndex(&*(&similaritydata as *const <SimilarityData as ::windows::core::Abi>::Abi as *const <SimilarityData as ::windows::core::DefaultType>::DefaultType), numberofmatchesrequired, ::core::mem::transmute_copy(&findsimilarfileindexresults), resultssize, ::core::mem::transmute_copy(&resultsused)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BeginDump<Impl: ISimilarityTraitsTableImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, similaritytabledumpstate: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).BeginDump(::core::mem::transmute_copy(&similaritytabledumpstate)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetLastIndex<Impl: ISimilarityTraitsTableImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fileindex: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetLastIndex(::core::mem::transmute_copy(&fileindex)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ISimilarityTraitsTable>, ::windows::core::GetTrustLevel, CreateTable::<Impl, OFFSET>, CreateTableIndirect::<Impl, OFFSET>, CloseTable::<Impl, OFFSET>, Append::<Impl, OFFSET>, FindSimilarFileIndex::<Impl, OFFSET>, BeginDump::<Impl, OFFSET>, GetLastIndex::<Impl, OFFSET>)
    }
}
