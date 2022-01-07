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
    pub const fn new<Impl: ICompressorImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ICompressorVtbl {
        unsafe extern "system" fn FinishAsync<Impl: ICompressorImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).FinishAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DetachStream<Impl: ICompressorImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).DetachStream() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ICompressor>, base.5, FinishAsync::<Impl, OFFSET>, DetachStream::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ICompressorFactoryImpl: Sized {
    fn CreateCompressor(&self, underlyingstream: &::core::option::Option<super::Streams::IOutputStream>) -> ::windows::core::Result<Compressor>;
    fn CreateCompressorEx(&self, underlyingstream: &::core::option::Option<super::Streams::IOutputStream>, algorithm: CompressAlgorithm, blocksize: u32) -> ::windows::core::Result<Compressor>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ICompressorFactory {
    const NAME: &'static str = "Windows.Storage.Compression.ICompressorFactory";
}
#[cfg(feature = "implement_exclusive")]
impl ICompressorFactoryVtbl {
    pub const fn new<Impl: ICompressorFactoryImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ICompressorFactoryVtbl {
        unsafe extern "system" fn CreateCompressor<Impl: ICompressorFactoryImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, underlyingstream: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateCompressor(&*(&underlyingstream as *const <super::Streams::IOutputStream as ::windows::core::Abi>::Abi as *const <super::Streams::IOutputStream as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateCompressorEx<Impl: ICompressorFactoryImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, underlyingstream: ::windows::core::RawPtr, algorithm: CompressAlgorithm, blocksize: u32, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateCompressorEx(&*(&underlyingstream as *const <super::Streams::IOutputStream as ::windows::core::Abi>::Abi as *const <super::Streams::IOutputStream as ::windows::core::DefaultType>::DefaultType), algorithm, blocksize) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ICompressorFactory>, base.5, CreateCompressor::<Impl, OFFSET>, CreateCompressorEx::<Impl, OFFSET>)
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
    pub const fn new<Impl: IDecompressorImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IDecompressorVtbl {
        unsafe extern "system" fn DetachStream<Impl: IDecompressorImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).DetachStream() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IDecompressor>, base.5, DetachStream::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IDecompressorFactoryImpl: Sized {
    fn CreateDecompressor(&self, underlyingstream: &::core::option::Option<super::Streams::IInputStream>) -> ::windows::core::Result<Decompressor>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IDecompressorFactory {
    const NAME: &'static str = "Windows.Storage.Compression.IDecompressorFactory";
}
#[cfg(feature = "implement_exclusive")]
impl IDecompressorFactoryVtbl {
    pub const fn new<Impl: IDecompressorFactoryImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IDecompressorFactoryVtbl {
        unsafe extern "system" fn CreateDecompressor<Impl: IDecompressorFactoryImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, underlyingstream: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateDecompressor(&*(&underlyingstream as *const <super::Streams::IInputStream as ::windows::core::Abi>::Abi as *const <super::Streams::IInputStream as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IDecompressorFactory>, base.5, CreateDecompressor::<Impl, OFFSET>)
    }
}
