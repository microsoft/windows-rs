#[cfg(all(feature = "Foundation", feature = "Storage_Streams", feature = "implement_exclusive"))]
pub trait ICompressor_Impl: Sized + super::super::Foundation::IClosable_Impl + super::Streams::IOutputStream_Impl {
    fn FinishAsync(&mut self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>>;
    fn DetachStream(&mut self) -> ::windows::core::Result<super::Streams::IOutputStream>;
}
#[cfg(all(feature = "Foundation", feature = "Storage_Streams", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ICompressor {
    const NAME: &'static str = "Windows.Storage.Compression.ICompressor";
}
#[cfg(all(feature = "Foundation", feature = "Storage_Streams", feature = "implement_exclusive"))]
impl ICompressor_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICompressor_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICompressor_Vtbl {
        unsafe extern "system" fn FinishAsync<Impl: ICompressor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn DetachStream<Impl: ICompressor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
pub trait ICompressorFactory_Impl: Sized {
    fn CreateCompressor(&mut self, underlyingstream: &::core::option::Option<super::Streams::IOutputStream>) -> ::windows::core::Result<Compressor>;
    fn CreateCompressorEx(&mut self, underlyingstream: &::core::option::Option<super::Streams::IOutputStream>, algorithm: CompressAlgorithm, blocksize: u32) -> ::windows::core::Result<Compressor>;
}
#[cfg(all(feature = "Storage_Streams", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ICompressorFactory {
    const NAME: &'static str = "Windows.Storage.Compression.ICompressorFactory";
}
#[cfg(all(feature = "Storage_Streams", feature = "implement_exclusive"))]
impl ICompressorFactory_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICompressorFactory_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICompressorFactory_Vtbl {
        unsafe extern "system" fn CreateCompressor<Impl: ICompressorFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, underlyingstream: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn CreateCompressorEx<Impl: ICompressorFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, underlyingstream: ::windows::core::RawPtr, algorithm: CompressAlgorithm, blocksize: u32, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
pub trait IDecompressor_Impl: Sized + super::super::Foundation::IClosable_Impl + super::Streams::IInputStream_Impl {
    fn DetachStream(&mut self) -> ::windows::core::Result<super::Streams::IInputStream>;
}
#[cfg(all(feature = "Foundation", feature = "Storage_Streams", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IDecompressor {
    const NAME: &'static str = "Windows.Storage.Compression.IDecompressor";
}
#[cfg(all(feature = "Foundation", feature = "Storage_Streams", feature = "implement_exclusive"))]
impl IDecompressor_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDecompressor_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDecompressor_Vtbl {
        unsafe extern "system" fn DetachStream<Impl: IDecompressor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
pub trait IDecompressorFactory_Impl: Sized {
    fn CreateDecompressor(&mut self, underlyingstream: &::core::option::Option<super::Streams::IInputStream>) -> ::windows::core::Result<Decompressor>;
}
#[cfg(all(feature = "Storage_Streams", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IDecompressorFactory {
    const NAME: &'static str = "Windows.Storage.Compression.IDecompressorFactory";
}
#[cfg(all(feature = "Storage_Streams", feature = "implement_exclusive"))]
impl IDecompressorFactory_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDecompressorFactory_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDecompressorFactory_Vtbl {
        unsafe extern "system" fn CreateDecompressor<Impl: IDecompressorFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, underlyingstream: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
