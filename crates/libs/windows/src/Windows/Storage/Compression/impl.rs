#[cfg(all(feature = "Foundation", feature = "Storage_Streams", feature = "implement_exclusive"))]
pub trait ICompressorImpl: Sized + IClosableImpl + IOutputStreamImpl {
    fn FinishAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>>;
    fn DetachStream(&self) -> ::windows::core::Result<super::Streams::IOutputStream>;
}
#[cfg(all(feature = "Foundation", feature = "Storage_Streams", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ICompressor {
    const NAME: &'static str = "Windows.Storage.Compression.ICompressor";
}
#[cfg(all(feature = "Foundation", feature = "Storage_Streams", feature = "implement_exclusive"))]
impl ICompressorVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICompressorImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICompressorVtbl {
        unsafe extern "system" fn FinishAsync<Impl: ICompressorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FinishAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DetachStream<Impl: ICompressorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DetachStream() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ICompressor, BASE_OFFSET>(),
            FinishAsync: FinishAsync::<Impl, IMPL_OFFSET>,
            DetachStream: DetachStream::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICompressor as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Storage_Streams", feature = "implement_exclusive"))]
pub trait ICompressorFactoryImpl: Sized {
    fn CreateCompressor(&self, underlyingstream: &::core::option::Option<super::Streams::IOutputStream>) -> ::windows::core::Result<Compressor>;
    fn CreateCompressorEx(&self, underlyingstream: &::core::option::Option<super::Streams::IOutputStream>, algorithm: CompressAlgorithm, blocksize: u32) -> ::windows::core::Result<Compressor>;
}
#[cfg(all(feature = "Storage_Streams", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ICompressorFactory {
    const NAME: &'static str = "Windows.Storage.Compression.ICompressorFactory";
}
#[cfg(all(feature = "Storage_Streams", feature = "implement_exclusive"))]
impl ICompressorFactoryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICompressorFactoryImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICompressorFactoryVtbl {
        unsafe extern "system" fn CreateCompressor<Impl: ICompressorFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, underlyingstream: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateCompressor(&*(&underlyingstream as *const <super::Streams::IOutputStream as ::windows::core::Abi>::Abi as *const <super::Streams::IOutputStream as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateCompressorEx<Impl: ICompressorFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, underlyingstream: ::windows::core::RawPtr, algorithm: CompressAlgorithm, blocksize: u32, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateCompressorEx(&*(&underlyingstream as *const <super::Streams::IOutputStream as ::windows::core::Abi>::Abi as *const <super::Streams::IOutputStream as ::windows::core::DefaultType>::DefaultType), algorithm, blocksize) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ICompressorFactory, BASE_OFFSET>(),
            CreateCompressor: CreateCompressor::<Impl, IMPL_OFFSET>,
            CreateCompressorEx: CreateCompressorEx::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICompressorFactory as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Storage_Streams", feature = "implement_exclusive"))]
pub trait IDecompressorImpl: Sized + IClosableImpl + IInputStreamImpl {
    fn DetachStream(&self) -> ::windows::core::Result<super::Streams::IInputStream>;
}
#[cfg(all(feature = "Foundation", feature = "Storage_Streams", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IDecompressor {
    const NAME: &'static str = "Windows.Storage.Compression.IDecompressor";
}
#[cfg(all(feature = "Foundation", feature = "Storage_Streams", feature = "implement_exclusive"))]
impl IDecompressorVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDecompressorImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDecompressorVtbl {
        unsafe extern "system" fn DetachStream<Impl: IDecompressorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DetachStream() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IDecompressor, BASE_OFFSET>(), DetachStream: DetachStream::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDecompressor as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Storage_Streams", feature = "implement_exclusive"))]
pub trait IDecompressorFactoryImpl: Sized {
    fn CreateDecompressor(&self, underlyingstream: &::core::option::Option<super::Streams::IInputStream>) -> ::windows::core::Result<Decompressor>;
}
#[cfg(all(feature = "Storage_Streams", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IDecompressorFactory {
    const NAME: &'static str = "Windows.Storage.Compression.IDecompressorFactory";
}
#[cfg(all(feature = "Storage_Streams", feature = "implement_exclusive"))]
impl IDecompressorFactoryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDecompressorFactoryImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDecompressorFactoryVtbl {
        unsafe extern "system" fn CreateDecompressor<Impl: IDecompressorFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, underlyingstream: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateDecompressor(&*(&underlyingstream as *const <super::Streams::IInputStream as ::windows::core::Abi>::Abi as *const <super::Streams::IInputStream as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IDecompressorFactory, BASE_OFFSET>(),
            CreateDecompressor: CreateDecompressor::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDecompressorFactory as ::windows::core::Interface>::IID
    }
}
