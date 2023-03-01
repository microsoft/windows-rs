#[doc = "*Required features: `\"Win32_Networking_RemoteDifferentialCompression\"`, `\"implement\"`*"]
pub trait IFindSimilarResults_Impl: Sized {
    fn GetSize(&self) -> ::windows::core::Result<u32>;
    fn GetNextFileId(&self, numtraitsmatched: *mut u32, similarityfileid: *mut SimilarityFileId) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IFindSimilarResults {}
impl IFindSimilarResults_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFindSimilarResults_Impl, const OFFSET: isize>() -> IFindSimilarResults_Vtbl {
        unsafe extern "system" fn GetSize<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFindSimilarResults_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, size: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetSize() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(size, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetNextFileId<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFindSimilarResults_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, numtraitsmatched: *mut u32, similarityfileid: *mut SimilarityFileId) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetNextFileId(::core::mem::transmute_copy(&numtraitsmatched), ::core::mem::transmute_copy(&similarityfileid)).into()
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetSize: GetSize::<Identity, Impl, OFFSET>,
            GetNextFileId: GetNextFileId::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFindSimilarResults as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Networking_RemoteDifferentialCompression\"`, `\"Win32_Foundation\"`, `\"implement\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub trait IRdcComparator_Impl: Sized {
    fn Process(&self, endofinput: super::super::Foundation::BOOL, endofoutput: *mut super::super::Foundation::BOOL, inputbuffer: *mut RdcBufferPointer, outputbuffer: *mut RdcNeedPointer, rdc_errorcode: *mut RDC_ErrorCode) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::RuntimeName for IRdcComparator {}
#[cfg(feature = "Win32_Foundation")]
impl IRdcComparator_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRdcComparator_Impl, const OFFSET: isize>() -> IRdcComparator_Vtbl {
        unsafe extern "system" fn Process<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRdcComparator_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, endofinput: super::super::Foundation::BOOL, endofoutput: *mut super::super::Foundation::BOOL, inputbuffer: *mut RdcBufferPointer, outputbuffer: *mut RdcNeedPointer, rdc_errorcode: *mut RDC_ErrorCode) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Process(::core::mem::transmute_copy(&endofinput), ::core::mem::transmute_copy(&endofoutput), ::core::mem::transmute_copy(&inputbuffer), ::core::mem::transmute_copy(&outputbuffer), ::core::mem::transmute_copy(&rdc_errorcode)).into()
        }
        Self { base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(), Process: Process::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRdcComparator as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Networking_RemoteDifferentialCompression\"`, `\"Win32_Foundation\"`, `\"implement\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub trait IRdcFileReader_Impl: Sized {
    fn GetFileSize(&self) -> ::windows::core::Result<u64>;
    fn Read(&self, offsetfilestart: u64, bytestoread: u32, bytesactuallyread: *mut u32, buffer: *mut u8, eof: *mut super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn GetFilePosition(&self) -> ::windows::core::Result<u64>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::RuntimeName for IRdcFileReader {}
#[cfg(feature = "Win32_Foundation")]
impl IRdcFileReader_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRdcFileReader_Impl, const OFFSET: isize>() -> IRdcFileReader_Vtbl {
        unsafe extern "system" fn GetFileSize<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRdcFileReader_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, filesize: *mut u64) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetFileSize() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(filesize, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Read<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRdcFileReader_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, offsetfilestart: u64, bytestoread: u32, bytesactuallyread: *mut u32, buffer: *mut u8, eof: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Read(::core::mem::transmute_copy(&offsetfilestart), ::core::mem::transmute_copy(&bytestoread), ::core::mem::transmute_copy(&bytesactuallyread), ::core::mem::transmute_copy(&buffer), ::core::mem::transmute_copy(&eof)).into()
        }
        unsafe extern "system" fn GetFilePosition<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRdcFileReader_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, offsetfromstart: *mut u64) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetFilePosition() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(offsetfromstart, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetFileSize: GetFileSize::<Identity, Impl, OFFSET>,
            Read: Read::<Identity, Impl, OFFSET>,
            GetFilePosition: GetFilePosition::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRdcFileReader as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Networking_RemoteDifferentialCompression\"`, `\"Win32_Foundation\"`, `\"implement\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub trait IRdcFileWriter_Impl: Sized + IRdcFileReader_Impl {
    fn Write(&self, offsetfilestart: u64, bytestowrite: u32) -> ::windows::core::Result<u8>;
    fn Truncate(&self) -> ::windows::core::Result<()>;
    fn DeleteOnClose(&self) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::RuntimeName for IRdcFileWriter {}
#[cfg(feature = "Win32_Foundation")]
impl IRdcFileWriter_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRdcFileWriter_Impl, const OFFSET: isize>() -> IRdcFileWriter_Vtbl {
        unsafe extern "system" fn Write<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRdcFileWriter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, offsetfilestart: u64, bytestowrite: u32, buffer: *mut u8) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Write(::core::mem::transmute_copy(&offsetfilestart), ::core::mem::transmute_copy(&bytestowrite)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(buffer, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Truncate<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRdcFileWriter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Truncate().into()
        }
        unsafe extern "system" fn DeleteOnClose<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRdcFileWriter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.DeleteOnClose().into()
        }
        Self {
            base__: IRdcFileReader_Vtbl::new::<Identity, Impl, OFFSET>(),
            Write: Write::<Identity, Impl, OFFSET>,
            Truncate: Truncate::<Identity, Impl, OFFSET>,
            DeleteOnClose: DeleteOnClose::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRdcFileWriter as ::windows::core::ComInterface>::IID || iid == &<IRdcFileReader as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Networking_RemoteDifferentialCompression\"`, `\"Win32_Foundation\"`, `\"implement\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub trait IRdcGenerator_Impl: Sized {
    fn GetGeneratorParameters(&self, level: u32) -> ::windows::core::Result<IRdcGeneratorParameters>;
    fn Process(&self, endofinput: super::super::Foundation::BOOL, endofoutput: *mut super::super::Foundation::BOOL, inputbuffer: *mut RdcBufferPointer, depth: u32, outputbuffers: *mut *mut RdcBufferPointer, rdc_errorcode: *mut RDC_ErrorCode) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::RuntimeName for IRdcGenerator {}
#[cfg(feature = "Win32_Foundation")]
impl IRdcGenerator_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRdcGenerator_Impl, const OFFSET: isize>() -> IRdcGenerator_Vtbl {
        unsafe extern "system" fn GetGeneratorParameters<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRdcGenerator_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, level: u32, igeneratorparameters: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetGeneratorParameters(::core::mem::transmute_copy(&level)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(igeneratorparameters, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Process<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRdcGenerator_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, endofinput: super::super::Foundation::BOOL, endofoutput: *mut super::super::Foundation::BOOL, inputbuffer: *mut RdcBufferPointer, depth: u32, outputbuffers: *mut *mut RdcBufferPointer, rdc_errorcode: *mut RDC_ErrorCode) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Process(::core::mem::transmute_copy(&endofinput), ::core::mem::transmute_copy(&endofoutput), ::core::mem::transmute_copy(&inputbuffer), ::core::mem::transmute_copy(&depth), ::core::mem::transmute_copy(&outputbuffers), ::core::mem::transmute_copy(&rdc_errorcode)).into()
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetGeneratorParameters: GetGeneratorParameters::<Identity, Impl, OFFSET>,
            Process: Process::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRdcGenerator as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Networking_RemoteDifferentialCompression\"`, `\"implement\"`*"]
pub trait IRdcGeneratorFilterMaxParameters_Impl: Sized {
    fn GetHorizonSize(&self) -> ::windows::core::Result<u32>;
    fn SetHorizonSize(&self, horizonsize: u32) -> ::windows::core::Result<()>;
    fn GetHashWindowSize(&self) -> ::windows::core::Result<u32>;
    fn SetHashWindowSize(&self, hashwindowsize: u32) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IRdcGeneratorFilterMaxParameters {}
impl IRdcGeneratorFilterMaxParameters_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRdcGeneratorFilterMaxParameters_Impl, const OFFSET: isize>() -> IRdcGeneratorFilterMaxParameters_Vtbl {
        unsafe extern "system" fn GetHorizonSize<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRdcGeneratorFilterMaxParameters_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, horizonsize: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetHorizonSize() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(horizonsize, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetHorizonSize<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRdcGeneratorFilterMaxParameters_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, horizonsize: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetHorizonSize(::core::mem::transmute_copy(&horizonsize)).into()
        }
        unsafe extern "system" fn GetHashWindowSize<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRdcGeneratorFilterMaxParameters_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hashwindowsize: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetHashWindowSize() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(hashwindowsize, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetHashWindowSize<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRdcGeneratorFilterMaxParameters_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hashwindowsize: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetHashWindowSize(::core::mem::transmute_copy(&hashwindowsize)).into()
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetHorizonSize: GetHorizonSize::<Identity, Impl, OFFSET>,
            SetHorizonSize: SetHorizonSize::<Identity, Impl, OFFSET>,
            GetHashWindowSize: GetHashWindowSize::<Identity, Impl, OFFSET>,
            SetHashWindowSize: SetHashWindowSize::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRdcGeneratorFilterMaxParameters as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Networking_RemoteDifferentialCompression\"`, `\"implement\"`*"]
pub trait IRdcGeneratorParameters_Impl: Sized {
    fn GetGeneratorParametersType(&self) -> ::windows::core::Result<GeneratorParametersType>;
    fn GetParametersVersion(&self, currentversion: *mut u32, minimumcompatibleappversion: *mut u32) -> ::windows::core::Result<()>;
    fn GetSerializeSize(&self) -> ::windows::core::Result<u32>;
    fn Serialize(&self, size: u32, parametersblob: *mut u8, byteswritten: *mut u32) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IRdcGeneratorParameters {}
impl IRdcGeneratorParameters_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRdcGeneratorParameters_Impl, const OFFSET: isize>() -> IRdcGeneratorParameters_Vtbl {
        unsafe extern "system" fn GetGeneratorParametersType<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRdcGeneratorParameters_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, parameterstype: *mut GeneratorParametersType) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetGeneratorParametersType() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(parameterstype, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetParametersVersion<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRdcGeneratorParameters_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, currentversion: *mut u32, minimumcompatibleappversion: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetParametersVersion(::core::mem::transmute_copy(&currentversion), ::core::mem::transmute_copy(&minimumcompatibleappversion)).into()
        }
        unsafe extern "system" fn GetSerializeSize<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRdcGeneratorParameters_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, size: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetSerializeSize() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(size, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Serialize<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRdcGeneratorParameters_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, size: u32, parametersblob: *mut u8, byteswritten: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Serialize(::core::mem::transmute_copy(&size), ::core::mem::transmute_copy(&parametersblob), ::core::mem::transmute_copy(&byteswritten)).into()
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetGeneratorParametersType: GetGeneratorParametersType::<Identity, Impl, OFFSET>,
            GetParametersVersion: GetParametersVersion::<Identity, Impl, OFFSET>,
            GetSerializeSize: GetSerializeSize::<Identity, Impl, OFFSET>,
            Serialize: Serialize::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRdcGeneratorParameters as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Networking_RemoteDifferentialCompression\"`, `\"implement\"`*"]
pub trait IRdcLibrary_Impl: Sized {
    fn ComputeDefaultRecursionDepth(&self, filesize: u64) -> ::windows::core::Result<u32>;
    fn CreateGeneratorParameters(&self, parameterstype: GeneratorParametersType, level: u32) -> ::windows::core::Result<IRdcGeneratorParameters>;
    fn OpenGeneratorParameters(&self, size: u32, parametersblob: *const u8) -> ::windows::core::Result<IRdcGeneratorParameters>;
    fn CreateGenerator(&self, depth: u32, igeneratorparametersarray: *const ::core::option::Option<IRdcGeneratorParameters>) -> ::windows::core::Result<IRdcGenerator>;
    fn CreateComparator(&self, iseedsignaturesfile: ::core::option::Option<&IRdcFileReader>, comparatorbuffersize: u32) -> ::windows::core::Result<IRdcComparator>;
    fn CreateSignatureReader(&self, ifilereader: ::core::option::Option<&IRdcFileReader>) -> ::windows::core::Result<IRdcSignatureReader>;
    fn GetRDCVersion(&self, currentversion: *mut u32, minimumcompatibleappversion: *mut u32) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IRdcLibrary {}
impl IRdcLibrary_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRdcLibrary_Impl, const OFFSET: isize>() -> IRdcLibrary_Vtbl {
        unsafe extern "system" fn ComputeDefaultRecursionDepth<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRdcLibrary_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, filesize: u64, depth: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.ComputeDefaultRecursionDepth(::core::mem::transmute_copy(&filesize)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(depth, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateGeneratorParameters<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRdcLibrary_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, parameterstype: GeneratorParametersType, level: u32, igeneratorparameters: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CreateGeneratorParameters(::core::mem::transmute_copy(&parameterstype), ::core::mem::transmute_copy(&level)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(igeneratorparameters, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OpenGeneratorParameters<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRdcLibrary_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, size: u32, parametersblob: *const u8, igeneratorparameters: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.OpenGeneratorParameters(::core::mem::transmute_copy(&size), ::core::mem::transmute_copy(&parametersblob)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(igeneratorparameters, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateGenerator<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRdcLibrary_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, depth: u32, igeneratorparametersarray: *const *mut ::core::ffi::c_void, igenerator: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CreateGenerator(::core::mem::transmute_copy(&depth), ::core::mem::transmute_copy(&igeneratorparametersarray)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(igenerator, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateComparator<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRdcLibrary_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, iseedsignaturesfile: *mut ::core::ffi::c_void, comparatorbuffersize: u32, icomparator: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CreateComparator(::windows::core::from_raw_borrowed(&iseedsignaturesfile), ::core::mem::transmute_copy(&comparatorbuffersize)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(icomparator, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateSignatureReader<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRdcLibrary_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ifilereader: *mut ::core::ffi::c_void, isignaturereader: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CreateSignatureReader(::windows::core::from_raw_borrowed(&ifilereader)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(isignaturereader, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetRDCVersion<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRdcLibrary_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, currentversion: *mut u32, minimumcompatibleappversion: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetRDCVersion(::core::mem::transmute_copy(&currentversion), ::core::mem::transmute_copy(&minimumcompatibleappversion)).into()
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            ComputeDefaultRecursionDepth: ComputeDefaultRecursionDepth::<Identity, Impl, OFFSET>,
            CreateGeneratorParameters: CreateGeneratorParameters::<Identity, Impl, OFFSET>,
            OpenGeneratorParameters: OpenGeneratorParameters::<Identity, Impl, OFFSET>,
            CreateGenerator: CreateGenerator::<Identity, Impl, OFFSET>,
            CreateComparator: CreateComparator::<Identity, Impl, OFFSET>,
            CreateSignatureReader: CreateSignatureReader::<Identity, Impl, OFFSET>,
            GetRDCVersion: GetRDCVersion::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRdcLibrary as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Networking_RemoteDifferentialCompression\"`, `\"Win32_Foundation\"`, `\"implement\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub trait IRdcSignatureReader_Impl: Sized {
    fn ReadHeader(&self) -> ::windows::core::Result<RDC_ErrorCode>;
    fn ReadSignatures(&self, rdcsignaturepointer: *mut RdcSignaturePointer, endofoutput: *mut super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::RuntimeName for IRdcSignatureReader {}
#[cfg(feature = "Win32_Foundation")]
impl IRdcSignatureReader_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRdcSignatureReader_Impl, const OFFSET: isize>() -> IRdcSignatureReader_Vtbl {
        unsafe extern "system" fn ReadHeader<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRdcSignatureReader_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rdc_errorcode: *mut RDC_ErrorCode) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.ReadHeader() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(rdc_errorcode, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ReadSignatures<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRdcSignatureReader_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rdcsignaturepointer: *mut RdcSignaturePointer, endofoutput: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.ReadSignatures(::core::mem::transmute_copy(&rdcsignaturepointer), ::core::mem::transmute_copy(&endofoutput)).into()
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            ReadHeader: ReadHeader::<Identity, Impl, OFFSET>,
            ReadSignatures: ReadSignatures::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRdcSignatureReader as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Networking_RemoteDifferentialCompression\"`, `\"implement\"`*"]
pub trait IRdcSimilarityGenerator_Impl: Sized {
    fn EnableSimilarity(&self) -> ::windows::core::Result<()>;
    fn Results(&self) -> ::windows::core::Result<SimilarityData>;
}
impl ::windows::core::RuntimeName for IRdcSimilarityGenerator {}
impl IRdcSimilarityGenerator_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRdcSimilarityGenerator_Impl, const OFFSET: isize>() -> IRdcSimilarityGenerator_Vtbl {
        unsafe extern "system" fn EnableSimilarity<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRdcSimilarityGenerator_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.EnableSimilarity().into()
        }
        unsafe extern "system" fn Results<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRdcSimilarityGenerator_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, similaritydata: *mut SimilarityData) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Results() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(similaritydata, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            EnableSimilarity: EnableSimilarity::<Identity, Impl, OFFSET>,
            Results: Results::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRdcSimilarityGenerator as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Networking_RemoteDifferentialCompression\"`, `\"Win32_Foundation\"`, `\"implement\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub trait ISimilarity_Impl: Sized {
    fn CreateTable(&self, path: &::windows::core::PCWSTR, truncate: super::super::Foundation::BOOL, securitydescriptor: *const u8, recordsize: u32) -> ::windows::core::Result<RdcCreatedTables>;
    fn CreateTableIndirect(&self, mapping: ::core::option::Option<&ISimilarityTraitsMapping>, fileidfile: ::core::option::Option<&IRdcFileWriter>, truncate: super::super::Foundation::BOOL, recordsize: u32) -> ::windows::core::Result<RdcCreatedTables>;
    fn CloseTable(&self, isvalid: super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn Append(&self, similarityfileid: *const SimilarityFileId, similaritydata: *const SimilarityData) -> ::windows::core::Result<()>;
    fn FindSimilarFileId(&self, similaritydata: *const SimilarityData, numberofmatchesrequired: u16, resultssize: u32) -> ::windows::core::Result<IFindSimilarResults>;
    fn CopyAndSwap(&self, newsimilaritytables: ::core::option::Option<&ISimilarity>, reportprogress: ::core::option::Option<&ISimilarityReportProgress>) -> ::windows::core::Result<()>;
    fn GetRecordCount(&self) -> ::windows::core::Result<u32>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::RuntimeName for ISimilarity {}
#[cfg(feature = "Win32_Foundation")]
impl ISimilarity_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISimilarity_Impl, const OFFSET: isize>() -> ISimilarity_Vtbl {
        unsafe extern "system" fn CreateTable<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISimilarity_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, path: ::windows::core::PCWSTR, truncate: super::super::Foundation::BOOL, securitydescriptor: *const u8, recordsize: u32, isnew: *mut RdcCreatedTables) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CreateTable(::core::mem::transmute(&path), ::core::mem::transmute_copy(&truncate), ::core::mem::transmute_copy(&securitydescriptor), ::core::mem::transmute_copy(&recordsize)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(isnew, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateTableIndirect<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISimilarity_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, mapping: *mut ::core::ffi::c_void, fileidfile: *mut ::core::ffi::c_void, truncate: super::super::Foundation::BOOL, recordsize: u32, isnew: *mut RdcCreatedTables) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CreateTableIndirect(::windows::core::from_raw_borrowed(&mapping), ::windows::core::from_raw_borrowed(&fileidfile), ::core::mem::transmute_copy(&truncate), ::core::mem::transmute_copy(&recordsize)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(isnew, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CloseTable<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISimilarity_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, isvalid: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.CloseTable(::core::mem::transmute_copy(&isvalid)).into()
        }
        unsafe extern "system" fn Append<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISimilarity_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, similarityfileid: *const SimilarityFileId, similaritydata: *const SimilarityData) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Append(::core::mem::transmute_copy(&similarityfileid), ::core::mem::transmute_copy(&similaritydata)).into()
        }
        unsafe extern "system" fn FindSimilarFileId<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISimilarity_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, similaritydata: *const SimilarityData, numberofmatchesrequired: u16, resultssize: u32, findsimilarresults: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.FindSimilarFileId(::core::mem::transmute_copy(&similaritydata), ::core::mem::transmute_copy(&numberofmatchesrequired), ::core::mem::transmute_copy(&resultssize)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(findsimilarresults, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CopyAndSwap<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISimilarity_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newsimilaritytables: *mut ::core::ffi::c_void, reportprogress: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.CopyAndSwap(::windows::core::from_raw_borrowed(&newsimilaritytables), ::windows::core::from_raw_borrowed(&reportprogress)).into()
        }
        unsafe extern "system" fn GetRecordCount<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISimilarity_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, recordcount: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetRecordCount() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(recordcount, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            CreateTable: CreateTable::<Identity, Impl, OFFSET>,
            CreateTableIndirect: CreateTableIndirect::<Identity, Impl, OFFSET>,
            CloseTable: CloseTable::<Identity, Impl, OFFSET>,
            Append: Append::<Identity, Impl, OFFSET>,
            FindSimilarFileId: FindSimilarFileId::<Identity, Impl, OFFSET>,
            CopyAndSwap: CopyAndSwap::<Identity, Impl, OFFSET>,
            GetRecordCount: GetRecordCount::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISimilarity as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Networking_RemoteDifferentialCompression\"`, `\"Win32_Foundation\"`, `\"implement\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub trait ISimilarityFileIdTable_Impl: Sized {
    fn CreateTable(&self, path: &::windows::core::PCWSTR, truncate: super::super::Foundation::BOOL, securitydescriptor: *const u8, recordsize: u32) -> ::windows::core::Result<RdcCreatedTables>;
    fn CreateTableIndirect(&self, fileidfile: ::core::option::Option<&IRdcFileWriter>, truncate: super::super::Foundation::BOOL, recordsize: u32) -> ::windows::core::Result<RdcCreatedTables>;
    fn CloseTable(&self, isvalid: super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn Append(&self, similarityfileid: *const SimilarityFileId) -> ::windows::core::Result<u32>;
    fn Lookup(&self, similarityfileindex: u32, similarityfileid: *mut SimilarityFileId) -> ::windows::core::Result<()>;
    fn Invalidate(&self, similarityfileindex: u32) -> ::windows::core::Result<()>;
    fn GetRecordCount(&self) -> ::windows::core::Result<u32>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::RuntimeName for ISimilarityFileIdTable {}
#[cfg(feature = "Win32_Foundation")]
impl ISimilarityFileIdTable_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISimilarityFileIdTable_Impl, const OFFSET: isize>() -> ISimilarityFileIdTable_Vtbl {
        unsafe extern "system" fn CreateTable<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISimilarityFileIdTable_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, path: ::windows::core::PCWSTR, truncate: super::super::Foundation::BOOL, securitydescriptor: *const u8, recordsize: u32, isnew: *mut RdcCreatedTables) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CreateTable(::core::mem::transmute(&path), ::core::mem::transmute_copy(&truncate), ::core::mem::transmute_copy(&securitydescriptor), ::core::mem::transmute_copy(&recordsize)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(isnew, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateTableIndirect<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISimilarityFileIdTable_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fileidfile: *mut ::core::ffi::c_void, truncate: super::super::Foundation::BOOL, recordsize: u32, isnew: *mut RdcCreatedTables) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CreateTableIndirect(::windows::core::from_raw_borrowed(&fileidfile), ::core::mem::transmute_copy(&truncate), ::core::mem::transmute_copy(&recordsize)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(isnew, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CloseTable<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISimilarityFileIdTable_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, isvalid: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.CloseTable(::core::mem::transmute_copy(&isvalid)).into()
        }
        unsafe extern "system" fn Append<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISimilarityFileIdTable_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, similarityfileid: *const SimilarityFileId, similarityfileindex: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Append(::core::mem::transmute_copy(&similarityfileid)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(similarityfileindex, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Lookup<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISimilarityFileIdTable_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, similarityfileindex: u32, similarityfileid: *mut SimilarityFileId) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Lookup(::core::mem::transmute_copy(&similarityfileindex), ::core::mem::transmute_copy(&similarityfileid)).into()
        }
        unsafe extern "system" fn Invalidate<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISimilarityFileIdTable_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, similarityfileindex: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Invalidate(::core::mem::transmute_copy(&similarityfileindex)).into()
        }
        unsafe extern "system" fn GetRecordCount<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISimilarityFileIdTable_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, recordcount: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetRecordCount() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(recordcount, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            CreateTable: CreateTable::<Identity, Impl, OFFSET>,
            CreateTableIndirect: CreateTableIndirect::<Identity, Impl, OFFSET>,
            CloseTable: CloseTable::<Identity, Impl, OFFSET>,
            Append: Append::<Identity, Impl, OFFSET>,
            Lookup: Lookup::<Identity, Impl, OFFSET>,
            Invalidate: Invalidate::<Identity, Impl, OFFSET>,
            GetRecordCount: GetRecordCount::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISimilarityFileIdTable as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Networking_RemoteDifferentialCompression\"`, `\"implement\"`*"]
pub trait ISimilarityReportProgress_Impl: Sized {
    fn ReportProgress(&self, percentcompleted: u32) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for ISimilarityReportProgress {}
impl ISimilarityReportProgress_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISimilarityReportProgress_Impl, const OFFSET: isize>() -> ISimilarityReportProgress_Vtbl {
        unsafe extern "system" fn ReportProgress<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISimilarityReportProgress_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, percentcompleted: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.ReportProgress(::core::mem::transmute_copy(&percentcompleted)).into()
        }
        Self { base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(), ReportProgress: ReportProgress::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISimilarityReportProgress as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Networking_RemoteDifferentialCompression\"`, `\"Win32_Foundation\"`, `\"implement\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub trait ISimilarityTableDumpState_Impl: Sized {
    fn GetNextData(&self, resultssize: u32, resultsused: *mut u32, eof: *mut super::super::Foundation::BOOL, results: *mut SimilarityDumpData) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::RuntimeName for ISimilarityTableDumpState {}
#[cfg(feature = "Win32_Foundation")]
impl ISimilarityTableDumpState_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISimilarityTableDumpState_Impl, const OFFSET: isize>() -> ISimilarityTableDumpState_Vtbl {
        unsafe extern "system" fn GetNextData<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISimilarityTableDumpState_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, resultssize: u32, resultsused: *mut u32, eof: *mut super::super::Foundation::BOOL, results: *mut SimilarityDumpData) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetNextData(::core::mem::transmute_copy(&resultssize), ::core::mem::transmute_copy(&resultsused), ::core::mem::transmute_copy(&eof), ::core::mem::transmute_copy(&results)).into()
        }
        Self { base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(), GetNextData: GetNextData::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISimilarityTableDumpState as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Networking_RemoteDifferentialCompression\"`, `\"Win32_Foundation\"`, `\"implement\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub trait ISimilarityTraitsMappedView_Impl: Sized {
    fn Flush(&self) -> ::windows::core::Result<()>;
    fn Unmap(&self) -> ::windows::core::Result<()>;
    fn Get(&self, index: u64, dirty: super::super::Foundation::BOOL, numelements: u32) -> ::windows::core::Result<SimilarityMappedViewInfo>;
    fn GetView(&self, mappedpagebegin: *mut *mut u8, mappedpageend: *mut *mut u8);
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::RuntimeName for ISimilarityTraitsMappedView {}
#[cfg(feature = "Win32_Foundation")]
impl ISimilarityTraitsMappedView_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISimilarityTraitsMappedView_Impl, const OFFSET: isize>() -> ISimilarityTraitsMappedView_Vtbl {
        unsafe extern "system" fn Flush<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISimilarityTraitsMappedView_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Flush().into()
        }
        unsafe extern "system" fn Unmap<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISimilarityTraitsMappedView_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Unmap().into()
        }
        unsafe extern "system" fn Get<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISimilarityTraitsMappedView_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u64, dirty: super::super::Foundation::BOOL, numelements: u32, viewinfo: *mut SimilarityMappedViewInfo) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Get(::core::mem::transmute_copy(&index), ::core::mem::transmute_copy(&dirty), ::core::mem::transmute_copy(&numelements)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(viewinfo, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetView<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISimilarityTraitsMappedView_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, mappedpagebegin: *mut *mut u8, mappedpageend: *mut *mut u8) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetView(::core::mem::transmute_copy(&mappedpagebegin), ::core::mem::transmute_copy(&mappedpageend))
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Flush: Flush::<Identity, Impl, OFFSET>,
            Unmap: Unmap::<Identity, Impl, OFFSET>,
            Get: Get::<Identity, Impl, OFFSET>,
            GetView: GetView::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISimilarityTraitsMappedView as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Networking_RemoteDifferentialCompression\"`, `\"implement\"`*"]
pub trait ISimilarityTraitsMapping_Impl: Sized {
    fn CloseMapping(&self);
    fn SetFileSize(&self, filesize: u64) -> ::windows::core::Result<()>;
    fn GetFileSize(&self) -> ::windows::core::Result<u64>;
    fn OpenMapping(&self, accessmode: RdcMappingAccessMode, begin: u64, end: u64) -> ::windows::core::Result<u64>;
    fn ResizeMapping(&self, accessmode: RdcMappingAccessMode, begin: u64, end: u64) -> ::windows::core::Result<u64>;
    fn GetPageSize(&self, pagesize: *mut u32) -> ();
    fn CreateView(&self, minimummappedpages: u32, accessmode: RdcMappingAccessMode) -> ::windows::core::Result<ISimilarityTraitsMappedView>;
}
impl ::windows::core::RuntimeName for ISimilarityTraitsMapping {}
impl ISimilarityTraitsMapping_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISimilarityTraitsMapping_Impl, const OFFSET: isize>() -> ISimilarityTraitsMapping_Vtbl {
        unsafe extern "system" fn CloseMapping<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISimilarityTraitsMapping_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.CloseMapping()
        }
        unsafe extern "system" fn SetFileSize<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISimilarityTraitsMapping_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, filesize: u64) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetFileSize(::core::mem::transmute_copy(&filesize)).into()
        }
        unsafe extern "system" fn GetFileSize<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISimilarityTraitsMapping_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, filesize: *mut u64) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetFileSize() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(filesize, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OpenMapping<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISimilarityTraitsMapping_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, accessmode: RdcMappingAccessMode, begin: u64, end: u64, actualend: *mut u64) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.OpenMapping(::core::mem::transmute_copy(&accessmode), ::core::mem::transmute_copy(&begin), ::core::mem::transmute_copy(&end)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(actualend, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ResizeMapping<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISimilarityTraitsMapping_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, accessmode: RdcMappingAccessMode, begin: u64, end: u64, actualend: *mut u64) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.ResizeMapping(::core::mem::transmute_copy(&accessmode), ::core::mem::transmute_copy(&begin), ::core::mem::transmute_copy(&end)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(actualend, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPageSize<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISimilarityTraitsMapping_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pagesize: *mut u32) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetPageSize(::core::mem::transmute_copy(&pagesize))
        }
        unsafe extern "system" fn CreateView<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISimilarityTraitsMapping_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, minimummappedpages: u32, accessmode: RdcMappingAccessMode, mappedview: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CreateView(::core::mem::transmute_copy(&minimummappedpages), ::core::mem::transmute_copy(&accessmode)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(mappedview, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            CloseMapping: CloseMapping::<Identity, Impl, OFFSET>,
            SetFileSize: SetFileSize::<Identity, Impl, OFFSET>,
            GetFileSize: GetFileSize::<Identity, Impl, OFFSET>,
            OpenMapping: OpenMapping::<Identity, Impl, OFFSET>,
            ResizeMapping: ResizeMapping::<Identity, Impl, OFFSET>,
            GetPageSize: GetPageSize::<Identity, Impl, OFFSET>,
            CreateView: CreateView::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISimilarityTraitsMapping as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Networking_RemoteDifferentialCompression\"`, `\"Win32_Foundation\"`, `\"implement\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub trait ISimilarityTraitsTable_Impl: Sized {
    fn CreateTable(&self, path: &::windows::core::PCWSTR, truncate: super::super::Foundation::BOOL, securitydescriptor: *const u8) -> ::windows::core::Result<RdcCreatedTables>;
    fn CreateTableIndirect(&self, mapping: ::core::option::Option<&ISimilarityTraitsMapping>, truncate: super::super::Foundation::BOOL) -> ::windows::core::Result<RdcCreatedTables>;
    fn CloseTable(&self, isvalid: super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn Append(&self, data: *const SimilarityData, fileindex: u32) -> ::windows::core::Result<()>;
    fn FindSimilarFileIndex(&self, similaritydata: *const SimilarityData, numberofmatchesrequired: u16, findsimilarfileindexresults: *mut FindSimilarFileIndexResults, resultssize: u32, resultsused: *mut u32) -> ::windows::core::Result<()>;
    fn BeginDump(&self) -> ::windows::core::Result<ISimilarityTableDumpState>;
    fn GetLastIndex(&self) -> ::windows::core::Result<u32>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::RuntimeName for ISimilarityTraitsTable {}
#[cfg(feature = "Win32_Foundation")]
impl ISimilarityTraitsTable_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISimilarityTraitsTable_Impl, const OFFSET: isize>() -> ISimilarityTraitsTable_Vtbl {
        unsafe extern "system" fn CreateTable<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISimilarityTraitsTable_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, path: ::windows::core::PCWSTR, truncate: super::super::Foundation::BOOL, securitydescriptor: *const u8, isnew: *mut RdcCreatedTables) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CreateTable(::core::mem::transmute(&path), ::core::mem::transmute_copy(&truncate), ::core::mem::transmute_copy(&securitydescriptor)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(isnew, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateTableIndirect<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISimilarityTraitsTable_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, mapping: *mut ::core::ffi::c_void, truncate: super::super::Foundation::BOOL, isnew: *mut RdcCreatedTables) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CreateTableIndirect(::windows::core::from_raw_borrowed(&mapping), ::core::mem::transmute_copy(&truncate)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(isnew, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CloseTable<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISimilarityTraitsTable_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, isvalid: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.CloseTable(::core::mem::transmute_copy(&isvalid)).into()
        }
        unsafe extern "system" fn Append<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISimilarityTraitsTable_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, data: *const SimilarityData, fileindex: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Append(::core::mem::transmute_copy(&data), ::core::mem::transmute_copy(&fileindex)).into()
        }
        unsafe extern "system" fn FindSimilarFileIndex<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISimilarityTraitsTable_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, similaritydata: *const SimilarityData, numberofmatchesrequired: u16, findsimilarfileindexresults: *mut FindSimilarFileIndexResults, resultssize: u32, resultsused: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.FindSimilarFileIndex(::core::mem::transmute_copy(&similaritydata), ::core::mem::transmute_copy(&numberofmatchesrequired), ::core::mem::transmute_copy(&findsimilarfileindexresults), ::core::mem::transmute_copy(&resultssize), ::core::mem::transmute_copy(&resultsused)).into()
        }
        unsafe extern "system" fn BeginDump<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISimilarityTraitsTable_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, similaritytabledumpstate: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.BeginDump() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(similaritytabledumpstate, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetLastIndex<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISimilarityTraitsTable_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fileindex: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetLastIndex() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(fileindex, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            CreateTable: CreateTable::<Identity, Impl, OFFSET>,
            CreateTableIndirect: CreateTableIndirect::<Identity, Impl, OFFSET>,
            CloseTable: CloseTable::<Identity, Impl, OFFSET>,
            Append: Append::<Identity, Impl, OFFSET>,
            FindSimilarFileIndex: FindSimilarFileIndex::<Identity, Impl, OFFSET>,
            BeginDump: BeginDump::<Identity, Impl, OFFSET>,
            GetLastIndex: GetLastIndex::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISimilarityTraitsTable as ::windows::core::ComInterface>::IID
    }
}
